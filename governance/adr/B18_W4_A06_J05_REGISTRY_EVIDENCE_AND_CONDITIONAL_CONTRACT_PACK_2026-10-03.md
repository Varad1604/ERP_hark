# A06 / J05 — Registry Evidence and Conditional Contract Admission Pack

**Date:** 2026-10-03  
**Status:** EVIDENCE PACK / NOT ACTIVE EVENT DEFINITION  
**Pinned architecture baseline:** `84029c76947dcb7c542e7a9505f01dcc3191c1f3`  
**Related:** Architecture #152, #144, documentation-only merged PR #151 and frontend #67.

## Independently inspected evidence

- Locked L2-J00 §27.7 names `GoodsReceiptVarianceRecorded`; locked specialized L2-J05 §6, L3-J-050 and relevant downstream documents use `ReceiptVarianceRecorded`. Merged PR #151 contains a **PROPOSED**, not activated, versioned catalog clarification preferring the specialized name. J05 is the unambiguous semantic publisher.
- Locked L2-A06 EIP-05 requires an active registry definition before publication; L3-A-018 and L4-A-018 require a governed schema/version, approved registry record and actual activation. L3-A-060 requires immutable successor versions and consumer migration when changing an active contract. Approved L5 ADR-L5-0035 selects a CloudEvents-compatible JSON envelope and governed JSON Schema 2020-12 catalog, with Tenant/Institution, correlation/causation, source and version context where applicable.
- Architecture and current standalone frontend Git trees contain no inspected authenticated active A06 registry export for this event. A historical naming-collision report is not an event-definition catalog.
- A separately discovered connected repository, `DhruvNaik26/SutraERP`, is an older full-stack Rust application whose latest visible `main` commit was dated 2026-07-04. Its `backend/src/events/types.rs` enum has no J05 receipt-variance event, and `backend/src/events/bus.rs` uses a Tokio **in-memory broadcast bus** rather than the current approved governed A06 registry/NATS specification. Its checked-in database schema offers no verified J05 active registry. This is evidence **only about that older repository**, not a conclusion about any external deployed environment. Do not wire Batch-18 frontend to this backend, or claim it satisfies A06, without a separate authoritative integration decision.

## Registry lookup — required real output, not a generated sample

An A06-qualified operator must query the **actual authoritative registry** for *both* candidate names and attach authenticated record IDs/read-only export plus environment, governance source, query timestamp and actor. For each result, record definition ID, exact canonical name and meaning, J05 publisher identity, lifecycle/default status, immutable Event Version/Schema IDs and hashes, transport-independent envelope profile, authorization and approved reviewer provenance. Also capture the actual current subscriptions and supported versions for J06, J08, F14 and any other **registered** consumers; architectural interest alone is not a subscription. Do not invent an API URL or output values.

| Registry finding | Required governed action | Publication disposition |
|---|---|---|
| Existing Active `ReceiptVarianceRecorded`, compatible schema | Independently inspect immutable approved schema for original finalized receipt ID, original version, new supplemental fact/version, PO context, tenant/institution, authority and causation; obtain actual J05/A06 confirmation and subscribed consumer compatibility | Permit *design consideration* only after institution policy, J04 race protocol, A04/SoD and J05 atomic outbox acceptance; this file is not runtime authorization |
| Existing Active `GoodsReceiptVarianceRecorded` | Preserve live name; reopen proposed §27.7 clarification decision and examine name/semantic compatibility, versions and migrations under L3-A-060. Never rename historical publication | Keep positive J05 release blocked until approved source/registry synchronization |
| Neither name Active; registry exists | Follow L3-A-018: Draft definition → schema/version/contract → security and owner/consumer review → Approved → authorized activation → discoverability, then verify L3-A-019 delivery preconditions | No positive event before activation and separately approved receipt policy |
| No actual operational registry/verifiable export | Escalate A06 platform implementation/read-only verification dependency under Architecture #152. A mock contract cannot establish Active | BLOCKED; retain request-only and UNKNOWN protections |
| Existing definition but inadequate original/supplemental lineage | Retain immutable predecessor; propose versioned JSON Schema successor under L3-A-060 and prove each affected consumer compatibility/migration | BLOCKED until successor accepted and active |

## Conditional proposed contract-shape review matrix (not a released schema)

The owning domain and A06 should assess whether an existing schema provides all of the following, retaining actual published field names only after registry inspection: tenant and institution scope; canonical J05 publisher/source; immutable event/definition/version references; original receipt ID, line and committed original history version; immutable authorized supplemental corrective fact reference and version; original and current J04 source witness/version; event correlation and causation; true effective A04/variance-approver authority/policy provenance; recorded timestamp and approved auditable lineage; independently consumable idempotent source/economic-effect identity. No chosen field names, required set, numeric tolerances, authorization grants, actual JSON Schema or active version is asserted here. Consumer effects are separately owned and never implied by publication.

## Validation and transfer gate

1. **A06:** attach real registry export and exact defined status, version/schema/hash/owner plus verified subscription list. If no source exists, record an explicit backend/service gap and route the governed implementation before continuing.
2. **J05/Chetan:** compare original vs supplemental source semantics, approval chain and producer-outbox atomicity to a *real* schema; do not treat the Lead's reported owner review as a missing schema.
3. **Affected consumers:** J06/J08/F14 separately document actual version support and consequences, or `NOT_SUBSCRIBED` if verified. Include migration and independently tested projection acceptance if required.
4. **Decision:** apply proposed §27.7 amendment only when it agrees with authoritative registry and governed owner review. Resolve mismatches by new formal decision/version, not by rewriting locked history or fake aliases.
5. **Production validation:** after upstream #144 policy and J04/A04 dependencies are independently satisfied, implement on a separate source-owner branch and pass positive/negative crash, race, replay, type/build/browser and HOME acceptance. Full B18 C9 remains independent.

**Audited current disposition:** Repository evidence is sufficient to prepare the contract review but **insufficient to verify any Active J05 correction event**. This pack is safe to merge as evidence-only documentation; do not mark Architecture #152, #144 or frontend #67 done on its basis.
