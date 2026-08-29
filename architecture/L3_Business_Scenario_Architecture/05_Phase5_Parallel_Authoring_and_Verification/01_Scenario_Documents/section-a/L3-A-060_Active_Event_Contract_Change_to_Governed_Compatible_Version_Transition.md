# Active Event Contract Change to Governed Compatible Version Transition

# 1. Scenario Metadata

**Scenario ID:** L3-A-060  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Event Infrastructure  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L3-A-018; L3-A-059  
**Referenced By:** Event publishers, subscribers, replay and historical event investigation

---

# 2. Scenario Purpose

Govern compatible or breaking evolution of an Active Event Definition, Schema or Version through immutable successor creation, compatibility review, approval, activation, consumer migration and controlled predecessor deprecation or retirement.

---

# 3. Business Outcome

A governed compatible successor is approved and activated where applicable, consumer migration and compatibility are explicit, the prior version is deprecated or retired under policy, and historical event-version references remain immutable

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An active event definition, schema or version requires compatible evolution, breaking change, deprecation or retirement.

## 4.2 End Boundary

An approved successor is Active where required, every affected consumer has explicit compatibility/migration treatment, the predecessor is governed as Active-compatible, Deprecated or Retired, and historical instances retain original version references.

## 4.3 In Scope

- Change classification and ownership.
- Immutable successor Schema and Event Version.
- Naming, structure, security and compatibility validation.
- Architectural review and approval.
- Default-version activation and consumer migration planning.
- Predecessor deprecation or retirement.
- Historical references and governance evidence.

## 4.4 Out of Scope

- Rewriting published event instances.
- Consumer business-code implementation.
- Delivery retry, DLQ or replay execution.
- Broker, serialization library, API or deployment design.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Upstream | L3-A-018 — Event Definition Proposal to Active Canonical Event Contract | Supplies the Active predecessor. |
| Related | L3-A-059 — Consumer Subscription Request | Records supported versions and migration eligibility. |
| Related | L3-A-021 — Historical Event Replay | Replays immutable events under their original versions. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Event Infrastructure / Section A | Owns Event Definition, Schema, Version, registry, compatibility and lifecycle. |
| Authorization & Policy / Section A | Supplies authorization and governance approval decisions. |
| Audit Infrastructure / Section A | Preserves immutable version and decision history. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Domain Owner | Owns event business meaning and proposes semantic change. | Cannot mutate an Active schema in place. |
| Platform Architect | Reviews naming, boundaries, compatibility and architectural compliance. | Does not own business semantics. |
| Consumer Owner | Declares supported versions and migration progress. | Cannot change the canonical contract. |
| Platform Administrator | Operates approved activation/deprecation controls. | Cannot bypass governance. |

---

# 7. Preconditions and Entry Criteria

- The predecessor Event Definition/Version is Active and registered.
- Canonical ownership is known.
- Change rationale and proposed schema are available.
- Active consumers and their supported versions are discoverable.
- Compatibility, security, Tenant and governance policies are available.

---

# 8. Trigger

An active event definition, schema or version requires compatible evolution, breaking change, deprecation or retirement

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive and authorize the contract-change requirement. | Domain Owner; Event Infrastructure | Governed change begins. |
| 2 | Resolve predecessor definition, active/default version, schema, owner and consumers. | Event Registry Service | Complete impact scope is established. |
| 3 | Classify the change as backward, forward, fully compatible or breaking. | Platform Architect; Event Infrastructure | Required governance path is selected. |
| 4 | Create immutable successor Schema and Event Version in Draft. | Event Registry Service | Predecessor remains unchanged. |
| 5 | Validate structure, metadata, naming, ownership, security, Tenant isolation and compatibility. | Schema Validation; Platform Architect | Invalid successor cannot progress. |
| 6 | Define consumer migration, coexistence and retirement timing. | Consumer Owners; Event Infrastructure | Version transition is explicit. |
| 7 | Complete architectural review and governance approval. | Platform Architect; Authorization & Policy | Successor reaches Approved. |
| 8 | Activate the successor and designate the default version where applicable. | Event Infrastructure | Successor reaches Active and is publishable. |
| 9 | Maintain permitted predecessor support during migration. | Event Infrastructure | Compatible consumers continue safely. |
| 10 | Deprecate the predecessor with migration guidance, then retire it when policy criteria are met. | Event Infrastructure | Future publication is discouraged then prohibited. |
| 11 | Preserve all versions, schemas, decisions, subscriptions and original historical event-version references. | Event Infrastructure; Audit Infrastructure | Evolution remains traceable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome |
|---|---|---|---|
| Compatible additive change | Existing consumers remain safe. | Activate successor and permit governed coexistence. | Migration may be gradual. |
| Breaking change | Existing consumers are incompatible. | Require a new governed version and explicit migration before predecessor retirement. | No in-place schema mutation. |
| Deprecation only | No successor is required. | Communicate schedule and restrict new adoption. | Existing consumers remain supported during policy period. |
| Retirement blocked | Active consumers have not migrated. | Extend deprecation or apply approved exception. | Predecessor is not retired prematurely. |
| Validation rejected | Successor violates governance. | Return to revision. | Active predecessor remains current. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Compatibility is ambiguous. | Consumer safety cannot be assured. | Do not approve or activate. | Event Infrastructure |
| Successor schema fails validation. | Contract is not publishable. | Correct Draft successor. | Event Infrastructure |
| Consumer migration plan is incomplete. | Retirement could disrupt consumers. | Retain predecessor under governed support. | Consumer Owner; Event Infrastructure |
| Activation is interrupted. | Default version is uncertain. | Preserve predecessor and revalidate registry state; exact command recovery is undefined. | Event Infrastructure |
| Historical reference would change. | Audit and replay integrity would be violated. | Reject the operation. | Event Infrastructure; Audit Infrastructure |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| Activated schemas and versions are immutable. | Structural change requires a successor. | L2-A06 SM-05, VM-01 |
| Every version declares compatibility. | Migration and coexistence are explicit. | L2-A06 VM-03 |
| Only one default active version exists unless governed otherwise. | New publication resolves deterministically. | L2-A06 VM-05 |
| New definitions, versions and contracts undergo architectural review. | Approval precedes activation. | L2-A06 EG-02 |
| Deprecated versions remain supported during migration. | Deprecation does not erase history. | L2-A06 §22.6 |
| Retired definitions/versions remain discoverable and non-publishable. | Historical references remain valid. | L2-A06 §22.7, Event Version invariants |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Successor Event Version | Absent | `Draft → Review → Approved → Active`. | Active compatible/governed successor. |
| Successor Event Schema | Absent | Create, validate, approve and activate. | Immutable active schema. |
| Predecessor Event Version | Active | Remain supported, then `Active → Deprecated → Retired`. | Historical discoverable version. |
| Event Subscription | Active | Update declared supported versions through subscription governance. | Explicit compatibility/migration state. |
| Historical Event Instance | Published | Retain original version reference. | Unchanged and replayable. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Event Determination

L2-A06 defines lifecycle states and governance but no canonical EventVersionCreated, SchemaApproved, EventVersionActivated, EventVersionDeprecated or EventVersionRetired events and publishers. None is invented.

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Successor contract | Event Registry Service | Publishers and Consumer Owners | Schema, version and compatibility. |
| Consumer compatibility | Consumer Owner | Event Governance | Supported versions and migration readiness. |
| Governance decision | Platform Architect / Authorization & Policy | Event Infrastructure | Approval or return. |
| Active/default version | Event Registry Service | Publication Service | Publishable version. |
| Historical version resolution | Event Registry Service | Replay/Audit consumers | Original immutable contract. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

Contract creation, approval, activation, deprecation and retirement are privileged, Tenant-aware operations.

## 15.2 Approval and Separation of Duty

Architectural review precedes activation. L2-A06 names logical roles but does not define an exact transition-specific approval matrix.

## 15.3 Audit Obligations

Record owner, predecessor/successor, schemas, classification, compatibility evidence, consumers, migration plan, review, approval, lifecycle transitions, actor, timestamps and correlation.

---

# 16. Completion Criteria and Postconditions

- Change was classified and authorized.
- An immutable successor was created where needed.
- Schema, security, ownership and compatibility validation succeeded.
- Consumer migration and coexistence were explicit.
- Approved successor became Active/default where applicable.
- Predecessor was deprecated or retired only under policy.
- Historical event-version references remain unchanged.
- Governance history is auditable.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|
| A060-ISSUE-01 | Non-blocking | L2-A06 provides Definition, Schema and Version states but no owned commands, complete transition matrix or canonical lifecycle events/publishers. | Use source-defined states and governance roles; define command/event contracts before L4. |
| A060-ISSUE-02 | Non-blocking | Event Definition uses both `Under Review` and `Review` wording across aggregate/lifecycle sections. | Treat them as the same review position for this Draft; normalize the canonical state name before locking. |

No schema mutation, event, command or migration mechanism is invented.
