# Examination Policy Definition to Institutional Activation

# 1. Scenario Metadata

**Scenario ID:** `L3-I-001`  
**Architecture Level:** L3 – Business Scenario Architecture  
**Primary Owner:** Examination Policy & Configuration (I01)  
**Participating Domains:** I (Examination), B (Governance), A (Platform)  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Devang  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-I01_Examination_Policy_and_Configuration_Workflows.md` §8.1, §11.1; `DEPENDENCY_MAP.md`  
**Referenced By:** `L3-I-002`; `L3-I-055`; Future L4 System Behavior Specifications  

---

# 2. Scenario Purpose

Governs the formulation, statutory Academic Council review, grading scale/passing threshold configuration, and institutional activation of an Examination Policy.

---

# 3. Business Outcome

Authoritative Examination Policy published in `POLICY_ACTIVE_LOCKED` state, binding all subsequent exam sessions to approved evaluation and grading parameters.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Academic Council or Board of Examinations directive to establish or overhaul examination governance rules.

## 4.2 End Boundary

Policy activated, version locked, institutional notification published to faculty, students, and departments.

## 4.3 In Scope

- Comprehensive execution of examination policy definition to institutional activation under statutory institutional academic regulations.
- Governance review, multi-tier committee approvals, and version-controlled policy publishing.
- Cross-domain notification and binding to subsequent examination cycles.

## 4.4 Out of Scope

- Day-to-day invigilation duty rosters or room allocation logistics.
- External accreditation agency compliance submissions.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Core Standard | `L3-I-001` | Establishes master examination governance baseline |
| Governs | `L3-I-055` | Theory examinations operate under active policy rules |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Examination Policy & Configuration (I01) | Primary Owning Domain — formulates policy parameters, manages approval workflow, and publishes active regulations. |
| Institution & Structure (Section B) | Academic Council ratification, governance resolutions, and statutory authority decrees. |
| Platform Foundation (Section A) | Policy versioning storage, cryptographic approval signing, and institutional broadcast. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Academic Council / Board of Examinations | Statutory academic body reviewing and ratifying policy provisions | Approves / Decrees |
| Controller of Examinations (CoE) | Chief executive officer responsible for operational exam governance | Initiates / Enforces |
| Dean of Academic Affairs | Academic leadership ensuring alignment with degree statutes | Verifies / Attests |
| Lead Examination Policy Architect | Policy author drafting statutory clauses and decision tables | Drafts / Configures |

---

# 7. Preconditions and Entry Criteria

- Statutory mandate or directive issued by university leadership.
- Academic Council convened with approved quorum.
- Policy repository in `ACTIVE` governance mode.

---

# 8. Trigger

Formal directive to execute examination policy definition to institutional activation submitted to Board of Examinations.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Policy proposal and statutory parameter drafting initiated | Lead Examination Policy Architect / Examination Policy | Policy docket created in `POLICY_DRAFT` state. |
| 2 | Academic scrutiny and regulatory compliance review conducted | Dean of Academic Affairs / Academic Affairs | Policy clauses verified against UGC/AICTE statutory guidelines. |
| 3 | Board of Examinations review and formal submission to Academic Council | Controller of Examinations (CoE) / Examination Domain | Executive endorsement signed and docket forwarded. |
| 4 | Academic Council formal debate, resolution passing, and statutory decree | Academic Council / Board of Examinations / Governance | Council resolution approved and digitally sealed. |
| 5 | State transition committed in Examination Policy Registry and canonical event published | Examination Policy | Policy aggregate transitioned to `POLICY_ACTIVE_LOCKED`; `Examination_Policy_Activated` broadcast. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Alt-1 | Conditional Council Ratification | Council requests clause modification prior to final assent | Drafting committee incorporates specific amendment; Chair grants executive assent | Academic Council / Board of Examinations |
| Alt-2 | Expedited Executive Ordinance | Urgent regulatory compliance deadline between Council meetings | Vice Chancellor issues emergency executive order; tabled for ex-post facto Council ratification | Controller of Examinations (CoE) |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|---|
| Fail-1 | Council rejects proposed policy clauses | Policy rejected; cannot proceed to activation | Revert docket to `DRAFT_REJECTED`; return to drafting committee with Council minutes | Examination Policy |
| Fail-2 | Regulatory conflict with university statutes detected | Policy review suspended | Place docket on `STATUTORY_HOLD`; refer to University Legal Counsel | Examination Policy |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-I01-001: Statutory Academic Council Quorum & Approval Invariant | Examination Policy | Mandatory institutional authority required for valid policy enactment | `L2-I01_Examination_Policy_and_Configuration_Workflows.md` §13 |
| BR-I01-VER: Immutable Policy Versioning Invariant | Examination Policy | An activated policy version is permanently locked; modifications require a new version | `L2-I01_Examination_Policy_and_Configuration_Workflows.md` §8.1 |
| BR-I01-AUD: Statutory Record Preservation Rule | Examination Policy | Full legislative history, council minutes, and votes must be preserved perpetually | `L2-I01_Examination_Policy_and_Configuration_Workflows.md` §6.5 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Examination Policy Aggregate Aggregate | `POLICY_DRAFT` | Formulated, ratified, and activated under governance workflow | `POLICY_ACTIVE_LOCKED` | Examination Policy | `L2-I01_Examination_Policy_and_Configuration_Workflows.md` §8.1 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

**Published Business Events:**
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `Examination_Policy_Activated` | Examination Policy | Academic Delivery, Student Lifecycle, Faculty, Administration | Broadcasts authoritative policy enactment across university | `L2-I01_Examination_Policy_and_Configuration_Workflows.md` §6.4 |

**Consumed Business Events:**
| Business Event | Source Domain / Publisher | Consumption Effect | Handling Domain / Role | Authoritative Source |
|---|---|---|---|---|
| `Academic_Council_Resolution_Passed` | Governance (Section B) | Authorizes policy enactment and binding | Examination Policy / Controller of Examinations (CoE) | `L2-B01_Governance_Workflows.md` |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Governance (Section B) | Examination Policy | VerifyCouncilQuorumContract | Synchronous validation of Academic Council voting quorum and assent | `L2-I01_Examination_Policy_and_Configuration_Workflows.md` §9 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Policy Draft Formulation | Lead Examination Policy Architect | Formulates regulatory clauses and parameter tables | Examination Policy |
| Executive Endorsement | Controller of Examinations (CoE) | Endorses draft for statutory Council consideration | Examination Policy |
| Statutory Ratification | Academic Council / Board of Examinations | Highest university authority for academic regulations | Governance / Examination |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Separation of Formulation and Ratification | Policy Drafter vs Academic Council | Drafter cannot unilaterally ratify or activate policy without Council assent. |
| Executive Enforcement Independence | Controller of Examinations vs Academic Council | CoE enforces policy as approved without discretionary power to alter Council rules. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Council Resolution & Minutes | Digitally signed resolution signed by Council Chairman and Registrar | Governance / Examination |
| Master Policy Artifact | Cryptographically hashed policy document locked in institutional registry | Examination Policy |
| Statutory Publication Certificate | Timestamped publication ledger certifying distribution to all colleges | Examination Policy |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Policy aggregate transitioned to `POLICY_ACTIVE_LOCKED` in Examination Policy Master.
- Academic Council resolution seal verified and bound.
- `Examination_Policy_Activated` broadcast emitted and acknowledged.

## 16.2 Postconditions

- All subsequent examination planning and evaluation workflows strictly bind to this active policy.
- Faculty and students queryable rules reflect updated standards.
- Institutional compliance posture certified.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open architectural issues.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Statutory Quorum Legitimacy | Academic Council meeting fulfills statutory attendance requirements | Legal challenge to policy validity |
| Timely Publication | Policy broadcast before registration opens for next exam cycle | Grandfathering disputes with student cohorts |

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Web portal CMS rendering of policy text | L4 / L5 | User interface layout and formatting is a software design concern |
