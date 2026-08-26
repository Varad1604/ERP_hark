# Plan and Approve an Examination Schedule

## 1. Specification Metadata

**Provisional Specification ID:** L4-I-007  
**Architecture Level:** L4 — System Behavior Specification  
**Realizes Scenario:** L3-I-007  
**Primary Business Owner:** Examination Planning & Scheduling  
**Status:** Draft  
**Pilot Classification:** Provisional — not canonical naming or granularity  
**Primary Sources:** L3-I-007 v1.1 Architecture Approved; locked L2-I03  

## 2. Behavioral Objective & Boundary

SutraERP must support the preparation, validation, review and approval of an Examination Schedule within an Active Examination Session while preserving authoritative upstream facts, preventing approval while material conflicts remain unresolved, and stopping this scenario at the approved schedule outcome rather than absorbing publication.

The behavior begins when an Active Examination Session and governed scheduling need exist and ends when the schedule is institutionally approved or reaches an allowed non-approval outcome.

## 3. Required Business Context

- Active Examination Session from Examination Session Management.
- Authoritative examination policy and planning rules.
- Academic Calendar, curriculum, campus/infrastructure and resource information from their owning domains.
- Source-supported business actors and decision authority.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-007-BH-001 | The system shall prevent examination-schedule planning from beginning unless the governing Examination Session is Active. | L3-I-007 §§4, 7, 11, 12; L2-I03 §§4.1, 8.2, 10.1 |
| L4-I-007-BH-002 | The system shall support preparation of the Examination Schedule using the authoritative planning context and source-owned contributions required by L2-I03. | L3-I-007 §§4, 9; L2-I03 §§5, 7, 8 |
| L4-I-007-BH-003 | The system shall use source-owned calendar, curriculum, venue, facility and resource information without transferring ownership of those facts to Examination Planning. | L3-I-007 §§5, 9, 14; L2-I03 §§4.5, 4.9, 13 |
| L4-I-007-BH-004 | The system shall evaluate the schedule against the applicable authoritative conflict-validation conditions before approval. | L3-I-007 §§9, 11, 12; L2-I03 §§5.6, 10.2–10.5 |
| L4-I-007-BH-005 | The system shall prevent schedule approval while any material scheduling conflict required by the governing architecture remains unresolved. | L3-I-007 §§10, 11, 12 |
| L4-I-007-BH-006 | The system shall require the source-defined institutional approval conditions to be satisfied before establishing the Examination Schedule as Approved. | L3-I-007 §§6, 9, 12, 15; L2-I03 §§5.7, 9.5 |
| L4-I-007-BH-007 | The system shall preserve the approved Examination Schedule as the authoritative Examination Planning outcome for this scenario. | L3-I-007 §§3, 13, 16 |
| L4-I-007-BH-008 | The system shall not publish the Examination Schedule as part of this scenario; publication remains a separately governed downstream lifecycle action. | L3-I-007 §§9, 12, 13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-007-BH-009 | The system shall recognize `Examination Session Activated` as the authoritative event establishing the planning-entry condition where event-driven initiation is used. | L3-I-007 §14; L2-I03 §12 |
| L4-I-007-BH-010 | The system shall publish `Examination Planning Initiated` when planning begins. | L3-I-007 §14; L2-I03 §11 |
| L4-I-007-BH-011 | The system shall publish `Examination Schedule Created` when the initial schedule is prepared. | L3-I-007 §14; L2-I03 §11 |
| L4-I-007-BH-012 | The system shall publish `Conflict Validation Completed` when governed conflict validation concludes. | L3-I-007 §14; L2-I03 §11 |
| L4-I-007-BH-013 | The system shall publish `Schedule Approved` when institutional approval is granted. | L3-I-007 §§14, 16; L2-I03 §11 |
| L4-I-007-BH-014 | The system shall not emit `Schedule Published` as completion of this scenario because publication remains outside its end boundary. | L3-I-007 §§9, 14, 16 |

## 6. Failure / Rejection / Continuation

- If the governing Examination Session is not Active, the system shall prevent planning progression.
- If authoritative planning inputs are missing or inconsistent, the system shall prevent affected progression until the owning domain resolves them.
- If conflict validation fails, the system shall return the schedule to governed planning/correction rather than allow approval.
- If approval is rejected, the system shall preserve the non-approved schedule and permit only source-supported modification, replanning, cancellation or replacement paths.

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to establish the governing Active Examination Session, planning context, source-owned contributions, schedule versions where changed, conflict-validation result, responsible participants, approval decision and final approved outcome.

## 8. Observable Completion Conditions

The behavior is complete when an Examination Schedule has satisfied required planning, conflict-validation and approval conditions and is established as Approved, while no publication or downstream execution outcome has been absorbed.

## 9. Traceability & Provenance

**Parent:** L3-I-007 — Plan and Approve an Examination Schedule  
**Primary verification source:** L2-I03 — Examination Planning & Scheduling Workflows  

Every BH derives from the approved L3 parent. L2-I03 is used only to verify exact lifecycle entry, actors, rules, event names and boundary conditions.

## 10. Open Issues / Deferred Design

No pilot-blocking upstream issue remains after the 2026-08-26 L3 remediation and re-review.

Scheduling algorithms, optimization techniques, persistence, API/interface design, event transport, technical access control, UI and infrastructure remain deferred.
