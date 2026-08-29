# Institutional Progression Policy Lifecycle to Effective Progression Governance

**Scenario ID:** L3-C-075  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Progression  
**Participating Domains:** B, C  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-C04 — Student Academic Progression Workflows; materially applicable L2-B01 Institution Workflows, L2-B02 Organisation Workflows, L2-B04 Curriculum Framework Workflows, L2-B05 Academic Calendar Workflows, L2-C02 Student Lifecycle & Activation Workflows, L2-C03 Student Academic Registration Workflows, and L2-A04 Authorization & Policy Workflows  
**Referenced By:** C027 — Student Academic Progression Decision; C029 — Conditional Promotion Condition Fulfilment; C030 — Academic Probation Management and Resolution; C031 — Academic Detention and Repeat Determination; C032 — Student Credit Transfer and Recognition; C033 — Student Programme Migration; C034 — Student Academic Break; C035 — Student Rejoining After Academic Break; C036 — Qualification Eligibility Determination; C038 — Progression Decision Revision After Re-evaluation or Appeal

## 1. Scenario Metadata

This scenario preserves the frozen Section C identity: L3-C-075, title, Primary Owner, participating domains, Critical type and P0 priority.

The authoritative L2-C04 architecture identifies Student Academic Progression as the canonical owner of progression policies and progression decisions. It also states that progression is policy-driven, configurable, deterministic, auditable and reproducible.

This scenario governs the business lifecycle by which an institutional progression policy becomes an effective policy position usable by Academic Progression. It does not define implementation mechanics or create a separate Policy Management owner.

## 2. Scenario Purpose

Establish an authoritative, institutionally governed progression-policy position before that policy is used to make academic progression decisions.

The purpose is to ensure that progression decisions are evaluated against an approved and effective policy position while preserving policy history, applicability and auditability.

## 3. Business Outcome

An institutional progression policy is authoritatively established in its applicable effective position, with its governance history preserved and its effective policy position available for Academic Progression decisions.

The outcome is a governed policy position—not an individual student's progression decision.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An institutional progression-policy definition or policy change requires governance and determination of its effective applicability.

### 4.2 End Boundary

The progression policy reaches its authoritative effective business position and can be applied by Academic Progression to subsequent progression evaluations.

### 4.3 In Scope

- Governance of progression-policy definition or change.
- Validation of the policy against applicable institutional progression governance.
- Establishment of the effective policy position.
- Preservation of policy history and version continuity.
- Establishment of applicability to the relevant academic/institutional context.
- Making the effective policy position available to Academic Progression decisions.

### 4.4 Out of Scope

- Individual Student Academic Progression decisions.
- Conditional promotion decisions.
- Academic probation or detention decisions.
- Examination evaluation or result processing.
- Academic registration.
- Curriculum ownership.
- StudentContext mutation.
- Technical policy engines, APIs, databases or access-control implementation.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | C027 — Student Academic Progression Decision | The effective policy constrains individual progression evaluation. |
| Related | C029 — Conditional Promotion Condition Fulfilment | Conditional progression applies the effective progression policy. |
| Related | C030 — Academic Probation Management and Resolution | Probation criteria are policy-governed. |
| Related | C031 — Academic Detention and Repeat Determination | Detention/repeat decisions are policy-governed. |
| Related | C038 — Progression Decision Revision After Re-evaluation or Appeal | Revised decisions must apply the applicable policy position. |

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Academic Progression — C | Canonical owner of institutional progression policy within the Academic Progression bounded context; governs effective applicability for progression decisions. |
| Institution / Organisation — B | Provides authoritative institutional and organisational context within which the progression policy applies. |

Participation by B does not transfer progression-policy ownership from Academic Progression.

The locked L2-C04 architecture explicitly states that Student Academic Progression owns progression policies and that progression decisions are policy-driven.

## 6. Business Actors, Roles and Responsibilities

The accessible locked L2-C04 source establishes that progression policy requires authorised institutional governance and that manual overrides require authorised approval and complete audit history. It does not expose a sufficiently specific canonical actor/role name for this scenario.

Therefore no invented role such as “Policy Approver” or “Academic Committee” is introduced.

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authoritative institutional policy authority — exact canonical role not defined in accessible L2-C04 material | Provides the governed institutional decision required for the progression-policy position. | Authority must remain institutionally defined upstream. |

**Open role-definition gap:** the canonical actor/role responsible for approving or authorising progression-policy changes is not sufficiently defined in the accessible L2-C04 material and must not be invented at L3.

## 7. Preconditions and Entry Criteria

- A progression policy or proposed progression-policy change exists within the institutional academic-governance context.
- The applicable institution/organisation context is authoritative.
- The policy is within the scope of Academic Progression governance.
- Required institutional authority and evidence are available according to the applicable upstream governance.
- The policy's applicability/effective period can be determined from authoritative academic context.

## 8. Trigger

An institutional progression policy is created, revised, superseded, or otherwise requires a governed determination of its effective applicability.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the progression policy and the institutional/academic context to which it applies. | Academic Progression / Institution context | Policy scope is established. |
| 2 | Validate the policy against applicable institutional progression governance and academic context. | Academic Progression with authoritative institutional participation | Policy is either eligible for governance completion or remains unresolved. |
| 3 | Obtain the institutionally required policy decision/authorisation. | Authoritative institutional policy authority | Policy governance decision is established. |
| 4 | Establish the policy's effective applicability for Academic Progression. | Academic Progression | One authoritative effective policy position is established for the applicable context. |
| 5 | Preserve the prior policy history and the newly established policy position. | Academic Progression | Policy history remains traceable and reproducible. |
| 6 | Make the effective progression-policy position available to subsequent Academic Progression evaluations. | Academic Progression | Subsequent progression decisions can apply the effective policy. |

The Main Business Flow ends when the effective progression policy is established and available for Academic Progression use. Individual student progression is outside this scenario.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | Policy change is not authorised | Retain the existing effective policy position and preserve the proposed change as unresolved/not effective. | Existing effective policy remains authoritative. | Academic Progression |
| A2 | Policy has a future effective applicability | Record the governed future applicability without treating the policy as currently effective. | Current policy remains effective until the governed boundary. | Academic Progression |
| A3 | Policy applicability differs by authoritative institutional context | Establish the policy position only for the applicable governed context. | Policy applicability remains context-specific rather than globally assumed. | Academic Progression / Institution context |

No additional alternate path is introduced where the upstream architecture does not define one.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required policy authority is not established | The proposed policy cannot become the effective progression policy. | Retain the existing effective policy position until authorised governance is completed. | Academic Progression |
| Required institutional context is ambiguous | Policy applicability cannot be determined safely. | Resolve the authoritative institutional context before making the policy effective. | Institution / Organisation with Academic Progression |
| Policy change conflicts with authoritative academic governance | The proposed policy cannot supersede the applicable effective policy. | Resolve the governance conflict upstream; do not invent a local reconciliation. | Academic Progression / applicable institutional authority |
| Existing policy needs correction after becoming effective | Existing policy history must remain traceable. | Apply the governed policy-revision/correction lifecycle and establish the corrected effective position without erasing prior history. | Academic Progression |

No technical retry, queue, rollback, server failure or infrastructure recovery is defined.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Progression shall be determined through approved institutional policies rather than hard-coded business logic. | Academic Progression | Only an approved policy position may govern progression decisions. | L2-C04 AP-01 |
| Progression decisions must be deterministic, auditable, policy-driven and reproducible. | Academic Progression | The effective policy position and its history must remain traceable. | L2-C04 §§1.2, 2.8 |
| Every progression decision shall reference an approved institutional policy. | Academic Progression | The policy must be established before it can govern progression. | L2-C04 §2.8 |
| Progression decisions must be reproducible using the same policy version and academic data. | Academic Progression | Policy version/effective position is materially relevant to downstream decisions. | L2-C04 §2.8 |
| Historical progression decisions shall never be overwritten. | Academic Progression | Policy changes must not silently rewrite completed historical decisions. | L2-C04 §2.8 |
| Manual overrides require authorised approval and complete audit history. | Academic Progression | Any governed policy exception/override must retain authority and audit evidence. | L2-C04 §2.8 |

L3 applies these upstream rules; it does not create new progression-policy rules.

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Progression Policy / policy position | Existing or proposed policy position | Governance and applicability are determined | Effective progression-policy position | Academic Progression | L2-C04 §§1.8, 1.9, 2.8 |

The exact canonical state names and transition vocabulary for the progression-policy lifecycle are not sufficiently exposed in the accessible L2-C04 material used for this authoring run. No new state names are invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

The accessible L2-C04 material establishes event-driven collaboration and authoritative progression outcomes, but it does not expose a verified canonical event name specifically for “progression policy became effective.”

Therefore no new policy-activation event is invented.

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Canonical progression-policy-effective event | Not sufficiently defined in accessible L2-C04 material | Academic Progression decision workflows | Communicates the effective policy position where the upstream architecture defines such an event | L2-C04 event architecture — exact event catalogue requires verification |

The effective policy position itself remains owned by Academic Progression.

### 14.2 Synchronous Business Contracts / Hand-offs

No scenario-specific synchronous business contract is introduced here beyond the authoritative institutional-policy governance hand-off required by upstream architecture.

Events and synchronous contracts remain distinct.

## 15. Business Authorization, Approval and Audit Obligations

- The progression policy must be approved through the applicable institutional governance before it becomes effective.
- The exact canonical approving actor/role is not sufficiently defined in the accessible L2-C04 material and must not be invented.
- Policy applicability and effective position must remain auditable.
- Historical policy/progression context must remain reproducible.
- Manual overrides require authorised approval and complete audit history.
- This scenario does not define technical access control, IAM, permissions or enforcement mechanisms.

## 16. Completion Criteria and Postconditions

The scenario is complete when:

- The progression policy has passed the applicable institutional governance.
- Its effective applicability is established.
- The effective policy position is authoritative for Academic Progression.
- Policy history is preserved.
- The effective policy can be used by subsequent progression decision scenarios.
- No Student-specific progression decision is made as part of this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### OI-C075-01 — Progression Policy Authority Role

The accessible locked L2-C04 material requires approved institutional policy and authorised approval but does not expose a sufficiently specific canonical business actor/role for progression-policy approval.

**Impact:** Section 6 cannot safely name a specific approving role.

**Required action:** Resolve the canonical business authority upstream before this scenario is locked.

### OI-C075-02 — Policy Lifecycle State Vocabulary

The accessible L2-C04 material establishes progression-policy ownership and policy-driven governance but does not expose a verified canonical state/transition vocabulary specifically for the progression-policy lifecycle.

**Impact:** Section 13 cannot safely name exact lifecycle states.

**Required action:** Verify the detailed L2-C04 policy lifecycle/state definition before locking.

### OI-C075-03 — Policy Activation Event

The accessible L2-C04 material establishes event-driven collaboration but does not expose a verified canonical event name for “progression policy became effective.”

**Impact:** Section 14 cannot safely invent an event publisher/name.

**Required action:** Verify the L2-C04 event catalogue before locking.

### OI-C075-04 — Unresolved Policy Management Dependency

L2-C04 lists “Policy Management — unresolved, no owner” as an input source for progression regulations. At the same time, L2-C04 explicitly states that Student Academic Progression owns progression policies.

**Impact:** The external Policy Management ownership reference must not be treated as a second owner of the progression-policy lifecycle.

**Required action:** Preserve Academic Progression as the canonical owner and resolve the repository-level Policy Management dependency/ownership ambiguity through architecture change control.

### Classification

**PASS WITH ISSUES — not lock-ready.**

The core scenario outcome, ownership, policy-driven progression boundary and downstream purpose are source-grounded. The unresolved role, exact policy lifecycle states, exact policy-activation event, and external Policy Management dependency prevent a stronger classification without inventing upstream architecture.
