# Event & Messaging Architecture Baseline

## 1. Specification Metadata
**Specification ID:** `L5-C-02-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-02 — Event & Messaging Architecture`  
**Canonical Technical Responsibility:** Event/messaging infrastructure while canonical business-event ownership remains upstream.  
**Technical Architecture Owner:** `L5-C-02 — Event & Messaging Architecture`  
**Assigned Author:** Pallav  
**Version:** `1.2.0`  
**Status:** `Architecture Approved`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-B-02-003`; `L5-C-01`; `ADR-L5-0007`; `ADR-L5-0010`  
**Referenced By:** `L5-C-02-002`; `L5-C-02-003`; downstream event-consuming L5 specifications  
**Applicable ADRs:** `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0022`; `ADR-L5-0035`.
**Governing Sources:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology; L5 ADR Register.

## 2. Purpose, Scope & Non-Goals
Defines the reusable asynchronous messaging architecture through which already-committed domain facts are published and independently delivered to authorized consumers. It owns publication, routing, subscription and consumer-delivery boundaries; it does not own business-event semantics, business rules, publisher authority, owner-local transaction consistency, or consumer-domain business decisions.

In scope: publication boundary, transport/routing abstraction, durable subscriptions, event-envelope information categories, Tenant/Institution context propagation, fan-out, consumer isolation, and baseline QoS classes. Out of scope: B02-owned atomic state/publication-intent persistence, detailed retry/replay/order mechanics (`L5-C-02-002`), schema evolution (`L5-C-02-003`), concrete broker/protocol selection, and numeric service targets.

## 3. Upstream Applicability, Constraints & Assumptions
The locked architecture baseline requires hybrid communication: commands/queries use synchronous contracts while completed domain facts use domain events. Critical workflows may not depend solely on ephemeral in-process eventing. Canonical business-event ownership remains with the source domain.

### 3.1 Authoritative L4 Traceability
For this production validation/remediation cycle, canonical L4 specifications are treated as behavioral authority regardless of repository metadata status. The relations below are therefore **Validated architectural realization mappings**, not planning-only claims.

| L4 BH | C02-001 technical contribution | Relation state |
|---|---|---|
| `L4-A-018-BH-002/003/005/006/007/008` | Consumes active canonical event definitions/contracts without taking semantic ownership. | Validated |
| `L4-A-019-BH-001/002/004/005/006/007/008/010` | Publication, routing, eligible-subscription resolution, independent delivery and acknowledgement boundary. | Validated |
| `L4-A-020-BH-001/002/003` | Provides the consumer-isolation baseline used by C02-002 recovery. | Validated |

### 3.2 Authoritative Constraints
- Events represent already-occurred facts; remote commands shall not be disguised as events.
- Canonical publisher ownership is never transferred to broker, relay, registry or consumer.
- Tenant-scoped event access is authorization-enforced; Institution context remains distinct where applicable.
- One consumer failure shall not roll back a committed publisher fact or block unrelated consumers.
- Critical event delivery requires durable capability.
- Shared delivery/subscription/ordering semantics are governed by `ADR-L5-0010`.

## 4. Technical Ownership, Boundaries & Dependencies
**Owns:** Event Publication Boundary, Event Transport/Routing Boundary, Durable Subscription abstraction, fan-out/consumer isolation semantics, baseline envelope/context information categories.  
**Consumes:** B02-003/`ADR-L5-0007` durable publication intent; A04 Tenant/Institution isolation; D01/D02/D03 identity/authorization/trust; E05 operational telemetry.  
**Collaborates:** C01 synchronous contracts; C02-002 durability/recovery; C02-003 schema/versioning; E01 resilience.  
**Must not own:** business facts/events, source-domain lifecycle, owner-local transaction, consumer-domain state or audit truth.

References to D02/D04/E05 are consumed peer capabilities only. This specification does not invent their internal implementation before those peer architectures are stable.

## 5. Solution Architecture
### 5.1 Logical Elements
| Element | Responsibility | Handoff / Failure Boundary |
|---|---|---|
| Publication Dispatcher | Reads/accepts committed publication intent and forms a transportable event instance from the active event contract. | Begins after B02 durable handoff; never fabricates business fact. |
| Event Envelope | Carries transport-neutral information categories including event identity, contract type/version, source, occurrence/publication context, Tenant scope, and correlation/causation where applicable. | Canonical wire-field names or serialization are not fixed here; invalid envelope information is rejected/quarantined by C02 policy. |
| Routing Fabric | Resolves authorized topic/subscription routing. | Transport failure preserves durable work for critical class. |
| Durable Subscription | Independently retains eligible critical events for a consumer. | One consumer backlog/failure is isolated. |
| Consumer Delivery Adapter | Delivers event and obtains governed acknowledgement. | Consumer failure does not alter publisher fact. |
| Telemetry Hook | Emits publish/delivery/lag/failure evidence to E05. | Telemetry loss does not become business failure. |

### 5.2 Primary Flow
```text
Committed owner-local state + Durable Publication Intent (B02 / ADR-L5-0007)
→ Publication Dispatcher
→ validate active event contract + scope + publisher authorization
→ immutable Event Instance
→ Routing Fabric
→ independent Durable Subscriptions
→ authorized consumer delivery
→ acknowledgement / C02-002 failure path
→ E05 telemetry
```

### 5.3 Baseline Semantics
Critical delivery is designed for at-least-once-compatible processing, requiring idempotent consumers. Producers do not know concrete consumers. Published Event Instances are immutable. Ordering is not global and is introduced only for explicitly defined ordering scopes. Lower-QoS signaling may exist only where loss is explicitly acceptable and cannot satisfy critical ERP consistency requirements. These shared semantics are governed by `ADR-L5-0010`.

## 6. Architecture & Technology Decisions
| ID | Decision | Position | Rationale / Authority |
|---|---|---|---|
| AD-001 | Communication split | Commands/queries synchronous; domain facts asynchronous. | Locked repository baseline. |
| AD-002 | Messaging topology | Topic-oriented pub/sub with independent consumer subscriptions. | `ADR-L5-0010`. |
| AD-003 | Critical QoS | Durable, at-least-once-compatible delivery; idempotent consumers. | `ADR-L5-0010` + `ADR-L5-0007` handoff. |
| AD-004 | Ordering | Scoped only; no global ordering. | `ADR-L5-0010`. |
| AD-005 | Envelope portability | CloudEvents 1.0-compatible envelope with JSON payload and governed JSON Schema; SutraERP-defined JSON members use camelCase. | `ADR-L5-0035` |
| AD-006 | Product selection | NATS JetStream is the canonical broker/transport product. | `ADR-L5-0022` |

## 7. Security, Privacy, Quality & Operations
Use service identities and least-privilege publish/subscribe authorization. Transport protection is mandatory through D03-owned controls. Sensitive payload content is minimized. Production/non-production messaging is logically isolated. Capacity considerations include publish rate, consumer lag, backlog age, connection pressure and recovery throughput; numeric targets are not invented.

## 8. Architecture Conformance Requirements
- `CR-001`: An event shall represent an already-occurred publisher-owned fact.
- `CR-002`: Critical flows shall not rely solely on ephemeral delivery.
- `CR-003`: Event infrastructure shall not become canonical publisher/owner of upstream facts.
- `CR-004`: Consumer failure shall not roll back an already-committed publisher fact.
- `CR-005`: Tenant event access shall be authorization-enforced, not naming-convention-only.
- `CR-006`: Broker/vendor/protocol is non-canonical until governed selection.
- `CR-007`: Shared durable delivery/subscription/ordering behavior shall conform to `ADR-L5-0010`.
- `CR-008`: Envelope content defined here shall remain conceptual information categories unless a separately governed contract standard fixes wire representation.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking state |
|---|---|---|
| OD-001 | **Resolved:** NATS JetStream transport + JSON/JSON Schema + CloudEvents-compatible envelope. Subject naming remains C02-governed technical routing. | `ADR-L5-0022`; `ADR-L5-0035` |
| OD-002 | Numeric capacity/retention targets | Deferred to E02/approved operational profiles. |

No unresolved L4-authority blocker remains for this validation cycle because canonical L4 behavior is being treated as authoritative by explicit review instruction.

## 10. Traceability Summary
Canonical responsibility: approved L5 Candidate Corpus Register 105. Repository constraints: Locked hybrid communication baseline and domain-ownership rules. Validated BHs: L4-A-018, A-019, A-020. Cross-L5: B02/`ADR-L5-0007`, `ADR-L5-0010`, C01, C02-002/003, A04, D01/D02/D03, E01, E05.

## 11. Review Position
This specification is **Architecture Approved** following completed Stage-2 remediation and explicit architecture approval on 2026-09-12. It is lock-ready but shall not be marked `Locked` until distinct final-governance authorization is recorded.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0022`  
NATS JetStream is canonical while ADR-L5-0007/0010/0011 delivery, ownership and compatibility semantics remain authoritative.


## 2026-09-22 Backend Readiness Group 1 Synchronization

**Applicable locked decision:** `ADR-L5-0035`  
Canonical business-event wire representation is JSON governed by JSON Schema with a CloudEvents 1.0-compatible envelope. NATS subjects are routing identifiers only, not canonical business-event identity.
