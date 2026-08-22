# Qualification Eligibility Determination

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-036  
**Canonical Title:** Qualification Eligibility Determination  
**Primary Owner:** Academic Progression  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P0  
**Derivation:** KEEP  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-22  

## 1. Scenario Metadata

This document preserves the frozen Stage 3 identity and semantics for **L3-C-036 — Qualification Eligibility Determination**. No ownership, participating-domain, type or priority reclassification is introduced.

## 2. Scenario Purpose

Determine whether a Student has satisfied the academic progression requirements necessary to enter the Qualification Completion & Student Exit boundary.

## 3. Business Outcome

The Student is authoritatively determined to be **Qualification Eligible** or **Qualification Pending**. When eligible, Academic Progression publishes `QualificationEligible`, after which L2-C05 owns qualification completion processing.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins when the Student's academic progression position is ready for qualification-eligibility evaluation.

### 4.2 End Boundary
The scenario ends when qualification eligibility is established or remains pending; the scenario does not perform qualification completion or degree conferment.

### 4.3 In Scope
- Evaluate programme credits.
- Verify mandatory courses and curriculum completion.
- Verify institutional completion requirements.
- Verify applicable exit/NEP requirements.
- Establish Qualification Eligible or Qualification Pending.
- Publish `QualificationEligible` when eligible.

### 4.4 Out of Scope
- Qualification completion execution and degree conferment.
- Student Exit completion.
- Alumni transition.
- Modification of curriculum or examination outcomes.
- Technical implementation.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Predecessor | C027 — Student Academic Progression Decision |
| Related | C028 — Academic Credit Ledger Update |
| Downstream | C039 — Qualification Completion and Degree Conferment |

## 5. Ownership and Participating Domains

| Domain | Canonical Responsibility |
|---|---|
| Academic Progression (C) | Owns the qualification eligibility determination business decision and outcome within the L2-C04 boundary. |
| Qualification Completion & Student Exit (C05) | Owns downstream qualification completion after `QualificationEligible`; it does not decide academic progression eligibility. |

Cross-domain participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Progression | Determines qualification eligibility from authoritative academic information. | Canonical eligibility decision owner. |
| Qualification Completion & Student Exit | Consumes eligibility and performs its own downstream qualification process. | Downstream owner. |

No actor, role or authority not supported by the upstream architecture is invented.

## 7. Preconditions and Entry Criteria

- Student has an authoritative Academic Progression record.
- Applicable curriculum requirements are available.
- Relevant academic credits and progression outcomes are authoritative.
- Mandatory academic requirements can be evaluated.

## 8. Trigger

Qualification eligibility evaluation is initiated after the Student's applicable progression and completion data is available.

## 9. Main Business Flow

| Step | Business Behaviour | Owner |
|---|---|---|
| 1 | Gather authoritative programme credit and academic-completion information. | Academic Progression |
| 2 | Verify mandatory curriculum requirements. | Academic Progression |
| 3 | Verify minimum programme credits. | Academic Progression |
| 4 | Verify institution-specific completion requirements. | Academic Progression |
| 5 | Verify applicable exit/NEP requirements. | Academic Progression |
| 6 | Establish Qualification Eligible or Qualification Pending. | Academic Progression |
| 7 | If eligible, publish `QualificationEligible` to L2-C05. | Academic Progression |

The flow stops when the institutional business outcome defined in Section 3 is reached. Downstream consequences remain owned by their canonical domains.

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| Mandatory requirement incomplete | Retain eligibility as pending. | Qualification Pending |
| Minimum programme credits not satisfied | Retain eligibility as pending. | Qualification Pending |
| Institution-specific completion requirement incomplete | Retain eligibility as pending. | Qualification Pending |
| Applicable exit/NEP requirement incomplete | Retain eligibility as pending. | Qualification Pending |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Academic completion data is incomplete | Do not establish eligibility until authoritative data is complete. |
| A mandatory requirement remains unsatisfied | Preserve the pending position and identify the outstanding requirement. |
| Eligibility is established | Publish `QualificationEligible`; downstream C05 proceeds under its own ownership. |

No technical retry, queue, rollback, infrastructure failure or implementation exception model is introduced.

## 12. Business Rules and Decision Constraints

| Rule | Effect |
|---|---|
| **BR-QLF-001** Qualification Eligibility only occurs after successful progression evaluation. | Prevents premature qualification eligibility. |
| **BR-QLF-002** All mandatory curriculum requirements must be completed. | Mandatory completion is required. |
| **BR-QLF-003** Minimum programme credits must be satisfied. | Credit threshold is required. |
| **BR-QLF-004** Institution-specific completion requirements must be fulfilled. | Institutional completion obligations apply. |
| **BR-QLF-005** Qualification Eligibility transfers ownership to L2-C05. | Establishes the downstream boundary. |

Rules remain owned by L2-C04 or their upstream canonical owners; this L3 scenario does not create local rules.

## 13. State and Lifecycle Impact

| Concept / Aggregate | State Impact | Owner |
|---|---|---|
| Academic Progression | `Progressed` → `Qualification Eligible` when all requirements are satisfied. | Academic Progression |
| Qualification Completion | Receives eligibility after `QualificationEligible`. | L2-C05 |

No new lifecycle state or transition is introduced by this scenario.

## 14. Business Events and Cross-Domain Collaboration

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `QualificationEligible` | Academic Progression | Communicates that the Student is ready for qualification processing. |

Events remain distinct from synchronous business hand-offs. Canonical publishers are preserved exactly as defined upstream.

## 15. Business Authorization, Approval and Audit Obligations

Qualification Eligibility is an Academic Progression decision. The decision basis and evidence must remain auditable. Once eligibility is established, L2-C05 owns the subsequent qualification-completion process.

No technical IAM/RBAC model is introduced.

## 16. Completion Criteria and Postconditions

- All applicable academic progression requirements evaluated.
- Mandatory curriculum requirements satisfied.
- Minimum programme credits satisfied.
- Institution-specific completion requirements satisfied.
- `QualificationEligible` published when eligible.
- No qualification completion is performed here.

## 17. Open Issues, Assumptions and Deferred Questions

**OI-C036-01:** The exact institution-specific completion checklist is not enumerated in the available L2-C04 source; this scenario therefore does not invent one.

**OI-C036-02:** The exact downstream C05 contract detail is intentionally left to L4/L5 and is not invented here.

**Assumption:** L2-C05 is the canonical downstream owner after `QualificationEligible`. The L2-C04 source explicitly states this hand-off.

All unresolved upstream matters remain explicit rather than being silently resolved.

## Production Verification Record

- Frozen canonical identity expressed in the required `L3-C-036` format.
- Primary Owner, Participating Domains, Scenario Type and Priority preserved.
- L2-C04 ownership and dependency boundaries applied.
- Canonical business events and publishers preserved.
- No invented actors, rules, states, events or authority.
- No implementation-level mechanics.
- Business recovery only.
- Seventeen mandatory L3 sections present.
- Material source gaps are recorded in Section 17.
- Lifecycle status remains **Draft** because repository governance requires newly authored L3 scenarios to begin as Draft; this is a production-quality authoring artifact, not an approval/lock claim.
