# ADR-B18-W4-EXC-001 — W4 Receipt Corrections, AP Holds and Uncertain-Outcome Contracts

| Field | Value |
|---|---|
| Status | **BOUNDED STAGE-5 MOCK EXECUTION APPROVED by Lead Architect; D1–D4 positive real/technical contracts still PROPOSED / BLOCKED** |
| Prepared | 2026-10-03 |
| Workstream | Batch 18 W4, Procurement-to-Payables J05 |
| Tracking | Architecture issue #140; frontend implementation issue #49 |
| Required domain review | **Chetan (J05/J06/F08/F14/F04; accountable for Sections J and F)**, **Dhruv (A04/A06 platform and cross-domain Lead Architect)**, and **Varad/Devang (mandatory Finance peer-review team)** as their respective review roles require |
| Approval rule | Lead Architect's bounded decision is recorded separately in `ADR-B18-W4-EXC-001_LEAD_ARCHITECT_APPROVAL_2026-10-03.md`. This proposal does not amend Locked sources or replace missing real policies, technical source contracts or required Finance review. |
| Scope | The precise policy and missing interface decisions only; do not reopen already-locked L2/L3 ownership |


> **Accountability correction (2026-10-03):** Checked against `architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md` on Architecture main: **Chetan owns both Section J (J05/J06) and Section F (F08/F14/F04)**. **Dhruv owns Section A and leads cross-domain governance**. **Varad and Devang are the required Finance review team**. Pallav owns Sections B and D, not J05; historical mention as an L3 contributor does not confer J05 domain approval authority. The draft previously misassigned J05 to Pallav; all D1/D2 review requests have been corrected here. Because Chetan spans multiple consuming domains, distinguish each J05, J06, F14 and F08 decision and obtain the appropriate independent Finance peer review; shared human accountability is not permission to collapse domain boundaries or waive SoD.

## 1. Verified locked baseline — not open for reinvention

1. **J05**: Locked L2-J05 §§2–13 owns receipt/inspection, partial/cumulative/replacement/variance and finalised evidence, `ReceiptVarianceRecorded` and explicit variance approver authority. Receipt commands retain tenant/institution context and idempotency; J05 does **not** own J04 PO commitments, J06 stock, J08 assets, F14 invoices/liabilities, Treasury or GL.
2. **J05 correction**: Locked L3-J-049 and **L3-J-050** prescribe the Goods Receiver/Inspector's attributable error provenance, preserved **immutable original** receipt/inspection/disposition/downstream references, independent Receipt Variance Approver's permitted compensating scope, **supplemental** receipt/inspection evidence and the existing J05 `ReceiptVarianceRecorded` fact. Downstream J06, J08 and Finance each independently determine their own effect. L4-J-050 is **Review**, not an approved technical contract.
3. **F08**: Locked L2-F08 and approved 2026-08-21 amendment preserve `Section J → F14 VendorInvoiceApproved → F14 VendorLiabilityRecognized → F08 PayableRegistered → F08 PayableValidated`. F08 alone owns Payable suspension/readiness and explicitly does **not** execute Treasury settlement. Existing L2 `PayableSuspended` exists; `PayableValidated` does not prove settlement or payment readiness.
4. **AP exception**: Locked L3-F-046 governs an **existing validated** Payable's dispute/hold and three possible AP positions: governed resolution and readiness review, continued hold, or referral to the **separately owned** correction process. Locked L3-F-047/-048 preserve F04/F14/J05 correction and write-off ownership. L4-F-046 is **Draft**, not a locked implementation interface.
5. **Unknown command outcome**: L2-A06 owns transport/retry, A04 policy/SoD and A07 enterprise evidence. Locked business scenarios do **not** specify a trusted technical `NOT_APPLIED` verifier. The currently isolated W4 mock correctly retains an indeterminate retry lock and rejects arbitrary `verified:true` caller flags.

## 2. Detailed decision options — bounded mock dispositions approved separately, remaining positive contracts not adopted

### D1 — J05 multiple/partial deliveries, variance tolerance, replacement quantity

**Recommended conservative fixed Stage-5 mock policy:** Under one exact issued PO/line, identical tenant/institution/item/supplier, allow multiple independently inspected receipt identities with total **physical received** quantity not exceeding original ordered quantity. Separately accumulate received, accepted, rejected/nonaccepted. The already-tested fictional path is *two* partial receipts totalling 10 received, 7 accepted, and 3 nonaccepted; no claim of actual institutional tolerance or automatic PO closeout.

**Chetan, as the documented J05 accountable architect, must determine:** (a) permitted PO-line or quantity tolerance and policy version when receipt sum would exceed ordered quantity; (b) whether rejected physical delivery may be **replaced** without exhausting available delivered quantity, and which original/reference IDs and approvals qualify; (c) policy for short, excess, damage, substitution and return/reinspection and whether each is an existing event/inspection supplemental fact; (d) when J04 independently determines PO partial/exceptional/full closure. These decisions cannot be inferred by W4.

**Safe fallback until approval:** deny over-receipt/replacement attempts, preserve the factual evidence and provenance, allow bounded under-PO multiple receipts. Never claim all multi-receipt cases are complete under that fallback.

- [ ] Chetan/J05: approved with exact policy/source references and any limits
- [ ] Chetan/J05: amendments required — specify target Locked section and impact
- [ ] Chetan/J05: blocked — provide the accountable dependency

### D2 — J05 finalised corrective fact and independent downstream compensation

**Recommended L3-conformant demonstration:** Represent correction as a **supplemental J05 owner-history fact** referencing the exact immutable finalised receipt ID and current history witness, error originator + independent variance approver, changed lines/quantities/reason/inspection evidence and a separately visible **pending downstream action**. Record existing canonical business meaning `ReceiptVarianceRecorded` only for source-governed approved corrective outcomes. Do **not** rewrite prior `GoodsAccepted`; J06, J08 and F14 independently consume the correction and create their own governed consequences. A mock correction lacking approved actual policy/SoD source validation is explicitly **SIMULATED** and must not masquerade as a genuine correction or publish a real canonical event.

**Chetan, as the documented J05 accountable architect, must determine:** (a) exact institutional admissibility and required approval/evidence for each correction class, especially after an original `GoodsAccepted` has been consumed; (b) whether compensation may be recorded for the selected *fictional* Stage-5 example and the immutable evidence names/identity scope; (c) authoritative J05 correction fact supplied to J06 and F14 and who validates actual J06 stock reversal and Finance adjustment. **Chetan as the accountable J05/J06/F14 architect, with independent receipt, inventory and Finance responsibility checks,** must decide each separately owned downstream effect; never assume correction in J05 automatically changes stock or liability.

**Safe fallback:** capture observable **correction request / awaiting approval** with original unchanged, or deny the command when approved owner evidence cannot be established. No positive integrated correction or financial/stock reversal claim until the relevant domain decisions exist.

- [ ] Chetan/J05: approve exact J05 compensation/evidence semantics and source references
- [ ] Chetan/J06 and Chetan/F14: separately evidence and approve distinct any stock / Finance consequence
- [ ] Architecture: confirm any required amendment and L4 promotion

### D3 — F08 dispute/hold and release authority

**Already allowed by locked L2/L3:** F08 records a dispute reason and maintains suspension/readiness **BLOCKED** against a previously validated Payable; Finance F14 liability and J05 facts remain independent. An authorized AP documentary-only resolution may remove the hold, retaining full history, but does **not** itself authorize settlement or override an independently required J05/F14/F04 correction.

**Recommended minimal Stage-5 mock:** Limit a *fully labeled fictional* vendor-document dispute to separately identified synthetic requester and reviewer, a synthetic documentary reference, exact current F08-owner record/history, a `SUSPENDED` hold and `BLOCKED` payment readiness. Release only when the fictional **AP-owned documentation** is marked resolved under independent fictional review, and restore `VALIDATED` with readiness `PENDING` for separate eligibility evaluation. Actual source-side material discrepancies **remain blocked** until canonical source correction is independently verified.

**Chetan must determine:** (a) exact real AP permission/evaluation owner (A04), institution policy version, authorized requester/reviewer and SoD; (b) permitted dispute reasons and whether any case is AP-only documentary resolution versus source-side correction; (c) evidence that a source-side J05/F14/F04 correction has completed and the resulting AP evaluation; (d) precise readiness output when reopening, not automatically `READY`. Any new event/technical contract must be proposed upstream, not invented in W4.

**Safe fallback:** keep the **fictional** AP documentation example; block source correction dependent releases and real permission claims.

- [ ] Chetan: approve exact mock boundary and identify real AP policy/evidence sources
- [ ] Chetan + A04 owner: confirm real authority/SoD and release contract
- [ ] Finance/source owners: confirm dependent correction contracts

### D4 — Durable indeterminate outcome and independently source-verified recovery

**Decision requested from A06 / owning service + A04 / domain owners:** specify a trustworthy read/reconciliation interface by `tenantId, institutionId, owner, immutable commandId, transactionId, original command signature`. State whether verified `APPLIED` returns its current exact owner-history witnessed result, `NOT_APPLIED` is an independently verified terminal non-application fact allowing **new** command identity only after approval, and `UNKNOWN` retains the lock. Define which durable evidence/reference is required, authority to reconcile, audit receipt, expiry and cross-institution replay protections. Do not presume a network endpoint or event topic exists.

**Mandatory safety rule meanwhile:** `verified:true`, `sourceOwner`, `result` and `referenceId` supplied by an action caller are **not a trusted verification service**. Preserve the pending lock and immutable failed attempts. No technical retry based on guesswork. The new W4 isolated code already enforces the denial and tests it.

- [ ] A06 / owner service: provide verified technical/evidence contract and failure semantics
- [ ] A04 / domain owners: provide reconciliation authorization and audit requirements
- [ ] Architecture: decide whether implementation can occur in this Stage-5 scope

## 3. Dependencies, impact and acceptance

- **L2 ownership/authority:** J05 §9/J06/F14/F08/F04 and A04/A05/A06/A07. Do not transfer ownership or invent new publisher identities.
- **L3/L4:** L3-J-049/-050 and L3-F-046/-047/-048 are **Locked**, but L4-J-050 **Review** and L4-F-046 **Draft** are not approval substitutes. Any new technical implementation contract must follow formal governance.
- **L6:** Required `L6-RWF-0565` / `L3-J-050` corrective receipt and `L6-RWF-0206` / `L3-F-046` AP dispute workflows remain required. The tested standard `L6-RWF-0564` route does not certify these exceptions.
- **Evidence:** W4 frontend issue #49 and isolated branch `stage5-b18-w4-exceptions-followup`; newer combined HOME review candidate is still unmerged. All synthetic permissions must be visibly labeled.
- **Risk tradeoff:** unapproved positive transitions produce false business authority; indefinite blocking without accountable decisions prevents Stage-5 completion. The intended resolution is narrow source-owner decisions plus targeted implementation, not silent deferral to Stage 6.

## 4. Decision record — Lead-bounded approval recorded, technical owner decisions still pending

**Chetan (J05 decision D1–D2) / source link / date:** _PENDING_  
**Chetan (F08/F14 decision D2–D3, reviewed separately) / source link / date:** _PENDING_  
**J06 / F04 / A04 / A06 relevant-owner decisions:** _PENDING AS APPLICABLE_  
**Dhruv Lead Architect decision:** **APPROVED only for conservative synthetic behavior D1–D4 and fail-closed recovery**, as recorded in [`ADR-B18-W4-EXC-001_LEAD_ARCHITECT_APPROVAL_2026-10-03.md`](ADR-B18-W4-EXC-001_LEAD_ARCHITECT_APPROVAL_2026-10-03.md); **actual A04/A06 verifier contract and canonical amendment review still PENDING**  
**Required canonical L2/L3/L4 updates and re-lock evidence:** _NONE PERFORMED_  
**Effect on W4 closure:** **OPEN; no final handoff, J05 certification or amended gate.**

## 5. Review checklist

- [ ] Named domain owners have reviewed the *specific* unanswered conditions D1–D4 rather than generic principles already locked.
- [ ] Each approved/rejected decision has source evidence and explicit institutional mock-versus-real boundary.
- [ ] All affected source and consumer owners have independently accepted their respective consequences.
- [ ] Approved changes, if any, are incorporated into relevant canonical Locked documents and revalidated.
- [ ] Frontend W4 implementation and negative tests use *only* that approved authority.
- [ ] Targeted exact-current-HOME native/TypeScript/build/desktop+narrow browser evidence passes; relevant C8/C9 and independent HOME acceptance recorded separately.
