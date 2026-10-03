# SutraERP Lead Architect Decision — Batch 18 W4 Exception Boundaries

**Decision ID:** LAD-B18-W4-2026-10-03  
**Decision-maker:** Dhruv, Lead Architect (explicit instruction received in project conversation, 2026-10-03)  
**Status:** **LEAD-ARCHITECT APPROVED for the explicitly bounded Stage-5 synthetic mock and fail-closed safety rules below. NOT an approval of missing institutional policy, L4 technical interfaces, real domain-owner permissions, or final W4/J05 closure.**  
**Review provenance:** Lead Architect states the responsible parties have reviewed and approved the proposal **off-platform** and expressly authorizes this formal Lead Architect approval record in place of asking them to add individual GitHub notes. Their individual source-owner or Finance-peer-review attestations and the precise unrecorded implementation contracts were not independently available in the repository when this record was created. Their approval must not be misrepresented as submitted GitHub reviews or as a technical contract.  
**Reference:** [Proposed ADR-B18-W4-EXC-001, draft PR #143](https://github.com/DhruvNaik26/SutraERP_Architecture/pull/143); [architecture decision issues #140, #144–146](https://github.com/DhruvNaik26/SutraERP_Architecture/issues/140); [frontend W4 issue #49](https://github.com/DhruvNaik26/SutraERP_frontend/issues/49).  
**Source hierarchy:** Current Locked L2-J05/J06/F14/F08, L3-J-049/-050 and L3-F-046/-047/-048, TEAM_ASSIGNMENTS, repository CONTRIBUTING. This Lead decision does **not** rewrite Locked business contracts or reclassify Review/Draft L4.

## 1. Decision scope and accountable ownership

For the approved bounded mock, maintain the existing section owners: Chetan is accountable for **separately owned** J05, J06, F14 and F08 consequences; Dhruv for A04/A06 platform boundaries and Lead governance. Varad and Devang are the Finance Review Team under established policy. Pallav owns B and D, not J05. The Lead Architect's off-platform approval report is recorded here as a governance attestation, **not forged signatures or independently verified technical source evidence**.

### D1 — APPROVED: conservative multiple-receipt Stage-5 mock; other quantity policies remain blocked

For one exact, witnessed, issued J04 PO and institution/tenant/item/supplier, allow separately inspected immutable J05 receipt IDs and cumulative physical **received** quantity not exceeding the original ordered quantity. Track physically received, accepted and nonaccepted separately; no silent replacement, overlapping source history, invoice or inventory double recognition. The fictional 6+4 receipts and 4+3 accepted quantities are valid mock acceptance examples. Do **not** assume physical received equals accepted, PO closure follows automatically, or that this illustrative no-excess policy is a real institution's tolerance configuration.

**Not approved by this record:** positive excess-delivery or replacement offset against earlier rejected physical receipts, damaged/substituted returns, unprovided institutional tolerance values or J04 PO closeout policy. These require the actual policy/authority details in #144; continue fail-closed tests until supplied.

### D2 — APPROVED: corrective request, immutable evidence and owner separation; positive compensation still blocked

W4 may implement a clearly **fictional/request-only** correction submission referencing an immutable finalized J05 receipt, attributed originator and affected quantities/reason, observable **awaiting source-owner approval** state, and denial of untrusted/unapproved positive transitions. Do not overwrite a finalized `GoodsAccepted` or present caller-asserted `SIMULATED-` authority as actual approval. A genuine `ReceiptVarianceRecorded` positive outcome requires its independently governed J05 variance approval; already-consumed J06, J08 and F14/F08 obligations remain separate independently approved owner effects.

**Not approved by this record:** a particular positive receipt correction/replacement action, J06 stock reversal, F14/F04 liability adjustment, automatic cross-owner compensation, authorizer binding, tolerance or real institutional event/API contract. Record exact source authorization/policy and each consumer's compensation semantics through #144–145 before implementing those cases as positive outcomes.

### D3 — APPROVED: fictional AP documentary dispute and fail-closed material-source hold

For the *already validated* and independently witnessed F08 mock Payable, the fixed fictional documentary vendor dispute may use visibly synthetic requester and a **different** synthetic reviewer, an explicit synthetic reason/evidence reference, immutable F08 owner history, `SUSPENDED` and readiness `BLOCKED`. A fictional documentary-only resolution may restore its *existing* `VALIDATED` status with readiness **`PENDING`** for a separate eligibility check. Never infer payment eligibility, settlement, GL posting, real A04 permissions or an automatic change to independent J05/J06/F14 state.

**Not approved by this record:** a positive release of a material invoice/receipt discrepancy dependent on unverified external J05/F14/F04 correction, arbitrary dispute policy, real role grants/SoD or a settlement transition. Obtain distinct actual Finance/source decision evidence through #145.

### D4 — APPROVED: safe retry lock and denial of caller-asserted verification only

When an owner command's external outcome is uncertain, retain its original immutable key/signature, source history and pending retry lock. Deny arbitrary caller `verified:true` / `sourceOwner` / `result: NOT_APPLIED` references, changed command bodies, cross-tenant/institution replays and unsafe retry. An unresolved command remains **UNKNOWN / reconcile-before-retry** in this mock until actual independently obtained evidence exists.

**Not approved by this record:** any positive `NOT_APPLIED` reconciliation, new verifier URL/event/database, owner attestation token or clearance API. #146 must establish the durable trusted source-verification contract and A04 authorization/SoD before positive retry/reconciliation is implemented. This Lead decision cannot create a technical service absent from the canonical sources.

## 2. Evidence, implementation and W4 exit gates

- [Tested bounded mock candidate run #37099422197, attempt 2](https://github.com/DhruvNaik26/SutraERP_frontend/actions/runs/37099422197/attempts/2) has successful exact-source native regression, targeted C8 static preflight, TypeScript, production build and **4/4** desktop/narrow browser assertions. [Evidence artifact #11266722951](https://github.com/DhruvNaik26/SutraERP_frontend/actions/runs/37099422197/artifacts/11266722951). This is evidence for the **bounded synthetic example**, not a test of the unimplemented positive contracts above.
- Current HOME PR #46 already contains the matching source/test blobs for the bounded W4 candidate (verified 2026-10-03). Do not redundantly cherry-pick draft frontend PR #54 and overwrite newer HOME changes; reverify the exact current branch before future work.
- This approval authorizes W4 to build/test any further *explicitly synthetic request/blocked/observational* pathways consistent with the locked business semantics. It does not authorize an actual institution/production permission or simulated cross-domain posting.
- **Final W4 closure remains OPEN.** The user's earlier explicit directive requires remaining mandatory exception flows implemented and validated before handoff. Do not reinterpret this approval as a waiver, Stage-6 deferral, full C8/C9 pass, complete J05 runtime certification, frontend main merge or authenticated backend integration.
- Update this record or create a **separate accountable, source-linked decision** when actual D1 replacement tolerances, D2 positive corrections/independent compensation, D3 material-release authorization and D4 durable verifier become specified. Apply architecture amendment/re-lock and normal required peer review if canonical contracts change.

## 3. Formal disposition

**Lead Architect approval:** APPROVED **only within the exact boundaries in D1–D4 above**, pursuant to the Lead Architect's explicit message authorizing a written approval record.  
**Owner review report:** Reported by Lead Architect as completed off-platform. **No per-owner GitHub review or detailed technical owner contract is attested in this record.**  
**Governance:** Existing Finance independent review and canonical amendment rules are not repealed by this record. Whether further owner/peer attestations are required to merge canonical changes is governed by CONTRIBUTING; this bounded Lead approval is effective as an execution instruction without claiming those external records exist.  
**Remaining issue status:** #144, #145 and #146 open for precise positive contracts; #140 and frontend #49 open; draft PR #143 stays unmerged while those dependencies exist.
