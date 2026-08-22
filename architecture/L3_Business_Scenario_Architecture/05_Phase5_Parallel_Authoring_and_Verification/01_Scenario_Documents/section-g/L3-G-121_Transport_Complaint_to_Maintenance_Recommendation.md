# Transport Complaint to Maintenance Recommendation

**Scenario ID:** L3-G-121  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
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
| Final L3 ID | L3-G-121 |
| Canonical Title | Transport Complaint to Maintenance Recommendation |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Transport Management may encounter vehicle or transport conditions through a complaint that cannot be resolved solely through normal transport operations and instead require maintenance attention.

This scenario defines the complete G04 business outcome for that condition: the complaint is operationally assessed, the maintenance requirement is established, the recommendation is reviewed where required, and the maintenance concern is handed to the maintenance-owning process.

The scenario deliberately stops before maintenance execution because G04 explicitly does not own maintenance.

## 3. Business Outcome

A maintenance-relevant transport complaint is assessed and results in an authoritative G04 **maintenance recommendation** supported by the relevant transport operational context.

The recommendation becomes the business hand-off into the maintenance-owning process. The scenario does not claim repair, maintenance scheduling, work-order completion, vehicle restoration, or any G01 lifecycle outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A registered transport complaint has been identified as potentially concerning vehicle condition or another transport issue that may require maintenance attention.

### 4.2 End Boundary

The scenario ends when the G04 maintenance recommendation has been established and made available to the maintenance-owning process, subject to any configured G04 administrative review.

### 4.3 In Scope

- Review of the relevant transport complaint.
- Identification of the affected operational vehicle/context.
- Classification of the complaint.
- Operational assessment.
- Determination that maintenance attention is required.
- Establishment of the maintenance recommendation.
- Supporting evidence/context.
- Configured G04 review/approval.
- Business hand-off to the maintenance-owning process.
- Preservation of complaint/recommendation history.

### 4.4 Out of Scope

- G01 maintenance diagnosis as a maintenance-domain outcome.
- Work-order lifecycle.
- Repair execution.
- Preventive maintenance scheduling.
- Vendor repair.
- Vehicle asset lifecycle.
- Financial accounting.
- Technical integration implementation.

## 5. Ownership and Participating Domains

G04 owns the transport complaint and the transport-side determination that maintenance is required.

G01 is the canonical maintenance owner. The G04 workflow explicitly separates maintenance recommendation from maintenance execution.

The business boundary is:

`Vehicle Issue → Operational Assessment → Maintenance Recommendation → Administrative Review → Manual G01 Service Request`

The scenario therefore cannot legitimately claim a maintenance completion outcome.

## 6. Business Actors, Roles and Responsibilities

### Complaint Reporter

May be a student, parent, staff member, driver, conductor, or transport supervisor as supported by G04 complaint handling.

### Transport Coordinator / Operational Reviewer

Reviews the complaint and performs the G04 operational assessment.

### Transport Supervisor / Manager

Performs configured administrative review or approval where required.

### G01 Maintenance Authority

Receives the maintenance recommendation and owns the subsequent maintenance process.

The L3 scenario does not define G01's internal maintenance actors.

## 7. Preconditions and Entry Criteria

1. A transport complaint exists.
2. The complaint has sufficient context for operational assessment.
3. A vehicle or transport operational condition potentially requiring maintenance is identifiable.
4. G04 owns the operational assessment.
5. The maintenance concern has not already been established as a completed downstream maintenance outcome.
6. Any required G04 review authority can be resolved through configured policy.

## 8. Trigger

Operational review of a transport complaint identifies a vehicle or transport condition that may require maintenance attention.

## 9. Main Business Flow

### Phase A — Establish the Maintenance-Relevant Issue

1. Open/review the registered transport complaint.
2. Identify the affected route/trip and vehicle context where available.
3. Confirm that the complaint concerns a condition potentially requiring maintenance.
4. Assess the operational significance of the reported condition.
5. Determine whether the condition is appropriately treated as a maintenance requirement.

### Phase B — Establish the Recommendation

6. If maintenance is required, establish the G04 maintenance recommendation.
7. Record the operational basis for the recommendation.
8. Preserve relevant complaint evidence and transport context.
9. Apply configured administrative review where required.
10. Confirm the recommendation as the G04-owned business outcome.

### Phase C — Maintenance Boundary

11. Make the maintenance recommendation available to the G01 maintenance process.
12. Preserve the complaint and recommendation history.
13. Stop the scenario at the G04-to-G01 business boundary.

The scenario does not proceed into maintenance execution.

## 10. Alternate Business Flows

### 10.1 Maintenance Is Not Required

If assessment determines that maintenance is not required, no maintenance recommendation is established. The complaint proceeds through the broader G04 operational-resolution path.

### 10.2 Issue Is Operational Rather Than Maintenance-Related

If the issue can be addressed through route adjustment, driver/service corrective action, administrative action, or another G04 operational response, that appropriate scenario is used instead.

### 10.3 Administrative Review Rejects Recommendation

If configured review rejects the recommendation, the issue returns to the appropriate G04 operational review path. No G01 maintenance outcome is asserted.

### 10.4 Evidence Is Insufficient

If the available business evidence is insufficient to establish a maintenance requirement, the recommendation remains unresolved until the required business context is established.

## 11. Failure, Interruption and Business Recovery

### 11.1 Affected Vehicle Cannot Be Reliably Identified

The recommendation is not attached to an unverified vehicle context. G04 must establish authoritative transport context before finalising the recommendation.

### 11.2 Maintenance Need Cannot Be Established

The complaint does not produce a maintenance recommendation. It remains in the applicable G04 review/resolution path.

### 11.3 Recommendation Is Rejected

The issue returns to operational review and an alternative G04 resolution is determined where applicable.

### 11.4 G01 Does Not Complete Maintenance

This is outside the scenario boundary. G04 must not mark its own recommendation as maintenance completion.

### 11.5 Vehicle Requires Immediate Operational Restriction

If the vehicle condition affects immediate operational suitability, the applicable vehicle replacement/restriction workflow is separately invoked. The maintenance recommendation remains distinct from the operational vehicle decision.

## 12. Business Rules and Decision Constraints

1. Transport Management may recommend maintenance when operational issues are identified.
2. Maintenance execution remains within G01.
3. Vehicle ownership remains outside G04.
4. G04's maintenance-related workflow is:

   `Vehicle Issue → Operational Assessment → Maintenance Recommendation → Administrative Review → Manual G01 Service Request`

5. Complaint records preserve reporter, route, trip, category, severity, resolution, supporting evidence and audit history.
6. Complaint history remains immutable.
7. The scenario must not invent G01 maintenance states or transitions.
8. The scenario must not redefine vehicle asset ownership.
9. Configuration-driven transport policies remain authoritative.

No maintenance priority, repair SLA, work-order status, vendor assignment, inspection result, or completion criterion is invented.

## 13. State and Lifecycle Impact

The G04 Transport Complaint lifecycle is:

`Registered → Reviewed → Resolved → Closed`

The maintenance recommendation is an outcome of the reviewed complaint. It is not a new complaint lifecycle state.

The following local states are deliberately not introduced:
- `MaintenancePending`;
- `RepairRequested`;
- `AwaitingRepair`;
- `RepairComplete`.

Those concepts, if applicable, belong to the maintenance architecture.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

G04 defines the complaint events:
- `TransportComplaintRegistered`;
- `OperationalIssueEscalated`;
- `ComplaintResolved`.

G04 also defines the vehicle operational event `MaintenanceRecommended` with the meaning **“Maintenance recommendation issued.”**

Because the scenario reaches a vehicle maintenance recommendation, `MaintenanceRecommended` is the applicable canonical G04 vehicle event where its upstream semantics are satisfied. G04 remains the publisher.

The scenario does not invent a separate `ComplaintMaintenanceRecommended` event.

### 14.2 Synchronous Business Collaboration / Hand-offs

The primary cross-domain business boundary is the G04-to-G01 maintenance hand-off.

G04 supplies:
- affected transport context;
- operational assessment;
- maintenance recommendation;
- relevant complaint/evidence context.

G01 owns the subsequent maintenance business outcome.

This is a business hand-off, not a technical API definition.

## 15. Business Authorization, Approval and Audit Obligations

The recommendation must be supported by the G04 operational assessment and applicable complaint evidence.

Where administrative review is required, the recommendation becomes authoritative only after the configured business decision.

The business history should preserve:
- complaint context;
- affected transport/vehicle context;
- category;
- severity;
- operational assessment;
- recommendation;
- supporting evidence;
- review decision;
- G01 hand-off context.

No unsupported maintenance evidence artefact or approval role is invented.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Complaint identified.
- Maintenance-relevant condition established.
- Operational assessment completed.
- Maintenance requirement determined.
- Recommendation established.
- Required G04 review completed where applicable.
- `MaintenanceRecommended` used where the canonical vehicle-event semantics apply.
- Recommendation made available to G01.
- G04 complaint/recommendation history preserved.

### Postconditions

1. A G04-owned maintenance recommendation exists.
2. G01 owns the subsequent maintenance execution.
3. G04 has not claimed repair or maintenance completion.
4. The affected complaint remains historically traceable.
5. Vehicle asset ownership remains outside G04.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership or boundary conflict was identified.

The key source constraint is that `MaintenanceRecommended` is a G04 vehicle event, while G01 owns maintenance execution. The scenario therefore uses the event only for the recommendation fact and does not imply downstream completion.

### Assumptions

- Administrative review remains configuration driven.
- The G01 hand-off remains a business boundary.
- Vehicle operational context is sufficient to establish the recommendation without redefining asset ownership.

### Deferred Questions

None material to this scenario's frozen outcome.
