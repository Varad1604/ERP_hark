# L4-C-022 — Academic Registration Modification

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-022` — Academic Registration Modification  
**Primary Owner:** Academic Registration  
**Participating Domains:** C  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad

## 1. Purpose
Define behavior for an authorised modification of an existing Academic Registration, including add/drop and permitted academic changes, while preserving Academic Registration ownership and full revalidation.

## 2. Preconditions and Trigger
An existing registration is modifiable, the relevant registration period permits modification, and an authorised Student or administrative action requests a permitted change.

## 3. Main Behavior
1. Identify the target Academic Registration and current lifecycle position.
2. Confirm modification is permitted before Freeze unless an explicitly authorised exception applies.
3. Validate the requested change against offering availability, curriculum/programme rules, prerequisites/co-requisites, duplicate rules, credit limits, holds, and other applicable registration constraints.
4. Apply the permitted add/drop, elective, subject, project or equivalent modification.
5. Re-run the complete registration validation required by L2-C03.
6. If valid, retain the modified registration in its canonical lifecycle.
7. Obtain configured approval where required.
8. Publish `RegistrationModified` and the applicable canonical registration event(s), such as `CourseDropped`, only when the corresponding business fact has completed.
9. Preserve the prior and resulting business state in audit history.

## 4. Alternate Behavior
- Invalid modification: do not complete the requested change.
- Credit limit exceeded: reject/defer until resolved or authorised exception exists.
- Prerequisite/co-requisite failure: reject/defer.
- Freeze reached: modification is not permitted unless an explicit authorised policy path exists.
- Approval rejected: resulting approved state is not established.

## 5. Rules
- Modification does not transfer ownership from Academic Registration.
- Full validation is required after modification.
- Frozen registration is ordinarily immutable.
- Administrative exceptions require authority and audit evidence.
- Downstream domains consume resulting registration facts rather than directly mutating the Registration Aggregate.

## 6. Lifecycle and Events
The registration remains within the L2-C03 lifecycle; no new lifecycle state is introduced. Canonical publisher is Academic Registration. Relevant established events include `RegistrationModified` and `CourseDropped` where applicable.

## 7. Authorization and Audit
Authority follows configured L2-C03 institutional policy. Material actor, action, prior/resulting state, decision and exception evidence are auditable.

## 8. Business Failure Handling
| Condition | Behavior |
|---|---|
| Modification window closed | Stop unless authorised exception applies. |
| Validation fails | Do not complete the modification. |
| Duplicate/credit violation | Reject the violating change. |
| Approval unavailable/rejected | Do not complete the approval-dependent outcome. |

Technical retries, queues, APIs, databases and infrastructure are excluded.

## 9. Traceability
- L3: `L3-C-022`.
- L2: `L2-C03_Student_Academic_Registration_Workflows.md`.
- Related: C021 creation, C023 cancellation, C024 repeat/backlog, C025 finalization.

## 10. Open Issues
Exact institution-specific modification authority and routing remain configuration/policy where L2 does not establish one universal role.

**Classification: PASS WITH ISSUES — Draft**
