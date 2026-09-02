# Teaching Progress to Published Syllabus Coverage Record

## 1. Specification Metadata

**Specification ID:** L4-D-026  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Delivery — Teaching Session & Attendance  
**Realizes Scenario:** L3-D-026  
**Primary Business Owner:** Teaching Session & Attendance  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `L3-D-026_Teaching_Progress_to_Published_Syllabus_Coverage_Record.md`; `L2-D04_Teaching_Session_and_Attendance_Workflows.md`; Locked L4 governance/template/validation baseline  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

Record, validate and publish D04 Teaching Progress and actual syllabus/topic coverage using authoritative curriculum/Learning Outcome references.

**Start boundary:** An eligible Teaching Session permits progress recording.

**End boundary:** Teaching Progress reaches Published and becomes immutable historical instructional-progress truth.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Permitted Teaching Session state
- Authoritative D01 curriculum/topic/Learning Outcome references
- Conducting faculty authority
- Applicable D04 publication authority

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-026-BH-001 | The system shall verify the Teaching Session position and authoritative curriculum/Learning Outcome references before publication. | L3-D-026 §§3–13 |
| L4-D-026-BH-002 | The system shall record actual instructional progress against the Teaching Session. | L3-D-026 §§3–13 |
| L4-D-026-BH-003 | The system shall record actual coverage only using the canonical governed coverage states. | L3-D-026 §§3–13 |
| L4-D-026-BH-004 | The system shall validate consistency of session, topic/outcome references and recorded progress. | L3-D-026 §§3–13 |
| L4-D-026-BH-005 | The system shall require an authoritative D04 publication decision before Teaching Progress becomes Published. | L3-D-026 §§3–13 |
| L4-D-026-BH-006 | The system shall publish the validated Teaching Progress while retaining D01 ownership of planned curriculum definitions. | L3-D-026 §§3–13 |
| L4-D-026-BH-007 | The system shall prevent in-place editing of Published progress; any later correction must create new governed history. | L3-D-026 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-026-BH-008 | The system shall publish `TeachingProgressRecorded` and `TopicCoverageRecorded` as the corresponding actual teaching/coverage facts occur. | L3-D-026 §14; L2-D04_Teaching_Session_and_Attendance_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume D01 curriculum/topic/Learning Outcome references without ownership transfer; D05 may later consume Published D04 progress facts.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid session/curriculum reference prevents publication.
- Missing valid publication authority retains the current progress state.
- Interrupted publication preserves the last valid business position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve Teaching Session/faculty authority, curriculum references, actual coverage, validation/publication evidence and Published history.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- Teaching Progress is valid and Published
- canonical coverage states/references are retained
- historical Published progress remains immutable

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-026` — Teaching Progress to Published Syllabus Coverage Record  
**Primary verification source:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard  

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

No L4-specific open issue remains under the current Locked parent baseline.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

**Canonical Validation Note:** This specification is **Architecture Approved** under the final Architecture Approval issued by Dhruv / Lead Architect on 2026-09-02, as reported for this Section D batch. The specification is not `Locked`; movement to `Locked` requires separate explicit governance authorization.
