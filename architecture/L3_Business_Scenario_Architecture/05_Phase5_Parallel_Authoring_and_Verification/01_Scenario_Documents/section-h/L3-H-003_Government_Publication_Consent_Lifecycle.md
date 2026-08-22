# Government Publication Consent Lifecycle

# 1. Scenario Metadata

**Scenario ID:** L3-H-003  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, C, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; applicable Student Lifecycle identity context; Section A Authorization and Audit architecture  
**Referenced By:** Government credential publication, revocation, republication, reconciliation, and audit activities

# 2. Scenario Purpose

Govern the learner's government-publication consent through the H01 Consent lifecycle so that the effective consent state controls future eligible publication while prior consent evidence remains immutable and auditable.

# 3. Business Outcome

The effective consent state governs future publication while historical consent evidence remains immutable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a learner grants, changes, withdraws, or allows publication consent to expire.

## 4.2 End Boundary

The scenario ends when H01 records the resulting effective Consent state and that state governs future publication eligibility without altering historical consent evidence.

## 4.3 In Scope

- Resolving the canonical learner/student reference used by the H01 Consent record.
- Validating consent scope, authorization, effective dates, and current consent state.
- Granting, updating, withdrawing, or expiring H01 Consent.
- Enforcing one active consent for the same scope.
- Publishing the exact H01 Consent event corresponding to the completed change.
- Preserving immutable consent history and audit evidence.

## 4.4 Out of Scope

- Creating or changing the canonical Student identity or Student lifecycle.
- Publishing a credential; publication is a separate H01 scenario.
- Defining new consent states, consent scopes, or consent actors beyond locked H01.
- Technical access-control, persistence, or integration mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical Authorization and Audit ownership supporting authorised consent action and immutable evidence. |
| Student Lifecycle / Section C | Retains canonical learner/student identity and lifecycle; H01 references the Student and does not modify it. |
| APAAR & DigiLocker / Section H | Owns the Consent aggregate, Consent state, government-publication eligibility consequence, consent history, and H01 Consent events. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Grants, updates, or withdraws publication consent for the applicable scope. | Locked H01 authorization material assigns these consent actions to Student. |

Consent expiry is a policy-governed H01 lifecycle consequence rather than a new human decision.

# 7. Preconditions and Entry Criteria

- The referenced Student exists under its canonical Student Lifecycle owner.
- The requested consent scope is valid for the institution and intended government-publication context.
- The consent action is authorised.
- Existing consent state and effective dates are available for validation.

# 8. Trigger

A learner grants, changes, withdraws, or allows publication consent to expire

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Resolve the canonical Student reference and applicable publication-consent scope. | APAAR & DigiLocker consuming Student Lifecycle context | H01 has the authoritative learner reference without acquiring Student ownership. |
| 2 | Validate authorization, consent scope, effective dates, and existing Consent state. | APAAR & DigiLocker / Section A | Invalid consent changes are prevented. |
| 3 | Apply the source-defined Consent action: grant, update, withdraw, or expire. | Student / APAAR & DigiLocker | H01 Consent reaches the permitted effective state. |
| 4 | Enforce the invariant that only one active consent exists for the same scope. | APAAR & DigiLocker | Conflicting active consent is prevented. |
| 5 | Publish the exact H01 Consent event for the completed outcome. | APAAR & DigiLocker | `ConsentGranted`, `ConsentUpdated`, `ConsentWithdrawn`, or `ConsentExpired` communicates the completed H01 fact. |
| 6 | Preserve immutable consent history and audit evidence. | APAAR & DigiLocker / Platform Audit | Historical consent evidence remains traceable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Consent is granted from Pending | A valid learner grants consent for the scope. | Transition Consent to `Granted`. | Future publication may use the consent where policy requires it. | APAAR & DigiLocker |
| Granted consent is changed | A valid change is requested without withdrawing consent. | Apply the valid update while preserving history. | Updated consent becomes effective. | APAAR & DigiLocker |
| Granted consent is withdrawn | Learner withdraws active consent. | Transition Consent to `Withdrawn`. | Future publication governed by that consent is prevented. | APAAR & DigiLocker |
| Granted consent expires | Configured expiry is reached. | Transition Consent to `Expired`. | Future publication requiring that consent is prevented until a valid consent state exists. | APAAR & DigiLocker |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Student reference is invalid | Consent cannot be attached to an authoritative learner identity. | Correct the source reference before completing the consent action. | Student Lifecycle / APAAR & DigiLocker |
| Consent scope or dates are invalid | Requested consent state cannot be established. | Reject the invalid consent change and preserve existing consent history. | APAAR & DigiLocker |
| A conflicting active consent exists for the same scope | H01 invariant would be violated. | Resolve the existing consent state through the permitted Consent lifecycle before establishing a new active consent. | APAAR & DigiLocker |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only one active Consent exists for the same scope. | APAAR & DigiLocker | Duplicate active consent is prohibited. | L2-H01 §11.2 |
| Withdrawn consent immediately prevents future publication. | APAAR & DigiLocker | Publication eligibility changes from the effective withdrawal. | L2-H01 §11.2 |
| Historical consent records are immutable and every consent action is auditable. | APAAR & DigiLocker / Platform Audit | Prior consent evidence cannot be overwritten. | L2-H01 §§11.2, 52 |
| Consent status is limited to `Pending`, `Granted`, `Expired`, and `Withdrawn`. | APAAR & DigiLocker | L3 cannot invent additional consent states. | L2-H01 §§13, 62 |
| Consent validation must complete before publication where policy requires consent. | APAAR & DigiLocker | Effective Consent directly governs future H01 publication eligibility. | L2-H01 §§16, 29, 56 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Consent | `Pending` or `Granted` as applicable | Grant, update, withdrawal, or expiry is applied. | `Granted`, `Withdrawn`, or `Expired` according to the source-defined transition; an update preserves a valid current consent. | APAAR & DigiLocker | L2-H01 §§11.2, 13, 62 |
| Student | Existing canonical Student state | Referenced only. | Unchanged. | Student Lifecycle | L2-H01 ownership boundary / Student architecture |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H01 Consent publishes the exact completed fact: `ConsentGranted`, `ConsentUpdated`, `ConsentWithdrawn`, or `ConsentExpired`. The H01 event catalogue identifies Consent as the producer and Publication as the consumer.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Student Lifecycle supplies the authoritative Student reference. H01 retains only the government-sharing Consent record and applies the resulting effective Consent to future H01 Publication eligibility. No Student lifecycle ownership transfers to H01.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Grant consent | Student | Valid scope and authorization required. | APAAR & DigiLocker |
| Update consent | Student | Existing Consent and valid requested change required. | APAAR & DigiLocker |
| Withdraw consent | Student | Existing Consent required. | APAAR & DigiLocker |
| Expire consent | No human actor defined | Occurs according to effective expiry policy. | APAAR & DigiLocker |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or approval chain is defined for Student consent actions. None is invented.

## 15.3 Business Audit and Evidence Obligations

H01 preserves Student reference, consent action, previous state, new state, effective date, and executing actor where applicable. Historical consent evidence remains immutable.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Canonical Student reference and consent scope are resolved.
- The requested consent action is authorised and valid.
- H01 Consent reaches the permitted effective state.
- The exact corresponding Consent event is published.
- Historical consent evidence remains immutable and auditable.
- Future publication eligibility follows the effective Consent state.

## 16.2 Postconditions

- H01 holds the authoritative current government-publication Consent state.
- Student identity/lifecycle remains owned by Section C.
- Future H01 Publication evaluates the resulting Consent where policy requires it.
- Prior consent history remains available for audit.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-H01 §65 uses stale section-letter labels and describes `B — Student Lifecycle`, while the higher-authority current `DEPENDENCY_MAP.md` assigns Student Lifecycle to Section C. | Following the H01 label literally would misstate the frozen C participation. | `DEPENDENCY_MAP.md` and locked L1 ownership control; H01 dependency labels should be corrected upstream. |

**Architecture assessment:** `PASS WITH ISSUES`

The Consent aggregate, states, actor authority, invariants, events, audit obligations, and final outcome are explicit. The stale H01 section label is an upstream metadata defect but does not prevent a source-grounded Draft.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
