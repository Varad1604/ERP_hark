# Batch 18 W4 — D1–D4 Technical Design Review Index

**Status: THREE LINKED DESIGN PROPOSALS. NOT approved canonical architecture, real permissions or implemented backend contracts.**  
**Prepared:** 2026-10-03  
**Purpose:** Provide exact, implementable candidate decisions to resolve Architecture #144 (J05), #145 (Finance), #146 (trusted recovery), parent #140 and frontend W4 #49 without asking the Lead Architect to invent institutional policy.

## 1. Review in dependency order

| Order | Detailed design | Existing authoritative source | Required disposition |
|---|---|---|---|
| 1 | [D1/D2 J05 receipt quantity/replacement and finalized correction](ADR-B18-W4-D1-D2-RECEIPT-CORRECTION-DESIGN-PROPOSAL-2026-10-03.md) | Locked L2-J04/J05/J06, L3-J-049/J-050; L4-J-050 **Review** | Chetan/J05: exact replacement/overreceipt policy and positive correction; independently review J04/J06/F14 consequences |
| 2 | [D2/D3 independent F14/F04 compensation and F08 hold/release](ADR-B18-W4-D2-D3-FINANCE-COMPENSATION-AP-HOLD-DESIGN-PROPOSAL-2026-10-03.md) | Locked L2-F04/F08/F14 and 2026-08-21 amendments, L3-F-046/-047/-048; L4-F-046 **Draft** | Chetan/F08/F14 with Varad/Devang Finance review; Dhruv/A04 genuine role/SoD binding |
| 3 | [D4 independent source-authenticated uncertain outcome / durable NOT_APPLIED proof](ADR-B18-W4-D4-TRUSTED-OUTCOME-RECONCILIATION-DESIGN-PROPOSAL-2026-10-03.md) | Locked L2-A04/A05/A06/A07 and A05 2026-09-23 recovery amendment | Dhruv/A04/A05/A06 + affected domain owners: exact authoritative source-verifier/fencing technical contract |

## 2. Decisions that must not be confused

- **Existing, locked facts:** J05 owns receipt/inspection and the authorized `ReceiptVarianceRecorded` completed correction fact. J04 owns PO commitment and closure. J06 owns authoritative inventory. F14 alone publishes `VendorInvoiceMatched`, `VendorInvoiceApproved`, `VendorLiabilityRecognized`. F08 alone owns Payable and readiness. F04 owns Financial Adjustment. A06 transports completed business events, while A05 governs workflow recovery and domain owners determine their own business outcomes.
- **Existing tested mock scope:** current HOME PR #46 already includes conservative independently witnessed multi-receipt intake, separate receipt-level system inventory consequence, clearly fictional AP documentary hold and optional J05 **correction request only**. Successful pinned [run #37109152627](https://github.com/DhruvNaik26/SutraERP_frontend/actions/runs/37109152627) validates the selected mock fixture and targeted C8 preflight, not the positive unimplemented contracts in these proposals.
- **New *proposed* rules:** exact replacement entitlement and institution-specific excess boundaries; true finalized positive receipt correction authorization; independent F14/F04/J06 compensation in consumed cases; real AP material-source release policy; durable owner-authenticated NOT_APPLIED verifier with atomic fence and true A04 recovery permission.
- **Non-decisions:** `SIMULATED-` references cannot prove real A04 permission; no fictitious backend API/event/threshold is declared as installed; a Finance or stock consequence is never inferred from another domain's corrective event; the current AP mock `VALIDATED/PENDING` projection is not automatically the real F08 `Suspended→Reopened` event semantics.

## 3. Cross-document blockers / acceptance gates

1. First ratify the J05 quantity, actual variance authorization and corrected-source envelope without amending locked sources by implication.
2. Ratify separate J06 consequence and F14/F04 Finance interpretations per correction category. If no correction is required from an owner, **that owner must independently attest NOT_NEEDED**; absence of an event is not enough.
3. Confirm actual F08 lifecycle behavior for material-dispute release; an approved resolved-source fact is necessary but does not itself authorize payment or override Treasury.
4. Determine whether a source owner can actually provide atomic durable command-result **read plus fence**. If the architecture/implementation cannot guarantee prevention of late original application, no positive NOT_APPLIED recovery can be safely certified.
5. A04 real authorization, configured SoD, A05 workflow recovery, A06 event registry/delivery and A07 audit must remain distinct. No blanket Lead-approved demo permission makes real service grants or a valid source fence appear.
6. Implement positive integration **only against ratified specifications**; validate under exact current HOME, concurrency/replay/tenant negative tests, separately owner-witnessed consequences, then full relevant C8/C9. Keep W4 open until required exceptions and acceptance are independently verified.

## 4. Review/disposition template

For *each* unresolved row above, the accountable owner should record:

- **Decision:** APPROVE AS WRITTEN / APPROVE WITH AMENDMENTS / BLOCK, and explicit reference to the exact proposal version/commit.
- **Approval scope:** tenant/institution policy applicability, real vs synthetic use, allowed business categories, version and exceptional quantitative limits, where relevant.
- **Existing implementation reference:** approved exact code/service/event/schema/permission/registry contract, if it exists; otherwise confirm the upstream design+build dependency.
- **Source conflict:** what precise locked document/section requires a governed amendment/re-lock; do not silently revise locked semantics in this proposal.
- **Cross-owner review:** independent J04/J06/F14/F04 decisions as affected; Varad/Devang Finance review, Dhruv A04/A05/A06 cross-domain governance.
- **Evidence:** attributable reviewer, date, approved policy/contract reference, negative and positive acceptance results after implementation.

An issue may be closed once its *specified accountable decision and implementation dependency* are recorded, even if downstream execution remains in W4 #49; do not close W4/J05 certification merely because these papers exist.

## 5. Approval and implementation status

- Lead Architect formally approved the **bounded existing Stage-5 synthetic** D1–D4 mock/fail-closed behavior in [the separate Lead-approved record](ADR-B18-W4-EXC-001_LEAD_ARCHITECT_APPROVAL_2026-10-03.md) on draft PR #143. That existing approval is not retroactively extended to every proposal here.
- These three new documents are **architect-generated candidate specifications submitted for review**. No claim is made that the named owners separately approved them, that a trusted verifier already exists or that full W4 positive exception development is complete.
- Do not merge this proposal PR or any changes to canonical locked documents as if a proposal alone were the required source-owner decision.
