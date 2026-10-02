# Batch 18 J02 — Pre-Student Financial Clearance Contract Decision

**Status:** PROPOSED / DECISION REQUIRED — not approved architecture, not an executable API or event contract  
**Date:** 2026-10-01  
**Decision owner:** Lead Architect / canonical Architecture governance  
**Tracks:** Architecture issue #132; B18-J02 admission-approved applicant → Active StudentContext  
**Evidence baseline:** Architecture `main` `257094b95e2c3ce7c6b1bd91340fc6422cf97c4c`.

## 1. Verified constraints (not new decisions)

- **C01:** `AdmissionApproved` precedes financial clearance; `FinancialCleared` precedes `AdmissionConfirmed`. Existing C01 §WF-C01-04 consumes `FinancialClearanceCompleted`; C01 BR-C01-025/026 prohibit Admissions monetary balance calculation and Finance-owned record mutation. Its policy decision matrix allows fully paid, a valid installment arrangement, approved scholarship or approved waiver subject to policy.
- **C02:** Only C02 owns Student and StudentContext creation/activation following a valid `AdmissionConfirmed`. Existing persons or an earlier unrelated StudentContext cannot serve as evidence that this applicant has been activated.
- **F00/F01:** The Finance foundation illustrates `Admission Approved` → Finance creates Admission Charge. F01 alone resolves charge source, evaluates charging policy, calculates financial values, creates/validates/publishes Student Charge and publishes `ChargePublicationCompleted`. The locked 2026-09-23 publication amendment permits ordinary policy-driven generation without *universal* human approval. F01 §13.4 and §20.5 still describe Student Context among illustrative validation inputs, without an explicit pre-Student canonical identifier or link rule.
- **F02:** An existing received Unallocated Advance remains a **Liability**, not charge allocation, settlement, waiver or clearance. F02 publishes `PaymentAllocationCompleted` and `PaymentSettlementCompleted` only after valid owner-controlled conditions. Published financial facts must preserve their own aggregate references.
- **Identity:** Canonical Person/Admission/Institute association must be explicit; a display name, identical labels or a loosely similar payment reference is not a join key.

## 2. Decision A — pre-Student Finance obligation (proposed if allowed)

The Lead Architect must approve **whether** F01 may create a Student Charge for an Admission-approved *prospective* Student **without** a C02 Student/StudentContext. Approval requires an explicit allowed pre-Student source-identity contract, not an L6 adapter assumption:

1. Define the originating **C01-approved source fact** and its unique Admission, canonical Person and Institution/Institute scope. Do not substitute `AdmissionConfirmed`, which would cause a clearance circular dependency.
2. Define precisely how F01's existing Student Charge aggregate may reference that pre-Student obligation while honoring F01's required validation and identity invariants. Specify the source identity, any conditional absence of StudentContext, and a traceable, immutable link to the C02 StudentContext that may arise only *after* Admissions confirmation. Do **not** fabricate a new API, event or Student Charge state in this proposed record.
3. F01 independently evaluates configured financial obligations and publishes its **real** `ChargePublicationCompleted`; Admissions does not calculate monetary values.
4. F02 must explicitly correlate an existing pre-received Unallocated Advance, the canonical Payment ID, the exact published F01 charge, eligible allocations and Finance-owned `PaymentAllocationCompleted` / `PaymentSettlementCompleted`. Partial allocation or surplus remains appropriately outstanding or liability under existing F02 rules; neither is blanket clearance.
5. Define the exact Finance *source evidence and policy conditions* that C01 may consume to make its own `FinancialCleared` decision. Do not assert that an F02 receipt event alone establishes clearance.
6. Only C01 can then confirm the **same** Admission; only a subsequent C02 action creates/activates the **same** Person/Institute association as StudentContext through its approved own source contract.

**Preferred investigation direction (not approval):** Examine A before designing a brand-new alternate path because F00 depicts Admission Approved → Finance charge and F01 lists Admission charges as a supported source. The missing pre-Student identity/validation semantics are consequential; if any locked F01 invariant rules out A, reject A rather than weaken it.

## 2A. Option A formal specification — investigation authorized, contract NOT approved (2026-10-02)

**Authorization scope:** The Lead Architect has authorized formal investigation/specification of Option A within this existing draft PR. This is not approval of pre-Student F01 charge eligibility, any event/API, source identity, financial settlement rule, C01 monetary calculation, or downstream implementation.

### 2A.1 Exact architectural contract decisions to be entered in a governed amendment

| Contract boundary | Existing source support | Decision still required before implementation |
|---|---|---|
| C01 originating fact → F01 | F00 §4 illustrates Admission Approved leading to Finance-created Admission Charges; locked F01 publication amendment requires a valid owner-authored source fact. | Confirm whether the existing `AdmissionApproved` business fact is sufficient and its exact approved publication/consumption semantics; do not create an additional event by implication. |
| Pre-Student financial subject | F01 owns Admission charges and the Student Charge aggregate but references Student Scope/Student Context in validation inputs. | Explicitly permit or reject conditional absence of Student/StudentContext, define the authoritative Admission and Person references, institute/institution/tenant and applicable Programme/Session scope and every validation condition. Distinguish source-Admission identity from F01 charge identity. |
| F01 lifecycle | Locked 2026-09-23 amendment assigns charge-source resolution, Charge Policy, calculations, validation and `ChargePublicationCompleted` to F01. | Confirm pre-Student charge-source classification and idempotent source-obligation uniqueness under existing F01 invariants; do not silently create a new lifecycle state, universal manual approver or charge policy. |
| Received receipt → F02 | F02 and locked L3-F-009 treat unallocated advances as liabilities until eligible allocation. | Define authoritative payer/Payment and charge correlation, multi/partial allocation policy, surplus treatment and exact grounds for `PaymentAllocationCompleted` and `PaymentSettlementCompleted`. Preserve uncertainty, reversal and no-match failure boundaries. |
| Finance evidence → C01 | C01 consumes `FinancialClearanceCompleted` and owns its admission decision; F02 settlement alone is not automatically admission clearance. | Identify the approved owner of the clearance-completed source fact, how F01/F02 evidence and any authorized policy alternatives are evaluated, required charge coverage, version/scope checks and the exact admission-reference correlation. Do not declare F02 an automatic publisher of C01's clearance fact without source evidence. |
| C01 confirmation → C02 | Locked L3-C-001 and C02 require an already confirmed Admission before Student/StudentContext creation. | Specify the immutable link between pre-Student charge/Admission/Person and the later C02 Student/StudentContext, including reuse of valid historical Student identity and one ACTIVE StudentContext per Institute. The late linkage cannot retroactively rewrite financial history. |

### 2A.2 Explicitly unapproved decisions / conflicts

1. **Pre-Student F01 validation remains open:** F01 Student Scope and Student Context validation passages must be reviewed and formally amended or authoritatively interpreted *before* an Admission-scoped obligation is considered legal. The presence of Admission Fees in F01 does not waive these conditions.
2. **C01 Step 2 correction required:** C01 WF-C01-04 §2.5 Step 2 currently instructs “Calculate financial obligations”; C01 BR-C01-025 explicitly prohibits Admissions financial balance calculation. The proposed governed text is: “Determine applicable admission financial-obligation types and clearance requirements under institutional admission policy; obtain authoritative F01-calculated charges and Finance clearance evidence. Admissions shall not calculate financial monetary values or modify Finance aggregates.” This is proposed wording only; the locked C01 source is unchanged.
3. **Clearance evidence owner must be resolved:** C01's named `FinancialClearanceCompleted` input, F02 allocation/settlement publications and F01 charge publication are distinct facts. Document their existing approved producer/consumer lineage or escalate the genuine gap before drafting a technical or mock implementation.
4. **Source-only evidence is not an executable journey:** Approval would require affected L2 source and locked-amendment governance; derived L3-C-001, L3-F-001/L3-F-009, L4, locked L5, L6 and J02 mock test traces must be checked. No automatic alteration of locked downstream documents is authorized here.

### 2A.3 Acceptance gates for the subsequent decision (not test claims)

- **Identity positive:** Approved C01 fact and the identical canonical Person/Admission/Institute scope yield one legitimate F01 obligation when its full configured conditions are satisfied.
- **Identity negative:** Missing/wrong Admission, mismatched tenant/Institute, duplicate source request, mismatched Person or missing policy/version cannot produce a charge or grant clearance.
- **Finance positive:** F02 matches an existing Payment to the exact F01 charge under allowed allocation rules, independently records eligible allocation and applicable settlement and preserves remaining liability/obligation accurately.
- **Finance negative:** Received-only advance, ambiguous payer/charge, insufficient partial allocation, excess unapplied balance, reversal or uncertain outcome must not be represented as paid-in-full or cleared absent a distinct approved policy rule.
- **C01/C02 sequence:** C01 uses only approved clearance evidence for the same Admission, subsequently confirms that Admission; C02 alone creates/links and activates the eligible StudentContext with unchanged canonical identity and immutable prior histories.
- **Provenance:** Each acceptance artifact cites approved L2 decision/revision, L3/L4/L5/L6 mapping and genuine owner/state evidence. Static manifests or reviewer-local disconnected actions do not certify J02.

**Stop condition:** If existing locked F01 validation prohibits a pre-Student charge and governance declines a compatible controlled amendment, Option A is rejected; reconsider Option B through a separate explicit decision rather than inferring clearance. Until a complete approved decision is recorded, the dependent J02 path remains blocked.

---

## 3. Decision B — existing policy-permitted alternate clearance (if A is not approved)

The Lead Architect may choose the already described C01 policy path (approved waiver, approved scholarship or valid installment arrangement), **but must identify** the owner/service for the exact alternate decision, its canonical approved evidence, Admission/Person/Institute correlation, Finance obligations/status, and which C01 rule permits `FinancialCleared`.

The current unallocated payment **cannot** be re-labelled an approved waiver, settlement or installment. If this path is selected, expressly decide whether a *later* F01/F02 allocation and payment variant remains within B18-J02 or moves to a separately governed scope. This option cannot be considered a successful F01/F02 settlement test.

## 4. Required controlled textual reconciliation

Current C01 WF-C01-04 §2.5 Step 2 says “Calculate financial obligations,” but BR-C01-025 and F01 give Finance sole monetary-calculation authority. **Proposed interpretation for approval:** C01 determines *applicable obligation types and clearance conditions*; F01 calculates the actual amounts and manages all Finance aggregates. If the approved interpretation is different, record the exact governing source and amend the conflicting text explicitly.

## 5. Executable acceptance matrix after, and only after, decision approval

| Gate | Positive evidence required | Negative evidence required |
|---|---|---|
| C01 → F01 | One authorized source fact yields one valid F01 obligation for the exact Admission/Person/Institute under the approved pre-Student contract (only if A approved). | No `AdmissionConfirmed` trigger before clearance; wrong Person/Institute, duplicate request and invalid policy rejected. |
| F01 → F02 | Published charge, precise Payment identity, eligible allocation and actual settlement from F02 for that charge. | Unallocated, partial-without-policy, surplus, reversed, uncertain or wrong-charge payments do not grant clearance. |
| F02 / alternate → C01 | Source-valid applicable Finance/policy evidence consumed by C01; same applicant's clearance decision and immutable history. | Reviewer replay, broad textual statuses, wrong admission references and stale policy versions do not unlock clearance. |
| C01 → C02 | Same applicant's `AdmissionConfirmed` then source-owned Student/StudentContext association. | No preconfirmation activation, no duplicate active StudentContext per Institute, no reuse of unrelated Person/Student history. |
| B18 audit | Typed owner inputs and provenance, cross-domain exact ID/tenant match, state progression and independently executed desktop/narrow evidence. | Fake completion, duplicated callbacks, missing owner evidence, reset/preset mismatch and prohibited cross-domain writes must fail. |

## 6. Decision record / approvals needed

- [ ] Authorize **A** or **B**, including explicit rejection/limitations of the other.
- [ ] Resolve exact owning event/business fact and allowed pre-Student identity and later C02 linkage.
- [ ] Identify full Finance/policy evidence and C01 clearance eligibility.
- [ ] Approve C01 Step 2 textual correction; determine L2/L3/L4/L6 dependent documents and versioning.
- [ ] Approve acceptance tests and authorize implementation; record the canonical decision ID/revision.

**Until approved:** this document has no authority to change canonical contracts. Hold J02 F01/F02→C01/C02 completion and certification; continue independent Batch 18 journeys under existing authorization.
