# Research Activity to Ethics Decision and Continuing Compliance

## 1. Specification Metadata
**Specification ID:** L4-H-033  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-033  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-033; L2-H05 — Research Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern Research Ethics submission, review, approval/rejection, revision, renewal, continuing-compliance restriction and closure using only the locked H05 lifecycle and authority model.

## 3. Required Business Context
Research activity requiring Ethics oversight; mandatory evidence; committee/reviewer configuration; E03 researcher references; existing approved Ethics record and renewal window where applicable.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-033-BH-001 | Ethics review shall follow `Submitted → Under Review → Approved`, with `Under Review → Revision Required → Submitted` and `Under Review → Rejected` branches. | L3-H-033 §§9–13 |
| L4-H-033-BH-002 | Governed renewal shall require a valid existing `Approved` decision and configured renewal-window eligibility, create a successor approval version extending validity, preserve predecessor history and shall not introduce a `Renewed` lifecycle state. | L3-H-033 §§7,9–13 |
| L4-H-033-BH-003 | Continuing-compliance governance may move `Approved → Suspended → Approved`, `Approved / Suspended → Revoked`, or `Approved → Closed` only under the locked H05 conditions; `Revoked` is terminal for that approval record. | L3-H-033 §§9–13 |
| L4-H-033-BH-004 | Invalid/out-of-window renewal shall not extend validity; suspension, revocation or closure shall preserve all prior review, approval and renewal evidence. | L3-H-033 §§10–12 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-033-BH-005 | Research shall publish `EthicsApprovalRenewed`, `EthicsRevisionRequired`, `EthicsApprovalSuspended`, `EthicsApprovalReinstated`, `EthicsApprovalRevoked` and `EthicsApprovalClosed` for their corresponding facts; existing submission/review/approval/rejection events remain Research-owned. | L3-H-033 §14 |

## 6. Failure / Rejection / Continuation
Incomplete evidence, unauthorized review or invalid renewal conditions prevent progression. A revoked approval is not locally reopened; future approval requires a new governed submission.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-033-BH-006 | The system shall preserve approval version/validity, renewal request/eligibility, reviewers, conditions, decisions, suspension/reinstatement/revocation/closure rationale and complete immutable history. | L3-H-033 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-033-BH-007 | Completion shall require the Ethics Approval to reach the applicable governed approved, rejected, renewed-validity, suspended, reinstated, revoked or closed outcome with required evidence preserved and E03 ownership unchanged. | L3-H-033 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-033 — Research Activity to Ethics Decision and Continuing Compliance  
**Primary verification source:** Locked H05 governance/lifecycle amendment and base H05 renewal workflow.

## 10. Open Issues / Deferred Design
Historical blockers were resolved upstream; no parallel renewal state or new authority is introduced.