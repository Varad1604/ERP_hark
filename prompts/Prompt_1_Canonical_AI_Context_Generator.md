# Prompt 1 --- Canonical AI Context Generator

> Generate the canonical AI context for a single architecture section
> from its source documents.

------------------------------------------------------------------------

# Placeholders (Edit Before Running)

``` text
SECTION_NAME=H
SECTION_PREFIX=H
SOURCE_DIRECTORY={{SOURCE_DIRECTORY}}
OUTPUT_DIRECTORY={{OUTPUT_DIRECTORY}}
```

------------------------------------------------------------------------

# Prerequisites

1.  Execute Prompt 0 first.
2.  Ensure `AI_CONTEXT/` exists.
3.  Read:
    -   `AI_CONTEXT/README.md`
    -   `AI_CONTEXT/INDEX.md`
    -   Existing Canonical AI Context files
    -   Existing Dependency Contracts
4.  Use them only for terminology consistency.
5.  Source documents are always authoritative.

------------------------------------------------------------------------

# Objective

Create:

`AI_CONTEXT/SECTION_{{SECTION_PREFIX}}/{{SECTION_PREFIX}}_Canonical_AI_Context.md`

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

-   Process every Markdown file in `{{SOURCE_DIRECTORY}}`
-   Sort alphabetically.
-   Process every file completely.
-   Never skip sections silently.
-   Ignore generated artifacts and `AI_CONTEXT`.

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

------------------------------------------------------------------------

# Examples

Retain examples only if they explain architecture or workflow behavior.

Discard illustrative/business narrative examples.

------------------------------------------------------------------------

# Cross References

For every dependency:

-   Reference dependent section
-   Reference originating file

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

------------------------------------------------------------------------

# Validation

Before writing file verify:

-   Every source file processed
-   No hallucinations
-   No inferred content
-   All extracted statements traceable
-   Deterministic ordering preserved
-   Duplicate business rules preserved
-   Missing information marked

If validation fails:

Stop.

Report issues.

Do not generate partial output.
