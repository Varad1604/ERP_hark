# Notification & Communication Architecture Baseline

## 1. Specification Metadata

**Specification ID:** `L5-C-04-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-04 — Notification & Communication Architecture  
**Canonical Technical Responsibility:** Enterprise notification/communication capability; triggering business facts remain source-domain owned.  
**Technical Architecture Owner:** L5-C-04 — Notification & Communication Architecture  
**Assigned Author:** Chetan  
**Version:** `1.2.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Stage 3 Final Closure & Stage 4 Proceed Decision 2026-09-15; `L2-H02_Notification_Workflows.md` v1.0 Locked; `L5-C-01-001`; `L5-C-02-001`; `L5-A-04-002`; `L5-B-02-003`; `L5-D-02-004`; `L5-A-08-001`; `L5-A-08-002`; `L5-D-04-001`; `L5-E-05-001`.  
**Referenced By:** `L5-C-04-002`; `L5-C-04-003`; business/platform capabilities requiring governed notification/communication.  
**Applicable ADRs:** `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0007`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0013`; `ADR-L5-0021`; `ADR-L5-0022`; `ADR-L5-0033`; `ADR-L5-0034`; `ADR-L5-0041`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the reusable L5 technical baseline through which an authoritative source-domain event or an authorized communication request becomes one governed H02 Notification without transferring ownership of the originating business fact, transaction, decision, lifecycle or event.

The baseline establishes the common C04 technical control plane for:

- admission of notification triggers through governed C01/C02 boundaries;
- stable Notification identity and source-causation binding;
- technical realization of the locked H02 Notification lifecycle;
- common orchestration among template/policy resolution, scheduling/background execution and delivery;
- owner-local persistence/concurrency and lifecycle-event publication handoff;
- notification evidence and operational-telemetry handoffs.

### In Scope

- notification-trigger admission and provenance preservation;
- duplicate/replay-safe admission within the Notification boundary;
- technical Notification identity and Tenant/Institution context binding;
- technical realization of the canonical H02 Notification lifecycle;
- common orchestration of C04-002 and C04-003 responsibilities;
- Notification scheduling/execution coordination while preserving H02 scheduling semantics;
- controlled use of A08 for generic delayed/background technical execution;
- owner-local Notification state mutation and concurrency boundary;
- atomic Notification-state to lifecycle-event-publication-intent handoff;
- handoff of audit evidence to D04 and operational telemetry to E05.

### Out of Scope / Non-Goals

This specification does not own:

- originating business facts, transactions, rules, decisions, lifecycles, events or publishers;
- template approval authority or Template business lifecycle semantics;
- template/preference/subscription/quiet-hour/emergency-policy execution internals — `L5-C-04-002`;
- multi-channel provider selection, provider adapters, provider failover, Delivery Attempt/receipt normalization or final delivery-evidence internals — `L5-C-04-003`;
- synchronous API/contract infrastructure — `L5-C-01`;
- enterprise event transport, delivery, ordering, replay or schema compatibility — `L5-C-02`;
- persistence topology, transaction engine or database product — `L5-B-02`;
- authorization policy ownership — `L5-D-02`;
- generic Job/Work Unit/Attempt execution — `L5-A-08`;
- enterprise audit truth/storage — `L5-D-04`;
- operational observability platform — `L5-E-05`;
- provider, broker, database, framework, runtime or cloud-product selection;
- L6 screens, navigation, forms or interaction design.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-C-04-001 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-C-04-001` row | Enterprise notification/communication capability while triggering business facts remain source-domain owned. | Establishes canonical responsibility and ownership boundary. |
| L5 Governance | `L5_Stage3_Final_Closure_and_Stage4_Proceed_Decision_2026-09-15.md` | v1.0.0 / Locked | §6 Stage-4 Authorization | C04 is authorized to proceed with minimum dependency `C01 + C02`. | Authorizes Stage-4 Draft authoring against the stabilized dependency surface. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§4–6, 10–13, 19, 25–33 | H02 owns Notification orchestration/lifecycle/scheduling/delivery coordination while business domains retain business ownership; defines canonical Notification states/events and provider/event/audit boundaries. | Preserves Notification identity/lifecycle/event semantics and technical ownership boundaries. |
| L2 | `L2-H02_Notification_Template_Approval_Amendment_2026-09-01.md` | v1.1.0 / Locked | §§1–3 | Notification Template Approval Authority owns Template approval/publication authority; no new Template event is introduced. | Prevents C04-001 from absorbing Template approval authority; C04-002 consumes this authority technically. |
| L4 | `L4-H-010` | v1.0.0 / Draft | `BH-001`, `BH-002`, `BH-003`, `BH-004`, `BH-005`, `BH-006`, `BH-007` | Governed admission from source event/request, one Notification identity/lifecycle, source-ownership preservation, H02 lifecycle events and traceable completion. | Drives baseline trigger admission, Notification coordination, source isolation, lifecycle-event handoff and evidence. Relation state: **Specified** only. |
| L4 | `L4-H-011` | v1.0.0 / Draft | `BH-001`, `BH-002`, `BH-003`, `BH-004`, `BH-005`, `BH-006`, `BH-007` | Scheduling must preserve Notification identity, expiry/cancellation semantics, recurring-instance identity and schedule evidence. | Drives schedule/execution coordination and A08 boundary without redefining H02 scheduling semantics. Relation state: **Specified** only. |
| L4 | `L4-H-014` | v1.0.0 / Draft | `BH-001`, `BH-004`, `BH-005`, `BH-006`, `BH-007` | Emergency/critical source facts remain source-owned; H02 owns only its Notification outcome/events/evidence. | Constrains admission, identity, source isolation and lifecycle-event/evidence handling; policy/channel internals remain C04-002/003. Relation state: **Specified** only. |
| L4 | `L4-H-015` | v1.0.0 / Draft | `BH-004`, `BH-005`, `BH-006` | Delivery recovery keeps the same Notification identity, never repeats/reverses source transaction and emits only defined H02 events. | Constrains baseline identity/source boundary and lifecycle-event publication after delivery recovery. Relation state: **Specified** only. |
| L4 | `L4-H-016` | v1.0.0 / Draft | `BH-005`, `BH-007` | Provider-derived delivery evidence may drive permitted H02 transitions/events and final H02 outcome without changing source-business ownership. | Defines C04-003-to-baseline outcome handoff and lifecycle-event publication boundary. Relation state: **Specified** only. |
| L5 Peer | `L5-C-01-001` | v1.3.0 / Architecture Approved | §§4–6; `CR-001`–`CR-005` | Ownership-preserving synchronous contract boundary with HTTPS REST/JSON + OpenAPI 3.2 and the Group-1 wire profile. | Governs authorized synchronous Notification-request admission; C04 consumes rather than redefines the API profile. |
| L5 Peer | `L5-C-02-001` | v1.0.3 / Architecture Approved | §§4–8; `AD-001`–`AD-006`; `CR-001`–`CR-008` | Canonical event/messaging transport, durable subscriptions and consumer delivery while event semantics/publisher remain upstream-owned. | Carries incoming canonical source events and outgoing H02 lifecycle events; C04 does not duplicate event transport. |
| L5 Peer | `L5-A-04-002` | v1.1.0 / Architecture Approved | §§2–4 | Tenant/Institution context propagation and fail-closed scope handling. | Governs scope binding across Notification admission, state, background execution and delivery handoffs. |
| L5 Peer | `L5-B-02-003` | v1.1.0 / Architecture Approved | §§5–8; `AD-001`–`AD-004`; `CR-001`–`CR-011` | Owner-local transactions, optimistic concurrency and atomic state + Durable Publication Intent handoff. | Governs Notification state mutation/concurrency and lifecycle-event publication intent. |
| L5 Peer | `L5-D-02-004` | v1.0.0 / Architecture Approved | §§2–4 | Technical policy decision/enforcement with fail-closed protected execution. | Supplies authorization decisions for protected Notification admission/operations without C04 owning authorization policy. |
| L5 Peer | `L5-A-08-001`, `L5-A-08-002` | v1.3.0 / Architecture Approved | execution/job/retry boundaries | Generic background, scheduled and technically retryable execution with durable Job/Work Unit/Attempt identities. | Executes deferred/background technical work while C04 retains Notification/schedule semantics. |
| L5 Peer | `L5-D-04-001` | v1.1.1 / Locked | §§2–4 | Shared audit/evidence capture and handoff while source capabilities retain business meaning. | Receives Notification evidence; C04 does not become enterprise audit-storage owner. |
| L5 Peer | `L5-E-05-001` | v1.3.0 / Architecture Approved | §§2–5 | Operational telemetry/observability distinct from business audit truth. | Receives operational Notification telemetry and diagnostics. |
| ADR | `ADR-L5-0004` | Architecture Approved | Tenant/Institution isolation pattern | Validated scope must survive technical boundaries. | Constrains Notification scope propagation. |
| ADR | `ADR-L5-0006` | Architecture Approved | Owner-local transaction/concurrency pattern | Owner-local state mutation with explicit concurrency handling. | Constrains Notification state updates. |
| ADR | `ADR-L5-0007` | Architecture Approved | Atomic state-to-publication-intent handoff | State and required publication intent commit atomically; C02 owns delivery afterward. | Governs H02 lifecycle-event handoff. |
| ADR | `ADR-L5-0010`, `ADR-L5-0011` | Architecture Approved | Event delivery/compatibility patterns | Durable event delivery and event-contract evolution remain C02-owned. | Governs incoming/outgoing event transport consumed by C04. |
| ADR | `ADR-L5-0013` | Architecture Approved | Background Job/Work Unit/Attempt pattern | Generic asynchronous/scheduled/retry execution remains A08-owned. | Governs C04 use of generic background execution without transferring Notification semantics. |

### 3.2 L4 Applicability

All L4-H relations in this specification are **Specified**, not `Validated`, because the current `L4-H-010` through `L4-H-016` source documents are `Draft`. This specification does not promote those relations.

| L4 BH | L5-C-04-001 Role | L5 Evidence Target | Relation State |
|---|---|---|---|
| `L4-H-010-BH-001` | Owns baseline admission coordination | §§5.1–5.3; `AD-001`; `CR-001` | Specified |
| `L4-H-010-BH-002` | Owns common lifecycle coordination; delegates C04-002/003 internals | §§5.1–5.5; `AD-002`; `CR-003`–`CR-006` | Specified |
| `L4-H-010-BH-003/004` | Owns terminal-state coordination/source isolation | §§5.4, 5.7; `CR-008` | Specified |
| `L4-H-010-BH-005` | Owns C04-side lifecycle-event publication handoff only | §5.8; `AD-004`; `CR-009` | Specified |
| `L4-H-010-BH-006/007` | Owns common Notification traceability/evidence handoff | §§5.2, 5.9; `CR-002`, `CR-011` | Specified |
| `L4-H-011-BH-001`–`BH-007` | Owns Notification schedule coordination; consumes A08 for generic execution | §§5.5–5.6; `AD-005`; `CR-006`, `CR-007` | Specified |
| `L4-H-014-BH-001/004/005/006/007` | Preserves source-emergency ownership while coordinating H02 outcome/events/evidence | §§5.2, 5.4, 5.8–5.9; `CR-002`, `CR-008`, `CR-009` | Specified |
| `L4-H-015-BH-004/005/006` | Preserves Notification identity/source transaction and event/evidence handoffs | §§5.2, 5.7–5.9; `CR-002`, `CR-008`, `CR-009` | Specified |
| `L4-H-016-BH-005/007` | Consumes C04-003 provider-independent outcome and completes permitted H02 transition/event | §§5.4, 5.8–5.9; `CR-009`, `CR-011` | Specified |

### 3.3 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Triggering business facts/events and completed source transactions remain source-domain owned. | Candidate Register `L5-C-04-001`; L2-H02 §§5.2, 6; `L4-H-010-BH-004/005` | C04 may communicate an outcome but cannot mutate, reverse or republish the source fact as its own. |
| C-002 | H02 owns the canonical Notification lifecycle and exact Notification lifecycle events. | L2-H02 §§11–12, 32 | L5 technical state/flow must use the upstream states/events without creating replacements. |
| C-003 | Business domains shall not invoke external notification providers directly. | L2-H02 §§4.2 NP-05, 6 | Provider/channel delivery remains behind the C04 family boundary and C04-003. |
| C-004 | Notification processing preserves Tenant isolation. | L2-H02 NP-07; `ADR-L5-0004`; `L5-A-04-002` | Scope is bound at admission and preserved across persistence, background execution and delivery. |
| C-005 | Notification retries/failover shall not repeat the originating business transaction or create a new Notification solely for recovery. | L2-H02 §6; `L4-H-015-BH-002/004` | Recovery is confined to communication execution under the same Notification identity. |
| C-006 | Notification lifecycle events are published only after the corresponding governed transition. | L2-H02 §32.3; applicable L4-H BHs; `ADR-L5-0007` | State change and publication intent are atomically coupled before C02 delivery. |

### 3.4 Assumptions

No material architectural assumption is treated as an approved fact in this specification. Unresolved source/dependency items are recorded in Section 9.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

`L5-C-04-001` owns the common technical baseline for:

- governed Notification trigger admission;
- technical Notification identity and provenance binding;
- technical realization/guarding of the upstream-defined H02 Notification lifecycle;
- common coordination across C04-002 and C04-003;
- Notification schedule/execution handoff semantics at the C04 boundary;
- owner-local Notification state/concurrency integration with B02;
- H02 Notification lifecycle-event publication-intent handoff to C02;
- common audit/telemetry handoff points.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-C-01-001` / C01 family | Governed synchronous contracts | C01 owns request/response transport and contract mechanics; C04 consumes an authorized request boundary. | Supports explicit communication requests without direct internal/provider invocation. |
| `L5-C-02-001` / C02 family | Canonical event/messaging transport | C02 owns event publication infrastructure, delivery, ordering/recovery and compatibility; C04 owns Notification processing after intake. | Supports event-triggered admission and H02 lifecycle-event delivery. |
| `L5-C-04-002` | Template, preference & communication-policy execution | C04-002 owns technical resolution/execution of approved Template/preference/policy semantics. | Produces governed resolution context required before delivery. |
| `L5-C-04-003` | Multi-channel delivery/provider abstraction/evidence | C04-003 owns channel/provider execution, failover and provider-independent delivery evidence. | Executes communication delivery while baseline retains Notification identity/lifecycle coordination. |
| `L5-A-04-002` | Tenant/Institution scope propagation | A04 owns scope semantics; C04 carries and enforces them at its boundaries. | Prevents cross-Tenant/Institution notification leakage. |
| `L5-B-02-003` | Transaction/concurrency/state-publication handoff | B02 owns transaction/concurrency mechanisms and atomic publication-intent pattern. | Makes Notification state transitions and lifecycle-event handoff consistency-safe. |
| `L5-D-02-004` | Authorization policy decision/enforcement | D02 owns authorization decisions; C04 consumes them. | Protects admission and privileged Notification operations. |
| `L5-A-08-001/002` | Background/scheduled/retryable technical execution | A08 owns generic Job/Work Unit/Attempt mechanics; C04 owns Notification scheduling meaning. | Runs deferred/background work without duplicating Notification lifecycle. |
| `L5-D-04-001` | Audit/evidence capture | D04 owns enterprise evidence capability. | Preserves attributable Notification evidence without C04 owning audit persistence. |
| `L5-E-05-001` | Operational telemetry | E05 owns telemetry/operations architecture. | Makes notification processing operationally observable without treating telemetry as audit truth. |

### Must Not Own

C04-001 shall not:

- become canonical owner/publisher of a source-domain business event;
- change the originating business outcome because communication succeeds or fails;
- invent a Notification state, Template state, business actor, approval authority or canonical event;
- directly call provider-specific interfaces or duplicate C04-003 provider abstraction;
- implement its own event broker/durable subscription/replay infrastructure;
- mutate another owner's persistence or extend transactions across owners;
- treat A08 Job/Work Unit/Attempt states as Notification lifecycle states;
- treat E05 telemetry as enterprise audit truth.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| Notification Trigger Admission Boundary | Control Point | Accept a canonical source event or authorized communication request and establish eligibility to create one H02 Notification. | C01 request or C02-delivered event; validated scope; authorization; source causation/correlation. | No canonical Notification state before admission succeeds. | Admitted trigger context or governed rejection. | C01, C02, A04, D02 | Invalid/unresolvable context fails before Notification creation; no business outcome is changed. | `L4-H-010-BH-001`; L2-H02 §§6, 9. |
| Notification Identity & Provenance Binder | Component | Create/stabilize one Notification identity and bind source, scope, causation/correlation and duplicate-suppression context. | Admitted trigger context. | H02 Notification identity and immutable provenance references. | Notification in upstream-defined initial `Draft` state. | B02, A04 | Duplicate/replayed admission shall not silently create duplicate canonical Notifications within the same governed identity scope. | L2-H02 §§11.2–11.5, 11.9; `L4-H-010-BH-006`. |
| Notification Lifecycle Coordinator | Component | Enforce only the canonical H02 Notification lifecycle and coordinate family handoffs. | Current Notification state; C04-002 resolution result; scheduling/execution outcome; C04-003 delivery outcome. | Canonical H02 Notification state only. | Next permitted state + evidence/event intent where applicable. | B02, C04-002, C04-003 | Invalid/stale transition or concurrency conflict does not overwrite current state. | L2-H02 §12; `L4-H-010-BH-002/003`. |
| Template/Recipient/Policy Resolution Handoff | Contract / Control Point | Delegate technical Template, recipient, preference/subscription, channel-policy and permitted-override resolution. | Notification identity/context + source-owned references. | No C04-001 ownership of child policy/template internals. | Resolved notification context or governed resolution failure. | C04-002 | Resolution failure does not cause direct provider delivery or source transaction mutation. | Candidate Register `L5-C-04-002`; L4-H-010/013/014. |
| Notification Schedule Coordinator | Component | Preserve H02 scheduling identity/eligibility, cancellation/expiry and recurring-instance rules. | Resolved Notification; schedule/window/expiry/recurrence context where applicable. | Canonical Notification/Schedule references; no A08 Job state as business state. | Immediate delivery eligibility or generic A08 execution request. | A08, B02 | Expired/cancelled Notification is not released for delivery; schedule conflict preserves identity/history. | L2-H02 §19; `L4-H-011-BH-001`–`BH-007`. |
| Background Execution Handoff | Contract / Adapter | Delegate generic deferred/background technical work to A08 while retaining Notification scheduling meaning. | Stable Notification reference, scope, release condition, cancellation/revocation reference, authority context. | A08 Job/Work Unit/Attempt state is technical and separate. | Technical execution status/evidence back to Schedule/Lifecycle Coordinator. | A08-001/002 | A08 failure cannot silently become H02 business/lifecycle failure; C04 evaluates only permitted H02 consequence. | `ADR-L5-0013`; Stage-3 closure decision. |
| Delivery Handoff | Contract / Control Point | Send a delivery-ready Notification to C04-003 and receive provider-independent delivery outcome/evidence. | Notification identity/scope; rendered content/reference; permitted channel context; priority/schedule/expiry; correlation. | C04-003-owned Delivery/Attempt/receipt evidence. | Delivery outcome/evidence to Lifecycle Coordinator. | C04-003 | Provider/channel failure stays inside delivery boundary and never alters source-domain business state. | Candidate Register `L5-C-04-003`; L4-H-014/015/016. |
| Notification State Repository Boundary | Store Boundary | Persist C04-owned technical realization of H02 Notification state using owner-local B02 rules. | Valid transition + expected current version/precondition. | Notification state/provenance/evidence references; physical persistence B02-owned. | Durable committed state or explicit conflict/failure. | B02-003 | No lost update; no cross-owner write; no remote acknowledgement inside local transaction. | `ADR-L5-0006`; `L5-B-02-003`. |
| Lifecycle Event Publication Handoff | Control Point | Couple a committed H02 lifecycle transition with its required Durable Publication Intent, then hand delivery to C02. | Transition result + exact upstream-defined H02 event identity/context. | Durable Publication Intent is technical state, not a new canonical event. | C02 delivery of the exact H02 event. | B02-003, C02 | C02 outage after commit does not roll back committed Notification state. | `ADR-L5-0007`; L2-H02 §32. |
| Evidence & Telemetry Handoff | Adapter | Supply Notification evidence to D04 and operational signals to E05. | Notification identity, source correlation, transition/outcome/failure references. | No D04/E05 ownership is duplicated. | D04 evidence reference; E05 telemetry. | D04-001, E05-001 | Telemetry loss does not rewrite Notification truth; mandatory evidence follows D04's own completion rules. | L2-H02 §§34–35; D04/E05 boundaries. |

### 5.2 Primary Technical Flow

```text
Canonical source-domain event
        via C02
            OR
Authorized communication request
        via C01
            ↓
Notification Trigger Admission Boundary
- validate Tenant/Institution scope
- consume authorization decision where required
- preserve source publisher / causation / correlation
- apply duplicate/replay admission protection
            ↓
Notification Identity & Provenance Binder
            ↓
one H02 Notification in canonical initial state
            ↓
C04-002 Template / Recipient / Preference / Policy Resolution
            ↓
Notification Lifecycle Coordinator
            ↓
Immediate eligibility
     OR
C04 Schedule Coordinator → A08 generic deferred/background execution
            ↓
C04-003 Multi-Channel Delivery / Provider Abstraction / Evidence
            ↓
provider-independent delivery outcome
            ↓
Notification Lifecycle Coordinator
            ↓
owner-local state + required lifecycle-event publication intent
        committed atomically through B02
            ↓
C02 lifecycle-event delivery
            ↓
D04 evidence handoff + E05 operational telemetry
```

The source business transaction is outside this flow's mutation boundary. Notification completion, cancellation, expiry or failure shall not roll back or reinterpret the originating business outcome.

### 5.3 Trigger Admission and Duplicate Protection

A trigger may enter C04 through exactly one governed interaction class:

1. **canonical source event** delivered by C02 while retaining its source-domain publisher and event meaning; or
2. **authorized communication request** admitted through C01.

The admission boundary shall establish enough technical context to determine whether a Notification may be created:

```text
NotificationIngressContext
{
  source trigger identity and interaction class
  source-owner / canonical-publisher reference
  validated Tenant / Institution scope
  authorization decision/reference where required
  causation / correlation context
  stable request/event duplicate-suppression reference where defined
  source-provided communication classification/request context where defined
  H02/C04-governed Notification category/priority context after applicable resolution
  schedule / expiry context where applicable
}
```

These are conceptual information categories, not wire-field names.

A duplicate/replayed trigger shall not silently create another canonical Notification when the same governed source/request identity is already represented. Duplicate suppression shall preserve historical evidence and shall not erase valid event history.

C04 shall not poll another owner's persistence to infer that a business fact occurred.

### 5.4 Notification Identity and Lifecycle Coordination

The technical realization shall preserve the upstream H02 Notification states exactly:

```text
Draft
→ Validated
→ RecipientResolved
→ TemplateResolved
→ Scheduled or Queued
→ Delivering
→ Delivered / PartiallyDelivered / Failed

Scheduled / Queued
→ Cancelled or Expired
```

Only transitions permitted by the locked H02 lifecycle may be represented. No local technical convenience state may be promoted into the canonical Notification lifecycle.

Key invariants:

- Notification identity is immutable after creation.
- Provider changes, retries and fallback do not change Notification identity.
- a recurring schedule creates independent Notification instances as defined upstream;
- terminal H02 states are not silently re-entered;
- source-domain state is never changed by a Notification transition;
- each transition is concurrency-checked against current durable Notification state.

C04-001 does not own the business meaning of H02 lifecycle states; it owns their technical realization and coordination within the C04 responsibility.

### 5.5 C04-002 Resolution Boundary

Before delivery, C04-001 delegates the following technical responsibilities to C04-002:

- approved Template/version resolution;
- recipient resolution/expansion;
- technical preference/subscription resolution;
- quiet-hour and communication-policy evaluation;
- allowed emergency/critical override execution;
- rendering/localization/variable-resolution controls assigned to the C04 family.

The returned conceptual context is:

```text
ResolvedNotificationContext
{
  Notification identity and validated scope
  approved Template/version reference
  resolved eligible recipient reference/set
  rendered-content reference or equivalent delivery-ready content
  permitted channel/policy context
  preference/subscription/quiet-hour decision references where applicable
  authorized emergency override reference where applicable
}
```

C04-001 shall not independently reproduce C04-002 decision logic. If required context cannot be resolved, delivery does not begin and only an upstream-defined H02 outcome/transition may be applied.

### 5.6 Scheduling and Background Execution

C04 retains the technical realization of H02 Notification scheduling semantics:

- schedule/delivery-window/expiry context remains bound to the Notification;
- cancellation before execution prevents later delivery;
- expiry before execution prevents later delivery;
- schedule modification preserves Notification identity and history;
- recurring schedules create independent Notification instances;
- H02 scheduling events are emitted only for their corresponding upstream-defined transitions.

Where implementation requires a deferred/background technical execution mechanism, C04 consumes A08:

```text
H02 Notification Schedule
        ↓
C04 schedule eligibility
        ↓
A08 generic Job / Work Unit / Attempt execution
        ↓
technical execution status/evidence
        ↓
C04 re-evaluates current Notification eligibility
        ↓
delivery handoff or permitted H02 terminal outcome
```

A08 `Job`, `Work Unit` and `Attempt` identities/states are technical execution constructs and shall never replace H02 Notification or Notification Schedule lifecycle semantics.

### 5.7 C04-003 Delivery Boundary

After resolution and execution eligibility are satisfied, C04-001 hands a delivery-ready Notification to C04-003.

Conceptually, the handoff includes only information necessary to preserve the boundary:

```text
DeliveryReadyNotification
{
  Notification identity
  Tenant / Institution scope
  recipient and rendered-content references
  permitted channel context
  priority / schedule / expiry context
  causation / correlation context
}
```

C04-003 returns a provider-independent result such as:

```text
DeliveryOutcome
{
  Notification identity
  channel outcome references
  Delivery Attempt / receipt evidence references
  failure classification where applicable
  aggregate delivery outcome sufficient for H02 lifecycle evaluation
}
```

Provider-specific request/response schemas, credentials, adapters, failover algorithms and receipt normalization remain C04-003-owned.

A delivery failure, fallback or retry:

- preserves the same Notification identity;
- creates/retains distinct Delivery Attempt evidence according to C04-003/upstream rules;
- never repeats the originating business transaction;
- never changes source-domain business state.

### 5.8 State, Concurrency and Lifecycle-Event Publication

Notification state mutation follows B02 owner-local transaction and concurrency rules.

For a transition that requires an H02 Notification lifecycle event:

```text
read current Notification state + version/precondition
        ↓
validate transition against locked H02 lifecycle
        ↓
apply owner-local Notification state mutation
        +
create Durable Publication Intent for exact H02 event
        ↓
SAME owner-local transaction
        ↓
commit
        ↓
C02 publishes/delivers event
```

Rules:

1. an optimistic-concurrency conflict or equivalent precondition failure shall not silently overwrite a newer Notification state;
2. C02 broker/consumer acknowledgement shall not extend the C04 owner-local transaction;
3. after a successful state + publication-intent commit, C02 unavailability is a delivery-recovery concern and shall not roll back the Notification transition;
4. the Durable Publication Intent is technical state and does not become a second canonical event or publisher;
5. only upstream-defined H02 Notification events may be handed to C02.

The canonical H02 event set currently includes:

- `NotificationCreated`
- `NotificationValidated`
- `RecipientResolved`
- `TemplateResolved`
- `NotificationScheduled`
- `NotificationQueued`
- `DeliveryStarted`
- `DeliveryCompleted`
- `DeliveryFailed`
- `NotificationCancelled`
- `NotificationExpired`
- `NotificationRetryStarted`
- `NotificationRetryCompleted`
- `NotificationDelivered`

C04-001 shall not invent a preference-change, provider-fallback, terminal-recovery, emergency-completion or Template lifecycle event where upstream defines none.

### 5.9 Evidence and Operational Observability

C04 shall make attributable technical evidence available for D04 while retaining Notification-owned business/platform meaning.

Material evidence references include, where applicable:

- Notification identity and validated Tenant/Institution scope;
- source trigger identity, canonical publisher/source-owner reference and causation/correlation;
- Template/version and recipient/channel resolution references;
- schedule/expiry/cancellation facts;
- lifecycle transition/outcome;
- C04-003 Delivery Attempt/receipt evidence references;
- authorization/policy decision references where required.

D04 owns enterprise evidence capture/storage/integrity. E05 owns operational telemetry such as execution health, latency, queue/backlog/failure signals and diagnostics. Telemetry shall not be treated as canonical business audit truth.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-001` | Governed dual ingress | A Notification trigger enters C04 through a C02-delivered canonical source event or C01-governed authorized communication request. | Candidate Register; L2-H02 §§9, 33; `L4-H-010-BH-001`. | Preserves source ownership while supporting event-driven and explicitly requested communication. | Existing C01/C02 governance; no new ADR |
| `AD-002` | One canonical Notification identity | C04 coordinates one H02 Notification identity/lifecycle for one admitted governed notification; retry/failover do not create a replacement Notification. | L2-H02 §§11–12; L4-H-010/H015. | Preserves lifecycle traceability and prevents communication recovery from becoming a new business transaction. | None |
| `AD-003` | Event transport and communication delivery remain distinct | C02 transports canonical internal events; C04-003 performs user/system communication delivery through channels/providers. | Candidate Register boundaries; L2-H02 §§3, 5.4, 29–33. | Prevents duplicate technical ownership and broker/provider conflation. | `ADR-L5-0010`, `ADR-L5-0011` constrain C02 only |
| `AD-004` | Owner-local lifecycle commit and publication handoff | Notification state updates use B02 owner-local concurrency, and a required lifecycle event is coupled through atomic state + Durable Publication Intent before C02 delivery. | `L5-B-02-003`; L2-H02 §32.3. | Prevents state/event divergence without extending transactions across owners. | `ADR-L5-0006`, `ADR-L5-0007` |
| `AD-005` | Generic background execution is delegated | C04 retains Notification scheduling meaning while A08 supplies generic Job/Work Unit/Attempt execution for deferred/background work. | L2-H02 §19; Stage-3 closure; A08 boundaries. | Avoids a second scheduler/executor architecture and prevents technical Job states from redefining Notification lifecycle. | `ADR-L5-0013` |
| `AD-006` | C04 family decomposition | 001 owns common admission/identity/lifecycle/handoffs; 002 owns Template/preference/policy execution; 003 owns multi-channel/provider/failover/delivery evidence. | Frozen 105-specification corpus. | Preserves exactly one technical owner for each C04 responsibility. | None |
| `AD-007` | Technology position | C04 inherits canonical C01 REST/OpenAPI, C02 NATS JetStream and A08 worker standards; communication-provider vendors remain intentionally pluggable under `ADR-L5-0033`. | L5 governance + canonical peer ADRs. | Prevents local re-selection while preserving provider neutrality. | Existing Locked ADRs |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant / Institution isolation | Scope is validated at admission and preserved across state, scheduling/background execution, delivery and evidence handoffs; missing/inconsistent protected scope fails closed. | L2-H02 NP-07; `ADR-L5-0004` | `L5-A-04-002` |
| Authorization | Protected Notification operations consume D02 authorization; C04 does not infer authority from a request, channel or provider. | `L4-H-010-BH-001`; D02 ownership | `L5-D-02-004` |
| Source-business integrity | Communication failure/cancel/expiry/retry shall not roll back, repeat or alter source transactions. | L2-H02 §6; L4-H-010/H015 | Source domain + C04 boundary |
| Reliability / recovery | Communication retries/failover remain within C04-003/upstream policy; generic technical retries use A08; event-delivery recovery uses C02. | L4-H-015; Stage-3/C02 architecture | C04-003, A08, C02 |
| Concurrency / consistency | Notification state transitions use owner-local explicit conflict detection and atomic event-intent handoff. | `ADR-L5-0006`, `ADR-L5-0007` | `L5-B-02-003` |
| Audit / evidence | Notification evidence is handed to D04; C04 retains source Notification meaning but not enterprise evidence persistence/integrity ownership. | L2-H02 §5.5; D04 baseline | `L5-D-04-001` |
| Observability | Runtime diagnostics/metrics are handed to E05 and remain distinct from audit truth. | L2-H02 §34; E05 baseline | `L5-E-05-001` |
| Capacity / performance | C04 exposes measurable workload dimensions but does not invent numeric throughput, latency, queue or retry targets. | L5 governance | `L5-E-02` |
| Provider / runtime portability | Baseline is provider/runtime neutral; provider internals remain C04-003, deployment remains E04. | L2-H02 NP-04/05 | C04-003, `L5-E-04` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | Every admitted Notification shall originate from a C01-governed authorized communication request or a C02-delivered canonical source event with preserved source and scope context. | §§5.2–5.3; `AD-001` | `L4-H-010-BH-001`; L2-H02 §§9, 33 |
| `CR-002` | C04 shall preserve source-domain ownership/publisher identity and shall not modify, reverse or repeat the originating business transaction because Notification processing succeeds or fails. | §§5.2, 5.7 | `L4-H-010-BH-004/005`; `L4-H-015-BH-004` |
| `CR-003` | Notification identity shall remain immutable after creation; retries, provider fallback and repeated Delivery Attempts shall not create a replacement Notification solely for recovery. | §5.4; `AD-002` | L2-H02 §§11–12; `L4-H-015-BH-002/004` |
| `CR-004` | C04-001 shall represent only the canonical H02 Notification lifecycle states/transitions and shall not invent a substitute business/platform lifecycle. | §5.4 | L2-H02 §12 |
| `CR-005` | Template, recipient, preference/subscription, quiet-hour and permitted emergency-policy resolution shall use the C04-002 boundary before delivery where applicable; C04-001 shall not duplicate that logic. | §5.5; `AD-006` | Candidate Register; L4-H-010/013/014 |
| `CR-006` | Provider/channel dispatch, provider failover, Delivery Attempt/receipt processing and provider-independent delivery evidence shall use C04-003; C04-001 shall not call provider-specific interfaces directly. | §5.7; `AD-003`, `AD-006` | Candidate Register; L2-H02 NP-05; L4-H-015/H016 |
| `CR-007` | Generic delayed/background technical execution may use A08, but A08 Job/Work Unit/Attempt states shall not replace Notification or Notification Schedule lifecycle state. | §5.6; `AD-005` | L2-H02 §19; `ADR-L5-0013` |
| `CR-008` | A Notification that is cancelled or expired before permitted execution shall not be released for delivery, and source business state shall remain unchanged. | §§5.4, 5.6 | `L4-H-011-BH-004`; `L4-H-010-BH-004` |
| `CR-009` | A committed H02 transition requiring a lifecycle event shall atomically persist the Notification state change and Durable Publication Intent before C02 delivery; C02 failure after commit shall not roll back Notification state. | §5.8; `AD-004` | `ADR-L5-0007`; L2-H02 §32.3 |
| `CR-010` | Concurrent Notification mutation shall use explicit owner-local conflict detection; stale writers shall not silently overwrite a newer state. | §§5.4, 5.8; `AD-004` | `ADR-L5-0006`; `L5-B-02-003` |
| `CR-011` | Only upstream-defined H02 Notification lifecycle events may be handed to C02; no local preference-change, fallback, terminal-recovery, emergency-completion or Template event shall be invented. | §5.8 | L2-H02 §32; L4-H-011/H013/H014/H015 |
| `CR-012` | C02 event transport and C04-003 communication-channel delivery shall remain separate technical responsibilities. | §§4, 5.7–5.8; `AD-003` | Candidate Register; C02/C04 boundaries |
| `CR-013` | Notification evidence shall be handed to D04 and operational telemetry to E05 without treating telemetry as audit truth or transferring Notification/source ownership. | §5.9 | L2-H02 §§5.5, 34; D04/E05 boundaries |
| `CR-014` | No provider, broker, API style, database, queue, framework, runtime product or numeric operational target shall become canonical through this baseline without applicable governance. | `AD-007` | L5 Governance; ADR methodology |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Material Notification L4 sources `L4-H-010` through `L4-H-016` remain `Draft`; relations used here cannot be `Validated`. | Upstream / L4 | Current L4 lifecycle status | L4 governance / authorized architecture review | **Yes for Architecture Approved; No for this authorized Draft** | Approve applicable L4 behavior or issue explicit governance authorization for approval-level use, then revalidate affected C04 relations. |
| `OD-002` | C04-001, C04-002 and C04-003 are all present on canonical `main`; the former child-specification authoring/integration concern is resolved. | Resolved repository item / C04 family | Canonical repository state after family integration | Repository governance / architecture review | No | No further family-integration action is required; maintain cross-file ownership and contract consistency when any C04 sibling changes materially. |
| `OD-003` | **Resolved dependency:** C01 synchronous profile is HTTPS REST/JSON + OpenAPI 3.2 with the Group-1 wire/version/error/idempotency standard. | Dependency / ADR | `ADR-L5-0021`; `ADR-L5-0034` | C01 | No blocker | Consume C01; do not redefine it locally. |
| `OD-004` | Provider/channel product selection and product-specific failover/receipt mechanics are intentionally deferred to C04-003 and applicable technology governance. | Dependency / Technology | No governed product selection | C04-003 technical owner / Lead Architect where ADR required | No for baseline | Keep baseline provider-neutral; resolve only in C04-003 or later approved ADR/technology standard if material. |

---

## 10. Traceability Summary

### Primary L4 Applicability
- `L4-H-010-BH-001`–`BH-007` — governed trigger-to-Notification baseline, source isolation, lifecycle events/evidence.
- `L4-H-011-BH-001`–`BH-007` — scheduling identity, cancellation/expiry and recurring-instance coordination.
- `L4-H-014-BH-001/004/005/006/007` — source-owned emergency fact with H02-only communication outcome/events/evidence.
- `L4-H-015-BH-004/005/006` — recovery preserves Notification/source identity and event/evidence boundaries.
- `L4-H-016-BH-005/007` — final delivery-outcome-to-H02 transition/event handoff.

All above relations are **Specified**, not `Validated`, while their L4 sources remain Draft.

### Material Higher-Level / ADR Constraints
- `L2-H02_Notification_Workflows.md` — canonical Notification ownership, lifecycle, scheduling, provider/event/audit boundaries and exact H02 events.
- `L2-H02_Notification_Template_Approval_Amendment_2026-09-01.md` — Template Approval Authority and lifecycle authority.
- `ADR-L5-0004` — Tenant/Institution isolation.
- `ADR-L5-0006` — owner-local transaction/concurrency.
- `ADR-L5-0007` — atomic state-to-publication-intent handoff.
- `ADR-L5-0010`, `ADR-L5-0011` — C02 delivery and event compatibility.
- `ADR-L5-0013` — A08 generic background/scheduled/retry execution.

### Material L5 Dependencies
- `L5-C-01-001` — synchronous request boundary.
- `L5-C-02-001` — canonical event/messaging transport.
- `L5-C-04-002` — Template/preference/policy execution.
- `L5-C-04-003` — multi-channel/provider/failover/delivery evidence.
- `L5-A-04-002` — Tenant/Institution context.
- `L5-B-02-003` — transaction/concurrency/publication handoff.
- `L5-D-02-004` — authorization enforcement.
- `L5-A-08-001/002` — generic background/scheduled execution.
- `L5-D-04-001` — audit/evidence.
- `L5-E-05-001` — operational observability.

### Architecture Decisions
- `AD-001`–`AD-007`

### Conformance Requirements
- `CR-001`–`CR-014`

### Open / Blocking Items
- `OD-001` — Draft L4 sources block Architecture Approved status.
- `OD-002` — C04 family canonical repository integration is resolved; no repository blocker remains.
- `OD-003` — C01 transport/style dependency is resolved by `ADR-L5-0021` and `ADR-L5-0034`; no C04 blocker remains.
- `OD-004` — provider/product specifics deferred to C04-003/technology governance.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0033`  
Payment, banking/treasury and communication providers remain deployment/institution choices behind C03/C04 adapters. No repository-wide external-provider vendor is canonical; provider-specific semantics may not become business architecture.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled C04 with the locked API/broker/background-execution standards while preserving `ADR-L5-0033` provider pluggability and the separate L4-source approval blocker.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
