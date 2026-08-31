# L4-C-075 — Institutional Progression Policy Lifecycle to Effective Progression Governance

**Scenario ID:** L4-C-075  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-C-075 — Institutional Progression Policy Lifecycle to Effective Progression Governance  
**Primary Owner:** Academic Progression  
**Participating Domains:** B, C  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata

This specification derives from locked L3-C-075 and preserves Academic Progression ownership of progression policy and the institutional context participation boundary.

## 2. System Behavior Purpose

Specify observable behavior for establishing an authoritative effective progression-policy position before it is used by Academic Progression to evaluate individual progression decisions.

## 3. System Behavior Outcome

An approved progression policy reaches its applicable effective business position, its history is preserved, and the effective policy position is available for subsequent Academic Progression evaluations.

## 4. System Behavior Scope and Boundary

### 4.1 Start Boundary
A progression policy or policy change requires institutional governance and determination of effective applicability.

### 4.2 End Boundary
The progression policy reaches its authoritative effective business position and is available for subsequent Academic Progression use.

### 4.3 In Scope
- Govern progression-policy definition or change.
- Validate the policy against applicable institutional progression governance and academic context.
- Establish effective applicability.
- Preserve policy history and version continuity.
- Make the effective policy position available to subsequent Academic Progression evaluations.

### 4.4 Out of Scope
- Individual Student progression decisions.
- Conditional promotion, probation, detention, repeat, credit transfer, or programme migration decisions.
- Examination processing.
- Academic registration.
- Curriculum ownership.
- StudentContext mutation.
- Technical policy engines, APIs, databases, or access-control implementation.

## 5. Ownership and Participating Domains

Academic Progression is the canonical owner of institutional progression policy within its bounded context. Institution and Organisation provide authoritative institutional and organisational context. Participation by B does not transfer progression-policy ownership from Academic Progression.

## 6. Business Actors, Roles and Responsibilities

The authoritative L2-C04 source requires authorised institutional governance but does not expose a sufficiently specific canonical approving role. No role such as Policy Approver or Academic Committee is invented.

## 7. Preconditions and Entry Criteria

- A progression policy or proposed progression-policy change exists.
- The applicable institution/organisation context is authoritative.
- The policy is within Academic Progression governance.
- Required institutional authority and evidence are available according to upstream governance.
- Applicability/effective period can be determined from authoritative academic context.

## 8. Trigger

A progression policy is created, revised, superseded, or otherwise requires a governed determination of effective applicability.

## 9. Main System Behavior

1. The system identifies the progression policy and applicable institutional/academic context.
2. The system validates the policy against applicable institutional progression governance and academic context.
3. The system records the institutionally required policy decision/authorisation when established by the authoritative governance.
4. The system establishes the policy's effective applicability for Academic Progression.
5. The system preserves the prior policy history and the newly established policy position.
6. The system makes the effective policy position available to subsequent Academic Progression evaluations.

## 10. Alternate System Behaviors

### 10.1 Policy Not Authorised
The system retains the existing effective policy position and does not make the proposed change effective.

### 10.2 Future Effective Applicability
The system records the governed future applicability without treating the policy as currently effective.

### 10.3 Context-Specific Applicability
The system establishes the policy position only for the applicable governed institutional context rather than assuming global applicability.

## 11. Failure, Interruption and Business Recovery

If required policy authority is not established, the proposed policy cannot become effective and the existing effective policy position remains authoritative. If institutional context is ambiguous, the policy is not made effective until the authoritative context is resolved. Conflicts with institutional academic governance remain subject to upstream resolution.

No technical retry, queue, rollback, server, or infrastructure recovery behavior is specified.

## 12. Business Rules and Decision Constraints

- Progression is determined through approved institutional policies rather than hard-coded business logic.
- Progression decisions must be deterministic, auditable, policy-driven, and reproducible.
- Every progression decision shall reference an approved institutional policy.
- Historical progression decisions shall not be overwritten.
- Manual overrides require authorised approval and complete audit history.
- Academic Progression remains the canonical owner of progression policy.

## 13. State and Lifecycle Impact

The progression-policy lifecycle reaches its canonical effective policy position as defined by upstream architecture. Exact state names and transition vocabulary are not introduced where they are not exposed in the authoritative L2-C04 source.

## 14. Business Events and Cross-Domain Collaboration

The accessible L2-C04 source establishes event-driven collaboration but does not expose a verified canonical event name specifically representing progression-policy effectiveness. No new policy-activation event is fabricated.

The effective policy position remains owned by Academic Progression.

## 15. Business Authorization, Approval and Audit Obligations

- Progression policy must pass applicable institutional governance before becoming effective.
- The exact canonical approving role is not sufficiently defined in the accessible L2-C04 source and is not invented.
- Policy applicability and effective position must remain auditable.
- Historical policy/progression context must remain reproducible.
- Manual overrides require authorised approval and complete audit history.
- No technical IAM, permissions, or enforcement mechanism is specified.

## 16. Completion Criteria and Postconditions

- Applicable institutional governance has been completed.
- Effective applicability has been established.
- The effective progression-policy position is authoritative for Academic Progression.
- Policy history is preserved.
- The effective policy can be used by subsequent progression decision scenarios.
- No Student-specific progression decision is made by this behavior.

## 17. Open Issues, Assumptions and Deferred Questions

### OI-C075-01 — Progression Policy Authority Role
The canonical business actor/role responsible for progression-policy approval is not sufficiently defined in the accessible L2-C04 material.

### OI-C075-02 — Policy Lifecycle State Vocabulary
The verified canonical state/transition vocabulary for the progression-policy lifecycle is not sufficiently exposed in the accessible L2-C04 material.

### OI-C075-03 — Policy Activation Event
A verified canonical event name for progression-policy effectiveness is not exposed in the accessible L2-C04 material.

### OI-C075-04 — Policy Management Dependency
L2-C04 references Policy Management as unresolved with no owner while also assigning progression-policy ownership to Academic Progression. No second owner is introduced here; the repository-level dependency ambiguity remains for governance resolution.

**Classification: PASS WITH ISSUES — not lock-ready until the upstream gaps are resolved.**

## Production Verification Record

- [x] L3 identity, ownership, domains, type, and priority preserved.
- [x] Academic Progression policy boundary preserved.
- [x] Institutional participation boundary preserved.
- [x] No implementation architecture introduced.
- [x] No unverified state, event, role, or ownership invented.
- [x] Status retained as Draft.
