# Erroneous Person Consolidation to Restored Identity Separation

# 1. Scenario Metadata

**Scenario ID:** L3-A-006  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Person & Identity  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  ; `L2-A02_User_Account_and_Person_Reconciliation_Authority_Amendment_2026-09-23.md`
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Correct an erroneous Person consolidation through governed separation so the individuals affected by an incorrect merge, identity fraud or data-migration error are again represented by the correct canonical Persons while references, identity history and audit history remain preserved.

---

# 3. Business Outcome

The consolidation is reversed through governed Person separation, references are redistributed to the correct canonical Persons, and identity and audit history are preserved.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved administrative review determines that consolidated Person records must be separated because of an incorrect merge, detected identity fraud, or a data migration error.

## 4.2 End Boundary

The intended end boundary is reached when the erroneous consolidation has been reversed through governed Person separation, references have been redistributed to the correct canonical Persons, identity and audit history have been preserved, and `PersonSplit` has been published by the Person aggregate.

The exact Person and Identity lifecycle states, associated Identity handling and downstream reference-redistribution contract are unresolved upstream and are recorded in Section 17.

## 4.3 In Scope

- Validation of the approved separation determination.
- Review of the original merge and its preserved history.
- Split-eligibility, dependency and legal-compliance validation.
- Confirmation of required administrative approval.
- Preparation of the required reference-redistribution plan.
- Creation of a new Person as required by the authoritative split workflow.
- Redistribution of references to the correct canonical Persons.
- Preservation of identity and audit history.
- Publication of `PersonSplit`.
- Generation and preservation of immutable audit evidence.

## 4.4 Out of Scope

- Initial duplicate identification and canonical Person reconciliation under L3-A-005.
- Normal establishment of an unrelated new Person under L3-A-004.
- Re-performing or silently modifying the original merge decision.
- Credential, authentication-factor, Authentication Provider or Authentication Session lifecycles owned by L2-A03.
- Role, Role Assignment, Delegation or authorization-policy lifecycle changes owned by L2-A04.
- Redesign or re-ownership of downstream business-domain records.
- Technical recovery, rollback or restoration mechanisms.

The corrective creation of a Person explicitly required by the L2-A02 split workflow remains inside A006, but its relationship to the normal A004 Person–Identity establishment sequence is unresolved in Section 17.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-A-004 — Person Establishment to Active Canonical Identity | Governs normal Person and Identity establishment; A006 contains a corrective Person-creation step whose lifecycle relationship to A004 is undefined upstream. |
| Preceded by | L3-A-005 — Duplicate Person Identification to Canonical Identity Reconciliation | A006 reverses a prior consolidation only after an approved review determines that the merge was incorrect or otherwise requires separation. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform & Tenant / Section A | Owns the Tenant boundary and prevents cross-Tenant identity execution. |
| Person & Identity / Section A | Owns the Person and Identity aggregates, merge and split history, split validation, Person separation, canonical references, `PersonSplit` and the final restored-separation outcome. |
| Authorization & Policy / Section A | Owns authorization policies governing who may review, approve and perform the separation; it does not own the Person split decision or lifecycle. |
| Audit Infrastructure / Section A | Owns immutable platform audit evidence generated from canonical business events; it does not own the identity-separation decision. |

Business domains retain ownership of their own business records. Redistribution of identity references does not transfer ownership of those records to Person & Identity.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Persons | The real individuals whose identities were incorrectly represented by the consolidation. | Subjects of the corrective separation; no review or approval authority is assigned by this scenario. |
| Authorized administrators | Review the consolidation history and perform permitted administrative split actions. | May act only under applicable authorization, approval and split policies. |

L2-A02 requires administrative review and approval but does not identify the specific canonical business role authorized to approve Person separation. The gap is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid `Active` Tenant context has been resolved under L2-A01.
- The consolidated Person and relevant historical merge sources belong to the same Tenant.
- A prior Person consolidation and its audit history exist.
- The approved administrative determination identifies an incorrect merge, detected identity fraud or data-migration error as the reason separation is required.
- The original merge history and information required for eligibility, dependency and legal-compliance validation are available.

---

# 8. Trigger

An approved administrative review determines that consolidated Person records must be separated because of an incorrect merge, detected identity fraud, or a data migration error.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Accept the approved administrative determination and identify the consolidation that must be separated. | Authorized administrators; Person & Identity | The corrective separation scope is established. |
| 2 | Review and validate the preserved history of the original merge. | Person & Identity | The prior consolidation and affected identity references are confirmed. |
| 3 | Validate split eligibility, dependencies and legal-compliance requirements. | Person & Identity | The proposed separation is eligible to continue or is blocked. |
| 4 | Establish the required reference-redistribution plan. | Person & Identity | The intended allocation of references between the correct canonical Persons is defined, subject to the unresolved collaboration contract in Section 17. |
| 5 | Confirm that the administrative approval required for Person split is valid for execution. | Person & Identity; Authorization & Policy | The separation is authorized to proceed. |
| 6 | Create the new Person required by the authoritative split workflow. | Person & Identity — Person aggregate | An additional canonical Person exists for the restored separation; its exact state and associated Identity lifecycle remain unresolved in Section 17. |
| 7 | Redistribute applicable references between the correct canonical Persons while preserving ownership of downstream business records. | Person & Identity | References are associated with the correct Persons, subject to the unresolved hand-off in Section 17. |
| 8 | Preserve the original consolidation history, separation history and identity history. | Person & Identity | Historical reconstruction remains possible without rewriting the original merge evidence. |
| 9 | Publish `PersonSplit` from the Person aggregate. | Person & Identity — Person aggregate | The governed separation is announced through its canonical event. |
| 10 | Preserve immutable audit evidence of the completed separation. | Person & Identity; Audit Infrastructure | The separation decision and outcome remain auditable. |

The flow ends at restored identity separation and does not reopen the prior A005 merge as though it had never occurred.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow toward the same restored-separation outcome is completely defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The separation determination is not approved or does not identify an authoritative A006 cause. | The scenario lacks a valid frozen trigger. | Do not initiate Person separation through A006. | Person & Identity |
| The original merge history cannot be validated. | Split eligibility and correct reference redistribution cannot be established. | Block separation until complete authoritative merge history is available. | Person & Identity |
| The required audit trail is incomplete. | L2-A02 split validation is not satisfied. | Block separation until the audit-history requirement is authoritatively satisfied. | Person & Identity |
| The reference-redistribution plan is missing or incomplete. | Correct allocation of business references cannot be assured. | Block separation until an approved complete redistribution plan exists. | Person & Identity |
| Dependency validation fails. | Referential integrity cannot be preserved. | Block separation until the dependency conflict is resolved. | Person & Identity |
| Legal-compliance validation fails. | The proposed separation is ineligible. | Block separation until the legal restriction is authoritatively resolved. | Person & Identity |
| Required split approval is not valid or cannot be confirmed. | The administrative operation is unauthorized. | Stop progression pending an authoritative approval outcome. | Person & Identity; Authorization & Policy |
| The split requires an undefined Person or Identity lifecycle transition. | The restored identity outcome cannot be completed without inventing states or transitions. | Stop the affected lifecycle action pending an authoritative L2-A02 decision. | Person & Identity |
| Downstream-owned references cannot be redistributed through an approved contract. | The frozen final outcome cannot be completed. | Stop completion pending definition of the required cross-domain collaboration contract. | Person & Identity |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every business operation executes within exactly one Tenant boundary, and cross-Tenant execution is prohibited. | Platform & Tenant | The separation must remain within the Tenant containing the original consolidation. | L2-A01 — Tenant Foundation and Tenant Isolation |
| PersonID is immutable. | Person & Identity | Separation must not rewrite the immutable identifiers of existing Persons. | L2-A02 — Person Identifier; Person Business Rules |
| Person split is an exceptional administrative operation. | Person & Identity | Separation proceeds only through the governed split workflow. | L2-A02 — Person Split; Split Workflow |
| Person split applies when an incorrect merge, identity fraud or data-migration mistake is identified. | Person & Identity | These conditions define the permitted separation causes. | L2-A02 — Person Split |
| Split requires administrative approval. | Person & Identity | Separation cannot proceed without governed approval. | L2-A02 — Person Split; Approval Policies |
| Split requires a complete audit trail. | Person & Identity | Original consolidation history must remain available and preserved. | L2-A02 — Split Validation |
| Split requires a reference-redistribution plan. | Person & Identity | References must be deliberately reassigned to the correct Persons. | L2-A02 — Split Validation |
| Split requires dependency and legal-compliance validation. | Person & Identity | Failure of either validation blocks the separation. | L2-A02 — Split Validation |
| Split shall preserve referential integrity. | Person & Identity | References must remain valid throughout separation. | L2-A02 — Identity Governance Business Rules |
| Complete audit history is preserved. | Person & Identity | The original merge and corrective separation remain historically reconstructable. | L2-A02 — Person Split |
| Business domains remain owners of their own business records. | Respective business domains | Reference redistribution does not transfer record ownership to Person & Identity. | L2-A02 — Identity Governance Architectural Constraints |
| Audit evidence generated from canonical business events is immutable. | Audit Infrastructure | Evidence of `PersonSplit` is preserved without rewriting the original consolidation evidence. | L2-A07 — Audit Evidence Generation; Audit Principles |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Consolidated Person | Not constrained by the authoritative split workflow | Retained as one of the independent canonical Persons after reference redistribution. | Exact resulting Person state is not defined for split. | Person & Identity | L2-A02 — Identity Split; Split Workflow |
| New Person created by split | Not defined | Created as the additional canonical Person required for restored separation. | Exact creation transition and resulting state are not defined for split. | Person & Identity | L2-A02 — Split Workflow |
| Associated Identity aggregates | Not defined | The frozen outcome requires restored identity separation, but L2-A02 defines no split-specific Identity reassociation or transition. | Not defined | Person & Identity | L2-A02 — Identity Aggregate; Identity Split |
| Audit Evidence | Canonical business event published | Immutable evidence is created for the completed separation without modifying original evidence. | Created once and immutable | Audit Infrastructure | L2-A07 — Audit Evidence Generation; Audit Evidence Lifecycle |

No Person or Identity lifecycle transition beyond the high-level split effect is asserted because L2-A02 does not define the required states and transitions. The gap is recorded in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PersonSplit` | Person aggregate | Audit Infrastructure, subject to audit-eligibility rules; downstream reference consumers are not defined | Announces completion of the governed Person separation and supports immutable audit evidence generation. | L2-A02 — Person Events; Split Workflow; L2-A07 — Audit Evidence Generation |

L2-A02 defines no `IdentitySplit` event and no split-specific Identity lifecycle events. L3 does not invent them.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No authoritative named synchronous business contract or governed hand-off is defined for redistributing downstream-owned references between the resulting canonical Persons.

The `Split Person` capability listed in L2-A02 does not define the required cross-domain reference-redistribution responsibilities. The gap is recorded in Section 17.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Conduct the administrative review leading to separation | Authorized administrators | Must act under applicable authorization policies and produce the approved determination required by the frozen trigger. | Person & Identity |
| Approve Person split | Not defined in locked L2-A02 or L2-A04 | Approval is mandatory; L3 does not assign an unsupported approving role. | Person & Identity |
| Execute approved Person separation | Authorized administrators | Permitted only after eligibility, audit, reference-plan, dependency, legal-compliance and approval requirements succeed. | Person & Identity |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty requirement is defined by the authoritative L2 architecture.

L3 does not infer that the administrative review, split approval and split execution must be performed by different actors.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the original consolidation history. | The complete audit trail of the original merge without modification or deletion. | Person & Identity |
| Preserve the approved separation determination. | The administrative-review outcome establishing the permitted separation cause. | Person & Identity |
| Preserve split approval. | The governed administrative approval required by L2-A02. | Person & Identity |
| Preserve the redistribution basis. | The reference-redistribution plan required by split validation. | Person & Identity |
| Preserve validation outcomes. | Evidence of dependency and legal-compliance validation. | Person & Identity |
| Preserve immutable evidence of separation. | Canonical audit evidence generated from `PersonSplit` without rewriting the original merge evidence. | Audit Infrastructure |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The approved administrative determination has been validated.
- Split eligibility, dependency and legal-compliance requirements have succeeded.
- The required audit trail and reference-redistribution plan exist.
- Required administrative approval has been confirmed.
- The erroneous consolidation has been separated into the correct canonical Persons.
- Applicable references have been redistributed without transferring ownership of downstream business records.
- Identity and audit history remain preserved.
- `PersonSplit` has been published by the Person aggregate.
- Immutable audit evidence of the separation exists.

Completion cannot be confirmed until the Person and Identity lifecycle effects and the cross-domain reference-redistribution contract identified in Section 17 are resolved.

## 16.2 Postconditions

- The affected real individuals are represented by separate canonical Persons within the applicable boundary.
- The original merge remains part of immutable historical evidence rather than being erased.
- The corrective separation and redistributed references remain auditable.
- Downstream identity-reference consumers may respond to `PersonSplit` only through approved collaboration mechanisms.
- Authentication and authorization lifecycles remain with L2-A03 and L2-A04 respectively.

---

## Canonical R3B Remediation

This scenario consumes the Locked A02 R3B amendment. Earlier split blockers are resolved as follows:

- Person Reconciliation Authority is the abstract split-approval authority;
- corrective split may restore two or more canonical Persons according to evidence and is not limited to one-to-two;
- each resulting Person follows normal A02 establishment/activation semantics appropriate to recovered evidence;
- each Identity/User Account receives an explicit governed disposition;
- A02 does not directly reassign Credential/MFA/Authentication Session state;
- downstream domains redistribute their own references after `PersonSplit`;
- A006 completes at authoritative A02 reconciliation and publication of `PersonSplit`.

No cross-domain business data is directly rewritten by Person & Identity.

# 17. Open Issues, Assumptions and Deferred Questions

No blocking issues remain after the Locked R3B upstream amendment(s).

No assumptions.
