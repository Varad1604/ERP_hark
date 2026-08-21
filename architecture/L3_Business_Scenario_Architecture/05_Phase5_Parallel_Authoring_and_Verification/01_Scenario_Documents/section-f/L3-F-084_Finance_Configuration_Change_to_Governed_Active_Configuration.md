# Finance Configuration Change to Governed Active Configuration

# 1. Scenario Metadata

**Scenario ID:** L3-F-084  
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
**Referenced By:** Finance operations that resolve configuration for calculation, workflow, accounting, settlement, reporting or retention behaviour

---

# 2. Scenario Purpose

Govern a change to Finance-owned configuration through scope and ownership validation, impact assessment, approval, publication and activation under the Platform Configuration Framework, while preserving prior versions and historical transaction references.

---

# 3. Business Outcome

A validated, approved and versioned configuration becomes effective for future processing and can be rolled back

L2-F00 and L2-A01 define configuration governance through activation and preservation of superseded versions, but neither defines rollback eligibility, rollback transitions, commands, events or publishers. Because rollback capability is an explicit part of the frozen outcome, this scenario is classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized change affects Finance calculation, workflow, accounting, settlement, reporting, or retention behavior.

## 4.2 End Boundary

The intended end boundary is reached when:

- the affected configuration item, owner, category and scope have been resolved;
- a distinct new version has been created with a reason and effective period;
- ownership, inheritance, dependencies, compatibility and approval have been validated;
- the new version has been published and activated for future Finance processing;
- the predecessor and all historical transaction references remain preserved;
- the effective configuration can be resolved deterministically by Finance operations;
- the entire change and activation are auditable; and
- an authoritative rollback capability exists for a governed return to a prior valid configuration.

The final rollback criterion cannot be completed canonically from the locked L2 sources.

## 4.3 In Scope

- Platform and Finance configuration ownership resolution.
- Tenant, institution, campus, department, programme and transaction scope resolution.
- Creation of a new Finance configuration version.
- Validation of ownership, supported values, completeness and version consistency.
- Rule dependency, calendar, accounting, regulatory, security and platform compatibility checks.
- Impact assessment.
- Approval through a governed workflow.
- Publication and activation.
- Deterministic effective-configuration resolution.
- Preservation of predecessor versions and historical references.
- Immutable configuration-change audit evidence.
- Identification of the unresolved rollback requirement.

## 4.4 Out of Scope

- Changing Platform-owned defaults or Tenant configuration from within Finance.
- Allowing another business domain to modify Finance-owned configuration.
- Changing the semantics or lifecycle of a Financial Rule; that belongs to L3-F-085.
- Mutating historical financial transactions after a configuration change.
- Bypassing hierarchical resolution or Platform publication.
- Inventing a rollback state, transition, command, event, publisher or approval authority.
- Technical feature flags, source-code changes, deployment, database, cache or configuration-file design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-F-085 — Financial Rule Change to Governed Active Policy | Governs financial rules under the Rule lifecycle rather than general Finance configuration. |
| Supporting | L3-A-056 — Platform and Tenant Configuration Requirement to Resolved Governed Configuration | Supplies Platform configuration and Tenant context without owning Finance-specific values. |
| Supporting | L3-A-014 — Workflow Definition to Activated Orchestration Availability | Provides governed workflow capability where approval orchestration is required. |
| Downstream | Applicable Section F operational scenarios | Consume only the published effective Finance configuration. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Finance Foundation / Section F | Primary owner. Owns Finance-specific configuration values, categories, versions, validation requirements, activation eligibility and historical references. |
| Platform & Tenant / Section A | Owns the Platform Configuration Framework, Platform defaults, Tenant configuration, hierarchy, inheritance, resolution and publication context. It does not interpret Finance-specific configuration. |
| Authorization & Policy / Section A | Supplies authorization and policy decisions for protected configuration-change activities. |
| Workflow Engine / Section A | Executes an applicable approval workflow without assuming ownership of the Finance configuration decision. |
| Event Infrastructure / Section A | May distribute a canonical configuration event after one is defined by the owning domain; it does not own the business fact. |
| Audit Infrastructure / Section A | Preserves immutable change, approval, activation and access evidence. |

The frozen participating-domain value remains exactly `A, F`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized Finance Configuration Requester | Proposes a justified change within an authorized Finance configuration scope. | May initiate the change; cannot approve or activate merely by requesting it. |
| Finance Configuration Owner | Confirms Finance ownership, meaning, category, scope and operational impact. | Owns the Finance-specific value and its business validity. |
| Configuration Validator | Performs required ownership, version, dependency and compatibility validation. | Produces validation evidence; approval authority is not implied. |
| Authorized Finance Approver | Grants or refuses the policy-required approval. | Authority must derive from Authorization & Policy and the configured workflow. |
| Platform Configuration Framework | Resolves hierarchy and publishes an immutable effective configuration context. | Owns the common framework, not Finance-specific semantics. |

L2-F00 lists example Finance roles and requires approval but does not assign these configuration responsibilities to named canonical actors. The labels above are functional responsibilities only.

---

# 7. Preconditions and Entry Criteria

- The Tenant and applicable institution context exist and permit Finance execution.
- The proposed change has an authorized requester, business justification and target scope.
- The configuration item belongs to exactly one owning domain.
- Finance owns the proposed Finance-specific configuration value.
- Platform, Tenant, institution and inherited parent configurations can be resolved.
- The predecessor version and its effective dates are available where a configuration already exists.
- Required dependency, compatibility, security, regulatory and approval information is available.
- A governed approval workflow can be selected where approval is required.
- Historical financial transactions retain their original configuration-version references.
- Audit and correlation context are available.

---

# 8. Trigger

An authorized change affects Finance calculation, workflow, accounting, settlement, reporting, or retention behavior

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authorized configuration-change proposal with its business justification. | Authorized Finance Configuration Requester; Finance Foundation | A governed change request begins. |
| 2 | Resolve the Tenant, institution and requested configuration scope. | Platform & Tenant; Finance Foundation | The change boundary is established. |
| 3 | Confirm that Finance owns the proposed configuration item and that no ownership overlap exists. | Finance Configuration Owner; Platform Configuration Framework | The single owning domain is established. |
| 4 | Identify the configuration category, predecessor version, affected behaviour, effective date and expiry date where applicable. | Finance Foundation | The proposed version context is defined. |
| 5 | Create a distinct Draft configuration version without changing the Active predecessor. | Finance Foundation | A versioned candidate exists in `Draft`. |
| 6 | Validate scope, ownership, supported values, completeness, inheritance, version consistency and dependency integrity. | Finance Foundation; Platform & Tenant | The candidate satisfies common configuration requirements. |
| 7 | Validate Finance-specific rule dependencies, financial calendar, accounting, regulatory and platform compatibility. | Finance Foundation | The candidate satisfies Finance activation constraints. |
| 8 | Complete impact assessment for calculation, workflow, accounting, settlement, reporting, retention and downstream consumers. | Finance Configuration Owner; affected Finance responsibilities | Business and historical effects are understood before approval. |
| 9 | Route the validated version through the applicable approval workflow. | Workflow Engine; Authorization & Policy; Authorized Finance Approver | Approval is granted or refused without transferring configuration ownership. |
| 10 | Record the approval reference, approver, change reason and effective timing. | Finance Foundation; Audit Infrastructure | Approval and accountability are traceable. |
| 11 | Publish the approved configuration version through the Platform Configuration Framework. | Platform & Tenant; Finance Foundation | A version-consistent published configuration context exists. |
| 12 | Activate the new version for its approved future processing scope. | Finance Foundation; Platform Configuration Framework | Only the new `Active` configuration participates in subsequent execution. |
| 13 | Mark the predecessor `Superseded` where replacement applies while retaining it for historical reconstruction. | Finance Foundation | Prior execution references remain valid and immutable. |
| 14 | Resolve and expose the effective configuration deterministically to Finance services as read-only context. | Platform & Tenant | Subsequent Finance processing receives the correct version. |
| 15 | Preserve immutable evidence of proposal, validation, impact assessment, approval, publication and activation. | Finance Foundation; Audit Infrastructure | The supported configuration-change outcome is auditable. |
| 16 | Establish the ability to roll back to an eligible prior configuration through an authoritative governed contract. | Finance Foundation; Platform & Tenant | **Blocked:** no rollback lifecycle, guard, command, event or publisher is defined upstream. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Future-effective version | Approval precedes the intended effective date. | Preserve the approved/published version without allowing it to participate until activation at the approved time. | The current Active version continues until the successor becomes Active. | Finance Foundation; Platform & Tenant |
| Scoped override | A lower scope needs a permitted override. | Validate that the property may override its inherited parent and preserve deterministic hierarchy. | The lower-scope Active version applies only within its approved scope. | Finance Foundation; Platform & Tenant |
| No existing version | The configuration item is being introduced for the first time. | Create and govern the first version through the same validation and approval path. | A first Active version becomes available without predecessor supersession. | Finance Foundation |
| Validation failure | One or more mandatory validations fail. | Retain the candidate outside Active and return it for correction. | No operational configuration changes. | Finance Foundation; Platform & Tenant |
| Approval refused | The proposed version is not approved. | Preserve the refusal and candidate history; do not publish or activate it. | The prior Active version remains effective. | Authorization & Policy; Workflow Engine; Finance Foundation |
| Rollback requested | A newly active configuration causes a governed recovery need. | Preserve both versions and determine rollback eligibility. | **Blocked:** the authoritative rollback path and resulting lifecycle states are undefined. | Finance Foundation; Platform & Tenant |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant or scope cannot be resolved. | The configuration cannot be applied deterministically. | Reject the change without affecting the Active version. | Platform & Tenant; Finance Foundation |
| Configuration ownership overlaps or is incorrect. | The change would violate single ownership. | Stop progression and route the item to its canonical owner. | Platform & Tenant; Finance Foundation |
| Parent configuration is missing or dependencies are cyclic. | Hierarchical resolution is invalid. | Reject activation until dependencies are corrected. | Platform & Tenant |
| Finance compatibility validation fails. | The configuration could cause invalid financial behaviour. | Retain the candidate outside Active and preserve validation evidence. | Finance Foundation |
| Required approval is not granted. | The candidate cannot become operational. | Do not publish or activate it. | Authorization & Policy; Workflow Engine |
| Publication or activation is interrupted. | The candidate must not be consumed as Active. | Continue using the prior Active version; preserve the incomplete outcome for governed recovery. | Platform & Tenant; Finance Foundation |
| Effective configuration cannot be resolved. | Finance processing cannot safely determine behaviour. | Prevent the affected financial execution until a valid resolved context is available. | Platform & Tenant; Finance Foundation |
| Rollback is required. | Operational recovery cannot be expressed canonically. | Do not invent a reversal or reactivate a prior version without upstream authority; escalate the architecture gap. | Finance Foundation; Platform & Tenant |
| Success-event publication is expected. | No canonical Finance configuration lifecycle event exists. | Do not invent an event or publisher. | Finance Foundation; Event Infrastructure |

No technical deployment rollback, database restoration, cache invalidation or configuration-file recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| The Platform owns configuration architecture; Finance owns only Finance-specific values. | Platform & Tenant; Finance Foundation | Framework and semantic ownership remain separated. | L2-A01 §6.2, §6.5; L2-F00 §80–§82 |
| Every configuration item has exactly one owner. | Platform & Tenant | Ownership overlap prevents progression. | L2-A01 §6.5, §6.15; L2-F00 §80.2, §82, §89 |
| Configuration resolution is hierarchical and deterministic. | Platform & Tenant | Identical execution contexts resolve the same effective version. | L2-A01 §6.4, §6.7–§6.8; L2-F00 §80.2 |
| Business domains consume resolved configuration rather than resolving hierarchy independently. | Platform & Tenant | Finance receives published read-only context. | L2-A01 §6.11, §6.14–§6.15; L2-F00 §83, §89 |
| Finance configuration is version controlled. | Finance Foundation | Changes create distinct versions and do not alter historical transactions. | L2-F00 §80.2, §86, §89 |
| The Finance configuration lifecycle is `Draft → Validation → Approval → Published → Active → Superseded → Archived`. | Finance Foundation | Only source-defined lifecycle positions may be used. | L2-F00 §84 |
| Platform configuration uses `Draft → Validation → Approved → Active → Superseded → Archived`. | Platform & Tenant | Publication remains a Platform framework activity, while the exact crosswalk to F00 `Published` is not specified. | L2-A01 §6.10 |
| Only Active configuration participates in Finance execution. | Finance Foundation | Candidate, Published, Superseded and Archived versions cannot govern new processing. | L2-F00 §84 |
| Activation requires validation and approval. | Finance Foundation; Platform & Tenant | No bypass of governance is permitted. | L2-F00 §85, §87, §89; L2-A01 §6.9–§6.10 |
| Historical transactions retain the effective configuration version. | Finance Foundation | Configuration updates never rewrite historical financial behaviour. | L2-F00 §80.2, §86, §89 |
| Every configuration change is auditable. | Finance Foundation; Audit Infrastructure | Previous/new versions, fields, reason, approver, activation time, scope and workflow reference are retained. | L2-F00 §88; L2-A01 §6.12, §6.15 |
| Rollback is not defined by either governing L2 source. | Finance Foundation; Platform & Tenant | The frozen recovery capability cannot be inferred from supersession. | L2-F00 and L2-A01 source gap |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Finance configuration candidate | No new version or `Draft` | Validated, approved, published and activated. | `Draft → Validation → Approval → Published → Active`. | Finance Foundation |
| Predecessor Finance configuration | `Active` where replacement applies | Remains active until successor activation, then is superseded. | `Active → Superseded`; later archival may occur separately. | Finance Foundation |
| Platform resolved configuration context | Existing valid context | Re-resolved and published after activation. | New immutable version-consistent context available to Finance. | Platform & Tenant |
| Historical financial transaction | References its original configuration version | No mutation. | Original version reference remains unchanged. | Applicable Finance aggregate |
| Rollback target | Prior valid version preserved | Would need governed reactivation or a successor based on prior values. | **Undefined:** no canonical rollback state or transition. | Finance Foundation; Platform & Tenant boundary unresolved |
| Audit evidence | Correlation context available | Receives immutable change and activation evidence. | Audit evidence available under A07 governance. | Audit Infrastructure |

L2-F00 and L2-A01 use different lifecycle wording around `Approval`/`Approved` and `Published`. This Draft follows the Finance-specific L2-F00 lifecycle for the Finance configuration while using A01 for framework resolution and publication.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Finance Configuration Event Determination

Neither L2-F00 nor L2-A01 defines canonical configuration-change event names or sole publishers.

| Required Business Fact | Source Resolution Result | Scenario Treatment |
|---|---|---|
| Configuration version created | No canonical event or publisher defined. | No event is invented. |
| Configuration validated | No canonical event or publisher defined. | No event is invented. |
| Configuration approved | No canonical event or publisher defined. | No event is invented. |
| Configuration published or activated | No canonical event or publisher defined. | No event is invented. |
| Prior configuration superseded | No canonical event or publisher defined. | No event is invented. |
| Configuration rolled back | No lifecycle contract, event or publisher defined. | Blocking gap; no event is invented. |

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Platform and Tenant configuration context | Platform & Tenant | Finance Foundation | Parent hierarchy, Tenant scope, inherited values and validation context. |
| Finance configuration semantics | Finance Foundation | Platform Configuration Framework | Finance-owned values, category, scope, version and effective period. |
| Authorization Decision | Authorization & Policy | Finance Foundation; Workflow Engine | Permission and scope outcome for protected change activities. |
| Approval workflow outcome | Workflow Engine | Finance Foundation | Approval, refusal, approver and workflow reference. |
| Resolved effective configuration | Platform Configuration Framework | Finance services | Validated, complete, Tenant-aware, version-consistent and read-only context. |
| Configuration audit input | Finance Foundation; Platform & Tenant | Audit Infrastructure | Versions, changed fields, reason, approver, activation, actor and correlation evidence. |
| Rollback contract | Not defined | Finance Foundation; Platform & Tenant | Required eligibility, action and outcome are unresolved. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Proposal, validation, approval, publication and activation are protected activities.
- Authorization must respect Tenant, institution, configuration category and scope.
- Finance ownership does not confer permission on every Finance user.
- Platform framework ownership does not allow Platform to change Finance-specific semantics.
- No technical role, claim, ACL or enforcement mechanism is introduced.

## 15.2 Approval and Separation of Duty

- Configuration activation requires validation and approval.
- The configured Approval Workflow may use sequential, parallel, delegated or acting-authority behaviour where permitted upstream.
- Critical financial activities remain subject to SoD; the requester should not be assumed to approve their own change.
- L2-F00 does not name the exact configuration approver or define the approval matrix for this scenario.
- No rollback approver or override authority is invented.

## 15.3 Audit Obligations

The following must be auditable:

- Tenant, institution, campus and lower scope where applicable.
- Configuration ID, type, category and owning domain.
- Previous and new version.
- Changed fields and business reason.
- Effective and expiry dates.
- Validation results and dependency checks.
- Impact assessment.
- Approval reference, approver and workflow reference.
- Publication and activation time.
- Predecessor supersession.
- Effective-configuration resolution context.
- Any requested rollback and its unresolved or later governed outcome.
- Actor, timestamp, correlation and final result.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- the authorized change request was received within one Tenant context;
- Finance ownership and configuration scope were confirmed;
- a distinct Draft version was created;
- Platform and Finance validations succeeded;
- impact assessment was completed;
- required approval was granted and recorded;
- the version was published and became `Active` for future processing;
- the predecessor was preserved and superseded where applicable;
- Platform resolution exposed the correct immutable effective context;
- historical transactions retained their original configuration references;
- immutable audit evidence was preserved; and
- the Active configuration could be rolled back through a canonical governed path.

All criteria except the final rollback capability are supported. Because the frozen outcome explicitly requires rollback and no authoritative rollback contract exists, the scenario remains `BLOCKED`.

Postconditions:

- Future in-scope Finance operations consume the new Active version.
- Existing historical transactions remain unchanged.
- Platform and Finance ownership boundaries remain intact.
- No rollback state, command or event has been inferred.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Precise Scenario Impact | Required Resolution |
|---|---|---|---|---|
| F084-BLOCKER-01 | Blocking | The frozen outcome requires that Active Finance configuration “can be rolled back,” but L2-F00 and L2-A01 contain no rollback eligibility rules, lifecycle transition, command or recovery workflow. | The scenario cannot specify how a prior version becomes effective again without violating version history or lifecycle rules. | Define whether rollback creates a new version or reactivates a prior version, plus guards, approval and resulting states. |
| F084-BLOCKER-02 | Blocking | No canonical Finance configuration lifecycle events or sole publishers are defined. | Activation, supersession and rollback cannot be communicated as canonical completed business facts. | Define exact event names, publication conditions and one publisher per event. |
| F084-BLOCKER-03 | Blocking | L2-F00 and L2-A01 define no configuration-change commands and do not name the request, approval or activation authorities. | L4 cannot derive authoritative operation contracts or actor authority boundaries. | Define commands and policy-derived actor responsibilities. |
| F084-ISSUE-01 | Non-blocking | L2-F00 uses `Approval → Published → Active`, while L2-A01 uses `Approved → Active` and describes publication as a framework responsibility outside that lifecycle. | Finance-specific and Platform framework transition alignment is not explicit. | Add a crosswalk identifying how F00 publication maps to the A01 lifecycle. |

**Source Assessment:** BLOCKED

No unsupported actor, rollback rule, lifecycle state, command, event, publisher, approval authority or technical recovery mechanism is assumed by this Draft.
