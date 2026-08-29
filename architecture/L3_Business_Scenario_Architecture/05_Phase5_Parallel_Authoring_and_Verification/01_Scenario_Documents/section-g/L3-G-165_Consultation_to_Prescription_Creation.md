# Consultation to Prescription Creation

**Scenario ID:** L3-G-165  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-165 |
| Canonical Title | Consultation to Prescription Creation |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete healthcare outcome in which a consultation establishes that prescription treatment is required, the authorised medical professional creates the prescription associated with the Medical Encounter, and the prescription becomes available for the subsequent medicine-dispensing workflow.

The locked L2 defines Prescription as a G06 aggregate and identifies Prescription Management as an owned healthcare capability. Prescription details include prescribed medicines, dosage, duration and instructions. The Medical Encounter owns consultation, diagnosis, treatment and follow-up recommendation.

## 3. Business Outcome

An authorised healthcare professional has completed the relevant consultation decision, determined that a prescription is required, and created a valid prescription belonging to the Medical Encounter.

Medicine dispensing is deliberately outside this scenario and begins under the separate Prescription-to-Medicine-Dispensing scenario.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A healthcare consultation has reached the treatment decision and prescription is determined to be required.

### 4.2 End Boundary

A valid Prescription has been created and is available for the subsequent dispensing workflow.

### 4.3 In Scope

- Consultation context.
- Diagnosis/treatment context necessary for prescription.
- Determination that prescription is required.
- Prescribed medicines.
- Dosage.
- Duration.
- Instructions.
- Authorised medical professional.
- Association to Medical Encounter.
- Prescription creation and audit.

### 4.4 Out of Scope

- Medicine inventory management.
- Medicine dispensing.
- Pharmacy ERP operations.
- Financial accounting.
- External pharmacy settlement.
- Clinical treatment beyond the prescription decision.
- Technical prescription generation/storage.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns Medical Encounter and Prescription.

The L2 states that every Prescription belongs to one Medical Encounter.

### Section E — People & Workforce

E remains authoritative for Doctors, Nurses, Medical Officers, Healthcare Staff and Employee Information. G06 uses the professional context operationally but does not own employee identity.

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional / Doctor

Performs clinical judgement and creates the prescription when medically required.

### Medical Staff

Supports operational recording within configured healthcare responsibilities.

### Section E

Provides authoritative healthcare-professional/personnel context.

No technical authorization role is invented.

## 7. Preconditions and Entry Criteria

1. A Medical Encounter exists.
2. The consultation has reached treatment decision.
3. The patient is correctly associated with the encounter.
4. An authorised medical professional is conducting the consultation.
5. The professional determines that prescription treatment is required.
6. Required prescription information can be established.

## 8. Trigger

Clinical consultation determines that a prescription is required.

## 9. Main Business Flow

### Phase A — Clinical Decision

1. Identify the patient and Medical Encounter.
2. Confirm the responsible healthcare professional.
3. Complete the relevant clinical assessment, diagnosis and treatment decision.
4. Determine that prescription treatment is required.
5. Establish the medicines and treatment instructions under professional clinical judgement.

The L2 makes clear that medical professionals retain operational authority and the ERP records/governs rather than replacing clinical judgement.

### Phase B — Prescription Definition

6. Establish the prescribed medicines.
7. Establish dosage.
8. Establish duration.
9. Establish instructions.
10. Associate the prescription with exactly one Medical Encounter.
11. Associate the responsible medical professional.

### Phase C — Prescription Creation

12. Create the Prescription.
13. Validate that required prescription information is complete.
14. Preserve the prescription as part of the Medical Encounter's healthcare history.
15. Record the business audit evidence.

### Phase D — Downstream Handoff

16. Make the created prescription available to the subsequent medicine-dispensing workflow.
17. Do not dispense medicine within this scenario.
18. End the scenario.

The L2 separates Prescription Management from Medicine Dispensing, and Medicine Dispense must reference a valid Prescription.

## 10. Alternate Business Flows

### 10.1 No Prescription Required

The consultation completes without creating a Prescription.

### 10.2 Multiple Medicines

A single prescription may contain the medicines determined necessary by the authorised professional. The scenario does not impose an unsupported maximum.

### 10.3 Existing Medicine Constraints

If a prescription cannot be safely established under applicable healthcare policy, the medical professional determines the appropriate alternative clinical action.

### 10.4 Student Patient

C remains authoritative for student identity; G06 owns the prescription.

### 10.5 Employee Patient

E remains authoritative for employee/personnel context; G06 owns the prescription.

### 10.6 Follow-up Also Recommended

Follow-up is recorded as part of the Medical Encounter and proceeds through the separate follow-up scenario.

## 11. Failure, Interruption and Business Recovery

### 11.1 Patient/Encounter Association Invalid

No prescription is created against an invalid or invented Medical Encounter.

### 11.2 Professional Authority Cannot Be Established

Prescription creation does not proceed without the applicable business authority.

### 11.3 Required Prescription Information Incomplete

The prescription remains unresolved until the responsible professional supplies the required information.

### 11.4 Clinical Decision Changes Before Creation

The prescription is not created until the authorised professional establishes the final clinical decision.

### 11.5 Prescription Cannot Be Safely Established

The consultation follows the applicable alternative medical action; this scenario does not invent a prescription.

### 11.6 Recovery

Once the required clinical/business condition is established, prescription creation resumes from the unresolved business boundary.

No technical retry, queue, rollback, API, database or infrastructure behaviour is described.

## 12. Business Rules and Decision Constraints

The locked L2 establishes:

- Prescription is owned by G06.
- Every Prescription belongs to one Medical Encounter.
- Prescription includes prescribed medicines, dosage, duration and instructions.
- Medicine Dispense must reference one valid Prescription.
- Medical professionals retain clinical authority.
- Finance owns billing/accounting; G06 does not perform financial accounting.

The scenario does not invent dosage rules, drug-selection rules, clinical contraindication rules or prescription validity periods.

## 13. State and Lifecycle Impact

The L2 defines the Prescription lifecycle:

`Created → Issued → Dispensed → Completed`.

This scenario establishes the prescription at its creation/issuance boundary as supported by the business workflow.

The subsequent `Dispensed` and `Completed` outcomes belong to the medicine-dispensing lifecycle and are outside this scenario.

The Medical Encounter itself follows:

`Registered → Consultation → Treatment → Completed → Closed`.

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical owner/publisher for Prescription business facts because it owns the Prescription aggregate.

The retrieved authoritative content establishes that significant healthcare activities publish immutable domain events.

The exact canonical prescription-creation event name is not exposed in the retrieved authoritative excerpt, so no new event name is fabricated.

E participates by providing authoritative healthcare-professional/personnel context; it does not publish or own the G06 Prescription outcome.

## 15. Business Authorization, Approval and Audit Obligations

Prescription creation must be performed by an appropriately authorised medical professional under the configurable healthcare authorization model.

Medical confidentiality and need-to-know obligations apply.

Audit evidence should preserve:

- Medical Encounter.
- Patient context.
- Responsible medical professional.
- Prescription details.
- Medicines.
- Dosage.
- Duration.
- Instructions.
- Prescription creation outcome.
- Relevant lifecycle transition.

Every healthcare lifecycle transition is required to be audited.

No technical access control/IAM mechanism is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Medical Encounter identified.
2. Clinical treatment decision completed.
3. Prescription requirement established.
4. Authorised medical professional established.
5. Medicines, dosage, duration and instructions recorded.
6. Prescription associated with exactly one Medical Encounter.
7. Prescription created.
8. Audit evidence preserved.
9. Prescription is available for downstream dispensing.

### Postconditions

- A valid G06 Prescription exists.
- The Prescription remains associated with its Medical Encounter.
- No medicine has been represented as dispensed merely because the prescription was created.
- Employee/personnel ownership remains with E.
- Finance remains owner of billing/accounting.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 does not expose a detailed prescription clinical decision matrix.
2. The exact canonical PrescriptionCreated event name is not exposed in the retrieved authoritative excerpt.
3. The precise boundary between `Created` and `Issued` requires verification against the complete state/contract definition if the distinction is material.

### Assumptions

- G06 owns Prescription.
- E remains authoritative for healthcare-professional/personnel identity.
- Medicine dispensing is a separate downstream business scenario.
- Clinical judgement remains with the authorised medical professional.

### Deferred Questions

Verify the exact canonical prescription event and the authoritative `Created`/`Issued` transition semantics against the complete locked L2 event/state definitions before Architecture Approval/Lock.
