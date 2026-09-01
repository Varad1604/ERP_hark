# Integration Discrepancy to Reconciled External and Internal References

## 1. Specification Metadata
**Specification ID:** L4-H-080  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** External Education Integration  
**Realizes Scenario:** L3-H-080  
**Primary Business Owner:** External Education Integration  
**Version:** 1.0.2  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-080; L2-H10 — External Education Integration Workflows; `L2-H10_Provider_Reconciliation_and_Synchronization_Normalization_Amendment_2026-09-01.md`; applicable Locked source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an H10 Reconciliation Record from discrepancy detection through assignment, supported resolution, verification and closure or governed escalation while leaving canonical internal source state and authoritative external evidence unchanged.

## 3. Required Business Context
A concrete duplicate, missing, stale or conflicting integration discrepancy exists; authoritative internal/external references can be identified; H10 owns the Reconciliation Record only; source domains retain ownership of canonical facts.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-080-BH-001 | On detection of a concrete integration discrepancy, the system shall establish the Reconciliation Record in `Detected` and publish `ReconciliationIssueDetected`. | L3-H-080 §§7–9,14; L2-H10 normalization amendment §4 |
| L4-H-080-BH-002 | When a responsible H10 reviewer is assigned, the system shall progress `Detected → Assigned`, record the assignment and publish `ReconciliationAssigned`. | L2-H10 normalization amendment §4 |
| L4-H-080-BH-003 | The system shall compare internal/external references against authoritative source state and shall record only a source-supported resolution without silently rewriting either source of authority. | L3-H-080 §§5,9,11–12 |
| L4-H-080-BH-004 | A supported resolution shall progress the Reconciliation Record to `Resolved` and publish `ReconciliationResolved`; successful verification shall progress `Resolved → Verified` and publish `ReconciliationVerified`. | L2-H10 normalization amendment §4 |
| L4-H-080-BH-005 | After verified resolution, the system shall progress `Verified → Closed` and publish `ReconciliationClosed`. | L2-H10 normalization amendment §4 |
| L4-H-080-BH-006 | When authoritative source state cannot support resolution, the system shall progress the unresolved case to `Escalated` from an eligible `Detected`, `Assigned` or `Resolved` position and shall not represent it as reconciled or closed. | L2-H10 normalization amendment §4 |
| L4-H-080-BH-007 | An Escalated case shall remain unresolved until authoritative source state supports a new governed resolution; no additional event is invented solely for the Escalated state. | L2-H10 normalization amendment §4 |

## 5. Cross-Domain & Event Behavior
H10 shall publish `ReconciliationIssueDetected`, `ReconciliationAssigned`, `ReconciliationResolved`, `ReconciliationVerified` and `ReconciliationClosed` only for H10 reconciliation facts. Source domains shall provide authoritative confirmation and retain ownership of canonical source facts.

## 6. Failure / Rejection / Continuation
Unresolved or conflicting authoritative evidence shall prevent false reconciliation and closure. Escalation preserves the unresolved case without authorizing H10 to mutate source-domain state.

## 7. Audit & Evidence Behavior
The system shall preserve discrepancy classification, lifecycle progression, assignment, compared internal/external references, source confirmations, resolution basis, verification, escalation and closure history.

## 8. Observable Completion Conditions
The successful behavior completes when the Reconciliation Record reaches `Closed` through `Detected → Assigned → Resolved → Verified → Closed`. An unresolved governed outcome is observable as `Escalated`, with canonical source state unchanged unless its owning domain independently changes it.

## 9. Traceability & Provenance
**Parent:** L3-H-080 — Integration Discrepancy to Reconciled External and Internal References  
**Primary verification sources:** L2-H10 — External Education Integration Workflows; L2-H10 Provider, Reconciliation and Synchronization Normalization Amendment — 2026-09-01.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. No event is invented solely for `Escalated`. Data, protocol and reconciliation implementation remain deferred.
