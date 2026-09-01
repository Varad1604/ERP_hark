# Missing Person Report to Security Response

## 1. Specification Metadata


**Specification ID:** ** `L4-G-203`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-203` — Missing Person Report to Security Response  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-203_Missing_Person_Report_to_Security_Response.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed Security Operations response to a missing-person report from formal intake through active search, rapid-response coordination, and either safe tracing/reunion or handover to police jurisdiction.

The behavior begins when a formal missing-person report is lodged. It ends when the Missing Person Case Aggregate reaches `PERSON_TRACED_OR_POLICE_HANDOVER`.

This specification does not define CCTV technology, physical search tactics, police investigation, family communications technology, or downstream welfare case management.

---

## 3. Required Business Context

- Formal missing-person report.
- Missing person identity and affiliation context.
- Parent / Warden / Department Head reporting context.
- Security Investigation Lead.
- Relevant campus movement and exit information.
- Parent-defined 60-minute rapid-search rule.
- Police-liaison context where required.
- Security Operations ownership of the Missing Person Case Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-203-BH-001` | When a formal missing-person report is accepted, the system shall establish the Missing Person Case Aggregate in `SEARCH_ACTIVE`. | L3-G-203 §§4, 9, 13 |
| `L4-G-203-BH-002` | The system shall retain the missing-person identity, reporter, and relevant campus-context facts required for the search response. | L3-G-203 §§2–4 |
| `L4-G-203-BH-003` | The system shall expose the parent-defined immediate 60-minute rapid-search obligation once the case enters `SEARCH_ACTIVE`. | L3-G-203 §12 |
| `L4-G-203-BH-004` | The system shall preserve search-response results sufficient to determine whether the person has been safely traced or statutory police handover is required. | L3-G-203 §§2–3 |
| `L4-G-203-BH-005` | Where the person is found, the system shall preserve the welfare-verification outcome required by the parent before completing the traced path. | L3-G-203 §§3–4 |
| `L4-G-203-BH-006` | Where police jurisdiction is assumed, the system shall preserve the statutory handover outcome without representing Security Operations as the owner of the police investigation. | L3-G-203 §§3–4, 6 |
| `L4-G-203-BH-007` | On governed completion, the system shall transition the Missing Person Case Aggregate from `SEARCH_ACTIVE` to `PERSON_TRACED_OR_POLICE_HANDOVER`. | L3-G-203 §§9, 13, 16 |
| `L4-G-203-BH-008` | Any parent-supported supervisory exception shall be recorded with its authority and justification without weakening the rapid-search obligation unless the parent expressly allows it. | L3-G-203 §§10, 15 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-203-BH-009` | The system shall recognize authoritative student/workforce/visitor and police-jurisdiction facts from their owning sources without transferring ownership to Security Operations. | L3-G-203 §§5–6 |
| `L4-G-203-BH-010` | When the case reaches `PERSON_TRACED_OR_POLICE_HANDOVER`, the system shall produce the canonical `Missing_Person_Response_Completed` business event on behalf of Security Operations. | L3-G-203 §§14, 16 |

No CCTV implementation, geolocation, police-system integration, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If required identity or case-verification information is incomplete, the system shall preserve the active unresolved case rather than fabricate a completion.
- If the person remains untraced beyond Security Operations jurisdiction, the system shall preserve the police-handover path where supported.
- If a welfare verification is not complete for a traced person, the system shall not silently treat the traced path as fully complete.
- Technical search-tool failures are outside L4 and are not modeled as implementation recovery behavior.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- formal missing-person report;
- reporting party;
- missing-person identity/context;
- rapid-search initiation;
- search-response outcomes;
- welfare verification where traced;
- police-handover basis where applicable;
- resulting `PERSON_TRACED_OR_POLICE_HANDOVER` position;
- publication of `Missing_Person_Response_Completed`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when either:

- the person is traced and welfare verification is recorded; or
- police jurisdiction is formally assumed and handover is recorded;

and the aggregate reaches `PERSON_TRACED_OR_POLICE_HANDOVER` with `Missing_Person_Response_Completed` produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-203` — Missing Person Report to Security Response  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

CCTV/video systems, location technology, police-system integration, notification mechanisms, persistence, event transport, audit implementation, and UI/UX remain deferred.
