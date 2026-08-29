# Registration Amendment to Authoritative Corrected Registration

**Scenario ID:** L3-I-021  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, D, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-I04 Examination Registration Workflows; applicable Student, Curriculum Framework, and Academic Delivery architecture  
**Referenced By:** Downstream examination registration consumers; None yet for this L3 scenario

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-021 |
| Canonical Title | Registration Amendment to Authoritative Corrected Registration |
| Primary Owner | Examination Registration |
| Participating Domains | C, D, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs a controlled amendment to an Examination Registration when an approved or pending registration requires a valid business correction.

The purpose is to establish a corrected authoritative registration without bypassing Examination Registration lifecycle governance, applicable policy requirements, or audit obligations.

## 3. Business Outcome

A permitted registration amendment is reviewed and established as an authoritative corrected Examination Registration, with the amended registration remaining valid for downstream examination processing.

## 4. Scenario Boundary and Scope

**Start Boundary:** An existing Examination Registration requires a permitted amendment to its examination subjects, category, or other registration information.

**In Scope:**
- Identification of the amendment requirement.
- Validation of the requested amendment.
- Review and approval where required.
- Establishment of the corrected registration.
- Preservation of registration history.
- Publication of the applicable registration lifecycle change.

**Out of Scope:**
- Changes to curriculum ownership or academic policy.
- Changes to the candidate's authoritative Student record.
- Hall ticket generation or examination conduct.
- Technical implementation of amendment processing.

**End Boundary:** The corrected Examination Registration is authoritative and available for subsequent governed examination processing, or the amendment is rejected without changing the authoritative registration.

## 5. Ownership and Participating Domains

**Primary Owner — Examination Registration:** Owns the authoritative Examination Registration and amendment outcome.

**Student (C):** Provides candidate academic context where relevant; it does not own the Examination Registration.

**Academic Delivery (D):** Provides applicable academic or subject context where relevant; it does not own the Examination Registration.

**Examination (I):** Governs registration amendment and preserves registration authority.

## 6. Business Actors, Roles and Responsibilities

- **Candidate (Student):** May initiate or request a permitted registration amendment where institutional policy allows.
- **Examination Administrator:** Manages registration amendment activities and administrative verification.
- **Department Administration:** Verifies departmental registration requirements where applicable.
- **Academic Administration:** Validates academic eligibility or applicability where applicable.
- **Controller of Examination:** Governs registration policies and approvals where institutional approval is required.

## 7. Preconditions and Entry Criteria

- An Examination Registration exists in a business state in which amendment is permitted.
- The candidate and registration context are identifiable.
- The requested amendment is permitted by institutional registration policy.
- Applicable academic and examination information needed to evaluate the amendment is available.
- The registration has not been locked, unless an approved exceptional process permits the amendment.

## 8. Trigger

A permitted change, correction, or administrative requirement is identified for an existing Examination Registration.

## 9. Main Business Flow

1. The amendment requirement is identified against the existing Examination Registration.
2. The requested change is established in the context of the candidate, examination session, and registered subjects or category.
3. Applicable registration, academic, and policy requirements are reviewed.
4. The amendment is reviewed and approved where institutional approval is required.
5. The Examination Registration is amended in accordance with the approved business decision.
6. The corrected registration becomes the authoritative registration position.
7. The amendment is preserved in the registration audit history.
8. The applicable registration amendment event is published for downstream business consumers.

## 10. Alternate Business Flows

### 10.1 Candidate-Initiated Amendment

Where policy permits candidate action, the candidate requests the amendment and the registration is subjected to the same validation and approval requirements.

### 10.2 Administrative Amendment

Where an institutional correction is required, the Examination Administrator manages the amendment while preserving the candidate-centric registration context.

### 10.3 Amendment Before Registration Locking

Where the registration is not yet locked, a permitted amendment may proceed through the normal controlled amendment lifecycle.

### 10.4 Approved Exceptional Amendment After Locking

Where an amendment after locking is permitted through exceptional institutional approval, the amendment follows that governed exception rather than bypassing registration governance.

## 11. Failure, Interruption and Business Recovery

### 11.1 Amendment Not Permitted

If the requested amendment is outside applicable institutional policy, the authoritative registration remains unchanged.

### 11.2 Validation Failure

If subject applicability, examination category, academic requirements, or another applicable registration condition is not satisfied, the amendment is not established.

### 11.3 Approval Rejected

Where approval is required and is rejected, the existing authoritative registration remains unchanged.

### 11.4 Amendment After Locking Without Exceptional Approval

A locked registration shall not be amended without the required exceptional institutional approval.

### 11.5 Correction of an Amendment Record

If a subsequently identified registration error requires correction, the corrected business position shall preserve traceability rather than erasing the historical amendment.

## 12. Business Rules and Decision Constraints

- Registration amendments must follow institutional policy.
- Amendments normally occur before registration locking unless exceptional approval is granted.
- Subject changes must remain consistent with programme, curriculum, academic period, examination category, and examination policy requirements.
- Unauthorised amendments are not permitted.
- Every amendment shall preserve complete audit history.
- Ownership of student and academic information remains with the originating domains.
- The corrected registration remains owned by Examination Registration.
- An amendment does not bypass required registration approval or confirmation requirements.

## 13. State and Lifecycle Impact

The scenario applies to the Examination Registration lifecycle through the **Registration Amendment** activity defined by L2-I04.

A permitted amendment changes the authoritative registration content while preserving the governed registration lifecycle. The scenario does not introduce a new lifecycle state.

Where required, the amended registration continues through the applicable review, approval, confirmation, and locking stages. If the amendment is rejected, the existing authoritative registration remains in force.

## 14. Business Events and Cross-Domain Collaboration

The Examination Registration domain is the canonical publisher for registration lifecycle events.

Material event:

- **Registration Amended** — published when a registration modification is approved and established.

Other lifecycle events may apply when the amended registration subsequently progresses:

- **Registration Approved** — Examination Registration.
- **Registration Confirmed** — Examination Registration.
- **Registration Locked** — Examination Registration.

Student and Academic Delivery domains provide applicable business information while retaining ownership of their respective information.

## 15. Business Authorization, Approval and Audit Obligations

- Amendments must follow institutional registration policy.
- Where approval is required, the amendment must receive the applicable institutional approval before becoming authoritative.
- Administrative corrections must remain attributable to the authorised institutional process.
- Every significant amendment must generate audit records.
- The audit history must preserve historical traceability and accountability.
- No technical access control or access-control mechanism is defined at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The requested amendment has been validated against applicable business requirements.
- Required approval has been obtained where applicable.
- The corrected Examination Registration is established as authoritative.
- The amendment is auditable.
- The applicable registration amendment event is published.

**Postconditions:**
- The authoritative registration reflects the approved correction.
- Downstream consumers may use the corrected registration subject to their own business prerequisites.
- The prior registration history remains traceable.
- No student, curriculum, or academic ownership is transferred.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 supports amendment after locking only through approved exceptional workflows but does not define a separate amendment-after-locking lifecycle state. No new state is introduced here.
2. The exact approval authority for each amendment category is institution-specific and governed by applicable policy; this scenario does not invent a new approval role.
3. L2-I04 identifies the **Registration Amended** event but does not define additional amendment-specific events; no additional event is introduced.
