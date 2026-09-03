# Cancelled Opportunity Restoration to Governed Examination Opportunity

## 1. Specification Metadata

**Specification ID:** L4-I-141  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-141 — Cancelled Opportunity Restoration to Governed Examination Opportunity  
**Primary Business Owner:** Supplementary, Backlog & Improvement Examination  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-141; L2-I12; L2-I15 where applicable; L2-I02; L2-I03; L2-I04/L2-I05 as required  
**Referenced By:** Downstream registration and examination workflows

## 2. Behavioral Objective & Boundary

Govern restoration of a previously cancelled subsequent-examination opportunity only after an authoritative decision permits restoration. I12 owns the opportunity; session, schedule, Registration and Hall Ticket lifecycles remain independently owned.

## 3. Required Business Context

- Authoritative cancellation history and restoration decision.
- Applicable candidate/examination scope.
- Valid session and schedule positions can be established.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-141-BH-001 | Identify the cancelled opportunity, historical cancellation and authorized restoration basis. | L3-I-141 §§3,7–9 |
| L4-I-141-BH-002 | Validate or establish the required Examination Session through I02 and schedule through I03. | L3-I-141 §§5,9,12 |
| L4-I-141-BH-003 | Re-enter the opportunity into an existing valid I12 lifecycle position without inventing a `Restored` state. | L3-I-141 §§9,12–13 |
| L4-I-141-BH-004 | Route any required Registration or Hall Ticket consequence through I04/I05 rather than resurrecting historical records locally. | L3-I-141 §§9–12 |
| L4-I-141-BH-005 | Preserve cancellation and restoration history and prevent restoration beyond authorized scope. | L3-I-141 §§3,12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-141-BH-006 | I02/I03/I04/I05 and I15 shall retain publisher ownership for their applicable lifecycle facts, while I12 exposes the resulting governed opportunity through its existing lifecycle milestones. No `OpportunityRestored` event is introduced. | L3-I-141 §14 |

## 6. Failure / Rejection / Continuation

- Without authoritative restoration, the cancelled position remains controlling.
- Invalid session or unavailable schedule prevents operational restoration.
- Historical registration shall not be silently revived.
- A revoked restoration follows the prevailing canonical governance.

## 7. Audit & Evidence Behavior

Preserve cancellation history, restoration authority, session/schedule decisions, downstream routing and resulting opportunity position.

## 8. Observable Completion Conditions

Completion occurs when the authorized opportunity is re-established in an existing I12 lifecycle position with valid session/schedule context and canonical downstream workflows may proceed.

## 9. Traceability & Provenance

**Parent:** L3-I-141  
**Primary verification sources:** L2-I12, I02, I03 and applicable I15/I04/I05 workflows.

## 10. Open Issues / Deferred Design

No dedicated restoration state/event is defined. Exact restoration authority and permissible conditions remain upstream policy-owned.

## Canonical Validation Note

Historical cancellation remains traceable and no owned lifecycle is silently reversed.
