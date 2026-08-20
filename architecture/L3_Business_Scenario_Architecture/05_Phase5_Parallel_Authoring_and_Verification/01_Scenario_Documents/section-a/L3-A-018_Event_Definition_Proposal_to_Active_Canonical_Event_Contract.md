# Event Definition Proposal to Active Canonical Event Contract

# 1. Scenario Metadata

**Scenario ID:** L3-A-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Event Infrastructure  
**Participating Domains:** A, VAR:Event-Owning Domain  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Domains publishing or consuming canonical enterprise events; L3-A-019 — Committed Business Fact to Reliable Consumer Delivery

---

# 2. Scenario Purpose

Govern a proposed completed business fact as a discoverable canonical Event Definition, Schema, Version and Contract while preserving the proposing domain's exclusive ownership of the fact and its semantics.

---

# 3. Business Outcome

The event definition, ownership, schema, version and contract are reviewed, approved, registered and made available for canonical publication without transferring ownership of the business fact.

The locked L2 architecture defines the required aggregates, lifecycle, governance roles and activation controls. This Draft is classified `PASS WITH ISSUES` because it does not define canonical commands or lifecycle event names and uses inconsistent wording for the review state.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a platform or business domain identifies a completed fact that requires governed enterprise publication.

## 4.2 End Boundary

The scenario ends when one Event Definition and initial Event Version are `Active`, their approved immutable schema and contract are registered and discoverable, publication eligibility is established, and governance history is traceable.

## 4.3 In Scope

- Business meaning and single owning-domain confirmation.
- Canonical naming, classification and duplicate assessment.
- Event Schema, metadata, compatibility and Event Contract definition.
- Architectural, business, security, Tenant and compatibility review.
- Approval, authoritative registration and activation.
- Permanent governance and lifecycle traceability.

## 4.4 Out of Scope

- Creating the underlying business fact.
- Transferring business ownership to Event Infrastructure.
- Publishing an Event Instance or delivering it to consumers.
- Registering or approving consumer subscriptions.
- Technical payload formats, protocols, APIs, messaging products, storage or deployment design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-A-019 — Committed Business Fact to Reliable Consumer Delivery | Uses the active canonical Event Contract for publication and delivery. |
| Related | L3-A-021 — Historical Event Replay Request to Controlled Redelivery Completion | Historical instances retain their original definition and version. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Event Infrastructure / Section A | Primary owner. Owns Event Definition, Event Schema, Event Version, Event Registry stewardship, validation, lifecycle governance and activation. |
| VAR:Event-Owning Domain | Owns the completed business fact, its business meaning and the event semantics; supplies and approves domain-owned content. |
| Platform & Tenant / Section A | Supplies Tenant boundaries and platform configuration. |
| Person & Identity / Section A | Supplies canonical actor references. |
| Authentication / Section A | Establishes authenticated context for protected governance actions. |
| Authorization & Policy / Section A | Supplies authorization and governance-policy decisions. |
| Audit Infrastructure / Section A | Owns immutable evidence generated from governance activity. |

The frozen Participating Domains value remains exactly `A, VAR:Event-Owning Domain`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Domain Owner | Owns the Event Definition's business semantics and confirms the single authoritative domain owner. | Cannot activate an unreviewed definition or transfer fact ownership to Event Infrastructure. |
| Platform Architect | Defines event standards and evaluates architectural compliance. | Does not own the business fact or its domain decision. |
| Platform Administrator | Performs governed Event Infrastructure operational actions. | Activation remains authenticated, authorized, auditable and policy governed. |
| Security Administrator | Oversees applicable security and privileged-operation controls. | Does not approve business semantics. |

L2-A06 does not map a single named actor to every transition; this is recorded in Section 17 without inventing additional roles.

---

# 7. Preconditions and Entry Criteria

- The proposed event represents a completed business fact rather than a command, intention or incomplete work.
- Exactly one authoritative Event-Owning Domain is identified.
- The business meaning, proposed canonical name, classification and ownership information are available.
- The proposed Event Schema, mandatory metadata, compatibility policy and security classification can be defined.
- Applicable architectural, business, Tenant, security and authorization policies can be evaluated.

---

# 8. Trigger

A platform or business domain identifies a completed fact that requires governed enterprise publication.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Submit the completed-fact proposal and its business meaning for canonical event governance. | Domain Owner; VAR:Event-Owning Domain | A proposed Event Definition enters `Draft`. |
| 2 | Confirm that the proposal is a completed fact, has one owning domain and does not duplicate an existing canonical definition. | Event Registry Service; Platform Architect; Domain Owner | The proposal is eligible for detailed definition. |
| 3 | Establish the canonical name, classification, ownership, description and publication eligibility rules. | Event Infrastructure; Domain Owner | Event Definition semantics and ownership are explicit. |
| 4 | Define the initial Event Schema, mandatory metadata, validation rules, security classification and compatibility policy. | Event Infrastructure; VAR:Event-Owning Domain supplies business structure | The structural contract is available for review. |
| 5 | Define the initial Event Version and Event Contract linking the definition, schema, semantics, ownership and lifecycle. | Event Registry Service | A complete candidate canonical contract exists. |
| 6 | Move the proposal to the review lifecycle and evaluate naming, schema, ownership, domain boundary, compatibility, security, Tenant isolation and governance requirements. | Platform Architect; Security Administrator; Event Infrastructure; Domain Owner | Required review outcomes are established. |
| 7 | Complete governance approval, lock the approved schema and initial version, and establish the authoritative registry entry. | Event Infrastructure; authorized governance roles | The definition reaches `Approved`. |
| 8 | Activate the approved Event Definition and initial Event Version through a governed operational action. | Platform Administrator; Event Registry Service | The definition and default version reach `Active`. |
| 9 | Make the active Event Contract discoverable to authorized publishers and consumers. | Event Registry Service | Canonical publication eligibility is available. |
| 10 | Preserve ownership, review, approval, registration, activation and lifecycle history. | Event Infrastructure; Audit Infrastructure | The outcome remains permanently traceable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Existing definition satisfies the need | A semantically equivalent active definition already exists. | Reject the duplicate proposal and direct the domain to the governed existing contract. | No new definition is created. | Event Registry Service; Domain Owner |
| Review requires correction | Naming, schema, compatibility, security or ownership validation identifies a correctable deficiency. | Return the proposal to editable preparation, correct it and repeat the required review. | Activation waits until every review condition succeeds. | Event Infrastructure; VAR:Event-Owning Domain |
| Additional compatible version needed | The business meaning is stable but a governed structural evolution is required. | Create and review a new immutable Event Version and Schema under the existing definition. | The definition retains one canonical meaning and governed version history. | Event Infrastructure; Domain Owner |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Proposal is not a completed business fact. | It is ineligible to become a canonical event. | Reject it without registration or activation. | Event Infrastructure; VAR:Event-Owning Domain |
| Owning domain is absent or disputed. | Canonical ownership cannot be established. | Stop review until exactly one owner is governed. | Domain Owner; Platform Architect |
| Canonical name duplicates an existing definition. | Registry uniqueness would be violated. | Use the existing definition or submit a materially distinct business meaning. | Event Registry Service |
| Schema or mandatory metadata validation fails. | The contract cannot become publishable. | Correct the Draft and repeat review; do not activate. | Event Infrastructure; VAR:Event-Owning Domain |
| Security, Tenant or compatibility review fails. | Enterprise publication would be unsafe or inconsistent. | Retain the definition outside `Approved` and resolve the identified governance deficiency. | Event Infrastructure; Security Administrator |
| Governance approval is denied or incomplete. | The definition cannot become authoritative or active. | Do not activate or expose it for publication. | Event Infrastructure |
| Activation is unauthorized. | The approved definition remains unavailable for runtime publication. | Retain `Approved` until an authorized governed action succeeds. | Platform Administrator; Authorization & Policy |

No technical failure or recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Events represent completed business facts, never commands or intentions. | VAR:Event-Owning Domain; Event Infrastructure | Ineligible proposals are rejected. | L2-A06 EIP-01 |
| Business domains own business facts and semantics. | VAR:Event-Owning Domain | Registration does not transfer ownership. | L2-A06 EIP-02; EG-01 |
| Every event has exactly one authoritative owning domain. | Event Infrastructure | Shared ownership is prohibited. | L2-A06 ER-05; EG-01 |
| Registration and an approved contract precede publication. | Event Infrastructure | A Draft or unregistered definition cannot be published. | L2-A06 ER-02; EC-01 |
| Canonical names are globally unique. | Event Infrastructure | Duplicate definitions are prohibited. | L2-A06 Event Definition invariants |
| Only approved definitions and versions may become active. | Event Infrastructure | Review and governance approval cannot be bypassed. | L2-A06 AGP-05; VM-04 |
| Activated schemas and published versions are immutable. | Event Infrastructure | Structural change requires a governed new version. | L2-A06 SM-05; VM-01 |
| Only one default active version exists unless explicitly governed otherwise. | Event Infrastructure | Publication resolves predictably. | L2-A06 Event Version invariants; VM-05 |
| Retired definitions never return to `Active`. | Event Infrastructure | Canonical identity cannot be reused. | L2-A06 Event Definition invariants |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Event Definition | Does not exist | Create, review, approve, register and activate. | `Draft → Review → Approved → Active` | Event Registry Service |
| Event Schema | Proposed initial structure | Validate, approve and associate with the initial version. | `Active` and immutable after activation. | Event Registry Service |
| Event Version | Does not exist | Create, review, approve and activate initial version. | `Draft → Review → Approved → Active` | Event Registry Service |
| Event Contract | Candidate agreement | Approve and make discoverable with the active definition/version. | Active canonical contract. | Event Infrastructure |
| Business fact | Domain-owned semantic concept | Referenced and described; no fact instance is created. | Ownership unchanged. | VAR:Event-Owning Domain |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

L2-A06 defines Event Definition and Version lifecycle states but does not define exact canonical business-event names for definition creation, approval, registration or activation. No lifecycle event is invented in this Draft.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Contract / Hand-off | Business Information / Outcome |
|---|---|---|---|
| VAR:Event-Owning Domain | Event Infrastructure | Completed-fact proposal | Business meaning, ownership, classification and proposed content. |
| Event Registry Service | Governance participants | Definition, Schema, Version and Contract review package | Naming, structural, compatibility, security and Tenant evidence. |
| Authorization & Policy | Event Infrastructure | Authorization/governance decision | Governed allow or deny outcome for protected lifecycle actions. |
| Event Registry Service | Authorized publishers and consumers | Active canonical Event Contract | Definition identity, owner, version, schema, metadata, compatibility, security and lifecycle status. |
| Event Infrastructure | Audit Infrastructure | Governance and lifecycle evidence | Traceable review, approval, registration and activation history. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Proposal review, registration and activation are protected governance actions.
- Every publisher, consumer and administrative user must be authenticated.
- Authorization & Policy supplies decisions; Event Infrastructure owns lifecycle transitions.
- Least privilege and Tenant protection apply throughout.

## 15.2 Approval and Separation of Duty

- Architectural and business review is mandatory before activation.
- Domain Owner confirms business semantics; Platform Architect assesses architectural compliance.
- L2-A06 defines no scenario-specific separation-of-duty or override rule, so none is introduced.

## 15.3 Audit Obligations

The canonical name, owning domain, business meaning, classification, schema, version, compatibility, security classification, review results, approval information, registry identity, activation action, actors and lifecycle history must remain traceable.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when:

- the proposal is confirmed as a completed business fact;
- exactly one Event-Owning Domain retains ownership;
- canonical naming and duplicate checks succeed;
- the initial Schema, Version and Event Contract are complete;
- architectural, business, security, Tenant and compatibility review succeeds;
- governance approval is recorded;
- the authoritative registry entry exists;
- the Event Definition and default Event Version are `Active`;
- the active contract is discoverable for authorized publication and subscription; and
- governance and lifecycle history is preserved.

Postconditions:

- The underlying business fact remains owned by the Event-Owning Domain.
- No Event Instance has been published.
- The active schema/version cannot be modified in place.
- Future publication must use the registered active contract.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **A018-ISSUE-01:** L2-A06 alternates between `Under Review` in the aggregate state list and `Review` in the canonical definition/version lifecycle. | The intended review stage is clear, but its exact canonical label is inconsistent. | Select one lifecycle-state name and align all Event Definition, Schema and Version sections. |
| **A018-ISSUE-02:** L2-A06 defines lifecycle responsibilities and logical roles but no canonical definition/version commands or exact transition-to-role matrix. | L3 can establish the governed outcome, but L4 cannot derive exact command contracts and action authority without clarification. | Define lifecycle commands and explicitly map review, approval, registration and activation authority. |
| **A018-ISSUE-03:** L2-A06 defines no canonical events for definition/version lifecycle transitions. | No lifecycle event can be listed without invention; availability relies on the registry contract and audit history. | Define event names and publishers if these transitions require enterprise event publication. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
