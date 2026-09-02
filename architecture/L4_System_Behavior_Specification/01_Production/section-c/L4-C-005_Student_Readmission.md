# Student Readmission

## 1. Specification Metadata

**Specification ID:** L4-C-005  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-005 — Student Readmission  
**Primary Business Owner:** Admissions  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-C-005; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

the system shall govern governed readmission of an eligible applicant against an inactive historical Admission, using the configured institutional strategy to restore or create the appropriate admission lifecycle. The behavior ends when the readmission outcome is complete and `AdmissionReadmitted` is published.

## 3. Required Business Context

- Historical Admission exists and is inactive.
- Requester satisfies applicable authentication/authorization requirements.
- Readmission policy and eligibility are resolved.
- Duplicate active admissions are prohibited.
- Admissions owns the readmission outcome and event.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-005-BH-001 | When a readmission request is received, the system shall validate the historical Admission, requester authority and applicable eligibility before establishing an outcome. | L3-C-005 §§7, 9, 12; L2-C01 WF-C01-09 |
| L4-C-005-BH-002 | When policy resolves to restoration and eligibility is satisfied, the system shall apply the authorized restoration outcome. | L3-C-005 §§9, 10, 13; L2-C01 |
| L4-C-005-BH-003 | When policy requires a new admission lifecycle, the system shall establish the new Admission while preserving the historical Admission relationship. | L3-C-005 §§9, 10, 13; L2-C01 |
| L4-C-005-BH-004 | The system shall prevent establishment of a duplicate active admission. | L3-C-005 §§7, 11, 12; L2-C01 |
| L4-C-005-BH-005 | The system shall preserve historical Admission information regardless of readmission strategy. | L3-C-005 §§2, 3, 11, 13; L2-C01 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-005-BH-006 | On successful readmission, the system shall publish the canonical `AdmissionReadmitted` business event under Admissions ownership. | L3-C-005 §§3, 14; L2-C01 |
| L4-C-005-BH-007 | The system shall use Academic Delivery session/programme information only as eligibility context and shall not transfer Admission ownership. | L3-C-005 §§5, 14; L2-C01 |

## 6. Failure / Rejection / Continuation

- Prevent readmission when eligibility or policy conditions fail.
- Preserve the historical Admission when a request is rejected.
- Prevent duplicate active admissions.
- Authentication/authorization failure shall prevent corresponding request progression.
- Readmission shall not overwrite historical Admission identity/history.

## 7. Audit & Evidence Behavior

Preserve attributable evidence of the readmission request, eligibility/strategy outcome and resulting Admission relationship/history through applicable Admissions and Audit capabilities.

## 8. Observable Completion Conditions

The configured restoration or new-admission strategy completes, historical linkage is preserved, no duplicate active admission exists, required evidence is available, and `AdmissionReadmitted` has been published.

## 9. Traceability & Provenance

**Parent:** L3-C-005 — Student Readmission  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
