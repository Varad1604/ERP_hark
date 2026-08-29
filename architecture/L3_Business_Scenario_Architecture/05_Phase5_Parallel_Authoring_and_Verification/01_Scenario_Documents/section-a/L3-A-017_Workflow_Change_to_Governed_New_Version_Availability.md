# Workflow Change to Governed New Version Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-017  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workflow Engine  
**Participating Domains:** A, VAR:Workflow-Owning Domain  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A05 — Workflow Engine Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-014 — Workflow Definition to Activated Orchestration Availability  
**Referenced By:** Workflow-owning domains requiring governed orchestration change

---

# 2. Scenario Purpose

Create and govern a successor Workflow Version for an approved structural or orchestration change while preserving every published version, protecting active Workflow Instances from silent change and governing any explicit migration separately.

---

# 3. Business Outcome

A new validated immutable workflow version becomes available for new executions while existing instances retain their originating version or follow an explicitly governed migration path.

L2-A05 defines version immutability, validation, publication, activation and migration safeguards, but does not define the canonical change approver, publication/activation authorities or definition-version command contracts. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an active workflow requires an approved structural or orchestration change.

## 4.2 End Boundary

The intended end boundary is reached when a successor immutable Workflow Version has passed mandatory validation, approval, publication and activation; it is available for new Workflow Instances; existing instances remain bound to their originating version unless each approved migration satisfies the governed migration requirements; and all outcomes are auditable.

## 4.3 In Scope

- Confirmation of the Workflow-Owning Domain and approved change boundary.
- Creation of a new Draft derived from, but independent of, an existing published version.
- Structural, routing, assignment, policy, event, input/output, dependency and compatibility changes.
- Mandatory validation and governance approval.
- Publication and activation of a new immutable version.
- Default preservation of existing instances on their originating versions.
- Explicit migration planning, compatibility validation, governance approval, administrator authorization and audit recording where migration is proposed.
- Canonical version events and immutable audit evidence.

## 4.4 Out of Scope

- Mutating a published Workflow Version in place.
- Automatically migrating active instances.
- Changing the Workflow-Owning Domain's business rules, decisions, commands, states or events without that domain's approval.
- Executing new Workflow Instances; that belongs to A015.
- Recovering failed Workflow Instances; that belongs to A016.
- Inventing a change requester, approver, administrator, command or migration state.
- APIs, schemas, deployment, source-code or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Upstream | L3-A-014 — Workflow Definition to Activated Orchestration Availability | Establishes the governed definition and original version lifecycle. |
| Followed by | L3-A-015 — Workflow Initiation to Completed Orchestrated Execution | New instances may use the newly activated version. |
| Related | L3-A-016 — Workflow Execution Failure to Controlled Recovery Outcome | Failed instances remain on their originating version unless a separately governed migration is approved. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Workflow Engine / Section A | Primary owner. Owns Workflow Definition, Workflow Version, validation, publication, activation, instance-version binding, migration controls and workflow lifecycle events. |
| VAR:Workflow-Owning Domain | Owns the changed business-process requirement, business semantics, decisions, commands, states and business events. |
| Platform & Tenant / Section A | Supplies Tenant context and configuration. |
| Person & Identity / Section A | Supplies canonical actor references for protected change and migration actions. |
| Authentication / Section A | Supplies authenticated context for protected lifecycle actions. |
| Authorization & Policy / Section A | Supplies authorization and policy decisions for approval, activation and migration. |
| Event Infrastructure / Section A | Distributes canonical version events without owning them. |
| Audit Infrastructure / Section A | Owns immutable evidence derived from canonical version and migration facts. |

The frozen Participating Domains value remains exactly `A, VAR:Workflow-Owning Domain`.

---

# 6. Business Actors, Roles and Responsibilities

L2-A05 records Actors as absent. It requires governance approval and, for migration, Administrator Authorization, but does not identify the canonical change requester, definition author, governance approver, publishing authority, activation authority or administrator role. The Workflow-Owning Domain is a participant, not a substitute actor. None of these roles is invented.

---

# 7. Preconditions and Entry Criteria

- Exactly one Tenant and Workflow-Owning Domain are resolved.
- An existing published or activated immutable Workflow Version exists.
- The owning domain has approved a structural or orchestration change requirement.
- The business effects and boundaries of the proposed change are known.
- Required domain, policy, event, assignment, input/output and dependency contracts can be resolved.
- Active Workflow Instances and their originating-version bindings can be identified.
- Any migration proposal can be evaluated independently from successor-version availability.
- No request attempts to edit a published Workflow Version in place.

---

# 8. Trigger

An active workflow requires an approved structural or orchestration change.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the approved structural or orchestration change requirement from the Workflow-Owning Domain. | VAR:Workflow-Owning Domain; Workflow Engine | A governed successor-version requirement is established. |
| 2 | Confirm the owning domain, Tenant, current version and business ownership boundary. | Workflow Engine; Platform & Tenant; VAR:Workflow-Owning Domain | The change cannot cross Tenant or ownership boundaries. |
| 3 | Create a new independent Workflow Definition Draft derived from the relevant prior version without modifying that version. | Workflow Engine | A successor Draft enters the governed definition lifecycle. |
| 4 | Apply the approved changes to workflow structure, routing, assignments, references and input/output contracts while preserving domain ownership. | Workflow Engine; VAR:Workflow-Owning Domain supplies business semantics | The proposed successor definition is complete. |
| 5 | Evaluate the effect on policy references, event contracts, dependencies, data/contract compatibility and active-instance behaviour. | Workflow Engine; applicable participating domains | Version and cross-domain impacts are known before approval. |
| 6 | Complete mandatory structural, policy, event, version, dependency and compatibility validation. | Workflow Engine | The Draft either satisfies validation or remains ineligible for approval. |
| 7 | Obtain governance approval and complete `Validation → Approved`. | Canonical governance approver undefined; Workflow Engine | The successor is approved for publication. |
| 8 | Publish the successor as an immutable Workflow Version, complete `Approved → Published` and publish `WorkflowVersionPublished`. | Workflow Engine | The governed successor version becomes executable. |
| 9 | Activate the published successor for new Workflow Instances, complete `Published → Activated` and publish `WorkflowVersionActivated`. | Canonical activation authority undefined; Workflow Engine | New executions may select the successor version. |
| 10 | Preserve every existing Workflow Instance on its originating Workflow Version by default. | Workflow Engine | No active instance changes silently. |
| 11 | If migration is proposed, require a Migration Plan, Compatibility Validation, Governance Approval, Administrator Authorization and Audit Recording before changing any eligible instance-version relationship. | Workflow Engine; canonical authorities undefined | Migration is explicit and separately governed. |
| 12 | When migration is unsafe or not approved, continue the existing instance on its original version. | Workflow Engine | Execution integrity takes precedence over version convergence. |
| 13 | Preserve immutable evidence of change rationale, validation, approval, publication, activation and every migration decision. | Workflow Engine; Audit Infrastructure | The successor-version outcome is traceable. |

Steps 7, 9 and 11 cannot be finalized as exact business-authority behaviour because the relevant actors and commands are not defined upstream.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| No active instances | No current Workflow Instance uses the prior version. | Publish and activate the successor after normal validation and approval. | New executions use the activated successor; prior versions remain immutable. | Workflow Engine |
| Active instances retained | One or more instances use the prior version and no migration is proposed. | Keep each instance bound to its originating version. | The successor is available only to new executions. | Workflow Engine |
| Migration proposed and safe | A Migration Plan exists and all compatibility, governance and authorization requirements succeed. | Apply only the explicitly approved migration path to eligible instances and record each outcome. | Approved instances follow the governed successor path. | Workflow Engine; applicable participants |
| Migration unsafe or rejected | Compatibility, governance or authorization fails. | Do not migrate; continue each instance on its originating version. | The successor remains available for new executions without endangering active instances. | Workflow Engine |
| Prior version retirement considered | The successor is active and prior-version use is being reviewed. | Retire the prior version only through its separate governed lifecycle and without invalidating bound active instances. | Retirement is not required for A017 completion. | Workflow Engine |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Change ownership or boundary is unresolved. | The Workflow Engine cannot preserve business ownership. | Stop the change until the Workflow-Owning Domain supplies an approved boundary. | VAR:Workflow-Owning Domain; Workflow Engine |
| A published version is targeted for direct modification. | Version immutability would be violated. | Reject in-place change and create a successor Draft instead. | Workflow Engine |
| Mandatory validation fails. | The successor cannot be published. | Correct the Draft and repeat the full required validation. | Workflow Engine; applicable participants |
| Governance approval is absent. | The successor cannot enter `Approved`. | Do not publish or activate it. | Workflow Engine |
| Publication is incomplete. | No valid immutable successor exists. | Do not activate; partial publication is prohibited. | Workflow Engine |
| Activation fails or is unauthorized. | The successor cannot become current for new instances. | Retain it outside `Activated`; exact activation authority and recovery contract are undefined. | Workflow Engine; Authorization & Policy |
| Active instance is selected for automatic migration. | Originating-version integrity would be violated. | Reject automatic migration and require the complete governed migration path. | Workflow Engine |
| Migration compatibility fails. | The instance cannot safely use the successor. | Continue on the originating version. | Workflow Engine; applicable participating domain |
| Migration approval or authorization is absent. | The instance-version relationship cannot change. | Do not migrate; record the denied or incomplete outcome. | Workflow Engine; Authorization & Policy |
| Event publication condition is not satisfied. | Consumers cannot rely on an incomplete version fact. | Do not publish the corresponding success event. | Workflow Engine; Event Infrastructure |

No technical rollback, deployment recovery or data-conversion mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Published Workflow Versions are immutable. | Workflow Engine | Every structural or orchestration change creates a successor version. | L2-A05 WEP-05, WD-02 and version rules |
| Mandatory validation precedes publication. | Workflow Engine | An invalid successor cannot become available. | L2-A05 WD-04 and validation rules |
| Partial publication is prohibited. | Workflow Engine | Publication must complete as one governed outcome. | L2-A05 publication rules |
| Only published versions may execute. | Workflow Engine | New instances cannot use the successor before publication. | L2-A05 RE-01 |
| Workflow Engine owns orchestration; domains own business behaviour. | Workflow Engine; VAR:Workflow-Owning Domain | Version change cannot transfer rule or decision ownership. | L2-A05 WEP-01–WEP-03 |
| Existing instances retain their originating version by default. | Workflow Engine | Activation affects new execution, not active instances. | L2-A05 versioning and migration rules |
| Migration is never automatic. | Workflow Engine | Every migration requires explicit governance. | L2-A05 migration rules |
| Migration requires a plan, compatibility validation, governance approval, administrator authorization and audit recording. | Workflow Engine; Authorization & Policy | Instance-version change is allowed only after all safeguards succeed. | L2-A05 migration controls |
| Unsafe migration must not proceed. | Workflow Engine | The original version continues for the affected instance. | L2-A05 migration rules |
| Workflow lifecycle events are published by Workflow Engine. | Workflow Engine; Event Infrastructure | Version facts retain one owner and canonical distribution. | L2-A05 event catalogue; L2-A06 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Prior Workflow Version | `Published` or `Activated` | Referenced as predecessor; never modified. | Unchanged unless separately retired. | Workflow Engine |
| Successor Workflow Definition / Version | Does not exist | Create, change and validate an independent Draft. | `Draft → Validation` | Workflow Engine |
| Successor Workflow Definition / Version | `Validation` | Apply governance approval. | `Approved` | Workflow Engine |
| Successor Workflow Version | `Approved` | Publish immutably. | `Published` | Workflow Engine |
| Successor Workflow Version | `Published` | Activate for new instances. | `Activated` | Workflow Engine |
| Existing Workflow Instance | Bound to originating version | Preserve binding by default. | Continues on originating version. | Workflow Engine |
| Migration-eligible Workflow Instance | Explicit migration proposed | Change binding only after every migration safeguard succeeds. | Governed migrated relationship; L2-A05 defines no migration state. | Workflow Engine |
| Workflow-Owning Domain business process | Approved change requirement | Supplies semantics without transferring lifecycle ownership. | Domain-defined process remains owned by the domain. | VAR:Workflow-Owning Domain |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Publication Condition | Business Effect / Consumers |
|---|---|---|---|
| `WorkflowDefinitionCreated` | Workflow Engine | The independent successor Draft is created. | Owning domain and audit receive the new definition fact. |
| `WorkflowDraftUpdated` | Workflow Engine | A governed change to the successor Draft completes. | Review and audit consumers receive the Draft-change fact. |
| `WorkflowVersionPublished` | Workflow Engine | Validation and approval succeed and the immutable successor is published. | The successor becomes available as an executable version. |
| `WorkflowVersionActivated` | Workflow Engine | The published successor becomes current for new executions. | Owning domain may initiate new instances on that version. |
| `WorkflowVersionRetired` | Workflow Engine | A separate governed retirement transition succeeds. | Consumers learn that the retired version is unavailable for new execution; retirement is outside A017 completion. |

L2-A05 defines no canonical migration event. This Draft therefore does not invent one; migration evidence is retained through governed lifecycle and audit contracts pending upstream clarification.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Contract / Hand-off | Business Information / Outcome |
|---|---|---|---|
| VAR:Workflow-Owning Domain | Workflow Engine | Approved change requirement and business ownership boundary | Changed business semantics and expected orchestration outcome. |
| Workflow Engine | Applicable domains | Successor input/output, policy, event and dependency validation | Compatibility and governance requirements for the new version. |
| Workflow Engine | VAR:Workflow-Owning Domain | Published and activated successor version | Version identity and governed input/output contracts for new executions. |
| Workflow Engine | Authorization & Policy | Protected lifecycle or migration action | Context required for an authoritative decision. |
| Authorization & Policy | Workflow Engine | Approval/authorization decision | Governed allow or deny outcome; approving actors are undefined. |
| Workflow Engine | Audit Infrastructure | Canonical version events and migration decision evidence | Immutable correlated change history. |

L2-A05 defines runtime initiation/resumption/cancellation contracts but no canonical commands for creating, changing, validating, approving, publishing or activating a version, or for requesting and authorizing migration.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Structural and orchestration changes, publication, activation and migration are protected lifecycle actions.
- Authentication identifies an actor but does not authorize a change.
- Authorization & Policy supplies decisions; Workflow Engine owns Workflow Version and instance-version lifecycle.
- The Workflow-Owning Domain must approve the business semantics it owns.

## 15.2 Approval and Separation of Duty

- Governance approval is required before publication.
- Migration requires Governance Approval and Administrator Authorization in addition to a Migration Plan and Compatibility Validation.
- L2-A05 does not name these actors or define a universal scenario-specific SoD rule.
- No approver, administrator, override authority or separation-of-duty constraint is invented.

## 15.3 Audit Obligations

The following must be auditable: Tenant; workflow and predecessor/successor version identities; change rationale and owning domain; changed contracts and references; validation results; governance approval; publication and activation; active-instance inventory; migration plan and compatibility result; administrator authorization; each migration or retention decision; canonical events; actors, timestamps and correlation.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- the change requirement and Workflow-Owning Domain were established;
- the prior published version remained immutable;
- a separate successor Draft was created;
- every mandatory validation succeeded;
- governance approval was granted;
- the successor was published as an immutable Workflow Version;
- `WorkflowVersionPublished` was published by Workflow Engine;
- the successor was activated for new instances;
- `WorkflowVersionActivated` was published by Workflow Engine;
- every existing instance retained its originating version by default;
- any migration followed all mandatory safeguards, and unsafe or unauthorized migration did not proceed; and
- change and migration evidence was preserved.

These criteria cannot yet be validated as a complete canonical outcome because change approval, publication, activation and migration authorities and their command contracts are not fully defined.

Postconditions:

- The activated successor is available for new executions.
- Every published version remains immutable.
- Existing instances continue on their originating version unless explicitly governed migration succeeded.
- No automatic migration or business-ownership transfer occurred.
- Retirement of the predecessor is a separate lifecycle decision.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A017-BLOCKER-01:** L2-A05 records Actors as absent. It requires governance approval and Administrator Authorization for migration but identifies no change requester, governance approver, publishing authority, activation authority or canonical administrator. | Required authority in Sections 6, 9 and 15 cannot be allocated without invention. | Define canonical actors and their approval, activation and migration authority boundaries. |
| **A017-BLOCKER-02:** L2-A05 defines runtime commands but no canonical definition/version or migration commands. | The state paths and safeguards exist, but the authoritative change, approval, publication, activation and migration contracts are missing. | Define canonical commands, inputs, preconditions and outcomes for Workflow Version change and migration. |
| **A017-BLOCKER-03:** L2-A05 defines no canonical migration event or migration-specific lifecycle state. | A governed migration can be described only as safeguards and an auditable relationship change; its exact event-driven completion contract is unavailable. | Define whether migration is an owned lifecycle, its states, canonical event(s) and sole publisher. |
| **A017-ISSUE-01:** The definition lifecycle includes `Draft`, `Validation`, `Approved`, `Published`, `Activated` and `Retired`, while Workflow Metadata lists only `Draft`, `Published` and `Retired`. | Version-state vocabulary is internally inconsistent, although the transition path is otherwise explicit. | Align Workflow Metadata with the canonical definition/version lifecycle. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
