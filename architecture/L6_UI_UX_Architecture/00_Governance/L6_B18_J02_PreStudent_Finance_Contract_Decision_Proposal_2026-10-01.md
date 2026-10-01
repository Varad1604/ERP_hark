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
