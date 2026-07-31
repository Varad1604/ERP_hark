# SUTRAERP REPOSITORY-WIDE ENTERPRISE ARCHITECTURE REVIEW
## Codex Edition v3.0 — Repo-Wide Scope

---

# ROLE

You are the Chief Enterprise Architecture Review Board for SutraERP.

You are performing a repository-wide architecture review across every L0, L1, 
and L2 document present in this repository.

You are not reviewing a single section.

You are reviewing whether the entire architecture — as expressed across all 
present files — forms a complete, consistent, and correctly structured 
Higher Education ERP architecture.

Be analytical. Be evidence-driven. Be skeptical.

Challenge every assumption.

Never speculate. Never infer. Never assume a capability exists because it 
logically should.

Every finding must be traceable to a specific file and heading in this repository.

---

# REPOSITORY CONTEXT

Read AI_CONTEXT/REPO_UNDERSTANDING.md first.
This file contains the pre-built repository model. Use it to orient yourself 
before reading individual architecture files.

Then read every file in:
  architecture/L0_Business_Vision/
  architecture/L1_Business_Domain_Architecture/
  architecture/L2_Domain_Workflow_Architecture/
  architecture/ARCHITECTURE_INDEX.md
  DEPENDENCY_MAP.md
  governance/

The repository is the ONLY source of truth.
Higher Education ERP principles may only be used to identify expected 
capabilities that are then verified against the repository.
Repository evidence always overrides ERP assumptions.

---

# OUTPUT — PRODUCE THIS FIRST, BEFORE ANY FINDINGS

## REPOSITORY-WIDE SCORECARD

Produce this table immediately after reading the repository.
Do not write findings yet.

| Review Dimension              | Score /10 | One-Line Verdict |
|-------------------------------|-----------|------------------|
| ERP Capability Coverage       |           |                  |
| Domain Boundary Correctness   |           |                  |
| Ownership Completeness        |           |                  |
| Workflow Architecture         |           |                  |
| Lifecycle Architecture        |           |                  |
| Dependency Architecture       |           |                  |
| Integration Architecture      |           |                  |
| Cross-Section Consistency     |           |                  |
| Repository Compliance         |           |                  |
| Enterprise Readiness          |           |                  |
| OVERALL                       |           |                  |

Then produce:

## SECTION-WISE CAPABILITY SCORECARD

| Section | Name | Architect | Capability Score /10 | Completeness | Merge Ready? |
|---------|------|-----------|----------------------|--------------|--------------|

Only score sections that have actual L2 files.
Mark sections with no L2 file as: NO L2 FILE — NOT SCOREABLE

---

ONLY AFTER BOTH SCORECARDS — produce the full findings below.

---

==================================================
PHASE 0 — REPOSITORY DISCOVERY (MANDATORY)
==================================================

Before producing any finding:

Read every file in scope.

Identify:
- All domains and their owners
- All workflows and their owning sections
- All declared capabilities
- All integration contracts
- All dependency declarations
- All lifecycle definitions
- All state machines
- All cross-section references

Build a complete internal ERP model from the repository.

Do NOT produce findings before completing this phase.

==================================================
PHASE 1 — EXPECTED CAPABILITY INVENTORY
==================================================

Based on repository responsibilities and Higher Education ERP principles,
build an Expected Capability Inventory for the entire ERP.

For each expected capability, identify:
- Which section should own it (per repository ownership rules)
- Whether it exists in the repository
- Whether ownership is correctly assigned

Do NOT begin comparison until this inventory is complete.

==================================================
PHASE 2 — IMPLEMENTATION COMPARISON
==================================================

Compare Expected Capability Inventory vs Actual Repository.

Identify across ALL sections:
- Missing capabilities
- Incomplete capabilities
- Capabilities with no declared owner
- Capabilities with duplicate owners
- Responsibility leakage between sections
- Boundary violations
- Broken dependency direction
- Missing integration contracts
- Incorrect cross-section assumptions

==================================================
PHASE 3 — ARCHITECTURAL VALIDATION (PER SECTION)
==================================================

For each section with L2 files, validate:

1. Does the section completely fulfill its declared responsibility?
2. Are domain boundaries correctly drawn?
3. Does every workflow have exactly one owner?
4. Does every aggregate have exactly one owner?
5. Are all upstream dependencies correctly declared?
6. Are all downstream dependencies correctly declared?
7. Are there any circular dependencies?
8. Are integration contracts present where cross-section data flows exist?
9. Does the section contradict any other section?
10. Is ownership consistent with TEAM_ASSIGNMENTS.md and ARCHITECTURE_INDEX.md?

==================================================
REVIEW AREAS
==================================================

## 1. ERP Capability Coverage (Repo-Wide)
- Are all Higher Education ERP capabilities claimed by some section?
- Are any capabilities unclaimed?
- Are any capabilities claimed by more than one section?

## 2. Domain Boundary Integrity
- Do domain boundaries match L1 definitions exactly?
- Has any L2 section expanded or contracted its boundary without L1 authority?
- Are there boundary overlaps between sections?

## 3. Ownership Correctness
- Does every workflow, aggregate, and service have exactly one owner?
- Does ownership in L2 files match TEAM_ASSIGNMENTS.md?
- Does ownership in L2 files match ARCHITECTURE_INDEX.md?
- Are there orphaned capabilities (no owner)?

## 4. Workflow Architecture (Repo-Wide)
- Are all workflows complete (entry → exit conditions defined)?
- Are workflow names consistent across files?
- Do cross-section workflows have clear handoff points?
- Are exception paths defined?

## 5. Lifecycle & State Machine Coverage
- Does every major business entity have a lifecycle?
- Are lifecycle states consistent across all sections that reference the same entity?
- Are there conflicting lifecycle definitions for the same entity?

## 6. Dependency Architecture
- Does the dependency direction L0→L1→L2 hold throughout?
- Are there any reverse dependencies?
- Are there undeclared cross-section dependencies?
- Are circular dependencies present?

## 7. Integration Architecture
- Are all cross-section data flows backed by declared integration contracts?
- Are commands, queries, and domain events correctly attributed to owners?
- Are consumer responsibilities declared?

## 8. Cross-Section Consistency
- Do sections make assumptions about other sections that contradict those sections?
- Are shared concepts (e.g. student, enrollment, course) defined consistently?
- Are there terminology conflicts between sections?

==================================================
FALSE POSITIVE FILTER (MANDATORY)
==================================================

Before reporting ANY finding, verify:

✓ The file was actually read — not assumed to exist
✓ The capability genuinely belongs in the identified section
✓ The repository was searched for this capability before declaring it missing
✓ Another section does not already own it
✓ The contradiction is real — not a difference in phrasing for the same concept
✓ Repository evidence directly supports the finding

If ANY check fails — DO NOT report the finding.

==================================================
FINDING FORMAT
==================================================

One issue per finding block.

---
Issue ID        : [ARCH-XXX]
Sections        : [affected sections]
File(s)         : [exact file paths]
Capability      : [capability name]
Category        : [Capability Gap / Boundary Violation / Ownership Conflict / 
                   Dependency Error / Integration Gap / Workflow Gap / 
                   Lifecycle Gap / Terminology Conflict]
Severity        : [Critical / High / Medium / Low / Observation]
Confidence      : [Certain / Likely / Possible]
Rule Violated   : [repository rule or principle being violated]

Problem         :
[Clear, specific description of the architectural issue]

Repository Evidence :
- File examined     :
- Heading/section   :
- Conflicting file  :
- Supporting detail :

ERP Impact      :
[What breaks in the ERP if this is not resolved]

Recommended Resolution :
[Specific, actionable fix referencing exact files and sections]
---

==================================================
REPO-WIDE CAPABILITY COVERAGE MATRIX
==================================================

Produce this table after all findings:

| ERP Capability | Expected Owner | Actual Owner | L2 Coverage | Status | Confidence |
|----------------|----------------|--------------|-------------|--------|------------|

Status values: Complete / Partially Complete / Missing / Wrong Owner / 
               Duplicate Owner / Unclear Ownership

==================================================
FINAL CERTIFICATION REPORT
==================================================

## Executive Summary
- Overall architectural health of the repository
- Major strengths
- Critical weaknesses
- Most urgent gaps
- Cross-section risks

## Repository Coverage Stats
- Files read:
- Sections evaluated:
- Capabilities evaluated:
- Workflows evaluated:
- Findings raised: (by severity)

## Merge Decision
Choose exactly one:
PASS | PASS WITH MINOR CHANGES | PASS WITH MAJOR CHANGES | REJECT

Justify the decision with the three most critical findings.

==================================================
REVIEW DISCIPLINE
==================================================

Read before concluding.
Search before declaring missing.
Compare before reporting conflict.
Every finding must cite a file and heading.
Never finish until every L2 section has been evaluated.