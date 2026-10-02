# ADR-B18-FIN-001 — Pre-Student Admission Charge and Admissions Clearance Authority

| Field | Value |
|---|---|
| ADR ID | ADR-B18-FIN-001 |
| Decision date | 2026-10-02 |
| Decision authority | Lead Architect (approval expressly given in SutraERP project conversation, following G2 draft review) |
| Decision status | **Lead Architect design direction approved; controlled incorporation pending mandatory domain/source review and evidence gates** |
| Governs | Architecture issue #132 and PR #135; B18-J02 |
| Architecture baseline inspected | `main` `2bd00c68afaa84ec1d513d1f821a9c0feb53aa5b` |
| Change type | Cross-domain L2 behavioral clarification; additive scoped Finance source contract and C01 clearance-input correction |
| Replaces | No prior canonical architecture until formally incorporated |
| Scope of approval | Investigated Option A with the source-safe Gate A restrictions and **Gate B2** C01-owned evaluation of existing authoritative Finance evidence |
| Out of scope | New event schema/API, new Finance clearance publisher, invented policy, new Student Charge aggregate/lifecycle, bypass of human Finance reviews, production J02 certification |

## 1. Need and verified authoritative constraints

Existing C01 `AdmissionApproved` is published by C01 and listed for Finance consumption; F00 depicts approval-triggered admission charges; F01 recognizes Admissions as a Charge Source but does not conclusively define the prospective Admission financial subject when no C02 StudentContext yet exists. Locked F01 2026-09-23 publication authority assigns all Student Charge evaluation, calculation, validation and publication to F01. A02 owns the immutable canonical Person within a Tenant; A01 governs Tenant; C01 owns Admission; C02 alone creates/activates Student/StudentContext after lawful C01 confirmation.

C01 owns its policy-dependent `FinancialCleared` outcome but includes `FinancialClearanceCompleted` as a consumed input in §20.5. Main-branch exact-term search identified only the C01 occurrence; the inspected F00/F01/F02/F07/F11 and locked L3/L4 Finance scenarios establish separate charge, payment, allocation, settlement, receivable and aid owners, **not** an exact owner-published contract for that composite input. This is a scoped evidence gap, not an assertion about every possible source. C01 WF-C01-04 §2.5 Step 2 says to calculate financial obligations, contradicting BR-C01-025 and F01's sole monetary-calculation authority.

## 2. Approved design direction — two jointly necessary gates

### Gate A — conditionally permit prospective Admission financial subject in F01

Approve the **architectural direction** of a narrowly scoped, policy-valid Admission-originated F01 Student Charge **before** C02 Student/StudentContext establishment. `AdmissionApproved` is the existing C01 source event; an approved admission by itself does **not** automatically generate a charge or certify its source-context completeness.

Controlled incorporation is contingent on explicitly defining and validating:
1. the C01 source's immutable Admission ID, authenticated A02 PersonID, A01 Tenant, owner-verified institutional/Institute boundary and every actually applicable Programme/Session/financial-period/charge-policy scope;
2. the approved means of obtaining the event-specific Person/Institute and applicable other source references that the generic C01 event structure does not guarantee;
3. F01-owned source resolution, duplicate source-obligation prevention, Charge Policy/Fee Structure, amount calculation, context and publication-readiness validation;
4. the **exception only for the absent C02 StudentContext** under that approved prospective source and applicable policy, never a general waiver of F01's other validation requirements or a newly created Student;
5. owner-correct cancellation, reversals and subsequent immutable same-Admission/Person-to-C02 association without changing historic financial records.

The draft in PR #135 (`L2-F01_PreStudent_Admission_Charge_Context_Amendment_PROPOSED_2026-10-02.md`) is an implementation candidate for this **decision direction**, not yet an operative canonical exception.

### Gate B — choose B2: C01-owned admission clearance from owner-published Finance evidence

Approve the **architectural direction** of C01 determining its own policy-dependent admission financial eligibility from independently authoritative current Finance facts (not from a newly invented Finance `FinancialClearanceCompleted` event). F01 owns individual charges/outstanding financial position; F02 owns receipt, Payment Source, allocation and settlement evidence; F07 owns its receivable representation where applicable. B01 retains versioned institutional business-policy definitions; F01 retains its own charging-policy and calculation authority; C07 owns scholarship eligibility and F11 the subsequent financial-assistance decision. These ownership boundaries must remain distinct.

Before canonical adoption:
1. explicitly reconcile/replace C01's orphan inbound `FinancialClearanceCompleted` reference in §20.5 with the approved *business evidence class*, **not** a fictional replacement event;
2. enumerate the actual existing owner-published contracts or formally approve any missing source-consumption contracts, including current-time, same-Admission/Person/Institute matching, completeness of all required charges, installment/aid/waiver scope and adverse reversal/hold handling;
3. replace WF-C01-04 Step 2 with source-safe wording: C01 identifies required **obligation types and institutional clearance conditions**, and consumes authoritative **F01-calculated** amounts and Finance-owned status. C01 never calculates money, modifies Finance records or publishes F01/F02 source facts;
4. require C01 to publish its existing `FinancialCleared` only after its independently valid admission-policy decision, then satisfy separate confirmation gates before its existing `AdmissionConfirmed`; C02 acts only afterward.

This selects B2 and *does not* select B1. Should reviewers prove an actual pre-existing `FinancialClearanceCompleted` source contract, record a controlled reconsideration rather than silently alter this decision.

## 3. Explicit non-decisions and failed shortcuts

- **No defined event-specific C01 Person/Institute payload or API** exists merely because generic C01 event metadata includes an Admission, Institution and Campus. Every extra required relationship must have a specifically approved authoritative source contract.
- **No new Finance composite-clearance event, publisher or alias** is created. F02 `PaymentSettlementCompleted` is not C01 `FinancialCleared` and does not establish the entire admission clearance decision.
- **No early StudentContext** and no implicit linkage to an unrelated Student/Institute. C02 alone enforces canonical Student identity and one Active StudentContext per Institute.
- **No receipt-only clearance.** An F02 Unallocated Advance stays a liability pending approved allocation; a partial settlement needs separately authorized payment policy if it is to qualify.
- **No unconditional policy exception.** B01 institutional policy, F01 Charge Policy and C07→F11 assistance lineage retain their separate authorities.
- **No production implementation authority** is conferred by this design-direction approval absent precise, adopted and review-validated contracts.

## 4. Mandatory incorporation and review record

| Gate | Evidence needed | Status at decision |
|---|---|---|
| Lead Architect chooses Option A and B2 | Explicit project-conversation approval following the immediately preceding G2 review and proposal presentation | **Recorded as approved design direction**, not a GitHub PR review submission |
| Finance domain owner (Chetan) | F01 permissible prospective identity, policy applicability, validations, duplicate/reversal/amount controls; F02/F07 source-evidence availability | **Pending human review** |
| Finance Review Team (Varad and Devang) | Independently review Finance changes under locked Constitution Vol. 5 | **Pending human review** |
| Admissions and affected source owners | C01 source fields; A02 Person/Tenant; B01 institution/policy; C07/F11 aid where applicable; C02 delayed StudentContext | **Pending source-owner review** |
| Contract completeness | Actual identity source and exact eligible current F01/F02/F07 owner evidence; C01 input reconciliation | **Pending; adoption hard gate** |
| Canonical source incorporation | Approved scoped L2-F01 and L2-C01 amendments, version/status accuracy, approved references and publication/validation | **Not yet incorporated** |
| Downstream | Impacted locked L3-C-001/F-001/F-007/F-009, L4 equivalents, appropriate L5 and L6 evidence; no lower-level new authority | **Pending** |
| J02 testing | Same-subject valid/invalid charge, allocation, policy clearance and only later C02 activation under real approved contracts | **Not authorized as complete** |

Until mandatory peer reviews and exact source-context/clearance-evidence contracts are verified, **PR #135 must stay unmerged and its two L2 amendment candidates remain noncanonical.** The Lead Architect's selected direction narrows the decision; it cannot fabricate missing evidence or other reviewers' approvals.

## 5. Trace and revision integrity

Reference proposal PR #135, issue #132 and these separately proposed files:
- `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F01_PreStudent_Admission_Charge_Context_Amendment_PROPOSED_2026-10-02.md`
- `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C01_Admission_Finance_Clearance_Evidence_Amendment_PROPOSED_2026-10-02.md`

Upon peer review, record actual reviewer identity, comments, explicit decision on each unresolved exact contract, adopted source revision, downstream impact and required GitHub review/merge gate. A later canonical incorporation record may supersede this *pending-incorporation status* but must preserve the history of this Lead Architect selection and any material changes.
