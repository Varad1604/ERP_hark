# Transport Complaint to Operational Resolution

## 1. Specification Metadata

**Specification ID:** L4-G-120  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-120 — Transport Complaint to Operational Resolution  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-120; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a Transport Complaint from registration through authoritative G04 review, resolution and closure without claiming completion of separately owned maintenance, personnel or communication outcomes.

## 3. Required Business Context

- Complaint and available transport context are identifiable.
- Student/personnel facts remain source-owned by C/E where required.
- Transport Management owns the Transport Complaint lifecycle and operational resolution.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-120-BH-001 | Register the complaint in the authoritative `Registered` complaint position with the available reporter and transport context. | L3-G-120 §§3, 9, 13 |
| L4-G-120-BH-002 | Preserve source-owned student/personnel context and complaint evidence required for review without creating substitute facts. | L3-G-120 §§5, 9, 11–12 |
| L4-G-120-BH-003 | Progress the complaint to `Reviewed` when the governed operational review has established the complaint context and applicable resolution path. | L3-G-120 §§9, 13 |
| L4-G-120-BH-004 | Record only an authorised G04-owned operational resolution, making separately owned maintenance, personnel or communication concerns available to their canonical owners without claiming their downstream completion. | L3-G-120 §§3, 5, 9–12 |
| L4-G-120-BH-005 | Progress the complaint to `Resolved` when the G04-owned resolution is established and to `Closed` when the G04 complaint obligation is complete. | L3-G-120 §§3, 9, 13, 16 |
| L4-G-120-BH-006 | Preserve complaint, review, resolution, closure and hand-off history. | L3-G-120 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-120-BH-007 | On complaint registration, Transport Management shall publish `TransportComplaintRegistered`. | L3-G-120 §§9, 14 |
| L4-G-120-BH-008 | When the complaint is formally escalated through the governed G04 path, Transport Management shall publish `OperationalIssueEscalated`. | L3-G-120 §§10, 14 |
| L4-G-120-BH-009 | When the complaint reaches `Resolved`, Transport Management shall publish `ComplaintResolved`. | L3-G-120 §§9, 14, 16 |

## 6. Failure / Rejection / Continuation

- Insufficient business context shall keep the complaint within the governed review path until authoritative information is available.
- A rejected proposed resolution shall return to the applicable business review path; no new lifecycle state shall be invented.
- A maintenance, personnel or communication consequence shall remain with its canonical owner after G04 establishes the transport-side resolution/handoff.

## 7. Audit & Evidence Behavior

Preserve reporter and transport context, classification/severity where configured, supporting evidence, review, authority, selected resolution, hand-offs, lifecycle progression and final closure history.

## 8. Observable Completion Conditions

The complaint has reached `Resolved`, `ComplaintResolved` has been published, the G04 complaint obligation has reached `Closed`, and any separately owned downstream concern remains outside the claimed completion boundary.

## 9. Traceability & Provenance

**Parent:** L3-G-120 — Transport Complaint to Operational Resolution  
**Primary verification source:** L3-G-120 §§3, 5, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No material event, lifecycle or ownership gap remains for the core complaint-resolution outcome. Downstream maintenance/personnel/communication execution remains separately governed. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
