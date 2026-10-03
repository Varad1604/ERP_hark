# ADR-B18-W4-D2-D3 — Independent Finance Compensation and Governed Payable Hold/Release

| Field | Value |
|---|---|
| Classification | **TECHNICAL DESIGN PROPOSAL — NOT A LOCKED SOURCE OR APPROVED PRODUCTION PERMISSION CONTRACT** |
| Prepared | 2026-10-03 |
| Decision owner | Chetan, separately for F14/F08 and affected Section J/J06; Finance independent review: Varad and Devang; platform A04: Dhruv |
| Tracking | Architecture #145 (with J05 #144), parent #140, frontend W4 #49 |
| Demonstrated scope | Bounded visibly fictional documentary AP hold/release, validated in HOME; no material source-dependent release |

## 1. Source-backed rules and unsatisfied detail

**Verified Locked**:
- `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md` §§1–6: F14 consumes independent Section J facts. F14 **alone** owns and publishes `VendorInvoiceMatched`, `VendorInvoiceApproved` and `VendorLiabilityRecognized`. The last is the single canonical handoff to F08.
- `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md` §§1–5: F08 begins only after F14 liability recognition and owns `PayableRegistered`, `PayableValidated`, Payable aggregate, suspension and payment readiness. Treasury alone owns actual settlement, Accounting its journal consequences. No cross-owner mutation.
- Locked `L2-F08_Accounts_Payable_Management.md`: preserves compensating rather than destructive changes; suspended Payables cannot progress to payment readiness/settlement. It lists `PayableSuspended` and `PayableReopened`; verify their **precise existing semantics** before reusing them in any new technical path.
- Locked `L2-F04_Financial_Adjustment_Management.md` §29.1: a correction or reversal is a **new** Financial Adjustment instance with its own approval lifecycle and immutable original reference; adjustment publication/application is not implied by creation or referral.
- Locked `L3-F-046`: AP may maintain hold, resolve with governed evidence or refer to source owner. `L3-F-047`: AP incorporates only independently completed authoritative F04/F14/J corrective facts. `L3-F-048`: AP write-off is referral only until F04 approves/applies it.
- `L4-F-046` is **Draft** and not an approved API, role grant or policy. The existing HOME W4 fictional documentation-only hold uses SUSPENDED/BLOCKED and subsequently VALIDATED/PENDING. That is a **mock review example**, not evidence that the exact real `PayableReopened` event should be fired from SUSPENDED.
- The Lead Architect's bounded approval permits only the fixed synthetic documentary demonstration and separate requests; it does **not** approve a real material-dispute release or create real Finance authorization.

**Gap to resolve:** the Locked documents define ownership and business outcomes but do not supply institutional numeric matching thresholds, real role assignments/SoD configuration, correction payload/API, or exactly how F14 compensates a consumed invoice/liability for each J05 correction type. Treat these as proposed technical choices.

## 2. Independent causal chain and no-automatic-compensation principle

Represent each participating owner as an independently validated, versioned and audited step, not a single mutable purchasing–Finance transaction.

`J05 corrected source fact (only if independently approved) → [J06 stock consequence independently evaluated] AND [F14 corrected invoice/liability interpretation independently evaluated] → [F04 separately approved adjustment, only where required] → [F08 updated Payable assessment]`.

These arrows are **causal dependencies**, not authorization transfers, synchronous cross-domain writes or a guarantee that every correction implies a new Finance adjustment. A J05 correction may affect only inspection description without quantity or monetary implications; it must **not** automatically reduce an already recognized liability. An inventory change need not imply a monetary adjustment. No F08 operation may publish an F14 or F04 fact.

### Proposed source-correction dependency witness

A read-only, exact-source observation may include:
`tenant/institution, transaction/PO/receipt line, original J05 witness, approved J05 correction witness, J06 consequence {not needed|witnessed|waiting|blocked|unknown}, F14 invoice/liability consequence {not needed|witnessed|waiting|blocked|unknown}, applicable F04 adjustment reference/approved/applied stage, F08 Payable current-version witness, audit correlation/causation`.

These fields describe an **illustrative composite read model**, not an approved shared aggregate or existing backend API. Each status must be derived from the owning history only; do not treat absence of a consumer event as proof of `NOT_NEEDED`. That determination requires the appropriate owning policy/result. A partial state (J06 corrected, F14 unknown) is **not** a synchronized completed correction.

### Proposed monetary-evidence rule

F14 may reconsider recognized monetary exposure only on source-authoritative receipt/PO/vendor/invoice facts, true Finance matching/approval policy and exact original invoice/liability identity; determine whether it must re-match, reject or issue a separately governed revised Finance interpretation. Distinguish a new ordinary F04 adjustment arising from changed authoritative facts from correcting/reversing an **earlier published F04 adjustment**. The canonical Reversed/Corrected Adjustment Reference always points to that published F04 original, **not** an F14 invoice or liability. See controlling §7. Reconcile F14 and F04 without double-posting or aliasing their canonical event publishers. Never simply subtract quantity times a fixed demo price from the Payable, directly mutate outstanding liability from a J05 fact, or infer a credit-note authority from receipt discrepancy alone.

**Chetan/F14 + Varad/Devang review decision required:** exact invoice matching tolerances, adjustment applicability matrix for received/accepted/rejected/damaged/returned/substituted quantity or invoice amount changes, currency/tax/discount effects and whether any revised liability publication is permitted using established event registry. If no canonical event covers a required final business fact, propose an upstream governed addition instead of inventing event semantics locally.

## 3. Proposed F08 dispute / hold and resolution design — D3

### Entry evidence and authorized state

An existing F08 Payable is history-witnessed, traces to the *same* F14 `VendorLiabilityRecognized` identity and exact tenant/institution/vendor/invoice/amount/currency, and is in a lifecycle position supported by the real F08 suspension policy. For the current bounded W4 mock, use the already validated Payable as the fixed entry case. A disputed/material amount whose accounting or settlement state cannot be independently witnessed is **BLOCKED**, not assumed eligible for AP-only resolution.

**Proposed review-case concept (not a new canonical F08 aggregate/event):** internal hold case stores exact Payable current version; source reason category; submitted evidence; maker and separate reviewer identities; A04 authorization decision IDs/policy version; source-owner references; and an immutable sequence of hold, review, maintained, referral and release decisions. Do not add a new canonical state merely to track review progress. Persist the case within an implementation boundary compliant with F08 aggregate ownership.

### Allowed resolution branches

| Source/evidence condition | Proposed F08 decision | Mandatory safeguard |
|---|---|---|
| Known vendor-document issue confined to F08 review; validated proof of AP-owned documentary completion and true AP authorization | Remove only the F08-owned hold under governed review; **reevaluate** readiness separately | A04 verified actor scope and applicable maker/checker; no automatic READY or settlement |
| External J05/J04/F14 discrepancy unresolved or source result UNKNOWN | Maintain SUSPENDED / BLOCKED, with documented referral | Caller-provided correction flags and mock references cannot override owner absence |
| Real J05 corrective fact exists; F14 still evaluating invoice/liability | Maintain hold or non-ready AP position | A J05 approval alone cannot stand in for F14 |
| F14 and, where required, F04 approved/applied consequences are independently witnessed and the current Payable is reconciled | Permit separately authorized F08 review against latest owner witnesses | Distinct versioned decision; maintain if inconsistent or settlement obligations unresolved |
| Already partially/fully settled, legal constraint, conflicting audit hold or active institution policy deny | Do not reuse documentation-only release; refer to source/Treasury/legal governed process | No reversal of settlement reference or retroactive change without owning process |
| F04 write-off merely requested or proposed | Maintain current Payable and record referral | Write-off not claimed until F04 independently approved/applied |

The first row's mock release currently restores **VALIDATED/PENDING** only. For real policy, `L2-F08` describes a `Suspended → Reopened` lifecycle; **a real SUSPENDED → VALIDATED transition must not be asserted without resolving this source-level semantics distinction.** Proposal: keep `VALIDATED/PENDING` as a tagged fixture projection, while Chetan decides the actual `Reopened` lifecycle operation and applicable canonical event meaning for production. Never label fictional fixture semantics as the authoritative F08 lifecycle.

### Authorization and source validation

Before a real hold or release, evaluate an A04 decision under effective role assignment/policy/SoD; record immutable authorization decision ID, matched scope and policy version. Local UI checkbox, `SIMULATED-` document, caller `approved:true` or inferred staff role is not authorization. Hold review must reference the exact latest Payable version; concurrent source changes invalidate stale release evidence. A document-only release requires *explicit proof* that no source-side material discrepancy exists; absence of a correction event is insufficient evidence that no correction is necessary.

**No Treasury execution:** a successful F08 readiness evaluation is not a payment order, payment release, bank confirmation, journal posting or settlement. Ensure the UI labels and audit logs distinguish AP eligibility, payment authorization and actual settlement.

## 4. Failure, recovery and versioning

1. F08 and F14 each maintain their own durable idempotency and source-version checks. Duplicate same-signature correction/hold requests return the previous witnessed result; changed payload or cross-scope collision is denied.
2. If F14/F04 source outcomes are pending/contradictory/externally uncertain, F08 remains non-ready. Technical uncertain-command resolution is governed by the companion D4 proposal, not ad hoc Finance retry.
3. After a held Payable's version or source liability changes, prior release approval **must not silently apply** to a new version. Reevaluate exact policy and authority.
4. All prior business facts remain immutable; append corrected facts only in canonical owner. F08 records prior/resulting liability/readiness and source correlation but no cross-owner mutation.
5. If real Finance adjustment is needed, F04's forward-only adjustment lifecycle applies; even when the original financial adjustment was published, its state is never destructively rewritten.
6. Preserve institution/tenant isolation at read, command, audit, event and replay boundaries, including source-owner and Finance references. Never infer rights from a payable ID alone.

## 5. Proposed integration-test matrix

- Original validated Payable → independent F08 documentary hold → source-confirmed documentary release → **non-ready until separate readiness assessment**.
- Same source facts, two repeated exact hold/release command IDs, changed-signature replay and concurrent source/Payable version change.
- Unsupported or untrusted actor, maker/checker collision, expired delegation/policy, conflicting explicit deny, wrong tenant/institution/vendor/invoice or amount/currency.
- Positive J05 receipt correction **only after ratified D1/D2 policy**; independent J06 and F14 successes; J06 done/F14 delayed; F14 done/J06 delayed; F04 needed vs F04 unnecessary with authenticated owner disposition.
- F14 amount changed after payable registration: F08 stays blocked while awaiting authoritative liability reconciliation.
- Fraudulent invoice/PO source match, rejected/returned quantity overlap, material issue disguised as documentary-only, absent source fact and F04 write-off request masquerading as approval.
- Prior settlement/partial settlement: deny simplistic release, preserve Treasury references and route to owning exception process.
- Cross-scope and changed-body replay, tampered history, crash after F14 before F08, recovery UNKNOWN, repeat browser reset.
- Independent audit of hold/maintain/referral/release, prior/resulting versions, applicable permission proof, explicit *no payment/GL side effects*.

## 6. Decision register and gated acceptance

| Item | Proposed design direction | Required ratification |
|---|---|---|
| F14 post-J05-correction interpretation | Reevaluate from approved source fact and exact original invoice/liability; no default monetary delta | Chetan/F14 with independent Varad/Devang Finance review; exact cases and event semantics |
| F04 effect | New independent correction/reversal adjustment only when policy requires; original immutable | Chetan/F04 and independent Finance review; actual lifecycle and accounting treatment |
| Material Payable hold | F08 block by default until complete owner evidence | Chetan/F08 policy and true A04 permission binding |
| AP-owned documentary release | Explicit authorized review and separate readiness assessment | Chetan: real `Reopened` vs mock `VALIDATED/PENDING` compatibility decision |
| Material-source release | Only after independently witnessed complete source correction and Finance effect | Chetan/F08/F14/J05 and relevant J06/F04 owners; peer review |
| Role grants/SoD | Enforce actual A04, not synthetic flags | Dhruv/A04 authorized policy mapping |
| No actual correction contracts | Continue already validated fixed synthetic hold and **deny** all material release | Current Lead-approved fixture; not full W4 certification |

**Disposition: PROPOSED, unmerged and non-authoritative.** No false claim that F14 publishes a new liability-correction event, that `PayableReopened` means a particular technical implementation, or that the real policy permits the sample fixture's `VALIDATED/PENDING` transition. Preserve locked source meanings until explicit accountable decision and any formally approved amendments.

  
## 7. Controlling D2 Finance decision matrix: source liability, new F04, prior F04 correction

**Proposal pending actual Finance policy and registered contract.** Require exact tenant/institution, original and current J05 correction fact/receipt version, J04 issued version, F14 invoice/match/recognized-liability identity/version, F04 identity/category/original-adjustment reference/current stage, F08 current Payable/settlement version. Preserve L2-F04 §29.4 frozen initiation references: never retroactively edit an already created F04 adjustment because its source changed. No J05-induced direct payable/liability/journal mutation.

| Case | Proper owner and immutable reference | F04 treatment | AP evidence and safety gate |
|---|---|---|---|
| Corrected J05 fact has no monetary effect | F14 independently attests its original invoice/liability interpretation still holds | No automatic adjustment | F08 independently assesses other holds; absence of event is not NOT_NEEDED |
| A — underlying procurement/Finance original liability changes | **F14** re-evaluates original source invoice/match/liability using current J05/J04/vendor and institutional financial policy | Not automatically F04. If policy explicitly requires distinct financial-position adjustment, use B below; any needed F14 *revised liability event* not present in registry is an **upstream contract blocker** | F08 remains non-ready until authenticated F14 completed interpretation and any independently required F04 result; never infer a revised liability event |
| B — a new approved modification to financial position from authoritative changed facts, **without an erroneous prior F04** | **F04** initiates an ordinary new Financial Adjustment using valid source and financial-position references | F04's normal separate approval, Published then Pending Application/Applied; **do not use** Corrected/Reversed Adjustment Reference to an F14 invoice/liability | Published may be observed but does **not** prove balance applied; consumption changing Payable liability waits for owner-witnessed **Applied** plus applicable F14/F08 source policy |
| C — correct or reverse an **earlier published F04 Financial Adjustment** | **F04** creates a *successor* Correction Adjustment or Reversal Adjustment with its **Corrected/Reversed Adjustment Reference pointing to original published F04 ID** (locked L2-F04 §29.1) | Original F04 remains immutable; new successor follows full normal lifecycle; publication and application are different facts | Do not apply successor economic effect to Payable before independently proven Applied where financial position changes |
| Prior F04 merely Draft/unpublished | F04 uses its existing pre-publication review/cancellation process | §29.1 published-original correction/reversal reference inapplicable | Do not consume draft/referred adjustments |
| J07-issued stock, J08-registered asset or applicable F12 capitalization already occurred | J07 governs operational return/movement; J06 ledger; J08 operational asset; F12 applicable financial asset accounting; F14 still owns vendor liability | F04 only if an **additional, nonduplicative** financial modification is required by policy | All owner effects independent; no silent negative stock, operational asset deletion or double financial posting |
| Partial/full F10 settlement or unknown F04/F14 outcome | Treasury/Finance owner correction required, no unilateral F08 rollback | F04 only through its governed policy | Maintain hold and preserve settlement references |

**No double posting:** obtain source-owner classification and calculation of one economic effect with currency, tax, discount, prior liability, correction version, financial period and prior F04/Accounting application witnesses. A unique owner-defined source-effect key binds that classification to one responsible application path; F14 original-liability reinterpretation, F04 delta and Accounting posting must not each independently debit/credit the same change. Require owner-certified NOT_NEEDED from any participating owner whose action is omitted. No source-reviewed F14 revised-liability contract, exact monetary policy, applicable F04 reason or idempotent posting mechanism has been verified as deployed; do not guess one.

## 8. Controlling D3 production F08 flow and lifecycle conflict

**Proposed production sequence:** current F08 Payable with exact F14 recognized-liability witness and settlement/version → F08-owned authorized hold/suspension → source-referral to J05/J04/F14, J07/J06, J08/applicable F12, F04 where relevant → independent owner-completed results or certified NOT_NEEDED → new true A04-authorized F08 reviewer decision against latest Payable/source versions → only the actually ratified F08 reopening lifecycle operation → **separate** payment-readiness decision. Missing source result/UNKNOWN, conflicting holds, expired permission, already settled payment or changed version means remain non-ready. F10 independently owns payment execution; F05/Accounting independently own journal consequences.

**Verified upstream ambiguity:** locked F08 lifecycle illustrates `Suspended → Reopened`; its canonical `PayableReopened` event description refers to a **previously closed** Payable. Chetan/F08 must reconcile whether a suspended-case resolution can publish that exact event under an approved semantic clarification or needs a separate governed registry contract. Neither is established by the fictional HOME projection `SUSPENDED → VALIDATED/PENDING`. Until this source clarification and real A04 permission/evidence mapping is ratified, **positive production suspended-case release is BLOCKED**, including documentary-only release. Do not infer that a released hold itself means payment-ready or paid; preserve current liability and settlement evidence.

**Acceptance cases:** F04 source-liability versus ordinary-new-adjustment versus published-prior-adjustment successor; Published without Applied; two paths claim same monetary delta; registered J08 asset with already posted F12 capitalization; prior J07 issue/return; documentary-only proof versus concealed material source discrepancy; concurrent Payable version change; source corrected but F14 still UNKNOWN; F10 partially settled; F08 audit and independent readiness, never inferred settlement.

## 9. Formal ADR fields — version 0.2; human decision not recorded

**Target:** locked L2-F04 §29.1/§29.4, F08/F14 plus 2026-08-21 amendments, L3-F-046/-047/-048 and affected L4-F-046 Draft; no locked source changed. **Need:** disambiguate original liability, independent new F04 and previously published F04 corrective successors; resolve unsafe production F08 release mapping. **Proposed decision:** controlling §§7–8, subject to accountable source approval. **Impact analysis:** J05/J07/J06/J08 and applicable F12, F14, F04, F08, F10, F05, A04/A06/A07 each separately owns effects. **Review:** Chetan/F14/F04/F08 and Section J as relevant, independently Varad/Devang for Finance, Dhruv/A04/Lead.

### Decision

<!-- Deliberately blank until formal human source-owner, Finance review and Lead disposition against exact PR version. -->

### Acceptance checklist (production validation unexecuted)

- [ ] F14 exact liability correction/republication contract and monetary policy ratified, or recorded as upstream blocker.
- [ ] Ordinary new F04 vs published original F04 correction/reversal references and Published/Applied lifecycle independently accepted.
- [ ] Original/source-version and one-economic-effect idempotency tests including F05 posting and actual F10 settlement safeguards.
- [ ] F08 suspended-case release event semantics and A04 policy resolved with independent payment-readiness verification.
- [ ] J07/J06, J08 and applicable F12 cross-domain consequences independently verified.

**Approval provenance:** prior bounded synthetic Lead approval exists only on separate still-unmerged draft [PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143); it does not authorize these real monetary/reopening contracts.
