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

F14 may reconsider recognized monetary exposure only on source-authoritative receipt/PO/vendor/invoice facts, true Finance matching/approval policy and exact original invoice/liability identity; determine whether it must re-match, reject or issue a separately governed revised Finance interpretation. If an F04 correction/reversal is required, initiate a **separate** F04 adjustment with immutable original reference; wait for independently witnessed F04 approved/applied outcome. Reconcile F14 and F04 without double-posting or aliasing their canonical event publishers. Never simply subtract quantity times a fixed demo price from the Payable, directly mutate outstanding liability from a J05 fact, or infer a credit-note authority from receipt discrepancy alone.

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
