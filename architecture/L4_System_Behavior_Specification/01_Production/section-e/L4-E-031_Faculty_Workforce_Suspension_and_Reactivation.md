# Faculty Workforce Suspension and Reactivation

## 1. Specification Metadata

**Specification ID:** L4-E-031  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-031 — Faculty Workforce Suspension and Reactivation  
**Primary Business Owner:** Faculty Workforce  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-031; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern suspension and subsequent reactivation of an Active Faculty Workforce. L4 begins when an authorised suspension/restoration decision is available and ends when Suspended or Active is authoritative and the applicable canonical lifecycle event has been published. Technical implementation details remain outside this specification.

## 3. Required Business Context

- Authoritative Faculty Workforce context exists.
- Applicable upstream business policy and governance conditions are satisfied.
- For reactivation, the Faculty Workforce is Suspended and the suspension conditions have been resolved.
- Relevant cross-domain facts remain source-owned.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-031-BH-001 | The system shall resolve the Faculty Workforce and its current authoritative lifecycle state. | L3-E-031 §§7, 9, 13 |
| L4-E-031-BH-002 | For suspension, the system shall validate the applicable suspension condition and governance authority before changing the lifecycle state. | L3-E-031 §§7, 9, 12, 15 |
| L4-E-031-BH-003 | For a valid suspension, the system shall transition the Faculty Workforce from **Active → Suspended** through the canonical suspension workflow. | L3-E-031 §§9, 12, 13 |
| L4-E-031-BH-004 | After the suspension becomes authoritative, the system shall publish `FacultySuspended` under Faculty Workforce publisher responsibility. | L3-E-031 §§9, 14.1, 16.1 |
| L4-E-031-BH-005 | While the Faculty Workforce is Suspended, the system shall prevent that Faculty Workforce from participating in academic activities. | L3-E-031 §§9, 12, 16.1 |
| L4-E-031-BH-006 | When restoration conditions are resolved, the system shall transition the Faculty Workforce from **Suspended → Active** through the canonical reactivation workflow. | L3-E-031 §§7, 9, 12, 13 |
| L4-E-031-BH-007 | After reactivation becomes authoritative, the system shall publish `FacultyReactivated` under Faculty Workforce publisher responsibility. | L3-E-031 §§9, 14.1, 16.1 |
| L4-E-031-BH-008 | The system shall preserve suspension history, effective-period information and attributable audit information across suspension and reactivation. | L3-E-031 §§12, 15, 16.2 |

## 5. Cross-Domain & Event Behavior

Other domains may consume the authoritative Faculty Workforce lifecycle facts according to their own responsibilities. Faculty Workforce retains lifecycle ownership. No additional synchronous business hand-off or completion event is required beyond the canonical `FacultySuspended` and `FacultyReactivated` events established above.

## 6. Failure / Rejection / Continuation

- If suspension cannot validly be established, the current authoritative Faculty Workforce lifecycle state shall remain unchanged.
- If restoration conditions are not resolved, the Faculty Workforce shall remain Suspended.
- Reactivation shall restore the existing Faculty Workforce; it shall not create a new Faculty Workforce record.
- Technical retry, database rollback, queue recovery, and infrastructure mechanisms are excluded.

## 7. Audit & Evidence Behavior

The system shall preserve attributable audit evidence of the initiation context, applicable governance evaluation, authorised decision, resulting lifecycle transition, event publication, suspension history and restoration outcome.

## 8. Observable Completion Conditions

Suspension completes when the Faculty Workforce is authoritatively Suspended, academic participation is restricted, `FacultySuspended` is published, and required history is preserved. Reactivation completes when restoration conditions are resolved, the Faculty Workforce is authoritatively Active, participation is restored, `FacultyReactivated` is published, and required history is preserved. No new lifecycle state is introduced.

## 9. Traceability & Provenance

**Parent:** L3-E-031 — Faculty Workforce Suspension and Reactivation  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
