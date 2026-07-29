# SutraERP Team Assignments

## Purpose

This document defines architecture ownership and review responsibility for the SutraERP repository.

---

## Canonical Architecture Hierarchy

```text
L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6
```

| Level | Name | Definition |
|-------|------|------------|
| L0 | Business Vision | Defines why SutraERP exists, strategic intent, architectural context, and constraints. |
| L1 | Business Domain Architecture | Defines the canonical business domains, domain boundaries, and ownership. |
| L2 | Domain Workflow Architecture | Defines workflows, lifecycles, rules, and dependencies within an individual business domain. |
| L3 | Business Scenario Architecture | Defines independent end-to-end business scenarios that may derive information from one or many L2 workflow documents; L3 numbering is not dependent on L2 numbering. |
| L4 | System Behavior Specification | Defines expected system behavior for approved business scenarios without implementation design. |
| L5 | Solution Architecture | Defines engineering solution architecture needed to implement L4 behavior without changing business rules. |
| L6 | UI / UX Architecture | Defines user interaction, navigation, screens, and user journeys derived from L5 solution architecture without owning business logic. |

---

# Dhruv

## Role

**Lead Architect**

## Primary Sections

- Section A - Platform Foundation
- Section H - Institutional Services & Intelligence

## Repository-Wide Responsibilities

- Maintain repository-wide architectural consistency.
- Review architecture across all sections and all levels.
- Approve or request architectural changes.
- Resolve multi-domain architectural conflicts.
- Approve lock changes.
- Maintain architecture standards and naming consistency.

Dhruv remains Lead Architect with repository-wide architectural review responsibilities.

---

# Pallav

## Role

**Architect**

## Primary Sections

- Section B - Institution & Structure
- Section D - Academic Delivery, including D07 Internal Assessment

## Responsibilities

- Design and maintain assigned architecture.
- Ensure consistency within Sections B and D, including D07 Internal Assessment.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

---

# Varad

## Role

**Architect**

## Primary Sections

- Section C - Student Lifecycle
- Section E - People & Workforce

## Responsibilities

- Design and maintain assigned architecture.
- Ensure consistency within Sections C and E.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

---

# Chetan

## Role

**Architect**

## Primary Sections

- Section F - Finance
- Section J - Enterprise Resource Management

## Responsibilities

- Design and maintain Finance architecture.
- Design and maintain Enterprise Resource Management architecture, including vendor management, supplier management, procurement, inventory management, stock management, warehouse management, asset lifecycle management, and resource management.
- Ensure consistency within Sections F and J.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

Chetan remains the implementation and domain owner of Finance.

---

# Devang

## Role

**Architect**

## Primary Sections

- Section G - Campus Operations
- Section I - Examination

## Responsibilities

- Design and maintain assigned architecture.
- Ensure consistency within Sections G and I.
- Coordinate with dependent architects when architectural changes are required.
- Submit completed documents for architecture review.

---

# Finance Review Team

## Members

- Varad
- Devang

## Responsibility

The Finance Review Team provides review responsibility only. It does not transfer implementation ownership or domain ownership away from Chetan.

---

# Ownership Rules

- Every Level 2 document has exactly one primary owner.
- Only the assigned owner may modify their architecture documents unless a governance migration explicitly authorizes repository-wide synchronization.
- Multi-section architectural changes require discussion with the respective owners.
- Every completed document must be reviewed and approved by the Lead Architect before merging.
- Locked documents may only be modified after approval from the Lead Architect.

---

# Team Summary

| Architect | Primary Sections | Review Responsibilities |
|-----------|------------------|-------------------------|
| Dhruv | A, H | Repository-wide architectural review |
| Pallav | B, D | Assigned section review |
| Varad | C, E | Assigned section review; Finance Review Team |
| Chetan | F, J | Assigned section review; Finance implementation and domain owner |
| Devang | G, I | Assigned section review; Finance Review Team |

---

**Project:** SutraERP

**Document Status:** Active

**Maintained By:** Lead Architect
