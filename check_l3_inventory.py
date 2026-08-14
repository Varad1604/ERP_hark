#!/usr/bin/env python3
"""
SutraERP L3 Phase 1 inventory conformance checker.

Validates candidate inventory files against the locked Phase 1 rules in:

  architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Phase1_Discovery_Rules.md
  architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Scenario_Type_and_Priority_Rules.md

Run before accepting any Phase 1 inventory submission.

    python check_l3_inventory.py                    # check all Phase 1 inventories
    python check_l3_inventory.py path/to/file.md    # check specific files
    python check_l3_inventory.py --quiet            # errors and warnings only
    python check_l3_inventory.py --full             # list every affected candidate

Exit code 0 when no errors are found, 1 otherwise. Warnings do not fail the run.
"""

from __future__ import annotations

import argparse
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent
DEFAULT_DIR = (
    REPO_ROOT
    / "architecture"
    / "L3_Business_Scenario_Architecture"
    / "01_Phase1_Discovery"
)

# Discovery Rules section 6 - Phase 1 candidate ID format.
CANDIDATE_ID = re.compile(r"^CAN-L3-([A-JX])-(\d{3,4})$")

# A table row that presents itself as a candidate. Anything matching this is
# held to the full locked format, which is what stops non-conforming rows
# (for example expanded variant IDs) from hiding inside an inventory.
CANDIDATE_ROW = re.compile(r"^\|\s*CAN-L3-[A-Za-z0-9]+-[0-9]+.*\|")

FIELDS = (
    "Candidate ID",
    "Scenario Title",
    "Trigger",
    "Final Outcome",
    "Primary Owner",
    "Domains",
    "Type",
    "Priority",
)

# Type and Priority Rules sections 3 and 8.
VALID_TYPES = {"Simple", "Cross-domain", "Critical"}
VALID_PRIORITIES = {"P0", "P1", "P2"}

# Discovery Rules section 11 - section letters or governed VAR:<role> tokens.
SECTION_LETTERS = set("ABCDEFGHIJ")
VAR_TOKEN = re.compile(r"^VAR:[A-Z][A-Za-z -]+$")
SUBDOMAIN_TOKEN = re.compile(r"^[A-J]\d{2}$")

# Discovery Rules section 8 - action fragments that are not standalone candidates.
FRAGMENT_VERBS = {
    "validate", "generate", "send", "check", "update", "save", "approve",
    "verify", "calculate", "notify", "display", "view", "browse", "search",
    "configure", "record", "submit", "assign",
}

# Discovery Rules section 9 - titles that are too broad.
BROAD_PREFIX = re.compile(r"^(Complete|Manage|Handle|Process|Do)\b", re.IGNORECASE)

# Tell-tale suffixes of matrix-generated path enumerations rather than
# discovered business outcomes.
GENERATED_SUFFIXES = (
    "Normal Governed Path",
    "Alternative Governed Path",
    "Blocked or Unsuccessful Governed Outcome",
    "Governed Recovery",
    "Governance-Controlled Path",
    "Historical Integrity Path",
    "Authoritative Cross-Domain Context",
)

# Distinct-outcome ratio thresholds. A healthy inventory sits at or near 100%.
# The 954-row Section D catalogue withdrawn on 2026-08-14 sat at 48%.
RATIO_ERROR = 0.85
RATIO_WARN = 0.97

# Below this row count the statistical checks are not meaningful.
MIN_ROWS_FOR_STATS = 50

# How many example candidate IDs to show per grouped finding.
SAMPLE_SIZE = 3

ERROR = "ERROR"
WARN = "WARN"


class Report:
    """Collects findings grouped by (level, file, kind) so that one systemic
    problem reports as one line rather than hundreds."""

    def __init__(self) -> None:
        self._groups: dict[tuple[str, str, str], list[str]] = defaultdict(list)
        self._order: list[tuple[str, str, str]] = []
        self.notes: list[str] = []

    def add(self, level: str, source: str, kind: str, subject: str = "") -> None:
        key = (level, source, kind)
        if key not in self._groups:
            self._order.append(key)
        self._groups[key].append(subject)

    def note(self, msg: str) -> None:
        self.notes.append(msg)

    def groups(self, level: str) -> list[tuple[str, str, list[str]]]:
        return [
            (key[1], key[2], self._groups[key])
            for key in self._order
            if key[0] == level
        ]

    def count(self, level: str) -> int:
        return sum(len(v) for k, v in self._groups.items() if k[0] == level)


class Row:
    __slots__ = ("line_no", "cells")

    def __init__(self, line_no: int, cells: list[str]) -> None:
        self.line_no = line_no
        self.cells = cells

    def get(self, name: str) -> str:
        return self.cells[FIELDS.index(name)]

    @property
    def ref(self) -> str:
        return f"{self.get('Candidate ID')} (line {self.line_no})"


def split_row(raw: str) -> list[str]:
    """Split a markdown table row into its cells."""
    stripped = raw.strip()
    if stripped.startswith("|"):
        stripped = stripped[1:]
    if stripped.endswith("|"):
        stripped = stripped[:-1]
    return [cell.strip() for cell in stripped.split("|")]


def parse(path: Path, report: Report) -> list[Row]:
    rows: list[Row] = []
    text = path.read_text(encoding="utf-8", errors="replace")

    for line_no, raw in enumerate(text.splitlines(), start=1):
        if not CANDIDATE_ROW.match(raw):
            continue
        cells = split_row(raw)
        if len(cells) != len(FIELDS):
            label = cells[0] if cells else "?"
            report.add(
                ERROR,
                path.name,
                f"row has the wrong number of fields; the locked format is "
                f"{len(FIELDS)} fields ({', '.join(FIELDS)}) "
                f"[Discovery Rules section 4]",
                f"{label} (line {line_no}, {len(cells)} fields)",
            )
            continue
        rows.append(Row(line_no, cells))

    return rows


def check_row(row: Row, source: str, report: Report) -> None:
    if not CANDIDATE_ID.match(row.get("Candidate ID")):
        report.add(
            ERROR,
            source,
            "candidate ID does not match CAN-L3-<AREA>-<NNN> "
            "[Discovery Rules section 6]",
            row.ref,
        )

    for field in ("Scenario Title", "Trigger", "Final Outcome", "Primary Owner", "Domains"):
        if not row.get(field):
            report.add(ERROR, source, f"{field} is empty", row.ref)

    ctype = row.get("Type")
    if ctype not in VALID_TYPES:
        report.add(
            ERROR,
            source,
            f"Type must be one of {', '.join(sorted(VALID_TYPES))} "
            f"[Type and Priority Rules section 3]",
            f"{row.ref} has '{ctype}'",
        )

    priority = row.get("Priority")
    if priority not in VALID_PRIORITIES:
        report.add(
            ERROR,
            source,
            f"Priority must be one of {', '.join(sorted(VALID_PRIORITIES))} "
            f"[Type and Priority Rules section 8]",
            f"{row.ref} has '{priority}'",
        )

    for token in (t.strip() for t in row.get("Domains").split(",")):
        if not token or token in SECTION_LETTERS or VAR_TOKEN.match(token):
            continue
        if SUBDOMAIN_TOKEN.match(token):
            # Deviation rather than corruption. Subdomain codes are more precise
            # than section letters but are not what section 11 specifies, and
            # mixing conventions across inventories breaks Phase 2 grouping.
            report.add(
                WARN,
                source,
                "Domains uses subdomain codes rather than section letters; "
                "confirm the intended convention "
                "[Discovery Rules section 11]",
                f"{row.ref} has '{token}'",
            )
        else:
            report.add(
                ERROR,
                source,
                "Domains token is neither a section letter nor a VAR:<role> token "
                "[Discovery Rules section 11]",
                f"{row.ref} has '{token}'",
            )

    title = row.get("Scenario Title")
    first_word = title.split(" ", 1)[0].strip().lower()
    if first_word in FRAGMENT_VERBS:
        report.add(
            WARN,
            source,
            "title begins with an action verb and reads as a workflow step "
            "rather than a business outcome [Discovery Rules section 8]",
            row.ref,
        )
    if BROAD_PREFIX.match(title):
        report.add(
            WARN,
            source,
            "title may be too broad and hide several independent outcomes "
            "[Discovery Rules section 9]",
            row.ref,
        )
    if any(title.endswith(suffix) for suffix in GENERATED_SUFFIXES):
        report.add(
            WARN,
            source,
            "title ends with a generated path-enumeration suffix rather than "
            "naming a discovered outcome",
            row.ref,
        )


def check_file(path: Path, report: Report) -> list[Row]:
    source = path.name
    rows = parse(path, report)
    if not rows:
        report.add(WARN, source, "no candidate rows found", source)
        return rows

    for row in rows:
        check_row(row, source, report)

    for cid, count in Counter(r.get("Candidate ID") for r in rows).items():
        if count > 1:
            report.add(ERROR, source, "duplicate candidate ID", f"{cid} x{count}")

    for title, count in Counter(r.get("Scenario Title") for r in rows).items():
        if count > 1:
            report.add(WARN, source, "duplicate scenario title", f"'{title}' x{count}")

    # Duplicate final outcomes. This is the check that catches generated
    # inventories: distinct titles hiding identical business results.
    outcomes = [r.get("Final Outcome") for r in rows]
    by_outcome: dict[str, list[str]] = defaultdict(list)
    for row in rows:
        by_outcome[row.get("Final Outcome")].append(row.get("Candidate ID"))
    for text, ids in by_outcome.items():
        if len(ids) > 1:
            preview = text if len(text) <= 70 else text[:67] + "..."
            report.add(
                WARN,
                source,
                "candidates share an identical Final Outcome",
                f'{", ".join(ids)} -- "{preview}"',
            )

    distinct = len(set(outcomes))
    ratio = distinct / len(rows)
    summary = f"{source}: {len(rows)} candidates, {distinct} distinct outcomes ({ratio:.0%})"
    if ratio < RATIO_ERROR:
        report.add(
            ERROR,
            source,
            f"distinct-outcome ratio is below {RATIO_ERROR:.0%}, which indicates a "
            f"generated rather than discovered inventory; consolidate to one row "
            f"per business outcome",
            f"{distinct} distinct outcomes across {len(rows)} rows ({ratio:.0%})",
        )
    elif ratio < RATIO_WARN:
        report.add(
            WARN,
            source,
            f"distinct-outcome ratio is below {RATIO_WARN:.0%}; review for near-duplicates",
            f"{distinct} distinct outcomes across {len(rows)} rows ({ratio:.0%})",
        )
    else:
        report.note(summary)

    if len(rows) >= MIN_ROWS_FOR_STATS:
        check_statistics(source, rows, report)

    return rows


def check_statistics(source: str, rows: list[Row], report: Report) -> None:
    """Detect Type and Priority assigned by formula rather than by judgement."""

    # Type and Priority Rules section 6: "Critical is based on institutional
    # risk, not the number of participating domains." If Type is perfectly
    # predicted by domain count, it was derived, not decided.
    by_breadth: dict[bool, set[str]] = defaultdict(set)
    for row in rows:
        by_breadth["," in row.get("Domains")].add(row.get("Type"))

    if len(by_breadth) == 2 and all(len(v) == 1 for v in by_breadth.values()):
        single = next(iter(by_breadth[False]))
        multi = next(iter(by_breadth[True]))
        if single != multi:
            report.add(
                WARN,
                source,
                "Type appears derived from the number of participating domains "
                "rather than from institutional risk "
                "[Type and Priority Rules sections 6-7]",
                f"every single-domain row is '{single}', every multi-domain row is '{multi}'",
            )

    # A near-uniform Type or Priority column means it was not actually assessed.
    for field, rules in (("Type", "section 7"), ("Priority", "section 12")):
        counts = Counter(r.get(field) for r in rows)
        value, n = counts.most_common(1)[0]
        share = n / len(rows)
        if share > 0.90 and len(counts) > 1:
            report.add(
                WARN,
                source,
                f"{field} is near-uniform; confirm it was assessed per candidate "
                f"[Type and Priority Rules {rules}]",
                f"{share:.0%} of {len(rows)} rows are '{value}'",
            )

    # Primary Owner convention should be consistent within a file so that
    # Phase 2 consolidation can group by owner.
    owners = {r.get("Primary Owner") for r in rows}
    coded = {o for o in owners if re.match(r"^[A-J]\d{2}\b", o)}
    if coded and len(coded) != len(owners):
        freeform = sorted(owners - coded)
        report.add(
            WARN,
            source,
            "Primary Owner mixes subdomain codes with free text; pick one convention",
            f"{len(freeform)} free-text owners, e.g. {', '.join(freeform[:SAMPLE_SIZE])}",
        )


def emit(report: Report, level: str, full: bool) -> None:
    groups = report.groups(level)
    if not groups:
        return

    total = report.count(level)
    print(f"{level}S ({total})" if level == ERROR else f"{level}INGS ({total})")
    for source, kind, subjects in groups:
        n = len(subjects)
        scope = f"{n} candidates" if n > 1 else "1 candidate"
        print(f"  [{source}] {kind}")
        shown = subjects if full else subjects[:SAMPLE_SIZE]
        for subject in shown:
            print(f"      {subject}")
        if not full and n > SAMPLE_SIZE:
            print(f"      ... and {n - SAMPLE_SIZE} more ({scope} total)")
    print()


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Validate SutraERP L3 Phase 1 candidate inventories."
    )
    parser.add_argument(
        "paths",
        nargs="*",
        type=Path,
        help="Inventory files to check. Defaults to all Phase 1 inventories.",
    )
    parser.add_argument("--quiet", action="store_true", help="Suppress the summary block.")
    parser.add_argument(
        "--full",
        action="store_true",
        help="List every affected candidate instead of a sample.",
    )
    args = parser.parse_args()

    files = sorted(args.paths) if args.paths else sorted(
        DEFAULT_DIR.glob("L3_Phase1_Inventory_*.md")
    )

    if not files:
        print("No inventory files found.", file=sys.stderr)
        return 1

    missing = [p for p in files if not p.is_file()]
    for path in missing:
        print(f"No such file: {path}", file=sys.stderr)
    if missing:
        return 1

    report = Report()
    all_ids: dict[str, set[str]] = defaultdict(set)
    total = 0

    for path in files:
        rows = check_file(path, report)
        total += len(rows)
        for row in rows:
            all_ids[row.get("Candidate ID")].add(path.name)

    # Candidate IDs must be unique across the whole programme, not just per file.
    for cid, sources in sorted(all_ids.items()):
        if len(sources) > 1:
            report.add(
                ERROR,
                "cross-inventory",
                "candidate ID is used in more than one inventory",
                f"{cid} in {', '.join(sorted(sources))}",
            )

    if not args.quiet and report.notes:
        print("Summary")
        for note in report.notes:
            print(f"  {note}")
        print()

    emit(report, WARN, args.full)
    emit(report, ERROR, args.full)

    n_err = report.count(ERROR)
    n_warn = report.count(WARN)
    verdict = "PASS" if n_err == 0 else "FAIL"
    print(
        f"{verdict} -- checked {len(files)} file(s), {total} candidates: "
        f"{n_err} error(s), {n_warn} warning(s)."
    )
    return 1 if n_err else 0


if __name__ == "__main__":
    sys.exit(main())
