#!/usr/bin/env python3
"""
Recursively detect and repair common mojibake in a project repository.

Examples:

    # Preview only; no files are modified
    python fix_mojibake.py /path/to/project

    # Apply changes and create .bak backups
    python fix_mojibake.py /path/to/project --apply --backup

    # Apply changes without backups
    python fix_mojibake.py /path/to/project --apply

    # Show detailed diagnostics
    python fix_mojibake.py /path/to/project --verbose

Coverage:
    The script recognises UTF-8 text that was mis-decoded as Windows-1252,
    Latin-1, Mac Roman, DOS code pages (437/850) and several other single-byte
    code pages, including double- and triple-encoded text, mojibake produced
    by lossy decoders (U+FFFD substitutions), stray byte-order marks and
    HTML-escaped mojibake.

Important:
    Commit or back up your repository before using --apply.
"""

from __future__ import annotations

import argparse
import difflib
import os
import re
import shutil
import sys
from dataclasses import dataclass, field
from functools import lru_cache
from html.entities import html5
from pathlib import Path


# Directories normally containing generated, cached, vendored, or external files.
DEFAULT_EXCLUDED_DIRS = {
    ".git",
    ".hg",
    ".svn",
    ".idea",
    ".vscode",
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    ".tox",
    ".venv",
    "venv",
    "env",
    "node_modules",
    "vendor",
    "dist",
    "build",
    "coverage",
    ".next",
    ".nuxt",
    ".output",
    "target",
    "bin",
    "obj",
}

# Files that should generally not be rewritten even when they happen to decode.
EXCLUDED_FILENAMES = {
    "package-lock.json",
    "npm-shrinkwrap.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "poetry.lock",
    "composer.lock",
    "Cargo.lock",
}

# Common binary extensions. Binary detection is also performed using file bytes.
BINARY_EXTENSIONS = {
    ".7z",
    ".a",
    ".apk",
    ".avi",
    ".bin",
    ".bmp",
    ".class",
    ".dll",
    ".doc",
    ".docx",
    ".dylib",
    ".eot",
    ".exe",
    ".gif",
    ".gz",
    ".ico",
    ".jar",
    ".jpeg",
    ".jpg",
    ".lockb",
    ".mov",
    ".mp3",
    ".mp4",
    ".o",
    ".otf",
    ".pdf",
    ".png",
    ".pyc",
    ".rar",
    ".so",
    ".tar",
    ".tif",
    ".tiff",
    ".ttf",
    ".wasm",
    ".webm",
    ".webp",
    ".woff",
    ".woff2",
    ".xls",
    ".xlsx",
    ".zip",
}


# ---------------------------------------------------------------------------
# Mojibake signatures
#
# Every table below maps a literal fragment to a suspicion weight. Weights are
# additive and only guide the decision: a repair is accepted solely when it
# lowers the total score, so an over-eager signature cannot corrupt good text.
# ---------------------------------------------------------------------------

# UTF-8 bytes decoded as Windows-1252 or Latin-1. This is by far the most
# common failure and covers Western European text, punctuation and emoji.
CP1252_FRAGMENTS = {
    # Generic lead bytes for two-byte sequences.
    "Ã": 4,
    "Â": 4,
    # Latin-1 supplement letters (C3 xx).
    "Ã ": 6,
    "Ã¡": 6,
    "Ã¢": 6,
    "Ã£": 6,
    "Ã¤": 6,
    "Ã¥": 6,
    "Ã¦": 6,
    "Ã§": 6,
    "Ã¨": 6,
    "Ã©": 6,
    "Ãª": 6,
    "Ã«": 6,
    "Ã¬": 6,
    "Ã\xad": 6,
    "Ã®": 6,
    "Ã¯": 6,
    "Ã°": 6,
    "Ã±": 6,
    "Ã²": 6,
    "Ã³": 6,
    "Ã´": 6,
    "Ãµ": 6,
    "Ã¶": 6,
    "Ã¸": 6,
    "Ã¹": 6,
    "Ãº": 6,
    "Ã»": 6,
    "Ã¼": 6,
    "Ã½": 6,
    "Ã¾": 6,
    "Ã¿": 6,
    "ÃŸ": 6,
    "Ã‰": 6,
    "Ã–": 6,
    "Ãœ": 6,
    "Ã„": 6,
    "Ã…": 6,
    "Ã‡": 6,
    "Ã‘": 6,
    "Ã˜": 6,
    # Latin-1 punctuation and symbols (C2 xx).
    "Â\xa0": 6,
    "Â¡": 6,
    "Â¢": 6,
    "Â£": 6,
    "Â¤": 6,
    "Â¥": 6,
    "Â¦": 6,
    "Â§": 6,
    "Â¨": 6,
    "Â©": 6,
    "Âª": 6,
    "Â«": 6,
    "Â¬": 6,
    "Â\xad": 6,
    "Â®": 6,
    "Â¯": 6,
    "Â°": 6,
    "Â±": 6,
    "Â²": 6,
    "Â³": 6,
    "Â´": 6,
    "Âµ": 6,
    "Â¶": 6,
    "Â·": 6,
    "Â¸": 6,
    "Â¹": 6,
    "Âº": 6,
    "Â»": 6,
    "Â¼": 6,
    "Â½": 6,
    "Â¾": 6,
    "Â¿": 6,
    # General punctuation (E2 80 xx).
    "â€": 5,
    "â€˜": 6,
    "â€™": 6,
    "â€š": 6,
    "â€›": 6,
    "â€œ": 6,
    "â€\x9d": 6,
    "â€ž": 6,
    "â€Ÿ": 6,
    "â€“": 6,
    "â€”": 6,
    "â€•": 6,
    "â€¦": 6,
    "â€¢": 6,
    "â€ ": 6,
    "â€¡": 6,
    "â€°": 6,
    "â€²": 6,
    "â€³": 6,
    "â€¹": 6,
    "â€º": 6,
    "â€‹": 8,  # zero-width space
    "â€Œ": 8,  # zero-width non-joiner
    "â€\x8d": 8,  # zero-width joiner
    "â€Ž": 8,  # left-to-right mark
    "â€\x8f": 8,  # right-to-left mark
    "â€¨": 8,  # line separator
    # Currency, letterlike symbols, arrows, maths, shapes (E2 8x-9x xx).
    "â‚¬": 6,
    "â‚¹": 6,  # rupee sign
    "â‚±": 6,
    "â„¢": 6,
    "â„–": 6,
    "â„¹": 6,
    "â…": 6,
    "â†": 6,
    "â†’": 6,
    "â†\x90": 6,
    "â†‘": 6,
    "â†“": 6,
    "â†”": 6,
    "â‡": 6,
    "âˆ": 6,
    "â‰": 6,
    "âŒ": 6,
    "â–": 6,
    "â—": 6,
    "â˜": 6,
    "â™": 6,
    "âš": 6,
    "âœ": 6,
    "â­": 6,
    "â®": 6,
    "â¯": 6,
    "â°": 6,
    # Emoji and supplementary planes (F0 9F xx xx).
    "ðŸ": 7,
    "ð\x9f": 7,
    "ðŸ˜": 8,
    "ðŸ‘": 8,
    "ðŸ”": 8,
    "ðŸ“": 8,
    "ðŸš": 8,
    "ðŸŽ": 8,
    "ðŸ¥": 8,
    "ðŸ§": 8,
    "ðŸ ": 8,
    "ðŸ¡": 8,
    # Non-Latin scripts commonly seen in mixed-language repositories.
    "à¤": 6,  # Devanagari
    "à¥": 6,
    "àª": 6,  # Gujarati
    "à¬": 6,  # Odia
    "à®": 6,  # Tamil
    "à°": 6,  # Telugu
    "à²": 6,  # Kannada
    "à´": 6,  # Malayalam
    "à¦": 6,  # Bengali
    "à¨": 6,  # Gurmukhi
    "à¸": 6,  # Thai
    "Ð°": 5,  # Cyrillic
    "Ð¾": 5,
    "Ñ€": 5,
    "Ñ\x81": 5,
    "Î±": 5,  # Greek
    "Î²": 5,
    "Ï‚": 5,
    "Ø§": 5,  # Arabic
    "Ø¹": 5,
    "Ù„": 5,
    "×\x90": 5,  # Hebrew
    "×‘": 5,
    "×©": 5,
    "ã€": 6,  # CJK / kana
    "ãƒ": 6,
    "ä¸": 6,
    "æ—": 6,
    "é–": 6,
    "í•": 6,  # Hangul
    "ì„": 6,
}

# UTF-8 bytes decoded as Mac Roman. C3 renders as U+221A and E2 as U+2021.
MAC_ROMAN_FRAGMENTS = {
    "√©": 6,
    "√®": 6,
    "√†": 6,
    "√°": 6,
    "√§": 6,
    "√¥": 6,
    "√±": 6,
    "√º": 6,
    "√∂": 6,
    "√ü": 6,
    "√ß": 6,
    "√Ø": 6,
    "√Ç": 6,
    "√¢": 6,
    "‚Ä": 6,
    "‚Äô": 7,
    "‚Äú": 7,
    "‚Äù": 7,
    "‚Äì": 7,
    "‚Äî": 7,
    "‚Ä¶": 7,
    "‚Ä¢": 7,
    "√Ø¬ø¬Ω": 10,
    "ÔøΩ": 10,
    "√¢‚Ç¨": 8,
}

# UTF-8 bytes decoded as DOS code page 437 or 850 (console redirection output).
DOS_FRAGMENTS = {
    "ΓÇ": 6,
    "ΓÇÖ": 7,
    "ΓÇÜ": 7,
    "ΓÇ£": 7,
    "ΓÇ¥": 7,
    "ΓÇô": 7,
    "ΓÇö": 7,
    "ΓÇ¦": 7,
    "ΓÇó": 7,
    "ΓÇÿ": 7,
    "ΓÇ×": 7,
    "Γé¼": 7,
    "Γäó": 7,
    "├©": 6,
    "├¡": 6,
    "├¢": 6,
    "├¤": 6,
    "├╝": 6,
    "├ñ": 6,
    "├╢": 6,
    "├ƒ": 6,
    "┬á": 6,
    "┬░": 6,
    "┬®": 6,
    "ÔÇ": 6,
    "ÔÇÖ": 7,
    "ÔÇ£": 7,
    "ÔÇ¥": 7,
    "ÔÇô": 7,
    "ÔÇö": 7,
    "ÔÇÿ": 7,
    "ÔÇ¦": 7,
    "Ô£ô": 7,
    "Ôé¼": 7,
}

# UTF-8 bytes decoded as a Cyrillic, Central European or Turkish code page.
CODEPAGE_FRAGMENTS = {
    "Ð²Ð": 7,
    "Ñ\x80Ð": 7,
    "РїРѕ": 8,
    "СЂС": 8,
    "Ð¿Ñ€Ð¸": 8,
    "Ã‚Âµ": 8,
    "ÄŸ": 5,  # UTF-8 read as cp1254 (Turkish)
    "Å¾": 5,  # UTF-8 read as cp1250 (Central European)
    "Å¡": 5,
    "Å„": 5,
    "Ä…": 5,
    "Ä™": 5,
    "Å‚": 5,
    "Ä‡": 5,
    "Ã¦â€": 8,
    "цЪ": 6,  # UTF-8 read as KOI8-R
    "ц√": 6,
}

# Text that was mangled twice or three times. These are strong signals: no
# natural language produces them.
MULTI_ENCODED_FRAGMENTS = {
    "Ã‚Â": 9,
    "ÃƒÂ": 9,
    "Ãƒâ€": 10,
    "ÃƒÆ’": 11,
    "Ã¢â‚¬": 10,
    "Ã¢â‚¬â„¢": 12,
    "Ã¢â‚¬Å“": 12,
    "Ã¢â‚¬\x9d": 12,
    "Ã¢â‚¬â€œ": 12,
    "Ã¢â‚¬Â¦": 12,
    "Ã°Å¸": 11,
    "Ã‚Â£": 10,
    "Ã‚Â©": 10,
    "Ã‚Â\xa0": 10,
    "Ã\x83Â": 10,
    "Ã¯Â»Â¿": 12,
    "Ã¯Â¿Â½": 12,
}

# Structural damage: byte-order marks, replacement characters and remnants of
# a decoder that gave up.
STRUCTURAL_FRAGMENTS = {
    "ï»¿": 8,  # UTF-8 BOM decoded as cp1252
    "ï¿½": 10,  # U+FFFD decoded as cp1252
    "�": 12,  # replacement character
    "﻿": 6,  # stray byte-order mark
    "\x1a": 4,  # DOS end-of-file marker in text
    "ØŒ": 5,
    "¤": 2,
}

# Mojibake that was subsequently HTML-escaped. Only scored by default; pass
# --fix-entities to also attempt a repair.
HTML_ENTITY_FRAGMENTS = {
    "&Acirc;": 4,
    "&Atilde;": 4,
    "&atilde;": 4,
    "&acirc;": 4,
    "&Auml;&sup3;": 5,
    "&acirc;&euro;&trade;": 8,
    "&acirc;&#128;&#153;": 8,
    "&Atilde;&copy;": 8,
    "&Atilde;&sup3;": 8,
    "&Acirc;&nbsp;": 8,
}

# Characters and fragments commonly produced by encoding round-trip errors.
SUSPICIOUS_FRAGMENTS: dict[str, int] = {}

for _table in (
    CP1252_FRAGMENTS,
    MAC_ROMAN_FRAGMENTS,
    DOS_FRAGMENTS,
    CODEPAGE_FRAGMENTS,
    MULTI_ENCODED_FRAGMENTS,
    STRUCTURAL_FRAGMENTS,
    HTML_ENTITY_FRAGMENTS,
):
    # Later tables must not silently lower an earlier, stronger weight.
    for _fragment, _weight in _table.items():
        SUSPICIOUS_FRAGMENTS[_fragment] = max(
            SUSPICIOUS_FRAGMENTS.get(_fragment, 0),
            _weight,
        )

del _table, _fragment, _weight

# Control characters that commonly appear when text was decoded incorrectly.
BAD_CONTROL_CODEPOINTS = set(range(0x80, 0xA0))

# A generic detector for any mis-decoded UTF-8 sequence, including scripts not
# covered by the literal tables above: a character that stands for a UTF-8 lead
# byte (0xC2-0xFF) immediately followed by one that stands for a continuation
# byte (0x80-0xBF). Undefined cp1252 bytes decode to U+FFFD, which is included
# so that lossy mojibake is caught as well.
_LEAD_CHARS = bytes(range(0xC2, 0x100)).decode("cp1252")
_CONTINUATION_CHARS = bytes(range(0x80, 0xC0)).decode("cp1252", errors="replace")

MOJIBAKE_PAIR_PATTERN = re.compile(
    "[{lead}][{continuation}]".format(
        lead=re.escape(_LEAD_CHARS),
        continuation=re.escape("".join(sorted(set(_CONTINUATION_CHARS)))),
    )
)

MOJIBAKE_PAIR_WEIGHT = 3

# Code points that a wrong single-byte table can produce while still forming
# valid UTF-8. Real text practically never contains them.
PRIVATE_USE_RANGES = (
    (0xE000, 0xF8FF),
    (0xF0000, 0xFFFFD),
    (0x100000, 0x10FFFD),
)

NON_CHARACTER_CODEPOINTS = frozenset(
    list(range(0xFDD0, 0xFDF0))
    + [plane * 0x10000 + offset for plane in range(17) for offset in (0xFFFE, 0xFFFF)]
)

# UTF-8 read as a Cyrillic or Greek code page produces alternating case inside
# words ("РїРѕ", "Ξ±ΟΞ¿"). A capital letter directly after a lowercase one is
# effectively nonexistent in those scripts, unlike Latin camelCase.
BICAMERAL_SCRIPT_RANGES = (
    (0x0370, 0x03FF),  # Greek
    (0x0400, 0x04FF),  # Cyrillic
    (0x0500, 0x052F),  # Cyrillic supplement
)

MIDWORD_CAPITAL_WEIGHT = 4

# Single-byte encodings worth testing when reversing a bad decode. cp1252 and
# latin-1 come first because they cover the overwhelming majority of cases.
CANDIDATE_ENCODINGS = (
    "cp1252",
    "latin-1",
    "mac_roman",
    "cp437",
    "cp850",
    "cp1250",
    "cp1251",
    "cp1253",
    "cp1254",
    "cp1256",
    "cp1257",
    "koi8-r",
)

# Sequences that cannot be reversed by re-encoding because information was
# destroyed: Windows-1252 has no mapping for 0x81, 0x8D, 0x8F, 0x90 and 0x9D,
# so decoders substitute U+FFFD before the text is ever written to disk. Only
# unambiguous sequences are listed.
LOSSY_SEQUENCES = {
    "â€�": "”",  # E2 80 9D right double quotation mark
    "â€��": "”",
    "Ã¢â‚¬�": "”",  # double-encoded variant
    "ï¿�": "�",
    "���": "�",
}

# The characters Windows-1252 places in the C1 range. Together with the
# Latin-1 supplement they are exactly the characters a mis-decode can emit,
# which bounds what --fix-entities is allowed to unescape.
CP1252_SPECIAL_CODEPOINTS = frozenset(
    ord(char)
    for char in bytes(range(0x80, 0xA0)).decode("cp1252", errors="ignore")
)


def _is_mojibake_codepoint(codepoint: int) -> bool:
    return 0x80 <= codepoint <= 0xFF or codepoint in CP1252_SPECIAL_CODEPOINTS


# HTML entities for characters that mojibake is made of. Unescaping them is
# only useful when a repair pass can then reverse the damage, so --fix-entities
# keeps the result solely when the score improves; legitimate entities such as
# &copy; in clean HTML are therefore left alone.
MOJIBAKE_ENTITIES = {
    f"&{name}": char
    for name, char in html5.items()
    if name.endswith(";")
    and len(char) == 1
    and _is_mojibake_codepoint(ord(char))
}

NUMERIC_ENTITY_PATTERN = re.compile(r"&#(?:x([0-9a-fA-F]+)|(\d+));")


@dataclass
class RepairResult:
    original: str
    repaired: str
    original_score: int
    repaired_score: int
    transformations: list[str] = field(default_factory=list)

    @property
    def changed(self) -> bool:
        return self.original != self.repaired

    @property
    def improvement(self) -> int:
        return self.original_score - self.repaired_score


def mojibake_score(text: str) -> int:
    """
    Estimate how likely text is to contain mojibake.

    Higher scores indicate more suspicious text. This is deliberately
    conservative: the script only accepts transformations that reduce it.
    """
    score = 0

    for fragment, weight in SUSPICIOUS_FRAGMENTS.items():
        if fragment in text:
            score += text.count(fragment) * weight

    for char in text:
        codepoint = ord(char)

        if codepoint in BAD_CONTROL_CODEPOINTS:
            score += 5

        # Surrogate code points should not occur in normal Unicode text.
        if 0xD800 <= codepoint <= 0xDFFF:
            score += 10

        # Private-use and non-characters mean a candidate encoding produced
        # valid but meaningless UTF-8 out of unrelated bytes.
        if any(low <= codepoint <= high for low, high in PRIVATE_USE_RANGES):
            score += 8

        if codepoint in NON_CHARACTER_CODEPOINTS:
            score += 10

    # Generic lead-byte/continuation-byte pairs catch scripts and symbols that
    # the literal tables do not enumerate.
    score += MOJIBAKE_PAIR_WEIGHT * len(MOJIBAKE_PAIR_PATTERN.findall(text))

    score += MIDWORD_CAPITAL_WEIGHT * count_midword_capitals(text)

    return score


def count_midword_capitals(text: str) -> int:
    """
    Count lowercase-then-uppercase transitions inside Greek or Cyrillic words.

    This is the fingerprint of UTF-8 read through a Cyrillic or Greek code
    page, which the literal fragment tables cannot cover: every damaged
    character is an ordinary letter of the target script.
    """
    count = 0
    previous_is_lower = False

    for char in text:
        codepoint = ord(char)
        bicameral = any(
            low <= codepoint <= high for low, high in BICAMERAL_SCRIPT_RANGES
        )

        if bicameral and previous_is_lower and char.isupper():
            count += 1

        previous_is_lower = bicameral and char.islower()

    return count


def top_signatures(text: str, limit: int = 4) -> str:
    """
    Summarise the strongest mojibake signatures found in text, for reporting.
    """
    found: list[tuple[int, str, int]] = []

    for fragment, weight in SUSPICIOUS_FRAGMENTS.items():
        count = text.count(fragment)

        if count:
            found.append((count * weight, fragment, count))

    found.sort(key=lambda item: (-item[0], item[1]))

    parts = [
        f"{fragment!a} x{count}"
        for _, fragment, count in found[:limit]
    ]

    pairs = len(MOJIBAKE_PAIR_PATTERN.findall(text))

    if pairs:
        parts.append(f"lead/continuation pairs x{pairs}")

    return ", ".join(parts)


@lru_cache(maxsize=None)
def _is_encodable(char: str, encoding: str) -> bool:
    try:
        char.encode(encoding)
    except UnicodeEncodeError:
        return False

    return True


def try_round_trip(text: str, source_encoding: str) -> str | None:
    """
    Reverse a common mojibake operation:

        original UTF-8 bytes
          -> incorrectly decoded as source_encoding
          -> stored as Unicode

    Repair:

        Unicode
          -> encode using source_encoding
          -> decode correctly as UTF-8
    """
    try:
        return text.encode(source_encoding).decode("utf-8")
    except (UnicodeEncodeError, UnicodeDecodeError):
        return None


def repair_run(run: str, source_encoding: str) -> str | None:
    """
    Repair a single damaged run, tolerating a truncated sequence at either end.

    A run can begin or end in the middle of a UTF-8 sequence when the byte that
    completed it was destroyed (typically replaced with U+FFFD by a decoder
    that had no mapping for it). Single-byte encodings keep a one-to-one
    character-to-byte mapping, so the undecodable edge characters can simply be
    left in place while the rest of the run is recovered.
    """
    candidate = try_round_trip(run, source_encoding)

    if candidate is not None:
        return candidate

    data = run.encode(source_encoding)

    # Try the smallest possible trim first: a UTF-8 sequence is at most four
    # bytes, so three characters at either end is enough.
    for total_trim in range(1, 7):
        for leading in range(min(total_trim, 3) + 1):
            trailing = total_trim - leading

            if trailing > 3 or leading + trailing >= len(data):
                continue

            try:
                middle = data[leading : len(data) - trailing].decode("utf-8")
            except UnicodeDecodeError:
                continue

            suffix = run[len(run) - trailing :] if trailing else ""

            return run[:leading] + middle + suffix

    return None


def try_segment_round_trip(text: str, source_encoding: str) -> str | None:
    """
    Repair damaged runs individually instead of requiring the whole file to
    round-trip.

    A single un-encodable character (for example correctly stored Devanagari
    or an emoji next to the damaged text) would otherwise abort the repair of
    an entire file. Each maximal run of non-ASCII characters that the source
    encoding can represent is converted on its own, and kept only when that
    run becomes less suspicious.
    """
    pieces: list[str] = []
    changed = False
    index = 0
    length = len(text)

    while index < length:
        char = text[index]

        if ord(char) < 0x80 or not _is_encodable(char, source_encoding):
            pieces.append(char)
            index += 1
            continue

        end = index

        while (
            end < length
            and ord(text[end]) >= 0x80
            and _is_encodable(text[end], source_encoding)
        ):
            end += 1

        run = text[index:end]
        candidate = repair_run(run, source_encoding)

        if (
            candidate is not None
            and candidate != run
            and mojibake_score(candidate) < mojibake_score(run)
        ):
            pieces.append(candidate)
            changed = True
        else:
            pieces.append(run)

        index = end

    if not changed:
        return None

    return "".join(pieces)


def try_lossy_repair(text: str) -> str | None:
    """
    Replace mojibake that cannot be reversed by re-encoding, because the
    original bytes were already replaced with U+FFFD, and drop stray
    byte-order marks.
    """
    repaired = text

    for damaged, replacement in LOSSY_SEQUENCES.items():
        if damaged in repaired:
            repaired = repaired.replace(damaged, replacement)

    # decode_utf8 removes a leading BOM, so any remaining one is an artefact of
    # concatenated or re-encoded files.
    if "﻿" in repaired:
        repaired = repaired.replace("﻿", "")

    if repaired == text:
        return None

    return repaired


def unescape_mojibake_entities(text: str) -> str:
    """
    Convert HTML entities that only occur in escaped mojibake back into the
    characters they represent, so a normal repair pass can reverse them.

    Numeric references are limited to the ranges a mis-decode can produce.
    """
    result = text

    for entity, char in MOJIBAKE_ENTITIES.items():
        if entity in result:
            result = result.replace(entity, char)

    def _replace_numeric(match: re.Match[str]) -> str:
        hex_digits, decimal_digits = match.groups()
        codepoint = int(hex_digits, 16) if hex_digits else int(decimal_digits)

        in_latin1_supplement = 0x80 <= codepoint <= 0xFF
        in_cp1252_specials = codepoint in {
            0x20AC, 0x201A, 0x0192, 0x201E, 0x2026, 0x2020, 0x2021,
            0x02C6, 0x2030, 0x0160, 0x2039, 0x0152, 0x017D, 0x2018,
            0x2019, 0x201C, 0x201D, 0x2022, 0x2013, 0x2014, 0x02DC,
            0x2122, 0x0161, 0x203A, 0x0153, 0x017E, 0x0178,
        }

        if in_latin1_supplement or in_cp1252_specials:
            return chr(codepoint)

        return match.group(0)

    return NUMERIC_ENTITY_PATTERN.sub(_replace_numeric, result)


def _run_passes(
    text: str,
    max_passes: int,
    allow_lossy: bool,
) -> tuple[str, int, list[str]]:
    """
    Repeatedly apply the best available repair while the text keeps improving.
    """
    current = text
    current_score = mojibake_score(current)
    transformations: list[str] = []

    for _ in range(max_passes):
        whole: list[tuple[int, str, str]] = []
        partial: list[tuple[int, str, str]] = []
        lossy: list[tuple[int, str, str]] = []

        for encoding in CANDIDATE_ENCODINGS:
            candidate = try_round_trip(current, encoding)

            if candidate is not None and candidate != current:
                whole.append(
                    (mojibake_score(candidate), candidate, f"{encoding} -> UTF-8")
                )

            candidate = try_segment_round_trip(current, encoding)

            if candidate is not None and candidate != current:
                partial.append(
                    (
                        mojibake_score(candidate),
                        candidate,
                        f"{encoding} -> UTF-8 (partial)",
                    )
                )

        if allow_lossy:
            candidate = try_lossy_repair(current)

            if candidate is not None and candidate != current:
                lossy.append(
                    (mojibake_score(candidate), candidate, "lossy sequence repair")
                )

        # Tiers are tried in order of confidence. Converting the whole text at
        # once must win over per-run conversion even when a run-level score
        # looks better: repairing one run of a multiply-encoded sequence in
        # isolation destroys the structure the next pass needs.
        selected: tuple[int, str, str] | None = None

        for tier in (whole, partial, lossy):
            if not tier:
                continue

            best = min(tier, key=lambda item: item[0])

            # Never apply a transformation unless the text becomes less
            # suspicious.
            if best[0] < current_score:
                selected = best
                break

        if selected is None:
            break

        current_score, current, label = selected
        transformations.append(label)

    return current, current_score, transformations


def repair_text(
    text: str,
    max_passes: int = 4,
    allow_lossy: bool = True,
    fix_entities: bool = False,
) -> RepairResult:
    """
    Apply conservative mojibake repair passes.

    Several source encodings are tested, both across the whole text and on
    individual damaged runs. A candidate is accepted only when it lowers the
    mojibake score.
    """
    original = text
    original_score = mojibake_score(original)

    repaired, repaired_score, transformations = _run_passes(
        original,
        max_passes,
        allow_lossy,
    )

    if fix_entities:
        unescaped = unescape_mojibake_entities(original)

        if unescaped != original:
            alternative, alternative_score, alternative_steps = _run_passes(
                unescaped,
                max_passes,
                allow_lossy,
            )

            # Unescaping alone increases the score, so it is only worth keeping
            # when the following passes more than pay for it.
            if alternative_score < repaired_score:
                repaired = alternative
                repaired_score = alternative_score
                transformations = ["html entities -> characters", *alternative_steps]

    return RepairResult(
        original=original,
        repaired=repaired,
        original_score=original_score,
        repaired_score=repaired_score,
        transformations=transformations,
    )


def looks_binary(data: bytes) -> bool:
    """
    Heuristic binary-file detection.

    NUL bytes are a strong indicator. The control-byte ratio catches many
    other binary formats while allowing tabs, line endings, and form feeds.
    """
    if not data:
        return False

    if b"\x00" in data:
        return True

    sample = data[:8192]
    allowed_controls = {9, 10, 12, 13}
    suspicious_controls = sum(
        1
        for byte in sample
        if byte < 32 and byte not in allowed_controls
    )

    return suspicious_controls / len(sample) > 0.05


def decode_utf8(data: bytes) -> tuple[str, bool] | None:
    """
    Decode UTF-8 while remembering whether the file contains a UTF-8 BOM.
    """
    has_bom = data.startswith(b"\xef\xbb\xbf")

    try:
        if has_bom:
            return data.decode("utf-8-sig"), True
        return data.decode("utf-8"), False
    except UnicodeDecodeError:
        return None


def encode_utf8(text: str, has_bom: bool) -> bytes:
    encoding = "utf-8-sig" if has_bom else "utf-8"
    return text.encode(encoding)


def unified_diff(path: Path, before: str, after: str) -> str:
    before_lines = before.splitlines(keepends=True)
    after_lines = after.splitlines(keepends=True)

    return "".join(
        difflib.unified_diff(
            before_lines,
            after_lines,
            fromfile=f"{path} (before)",
            tofile=f"{path} (after)",
            n=2,
        )
    )


def iter_files(
    root: Path,
    excluded_dirs: set[str],
):
    """
    Recursively yield files without following directory symlinks.
    """
    for current_root, dirs, files in os.walk(root, followlinks=False):
        current_path = Path(current_root)

        dirs[:] = [
            directory
            for directory in dirs
            if directory not in excluded_dirs
            and not (current_path / directory).is_symlink()
        ]

        for filename in files:
            path = current_path / filename

            if path.is_symlink():
                continue

            yield path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Recursively detect and repair common mojibake.",
    )

    parser.add_argument(
        "root",
        type=Path,
        help="Root directory of the repository.",
    )

    parser.add_argument(
        "--apply",
        action="store_true",
        help="Write repairs to disk. Without this flag, only preview changes.",
    )

    parser.add_argument(
        "--backup",
        action="store_true",
        help="Create a .bak copy before modifying each file.",
    )

    parser.add_argument(
        "--verbose",
        action="store_true",
        help="Print skipped files and detailed diagnostics.",
    )

    parser.add_argument(
        "--show-diff",
        action="store_true",
        help="Print a unified diff for every proposed repair.",
    )

    parser.add_argument(
        "--max-size-mb",
        type=float,
        default=10.0,
        help="Skip files larger than this size. Default: 10 MB.",
    )

    parser.add_argument(
        "--min-improvement",
        type=int,
        default=2,
        help=(
            "Minimum mojibake-score reduction needed to accept a repair. "
            "Default: 2."
        ),
    )

    parser.add_argument(
        "--max-passes",
        type=int,
        default=4,
        help=(
            "Maximum repair passes per file. Raise this for triple-encoded "
            "text. Default: 4."
        ),
    )

    parser.add_argument(
        "--exclude-dir",
        action="append",
        default=[],
        help="Additional directory name to exclude. May be repeated.",
    )

    parser.add_argument(
        "--include-lockfiles",
        action="store_true",
        help="Also inspect generated dependency lockfiles.",
    )

    parser.add_argument(
        "--include-self",
        action="store_true",
        help=(
            "Also inspect this script. Its signature tables contain mojibake "
            "on purpose, so it is skipped by default."
        ),
    )

    parser.add_argument(
        "--no-lossy",
        action="store_true",
        help=(
            "Do not repair sequences whose original bytes were destroyed "
            "(U+FFFD substitutions, stray byte-order marks)."
        ),
    )

    parser.add_argument(
        "--fix-entities",
        action="store_true",
        help=(
            "Also attempt to repair mojibake that was subsequently "
            "HTML-escaped. Off by default: intentional entities in HTML "
            "should not be rewritten."
        ),
    )

    return parser.parse_args()


def main() -> int:
    args = parse_args()
    root = args.root.expanduser().resolve()

    if not root.exists():
        print(f"Error: path does not exist: {root}", file=sys.stderr)
        return 2

    if not root.is_dir():
        print(f"Error: path is not a directory: {root}", file=sys.stderr)
        return 2

    if args.backup and not args.apply:
        print(
            "Note: --backup has no effect without --apply.",
            file=sys.stderr,
        )

    excluded_dirs = DEFAULT_EXCLUDED_DIRS | set(args.exclude_dir)
    max_size = int(args.max_size_mb * 1024 * 1024)

    try:
        self_path = Path(__file__).resolve()
    except (OSError, NameError):
        self_path = None

    scanned = 0
    changed = 0
    written = 0
    suspicious_unfixed = 0
    skipped_binary = 0
    skipped_non_utf8 = 0
    errors = 0

    mode = "APPLY" if args.apply else "DRY RUN"
    print(f"[{mode}] Scanning: {root}")

    for path in iter_files(root, excluded_dirs):
        scanned += 1

        try:
            if path.name in EXCLUDED_FILENAMES and not args.include_lockfiles:
                if args.verbose:
                    print(f"SKIP lockfile: {path}")
                continue

            if (
                self_path is not None
                and not args.include_self
                and path.resolve() == self_path
            ):
                if args.verbose:
                    print(f"SKIP self: {path}")
                continue

            if path.suffix.lower() in BINARY_EXTENSIONS:
                skipped_binary += 1
                if args.verbose:
                    print(f"SKIP binary extension: {path}")
                continue

            file_size = path.stat().st_size

            if file_size > max_size:
                if args.verbose:
                    print(
                        f"SKIP large file ({file_size} bytes): {path}"
                    )
                continue

            data = path.read_bytes()

            if looks_binary(data):
                skipped_binary += 1
                if args.verbose:
                    print(f"SKIP binary content: {path}")
                continue

            decoded = decode_utf8(data)

            if decoded is None:
                skipped_non_utf8 += 1
                if args.verbose:
                    print(f"SKIP non-UTF-8 text: {path}")
                continue

            text, has_bom = decoded
            result = repair_text(
                text,
                max_passes=args.max_passes,
                allow_lossy=not args.no_lossy,
                fix_entities=args.fix_entities,
            )

            if not result.changed:
                if result.original_score > 0:
                    suspicious_unfixed += 1
                    print(
                        f"REVIEW: {path} "
                        f"(suspicion score: {result.original_score}; "
                        f"{top_signatures(text)})"
                    )
                elif args.verbose:
                    print(f"OK: {path}")
                continue

            if result.improvement < args.min_improvement:
                suspicious_unfixed += 1
                print(
                    f"REVIEW: {path} "
                    f"(possible improvement only {result.improvement}; "
                    f"{top_signatures(text)})"
                )
                continue

            changed += 1
            relative_path = path.relative_to(root)

            # A non-zero score after a repair means bytes were destroyed before
            # the file reached disk and the rest could only be recovered around
            # them. Those files still deserve a look.
            residue = (
                f"; residual damage: {top_signatures(result.repaired, limit=2)}"
                if result.repaired_score
                else ""
            )

            print(
                f"{'FIXED' if args.apply else 'WOULD FIX'}: "
                f"{relative_path} "
                f"[score {result.original_score} -> "
                f"{result.repaired_score}; "
                f"{', '.join(result.transformations)}"
                f"{residue}]"
            )

            if args.show_diff:
                print(unified_diff(relative_path, text, result.repaired))

            if args.apply:
                if args.backup:
                    backup_path = path.with_name(path.name + ".bak")

                    if backup_path.exists():
                        raise FileExistsError(
                            f"Backup already exists: {backup_path}"
                        )

                    shutil.copy2(path, backup_path)

                repaired_bytes = encode_utf8(result.repaired, has_bom)

                # Write atomically where possible.
                temporary_path = path.with_name(path.name + ".mojibake-tmp")
                temporary_path.write_bytes(repaired_bytes)

                # Preserve original permissions.
                shutil.copymode(path, temporary_path)
                temporary_path.replace(path)
                written += 1

        except (OSError, UnicodeError) as exc:
            errors += 1
            print(f"ERROR: {path}: {exc}", file=sys.stderr)

    print()
    print("Summary")
    print(f"  Files scanned:             {scanned}")
    print(f"  Repairs identified:        {changed}")
    print(f"  Files written:             {written}")
    print(f"  Suspicious/manual review:  {suspicious_unfixed}")
    print(f"  Binary files skipped:      {skipped_binary}")
    print(f"  Non-UTF-8 files skipped:   {skipped_non_utf8}")
    print(f"  Errors:                    {errors}")

    if not args.apply and changed:
        print()
        print("No files were modified. Re-run with --apply after reviewing.")

    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
