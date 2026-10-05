# L2-C01 — Admission Finance Clearance Evidence — Controlled Amendment

| Attribute | Value |
|---|---|
| Status | **ADOPTED — canonical additive L2 amendment v1.0.0** |
| Target | Locked L2-C01 Admissions Workflows, WF-C01-04 and §20.5 |
| Version | Additive amendment v1.0.0 |
| Decision date | 2026-10-02; adopted 2026-10-05 |
| Decision authority | Lead Architect; cross-domain Finance/Admissions review required |
| Tracks | Architecture issue #132, PR #135 and ADR-B18-FIN-001 |
| Source baseline | Architecture main `2bd00c68afaa84ec1d513d1f821a9c0feb53aa5b` |

> **Adoption record:** Lead Architect Option A+B2 direction was previously recorded in ADR-B18-FIN-001. On 2026-10-05 the project owner explicitly attested that the required Finance/source review approved this bounded contract and authorized proceeding. This amendment is adopted for Stage-5 J02 mock-operational scope. No new Finance composite event, production API, broker contract or backend release is authorized.

## 1. Verified source tension

L2-C01 WF-C01-04 §2.5 Step 2 currently directs Admissions to “Calculate financial obligations.” Its BR-C01-025 explicitly prohibits Admissions from calculating financial balances; BR-C01-026 assigns all financial records to Finance. F01 alone decides applicability, resolves Fee Structure and calculates monetary values. C01 already publishes `AdmissionApproved`, then separately determines/publishes its own `FinancialCleared`; C01 confirmation consumes `FinancialClearanceCompleted` by name in §20.5. Repository search for exact `FinancialClearanceCompleted` found only C01 in the inspected main-branch search; inspected F00/F01/F02/F07/F11, locked L3-F-007/F-009 and L4-F-007/F-009 do not identify a canonical publisher for that exact Finance event. This is a scoped verification finding, not proof that no further authorized source exists.

## 2. Adopted correction of the C01 monetary wording

**Replace** WF-C01-04 §2.5 Step 2's imperative `Calculate financial obligations.` with:

> Determine the applicable **admission obligation types**, policy-based clearance conditions and their institutional scope. Obtain the **authoritative F01 monetary obligations** and Finance-owned payment, allocation, settlement, receivable or other applicable owner-published evidence. Admissions must not calculate monetary values or balances, construct Student Charges, adjust a Finance aggregate or publish any Finance-owned event.

Step 3's existing Finance status read and all WF-C01-04 audit, negative, reversal and policy gates remain applicable. This wording is the adopted interpretation for this additive amendment; the underlying historical source text is not silently rewritten.

## 3. Adopted no-new-event clearance boundary

**CE-001 — Distinguish owner facts and decision:** C01 is the sole owner of its admission financial-eligibility/clearance state, policy-driven decision, immutable clearance history and `FinancialCleared` publication. F01 remains owner of Student Charge and outstanding monetary information; F02 remains owner of Payment/Allocation/Settlement and publishes its own canonical milestones; F07 remains owner of Receivable representations; F11 owns approved financial-assistance decisions, while **C07 owns scholarship eligibility evaluation and its authoritative eligibility records** consumed by F11. C01 never owns or recreates these Finance facts.

**CE-002 — Evidence-based C01 evaluation (B2):** Instead of assuming an undefined Finance publisher for a composite `FinancialClearanceCompleted` event, C01 MAY evaluate financial clearance from **verified, current, source-owned Finance facts** plus the applicable authorized admission policy, under this adopted amendment and approved owner-source evidence semantics. Absence, contradictory scope, insufficient/uncertain payment, missing owner evidence, mismatched policy version and active reversal/hold fail closed. `PaymentSettlementCompleted` alone is neither a substitute event nor automatic clearance for the complete required obligation set.

**CE-003 — Existing inbound term must not be silently repurposed:** Until a genuine approved source owner/contract for `FinancialClearanceCompleted` is identified (alternative B1), its C01 §20.5 usage shall be treated as an unresolved input contract rather than evidence of an extant Finance event. ADR-B18-FIN-001 selects **B2**. For this amendment, the `FinancialClearanceCompleted` entry in C01 §20.5 is governed as the following evidence class rather than as proof of an extant Finance composite event: `Authoritative Finance-owned charge/payment/allocation/settlement/receivable evidence required by the applicable admission clearance policy, through separately approved source contracts.` It is **not** a new event name.

**CE-004 — Clearance domain rules:** Under the approved policy the same canonical A02 Person, C01 Admission, A01 Tenant and authorized B01 institutional/Institute context must match throughout. C01 confirms that every currently applicable admission financial obligation is either fully covered by F01/F02 verified status or qualifies under an expressly applicable B01-owned institutional policy for installments, a C07-eligible/F11-approved scholarship or an F11-governed assistance waiver **with independently authoritative owner evidence**. F01 retains its separate owned Charge Policy; no alternative implies actual settlement. C01 must not transform zero-allocation funds into settlement or assume that clearing one charge clears other obligations. Its `FinancialCleared` decision is invalid where an adverse reversal or material owner-confirmed status change negates the qualifying facts; recovery follows C01's existing owned workflow without rewriting payment history.

**CE-005 — Admission/Student transition:** Only following a lawful C01 clearance decision (or its separately authorized policy-permitted alternate) may C01 proceed to its own `AdmissionConfirmed`, with independent admission state, verification and authority gates. C02 alone thereafter creates/reuses the canonical Student, creates StudentContext and authorizes activation, preserving one Active StudentContext per Institute and immutable original C01/F01/F02 lineage.

**CE-006 — Business-fact vs implementation distinction:** This amendment deliberately defines **no** new Finance aggregate, `FinancialClearanceCompleted` publisher, technical event, query, API, projection schema, permission code, or broker transport. Concrete interactions and correlation representations require approved L4/L5 tracing. If reliable, suitably timely current Finance evidence cannot be provided by existing owner contracts, no C01 clearance shall be inferred; the unresolved technical/business contract must be escalated before any journey completion claim.

## 4. Negative and positive requirements

| Case | Required result |
|---|---|
| All applicable published F01 charges are covered by verified F02 allocation/settlement and owner-valid current balance | C01 may grant clearance only if every additional admission/verification/policy condition also passes. |
| Policy permits installment, eligible F11 assistance or waiver for the same Admission | C01 applies only the particular approved policy condition and independently authoritative evidence; any remaining obligation continues under its Finance owner. |
| Unallocated advance, unrelated Payment/charge, ambiguous payer, wrong Person/Admission/Tenant/Institute | No inferred clearance; C01 remains pending or follows its governed negative/hold path. |
| Insufficient partial payment, excess unapplied liability, stale policy, reversal, unknown settlement status or missing Finance owner proof | No automatic admission clearance. |
| Repeat event/request for same cleared Admission | Prevent duplicate C01 state/event outcomes while retaining audit history. |
| Admission not actually Approved or student creation requested early | Reject progression regardless of apparent receipt/settlement. |

## 5. Adoption and downstream validation record

- [x] Lead Architect selected **B2** with C01-owned evaluation of authoritative source Finance facts.
- [x] Finance and Admissions owners validate exact evidence semantics, present-time consistency, distinct B01 institutional policy and F01 Charge Policy ownership, C07 scholarship eligibility/F11 assistance-decision lineage and recovery conditions.
- [x] Lead Architect approves the C01 Step 2 and §20.5 text amendments and identifies affected downstream locked L3-C-001/L3-F-001/L3-F-007/L3-F-009, L4 equivalents, L5 mappings and L6 J02 evidence.
- [x] The companion F01 pre-Student subject amendment is adopted as Gate A. Neither amendment alone certifies J02 runtime completion.
- [x] Exact adopted version and ADR are recorded; Stage-5 frontend validation remains a separate gate.

**Companion Gate A:** `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F01_PreStudent_Admission_Charge_Context_Amendment_2026-10-05.md`.
