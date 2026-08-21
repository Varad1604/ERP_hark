# Financial Rule Change to Governed Active Policy

# 1. Scenario Metadata

**Scenario ID:** L3-F-085  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Finance Foundation  
**Participating Domains:** A, F  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L2-F00 — Finance Domain Model & Foundation; applicable L2-F01 through L2-F14 domain workflows  
**Referenced By:** Finance services evaluating fee, payment, refund, scholarship, budget, procurement, payroll and accounting rules

---

# 2. Scenario Purpose

Govern the creation or revision of a Finance-owned rule through deterministic scoping, validation, testing, approval, publication and activation so future financial decisions use the effective version while historical transactions retain their original rule version, inputs, decision and explanation.

---

# 3. Business Outcome

Testing and approval produce an effective rule version while historical decisions retain their governing version

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a new or revised financial rule is proposed.

## 4.2 End Boundary

The scenario ends when:

- the proposed rule’s owner, category, scope and business justification have been established;
- a distinct version has progressed through `Draft`, `Validation`, `Testing`, `Approval` and `Published`;
- the approved version has become `Active` for future eligible financial execution;
- any predecessor has been preserved and moved to `Deprecated` where replacement applies;
- future rule resolution selects the applicable Active version deterministically;
- historical transactions retain the rule version and decision used originally; and
- rule governance and evaluation evidence remain immutable and auditable.

## 4.3 In Scope

- New and revised Finance-owned rules.
- Fee, payment, refund, scholarship, budget, procurement, payroll and accounting rule categories.
- Platform, Tenant, institution and lower-scope context resolution.
- Rule ownership and scope validation.
- Creation of a distinct version with effective and expiry dates where applicable.
- Validation and controlled testing.
- Conflict and precedence evaluation.
- Approval, publication and activation.
- Deprecation of the predecessor where applicable.
- Deterministic future rule evaluation.
- Historical rule-version retention and explainable decisions.
- Immutable governance and evaluation audit evidence.

## 4.4 Out of Scope

- Changing general Finance configuration under L3-F-084 unless the item is a Financial Rule.
- Direct mutation of financial data by a rule.
- Re-evaluating or rewriting historical transactions after a rule change.
- Hardcoding institution-specific rules in Finance services.
- Letting the Platform Rule Engine own Finance rule meaning or business decisions.
- Inventing rule-change commands, lifecycle events, publishers or named approval actors.
- Technical rule language, expression syntax, runtime, database, cache or deployment design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-F-084 — Finance Configuration Change to Governed Active Configuration | Governs general Finance configuration and Platform resolution; F085 owns the specific Financial Rule lifecycle. |
| Supporting | L3-A-014 — Workflow Definition to Activated Orchestration Availability | Supplies governed workflow capability for approval orchestration where configured. |
| Supporting | L3-A-052 — Authorization Policy Change to Governed Active Policy Version | Owns A04 authorization-policy lifecycle, not Finance business rules. |
| Downstream | Applicable Section F operational scenarios | Resolve and evaluate only Active rules before financial state transitions. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Finance Foundation / Section F | Primary owner. Owns Finance rule meaning, category, scope, version, lifecycle, business validation, decision outcomes and historical references. |
| Platform & Tenant / Section A | Supplies Tenant and hierarchical configuration context and provides Platform Rule Engine execution infrastructure without owning Finance rule semantics. |
| Authorization & Policy / Section A | Authorizes protected rule-governance activities and evaluates applicable role, permission, scope and SoD controls. |
| Workflow Engine / Section A | Executes the configured approval workflow without owning the rule or approval decision. |
| Event Infrastructure / Section A | May distribute canonical rule lifecycle events after the owning domain defines them; it does not invent or own those facts. |
| Audit Infrastructure / Section A | Preserves immutable rule governance and evaluation evidence. |

The frozen participating-domain value remains exactly `A, F`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized Financial Rule Proposer | Proposes a new rule or a justified revision within an authorized Finance scope. | May create the proposal; cannot make it Active by proposal alone. |
| Finance Rule Owner | Owns rule meaning, category, scope, expected outcomes and business justification. | Accountable for semantic validity and future use. |
| Rule Validator and Tester | Validates the rule and records controlled test evidence for expected, boundary, conflict and failure cases. | Does not gain approval authority merely through testing. |
| Authorized Finance Approver | Grants or refuses the policy-required approval. | Authority derives from Authorization & Policy and the configured approval workflow. |
| Finance Service | Resolves and evaluates the applicable Active rule before a financial transition. | Consumes decisions and never bypasses the Rule Engine pipeline. |

L2-F00 lists example Finance roles and rule-governance data, including `Owner`, `Created By` and `Approved By`, but does not map this scenario to named canonical actors. These labels are functional responsibilities only.

---

# 7. Preconditions and Entry Criteria

- The Tenant and institutional Finance context are valid.
- The proposal has an authorized initiator and business justification.
- The proposed rule belongs to Finance and one supported rule category.
- The intended Platform, institution, campus, department, programme, financial category or transaction scope can be resolved.
- The predecessor rule and version are available where a revision is proposed.
- Required inputs, expected decisions, explanations and conflict cases are defined sufficiently for validation and testing.
- Mandatory compliance and Platform safety rules are identifiable.
- A governed approval workflow can be selected where required.
- Historical transactions retain their original rule-version references.
- Audit and correlation context are available.

---

# 8. Trigger

A new or revised financial rule is proposed

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the new-rule or rule-revision proposal with its business justification. | Authorized Financial Rule Proposer; Finance Foundation | A governed rule-change request begins. |
| 2 | Resolve the Tenant, institution and applicable lower-scope context. | Platform & Tenant; Finance Foundation | The proposed rule boundary is established. |
| 3 | Confirm exclusive Finance ownership and classify the rule as fee, payment, refund, scholarship, budget, procurement, payroll or accounting. | Finance Rule Owner | The canonical rule owner and category are established. |
| 4 | Resolve the predecessor and current Active version where the proposal is a revision. | Finance Foundation | Version lineage and current behaviour are known. |
| 5 | Create a distinct rule version in `Draft` with rule ID, name, category, owner, scope, effective period and justification. | Finance Foundation | A governed candidate version exists without changing the current Active rule. |
| 6 | Validate rule completeness, supported inputs and outputs, ownership, scope, version consistency and effective dates. | Rule Validator; Finance Foundation | The candidate may progress from `Validation` only if structurally and semantically valid. |
| 7 | Validate that the rule is deterministic, explainable, independent of workflow implementation and incapable of directly modifying financial data. | Rule Validator; Finance Foundation | Mandatory Rule Engine constraints are satisfied. |
| 8 | Test expected, boundary, failure and conflict cases using controlled input contexts. | Rule Validator and Tester; Finance Foundation | Test evidence demonstrates the proposed decisions and explanations. |
| 9 | Apply deterministic conflict precedence: highest scope, more specific rule, latest Active version, explicit deny, mandatory compliance and Platform safety. | Finance Foundation — Rule Engine | Conflicts are resolved under the canonical precedence order. |
| 10 | Complete `Testing → Approval` only after required validation and test evidence is available. | Finance Foundation; Workflow Engine | The candidate is ready for an authorized approval decision. |
| 11 | Authorize and record approval or refusal through the governed workflow. | Authorized Finance Approver; Authorization & Policy; Workflow Engine | The rule is approved for publication or retained outside operational use. |
| 12 | Record version, owner, effective dates, approval reference, approver, status, scope and justification. | Finance Foundation; Audit Infrastructure | Governance metadata is complete and auditable. |
| 13 | Publish the approved rule version and then activate it at its approved effective time. | Finance Foundation | The candidate progresses `Approval → Published → Active`. |
| 14 | Move the predecessor from `Active` to `Deprecated` where replacement applies, without altering its historical references. | Finance Foundation | One applicable latest Active version governs future execution and the predecessor remains preserved. |
| 15 | Make the Active rule available to the canonical evaluation pipeline for future Finance operations. | Finance Foundation; Platform Rule Engine | Finance services can resolve and evaluate the effective version before state transitions. |
| 16 | Preserve immutable rule-governance evidence and ensure future evaluations record version, inputs, timestamp, decision, explanation and outcome. | Finance Foundation; Audit Infrastructure | The rule change and subsequent decisions remain reproducible. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| First rule version | No predecessor exists. | Govern the initial Draft through validation, testing, approval, publication and activation. | A first Active rule becomes available. | Finance Foundation |
| Future-effective rule | Approval occurs before the effective date. | Preserve the approved/published version without using it in execution until activation. | The current Active version remains effective until the approved time. | Finance Foundation |
| Scoped rule | A permitted lower-scope rule is proposed. | Validate scope and precedence against inherited rules. | The Active rule applies only within its approved scope. | Finance Foundation; Platform & Tenant |
| Multiple applicable rules | More than one rule matches an execution context. | Apply the canonical precedence order and preserve the explanation. | A deterministic decision is returned. | Finance Foundation — Rule Engine |
| Approval required outcome | Testing shows a business operation should return `ALLOW_WITH_APPROVAL`. | Preserve that rule output and the explanation; the later operational approval remains separate from approving this rule version. | The Active rule can require later transaction approval without conflating approvals. | Finance Foundation; Workflow Engine |
| Validation or testing failure | The candidate is incomplete, nondeterministic, unsafe or produces unacceptable results. | Keep it outside `Published` and `Active`; correct and retest the candidate. | The current Active version remains unchanged. | Finance Foundation |
| Approval refused | The candidate is not approved. | Preserve the decision and candidate history without publication or activation. | No change to future rule evaluation. | Authorization & Policy; Workflow Engine; Finance Foundation |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Rule owner or category cannot be resolved. | The proposal cannot enter a governed Finance lifecycle. | Reject or correct the proposal before Draft creation. | Finance Foundation |
| Scope is invalid or prohibited. | The rule could override authority outside its boundary. | Stop progression and correct the scope. | Finance Foundation; Platform & Tenant |
| Validation fails. | The candidate cannot safely be tested or approved. | Retain it outside operational states and preserve findings. | Finance Foundation |
| Testing fails or is incomplete. | Expected outcomes are not demonstrated. | Do not progress to Approval; revise and retest. | Finance Foundation |
| Rule conflict is unresolved. | Deterministic future evaluation is not assured. | Apply the canonical precedence rules or reject the candidate where ambiguity remains. | Finance Foundation |
| Approval is denied or interrupted. | The rule cannot be Published or Active. | Preserve the non-approved outcome; continue using the current Active version. | Authorization & Policy; Workflow Engine |
| Activation is interrupted. | The candidate must not participate in business execution. | Continue using the prior Active version until activation completes canonically. | Finance Foundation |
| Rule evaluation fails during later execution. | Unauthorized or indeterminate financial processing could occur. | Prevent the affected financial state transition and preserve the failure evidence. | Finance Foundation — Rule Engine |
| A lifecycle success event is expected. | L2-F00 defines no canonical rule-version events or publishers. | Do not invent an event; use preserved audit evidence until upstream contracts exist. | Finance Foundation; Event Infrastructure |

No technical retry, expression-engine recovery, cache invalidation or deployment mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Institution-specific financial policies are externalized as configurable rules. | Finance Foundation | Finance services cannot hardcode the proposed policy. | L2-F00 §49.2 — Rule Externalization |
| Finance owns finance-rule semantics; the Platform Rule Engine provides execution infrastructure. | Finance Foundation; Platform | Ownership is not transferred to shared infrastructure. | L2-F00 §58(9) |
| Rule evaluation is deterministic. | Finance Foundation | Identical inputs and effective versions must produce the same outcome. | L2-F00 §49.2, §58 |
| Every Financial Rule is versioned. | Finance Foundation | A revision creates a distinct version. | L2-F00 §49.2 — Versioned Rules |
| Historical transactions retain the rule version used. | Finance Foundation | Rule changes never rewrite prior financial decisions. | L2-F00 §49.2, §57–§58 |
| Every evaluation produces an explanation. | Finance Foundation | Decision rationale becomes part of the audit trail. | L2-F00 §49.2 — Explainable Decisions |
| Rule scope may range from Platform to a specific transaction. | Finance Foundation; Platform & Tenant | Scope and permitted override behaviour must be resolved before activation. | L2-F00 §49.2 — Scoped Configuration |
| The rule lifecycle is `Draft → Validation → Testing → Approval → Published → Active → Deprecated → Archived`. | Finance Foundation | Only source-defined states and order may be used. | L2-F00 §55 |
| Only Active rules participate in business execution. | Finance Foundation | Draft, Testing, Published, Deprecated and Archived versions cannot govern new decisions. | L2-F00 §55 |
| Conflict resolution follows the canonical precedence order. | Finance Foundation | Scope, specificity, version, deny, compliance and safety precedence are deterministic. | L2-F00 §54 |
| Rules never directly modify financial data. | Finance Foundation | The output informs Finance services; the rule cannot perform the transaction. | L2-F00 §52, §58 |
| Rule evaluation occurs before financial state transitions and cannot be bypassed. | Finance Foundation | A failed or denied evaluation prevents unauthorized execution. | L2-F00 §51, §58 |
| Every rule records governance metadata and every evaluation records immutable evidence. | Finance Foundation; Audit Infrastructure | Approval and historical reproducibility are mandatory. | L2-F00 §56–§57 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Proposed Financial Rule version | Does not exist or `Draft` | Validated, tested, approved, published and activated. | `Draft → Validation → Testing → Approval → Published → Active`. | Finance Foundation |
| Predecessor Financial Rule | `Active` where a revision applies | Continues governing execution until successor activation, then leaves active use. | `Active → Deprecated`; later archival is separate. | Finance Foundation |
| Historical financial transaction | References its original rule version and decision | No mutation or re-evaluation. | Original reference, decision and explanation remain unchanged. | Applicable Finance aggregate |
| Future Finance operation | Not yet evaluated | Resolves the latest applicable Active rule and receives a decision and explanation. | Proceeds or stops according to the rule output and later workflow. | Applicable Finance service |
| Rule evaluation audit | No evaluation evidence yet for a future operation | Records version, input context, time, decision, explanation, duration and outcome. | Immutable evidence available. | Finance Foundation; Audit Infrastructure |

L2-F00 defines the rule lifecycle and the rule-evaluation pipeline but not a distinct state machine for a rule-change request or approval workflow instance. This Draft does not add those states.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Financial Rule Event Determination

L2-F00 does not define canonical Financial Rule lifecycle event names, an Aggregate-to-Event Matrix or sole publishers.

| Required Business Fact | Source Resolution Result | Scenario Treatment |
|---|---|---|
| Rule version created | No canonical event or publisher defined. | No event is invented. |
| Rule validation or testing completed | No canonical event or publisher defined. | No event is invented. |
| Rule approved | No canonical event or publisher defined. | No event is invented. |
| Rule published or activated | No canonical event or publisher defined. | No event is invented. |
| Prior rule deprecated | No canonical event or publisher defined. | No event is invented. |
| Rule archived | No canonical event or publisher defined. | No event is invented. |

The absence of lifecycle events does not erase the upstream-defined states, governance metadata or immutable audit requirements.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant and configuration scope | Platform & Tenant | Finance Foundation | Tenant, institution and hierarchical scope context. |
| Authorization Decision | Authorization & Policy | Finance Foundation; Workflow Engine | Permission and scope outcome for rule governance. |
| Approval workflow outcome | Workflow Engine | Finance Foundation | Approval or refusal plus workflow and actor reference. |
| Active Financial Rule | Finance Foundation | Platform Rule Engine; Finance services | Versioned rule, category, scope, effective period and evaluation definition. |
| Rule decision and explanation | Finance Rule Engine | Finance service | `ALLOW`, `DENY`, `ALLOW_WITH_APPROVAL`, `ALLOW_WITH_WARNING`, `CALCULATE_VALUE`, `GENERATE_ADJUSTMENT`, `ESCALATE` or `REQUIRE_DOCUMENT`, with explanation. |
| Rule governance and evaluation audit input | Finance Foundation | Audit Infrastructure | Versions, actors, approval, inputs, decisions, explanations, time and correlation evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Rule proposal, validation, testing, approval, publication and activation are protected activities.
- Authorization must respect Tenant, institution, rule category and scope.
- Finance ownership of rule semantics does not grant unrestricted change authority.
- Platform execution infrastructure cannot change Finance rule meaning.
- Future Finance services remain obligated to evaluate the applicable rule before protected state transitions.

## 15.2 Approval and Separation of Duty

- A rule cannot reach `Published` or `Active` without completing `Approval`.
- Approval must use an authorized workflow and preserve its approval reference and approver.
- Critical financial activities remain subject to SoD; proposal, validation, testing and approval responsibilities must follow the applicable policy.
- L2-F00 does not name the exact approver or define a rule-category approval matrix.
- The approval of a rule version is distinct from any later transaction approval produced by `ALLOW_WITH_APPROVAL`.

## 15.3 Audit Obligations

The following must be auditable:

- Tenant, institution and lower scope.
- Rule ID, name, category, version and owner.
- Predecessor version and change justification.
- Effective and expiry dates.
- Validation findings and test cases/outcomes.
- Conflict and precedence assessment.
- Approval reference, approver and workflow reference.
- Publication, activation and predecessor deprecation.
- Historical transaction references to governing versions.
- Each later evaluation’s transaction reference, version, inputs, timestamp, decision, explanation, duration and outcome.
- Actor, timestamp, correlation and final change result.

---

# 16. Completion Criteria and Postconditions

The scenario is complete when:

- the authorized proposal and business justification were received;
- Finance ownership, rule category and scope were established;
- a distinct Draft version was created;
- mandatory structural, semantic and safety validations succeeded;
- deterministic testing, boundary testing and conflict assessment succeeded;
- the required approval was granted and recorded;
- the rule progressed through `Published` to `Active` at its effective time;
- any predecessor was preserved and deprecated where applicable;
- future Finance services could resolve and evaluate the correct Active version;
- historical transactions retained their original rule-version references and decisions; and
- immutable governance and evaluation audit evidence was preserved.

Postconditions:

- Only the applicable Active rule participates in future execution.
- The predecessor remains available for historical reconstruction.
- No historical financial decision is recalculated merely because the rule changed.
- Finance retains rule ownership while Platform provides execution infrastructure.
- No unsupported lifecycle event or command has been introduced.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Type | Issue / Deferred Question | Precise Scenario Impact | Disposition |
|---|---|---|---|---|
| F085-ISSUE-01 | Upstream command gap | L2-F00 defines the full rule lifecycle but no commands for proposing, validating, testing, approving, publishing, activating, deprecating or archiving a version. | L3 business behaviour is source-supported, but L4 cannot derive canonical operation contracts. | Define authoritative rule-lifecycle commands before L4 specification. |
| F085-ISSUE-02 | Upstream event gap | L2-F00 defines no canonical Financial Rule lifecycle events or sole publishers. | Downstream consumers cannot subscribe to an authoritative activation or deprecation fact without a new L2 decision. | Define exact event names, publication conditions and publishers if asynchronous notification is required. |
| F085-ISSUE-03 | Actor and approval gap | L2-F00 records `Owner`, `Created By`, `Approved By` and Approval Reference but does not name canonical rule proposers, testers or approvers or a category-specific approval matrix. | Responsibilities can be separated functionally, but exact approval authority remains policy-derived. | Define actor mappings and approval policy before Architecture Approval if named roles are required. |
| F085-ISSUE-04 | Transition-guard granularity | L2-F00 provides the lifecycle order but not a transition matrix specifying every guard, refusal path, future-effective behaviour or recovery transition. | The main outcome is exact, but L4 needs additional guard contracts. | Add a rule transition matrix without changing the frozen lifecycle. |

**Source Assessment:** PASS WITH ISSUES

No unsupported actor, approval authority, lifecycle state, command, event, publisher, rule syntax or technical execution mechanism is assumed by this Draft.
