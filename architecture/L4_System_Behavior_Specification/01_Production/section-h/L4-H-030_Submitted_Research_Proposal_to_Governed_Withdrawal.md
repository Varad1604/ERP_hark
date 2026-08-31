# Submitted Research Proposal to Governed Withdrawal

## 1. Specification Metadata
**Specification ID:** L4-H-030  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-030  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-030; L2-H05 — Research Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern withdrawal of an eligible Research Proposal before the downstream project boundary while preserving all prior proposal/review/approval evidence.

## 3. Required Business Context
Proposal in `Submitted`, `Under Review` or eligible `Approved`; authorized requestor; withdrawal reason; no established Research Project/irreversible downstream fact for Approved withdrawal.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-030-BH-001 | On an authorized withdrawal request, the system shall validate requestor authority, reason, current Proposal state and, for `Approved`, absence of an established Research Project or other irreversible downstream fact. | L3-H-030 §§7–9 |
| L4-H-030-BH-002 | An eligible `Submitted`, `Under Review` or `Approved` Proposal shall move to `Withdrawal Requested` and, after the governed decision, to `Withdrawn`. | L3-H-030 §§9,12–13 |
| L4-H-030-BH-003 | Once a Research Project exists, Proposal withdrawal shall not be used; subsequent disruption shall be governed by the Research Project lifecycle. | L3-H-030 §§10,12 |
| L4-H-030-BH-004 | `Withdrawn` shall be terminal for that Proposal record and shall preserve all prior submission, review, approval and withdrawal history. | L3-H-030 §§11–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-030-BH-005 | Research shall publish `ResearchProposalWithdrawalRequested` and `ResearchProposalWithdrawn`; E03 workforce facts remain referenced and unchanged. | L3-H-030 §14 |

## 6. Failure / Rejection / Continuation
Invalid authority or lifecycle position shall reject the withdrawal without changing Proposal state or deleting evidence.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-030-BH-006 | The system shall preserve requestor, authority basis, reason, prior state, Research Director decision where required, resulting state and complete Proposal lineage. | L3-H-030 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-030-BH-007 | Completion shall require an eligible Proposal to reach `Withdrawn`, applicable Research events to be published, prior history preserved and further normal Proposal progression prevented. | L3-H-030 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-030 — Submitted Research Proposal to Governed Withdrawal  
**Primary verification source:** Locked H05 governance/lifecycle amendment.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation.