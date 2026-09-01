# Finance Configuration Change to Governed Active Configuration

## 1. Specification Metadata
**Specification ID:** L4-F-084  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Finance Foundation  
**Realizes Scenario:** L3-F-084  
**Primary Business Owner:** Finance Foundation  
**Version:** 1.1.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-F-084; L2-F00 Finance Configuration Rollback Amendment; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

## 2. Behavioral Objective & Boundary
Make Finance configuration changes and rollback observable without reactivating or rewriting historical versions.

## 3. Required Business Context
Finance owns configuration meaning; Section A owns configuration-framework capability; Finance Configuration Approval Authority owns approval/rollback decisions.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior |
|---|---|
| L4-F-084-BH-001 | Create a distinct Finance configuration version for every governed change. |
| L4-F-084-BH-002 | Permit activation only after validation and approval. |
| L4-F-084-BH-003 | On activation, make the new version Active for future processing and move the prior Active version to Superseded. |
| L4-F-084-BH-004 | For rollback, require `FinanceConfigurationRollbackAuthorized` before creating a new version based on an eligible prior approved configuration. |
| L4-F-084-BH-005 | Progress the rollback version through the same forward lifecycle; never reactivate the historical Superseded version. |
| L4-F-084-BH-006 | Preserve each historical transaction's governing configuration reference. |

## 5. Cross-Domain & Event Behavior
Finance publishes `FinanceConfigurationRollbackAuthorized`, `FinanceConfigurationVersionActivated`, and `FinanceConfigurationVersionSuperseded`; Section A does not publish Finance semantic events.

## 6. Failure / Rejection / Continuation
Failed validation/approval keeps the current Active version unchanged.

## 7. Audit & Evidence Behavior
Record rollback reason, basis version, new version, authority, validation/approval and activation evidence.

## 8. Observable Completion Conditions
Exactly one governed version is Active for future processing and predecessor/history remain intact.

## 9. Traceability & Provenance
Parent: L3-F-084. Governing amendment: L2-F00 Finance Configuration Rollback Amendment dated 2026-09-01.

## 10. Open Issues / Deferred Design
None at L4.
