# Medical Event to Student Portal Health Information

## 1. Specification Metadata

**Specification ID:** L4-G-180  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-180 — Medical Event to Student Portal Health Information  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Approved L3-G-180; locked L2-G06 healthcare events and C/H ownership semantics  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

The system shall make only permitted G06 medical information available through the Student Portal business context for the correct student, while preserving G06 medical ownership and C student-identity authority.

## 3. Required Business Context

- G06 owns the authoritative medical event/record.
- C owns student identity/context.
- H provides Student Portal service/projection capability.
- Disclosure is constrained by applicable healthcare/institutional policy.
- Portal availability is not a new medical-record lifecycle state.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-180-BH-001 | When a G06 medical event concerns a student, the system shall associate it with authoritative C student context before making student-facing information available. | L3-G-180 §§7,9 |
| L4-G-180-BH-002 | The system shall determine whether the medical information is eligible for student-facing availability and shall expose only the permitted business information. | L3-G-180 §§9,12 |
| L4-G-180-BH-003 | The system shall prevent student-facing publication when the event is not eligible for disclosure or the student association is unresolved. | L3-G-180 §§10,11 |
| L4-G-180-BH-004 | The system shall preserve traceability from student-facing information to the authoritative G06 medical fact and shall not permit H to alter that fact. | L3-G-180 §§9,12,15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-180-BH-005 | The system shall use the canonical event of the originating G06 healthcare workflow and shall not invent a generic portal-publication event. | L3-G-180 §14 |
| L4-G-180-BH-006 | The system shall make permitted information available to H's Student Portal context without transferring medical or student-identity ownership. | L3-G-180 §§5,14 |

## 6. Failure / Rejection / Continuation

- If student identity cannot be reliably resolved, the system shall not publish student-facing health information.
- If disclosure is not permitted, the medical fact shall remain within G06 and shall not be exposed through the portal context.
- If source medical information is incomplete, the system shall not publish an unsupported student-facing outcome.
- If downstream information becomes inconsistent with G06 truth, the representation shall be reconciled through the governed business correction/supersession process.

## 7. Audit & Evidence Behavior

The system shall preserve source medical event, authoritative student context, permitted information, disclosure basis where defined, source-to-consumer traceability and applicable correction/supersession evidence.

## 8. Observable Completion Conditions

Completion occurs when permitted health information is available through the Student Portal context with authoritative student association and source traceability, or when a governed decision establishes that no student-facing publication is permitted.

## 9. Traceability & Provenance

**Parent:** L3-G-180 — Medical Event to Student Portal Health Information  
**Primary verification source:** L2-G06 — healthcare event semantics; C student identity; H downstream service role  

## 10. Open Issues / Deferred Design

The complete event-by-event disclosure matrix and business contract/read-model vocabulary are not defined by the source scenario. Technical portal UI, APIs, IAM, persistence and infrastructure are deferred.