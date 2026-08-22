# Compliance Finding to Verified CAPA Closure

# 1. Scenario Metadata

**Scenario ID:** L3-H-067  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Remediation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Compliance review, audit, readiness, and quality-governance scenarios

# 2. Scenario Purpose

Govern a compliance or quality finding through root-cause analysis, corrective and preventive action planning, accountable execution, evidence submission, effectiveness verification, closure approval, and final CAPA closure while preserving ownership of remediation work in the responsible business domain.

# 3. Business Outcome

Root cause, corrective and preventive actions, evidence, effectiveness review, reopening where needed, and final closure are governed end to end

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An audit, assessment, review, or monitoring activity records a nonconformity or observation.

## 4.2 End Boundary

H09 supports CAPA creation through verified closure. The frozen outcome additionally requires reopening where needed, but H09 defines no CAPA reopen state, transition, rule, action, authority, or event. The full frozen end boundary therefore cannot be represented authoritatively.

## 4.3 In Scope

- Finding/observation linkage to a CAPA.
- Root-cause analysis.
- Corrective and preventive action definition.
- Action-owner assignment and milestones.
- CAPA plan approval.
- Remediation execution by the accountable domain.
- Supporting evidence, implementation verification, effectiveness review, closure approval, and closure.

## 4.4 Out of Scope

- Inventing a CAPA reopen lifecycle.
- Transferring ownership of operational remediation into H09.
- Modifying the source finding, observation, risk, or operational record outside its canonical owner.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns the CAPA aggregate, CAPA governance lifecycle, verification, effectiveness review, closure approval, and CAPA history. |
| VAR:Remediation-Owning Domain | Owns the underlying business changes or corrective execution required to remediate the issue. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Participates in CAPA governance and quality oversight. | Reviews |
| Compliance Cell | Coordinates compliance remediation and follow-up. | Reviews / coordinates |
| Department Head | Participates where remediation belongs to a departmental scope. | Reviews / accountable participation |
| Action Owner | Executes the approved remediation actions and maintains progress/evidence. | Performs |
| Internal Auditor | Participates in verification where the CAPA originates from audit activity. | Reviews / verifies |
| Accountable governance reviewer | Agrees with the responsible action owner on CAPA effectiveness before closure. | Closure approval participant |

# 7. Preconditions and Entry Criteria

- A source-supported Finding, Observation, Compliance Gap, Management Review, Quality Review, Institutional Risk, Policy Violation, Strategic Review, Institutional Review, or equivalent approved governance trigger exists.
- A responsible remediation owner can be assigned.

# 8. Trigger

An audit, assessment, review, or monitoring activity records a nonconformity or observation

# 9. Main Business Flow

1. H09 creates a CAPA linked to the source governance trigger.
2. The responsible participants perform root-cause analysis.
3. Corrective and preventive actions are defined.
4. H09 assigns the Action Owner and governed milestones.
5. The CAPA plan is approved through the H09 governance process.
6. The remediation-owning domain executes the approved business remediation without transferring its operational ownership to H09.
7. Supporting evidence is submitted to the CAPA record.
8. H09 verifies implementation.
9. H09 assesses effectiveness against the governed CAPA objective.
10. The accountable governance reviewer and responsible action owner agree on effectiveness as required for closure approval.
11. H09 closes the CAPA and preserves its complete implementation history.
12. If later evidence requires reopening, stop: no authoritative H09 reopen transition exists.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| CAPA effectiveness is not established | Verification or effectiveness review is unsuccessful | Keep the CAPA outside `Closed` and continue governed remediation/verification | Closure withheld | Accreditation & Compliance / remediation owner |
| CAPA becomes overdue | Approved milestones are not met | Record the source-defined overdue condition and continue escalation/follow-up | CAPA remains open | Accreditation & Compliance |
| Reopening is required | A previously closed CAPA is later shown to be ineffective | Upstream architecture must define the reopen path | Frozen alternate branch blocked | Accreditation & Compliance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Remediation owner is not assigned | CAPA cannot progress validly | Establish the responsible action owner | Accreditation & Compliance / remediation owner |
| Supporting evidence is insufficient | Effectiveness cannot be verified | Continue remediation and provide sufficient evidence | Remediation-owning domain |
| Closure approval lacks effectiveness agreement | Closure condition is not met | Continue review until the accountable reviewer and responsible action owner agree on effectiveness | Accreditation & Compliance / action owner |
| Reopen condition occurs after closure | Frozen outcome requires a lifecycle branch not defined upstream | Escalate the architecture gap; do not invent a local state | Accreditation & Compliance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every CAPA originates from an approved governance trigger | H09 | CAPA must preserve source-trigger lineage | L2-H09 BR-H09-025 |
| CAPA closure requires effectiveness verification | H09 | Closure cannot precede verified effectiveness | L2-H09 BR-H09-026 |
| CAPA ownership remains assigned throughout its lifecycle | H09 | Responsible ownership is mandatory | L2-H09 BR-H09-027 |
| Completed CAPAs retain complete implementation history | H09 | Closure history remains traceable | L2-H09 BR-H09-028 |
| CAPA closure approval requires the accountable governance reviewer and responsible action owner to agree on effectiveness | H09 | Final closure requires source-defined joint effectiveness agreement | L2-H09 §2.5 Approval Responsibilities |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| CAPA | Created | Approve and assign governed remediation | Approved → Assigned | H09 | L2-H09 H09-A08 |
| CAPA | Assigned | Execute remediation | Implementation | H09 | L2-H09 H09-A08 |
| CAPA | Implementation | Preserve supporting proof | Evidence Submitted | H09 | L2-H09 H09-A08 |
| CAPA | Evidence Submitted | Verify implementation and effectiveness | Verification → Effectiveness Review | H09 | L2-H09 H09-A08 |
| CAPA | Effectiveness Review | Obtain valid closure approval | Closed | H09 | L2-H09 H09-A08; H09-10 |

H09 §7 omits the `Approved` and `Evidence Submitted` positions that are present in H09-A08. No reopen state is defined in either lifecycle representation.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CAPA Created | CAPA aggregate / H09 | Governance and remediation participants | Records CAPA creation | L2-H09 §11 |
| CAPA Assigned | CAPA aggregate / H09 | Action Owner and governance participants | Records accountable assignment | L2-H09 §11 |
| CAPA Approved | CAPA aggregate / H09 | Remediation participants | Records approved plan | L2-H09 §11 |
| CAPA Milestone Completed | CAPA aggregate / H09 | Governance participants | Records governed progress | L2-H09 §11 |
| CAPA Overdue | CAPA aggregate / H09 | Governance participants | Records overdue remediation | L2-H09 §11 |
| CAPA Verified | CAPA aggregate / H09 | Governance participants | Records successful verification | L2-H09 §11 |
| CAPA Closed | CAPA aggregate / H09 | Governance participants | Records valid closure | L2-H09 §11 |

No canonical CAPA reopened event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The remediation-owning domain executes its business correction under its own ownership and supplies completion/effectiveness evidence to H09. H09 governs the CAPA record and closure decision without taking ownership of the corrected source-domain lifecycle.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

CAPA plan approval and closure occur within H09 governance. Closure requires agreement on effectiveness between the accountable governance reviewer and the responsible action owner.

## 15.2 Separation of Duties, Overrides and Exceptions

No CAPA may be closed without effectiveness verification. Operational remediation remains owned by the responsible business domain.

## 15.3 Business Audit and Evidence Obligations

Preserve source-trigger lineage, root-cause analysis, approved actions, ownership, milestones, implementation evidence, verification record, effectiveness assessment, closure approval, and complete implementation history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The source-supported happy path completes when effectiveness is verified, closure approval is valid, and the CAPA reaches `Closed`. The full frozen scenario is not complete because a required reopening branch remains undefined upstream.

## 16.2 Postconditions

Closed CAPA history remains traceable; no unsupported reopen state is asserted.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen semantics require `reopening where needed`.
- H09 defines no CAPA reopen state, transition, rule, action, authority, or event.
- H09-A08 includes `Approved` and `Evidence Submitted`, while §7 omits them.

The missing reopen branch materially affects the frozen lifecycle outcome.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H09 must define whether a closed CAPA can reopen and, if so, the canonical transition, authority, state, and event semantics.
