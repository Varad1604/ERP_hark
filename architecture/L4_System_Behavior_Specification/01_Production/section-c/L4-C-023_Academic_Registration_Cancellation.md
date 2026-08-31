# L4-C-023 — Academic Registration Cancellation

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-023` — Academic Registration Cancellation  
**Primary Owner:** Academic Registration  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad

## 1. Purpose
Define behavior for cancelling an Academic Registration under an authorised Student request, administrative action, or policy-enforcement condition.

## 2. Preconditions and Trigger
An applicable registration exists and cancellation is permitted by the governing registration lifecycle/policy. The trigger is a valid Student request, authorised administrative action, or policy enforcement.

## 3. Main Behavior
1. Identify the registration and confirm its current lifecycle state.
2. Validate that cancellation is permitted.
3. Validate required authority where cancellation is administrative or approval-controlled.
4. Record the cancellation reason as required by L2-C03.
5. Apply the canonical cancellation lifecycle transition.
6. Preserve the cancelled registration as historical business evidence.
7. Publish `StudentRegistrationCancelled` only after successful cancellation.
8. Record complete audit evidence.

## 4. Alternate Behavior
- Cancellation not permitted in current state: stop and preserve reason.
- Authority absent: stop pending authorised action.
- Required business evidence unavailable: do not infer it.
- Existing downstream consequences do not change ownership of the cancelled registration.

## 5. Rules
Cancellation reason recording is mandatory where defined by L2-C03. Academic Registration remains canonical owner. Participating domains retain ownership of their own consequences and consume the cancellation fact through governed collaboration.

## 6. Lifecycle and Events
The applicable registration branch transitions to **Cancelled**. Canonical publisher: Academic Registration. Canonical event: `StudentRegistrationCancelled`. No new cancellation event is invented.

## 7. Authorization and Audit
Cancellation authority is institutionally configured where L2-C03 does not define one universal role. The cancellation decision, reason, actor, applicable authority and lifecycle transition are auditable.

## 8. Business Failure Handling
| Condition | Behavior |
|---|---|
| Invalid cancellation trigger | Do not cancel. |
| Cancellation not allowed | Preserve registration and return business rejection. |
| Authority unavailable | Defer until authorised. |
| Required reason absent | Do not complete cancellation. |

## 9. Traceability
L3-C-023; L2-C03 Registration Cancellation workflow; related C022 modification and C025 finalization.

## 10. Boundary
No API, database, UI, service, queue, retry, infrastructure or deployment design is specified.

## 11. Open Issue
L2-C03 does not establish one universal cancellation authority across all institutions; no universal role is invented.

**Classification: PASS WITH ISSUES — Draft**
