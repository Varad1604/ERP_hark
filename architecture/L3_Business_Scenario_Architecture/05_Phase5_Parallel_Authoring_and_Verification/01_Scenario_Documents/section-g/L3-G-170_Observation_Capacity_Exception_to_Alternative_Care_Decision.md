# Observation Capacity Exception to Alternative Care Decision

**Scenario ID:** L3-G-170  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E, H  
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
| Final L3 ID | L3-G-170 |
| Canonical Title | Observation Capacity Exception to Alternative Care Decision |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional outcome when observation care is clinically indicated but the institutional observation capability cannot accommodate the patient, requiring an authorised alternative-care decision.

The locked L2 defines Observation Care and Observation Stay within Institutional Health Services and separately defines Referral Management. It also makes clear that institutional healthcare is distinct from external hospital management.

The scenario therefore does not invent an additional observation lifecycle state to represent "capacity full." It treats capacity unavailability as a business exception that prevents or interrupts admission and requires a governed alternative-care decision.

## 3. Business Outcome

The institution has recognised that required observation care cannot be accommodated through the available institutional observation capability, an authorised healthcare professional has determined the appropriate alternative care path, and the patient is routed to that alternative without falsely recording an Observation Stay as successfully admitted.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Observation care is determined to be required, but institutional observation capacity is unavailable or insufficient.

### 4.2 End Boundary

An authorised alternative-care decision has been established and the patient is routed to the appropriate healthcare path.

### 4.3 In Scope

- Observation requirement.
- Capacity exception.
- Validation that observation cannot be accommodated.
- Patient/Medical Encounter continuity.
- Alternative-care assessment.
- External referral where required.
- Alternative institutional care where available.
- Follow-up where appropriate.
- Healthcare audit continuity.
- Notification participation where configured.

### 4.4 Out of Scope

- Creation of a new Observation Stay when capacity is unavailable.
- Hospital admission management.
- External hospital treatment.
- Facility/infrastructure capacity ownership.
- Procurement.
- Technical capacity calculation.
- Technical routing or notification implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

Owns the medical decision and the observation-care outcome.

### Section E — People & Workforce

Provides healthcare-professional/personnel context. It does not own the observation decision.

### Section H — Notifications

May deliver configured notifications.

Where physical capacity is governed by another campus capability, that ownership remains outside G06; G06 consumes the business availability information needed to make the healthcare decision.

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Determines that observation is clinically required and decides the appropriate alternative care when institutional observation cannot be provided.

### Medical Staff

Identifies the capacity exception and supports the alternative-care process within authorised operational responsibility.

### Patient

Receives the alternative-care outcome.

### Section E

Provides authoritative healthcare-personnel context.

### Section H

Provides notification delivery where configured.

No technical roles or invented authority structures are introduced.

## 7. Preconditions and Entry Criteria

1. A patient has a valid Medical Encounter.
2. Observation care has been determined to be clinically required.
3. Institutional observation capability is unavailable or insufficient for the required care.
4. The capacity exception is established as a business fact.
5. An authorised healthcare professional can determine the alternative-care path.

## 8. Trigger

Required observation care cannot be accommodated because institutional observation capacity is unavailable or insufficient.

## 9. Main Business Flow

### Phase A — Establish the Exception

1. Identify the patient and Medical Encounter.
2. Confirm the observation requirement.
3. Establish that the institutional observation capability cannot accommodate the patient.
4. Record the observation-capacity exception as part of the healthcare decision context.
5. Do not create or advance an Observation Stay as though admission had succeeded.

### Phase B — Assess Alternative Care

6. Refer the capacity exception to the authorised healthcare professional.
7. Review the patient's healthcare condition and the reason observation was required.
8. Determine the safest and applicable alternative-care path within institutional policy.
9. Determine whether an alternative institutional healthcare path is sufficient.
10. If institutional care is insufficient, determine whether external referral is required.

### Phase C — Establish the Alternative

11. If an institutional alternative is appropriate, route the patient to that healthcare workflow.
12. If external care is required, establish the Referral through the G06 referral workflow.
13. If further consultation or follow-up is appropriate, route to the applicable Medical Encounter/follow-up workflow.
14. Preserve the fact that Observation Stay admission did not successfully occur due to capacity.

### Phase D — Communicate and Close

15. Record the alternative-care decision.
16. Where configured, provide the relevant notification through H.
17. Preserve the complete business evidence.
18. End the scenario.

## 10. Alternate Business Flows

### 10.1 Alternative Institutional Care Available

If another institutional healthcare path can safely address the immediate need, the patient is routed there without creating a false observation admission.

### 10.2 External Referral Required

If institutional care is insufficient, the case proceeds through Referral Management.

### 10.3 Capacity Becomes Available Before Alternative Decision

If authoritative capacity becomes available before the alternative-care decision is finalised, observation admission may proceed through the normal Observation Admission workflow, subject to the authorised medical decision.

### 10.4 Follow-up Rather Than Immediate Observation

Where observation is no longer required after reassessment, the patient may proceed through the appropriate follow-up or ordinary healthcare outcome.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Required Business Handling |
|---|---|
| Capacity exception cannot be established | Do not invent an unavailable-capacity condition; obtain authoritative availability information. |
| Responsible healthcare authority unavailable | Preserve the unresolved medical requirement and obtain authorised clinical decision before selecting the alternative. |
| No acceptable institutional alternative | Route to external referral where clinically required. |
| External referral cannot yet be established | Preserve the unresolved healthcare requirement and continue the applicable emergency/medical care path. |
| Capacity changes during assessment | Reassess using the authoritative current business condition; do not duplicate observation outcomes. |
| Patient identity/encounter context is unresolved | Do not attach the alternative-care outcome to an invented patient or encounter. |

No technical retry, queue, rollback, infrastructure recovery or API behaviour is described.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Application |
|---|---|---|
| Observation Care is a G06 capability | G06 | Capacity exception is evaluated within institutional healthcare operations. |
| Observation Stay lifecycle is `Admitted → Observed → Released` | G06 | Capacity exception does not create a new lifecycle state. |
| Medical professionals retain operational authority | G06 | Alternative-care decision remains medically authorised. |
| External hospital management remains outside G06 | Boundary | Referral may establish external care without owning it. |
| Student identity remains C-owned | C | G06 consumes identity context only. |
| Employee information remains E-owned | E | G06 consumes workforce context only. |
| Healthcare activity remains auditable | G06 | Capacity exception and alternative decision are traceable. |

The L2 does not provide a universal capacity threshold or prioritisation algorithm; none is invented.

## 13. State and Lifecycle Impact

### Observation Stay

No Observation Stay is treated as successfully admitted merely because observation was clinically considered.

The authoritative Observation Stay lifecycle remains:

`Admitted → Observed → Released`.

A capacity exception occurs before successful admission or otherwise prevents the intended observation path; it is not represented as a new lifecycle state.

### Medical Encounter

The Medical Encounter remains the authoritative healthcare context and receives the capacity exception and alternative-care outcome.

### Referral

Where external care is required, Referral follows the L2 lifecycle:

`Recommended → Accepted → Completed`.

This scenario establishes the referral path only; the separate referral scenario governs its completion.

## 14. Business Events and Cross-Domain Collaboration

G06 owns the medical decision and therefore remains the canonical publisher of the resulting healthcare fact.

The exact canonical event name for an Observation Capacity Exception is not exposed in the authoritative event material retrieved for this pass. No event name is invented.

If a referral is created, the canonical publisher remains G06 because Referral is a G06-owned aggregate.

H may consume configured healthcare facts for notification delivery.

Any capacity information originating from another owned campus capability remains owned by that capability; consuming capacity information does not transfer ownership to G06.

## 15. Business Authorization, Approval and Audit Obligations

The alternative-care decision must be established by the appropriately authorised healthcare professional.

Business evidence should preserve:

| Obligation | Evidence | Owner |
|---|---|---|
| Observation requirement | Healthcare decision establishing need for observation | G06 |
| Capacity exception | Authoritative business condition preventing observation admission | G06 / source capability as applicable |
| Alternative-care decision | Authorised healthcare outcome | G06 |
| Referral, if required | Referral outcome and lifecycle position | G06 |
| Healthcare traceability | Link to patient and Medical Encounter | G06 |

No technical access-control or infrastructure mechanism is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Observation requirement is established.
- Institutional observation capacity exception is established.
- Patient and Medical Encounter remain correctly identified.
- An authorised alternative-care decision is made.
- The patient is routed to the appropriate alternative healthcare path.
- No false Observation Stay admission is recorded.
- Any required referral/follow-up path is established.
- Relevant healthcare evidence is preserved.

### Postconditions

- The observation-capacity exception is traceable.
- The alternative-care decision is authoritative within G06.
- The Observation Stay lifecycle is not falsely advanced.
- External hospital care, if required, remains externally owned.
- Patient/personnel ownership remains with C/E.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G06 does not define a universal observation-capacity threshold or prioritisation rule | L3 cannot determine capacity numerically or define queueing/prioritisation policy | Apply the applicable institutional configuration/policy when defined |
| Exact canonical capacity-exception event name is not exposed | Event cannot be named authoritatively | Verify complete locked L2 event catalogue before approval/lock |

### Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Capacity information may be consumed from the authoritative campus capability without transferring ownership | Repository ownership/dependency principles | Boundary may require refinement if upstream ownership changes |
| A capacity exception does not itself create a new Observation Stay state | Locked lifecycle has only three states | Introducing a new state would require L2 architecture change |

### Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Detailed system behaviour for evaluating live observation capacity | L4 | L3 remains implementation-independent |
