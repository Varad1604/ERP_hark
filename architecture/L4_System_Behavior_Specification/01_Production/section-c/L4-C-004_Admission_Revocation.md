# Admission Revocation

**Specification ID:** L4-C-004  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-004  
**Primary Business Owner:** Admissions  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-004; L2-C01 — Admissions Workflows, WF-C01-08; L2-C02 for post-creation Student Lifecycle consequence  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support institutionally authorized revocation of an Admission Record based on an established revocation basis and supporting evidence. The behavior ends when the Admission is `Revoked`, history/evidence are preserved and `AdmissionRevoked` is published.

## 3. Required Business Context

- Admission exists and is eligible for revocation.
- Revocation authority and policy are established.
- Required supporting evidence and reason are available.
- Admissions owns the Admission lifecycle and `AdmissionRevoked`.
- Student Lifecycle owns any post-creation StudentContext consequence.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-004-BH-001 | When an authorized revocation basis is received, the system shall validate revocation authority, policy and Admission eligibility before transition. | L3-C-004 §§7, 9, 12; L2-C01 |
| L4-C-004-BH-002 | When required supporting evidence and reason are established, the system shall transition the Admission Record to `Revoked`. | L3-C-004 §§9, 12, 13; L2-C01 |
| L4-C-004-BH-003 | The system shall preserve revocation history and required business audit evidence. | L3-C-004 §§3, 11, 12; L2-C01 |
| L4-C-004-BH-004 | The system shall prevent revocation when required authority, evidence or eligibility is absent. | L3-C-004 §§7, 11; L2-C01 |
| L4-C-004-BH-005 | The system shall not directly modify StudentContext or Finance-owned records as a consequence of Admission revocation. | L3-C-004 §§5, 11; L1/L2 ownership boundaries |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-004-BH-006 | On successful revocation, the system shall publish the canonical `AdmissionRevoked` business event under Admissions ownership. | L3-C-004 §§3, 14; L2-C01 |
| L4-C-004-BH-007 | When `AdmissionRevoked` affects an existing StudentContext, the system shall make the authoritative revocation fact available for Student Lifecycle's governed consequence. | L3-C-004 §§3, 5, 9, 14; L2-C02 |

## 6. Failure / Rejection / Continuation

- Prevent revocation when authority is invalid.
- Do not complete revocation without required supporting evidence.
- Preserve an existing `Revoked` terminal outcome.
- If eligibility fails, leave the Admission lifecycle unchanged.
- Post-creation StudentContext handling remains Student Lifecycle-owned.

## 7. Audit & Evidence Behavior

Preserve revocation basis, reason, supporting evidence reference and resulting `Revoked` outcome as attributable business evidence through the authoritative Admissions/Audit capabilities.

## 8. Observable Completion Conditions

The authorized revocation decision is established, required evidence is preserved, the Admission is `Revoked`, history is preserved and `AdmissionRevoked` has been published.

## 9. Traceability & Provenance

**Parent:** L3-C-004 — Admission Revocation  
**Primary verification source:** L2-C01 — Admissions Workflows, WF-C01-08.  
**Secondary verification:** L2-C02 for Student Lifecycle consumption of `AdmissionRevoked`.

## 10. Open Issues / Deferred Design

No L4-specific issue identified. Technical authorization, evidence storage, event transport, persistence, infrastructure and UI behavior are deferred.
