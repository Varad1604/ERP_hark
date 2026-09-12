# Event & Messaging Architecture Baseline

## 1. Specification Metadata
**Specification ID:** `L5-C-02-001`  
**Architecture Level:** L5 — Solution Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-12`  
**Governing Sources:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology; L5 ADR Register.  
**Canonical Technical Responsibility:** Event/messaging infrastructure while canonical business-event ownership remains upstream.  
**Applicable ADRs:** `ADR-L5-0007` constrains the B02→C02 handoff; no broker/product ADR is selected.

## 2. Purpose, Scope & Non-Goals
Defines the reusable asynchronous messaging architecture through which already-committed domain facts are published and independently delivered to authorized consumers. It owns publication, routing, subscription and consumer-delivery boundaries; it does not own business-event semantics, business rules, publisher authority, owner-local transaction consistency, or consumer-domain business decisions.

In scope: publication boundary, transport/routing abstraction, durable subscriptions, event envelope categories, Tenant/Institution context propagation, fan-out, consumer isolation, and baseline QoS classes. Out of scope: B02-owned atomic state/publication-intent persistence, detailed retry/replay/order mechanics (`L5-C-02-002`), schema evolution (`L5-C-02-003`), concrete broker/protocol selection, and numeric service targets.

## 3. Upstream Applicability, Constraints & Assumptions
The locked architecture baseline requires hybrid communication: commands/queries use synchronous contracts while completed domain facts use domain events. Critical workflows may not depend solely on ephemeral in-process eventing. Canonical business-event ownership remains with the source domain.

### 3.1 L4 Planning Traceability
The current `main` L4 sources below are `Draft`; therefore these relations are **Specified / planning-only**, not `Validated`.

| L4 BH | C02-001 technical contribution | Relation state |
|---|---|---|
| `L4-A-018-BH-002/003/005/006/007/008` | Consumes active canonical event definitions/contracts without taking semantic ownership. | Specified |
| `L4-A-019-BH-001/002/004/005/006/007/008/010` | Publication, routing, eligible-subscription resolution, independent delivery and acknowledgement boundary. | Specified |
| `L4-A-020-BH-001/002/003` | Provides the consumer-isolation baseline used by C02-002 recovery. | Specified |

The approved 105-specification L5 corpus boundary independently establishes this specification's canonical technical responsibility; Draft L4 is not treated as immutable authority.

### 3.2 Authoritative Constraints
- Events represent already-occurred facts; remote commands shall not be disguised as events.
- Canonical publisher ownership is never transferred to broker, relay, registry or consumer.
- Tenant-scoped event access is authorization-enforced; Institution context remains distinct where applicable.
- One consumer failure shall not roll back a committed publisher fact or block unrelated consumers.
- Critical event delivery requires durable capability.

## 4. Technical Ownership, Boundaries & Dependencies
**Owns:** Event Publication Boundary, Event Transport/Routing Boundary, Durable Subscription abstraction, fan-out/consumer isolation semantics, baseline envelope/context categories.  
**Consumes:** B02-003/ADR-L5-0007 durable publication intent; A04 Tenant/Institution isolation; D01/D02/D03 identity/authorization/trust; E05 operational telemetry.  
**Collaborates:** C01 synchronous contracts; C02-002 durability/recovery; C02-003 schema/versioning; E01 resilience.  
**Must not own:** business facts/events, source-domain lifecycle, owner-local transaction, consumer-domain state or audit truth.

## 5. Solution Architecture
### 5.1 Logical Elements
| Element | Responsibility | Handoff / Failure Boundary |
|---|---|---|
| Publication Dispatcher | Reads/accepts committed publication intent and forms a transportable event instance from the active event contract. | Begins after B02 durable handoff; never fabricates business fact. |
| Event Envelope | Carries event ID, type/version, source, occurrence/publication context, Tenant scope, and correlation/causation where applicable. | Invalid envelope is rejected/quarantined by C02 policy. |
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
Critical delivery is designed for at-least-once-compatible processing, requiring idempotent consumers. Producers do not know concrete consumers. Published Event Instances are immutable. Ordering is not global and is introduced only for explicitly defined ordering scopes. Lower-QoS signaling may exist only where loss is explicitly acceptable and cannot satisfy critical ERP consistency requirements.

## 6. Architecture & Technology Decisions
| ID | Decision | Position | Rationale / Authority |
|---|---|---|---|
| AD-001 | Communication split | Commands/queries synchronous; domain facts asynchronous. | Locked repository baseline. |
| AD-002 | Messaging topology | Topic-oriented pub/sub with independent consumer subscriptions. | Decoupling and optional-module degradation. |
| AD-003 | Critical QoS | Durable, at-least-once-compatible delivery; idempotent consumers. | Critical ERP consistency + ADR-L5-0007 handoff. |
| AD-004 | Ordering | Scoped only; no global ordering. | Scalability and correctness separation. |
| AD-005 | Envelope portability | Prefer open, transport-neutral envelope concepts; product-neutral. | Avoid vendor lock-in. |
| AD-006 | Product selection | Broker/protocol/product remains unselected. | Requires ADR if made canonical. |

## 7. Security, Privacy, Quality & Operations
Use service identities and least-privilege publish/subscribe authorization. Transport protection is mandatory through D03-owned controls. Sensitive payload content is minimized. Production/non-production messaging is logically isolated. Capacity considerations include publish rate, consumer lag, backlog age, connection pressure and recovery throughput; numeric targets are not invented.

## 8. Architecture Conformance Requirements
- `CR-001`: An event shall represent an already-occurred publisher-owned fact.
- `CR-002`: Critical flows shall not rely solely on ephemeral delivery.
- `CR-003`: Event infrastructure shall not become canonical publisher/owner of upstream facts.
- `CR-004`: Consumer failure shall not roll back an already-committed publisher fact.
- `CR-005`: Tenant event access shall be authorization-enforced, not naming-convention-only.
- `CR-006`: Broker/vendor/protocol is non-canonical until governed selection.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking state |
|---|---|---|
| OD-001 | Canonical broker/transport technology and protocol profile | Does not block Review; blocks any claim of product lock. |
| OD-002 | Numeric capacity/retention targets | Deferred to E02/approved operational profiles. |
| OD-003 | `L4-A-018/A-019/A-020` remain Draft | **Blocks Architecture Approved unless L4 is approved or explicit canonical Draft-L4 authority is recorded.** |

## 10. Traceability Summary
Canonical responsibility: approved L5 Candidate Corpus Register 105. Repository constraints: Locked hybrid communication baseline and domain-ownership rules. Planning BHs: L4-A-018, A-019, A-020. Cross-L5: B02/ADR-L5-0007, C01, C02-002/003, A04, D01/D02/D03, E01, E05.
