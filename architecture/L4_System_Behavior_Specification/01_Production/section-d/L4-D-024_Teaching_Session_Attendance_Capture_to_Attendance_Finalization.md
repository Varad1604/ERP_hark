# Teaching Session Attendance Capture to Attendance Finalization

## 1. Specification Metadata

**Specification ID:** L4-D-024  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Delivery — Teaching Session & Attendance  
**Realizes Scenario:** L3-D-024  
**Primary Business Owner:** Teaching Session & Attendance  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `L3-D-024_Teaching_Session_Attendance_Capture_to_Attendance_Finalization.md`; `L2-D04_Teaching_Session_and_Attendance_Workflows.md`; Locked L4 governance/template/validation baseline  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

Capture canonical attendance for eligible students and govern the Attendance Register through submission, optional policy-required approval and Finalized.

**Start boundary:** An eligible Teaching Session has an open Attendance Register and authoritative student eligibility facts.

**End boundary:** The Attendance Register reaches Finalized and becomes the official attendance truth for the Teaching Session.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Attendance Register belonging to exactly one Teaching Session
- Authoritative Section C registration/eligibility facts
- Canonical attendance classifications and applicable segments
- Institutional attendance approval/finalization policy

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-024-BH-001 | The system shall verify Teaching Session/Attendance Register eligibility before attendance processing. | L3-D-024 §§3–13 |
| L4-D-024-BH-002 | The system shall record at most one canonical attendance record per eligible student for the Teaching Session and use only canonical attendance classifications/segments. | L3-D-024 §§3–13 |
| L4-D-024-BH-003 | The system shall validate Section C student eligibility references without modifying them. | L3-D-024 §§3–13 |
| L4-D-024-BH-004 | The system shall prevent submission/finalization when duplicate or invalid attendance entries remain. | L3-D-024 §§3–13 |
| L4-D-024-BH-005 | The system shall record Submitted when attendance is ready for governed processing. | L3-D-024 §§3–13 |
| L4-D-024-BH-006 | The system shall where institutional policy requires approval, require the authoritative approval outcome before finalization; where policy does not, follow the supported direct Submitted → Finalized path. | L3-D-024 §§3–13 |
| L4-D-024-BH-007 | The system shall require an authoritative D04 finalization decision before moving the Attendance Register to Finalized. | L3-D-024 §§3–13 |
| L4-D-024-BH-008 | The system shall transition the Attendance Register to Finalized only through the governed lifecycle and prevent normal modification thereafter. | L3-D-024 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-024-BH-009 | The system shall publish `AttendanceRegisterOpened` when capture opens. | L3-D-024 §14; L2-D04_Teaching_Session_and_Attendance_Workflows.md exact event semantics |
| L4-D-024-BH-010 | The system shall publish `AttendanceRecorded` as governed attendance capture occurs. | L3-D-024 §14; L2-D04_Teaching_Session_and_Attendance_Workflows.md exact event semantics |
| L4-D-024-BH-011 | The system shall publish `AttendanceSubmitted` when submission completes. | L3-D-024 §14; L2-D04_Teaching_Session_and_Attendance_Workflows.md exact event semantics |
| L4-D-024-BH-012 | The system shall publish `AttendanceApproved` only on the approval-required path when authoritative approval occurs. | L3-D-024 §14; L2-D04_Teaching_Session_and_Attendance_Workflows.md exact event semantics |
| L4-D-024-BH-013 | The system shall publish `AttendanceFinalized` when the register reaches Finalized. | L3-D-024 §14; L2-D04_Teaching_Session_and_Attendance_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume Section C student registration/eligibility facts without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid eligibility/reference prevents affected attendance progression.
- Duplicate/invalid attendance blocks submission/finalization until corrected.
- Rejected/withheld required approval prevents Finalized.
- Interrupted processing preserves the last valid attendance position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve student/session references, recorded classifications/segments, validation evidence, approval path/authority where applicable, state history and finalization fact.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the Attendance Register is Finalized
- required approval occurred where applicable
- `AttendanceFinalized` is published
- Section C facts remain source-owned

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-024` — Teaching Session Attendance Capture to Attendance Finalization  
**Primary verification source:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard  

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

The parent does not establish one universal institutional finalizer role; no role or technical permission is invented here.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

**Canonical Validation Note:** This specification is **Architecture Approved** under the final Architecture Approval issued by Dhruv / Lead Architect on 2026-09-02, as reported for this Section D batch. The specification is not `Locked`; movement to `Locked` requires separate explicit governance authorization.
