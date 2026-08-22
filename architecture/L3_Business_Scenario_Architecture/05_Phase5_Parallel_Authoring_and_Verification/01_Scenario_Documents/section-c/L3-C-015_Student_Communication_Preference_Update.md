# Student Communication Preference Update

**Scenario ID:** L3-C-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Frozen Stage 3 Section C canonical semantics; locked L1 Business Domain Architecture; locked L2-C02 — Student Lifecycle & Activation Workflows; L3_Business_Scenario_Template_v2.0.md  
**Referenced By:** Future L4 System Behavior Specification, if required

---

The frozen Stage 3 Section C canonical register defines C-015 as:

- **Canonical scenario title:** Student Communication Preference Update
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** C
- **Scenario Type:** Simple
- **Priority:** P2
- **Derivation:** KEEP
- **Construction Status:** Not Authored

These values are frozen and are preserved unchanged. fileciteturn73file0L2-L2

The Phase 5 operating rules state that the exact filename is derived mechanically from the frozen canonical title. fileciteturn80file2L1-L2

Accordingly:

**Exact filename:** `L3-C-015_Student_Communication_Preference_Update.md`

**Target section:** `section-c`

The Phase 5 workload allocation assigns the complete Section C range `L3-C-001` through `L3-C-075` to **Varad**. fileciteturn80file2L1-L2

---

## 2. Scenario Purpose

This scenario governs the controlled business update of a Student's communication preferences within the canonical Student Lifecycle domain.

The purpose is to allow the Student's permitted communication preferences to be changed while preserving Student ownership, the canonical Student identity, applicable institutional policy, and the distinction between business preference data and the communication service that may later act on those preferences.

Communication preference maintenance is part of the Student-owned profile context: the locked L2-C02 Student Aggregate model identifies **Communication Preferences** as a Student Aggregate component, including preferred language, notification preference, email consent, SMS consent, portal consent, and mobile-app preference. fileciteturn78file0L2-L2

---

## 3. Business Outcome

The singular institutional outcome is:

> **The Student's permitted communication preferences are updated in the authoritative Student profile, the resulting preferences are valid under applicable institutional policy, and the updated preference state is available as the canonical Student-owned preference information.**

The scenario does not itself deliver a communication.

It changes the Student's communication preference information; subsequent communication delivery remains outside the Student Lifecycle ownership boundary.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a Student requests a permitted change to communication preferences maintained as part of the Student profile.

The Student Lifecycle architecture explicitly identifies Communication Preferences as part of the Student Aggregate. fileciteturn78file0L2-L2

## 4.2 End Boundary

The scenario ends when:

1. the requested communication-preference change is validated;
2. any applicable business policy conditions have been satisfied;
3. the permitted preference information is updated in the authoritative Student profile; and
4. the completed change is available as the Student's current canonical preference state.

The scenario does not continue into communication delivery, notification scheduling, subscription execution, or provider interaction.

## 4.3 In Scope

- Student communication-preference changes.
- Preferred language, where governed as a Student communication preference.
- Notification preference.
- Email consent.
- SMS consent.
- Portal consent.
- Mobile-app preference.
- Validation of permitted preference changes.
- Preservation of Student ownership.
- Auditability of the completed Student profile change.

The Student Aggregate model explicitly identifies these communication-preference categories. fileciteturn78file0L2-L2

## 4.4 Out of Scope

- Notification delivery.
- Notification scheduling.
- Communication-template management.
- Communication-provider management.
- Recipient routing execution.
- Communication subscription infrastructure.
- Identity management.
- Student Identity Correction — C-016.
- Student Profile Update — C-014, except where a preference change is specifically the canonical outcome of this scenario.
- Technical APIs, databases, services, queues, UI, IAM/RBAC, deployment, or infrastructure.

## 4.5 Boundary with C-014

C-014 is the broader **Student Profile Update** scenario.

C-015 is the canonical scenario specifically concerned with the Student's communication-preference outcome.

The existence of shared Student profile ownership does not merge the two canonical scenarios. The frozen Stage 3 register establishes C-014 and C-015 as separate canonical scenarios. fileciteturn73file0L2-L2

---

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| **C — Student Lifecycle** | Canonical owner of the Student Aggregate and its Student communication-preference information; owns the completed preference-update outcome. |

The locked L2-C02 architecture states that Student Lifecycle is the exclusive owner of the Student Aggregate and associated StudentContext. fileciteturn78file0L2-L2

The Student Aggregate explicitly contains Communication Preferences. fileciteturn78file0L2-L2

No other business domain is required to own the C-015 outcome.

The Communication domain, where applicable, may consume the resulting preference information, but consumption does not transfer ownership from Student Lifecycle.

---

## 6. Business Actors, Roles and Responsibilities

The authoritative material available for this authoring pass establishes Student Lifecycle ownership and identifies Student as the canonical Student persona, but it does not expose a sufficiently precise C-015-specific actor/approval matrix.

Accordingly, no unsupported institutional role is invented.

| Business Responsibility | Authoritative Boundary |
|---|---|
| Request a permitted communication-preference change | Student / authorised Student business context |
| Validate and apply the Student preference change | Student Lifecycle |
| Consume resulting communication preference information for communication activity | Communication capability, where applicable; ownership of Student preference remains with Student Lifecycle |

No technical access control/technical identity profiles is introduced.

If a C-015-specific approval authority exists in locked L2 architecture, it must be adopted exactly from that source rather than inferred.

---

## 7. Preconditions and Entry Criteria

The following conditions are required:

1. The Student exists as a canonical Student.
2. The Student is owned by Student Lifecycle.
3. The requested information is within the Communication Preferences component of the Student Aggregate.
4. The requested preference change is permitted by applicable institutional policy.
5. Any applicable business validation condition is satisfied.
6. The change does not attempt to modify Student identity or another separately owned concept.

The locked Student Aggregate architecture establishes Student Lifecycle ownership and includes Communication Preferences as a Student Aggregate component. fileciteturn78file0L2-L2

Because the exact C-015 workflow precondition list was not retrievable from the available authoritative source material during this pass, no additional precondition is invented.

---

## 8. Trigger

The business trigger is:

> **A permitted request is made to change the Student's communication preferences.**

This is a business trigger, not an API call, technical message, queue operation, database mutation, or UI action.

The exact canonical C-015 trigger wording is not exposed in the retrieved authoritative L2 evidence; therefore the trigger is stated only at the business level supported by the scenario title and Student Aggregate ownership.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Receive a request to change the Student's communication preferences. | Student Lifecycle | Preference-update processing begins. |
| 2 | Confirm that the request concerns Student-owned Communication Preferences. | Student Lifecycle | Request is within C-015 scope. |
| 3 | Validate the requested preference values against applicable institutional policy and permitted preference choices. | Student Lifecycle | Requested change is business-valid. |
| 4 | Confirm that the request does not alter Student identity or another separately owned business concept. | Student Lifecycle | Ownership and identity boundaries remain intact. |
| 5 | Apply the permitted communication-preference change to the Student profile. | Student Lifecycle | Current Student communication preferences are updated. |
| 6 | Preserve the completed change as auditable Student-owned business information. | Student Lifecycle | Preference change remains historically traceable. |
| 7 | Establish the completed preference-update outcome for downstream communication consumers. | Student Lifecycle | Updated preference state is authoritative. |
| 8 | End the scenario. | Student Lifecycle | Student Communication Preference Update is complete. |

The flow stops at the Student-owned business outcome.

It does not proceed into communication delivery or downstream consumer processing.

---

## 10. Alternate Business Flows

## AF-01 — Single Preference Change

A Student changes one permitted communication preference.

The requested preference is validated and the Student profile is updated.

The scenario ends with the updated preference as the authoritative Student-owned state.

## AF-02 — Multiple Preference Changes

A Student changes multiple permitted communication preferences as one business request.

The requested preferences are validated together and the permitted changes are applied as one Student communication-preference outcome.

## AF-03 — Policy-Constrained Preference Change

Where institutional policy restricts a requested communication preference, the change proceeds only when the applicable business condition is satisfied.

The scenario remains the same C-015 outcome.

No new policy rule is invented here.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution |
|---|---|---|
| Student does not exist | Preference cannot be updated. | Do not change Student state; resolve the Student identification issue through the applicable Student Lifecycle process. |
| Requested information is outside Communication Preferences | C-015 cannot own the requested change. | Route the request to the appropriate canonical Student scenario. |
| Requested preference is not permitted by institutional policy | Preference change cannot complete. | Retain the existing preference and resolve the policy condition before resubmission. |
| Requested value is invalid under the applicable business rules | Preference change cannot complete. | Correct the preference request and resubmit. |
| Requested change would alter Student identity | C-015 cannot perform the change. | Route to the authoritative identity-correction process where applicable. |
| Applicable approval or business condition is not satisfied | Preference change remains incomplete. | Satisfy the authoritative condition before completing the change. |

No technical retry, queue, rollback, database recovery, infrastructure failure handling, or exception implementation is defined at L3.

---

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Effect |
|---|---|---|
| Student Lifecycle owns the Student Aggregate. | Student Lifecycle | Communication preferences remain Student-owned. |
| Communication Preferences are part of the Student Aggregate. | Student Lifecycle | C-015 operates within Student ownership. |
| Student identity is immutable. | Student Lifecycle | Preference updates cannot change Student identity. |
| Preference changes must remain within applicable institutional policy. | Student Lifecycle / authoritative institutional policy | Unsupported preference changes cannot complete. |
| Communication preference information must not become a competing Communication-domain master record. | Student Lifecycle | Communication consumers use the Student-owned preference outcome without acquiring Student ownership. |
| C-015 is distinct from Student Identity Correction. | Student Lifecycle | Preference changes cannot be used to bypass the identity-correction boundary. |
| C-015 is distinct from communication delivery. | Student Lifecycle / Communication boundary | Updating a preference does not itself constitute sending a communication. |

The authoritative Student Aggregate definition explicitly includes preferred language, notification preference, email consent, SMS consent, portal consent, and mobile-app preference. fileciteturn78file0L2-L2

The locked L2 architecture also states that Student ownership remains with Student Lifecycle and that downstream domains do not directly modify Student master information. fileciteturn78file0L2-L2

---

## 13. State and Lifecycle Impact

| Business Concept | Entry | Scenario Impact | Result | Owner |
|---|---|---|---|---|
| Student Aggregate | Existing canonical Student | Communication-preference information changes | Same Student Aggregate with updated permitted preferences | Student Lifecycle |
| Student lifecycle state | Existing lifecycle state | No lifecycle transition is required merely to change communication preference | Same lifecycle state | Student Lifecycle |
| Student identity | Existing immutable identity | No change | Same Student identity | Student Lifecycle |
| Communication Preferences | Existing current preference state | Permitted preferences are updated | New authoritative preference state | Student Lifecycle |

C-015 does **not** introduce a new Student lifecycle state.

It is a profile/preference modification rather than a lifecycle transition.

The Student Aggregate remains the canonical identity anchor.

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Event

The exact canonical event name and publisher for the **C-015 Student Communication Preference Update** outcome were not retrievable from the authoritative L2 evidence available during this authoring pass.

Therefore:

> **No event name is invented or inferred from the scenario title.**

This is a material architecture gap because the L3 validation rules require every listed business event to preserve its exact canonical name and publisher.

If the locked C02 workflow defines a C-015-specific preference-update event, that exact event and Student Lifecycle publisher must be used.

## 14.2 Communication Domain Boundary

The L1 architecture identifies **Communication** as a distinct domain owned by Section H. fileciteturn81file6L1-L2

However, Communication does not own the Student Aggregate or Student-owned preference data.

Therefore:

- Student Lifecycle owns the preference information;
- Communication may consume the authoritative preference outcome;
- Communication does not become the canonical owner of Student preference state;
- C-015 does not include communication delivery.

## 14.3 Synchronous Business Hand-off

No specific C-015 cross-domain synchronous hand-off is asserted because the retrieved authoritative material does not define one for this scenario.

No technical API or contract is invented.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

The Student communication-preference change must occur within the authoritative business authorization boundary for Student profile information.

No technical IAM/RBAC implementation is defined.

## 15.2 Approval

The retrieved authoritative evidence does not establish a C-015-specific approval requirement.

Accordingly, this scenario does not invent an approval requirement or approving role.

If the locked C02 workflow defines approval for restricted communication preferences, that rule must be applied exactly.

## 15.3 Audit

The locked Student Lifecycle architecture requires aggregate modifications and lifecycle operations to remain auditable. fileciteturn78file0L2-L2

The completed communication-preference change must therefore remain historically traceable.

The scenario does not prescribe the technical audit mechanism.

## 15.4 Separation of Duty

No C-015-specific separation-of-duty rule is exposed in the retrieved authoritative material.

No additional separation-of-duty rule is invented.

---

## 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

C-015 is complete when:

1. the Student exists;
2. the requested information is within Student-owned Communication Preferences;
3. the requested change is permitted by applicable business policy;
4. the requested preference values are valid;
5. Student identity remains unchanged;
6. the permitted communication preference is updated in the authoritative Student profile;
7. the completed change is auditable; and
8. the Student's current communication-preference state is authoritative.

## 16.2 Postconditions

| Concern | Postcondition |
|---|---|
| Student identity | Unchanged |
| Student Aggregate | Remains owned by Student Lifecycle |
| Communication Preferences | Reflect the completed permitted change |
| Student lifecycle | Unchanged |
| Communication ownership | Student preference remains owned by Student Lifecycle |
| Downstream Communication | May use the resulting preference according to its own architecture |
| Audit | Change remains historically traceable |
| Scenario | Student Communication Preference Update is complete |

## 16.3 Downstream Consequence Boundary

The scenario ends before:

- notification delivery;
- communication scheduling;
- template resolution;
- provider selection;
- delivery confirmation;
- unsubscribe processing in a separately owned communication capability;
- technical message processing.

Those are not Student Lifecycle outcomes.

## 16.4 End Boundary

The end boundary is reached when the Student's permitted communication preferences have been successfully established as the current authoritative Student-owned preference state.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

### OI-C015-01 — Exact C-015 L2 Workflow Definition

The locked L2-C02 document is authoritative and confirms Communication Preferences as part of the Student Aggregate, but the retrieved source material available for this authoring pass did not expose the complete dedicated workflow definition for **Student Communication Preference Update**.

**Impact:** The exact C-015 trigger, preconditions, business rules, approval conditions, completion event, event publisher, and detailed workflow sequence cannot safely be asserted.

**Disposition:** Preserve only the unambiguous Student Aggregate ownership and Communication Preferences semantics. Confirm the dedicated locked L2 workflow before Architecture Approval.

---

### OI-C015-02 — Canonical Completion Event

The exact event name and publisher for C-015 were not exposed in the retrieved authoritative material.

**Impact:** Event ownership is a material L3 validation requirement.

**Disposition:** Do not invent an event. Confirm the canonical event registry / applicable locked L2 workflow before Architecture Approval.

---

### OI-C015-03 — Exact Preference Governance Rules

The Student Aggregate identifies communication-preference categories, but the retrieved evidence does not expose the complete rule set defining which individual preferences require consent, approval, restriction, or other institutional conditions.

**Impact:** Detailed policy rules cannot be safely invented.

**Disposition:** Apply only authoritative upstream rules once confirmed.

---

### OI-C015-04 — Communication Domain Hand-off

Section H owns Communication, while Section C owns Student. The exact business hand-off for updated Student communication preferences was not exposed in the retrieved authoritative source.

**Impact:** A detailed cross-domain contract/event cannot safely be specified.

**Disposition:** Preserve ownership boundaries and derive the exact hand-off from the authoritative L2 architecture before final approval.

---

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| C-015 is specifically the Student communication-preference outcome, separate from generic profile update C-014. | Frozen Stage 3 canonical scenario set. fileciteturn73file0L2-L2 | Boundary would require architecture change control. |
| Student Lifecycle owns the communication preference information. | Locked Student Aggregate ownership. fileciteturn78file0L2-L2 | Would require upstream ownership change. |
| Communication is a downstream consuming capability rather than the owner of Student preference information. | Locked Section H ownership model and Student Aggregate ownership. fileciteturn81file6L1-L2 fileciteturn78file0L2-L2 | Would require ownership review if contradicted. |
| Updating communication preference does not itself send a communication. | Scenario boundary and domain ownership separation. | Would require a boundary correction if contradicted by locked workflow architecture. |

Assumptions do not override locked architecture.

## 17.3 Deferred Questions

1. What is the exact locked L2 workflow governing C-015?
2. What exact business event is published when the preference update completes?
3. Who is the canonical publisher of that event?
4. Which communication-preference changes require explicit consent or approval?
5. What exact business hand-off exists between Student Lifecycle and Communication?
6. Which downstream communication outcomes are triggered by a changed Student preference?

These questions must be resolved before the scenario can advance to Architecture Approved if they materially affect the final outcome, event, authority, rule, or cross-domain boundary.

---

# Validation and Review Classification

## Frozen Identity Validation

- **Final ID:** `L3-C-015`
- **Canonical Title:** Student Communication Preference Update
- **Primary Owner:** Student Lifecycle
- **Participating Domains:** C
- **Type:** Simple
- **Priority:** P2
- **Derivation:** KEEP
- **Assigned Author:** Varad
- **Status:** Draft

The frozen Stage 3 register confirms the C-015 canonical identity and classification. fileciteturn73file0L2-L2

## Filename Validation

The Phase 5 naming rule mechanically derives the filename from the canonical title. fileciteturn80file2L1-L2

Therefore:

`L3-C-015_Student_Communication_Preference_Update.md`

## Ownership Validation

Student Lifecycle is the canonical owner of the Student Aggregate and associated StudentContext. fileciteturn78file0L2-L2

Communication Preferences are explicitly part of the Student Aggregate. fileciteturn78file0L2-L2

## Domain Boundary Validation

Section H owns Communication, but this does not transfer ownership of Student-owned communication preference data. The Student Lifecycle domain remains authoritative for Student master information. fileciteturn81file6L1-L2 fileciteturn78file0L2-L2

## Lifecycle Validation

No Student lifecycle state or transition is invented.

C-015 changes profile preference information while preserving the existing Student lifecycle position.

## Event Validation

**BLOCKING GAP:** The exact C-015 completion event and canonical publisher are not exposed in the retrieved authoritative material.

No event has been invented.

## Implementation Independence

The document contains no:

- API definitions
- Business Data Structures
- database design
- services/classes
- technical access control/IAM
- UI design
- deployment/infrastructure design
- queues
- technical retries
- rollback mechanics
- framework/language choices

## Template Validation

All 17 mandatory v2.0 sections are present:

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

The locked v2.0 template requires these exact 17 sections and requires material upstream ambiguity to be recorded rather than silently resolved. fileciteturn81file3L1-L1

---

# Final Classification

**BLOCKED — Draft**

C-015 has a clear frozen identity, ownership, participating domain, Type, Priority, and broad business boundary.

However, the retrieved authoritative L2 material does not expose enough of the dedicated C-015 workflow to safely assert the exact trigger, detailed rules, approval conditions, canonical completion event/publisher, and Student Lifecycle ↔ Communication hand-off.

Per Phase 5 governance, those gaps are explicitly recorded rather than invented. A material unresolved event/authority/rule/boundary issue prevents Architecture Approval or Locking. fileciteturn80file3L1-L2

**Status remains `Draft`.**

AI authoring is not Architecture Approval or Locking.
