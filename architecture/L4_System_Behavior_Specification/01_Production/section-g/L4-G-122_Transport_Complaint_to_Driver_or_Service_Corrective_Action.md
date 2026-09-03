# Transport Complaint to Driver or Service Corrective Action

## 1. Specification Metadata

**Specification ID:** L4-G-122  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-122 — Transport Complaint to Driver or Service Corrective Action  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-122; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a driver- or service-related Transport Complaint through authorised G04 corrective action and complaint closure while preserving People & Workforce ownership of personnel-governed consequences.

## 3. Required Business Context

- A registered complaint has been identified as a driver-behaviour or transport-service concern.
- Driver/personnel facts remain authoritative in Section E.
- G04 owns the transport complaint and transport operational corrective action.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-122-BH-001 | Identify the registered complaint and affected driver/service transport context. | L3-G-122 §§3, 7–9 |
| L4-G-122-BH-002 | Use authoritative Section E personnel context where driver facts are material and preserve the evidence required for governed complaint review. | L3-G-122 §§5, 9, 11–12 |
| L4-G-122-BH-003 | Determine the applicable G04 transport corrective action through the configured review and authority path. | L3-G-122 §§9, 12, 15 |
| L4-G-122-BH-004 | Record an authorised transport-level corrective action, including driver counselling or service correction where supported, without creating or changing an E-owned personnel lifecycle state. | L3-G-122 §§3, 5, 9, 12–13 |
| L4-G-122-BH-005 | When the matter requires a personnel-governed consequence beyond G04 authority, make the relevant complaint/personnel context available to Section E without representing the E-owned downstream outcome as complete. | L3-G-122 §§3, 5, 9–12, 16 |
| L4-G-122-BH-006 | Progress the complaint to `Resolved` when the G04 corrective outcome is established and to `Closed` when the G04 complaint obligation is complete. | L3-G-122 §§9, 13, 16 |
| L4-G-122-BH-007 | Preserve complaint, evidence, authority, corrective action, hand-off, resolution and closure history. | L3-G-122 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-122-BH-008 | On complaint registration, Transport Management shall publish `TransportComplaintRegistered`. | L3-G-122 §14 |
| L4-G-122-BH-009 | When the issue is formally escalated through the governed G04 path, Transport Management shall publish `OperationalIssueEscalated`. | L3-G-122 §14 |
| L4-G-122-BH-010 | When the complaint reaches `Resolved`, Transport Management shall publish `ComplaintResolved`. No `DriverCounselled` or `CorrectiveActionCompleted` event shall be invented. | L3-G-122 §§14, 16–17 |

## 6. Failure / Rejection / Continuation

- Insufficient evidence shall keep the complaint within the governed review path and prevent unsupported corrective action.
- Unconfirmed driver/personnel context shall be obtained from Section E rather than recreated in G04.
- A rejected corrective action shall return to the applicable G04 business review path without inventing a new complaint state.
- Personnel-governed consequences remain outside the G04 completion boundary.

## 7. Audit & Evidence Behavior

Preserve complaint evidence, authoritative driver/service context, review, authority, corrective action, E hand-off where required, canonical event evidence, resolution and closure history.

## 8. Observable Completion Conditions

The G04 transport corrective action or governed non-corrective resolution is established, any required E-owned personnel hand-off is recorded without claiming its completion, `ComplaintResolved` is published at resolution, and the complaint reaches `Closed` under the authoritative lifecycle.

## 9. Traceability & Provenance

**Parent:** L3-G-122 — Transport Complaint to Driver or Service Corrective Action  
**Primary verification source:** L3-G-122 §§3, 5, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No dedicated corrective-action event exists upstream; only the canonical complaint/escalation events are used. Personnel-governed consequences remain Section E-owned. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
