# ADR-B18-W4-D1-D2 — Receipt Quantity, Replacement and Finalized-Correction Contract

| Field | Value |
|---|---|
| Classification | **TECHNICAL DESIGN PROPOSAL — NOT LOCKED OR APPROVED AS A PRODUCTION POLICY** |
| Prepared | 2026-10-03 |
| Owner for disposition | Chetan — Section J / J05 and separately J06, with Dhruv cross-domain governance |
| Work item | Architecture #144; parent #140; frontend W4 #49 |
| Proposed level | Implementation-facing design proposal for downstream L4/L5 review, not an amendment to Locked L2/L3 |
| Existing authorization | Prior bounded synthetic approval is recorded only on separate still-unmerged draft [PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143); it is not a canonical file in PR #148 and grants no positive production correction authorization |

## 1. Canonical constraints verified

- Locked `L2-J05_Goods_Receipt_Inspection_and_Acceptance.md` §§3–17: J05 owns physical receiving, receipt/inspection disposition, replacement/variance paths, `ReceiptVarianceRecorded`, finalized immutable history and conditional receipt-variance SoD. J04 alone owns PO commitment/amendment/closure; J06 stock ledger, J08 assets and Section F invoice/accounting remain independent.
- Locked `L3-J-049` §§7–15: delivery is against authoritative issued PO, variance and inspection have governed dispositions and accepted facts are consumed independently.
- Locked `L3-J-050` §§6–15: Goods Receiver/Inspector supplies attributable finalized-error evidence; independent Receipt Variance Approver confirms *permitted* correction; J05 appends supplemental receipt/inspection evidence and only on actual approval publishes its existing `ReceiptVarianceRecorded`. J06, J08 and Finance determine separate consequences.
- `L4-J-050` is **Review**, not an approved payload/API or finalized event contract. Its status line historically attributes document preparation to Pallav; controlling `TEAM_ASSIGNMENTS.md` assigns Section J architectural accountability to Chetan. Do not change the locked parent on that basis.
- The HOME W4 implementation `29338df00ccb957bde8be30f5265c0180b51822a` already implements a **fictional request-only** J05 pending correction. It must not masquerade as an independently approved `ReceiptVarianceRecorded`.

## 2. Proposed quantity model — decision D1

Maintain these independent nonnegative, unit-normalized values per exact (tenant, institution, issued PO version, PO line, item, approved supplier): `ordered_qty`, `physical_received_qty`, `accepted_qty`, `rejected_qty`, `returned_qty`, `replacement_qty` and `approved_excess_qty`. Values are **ledger projections from immutable, source-witnessed facts**, never caller-controlled counters. Inspect/receive lines have distinct immutable IDs; correct and compensate using supplementary facts rather than changing previous snapshots.

**Proposed safe default pending an institutional policy:** cap **ordinary-class** arrivals against the exact current issued J04 PO-line ordinary capacity, independently from separately approved replacement-class arrivals. Gross physical arrivals may exceed original ordered quantity only through independently authorized replacement (or approved excess), and never imply extra accepted commitment. Configure `approved_excess_qty=0` unless the J05 variance authority and J04 PO commitment policy explicitly permit a higher boundary. The existing fictional 6+4 received / 4+3 accepted example remains valid, but is neither a real institutional tolerance nor proof that the PO is closed or payable amounts aggregate automatically.

**Replacement problem requiring an explicit policy answer:** a rejected delivery physically arrived, so counting every replacement delivery against original gross arrivals may prevent legitimate completion. Do **not** silently subtract rejected or returned goods from `physical_received_qty` or assume a replacement auto-releases capacity. Proposal: classify replacement as a **separate, explicitly authorized** delivery linked to the exact rejected/returned receipt line and accepted supplier replacement disposition. Evaluate a distinct `replacement_entitlement_qty` against approved rejection/return facts and remaining unfulfilled ordered acceptable quantity. A possible institutional entitlement bound uses disjoint approved rejected/returned portions and remaining authorized acceptable commitment, but its formula is **not ratified**; until approved, deny positive replacements. Section 6 below supersedes any gross-arrival capacity implication in this section. Track original gross receipts, approved replacement receipts and net accepted position separately and preserve each source fact. If original rejected goods were retained or returned, record the actual disposition; do not presume physical reversal.

**Over-delivery:** deny by default; if governance enables it, J05's explicit variance decision and the independently authoritative J04 commitment policy must both allow the exact quantity/line/currency context. A PO revision and variance permission are independent; W4 may not issue/amend a PO on J05's behalf.

**Proposed eligibility and invariants:**
1. Source facts must match exact tenant, institution, transaction, PO ID/version, line, item, supplier and inspection/receipt identity, with time-valid status and units. No same-ID cross-scope alias.
2. Replayed same command plus original immutable signature returns the same witnessed outcome; changed-body replays are rejected. Concurrent receipt admission must be serializable at PO-line capacity boundary; **transaction mechanism is a design choice, not an existing implemented backend claim**.
3. Partial acceptance may coexist with rejected/damaged variance and more than one receipt; no acceptance may exceed actually inspected receiving for its receipt.
4. Approval policy version, approving actor/SoD decision ID, expiry and original PO-line capacity witness must be recorded with any exceptional receipt. Missing or stale evidence means BLOCKED, not optimistic acceptance.
5. J04 independently resolves commitment/closure; cumulative acceptance cannot automatically close a PO or trigger Treasury readiness.

## 3. Proposed finalized correction contract — decision D2

**Selected preferred model:** a J05-owned correction request/approval/supplemental-evidence sequence **within the existing GoodsReceipt consistency boundary**, with a stable request ID indexing immutable evidence, not a new canonical Correction Case aggregate or event. The existing HOME separate fictional request projection establishes no real second aggregate. Section 7 specifies the positive commit.

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

  
## 6. Controlling D1 remediation — independent physical-arrival, replacement and acceptance accounts

**Candidate implementation contract, not ratified institutional policy.** Preserve append-only exact source references and an original PO version on each physical arrival. Re-evaluate prospective admissions against the **current issued** J04 PO version and line, approved J02 supplier and correct tenant/institution. Maintain canonical approved item-specific units, conversion version and deterministic rounding (never caller-chosen). Each physical delivery/line must have a unique owner-witnessed arrival identity; accepted, rejected, damaged and returned labels must identify **disjoint affected physical portions**, not add the same rejected-then-returned unit twice.

| Proposed derived account | Purpose and invariant |
|---|---|
| O — ordered | Current J04-issued PO-line commitment/version; J04 alone amends/closes |
| N — ordinary gross arrivals | Independently witnessed ordinary physical arrivals and in-flight reservations |
| R — replacement gross arrivals | Independently witnessed approved replacement arrivals, never counted again as N |
| G — total gross arrivals | N + R + separately approved excess physical arrivals; **never reduced by a return/correction** |
| A — corrected net accepted | Prior J05 inspected acceptance and authorized signed supplemental dispositions; never exceeds physically eligible inspected portions or J04-authorized commitment plus separately permitted excess |
| Q — rejection/damage/return | Source-linked disjoint unit portions with explicit returned-versus-retained physical custody, not mechanically additive categories |
| T and C — replacement entitlement/consumption | J05 policy-authorized, original-disposition-linked disjoint eligible portions T; replacement arrival reservations/consumption C, always C <= T |
| E — explicit excess authorization | Requires **both** compatible J04 commitment/version authority and J05 variance permission, never inferred from a numeric UI tolerance |

**Separate admission rules and locking.** (1) Ordinary: proposed safe default N + prospective ordinary arrival <= O, counting atomic pending reservations; no E without independent J04 and J05 permission. (2) Replacement: require approved supplier replacement disposition linked to exact original rejection/return line, quantity portions, custody state, current PO-line/version, unit-conversion profile and J05 institutional policy version. Atomically reserve/consume C from eligible disjoint T. Replacement adds to G and R, not N. G may exceed O; A may **not** exceed the independently authorized commitment plus E. Candidate entitlement may be no more than independently approved eligible rejected/returned portions and any institution-policy-specified remaining acceptable PO capacity; **do not adopt a min formula as policy** or assume that rejected means physically returned. Missing ratified policy/consent is BLOCKED.

**Concurrency and corrections.** A PO amendment, two ordinary arrivals, two replacements claiming the same source portion, and a corrective request must serialize at an owner-controlled PO-line admission/version boundary (or approved equivalent atomic reservation). Recheck J04 current issued version, all J05 source history versions, existing in-flight reservations, physical arrival identity and overlapping original portions at commitment. A new command ID cannot create the same arrival twice: apply D4 business-intent admission, without suppressing legitimate distinct cumulative arrivals on one PO. The original gross arrival record is immutable even after later return, and an accepted-then-issued unit cannot be silently treated as still on hand. A version or identity mismatch denies/requires governed re-review.

**Positive illustrative policy fixture, not approved business entitlement:** O=10 ordinary arrivals N=10 with 3 disjoint rejected units; after an explicitly configured and approved T=3, one replacement arrival R=3 makes G=13/C=3 while corrected maximum A remains 10 absent independent E. Without approved T, replacement is blocked. Two racing workers must never collectively consume T more than once.

## 7. Controlling D2 remediation — preferred GoodsReceipt persistence and owner completion

Persist the immutable request ID, original finalised GoodsReceipt/line/history version, original and checked current J04 PO ID/line/version, original inspection/disposition witness, normalized affected portions, correction class and rationale, originator identity and evidence as **J05-owned supplementary request evidence within the existing GoodsReceipt consistency boundary**. No new canonical aggregate, state or request event. The existing HOME mock remains request-only. An independently authorized Receipt Variance Approver and true effective A04 decision/SoD must cover the current exact lines, quantities and policy/version; stale approval or version requires a new governed decision.

**Atomic positive commit (proposed):** compare-and-set the current GoodsReceipt history and shared PO-line admission/entitlement versions; verify actual permitted correction and original source references; atomically commit exactly one supplemental correction fact, advancement of the J05 history version, dedup/outcome identity and the existing **registered J05 ReceiptVarianceRecorded outbox fact**. Neither original finalised evidence nor original published events change. If the existing registered payload cannot faithfully carry the original/supplemental versions required by consumers, positive implementation is **BLOCKED for governed event-contract review**. A request/pending decision does not publish ReceiptVarianceRecorded. A06 owns delivery/replay; no second business publication or client-defined event is permitted. J05 success certifies **only J05**, never a dependent owner's completion.

| Post-correction scenario | Independent consequence, never J05's direct mutation |
|---|---|
| No downstream stock/monetary effect | Each applicable source owner attests NOT_NEEDED from its own history; absence of events is insufficient |
| J06 stock receipt already applied, still available | J06 independently evaluates permitted ledger correction, availability, reservation and immutable original stock facts |
| Issued, transferred, returned or reserved stock | **J07** independently owns movement/return/transfer/custody correction first where necessary; J06 consumes J07 facts and applies only its own ledger consequence; never assume negative stock |
| Already registered/assigned operational asset | **J08** alone determines supplemental asset correction, custody/condition and permitted lifecycle consequence; do not delete AssetRegistered |
| Asset also financially capitalized | **F12**, only where established J08/F12 contracts apply, determines financial asset accounting/depreciation implications; Finance alone handles its actual monetary posting |
| Invoice/liability/Payable previously consumed | **F14** separately reassesses original invoice/liability; **F04** only as required by its actual distinct adjustment cases in companion Finance proposal; **F08** separately re-evaluates current Payable/readiness |

Each independent owner returns owner-witnessed NOT_NEEDED, WAITING, APPLIED, BLOCKED or UNKNOWN as an **observational status**, not a new canonical lifecycle. Track exact source version, event identity, causation and owner-side idempotency; a successful J05 supplement is not a cross-domain transaction. D4 governs uncertain owner commands independently.

**Expanded proposed acceptance cases:** J04 version change during two concurrent ordinary/replacement admissions; source portion rejected-then-returned without double grant; duplicate physical arrival with distinct command IDs; missing conversion/rounding; request-only no event; positive supplemental+outbox atomicity and delivery delay; stale A04 approval/SoD denial; J07 stock already issued/transferred and negative-stock safeguard; J08 assigned asset and applicable F12 capitalization; F14/J06 independent partial failure; cross-tenant/institution/PO-line original-source mismatch.

## 8. Formal ADR fields — version 0.2, decision not recorded

**Target:** locked L2-J04/J05/J06/J07/J08, L3-J-049/-050 and downstream applicable L4/L5 contracts; **no locked document edited**. **Need:** previously contradictory gross-receipt/replacement admission and ambiguous correction consistency; missing institutional policy and positive permission. **Proposed decision:** sections 6–7 supersede conflicting earlier candidate language, subject to explicit source-owner/Lead acceptance. **Accountable owner:** Chetan/J05, affected J04/J06/J07/J08 owners and Dhruv/A04/A06; F12 and Finance reviewed separately. **Impact analysis:** J04 commitments, J05 canonical history, J06/J07 stock, J08 assets, applicable F12, F14/F04/F08 Finance, A04 SoD, A06 registry/outbox and A07 audit retain separate ownership. L4-J-050 is Review and its historic Pallav metadata contradicts controlling Chetan Section J roster; reconcile via separate governed L4 action, not silent edits.

### Decision

<!-- Intentionally blank. Human owner/review approval must identify exact version/commit and adopted policies/contracts. -->

### Acceptance checklist (production criteria remain unexecuted)

- [ ] Chetan adopts/revises explicit institution-scoped replacement entitlement, return custody, exact PO version, unit precision and excess decision.
- [ ] Actual A04/SoD and approved J05 correction class/event schema permit positive supplemental outcomes.
- [ ] Concurrent receipt/correction/replacement and J05 outbox atomicity tests pass against an approved backend contract.
- [ ] J07/J06, J08 and applicable F12, F14/F04/F08 independent consequences and denial/UNKNOWN cases are validated.

**Provenance:** Existing bounded Stage-5 fictional Lead approval is documented on still-unmerged separate [PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143); it does not approve these positive production contracts or create a canonical file in this branch.
