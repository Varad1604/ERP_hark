# SutraERP Level 3 — Business Scenario Architecture

**Architecture Level:** L3 — Business Scenario Architecture  
**Repository Area:** `architecture/L3_Business_Scenario_Architecture`  
**Status:** Phase 1 Final / Frozen; Phase 2 Final / Closed; Phase 3 Locked / Complete  
**Current Stage:** Canonical L3 Scenario Set Construction  
**Canonical Template:** `00_Governance/L3_Business_Scenario_Template_v2.0.md`  
**Canonical Naming Standard:** `00_Governance/L3_Naming_Numbering_Standard.md` v1.1

---

## 1. Purpose

This directory contains the Level 3 Business Scenario Architecture for SutraERP.

L3 describes complete institutional business scenarios. Each scenario represents one end-to-end institutional business outcome that may involve one or more L2 workflow documents while preserving the canonical ownership of every participating domain.

L3 depends on L2 and must not redefine L1/L2 ownership, states, events, business rules, lifecycles, aggregates, contracts, or dependencies.

---

## 2. Architecture Boundary

L3 is business architecture.

L3 may describe:

- business actors and roles;
- business responsibilities and authority;
- scenario boundaries, preconditions, triggers, outcomes, and postconditions;
- business flows, alternatives, and business recovery;
- upstream-owned rules, lifecycle impacts, events, and contracts;
- business approval, separation-of-duty, audit, and evidence obligations.

L3 must not define:

- APIs;
- database schemas;
- software classes or components;
- technical RBAC permissions or IAM design;
- UI screens or navigation;
- deployment or infrastructure configuration;
- technical error-handling mechanics;
- programming or technology choices.

Those belong to later architecture levels.

---

## 3. Architecture Hierarchy

```text
L0 — Business Vision
↓
L1 — Business Domain Architecture
↓
L2 — Domain Workflow Architecture
↓
L3 — Business Scenario Architecture
↓
L4 — System Behavior Specification
↓
L5 — Solution Architecture
↓
L6 — UI / UX Architecture
```

L3 numbering is independent of L2 numbering.

---

## 4. Completed Phase Baseline

### Phase 1 — Final / Frozen

- 1,096 active scenario candidates;
- candidate IDs normalized once on 2026-08-17 and then frozen;
- no later consolidation decision may renumber or delete the frozen baseline.

Phase 1 candidate format:

```text
CAN-L3-<AREA>-<NNN>
```

### Phase 2 — Final / Closed

Authoritative register:

`02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md`

Final closure state:

```text
Phase 2 register rows: 1,124
BLOCKED: 0
REVIEW-REQUIRED: 0
Resolved SPLIT-REQUIRED: 2
```

Phase 2 Working IDs (`P2W-...`) are traceability identifiers only and are never final L3 IDs.

The two resolved split decisions are instantiated only during canonical scenario-set construction.

### Phase 3 — Locked / Complete

Phase 3 established and reviewed the canonical L3 scenario template.

Governing artifacts:

- `00_Governance/L3_Business_Scenario_Template_v2.0.md` — **Locked canonical template**;
- `00_Governance/L3_Naming_Numbering_Standard.md` v1.1 — **Locked naming/numbering standard**;
- `00_Governance/L3_Phase3_Canonical_Template_Design_Decision_2026-08-18.md` — locked design decisions;
- `00_Governance/L3_Phase3_Traceability_Governance_Alignment_2026-08-18.md` — locked traceability placement rule;
- `03_Phase3_Canonical_Template/README.md` — Phase 3 closure status;
- `../../governance/policies/Architecture_Level_Validation_Standards.md` — L3 validation gate;
- `../../generated/reviews/L3_Phase3_Canonical_Template_Architecture_Review_2026-08-18.md` — review evidence.

`00_Governance/L3_Business_Scenario_Template_v1.0.md` is retained only as historical pilot evidence and must not govern new canonical L3 authoring.

---

## 5. Locked Canonical Scenario Structure

Every canonical L3 scenario uses:

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

## 6. Actors and RBAC Boundary

L3 identifies the business actors and roles that initiate, review, decide, approve, reject, perform, or receive business outcomes where those roles are supported by authoritative architecture.

These are business requirements for later access-control design.

L3 does not define technical RBAC roles, permission codes, IAM groups, claims, ACLs, or enforcement mechanisms.

Section A retains canonical ownership of Authorization and Audit infrastructure. Business domains retain ownership of their own business decisions, approval rules, evidence obligations, and outcomes.

---

## 7. Failure and Recovery Rule

Every canonical scenario includes a business-level Failure, Interruption and Business Recovery section.

It may describe governed business rejection, correction, reversal, suspension, resubmission, interruption, or terminal outcome only where supported upstream.

Technical retry, server, database, queue, transaction, exception, or infrastructure recovery belongs to later levels and is prohibited from L3.

---

## 8. Traceability Rule

Detailed Phase 1/Phase 2 consolidation lineage remains authoritative in the Phase 2 register.

Final canonical scenario documents do not need mandatory candidate-ID, P2W-ID, disposition, or watchlist-history columns.

Every canonical scenario still preserves architectural traceability through:

- `Depends On` metadata identifying authoritative higher-level architecture;
- selective L1/L2 source references where ownership, rule, state, event, contract, or disputed behaviour requires evidence.

---

## 9. Final L3 Numbering

Final scenario IDs use:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>
```

Final file names use:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md
```

The Primary Owner is the canonical owner of the final business outcome.

Final IDs must be assigned against the constructed canonical scenario set. They must not be created mechanically by copying Phase 1 candidate numbers or Phase 2 working IDs, and the 1,124 Phase 2 rows must not be treated as a guaranteed one-row-to-one-final-scenario list.

---

## 10. Current Repository Structure

```text
L3_Business_Scenario_Architecture/
│   README.md
│
├── 00_Governance/
│   ├── L3_Business_Scenario_Template_v1.0.md
│   ├── L3_Business_Scenario_Template_v2.0.md
│   ├── L3_Naming_Numbering_Standard.md
│   ├── L3_Phase3_Canonical_Template_Design_Decision_2026-08-18.md
│   ├── L3_Phase3_Traceability_Governance_Alignment_2026-08-18.md
│   ├── L3_Phase1_...
│   └── L3_Section_..._Phase2_Consolidation_Watchlist.md
│
├── 01_Phase1_Discovery/
│   └── five frozen team inventory files
│
├── 02_Phase2_Consolidation/
│   └── L3_Phase2_Proposed_Canonical_Scenario_Register.md
│
└── 03_Phase3_Canonical_Template/
    └── README.md
```

New scenario folders should be created only when canonical scenario-set construction establishes their repository structure.

---

## 11. Current Status

```text
Phase 1 candidate inventory: Final / Frozen — 1,096 active candidates
Phase 2 consolidation: Final / Closed — 1,124 traceability rows
Phase 2 unresolved blockers: 0
Phase 3 canonical template: Locked / Complete
Canonical template: v2.0
Naming/numbering standard: v1.1 Locked
Canonical L3 scenario set: Not yet constructed
Final L3 scenario documents: 0
Final L3 IDs assigned: 0
```

---

## 12. Next Authorized Activity

The next activity is **Canonical L3 Scenario Set Construction**.

That work must:

1. start from the Phase 2 final register and resolved watchlists;
2. construct the actual final canonical scenario set, including resolved split children and consolidation outcomes;
3. verify final ownership against locked L1/L2 architecture;
4. use the locked v2.0 canonical template;
5. assign final L3 IDs only after the canonical scenario set has been determined;
6. preserve implementation independence.

Do not generate one final scenario document per Phase 2 register row by default.