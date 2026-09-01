# Faculty Workforce Suspension and Reactivation

## 1. Specification Metadata
**Specification ID:** L4-E-031  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Faculty Workforce  
**Realizes Scenario:** L3-E-031  
**Primary Business Owner:** Faculty Workforce  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-031 — Faculty Workforce Suspension and Reactivation; L2-E03 — Faculty Workforce Workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern suspension and subsequent reactivation of an Active Faculty Workforce. L4 begins when an authorised suspension/restoration decision is available and ends when Suspended or Active is authoritative. Technical implementation details remain outside this specification.

## 3. Required Business Context
The Faculty Workforce exists and its current lifecycle state is authoritative. Suspension uses WF-E03-008; reactivation uses WF-E03-009. A non-Active Faculty Workforce shall not participate in academic activities.

## 4. Behavioural Specification
1. Resolve the Faculty Workforce and current state.
2. Validate the applicable governance condition.
3. For suspension, transition Active → Suspended.
4. Publish `FacultySuspended`.
5. Restrict academic participation while Suspended.
6. When restoration conditions are satisfied, transition Suspended → Active.
7. Publish `FacultyReactivated`.
8. Preserve effective history and audit information.

## 5. Alternate and Exceptional Behaviour
Invalid suspension conditions leave the current state unchanged. Unresolved restoration conditions leave the Faculty Workforce Suspended. Reactivation does not create a new faculty representation.

## 6. Business Rules
- WF-E03-008 governs Faculty Suspension.
- WF-E03-009 governs Faculty Reactivation.
- Canonical lifecycle is Active → Suspended → Active.
- Non-Active Faculty Workforce cannot participate in academic activities.
- Historical lifecycle information remains preserved.

## 7. Events and Collaboration
| Event | Publisher | Meaning |
|---|---|---|
| `FacultySuspended` | Faculty Workforce | Faculty participation is suspended. |
| `FacultyReactivated` | Faculty Workforce | Suspended faculty participation is restored. |

## 8. Authorization, Audit & Traceability
Suspension and reactivation follow L2-E03 governance. Effective period, decision context and lifecycle history shall remain auditable. No technical IAM model is defined.

## 9. Completion & Validation
Completion requires the requested authoritative lifecycle state, correct canonical transition, applicable event publication and preserved history. No new lifecycle state is introduced.

## 10. Out of Scope
APIs, database schema, services, code, UI, infrastructure, queues, deployment and technical retry/rollback behaviour.