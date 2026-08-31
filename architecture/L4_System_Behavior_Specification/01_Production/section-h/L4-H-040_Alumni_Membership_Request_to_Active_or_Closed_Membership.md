# Alumni Membership Request to Active or Closed Membership

## 1. Specification Metadata
**Specification ID:** L4-H-040  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Alumni  
**Realizes Scenario:** L3-H-040  
**Primary Business Owner:** Alumni  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-040; L2-H06 — Alumni Workflows; `L2-H06_Alumni_Engagement_Lifecycle_Amendment_2026-08-26.md`; Section A identity/authorization  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern H06 Alumni Membership from request/review to active participation, rejection, suspension/reactivation or closure without creating `Renewed` or `Expired` states or re-owning identity facts.

## 3. Required Business Context
Existing Alumni Profile; permitted membership request; canonical identity references; authorized H06 participants; membership conditions.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-040-BH-001 | Membership shall follow `Requested → Under Review → Active → Closed`, with `Under Review → Rejected`, `Active → Suspended → Active`, and `Active / Suspended → Closed` branches. | L3-H-040 §§9–13 |
| L4-H-040-BH-002 | Invalid identity/context or failed review shall prevent `Active`; a `Suspended` membership may return to `Active` only through the governed restoration branch. | L3-H-040 §§7,10–13 |
| L4-H-040-BH-003 | A `Closed` membership shall remain historical; where policy permits later renewal, the system shall create a new governed membership request rather than rewrite the closed record. | L3-H-040 §§9–13,16 |
| L4-H-040-BH-004 | H06 shall not introduce `Renewed` or `Expired` lifecycle states; expiry terminology shall be represented through governed ending/closure of the membership relationship. | L3-H-040 §§4,12–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-040-BH-005 | H06 shall publish `AlumniMembershipActivated`, `AlumniMembershipRejected`, `AlumniMembershipSuspended` and `AlumniMembershipClosed` for their corresponding facts; Section A identity facts remain referenced only. | L3-H-040 §14 |

## 6. Failure / Rejection / Continuation
Ineligible requests remain non-active; closed history is immutable and subsequent membership is a new governed relationship.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-040-BH-006 | The system shall preserve request, review, decision, suspension/reactivation/closure, prior membership history and any subsequent new membership request. | L3-H-040 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-040-BH-007 | Completion shall require an authoritative H06 membership outcome with complete history preserved and no rewrite of a closed predecessor. | L3-H-040 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-040 — Alumni Membership Request to Active or Closed Membership  
**Primary verification source:** Locked H06 lifecycle amendment.

## 10. Open Issues / Deferred Design
Former rejection/suspension/closure gaps are resolved upstream; no parallel renewal/expiry states are introduced.