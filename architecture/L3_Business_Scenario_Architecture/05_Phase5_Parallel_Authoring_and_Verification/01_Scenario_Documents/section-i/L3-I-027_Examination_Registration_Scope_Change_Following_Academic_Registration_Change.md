# Examination Registration Scope Change Following Academic Registration Change

**Scenario ID:** L3-I-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, D, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I04 Examination Registration Workflows; authoritative Student and Curriculum/Academic Delivery architecture; applicable Examination Policy, Session, and Schedule architecture  
**Referenced By:** Downstream examination workflows consuming authoritative Examination Registration

## 2. Scenario Purpose

This scenario governs the correction of an Examination Registration scope when an authoritative academic registration change affects the subjects or examination participation that the candidate is eligible to register for.

The scenario preserves ownership of academic registration and curriculum information in the originating domains while ensuring that Examination Registration remains the authoritative owner of examination participation.

## 3. Business Outcome

The candidate's Examination Registration scope is reconciled with the authoritative academic registration change, resulting in an accurate Examination Registration that contains only examination participation permitted by the current academic context and applicable examination policy.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authoritative academic registration change affects the candidate's applicable examination subjects or registration scope.

**In Scope:**
- Recognition of the relevant academic change.
- Identification of affected examination registrations and registration items.
- Re-evaluation of examination subject applicability.
- Controlled amendment of the Examination Registration.
- Required review or approval.
- Preservation of prior registration history.
- Publication of the applicable registration amendment/confirmation outcome.

**Out of Scope:**
- Creating or modifying the academic registration itself.
- Changing curriculum ownership or curriculum rules.
- Changing examination policy.
- Recomputing financial obligations owned by Finance.
- Hall ticket generation.
- Technical synchronization or integration mechanics.

**End Boundary:** The Examination Registration reflects the authorised examination scope resulting from the academic registration change, or remains unchanged when the academic change does not alter examination participation.

## 5. Ownership and Participating Domains

**Examination Registration (I) — Primary Owner:** Owns examination participation and the resulting registration scope.

**Student Lifecycle (C):** Owns the candidate's academic/student registration context.

**Academic Delivery / Curriculum (D):** Owns programme, course, subject, and academic applicability information used to determine the affected scope.

The academic change itself remains owned by C/D. Examination Registration consumes the resulting authoritative business information and governs the examination consequence.

## 6. Business Actors, Roles and Responsibilities

- **Candidate (Student):** Participates in the academic change process and may be affected by the resulting examination registration adjustment.
- **Examination Administrator:** Reviews the impact on Examination Registration and manages the controlled amendment.
- **Academic Administration:** Provides or validates the authoritative academic registration context where applicable.
- **Department Administration:** Verifies departmental requirements where applicable.
- **Controller of Examination:** Governs registration policy and approval where required.

## 7. Preconditions and Entry Criteria

- An Examination Registration exists for the candidate.
- The affected academic registration is authoritative in its owning domain.
- The relevant Examination Session is active and the applicable examination schedule is established.
- The changed academic context can be related to the candidate's examination registration.
- Applicable examination policy and subject-registration rules are available.
- The registration is in a lifecycle position where amendment is permitted, or an authorised exceptional process exists if it is locked.

## 8. Trigger

An authoritative academic registration change alters the candidate's academic applicability in a way that may affect the scope of an existing Examination Registration.

## 9. Main Business Flow

1. The Examination Registration domain receives or becomes aware of the authoritative academic registration change through established business collaboration.
2. The affected candidate, examination session, registration, and registration items are identified.
3. The changed academic context is evaluated against the current Examination Registration scope.
4. Examination Registration determines whether subjects must be added, removed, corrected, or otherwise reconciled under applicable examination policy.
5. The proposed registration scope change is reviewed and approved where institutional registration rules require approval.
6. The Examination Registration is amended to reflect the authorised examination participation scope.
7. The amendment preserves the previous registration position and the reason for the change in audit history.
8. **Registration Amended** is published by Examination Registration.
9. If the amended registration remains eligible and reaches confirmation under the applicable lifecycle, **Registration Confirmed** is published when confirmation occurs.
10. Downstream examination workflows consume the resulting authoritative registration position.

## 10. Alternate Business Flows

### 10.1 Academic Change Has No Examination Impact

If the academic registration change does not affect examination subject applicability or registration eligibility, the Examination Registration remains unchanged.

### 10.2 Subject Becomes Inapplicable

If an academic change makes a registered examination subject inapplicable, the affected registration item is removed or otherwise amended according to the applicable registration policy.

### 10.3 Subject Becomes Newly Applicable

If an academic change makes an examination subject applicable, addition occurs only where the examination registration policy, registration window, and candidate eligibility permit it.

### 10.4 Multiple Registration Items Affected

Where several registration items are affected, the Examination Registration is reconciled as one governed business outcome while preserving item-level scope.

### 10.5 Change After Registration Locking

If the registration is already locked, any change requires the exceptional institutional approval permitted by L2-I04. No routine amendment is performed after locking.

## 11. Failure, Interruption and Business Recovery

### 11.1 Academic Change Cannot Be Established as Authoritative

If the academic registration change is not authoritative in its owning domain, Examination Registration shall not change its authoritative scope on the basis of an unconfirmed academic position.

### 11.2 Resulting Examination Scope Is Not Eligible

If the changed academic scope does not satisfy examination eligibility or subject applicability, the affected registration item is not added or is subject to the applicable rejection/withdrawal outcome.

### 11.3 Required Registration Approval Is Not Granted

If institutional approval is required and is not granted, the existing authoritative registration remains unchanged until a valid business decision is established.

### 11.4 Registration Is Locked

A locked registration is not routinely modified. The case follows the approved exceptional process defined by institutional registration policy.

### 11.5 Academic Change Is Reversed

If the authoritative academic domain subsequently reverses or corrects the academic registration change, Examination Registration evaluates the new authoritative position through the same governed amendment process. It does not silently restore a prior state.

## 12. Business Rules and Decision Constraints

- Examination Registration must align with applicable academic structure and subject applicability.
- Curriculum ownership remains with the Curriculum Framework domain.
- Student academic information remains owned by the Student domain.
- Registration amendments must follow institutional policy.
- Amendments normally occur before registration locking unless exceptional approval is granted.
- Unauthorised amendments are not permitted.
- Every amendment must preserve complete audit history.
- Only eligible registration subjects may form the authoritative examination scope.
- Examination Registration remains the owner of examination participation even when academic information originates elsewhere.

## 13. State and Lifecycle Impact

The scenario uses the **Registration Amendment** capability of L2-I04.

A permitted academic-driven scope change moves the Examination Registration through the existing amendment lifecycle. The scenario does not introduce a new lifecycle state.

If the registration is confirmed and amended, it may require the applicable downstream confirmation/locking treatment according to institutional policy. If the registration is already locked, only an approved exceptional workflow may change it.

## 14. Business Events and Cross-Domain Collaboration

The Examination Registration domain is the canonical publisher for registration lifecycle events.

Relevant events include:

- **Registration Amended** — when the authorised examination scope is changed.
- **Registration Confirmed** — when the resulting registration becomes operationally valid.
- **Registration Locked** — when the resulting registration is frozen for examination operations.

L2-I04 consumes student academic status and curriculum information from their canonical owners. It does not define a dedicated academic-registration-change event in its event catalogue, so no such event is invented here.

The originating academic domain remains responsible for publishing its own authoritative academic changes.

## 15. Business Authorization, Approval and Audit Obligations

- The academic change must be authoritative in its originating domain before it is used as the basis for an examination registration change.
- Examination Registration amendment must follow applicable institutional policy.
- Required institutional review or approval must be completed where required.
- The amendment and its business basis must be auditable.
- The prior registration scope must remain historically traceable.
- No technical authorization or integration mechanism is specified at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The authoritative academic change has been established.
- Its examination impact has been evaluated.
- A valid resulting registration scope has been determined.
- Required registration approval has been completed where applicable.
- The authoritative Examination Registration reflects the approved scope.
- Audit history preserves the prior and resulting positions.
- **Registration Amended** is published when a change is established.

**Postconditions:**
- Examination Registration accurately reflects current permitted examination participation.
- Downstream examination workflows consume the corrected authoritative scope.
- Academic ownership remains with C/D.
- Registration ownership remains with Examination Registration.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 defines subject registration, curriculum applicability, controlled amendment, and consumption of Student/Curriculum information, but it does not define a dedicated event named for an academic registration change. No new event is introduced.
2. The exact academic-change categories that require examination scope adjustment are institution-dependent; this scenario therefore does not create a universal mapping beyond the authoritative applicability decision.
3. If a locked registration requires change, the exact exceptional authority is governed by institutional policy and is not invented here.
