# ADR-B18-W4-D1-D2 — Receipt Quantity, Replacement and Finalized-Correction Contract

| Metadata | Value |
|---|---|
| ADR/version | B18-W4-D1-D2 / v0.3 editorial consolidation (2026-10-03) |
| Status | **LEAD-ACCEPTED DESIGN DIRECTION; positive production contracts BLOCKED** |
| Target | Locked L2-J04/J05/J06/J07/J08 and L3-J-049/-050; applicable downstream L4/L5 contracts |
| Accountable disposition | Chetan (Section J / J05; separately affected Section J owners), Dhruv (A04/A06 and cross-domain governance); Finance owners/reviewers for downstream implications |
| Related work | Architecture #144, #140, draft PR #148; frontend W4 #49 |
| Prior authorization | Only the bounded fictional mock/fail-closed behavior recorded on separate **still-unmerged** [draft PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143); not positive production correction approval |

## Need

Locked J05 permits partial, cumulative, replacement and variance paths and immutable finalized correction but does **not** ratify a numerical institutional replacement/excess formula, source-approved positive correction categories, a production permission binding or a released corrective-event payload. The earlier draft conflated gross arrivals with ordinary-arrival capacity and offered two incompatible persistence choices. This ADR proposes one coherent design while explicitly containing missing owner policy and cross-domain contracts.

## 1. Canonical constraints verified

- Locked `L2-J05_Goods_Receipt_Inspection_and_Acceptance.md` §§3–17: J05 owns physical receiving, receipt/inspection disposition, replacement/variance paths, `ReceiptVarianceRecorded`, finalized immutable history and conditional receipt-variance SoD. J04 alone owns PO commitment/amendment/closure; J06 stock ledger, J08 assets and Section F invoice/accounting remain independent.
- Locked `L3-J-049` §§7–15: delivery is against authoritative issued PO, variance and inspection have governed dispositions and accepted facts are consumed independently.
- Locked `L3-J-050` §§6–15: Goods Receiver/Inspector supplies attributable finalized-error evidence; independent Receipt Variance Approver confirms *permitted* correction; J05 appends supplemental receipt/inspection evidence and only on actual approval publishes its existing `ReceiptVarianceRecorded`. J06, J08 and Finance determine separate consequences.
- `L4-J-050` is **Review**, not an approved payload/API or finalized event contract. Its status line historically attributes document preparation to Pallav; controlling `TEAM_ASSIGNMENTS.md` assigns Section J architectural accountability to Chetan. Do not change the locked parent on that basis.
- The HOME W4 implementation `29338df00ccb957bde8be30f5265c0180b51822a` already implements a **fictional request-only** J05 pending correction. It must not masquerade as an independently approved `ReceiptVarianceRecorded`.

## 2. Proposed D1 quantity and receipt-admission design

All quantities are immutable-fact-derived, nonnegative projections after approved **item-specific unit conversion and precision/rounding**; no caller-controlled counters. Partition by tenant, institution, issued PO identity, line/item and approved supplier, preserving the original J04 PO version on every historical receipt and checking the **current issued** J04 PO version for a prospective admission. Every physical arrival/line and relevant disjoint unit portion has an owner-witnessed immutable source identity. Do not add a unit to both rejected and returned totals just because the same rejected unit was later returned.

| Account | Meaning / accounting invariant |
|---|---|
| `O` | J04-issued original/current PO-line ordered commitment at the *checked version*; J04 alone amends/closes |
| `N` | **Ordinary-class** physical arrivals, including live ordinary-capacity reservations; never includes replacements or separately classified excess |
| `R` | **Replacement-class** physical arrivals tied to approved distinct original rejected/returned source portions and consumed entitlement |
| `X` | Separately approved *excess-class* physical arrivals with exact source authorization, not retroactively hidden inside N |
| `G` | **Lifetime gross physical arrivals = N + R + X** (excluding uncommitted reservations); actual arrivals are never subtracted after return/correction |
| `A` | **Corrected net accepted position:** approved J05 inspected acceptances plus authorized signed supplemental corrections, preserving original histories |
| `Q` | Source-linked rejected, damaged and returned portions with disjoint counting and explicitly witnessed retained/returned custody; labels may overlap, quantities must not |
| `T/C` | Source-approved **replacement entitlement** T for disjoint eligible original portions and **consumed/reserved entitlement** C; `0 ≤ C ≤ T` |
| `E` | Independently granted excess acceptance allowance tied to J04-compatible commitment and J05 variance decision; not a demo threshold |

**Separate proposed admission rules, not ratified institutional policy:**

1. **Ordinary:** under an issued J04 PO and valid J02 supplier, require `N + proposed ordinary quantity ≤ O` after counting atomic pending ordinary reservations. Without independently ratified J04-compatible excess and J05 variance authorization, reject excess. `G ≤ O` is **never** a general cap: approved replacements legitimately increase gross arrivals.
2. **Replacement:** identify the exact original rejected/returned receipt line, disjoint unit portions, supplier-replacement disposition, physical return/retention evidence, current issued PO-line/version and J05 policy/approval version. Require positive *source-owned* entitlement T, atomically reserve/consume C once per eligible portion and add each actual replacement to R/G, **not** N. A candidate entitlement could be limited by approved eligible rejected/returned portions and policy-defined remaining acceptable commitment, but its calculation is **not approved**. In the absence of explicit institutional entitlement policy or matching custody proof, the positive path is **BLOCKED**.
3. **Explicit excess:** any X or E additionally needs an independently authorized J04 commitment/version decision **and** J05's applicable variance authority for identical scope, item, units and quantities. A J05 decision alone never amends the PO. Without actual joint governance, excess is denied.

The corrected net acceptance A must be supported by source-witnessed eligible inspected portions and cannot exceed `O + E` at the applicable independently authorized commitment/version. A is **not** `G - rejected - returned`: overlapping dispositions and accepted-then-returned/issued units require authorized supplemental corrections and separate downstream consequences. A replacement may restore the acceptable position but cannot double-count the original rejected or already replaced portion.

**Atomicity and version boundary (proposed):** serialize independent overlapping and concurrent J05 arrivals, corrections and entitlement reservations at the PO-line admission boundary (or an equivalent proven owner-safe protocol) while comparing current J05 receipt/entitlement versions and independently authoritative J04 PO-line version. A J04 amendment racing a J05 commitment must cause safe revalidation/rejection unless an approved J04↔J05 version-reservation contract prevents stale admission; **no cross-service transaction is claimed to exist**. Canonicalize approved unit conversions before the capacity check. D4's owner-bound business-intent uniqueness prevents the same physical arrival under a cosmetically new command ID without conflating two genuine distinct deliveries sharing one PO.

**Illustrative positive fixture only:** O=10, ordinary N=10 with three disjoint rejected units; *only if institution policy explicitly grants* T=3 and supplier replacement is approved, R=3 gives G=13/C=3 and possible A=10 absent E. Without T, reject the replacement. Two competing arrivals cannot consume T=3 more than once. This is neither institutional policy nor a claim of deployed admission logic.

## 3. Proposed D2 immutable finalized-correction design

**Selected minimum persistence model:** J05 owns request, independent decision and supplemental evidence under the existing canonical **GoodsReceipt consistency boundary**. `correction_request_id` indexes append-only evidence; it does not establish another canonical aggregate, event, business state or published outcome. The HOME implementation is fictional **request-only**, with no positive production correction or `ReceiptVarianceRecorded` publication.

**Minimum internal request/evidence envelope (illustrative, not approved API):** exact tenant/institution/transaction; request/intent identity; original finalised receipt ID/line/history version and original PO/line/version; latest checked J04 issued version; affected original inspected physical portions, item/normalized unit/quantity/condition, reason, original immutable inspector/receiver evidence and downstream references; maker, witnesses and timestamps; proposed correction category and impact. Record versioned receipt policy and independent correction/variance approver/A04 SoD decision. Suggested internal case phases (requested, evidence verified, awaiting authority, approved for supplemental recording, recorded, rejected or blocked) are **not** new L2 GoodsReceipt states.

**Positive commit eligibility:** J05 must independently witness the original finalised receipt and newest J05 history; original maker/inspector error provenance and all immutable source references; actual institution-permitted correction category/quantity/return/replacement rule and policy version; true scope/effective-date A04 decision and independent Receipt Variance Approver with configured maker/inspector SoD; current J04 issued PO version and any shared capacity/entitlement reservation guard. Any stale, absent, conflicting or cross-tenant/institution witness blocks completion. Do not rewrite any original `GoodsReceived`, `GoodsAccepted`, rejection, inspection or audit fact.

**Atomic positive completion (candidate owner implementation):** compare current GoodsReceipt history and relevant owner-scoped admission/version witnesses; durably commit exactly one J05 supplemental corrective fact, corrected J05 history version and command/intent outcome with the **already canonical and registered `ReceiptVarianceRecorded` outbox fact** in one failure-atomic J05 boundary. J05 alone is the business publisher; A06 owns registered transport/replay and A07 immutable audit. If the currently approved event version **cannot** carry the exact original and supplemental version/source references necessary for consumer safety, positive publication and dependent implementation remain BLOCKED for governed contract review. Never improvise a changed payload/new event or publish a completed fact for a pending request. A fresh correction to the same original follows separately authorized source-version guards and cannot silently reuse the old decision.

**J05 success scope:** committed supplemental source fact and associated durable canonical outbox entry establish *only* J05 corrective completion. Independent consumer results are never implied by producer success; an absent event is never evidence of no effect. Correlate original and corrected source identity/version, event/causation and owner-side idempotency across boundaries.

## 4. Impact analysis and independent consumer routing

| Source consequence before J05 correction | Separate accountable outcome; safe missing-evidence behavior |
|---|---|
| No ledger, asset or financial effect | Each **applicable owner** positively certifies NOT_NEEDED against exact source versions; silence is not certification |
| J06 recognized stock still available | J06 independently decides allowed adjustment, on-hand, reservation and ledger consequence, never destructively reverses receipt |
| J07-issued/transferred/returned or reserved stock | **J07** alone executes governed operational issue/return/transfer/custody exception; **J06** subsequently applies its own stock-ledger and availability consequences. Never force negative stock to balance books |
| J08 already registered/assigned operational asset | **J08** independently assesses custody/condition and source-linked supplemental asset action; never delete original registration |
| Financial asset capitalization/depreciation already occurred | **F12 only when authoritative J08/F12 contracts apply**; it decides independent financial asset treatment and coordinates actual Finance posting owners |
| F14 recognized invoice/liability or F08 registered Payable | **F14** independently reassesses underlying vendor invoice/liability; **F04** only for the distinct applicable financial-adjustment cases in the companion ADR; **F08** separately assesses Payable/readiness after source-complete Finance evidence |

Per-owner observational reconciliation values may be NOT_NEEDED, WAITING, APPLIED, BLOCKED or UNKNOWN, **not** new canonical business states. Partial success remains visible: committed J05 correction plus blocked J07/J06 or F14 does not mean cross-domain completion. J04 alone governs PO closure, F10 Treasury alone settles, and F05 Accounting alone performs journal consequences. D4 separately governs uncertain command outcomes and duplicate intent.

## 5. Acceptance matrix and unresolved policy decisions

| Scenario to verify when an approved implementation exists | Required result |
|---|---|
| Two legitimate under-PO partial arrivals, distinct independently witnessed delivery IDs | Independent N and accepted facts; no accidental duplicate elimination |
| Same physical arrival via changed command ID, altered body or another institution | DENY/reconcile under D4; do not double-count G or change tenant |
| Rejected-then-returned original unit, overlapping damaged labels, partial replacement | One eligible source portion/entitlement consumption, preserved physical custody |
| O=10, N=10, three rejected, policy-authorized R=3 | G=13, A≤10 absent independent E; without policy, replacement BLOCKED |
| Two concurrent replacement claims, PO amendment during J05 receipt or correction | At most one version-valid capacity/entitlement outcome; missing J04↔J05 safe coordination BLOCKED |
| Different unit conversions, fractional precision or stale conversion profile | Canonical approved conversion/rounding or explicit DENY |
| J05 pending request, denied maker-checker, expired true A04 policy or stale original receipt | Immutable pending/rejected history, **no** completed business event |
| Authorized J05 supplemental commit and failed/delayed outbox delivery | Atomic source fact plus registered outbox or complete rollback; A06 retry only transports |
| Previously J07-issued stock/J08 asset/applicable F12 capitalization and F14 invoice | Separate governed effects/partial owner results; no negative stock, asset deletion or automatic liability mutation |

**Owner decisions still required:** Chetan/J05 must explicitly ratify exact replacement entitlement, original return/retention treatment, institutional unit/rounding/excess limits and correction classes; J04 owner must validate version/amendment/commitment handshake; actual A04 permission and SoD, J05 `ReceiptVarianceRecorded` registered version and affected downstream J06/J07/J08/Finance contracts must be verified. Positive cases are **design acceptance criteria, not executed tests**.

## Proposal, Decision and acceptance governance

**Proposal:** adopt §§2–4 only after the accountable owner resolves their stated policy/interface boundaries. These are downstream implementation-facing design inputs, not locked L2/L3 amendments by implication. **Further impact:** L4-J-050 remains **Review** and historically attributes owner metadata to Pallav whereas the controlling L2 `TEAM_ASSIGNMENTS.md` makes Chetan accountable for Section J. Reconcile this only through separate governed L4 review/metadata disposition, not a silent edit to a locked parent.

### Decision

**LEAD-LEVEL CONDITIONAL DESIGN-DIRECTION ACCEPTANCE — NOT PRODUCTION CONTRACT APPROVAL (2026-10-03).** Lead approves **D1/D2 design direction only** at reviewed v0.3 baseline `05b3db1...`, with reported off-platform collaborator consensus. The exact institutional replacement/excess policy, J04 concurrency/version handshake, actual positive J05 permission/SoD and registered correction-event schema remain **UNRESOLVED/BLOCKED**. No real positive correction/compensation is authorized without these facts.

**Provenance:** Dhruv's direct instruction and his report that collaborators approved off-platform, recorded in [Lead direction and reported review attestation](B18_W4_LEAD_DESIGN_DIRECTION_AND_REPORTED_REVIEW_ATTESTATION_2026-10-03.md). Individual GitHub reviews, precise unseen policy references, and backend evidence are not invented. This decision records the reviewed v0.3 baseline, not an unrestricted approval of unstated details.

### Acceptance checklist — currently unexecuted

- [ ] J05/J04 authoritative quantitative, institutional policy and version-reservation decisions recorded.
- [ ] Real positive J05 correction category, A04/SoD, current source guards and registered outbox schema ratified.
- [ ] Concurrent physical arrival/replacement, correction/outbox and all negative tests passed on approved backend.
- [ ] Independent J07/J06, J08 and applicable F12, F14/F04/F08/F10 outcomes verified with no inferred cross-domain completion.
- [ ] Required owner/Lead review and L4-J-050 ownership metadata governed.

**Approval provenance:** bounded fiction-only mock approval is on separate unmerged [draft PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143); it cannot approve positive production contracts here. This ADR is **PROPOSED**.
