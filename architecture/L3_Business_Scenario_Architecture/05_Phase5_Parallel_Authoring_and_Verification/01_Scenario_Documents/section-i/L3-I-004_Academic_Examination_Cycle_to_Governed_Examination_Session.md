# Academic Examination Cycle to Governed Examination Session

# 1. Scenario Metadata

**Scenario ID:** L3-I-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Session Management  
**Participating Domains:** B, I  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Review  
**Version:** 1.0  
**Owner:** Examination Session Management  
**Assigned Author:** Pallav  
**Last Updated:** 2026-08-24  
**Depends On:** `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I02_Examination_Session_Management_Workflows.md`; `architecture/L3_Business_Scenario_Architecture/04_Phase4_Canonical_Set_Construction/03_Canonical_Register/Section_I_001_070_Canonical_Register.md`; `DEPENDENCY_MAP.md`; `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** Known downstream Examination Session consumers identified by L2-I02 and `DEPENDENCY_MAP.md`; exact L3 scenario references only where canonically registered  

**Canonical Source Candidate:** CAN-L3-I-004  
**Primary L2 Source:** L2-I02  

---

# 2. Scenario Purpose

This scenario governs the establishment of a governed Examination Session for an academic examination cycle as one complete institutional business outcome. It applies the frozen L3 identity and locked L2-I02 architecture without redefining upstream ownership, policy, lifecycle, or authority.

# 3. Business Outcome

A governed Examination Session is established under the authoritative Examination Session Management lifecycle, with its institutional, academic-period, policy, applicability, approval, and activation conditions governed by L2-I02. The resulting session is available for downstream examination workflows only when it reaches the L2-I02 state required for operational consumption.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an academic examination cycle requires an Examination Session and the authoritative institutional, academic-period, and examination-policy context needed by L2-I02 is available.

## 4.2 End Boundary

The scenario ends when the Examination Session has been established through the applicable L2-I02 governance steps and the resulting authoritative session position is available for its permitted downstream use. Closure and archival are outside this scenario and remain owned by L2-I16.

## 4.3 In Scope

- Examination Session initiation and configuration.
- Association to an approved Examination Policy.
- Validation of academic applicability and institutional context.
- Review, approval, scheduling, and activation where required by L2-I02.
- Preservation of L2-I02 lifecycle integrity.
- Source-supported business events and cross-domain hand-offs.

## 4.4 Out of Scope

- Examination planning and scheduling execution owned by L2-I03.
- Candidate examination registration owned by L2-I04.
- Hall ticket, examination conduct, evaluation, results, revaluation, and other downstream examination outcomes.
- Examination Session closure and archival owned by L2-I16.
- Redefinition of Institution, Academic Calendar, Curriculum, Organisation, or Examination Policy facts.
- APIs, databases, user interfaces, technical access control, integration mechanisms, deployment, or other L4/L5 concerns.

## 4.5 Related Scenarios — Optional

Downstream Examination scenarios may consume the governed Examination Session according to their own canonical preconditions and ownership boundaries. No downstream lifecycle is absorbed into this scenario.

# 5. Ownership and Participating Domains

| Domain / Capability | Responsibility in Scenario |
|---|---|
| Examination Session Management | Owns the Examination Session lifecycle, governance, approval, activation, suspension, reopening, version management, and the governed scenario outcome. |
| Institution and Academic Structure (B) | Provides authoritative institutional, academic-period, curriculum/applicability, and organisational context through the applicable canonical owners; ownership does not transfer to Examination. |
| Examination (I) | Provides the governing Examination Policy and Examination Session Management responsibilities defined by L2-I01/L2-I02. |

The Primary Owner remains Examination Session Management. Participating domains retain ownership of their own facts and decisions.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in This Scenario | Authoritative Source |
|---|---|---|
| Examination Administrator | Creates and manages the Examination Session through its governed pre-operational lifecycle. | L2-I02 §6 |
| Controller of Examination | Exercises the source-defined approval and activation authority applicable to the Examination Session. | L2-I02 §6, §8 |
| Academic Administration | Provides academic-governance input for session applicability. | L2-I02 §6 |
| Institution Administration | Provides institutional examination-governance context. | L2-I02 §6 |
| Curriculum Domain | Provides programme and course applicability without transferring ownership. | L2-I02 §6, §13 |

Closure authority is not assigned in this scenario; session closure and archival are owned by L2-I16 under the repository ownership ruling recorded in L2-I02.

# 7. Preconditions and Entry Criteria

- The Institution context required by the Examination Session exists and is authoritative.
- The applicable academic period is published and valid for the intended session scope.
- An approved Examination Policy exists and can govern the session.
- Required programme, course, curriculum, and organisational applicability facts are authoritative where applicable.
- The Examination Session begins from a lifecycle position permitted by L2-I02.
- No unresolved ownership or lifecycle conflict prevents progression.

# 8. Trigger

An academic examination cycle requires establishment of a governed Examination Session.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Initiate the proposed Examination Session for the academic examination cycle. | Examination Administrator / Examination Session Management | A Draft Examination Session exists. |
| 2 | Configure the session purpose, classification, academic period, programme/curriculum applicability, organisational scope, and administrative ownership using authoritative source-domain facts. | Examination Administrator with participating-domain inputs | The session has a reviewable governed configuration. |
| 3 | Associate the approved Examination Policy governing the session. | Examination Session Management | The session is governed by an authoritative Examination Policy. |
| 4 | Review the configured session for policy compliance, academic applicability, administrative approval, institutional governance, and multi-domain consistency. | Source-supported review responsibilities | The session is eligible for approval or is returned/blocked. |
| 5 | Approve the Examination Session only when L2-I02 approval conditions are satisfied. | Controller of Examination / source-defined authority | The session reaches the approved lifecycle position. |
| 6 | Progress the approved session through the source-defined scheduled/activation lifecycle when its applicable conditions are satisfied. | Examination Session Management / Controller of Examination | The session reaches the L2-I02 position permitted for operational consumption. |
| 7 | Publish the exact L2-I02 business milestones produced by the completed lifecycle steps and make the governed session available to authorized downstream consumers. | Examination Session Management | Downstream workflows can consume the authoritative session without acquiring ownership. |

# 10. Alternate Business Flows

## 10.1 Missing or Invalid Academic Context

If the academic period, programme, curriculum, organisational scope, or other required source-owned applicability fact is unavailable or inconsistent, approval does not proceed. The owning domain resolves the authoritative fact before the scenario resumes.

## 10.2 Missing or Inactive Examination Policy

If an approved governing Examination Policy is not available, the session remains pre-operational and cannot proceed to activation until L2-I02 policy-association requirements are satisfied.

## 10.3 Approval Rejection or Incomplete Configuration

If institutional review rejects the session or mandatory configuration is incomplete, the session remains unavailable for operational consumption and follows the source-supported correction, cancellation, or replacement path.

## 10.4 Activation Preconditions Not Satisfied

If activation prerequisites are not satisfied, activation does not occur and downstream operational examination workflows remain blocked until the authoritative deficiency is resolved.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authoritative source fact becomes unavailable or inconsistent | Progression cannot be validated safely. | Preserve the last valid session position and obtain corrected authoritative input from its canonical owner. | Examination Session Management and source-owning domain |
| Approval is rejected | The session cannot progress to operational use. | Return for permitted modification, cancel, or replace according to L2-I02. | Examination Session Management |
| Activation prerequisite fails | The session remains unavailable to downstream examination workflows. | Resolve the unmet L2-I02 prerequisite and repeat the governed validation/approval step as applicable. | Examination Session Management and relevant source owner |
| A later authoritative baseline invalidates a pending configuration before activation | Activation could produce an inconsistent session. | Revalidate the affected applicability/policy facts before progression; do not apply stale approval silently. | Examination Session Management |

Technical retry, messaging, persistence, and recovery mechanisms are outside L3.

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Scenario Effect | Owning Domain / Source |
|---|---|---|
| Every Examination Session belongs to a single Institution. | The session cannot be established without authoritative institutional ownership. | Examination Session Management / L2-I02 §9.1 |
| Every Examination Session references an approved academic period. | Invalid or inactive academic-period context prevents approval. | Academic Calendar fact consumed by Examination / L2-I02 §9.1, §13.4 |
| Every Examination Session references one approved Examination Policy, and the policy must be active before activation. | Policy association is mandatory and policy ownership remains external to the session lifecycle. | Examination Policy / L2-I02 §9.2 |
| Activation requires completed institutional approval, policy association, validated academic applicability, assigned administrative ownership, and required governance activity. | A session cannot become operational until every source-defined activation condition is satisfied. | Examination Session Management / L2-I02 §9.4 |
| Only Active Examination Sessions may be consumed by downstream examination workflows. | Pre-operational or suspended sessions cannot authorize downstream operational processing. | Examination Session Management / L2-I02 §9.5 |
| Session closure and archival are not owned by L2-I02. | This scenario stops before the L2-I16 closure lifecycle. | Examination Session Management / L2-I02 ownership ruling |

No local threshold, deadline, formula, authority, lifecycle state, or business rule is introduced.

# 13. State and Lifecycle Impact

| Owned Business Concept | Relevant L2-I02 Lifecycle | Scenario Impact | Canonical Owner | Authoritative Source |
|---|---|---|---|---|
| Examination Session | Draft → Approved → Scheduled → Active, subject to the review and governance conditions defined by L2-I02 | Establishes the governed Examination Session and progresses it only through source-defined states and transitions needed for the scenario outcome. | Examination Session Management | L2-I02 §8, §20 |

Active sessions may later be Suspended and Reopened under L2-I02. Closure and archival states beginning with the hand-off to L2-I16 are outside this scenario. No hidden or synonymous lifecycle state is created here.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

### Published Events

| Business Event | Canonical Publisher | Published When | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Examination Session Created | Examination Session Management | The session is successfully initiated. | Expose the authoritative existence of the governed session. | L2-I02 §11 |
| Examination Session Approved | Examination Session Management | Institutional approval is completed. | Communicate the approved session milestone to permitted consumers. | L2-I02 §11 |
| Examination Session Activated | Examination Session Management | The session becomes operational. | Authorize downstream workflows that require an Active session. | L2-I02 §11 |

`Regular`, `Supplementary`, `Improvement`, `Makeup`, `Special`, and similar terms are Examination Session classifications, not business events.

### Consumed Events

| Business Event | Canonical Publisher | Business Purpose | Authoritative Source |
|---|---|---|---|
| Academic Period Published | Academic Calendar Domain | Establish authoritative academic-period context for the session. | L2-I02 §12 |
| Examination Policy Approved | L2-I01 | Establish the approved governing Examination Policy. | L2-I02 §12 |
| Curriculum Updated / Programme Structure Changed | Curriculum Framework Domain | Provide authoritative applicability context where relevant before approval/activation. | L2-I02 §12 |
| Institution Updated / Organisation Structure Updated | Owning Institution / Organisation domains | Provide authoritative institutional and organisational context where relevant. | L2-I02 §12 |

Only exact upstream events identified by L2-I02 are treated as business events.

## 14.2 Business Contracts and Cross-Domain Hand-offs

L2-I02 records multi-domain interactions but its canonical crosswalk explicitly identifies a missing formal **Contracts** section. Therefore this L3 scenario records only the business hand-offs directly supported by L2-I02 and does not invent contract names or technical interfaces.

| Provider / From Domain | Consumer / To Domain | Business Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institution / Organisation | Examination Session Management | Authoritative institutional ownership, governance, and organisational applicability facts | Establish the governed institutional scope of the session. | L2-I02 §13.1–13.2 |
| Curriculum Framework | Examination Session Management | Programme, course, and curriculum applicability facts | Establish authoritative academic applicability. | L2-I02 §13.3 |
| Academic Calendar | Examination Session Management | Academic period and examination-window context | Establish the authoritative academic time context. | L2-I02 §13.4 |
| Examination Policy (L2-I01) | Examination Session Management | Approved governing Examination Policy | Govern session approval and activation without transferring policy ownership. | L2-I02 §13.5 |
| Examination Session Management | Downstream Examination workflows | Approved/Active Examination Session context and source-defined lifecycle milestones | Permit governed downstream examination processing. | L2-I02 §§4.1, 5.4, 23 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Authority Constraint | Authoritative Source |
|---|---|---|---|
| Create and configure Examination Session | Examination Administrator | Operates within the institutional and examination governance defined by L2-I02. | L2-I02 §6 |
| Approve / activate Examination Session | Controller of Examination | May progress the session only after the source-defined review, policy, applicability, and governance conditions are satisfied. | L2-I02 §§6, 8, 9.4 |

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty, override, or exceptional authority is asserted by this scenario beyond the authoritative L2-I02 architecture. No assumed administrator override may bypass source-defined lifecycle conditions.

## 15.3 Business Audit and Evidence Obligations

The scenario preserves evidence of session creation, configuration, policy association, review, approval, activation, applicable source facts, responsible actors, lifecycle changes, and any governed exception or rejection required by L2-I02.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Examination Session is associated with an authoritative Institution and academic period.
- An approved governing Examination Policy is associated.
- Academic and organisational applicability is authoritative where required.
- Source-defined review and approval obligations are satisfied for the resulting lifecycle position.
- The Examination Session has reached only a lifecycle position permitted by L2-I02.
- Exact source-supported business events and hand-offs generated by the completed steps are preserved.
- No downstream Examination lifecycle or L2-I16 closure responsibility is absorbed.

## 16.2 Postconditions

- Examination Session Management remains the canonical owner of the governed Examination Session.
- Downstream Examination workflows may consume the session only according to the lifecycle conditions defined by L2-I02.
- Institution, Organisation, Curriculum, Academic Calendar, and Examination Policy facts remain owned by their canonical domains.
- Historical and audit evidence remains traceable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

**OPEN — Architecture approval/lock provenance requires independent governance verification.** The previous authored version carried the reference `ACR-I-2026-08-21-002` and a post-approval lock assertion, but no independent authoritative approval artifact was identified in the repository evidence used for this remediation. Because this file also required material metadata and event-model corrections, its status is returned to **Review** pending governed re-approval/re-locking. This does not assert that the earlier approval was invalid; it records that the approval evidence could not be independently verified.

**UPSTREAM TRACEABILITY GAP — L2-I02 Contracts.** L2-I02's own canonical crosswalk records that a formal Contracts section is not present. This L3 scenario therefore uses only source-supported business hand-offs and does not create canonical contract names locally.

## 17.2 Assumptions

No assumption overrides locked architecture. No additional actor, event, state, contract, rule, authority, or downstream ownership is inferred beyond the cited sources.

## 17.3 Deferred Questions

- Governance must verify or replace the prior `ACR-I-2026-08-21-002` approval/lock evidence before this corrected version can become Architecture Approved or Locked.
- Any formal canonical cross-domain contract model missing from L2-I02 requires upstream architecture resolution, not local L3 invention.
- APIs, UI, data models, technical integration, security implementation, deployment, and other implementation/technical design concerns remain deferred to L4–L5.
