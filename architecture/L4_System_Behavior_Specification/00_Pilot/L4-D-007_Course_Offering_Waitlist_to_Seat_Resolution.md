# Course Offering Waitlist to Seat Resolution

## 1. Specification Metadata

**Provisional Specification ID:** L4-D-007  
**Architecture Level:** L4 — System Behavior Specification  
**Realizes Scenario:** L3-D-007  
**Primary Business Owner:** Course & Course Offering  
**Status:** Draft  
**Pilot Classification:** Provisional — not canonical naming or granularity  
**Primary Sources:** L3-D-007; locked L2-D01  

## 2. Behavioral Objective & Boundary

SutraERP must support governed waitlist resolution when a seat becomes authoritatively available, apply the authoritative institutional promotion policy, preserve Course & Course Offering ownership of the waitlist outcome, and hand the completed promotion fact to Section C without creating the Section C-owned registration transaction.

The behavior begins when a governed Course Offering seat is authoritatively available while waitlisted demand exists and ends when a valid promotion/seat-offer outcome is established and made available to Section C, or when no valid promotion can occur.

## 3. Required Business Context

- Authoritative Course Offering capacity / seat-availability context from D01.
- Existing ordered waitlist and applicable institution-configured promotion policy.
- Section C-owned eligibility and registration facts required for continuing eligibility confirmation.
- Supported manual-promotion authority where the institutional policy permits manual action.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-007-BH-001 | The system shall establish that a governed seat is authoritatively available before attempting waitlist promotion. | L3-D-007 §§7, 9, 11; L2-D01 waitlist/capacity rules |
| L4-D-007-BH-002 | The system shall apply the authoritative institution-configured waitlist promotion policy when determining a candidate demand. | L3-D-007 §§6, 9, 12; L2-D01 §§34–35 |
| L4-D-007-BH-003 | The system shall use Section C-owned eligibility information to determine whether the selected demand remains eligible without taking ownership of that eligibility fact. | L3-D-007 §§5, 9, 14 |
| L4-D-007-BH-004 | The system shall prevent promotion of a selected demand that is no longer eligible. | L3-D-007 §§9, 11 |
| L4-D-007-BH-005 | The system shall establish the governed promotion / seat-offer outcome under Course & Course Offering ownership. | L3-D-007 §§3, 5, 9, 13 |
| L4-D-007-BH-006 | The system shall not create or complete the Section C-owned registration transaction as part of waitlist promotion. | L3-D-007 §§2, 4, 5, 12 |
| L4-D-007-BH-007 | Where manual promotion is permitted, the system shall permit it only under the same authoritative waitlist policy and applicable business authority. | L3-D-007 §§6, 10, 15 |
| L4-D-007-BH-008 | If no authoritative promotion rule is available, the system shall prevent arbitrary candidate selection and leave the waitlist unresolved pending policy resolution. | L3-D-007 §11 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-007-BH-009 | The system shall publish `WaitlistStudentPromoted` when the governed promotion outcome is completed. | L3-D-007 §14; L2-D01 event catalogue |
| L4-D-007-BH-010 | The system shall make the governed waitlist promotion / seat-offer outcome available to Section C for its separately owned registration responsibilities. | L3-D-007 §§3, 14 |
| L4-D-007-BH-011 | Where `WaitlistPromotionTriggered` is materially used, the system shall preserve D01 as its canonical publisher. | L3-D-007 §14; L2-D01 event catalogue |

## 6. Failure / Rejection / Continuation

- If no authoritative seat is available, the system shall not promote demand and shall preserve the applicable waitlist position.
- If the selected demand is no longer eligible, the system shall follow the authoritative promotion policy for the next permitted outcome rather than invent ordering.
- If the promotion policy is undefined, the system shall stop promotion and surface the unresolved business-policy condition.

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to establish the seat-availability basis, applicable promotion policy, selected demand, continuing-eligibility result, manual authority where applicable, and final promotion outcome.

## 8. Observable Completion Conditions

The pilot behavior is complete when either:

- a valid governed promotion / seat-offer outcome exists, D01 remains its owner, `WaitlistStudentPromoted` is published, and Section C can consume the result; or
- a governed no-promotion condition exists and no arbitrary or unsupported promotion has occurred.

## 9. Traceability & Provenance

**Parent:** L3-D-007 — Course Offering Waitlist to Seat Resolution  
**Primary verification source:** L2-D01 — Course & Course Offering Workflows  

Every BH above is derived from the parent scenario and uses L2 only to verify authoritative waitlist rules, ownership, and event names.

## 10. Open Issues / Deferred Design

No pilot-blocking upstream issue identified.

API design, persistence, event transport, queue/broker technology, retry handling, technical access control and UI interaction remain deferred to later architecture levels.
