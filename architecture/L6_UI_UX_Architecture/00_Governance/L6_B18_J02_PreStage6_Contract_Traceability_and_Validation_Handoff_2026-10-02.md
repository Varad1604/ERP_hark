# B18-J02 — Pre-Stage-6 Contract Traceability, Risk and Validation Handoff

| Attribute | Verified or governed value |
|---|---|
| Artifact status | **Pre-Stage-6 review package only; not canonical L2 and not journey acceptance** |
| Prepared | 2026-10-02 |
| Lead Architect decision | ADR-B18-FIN-001: Option A prospective Admission-origin F01 charge + B2 C01-owned clearance from authoritative Finance facts — *design direction approved*, incorporation conditional |
| Architecture inspected baseline | `main` `2bd00c68afaa84ec1d513d1f821a9c0feb53aa5b` |
| Architecture proposal | Issue #132 / PR #135, branch `batch18-j02-finance-clearance-decision-proposal-20261001` |
| Frontend inspected | Open PR #46, `stage5-batch18-cross-domain-journey-exception-convergence` (2026-10-02 read); mock-review evidence only |
| Human Finance review | **Deferred by Lead Architect to Stage 6**; not performed, not passed, not waived |
| Canonical adoption and J02 final certification | **BLOCKED** until exact approved contracts, applicable peer review and integration evidence exist |

## 1. Decision/evidence classification

The Lead Architect selected the architectural **direction** of Option A and Gate B2. ADR-B18-FIN-001 records that choice. This does not itself prove missing event-specific C01 identity/context data, grant a new F01 API, establish an F02→C01 composite event, or substitute for the mandatory Finance/domain reviews under locked Constitution Volume 5. The two L2 candidate documents in PR #135 explicitly remain Proposed and cannot be represented as merged canonical amendments.

**Stage-6 deferral instruction:** Perform all independent read-only validation and proposal refinement that can be completed safely now; preserve the human Finance review and contract-adoption gates for Stage 6 rather than marking them implicitly satisfied or triggering unnecessary CI. No Stage-5 frontend integration code should imitate an unresolved production contract.

## 2. Source-to-behavior traceability matrix

| Boundary or concern | Current inspected authority / exact revision | Verified fact | Proposed / unresolved contract requirement | Acceptance evidence |
|---|---|---|---|---|
| Canonical Person | `L2-A02_Person_and_Identity_Workflows.md` blob `ed5c037b003df5b602c3bea0e14bcef869675c45` | A02 owns canonical Person and immutable tenant-scoped PersonID; A01 supplies Tenant foundation. | Approve authoritative PersonID resolution for this specific C01 Admission and source event; do not infer from matching display names. | Wrong Person and cross-Tenant/Institution negative tests; one canonical identity. |
| Institution/Institute/policy | `L2-B01_Institution_Workflow.md` blob `3f2c174ef3ddfd310b44d6af1633a25820ab2808` | B01 owns versioned institutional business policy; different from F01-owned Charge Policy. | Verify actual Institute/institutional identity mapping and effective policy/version for C01 clearance and F01 context without transferring B01 ownership. | Reject mismatched Institute and stale policy; institutional policy context trace. |
| Approved source | `L2-C01_Admissions_Workflows.md` blob `e37922977e70b9f6119f78630dc929b104de6aec`; locked `L3-C-007` blob `8150395ea8b9eb376643d5e5bd0c35294ff02707` | C01 owns `AdmissionApproved`; Finance is listed consumer; L3-C-007 ends at approval, before financial clearance. Generic C01 event fields identify Admission/Institution/Campus, but not mandatory Person/Institute/Programme/Session fields. | Agree event-specific identity or a separately authorized source-context retrieval contract; valid non-cancelled Admission and scoped source-obligation uniqueness. | One actual valid C01 fact, duplicate-replay guard and bad/stale subject rejection. |
| Prospective F01 obligation | `L2-F00` blob `d13581733d2ab96c09a620bd8d15207e4394e699`; `L2-F01` blob `84f65e37ed7f0c1874f3ca6d757238ccb1a31737`; locked F01 publication amendment blob `6805b407776b6b01e6e57b95a171e7c18c4ea04c`; locked L3-F-001 blob `3596a9ca5ebbfd8d547249742daee0cb2004e340`, L4-F-001 blob `81f786248e88b903ae4f86dad1189a06d707b4bc` | Admissions is recognized F01 Charge Source; standard F01 source-driven charge does not universally require manual approval; F01 alone calculates, validates and publishes. | Adopt narrow prospective-Admission subject exception only after source and Finance review; specify all required predicates, applicable policies, single originating obligation and immutable subject correlation. | `ChargePublicationCompleted` owned by F01 after applicable validation; reject duplicate, no-policy and wrong-context charges. |
| F02 receipt and settlement | `L2-F02` blob `3151482ee2eb16509ad76bbf0a290a601a41b91a`; L3-F-009 blob `59d778aacd16d55a40faf3ebe809feb8871258ab`; L4-F-009 blob `604d27d54df348f20d9846036ed01ad6ad6840a7` | Unallocated Advance is a Liability; settlement derives from validated allocation; F02 owns milestone publications. | Approve/reuse the precise owner-authorized payment-to-new-charge correlation, current settlement/outstanding evidence and reversal/uncertainty behavior. | Same F01 charge allocated by actual owner-controlled F02 outcome; receipt-only, excess, partial, ambiguous and reversed negatives. |
| C01 clearance | `L2-C01` WF-C01-04 §§2.5,3.2–3.6, §20.5; proposed Gate B2 L2-C01 amendment in PR #135 | C01 owns `FinancialCleared`; inspected current sources do **not** prove an owner-published `FinancialClearanceCompleted` input. C01 Step 2 wording conflicts with BR-C01-025. | Approved source-safe repair: C01 identifies obligation types and institutional eligibility, consumes exact *authoritative current* Finance facts and independently decides clearance. Exact source evidence contracts, timeliness and policy-version requirements are still unverified. | No C01 clearance from unallocated/partial or wrong-subject receipt; clear only on the correct full eligible obligation set or a genuinely approved alternative. |
| Aid variant (conditional only) | `L2-F11` blob `a58bb93afae7d0b13bcf0e5a7569c7929350554c` and L2-C07 | C07 owns scholarship eligibility; F11 owns resulting financial assistance decision, while B01 policy scope remains independent. | Any scholarship/waiver/installment exception needs its own exact approved source and applicable policy, not a generic F02 settlement claim. | Never relabel unapplied payment as aid/clearance; require signed-off relevant evidence. |
| C01→C02 handoff | `L2-C02` blob `583d520d92a617165646c94b2c5b2f5d306813fd`; L3-C-001 blob `f3085f9b24bba6011391cd68279d0919b3368307`; L4-C-001 blob `76fc66f843d2f28b29b96fa4b585bca5cd45e077` | C01 `AdmissionConfirmed` is a distinct later fact; C02 exclusively establishes canonical Student/StudentContext and authorized Active state with one Active context per Institute. | Adopt immutable historical source association from prospective C01/F01/F02 chain to subsequent C02 identity; no retroactive Finance rewriting. | Same Admission/Person/Institute linkage, no early context, no duplicate canonical Student/context; only C02 publishes activation. |
| L5 implementation and L6 | L4-to-L5 `section-c.md` blob `a0861485e44f5560a771d96867cab8d6cc146cf2`; L6 PR #46 J02 owner/read-model scripts | L5 has generic owner-safe transport/persistence/verification contributions; no inspected case-specific approved pre-Student C01/F01/F02 implementation contract. L6 reviewer-local mock and synthetic inputs are not business evidence. | Post-L2-approval update exact L4 behavior IDs, L5 technical contribution/correlation semantics and J02 real-vs-mock acceptance mappings; no new API solely from this package. | Static/source guards first, targeted typecheck/build, then Stage-6 genuine same-subject browser evidence. |

The L5 section-c mapping revision has been verified above; validate its precise scenario-specific applicability at Stage 6 rather than assuming it establishes a dedicated pre-Student Finance evidence transport contract. L3-C-001's current wording that Finance “provides authoritative financial-clearance outcome” requires an **impact review** after B2 is adopted: preserve Finance ownership of factual financial status while attributing the admission clearance *decision* to C01. Do not silently rewrite locked L3 now.

## 3. Exact frontend evidence and nonclaims (current B18 working branch)

| Existing inspected source | Content actually evidenced | What must NOT be inferred |
|---|---|---|
| `scripts/stage5-batch18-journeys-part1-manifest.json` | J02 references L6-RWF-0052 / L3-C-007 and L6-RWF-0053 / L3-C-001 and explicitly blocks runtime completion. | A static two-step reference is not an executable F01/F02/C01/C02 path. |
| `app/mock/operations/batch18-j02-admissions-owner.mjs` blob `f60c8524184487af06e03b2bad3ee4b8fba107dc` | Sample, owner-local `AdmissionApproved`; message explicitly states Finance clearance, confirmation and Student creation absent. | This does not authorize a real Finance event/API or clearance. |
| `app/mock/operations/batch18-j02-finance-owner.mjs` blob `75a4e8cedde146b4640945c67818264548bb41e4` | Sample F02 reconciliation retains zero-allocation receipt as Liability and no settlement/Admissions clearance. | An advance cannot be used to fake F01 charge, F02 settlement or C01 decision. |
| `app/mock/read-models/j02-admission-progress.mjs` blob `987212de859258ab3d875204b050d344d6480428` | Correctly keeps `cleared`, `confirmed`, `activated` false absent owner-backed contracts. | Display progression is not an authoritative mutation. |
| `app/mock/read-models/admission-student-continuity.mjs` blob `068235e3545b4895bb12076a02a46248c245733d` | Owner reference checks and synthetic-only test injection are separated from source facts. | Successful synthetic candidate cannot certify real same-subject continuity. |
| `scripts/validate-stage5-batch18-j02-continuity.mjs` blob `6a8cca4929cf1d85e24b61ca7bdd9d99aecefbb1` | Contains explicit source-negative guards and synthetic candidate noncertification checks. | Script content inspection is not a claim that it was executed or passed in this W1 run. |

Current frontend PR #46 branch inspected against frontend `main` has many additional frontend-only commits and may move independently. Do not cherry-pick, reset, force-push, edit shared package files or dispatch self-hosted CI from W1. Re-fetch its exact head and diff in Stage 6 before any code integration.

## 4. Pre-Stage-6 acceptance design (specification, not executed tests)

1. **Source chain:** An owner-published C01 `AdmissionApproved` with exact admitted subject is verified by F01 using specifically approved Person and Institute authority; replay of same originating obligation does not duplicate Charge.
2. **Charge validation:** Correctly scoped effective policy produces one F01-owned `ChargePublicationCompleted`. Unsupported pre-Student subtype, wrong subject, missing policy/context, cancellation or source mismatch fail closed.
3. **Payment continuation:** F02 matches a genuine existing Payment and eligible charge, preserves F02 Payment Source, creates only valid allocation/settlement and retains original unallocated/surplus/reversal liability history.
4. **Admission clearance:** Only source-valid, suitably current Finance facts *for all applicable obligations* are evaluated against the correct B01 institutional version plus C01-governed admission policy; C01 alone records/publishes `FinancialCleared`. No new `FinancialClearanceCompleted` publisher is manufactured.
5. **Confirmation/activation:** The same approved subject alone reaches C01 `AdmissionConfirmed`; C02 alone establishes or legitimately reuses canonical Student and activates the correct Institute-specific StudentContext with valid Registrar authorization.
6. **Negative matrix:** Wrong Person, different Admission, different Tenant/Institute, duplicate source, stale policy, unknown receipt, zero allocation, under-allocation, surplus, reversal, uncertain outcome, missing required obligations, no confirmation, unrelated historical Student and second Active StudentContext each remain blocked.
7. **Frontend reproducibility:** From normal product entry points, verify shared owner-issued references across Admissions → Finance → Student. Reset/preset, desktop and narrow evidence must not preserve misleading prior cleared/activated states. Reviewer-only synthetic tests stay clearly labeled and cannot constitute real backend integration.

## 5. Stage-6 deferred human approval gate — exact actions

**Reviewers and responsibility:** Chetan — Finance domain owner; Varad and Devang — Finance Review Team per locked Constitution Volume 5. Admissions C01 (Varad) and any genuinely affected A02, B01, C02, C07/F11 owners must validate their domain boundaries as applicable. These are human role assignments, **not verified GitHub usernames**; do not guess identities.

**Review checklist:**
- [ ] Review the companion F01 prospective Admission amendment PS-001…PS-006 and its exact event-specific identity-source sufficiency.
- [ ] Review C01 B2 evidence-class selection and exact existing owner-contracts/timeliness; correct §20.5 and WF-C01-04 Step 2 only via approved L2 adoption.
- [ ] Determine approved event-specific Person/Institute/Programme/Session evidence without inventing a payload, ID join, broker event or API.
- [ ] Reconcile B01 versioned institutional policy, F01 charging policy and C07/F11 financial-assistance ownership.
- [ ] Review cancellation/reversal/uncertainty semantics and complete required-obligation coverage.
- [ ] Sign off owner/source review and exact final L2 amendment revisions; complete constitutional PR/ADR/index/version/merge checks.
- [ ] Update the affected locked L3/L4/L5 traceability only when upstream L2 amendments are canonical; validate J02 product journey with actual same-subject owner evidence separately.
- [ ] Record test execution/head/run outcomes and obtain explicit merge authorization for the reviewed scope; do not equate a future CI pass with architecture acceptance.

**Hold rules:** Prior to these checks, ADR direction remains approved but the L2 amendment candidates remain **Proposed**, PR #135 remains unmerged, issue #132 open and J02 still blocked for full Finance/Admissions/Student runtime certification. Independent B18 journeys can continue under their own separate approvals.
