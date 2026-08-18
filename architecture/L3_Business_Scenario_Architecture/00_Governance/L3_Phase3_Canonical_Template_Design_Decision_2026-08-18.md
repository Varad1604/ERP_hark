# SutraERP L3 Phase 3 — Canonical Scenario Template Design Decision

**Document:** L3 Phase 3 Canonical Template Design Decision  
**Architecture Level:** L3 — Business Scenario Architecture Governance  
**Date:** 2026-08-18  
**Status:** Approved Design Decisions — Template Review Pending  
**Scope:** Final canonical L3 scenario document structure  
**Affected Template:** `L3_Business_Scenario_Template_v2.0.md`

---

## 1. Purpose

This record captures the explicit Phase 3 design decisions approved for the final canonical L3 business scenario template.

It exists to prevent template changes from silently altering repository architecture or leaving design rationale only in conversation history.

This decision does not create final L3 scenarios, assign final L3 scenario IDs, or modify canonical L1/L2 ownership, states, events, lifecycles, business rules, or dependencies.

---

## 2. Repository Baseline

The Phase 3 design begins from the repository state in which:

- Phase 1 is Final / Frozen;
- the Phase 1 discovery baseline contains 1,096 active candidates;
- Phase 2 is Final / Closed;
- the Phase 2 consolidation register contains 1,124 traceability rows;
- Phase 2 has 0 `BLOCKED` rows;
- Phase 2 has 0 `REVIEW-REQUIRED` rows;
- the two `SPLIT-REQUIRED` cases are resolved for later canonical scenario construction;
- final canonical L3 scenario documents and final L3 IDs have not yet been created;
- `L3_Business_Scenario_Template_v1.0.md` is the historical pilot template.

---

## 3. Approved Template Decisions

### D-01 — Business Actors Are Mandatory

Canonical L3 scenarios must include a dedicated **Business Actors, Roles and Responsibilities** section.

Purpose:

- identify institutional actors and business roles participating in the scenario;
- state their business responsibilities and authority where supported;
- provide authoritative business input for later access-control and RBAC design.

Constraint:

L3 must not define technical RBAC roles, permission codes, IAM groups, claims, access-control lists, or enforcement mechanisms.

---

### D-02 — Authorization, Approval and Audit Are Mandatory

Canonical L3 scenarios must include a dedicated **Business Authorization, Approval and Audit Obligations** section.

The section may capture, when supported by authoritative architecture:

- initiation authority;
- decision and approval authority;
- separation-of-duty requirements;
- override or exception authority;
- business audit and evidence obligations.

Generic technical audit infrastructure remains outside scenario ownership and must not be duplicated in every scenario.

---

### D-03 — Preconditions, Trigger, Outcome and Postconditions Must Remain Distinct

The template must separately represent:

- Preconditions and Entry Criteria;
- Trigger;
- Business Outcome;
- Completion Criteria;
- Postconditions.

These concepts must not be collapsed into one field because they represent different business semantics and different scenario boundaries.

---

### D-04 — Failure / Recovery Is Mandatory at Business Level

Every canonical L3 scenario must contain a **Failure, Interruption and Business Recovery** section.

The section covers business-level situations in which normal scenario progression cannot continue, including governed rejection, suspension, interruption, correction, reversal, resubmission, or terminal outcomes when defined upstream.

If authoritative L2 architecture defines no scenario-specific failure or recovery behaviour, the scenario must say so explicitly rather than inventing one.

Technical failure handling such as server errors, database rollback, queue retries, exception handling, retry counts, or infrastructure recovery is prohibited from L3.

---

### D-05 — Alternate Flow and Failure / Recovery Are Different Concepts

An **Alternate Business Flow** is a valid governed path that differs from the normal successful flow while remaining within the scenario boundary.

A **Failure, Interruption or Recovery Flow** exists when normal progression cannot continue and a governed business resolution or terminal outcome is required.

The template must preserve this distinction.

---

### D-06 — Detailed Phase 1 / Phase 2 Lineage Is Not Duplicated in Every Scenario

The Phase 2 canonical scenario register remains the authoritative source for Phase 1 candidate lineage, Phase 2 working IDs, dispositions, merge/split/variant relationships, and consolidation history.

Canonical L3 scenario documents must not carry mandatory columns or tables that duplicate that register.

This decision supersedes only the earlier template-level expectation that every final L3 scenario document must repeat its original Phase 1 candidate ID in metadata.

It does **not** remove or alter Phase 1/Phase 2 traceability from the repository.

---

### D-07 — Split Scenario Lineage Remains in Consolidation History

Where one Phase 1 candidate results in more than one final canonical L3 scenario, the resulting scenarios inherit the existing consolidation lineage.

No new Phase 1 candidate IDs are manufactured for split children.

The Phase 2 consolidation register remains the authoritative mapping source.

---

### D-08 — Ownership Representation Must Stay Simple

The scenario template must identify:

```text
Domain / Section → Responsibility in Scenario
```

A separate mandatory `Explicit Non-Ownership` column is not required.

However, all responsibility descriptions must preserve canonical ownership and must not imply that participation transfers ownership.

---

### D-09 — Source References Are Selective, Not Blanket Step-Level Traceability

L3 must remain evidence-based without becoming citation-heavy.

Authoritative L2 references should be included where materially necessary to establish or verify:

- ownership;
- business rules;
- lifecycle states;
- event names or publishers;
- cross-domain contracts;
- disputed or non-obvious business behaviour.

A mandatory source-reference column on every main-flow step is not required.

---

### D-10 — Business Rules Are Referenced, Not Re-Owned by L3

L3 scenarios apply business rules owned upstream.

The final template must not create local `BR-01`, `BR-02`, or similar identifiers that could imply that L3 owns newly authored business rules.

Use existing authoritative identifiers where available and record the owning domain and source.

---

### D-11 — Events and Synchronous Contracts Must Remain Distinct

The template may place both under one **Business Events and Cross-Domain Collaboration** section, but it must maintain separate subsections for:

1. Published / Consumed Business Events; and
2. Business Contracts and Cross-Domain Hand-offs.

This preserves the architectural distinction between event publication and synchronous business collaboration.

Every event retains exactly one canonical publisher.

---

### D-12 — Related Scenario References Are Optional During Initial Construction

A small Related Scenarios subsection may record predecessor, successor, or materially related canonical L3 scenarios.

It must not invent final L3 IDs before those IDs are actually assigned.

---

### D-13 — L3 Supplies Business Inputs for Later RBAC; It Does Not Design RBAC

L3 may specify:

- who initiates;
- who reviews;
- who decides;
- who approves or rejects;
- who may authorize a business exception;
- separation-of-duty obligations;
- business evidence and audit obligations.

L4/L5 may later translate these business requirements into implementation access-control structures.

Technical permission models remain outside L3.

---

## 4. Resulting Template Structure

The approved design results in the following canonical scenario body:

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

## 5. Phase 3 Governance Impact

These decisions justify a major structural template revision from v1.0 to v2.0.

`L3_Business_Scenario_Template_v1.0.md` must remain preserved as historical pilot governance evidence.

`L3_Business_Scenario_Template_v2.0.md` is the proposed final Phase 3 template and must undergo architecture review before its lock status is changed to `Locked`.

Until that review is complete, creation of large-scale canonical scenario documents should not treat v2.0 as a locked governance baseline.

---

## 6. Repository Integrity Statement

This Phase 3 design decision:

- does not modify Phase 1 candidate rows or numbering;
- does not modify Phase 2 dispositions or traceability;
- does not create final L3 IDs;
- does not change canonical ownership;
- does not invent business events, states, rules, lifecycles, or dependencies;
- does not authorize implementation architecture inside L3.

Any future change to these decisions must be explicit, versioned, and repository-reviewed.