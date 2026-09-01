# Accreditation Framework Change to Active Governed Framework

## 1. Specification Metadata
**Specification ID:** L4-H-061  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accreditation & Compliance  
**Realizes Scenario:** L3-H-061  
**Primary Business Owner:** Accreditation & Compliance  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-061; L2-H09 — Accreditation & Compliance Workflows; `L2-H09_Framework_and_Compliance_Review_Normalization_Amendment_2026-09-01.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern a new or revised Accreditation Framework through the canonical H09 review, approval, publication and activation path while preserving historical framework immutability.

## 3. Required Business Context
A framework proposal or revision exists with criteria, evidence requirements, scoring, applicability and governance context sufficient for review.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-061-BH-001 | The system shall establish the candidate framework in `Draft` and progress it only through the canonical path `Draft → Under Review → Approved → Published → Active`. | L3-H-061 §§9,13; L2-H09 normalization amendment §1 |
| L4-H-061-BH-002 | Historical wording `Review` shall be interpreted as the same `Under Review` position and shall not create a competing lifecycle state. | L2-H09 normalization amendment §1 |
| L4-H-061-BH-003 | Invalid or unapproved changes shall not become Active; required revision shall return through the governed review path without modifying the current Active version in place. | L3-H-061 §§10–13 |
| L4-H-061-BH-004 | Only one applicable framework version may be Active for the governed scope, and historical framework versions shall remain immutable. | L3-H-061 §§12,16; L2-H09 normalization amendment §1 |

## 5. Cross-Domain & Event Behavior
The system shall preserve the exact parent-supported framework events, including `Accreditation Framework Created`, `Accreditation Framework Updated`, `Accreditation Framework Submitted for Review`, `Accreditation Framework Approved`, `Accreditation Framework Published` and `Accreditation Framework Activated`, from the H09-owned framework aggregate.

## 6. Failure / Rejection / Continuation
A framework that fails review or lacks required approval shall not progress to Published or Active. Successor changes to an Active framework require a new version.

## 7. Audit & Evidence Behavior
The system shall preserve framework version, criteria/evidence/scoring definition, review history, approval evidence, publication/activation history and immutable predecessor lineage.

## 8. Observable Completion Conditions
Completion requires the successor framework version to be approved, published and Active through the canonical `Under Review` path, with exactly one applicable Active version and immutable historical versions.

## 9. Traceability & Provenance
**Parent:** L3-H-061 — Accreditation Framework Change to Active Governed Framework  
**Primary verification sources:** L2-H09 — Accreditation & Compliance Workflows; L2-H09 Framework and Compliance Review Normalization Amendment — 2026-09-01.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Implementation and UI design remain deferred.
