# Academic Examination Cycle to Governed Examination Session

## 1. Specification Metadata

**Specification ID:** L4-I-004
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Examination — Session Management
**Realizes Scenario:** L3-I-004
**Primary Business Owner:** Examination Session Management
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-03
**Depends On:** L3-I-004 — Locked canonical parent; L2-I02 — Locked primary verification source; L4 Governance and Architecture Standard; L4 Validation Standard
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP must establish one governed Examination Session for the academic examination cycle using authoritative institutional, academic-period, applicability and Examination Policy facts, progress it only through source-defined session lifecycle conditions, and expose it to downstream Examination workflows only when it reaches the source-defined operational position.

The behavior starts when an examination cycle requires a governed session and ends when the session is established in the permitted operational position. Session closure and archival remain outside this specification and are owned by L2-I16.

## 3. Required Business Context

- Authoritative Institution and organisational context.
- Published/valid academic-period context.
- Approved Examination Policy.
- Authoritative programme, curriculum and applicability facts where required.
- Source-defined Examination Administrator and Controller of Examination responsibilities.
- L2-I02 lifecycle and approval conditions.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-004-BH-001 | The system shall establish an Examination Session only when the authoritative Institution, academic-period and approved Examination Policy context required by the parent is available. | L3-I-004 §§3–4, 7, 9, 12 |
| L4-I-004-BH-002 | The system shall associate the session with exactly one approved Examination Policy without altering policy ownership. | L3-I-004 §§5, 7, 9, 12 |
| L4-I-004-BH-003 | The system shall prevent session approval or activation while any parent-defined policy, applicability, ownership or governance condition remains unsatisfied. | L3-I-004 §§9–12, 15 |
| L4-I-004-BH-004 | The system shall progress the Examination Session only through the source-defined lifecycle positions required by the parent and shall not introduce a new state or transition. | L3-I-004 §§12–13 |
| L4-I-004-BH-005 | When the session becomes operationally consumable, the system shall make the authoritative session context available to permitted downstream Examination workflows without transferring ownership. | L3-I-004 §§3–5, 14, 16 |
| L4-I-004-BH-006 | The system shall not perform Examination Session closure or archival as part of this behavior. | L3-I-004 §§4, 13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-004-BH-007 | The system shall recognize `Academic Period Published` as authoritative academic-period context when that event establishes the parent entry condition. | L3-I-004 §§7, 14; L2-I02 §12 |
| L4-I-004-BH-008 | The system shall recognize `Examination Policy Approved` as authoritative policy context when that event establishes the governing policy condition. | L3-I-004 §§7, 14; L2-I02 §12 |
| L4-I-004-BH-009 | When the session is successfully initiated, the system shall publish `Examination Session Created` under Examination Session Management ownership. | L3-I-004 §§9, 14; L2-I02 §11 |
| L4-I-004-BH-010 | When institutional session approval is completed, the system shall publish `Examination Session Approved` under Examination Session Management ownership. | L3-I-004 §§9, 14, 16; L2-I02 §11 |
| L4-I-004-BH-011 | When the session becomes operational, the system shall publish `Examination Session Activated` under Examination Session Management ownership. | L3-I-004 §§9, 14, 16; L2-I02 §11 |

## 6. Failure / Rejection / Continuation

- If authoritative academic, institutional or policy context is missing or inconsistent, the system shall prevent affected progression and preserve the last valid session position.
- If session approval is rejected, the system shall keep the session unavailable for operational consumption and permit only the parent-supported modification, cancellation or replacement path.
- If activation prerequisites are not satisfied, the system shall prevent activation and downstream operational consumption.
- If authoritative source facts change before activation, the system shall require source-supported revalidation rather than silently applying stale approval.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of session creation, configuration, policy association, source facts used, review, approval, activation, responsible business authorities, lifecycle changes, rejection/exception outcomes and the canonical events produced by completed transitions.

## 8. Observable Completion Conditions

The behavior is complete when the Examination Session is established in a source-supported operational position, required approval and activation conditions are satisfied, material source facts and events are traceable, downstream consumers can use the authoritative session, and no closure/archival outcome has been absorbed.

## 9. Traceability & Provenance

**Parent:** L3-I-004 — Academic Examination Cycle to Governed Examination Session
**Primary verification source:** L2-I02 — Examination Session Management Workflows
**Additional verification source:** L2-I02 Contracts and Locked Decisions Amendment 2026-08-24

Every BH derives from the locked parent. L2-I02 verifies exact lifecycle, rules, events, publishers and cross-domain boundary already represented by the parent.

## 10. Open Issues / Deferred Design

No material L4 issue remains after event/completeness correction. Repository lock remains a separate governance action.

API/interface design, persistence, event transport, technical access control, infrastructure and UI/UX remain deferred.
