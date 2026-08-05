# Prompt 0 --- AI Context Repository Bootstrap

> **Purpose:** Initialize and maintain the AI Context repository used by
> downstream AI agents.

------------------------------------------------------------------------

# Execution Mode

This prompt runs **once** and bootstraps the repository for **all ten sections**
(A-J). There are no placeholders to edit. Do not run it per section.

------------------------------------------------------------------------

# Objective

Bootstrap the AI context repository and generate its artifacts for every
architecture section.

These artifacts are **AI context**, not documentation.

Goals:

-   Minimize tokens while preserving architectural correctness.
-   Never infer.
-   Preserve source traceability.
-   Maintain deterministic structure.
-   Preserve source hierarchy.
-   Generate repeatable outputs.

------------------------------------------------------------------------

# Repository Bootstrap

1.  Check repository root.
2.  If `/AI_CONTEXT` does not exist, create:

``` text
AI_CONTEXT/
│
├── README.md
├── INDEX.md
├── AUDITS/
├── SECTION_A/
├── SECTION_B/
├── SECTION_C/
├── SECTION_D/
├── SECTION_E/
├── SECTION_F/
├── SECTION_G/
├── SECTION_H/
├── SECTION_I/
└── SECTION_J/
```

3.  Open `.gitignore`.

If `AI_CONTEXT/` is missing, append exactly once.

Never duplicate entries.

------------------------------------------------------------------------

# Existing Context

If AI_CONTEXT exists:

Read:

-   README.md
-   INDEX.md
-   Every existing `<PREFIX>_Canonical_AI_Context.md`
-   Every Dependency_Contract.md
-   Every Audit

Use these only for terminology consistency.

The original source documents remain authoritative.

------------------------------------------------------------------------

# Source Discovery

Process every markdown file inside each section's source directory:

`architecture/L2_Domain_Workflow_Architecture/section-a/` through `section-j/`

Sort files alphabetically.

Ignore:

-   AI_CONTEXT
-   Archives
-   Temporary files
-   Generated outputs

------------------------------------------------------------------------

# Non-Negotiable Rules

-   NEVER infer.
-   NEVER invent entities.
-   NEVER invent workflows.
-   NEVER invent events.
-   NEVER invent business rules.
-   NEVER silently resolve conflicts.
-   NEVER remove duplicated business rules.

If missing:

Write:

> Not specified in Source.

------------------------------------------------------------------------

# Traceability

Every extracted section must include:

Source File

Original Heading

------------------------------------------------------------------------

# Preserve Hierarchy

Maintain:

Section

A01

A02

A03

...

------------------------------------------------------------------------

# Normalize

Additionally build normalized indexes for:

-   Entities
-   Services
-   Events
-   Workflows
-   Rules
-   Integrations
-   Reports
-   Configurations
-   Permissions
-   Dependencies

------------------------------------------------------------------------

# Conflict Handling

If conflicting information exists:

DO NOT resolve.

Add to Audit.

Include:

-   File A
-   File B
-   Description

Continue generation.

------------------------------------------------------------------------

# Missing Information

Write exactly:

Not specified in Source.

------------------------------------------------------------------------

# Examples

Keep examples ONLY when they explain architecture.

Discard illustrative examples.

------------------------------------------------------------------------

# Cross References

Create references to dependent sections.

Never fabricate links.

------------------------------------------------------------------------

# Output Files

For every section prefix P in A, B, C, D, E, F, G, H, I, J generate:

AI_CONTEXT/SECTION\_P/P\_Canonical_AI_Context.md

AI_CONTEXT/SECTION\_P/Dependency_Contract.md

AI_CONTEXT/AUDITS/P\_Canonical_Audit.md

Source directory for each is
`architecture/L2_Domain_Workflow_Architecture/section-<lowercase P>/`.

Expected source counts: A 11, B 6, C 9, D 8, E 8, F 15, G 11, H 11, I 17, J 11 -
**107 files total.** A mismatch is a stop condition for that section.

------------------------------------------------------------------------

# Replacement Policy

Every generated file is **fully replaced**, not merged or patched. Read an
existing file only to produce a difference report afterwards, never to carry
content forward.

Prior contexts contain known ownership errors (INT-027, INT-029 through
INT-033) that were generated before the source documents were corrected.
Regenerating from current sources is what removes them.

**Do not regenerate or read** `AI_CONTEXT/SECTION_J/Canonical_AI_Context.md`;
it is a deprecated unprefixed alias retained for backward compatibility only.

------------------------------------------------------------------------

# Update INDEX.md

Rewrite `AI_CONTEXT/INDEX.md` from the filesystem after all sections complete.

Record, per section: prefix, directory, generated status, and the exact basenames
present. Record the generation timestamp. Never hand-edit it from memory, and
never describe a file as generated unless it was written in this run.

------------------------------------------------------------------------

# Quality Gates

Before finishing verify:

-   All source files processed
-   No hallucinated content
-   No inferred content
-   All statements traceable
-   Duplicate rules preserved
-   Conflicts reported
-   Missing information reported
-   Deterministic ordering maintained

If any gate fails for a section:

Stop that section. Report the failure. Leave its existing files untouched.
Continue to the next section - one failing section must not block the other nine.

------------------------------------------------------------------------

# Final Report

Report per section: files written, source count processed against the expected
count, and any gate failure. Then report which ownership statements changed from
the previous version across the whole run, since those are the INT-027 and
INT-029 through INT-033 corrections landing.

Do not report success for any section whose files were not written.

------------------------------------------------------------------------

# Handoff

After successful bootstrap, continue with Prompt 1 (Canonical AI Context
Generator).
