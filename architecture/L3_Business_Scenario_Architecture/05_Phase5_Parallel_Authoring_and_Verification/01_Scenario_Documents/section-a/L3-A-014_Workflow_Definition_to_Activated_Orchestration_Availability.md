# Workflow Definition to Activated Orchestration Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-014  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workflow Engine  
**Participating Domains:** A, VAR:Workflow-Owning Domain  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Workflow-owning domains and L3-A-015 — Workflow Initiation to Completed Orchestrated Execution

---

# 2. Scenario Purpose

Establish a governed Workflow Definition and immutable Workflow Version that coordinates an approved business process while preserving the workflow-owning domain's ownership of business rules, decisions, state and outcomes.

---

# 3. Business Outcome

A validated, approved, immutable workflow version is published and activated for use by the owning business domain.

L2-A05 does not define the canonical governance-approval actor, activation authority or definition-lifecycle command contracts required to complete the outcome exactly. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a business domain requires a governed workflow to coordinate an approved business process.

## 4.2 End Boundary

The intended end boundary is reached when the approved Workflow Version is immutable, `Published`, `Activated` as the current version for new Workflow Instances, available through governed contracts, and auditable.

## 4.3 In Scope

- Confirmation of the workflow-owning domain and approved business-process boundary.
- Creation of a Workflow Definition in `Draft`.
- Definition of workflow metadata, orchestration structure, business references, input/output contracts, policy references and event references.
- Mandatory structural, policy, event, version, dependency and compatibility validation.
- Governance approval.
- Publication of an immutable Workflow Version.
- Activation for new Workflow Instances.
- Publication of canonical workflow lifecycle events.
- Audit evidence for definition, validation, approval, publication and activation.

## 4.4 Out of Scope

- Creating or changing the workflow-owning domain's business rules, decisions, lifecycle states or business events.
- Executing a Workflow Instance.
- Migrating an existing Workflow Instance.
- Retiring a Workflow Version.
- Designing user interfaces, APIs, database structures or implementation components.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-A-015 — Workflow Initiation to Completed Orchestrated Execution | An activated version becomes available for new governed executions. |
| Related | L3-A-016 — Workflow Execution Failure to Controlled Recovery Outcome | Owns controlled recovery after a running instance fails. |
| Related | L3-A-017 — Workflow Change to Governed New Version Availability | Owns governed successor-version creation after a structural or orchestration change. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Workflow Engine / Section A | Primary owner. Owns Workflow Definition, Workflow Version, validation, publication, activation and workflow lifecycle events. |
| VAR:Workflow-Owning Domain | Owns the approved business process, business rules, decisions, business state, business commands and business events coordinated by the workflow. |
| Platform & Tenant / Section A | Supplies Tenant context and configuration. |
| Person & Identity / Section A | Supplies identity references where the definition requires human-task participation. |
| Authentication / Section A | Supplies authenticated context for governed lifecycle actions. |
| Authorization & Policy / Section A | Supplies Authorization Decisions, assignment resolution and policy references without owning workflow structure. |
| Event Infrastructure / Section A | Governs event publication and distribution without owning workflow facts. |
| Audit Infrastructure / Section A | Owns immutable evidence generated from canonical workflow events. |

The frozen Participating Domains value remains exactly `A, VAR:Workflow-Owning Domain`.

---

# 6. Business Actors, Roles and Responsibilities

L2-A05 explicitly records that its Actors section is absent. It does not name:

- the actor who submits or authors a Workflow Definition;
- the governance approver;
- the actor authorized to publish a Workflow Version; or
- the actor authorized to activate it.

The workflow-owning domain is a canonical participant, not a substitute business actor. No author, administrator, approver or activation authority is invented.

---

# 7. Preconditions and Entry Criteria

- The Workflow-Owning Domain and approved business-process boundary are established.
- The request belongs to exactly one Tenant.
- The business process, business-rule ownership, decision ownership and expected orchestration boundary are defined by the owning domain.
- Required business references and input/output expectations are available.
- Required policy, event and dependency references can be resolved through canonical contracts.
- The proposed workflow does not require the Workflow Engine to own or evaluate business rules.

---

# 8. Trigger

A business domain requires a governed workflow to coordinate an approved business process.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the requirement for governed orchestration of the approved business process. | VAR:Workflow-Owning Domain; Workflow Engine | A Workflow Definition requirement is established. |
| 2 | Confirm the Workflow-Owning Domain and preserve its ownership of business rules, decisions, states, commands and events. | Workflow Engine; VAR:Workflow-Owning Domain | The orchestration boundary is established without ownership transfer. |
| 3 | Create the Workflow Definition in `Draft` and publish `WorkflowDefinitionCreated`. | Workflow Engine | The governed definition lifecycle begins. |
| 4 | Define the workflow metadata, orchestration structure, business references, input/output contracts, policy references, event references and validation metadata. | Workflow Engine; VAR:Workflow-Owning Domain supplies business semantics | A complete definition is available for validation. |
| 5 | Validate graph integrity, connectivity, routing, variables, assignments, policies, events, version compatibility and cross-domain dependencies. | Workflow Engine | The definition enters `Validation` and either satisfies or fails mandatory validation. |
| 6 | Obtain governance approval and complete `Validation → Approved`. | Canonical governance approver undefined; Workflow Engine | The definition is approved for publication. |
| 7 | Publish an immutable Workflow Version, complete `Approved → Published` and publish `WorkflowVersionPublished`. | Workflow Engine | An immutable executable version becomes available. |
| 8 | Activate the published version, complete `Published → Activated` and publish `WorkflowVersionActivated`. | Canonical activation authority undefined; Workflow Engine | The version becomes current for new Workflow Instances. |
| 9 | Make the activated version and its governed input/output contracts available to the Workflow-Owning Domain. | Workflow Engine | The owning domain may initiate later executions without owning workflow infrastructure. |
| 10 | Preserve immutable evidence of definition, validation, approval, publication and activation. | Workflow Engine; Audit Infrastructure | The orchestration-availability outcome remains traceable. |

Steps 6–8 cannot be finalized as exact authority and contract behaviour because the approver, activation authority and canonical definition commands are absent from L2-A05.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Template-based definition | A governed reusable template matches the approved business pattern. | Create a domain-specific copy, preserve template independence and validate the resulting Workflow Definition through the same lifecycle. | An independently governed immutable version is published and activated. | Workflow Engine; VAR:Workflow-Owning Domain |

Template modification never changes existing Workflow Definitions.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Workflow-Owning Domain or business-process boundary is unresolved. | Business ownership and orchestration scope cannot be preserved. | Stop authoring until the owning domain establishes the process boundary. | VAR:Workflow-Owning Domain; Workflow Engine |
| Mandatory definition information is incomplete. | Validation cannot establish a complete governed definition. | Correct the `Draft` before resubmitting it to validation. | Workflow Engine; VAR:Workflow-Owning Domain |
| Mandatory validation fails. | Publication is prohibited. | Retain the editable definition outside `Published`, correct the identified definition issue and repeat validation. | Workflow Engine |
| Governance approval is not granted. | `Validation → Approved` cannot complete. | Do not publish or activate the version. | Workflow Engine |
| Publication is incomplete. | No valid immutable executable version exists. | Do not activate; partial publication is prohibited. | Workflow Engine |
| Activation does not complete. | The version cannot be used for new instances as the active version. | Retain it outside `Activated`; exact recovery authority is undefined upstream. | Workflow Engine |
| Required lifecycle event cannot be published. | Canonical consumers and audit evidence cannot receive the completed workflow fact. | Do not invent or rename an event; preserve the incomplete outcome for governed resolution. | Workflow Engine; Event Infrastructure |

No technical retry, rollback or implementation recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Workflow Engine exclusively owns Workflow Definitions, Versions, validation, publication and activation. | Workflow Engine | No business domain may directly manipulate workflow infrastructure. | L2-A05 §§3, 8 and 17 |
| Business domains own business processes, rules, decisions, state machines, commands and events. | VAR:Workflow-Owning Domain | Workflow definition coordinates but never re-owns business behaviour. | L2-A05 §5 and WEP-02 |
| Only published Workflow Versions may be executed; Draft definitions cannot execute. | Workflow Engine | Activation cannot precede successful publication. | L2-A05 §§25 and 60 — RE-01 |
| Mandatory validation precedes publication. | Workflow Engine | Any validation failure prevents publication. | L2-A05 §§34–35 and WD-04 |
| Partial publication is prohibited. | Workflow Engine | Every publication stage must complete successfully. | L2-A05 §35 |
| Published Workflow Versions are immutable. | Workflow Engine | Any later change requires a new version under A017. | L2-A05 WEP-05, §§36 and 40 — WD-02 |
| Workflow Definitions expose explicit input and output contracts. | Workflow Engine | Activated orchestration must have governed execution boundaries. | L2-A05 §§30–31 and WD-03 |
| Workflow execution remains isolated within one Tenant. | Workflow Engine; Platform & Tenant | The definition and resulting executions cannot cross Tenant boundaries. | L2-A05 WEP-08 |
| Workflow events are published by the Workflow Engine and remain immutable. | Workflow Engine; Event Infrastructure | Lifecycle facts retain one publisher and governed distribution. | L2-A05 §§10, 22 and 105; L2-A06 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Workflow Definition / Version lifecycle | No definition exists | Create an editable definition and perform mandatory validation. | `Draft → Validation` | Workflow Engine | L2-A05 §26 |
| Workflow Definition / Version lifecycle | `Validation` | Complete required governance approval. | `Approved` | Workflow Engine | L2-A05 §26 |
| Workflow Version | `Approved` | Publish as an immutable executable version. | `Published` | Workflow Engine | L2-A05 §§26, 35–36 |
| Workflow Version | `Published` | Activate for new Workflow Instances. | `Activated` | Workflow Engine | L2-A05 §§26 and 35 |
| Workflow-Owning Domain business process | Approved business process | Supplies semantics and decisions; no business-domain lifecycle is changed by definition activation. | Unchanged. | VAR:Workflow-Owning Domain | L2-A05 WEP-02 and WD-06 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `WorkflowDefinitionCreated` | Workflow Engine | VAR:Workflow-Owning Domain; Audit Infrastructure | Establish that the governed definition lifecycle began. | L2-A05 §22 |
| `WorkflowVersionPublished` | Workflow Engine | VAR:Workflow-Owning Domain; Audit Infrastructure | Establish availability of an immutable executable version. | L2-A05 §§22 and 35 |
| `WorkflowVersionActivated` | Workflow Engine | VAR:Workflow-Owning Domain; Audit Infrastructure | Establish that the version is current for new instances. | L2-A05 §§22 and 35 |

Event Infrastructure owns governed distribution; it does not own the workflow facts.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| VAR:Workflow-Owning Domain | Workflow Engine | Approved business-process definition and ownership boundary | Supply business semantics without transferring decision ownership. | L2-A05 WEP-02 and WD-06 |
| Workflow Definition | Workflow-Owning Domain | Workflow Input Contract and Workflow Output Contract | Establish governed execution inputs and outputs. | L2-A05 §§30–31 |
| Platform & Tenant | Workflow Engine | Tenant Context and configuration | Establish isolation and applicable configuration. | L2-A05 §174 |
| Authorization & Policy | Workflow Engine | Authorization Decision, assignment resolution and policy references | Govern definition lifecycle actions and references. | L2-A05 §§129 and 174 |
| Event Infrastructure | Workflow Engine | Approved event definitions and contracts | Validate event references and distribute published workflow facts. | L2-A05 §§34 and 105; L2-A06 |
| Workflow Engine | Audit Infrastructure | Canonical workflow lifecycle events | Generate immutable evidence. | L2-A06; L2-A07 A07-W01 |

L2-A05 does not define canonical commands for creating, validating, approving, publishing or activating Workflow Definitions and Versions.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish business-process semantics | Canonical actor undefined | The Workflow-Owning Domain owns the approved business process and its decisions. | VAR:Workflow-Owning Domain |

The governance approver, publication authority and activation authority are not named upstream.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception role is defined for Workflow Definition approval or activation. Applicable Authorization Policy references are validated, but no new SoD rule is introduced.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Definition creation | `WorkflowDefinitionCreated`, owner domain and definition identity. | Workflow Engine |
| Validation | Mandatory validation outcome and identified deficiencies. | Workflow Engine |
| Governance approval | Approval outcome and authority reference; actor is undefined. | Workflow Engine |
| Publication | `WorkflowVersionPublished` and immutable version identity. | Workflow Engine |
| Activation | `WorkflowVersionActivated` and active-version relationship. | Workflow Engine |
| Immutable audit evidence | Evidence generated from canonical workflow events. | Audit Infrastructure |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Workflow-Owning Domain and approved business-process boundary were preserved.
- The Workflow Definition was created and validated.
- Every mandatory structural, policy, event, version and dependency validation succeeded.
- Governance approval was granted.
- An immutable Workflow Version was published.
- The published version was activated for new Workflow Instances.
- Input and output contracts became available to the owning domain.
- `WorkflowDefinitionCreated`, `WorkflowVersionPublished` and `WorkflowVersionActivated` were published by the Workflow Engine.
- Definition, approval, publication and activation remained auditable.

These criteria cannot yet be validated as a complete canonical outcome because the approval and activation actors and definition-lifecycle commands are undefined.

## 16.2 Postconditions

- The Workflow Version is `Activated` and available for new instances.
- The published version is immutable.
- No Workflow Instance has been started by this scenario.
- The Workflow-Owning Domain retains ownership of business rules, decisions, state and outcomes.
- Later changes require a governed successor version under A017.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A014-BLOCKER-01:** L2-A05 records Actors as absent and names no governance approver, publication authority or activation authority. | Material approval and activation responsibility cannot be assigned in Sections 6, 9 and 15. | Define the canonical business actors and authority boundaries. |
| **A014-BLOCKER-02:** L2-A05's canonical command catalogue contains runtime commands but no definition/version commands for creation, validation, approval, publication or activation. | The governed state-changing contracts required by later L4 behaviour are missing. | Define canonical Workflow Definition and Workflow Version command contracts. |
| **A014-ISSUE-01:** The §26 lifecycle contains `Draft`, `Validation`, `Approved`, `Published`, `Activated` and `Retired`, while §28 metadata Status lists only `Draft`, `Published` and `Retired`. | Metadata and lifecycle-state vocabulary are internally inconsistent, although the §26 transition path is explicit. | Align Workflow Metadata with the canonical definition lifecycle. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
