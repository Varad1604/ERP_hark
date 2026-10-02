# L2-C01 — Admission Finance Clearance Evidence — Proposed Controlled Amendment

| Attribute | Value |
|---|---|
| Status | Proposed — NOT approved canonical architecture |
| Target | Locked L2-C01 Admissions Workflows, WF-C01-04 and §20.5 |
| Proposed version | Separate additive amendment v1.0.0 after formal review |
| Date | 2026-10-02 |
| Decision authority | Lead Architect; cross-domain Finance/Admissions review required |
| Tracks | Architecture issue #132 and proposal PR #135 |
| Source baseline | Architecture main `2bd00c68afaa84ec1d513d1f821a9c0feb53aa5b` |

> This is an **approval candidate only**. No existing L2 source is changed by this document, no new Finance event is authorized and no downstream J02 implementation is released. This proposal must be adopted through explicit architectural approval and required reviews.

## 1. Verified source tension

L2-C01 WF-C01-04 §2.5 Step 2 currently directs Admissions to “Calculate financial obligations.” Its BR-C01-025 explicitly prohibits Admissions from calculating financial balances; BR-C01-026 assigns all financial records to Finance. F01 alone decides applicability, resolves Fee Structure and calculates monetary values. C01 already publishes `AdmissionApproved`, then separately determines/publishes its own `FinancialCleared`; C01 confirmation consumes `FinancialClearanceCompleted` by name in §20.5. Repository search for exact `FinancialClearanceCompleted` found only C01 in the inspected main-branch search; inspected F00/F01/F02/F07/F11, locked L3-F-007/F-009 and L4-F-007/F-009 do not identify a canonical publisher for that exact Finance event. This is a scoped verification finding, not proof that no further authorized source exists.

## 2. Proposed correction of the C01 monetary wording

**Replace** WF-C01-04 §2.5 Step 2's imperative `Calculate financial obligations.` with:

> Determine the applicable **admission obligation types**, policy-based clearance conditions and their institutional scope. Obtain the **authoritative F01 monetary obligations** and Finance-owned payment, allocation, settlement, receivable or other applicable owner-published evidence. Admissions must not calculate monetary values or balances, construct Student Charges, adjust a Finance aggregate or publish any Finance-owned event.

Step 3's existing Finance status read and all WF-C01-04 audit, negative, reversal and policy gates remain applicable. Exact wording must be reviewed against the full locked source before adoption.

## 3. Proposed no-new-event clearance boundary — requires decision

**CE-001 — Distinguish owner facts and decision:** C01 is the sole owner of its admission financial-eligibility/clearance state, policy-driven decision, immutable clearance history and `FinancialCleared` publication. F01 remains owner of Student Charge and outstanding monetary information; F02 remains owner of Payment/Allocation/Settlement and publishes its own canonical milestones; F07 remains owner of Receivable representations; F11 owns approved financial-assistance decisions within its defined eligibility boundary. C01 never owns or recreates these Finance facts.

**CE-002 — Evidence-based C01 evaluation (proposed B2 variant):** Instead of assuming an undefined Finance publisher for a composite `FinancialClearanceCompleted` event, C01 MAY evaluate financial clearance from **verified, current, source-owned Finance facts** plus the applicable authorized admission policy, provided this exact consumption contract and any needed C01 §20.5 source-text change are expressly approved. Absence, contradictory scope, insufficient/uncertain payment, missing owner evidence, mismatched policy version and active reversal/hold fail closed. `PaymentSettlementCompleted` alone is neither a substitute event nor automatic clearance for the complete required obligation set.

**CE-003 — Existing inbound term must not be silently repurposed:** Until a genuine approved source owner/contract for `FinancialClearanceCompleted` is identified (alternative B1), its C01 §20.5 usage shall be treated as an unresolved input contract rather than evidence of an extant Finance event. Formal adoption must explicitly choose **B1** (cite actual approved publisher and versioned source) or **B2** (approved C01 consumption of existing authoritative Finance facts with controlled change to §20.5). If B2 is adopted, proposed replacement for the `FinancialClearanceCompleted` entry in C01 §20.5 is: `Authoritative Finance-owned charge/payment/allocation/settlement/receivable evidence required by the applicable admission clearance policy, through separately approved source contracts.` It is **not** a new event name.

**CE-004 — Clearance domain rules:** Under the approved policy the same canonical A02 Person, C01 Admission, A01 Tenant and authorized B01 institutional/Institute context must match throughout. C01 confirms that every currently applicable admission financial obligation is either fully covered by F01/F02 verified status or explicitly qualifies for an existing policy-approved installment, F11-approved financial assistance, waiver or valid exception **with independently authoritative evidence and exact source owner**. C01 must not transform zero-allocation funds into settlement or assume that clearing one charge clears other obligations. Its `FinancialCleared` decision is invalid where an adverse reversal or material owner-confirmed status change negates the qualifying facts; recovery follows C01's existing owned workflow without rewriting payment history.

**CE-005 — Admission/Student transition:** Only following a lawful C01 clearance decision (or its separately authorized policy-permitted alternate) may C01 proceed to its own `AdmissionConfirmed`, with independent admission state, verification and authority gates. C02 alone thereafter creates/reuses the canonical Student, creates StudentContext and authorizes activation, preserving one Active StudentContext per Institute and immutable original C01/F01/F02 lineage.

**CE-006 — Business-fact vs implementation distinction:** This proposal deliberately defines **no** new Finance aggregate, `FinancialClearanceCompleted` publisher, technical event, query, API, projection schema, permission code, or broker transport. Concrete interactions and correlation representations require approved L4/L5 tracing. If reliable, suitably timely current Finance evidence cannot be provided by existing owner contracts, no C01 clearance shall be inferred; the unresolved technical/business contract must be escalated before any journey completion claim.

## 4. Negative and positive review requirements

| Case | Required result if this amendment is approved |
|---|---|
| All applicable published F01 charges are covered by verified F02 allocation/settlement and owner-valid current balance | C01 may grant clearance only if every additional admission/verification/policy condition also passes. |
| Policy permits installment, eligible F11 assistance or waiver for the same Admission | C01 applies only the particular approved policy condition and independently authoritative evidence; any remaining obligation continues under its Finance owner. |
| Unallocated advance, unrelated Payment/charge, ambiguous payer, wrong Person/Admission/Tenant/Institute | No inferred clearance; C01 remains pending or follows its governed negative/hold path. |
| Insufficient partial payment, excess unapplied liability, stale policy, reversal, unknown settlement status or missing Finance owner proof | No automatic admission clearance. |
| Repeat event/request for same cleared Admission | Prevent duplicate C01 state/event outcomes while retaining audit history. |
| Admission not actually Approved or student creation requested early | Reject progression regardless of apparent receipt/settlement. |

## 5. Governance prerequisites

- [ ] Lead Architect selects **B1** with precise existing approved producer and contract or **B2** with approved C01-owned evaluation of authoritative source Finance facts.
- [ ] Finance and Admissions owners validate exact evidence semantics, present-time consistency, policy ownership/version and recovery conditions.
- [ ] Lead Architect approves the C01 Step 2 and §20.5 text amendments and identifies affected downstream locked L3-C-001/L3-F-001/L3-F-007/L3-F-009, L4 equivalents, L5 mappings and L6 J02 evidence.
- [ ] The companion proposed F01 pre-Student subject amendment receives its separate Gate A approval. Neither proposal independently authorizes J02.
- [ ] Exact adopted version, Architecture decision record and repository merge/validation gates are recorded.

**Companion Gate A:** `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F01_PreStudent_Admission_Charge_Context_Amendment_PROPOSED_2026-10-02.md`.
