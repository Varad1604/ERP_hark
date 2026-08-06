# L2-[Xnn] — [Canonical Name from ARCHITECTURE_INDEX.md]

**Status:** Draft
**Architecture Level:** L2 — Domain Workflow Architecture
**Section:** [X] — [Section name, verbatim from ARCHITECTURE_INDEX.md]
**Document Owner:** [Architect from TEAM_ASSIGNMENTS.md]

---

## Template Governance

Architectural Level: L2 — Domain Workflow Architecture

Depends On: L1 — Business Domain Architecture

Expected Output: domain-owned workflow, lifecycle, rule, event, entity, and dependency architecture.

Downstream Reference: L3 Business Scenario Architecture.

**Required structure.** `governance/constitution/Volume_3_Architecture_Framework.md`
section 18 requires every L2 document to define the eleven sections below. The
document may organise its body however it needs — large L2 documents in this
repository use numbered `# N.` sections — but every required section must be
locatable. Documents that do not use these headings verbatim must carry a
**Canonical Section Crosswalk** table mapping each required section to where it
actually lives.

**Status.** Exactly one status, drawn from the thirteen-value canonical vocabulary
in `governance/constitution/Volume_2_Repository_Standards.md` section 7. Declare it
once, in the header above. Do not restate a different status in a footer.

**Events.** Canonical event names are globally unique with one semantic definition
(`L2-A06` lines 780-804). Qualify names with the owning aggregate or domain before
introducing them.

---

## Canonical Section Crosswalk

| Required section (Volume 3 section 18) | Location in this document |
|---|---|
| Purpose | § |
| Scope | § |
| Actors | § |
| Entities | § |
| Business Rules | § |
| Workflow | § |
| State Machine | § |
| Contracts | § |
| Events | § |
| Dependencies | § |
| Locked Decisions | § |

---

## 1. Purpose

Why this domain exists and what it is responsible for, without redefining L1 ownership.

## 2. Scope

What this document covers, and explicitly what it does not. Name the owning
document for anything excluded.

## 3. Actors

Human roles and system actors that participate in these workflows.

## 4. Entities

Canonical entities owned by this domain. Entities owned elsewhere are referenced
by ID only, with the owning document named.

## 5. Business Rules

Business rules, policies, and invariants this domain enforces.

## 6. Workflow

The workflows this domain executes: trigger, preconditions, steps, outcomes,
and failure handling.

## 7. State Machine

States and permitted transitions for entities owned here. State names and
transitions must match the owning L1 definition where one exists.

## 8. Contracts

Synchronous contracts this domain exposes and consumes.

## 9. Events

Events published by this domain, and events consumed from others. Separate the
two explicitly; do not merge them under one heading.

## 10. Dependencies

**Depends On** — upstream capabilities this domain requires to function.

**Referenced By** — downstream consumers of this domain's output. These are not
dependencies; listing them as such creates false cycles.

## 11. Locked Decisions

Architectural decisions frozen for this domain, each with rationale and date.
