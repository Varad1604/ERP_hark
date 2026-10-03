# ADR-B18-W4-D1-D2 — Receipt Quantity, Replacement and Finalized-Correction Contract

| Field | Value |
|---|---|
| Classification | **TECHNICAL DESIGN PROPOSAL — NOT LOCKED OR APPROVED AS A PRODUCTION POLICY** |
| Prepared | 2026-10-03 |
| Owner for disposition | Chetan — Section J / J05 and separately J06, with Dhruv cross-domain governance |
| Work item | Architecture #144; parent #140; frontend W4 #49 |
| Proposed level | Implementation-facing design proposal for downstream L4/L5 review, not an amendment to Locked L2/L3 |
| Existing authorization | Lead Architect's bounded *synthetic request-only* approval in `governance/adr/ADR-B18-W4-EXC-001_LEAD_ARCHITECT_APPROVAL_2026-10-03.md`; no blanket positive correction authorization |

## 1. Canonical constraints verified

- Locked `L2-J05_Goods_Receipt_Inspection_and_Acceptance.md` §§3–17: J05 owns physical receiving, receipt/inspection disposition, replacement/variance paths, `ReceiptVarianceRecorded`, finalized immutable history and conditional receipt-variance SoD. J04 alone owns PO commitment/amendment/closure; J06 stock ledger, J08 assets and Section F invoice/accounting remain independent.
- Locked `L3-J-049` §§7–15: delivery is against authoritative issued PO, variance and inspection have governed dispositions and accepted facts are consumed independently.
- Locked `L3-J-050` §§6–15: Goods Receiver/Inspector supplies attributable finalized-error evidence; independent Receipt Variance Approver confirms *permitted* correction; J05 appends supplemental receipt/inspection evidence and only on actual approval publishes its existing `ReceiptVarianceRecorded`. J06, J08 and Finance determine separate consequences.
- `L4-J-050` is **Review**, not an approved payload/API or finalized event contract. Its status line historically attributes document preparation to Pallav; controlling `TEAM_ASSIGNMENTS.md` assigns Section J architectural accountability to Chetan. Do not change the locked parent on that basis.
- The HOME W4 implementation `29338df00ccb957bde8be30f5265c0180b51822a` already implements a **fictional request-only** J05 pending correction. It must not masquerade as an independently approved `ReceiptVarianceRecorded`.

## 2. Proposed quantity model — decision D1

Maintain these independent nonnegative, unit-normalized values per exact (tenant, institution, issued PO version, PO line, item, approved supplier): `ordered_qty`, `physical_received_qty`, `accepted_qty`, `rejected_qty`, `returned_qty`, `replacement_qty` and `approved_excess_qty`. Values are **ledger projections from immutable, source-witnessed facts**, never caller-controlled counters. Inspect/receive lines have distinct immutable IDs; correct and compensate using supplementary facts rather than changing previous snapshots.

**Proposed default profile for real policy configuration:** `STRICT_NO_EXCESS` permits ordinary new delivery only if cumulative physical delivery does not exceed authoritative ordered quantity. Configure `approved_excess_qty=0` unless the J05 variance authority and J04 PO commitment policy explicitly permit a higher boundary. The existing fictional 6+4 received / 4+3 accepted example remains valid, but is neither a real institutional tolerance nor proof that the PO is closed or payable amounts aggregate automatically.

**Replacement problem requiring an explicit policy answer:** a rejected delivery physically arrived, so counting every replacement delivery against original gross arrivals may prevent legitimate completion. Do **not** silently subtract rejected or returned goods from `physical_received_qty` or assume a replacement auto-releases capacity. Proposal: classify replacement as a **separate, explicitly authorized** delivery linked to the exact rejected/returned receipt line and accepted supplier replacement disposition. Evaluate a distinct `replacement_entitlement_qty` against approved rejection/return facts and remaining unfulfilled ordered acceptable quantity. The proposed normal cap is `min(approved_rejected_or_returned_qty_not_previously_replaced, ordered_qty - net_accepted_qty)`; this is a candidate policy, **not an established locked formula**. Track original gross receipts, approved replacement receipts and net accepted position separately and preserve each source fact. If original rejected goods were retained or returned, record the actual disposition; do not presume physical reversal.

**Over-delivery:** deny by default; if governance enables it, J05's explicit variance decision and the independently authoritative J04 commitment policy must both allow the exact quantity/line/currency context. A PO revision and variance permission are independent; W4 may not issue/amend a PO on J05's behalf.

**Proposed eligibility and invariants:**
1. Source facts must match exact tenant, institution, transaction, PO ID/version, line, item, supplier and inspection/receipt identity, with time-valid status and units. No same-ID cross-scope alias.
2. Replayed same command plus original immutable signature returns the same witnessed outcome; changed-body replays are rejected. Concurrent receipt admission must be serializable at PO-line capacity boundary; **transaction mechanism is a design choice, not an existing implemented backend claim**.
3. Partial acceptance may coexist with rejected/damaged variance and more than one receipt; no acceptance may exceed actually inspected receiving for its receipt.
4. Approval policy version, approving actor/SoD decision ID, expiry and original PO-line capacity witness must be recorded with any exceptional receipt. Missing or stale evidence means BLOCKED, not optimistic acceptance.
5. J04 independently resolves commitment/closure; cumulative acceptance cannot automatically close a PO or trigger Treasury readiness.

## 3. Proposed finalized correction contract — decision D2

Separate a **Correction Case** (request and governed decision) from the original immutable `GoodsReceipt` and from the *existing* J05 `ReceiptVarianceRecorded` completed business fact. `Correction Case` is a **proposed implementation-supporting concept**, not a new canonical L2 aggregate or event. An adapter may instead use governed receipt supplemental evidence within the original aggregate if that better preserves canonical aggregate ownership.

**Suggested decision phases (internal design only):** `REQUESTED → EVIDENCE_VERIFIED → AUTHORITY_PENDING → APPROVED_FOR_SUPPLEMENTAL_RECORD → J05_CORRECTIVE_FACT_RECORDED`, with explicit `REJECTED` and `BLOCKED` terminal/holding dispositions. These are *not new canonical GoodsReceipt states*. The existing HOME mock only demonstrates `REQUESTED/PENDING_APPROVAL`. Do not emit a new canonical request event merely to represent these phases: locked A06 events represent completed business facts, not intentions.

**Minimum proposed request envelope** (schema illustrative, not an approved service API): `tenant_id, institution_id, transaction_id, correction_request_id, original_receipt_id, original_receipt_history_version, original_po_id/line/version, affected_lines[{original_receipt_line_id,item_id,quantity,unit,reason,condition,proposed_result}], error_originator_identity, immutable_original_inspection_ref, source_witness_refs[], evidence_refs[], initial_recorded_at`. Require a non-empty normalized rationale and affected quantities; identify whether this is quantity, inspection/disposition, goods return, substitution, or source-reference error.

**Before a positive supplemental J05 correction**, validate all of:
- Original finalised receipt and latest owner history are independently witnessed and still relevant; error provenance identifies actor, affected line, quantity, inspection outcome, reason and business impact. Do not mutate or delete original `GoodsAccepted`/rejection/inspection facts.
- The requested correction category, changed values and any overage/replacement treatment are permitted by the **actual institution's versioned** J05 policy; no fixed application-side tolerance or arbitrary `approved:true`.
- A04 supplies a verifiable applicable effective-dated authorization decision scoped to the precise operation, tenant, institution, PO/receipt and quantities; independent Receipt Variance Approver and applicable maker/inspector SoD are enforced. Do not invent an A04 endpoint/permission name.
- The authorized supplemental outcome is persisted once in the J05 consistency boundary, carries a stable case/correction identity, current source-version/approval witness, and references all affected original receipt facts.
- Only then J05 may publish existing `ReceiptVarianceRecorded` under approved registered event version and actual business completion. **Do not reuse `GoodsAccepted` to overwrite original meaning**; if policy requires a genuinely new acceptance fact, its specific established canonical meaning and schema must be verified first.
- Corrected source facts become available to J06, J08 and F14 **independently** with correlation and causation; no synchronous cross-domain mutation and no inferred acknowledgement that consumers already compensated.

## 4. Required consumer boundary and post-consumption cases

| Existing source situation | J05 responsibility | J06 / F14 responsibility |
|---|---|---|
| Corrective request only, no approved outcome | Preserve pending evidence, allow rejection/return to owner | **No** stock or liability change |
| Approved error in already-rejected and unconsumed receipt | Preserve original and source-approved corrected disposition | Consumers independently assess whether any new accepted fact is relevant |
| Approved decrease to previously accepted quantity already consumed by J06 | Append authorized corrective fact, identify original receipt reference | J06 evaluates reservation/movement/stock-adjustment preconditions and emits its **own** governed ledger consequence; never simply subtract and assume on-hand stock remains available |
| Approved change to accepted quantity already invoiced/recognized by F14 | Append authorized fact, preserve prior acceptance | F14 independently determines invoice match/liability interpretation and whether F04 adjustment is required; no J05-triggered payable mutation |
| Correction impacts both inventory and Finance | One J05 authorized factual correction | J06 and Finance process independently; each can be completed, blocked or awaiting evidence without rewriting the other owner's result |

**Partial failure rule:** an approved J05 supplemental fact does not become invalid because J06/F14 has not yet processed it. Report each consumer independently as `NOT_NEEDED / WAITING / APPLIED / BLOCKED / UNKNOWN` **observational projection categories**, not new canonical business states. If either consumer becomes uncertain, preserve its own deduplicated source reference and block dependent Finance/AP release decisions until genuine owner evidence resolves it.

## 5. Acceptance tests and policy decision register

Required fixture matrix: partial + rejected delivery; two under-PO receipts; unauthorized replacement; approved replacement under explicit fixture policy; policy version change between request and decision; excess with J05-only approval but no J04 commitment; cross-tenant/institution/PO-line collision; mutable history, duplicate request, command signature replay, concurrent quantity saturation; already-issued/consumed stock; F14 invoice recognized before/after J06; consumer partial failure; denied approval without true A04 evaluation; independent SoD; audit provenance and reset. Tests for *positive* approval and replacement are **design acceptance criteria only until the policy/interface is ratified**, not passing current coverage.

| Decision required | Proposed default | Final source-owner input needed |
|---|---|---|
| Net replacement entitlement | Independent approved replacement linked to rejected/returned original | Chetan/J05: eligibility, authority, quantity calculation and J04 implications |
| Excess allowance | Deny / zero unless both J04 authority and J05 approved policy | Institution policy ID/version and value/quantity/unit tolerance |
| Corrective scope | Supplement not overwrite, approved `ReceiptVarianceRecorded` only | J05 correction classes, authority decision evidence, precise permitted facts |
| Downstream stock | J06 separately evaluates adjustment | J06 approved adjustment reasons, available stock/reservations and owner evidence |
| Finance effect | F14 independently reassesses; F04 owns financial adjustments | See companion D2/D3 Finance proposal |
| Missing true permissions | Request-only / BLOCKED | Approved A04 authorization binding and policy data |

**Disposition:** PROPOSED. Chetan/J05 must explicitly adopt/change the quantity policy and positive correction contract, with distinct J06/F14 and cross-domain reviews. Lead Architect's prior approval applies to the **request-only synthetic** behavior already integrated into HOME, not the missing production terms. Do not change the locked J05/L3-J-050 texts or claim a runtime-approved transition before the required decision.
