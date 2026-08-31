# Admission Offer Expiry

**Specification ID:** L4-C-006  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-006  
**Primary Business Owner:** Admissions  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-006; locked L2-C01 — Admissions Workflows; 2026-08-22 expiry amendment under ADR-STU-ADM-EXP-001  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall recognize expiry of an Admission Offer when the authoritative configured validity condition is satisfied without required action. The behavior ends when the Admission is `Expired` and `AdmissionExpired` is published. Technical timing mechanisms are outside L4.

## 3. Required Business Context

- Admission Record is in `Admission Initiated`.
- Validity policy is configured upstream.
- Required action was not completed within that condition.
- `Expired` is terminal for the current Admission.
- Admissions owns expiry and `AdmissionExpired`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-006-BH-001 | When the authoritative configured validity condition is satisfied without required action, the system shall recognize the Admission as eligible for expiry. | L3-C-006 §§3, 7, 9; L2-C01 amendment |
| L4-C-006-BH-002 | When expiry eligibility is established and no contradictory terminal outcome applies, the system shall transition the Admission Record from `Admission Initiated` to `Expired`. | L3-C-006 §§3, 9, 13; ADR-STU-ADM-EXP-001 |
| L4-C-006-BH-003 | The system shall preserve the expired Admission and its historical evidence. | L3-C-006 §§3, 12, 16; L2-C01 |
| L4-C-006-BH-004 | The system shall not replace an existing `Cancelled`, `Withdrawn` or `Revoked` outcome with `Expired`. | L3-C-006 §§10, 12; L2-C01 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-006-BH-005 | On successful expiry, the system shall publish the canonical `AdmissionExpired` business event under Admissions ownership. | L3-C-006 §§3, 14; L2-C01 amendment |

## 6. Failure / Rejection / Continuation

- If the validity condition is not satisfied, the system shall leave the Admission unchanged.
- If another terminal outcome governs the Admission, that outcome remains authoritative.
- If the authoritative expiry policy cannot be established, the system shall not determine expiry.
- Downstream consequences remain owned by consuming domains.

## 7. Audit & Evidence Behavior

The system shall preserve evidence that the authoritative expiry condition was satisfied and that the Admission reached `Expired`, through the applicable Admissions/Audit capabilities.

## 8. Observable Completion Conditions

The configured validity condition is satisfied, no contradictory terminal outcome applies, the Admission is `Expired`, history is preserved, and `AdmissionExpired` has been published.

## 9. Traceability & Provenance

**Parent:** L3-C-006 — Admission Offer Expiry  
**Primary verification source:** L2-C01 — Admissions Workflows, as amended under ADR-STU-ADM-EXP-001.

## 10. Open Issues / Deferred Design

No open issue identified in the locked parent. Technical scheduling, timers, persistence, transport, interfaces and UI behavior are deferred.
