# Prompt 2 --- Dependency Contract Generator (All Sections)

> Generate the public dependency contract for **every** architecture section,
> replacing any existing contract in place.

------------------------------------------------------------------------

# Execution Mode

This prompt runs **once** and processes **all ten sections**. There are no
placeholders to edit. Do not run it per section.

Run it **after** Prompt 1, so each contract is derived from a freshly generated
Canonical AI Context rather than a stale one.

------------------------------------------------------------------------

# Section Register

| Prefix | Source Directory | Canonical Context | Output Contract |
|---|---|---|---|
| A | `.../section-a/` | `AI_CONTEXT/SECTION_A/A_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_A/Dependency_Contract.md` |
| B | `.../section-b/` | `AI_CONTEXT/SECTION_B/B_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_B/Dependency_Contract.md` |
| C | `.../section-c/` | `AI_CONTEXT/SECTION_C/C_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_C/Dependency_Contract.md` |
| D | `.../section-d/` | `AI_CONTEXT/SECTION_D/D_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_D/Dependency_Contract.md` |
| E | `.../section-e/` | `AI_CONTEXT/SECTION_E/E_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_E/Dependency_Contract.md` |
| F | `.../section-f/` | `AI_CONTEXT/SECTION_F/F_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_F/Dependency_Contract.md` |
| G | `.../section-g/` | `AI_CONTEXT/SECTION_G/G_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_G/Dependency_Contract.md` |
| H | `.../section-h/` | `AI_CONTEXT/SECTION_H/H_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_H/Dependency_Contract.md` |
| I | `.../section-i/` | `AI_CONTEXT/SECTION_I/I_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_I/Dependency_Contract.md` |
| J | `.../section-j/` | `AI_CONTEXT/SECTION_J/J_Canonical_AI_Context.md` | `AI_CONTEXT/SECTION_J/Dependency_Contract.md` |

Source directories are under
`architecture/L2_Domain_Workflow_Architecture/`.

**Only Sections A and G currently have a Dependency Contract.** The other eight
must be created. Do not treat a missing file as a reason to skip a section.

------------------------------------------------------------------------

# Replacement Policy

Each contract is **fully replaced**, not merged or patched. Generate entirely
from the current Canonical AI Context and the current source documents. Never
carry a statement forward because a previous version contained it.

------------------------------------------------------------------------

# Prerequisites

1.  Execute Prompt 0, then Prompt 1.
2.  Read:
    -   `AI_CONTEXT/INDEX.md`
    -   The section's own Canonical AI Context from the register
    -   `DEPENDENCY_MAP.md`, in particular Section 7A dependency classes
3.  The source repository remains authoritative.

------------------------------------------------------------------------

# Objective

For each section in the register, generate its Dependency Contract at the path
listed.

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

Reference sections by prefix (A-J), and state for each whether the
dependency is foundational or runtime per `DEPENDENCY_MAP.md` Section 7A.

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

If validation fails for a section:

Stop that section. Report the failure. Leave any existing file untouched.
Continue to the next section - one failing section must not block the others.

------------------------------------------------------------------------

# Final Report

After all ten sections, report:

-   Contracts written, and sections skipped with the reason
-   Which contracts were newly created versus replaced
-   Any published/consumed event that appears in a contract but not in the
    corresponding Canonical AI Context, which indicates the two disagree
-   Any conflict encountered and how it was recorded
