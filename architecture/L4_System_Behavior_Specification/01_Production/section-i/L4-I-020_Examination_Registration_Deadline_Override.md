# Examination Registration Deadline Override

## 1. Specification Metadata
**Specification ID:** L4-I-020  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-020 — Examination Registration Deadline Override  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-I-020 — Examination Registration Deadline Override; L2-I04 — Examination Registration Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an authorised exception to a closed examination registration deadline, permitting the affected registration to proceed only under the approved exception while preserving registration integrity and auditability.

**Start:** Examination registration requires consideration after the approved deadline has passed.  
**End:** The deadline exception is approved or rejected and the registration case has a governed disposition.

## 3. Required Business Context
- Active Examination Session.
- Published Examination Schedule.
- Closed registration window/deadline.
- Candidate and registration context.
- Authorised institutional exception basis.
- Required institutional authority.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-020-BH-001 | The system shall identify the expired or closed registration deadline affecting the registration case. | L3-I-020 §§2, 7–9 |
| L4-I-020-BH-002 | The system shall establish whether the case is eligible for an authorised deadline exception under applicable policy. | L3-I-020 §§2, 7, 12 |
| L4-I-020-BH-003 | The system shall retain the business justification and authority required for the exceptional override. | L3-I-020 §§3, 9, 12, 15 |
| L4-I-020-BH-004 | The system shall prevent the registration from proceeding outside the approved window unless the required exception is authorised. | L3-I-020 §§3, 11, 12 |
| L4-I-020-BH-005 | When the exception is approved, the system shall permit the registration to continue through the canonical Examination Registration lifecycle without bypassing other requirements. | L3-I-020 §§9, 12, 13, 16 |
| L4-I-020-BH-006 | The system shall preserve the override decision and resulting registration history. | L3-I-020 §§12, 15, 16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-020-BH-007 | The system shall preserve the canonical I04 publisher and registration lifecycle events when the affected registration subsequently progresses. | L3-I-020 §14; L2-I04 |
| L4-I-020-BH-008 | The system shall use authoritative institutional authorization capability without transferring ownership of the registration exception decision from Examination Registration. | L3-I-020 §5 |

No separate override event is invented.

## 6. Failure / Rejection / Continuation
- If required exception authority is absent or the request is rejected, the system shall prevent the registration from proceeding outside the approved window.
- If required business justification is missing, the system shall not complete the override.
- Approval of the deadline exception shall not bypass eligibility, review, approval, confirmation, or locking requirements.
- The existing registration governance shall remain authoritative if the exception is not approved.

## 7. Audit & Evidence Behavior
Preserve deadline context, exception basis, authority, justification, decision, resulting registration disposition, and relevant lifecycle history.

## 8. Observable Completion Conditions
Completion occurs when the exception has an explicit authorised decision, its justification is traceable, and the registration is either permitted to continue under the canonical lifecycle or remains governed by the closed deadline.

## 9. Traceability & Provenance
**Parent:** L3-I-020 — Examination Registration Deadline Override  
**Primary verification source:** L2-I04 — Examination Registration Workflows

## 10. Open Issues / Deferred Design
Institution-specific approval thresholds, exact officer roles, technical authorization, persistence, and interface mechanisms remain outside L4.