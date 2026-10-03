# ADR-B18-W4-D2-D3 — Independent Finance Compensation and Governed Payable Hold/Release

| Metadata | Value |
|---|---|
| ADR/version | B18-W4-D2-D3 / v0.3 editorial consolidation (2026-10-03) |
| Status | **LEAD-ACCEPTED DESIGN DIRECTION; positive production contracts BLOCKED** |
| Target | Locked L2-F04 §29.1/§29.4, F08/F14 and 2026-08-21 amendments; L3-F-046/-047/-048, L4-F-046 Draft |
| Accountable review | Chetan for each F14/F04/F08 scope; independent Finance reviewers Varad/Devang; Dhruv platform/Lead |
| Earlier authorization | Only bounded fictional Stage-5 approval on separate still-unmerged [PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143) |

## Need

Distinguish the original F14-recognized liability, a new ordinary F04 financial-position adjustment and a correction/reversal of an **earlier published F04 adjustment**. The HOME fictional `SUSPENDED → VALIDATED/PENDING` release is not production F08 lifecycle authority.

## 1. Canonical constraints and missing production contracts

**Verified Locked**:
- `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md` §§1–6: F14 consumes independent Section J facts. F14 **alone** owns and publishes `VendorInvoiceMatched`, `VendorInvoiceApproved` and `VendorLiabilityRecognized`. The last is the single canonical handoff to F08.
- `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md` §§1–5: F08 begins only after F14 liability recognition and owns `PayableRegistered`, `PayableValidated`, Payable aggregate, suspension and payment readiness. Treasury alone owns actual settlement, Accounting its journal consequences. No cross-owner mutation.
- Locked `L2-F08_Accounts_Payable_Management.md`: preserves compensating rather than destructive changes; suspended Payables cannot progress to payment readiness/settlement. It lists `PayableSuspended` and `PayableReopened`; verify their **precise existing semantics** before reusing them in any new technical path.
- Locked `L2-F04_Financial_Adjustment_Management.md` §29.1: a correction or reversal is a **new** Financial Adjustment instance with its own approval lifecycle and immutable original reference; adjustment publication/application is not implied by creation or referral.
- Locked `L3-F-046`: AP may maintain hold, resolve with governed evidence or refer to source owner. `L3-F-047`: AP incorporates only independently completed authoritative F04/F14/J corrective facts. `L3-F-048`: AP write-off is referral only until F04 approves/applies it.
- `L4-F-046` is **Draft** and not an approved API, role grant or policy. The existing HOME W4 fictional documentation-only hold uses SUSPENDED/BLOCKED and subsequently VALIDATED/PENDING. That is a **mock review example**, not evidence that the exact real `PayableReopened` event should be fired from SUSPENDED.
- The Lead Architect's bounded approval permits only the fixed synthetic documentary demonstration and separate requests; it does **not** approve a real material-dispute release or create real Finance authorization.

**Gap to resolve:** the Locked documents define ownership and business outcomes but do not supply institutional numeric matching thresholds, real role assignments/SoD configuration, correction payload/API, or exactly how F14 compensates a consumed invoice/liability for each J05 correction type. Treat these as proposed technical choices.



## 2. Independent causal chain and source-witnessed consequences

Represent each participating owner as an independently validated, versioned and audited step, not a single mutable purchasing–Finance transaction.

`J05 corrected source fact (only if independently approved) → [J06 stock consequence independently evaluated] AND [F14 corrected invoice/liability interpretation independently evaluated] → [F04 separately approved adjustment, only where required] → [F08 updated Payable assessment]`.

These arrows are **causal dependencies**, not authorization transfers, synchronous cross-domain writes or a guarantee that every correction implies a new Finance adjustment. A J05 correction may affect only inspection description without quantity or monetary implications; it must **not** automatically reduce an already recognized liability. An inventory change need not imply a monetary adjustment. No F08 operation may publish an F14 or F04 fact.

### Proposed source-correction dependency witness

A read-only, exact-source observation may include:
`tenant/institution, transaction/PO/receipt line, original J05 witness, approved J05 correction witness, J06 consequence {not needed|witnessed|waiting|blocked|unknown}, F14 invoice/liability consequence {not needed|witnessed|waiting|blocked|unknown}, applicable F04 adjustment reference/approved/applied stage, F08 Payable current-version witness, audit correlation/causation`.

These fields describe an **illustrative composite read model**, not an approved shared aggregate or existing backend API. Each status must be derived from the owning history only; do not treat absence of a consumer event as proof of `NOT_NEEDED`. That determination requires the appropriate owning policy/result. A partial state (J06 corrected, F14 unknown) is **not** a synchronized completed correction.



## 3. Proposed D2 financial decision matrix

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



## 4. Proposed D3 production F08 release contract

**Proposed production sequence:** current F08 Payable with exact F14 recognized-liability witness and settlement/version → F08-owned authorized hold/suspension → source-referral to J05/J04/F14, J07/J06, J08/applicable F12, F04 where relevant → independent owner-completed results or certified NOT_NEEDED → new true A04-authorized F08 reviewer decision against latest Payable/source versions → only the actually ratified F08 reopening lifecycle operation → **separate** payment-readiness decision. Missing source result/UNKNOWN, conflicting holds, expired permission, already settled payment or changed version means remain non-ready. F10 independently owns payment execution; F05/Accounting independently own journal consequences.

**Verified upstream ambiguity:** locked F08 lifecycle illustrates `Suspended → Reopened`; its canonical `PayableReopened` event description refers to a **previously closed** Payable. Chetan/F08 must reconcile whether a suspended-case resolution can publish that exact event under an approved semantic clarification or needs a separate governed registry contract. Neither is established by the fictional HOME projection `SUSPENDED → VALIDATED/PENDING`. Until this source clarification and real A04 permission/evidence mapping is ratified, **positive production suspended-case release is BLOCKED**, including documentary-only release. Do not infer that a released hold itself means payment-ready or paid; preserve current liability and settlement evidence.

**Acceptance cases:** F04 source-liability versus ordinary-new-adjustment versus published-prior-adjustment successor; Published without Applied; two paths claim same monetary delta; registered J08 asset with already posted F12 capitalization; prior J07 issue/return; documentary-only proof versus concealed material source discrepancy; concurrent Payable version change; source corrected but F14 still UNKNOWN; F10 partially settled; F08 audit and independent readiness, never inferred settlement.



## 5. Proposed acceptance tests

- Original validated Payable → independent F08 documentary hold → source-confirmed documentary release → **non-ready until separate readiness assessment**.
- Same source facts, two repeated exact hold/release command IDs, changed-signature replay and concurrent source/Payable version change.
- Unsupported or untrusted actor, maker/checker collision, expired delegation/policy, conflicting explicit deny, wrong tenant/institution/vendor/invoice or amount/currency.
- Positive J05 receipt correction **only after ratified D1/D2 policy**; independent J06 and F14 successes; J06 done/F14 delayed; F14 done/J06 delayed; F04 needed vs F04 unnecessary with authenticated owner disposition.
- F14 amount changed after payable registration: F08 stays blocked while awaiting authoritative liability reconciliation.
- Fraudulent invoice/PO source match, rejected/returned quantity overlap, material issue disguised as documentary-only, absent source fact and F04 write-off request masquerading as approval.
- Prior settlement/partial settlement: deny simplistic release, preserve Treasury references and route to owning exception process.
- Cross-scope and changed-body replay, tampered history, crash after F14 before F08, recovery UNKNOWN, repeat browser reset.
- Independent audit of hold/maintain/referral/release, prior/resulting versions, applicable permission proof, explicit *no payment/GL side effects*.



## 6. Decisions requiring ratification

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

  


## 7. Formal ADR decision control

**Target, need, proposal, impact, owner and reviewers** are stated above. §§3–4 are the single candidate decision; the remaining details cannot become institutional policy without accountable review.

### Decision

**LEAD-LEVEL CONDITIONAL DESIGN-DIRECTION ACCEPTANCE — NOT PRODUCTION CONTRACT APPROVAL (2026-10-03).** Lead approves **D2/D3 Finance design direction only**, with reported off-platform collaborator consensus. F14 monetary revision contract, actual F04 applicability/calculation and production F08 suspension-release event meaning remain **UNRESOLVED/BLOCKED**. The fictional AP documentary example retains only its prior separately bounded approval; no actual material-source release or payment readiness is inferred.

**Provenance:** Dhruv's direct instruction and his report that collaborators approved off-platform, recorded in [Lead direction and reported review attestation](B18_W4_LEAD_DESIGN_DIRECTION_AND_REPORTED_REVIEW_ATTESTATION_2026-10-03.md). Individual GitHub reviews, precise unseen policy references, and backend evidence are not invented. This decision records the reviewed v0.3 baseline, not an unrestricted approval of unstated details.

### Acceptance checklist (production validation unexecuted)

- [ ] F14 exact liability correction/republication contract and monetary policy ratified, or recorded as upstream blocker.
- [ ] Ordinary new F04 vs published original F04 correction/reversal references and Published/Applied lifecycle independently accepted.
- [ ] Original/source-version and one-economic-effect idempotency tests including F05 posting and actual F10 settlement safeguards.
- [ ] F08 suspended-case release event semantics and A04 policy resolved with independent payment-readiness verification.
- [ ] J07/J06, J08 and applicable F12 cross-domain consequences independently verified.

**Approval provenance:** prior bounded synthetic Lead approval exists only on separate still-unmerged draft [PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143); it does not authorize these real monetary/reopening contracts.
