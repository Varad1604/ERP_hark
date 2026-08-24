# Compliance Finding to Verified CAPA Closure

## 1. Scenario Metadata

**Scenario ID:** L3-H-067  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Remediation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Govern a compliance finding and its CAPA from approved finding through root-cause analysis, source-domain remediation evidence, verification, effectiveness review and final closure.

## 3. Business Outcome

Root cause, corrective and preventive actions, evidence, effectiveness review, reopening where needed, and final closure are governed end to end

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An audit, assessment, review, or monitoring activity records a nonconformity or observation.

### 4.2 End Boundary

Root cause, corrective and preventive actions, evidence, effectiveness review, reopening where needed, and final closure are governed end to end.

### 4.3 In Scope

- H09-owned governance behaviour materially required to reach the frozen business outcome.
- Applicable source-domain references, evidence and governed hand-offs without ownership transfer.
- Only upstream-defined rules, lifecycle positions, events, approvals and evidence obligations relevant to this scenario.

### 4.4 Out of Scope

- Operational business-state ownership belonging to participating source domains.
- Adjacent downstream institutional outcomes after this scenario's End Boundary.
- Later architecture-level implementation concerns.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance (H) | Owns the H09 governance outcome and applicable H09 lifecycles. |
| VAR:Remediation-Owning Domain | Owns correction of its operational business state and returns execution evidence to H09. |

Participation and evidence contribution do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Participates in finding/CAPA governance and effectiveness review. | Reviews |
| Compliance Cell | Coordinates finding and CAPA governance. | Performs / reviews |
| Department Head | Participates in finding/CAPA responsibility. | Participates |
| Action Owner | Executes assigned CAPA actions and provides evidence. | Performs |
| Internal Auditor | Participates in verification where applicable. | Verifies |

## 7. Preconditions and Entry Criteria

- A governed observation/nonconformity exists and can support a Finding.

## 8. Trigger

An audit, assessment, review, or monitoring activity records a nonconformity or observation

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register, classify, validate and approve the Finding. | Accreditation & Compliance | An approved Finding exists. |
| 2 | Create the CAPA, link its source, perform root-cause analysis and define corrective/preventive actions. | Accreditation & Compliance | A governed CAPA plan exists. |
| 3 | Assign the Action Owner and approve the CAPA plan. | Accreditation & Compliance | The CAPA enters assigned/implementation activity. |
| 4 | Execute operational corrections in `VAR:Remediation-Owning Domain` and return authoritative execution evidence. | VAR:Remediation-Owning Domain | Operational correction remains source-domain owned; evidence is returned to H09. |
| 5 | Verify implementation and assess effectiveness. | Accreditation & Compliance | Effectiveness is reviewed. |
| 6 | Approve and close the CAPA when effectiveness is verified. | Accreditation & Compliance | CAPA is Closed under the supported normal lifecycle. |

> The authoritative flow cannot be extended to the frozen End Boundary without resolving the material issue in Section 17.1.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Effectiveness review indicates remediation is insufficient and reopening is needed | Frozen Stage 3 requires governed reopening, but H09 CAPA defines no reopened state or transition/event. | Do not invent a reopening path; upstream resolution is required. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-023 — Critical Findings require CAPA creation. | Accreditation & Compliance | Requires CAPA for Critical findings. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-026 — CAPA closure requires effectiveness verification. | Accreditation & Compliance | Prevents closure before effectiveness review. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-027 — CAPA ownership remains assigned throughout its lifecycle. | Accreditation & Compliance | Preserves CAPA accountability. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |
| BR-H09-028 — completed CAPAs retain complete implementation history. | Accreditation & Compliance | Preserves remediation evidence/history. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Finding | Identified / Validated / Approved | Finding initiates CAPA and is governed through implementation/verification. | Closed | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A07 |
| CAPA | Created | Assignment, implementation, evidence, verification and effectiveness review. | Closed | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A08 / §7 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Finding Created | Finding aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Finding Approved | Finding aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| CAPA Created | CAPA aggregate | Governance reviewers | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| CAPA Assigned | CAPA aggregate | Governance reviewers | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| CAPA Approved | CAPA aggregate | Governance reviewers | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| CAPA Milestone Completed | CAPA aggregate | Governance reviewers | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| CAPA Verified | CAPA aggregate | Governance reviewers | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| CAPA Closed | CAPA aggregate | Governance reviewers | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Finding Closed | Finding aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Accreditation & Compliance | VAR:Remediation-Owning Domain | CAPA Execution Contract | Source domain corrects its operational state under the governed CAPA responsibility. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.3 |
| VAR:Remediation-Owning Domain | Accreditation & Compliance | CAPA execution evidence and authoritative business reference | Return evidence for H09 effectiveness verification and closure. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.3 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| CAPA closure | Accountable governance reviewer / Action Owner | H09 requires effectiveness agreement before closure. | Accreditation & Compliance |

### 15.2 Separation of Duties, Overrides and Exceptions

H09 requires the accountable governance reviewer and responsible action owner to agree on CAPA effectiveness for closure.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| CAPA implementation | Root-cause analysis, action plan and source-domain execution evidence | Accreditation & Compliance |
| CAPA closure | Effectiveness assessment and closure decision | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The frozen outcome requires verified effectiveness, governed reopening where needed, and final closure.
- Normal verified closure is supported, but the required reopening path is undefined upstream.

### 16.2 Postconditions

- Operational corrections remain owned by `VAR:Remediation-Owning Domain`.
- H09 retains the Finding/CAPA governance history and closure decision.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen Stage 3 requires `reopening where needed`; H09 CAPA has no Reopened state, transition, or canonical reopen event. | A required failure/recovery branch of the frozen outcome cannot be authored without inventing lifecycle semantics. | Define CAPA reopening semantics upstream in H09. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
