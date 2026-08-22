# Student Profile Update

**Scenario ID:** L3-C-014  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Frozen Stage 3 Section C canonical register; locked L2-C02 — Student Lifecycle & Activation Workflows; L3 Business Scenario Template v2.0  
**Referenced By:** Future L4 System Behavior Specification, if required

---

The frozen Stage 3 Section C canonical register defines this scenario as:

- **Final L3 ID:** L3-C-014
- **Canonical Title:** Student Profile Update
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** C
- **Scenario Type:** Simple
- **Priority:** P1
- **Derivation:** KEEP
- **Construction Status:** Not Authored

These frozen values are preserved without reclassification. fileciteturn73file0L2-L2

**Filename note:** The Phase 5 Operating Guide / Section 8 exact filename could not be independently retrieved from the accessible repository search during this authoring pass. The filename used for the artifact is therefore the repository naming-standard form:

`L3-C-014_Student_Profile_Update.md`

This filename must be checked against Phase 5 Operating Guide Section 8 before repository publication or locking.

---

## 2. Scenario Purpose

This scenario governs the controlled business process for updating Student-owned profile information after Student creation.

Its purpose is to maintain accurate and current Student master information while preserving immutable Student identity, canonical ownership, lifecycle integrity, applicable institutional policy, and historical auditability.

The scenario covers permitted profile changes without changing the Student's canonical identity, enrollment ownership, academic assignment ownership, or lifecycle ownership.

---

## 3. Business Outcome

The singular institutional outcome is:

> **The permitted Student profile information is successfully updated within the canonical Student record, all applicable business constraints are satisfied, the Student's immutable identity remains protected, and the resulting profile state is authoritative for the Student Lifecycle Domain.**

The outcome is a completed business change to Student-owned profile information.

The scenario does not extend into downstream technical synchronization or presentation of the updated information.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a permitted request to modify Student-owned profile information is submitted for an existing Student.

The locked L2-C02 workflow identifies Student Profile Update as a dedicated workflow and defines its purpose as modifying Student-owned profile information after Student creation. fileciteturn69file0L2-L2

## 4.2 End Boundary

The scenario ends when:

1. the requested profile change has passed applicable business validation;
2. any required business approval has been completed;
3. the permitted Student profile information has been updated;
4. the immutable Student identity remains unchanged;
5. the change is auditable; and
6. the resulting Student profile is the authoritative current profile.

The Main Business Flow stops at this business outcome.

## 4.3 In Scope

- Student-owned profile information updates.
- Personal information updates where editable.
- Contact information updates where editable.
- Address information updates.
- Guardian contact information updates where within Student profile ownership.
- Communication preference information where applicable to the Student profile.
- Preferred language information.
- Student photograph/profile information where permitted.
- Emergency contact information.
- Validation of editable attributes.
- Protection of immutable attributes.
- Applicable approval of restricted profile modifications.
- Auditability of completed profile changes.
- Publication of the canonical profile-update business event.

## 4.4 Out of Scope

- Student creation.
- Admission information changes.
- Academic assignment changes.
- Enrollment management.
- Student lifecycle status changes.
- Programme allocation.
- Financial information.
- Examination information.
- Identity correction as a separate institutional correction outcome.
- Technical authorization mechanisms.
- API, persistence, notification, read-model, or implementation processing.

## 4.5 Related Scenarios

No additional canonical related scenario is asserted here because the authoritative Final ID relationship set was not independently retrieved during this authoring pass.

The distinction from C-016 is material: C-016 is **Student Identity Correction**, whereas C-014 concerns ordinary permitted profile updates and does not redefine the institutional identity-correction process.

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **C — Student Lifecycle** | Canonical owner of the Student Aggregate and Student-owned profile information; validates and applies the permitted profile change; owns the completed Student Profile Update outcome. |

The locked L2-C02 architecture explicitly establishes Student Lifecycle as the exclusive owner of the Student Aggregate and associated StudentContext. It also identifies Student master/profile information as part of Student Lifecycle responsibility. fileciteturn75file0L2-L2

No other business domain participates as an owner in this Simple scenario.

Cross-domain consumption of resulting Student information does not transfer ownership.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student | Submit permitted changes to Student-owned profile information. | Initiates permitted profile update. |
| Registrar | Review or approve restricted profile modifications where institutional policy requires approval. | Reviews / approves where applicable. |
| Student Lifecycle | Validate and apply the permitted profile update and maintain the authoritative Student record. | Canonical owner and responsible business domain. |

The locked L2-C02 workflow explicitly identifies Student and Registrar as business actors for profile-update processing and identifies Registrar approval for restricted profile modifications where required. fileciteturn69file0L2-L2

The L2 source also names technical/supporting participants such as workflow, notification, analytics, and audit platforms. Those are not treated as L3 business actors because L3 must remain implementation-independent.

No technical access control/technical identity profiles is defined.

---

## 7. Preconditions and Entry Criteria

The following business conditions must hold before the scenario begins:

1. The Student exists.
2. The Student is within the authoritative Student Lifecycle domain.
3. The Student lifecycle state permits profile modification.
4. The requested profile attributes are editable under institutional policy.
5. The requested change does not attempt to modify an immutable Student identity attribute.
6. Any required institutional configuration or policy governing the requested profile change is applicable.
7. Where required, the change is subject to the applicable approval process.

The L2-C02 workflow explicitly requires the Student to exist, the lifecycle state to permit profile modification, the requested attributes to be editable, and configuration/policy to permit the operation. fileciteturn69file0L2-L2

---

## 8. Trigger

The business trigger is:

> **A permitted Student Profile Update request is submitted for an existing Student.**

The locked L2-C02 workflow identifies the Student Profile Update workflow as the canonical process for modifying Student-owned profile information after Student creation. fileciteturn69file0L2-L2

The L2 source lists multiple initiating channels, but those are interaction/implementation mechanisms and are intentionally not reproduced as L3 triggers.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Submit a permitted request to change Student-owned profile information. | Student | Profile update request begins. |
| 2 | Confirm that the identified Student exists and that the requested profile information falls within Student-owned profile scope. | Student Lifecycle | Request is within the canonical Student profile boundary. |
| 3 | Confirm that the Student lifecycle permits the requested profile modification. | Student Lifecycle | Profile modification is eligible to proceed. |
| 4 | Confirm that each requested attribute is editable under the applicable institutional policy. | Student Lifecycle | Editable profile information is identified. |
| 5 | Confirm that immutable Student identity attributes are not being changed through this scenario. | Student Lifecycle | Canonical identity remains protected. |
| 6 | Apply any required business validation and determine whether institutional approval is required. | Student Lifecycle | Request is either ready for completion or routed for required approval. |
| 7 | Where approval is required, obtain the applicable approval for the restricted profile modification. | Registrar, where applicable | Required business approval is satisfied. |
| 8 | Apply the permitted profile change to the Student's authoritative profile. | Student Lifecycle | Student profile reflects the approved change. |
| 9 | Preserve the change as an auditable Student lifecycle/business record. | Student Lifecycle | Change remains historically traceable. |
| 10 | Publish the canonical profile-update business event. | Student Lifecycle | Interested downstream domains can recognise that the Student profile changed. |
| 11 | Complete the scenario. | Student Lifecycle | Student Profile Update business outcome is achieved. |

The flow is deliberately limited to the institutional profile-update outcome. It does not expand into downstream consumption, notifications, read-model maintenance, or technical processing.

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| AF-01 | Restricted profile modification requires approval | The profile change proceeds through the applicable institutional approval before the Student profile is changed. | Approved profile change can complete. | Student Lifecycle / Registrar |
| AF-02 | Multiple permitted profile attributes are changed together | The permitted attributes are validated as one profile-update business request and applied within the same scenario outcome. | Student profile reflects the permitted combined change. | Student Lifecycle |
| AF-03 | Profile information is editable but requires an institution-specific policy condition | The request proceeds only after the applicable business condition is satisfied. | Valid profile change completes. | Student Lifecycle |

These are governed variants of the same Student Profile Update outcome. They do not create separate scenarios.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Student does not exist | The profile cannot be updated. | Reject the request without changing Student business state. | Student Lifecycle |
| Requested attribute is immutable | The requested change cannot be applied through this scenario. | Reject the prohibited attribute change; use the separate identity-correction process where applicable. | Student Lifecycle |
| Requested attribute is not editable under policy | The requested profile change cannot proceed. | Reject the prohibited profile modification or revise the request to permitted information. | Student Lifecycle |
| Student lifecycle state does not permit profile modification | Profile change cannot proceed at that time. | Reject or defer according to the applicable business policy. | Student Lifecycle |
| Required approval is rejected | The requested restricted profile change cannot complete. | Reject the profile change and preserve the existing profile state. | Student Lifecycle / Registrar |
| Business validation fails | The requested profile change cannot complete. | Correct the business information and submit a valid profile change. | Student / Student Lifecycle |
| Conflicting profile update condition exists | The requested change cannot safely complete as submitted. | Resolve the business conflict before resubmission. | Student Lifecycle |

The L2-C02 source explicitly identifies failure conditions including non-existent Student, unauthorized/immutable attribute modification, failed configuration or business validation, rejected approval, and failed authorization, and states that failures do not modify business state. fileciteturn69file0L2-L2

No technical retry, queue, rollback, server failure, timeout, business exception workflows, or infrastructure recovery is defined.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Student Lifecycle is the canonical owner of the Student Aggregate. | Student Lifecycle | All permitted profile changes remain within Student ownership. | L2-C02 |
| Student-owned profile information is managed by Student Lifecycle. | Student Lifecycle | Profile changes are applied to the canonical Student record. | L2-C02 |
| Requested attributes must be editable. | Student Lifecycle | Non-editable attributes cannot be changed through C-014. | L2-C02 WF-C02-12 |
| Immutable Student identity attributes cannot be changed through profile update. | Student Lifecycle | Student identity remains stable. | L2-C02 WF-C02-12 |
| Student ID is immutable. | Student Lifecycle | Profile update cannot alter Student ID. | L2-C02 |
| Student lifecycle state must permit profile modification. | Student Lifecycle | Certain lifecycle positions may prevent profile update. | L2-C02 WF-C02-12 |
| Institutional configuration/policy governs which profile changes are permitted. | Student Lifecycle | Profile changes are constrained by applicable institutional policy. | L2-C02 WF-C02-12 |
| Restricted profile modifications require applicable approval where policy requires it. | Student Lifecycle | Approval must precede completion of such changes. | L2-C02 WF-C02-12 |
| Profile changes must remain auditable. | Student Lifecycle | Completed profile changes remain historically traceable. | L2-C02 |

The locked workflow identifies personal, contact, address, guardian, communication-preference, photograph, and emergency-contact information as profile categories that may be updated according to institutional policy. fileciteturn69file0L2-L2

The same source identifies Student ID, Student Number, Admission Reference, Institution Identifier, Creation Timestamp, and Audit History as immutable through the profile-update workflow. fileciteturn69file0L2-L2

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Student Aggregate | Existing Student | Profile information is modified without changing canonical Student identity. | Same Student lifecycle position with updated permitted profile information | Student Lifecycle | L2-C02 |
| Student lifecycle | Any state permitting profile modification | No Student lifecycle transition is inherently caused by the profile update. | Same lifecycle position | Student Lifecycle | L2-C02 WF-C02-12 |
| Student identity | Existing canonical identity | Immutable identity remains unchanged. | Same canonical identity | Student Lifecycle | L2-C02 |

**Lifecycle conclusion:** C-014 is primarily a **profile-information modification**, not a Student lifecycle-state transition.

The scenario therefore does not invent a new Student state or transition.

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `StudentProfileUpdated` | Student Lifecycle | Interested dependent domains | Signals that the authoritative Student profile has been updated. | L2-C02 WF-C02-12 |

The locked C02 profile workflow explicitly identifies `StudentProfileUpdated` as the canonical domain event produced by successful profile-update processing. fileciteturn69file0L2-L2

Student Lifecycle is the publisher because the Student Lifecycle Domain is the canonical owner of the Student Aggregate and Student master information. fileciteturn75file0L2-L2

No event is republished by a consuming domain.

## 14.2 Consumed Business Events

No scenario-specific consumed business event is required by the retrieved authoritative C02 profile-update definition.

## 14.3 Synchronous Business Contracts / Hand-offs

C-014 is a Simple scenario with a single canonical participating business domain.

The retrieved L2 source describes internal workflow/service mechanics but does not provide a business-level cross-domain contract that is materially required to express the C-014 institutional outcome.

Accordingly, no technical service/API contract is reproduced.

## 14.4 Cross-Domain Boundary

The resulting `StudentProfileUpdated` event may be consumed by dependent domains, but consumption does not transfer ownership of Student profile information.

The Student Lifecycle Domain remains the canonical owner.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

The profile change must be initiated through an authorised business context.

L3 does not define technical permission codes, IAM groups, access-control lists, claims, or RBAC implementation.

The exact technical authorization mechanism remains a later architecture concern.

## 15.2 Approval

The locked C02 profile-update workflow states that Registrar approval may be required for restricted profile modifications. fileciteturn69file0L2-L2

Accordingly:

- ordinary permitted profile updates may complete without an additional approval where institutional policy does not require one;
- restricted modifications require the applicable approval before completion;
- a rejected required approval prevents the profile modification from completing.

No new approving role is introduced.

## 15.3 Audit

The Student Lifecycle architecture requires lifecycle operations to be auditable and maintains historical information. The profile-update workflow specifically requires an audit record for the completed change. fileciteturn69file0L2-L2

The business evidence must therefore establish:

- what profile change was requested;
- whether approval was required;
- the applicable approval outcome where required;
- that the profile change was completed;
- that the resulting Student profile remains historically traceable.

## 15.4 Separation of Duty

No C-014-specific separation-of-duty rule beyond the stated approval requirement is exposed in the retrieved authoritative sources.

No additional separation-of-duty rule is invented.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C-014 is complete when:

1. an existing Student has submitted a permitted profile update;
2. the requested information is within Student-owned profile scope;
3. the Student lifecycle permits the modification;
4. the requested attributes are editable;
5. immutable identity attributes remain unchanged;
6. applicable business validation succeeds;
7. any required Registrar approval is completed successfully;
8. the permitted profile information is updated in the authoritative Student record;
9. the change is auditable; and
10. `StudentProfileUpdated` is published by Student Lifecycle.

## 16.2 Postconditions

| Concern | Postcondition |
|---|---|
| Student identity | Unchanged |
| Student ID | Unchanged |
| Student profile | Updated with the permitted information |
| Student lifecycle state | Unchanged unless an independent lifecycle process separately changes it |
| Ownership | Remains with Student Lifecycle |
| Audit | Profile change remains historically traceable |
| Business event | `StudentProfileUpdated` has been published by Student Lifecycle |
| Downstream domains | May consume the published business event without acquiring ownership |
| Scenario | Student Profile Update business outcome is complete |

## 16.3 Downstream Consequence Boundary

The scenario ends after the Student Profile Update outcome and canonical event publication.

Any downstream domain reaction, notification, read-model update, analytics processing, or other consequence is outside the Main Business Flow.

## 16.4 End Boundary

The end boundary is reached when the authoritative Student profile contains the approved permitted change and the change is auditable.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Phase 5 Operating Guide Section 8 exact filename could not be independently retrieved through the accessible repository search during this authoring pass. | Repository filename correctness cannot be independently proven from the available source. | Confirm the exact assigned filename from Phase 5 Operating Guide Section 8 before repository publication/locking. |
| The retrieved L2-C02 workflow exposes technical initiation mechanisms and service-oriented details in addition to business semantics. | L3 must avoid carrying those implementation details into the scenario. | No L3 change required; technical details have intentionally been excluded. |
| The retrieved L2 profile workflow lists several profile categories and immutable attributes; exact institutional editability of each category may remain policy-dependent. | The scenario must not assume every listed attribute is universally editable. | Apply institutional policy/configuration as upstream defined; no local editability rule is created. |

None of these issues requires changing the frozen C-014 identity, ownership, type, or priority.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| C-014 is limited to Student-owned profile information and is not the identity-correction scenario. | Frozen C-014 title and distinct C-016 title. fileciteturn73file0L2-L2 | If contradicted upstream, boundary must be changed through architecture change control. |
| Student Lifecycle remains the canonical owner of the updated profile information. | Locked L2-C02 ownership model. fileciteturn75file0L2-L2 | Would require an upstream ownership decision. |
| `StudentProfileUpdated` is the canonical event for successful profile update. | Locked C02 workflow definition. fileciteturn69file0L2-L2 | Event mapping would need correction if an authoritative event registry contradicts it. |
| Registrar approval applies only where institutional policy requires approval for restricted profile modifications. | Locked C02 workflow. fileciteturn69file0L2-L2 | Exact approval conditions would need upstream clarification. |

Assumptions do not override locked architecture.

## 17.3 Deferred Questions

1. Which exact profile attributes are institutionally restricted at each deployment/configuration?
2. What exact business approval conditions distinguish ordinary from restricted profile modifications?
3. Are there any additional authoritative business rules in L1 or the enterprise rule registry that materially constrain C-014?
4. Are there additional canonical consumers of `StudentProfileUpdated` that should be named in the final L3 document?

These questions do not prevent a source-grounded Draft, but should be confirmed before Locking if the answers materially affect scenario behaviour.

---

# Validation and Review Classification

## Frozen Identity

- **Final ID:** L3-C-014
- **Title:** Student Profile Update
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** C
- **Type:** Simple
- **Priority:** P1
- **Derivation:** KEEP
- **Status:** Draft

The Stage 3 canonical register confirms these values. fileciteturn73file0L2-L2

## Ownership

Student Lifecycle is the canonical owner of the Student Aggregate and Student master information. fileciteturn75file0L2-L2

No ownership is transferred by the profile update.

## Workflow Alignment

The locked C02 architecture explicitly defines a **Student Profile Update Workflow** covering personal, contact, address, guardian, communication preference, photograph, and emergency-contact profile information, subject to institutional policy. fileciteturn69file0L2-L2

It also defines validation, approval, update, audit, and `StudentProfileUpdated` publication as part of the workflow. fileciteturn69file0L2-L2

## Lifecycle Alignment

The profile update does not invent or modify Student lifecycle states. It changes permitted profile information while preserving the Student's canonical identity and lifecycle position.

## Event Alignment

`StudentProfileUpdated` is retained exactly as exposed by the locked C02 architecture, with Student Lifecycle as canonical publisher. fileciteturn69file0L2-L2

## Implementation Independence

The scenario intentionally excludes:

- APIs
- commands
- payloads
- databases
- services
- technical access control/IAM
- UI/screens
- deployment
- queues
- technical retries
- rollback
- infrastructure failure handling

## Template Validation

All 17 mandatory L3 Template v2.0 sections are present:

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

The locked template requires exactly this 17-section canonical structure and requires unresolved upstream ambiguity to be recorded rather than silently resolved. fileciteturn68file0L2-L2

---

# Final Classification

**PASS WITH ISSUES — Draft**

C-014 is sufficiently supported by the locked L2-C02 architecture to author a complete, source-grounded L3 Draft.

The frozen scenario identity, ownership, scope, profile-update workflow, business actors, approval condition, immutable identity constraints, lifecycle impact, audit obligation, and canonical `StudentProfileUpdated` event are all supported by the retrieved authoritative architecture.

The remaining issues are non-blocking for Draft status, primarily the independently unverified Phase 5 exact filename and policy-level detail that may vary by institutional configuration.

**This document is Draft only. It is not Architecture Approved or Locked.**
