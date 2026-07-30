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

Important:
    Commit or back up your repository before using --apply.
"""

from __future__ import annotations

import argparse
import difflib
import os
import shutil
import sys
from dataclasses import dataclass
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

# Characters and fragments commonly produced by UTF-8/Windows-1252 mojibake.
SUSPICIOUS_FRAGMENTS = {
    "Ã": 4,
    "Â": 4,
    "â€": 5,
    "â€™": 6,
    "â€œ": 6,
    "â€\x9d": 6,
    "â€“": 6,
    "â€”": 6,
    "â€¦": 6,
    "â€¢": 6,
    "â„¢": 6,
    "ðŸ": 7,
    "ð\x9f": 7,
    "ï»¿": 8,
    "ï¿½": 10,
    "�": 12,
    "¤": 2,
}

# Control characters that commonly appear when text was decoded incorrectly.
BAD_CONTROL_CODEPOINTS = set(range(0x80, 0xA0))


@dataclass
class RepairResult:
    original: str
    repaired: str
    original_score: int
    repaired_score: int
    transformations: list[str]

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
        score += text.count(fragment) * weight

    for char in text:
        codepoint = ord(char)

        if codepoint in BAD_CONTROL_CODEPOINTS:
            score += 5

        # Surrogate code points should not occur in normal Unicode text.
        if 0xD800 <= codepoint <= 0xDFFF:
            score += 10

    return score


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


def repair_text(text: str, max_passes: int = 3) -> RepairResult:
    """
    Apply conservative mojibake repair passes.

    Both Windows-1252 and Latin-1 are tested. A candidate is accepted only
    when it lowers the mojibake score.
    """
    original = text
    original_score = mojibake_score(original)
    current = original
    current_score = original_score
    transformations: list[str] = []

    for _ in range(max_passes):
        candidates: list[tuple[int, str, str]] = []

        for encoding in ("cp1252", "latin-1"):
            candidate = try_round_trip(current, encoding)

            if candidate is None or candidate == current:
                continue

            candidate_score = mojibake_score(candidate)
            candidates.append((candidate_score, candidate, encoding))

        if not candidates:
            break

        candidate_score, candidate, encoding = min(
            candidates,
            key=lambda item: item[0],
        )

        # Never apply a transformation unless the text becomes less suspicious.
        if candidate_score >= current_score:
            break

        current = candidate
        current_score = candidate_score
        transformations.append(f"{encoding} -> UTF-8")

    return RepairResult(
        original=original,
        repaired=current,
        original_score=original_score,
        repaired_score=current_score,
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
            result = repair_text(text)

            if not result.changed:
                if result.original_score > 0:
                    suspicious_unfixed += 1
                    print(
                        f"REVIEW: {path} "
                        f"(suspicion score: {result.original_score})"
                    )
                elif args.verbose:
                    print(f"OK: {path}")
                continue

            if result.improvement < args.min_improvement:
                suspicious_unfixed += 1
                print(
                    f"REVIEW: {path} "
                    f"(possible improvement only {result.improvement})"
                )
                continue

            changed += 1
            relative_path = path.relative_to(root)

            print(
                f"{'FIXED' if args.apply else 'WOULD FIX'}: "
                f"{relative_path} "
                f"[score {result.original_score} -> "
                f"{result.repaired_score}; "
                f"{', '.join(result.transformations)}]"
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