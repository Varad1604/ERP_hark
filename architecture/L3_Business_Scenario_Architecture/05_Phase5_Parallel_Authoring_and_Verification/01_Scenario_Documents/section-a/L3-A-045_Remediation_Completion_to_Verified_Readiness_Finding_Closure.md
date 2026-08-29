# Remediation Completion to Verified Readiness Finding Closure

# 1. Scenario Metadata

**Scenario ID:** L3-A-045  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Operational Readiness Governance & Certification  
**Participating Domains:** A, VAR:Remediation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** undefined  
**Referenced By:** Assessment completion, certification eligibility and remediation governance  

---

# 2. Scenario Purpose

Verify externally completed remediation and either close verified Findings or reopen them for further corrective work without losing history.

---

# 3. Business Outcome

Remediation evidence is verified against governance policy and the affected findings are either verified and closed or formally reopened for additional remediation, with complete immutable governance history preserved

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A responsible business domain reports completion of corrective activity for one or more readiness findings.

## 4.2 End Boundary

Each in-scope Finding is either `Closed` after successful verification and approval or `Reopened` and returned to remediation, with the complete history preserved.

## 4.3 In Scope

- Completion report and remediation evidence reference.
- Plan and Finding verification.
- Policy outcome and lifecycle update.
- Verified closure or formal reopening.
- Events and immutable history.

## 4.4 Out of Scope

- Dynamic operation-specific Capability Readiness Evaluation and runtime `READY`/`NOT READY` outcomes.
- Operational remediation work owned by collaborating business domains.
- Authentication, authorization, event transport or audit-storage implementation.
- Technical APIs, databases, UI, deployment and infrastructure design.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|

| Upstream | L3-A-044 | Supplies Findings and Improvement Plan. |
| Followed by | L3-A-049 | Uses resolved Findings for governance completion. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|

| Operational Readiness Governance & Certification / Section A | Owns assessments, requirements, verification outcomes, findings, improvement plans, governance decisions and certificates. |
| Platform & Tenant / Section A | Supplies Tenant and operational-scope context. |
| Authorization & Policy / Section A | Supplies authority and policy context without owning readiness artifacts. |
| Workflow Engine / Section A | Coordinates workflow execution without owning governance decisions. |
| Event Infrastructure / Section A | Distributes canonical readiness events. |
| Audit Infrastructure / Section A | Preserves immutable governance evidence. |
| VAR:Remediation-Owning Domain | Owns corrective execution and submits completion evidence. |

The frozen Participating Domains value remains exactly `A, VAR:Remediation-Owning Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Boundary |
|---|---|---|

| Governance Authority | Authorizes assessments, verifies evidence, acknowledges findings and governs certificates. | Owns governance decisions, not operational remediation. |
| Responsible Domain | Owns operational evidence and remediation execution and reports completion. | Does not modify readiness governance artifacts directly. |
| System | Applies approved policy, monitors significant events and initiates governed reassessment triggers. | Does not create unsupported business decisions. |

---

# 7. Preconditions and Entry Criteria

- An active Improvement Plan exists.
- One or more associated Findings are under remediation/resolution submitted.
- The Responsible Domain reports completion.
- Supporting evidence references are available.
- Verification policy and Governance Authority are available.

---

# 8. Trigger

A responsible business domain reports completion of corrective activity for one or more readiness findings

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|

| 1 | Receive the remediation completion report and evidence references. | Responsible Domain | Governance verification begins. |
| 2 | Resolve the Improvement Plan and affected Findings. | Improvement Governance Service | The verification scope is fixed. |
| 3 | Transition the plan/finding context to Under Verification. | Assessment aggregate | Lifecycle reflects active verification. |
| 4 | Validate evidence accessibility and review corrective outcome. | Evidence Verification Service; Governance Authority | Evidence quality is established. |
| 5 | Evaluate policy compliance for each affected Finding. | Improvement Governance Service | A deterministic verification outcome is recorded. |
| 6 | On success, transition plan to Completed and Findings to Verified. | Assessment aggregate | Remediation is governance-verified. |
| 7 | Confirm governance approval and close only Verified Findings. | Governance Authority; Assessment aggregate | `ReadinessFindingClosed` becomes publishable. |
| 8 | On failed verification, transition affected Findings/Plan to Reopened and return to In Progress. | Assessment aggregate; Responsible Domain | Additional remediation is required. |
| 9 | Publish `ReadinessImprovementVerified` only for the source-defined verified outcome. | Assessment aggregate | Successful verification is announced. |
| 10 | Preserve all submissions, outcomes, transitions and audit history. | Assessment aggregate; Audit Infrastructure | No prior record is rewritten. |

---

# 10. Alternate Business Flows

| Case | Condition | Path | Outcome |
|---|---|---|---|

| Verification succeeds | Evidence satisfies policy. | Complete plan, verify and close approved Findings. | Terminal closure occurs. |
| Verification fails | Evidence/remediation is incomplete. | Reopen affected Finding/Plan and return to remediation. | No closure or success event is asserted. |
| Partial success | Only some Findings satisfy policy. | Close verified eligible Findings and reopen the others. | Per-Finding history remains distinct. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Governed Resolution | Owner |
|---|---|---|---|

| Plan/Finding mismatch | Verification scope is invalid. | Reject report for correction. | Improvement Governance Service |
| Evidence inaccessible | Verification cannot complete. | Request additional evidence. | Evidence Verification Service |
| Closure approval absent | Verified Finding cannot close. | Remain Verified pending approval. | Governance Authority |
| History modification attempted | Audit integrity is threatened. | Reject update and append a new outcome. | Assessment aggregate |

No technical retry, rollback, persistence or deployment mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|

| Verification is evidence- and policy-driven. | Completion reports alone cannot close Findings. | L2-A10 §§26.3, 36.5 |
| Only Verified Findings may close. | Closure requires governance approval. | L2-A10 §26.4 |
| Failed verification may reopen Findings. | Reopened history remains immutable. | L2-A10 §§18.4, 19.4 |
| Operational remediation remains external. | A10 verifies but does not perform corrective work. | L2-A10 §26.3 |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|

| Improvement Plan | In Progress/Resolution Submitted | Under Verification → Completed → Closed, or Reopened → In Progress. | Closed or active again. |
| Finding | Resolution Submitted | Under Verification → Verified → Closed, or Reopened. | Closed or returned to remediation. |
| Evidence Reference | Submitted | Verify and append outcome. | Immutable history. |
| Assessment | Active | Update governance status only. | Remains governed. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Condition |
|---|---|---|

| `ReadinessImprovementVerified` | Operational Readiness Assessment aggregate | Improvement successfully verifies. |
| `ReadinessFindingClosed` | Operational Readiness Assessment aggregate | A Verified Finding closes. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|

| Completion report/evidence | VAR:Remediation-Owning Domain | Improvement Governance Service | Claimed completion and references. |
| Verification outcome | Evidence/Improvement services | Assessment aggregate | Per-Finding policy result. |
| Closure approval | Governance Authority | Assessment aggregate | Authority to close. |
| Reopened obligation | Assessment aggregate | Responsible Domain | Additional remediation requirement. |
| Audit input | Assessment aggregate | Audit Infrastructure | All evidence and transitions. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Assessment, verification, finding, certification and lifecycle actions require authorized governance actors.
- Authorization decisions remain owned by L2-A04.
- A certificate is governance evidence; it does not authorize every operation.

## 15.2 Approval and Separation of Duty

- Governance Authority provides the approvals explicitly required by L2-A10.
- Responsible Domains may submit evidence and remediation completion but do not approve their own governance outcome.
- Policy-approved waivers and certificate transitions must remain traceable.

## 15.3 Audit Obligations

- Record Tenant, governed scope, actor, policy, command, prior/resulting state, decision, timestamps and correlation.
- Preserve assessment lineage, evidence references, findings, plans, certificate lineage and every governance outcome.
- A10 owns readiness facts; A07 owns immutable audit evidence.

---

# 16. Completion Criteria and Postconditions

Completion requires:

- The active Plan, Findings and Responsible Domain were resolved.
- Completion evidence was verified against policy.
- Each Finding received a deterministic outcome.
- Successful Findings became Verified and closed only with approval.
- Unsuccessful Findings were formally reopened for remediation.
- Source-defined events and immutable history were preserved.

Postconditions:

- Closed Findings remain queryable and immutable.
- Reopened work remains owned by the responsible domain.
- Assessment/certification completion proceeds only when obligations are satisfied.

---

# 17. Open Issues, Assumptions and Deferred Questions


| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|

| A045-ISSUE-01 | Non-blocking | L2-A10 defines Reopened lifecycle states and permits failed verification to reopen Findings, but provides no explicit ReopenReadinessFinding command or canonical reopening event. | Use the defined lifecycle outcome without inventing a command/event; add them before L4 if asynchronous reopening is required. |

No missing contract is supplied by inference.


