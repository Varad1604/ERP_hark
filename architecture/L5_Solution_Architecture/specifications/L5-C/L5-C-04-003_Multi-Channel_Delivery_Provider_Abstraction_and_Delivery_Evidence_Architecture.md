# Multi-Channel Delivery, Provider Abstraction & Delivery Evidence Architecture

## 1. Specification Metadata

**Specification ID:** `L5-C-04-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-04 — Notification & Communication Architecture  
**Canonical Technical Responsibility:** Multi-channel provider abstraction, controlled failover and delivery evidence.  
**Technical Architecture Owner:** L5-C-04 — Notification & Communication Architecture  
**Assigned Author:** Chetan  
**Version:** `1.2.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Stage 3 Final Closure & Stage 4 Proceed Decision 2026-09-15; `L2-H02_Notification_Workflows.md` v1.0 Locked; `L5-C-04-001` v1.2.0 Architecture Approved; `L5-C-04-002` v1.1.0 Architecture Approved; `L5-A-06-001`; `L5-A-06-002`; `L5-A-04-002`; `L5-B-02-003`; `L5-A-08-001`; `L5-A-08-002`; `L5-D-02-004`; `L5-D-03-002`; `L5-D-03-004`; `L5-D-04-001`; `L5-E-05-001`; `L5-C-02-001`.  
**Referenced By:** `L5-C-04-001`; Notification-consuming capabilities requiring provider-independent delivery outcome/evidence.  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0010`; `ADR-L5-0011`; `ADR-L5-0027`; `ADR-L5-0033`; `ADR-L5-0040`; `ADR-L5-0041`; `ADR-L5-0045`.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the technical architecture through which a delivery-ready H02 Notification is executed across one or more permitted communication channels while preserving:

- one canonical Notification identity;
- provider independence;
- independent channel outcomes;
- controlled provider selection, retry and failover;
- immutable Delivery Attempt history;
- provider-independent Delivery Receipt interpretation;
- exact association between provider evidence and Delivery Attempts;
- traceable delivery outcome handoff to `L5-C-04-001`;
- source-domain business ownership and outcome independence.

The architecture converts the provider-specific act of transmission into a governed H02 delivery outcome without allowing provider-specific APIs, credentials, status codes or operational failures to become business architecture.

### In Scope

- channel execution from the permitted channel context supplied by C04-002;
- H02 Delivery technical lifecycle realization;
- Delivery identity and association to exactly one Notification;
- independent per-channel delivery execution and result aggregation;
- provider abstraction interface responsibilities;
- provider configuration, eligibility and health-aware selection;
- provider authentication/credential use through D03-owned credential boundaries;
- required protected provider exchange through D03-owned cryptographic controls;
- controlled provider/channel retry and fallback/failover;
- immutable Delivery Attempt records;
- H02 notification-delivery dead-letter isolation and controlled reprocessing boundary;
- provider status/receipt intake and provider-independent normalization;
- immutable Delivery Receipt chronology;
- receipt-to-Delivery Attempt association;
- delivery-evidence aggregation for C04-001;
- D04 audit/evidence handoff;
- E05 operational metrics/diagnostic handoff.

### Out of Scope / Non-Goals

This specification does not own:

- source-domain business facts, decisions, transactions, lifecycles, events or publishers;
- Notification identity/lifecycle/scheduling or Notification lifecycle-event publication — `L5-C-04-001`;
- Template selection, recipient policy eligibility, rendered-content generation, Preference/Subscription/quiet-hour/Emergency policy execution — `L5-C-04-002`;
- enterprise event/message transport, broker subscriptions, event ordering/replay/message dead-letter semantics — `L5-C-02`;
- generic API style, contract transport or API-management technology — `L5-C-01`;
- generic background Job/Work Unit/Attempt architecture — `L5-A-08`;
- authentication/authorization policy — `L5-D-01/02`;
- cryptographic algorithm/protection implementation — `L5-D-03-002`;
- provider secret/certificate/machine-credential lifecycle — `L5-D-03-004`;
- physical persistence/database product — `L5-B-02`;
- enterprise audit storage/integrity — `L5-D-04`;
- observability backend/platform — `L5-E-05`;
- provider-specific SDK/framework/vendor selection, endpoint syntax, callback protocol, retry numeric values, timeout values or rate limits;
- L6 operational consoles, provider-management screens or notification-status UI.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-C-04-003 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-C-04-003` row | Multi-channel provider abstraction, controlled failover and delivery evidence. | Establishes the canonical responsibility and ownership boundary. |
| L5 Governance | `L5_Stage3_Final_Closure_and_Stage4_Proceed_Decision_2026-09-15.md` | v1.0.0 / Locked | §6 Stage-4 Authorization | C04 is authorized to proceed; minimum dependency `C01 + C02` is stable for downstream use. | Authorizes Stage-4 Draft authoring. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§17–18 | Channel selection is policy-driven; multi-channel execution preserves independent channel outcomes; one channel failure does not automatically fail remaining channels. | Governs channel-execution semantics. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§25–26 | Delivery owns provider assignment/dispatch/completion tracking; Delivery Attempts are independently traceable and immutable after completion. | Establishes Delivery/Attempt technical model. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§27–28 | Retry preserves Notification identity, creates new Delivery Attempts, uses policy-driven limits and immutable history; H02 Delivery DLQ isolates terminal/exhausted delivery work. | Governs controlled communication-delivery recovery. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§29–30 | Provider abstraction isolates provider-specific APIs; providers are interchangeable; provider selection/failover is policy/configuration driven; provider health influences selection/failover/retry. | Governs provider abstraction and failover. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §29.5 | Provider health model uses `Healthy`, `Degraded`, `Unavailable`, `Maintenance`, `Disabled`. | Supplies exact provider-health states; no local alternatives are invented. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §31 | Delivery Receipts are immutable, reference exactly one Delivery Attempt and use provider-independent interpretation. | Governs receipt normalization/evidence. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§32–35 | H02 lifecycle events remain canonical; monitoring is provider-independent; secure provider communication is required; provider credentials are not embedded in Templates. | Constrains event/evidence/security handoffs. |
| L4 | `L4-H-010` | v1.0.0 / Draft | `BH-003`–`BH-007` | Multiple permitted channels remain evidence for one Notification outcome; delivery failure does not change source outcome; H02 publishes only its defined events/evidence. | Drives delivery aggregation and source-business isolation. Relation state: **Specified** only. |
| L4 | `L4-H-014` | v1.0.0 / Draft | `BH-003`–`BH-007` | Critical eligible Notifications may use governed multi-channel delivery with independent channel outcomes; source emergency ownership is unchanged. | Drives priority/multi-channel execution boundary. Relation state: **Specified** only. |
| L4 | `L4-H-015` | v1.0.0 / Draft | `BH-001`–`BH-007` | Failed Delivery Attempt recovery/fallback uses same Notification identity; each later attempt is distinct immutable evidence; no provider-fallback/terminal-recovery event is invented. | Primary retry/failover/recovery authority. Relation state: **Specified** only. |
| L4 | `L4-H-016` | v1.0.0 / Draft | `BH-001`–`BH-007` | Provider evidence is associated with exactly one Delivery Attempt, normalized through H02 receipt semantics, preserved chronologically and used only for permitted H02 consequences. | Primary provider-receipt/evidence authority. Relation state: **Specified** only. |
| L5 Peer | `L5-C-04-001` | v1.2.0 / Architecture Approved | §5.7 C04-003 Delivery Boundary; §5.8 | Supplies `DeliveryReadyNotification`; consumes provider-independent `DeliveryOutcome`; retains Notification lifecycle and lifecycle-event publication. | Defines C04 family ingress/egress contract. |
| L5 Peer | `L5-C-04-002` | v1.1.0 / Architecture Approved | §5.9 Resolved Notification Context | Supplies rendered content, policy-eligible recipients and permitted channel/policy constraints. | Defines delivery eligibility input; C04-003 does not re-run policy. |
| L5 Peer | `L5-A-06-001/002` | v1.0.0 / Architecture Approved | configuration registration/resolution architecture | Generic configuration/version/scope/inheritance/effective dating. | Supports provider priority, failover/retry configuration without C04 duplicating generic configuration mechanics. |
| L5 Peer | `L5-A-04-002` | v1.1.0 / Architecture Approved | §§2–4 | Tenant/Institution scope propagation and fail-closed isolation. | Constrains provider configuration, delivery execution and evidence scope. |
| L5 Peer | `L5-B-02-003` | v1.1.0 / Architecture Approved | §§5–8 | Owner-local transaction/concurrency and explicit conflict handling. | Governs Delivery/Attempt/Receipt persistence consistency. |
| L5 Peer | `L5-D-02-004` | v1.0.0 / Architecture Approved | policy decision/enforcement | Authorization remains external to C04. | Protects provider configuration, controlled operational recovery/reprocessing and privileged actions without inventing roles. |
| L5 Peer | `L5-D-03-002` | v1.3.0 / Architecture Approved | §§2–7 | Owns cryptographic protection for protected technical exchanges; mandatory protection does not silently downgrade. | Protects provider communication where required. |
| L5 Peer | `L5-D-03-004` | v1.3.0 / Architecture Approved | §§2–7 | Owns application/integration secrets, certificates and machine credentials; secret material cannot propagate into ordinary config/log/evidence. | Supplies provider credentials/certificates without C04 owning secret lifecycle. |
| L5 Peer | `L5-D-04-001` | v1.1.1 / Locked | §§2–4 | Shared audit/evidence capture without transferring business meaning. | Receives immutable delivery-attempt/receipt/failover evidence references. |
| L5 Peer | `L5-E-05-001` | v1.3.0 / Architecture Approved | §§2–5 | Operational telemetry/observability remains distinct from business audit truth. | Receives provider/channel health, latency, failure and backlog diagnostics. |
| L5 Peer | `L5-C-02-001` | v1.0.3 / Architecture Approved | §§4–8 | Owns enterprise event/message transport while event semantics/publishers remain upstream-owned. | Carries H02 lifecycle events after C04-001 publication handoff; does not transport provider-delivery work as a competing C04 mechanism. |

### 3.2 L4 Applicability

All current H02 L4 sources used by this specification are `Draft`; these relations are **Specified**, not `Validated`.

| L4 BH | C04-003 Technical Contribution | L5 Evidence Target | Relation State |
|---|---|---|---|
| `L4-H-010-BH-003` | Aggregates independent channel outcomes into provider-independent delivery evidence for one Notification. | §§5.3, 5.10; `CR-004`, `CR-017` | Specified |
| `L4-H-010-BH-004/005` | Keeps delivery failure source-business independent and returns only H02 delivery result to C04-001 for exact lifecycle-event handling. | §§5.10–5.11; `CR-018`, `CR-019` | Specified |
| `L4-H-010-BH-006/007` | Preserves channel/attempt/receipt context required for final Notification evidence. | §§5.4, 5.12; `CR-020` | Specified |
| `L4-H-014-BH-003/004` | Executes critical eligible multi-channel delivery with independent channel outcomes; does not change source emergency state. | §§5.3, 5.10; `CR-004`, `CR-018` | Specified |
| `L4-H-014-BH-005/006/007` | Returns channel/final outcome evidence to C04-001; does not invent emergency-completion event. | §§5.10–5.12; `CR-019`, `CR-020` | Specified |
| `L4-H-015-BH-001` | Associates a failed attempt with the existing Notification and immutable attempt history; applies H02 delivery failure classification. | §§5.4, 5.7; `CR-009`, `CR-010` | Specified |
| `L4-H-015-BH-002` | Executes permitted retry/provider/channel fallback under the same Notification identity and distinct later Delivery Attempt. | §§5.7–5.9; `CR-011`–`CR-015` | Specified |
| `L4-H-015-BH-003/004` | Terminates or isolates exhausted recovery without invented Notification recovery state or source transaction repetition. | §§5.8–5.10; `CR-016`, `CR-018` | Specified |
| `L4-H-015-BH-005` | Returns actual recovery outcome to C04-001 for only the canonical H02 event path; no fallback/recovery event invented here. | §5.11; `CR-019` | Specified |
| `L4-H-015-BH-006/007` | Preserves attempts, failure class, recovery/fallback lineage and final outcome. | §5.12; `CR-020` | Specified |
| `L4-H-016-BH-001/002` | Associates each provider receipt with exactly one Delivery Attempt and stores immutable chronology. | §5.9; `CR-021`, `CR-022` | Specified |
| `L4-H-016-BH-003/004` | Maps only supported provider evidence to H02 receipt meaning; ambiguous/unsupported evidence remains unresolved. | §§5.9–5.10; `CR-023`, `CR-024` | Specified |
| `L4-H-016-BH-005` | Returns receipt-caused outcome to C04-001; C04-001 alone performs Notification transition/event publication handoff. | §5.11; `CR-019` | Specified |
| `L4-H-016-BH-006/007` | Preserves immutable receipt chronology, exact attempt association, normalized interpretation and governed outcome. | §5.12; `CR-020`–`CR-024` | Specified |

### 3.3 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| `C-001` | Business domains never directly invoke external notification providers. | L2-H02 §§4.2 NP-05, 6 | All provider access is behind C04-003 provider abstraction. |
| `C-002` | Provider-specific implementations contain no business rules and remain interchangeable. | L2-H02 §§6, 29 | Provider adapters translate transport/provider semantics only. |
| `C-003` | Multi-channel execution preserves independent channel outcomes. | L2-H02 §18; H014 | Channel failure does not automatically fail remaining permitted channels. |
| `C-004` | Delivery retries/failover preserve Notification identity and never repeat the originating business transaction. | L2-H02 §§6, 27, 30; H015 | Recovery is communication-only and evidence-preserving. |
| `C-005` | Every retry creates a new Delivery Attempt; attempt history is immutable. | L2-H02 §§26–27; H015 | No attempt record is rewritten or reused. |
| `C-006` | Provider health influences provider selection/failover/retry routing but not Notification identity/history. | L2-H02 §29.5 | Exact H02 provider health states are used; provider health is operational selection context only. |
| `C-007` | Receipts are immutable and reference exactly one Delivery Attempt. | L2-H02 §31; H016 | Ambiguous/unmatched receipt is unresolved, never forced onto a Notification. |
| `C-008` | Provider evidence does not determine source-business transaction success. | L2-H02 §31; H016 | Delivery outcome remains communication evidence only. |
| `C-009` | Required provider exchanges are protected and provider credentials are governed secrets/machine credentials. | L2-H02 §35; D03-002/004 | C04 consumes cryptographic protection/credential capabilities but does not own them. |
| `C-010` | H02 Notification lifecycle events are canonical and C04-003 shall not invent provider-fallback/terminal-recovery events. | H015; H016; L2-H02 §32 | C04-003 returns outcomes; C04-001 owns lifecycle transition/event publication handoff. |
| `C-011` | H02 delivery dead-letter processing is a Notification delivery concern, distinct from C02 message-delivery dead-letter behavior. | L2-H02 §§27–28 versus C02 ownership | Prevents duplicate event/messaging ownership. |

### 3.4 Assumptions

No material provider, retry, timeout, rate-limit, failover count or receipt-ordering assumption is promoted to canonical architecture.

Where provider capability differs, the provider adapter exposes only supported normalized H02 semantics. Unsupported capability is explicit and shall not be fabricated.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

`L5-C-04-003` owns:

- H02 Delivery technical execution boundary;
- H02 Delivery technical lifecycle realization;
- one-to-one Delivery-to-Notification association;
- per-channel delivery execution;
- multi-channel outcome aggregation;
- provider abstraction interface;
- provider configuration consumption and provider eligibility/health-aware selection;
- provider-specific request transformation and response/error normalization;
- provider selection and controlled failover;
- Delivery Attempt identity/history;
- H02 communication-delivery retry semantics and attempt lineage;
- H02 delivery dead-letter isolation/exhaustion boundary;
- controlled recovery/reprocessing request boundary without inventing a business actor;
- provider status/receipt intake;
- provider-independent receipt normalization;
- exact Receipt-to-Delivery-Attempt association;
- immutable receipt chronology;
- provider-independent delivery outcome/evidence returned to C04-001;
- channel/provider operational telemetry handoff.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-C-04-001` | Notification baseline/lifecycle | Supplies delivery-ready Notification and retains Notification lifecycle/event publication. | C04-003 shall not mutate source business state or own canonical Notification lifecycle events. |
| `L5-C-04-002` | Template/preference/policy execution | Supplies rendered content, policy-eligible recipients and permitted channel/policy constraints. | C04-003 shall not re-evaluate Preference/Subscription/quiet-hour/Emergency policy. |
| `L5-A-06-001/002` | Generic configuration | Owns configuration admission/version/scope/inheritance/effective dating. | Provider priority, provider eligibility, retry/failover policy configuration may use shared configuration mechanics. |
| `L5-A-04-002` | Tenant/Institution context | Supplies validated protected scope. | Provider configuration, attempts and evidence remain scope-safe. |
| `L5-B-02-003` | Transaction/concurrency | Supplies owner-local state mutation and explicit conflict handling. | Delivery/Attempt/Receipt updates cannot silently overwrite each other. |
| `L5-A-08-001/002` | Generic background/retryable technical execution | May execute deferred technical delivery work when invoked through the C04 baseline; C04-003 retains communication retry/failover decision and Delivery Attempt semantics. | Prevents C04-003 from creating a second generic Job/Work Unit/Attempt execution architecture. |
| `L5-D-02-004` | Authorization enforcement | Protects provider-configuration/operational recovery actions. | C04 does not invent an operator role or authority. |
| `L5-D-03-002` | Cryptographic protection | Protects provider exchange where required. | C04 shall not choose cryptographic primitives/algorithms. |
| `L5-D-03-004` | Provider secrets/certificates/machine credentials | Supplies scope-bound credential use. | Provider credentials never become ordinary C04 configuration/evidence. |
| `L5-D-04-001` | Audit/evidence | Receives delivery evidence references. | C04 does not own enterprise evidence persistence/integrity. |
| `L5-E-05-001` | Operational telemetry | Receives health, latency, failure, retry and backlog signals. | Operational diagnostics remain separate from canonical evidence. |
| `L5-C-02-001` | Canonical event transport | Carries C04-001-published H02 lifecycle events. | C04-003 shall not build competing event/message transport. |

### Must Not Own

C04-003 shall not:

- select/modify source-domain business outcomes or business events;
- re-resolve Template, recipient policy eligibility, Preference/Subscription, quiet-hour or Emergency policy decisions;
- change Notification identity or introduce a second Notification lifecycle;
- publish H02 lifecycle events directly as a competing owner to C04-001;
- invent provider-fallback, provider-switch, dead-letter, manual-recovery or terminal-recovery canonical business events;
- invent a human provider-operations/reprocessing actor or authority;
- create a second enterprise event/message retry/dead-letter subsystem that duplicates C02;
- create a new secrets/certificate/machine-credential lifecycle;
- embed credentials in provider configuration, Template, logs or evidence;
- choose cryptographic algorithms or silently downgrade protected transport;
- interpret provider delivery success as source-business success;
- prescribe provider vendor/SDK/framework, callback protocol or UI.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| Delivery Admission Boundary | Control Point | Accept only a delivery-ready Notification from C04-001/C04-002 and establish one H02 Delivery execution context. | Notification identity, validated scope, rendered-content reference, policy-eligible recipients, permitted channels, priority/schedule/expiry context. | Delivery identity/reference and association to exactly one Notification. | Per-channel execution plan. | C04-001/002, A04 | Missing/inconsistent required context rejects delivery; no provider call occurs. | L2-H02 §§18,25. |
| Channel Execution Coordinator | Component | Execute one or more permitted channels independently and aggregate channel outcomes. | Delivery context + permitted channels. | Channel execution/outcome references. | Provider-independent channel outcomes. | A06 config where ordering/strategy is configured | One channel failure does not automatically fail remaining channels. | L2-H02 §18; H014. |
| Provider Abstraction Boundary | Adapter / Control Point | Isolate provider-specific endpoint/request/response/error mechanics from H02 architecture. | Provider-independent delivery request. | Provider adapter configuration references only; credentials externalized to D03. | Provider request + normalized provider response/failure. | D03, A06 | Provider-specific failure cannot leak business semantics upstream. | L2-H02 §§29–30. |
| Provider Health & Selection Resolver | Component | Determine eligible provider according to channel, configured provider policy/priority and exact H02 provider-health state. | Channel, scope, provider configuration, health observations. | Provider selection evidence. | Selected eligible provider or controlled no-provider outcome. | A06, E05 | Disabled/Unavailable/Maintenance provider is not normally selected; no arbitrary fallback. | L2-H02 §29.5. |
| Delivery Attempt Controller | Component | Create one immutable attempt identity for each concrete provider execution and associate it with the Delivery/Notification. | Selected provider + channel + Delivery. | Delivery Attempt identity, number/order, start/end, result, failure reason, response metadata, correlation. | Completed attempt evidence. | B02, D04 | Completed attempt is not rewritten; retry/failover creates a new attempt. | L2-H02 §26; H015. |
| Protected Provider Exchange | Adapter | Use D03-owned credential/protection boundaries for provider authentication and protected transmission. | Provider endpoint/reference, credential reference, protected payload/context. | No secret lifecycle owned by C04. | Protected provider request/verified response. | D03-002/004 | Missing/invalid required credential/protection stops provider operation; no plaintext fallback. | L2-H02 §35; D03. |
| Delivery Failure Classifier | Component | Normalize provider/transport outcome into H02 delivery failure classification/context sufficient for retry/failover policy. | Completed failed attempt + provider-independent error mapping. | Failure classification/evidence. | Retry/failover/terminal recovery eligibility input. | A06 policy config | Unknown/unsupported failure is explicit; no automatic success. | H015. |
| Delivery Retry & Failover Coordinator | Component | Apply H02 policy-driven bounded retry/provider/channel fallback while preserving Notification/Delivery identity. | Failure class, immutable attempt history, retry/failover config, provider health, current expiry/eligibility context. | Recovery lineage + new attempt references. | Next attempt or recovery exhausted outcome. | A06, C04-001 eligibility context | Retry never repeats source transaction; unbounded retry is not canonicalized. | L2-H02 §§27,30; H015. |
| H02 Delivery Dead-Letter Boundary | Control Point / Store Boundary | Isolate exhausted/terminal delivery work and evidence for governed investigation/reprocessing. | Notification/Delivery/Attempt/failure lineage. | Immutable H02 delivery dead-letter entry/evidence. | Controlled recovery request or terminal evidence. | B02, D02, D04 | Does not become C02 message DLQ; no invented human actor/authority. | L2-H02 §28; H015 §6. |
| Provider Receipt Intake Boundary | Adapter / Control Point | Receive provider delivery-status evidence and associate it with exactly one existing Delivery Attempt. | Provider status evidence received through the provider abstraction + provider/attempt correlation context. | Raw provider evidence reference; no direct business success. | Normalization candidate or unresolved evidence. | C01/provider adapter boundary, D03 | Unmatched/ambiguous evidence is unresolved; not attached heuristically. | H016-BH-001/004. |
| Receipt Normalizer | Component | Map supported provider statuses into H02 provider-independent receipt types. | Provider evidence + exact provider adapter mapping. | Immutable normalized Delivery Receipt. | Receipt chronology + permitted delivery consequence. | Provider abstraction config | Unsupported provider status remains unresolved; no invented H02 receipt type. | L2-H02 §31; H016. |
| Delivery Outcome Aggregator | Component | Derive channel/Delivery outcome evidence from immutable attempts/receipts without changing source-business state. | Channel outcomes, attempts, normalized receipts. | Provider-independent Delivery outcome/evidence. | `DeliveryOutcome` to C04-001. | B02 | Intermediate receipts do not prematurely become delivered outcome. | H010/H014/H016. |
| Evidence & Telemetry Handoff | Adapter | Supply D04 evidence references and E05 operational telemetry. | Provider selection, attempts, retries, failover, receipt chronology, final outcome. | D04/E05-owned downstream records. | Evidence/telemetry references. | D04, E05 | Secrets/private credentials and unnecessary protected payload content are excluded. | L2-H02 §§31,34–35. |

### 5.2 Delivery Admission and Per-Channel Execution

C04-003 receives the family handoff:

```text
DeliveryReadyNotification
{
  Notification identity
  validated Tenant / Institution scope
  policy-eligible recipient reference set
  rendered-content reference
  permitted channel / communication-policy constraints
  priority / schedule / expiry context
  causation / correlation context
}
```

C04-003 does **not** re-evaluate whether a recipient may receive a Notification or whether quiet-hour/Emergency policy allows delivery. Those decisions are C04-002-owned.

For each admitted Delivery:

1. exactly one Notification identity is referenced;
2. permitted channels are treated as independently executable channel paths;
3. channel execution follows the already-resolved communication policy context;
4. provider assignment occurs only after the channel is eligible;
5. provider failure on one channel does not automatically fail another permitted channel;
6. aggregated channel evidence is returned to C04-001 for the canonical H02 Notification consequence.

Supported architectural delivery strategies come from locked H02 and may include:

- Primary Only;
- Sequential;
- Parallel;
- Fallback;
- Policy Driven.

These are architecture-level strategies, not provider/product implementation algorithms.

### 5.3 H02 Delivery State Model

C04-003 realizes only the Delivery states explicitly defined by locked H02:

| H02 Delivery State | Upstream Meaning |
|---|---|
| `Pending` | Awaiting execution. |
| `Queued` | Ready for provider assignment. |
| `Dispatching` | Being dispatched. |
| `In Progress` | Provider processing. |
| `Delivered` | Successfully delivered. |
| `Failed` | Permanently failed. |
| `Cancelled` | Delivery cancelled. |
| `Expired` | Delivery window expired. |

The current authoritative H02 source defines this state set but does not provide a complete canonical Delivery transition graph. C04-003 therefore shall not invent additional state transitions merely for implementation convenience. Actual progression shall be constrained by the documented H02 delivery, retry, cancellation, expiry and receipt behaviors.

This Delivery state model is separate from the canonical Notification lifecycle owned/coordinated by C04-001.

Rules:

- every Delivery references exactly one Notification;
- Delivery state does not become source-domain business state;
- retry/failover does not create a new Notification;
- Delivery cancellation/expiry cannot resurrect an expired/cancelled Notification;
- provider-specific statuses do not become additional canonical Delivery states;
- receipt evidence may cause only a consequence supported by upstream H02 behavior.

### 5.4 Delivery Attempt Model

Every concrete provider execution creates a distinct Delivery Attempt.

Conceptually:

```text
DeliveryAttempt
{
  attempt identity
  Delivery / Notification references
  channel
  provider reference
  attempt order/number
  start ordering/timestamp evidence
  completion ordering/timestamp evidence
  normalized result
  failure reason/classification where applicable
  provider-response metadata reference
  correlation / causation references
}
```

These are conceptual categories, not a physical schema.

Invariants:

1. one Delivery Attempt belongs to one Delivery and therefore one Notification;
2. completed Delivery Attempts are immutable;
3. provider response does not overwrite a prior attempt;
4. each retry or provider fallback creates a new Delivery Attempt;
5. the full attempt sequence remains reconstructable;
6. provider credentials/secret values are never persisted as attempt evidence;
7. an attempt is communication execution evidence only and never a source-business transaction attempt.

### 5.5 Provider Abstraction Boundary

Provider adapters isolate provider-specific mechanics.

Each provider adapter may perform only provider-integration responsibilities such as:

- provider authentication using D03-supplied credential references;
- provider-specific request transformation;
- provider response normalization;
- provider error mapping;
- receipt/status correlation extraction;
- provider health reporting.

Provider adapters shall not own:

- Notification lifecycle;
- Template/Preference/communication-policy rules;
- recipient eligibility decisions;
- source-domain business rules;
- source-domain business events;
- provider-credential lifecycle;
- cryptographic key/algorithm policy;
- enterprise audit truth.

Provider additions/replacements shall not require business domains to change their workflows or invoke provider APIs directly.

### 5.6 Provider Health and Selection

C04-003 uses the exact upstream H02 provider health states:

| Provider Health State | H02 Meaning | C04-003 Selection Consequence |
|---|---|---|
| `Healthy` | Available for normal governed use. | Eligible for normal selection. |
| `Degraded` | Available with reduced quality/reliability. | Eligible according to policy; lower priority/failover preference may apply. |
| `Unavailable` | Cannot accept/complete governed traffic. | Excluded from normal selection; failover may be evaluated. |
| `Maintenance` | Temporarily withdrawn for governed operational activity. | Excluded unless explicit policy allows otherwise; failover/hold follows policy. |
| `Disabled` | Administratively/policy disabled. | Not eligible until re-enabled by authoritative control. |

Provider health:

- influences provider selection, failover and retry routing;
- does not change Notification identity;
- does not rewrite Delivery Attempt history;
- is operational/provider context, not source-business state;
- is observable through E05;
- cannot override C04-002 channel/recipient/policy eligibility.

Provider selection uses governed configuration and shall not embed a hard-coded vendor preference in application logic.

### 5.7 Provider Credential and Protected-Exchange Boundary

Provider credentials, certificates and machine credentials are D03-004-owned.

C04-003 stores only references sufficient to request authorized technical use:

```text
ProviderCredentialUseContext
{
  provider / integration reference
  technical consumer
  purpose
  scope
  credential reference
}
```

Rules:

1. credential material is not embedded in Template content;
2. credential material is not ordinary provider configuration;
3. credential material does not appear in logs, telemetry, Delivery Attempts, receipts or D04 evidence;
4. invalid/expired/revoked credential prevents the affected provider operation;
5. credential possession does not create business authority.

Where a provider exchange requires protected transport/integrity, C04-003 consumes D03-002.

Mandatory protection failure shall not silently downgrade to an unprotected provider path.

### 5.8 Retry, Failover and Recovery

When an attempt fails:

```text
failed Delivery Attempt
        ↓
associate with same Notification / Delivery
        ↓
normalize failure classification
        ↓
evaluate H02 retry/failover policy
        ↓
is recovery permitted and Notification still eligible?
      /                                      \
    no                                        yes
    ↓                                          ↓
terminal/exhausted evidence              choose governed recovery
or H02 delivery DLQ                     retry same provider
                                         OR provider failover
                                         OR channel fallback
                                               ↓
                                      create NEW Delivery Attempt
```

Rules:

1. retry/failover is communication recovery only;
2. Notification identity remains unchanged;
3. source business transaction is never repeated, reversed or compensated by C04-003;
4. each later execution uses a new immutable Delivery Attempt;
5. provider/channel fallback is used only when the current H02 communication policy permits it;
6. provider health may influence failover routing;
7. retry limits/delays/backoff values are governed configuration, not hard-coded by this architecture;
8. permanent/non-retryable failure terminates ordinary retry processing;
9. an ambiguous/unclassified failure shall not be converted into success or unrestricted automatic retry;
10. where a retry/failover needs deferred or generic background technical execution, C04-003 uses the C04-001/A08 execution boundary rather than creating a second generic Job/Work Unit/Attempt engine;
11. C04-003 creates no provider-fallback or terminal-recovery canonical event;
12. recovery outcome is returned to C04-001 for the exact upstream-defined H02 Notification transition/event path.

### 5.9 H02 Delivery Dead-Letter Boundary

Locked H02 defines a delivery Dead-Letter Queue for terminal/exhausted Notification delivery work.

C04-003 interprets this as a logical **H02 delivery dead-letter isolation boundary**, independent of technology.

Conceptually, the dead-letter entry preserves:

```text
H02DeliveryDeadLetter
{
  Notification reference
  Delivery reference
  channel
  provider reference
  failure reason/classification
  retry/attempt history reference
  last failure ordering/timestamp evidence
  correlation reference
  operational evidence references
}
```

Rules:

- dead-letter evidence is immutable;
- historical attempts are never altered;
- controlled recovery creates a new delivery execution/attempt where authoritative policy/authorization permits;
- no universal human reprocessing actor or approval authority is invented;
- protected recovery/reprocessing actions consume D02 authorization;
- this boundary is **not** the C02 event/message DLQ;
- no queue/broker/storage product is selected.

### 5.10 Provider Receipt Intake and Normalization

A provider status becomes canonical H02 delivery evidence only after:

1. the evidence is accepted through the provider adapter boundary;
2. it can be associated with **exactly one existing Delivery Attempt**;
3. the provider-specific status can be mapped to a supported H02 receipt interpretation;
4. it is preserved as a new immutable chronological receipt.

Supported upstream H02 receipt types are:

- `Accepted`
- `Queued`
- `Sent`
- `Delivered`
- `Read`
- `Failed`
- `Expired`
- `Rejected`

C04-003 shall not create additional canonical receipt types merely because a provider exposes more status codes.

Mapping rules:

```text
provider-specific status
        ↓
provider adapter mapping
        ↓
supported H02 receipt?
      /               \
    no                 yes
    ↓                   ↓
Unresolved evidence   immutable Delivery Receipt
```

An unsupported or ambiguous provider status remains unresolved evidence.

An unmatched receipt is not attached to the "most likely" Notification/Delivery Attempt.

### 5.11 Receipt Consequence and Delivery Outcome Aggregation

Receipt meaning is provider-independent after normalization.

Upstream behavior establishes:

- `Accepted` / `Queued` / `Sent` remain intermediate evidence and do not prove recipient delivery;
- `Delivered` / `Read` may support the applicable delivered consequence;
- `Failed` / `Rejected` may support the applicable failure/recovery consequence;
- `Expired` may support the applicable expiry consequence.

A receipt alone does not determine source-business success.

For one multi-channel Notification, C04-003 aggregates channel evidence without hiding independent channel history.

Conceptual output:

```text
DeliveryOutcome
{
  Notification identity
  Delivery reference
  per-channel outcome references
  Delivery Attempt history references
  receipt chronology references
  normalized failure/recovery lineage where applicable
  aggregate delivery outcome sufficient for H02 Notification evaluation
  correlation/evidence references
}
```

C04-003 returns this provider-independent outcome to C04-001.

C04-001 then:

- validates the permitted H02 Notification transition;
- persists the Notification transition using B02;
- creates the exact required H02 lifecycle-event publication intent;
- hands event delivery to C02.

C04-003 does not become a second Notification event publisher.

### 5.12 Persistence, Concurrency and Evidence

C04-003 consumes B02 owner-local transaction/concurrency architecture.

#### Delivery / Attempt consistency

- Delivery and Attempt identities are stable;
- creation/completion mutations use explicit version/precondition checks where concurrent processing is possible;
- a stale callback/worker cannot silently overwrite a newer final attempt state;
- completed attempt evidence is immutable.

#### Receipt chronology

- each normalized receipt is append-only evidence;
- ordering/chronology is preserved;
- earlier receipts are not rewritten by later provider evidence;
- exactly-one-attempt association is retained.

#### Recovery lineage

The architecture must reconstruct:

```text
Notification
  ↓
Delivery
  ↓
Attempt 1 — failed
  ↓
retry/fallback decision
  ↓
Attempt 2 — provider/channel ...
  ↓
receipts ...
  ↓
final provider-independent delivery outcome
```

#### Audit handoff

D04 receives references sufficient to reconstruct:

- selected channel/provider;
- provider health/selection basis where material;
- every Delivery Attempt;
- normalized failure classification;
- retry/failover decision lineage;
- dead-letter isolation/reprocessing evidence;
- immutable receipt chronology;
- final provider-independent delivery outcome.

Secret/private credential material and unnecessary Notification payload content are excluded.

### 5.13 Operational Observability

E05 receives provider/channel operational telemetry such as:

- attempt count/rate;
- delivery latency;
- provider response time;
- provider health;
- retry/failover activity;
- provider/channel success/failure rate;
- H02 delivery dead-letter volume;
- unresolved/ambiguous receipt count;
- delivery backlog/age where observable.

No numeric threshold or SLA is invented here.

Operational telemetry:

- may influence provider health assessment where H02/provider policy permits;
- does not become D04 audit truth;
- does not itself change Notification state;
- does not expose credentials or protected content.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-001` | Provider abstraction | Every external communication provider is accessed through a C04-003 provider abstraction boundary; business domains never invoke provider APIs directly. | L2-H02 NP-05, §§6,29 | Preserves provider independence and canonical ownership. | None |
| `AD-002` | Delivery/Notification separation | C04-003 owns Delivery execution/evidence while C04-001 owns Notification identity/lifecycle/event publication. | Frozen C04 corpus; L2-H02 §§11–12,25–32 | Prevents duplicate lifecycle/event ownership. | None |
| `AD-003` | Multi-channel independence | Permitted channels execute independently; aggregate outcome preserves each channel's evidence. | L2-H02 §18; H014 | One provider/channel failure must not erase other outcomes. | None |
| `AD-004` | Attempt model | Every concrete provider execution has a distinct immutable Delivery Attempt; retries/fallback create new attempts under same Notification. | L2-H02 §§26–27; H015 | Preserves recovery traceability without duplicate Notification identity. | None |
| `AD-005` | Provider health model | Use exact H02 provider states `Healthy`, `Degraded`, `Unavailable`, `Maintenance`, `Disabled`; no local health states. | L2-H02 §29.5 | Keeps provider selection interoperable with upstream architecture. | None |
| `AD-006` | Retry/failover control | Recovery is policy/configuration driven, bounded and communication-only; no source-business retry. | L2-H02 §§27,30; H015 | Preserves business ownership and prevents uncontrolled loops. | None |
| `AD-007` | Delivery dead-letter separation | H02 delivery dead-letter isolation is C04-003-owned and distinct from C02 message/event dead-letter delivery. | L2-H02 §28 + C02 ownership | Avoids duplicate event-infrastructure ownership. | None |
| `AD-008` | Receipt normalization | Provider evidence becomes H02 evidence only after exactly-one-attempt association and mapping to supported H02 receipt types. | L2-H02 §31; H016 | Prevents provider-specific status from corrupting canonical delivery evidence. | None |
| `AD-009` | Credential/protection split | D03-004 owns provider credentials/certificates; D03-002 owns protected exchange; C04-003 only consumes them. | `ADR-L5-0002`; D03-002/004 | Prevents security lifecycle duplication. | `ADR-L5-0002` |
| `AD-010` | Configuration split | C04-003 owns provider/retry/failover meaning; generic configuration version/scope/inheritance/effective dating is A06-owned. | L2-H02 §§29–30,36; A06 | Prevents duplicate configuration platform architecture. | None |
| `AD-011` | Event publication split | C04-003 returns delivery outcomes; C04-001 owns Notification transition/publication-intent and C02 owns event delivery. | H015/H016 + C04-001/C02 boundaries | Prevents C04-003 from becoming a second H02 event publisher. | `ADR-L5-0010`, `ADR-L5-0011` constrain C02 |
| `AD-012` | Technology position | No provider vendor, SDK, API protocol, callback mechanism, secret store, crypto algorithm, queue/broker, database or numeric retry/timeout/rate-limit value is canonicalized here. | L5 Governance / ADR methodology | Maintains solution-architecture neutrality. | Required only if a later material canonical selection is made |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant / Institution isolation | Provider configuration, Delivery/Attempt/Receipt state and operational recovery remain bound to validated protected scope. | L2-H02 NP-07; `ADR-L5-0004` | `L5-A-04-002` |
| Provider credentials | Provider secrets/certificates/machine credentials remain scope-bound, controlled and absent from ordinary config/evidence/logs. | L2-H02 §35; D03-004 | `L5-D-03-004` |
| Protected provider exchange | Required transport/data protection cannot silently downgrade. | L2-H02 §35; D03-002 | `L5-D-03-002` |
| Authorization | Protected provider configuration/reprocessing actions consume D02 decisions; no universal operator role is invented. | H015 §6; D02 | `L5-D-02-004` |
| Provider independence | Provider-specific request/status/error vocabulary stays behind adapters. | L2-H02 §§29–31 | C04-003 |
| Delivery reliability | Each attempt is immutable; controlled retry/failover preserves identity/evidence and does not repeat source business action. | H015 | C04-003 |
| Evidence integrity | Receipt chronology and recovery lineage are reconstructable; D04 owns enterprise evidence persistence/integrity. | H015/H016 | `L5-D-04-001` |
| Operational observability | Health/latency/failure/retry/DLQ diagnostics go to E05 without becoming audit truth. | L2-H02 §34 | `L5-E-05-001` |
| Event/messaging integrity | H02 delivery retry/DLQ does not duplicate C02 event/message retry/dead-letter architecture. | C02 boundary | `L5-C-02-*` |
| Product neutrality | Vendor/API/SDK/queue/database/crypto choices remain unselected until governed. | L5 ADR methodology | Applicable owner |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | Business/platform domains shall not invoke communication-provider APIs directly; provider access shall occur only through C04-003 provider abstraction. | §5.5; `AD-001` | L2-H02 NP-05, §6 |
| `CR-002` | C04-003 shall consume rendered content, recipient eligibility and permitted channels from C04-002 and shall not re-run Template/Preference/quiet-hour/Emergency communication-policy decisions. | §§5.2, 4 | Frozen C04-002/003 boundaries |
| `CR-003` | Every Delivery shall reference exactly one canonical Notification; provider/channel retry or failover shall not create a replacement Notification. | §§5.3–5.4 | L2-H02 §§25–27; H015 |
| `CR-004` | Multi-channel execution shall preserve independent channel outcomes; one channel/provider failure shall not automatically fail remaining permitted channels. | §§5.2, 5.11; `AD-003` | L2-H02 §18; H014 |
| `CR-005` | C04-003 shall represent only the upstream H02 Delivery states `Pending`, `Queued`, `Dispatching`, `In Progress`, `Delivered`, `Failed`, `Cancelled`, `Expired`. | §5.3 | L2-H02 §25.3 |
| `CR-006` | Provider-specific statuses shall not become additional canonical Delivery lifecycle states. | §§5.3, 5.10 | L2-H02 §§25,31 |
| `CR-007` | Provider selection shall be configuration/policy driven and health aware; business domains shall never directly choose a concrete provider. | §§5.5–5.6 | L2-H02 §§17,29–30 |
| `CR-008` | Provider health shall use only the exact H02 states `Healthy`, `Degraded`, `Unavailable`, `Maintenance`, `Disabled` unless upstream architecture changes. | §5.6; `AD-005` | L2-H02 §29.5 |
| `CR-009` | Every concrete provider execution shall create one distinct Delivery Attempt associated with the existing Delivery/Notification. | §5.4; `AD-004` | L2-H02 §26; H015-BH-001/002 |
| `CR-010` | A completed Delivery Attempt shall be immutable; later provider responses/retries shall not rewrite prior attempt evidence. | §5.4 | L2-H02 §26; H015 |
| `CR-011` | Retry/failover shall preserve Notification identity and correlation and shall never repeat, reverse or compensate the source-domain business transaction. | §5.8; `AD-006` | L2-H02 §§6,27; H015-BH-004 |
| `CR-012` | Each retry/provider/channel fallback execution shall create a new Delivery Attempt; no previous attempt identity shall be reused. | §5.8 | L2-H02 §27; H015-BH-002 |
| `CR-013` | Recovery shall occur only where current H02 communication policy, Notification eligibility/expiry state and governed retry/failover configuration permit it. | §5.8 | H015; C04-001/002 boundaries |
| `CR-014` | Provider health may influence retry/failover routing but shall not change Notification identity or rewrite Delivery history. | §§5.6, 5.8 | L2-H02 §29.5 |
| `CR-015` | C04-003 shall not invent a provider-fallback, provider-switch, dead-letter or terminal-recovery canonical business event. | §§5.8–5.11 | H015-BH-005; L2-H02 §32 |
| `CR-016` | Exhausted/terminal delivery work may enter the logical H02 delivery dead-letter boundary without becoming C02 message DLQ state; controlled reprocessing shall preserve prior evidence. | §5.9; `AD-007` | L2-H02 §28; C02 ownership |
| `CR-017` | C04-003 shall return a provider-independent aggregate Delivery outcome that preserves all material per-channel evidence; it shall not hide failed channels behind a successful channel. | §5.11 | L2-H02 §18; H010/H014 |
| `CR-018` | Delivery failure, cancellation, expiry, retry or provider failover shall never alter source-domain business state or be interpreted as source-business success/failure. | §§5.8, 5.11 | H010, H014, H015, H016 |
| `CR-019` | C04-003 shall return delivery/recovery/receipt outcomes to C04-001; C04-001 retains Notification lifecycle transition and exact H02 lifecycle-event publication-intent ownership. | §5.11; `AD-011` | H015/H016; C04-001 |
| `CR-020` | Evidence shall preserve Notification/Delivery identity, every Delivery Attempt, failure classification, retry/failover lineage, receipt chronology and final provider-independent outcome. | §5.12 | H015-BH-006/007; H016-BH-006/007 |
| `CR-021` | Every provider Delivery Receipt shall reference exactly one existing Delivery Attempt. | §5.10 | L2-H02 §31; H016-BH-001 |
| `CR-022` | Delivery Receipts shall be append-only/immutable chronological evidence; later receipts shall not rewrite earlier receipts or attempts. | §§5.10, 5.12 | L2-H02 §31; H016-BH-002 |
| `CR-023` | Provider status evidence shall become canonical H02 receipt evidence only when it maps to a supported receipt type: `Accepted`, `Queued`, `Sent`, `Delivered`, `Read`, `Failed`, `Expired`, or `Rejected`. | §5.10; `AD-008` | L2-H02 §31; H016 |
| `CR-024` | Unsupported, ambiguous or unmatched provider evidence shall remain unresolved and shall not be attached heuristically or treated as authoritative delivery evidence. | §5.10 | H016-BH-004 |
| `CR-025` | `Accepted`, `Queued` and `Sent` receipt evidence shall not prematurely establish recipient delivery. | §5.11 | H016-BH-003 |
| `CR-026` | Provider secrets/certificates/machine credentials shall be consumed from D03-004, never embedded in Template content, ordinary provider configuration, logs or evidence. | §5.7; `AD-009` | L2-H02 §35; D03-004 |
| `CR-027` | Mandatory protected provider exchange shall consume D03-002 and shall not silently downgrade to an unprotected path. | §5.7; `AD-009` | D03-002 |
| `CR-028` | Provider/retry/failover configuration may consume A06 generic configuration mechanics, but C04-003 retains only communication-provider policy meaning and shall not duplicate A06 configuration architecture. | §§5.6, 5.8; `AD-010` | L2-H02 §36; A06 |
| `CR-029` | D04 evidence and E05 telemetry shall remain distinct; telemetry shall not substitute for immutable delivery evidence. | §§5.12–5.13 | D04/E05 boundaries |
| `CR-030` | No provider vendor, SDK, callback protocol, queue/broker, secret store, crypto algorithm, database, retry count, timeout, rate limit or numeric threshold shall become canonical through this specification without applicable governance. | `AD-012` | L5 Governance / ADR methodology |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Material C04-003 L4 sources `L4-H-010`, `L4-H-014`, `L4-H-015` and `L4-H-016` remain `Draft`; relations cannot be `Validated`. | Upstream / L4 | Current L4 lifecycle metadata | L4 governance / authorized architecture review | **Yes for Architecture Approved; No for this authorized Draft** | Approve applicable L4 behavior or issue explicit approval-level use authorization, then revalidate affected relations. |
| `OD-002` | C04-001, C04-002 and C04-003 are all present on canonical `main`; the family integration/reconciliation concern is resolved. | Resolved repository item / C04 family | Canonical repository state after family integration | Repository governance / architecture review | No | No further family-integration action is required; maintain synchronized cross-file contracts and ownership boundaries when any C04 specification changes materially. |
| `OD-003` | Exact communication-provider vendors, provider APIs/SDKs, status-ingestion mechanism, request/response formats and provider endpoint topology are unselected. | Technology / Implementation | No approved provider technology selection | C04-003 technical owner + Lead Architect where ADR is material | No for technology-neutral Draft | Select only through governed technology/ADR process if repository-wide or material. |
| `OD-004` | Numeric retry counts/backoff, timeout, rate-limit, provider-health thresholds and delivery SLAs are not defined canonically. | Configuration / Operations | No authoritative numeric policy/operational profile | H02 communication governance + E02/E05 / configuration owners as applicable | No for architecture | Supply through governed configuration/operational profiles; C04-003 shall not invent values. |
| `OD-005` | Exceptional human operational reprocessing actor/authority for H02 dead-letter recovery remains undefined upstream. | Upstream authority | H015 §6 / §10 | Upstream business/governance + D02 | No for ordinary Draft execution; blocks any claim of universal manual-reprocessing authority | Preserve evidence and require a valid D02/upstream authorization decision when such recovery is requested; do not invent actor/role. |

---

## 10. Traceability Summary

### Primary L4 Applicability

- `L4-H-010-BH-003`–`BH-007` — multi-channel delivery outcome/evidence and source-business isolation.
- `L4-H-014-BH-003`–`BH-007` — priority multi-channel execution with independent channel outcomes and source emergency ownership preserved.
- `L4-H-015-BH-001`–`BH-007` — failed Delivery Attempt classification, retry/failover, immutable attempt history, terminal recovery boundary and evidence.
- `L4-H-016-BH-001`–`BH-007` — provider-receipt association, normalization, immutable chronology and governed delivery outcome.

All current relations are **Specified**, not `Validated`, while the cited L4 sources remain Draft.

### Material Higher-Level / ADR Constraints

- `L2-H02_Notification_Workflows.md` §§17–18, 25–31, 34–36 — channels, multi-channel delivery, Delivery/Attempt/Retry/DLQ, provider abstraction/failover/health, receipts, monitoring/security/configuration.
- `ADR-L5-0002` — cryptographic protection, key/credential responsibility split.
- `ADR-L5-0004` — Tenant/Institution isolation.
- `ADR-L5-0006` — owner-local concurrency.
- `ADR-L5-0010`, `ADR-L5-0011` — C02 event-delivery/compatibility boundaries, consumed only for H02 lifecycle-event transport after C04-001 handoff.

### Material L5 Dependencies

- `L5-C-04-001` — Notification baseline/lifecycle/outcome/event handoff.
- `L5-C-04-002` — rendered content, policy-eligible recipients and channel/policy constraints.
- `L5-A-06-001/002` — generic provider/retry/failover configuration control/resolution.
- `L5-A-04-002` — Tenant/Institution context.
- `L5-B-02-003` — transaction/concurrency.
- `L5-A-08-001/002` — generic background/retryable technical execution where deferred delivery work requires it.
- `L5-D-02-004` — authorization enforcement.
- `L5-D-03-002` — cryptographic protection.
- `L5-D-03-004` — provider secrets/certificates/machine credentials.
- `L5-D-04-001` — audit/evidence.
- `L5-E-05-001` — operational telemetry.
- `L5-C-02-001` — canonical event transport after C04-001 publication handoff.

### Architecture Decisions

- `AD-001`–`AD-012`

### Conformance Requirements

- `CR-001`–`CR-030`

### Open / Blocking Items

- `OD-001` — Draft L4 sources block Architecture Approved status.
- `OD-002` — C04 family canonical repository integration is resolved; no repository blocker remains.
- `OD-003` — provider technology choices intentionally deferred.
- `OD-004` — numeric retry/timeout/rate-limit/health/SLA values intentionally deferred.
- `OD-005` — universal manual delivery-reprocessing authority remains upstream-defined and is not invented.


## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0033`  
Payment, banking/treasury and communication providers remain deployment/institution choices behind C03/C04 adapters. No repository-wide external-provider vendor is canonical; provider-specific semantics may not become business architecture.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
