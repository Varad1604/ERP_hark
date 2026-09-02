# Medical Professional Unavailability to Alternative Operational Assignment

## 1. Specification Metadata

**Specification ID:** L4-G-182  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-182 — Medical Professional Unavailability to Alternative Operational Assignment  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-182; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall govern recognition of medical-professional unavailability, assessment of affected G06 healthcare responsibilities, and establishment of an authorised alternative assignment or governed alternative business disposition. It shall not modify E-owned workforce records.

## 3. Required Business Context

- G06 owns healthcare operational assignment context.
- E owns workforce/personnel information and supplies authoritative context.
- Alternative assignment requires appropriate healthcare authority.
- Affected healthcare aggregates retain their own lifecycles.
- No universal qualification, priority or staffing rule is invented.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-182-BH-001 | When an assigned medical professional becomes unavailable, the system shall identify the affected G06 healthcare responsibilities and preserve their current business context. | L3-G-182 §§8,9 |
| L4-G-182-BH-002 | The system shall use authoritative E workforce context when evaluating potential alternative professionals. | L3-G-182 §§5,9 |
| L4-G-182-BH-003 | When a suitable authorised alternative exists, the system shall establish the alternative operational assignment and preserve its relationship to the affected responsibility. | L3-G-182 §§9,16 |
| L4-G-182-BH-004 | The system shall prevent assignment of an alternative that is not authorised for the affected healthcare responsibility. | L3-G-182 §§11,12 |
| L4-G-182-BH-005 | When no suitable alternative exists, the system shall require an explicit governed operational disposition rather than an unsupported substitute assignment. | L3-G-182 §§3,9,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-182-BH-006 | The system shall consume authoritative workforce context from E without transferring workforce ownership to G06. | L3-G-182 §14 |
| L4-G-182-BH-007 | Where the affected healthcare workflow produces a canonical G06 event, the system shall retain that workflow's canonical publisher and shall not invent an unavailability/reassignment event. | L3-G-182 §14 |

## 6. Failure / Rejection / Continuation

- If unavailability is not reliably established, the system shall not replace the existing assignment solely on an unconfirmed condition.
- If workforce context is unavailable, alternative selection shall not proceed on invented personnel information.
- If a candidate lacks required authority, the system shall reject that assignment candidate.
- If no suitable alternative exists, the system shall retain the affected responsibility's valid position and require the applicable rescheduling, restriction, referral or alternative-care disposition.
- If the original professional becomes available again, the system shall not automatically overwrite an established alternative assignment without a governed decision.

## 7. Audit & Evidence Behavior

The system shall preserve the original assignment, unavailability fact, affected responsibility, workforce context used, alternative assignment where established, business reason, resulting disposition and any affected healthcare lifecycle changes.

## 8. Observable Completion Conditions

Completion occurs when each affected healthcare responsibility has an authorised alternative assignment or an explicit governed alternative disposition. No E workforce lifecycle change is implied.

## 9. Traceability & Provenance

**Parent:** L3-G-182 — Medical Professional Unavailability to Alternative Operational Assignment  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No dedicated unavailability/reassignment event or universal professional-selection, staffing-ratio or priority rule is defined upstream. These remain upstream/configuration governed; technical scheduling and implementation remain deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
