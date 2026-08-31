# Academic Detention and Repeat Determination

## 1. Specification Metadata

**Specification ID:** L4-C-031  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Academic Progression  
**Realizes Scenario:** L3-C-031 — Academic Detention and Repeat Determination  
**Primary Business Owner:** Academic Progression  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-031; L2-C04 Student Academic Progression Workflows and its 2026-08-22 amendment; L2-C03 Student Academic Registration Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall support the authoritative determination that a Student is Detained when applicable progression requirements remain unsatisfied, including a policy-governed repeat semester/year determination where required. The behavior ends when detention and any required repeat determination are established; subsequent registration execution remains outside this specification.

## 3. Required Business Context

- Authoritative academic evidence and official Examination outcomes.
- Applicable approved progression policy.
- Existing progression position, including a failed Academic Probation review where applicable.
- Policy-governed repeat determination criteria.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-031-BH-001 | When authoritative evidence establishes that progression requirements are not satisfied and the Detention path applies, the system shall establish the Student as **Detained**. | L3-C-031 §§2–3, 8–9 |
| L4-C-031-BH-002 | The system shall apply the approved progression policy when determining whether detention is required. | L3-C-031 §§8, 12 |
| L4-C-031-BH-003 | When policy requires a repeat semester/year determination, the system shall establish that repeat consequence using authoritative academic and curriculum facts. | L3-C-031 §§3, 9, 12 |
| L4-C-031-BH-004 | The system shall preserve the detention decision together with the authoritative evidence supporting it. | L3-C-031 §§4.3, 11–12, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-031-BH-005 | On establishment of detention, the system shall publish the canonical `AcademicDetentionApplied` outcome owned by Academic Progression. | L3-C-031 §14; L2-C04 amendment |
| L4-C-031-BH-006 | The system shall not treat superseded `StudentDetained` as a separate canonical event. | L3-C-031 §14 |

## 6. Failure / Rejection / Continuation

- If authoritative academic evidence is incomplete, the system shall not finalize detention solely on incomplete evidence.
- If the repeat consequence cannot yet be determined, the system shall preserve valid detention while leaving the repeat consequence unresolved until governed determination is possible.
- If an authoritative appeal/review changes the evidence, the system shall support fresh evaluation without rewriting historical evidence.

## 7. Audit & Evidence Behavior

The system shall preserve the authoritative academic evidence considered, applicable policy basis, detention outcome, repeat determination where applicable, and material historical decision evidence.

## 8. Observable Completion Conditions

Completion occurs when the detention basis is established under approved policy, the Student is Detained, any required repeat consequence is established, historical evidence is preserved, and `AcademicDetentionApplied` has been published. Downstream registration is not part of completion.

## 9. Traceability & Provenance

**Parent:** L3-C-031 — Academic Detention and Repeat Determination  
**Primary verification source:** L2-C04 — Student Academic Progression Workflows, as amended 2026-08-22.  

All behaviors above trace to the approved L3 parent; no new L2-derived requirement is introduced.

## 10. Open Issues / Deferred Design

Exact repeat-semester/year selection criteria, downstream registration hand-off detail, and institution-specific appeal/approval roles remain upstream or downstream governance matters. API, persistence, event transport, retry, IAM and UI design are deferred to later levels.

## Canonical Validation Note

This specification is a Draft L4 artifact and is not Architecture Approved or Locked.
