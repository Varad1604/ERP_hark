# Prompt 0 --- AI Context Repository Bootstrap

> **Purpose:** Initialize and maintain the AI Context repository used by
> downstream AI agents.

------------------------------------------------------------------------

# Placeholders (Edit Before Running)

<!-- ``` text
SECTION_NAME={{SECTION_NAME}}
SECTION_PREFIX={{SECTION_PREFIX}}
SOURCE_DIRECTORY={{SOURCE_DIRECTORY}}
OUTPUT_DIRECTORY={{OUTPUT_DIRECTORY}}
DEPENDENT_SECTIONS={{DEPENDENT_SECTIONS}}
``` -->

------------------------------------------------------------------------

# Objective

Generate AI context artifacts for one architecture section.

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
└── SECTION_I/
```

The bootstrap structure must also include:

```text
AI_CONTEXT/SECTION_J/
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
-   Every Canonical_AI_Context.md
-   Every Dependency_Contract.md
-   Every Audit

Use these only for terminology consistency.

The original source documents remain authoritative.

------------------------------------------------------------------------

# Source Discovery

Process every markdown file inside:

{{SOURCE_DIRECTORY}}

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

Generate:

AI_CONTEXT/SECTION\_{{SECTION_PREFIX}}/Canonical_AI_Context.md

AI_CONTEXT/SECTION\_{{SECTION_PREFIX}}/Dependency_Contract.md

AI_CONTEXT/AUDITS/{{SECTION_PREFIX}}\_Canonical_Audit.md

------------------------------------------------------------------------

# Update INDEX.md

Maintain generated status.

Update timestamp.

Update available sections.

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

If any gate fails:

Stop.

Report failures.

Do not silently continue.

------------------------------------------------------------------------

# Handoff

After successful bootstrap, continue with Prompt 1 (Canonical AI Context
Generator).
