# ADR-FIN-ADJ-001 — Financial Adjustment Governance (Decision Package FIN-ADJ)

| Attribute | Value |
|---|---|
| ADR ID | ADR-FIN-ADJ-001 |
| Decision Package | FIN-ADJ (per `L3_Phase5_Chetan_Upstream_Architecture_Decision_Proposal_2026-08-20.md`, Section 3) |
| Target Document | `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F04_Financial_Adjustment_Management.md` (Status: Locked, Version 1.0.0) |
| Blocked Downstream Documents | `L3-F-020`, `L3-F-021`, `L3-F-022`, `L3-F-023`, `L3-F-024` (all Draft, Owner: Chetan) |
| Status | **Proposed — Non-Authoritative** |
| Decision Effect | None until approved by the named repository roles and incorporated into a re-locked `L2-F04` |
| Domain Owner | Finance — Financial Adjustments (Chetan) |
| Date | 2026-08-21 |
| Prepared By | Drafted on request, for review by Chetan (Finance domain owner), Dhruv (Lead Architect, lock approval), and the Finance Review Team (Varad, Devang) per `governance/constitution/Volume_5_Change_Management.md` §5 |

---

## 1. Need

`L2-F04` is Locked but self-admits two structural gaps via its own Canonical Section Crosswalk (added 2026-08-04, finding INT-018, lines 5–24): **Actors** and **Locked Decisions** have no counterpart anywhere in the document. In addition, its Section 17 Event Architecture explicitly labels every event name as "illustrative" rather than canonical (§17.4, §17.5), and its Section 14 Lifecycle and Section 15 State Architecture define only a forward-only, no-reversal lifecycle.

These gaps are not abstract. They are already blocking five drafted L3 scenarios, each of which isolates the exact same gaps into its own Section 17:

| Scenario | Blocking gap traceable to `L2-F04` |
|---|---|
| `L3-F-020` Approved Credit Note → Reduced Liability | No actor for post-approval publication/application (§6, §15.1); no canonical event for `Applied`/financial-position-updated (§14.1) |
| `L3-F-021` Approved Debit Note → Increased Liability | Same two gaps |
| `L3-F-022` Approved Write-Off → Governed Outcome | Same two gaps, **plus** a direct conflict: `L2-F00` §16 requires Board approval for a Student-Charge `Written Off` outcome, while `L2-F04` §11 leaves all approval authority generically external — the canonical approval authority for write-offs is unreconciled |
| `L3-F-023` Posted Adjustment → Reversal | The most severe gap: the frozen scenario title requires a `Reversal` outcome; `L2-F00`'s universal Finance state model contains `Posted`/`Reversed`, but `L2-F04` §14–15 contains neither state, and §14.5/§14.6 mandate forward-only, immutable lifecycle progression with **no reversal transition of any kind**. `L3-F-023` cannot state a valid state-and-lifecycle mapping at all without this ADR. |
| `L3-F-024` Adjustment Error → Corrected Successor Record | `L2-F04` defines no canonical successor/supersession relationship, no treatment for the erroneous original, and no confirmed end boundary (`Published` vs `Applied`) |

Every one of these L3 drafts correctly refuses to invent an answer locally. This ADR is the proposal that lets a real answer be reviewed and, if approved, locked.

---

## 2. Proposal

Every proposed answer below is constructed by extending an architectural principle `L2-F04` already states as Locked — none introduces a new aggregate, a new bounded context, or backward lifecycle movement, because those are foreclosed by §4 (Canonical Aggregate), §7.7 (Aggregate Constraints), and §14.5–14.6 (forward-only, immutable lifecycle). Where an answer required more than reuse of stated principles, that is flagged explicitly as a genuine open decision rather than drafted.

### 2.1 Actors (fills crosswalk gap 1 of 2)

`L2-F04` already separates domain responsibility from approval execution (§11: "Approval remains externally governed") and from security/audit infrastructure (§20, §21: "remains externally governed" / "ownership remains external"). The actor table below stays inside that existing boundary — it names who acts *within* Financial Adjustment Management, not who owns approval, security, or audit infrastructure.

| Actor | Responsibility | Authority Boundary |
|---|---|---|
| Adjustment Requestor | Initiates a Financial Adjustment; supplies source, category, reason, and affected financial-position reference. | Cannot self-approve or self-publish. |
| Adjustment Reviewer | Recommends an adjustment for enterprise approval after confirming source/category/reason completeness (a Financial Adjustment Management responsibility distinct from the automated Validation layers in §12). | Recommendation only; approval execution remains owned by the enterprise approval capability (Section A / Governance), per §11.4. |
| Enterprise Approval Authority | Produces the positive/negative approval outcome consumed by Financial Adjustment Management at the `Pending Approval → Approved` transition. | Owned entirely outside Financial Adjustment Management (§11.1–11.4); named here only so L3 actor tables have a non-invented label to cite. |
| Adjustment Publisher | Confirms publication readiness and executes the `Approved → Published → Pending Application → Applied` progression (§14.4, §16.5). | Cannot bypass validation (§12.6) or approval (§16.5 Readiness Principle). |
| Reversal Authority | Confirms the business basis and authorizes a Reversal-category adjustment (see §2.2 below) against a Published/Applied original. | Same approval-participation boundary as Adjustment Reviewer/Approval Authority — reversal is itself a Financial Adjustment and follows the same approval path, not a separate authority track. |
| Correction Authority | Confirms an error finding on a Published/Applied adjustment and directs correction via a successor record (see §2.2 below). | Cannot edit or withdraw the erroneous original (§7.7 constraint 4: historical adjustments remain immutable). |

**Open decision flagged, not drafted:** `L3-F-022`'s conflict — whether Student-Charge write-offs specifically require Board approval (`L2-F00` §16) as a stricter instance of "Enterprise Approval Authority," or whether that is a configuration-level constraint outside `L2-F04`'s scope — is a Finance-domain policy call this ADR does not resolve. Recommend routing it to Chetan as a scoped sub-decision within this package rather than blocking the rest of FIN-ADJ.

### 2.2 Lifecycle — reversal and correction mechanism (resolves `L3-F-023` and `L3-F-024`)

**Proposal: neither reversal nor correction introduces a new state, a new aggregate, or backward movement.** Both are represented as an ordinary new Financial Adjustment instance, going through the existing canonical lifecycle (§14.3) and existing operational states (§15.3) unchanged, distinguished only by two additive extensions §9.6 and §13.4 already anticipate:

1. **New canonical categories** (extending §9.4's list per its own Extensibility Principle §9.6, "future categories extend this architecture"): `Reversal Adjustment`, `Correction Adjustment`.
2. **New reference types** (extending §13.4's list per its own line, "future enterprise identifiers extend this architecture"): `Reversed Adjustment Reference`, `Corrected Adjustment Reference` — each an immutable pointer from the new instance back to the original, published Financial Adjustment.

Concretely:
- **Reversal** (`L3-F-023`): a new Financial Adjustment, Category = `Reversal Adjustment`, carrying a `Reversed Adjustment Reference` to the original. It progresses through the same `Draft → … → Applied` path as any adjustment (§14.3). The original's operational state and lifecycle stage are **never mutated** — this is what reconciles `L2-F00`'s universal `Reversed` state with `L2-F04`'s forward-only model: `Reversed` is not a state the original transitions into, it is a fact evidenced by the existence of a published Reversal Adjustment referencing it. Enterprise visibility of "this adjustment has been reversed" is a derived read from the reference graph, not a new state field.
- **Correction** (`L3-F-024`): a new Financial Adjustment, Category = `Correction Adjustment` (or the successor's own natural category, with the reference carrying the correction semantics — Finance domain owner to confirm preference), carrying a `Corrected Adjustment Reference` to the erroneous original. Same reasoning: no `Superseded` state is added to the original; supersession is evidenced by the successor's reference, consistent with §7.7 constraint 4 (historical adjustments remain immutable) and §14.5 (completed milestones remain immutable).
- **End boundary for `L3-F-024`:** resolves to `Published`, matching every other FIN-ADJ scenario's boundary pattern (`L3-F-020`–`022` all end at `Applied` because they represent the primary financial effect; a Correction's primary effect is establishing the authoritative corrected record, and its own application to the financial position follows the standard `Pending Application → Applied` path as a normal continuation of the same scenario shape already used by `L3-F-020`–`022`, not a separate unresolved boundary).

This also directly answers proposal item 4 (distinguish correction / reversal / rejection / cancellation / terminal failure), using only states `L2-F04` already declares at §15.3 — no new operational state is proposed:

| Concept | Mechanism | Resulting state of the *original* adjustment | Resulting state of the *new* record (if any) |
|---|---|---|---|
| Rejection | Negative approval outcome at `Pending Approval` | N/A — the adjustment itself moves to `Cancelled` with evidence reason "Approval Rejected" | None; resubmission is a new Financial Adjustment instance if pursued |
| Cancellation | Withdrawal by Financial Adjustment Management before `Published` | Moves to existing `Cancelled` state (§15.3) with evidence reason "Withdrawn" | None |
| Terminal failure | Unrecoverable validation or financial-consistency exception before publication (§23) | Moves to existing `Cancelled` state with evidence reason citing the exception category (§23.3) | None |
| Correction | New `Correction Adjustment` referencing the original | **Unchanged** — remains at whatever state/lifecycle it had already reached | New instance follows normal lifecycle to `Published`/`Applied` |
| Reversal | New `Reversal Adjustment` referencing the original | **Unchanged** — remains at whatever state/lifecycle it had already reached | New instance follows normal lifecycle to `Published`/`Applied` |

### 2.3 Events (fills the "illustrative" gap in §17.4/§17.5, resolves proposal item 3)

Promote the existing illustrative lists to canonical, single-publisher = Financial Adjustment Management (per §17.6 Ownership Principle, §18 Integration Architecture), and add two events required by §2.2 above:

| Event | Publisher | Material Consumers (per §18.5) | Status |
|---|---|---|---|
| `AdjustmentInitiated` | Financial Adjustment Management | Reporting, Analytics | Promoted from §17.4 illustrative list |
| `AdjustmentValidated` | Financial Adjustment Management | Reporting, Analytics | Promoted |
| `AdjustmentApproved` | Financial Adjustment Management | Reporting, Analytics, Notification | Promoted |
| `AdjustmentPublicationCompleted` | Financial Adjustment Management | Accounting, Reporting, Analytics, Notification, Bank Reconciliation | Promoted |
| `AdjustmentApplicationCompleted` | Financial Adjustment Management | Accounting, Reporting, Analytics | Promoted |
| `AdjustmentCompleted` | Financial Adjustment Management | Reporting, Analytics | Promoted |
| `AdjustmentSuspended` | Financial Adjustment Management | Reporting, Notification | Promoted |
| `AdjustmentCancelled` | Financial Adjustment Management | Reporting, Notification | Promoted |
| `AdjustmentArchived` | Financial Adjustment Management | Reporting | Promoted |
| `AdjustmentFinancialPositionUpdated` | Financial Adjustment Management | Accounting, Bank Reconciliation | Promoted from §17.5 |
| `AdjustmentAccountingReadinessConfirmed` | Financial Adjustment Management | Accounting | Promoted |
| `AdjustmentLifecycleStateChanged` | Financial Adjustment Management | Reporting, Analytics | Promoted |
| `AdjustmentStateTransitioned` | Financial Adjustment Management | Reporting, Analytics | Promoted |
| `AdjustmentReversalPublished` | Financial Adjustment Management | Accounting, Reporting, Analytics, Notification, Bank Reconciliation | **New** — required by §2.2; carries the `Reversed Adjustment Reference` |
| `AdjustmentCorrectionPublished` | Financial Adjustment Management | Accounting, Reporting, Analytics, Notification | **New** — required by §2.2; carries the `Corrected Adjustment Reference` |

### 2.4 Business recovery rules (proposal item 5)

| Situation | Proposed rule | Grounding |
|---|---|---|
| Duplicate adjustment | Before publication, Financial Adjustment Management verifies no other adjustment shares the same Source + Category + Reason + affected Financial Position Reference within a governed detection window; a match is contained pre-publication and routed to the Requestor for confirmation or withdrawal (`Cancelled`) — never silently rejected. | Extends §6 Deterministic Processing Principle and §12 Validation layers |
| Rejected approval | `Cancelled` with evidence reason "Approval Rejected"; resubmission only as a new instance, never resurrection of the rejected one. | §7.7 constraint 4 (immutability); §15.5 Deterministic Transition Principle |
| Uncertain approval/application outcome | The adjustment is held at its current state (`Pending Approval` / `Pending Application`) with no automatic transition until the same authority that owns that stage resolves the uncertainty. | §6 Deterministic Processing Principle — never guess an outcome |
| Partial application | Not permitted. Application is atomic: an adjustment applies fully to the financial position or remains contained in `Pending Application`. | §6 Deterministic Processing Principle; §16.6 constraint 5 (publication remains deterministic) |

### 2.5 Cross-domain hand-off contract (proposal item 6)

`L2-F04` §18.4 already lists upstream domains (Admissions, Academic, Scholarship, etc.) as providing "context only." Proposal: that context is captured at Adjustment initiation and is **not** re-fetched or re-validated at publication or application time. If an upstream fact changes after a Financial Adjustment is created, it does not retroactively alter the created adjustment (preserving §7.7 immutability); any required consequence is handled as a new Correction or Reversal Adjustment per §2.2, referencing the original. This makes the hand-off asynchronous and one-directional by construction, consistent with §18.6 Loose Coupling Principle — no synchronous cross-domain contract is introduced.

---

## 3. Impact Analysis

- **L1 boundary check** (`SutraERP_L1_Business_Domain_Architecture_Locked.md` §"Level 2 Dependency Order", line 3154): Section F (Finance) depends on Sections A, B, C. Nothing in this proposal assigns Finance-domain actors any authority over Section A-owned approval execution, audit infrastructure, or the Workflow Engine (`L2-A05`) — the Enterprise Approval Authority row in §2.1 is deliberately left external, consistent with `L2-F04` §11.1 and this L1 dependency direction.
- **`DEPENDENCY_MAP.md`**: contains no entry naming `L2-F04` or Financial Adjustment specifically; nothing in it conflicts with this proposal. (The map operates at domain, not per-document, granularity for Finance.)
- **Ripple to `L2-F04`'s declared "Consumed By" list** (document metadata, line 39): the two new events (`AdjustmentReversalPublished`, `AdjustmentCorrectionPublished`) and two new reference types are new inputs to Accounting, Reporting, and Analytics consumption — flag for those domain owners at Architecture Review, not a blocking conflict.
- **Metadata inconsistency found, not fixed here:** `L2-F04` line 39 lists "Consumed By: L2-F05 Accounting, **L2-F09 Reporting**, L2-F12 Analytics" — but `L2-F09` is titled "Financial Period and Closing," not Reporting; "Financial Reporting and Analytics Workflows" is `L2-F13`. This is likely a pre-existing mislabel in the Locked metadata that affects exactly the ripple list this ADR depends on. Recommend Architecture Review confirm and correct this alongside the FIN-ADJ amendment rather than treating it as this ADR's own error.
- **Cross-package consistency:** the Reversal/Correction mechanism proposed in §2.2 (new adjustment instance + reference, no new state) is offered as the reusable pattern for any future decision package facing the same "how do we represent an undo without backward lifecycle movement" question (e.g., FIN-GL's close/reopen, FIN-AP's payable reversal) — flagged for consistency review when those packages are drafted, not asserted as binding on them here.

---

## 4. Decision

*(Blank pending Architecture Review. Per `governance/constitution/Volume_5_Change_Management.md` §6, no implementation or Repository Update shall precede a recorded Decision here.)*

| Role | Name | Decision | Date |
|---|---|---|---|
| Finance domain/implementation owner | Chetan | **Approved** — recorded via Dhruv on 2026-08-21 | 2026-08-21 |
| Finance Review Team | Varad | **Approved** — recorded via Dhruv on 2026-08-21 | 2026-08-21 |
| Finance Review Team | Devang | **Approved** — recorded via Dhruv on 2026-08-21 | 2026-08-21 |
| Lead Architect (lock approval) | Dhruv | **Approved** — authorized directly in-session | 2026-08-21 |

All four named roles in Volume 5 §5 have now approved. Chetan's, Varad's, and Devang's approvals are recorded as reported by Dhruv in-session on 2026-08-21, not from a separate direct review session with each of them — noted here for traceability, not to cast doubt on the report.

One item from §2.1 remains genuinely unresolved and is **not** treated as approved by this Decision: the `L3-F-022` Board-approval-authority conflict. It is carried forward into the amended `L2-F04` as an explicit open item, not silently closed.

---

## 5. Acceptance Checklist

Cross-referenced against the source proposal's own Section 10 Acceptance Checklist:

| Required element | Present in this ADR? |
|---|---|
| Canonical aggregate and owner | Yes — unchanged: Financial Adjustment, owned by Financial Adjustment Management (§4.1–4.2, not amended) |
| Exact state names and transitions | Yes — §2.2; reuses existing §15.3 states, adds no new state |
| Exact actor names and decision authority | Yes — §2.1, with one item explicitly flagged unresolved rather than drafted |
| Exact event names and one publisher per event | Yes — §2.3, single publisher throughout |
| Consumers and dependency direction | Yes — §2.3 table; consistent with L1 dependency order (§3) |
| Synchronous business contracts where required | None required — §2.5 establishes the hand-off is asynchronous by design |
| Separation-of-duty and evidence obligations | Partially — §2.1 preserves existing separation; no new separation-of-duty rule beyond what §11/§15/§20 already state was identified as needed |
| Business-level rejection, correction, reversal, interruption, terminal-resolution semantics | Yes — §2.2 table |
| No implementation-level API, persistence, queue, retry-count, idempotency, or infrastructure design | Confirmed — this ADR stays at business/architecture level throughout |
| Traceability from amended L2 material to every affected L3 scenario | Yes — §1 table maps every gap to `L3-F-020`–`024` |

One item is not fully closed: the `L3-F-022` Board-approval-authority conflict (§2.1) is flagged, not resolved, and should not be read as accepted by omission.
