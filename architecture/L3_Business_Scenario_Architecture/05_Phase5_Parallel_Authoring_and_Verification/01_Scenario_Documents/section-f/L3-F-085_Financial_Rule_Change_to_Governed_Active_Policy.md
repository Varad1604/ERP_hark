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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-F00 — Finance Domain Model & Foundation  
**Referenced By:** Finance workflows that evaluate Finance-owned financial rules

# 2. Scenario Purpose

Govern a new or revised Finance-owned financial rule through versioning, validation, testing, approval, publication and activation while preserving deterministic evaluation, explainable decisions, canonical Finance ownership and historical rule-version references.

# 3. Business Outcome

Testing and approval produce an effective rule version while historical decisions retain their governing version

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A new or revised financial rule is proposed.

## 4.2 End Boundary

The intended end boundary is reached when a distinct Financial Rule version has completed testing and approval, become effective for future eligible Finance processing, and historical decisions retain the rule version under which they were made. Locked F00 defines this lifecycle and historical preservation, but does not define the canonical approval actor/authority or rule-lifecycle business event required by its general Finance event rule; therefore exact architecture completion remains blocked.

## 4.3 In Scope

- Finance Financial Rule ownership, category and scope.
- Creation of a distinct rule version.
- Validation, testing, approval, publication and activation under the canonical Rule lifecycle.
- Deterministic rule conflict resolution and evaluation.
- Preservation of predecessor versions and historical decision references.
- Explainable decisions and immutable evaluation evidence.
- Identification of unresolved event and authority gaps.

## 4.4 Out of Scope

- General Finance configuration that is not a Financial Rule.
- Direct modification of financial records by a rule.
- Re-evaluation or rewriting of historical transactions because a rule changes.
- Platform ownership of Finance rule meaning.
- Inventing rule lifecycle events, publishers or business approval roles.
- Technical rule languages, expression engines, storage, deployment or runtime mechanics.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-F-084 — Finance Configuration Change to Governed Active Configuration | Governs general Finance configuration rather than the Financial Rule lifecycle. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Finance Foundation / F | Owns Finance rule meaning, categories, scope, lifecycle, versioning, evaluation behavior, decisions and historical references. |
| Platform Foundation / A | Supplies foundational configuration context and shared platform capabilities without owning Finance rule semantics or decisions. |

Participation by Section A does not transfer ownership of the Financial Rule or its business outcomes.

# 6. Business Actors, Roles and Responsibilities

L2-F00 records rule governance attributes such as Owner, Created By and Approved By, but it does not establish canonical business actors or institutional roles specifically responsible for proposing, testing, approving, activating or deprecating a Finance Financial Rule. L2-A01 also records its Actors section as absent. No placeholder proposer, tester, approver or rule-owner role is invented.

# 7. Preconditions and Entry Criteria

- The Tenant and applicable Finance context are valid.
- The proposed rule belongs to Finance and has one canonical Finance owner.
- The rule category and intended scope can be resolved.
- A predecessor version is identifiable where a revision is proposed.
- Rule inputs, expected outputs and conflict context are sufficient for validation and testing.
- Historical financial transactions preserve the rule version originally used.

# 8. Trigger

A new or revised financial rule is proposed

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the proposed Financial Rule, Finance ownership, category, scope and business justification. | Finance Foundation / F | The proposal is inside the canonical Finance Rule boundary. |
| 2 | Create a distinct rule version without altering the current Active predecessor. | Finance Foundation / F | A governed candidate exists in `Draft`. |
| 3 | Validate the rule’s completeness, scope, version consistency, deterministic behavior and permitted inputs/outputs. | Finance Foundation / F | The candidate is eligible to progress only after successful validation. |
| 4 | Test the candidate against expected, boundary, failure and conflict conditions. | Finance Foundation / F | The candidate progresses through the source-defined `Testing` lifecycle position only when its behavior is demonstrated. |
| 5 | Apply the canonical conflict precedence where multiple rules may apply. | Finance Foundation / F | Rule application remains deterministic. |
| 6 | Progress the successfully tested candidate to the source-defined approval stage. | Finance Foundation / F | The rule awaits a valid approval outcome. |
| 7 | Publish and activate the approved rule version at its effective point. | Finance Foundation / F | The candidate progresses `Approval → Published → Active` and becomes eligible for future Finance execution. |
| 8 | Move the replaced predecessor from `Active` to `Deprecated` where applicable while preserving its historical references. | Finance Foundation / F | Prior decisions remain bound to their original governing version. |
| 9 | Preserve rule governance information and ensure future evaluations record the rule version, input context, decision, explanation and outcome. | Finance Foundation / F | The rule change and subsequent decisions remain reproducible and auditable. |

The exact human/institutional approval authority and required canonical lifecycle event remain undefined upstream.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| First rule version | No predecessor exists | Govern the first version through Draft, Validation, Testing, Approval, Published and Active | A first Active Financial Rule becomes available | Finance Foundation / F |
| Rule revision | An Active predecessor exists | Keep the predecessor effective until successor activation, then deprecate it | Historical decisions remain bound to the predecessor; future decisions use the new Active version | Finance Foundation / F |
| Multiple applicable rules | More than one rule applies | Apply the canonical precedence order | One deterministic result is produced | Finance Foundation / F |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Rule ownership, category or scope cannot be established | The proposal cannot enter the governed lifecycle | Correct or reject the proposal before activation | Finance Foundation / F |
| Validation fails | The candidate is not safe for testing or approval | Keep it outside operational use and correct it | Finance Foundation / F |
| Testing fails | Expected behavior is not demonstrated | Do not progress the candidate to approval | Finance Foundation / F |
| Rule conflict cannot be resolved under the canonical precedence order | Future decisions would not be deterministic | Reject the candidate or correct its scope/relationship before activation | Finance Foundation / F |
| Required approval is not obtained | The candidate cannot become operational | Keep the current Active version in effect | Finance Foundation / F |
| Rule evaluation fails during later Finance execution | The affected state transition is not authorized or deterministically decided | Prevent the affected financial transition and preserve the failure evidence | Finance Foundation / F |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial rules are externalized from Finance services | Finance Foundation / F | Institution-specific policy is governed as a rule rather than hardcoded behavior | L2-F00 §49.2 |
| Rule evaluation is deterministic | Finance Foundation / F | Identical inputs under the same governing rule version produce the same result | L2-F00 §49.2, §58 |
| Every Financial Rule supports versioning | Finance Foundation / F | A revision creates a distinct governed version | L2-F00 §49.2 |
| Historical transactions retain the rule version used | Finance Foundation / F | A new rule version cannot rewrite prior financial decisions | L2-F00 §49.2, §§57–58 |
| Every rule evaluation produces an explanation | Finance Foundation / F | Rule decisions remain explainable and auditable | L2-F00 §49.2 |
| Conflict resolution follows the source-defined precedence order | Finance Foundation / F | Scope, specificity, latest Active version, explicit deny, compliance and platform safety govern conflicts | L2-F00 §54 |
| The Financial Rule lifecycle is `Draft → Validation → Testing → Approval → Published → Active → Deprecated → Archived` | Finance Foundation / F | Only source-defined lifecycle positions may be used | L2-F00 §55 |
| Only Active rules participate in business execution | Finance Foundation / F | Candidate, Published, Deprecated and Archived versions do not govern new Finance decisions | L2-F00 §55 |
| Rules never directly modify financial data | Finance Foundation / F | Rule output informs Finance execution but does not perform the financial transaction | L2-F00 §58 |
| Rule evaluation occurs before financial state transitions and cannot be bypassed | Finance Foundation / F | Failed or denied evaluation prevents unauthorized financial execution | L2-F00 §§51, 58 |
| Every evaluation is auditable and historical decisions preserve governing version | Finance Foundation / F | Rule version, decision and explanation remain reconstructable | L2-F00 §§57–59 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Proposed Financial Rule version | Draft | Progresses through validation, testing, approval, publication and activation | `Draft → Validation → Testing → Approval → Published → Active` | Finance Foundation / F | L2-F00 §55 |
| Predecessor Financial Rule | Active | Replaced only after successor activation | `Active → Deprecated` | Finance Foundation / F | L2-F00 §55 |
| Historical financial decision | Existing decision tied to a rule version | No re-evaluation or mutation | Original governing version and decision remain unchanged | Applicable Finance aggregate / F | L2-F00 §49.2, §§57–58 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

F00 states that every financial activity is represented by one or more domain events and that every financial workflow shall publish domain events, but its published Finance event catalog contains no Financial Rule creation, validation, testing, approval, publication, activation, deprecation or archival event. No canonical publisher is therefore available for this scenario. No event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform Foundation / A | Finance Foundation / F | Platform configuration context | Supplies shared platform configuration scope without owning Finance rule meaning | L2-A01 §6 |
| Finance Foundation / F | Finance workflows / F | Active Financial Rule and decision | Governs future Finance execution through the canonical Rule evaluation pipeline | L2-F00 §§49–58 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Rule lifecycle requires approval, and F00 governance information includes Approved By and Approval Reference, but no canonical business actor or institutional role is assigned authority for this scenario’s Financial Rule approval. That authority is unresolved and is not inferred from generic example Finance roles.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override rule for Financial Rule creation/revision approval is defined by the authoritative L2 sources.

## 15.3 Business Audit and Evidence Obligations

F00 requires rule governance information and immutable evaluation history, including governing rule version, input context, decision, explanation and outcome. Historical financial decisions preserve the rule version used during evaluation.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported business outcome requires a Finance-owned rule version to complete Validation, Testing, Approval, Published and Active stages while historical decisions retain their governing version. The scenario cannot be fully certified until the canonical approval authority and required event semantics/publisher are defined upstream.

## 16.2 Postconditions

A successfully activated rule governs future eligible Finance decisions; a replaced predecessor is preserved in `Deprecated`, and historical decisions remain tied to their original rule version and explanation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| F00 requires financial activities/workflows to be represented by and publish domain events, but defines no canonical Financial Rule lifecycle event or publisher | Event completeness and sole publisher cannot be established for the rule-change lifecycle | Define the Financial Rule lifecycle event(s) and publisher(s), or explicitly exempt this lifecycle from the general Finance event requirement |
| F00 requires an Approval lifecycle stage and stores Approved By / Approval Reference but defines no canonical business actor or institutional role with rule-approval authority | Approval authority is materially unresolved | Define the canonical Finance Financial Rule approval authority or authoritative policy mapping |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
