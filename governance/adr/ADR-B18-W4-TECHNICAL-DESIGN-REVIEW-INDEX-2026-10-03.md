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

- Lead Architect formally approved the **bounded existing Stage-5 synthetic** D1–D4 mock/fail-closed behavior in [the still-unmerged separate PR #143 record](https://github.com/DhruvNaik26/SutraERP_Architecture/blob/stage5-b18-w4-source-owner-decision-packet-20261003/governance/adr/ADR-B18-W4-EXC-001_LEAD_ARCHITECT_APPROVAL_2026-10-03.md) (not a canonical file in this PR or Architecture main). That existing approval is not retroactively extended to every proposal here.
- These three new documents are **architect-generated candidate specifications submitted for review**. No claim is made that the named owners separately approved them, that a trusted verifier already exists or that full W4 positive exception development is complete.
- Do not merge this proposal PR or any changes to canonical locked documents as if a proposal alone were the required source-owner decision.


## 6. Independent review #5399884517 — v0.3 editorial consolidation of v0.2 findings

The initial full review on original head `49d400df184341d4b51dfadf35d9404ccd44bf56` remains historically visible. The changes below are **proposed design remedies**, not actual production capability, policy adoption or human approval.

| Finding | Verified issue and v0.2 design disposition | Required outstanding decision/evidence |
|---|---|---|
| 1. D1 quantities/replacement | Confirmed contradiction. D1 §2 counts N ordinary, R replacement and G gross physical separately, with disjoint entitlement T/C and independent approved E, safe deny without ratified policy. | Chetan/J05 ratifies exact entitlement, units/custody and tolerance; J04 controls PO commitment/version |
| 2. D2 correction persistence | Confirmed ambiguity. D1 §§3–4 selects supplemental request/decision/outcome within canonical J05 GoodsReceipt consistency boundary and atomic registered ReceiptVarianceRecorded outbox on actual completion. | Ratify positive correction categories, A04 SoD and existing event schema sufficiency |
| 3. Finance correction | Confirmed reference ambiguity. Finance §3 separates changed F14 original liability, ordinary new F04 adjustment, and correction/reversal of **an earlier published F04**; distinguish Published from Applied. | Chetan F14/F04/F08 plus Varad/Devang independently review money/event contract and one-economic-effect policy |
| 4. D4 late application | Confirmed unsafe temporary-fence wording. D4 §4 requires permanent original tombstone/non-revocable owner execution prohibition across crash, failover and migration. | Actual source-owner atomic implementation/retention proof; absent it UNKNOWN/no new retry |
| 5. D4 different ID | Confirmed missing intent admission. D4 §5 requires owner-defined stable business-intent key, atomic unique reservation, one authorized successor and concurrency tests. | Owner-certified source identity/unique-key and A04/domain permissions; not currently implemented evidence |
| 6. J07/J08/F12 | Confirmed omission. D1 §§3–4 and Finance §§3–4 route J07 movement, J06 ledger, J08 asset, applicable F12 asset accounting separately. | Owner-specific real contract applicability and post-consumption tests |
| 7. ADR governance | Confirmed structure/provenance gap. Each proposal now has explicit target/Need/Proposal/Impact, empty human Decision and acceptance checklist; #143 explicitly still unmerged. | Named human approvals and separate governed L4-J-050 Pallav-versus-Chetan metadata resolution |
| 8. F08 lifecycle | Confirmed upstream semantic inconsistency: lifecycle shows Suspended→Reopened; existing PayableReopened event describes a *previously closed* payable. Finance §4 defaults to hold until governed clarification. | Chetan/F08 authoritative suspended-release transition/event and real A04 policy |
| 9. Cross-domain completion | Confirmed original incompleteness. Below dependency gates require independent source version/owner witnesses, consumer-specific NOT_NEEDED and explicit partial/UNKNOWN outcomes. | Positive integration and owner review after missing policy/contracts are approved |

### Owner-separated dependency, handoff and acceptance matrix

| Step | Versioned authority before mutation | Outcome and partial failure boundary |
|---|---|---|
| J04/J05 ordinary, replacement and correction | Current J04 issued PO-line/version, J02 supplier, J05 line/arrival/source facts, configured units/policy and true A04 SoD; separate N/R/T/C/E admission | J05-only source correction completed only after supplemental history and existing registered ReceiptVarianceRecorded outbox are atomically committed |
| J07 then J06 where stock moved | J07 source-owned issue/transfer/return/custody outcome, J06 current stock ledger/reservations | Separate J07 fact and J06 ledger/availability fact; no inferred negative stock or cross-owner write |
| J08 and applicable F12 | J08 existing asset registration/assignment history, applicable F12 capitalization/valuation evidence | Independent operational asset and financial asset decisions; no asset deletion or assumed journal |
| F14 then F04 if independently needed | Source-matched original invoice/liability identity/version, corrected J05 and current F14 policy; distinct F04 reason/original F04 reference when appropriate | F14 original-liability result vs F04 ordinary/new or published-prior successor; F04 Applied needed for applied monetary effect; no duplicate posting |
| F08 readiness and separate F10 settlement | Exact current Payable/settlement version; certified source outcomes and real A04; ratified suspended-release event mapping | F08 may maintain a hold; reopening blocked pending upstream event semantics, readiness separately evaluated, F10 never inferred |
| A04/A05/A06/A07 and D4 uncertain owner commands | Owner immutable original command+intent signature/version, permanent tombstone for negative result; real separate A04 authority, A05 recovery and A07 audit | APPLIED return read-only, UNKNOWN fail closed, single authorized new ID only after permanently fenced NOT_APPLIED; A06 retry is event transport only |

**Never equate J05 completion with J07/J06/J08/F12/F14/F04/F08 completion.** Each material consumer explicitly records APPLIED, NOT_NEEDED, WAITING, BLOCKED or UNKNOWN **from its own authority**, as an observational reconciliation projection rather than new canonical state. Missing consumer event is not an owner-certified NOT_NEEDED. An unrelated owner failure does not retroactively erase a committed J05 fact.

### Canonical-source traceability

- **Locked J04/J05, L3-J-049/-050** → D1 §§2–4 PO admission, source-linked arrival/correction, J05 ReceiptVarianceRecorded.
- **Locked J06/J07/J08 and applicable F12** → D1 §§3–4 and Finance §§3–4 independent stock movement/ledger, operational asset and asset-finance consequences.
- **Locked F14/F08 August 21 amendments, F04 §29.1/§29.4 and L3-F-046/-047/-048** → Finance §§3–4 exact invoice/liability handoff, proper F04 original F04 reference and F08 hold/readiness source gate.
- **Locked A04/A05 September 23 recovery amendment/A06/A07** → D4 §§4–6 owner-certified command outcome, permanent execution protection, independent permission/workflow/event transport/audit.
- **TEAM_ASSIGNMENTS, Volume 5, ADR README** → Chetan F/J accountability, Varad/Devang independent Finance review, Dhruv platform/Lead review; original L4-J-050 Pallav metadata left untouched for governed reconciliation.

**Actual acceptance state:** v0.3 consolidated source-grounded proposal corrections only. Positive entitlement, F14 revised-liability event, F08 suspended-reopen publication, real A04 grants, permanent tombstone verifier/business-intent storage and runtime tests remain unverified or blocked pending their specified owners/contracts. Separate PR #143 remains unmerged. W4 remains OPEN; no CI, merge or frontend modification is authorized by this index.


## 7. Editorial consolidation checkpoint — v0.3

The v0.3 drafts **replace the duplicative v0.2 “controlling remediation” appendices with one ordered candidate decision in each of D1/D2, Finance D2/D3 and D4**, preserving the protected original independent review and the decision-specific test obligations. The amended v0.3 source headings are D1 §§2–5, Finance §§3–6 and D4 §§3–9; cross-document citations in this index have been updated. Reviewer comments on earlier commits remain historical evidence only. All decisions are intentionally blank; no positive production authority was added. Any residual cross-owner policy or event ambiguity is a BLOCKED dependency, not editorially assumed resolved.
