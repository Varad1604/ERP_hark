# ADR-B18-W4-D4 — Independently Verified Unknown-Command Recovery

| Field | Value |
|---|---|
| Classification | **TECHNICAL DESIGN PROPOSAL — NOT AN EXISTING BACKEND SERVICE OR APPROVED CONTRACT** |
| Prepared | 2026-10-03 |
| Decision owner | Dhruv / A04, A05, A06 and Lead Architecture, with the affected J05/F08 business owners |
| Tracking | Architecture #146; parent #140; frontend W4 #49 |
| Current approved safety | Fail closed: caller-asserted `verified:true` cannot clear a durable unknown-command retry lock |

## 1. Canonical architecture and missing interface

**Locked L2-A06** establishes immutable completed business facts, event registry-first governance, transactional outbox infrastructure, retry/DLQ/replay, event traceability and tenant isolation. A06 transports/recovers event publication and delivery; it does not decide whether a J05 business receipt or F08 payable action succeeded. **An absent outbox/delivery event alone cannot prove that a domain-owned business action was never applied.**

**Locked L2-A05 plus its 2026-09-23 Workflow Recovery Authority amendment** establishes A05 ownership of Workflow Instance recovery and approved continuation positions; the *workflow-owning domain* owns the truth/repeatability of the underlying business action. A05 cannot assume a domain outcome merely because its own checkpoint or an event-delivery attempt exists.

**Locked L2-A04** requires actual policy evaluation, institution/tenant scope, explicit deny precedence, auditable authorization decision and configured SoD. A UI-supplied `verified:true`, guessed permission, workflow role or impersonated verifier response is not a trusted A04 decision. **L2-A07** preserves enterprise audit evidence, not business-source outcome authority.

The current W4 mock rejects untrusted caller assertions and leaves an uncertain original command locked. Neither locked source specifies a **durable owner-outcome lookup** providing a verified `NOT_APPLIED` answer for an exact immutable domain command. This proposal defines a candidate interface for governed review; it **does not assert that a real endpoint, database, signature service or event topic exists**.

## 2. Safety objective and separated authorities

Never execute the original command again while its outcome is unknown. Separate:
1. **Source owner (J05/F08/etc.)** alone proves whether its domain action affected current owned history, including a durable result entry and relevant original aggregate version.
2. **A06** exposes reliable event-publication and transport evidence, never substitutes absence of publication for business nonapplication.
3. **A05 Workflow Engine** independently governs failed/recovering/running or cancelled workflow lifecycle and the last source-valid continuation point.
4. **A04** evaluates the requesting actor or authorized service's actual reconciliation permission and SoD; domain business approval remains with the source-owning domain.
5. **A07** durably preserves audit context, decisions, source proof, chronology, and who released a pending lock.

A single authenticated owner may supply positive outcome evidence, but an untrusted client field cannot impersonate that owner. Two-phase “check then retry” without durable fence/owner protection is unsafe because the original request may still finish after a `NOT_APPLIED` read.

## 3. Proposed abstract verifier contract

This is an **illustrative port/interface**, not a deployment URL, topic or approved event schema. Consumers request a read from the **authenticated source-owner boundary**, not generic A06 event-delivery history.

```text
readDomainCommandOutcome(
  scope = {tenantId, institutionId, ownerDomain, businessTransactionId,
           immutableCommandId, originalCommandSignature},
  requestContext = {callerIdentity, permissionDecisionReference,
                    correlationId, requestedAt}
) ->
  VERIFIED_APPLIED | VERIFIED_NOT_APPLIED | UNKNOWN | REFUSED
```

**Proposed verification record** (internal authenticated response only):

| Field | Meaning |
|---|---|
| `tenant_id, institution_id, domain_owner, aggregate_id, business_transaction_id` | Exact source-owned scope; never authorize from IDs without institutional context |
| `immutable_command_id, canonical_original_signature` | Exact original request; use stable canonical serialization and approved hash algorithm/version |
| `verification_result` | Strict result category defined below |
| `source_owner_identity, source_state_generation, owner_verification_record_id` | Trusted owner-side provenance; an opaque client-supplied name is inadequate |
| `source_history_reference, original_applied_result` | Present for APPLIED; points to the domain's currently independently witnessed original business outcome |
| `not_applied_fence_reference, fence_generation` | Mandatory for terminal NOT_APPLIED; durable fence prevents the original pending dispatch from later applying |
| `verified_at, evidence_validity, auth_context_ref, audit_correlation_id` | Source time, bounded validity, governance and audit references |
| `owner_verification_contract_version` | Registry/versioned binding of verified owner interface |

The source-owner service must obtain the proof from the authoritative business-state/command-deduplication boundary with the correct consistency guarantees. **Do not mandate a specific SQL table, network protocol, HMAC scheme, database, event registry payload or cloud service at this proposal level**. L4/L5 must independently assess deployment, cryptographic trust and operations before a production implementation can exist.

### Result semantics

- `VERIFIED_APPLIED`: source owner durably recognizes the *exact original signature and scope* as already applied, and returns its source-history-witnessed original outcome. Repeated command with that signature may return this existing result *read-only*, not re-execute the action. Any divergence or conflicting later owner history must be investigated; APPLIED must not silently assert that consumers (J06/F14/F08) completed their own consequences.
- `VERIFIED_NOT_APPLIED`: source owner proves no business effect for the *exact original command* **and durably invalidates/fences that command from any future late application** before presenting this result. Merely searching and finding no matching event/outbox row is **UNKNOWN**, not NOT_APPLIED. Proof needs an owner-controlled generation/lease boundary respected by all original command workers, including restored/late workers. Durable fence validity is not a caller-selectable TTL.
- `UNKNOWN`: source result is missing, inconsistent, racing, timed out, eventually consistent, stale, unverifiable, or the owner cannot guarantee a fence. Retain the lock and ask for authorized investigation. Unknown is **not a negative result**.
- `REFUSED`: request lacks true authorization, scope, source-version agreement or verifier validity; retain the lock, audit denial. Do not mask a refusal as NOT_APPLIED.

**Critical race test:** original worker pauses after acceptance but before visible event delivery; verifier reads. It must return APPLIED if effect is committed, UNKNOWN if not safely decidable, or VERIFIED_NOT_APPLIED only after the original worker is fenced from committing. The verifier must not return terminal nonapplication during an unfenced in-flight attempt.

## 4. Proposed recovery protocol and invariants

1. Before dispatch, durably record `PendingOriginalCommand` with exact immutable tenant/institution/owner/transaction/command/signature, source generation and intent. Ensure the writer cannot omit this record or silently replace its signature. This is a proposed coordination requirement, not an assertion about existing persistence.
2. On ambiguous timeout/crash/lost response, transition the local **technical recovery position** to `LOCKED_UNKNOWN` (implementation-side marker only; **not** a new J05/F08 business state or canonical event). Deny same-ID re-execution, changed signatures and duplicate different-ID requests purporting to repeat the same unresolved business intent.
3. Under an A04-governed request, consult the exact authenticated source owner. If no approved implementation port is configured, return UNKNOWN and keep the lock. Never create synthetic “source evidence” to achieve a green test.
4. If VERIFIED_APPLIED, reconcile local observational history to the independently witnessed original source result. Do not publish a new business fact or assume independent J06/F14/F08 effects; each downstream owner reconciles its own command and history separately.
5. If VERIFIED_NOT_APPLIED, verify current owner fence generation and true A04 clearance authorization, persist a **separate immutable reconciliation audit decision** and terminally retire the original command identity. Any subsequent authorized retry must use a **new** immutable command ID linked to original intent, exact scope, fence reference and approval. Never unlock/reuse the original identity. If fence expires, is revoked, cannot be confirmed or original intent has changed, return UNKNOWN/BLOCKED.
6. If UNKNOWN/REFUSED, preserve the existing indeterminate pending record and do not allow a new attempt under a cosmetically changed request ID; hold associated workflow continuation at the last valid source position. A05 recovery authorization cannot override an unknown business outcome.
7. A06 handles any separately confirmed event-delivery retry or replay **under its own event/subscription identity**; never translate A06 DLQ redelivery into a domain command retry or a new approval.
8. A07 records immutable request, evidence/result, policy version, decision identity, old/new command linkage, owner version/fence and chronology. No audit record substitutes for business owner truth.

### Recommended state-transition table (technical only)

| Existing recovery position | Evidence/decision | Proposed next position | Execution permission |
|---|---|---|---|
| `LOCKED_UNKNOWN` | Missing verifier, timeout, disagreement or A04 deny | `LOCKED_UNKNOWN` | NONE |
| `LOCKED_UNKNOWN` | True source APPLIED with exact original history | `RESOLVED_APPLIED` | Read original result only |
| `LOCKED_UNKNOWN` | Verified fenced NOT_APPLIED but reconciliation authorization missing | `LOCKED_AWAITING_AUTHORIZATION` | NONE; never release on mere source proof |
| `LOCKED_AWAITING_AUTHORIZATION` | Current owner fence + valid A04/domain decision + durable audit | `RESOLVED_NOT_APPLIED` | Authorize **new distinct** command only |
| Any | Scope mismatch, source contradiction, fence invalid or owner unavailable | `LOCKED_UNKNOWN` or `INVESTIGATION_REQUIRED` | NONE |

These labels are **proposed operational implementation markers**; they do not create canonical A05 Workflow Instance states or new event topics. The existing A05 `Failed → Recovering → Running/Cancelled` path continues to require separate workflow authority and a source-valid continuation position.

## 5. Proposed concurrency, security, and operational design

- **At-most-once owner effect per original command signature** is the business safety target; physical event delivery may retry under A06, but consumers deduplicate on proper event identity and source scope. Do not claim end-to-end exactly-once transport.
- Use a source-controlled atomic result/fence mechanism at the owner consistency boundary. The fence generation must be checked at the exact action commit boundary as well as during owner verification. If an original worker can commit without checking the fence, the proposed VERIFIED_NOT_APPLIED contract is **unimplementable** and must remain UNKNOWN.
- Canonicalize payloads at approved version; exclude untrusted presentation-only differences while preserving every business-relevant field. Reject hash collisions/version ambiguity and prohibit allowing the caller to choose the expected signature.
- Enforce tenant/institution isolation, resource context, actual A04 policy decision, effective delegation, explicit deny and configured SoD. Distinguish authorization to inspect outcome, reconcile local projection and **resubmit business intent**. These may be different permissions and approvers.
- Treat clock skew, long-running original workers, duplicated callback delivery, verifier outage, version migration, stale cache and shared owner outages as UNKNOWN until authenticated owner resolution. Cache must not return terminal NOT_APPLIED without a still-valid owner fence.
- Telemetry: surface unknown-command age, verifier health, mismatched signatures, stale fences, unauthorized attempts, denied second-command attempts, partial consumer reconciliation and tenant-scoped operational alerts. Alerting/metrics are operational design, not new canonical business events.
- If no authoritative source-owned verifier is implemented or permitted by current architecture, show an honest blocked/manual investigation UI and do not provide any positive retry action in the mock or production flow.

## 6. Executable design acceptance matrix

| Test | Required expected result |
|---|---|
| Caller sends `verified:true`, fake `sourceOwner` and `NOT_APPLIED` | DENY, original lock unchanged |
| Same command ID with changed quantity or institution | DENY and auditable signature/scope conflict |
| Different command ID attempts same unresolved original intent | DENY pending owner resolution |
| Owner APPLIED + exact signature/current source witness | Return original result read-only; no duplicate J05 receipt or F08 Payable effect |
| No published event exists but original business action may be in flight | UNKNOWN, never NOT_APPLIED |
| Original worker delayed while verifier tries to fence | Either block/complete original then APPLIED or fence it then NOT_APPLIED; **never both effects** |
| Signed-looking but unregistered/stale/wrong-domain/cross-tenant proof | REFUSED or UNKNOWN; no lock release |
| Genuine fenced NOT_APPLIED without current A04 decision | LOCKED_AWAITING_AUTHORIZATION |
| Genuine fenced NOT_APPLIED + correct A04/domain authority and immutable audit | Permit one new-ID intent referencing immutable prior command, never reusing original ID |
| Fence expires/revokes between read and retry | DENY new attempt until safe re-verification |
| J05 APPLIED with J06 consequence UNKNOWN | J05 result read-only; independently retain J06 lock and do not claim inventory done |
| F14 result UNKNOWN while F08 hold active | F08 remains BLOCKED; no settlement readiness |
| Cross-workflow recovery tries to force rerun | A05 cannot override unknown domain-owned result |
| Same authenticated verification repeated, crash before audit commit | Idempotent recovery decision; never duplicate business action |

**Positive** tests require a real **approved** owner verifier/fence implementation under a controlled fixture. Until one exists, only fail-closed negative tests are executable for the present HOME mock. Never fabricate a passing production verifier test by passing arbitrary in-memory `verified:true` into the owner kernel.

## 7. Decisions requiring explicit ratification

1. **Source proof implementation:** Can existing J05/F08 owner command-deduplication and history stores provide an atomic “read + durable fence” for the same immutable command? Identify actual owner service, authoritative persistence and approved consistency guarantee. If not, upstream platform work is required and this proposal remains blocked.
2. **Identity/version:** Agree exact required canonical signature algorithm/version, tenant/institution/transaction/aggregate dimensions, original command identity, data-retention period and owner history witness.
3. **Authorization:** Dhruv/A04 and affected domain owners specify actual review/recovery permission policy, source binding and SoD; A05 Workflow Recovery Authority remains independent and cannot decide business outcomes.
4. **Audit/transport:** A07 evidence identity and retention, A06 integration of publication/delivery retry distinct from domain result; no new canonical event without registry approval.
5. **Operational disposition:** Investigations when durable fencing is impossible (retain lock, review original source-owned state, potential manual downstream correction under separate business governance); never invent a “safe retry” waiver.
6. **Approval evidence:** Write accountable technical contract and deployment readiness separately from Lead's prior bounded approval. This design proposal is not that approval.

**Disposition:** PROPOSED. Current safe mock behavior (caller assertions denied, UNKNOWN lock retained) is already Lead-approved and implemented. Implement VERIFIED_NOT_APPLIED **only after** the source owner, A04/A05/A06/A07 and applicable domain owners ratify the contract and its anti-race fence; otherwise retain fail-closed recovery and leave the mandatory W4 positive scenario open.
