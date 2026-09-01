# L4-I-027 — Examination Registration Scope Change Following Academic Registration Change

## 1. Specification Metadata
**Specification ID:** L4-I-027  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-027 — Examination Registration Scope Change Following Academic Registration Change  
**Primary Business Owner:** Examination Registration  
**Participating Domains:** C, D, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Version:** 1.0  
**Status:** Draft

## 2. Behavioral Objective & Boundary
Govern an examination-registration scope reconciliation when an authoritative academic registration change affects examination participation.

**Start:** An authoritative academic registration change may affect examination scope.  
**End:** Examination Registration reflects the authorised resulting scope, or remains unchanged where there is no examination impact.

Out of scope: changing academic registration, curriculum rules, examination policy, finance obligations, hall-ticket generation, or technical synchronization.

## 3. Required Business Context
- Examination Registration exists for the candidate.
- The originating academic registration is authoritative.
- Examination Session and applicable schedule are established.
- Applicable examination registration policy is available.
- Registration is amendable or an authorised exceptional process applies.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-027-BH-001 | Identify affected candidate, examination session, registration, and registration items. | L3-I-027 §§3, 9 |
| L4-I-027-BH-002 | Evaluate the authoritative academic change against current examination scope. | L3-I-027 §§2, 9 |
| L4-I-027-BH-003 | Determine permitted additions, removals, corrections, or unchanged scope under examination policy. | L3-I-027 §§9, 10, 12 |
| L4-I-027-BH-004 | Apply required review and approval before an authorised scope amendment. | L3-I-027 §§9, 15 |
| L4-I-027-BH-005 | Establish the resulting Examination Registration scope as authoritative. | L3-I-027 §§3, 16 |
| L4-I-027-BH-006 | Preserve the prior scope and reason for amendment in historical evidence. | L3-I-027 §§9, 12, 15 |
| L4-I-027-BH-007 | Publish Registration Amended when an authorised amendment is established. | L3-I-027 §§9, 14 |
| L4-I-027-BH-008 | Preserve unchanged registration when the academic change has no examination impact. | L3-I-027 §10.1 |

## 5. Cross-Domain & Event Behavior
C and D remain owners of academic registration and curriculum information. I owns examination participation. The authoritative academic position is consumed through established business collaboration. Canonical I events are Registration Amended, Registration Confirmed, and Registration Locked; no new academic-change event is invented.

## 6. Failure / Rejection / Continuation
- Non-authoritative academic changes shall not alter examination scope.
- Ineligible resulting subjects shall not become authoritative examination participation.
- Missing required approval shall leave the existing authoritative registration unchanged.
- Locked registrations shall use only the approved exceptional process.
- A subsequent authoritative reversal shall be evaluated as a new governed amendment rather than silently restoring history.

## 7. Audit & Evidence Behavior
Preserve authoritative source context, affected registration, evaluation, decision, approval, resulting scope, prior scope, reason, and publication evidence.

## 8. Observable Completion Conditions
Completion occurs when the authorised examination scope is authoritative and traceable, or the analysis establishes that no examination change is required.

## 9. Traceability & Provenance
**Parent:** L3-I-027 — Examination Registration Scope Change Following Academic Registration Change

## 10. Open Issues / Deferred Design
Exact academic-change categories requiring scope adjustment and exceptional approval details remain governed by applicable institutional policy. Technical integration and synchronization are outside L4.