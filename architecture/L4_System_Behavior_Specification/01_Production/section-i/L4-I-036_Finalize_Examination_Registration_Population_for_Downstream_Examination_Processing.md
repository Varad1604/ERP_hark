# L4-I-036 — Finalize Examination Registration Population for Downstream Examination Processing

## 1. Specification Metadata
**Specification ID:** L4-I-036  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-036 — Finalize Examination Registration Population for Downstream Examination Processing  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0  
**Status:** Draft

## 2. Behavioral Objective & Boundary
Govern finalisation of the authoritative Examination Registration population so downstream examination processing consumes a controlled and complete participation set.

**Start:** Registration population is ready for final eligibility and completeness assessment.  
**End:** The final authoritative registration population is established and available for downstream examination processing.

## 3. Required Business Context
- Examination Session and schedule are established.
- Candidate registrations and applicable lifecycle positions are identifiable.
- Registration eligibility and applicable policy are available.
- Examination Registration owns participation population.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-036-BH-001 | Identify the candidate registrations eligible for final examination processing. | L3-I-036 |
| L4-I-036-BH-002 | Validate registration completeness and applicable eligibility conditions. | L3-I-036 |
| L4-I-036-BH-003 | Exclude registrations that do not satisfy authoritative finalisation conditions. | L3-I-036 |
| L4-I-036-BH-004 | Establish the final authoritative examination registration population. | L3-I-036 |
| L4-I-036-BH-005 | Preserve the basis and history of finalisation decisions. | L3-I-036 |
| L4-I-036-BH-006 | Make the final population available to downstream examination processing through established business collaboration. | L3-I-036 |

## 5. Cross-Domain & Event Behavior
Examination Registration remains the canonical owner of examination participation. Downstream examination processes consume the authoritative final population and do not redefine registration ownership. No unsupported finalisation event is invented.

## 6. Failure / Rejection / Continuation
- Registrations failing final eligibility shall not enter the authoritative final population.
- Incomplete authoritative inputs shall prevent finalisation until resolved.
- A finalised population shall not be silently changed without the applicable governed amendment/reopening path.

## 7. Audit & Evidence Behavior
Preserve population criteria, candidate eligibility, exclusions, finalisation authority, final population, and historical changes.

## 8. Observable Completion Conditions
Completion occurs when the authoritative eligible registration population is finalised, traceable, and available for downstream examination processing.

## 9. Traceability & Provenance
**Parent:** L3-I-036 — Finalize Examination Registration Population for Downstream Examination Processing

## 10. Open Issues / Deferred Design
Exact downstream consumption mechanisms and technical synchronization remain outside L4.