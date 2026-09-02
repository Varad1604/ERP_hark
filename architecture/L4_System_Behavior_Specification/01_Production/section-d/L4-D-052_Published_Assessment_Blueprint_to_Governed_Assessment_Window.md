# Published Assessment Blueprint to Governed Assessment Window

## 1. Specification Metadata

**Specification ID:** L4-D-052  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Delivery — Internal Assessment  
**Realizes Scenario:** L3-D-052  
**Primary Business Owner:** Internal Assessment  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `L3-D-052_Published_Assessment_Blueprint_to_Governed_Assessment_Window.md`; `L2-D07_Internal_Assessment_Workflows.md`; Locked L4 governance/template/validation baseline  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

Create and govern a blueprint-bound Assessment Window through Open, optional authorized extension and Closed.

**Start boundary:** A Published Internal Assessment Blueprint and valid academic context are available.

**End boundary:** The Assessment Window reaches Closed after the governed activity period; new submissions are prohibited thereafter.

This specification is limited to observable system behavior required by the Locked parent. It does not absorb adjacent domain outcomes or prescribe implementation mechanisms.

---

## 3. Required Business Context

- Published D07 blueprint
- Authoritative academic/calendar/Course Offering context
- Release/deadline policy
- Applicable institutional authority for exceptional extension/closure where required

All source-owned facts remain owned by their canonical business domains.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-D-052-BH-001 | The system shall require a Published blueprint before establishing/opening the Assessment Window. | L3-D-052 §§3–13 |
| L4-D-052-BH-002 | The system shall validate release, deadline, course context and policy before Open. | L3-D-052 §§3–13 |
| L4-D-052-BH-003 | The system shall transition the governed window to Open only under the applicable authoritative business action. | L3-D-052 §§3–13 |
| L4-D-052-BH-004 | The system shall allow assessment activity only while the governed Assessment Window state permits it. | L3-D-052 §§3–13 |
| L4-D-052-BH-005 | The system shall where policy permits an extension, require authoritative business approval/reason and retain the revised deadline. | L3-D-052 §§3–13 |
| L4-D-052-BH-006 | The system shall close the window at the governed deadline or valid authorized closure point. | L3-D-052 §§3–13 |
| L4-D-052-BH-007 | The system shall prevent new assessment submissions after Closed. | L3-D-052 §§3–13 |
| L4-D-052-BH-008 | The system shall require authoritative D07 business decisions for window actions that require business authority. | L3-D-052 §§3–13 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-D-052-BH-009 | The system shall publish `AssessmentWindowOpened` when the governed window opens. | L3-D-052 §14; L2-D07_Internal_Assessment_Workflows.md exact event semantics |

### Ownership-Preserving Collaboration

- Consume Section B/D01 academic/calendar/Course Offering context without ownership transfer.

Event names and publisher direction remain exactly those established by the parent/upstream architecture. No message schema, broker, queue, delivery or retry design is defined here.

---

## 6. Failure / Rejection / Continuation

- Invalid blueprint/context/deadline blocks Open.
- Missing extension/closure authority prevents the exceptional change.
- Interrupted progression preserves the last valid window position.

In every failure or interruption path, the system shall preserve the last valid business position and expose only the source-supported business continuation/rejection outcome. Technical rollback, retry and infrastructure recovery are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate this specification: Preserve blueprint reference, academic context, release/deadline, extension reason/decision where applicable, window-state history and opening fact.

Evidence behavior remains implementation-independent and does not prescribe tables, logs, storage engines or retention technology.

---

## 8. Observable Completion Conditions

- the window follows the D07 lifecycle to Closed
- any extension is source-supported/authorized
- new submissions are prohibited after Closed
- D07 retains window ownership

The specification is not complete merely because a technical action occurred; the applicable governed business outcome above must be objectively observable.

---

## 9. Traceability & Provenance

**Parent:** `L3-D-052` — Published Assessment Blueprint to Governed Assessment Window  
**Primary verification source:** `L2-D07_Internal_Assessment_Workflows.md`  
**Architecture boundary:** Locked L4 Governance and Architecture Standard; Locked L4 Validation Standard  

Every BH is derived from the Locked L3 parent. The L2 source is used only to verify exact owner, rule, lifecycle and event semantics already represented by the parent; this L4 specification does not silently repair or expand the L3 business scenario.

---

## 10. Open Issues / Deferred Design

The parent names no separate canonical closure event; L4 does not invent one.

The following remain deferred to L5/L6: API/interface design, persistence/data model, service/component design, event transport, technical access control, transaction/retry/rollback mechanics, infrastructure/deployment and UI/UX design.

---

**Canonical Validation Note:** This specification is **Architecture Approved** under the final Architecture Approval issued by Dhruv / Lead Architect on 2026-09-02, as reported for this Section D batch. The specification is not `Locked`; movement to `Locked` requires separate explicit governance authorization.
