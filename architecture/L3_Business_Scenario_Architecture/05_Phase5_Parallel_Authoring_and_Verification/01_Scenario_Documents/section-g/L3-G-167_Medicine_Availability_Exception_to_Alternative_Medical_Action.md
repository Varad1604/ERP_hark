# Medicine Availability Exception to Alternative Medical Action

**Scenario ID:** L3-G-167  
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

This scenario governs the complete institutional outcome when a prescribed medicine cannot be dispensed as ordinarily intended because the medicine is unavailable or otherwise cannot be issued through institutional medicine operations, and an authorised healthcare professional establishes an alternative medical action.

The locked L2 owns Prescription and Medicine Dispensing within G06, while also defining lightweight medicine inventory inside the healthcare domain. It does not turn the availability exception into an enterprise procurement or pharmacy-management process. fileciteturn273file0

The scenario therefore protects the patient from a false dispensing outcome while keeping the clinical alternative under authorised medical judgement.

## 3. Business Outcome

The prescribed medicine availability exception has been identified, the unavailable medicine is not falsely recorded as dispensed, and an authorised healthcare professional has established the applicable alternative medical action or other governed outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A valid Prescription reaches dispensing and one or more prescribed medicines cannot be dispensed as intended.

### 4.2 End Boundary

An authorised alternative medical action is established and recorded, or the case remains explicitly unresolved under the applicable healthcare policy.

### 4.3 In Scope

- Prescription identification.
- Medicine availability exception.
- Confirmation that dispensing cannot proceed as intended.
- Clinical/business review of alternatives.
- Alternative medical action.
- Patient/Medical Encounter continuity.
- Recording of the exception and outcome.
- Follow-up to the applicable healthcare workflow.
- Audit.
- Configured notification handoff.

### 4.4 Out of Scope

- Enterprise pharmacy procurement.
- Supplier management.
- Clinical diagnosis.
- Unauthorised medicine substitution.
- Technical inventory implementation.
- Financial accounting.
- External pharmacy operations.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

Owns Prescription, Medicine Dispense, medicine availability within its lightweight medicine operations, and the resulting healthcare action.

### Section E — People & Workforce

Provides authoritative healthcare-professional/personnel context. E does not own the medical decision.

### Section H — Notifications

May communicate configured healthcare outcomes but does not own the clinical/dispensing decision.

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Determines the medically appropriate alternative action when the prescribed medicine cannot be dispensed.

### Medical Staff / Dispensing Staff

Identifies the availability exception and records the business outcome.

### Patient

Participates in the alternative-care decision as appropriate.

### E

Provides authoritative professional context.

### H

Provides configured notification delivery.

No technical actor is invented.

## 7. Preconditions and Entry Criteria

1. A valid Prescription exists.
2. The Prescription belongs to a valid Medical Encounter.
3. Medicine dispensing has been attempted or assessed.
4. A prescribed medicine cannot be issued as intended.
5. The patient remains within the G06 healthcare context.
6. An authorised medical professional can establish the alternative medical action.

## 8. Trigger

A prescribed medicine is unavailable or otherwise cannot be dispensed as intended.

## 9. Main Business Flow

### Phase A — Establish the Availability Exception

1. Identify the Prescription.
2. Identify the affected prescribed medicine.
3. Confirm the associated Medical Encounter and patient.
4. Establish that the medicine cannot be dispensed as prescribed.
5. Record the availability exception.

### Phase B — Clinical Review

6. Refer the exception to the authorised medical professional.
7. Review the clinical context and existing prescription.
8. Determine the appropriate alternative medical action.
9. Ensure the alternative is established by the responsible healthcare authority rather than by dispensing staff acting independently.

### Phase C — Alternative Action

10. If an alternative treatment/medicine is clinically appropriate and permitted by policy, establish the alternative action.
11. If no acceptable alternative is available, establish the applicable unresolved or follow-up healthcare outcome.
12. Preserve the relationship to the original Medical Encounter and Prescription.
13. Do not represent the unavailable medicine as dispensed.

### Phase D — Record and Continue

14. Record the exception and resulting alternative medical action in healthcare history.
15. Where the alternative involves a new or revised prescription, continue through the authoritative Prescription workflow.
16. Where follow-up or referral is required, route to the applicable healthcare workflow.
17. Where configured, provide the resulting business fact to H for notification.

### Phase E — Complete

18. End when the alternative medical action or explicit unresolved outcome is established and recorded.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Alternative medicine/action available | Authorised professional identifies an acceptable alternative | Establish the alternative healthcare action | Patient receives a governed alternative path | G |
| No acceptable alternative | Clinical judgement does not support an alternative | Preserve unresolved/next-care requirement and follow applicable healthcare policy | No unsafe or fabricated dispensing outcome | G |
| Follow-up consultation required | Alternative requires further assessment | Route to applicable consultation/follow-up workflow | Further care remains governed | G |
| External referral required | Patient needs care beyond institutional scope | Route to referral workflow | External care remains externally owned | G |

The scenario does not define a local drug-substitution rule.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Availability exception not confirmed | No alternative action should be established on an unverified shortage | Confirm the business availability condition | G |
| Authorised medical professional unavailable | Alternative clinical decision cannot be completed | Preserve the exception and continue when appropriate medical authority is available | G/E |
| No clinically acceptable alternative | Original prescription cannot be fulfilled as intended | Record the unresolved outcome and follow the applicable healthcare path | G |
| Patient context cannot be confirmed | Alternative action cannot safely be associated | Resolve authoritative patient context before recording the action | G |
| Alternative action requires another workflow | Current scenario cannot complete that downstream lifecycle | Hand off to the owning healthcare workflow | G |

No technical retry, queue, rollback, API, database or infrastructure recovery is described.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Prescription is owned by G06 | G06 | Availability exception remains within healthcare ownership | L2-G06 |
| Every Prescription belongs to one Medical Encounter | G06 | Exception remains associated with the correct encounter | L2-G06 Business Invariants |
| Medicine Dispense references one valid Prescription | G06 | Unavailable medicine cannot be represented as a valid dispense | L2-G06 Business Invariants |
| Medical professionals retain operational authority | G06 | Alternative medical action must be clinically authorised | L2-G06 Design Principles / Authorization |
| Lightweight medicine inventory remains within G06 | G06 | Availability is assessed within the institutional medicine boundary | L2-G06 Medicine boundary |
| Enterprise pharmacy management remains outside G06 | Boundary | Exception cannot silently become procurement/pharmacy management | L2-G06 Scope |

No local substitution or therapeutic-equivalence rule is created.

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Prescription | Issued / dispensing-ready position | Required medicine cannot be dispensed as intended | Existing lifecycle continues only through an authorised alternative path | G06 | L2-G06 |
| Medicine Dispense | Not yet validly completed for affected medicine | Prevents false dispensing record | No dispense for unavailable medicine unless a valid alternative dispensing outcome is separately established | G06 | L2-G06 |
| Medical Encounter | Treatment / follow-up context | Receives the availability exception and alternative action | Healthcare history extended | G06 | L2-G06 |

The scenario does not create a new `Unavailable` Prescription state.

The Prescription lifecycle remains:

`Created → Issued → Dispensed → Completed`. fileciteturn273file0

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of healthcare facts concerning Prescription and Medicine Dispense because it owns those aggregates.

The locked L2 requires significant healthcare activities to publish immutable domain events, but the exact canonical event name for a medicine-availability exception is not exposed in the retrieved authoritative event definition.

Therefore no `MedicineUnavailable`, `MedicineAvailabilityException`, or similar event is invented.

H may consume the appropriate G06 event where configured for notification.

E supplies healthcare-professional/personnel context and does not acquire ownership of the medical decision.

## 15. Business Authorization, Approval and Audit Obligations

The alternative medical action must be established by an appropriately authorised healthcare professional.

The exception history should preserve:

- Original Prescription.
- Affected medicine.
- Availability condition.
- Responsible medical professional.
- Alternative action.
- Patient/Medical Encounter.
- Resulting healthcare outcome.
- Relevant lifecycle transition.

Medical confidentiality and need-to-know obligations apply.

Every healthcare lifecycle transition must be audited. fileciteturn273file0

No technical access-control implementation is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Valid Prescription identified.
2. Affected medicine identified.
3. Availability exception established.
4. Patient and Medical Encounter confirmed.
5. Authorised medical professional reviews the exception.
6. Alternative medical action or explicit unresolved outcome established.
7. No unavailable medicine falsely recorded as dispensed.
8. Resulting healthcare history recorded.
9. Any downstream workflow is handed off to its authoritative owner.
10. Audit evidence preserved.

### Postconditions

- The availability exception is traceable.
- The original prescription remains historically accurate.
- No false medicine-dispensing fact exists.
- Alternative healthcare action is governed by G06.
- Any new prescription/referral/follow-up remains under its appropriate G06 lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 does not define a universal medicine-substitution or therapeutic-equivalence policy.
2. The exact terminal business outcome for an unresolved availability exception is not enumerated.
3. The canonical event name for medicine availability exception is not exposed in the retrieved event definitions.

### Assumptions

- Clinical alternatives are established only by authorised medical professionals.
- G06 remains owner of the healthcare outcome.
- Procurement/vendor processes are not silently invoked as part of this scenario.

### Deferred Questions

Confirm the institution-specific medicine-substitution/alternative-action policy and canonical availability-exception event before Architecture Approval/Lock.
