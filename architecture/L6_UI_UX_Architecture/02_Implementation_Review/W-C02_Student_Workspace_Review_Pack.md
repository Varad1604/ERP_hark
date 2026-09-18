# W-C02 — Student Workspace UI Review Pack

**Workspace:** `W-C02 — Student Workspace`  
**Architecture Source:** `../L6_Student_Workspace_DRAFT.md`  
**Proceed Gate:** `../00_Governance/L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md`  
**Delivery Register:** `../L6_Experience_Slice_Register_DRAFT.md`  
**Implementation Status:** Architecture Approved / Slice Ready / Controlled frontend implementation authorized  
**Purpose:** Provide reproducible W-C02 experience-slice cases without inventing backend behavior.

---

## 1. Review Profiles and Context

Minimum presentation profiles:

- Student Operations / authorized staff presentation;
- Institutional Administrator presentation where applicable;
- Student self-service presentation.

Presentation profile is not authorization.

Default review context:

- Institution: Sutra Institute of Technology;
- Academic period: 2026–27 · Semester 1;
- representative Student: Aarav Desai / `STU-2026-0142`;
- fixture fidelity: **Presentation fixture** unless a case explicitly states otherwise.

---

## 2. Approved W-C02 Scope

```text
C-001
C-008 … C-019
```

This pack preserves Student Lifecycle ownership, Person & Identity boundaries, Document Services boundaries, source-owned cross-domain facts, and human-readable lifecycle history distinct from canonical audit evidence.

---

## 3. Existing Journey References

Retain these stable review references:

- **J1** — Find and understand a Student.
- **J2** — Inspect Student record and history.
- **J3** — Review a high-consequence lifecycle action.
- **J4** — Follow a cross-domain projection.
- **J5** — Student document request and follow-through.
- **J6** — Student self-service.

---

## 4. Pilot Review Cases

### W-C02-RC-01 — J1 Staff orientation through Student search

**Slice:** `ES-W-C02-01`  
**Start:** Student Operations presentation; confirmed sample Institution/period; Home or global search.  
**Entry:** Search for “Aarav Desai” or use Students → Student Workspace.  
**Goal:** Enter the correct Student context and understand current lifecycle, placement and related-source summaries.  
**Actions:** open search → choose Student result → verify workspace context → inspect Overview.  
**Expected:** Student identity, Student ID, Institution/period and lifecycle context are unambiguous; search entry resolves into W-C02 rather than a generic detail dialog.  
**Alternate:** no matching Student → legitimate empty/no-result presentation.  
**Accessibility:** keyboard search selection; focus lands meaningfully in resolved workspace.  
**Responsive:** entry remains usable on supported mobile/tablet shell.  
**Trace:** C-001/C-008/C-009 orientation contributions + approved W-C02 composition.  
**Completion boundary:** W-C02 is open on the selected Student.  
**Exclusions:** no authorization inference from search visibility.

### W-C02-RC-01L — J1 Student context loading

**Slice:** `ES-W-C02-01`  
**Start:** Student entry initiated but current protected Student context has not resolved.  
**Expected:** progress is clear; no prior Student record is reused as current data.  
**Completion boundary:** current-context resolution remains pending.

### W-C02-RC-01E — J1 Student not found / legitimate empty

**Slice:** `ES-W-C02-01`  
**Start:** Student search/discovery returns a valid no-result condition.  
**Expected:** clear no-result state and recovery wording; no implication that restricted/hidden records exist.  
**Completion boundary:** reviewer understands that empty is a valid source outcome.

### W-C02-RC-02 — J2 Record/history continuity

**Slice:** `ES-W-C02-01`  
**Start:** Aarav in staff W-C02 Overview.  
**Goal:** Inspect Student Record and Documents & History without losing Student/context.  
**Actions:** Overview → Student Record → Documents & History → return to Overview.  
**Expected:** subject/context persists; lifecycle history is labelled as human-readable orientation, not canonical audit evidence.  
**Accessibility:** local navigation keyboard operable and current section exposed semantically.  
**Completion boundary:** reviewer can inspect and return without context loss.

### W-C02-RC-03 — J6 Student self-orientation

**Slice:** `ES-W-C02-02`  
**Start:** Student presentation with current self-context.  
**Entry:** Student home/search/navigation into own Student context.  
**Goal:** understand My Details, Academic Context, Requests & Documents and Related Statuses.  
**Expected:** no staff lifecycle controls; same canonical Student context is presented more simply.  
**Responsive:** first-class mobile check.  
**Completion boundary:** Student can understand current context and relevant self-service information.

### W-C02-RC-04 — J5 Start supported document request

**Slice:** `ES-W-C02-03`  
**Start:** Requests & Documents / Documents & History.  
**Goal:** express intent to start a supported request without the UI inventing required business inputs.  
**Actions:** Start supported request → review supported request type/context → submit fixture intent.  
**Expected:** presentation explicitly states fixture/review fidelity where no released contract is evidenced; enabled controls perform the bounded review interaction rather than doing nothing.  
**Completion boundary:** client intent reaches the simulated submission boundary only.  
**Blocker:** exact production fields/payload remain unimplemented until released contract evidence exists.

### W-C02-RC-05 — J5 Client submission pending

**Slice:** `ES-W-C02-03`  
**Start:** document request submission.  
**Expected:** immediate pending state does not claim that the business request is complete or document is issued; duplicate submit is constrained.

### W-C02-RC-06 — J5 Business request in progress

**Slice:** `ES-W-C02-03`  
**Start:** returned request exists but source business process remains in progress.  
**Expected:** state is distinct from client pending and authoritative completion; “return later” continuity is visible.

### W-C02-RC-07 — J5 Authoritative request completion / result available

**Slice:** `ES-W-C02-03`  
**Expected:** completion is shown only as a fixture representing an authoritative source outcome; result access appears only when source supports it. Rendering is not described as business issuance unless source semantics establish issuance.

### W-C02-RC-08A — J5 Business rejection

**Slice:** `ES-W-C02-03`  
**Expected:** source-owned rejection is clearly presented and is not described as a technical transport failure.

### W-C02-RC-08B — J5 Technical failure

**Slice:** `ES-W-C02-03`  
**Expected:** technical failure is clearly presented without inferring business success, rejection or request creation.

### W-C02-RC-09 — J5 Indeterminate submission

**Slice:** `ES-W-C02-03`  
**Sequence:** submit → outcome becomes uncertain → attempt repeat.  
**Expected:** UI blocks/constrains unsafe repeat and requires reconciliation; does not display success or ordinary failure.

### W-C02-RC-10 — J4 Finance projection handoff

**Slice:** `ES-W-C02-04`  
**Start:** Related or Overview.  
**Goal:** understand Finance projection ownership and continue toward owner.  
**Actions:** inspect Finance projection → choose owner continuation → observe bounded Finance destination/preview → return to W-C02.  
**Expected:** projection remains read-only; owner is explicit; Student context survives return.  
**Completion boundary:** handoff reaches the currently implemented owner boundary, not an invented Finance workflow.

### W-C02-RC-11 — J4 Finance source degraded

**Slice:** `ES-W-C02-04`  
**Expected:** Finance projection becomes source-specific unavailable/degraded while Student-owned W-C02 content remains usable.

### W-C02-RC-12 — J3 Suspension review

**Slice:** `ES-W-C02-05`  
**Trace:** C-010.  
**Start:** staff Lifecycle section, current Student/context resolved.  
**Goal:** understand the action and consequence before any authoritative mutation.  
**Actions:** open Suspension → review current context/consequence/integration boundary → confirm review intent in fixture mode.  
**Expected:** no local eligibility or lifecycle transition is calculated; the UI makes the real integration boundary explicit.

### W-C02-RC-13 — J3 Suspension pending

**Slice:** `ES-W-C02-05`  
**Expected:** pending is visually/semantically distinct; unsafe duplicate action is constrained.

### W-C02-RC-14 — J3 Suspension business rejection

**Slice:** `ES-W-C02-05`  
**Expected:** authoritative rejection fixture is distinct from technical failure and does not change local lifecycle state.

### W-C02-RC-15 — J3 Suspension technical failure

**Slice:** `ES-W-C02-05`  
**Expected:** safe recovery is presented without inferring the business operation failed or succeeded.

### W-C02-RC-16 — J3 Suspension indeterminate

**Slice:** `ES-W-C02-05`  
**Sequence:** confirm → outcome unknown → attempt repeat.  
**Expected:** repeat is constrained; reconciliation required; Student lifecycle state is not locally changed.

---

## 5. Remaining W-C02 Governed Review Cases

The following cases use the same review-case shape defined above. They validate presentation, consequence, uncertainty handling and safe return only. They do not claim released mutation-contract integration.

### ES-W-C02-06 — Profile and communication-preference maintenance

**Trace:** C-014 / C-015.  
**Start:** staff Student Record with current Student/context resolved.  
**Goal:** understand what can be maintained without moving Person identity authority into W-C02.  
**Normal entry:** Student Record → Profile maintenance.  
**Completion boundary:** reviewer reaches the supported maintenance seam; exact production fields/options remain integration-owned.

- **W-C02-RC-19 — Review:** current profile/contact context and ownership boundary are visible before intent.
- **W-C02-RC-20 — Rejection:** source rejection leaves current Student details unchanged.
- **W-C02-RC-21 — Technical failure:** no change is inferred from transport/execution failure.

### ES-W-C02-07 — Enrollment and academic-assignment maintenance

**Trace:** C-008 / C-009.  
**Start:** staff Student Record.  
**Goal:** review current enrollment/assignment before a governed maintenance request.  
**Normal entry:** Student Record → Enrollment / academic assignment maintenance.  
**Completion boundary:** intent reaches presentation boundary only; Registration/Progression effects are not decided locally.

- **W-C02-RC-22 — Review:** current enrollment and assignment are presented with ownership boundaries.
- **W-C02-RC-23 — Rejection:** authoritative rejection does not alter local Student context.
- **W-C02-RC-24 — Indeterminate:** repeat is constrained pending reconciliation.

### ES-W-C02-08 — Restoration after suspension

**Trace:** C-011.  
**Start:** Lifecycle action list; representative restoration context.  
**Goal:** understand restoration consequence without inferring eligibility.  
**Normal entry:** Lifecycle → Restoration after suspension.

- **W-C02-RC-25 — Review**
- **W-C02-RC-26 — Pending**
- **W-C02-RC-27 — Business rejection**
- **W-C02-RC-28 — Indeterminate**

**Completion boundary:** no client-side lifecycle transition occurs.

### ES-W-C02-09 — Intra-Institute reassignment

**Trace:** C-012.  
**Goal:** review reassignment as its own institutional outcome, distinct from inter-Institute transfer.  
**Normal entry:** Lifecycle → Intra-Institute reassignment.

- **W-C02-RC-29 — Review**
- **W-C02-RC-30 — Pending**
- **W-C02-RC-31 — Business rejection**
- **W-C02-RC-32 — Indeterminate**

**Exclusion:** no C-013 transfer semantics are reused to decide C-012 validity.

### ES-W-C02-10 — Inter-Institute transfer and context re-establishment

**Trace:** C-013.  
**Goal:** review transfer consequence and context-re-establishment boundary.  
**Normal entry:** Lifecycle → Inter-Institute transfer.

- **W-C02-RC-33 — Review**
- **W-C02-RC-34 — Pending**
- **W-C02-RC-35 — Business rejection**
- **W-C02-RC-36 — Indeterminate**

**Exclusions:** no approving actor, completion event or automatic destination-context switch is invented.

### ES-W-C02-11 — Identity correction

**Trace:** C-016.  
**Goal:** review evidence-sensitive Student-side correction while preserving Person & Identity ownership.  
**Normal entry:** Lifecycle → Identity correction.

- **W-C02-RC-37 — Review**
- **W-C02-RC-38 — Pending**
- **W-C02-RC-39 — Business rejection**
- **W-C02-RC-40 — Indeterminate**

**Exclusion:** the client does not replace canonical Person identity or invent evidence/approval requirements.

### ES-W-C02-12 — Duplicate Student lifecycle resolution

**Trace:** C-017.  
**Goal:** review duplicate-resolution consequence without client-side merge semantics.  
**Normal entry:** Lifecycle → Duplicate Student resolution.

- **W-C02-RC-41 — Review**
- **W-C02-RC-42 — Pending**
- **W-C02-RC-43 — Business rejection**
- **W-C02-RC-44 — Indeterminate**

**Exclusions:** no Student/Person merge sequence is invented; the browser performs no merge.

### ES-W-C02-13 — Closure/archive and preserved historical context

**Trace:** C-019.  
**Goal:** review permanent closure consequence while preserving history.  
**Normal entry:** Lifecycle → Permanent record closure.

- **W-C02-RC-45 — Review**
- **W-C02-RC-46 — Pending**
- **W-C02-RC-47 — Business rejection**
- **W-C02-RC-48 — Indeterminate**

**Exclusions:** archival is not physical deletion; readable lifecycle history remains separate from canonical audit evidence.

**Shared accessibility/responsive checkpoints for RC-19…48:** keyboard entry into the action; named modal; focus restoration on close; state conveyed beyond color; consequence/context remains readable at supported narrow widths; indeterminate cases prevent unsafe repeat.

---

## 6. Context/Sequence Safety Cases

### W-C02-RC-17 — Late response after protected context change

1. Begin loading Student A.
2. Change Institution/academic protected context.
3. Old-context Student response returns.
4. Confirm old data cannot populate the current protected view.

**Fixture proof:** validates UI containment behavior only. Real session/scope enforcement remains an integration obligation under L5-G-01-003/A04.

### W-C02-RC-18 — Authorization denied

A curated denial case presents explicit server-authoritative refusal semantics. Hidden controls are not treated as the security mechanism.

---

## 7. State Applicability / Justified Exclusions

| Slice | Applicable review states | Justified exclusions |
|---|---|---|
| Staff orientation | normal, loading, empty, stale, denied, degraded projection | mutation success/rejection does not apply to read-only orientation |
| Student self-orientation | normal, loading, empty, stale, denied, degraded projection | staff action states excluded |
| Document request | normal, client pending, business in-progress, completion/result, rejection, technical failure, indeterminate, denied | exact production form validation excluded until released contract evidence exists |
| Finance projection | normal, loading/degraded, denied, stale | no W-C02 Finance mutation states |
| Suspension review seam | normal review, pending, rejection, technical failure, indeterminate, stale/denied | authoritative success transition is not locally simulated as a lifecycle change without real integration |

---

## 8. Fixture Catalogue

| Fixture key | Fidelity | Purpose |
|---|---|---|
| `student-aarav-active-v1` | Presentation fixture | normal staff/student orientation |
| `student-aarav-doc-pending-v1` | Presentation fixture | request in progress |
| `student-aarav-doc-complete-v1` | Presentation fixture | authoritative-completion/result presentation |
| `student-aarav-finance-degraded-v1` | Presentation fixture | independent related-source degradation |
| `student-aarav-suspension-review-v1` | Presentation fixture | high-consequence review seam |
| `student-aarav-stale-context-v1` | Presentation fixture | protected-context invalidation |
| `student-aarav-denied-v1` | Presentation fixture | authorization-denied presentation |

No fixture claims backend contract fidelity unless explicitly reclassified later.

---

## 9. Responsive and Accessibility Checkpoints

Review wide desktop, normal laptop, tablet/narrow desktop and Student self-service mobile. Staff mobile is reviewed only for tasks supported by the approved boundary.

Critical checks:

- keyboard-only entry and local navigation;
- visible focus;
- meaningful heading structure;
- accessible dialog name and focus restoration;
- state not communicated by color alone;
- readable rejection/failure/indeterminate explanation;
- touch targets on supported mobile tasks;
- reduced-motion behavior;
- reflow without losing Student/context/consequence information.

---

## 10. Finding Log

| ID | Case | Observation | Classification | Severity | Disposition | Status |
|---|---|---|---|---|---|---|
| W-C02-F-001 |  |  | Implementation defect / L6 UX architecture defect / L5 issue / upstream issue / Review-process issue / Implementation-local improvement |  |  | Open |

---

## 11. Acceptance Reporting

Record separately per case:

- implemented;
- fixture-tested;
- browser-reviewed;
- released-contract integrated;
- blocked/planned.

A passed presentation fixture is not evidence of production integration.
