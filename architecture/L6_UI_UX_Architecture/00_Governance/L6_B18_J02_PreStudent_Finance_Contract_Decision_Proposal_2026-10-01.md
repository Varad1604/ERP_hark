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

## 2B. Source-verified G2 disposition and exact decision language — 2026-10-02

**Inspection baseline:** Architecture `main` `2bd00c68afaa84ec1d513d1f821a9c0feb53aa5b` (post-G1 merge). This PR's proposal branch remains on an older merge base with disjoint changed files; this dossier records the newly checked source tree but does not claim the branch has been rebased or merged. All following conclusions are review evidence or **conditional recommendations**, not approved new architecture.

### 2B.1 Findings that constrain the decision

| Source | Verifiable constraint or tension | Disposition |
|---|---|---|
| L2-A02 Person & Identity §§3, 4 | **A02**, not A01, owns the canonical Person Aggregate; Person has exactly one canonical identity within its Tenant. Other domains reference immutable PersonID. A01 provides platform/tenant foundation, not Person ownership. | Any draft saying “A01 canonical Person” must be interpreted/corrected to A02 Person ownership with A01 tenant validation. |
| L2-C01 WF-C01-03, §9.6, §9.12 | C01 publishes `AdmissionApproved`; the Finance consumer list expressly includes `AdmissionApproved` and `AdmissionCancelled`. | `AdmissionApproved` is an **existing source event**, not a proposed new event name. Confirm exact event-specific scope/correlation and successful published outcome; do not assume a new API/schema. |
| L2-F00 §4 Principle 2; L2-F01 §§9.4–9.6 | F00 illustrates approved admission → published Admissions event → Finance-generated Admission Charge; F01 classifies Admissions as a canonical Charge Source and defines source metadata, including institutional/session/student scope and source identifier. | Supports investigating pre-confirmation charging; neither passage expressly authorizes omitting StudentContext in mandatory F01 validation. |
| L2-F01 §§13.4, 20.4–20.5, 22.4; locked F01 publication-authority amendment 2026-09-23 | Student Context occurs among **illustrative** calculation/context-validation inputs, and §22.4 lists Admission Confirmation/Enrollment/Registration/Admission Cancellation as **illustrative** upstream sources. Locked amendment permits standard source-driven publication subject to *all existing* policy/validation conditions. | Do not treat examples as a categorical prohibition or blanket permission. Authoritatively determine whether a prospective-admission source satisfies required Student Scope without an existing StudentContext, and explicitly reconcile the §22.4 illustration with the F00 approved-admission example. |
| L2-F01 §§9.7, 10.3; locked L3-F-001; L4-F-001 | One authoritative source obligation must create at most one canonical charge; F01 owns eligibility/policy, valuation, validation and `ChargePublicationCompleted`. L4-F-001 forbids a duplicate charge for the same obligation. | Specify source-obligation uniqueness using approved source identity and policy/version semantics. The proposal cannot define a new aggregate/lifecycle state or authorize admission-side monetary calculations. |
| L2-F02 §§12.4, 13.4–13.5, 17.2; locked L3-F-009; L4-F-009 | Zero allocation is a liability, partial settlement leaves outstanding balance, and `PaymentAllocationCompleted` / `PaymentSettlementCompleted` follow their actual F02 milestones. L4-F-009 prohibits inventing a `ReceiptReconciled` event. | Require authoritative payer/payment/charge matching and separate allocation/settlement evidence. A received advance is not admission financial clearance. |
| L2-C01 WF-C01-04 §§2.5, 3.2–3.6, §20.5; L4-C-001 | C01 owns `FinancialCleared` and consumes `FinancialClearanceCompleted` by name. F01 publishes charge milestones; F02 publishes payment milestones. **No exact Finance publisher/schema for C01's named `FinancialClearanceCompleted` has been established by the inspected F00/F01/F02 sources.** | **Separate mandatory decision**: trace and affirm an already-approved owner-published clearance evidence contract, or explicitly govern the missing Finance→Admissions evidence boundary. Never alias F02 settlement directly to C01 clearance. |
| L2-C02; locked L3-C-001; L4-C-001 | Only a confirmed Admission permits C02 Student/StudentContext creation, and only owner-authorized activation permits Active status; one Active StudentContext per Institute. | Define immutable source-to-created relationship, including Person, Admission and Institute; preserve original F01/F02 historical financial references on subsequent C02 linkage. |

**Authority caution:** the C01 “financial clearance completed” input and C01-owned `FinancialCleared` output must remain distinct. Do not invent a Finance publisher merely to close the wording discrepancy. The absence of an identified publisher in the inspected F00/F01/F02 corpus is a verified *inspection limitation and a contract gap to resolve*, not proof that no other approved Finance amendment could exist.

### 2B.2 Recommended conditional resolution wording for Lead Architect review

> *Proposed, not adopted:* C01's existing authoritative `AdmissionApproved` outcome may supply an Admissions-owned source obligation to F01 **only after** the governing L2 amendment explicitly validates an Admission-scoped prospective Person/Institute context as a permitted pre-Student Student Charge subject. A02 owns PersonID; C01 owns Admission identity and the initiating fact; applicable A01 tenant and B01 institution/Institute boundaries must be checked; F01 alone decides charge applicability, source classification, amount, structural/context validation, uniqueness and publication. Missing StudentContext is permitted **only if** the approved pre-Student source/eligibility exception explicitly says so, and it cannot be interpreted as an already-created Student or StudentContext. F02 separately resolves existing Payment identity and source, establishes valid charge allocation, and publishes its own applicable milestones. C01 consumes only the explicitly governed, source-valid financial-clearance evidence and then independently determines `FinancialCleared`; only later may C01 publish `AdmissionConfirmed` and C02 create/activate the authoritative StudentContext. Original Person/Admission/charge/payment/history references stay traceable without retrospective mutation.

This wording intentionally does **not** settle unresolved source policy, record shape, technical API, specific exception predicates, charge-to-new-Student linkage mechanism or missing Finance clearance fact publisher. Those require the explicit authority decisions below.

### 2B.3 Mandatory authority decisions (none inferred from “investigate Option A”)

- [ ] **F01 pre-Student eligibility:** confirm or reject Admission-scoped Student Charge without a StudentContext, and specify exact source-identity/validation conditions and applicable policy/version owner references. If rejected, stop Option A and formally revisit Option B.
- [ ] **Identity and lineage:** confirm canonical A02 PersonID, C01 Admission reference, A01 Tenant, B01 institutional/Institute scope and any required Programme/Session scope; approve how Finance preserves original references and subsequently links (without rewriting historical facts) to a legitimate C02 Student/StudentContext.
- [ ] **Exact originating fact and obligation uniqueness:** affirm C01 `AdmissionApproved` as the valid trigger *for this context* (or cite a different existing approved source fact), its publication prerequisites, source-obligation key and repeat/cancellation handling. No new event or technical contract is implied.
- [ ] **Finance semantics:** specify policy-valid partial/full allocation, payment-source identity and settlement evidence for the existing advance, plus outstanding/surplus, reversal and indeterminate-result handling under existing F02 rules.
- [ ] **Finance→C01 clearance evidence:** identify the exact approved publisher and meaning of `FinancialClearanceCompleted` or authorize a controlled C01/F-source reconciliation; approve the monetary/clearance conditions for this representative scenario without assuming `PaymentSettlementCompleted` is itself a clearance decision.
- [ ] **C01 wording and downstream verification:** approve the narrow WF-C01-04 §2.5 Step 2 correction against BR-C01-025 and F01 ownership, then identify required L2 amendments and L3-C-001/F-001/F-009, L4-C-001/F-001/F-009, locked L5 mapping and L6 J02 traceability/version checks.

**Recommended decision order:** first settle whether the existing F01 invariants support the prospective-Admission subject. In parallel resolve the C01 clearance-evidence publisher and Step 2 text conflict; then finalize F02 linkage and owner-controlled terminal conditions. Defer integration and terminal J02 claims until these approvals exist and tests show exact same-subject continuity.

---

## 2C. Two-gate formal decision package — escalation-ready, NOT adopted (2026-10-02)

A wider repository code search for exact `FinancialClearanceCompleted` returned **only** `L2-C01_Admissions_Workflows.md` among indexed main-branch files. The exact term occurs there as a consumed input; the same C01 source publishes **`FinancialCleared`** as its own subsequent decision. Additional source inspection covered L2-F07 Accounts Receivable, the locked F07 amendment, L2-F11 financial aid, locked L3-F-007/L3-F-009, and corresponding L4-F-007/L4-F-009. These documents establish Finance-owned, *distinct* obligation, settlement, receivable and aid representations; no inspected source defines an authoritative publisher of the C01-named composite event. **This is a scoped negative search finding, not a proof that no unpublished or non-indexed approved contract exists.**

### Gate A — pre-Student Student Charge subject

**Proposed governing resolution, awaiting Lead Architect's contract approval:** An already-published C01 `AdmissionApproved` may represent a legitimate admission-fee business obligation *before* C01 confirmation where applicable F01 Charge Policy makes the obligation due. F01, not C01, must decide whether the exact approved Admission, immutable A02 PersonID and authorized tenant/institution/Institute plus relevant academic/session/financial-period scope satisfy F01's mandatory structural, context, policy and financial validation. An expressly approved **prospective Admission-scoped context** may be used instead of a not-yet-existent C02 StudentContext *only for that governed class of pre-Student obligations*. This is a proposed narrowly conditional interpretation, not a general StudentContext waiver.

Required source amendments if approved:
- **L2-F01 controlled amendment** must state whether the prospective Admission subject is legal, its accepted source-context requirements and precise scope of the StudentContext-absent case; preserve all unaffected F01 invariants. Review §§9.6, 12.4, 13.4, 20.4–20.5 and 22.4 explicitly. Disallow charge publication on unresolved A02 identity, invalid institution/Institute or absent required charging policy.
- **L2-C01 alignment** may cite the existing `AdmissionApproved` event and its existing Finance consumption from §9.12, clarify the qualifying approval/business-obligation timing and require owner-accurate source identifiers. It cannot create a Student Charge, compute its monetary value or emit F01 publication events.
- **C02 linkage rule** must be recorded only as an immutable association of the *existing* prospective Admission/Person financial lineage with the legitimately subsequent C02 Student/StudentContext. Whether the link is F01-maintained, a read projection or another approved mechanism remains an L5 design choice contingent on approved L2 contracts; it cannot rewrite the original source reference.

**Gate A remains unapproved** until the full identity/context exception and exact F01 validation predicates are accepted, not merely because F00 includes an illustrative approved-admission charge path.

### Gate B — authoritative Finance evidence into C01

**Existing verified parts:** F01 owns the published Student Charge, F02 owns Payment Source, Payment allocation and applicable `PaymentAllocationCompleted` / `PaymentSettlementCompleted`; F07 represents receivable state from Finance facts; C01 owns its policy-dependent `FinancialCleared` admission decision. C01 already references an inbound `FinancialClearanceCompleted` but no inspected Finance source owns that exact published event.

**Explicit architectural choice required, do not silently choose:**

- **B1 — reuse existing approved source contract:** supply the precise current approved source document, semantic owner, business-fact name, eligibility basis and version proving that `FinancialClearanceCompleted` already exists as a valid published Finance/clearance decision applicable to the same Admission and source obligations. Reconcile C01 consumption and required traceability against that actual contract.
- **B2 — controlled new/revised cross-domain contract:** if no existing owner is verified, Lead Architect must separately designate a suitable canonical Finance capability for authoritative *clearance evidence aggregation* or approve C01's own governed assessment from source-owned Finance obligation/payment/receivable facts **without falsely attributing a new Finance-owned decision**. Review the C01 named input for rename/removal through controlled source approval; assign any new event's publisher **only after** an explicit decision. If C01 assesses compliance, it can publish its existing `FinancialCleared` only when its institutional-policy and source-validation requirements are actually met; it cannot calculate amounts or mutate Finance aggregates.

Do not equate `PaymentSettlementCompleted` with full Admission clearance: several qualifying obligations, installment-policy exceptions, reversal holds and approved waiver/scholarship conditions can affect admission eligibility differently. Any adopted contract needs exact admission, payer, canonical charge, current outstanding position, applicable policy version, effective-time and reversal/exception semantics without presuming a new schema.

### Cross-gate mandatory tests and stop conditions

| Evidence family | Minimum independently verifiable condition |
|---|---|
| Source identity | The existing `AdmissionApproved` belongs to the exact A02 Person/Tenant, C01 Admission, authorized B01 institution/Institute and applicable session/scope; unrelated applicants and stale or cancelled admissions fail. |
| Charge authority | F01 policy applicability, permitted pre-Student context, unique source obligation, correct amount owner and actual `ChargePublicationCompleted` must be verified. Missing or unresolved StudentContext absence *without an approved exception* fails. |
| Allocation | F02 actual Payment Source/Payment ID and the precise F01 charge are matched; only legitimately allocable value produces allocation and derivative settlement. Unallocated liability, ambiguity, surplus, invalid partial payment, failed or reversed payment cannot be disguised as clearance. |
| Clearance | Gate B produces only its approved owner-specific source evidence and policy-eligible result for the same Admission and obligation set; unrelated Finance events alone do not grant C01 `FinancialCleared`. |
| Confirmation and activation | C01 confirmation occurs only after its own permitted clearance determination; C02 thereafter creates/links StudentContext without duplicating canonical Student identity or violating one Active StudentContext per Institute. |
| Traceability | Approval record states authoritative source revision, exactly who owns each fact, L3-C-001/F-001/F-007/F-009 and L4 correspondences, L5 provider/consumer contribution and L6 J02 genuine-vs-mock evidence boundaries. |

**Approval dependency:** Gate A and Gate B are independent decisions, both necessary for Option A's representative end-to-end J02 route. The previously authorized selection of Option A for *formal investigation* did not resolve either. This proposal, and the branch containing it, must remain unmerged and must not unblock dependent J02 until those decisions and source-controlled amendments are approved.

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
