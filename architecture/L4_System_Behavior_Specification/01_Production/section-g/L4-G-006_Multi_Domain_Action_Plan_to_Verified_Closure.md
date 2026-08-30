# Multi-Domain Action Plan to Verified Closure

## 1. Specification Metadata

**Specification ID:** L4-G-006  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations Governance  
**Realizes Scenario:** L3-G-006  
**Primary Business Owner:** Campus Operations Governance  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-G-006_Multi_Domain_Action_Plan_to_Verified_Closure.md`; L2-G00 — Campus Operations Review & Governance  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern one Multi-Domain Action Plan from a source governance finding through assignment, source-domain execution tracking, completion verification and formal closure without transferring operational execution into G00.

The behavior begins when a governance finding requires corrective action and ends when the Action Plan reaches `Closed` through successful `Verified` completion with assignment, progress, verification and governance history preserved.

## 3. Required Business Context

- One governance finding exists and requires corrective action.
- The Action Plan can reference that finding.
- Responsible domains/owners, due-date policy, dependencies and verification expectations can be established.
- Operational execution remains within each responsible source domain.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-006-BH-001 | The system shall prevent creation of a standalone Multi-Domain Action Plan when no source governance finding can be referenced. | L3-G-006 §§7, 11–12, 16 |
| L4-G-006-BH-002 | On valid creation, the system shall place the Action Plan at `Created` and preserve its governance-finding reference. | L3-G-006 §§9, 12–13 |
| L4-G-006-BH-003 | Before assignment, the system shall establish the governed actions, responsible domains/owners, due dates, dependencies and verification expectations. | L3-G-006 §§7, 9, 12 |
| L4-G-006-BH-004 | On valid assignment, the system shall place the Action Plan at `Assigned` while preserving prior assignment history. | L3-G-006 §§9–10, 12–13 |
| L4-G-006-BH-005 | As responsible domains perform their own operational work, the system shall permit G00 to record governance progress and progress the Action Plan to `In Progress` without assuming execution ownership. | L3-G-006 §§2, 5, 9, 12–13 |
| L4-G-006-BH-006 | When an assigned action becomes overdue under configured governance rules, the system shall record the overdue condition without inventing a new lifecycle state. | L3-G-006 §§9, 12–14 |
| L4-G-006-BH-007 | The system shall prevent `Verified` when required completion evidence is absent or fails the configured verification rules. | L3-G-006 §§9–12, 16 |
| L4-G-006-BH-008 | On successful completion verification by the source-supported verification authority, the system shall place the Action Plan at `Verified`. | L3-G-006 §§6, 9, 12–13, 15 |
| L4-G-006-BH-009 | The system shall permit `Closed` only after `Verified` and shall preserve the complete Action Plan governance history on closure. | L3-G-006 §§9, 12–13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-006-BH-010 | On the corresponding canonical facts, the system shall publish `ActionPlanCreated`, `ActionAssigned`, `ActionOverdue` where applicable, `ActionVerified` and `ActionPlanClosed` under G00 ownership. | L3-G-006 §14 |
| L4-G-006-BH-011 | The G00-to-responsible-domain hand-off shall preserve the assigned governance action, due date, dependencies and verification expectations while operational execution remains owned by the responsible domain. | L3-G-006 §§5, 9, 14 |

## 6. Failure / Rejection / Continuation

Missing responsible ownership blocks valid assignment or continuation. Incomplete operational execution keeps the Action Plan outside `Verified`. Failed verification keeps the plan outside `Verified` and `Closed` until corrective work is completed by the responsible source domain and successfully reverified. Reassignment may occur only with prior assignment history preserved.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-006-BH-012 | The system shall preserve the source finding, assignments and reassignment history, responsible domains/owners, due dates, dependencies, progress, overdue conditions, completion evidence, verification result, lifecycle transitions, closure and canonical event history. | L3-G-006 §§10, 12, 15–16 |

## 8. Observable Completion Conditions

Completion requires the Action Plan to reference one governance finding and to have progressed `Created → Assigned → In Progress → Verified → Closed`, with assigned work executed by responsible domains, verification completed under the source-supported authority and required canonical/audit evidence preserved.

## 9. Traceability & Provenance

**Parent:** L3-G-006 — Multi-Domain Action Plan to Verified Closure  
**Primary verification source:** L2-G00 — Campus Operations Review & Governance

## 10. Open Issues / Deferred Design

No L4 correctness issue remains. L2-G00 does not name a separate universal final closer role; this specification therefore constrains closure by the canonical `Verified → Closed` lifecycle and does not invent an additional actor. Technical implementation remains deferred.