# L4-G-132 — Library Resource Return to Closed Circulation

## 1. Specification Metadata
**Specification ID:** L4-G-132  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-132 — Library Resource Return to Closed Circulation  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern return of an actively borrowed library resource through return processing and closure of the borrowing while recording the resulting copy condition and circulation position.

**Start:** An Active Borrowing reaches a return action.  
**End:** Borrowing is Closed and the resource reaches its governed post-return circulation position.

## 3. Required Business Context
- An Active Borrowing exists.
- The borrowed copy is identifiable.
- Applicable return, condition, and circulation rules are available.
- Library Management owns borrowing and copy operational lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-132-BH-001 | Register the resource return against the authoritative Active Borrowing. | L3-G-132 |
| L4-G-132-BH-002 | Validate the returned copy against the active borrowing and applicable return conditions. | L3-G-132 |
| L4-G-132-BH-003 | Record applicable return and condition information. | L3-G-132 |
| L4-G-132-BH-004 | Close the Active Borrowing when governed return conditions are satisfied. | L3-G-132 |
| L4-G-132-BH-005 | Determine and record the copy's resulting operational circulation position. | L3-G-132 |
| L4-G-132-BH-006 | Preserve borrowing, return, and condition history. | L3-G-132 |

## 5. Cross-Domain & Event Behavior
The closed borrowing and post-return resource outcome shall be available to authorised library consumers without transferring Library Management ownership. No unsupported event name is introduced.

## 6. Failure / Rejection / Continuation
- A return that cannot be matched to an Active Borrowing shall not close an unrelated borrowing.
- Unresolved return or condition information shall retain the process in its applicable business position.
- A damaged or restricted copy shall not be made available for circulation contrary to policy.

## 7. Audit & Evidence Behavior
Preserve borrowing identity, return action, condition assessment, closure decision, resulting copy status, and historical evidence.

## 8. Observable Completion Conditions
Completion occurs when the borrowing is Closed, the returned copy has a governed operational position, and return history is traceable.

## 9. Traceability & Provenance
**Parent:** L3-G-132 — Library Resource Return to Closed Circulation

## 10. Open Issues / Deferred Design
Technical scanning, persistence, circulation interfaces, and notification mechanisms remain outside L4.