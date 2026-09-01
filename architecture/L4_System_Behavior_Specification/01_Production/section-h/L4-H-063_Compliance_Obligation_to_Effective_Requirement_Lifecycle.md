# Compliance Obligation to Effective Requirement Lifecycle

## 1. Specification Metadata
**Specification ID:** L4-H-063  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accreditation & Compliance  
**Realizes Scenario:** L3-H-063  
**Primary Business Owner:** Accreditation & Compliance  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-063; L2-H09 — Accreditation & Compliance Workflows; `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`; `L2-H09_Compliance_Requirement_Expiry_Reconciliation_Amendment_2026-08-27.md`; applicable Locked obligation/source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an H09 Compliance Requirement from identification through applicability, active monitoring and its authoritative satisfaction, waiver, remediation, expiry-review, retirement or closure outcomes while preserving source-obligation ownership and complete history.

## 3. Required Business Context
A legal, regulatory, accreditation or institutional obligation has been identified or changed; an accountable Business Owner can be assigned; H09 owns the Compliance Requirement record/lifecycle; the Accreditation and Compliance Governance Authority owns governed waiver, retirement and applicable post-expiry decisions.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-063-BH-001 | The system shall establish the Compliance Requirement in `Identified`, classify applicability, and when applicable progress it `Identified → Applicable → Active` with one accountable Business Owner and governed evidence obligations. | L3-H-063 §§6–9,12–13 |
| L4-H-063-BH-002 | When authoritative evidence demonstrates fulfillment, the system shall transition `Active → Satisfied`, publish `ComplianceRequirementSatisfied`, and permit later `Satisfied → Closed` when formal completion conditions apply. | L3-H-063 §§9,12–14 |
| L4-H-063-BH-003 | When a waiver is requested, the system shall transition `Applicable` or `Active → Waiver Requested` and publish `ComplianceWaiverRequested`; an authorized approval shall transition to `Waived` and publish `ComplianceRequirementWaived`. | L3-H-063 §§6,9–10,12–15 |
| L4-H-063-BH-004 | When noncompliance is identified, the system shall transition `Active → Noncompliant`; only governed remediation supported by authoritative evidence may return the requirement `Noncompliant → Active`. | L3-H-063 §§9,11–13 |
| L4-H-063-BH-005 | When the governed validity period ends, the system shall transition `Applicable`, `Active` or `Satisfied → Expired`, publish `ComplianceRequirementExpired`, then transition `Expired → Under Review` and publish `ComplianceRequirementExpiryReviewStarted`. | L3-H-063 §§9–10,12–14 |
| L4-H-063-BH-006 | After expiry review, the system shall transition the Requirement to `Retired`, `Applicable`, or `Active` according to the authoritative continuation decision, or establish a governed successor Requirement/version when materially revised. | L3-H-063 §§9–13,15 |
| L4-H-063-BH-007 | When an applicable current Requirement version is formally withdrawn or superseded independently of expiry, the system shall transition that version to `Retired` and publish `ComplianceRequirementRetired` while preserving predecessor history. | L3-H-063 §§9–14 |
| L4-H-063-BH-008 | The system shall keep `Expired` and `Retired` distinct: expiry shall never be treated as satisfaction, waiver or retirement without the required governed review/decision. | L3-H-063 §§3,12–13,17 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-063-BH-009 | H09 shall publish `ComplianceWaiverRequested`, `ComplianceRequirementWaived`, `ComplianceRequirementRetired`, `ComplianceRequirementSatisfied`, `ComplianceRequirementExpired`, and `ComplianceRequirementExpiryReviewStarted` for the corresponding H09-owned lifecycle facts. | L3-H-063 §14 |
| L4-H-063-BH-010 | The underlying source obligation and operational evidence shall remain owned by `VAR:Obligation-Owning Domain` or its resolved canonical owner; H09 governance shall not transfer that ownership. | L3-H-063 §§5,11–14,16 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-063-BH-011 | Missing evidence shall prevent satisfaction, waiver or expiry-review decisions from being represented as complete. | L3-H-063 §11 |
| L4-H-063-BH-012 | A waiver denial shall leave the Requirement on its applicable non-waived lifecycle; failed remediation shall not return a `Noncompliant` Requirement to `Active`. | L3-H-063 §§10–12 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-063-BH-013 | The system shall preserve obligation source, classification, applicability, accountable Business Owner, monitoring evidence, satisfaction/waiver/remediation, expiry and expiry-review evidence, renewal/continuation, retirement/successor decisions and complete lifecycle history. | L3-H-063 §15 |

## 8. Observable Completion Conditions
The behavior is complete when the Compliance Requirement reaches its applicable governed H09 outcome—such as `Active`, `Satisfied`, `Closed`, `Waived`, `Noncompliant`, `Expired`/`Under Review`, or `Retired`—with the applicable authority/evidence established and source-obligation ownership unchanged.

## 9. Traceability & Provenance
**Parent:** L3-H-063 — Compliance Obligation to Effective Requirement Lifecycle  
**Primary verification source:** L2-H09 — Accreditation & Compliance Workflows  
**Expiry reconciliation:** `L2-H09_Compliance_Requirement_Expiry_Reconciliation_Amendment_2026-08-27.md`

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. The locked expiry reconciliation keeps `Expired` distinct from `Retired`; L4 does not collapse those states. L5/L6 design remains deferred.
