# Erroneous Examination Registration to Traceable Corrected Registration

**Scenario ID:** L3-I-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-I04 Examination Registration Workflows; applicable Student Domain architecture  
**Referenced By:** Downstream examination registration consumers; None yet for this L3 scenario

## 2. Scenario Purpose

This scenario governs correction of an erroneous Examination Registration while preserving the authoritative business history of the registration and the correction.

The purpose is to ensure that a registration error is resolved through a controlled business correction rather than by obscuring or bypassing the existing registration history.

## 3. Business Outcome

An erroneous Examination Registration is resolved through an authorised, traceable correction, resulting in an authoritative valid registration position with the correction history preserved.

## 4. Scenario Boundary and Scope

**Start Boundary:** An existing Examination Registration is identified as erroneous or inconsistent with the applicable registration requirements.

**In Scope:**
- Identification of the registration error.
- Validation of the correct registration position.
- Controlled correction and approval where required.
- Preservation of historical traceability.
- Publication of the applicable registration amendment outcome.

**Out of Scope:**
- Changes to Student-domain ownership.
- Changes to curriculum or examination policy.
- Hall ticket generation and examination conduct.
- Technical correction mechanisms or data repair.

**End Boundary:** The corrected registration position is authoritative and the history of the error and correction remains traceable.

## 5. Ownership and Participating Domains

**Primary Owner — Examination Registration:** Owns the Examination Registration and the authoritative correction outcome.

**Student (C):** Provides candidate context and relevant student information while retaining ownership of Student records.

**Examination (I):** Governs the registration lifecycle, correction, and authoritative registration position.

## 6. Business Actors, Roles and Responsibilities

- **Candidate (Student):** May identify or report an erroneous registration where permitted.
- **Examination Administrator:** Reviews the registration discrepancy and manages the correction.
- **Controller of Examination:** Provides governance and approval where required by institutional registration policy.
- **Academic Administration:** May provide relevant academic verification where applicable.

## 7. Preconditions and Entry Criteria

- An Examination Registration exists.
- A specific registration error or inconsistency has been identified.
- The candidate and affected registration are identifiable.
- The information required to establish the correct registration position is available.
- Any required institutional authority for correction is available.

## 8. Trigger

An erroneous Examination Registration is identified through candidate, administrative, or institutional review.

## 9. Main Business Flow

1. The erroneous registration is identified and the affected registration context is established.
2. The discrepancy is reviewed against the applicable examination registration requirements.
3. The correct registration position is established using authoritative business information.
4. The correction is reviewed and approved where institutional approval is required.
5. The Examination Registration is amended to the approved correct position.
6. The corrected registration becomes the authoritative business position.
7. The error, correction, and applicable decision history remain traceable through registration audit history.
8. The **Registration Amended** event is published for relevant downstream business consumers.

## 10. Alternate Business Flows

### 10.1 Candidate-Reported Error

Where the candidate identifies the error, the report is evaluated through the institutional registration amendment process.

### 10.2 Administrative Error Detection

Where the error is identified by Examination Administration, the administrator initiates the governed correction while preserving the candidate-centric registration context.

### 10.3 Error Limited to a Registration Subject

Where the error affects only a registered subject, the correction is limited to the affected registration scope while preserving the remainder of the authoritative registration.

### 10.4 Error Requiring Academic Verification

Where establishing the correct registration position requires academic information, the relevant academic domain provides the authoritative information without transferring registration ownership.

## 11. Failure, Interruption and Business Recovery

### 11.1 Correct Registration Position Cannot Be Established

If authoritative information is insufficient to establish the correct registration position, the correction shall not be finalised until the required business information is available.

### 11.2 Correction Rejected

If the proposed correction is rejected under institutional policy or approval requirements, the existing authoritative registration remains unchanged.

### 11.3 Candidate Context Does Not Match

If the proposed correction conflicts with the authoritative candidate context, the correction shall not be established until the discrepancy is resolved through the appropriate business authority.

### 11.4 Locked Registration

If the erroneous registration is locked, correction requires the exceptional institutional approval defined by the registration workflow.

## 12. Business Rules and Decision Constraints

- Registration amendments must follow institutional policy.
- Unauthorised amendments are not permitted.
- Corrections must preserve complete audit history.
- A correction does not transfer ownership of Student or academic information.
- The Examination Registration domain remains the owner of the authoritative registration.
- Where a registration is locked, amendment requires exceptional institutional approval.
- A correction must establish a valid registration position rather than merely overwrite the historical record.
- Significant correction activity must remain auditable and historically traceable.

## 13. State and Lifecycle Impact

The scenario applies to the Examination Registration lifecycle through the controlled **Registration Amendment** activity.

The correction changes the authoritative registration position without introducing a new lifecycle state. Where the registration remains eligible for further processing, it continues through the applicable registration lifecycle.

If the registration is locked, the correction may proceed only through the approved exceptional workflow defined by L2-I04.

## 14. Business Events and Cross-Domain Collaboration

The Examination Registration domain is the canonical publisher for registration lifecycle events.

Material event:

- **Registration Amended** — published when the corrected registration modification is approved and established.

Subsequent registration lifecycle events may include:

- **Registration Confirmed** — Examination Registration.
- **Registration Locked** — Examination Registration.

The Student domain provides authoritative candidate context where required. Its information ownership remains unchanged.

## 15. Business Authorization, Approval and Audit Obligations

- The correction must follow the institutional registration amendment and approval requirements.
- Where an exceptional correction is required after locking, appropriate institutional authority must be established.
- The business correction and its justification must remain traceable.
- Significant registration correction activity must generate audit records.
- Historical traceability must be preserved so that the erroneous position and resulting correction remain accountable.
- No technical access control, database correction, or implementation mechanism is defined at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The erroneous registration has been evaluated.
- The correct registration position has been established from authoritative business information.
- Required approval has been obtained where applicable.
- The corrected registration is authoritative.
- The correction history is auditable and traceable.
- The applicable registration amendment event has been published.

**Postconditions:**
- The candidate's Examination Registration reflects the approved valid position.
- Historical evidence of the correction remains available.
- Downstream examination workflows can consume the corrected authoritative registration subject to their own prerequisites.
- Ownership boundaries remain unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 does not define a separate lifecycle state specifically for an erroneous registration; this scenario therefore uses the existing Registration Amendment activity.
2. The exact institutional authority for each correction type is policy-dependent and is not invented here.
3. The source defines **Registration Amended** as the canonical event for an approved registration modification; no separate error-correction event is introduced.
