# Question Paper Issue During Conduct to Governed Examination Disposition

## 1. Specification Metadata


**Specification ID:** ** `L4-I-065`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination — Theory Examination Conduct (I07)  
**Realizes Scenario:** ** `L3-I-065` — Question Paper Issue During Conduct to Governed Examination Disposition  
**Primary Business Owner:** Theory Examination Conduct (I07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-I-065_Question_Paper_Issue_During_Conduct_to_Governed_Examination_Disposition.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed handling of a question-paper defect discovered during live theory conduct, resulting in an authoritative errata disposition and compensatory timing outcome.

The behavior begins when a printing defect, missing question/page, or erroneous subject code is reported during the examination. It ends when the Question Paper Errata Docket Aggregate reaches `ERRATA_DISPATCHED_AND_TIME_EXTENDED`.

This specification does not define printing systems, messaging infrastructure, or examination evaluation.

---

## 3. Required Business Context

- Active theory examination.
- Reported question-paper defect.
- Controller of Examinations.
- Subject Matter Expert / Dean.
- Chief Superintendent.
- Hall Invigilators.
- Parent-defined centralized subject-expert errata clearance rule.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-I-065-BH-001` | When a question-paper defect is reported during conduct, the system shall establish the Question Paper Errata Docket in `ERRATA_REPORTED`. | L3-I-065 §§4, 9, 13 |
| `L4-I-065-BH-002` | The system shall preserve the defect type, affected examination, and affected centres/halls. | L3-I-065 §§2–3 |
| `L4-I-065-BH-003` | The system shall require centralized subject-expert clearance under BR-I07-011 before an errata disposition is treated as authoritative. | L3-I-065 §12 |
| `L4-I-065-BH-004` | The system shall preserve the authoritative clarification/errata content without inventing academic content. | L3-I-065 §§2–3 |
| `L4-I-065-BH-005` | The system shall preserve distribution/announcement of the approved errata across affected halls at the business-behavior level. | L3-I-065 §§3–4 |
| `L4-I-065-BH-006` | The system shall preserve the compensatory time granted as a consequence of the issue. | L3-I-065 §3 |
| `L4-I-065-BH-007` | The system shall prevent final resolution while the reported issue lacks authoritative clearance or affected-hall disposition. | L3-I-065 §§11–12 |
| `L4-I-065-BH-008` | On governed disposition, the system shall transition the Question Paper Errata Docket from `ERRATA_REPORTED` to `ERRATA_DISPATCHED_AND_TIME_EXTENDED`. | L3-I-065 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-I-065-BH-009` | The system shall recognize scheduling and candidate facts from their canonical owners without transferring ownership to I07. | L3-I-065 §§5, 14 |
| `L4-I-065-BH-010` | When the errata disposition is completed, the system shall produce the canonical `Question_Paper_Errata_Disposed` business event. | L3-I-065 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If subject-expert clearance cannot be established, the issue shall remain unresolved.
- If affected centres/halls have not received the authoritative disposition, the system shall not represent the errata process as complete.
- A broader paper compromise shall be handled under its own governed scenario rather than silently converted into an errata case.

---

## 7. Audit & Evidence Behavior

Preserve issue report, defect classification, subject-expert clearance, errata content, affected halls, announcement/disposition evidence, compensatory time, final state, and canonical event.

---

## 8. Observable Completion Conditions

Completion requires:

- `ERRATA_DISPATCHED_AND_TIME_EXTENDED`;
- authoritative errata clearance attributable;
- affected halls have a governed disposition;
- compensatory time is recorded;
- `Question_Paper_Errata_Disposed` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-I-065`  
**Primary verification source:** `L2-I07_Theory_Examination_Conduct_Workflows.md` §8 (Conduct Execution), §11 (Failure Recovery), §14 (Events)

---

## 10. Open Issues / Deferred Design

The parent assigns “highest executive authority” wording to Hall Invigilators in one generic role table. L4 does not reinterpret that as new statutory authority; it preserves the clearly supported CoE and subject-expert approval behavior.

Broadcast technology, notification systems, persistence, event transport, and UI/UX remain deferred.
