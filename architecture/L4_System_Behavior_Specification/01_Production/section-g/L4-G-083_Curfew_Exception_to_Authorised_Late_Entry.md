# Curfew Exception to Authorised Late Entry

## 1. Specification Metadata

**Specification ID:** L4-G-083  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-083 — Curfew Exception to Authorised Late Entry  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-083; L2-G03 — Hostel Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Govern a resident curfew exception through policy evaluation and authorised late entry without converting an authorised exception into a curfew violation or changing Student Lifecycle ownership.

## 3. Required Business Context

- Active hostel residency exists.
- Applicable curfew policy is authoritative.
- A configured hostel authority can decide the exception.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-083-BH-001 | The system shall register the curfew-exception condition against the resident's hostel context. | L3-G-083 §§7–9 |
| L4-G-083-BH-002 | The system shall evaluate the circumstance against configured curfew and exception policy. | L3-G-083 §§9, 12 |
| L4-G-083-BH-003 | The system shall require the applicable configured hostel authority before establishing an authorised exception. | L3-G-083 §§6, 9, 15 |
| L4-G-083-BH-004 | When authorised, the system shall record the late-entry outcome as a governed exception and shall not classify that same outcome as an unauthorised curfew violation. | L3-G-083 §§3, 9, 12, 16 |
| L4-G-083-BH-005 | When the exception is not authorised, the system shall not grant late-entry authorisation and shall leave any resulting violation to the separate curfew-violation scenario. | L3-G-083 §§10–11, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-083-BH-006 | Hostel Management remains the canonical publisher of the authorised curfew-exception fact; no exact event name shall be invented because the Locked parent identifies the event vocabulary as unresolved. | L3-G-083 §§14, 17 |
| L4-G-083-BH-007 | No cross-domain business hand-off is required for this G-only scenario. | L3-G-083 §§5, 14.2 |

## 6. Failure / Rejection / Continuation

- Unsupported exceptions shall not be authorised.
- Missing authority shall leave the request unresolved rather than implicitly approved.
- A non-authorised late return is governed separately by L3-G-086.

## 7. Audit & Evidence Behavior

Preserve the resident context, curfew basis, exception reason, authority, decision and authorised late-entry outcome.

## 8. Observable Completion Conditions

Completion occurs when the exception is authorised and late entry accepted, or when the request is not authorised and no unsupported exception is recorded.

## 9. Traceability & Provenance

**Parent:** L3-G-083 — Curfew Exception to Authorised Late Entry  
**Primary verification source:** L3-G-083 §§3, 5, 9, 11–17 and authoritative L2-G03 architecture.

## 10. Open Issues / Deferred Design

Exact curfew-exception lifecycle/event identifiers remain unresolved upstream. Technical implementation remains deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
