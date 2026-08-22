# Prescription to Medicine Dispensing

**Scenario ID:** L3-G-166  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the complete institutional outcome in which an authorised prescription is presented for fulfilment, the prescribed medicines are assessed for institutional availability and dispensing eligibility, the medicines that can be issued are dispensed, and the resulting dispensing outcome is recorded against the healthcare history.

The locked L2 places Prescription Management and Medicine Dispensing within Institutional Health Services. It defines `Prescription` and `Medicine Dispense` as G06-owned aggregates and states that every Medicine Dispense references one valid Prescription. Prescription information includes prescribed medicines, dosage, duration and instructions; Medicine Dispense includes dispensed medicines, quantity, dispensing date, dispensing professional and medicine issue history. fileciteturn273file0

The scenario does not turn G06 into an enterprise pharmacy system and does not transfer ownership to Finance, E, H, or an external provider.

## 3. Business Outcome

A valid prescription has been evaluated for institutional dispensing, the medicines that are authorised and available have been dispensed according to the prescription and applicable healthcare policy, and the dispensing outcome has been recorded as part of the patient's healthcare history.

Where a prescribed medicine cannot be dispensed, the case is handled through the separate **Medicine Availability Exception to Alternative Medical Action** scenario rather than falsely recording a completed dispense.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A valid G06 Prescription is available for medicine dispensing.

### 4.2 End Boundary

The dispensing decision and resulting medicine issue are recorded, with the Prescription/Medicine Dispense lifecycle positioned for its next authoritative state.

### 4.3 In Scope

- Prescription identification and validation.
- Patient/Medical Encounter association.
- Prescription medicine review.
- Medicine availability assessment within institutional medicine operations.
- Dispensing eligibility.
- Medicine issue.
- Quantity recorded.
- Dispensing date recorded.
- Dispensing professional recorded.
- Medicine issue history.
- Prescription lifecycle progression.
- Healthcare audit history.
- Configured notification handoff where applicable.

### 4.4 Out of Scope

- Prescription creation.
- Clinical diagnosis or treatment decision.
- Enterprise pharmacy management.
- External pharmacy inventory.
- Financial accounting or payment collection.
- Supplier procurement.
- Technical inventory implementation.
- Technical notification delivery.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns:

- Prescription.
- Medicine Dispense.
- Healthcare operational history.
- Medicine dispensing workflow.

The L2 states that Medicine Dispense is a G06 aggregate and that lightweight medicine inventory remains within the domain, while enterprise pharmacy management remains outside scope. fileciteturn273file0

### Section E — People & Workforce

E remains authoritative for employee/personnel information, including healthcare staff. G06 consumes professional context without acquiring employee ownership.

### Section H — Notifications

H may deliver configured notifications concerning healthcare outcomes. H does not own Prescription or Medicine Dispense.

## 6. Business Actors, Roles and Responsibilities

### Medical Staff / Authorised Dispensing Professional

Reviews the valid prescription and performs the business act of medicine dispensing within the applicable healthcare authority.

### Patient

Receives the prescribed medicine where dispensing is approved and available.

### Section E

Provides authoritative healthcare-professional/personnel context.

### Section H

Provides notification delivery when configured.

No technical access control/technical identity profiles is invented.

## 7. Preconditions and Entry Criteria

1. A Prescription exists.
2. The Prescription belongs to exactly one Medical Encounter.
3. The prescription is at an appropriate lifecycle position for dispensing.
4. The prescribed medicines and relevant dosage/duration/instructions are available.
5. The patient is sufficiently identified.
6. An authorised dispensing professional is available.
7. Applicable medicine-availability and dispensing policy is established.

## 8. Trigger

A valid Prescription becomes ready for institutional medicine dispensing.

## 9. Main Business Flow

### Phase A — Prescription Validation

1. Identify the Prescription.
2. Identify the associated Medical Encounter and patient.
3. Confirm that the Prescription belongs to exactly one Medical Encounter.
4. Confirm the prescription is at the appropriate business lifecycle position.
5. Review the prescribed medicines.
6. Review dosage, duration and instructions.

### Phase B — Dispensing Assessment

7. Determine whether the prescribed medicines are available through institutional medicine operations.
8. Determine the quantity that may be dispensed under the prescription and applicable healthcare policy.
9. Identify any medicine-specific availability restriction that prevents ordinary dispensing.
10. If the prescribed medicines are available and the dispensing conditions are satisfied, proceed to issue.

### Phase C — Medicine Dispensing

11. Dispense the applicable prescribed medicines.
12. Record the dispensed medicine(s).
13. Record quantity.
14. Record dispensing date.
15. Record dispensing professional.
16. Preserve the medicine issue history.
17. Associate the Medicine Dispense with the valid Prescription.

The L2 explicitly requires every Medicine Dispense to reference one valid Prescription. fileciteturn273file0

### Phase D — Prescription Lifecycle Progression

18. Advance the Prescription through its authoritative dispensing lifecycle.
19. Do not represent a prescription as dispensed merely because it was created.
20. Preserve the dispensing outcome in healthcare history.
21. Where configured, expose the completed business fact to H for notification.

### Phase E — Complete

22. End when the dispensing outcome has been recorded.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Partial institutional availability | Some prescribed medicines are available and others are not | Dispense only what the applicable policy permits and route unavailable medicine(s) to the availability-exception path | Dispensing outcome remains truthful and traceable | G |
| No medicine available | Required medicine cannot be issued through institutional stock | Enter Medicine Availability Exception to Alternative Medical Action | No false dispense is recorded | G |
| Patient declines available medicine | Patient does not accept the offered medicine | Record the applicable business outcome without representing an unaccepted issue as completed dispensing | Accurate dispensing history | G |
| Follow-up required | Dispensing outcome requires further healthcare action | Preserve the outcome and route to the applicable healthcare workflow | Follow-up remains separately governed | G |

No unsupported partial-dispensing policy is treated as mandatory; institution-specific policy remains authoritative.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Prescription invalid for dispensing | Medicine cannot be dispensed against the invalid prescription | Resolve the prescription condition through the appropriate healthcare workflow | G |
| Prescription not associated with a valid Medical Encounter | Dispensing cannot be completed | Establish the authoritative encounter association before dispensing | G |
| Medicine unavailable | Required medicine cannot be issued | Route to Medicine Availability Exception to Alternative Medical Action | G |
| Required dispensing authority unavailable | Dispensing cannot be completed by an unauthorised actor | Resume when an authorised dispensing professional can perform the business action | G/E |
| Dispensing outcome cannot be established | Healthcare history must not claim an unconfirmed issue | Preserve the unresolved business condition and complete only after authoritative evidence exists | G |

Technical retries, queues, rollback, server failures, business exception workflows and infrastructure recovery are not described.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Medicine Dispense references one valid Prescription | G06 | Dispensing cannot be recorded against an unrelated prescription | L2-G06 Business Invariants |
| Every Prescription belongs to one Medical Encounter | G06 | Prescription and dispensing remain tied to one healthcare encounter | L2-G06 Business Invariants |
| Prescription contains prescribed medicines, dosage, duration and instructions | G06 | Dispensing must respect the authoritative prescription information | L2-G06 Prescription aggregate |
| Lightweight medicine inventory remains within G06 | G06 | Institutional medicine availability may be assessed within G06 | L2-G06 Medicine Dispense / Inventory boundary |
| Enterprise pharmacy management remains outside G06 | External boundary | Scenario must not introduce pharmacy ownership | L2-G06 scope boundary |
| Medical history shall not be deleted | G06 | Dispensing history remains traceable | L2-G06 Business Invariants |

The scenario does not invent drug-selection, dosage, substitution, or clinical-equivalence rules.

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Prescription | Created / Issued, as applicable to the authoritative lifecycle | Makes the prescription available for fulfilment and records dispensing | Dispensed / subsequent lifecycle position | G06 | L2-G06 |
| Medicine Dispense | Business creation at dispensing | Records the actual medicine issue | Dispensing outcome recorded | G06 | L2-G06 |
| Medical Encounter | Registered / Consultation / Treatment context | Receives the healthcare dispensing history | Existing encounter history extended | G06 | L2-G06 |

The L2 defines the Prescription lifecycle as:

`Created → Issued → Dispensed → Completed`.

This scenario does not invent an additional Prescription state. fileciteturn273file0

## 14. Business Events and Cross-Domain Collaboration

G06 remains the canonical publisher of Prescription and Medicine Dispense business facts because it owns both aggregates.

The locked L2 establishes that significant healthcare activities publish immutable domain events. The exact medicine-dispensing event name is not exposed in the authoritative source content retrieved for this drafting pass.

Therefore this scenario deliberately does **not** invent a `MedicineDispensed` or similarly named event.

H may consume the applicable G06 healthcare event for notification delivery where configured.

E provides authoritative healthcare-professional/personnel context and does not become owner or publisher of the G06 medicine-dispensing outcome.

## 15. Business Authorization, Approval and Audit Obligations

Medicine dispensing must be performed by an appropriately authorised healthcare actor under the configurable healthcare authorization model.

Audit evidence must preserve, as applicable:

- Prescription reference.
- Medical Encounter reference.
- Patient context.
- Prescribed medicine(s).
- Dispensed medicine(s).
- Quantity.
- Dispensing date.
- Dispensing professional.
- Dispensing outcome.
- Relevant lifecycle transition.

Medical confidentiality and need-to-know obligations apply.

Every healthcare lifecycle transition must be audited. fileciteturn273file0

No technical access control/IAM mechanism is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Valid Prescription identified.
2. Patient and Medical Encounter association confirmed.
3. Prescribed medicine(s) reviewed.
4. Availability/dispensing conditions evaluated.
5. Authorised dispensing professional established.
6. Medicine(s) dispensed where permitted.
7. Quantity, date and professional recorded.
8. Medicine Dispense linked to the valid Prescription.
9. Prescription lifecycle progressed according to the authoritative lifecycle.
10. Healthcare history/audit evidence preserved.

### Postconditions

- A truthful Medicine Dispense outcome exists where dispensing occurred.
- No unavailable medicine is falsely represented as dispensed.
- Prescription remains owned by G06.
- Medicine Dispense remains owned by G06.
- Employee/personnel ownership remains with E.
- Notification delivery, where configured, remains H-owned.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The exact canonical medicine-dispensing event name is not exposed in the retrieved L2 event definitions.
2. The L2 does not enumerate a universal partial-dispensing/substitution policy.
3. The precise business boundary between Prescription `Issued` and `Dispensed` requires verification against the complete locked state/contract definitions.

### Assumptions

- G06 owns the complete institutional medicine-dispensing outcome.
- Medicine availability is assessed against G06's lightweight medicine operations.
- Clinical substitution decisions remain subject to authorised healthcare judgement/policy.

### Deferred Questions

Verify the exact canonical dispensing event and authoritative partial-dispensing/substitution rules before Architecture Approval/Lock.
