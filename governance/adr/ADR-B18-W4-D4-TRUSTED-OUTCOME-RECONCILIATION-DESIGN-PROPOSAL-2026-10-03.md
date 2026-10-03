# ADR-B18-W4-D4 — Independently Verified Unknown-Command Recovery

| Metadata | Value |
|---|---|
| ADR/version | B18-W4-D4 / v0.3 consolidated proposal (2026-10-03) |
| Status | **PROPOSED; Decision deliberately blank; no deployed verifier claimed** |
| Target | Locked L2-A04/A05/A06/A07, A05 September 23 recovery amendment and affected J05/F08 source boundaries |
| Accountable review | Dhruv A04/A05/A06/platform/Lead, Chetan for separate J05/F08 domain consistency |
| Tracking | Architecture #146 and parent #140; frontend W4 #49 |
| Earlier approval | Bounded fail-closed fictional mock only, recorded on separate still-unmerged [PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143) |

## Need

Neither absence of A06 delivery evidence nor a caller assertion proves nonapplication. Earlier wording gave a temporary fence potentially revocable terminal significance and lacked cross-command stable business-intent deduplication. **No real source-owner verifier, permanent tombstone, unique intent mechanism or permission is established as deployed.** The sole safe present behavior without it is UNKNOWN, no positive retry.

## 1. Locked-source constraints

**Locked L2-A06** establishes immutable completed business facts, event registry-first governance, transactional outbox infrastructure, retry/DLQ/replay, event traceability and tenant isolation. A06 transports/recovers event publication and delivery; it does not decide whether a J05 business receipt or F08 payable action succeeded. **An absent outbox/delivery event alone cannot prove that a domain-owned business action was never applied.**

**Locked L2-A05 plus its 2026-09-23 Workflow Recovery Authority amendment** establishes A05 ownership of Workflow Instance recovery and approved continuation positions; the *workflow-owning domain* owns the truth/repeatability of the underlying business action. A05 cannot assume a domain outcome merely because its own checkpoint or an event-delivery attempt exists.

**Locked L2-A04** requires actual policy evaluation, institution/tenant scope, explicit deny precedence, auditable authorization decision and configured SoD. A UI-supplied `verified:true`, guessed permission, workflow role or impersonated verifier response is not a trusted A04 decision. **L2-A07** preserves enterprise audit evidence, not business-source outcome authority.

The current W4 mock rejects untrusted caller assertions and leaves an uncertain original command locked. Neither locked source specifies a **durable owner-outcome lookup** providing a verified `NOT_APPLIED` answer for an exact immutable domain command. This proposal defines a candidate interface for governed review; it **does not assert that a real endpoint, database, signature service or event topic exists**.



## 2. Ownership and recovery authority

Never execute the original command again while its outcome is unknown. Separate:
1. **Source owner (J05/F08/etc.)** alone proves whether its domain action affected current owned history, including a durable result entry and relevant original aggregate version.
2. **A06** exposes reliable event-publication and transport evidence, never substitutes absence of publication for business nonapplication.
3. **A05 Workflow Engine** independently governs failed/recovering/running or cancelled workflow lifecycle and the last source-valid continuation point.
4. **A04** evaluates the requesting actor or authorized service's actual reconciliation permission and SoD; domain business approval remains with the source-owning domain.
5. **A07** durably preserves audit context, decisions, source proof, chronology, and who released a pending lock.

A single authenticated owner may supply positive outcome evidence, but an untrusted client field cannot impersonate that owner. Two-phase “check then retry” without durable fence/owner protection is unsafe because the original request may still finish after a `NOT_APPLIED` read.



## 3. Illustrative trusted verifier interface (not an API)

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



## 4. Proposed owner-certified permanent terminal outcomes

**Proposed contract, not an implemented verifier:** the source owner must atomically establish exactly one of (a) original command business effect + persistent exact original command signature/outcome + relevant registered transactional outbox business fact, or (b) proven original non-effect + **non-expiring, non-revocable owner-side tombstone** (or equivalent permanent execution prohibition) for the exact original command/scope. Every original business-effect commit path—including delayed workers, old queue deliveries, failover, service restoration and data migration—must check that durable source-side decision at commit, not only at initial dispatch. Once a terminal NOT_APPLIED is certified, **no eventual original worker can commit the old ID**. Temporary owner-verification attestations, leases, A04 permissions and authorization sessions are *separate* expiring objects; their expiry blocks successor admission but never removes original tombstone.

A serializable owner-side outcome/fence decision must race safely with original workers: if original commits first, verification returns APPLIED with business witness; if the tombstone commits first, every original worker is forever rejected; if uncertain, UNKNOWN. Absence of an event/outbox row or eventually consistent read never proves nonapplication. Tombstone and signature/tenant/institution/aggregate/owner identity require durable owner-level uniqueness and an enforceable retention/migration prohibition on deletion while any late original can exist. Even after archival, preserve equivalent enforced terminal metadata across restores, backups, imports, software rollback, disaster recovery and owner replacement; if any execution path bypasses it, prohibit terminal NOT_APPLIED and positive retry.

**Multi-service crash recovery:** source-side owner outcome/tombstone is atomic **inside owner consistency**; do not fabricate one global transaction spanning A04 authorization, A07 audit and domain owner. An owner-certified terminal fence with A04 decision missing leaves LOCKED_AWAITING_AUTHORIZATION; an approved decision without durable A07 receipt or new-intent reservation still has **zero execution permission**. Recovered reconciliation must independently re-witness permanent owner tombstone, current A04/SoD, exact business eligibility and A07 attributable record, then obtain atomic owner successor admission. Crashes after tombstone/before authorization, after authorization/before audit, after audit/before admission or after admission/before dispatch re-enter safe reconciliation; never replay original. A05 workflow recovery and A06 event replay remain independent.



## 5. Proposed stable business-intent uniqueness across command IDs

An original immutable command ID/signature protects **same-ID** replay but not changed-ID duplicate physical/business effect. Each business owner therefore proposes a stable scoped **business-intent key** derived from tenant, institution, owning domain, aggregate/line, operation type, independently witnessed physical/source obligation identity and **expected authoritative aggregate/PO/Payable version**. A J05 intent distinguishes separate independently witnessed physical shipment/arrival identities on one PO but detects changed-ID submission of the *same* arrival/line; an F08 intent distinguishes a new governed hold/release decision from an unchanged dispute-case/current-Payable-version duplicate. A changed aggregate version **alone** cannot authorize physically re-processing the same shipment or already-decided payable effect. Exact key encoding/equivalence, source ID authentication, normalization and persistence are owner policy/contract decisions; clients cannot generate a second source shipment by inventing a label.

**Atomic owner admission:** record one active/terminal reservation per scoped true intent and current expected aggregate version with source-side uniqueness/serializable admission; bind one original command ID, canonical signature and immutable source version. Same ID/same signature returns original witnessed result if APPLIED or remains locked if UNKNOWN. Same ID/changed signature is refused. A different ID for the *same unresolved or APPLIED* intent is denied/read-only; never allow bypass by a cosmetic new ID, new browser, second worker or refreshed aggregate version. Legitimately distinct arrival/intent needs separate independently authenticated source evidence, not just a new UUID.

A **successor new ID** for the same intent is eligible only after source-certified permanent NOT_APPLIED tombstones the previous ID, a fresh current J05/F08 domain eligibility decision plus real A04/SoD permission exists, immutable A07 audit evidence is witnessed, and one atomic source-owner new-intent reservation binds one successor to the original. Competing successor candidates may not both reserve. The retired original tombstone remains permanent regardless of successor success/UNKNOWN/failure. A successor has its own immutable identity/outcome/fencing lifecycle. An unresolved successor blocks third-ID attempts for the same underlying intent. Each downstream owner deduplicates on its own exact source correction, event and economic-effect lineage without assuming the upstream owner made the downstream change.

**Required concurrent/crash acceptance:** two original workers vs owner terminal certification, failover and restored late original after a temporary lease expires; archive/tombstone migration plus restore/repeated same old-ID execution; same ID changed-body/cross-institution; new ID same unresolved original and same APPLIED original; three simultaneous successor IDs after approved NOT_APPLIED (at most one admitted); authorization/audit/re-reservation crash points; original source-intent collision with different physical delivery reference forged by client; two legitimate independent cumulative shipments sharing one PO; A05 workflow recovery trying to circumvent source UNKNOWN; A06 event redelivery without domain command re-execution. Any inability to enforce durable original prohibition or stable intent identity means **UNKNOWN/BLOCKED, no positive retry**.



## 6. Safe reconciliation and successor admission protocol

1. Before dispatch, durably record `PendingOriginalCommand` with exact immutable tenant/institution/owner/transaction/command/signature, source generation and intent. Ensure the writer cannot omit this record or silently replace its signature. This is a proposed coordination requirement, not an assertion about existing persistence.
2. On ambiguous timeout/crash/lost response, transition the local **technical recovery position** to `LOCKED_UNKNOWN` (implementation-side marker only; **not** a new J05/F08 business state or canonical event). Deny same-ID re-execution, changed signatures and duplicate different-ID requests purporting to repeat the same unresolved business intent.
3. Under an A04-governed request, consult the exact authenticated source owner. If no approved implementation port is configured, return UNKNOWN and keep the lock. Never create synthetic “source evidence” to achieve a green test.
4. If VERIFIED_APPLIED, reconcile local observational history to the independently witnessed original source result. Do not publish a new business fact or assume independent J06/F14/F08 effects; each downstream owner reconciles its own command and history separately.
5. If VERIFIED_NOT_APPLIED, re-witness the non-expiring original tombstone and true A04 clearance authorization, persist a **separate immutable reconciliation audit decision** and terminally retire the original command identity. Any subsequent authorized retry must use a **new** immutable command ID linked to original intent, exact scope, fence reference and approval. Never unlock/reuse the original identity. The original owner-side **terminal fence/tombstone must never expire or revoke**. A transient token/lease or authorization expiry prevents new admission until fresh clearance but cannot re-enable the retired command; if permanent protection cannot be verified, retain UNKNOWN.
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



## 7. Security, migration and operational safeguards

- **At-most-once owner effect per original command signature** is the business safety target; physical event delivery may retry under A06, but consumers deduplicate on proper event identity and source scope. Do not claim end-to-end exactly-once transport.
- Use a source-controlled atomic result/fence mechanism at the owner consistency boundary. The fence generation must be checked at the exact action commit boundary as well as during owner verification. If an original worker can commit without checking the fence, the proposed VERIFIED_NOT_APPLIED contract is **unimplementable** and must remain UNKNOWN.
- Canonicalize payloads at approved version; exclude untrusted presentation-only differences while preserving every business-relevant field. Reject hash collisions/version ambiguity and prohibit allowing the caller to choose the expected signature.
- Enforce tenant/institution isolation, resource context, actual A04 policy decision, effective delegation, explicit deny and configured SoD. Distinguish authorization to inspect outcome, reconcile local projection and **resubmit business intent**. These may be different permissions and approvers.
- Treat clock skew, long-running original workers, duplicated callback delivery, verifier outage, version migration, stale cache and shared owner outages as UNKNOWN until authenticated owner resolution. Cache cannot independently certify terminal nonapplication without a durable owner tombstone that permanently blocks the original; transient freshness gates successor admission only.
- Telemetry: surface unknown-command age, verifier health, mismatched signatures, stale fences, unauthorized attempts, denied second-command attempts, partial consumer reconciliation and tenant-scoped operational alerts. Alerting/metrics are operational design, not new canonical business events.
- If no authoritative source-owned verifier is implemented or permitted by current architecture, show an honest blocked/manual investigation UI and do not provide any positive retry action in the mock or production flow.



## 8. Acceptance matrix (positive cases unexecuted)

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
| Transient verification token/lease expires between proof and successor admission | DENY successor until reverified; original permanent tombstone remains effective |
| J05 APPLIED with J06 consequence UNKNOWN | J05 result read-only; independently retain J06 lock and do not claim inventory done |
| F14 result UNKNOWN while F08 hold active | F08 remains BLOCKED; no settlement readiness |
| Cross-workflow recovery tries to force rerun | A05 cannot override unknown domain-owned result |
| Same authenticated verification repeated, crash before audit commit | Idempotent recovery decision; never duplicate business action |

**Positive** tests require a real **approved** owner verifier/fence implementation under a controlled fixture. Until one exists, only fail-closed negative tests are executable for the present HOME mock. Never fabricate a passing production verifier test by passing arbitrary in-memory `verified:true` into the owner kernel.



## 9. Accountable decisions and implementation blockers

1. **Source proof implementation:** Can existing J05/F08 owner command-deduplication and history stores provide an atomic “read + durable fence” for the same immutable command? Identify actual owner service, authoritative persistence and approved consistency guarantee. If not, upstream platform work is required and this proposal remains blocked.
2. **Identity/version:** Agree exact required canonical signature algorithm/version, tenant/institution/transaction/aggregate dimensions, original command identity, data-retention period and owner history witness.
3. **Authorization:** Dhruv/A04 and affected domain owners specify actual review/recovery permission policy, source binding and SoD; A05 Workflow Recovery Authority remains independent and cannot decide business outcomes.
4. **Audit/transport:** A07 evidence identity and retention, A06 integration of publication/delivery retry distinct from domain result; no new canonical event without registry approval.
5. **Operational disposition:** Investigations when durable fencing is impossible (retain lock, review original source-owned state, potential manual downstream correction under separate business governance); never invent a “safe retry” waiver.
6. **Approval evidence:** Write accountable technical contract and deployment readiness separately from Lead's prior bounded approval. This design proposal is not that approval.

**Disposition:** PROPOSED. Current safe mock behavior (caller assertions denied, UNKNOWN lock retained) is already Lead-approved and implemented. Implement VERIFIED_NOT_APPLIED **only after** the source owner, A04/A05/A06/A07 and applicable domain owners ratify the contract and its anti-race fence; otherwise retain fail-closed recovery and leave the mandatory W4 positive scenario open.

  


## 10. Formal ADR governance

**Target, need, proposed choice, impacts and required owner review** are set out above. §§4–6 are the sole controlling proposed safety contract; no approved deployment or positive recovery is inferred.

### Decision

<!-- Deliberately blank for named human architecture approvals against exact commit. -->

### Acceptance checklist (positive implementation unverified)

- [ ] Real domain owner verifier/tombstone, signature/intent identity uniqueness, retention migration and atomic commit mechanism approved.
- [ ] Source-owned canonical key and A04 permission/SoD scope with separate A05 workflow recovery contracts ratified.
- [ ] Positive concurrency/failover/late worker/crash/repeated successor tests show no original or duplicate new-ID effects.
- [ ] A06 event replay does not re-run domain command; A07 immutable audit evidence separately witnessed.
- [ ] Without demonstrably deployed owner verifier, UNKNOWN remains locked and positive recovery prohibited.

**Approval provenance:** separate draft [PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143) covers bounded synthetic fail-closed behavior only, not a trusted backend verifier.
