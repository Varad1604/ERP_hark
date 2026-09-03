# Reading Room Closure to Restricted Library Usage

## 1. Specification Metadata

**Specification ID:** L4-G-144  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-144 — Reading Room Closure to Restricted Library Usage  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-144; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an operating Reading Room to the authoritative **Operational Closure** condition, ending ordinary availability for the applicable operating period. “Restricted library usage” is a consequence of closure, not a separate lifecycle state, and a later opening remains a separate occurrence.

## 3. Required Business Context

- Reading Room is operationally available or within an active operating period.
- Operating period, closing condition and applicable Reading Room rules are authoritative.
- Required Library Management operational authority is available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-144-BH-001 | Identify the Reading Room and establish its current operating condition, applicable operating period, closing condition and rules. | L3-G-144 §§7,9,12 |
| L4-G-144-BH-002 | Complete the operational monitoring, usage supervision and active-usage handling required by the authoritative closure condition. | L3-G-144 §§3,9–10 |
| L4-G-144-BH-003 | Establish **Operational Closure** only when the business closure condition is authoritative; do not falsely close the room where the condition is unclear. | L3-G-144 §§9,11,13 |
| L4-G-144-BH-004 | End ordinary Reading Room availability for the applicable operating period and preserve closure history. | L3-G-144 §§3,9,15–16 |
| L4-G-144-BH-005 | Do not create `Closing`, `Closed`, `Restricted`, `Locked` or other unsupported lifecycle states; a future opening remains a separate scenario. | L3-G-144 §§12–13,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-144-BH-006 | The retrieved source does not expose a dedicated canonical Reading Room closure event. This specification shall not invent `ReadingRoomClosed`, `ReadingRoomClosureCompleted` or `ReadingRoomRestricted`. G is the sole canonical owner of this outcome. | L3-G-144 §§5,14,17 |

## 6. Failure / Rejection / Continuation

- No default closing condition or rule shall be invented.
- If active usage requires supervision, closure shall not be represented as complete until the authoritative business condition is satisfied.
- If an authoritative decision is required, no substitute authority shall be invented.
- Technical retry, rollback, queue, API and infrastructure mechanics remain outside L4 business behavior.

## 7. Audit & Evidence Behavior

Preserve Reading Room identity, operating period, closing condition, applicable rules, relevant monitoring/supervision evidence, closure decision and Operational Closure history.

## 8. Observable Completion Conditions

Completion occurs when the correct Reading Room reaches its authoritative **Operational Closure**, ordinary use has ended for the period, closure history is auditable, and no library-wide closure, membership change or later opening has been implied.

## 9. Traceability & Provenance

**Parent:** L3-G-144 — Reading Room Closure to Restricted Library Usage  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No dedicated Reading Room closure event or detailed closure checklist is exposed upstream; neither is invented. Technical implementation remains deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
