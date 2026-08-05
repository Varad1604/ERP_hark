# Prompt 1 --- Canonical AI Context Generator (All Sections)

> Generate the canonical AI context for **every** architecture section from its
> source documents, replacing any existing context in place.

------------------------------------------------------------------------

# Execution Mode

This prompt runs **once** and processes **all ten sections**. There are no
placeholders to edit. Do not run it per section.

------------------------------------------------------------------------

# Section Register

Process these in order. Counts are the expected number of `L2-*.md` source files
as of 2026-08-05; a mismatch is a stop condition, not something to work around.

| Prefix | Source Directory | Output File | Expected Sources |
|---|---|---|---:|
| A | `architecture/L2_Domain_Workflow_Architecture/section-a/` | `AI_CONTEXT/SECTION_A/A_Canonical_AI_Context.md` | 11 |
| B | `architecture/L2_Domain_Workflow_Architecture/section-b/` | `AI_CONTEXT/SECTION_B/B_Canonical_AI_Context.md` | 6 |
| C | `architecture/L2_Domain_Workflow_Architecture/section-c/` | `AI_CONTEXT/SECTION_C/C_Canonical_AI_Context.md` | 9 |
| D | `architecture/L2_Domain_Workflow_Architecture/section-d/` | `AI_CONTEXT/SECTION_D/D_Canonical_AI_Context.md` | 8 |
| E | `architecture/L2_Domain_Workflow_Architecture/section-e/` | `AI_CONTEXT/SECTION_E/E_Canonical_AI_Context.md` | 8 |
| F | `architecture/L2_Domain_Workflow_Architecture/section-f/` | `AI_CONTEXT/SECTION_F/F_Canonical_AI_Context.md` | 15 |
| G | `architecture/L2_Domain_Workflow_Architecture/section-g/` | `AI_CONTEXT/SECTION_G/G_Canonical_AI_Context.md` | 11 |
| H | `architecture/L2_Domain_Workflow_Architecture/section-h/` | `AI_CONTEXT/SECTION_H/H_Canonical_AI_Context.md` | 11 |
| I | `architecture/L2_Domain_Workflow_Architecture/section-i/` | `AI_CONTEXT/SECTION_I/I_Canonical_AI_Context.md` | 17 |
| J | `architecture/L2_Domain_Workflow_Architecture/section-j/` | `AI_CONTEXT/SECTION_J/J_Canonical_AI_Context.md` | 11 |

**Total: 107 source files across 10 sections.**

------------------------------------------------------------------------

# Replacement Policy

Each output file is **fully replaced**, not merged, appended to, or patched.

1.  Read the existing file first, if present.
2.  Generate the new content entirely from current source documents.
3.  Overwrite the file.
4.  Never carry a statement forward from the previous version because it was
    there before. If the current sources do not support it, it does not survive.

**This matters.** The previous contexts contain known ownership errors recorded
as INT-027 and INT-029 through INT-033 --- for example the Section B context
claimed Campus owns Building, Wing, Floor, Room and Lab, which `L2-G10` owns.
Those files were generated before the source corrections. Regenerating from
current sources is what removes the errors; preserving prior content
reintroduces them.

Do not read the previous version for anything except a post-generation
difference report.

------------------------------------------------------------------------

# Deprecated Aliases

`AI_CONTEXT/SECTION_J/Canonical_AI_Context.md` is a deprecated unprefixed alias.
Leave it untouched, do not regenerate it, and do not read it as a source. Section
J's canonical output is the prefixed basename in the register above.

------------------------------------------------------------------------

# Prerequisites

1.  Execute Prompt 0 first.
2.  Ensure `AI_CONTEXT/` exists.
3.  Read once, before processing any section:
    -   `AI_CONTEXT/README.md`
    -   `AI_CONTEXT/INDEX.md`
    -   `architecture/ARCHITECTURE_INDEX.md` --- canonical section and document names
    -   `architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md` --- canonical owners
    -   `DEPENDENCY_MAP.md` --- foundational vs runtime dependency classes
4.  Use them only for terminology and ownership consistency.
5.  **Source documents are always authoritative.**

------------------------------------------------------------------------

# Objective

For each section in the register, create the output file listed.

The file is optimized for AI agents, not humans.

Goals:

-   Preserve correctness
-   Minimize tokens
-   Never infer
-   Preserve traceability
-   Preserve source hierarchy
-   Maintain deterministic ordering

------------------------------------------------------------------------

# Source Processing Rules

-   Process every Markdown file in the section's source directory.
-   Sort alphabetically.
-   Process every file completely.
-   Never skip sections silently.
-   Ignore generated artifacts and `AI_CONTEXT`.
-   Ignore anything under `archive/`.

------------------------------------------------------------------------

# Ownership Extraction Rules

Ownership errors are the defect class this corpus is most prone to. When a source
document states that another document owns a concept, that statement is binding
and must appear in the context.

-   Record the owner exactly as the source names it.
-   Where a source says it does **not** own something, record that disclaimer.
-   Never list an entity under a section's owned entities when any source assigns
    it elsewhere.
-   Where two sources conflict, apply the Conflict Handling rules; do not choose
    silently.

------------------------------------------------------------------------

# Extraction Rules

Extract only information explicitly present.

Never infer.

Extract:

-   Scope
-   Responsibilities
-   Domain ownership
-   Canonical entities
-   Services
-   Workflows
-   State machines
-   Business rules
-   Configuration
-   Events
-   Read models
-   Integrations
-   Reports
-   Permissions
-   APIs
-   Multi-domain dependencies
-   Architectural constraints
-   Glossary

If absent:

`Not specified in Source.`

------------------------------------------------------------------------

# Preserve Source Hierarchy

For each source file:

-   File name
-   Original heading
-   Extracted content
-   Source reference

Never merge source sections.

------------------------------------------------------------------------

# Normalized AI Index

After hierarchy, generate normalized indexes:

1.  Domain Overview
2.  Owned Entities
3.  Business Capabilities
4.  Services
5.  Workflows
6.  State Machines
7.  Events Published
8.  Events Consumed
9.  Read Models
10. Integrations
11. Configuration
12. Permissions
13. Reports
14. Dependencies
15. Constraints
16. Glossary

Each item must reference originating file(s).

Events Published and Events Consumed are separate indexes. Never merge them: a
consumed event is not evidence of ownership.

------------------------------------------------------------------------

# Examples

Retain examples only if they explain architecture or workflow behavior.

Discard illustrative/business narrative examples.

------------------------------------------------------------------------

# Cross References

For every dependency:

-   Reference dependent section
-   Reference originating file
-   State whether it is foundational or runtime, per `DEPENDENCY_MAP.md` Section 7A

Never fabricate links.

------------------------------------------------------------------------

# Token Optimization

-   Remove repetitive prose.
-   Preserve business meaning.
-   Preserve canonical terminology.
-   Prefer concise bullet lists over paragraphs.
-   Do not remove business rules.

------------------------------------------------------------------------

# Output Order

1.  AI Executive Summary
2.  Dependency Manifest
3.  Source Hierarchy
4.  Normalized Index
5.  Source Mapping
6.  Missing Information
7.  Cross References
8.  Architectural Constraints
9.  Glossary

This order is mandatory.

Every file must open with a generation header recording the generation date, the
section prefix, the source directory, and the exact count of source files
processed.

------------------------------------------------------------------------

# Per-Section Validation

Before writing each file verify:

-   Source file count matches the register
-   Every source file processed
-   No hallucinations
-   No inferred content
-   All extracted statements traceable
-   Deterministic ordering preserved
-   Duplicate business rules preserved
-   Missing information marked
-   No entity listed as owned where a source assigns it to another section
-   Published and consumed events separated

If validation fails for a section:

Stop that section. Report the issue. Leave the existing file untouched. Continue
to the next section --- one failing section must not block the other nine.

------------------------------------------------------------------------

# Final Report

After all ten sections, report:

-   Sections regenerated, and sections skipped with the reason
-   Source file count processed per section, against the register
-   For each section, ownership statements that **changed** from the previous
    version, since those are the INT-027/029-033 corrections landing
-   Any conflict encountered and how it was recorded
-   Any source file that produced no extractable content

Do not report success for a section whose file was not written.
