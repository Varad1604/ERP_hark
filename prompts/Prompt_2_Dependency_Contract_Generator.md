# Prompt 2 --- Dependency Contract Generator

> Generate a compact, deterministic dependency contract for a single
> architecture section. This artifact is optimized for downstream AI
> agents and cross-section integration.

------------------------------------------------------------------------

# Placeholders (Edit Before Running)

``` text
SECTION_NAME={{SECTION_NAME}}
SECTION_PREFIX={{SECTION_PREFIX}}
SOURCE_DIRECTORY={{SOURCE_DIRECTORY}}
OUTPUT_DIRECTORY={{OUTPUT_DIRECTORY}}
DEPENDENT_SECTIONS={{DEPENDENT_SECTIONS}}
```

------------------------------------------------------------------------

# Prerequisites

1.  Execute Prompt 0.
2.  Execute Prompt 1.
3.  Read:
    -   AI_CONTEXT/INDEX.md
    -   AI_CONTEXT/SECTION\_\*/Canonical_AI_Context.md (existing)
    -   AI_CONTEXT/SECTION\_{{SECTION_PREFIX}}/Canonical_AI_Context.md
4.  The source repository remains authoritative.

------------------------------------------------------------------------

# Objective

Generate:

`AI_CONTEXT/SECTION_{{SECTION_PREFIX}}/Dependency_Contract.md`

This file is **not** documentation. It is the public contract other AI
agents use when reasoning about this section.

Optimize for: - Minimal tokens - Maximum correctness - Deterministic
structure - Zero inference

------------------------------------------------------------------------

# Mandatory Rules

-   NEVER infer.
-   NEVER invent APIs, events, entities, workflows, or rules.
-   NEVER silently resolve conflicts.
-   If information is absent, write: `Not specified in Source.`

------------------------------------------------------------------------

# Extract Only Public Contracts

Extract only externally consumable information.

## 1. Section Identity

-   Name
-   Prefix
-   Purpose

## 2. Domain Ownership

-   Owned entities
-   Explicit ownership boundaries

## 3. Public Business Capabilities

## 4. Published Events

For each event include: - Event name - Producer - Description - Source
reference

## 5. Consumed Events

For each event include: - Event name - Consumer - Purpose - Source
reference

## 6. Public Read Models

Include: - Read model - Description - Consumers

## 7. Integration Points

For each: - Internal / External - Direction - Purpose - Source reference

## 8. Public APIs

If documented.

Otherwise: `Not specified in Source.`

## 9. Configuration Exposed

Only configuration affecting downstream consumers.

## 10. Permissions Affecting Integrations

## 11. Reports Exposed

## 12. Dependencies

List:

-   Depends On
-   Consumed By

Reference sections using: {{DEPENDENT_SECTIONS}}

## 13. Architectural Constraints

Only explicit constraints.

## 14. Prohibited Assumptions

List anything downstream sections must NOT assume.

------------------------------------------------------------------------

# Cross References

Every extracted item must reference: - Source file - Original heading

------------------------------------------------------------------------

# Output Order

1.  Overview
2.  Dependency Manifest
3.  Owned Entities
4.  Public Capabilities
5.  Published Events
6.  Consumed Events
7.  Read Models
8.  Integration Points
9.  Public APIs
10. Configuration
11. Permissions
12. Reports
13. Dependencies
14. Architectural Constraints
15. Prohibited Assumptions
16. Source Mapping
17. Missing Information

This order is mandatory.

------------------------------------------------------------------------

# Validation

Before writing:

Verify:

-   All public contracts traceable
-   No inferred information
-   No hallucinated contracts
-   Deterministic ordering maintained
-   Source references present
-   Missing information explicitly marked

If validation fails:

Stop generation.

Report failures.

Do not emit partial output.
