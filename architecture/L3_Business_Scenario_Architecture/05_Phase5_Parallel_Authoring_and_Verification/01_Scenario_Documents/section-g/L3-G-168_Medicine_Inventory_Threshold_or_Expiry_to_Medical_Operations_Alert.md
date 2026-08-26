# Medicine Inventory Threshold or Expiry to Medical Operations Alert

**Scenario ID:** L3-G-168  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-168 |
| Canonical Title | Medicine Inventory Threshold or Expiry to Medical Operations Alert |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the operational healthcare outcome in which an institutional medicine inventory condition reaches an applicable threshold or expiry-related condition, the condition is recognised as relevant to medical operations, and an operational alert is established so responsible healthcare operations can take appropriate business action.

The locked L2 explicitly states that lightweight medicine inventory remains within Institutional Health Services while enterprise pharmacy management remains outside scope. It also states that healthcare operations are configuration driven and that significant healthcare activity is auditable.

This scenario therefore addresses **medical operational awareness and action**, not procurement or enterprise inventory management.

## 3. Business Outcome

A material medicine inventory threshold or expiry condition has been identified, an operational medical alert has been established for the responsible healthcare function, and the condition is available for appropriate healthcare action without transferring medicine ownership to another domain.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An institutional medicine inventory condition reaches a configured threshold or expiry-related condition relevant to healthcare operations.

### 4.2 End Boundary

The condition has been identified, recorded and communicated as an operational healthcare alert, with the appropriate next business action established where the authoritative policy defines one.

### 4.3 In Scope

- Medicine inventory condition recognition.
- Threshold condition.
- Expiry condition.
- Medical operations impact assessment.
- Operational alert.
- Responsible healthcare actor.
- Availability implications.
- Patient-care continuity considerations.
- Notification handoff where configured.
- Audit/evidence.

### 4.4 Out of Scope

- Enterprise inventory ownership.
- Procurement.
- Supplier qualification.
- Purchase orders.
- Financial accounting.
- Pharmacy ERP.
- Technical monitoring infrastructure.
- Notification delivery implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

Owns the lightweight medicine inventory context used for institutional healthcare operations and the resulting medical operations alert.

The L2 explicitly keeps lightweight medicine inventory within G06 and excludes enterprise pharmacy management.

### Section E — People & Workforce

Provides healthcare-professional/personnel context for responsible medical operations.

### Section H — Notifications

May deliver configured alerts/notifications. H does not own medicine inventory or the medical operational decision.

## 6. Business Actors, Roles and Responsibilities

### Medical Staff / Healthcare Operations Staff

Review the inventory condition, assess its medical-operational impact and initiate the appropriate governed action.

### Authorised Medical Professional

May determine clinical implications where the inventory condition affects prescribed treatment or medicine availability.

### Section E

Provides authoritative personnel context.

### Section H

Delivers configured operational notification.

No procurement or warehouse role is invented within G06.

## 7. Preconditions and Entry Criteria

1. Medicine inventory is within the G06 lightweight medicine boundary.
2. A threshold or expiry condition is defined or otherwise identifiable under authoritative policy.
3. The medicine is relevant to institutional healthcare operations.
4. Responsible healthcare operations can be identified.
5. Applicable notification/alert policy is available where configured.

## 8. Trigger

A medicine inventory threshold or expiry condition becomes materially relevant to medical operations.

## 9. Main Business Flow

### Phase A — Identify the Inventory Condition

1. Identify the medicine.
2. Identify the applicable inventory condition.
3. Determine whether the condition is threshold-related or expiry-related.
4. Establish the business significance of the condition under the applicable policy.

### Phase B — Assess Medical Operational Impact

5. Determine whether the condition may affect medicine availability.
6. Identify whether prescriptions or expected patient-care activities could be affected.
7. Determine whether the condition requires medical operational attention.
8. Preserve the inventory condition and its medical-operational context.

### Phase C — Establish Alert

9. Establish the operational medical alert.
10. Identify the responsible healthcare operational actor.
11. Record the medicine and condition requiring attention.
12. Where configured, provide the alert through H.

### Phase D — Governed Follow-through

13. Determine the applicable next healthcare operation according to policy.
14. If medicine availability is affected for a patient prescription, route to the Medicine Availability Exception to Alternative Medical Action scenario.
15. If replenishment/procurement is required, preserve the healthcare alert and hand off to the appropriate external/enterprise process without claiming procurement ownership in G06.
16. Preserve the audit evidence.

### Phase E — Complete

17. End when the medical operations alert and its applicable next-action routing have been established.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Threshold condition | Inventory reaches the configured operational threshold | Establish medical operations alert | Responsible staff are made aware | G |
| Expiry condition | Medicine reaches an applicable expiry condition | Establish expiry-related medical operations alert and restrict affected use according to policy | Expiry risk is visible to healthcare operations | G |
| Patient prescription affected | Availability may affect an existing prescription | Route to Medicine Availability Exception to Alternative Medical Action | Patient-care continuity is addressed through healthcare workflow | G |
| Procurement required | Replenishment is required | Handoff to the appropriate procurement/inventory owner/process | G06 does not become procurement owner | G / J as governed |
| No material medical impact | Condition does not require an operational response under policy | Record the condition without unnecessary escalation | Accurate operational history | G |

The scenario does not create a local procurement lifecycle.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Threshold/expiry condition cannot be established | Alert cannot be based on an unsupported condition | Establish authoritative inventory condition first | G |
| Medical impact cannot be determined | Appropriate operational response is unclear | Route to responsible healthcare staff for assessment | G |
| Alert recipient/responsible function cannot be established | Alert cannot produce the intended business response | Resolve the responsible healthcare authority under existing operational policy | G |
| Existing prescription is affected | Patient-care continuity may be at risk | Route to Medicine Availability Exception to Alternative Medical Action | G |
| Replenishment is required | G06 cannot own enterprise procurement | Preserve alert and hand off to the authoritative procurement/inventory process | G / J |

No technical monitoring retries, queues, infrastructure failures or notification-delivery mechanics are described.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Lightweight medicine inventory remains within G06 | G06 | Inventory condition is relevant to G06 healthcare operations | L2-G06 scope |
| Enterprise pharmacy management remains outside G06 | Boundary | Alert must not become pharmacy-management ownership | L2-G06 scope |
| Healthcare operations are configuration driven | G06 | Threshold/expiry treatment must respect applicable configuration | L2-G06 Design Principles |
| Medical professionals retain operational authority | G06 | Clinical implications remain with healthcare professionals | L2-G06 Design Principles |
| Medical history and healthcare activity must remain auditable | G06 | Alert/action history must be traceable | L2-G06 Audit/Invariants |

The authoritative L2 retrieved does not provide a universal numeric threshold or universal expiry-alert lead period. No such values are invented here.

## 13. State and Lifecycle Impact

No new Medicine Inventory lifecycle state is introduced.

The scenario creates an **operational alert outcome** associated with an existing inventory condition.

Where the condition affects a prescription, the Prescription remains governed by its existing lifecycle:

`Created → Issued → Dispensed → Completed`.

Where a medicine cannot be dispensed, the separate Medicine Availability Exception scenario governs the patient-care outcome.

The scenario does not re-own the Prescription or Medical Encounter lifecycle.

## 14. Business Events and Cross-Domain Collaboration

G06 remains the canonical owner of the medicine operational fact.

The locked L2 establishes immutable domain events for significant healthcare activities, but the retrieved authoritative event definitions do not expose a dedicated medicine-inventory-threshold or medicine-expiry event name.

Therefore no `MedicineInventoryThresholdReached`, `MedicineExpired`, or `MedicalOperationsAlertRaised` event is invented.

H may consume the applicable G06 business fact for notification delivery where configured.

If replenishment crosses into another enterprise domain, the owning domain retains ownership; G06 communicates the medical operational requirement without acquiring procurement/inventory ownership.

## 15. Business Authorization, Approval and Audit Obligations

Inventory conditions affecting healthcare operations must be reviewed under the applicable healthcare operational authority.

Where clinical consequences exist, an authorised medical professional retains clinical decision authority.

Audit evidence should preserve:

- Medicine.
- Inventory condition.
- Threshold/expiry classification.
- Medical-operational impact.
- Alert decision.
- Responsible healthcare actor.
- Follow-through routing.
- Relevant resulting healthcare action.

Medical confidentiality applies if the alert identifies a patient or patient-specific prescription.

No technical access control/IAM implementation is specified.

## 16. Completion Criteria and Postconditions

### Completion Criteria

1. Medicine inventory condition identified.
2. Threshold/expiry condition classified.
3. Medical-operational significance assessed.
4. Responsible healthcare function identified.
5. Operational alert established.
6. Applicable downstream healthcare action identified where required.
7. Patient-specific availability exception routed where necessary.
8. Procurement/inventory ownership remains outside G06 where applicable.
9. Audit evidence preserved.

### Postconditions

- Material inventory condition is visible to responsible healthcare operations.
- Affected patient-care risk can be handled through the appropriate healthcare scenario.
- G06 remains owner of the medical operational context.
- No enterprise procurement or pharmacy lifecycle is fabricated.
- H remains notification owner where notification is configured.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The retrieved L2 does not specify universal numeric inventory thresholds.
2. It does not specify a universal expiry-warning period.
3. It does not expose a dedicated canonical event name for threshold/expiry alerts.
4. The exact downstream procurement handoff is not defined in the retrieved G06 source.

### Assumptions

- Threshold and expiry policies are configuration-driven where applicable.
- G06 owns the medical operational alert, not enterprise inventory/procurement.
- Patient-specific availability consequences use the separate Medicine Availability Exception scenario.

### Deferred Questions

Confirm the authoritative threshold/expiry policy, alert severity rules, canonical event name, and any procurement handoff contract before Architecture Approval/Lock.
