# SutraERP L3 Phase 3 — Canonical Scenario Template Design Decision

**Architecture Level:** L3 — Business Scenario Architecture Governance  
**Version:** 1.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Date:** 2026-08-18  
**Scope:** Canonical L3 scenario document structure  
**Affected Template:** `L3_Business_Scenario_Template_v2.0.md`

---

## 1. Purpose

This record captures the Phase 3 design decisions that govern the canonical L3 scenario template.

It does not create final L3 scenarios, assign final L3 IDs, or modify canonical L1/L2 ownership, states, events, lifecycles, rules, or dependencies.

---

## 2. Repository Baseline

Phase 3 begins from this established state:

- Phase 1 is Final / Frozen with 1,096 active candidates;
- Phase 2 is Final / Closed with 1,124 traceability rows;
- Phase 2 has 0 `BLOCKED` rows;
- Phase 2 has 0 `REVIEW-REQUIRED` rows;
- the two `SPLIT-REQUIRED` cases are resolved for later canonical scenario construction;
- final canonical L3 scenario documents and final L3 IDs have not yet been created;
- template v1.0 is retained as historical pilot evidence.

---

## 3. Locked Decisions

### D-01 — Business Actors Are Mandatory

Every canonical L3 scenario includes **Business Actors, Roles and Responsibilities**.

Actors and roles must be supported by authoritative business architecture. L3 may define business responsibilities and authority but must not define technical permission models.

### D-02 — Authorization, Approval and Audit Are Mandatory

Every scenario includes **Business Authorization, Approval and Audit Obligations** covering applicable business initiation authority, decision authority, approval, separation of duties, exception authority, and business evidence obligations.

Section A retains canonical ownership of Authorization and Audit infrastructure. Business domains retain ownership of their underlying business decisions, rules, evidence obligations, and outcomes.

### D-03 — Business Boundary Concepts Stay Distinct

The template separately represents:

- Preconditions and Entry Criteria;
- Trigger;
- Business Outcome;
- Completion Criteria;
- Postconditions.

### D-04 — Failure / Recovery Is Mandatory at Business Level

Every scenario contains **Failure, Interruption and Business Recovery**.

Only business-level failure, rejection, interruption, suspension, correction, reversal, resubmission, or terminal handling defined upstream may be represented. Technical error handling is prohibited.

Where upstream architecture defines no scenario-specific recovery path, the scenario says so explicitly rather than inventing one.

### D-05 — Alternate Flow and Failure / Recovery Are Different

An Alternate Business Flow is another valid governed path inside the same scenario boundary.

Failure / Interruption / Recovery applies when normal progression cannot continue and a governed resolution or terminal outcome is required.

### D-06 — Detailed Phase 1 / Phase 2 Lineage Is Not Duplicated

The Phase 2 canonical scenario register remains authoritative for Phase 1 IDs, Phase 2 working IDs, dispositions, and merge/split/variant/duplicate history.

Canonical scenarios do not carry mandatory duplicate consolidation-history tables or candidate-ID metadata.

### D-07 — Split Lineage Remains in Phase 2

Resolved split children inherit the existing Phase 2 lineage. No new Phase 1 candidate IDs are manufactured.

### D-08 — Ownership Representation Stays Simple

The template uses:

```text
Domain / Section -> Responsibility in Scenario
```

No mandatory `Explicit Non-Ownership` column is required. Responsibility wording must nevertheless preserve canonical ownership.

### D-09 — Source References Are Selective

Authoritative L2 references are required where materially necessary to establish ownership, rules, lifecycle states, events, contracts, or disputed/non-obvious behaviour.

A blanket source-reference column on every main-flow step is not required.

### D-10 — Business Rules Are Referenced, Not Re-Owned

L3 applies upstream-owned rules and must not create local rule identifiers that could imply new rule ownership.

### D-11 — Events and Synchronous Contracts Stay Distinct

The template groups both under **Business Events and Cross-Domain Collaboration** but uses separate subsections for:

1. Published / Consumed Business Events; and
2. Business Contracts and Cross-Domain Hand-offs.

Every event retains exactly one canonical publisher.

### D-12 — Related Scenario References Are Optional During Construction

Related-scenario references are used only when final canonical L3 IDs exist. Placeholder final IDs must not be invented.

### D-13 — L3 Supplies Business Inputs for Future RBAC

L3 may state who initiates, reviews, decides, approves, rejects, authorizes exceptions, and what separation-of-duty or audit obligations apply.

Technical RBAC structures remain a later-level implementation concern.

### D-14 — Higher-Level Traceability Uses Lightweight Metadata

Repository constitutional traceability remains mandatory even though Phase 1/Phase 2 consolidation lineage is not duplicated.

Every canonical L3 scenario therefore carries `Depends On` metadata for the authoritative L1/L2 architecture materially governing the scenario, plus selective source references where evidence is needed.

This does not reintroduce candidate/P2W/disposition columns.

---

## 4. Locked Canonical Template Structure

```text
1. Scenario Metadata
2. Scenario Purpose
3. Business Outcome
4. Scenario Boundary and Scope
5. Ownership and Participating Domains
6. Business Actors, Roles and Responsibilities
7. Preconditions and Entry Criteria
8. Trigger
9. Main Business Flow
10. Alternate Business Flows
11. Failure, Interruption and Business Recovery
12. Business Rules and Decision Constraints
13. State and Lifecycle Impact
14. Business Events and Cross-Domain Collaboration
15. Business Authorization, Approval and Audit Obligations
16. Completion Criteria and Postconditions
17. Open Issues, Assumptions and Deferred Questions
```

---

## 5. Governance Result

These decisions justify the major structural change from pilot template v1.0 to canonical template v2.0.

`L3_Business_Scenario_Template_v2.0.md` is the governing template for new canonical L3 scenario authoring.

Template v1.0 remains historical pilot evidence and must not govern new canonical scenario documents.

---

## 6. Integrity Statement

These decisions do not:

- modify Phase 1 candidate rows or numbering;
- modify Phase 2 dispositions;
- create final L3 IDs;
- change canonical ownership;
- invent events, states, rules, lifecycles, contracts, or dependencies;
- authorize implementation architecture inside L3.

Any future structural change requires explicit review, versioning, repository synchronization, and Lead Architect approval.