# Posted Adjustment to Reversal

# 1. Scenario Metadata

**Scenario ID:** L3-F-023  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Financial Adjustment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md` v1.1.0 (as amended by `ADR-FIN-ADJ-001`); frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs the business reversal of a posted Financial Adjustment while preserving the original adjustment and historical financial record as immutable evidence. It exists to ensure that a required correction is represented through a traceable reversing financial outcome rather than by editing or deleting the original posted adjustment.

The `Posted` and `Reversal` terminology in the frozen scenario title is reconciled with the specific Financial Adjustment lifecycle and state model by `L2-F04` §29.1 (ADR-FIN-ADJ-001, approved 2026-08-21 by Dhruv, Lead Architect): `Posted` maps to any original Financial Adjustment that has reached `Applied` (its financial effect is already part of the institutional financial position) or a later state (`Completed`, `Archived`); `Reversal` is represented as a new Financial Adjustment instance — never a backward transition of the original — per the mechanism described below.

---

# 3. Business Outcome

The posted Financial Adjustment has a governed, auditable reversal outcome that neutralizes or corrects its financial effect without changing the original adjustment or its historical evidence.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an existing Financial Adjustment in the `Applied`, `Completed`, or `Archived` operational state (the resolved mapping for the frozen term `Posted`, per `L2-F04` §29.1) requires reversal under an established business correction requirement.

## 4.2 End Boundary

The scenario ends when the original adjustment remains unchanged and immutable, and a new Reversal Adjustment — carrying a `Reversed Adjustment Reference` to the original — has completed validation and approval, been published, and reached the `Applied` operational state and lifecycle milestone, neutralizing the original's financial effect on the applicable financial position.

## 4.3 In Scope

- Identification of the original posted Financial Adjustment and its financial effect.
- Confirmation of the business basis and authority for reversal.
- Establishment of a reversing financial outcome without modifying the original adjustment.
- Preservation of complete references between the original adjustment and reversal outcome.
- Preservation of approval, decision, financial-effect, and historical audit evidence.

## 4.4 Out of Scope

- Correction of an adjustment through a corrected successor record, which belongs to `L3-F-024`.
- Editing, deleting, or overwriting the original Financial Adjustment.
- Reversal of a Journal Entry under Accounting and General Ledger Management.
- Reversal of a Payment or Refund under their respective Finance capabilities.
- Definition of a new Financial Adjustment state, lifecycle stage, event, approval role, or reversal contract.
- Accounting implementation of the reversal consequence.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | `L3-F-024` Adjustment Error to Corrected Successor Record | Governs correction through a successor record rather than the reversal outcome defined here. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Adjustment Management / Section F | Owns the original Financial Adjustment, its immutable history, the governed reversal outcome, financial lineage, and adjustment-level audit obligations. |

The frozen Stage 3 row defines only Section F as participating. Accounting may consume accounting-ready reversal context, but journals, ledger postings, and accounting reversals remain owned by Accounting and General Ledger Management and are outside this scenario outcome.

---

# 6. Business Actors, Roles and Responsibilities

Resolved by `L2-F04` §28.2 (ADR-FIN-ADJ-001):

| Actor | Role in this scenario |
|---|---|
| Reversal Authority | Confirms the business basis for reversal and authorizes the new Reversal Adjustment (Steps 1–2). Follows the same approval-participation boundary as any other adjustment actor — reversal is not a separate authority track. |
| Adjustment Requestor / Reversal Authority | Initiates the Reversal Adjustment, carrying the `Reversed Adjustment Reference` (Step 3). |
| Enterprise Approval Authority | Produces the positive approval outcome consumed at the Reversal Adjustment's `Pending Approval → Approved` transition (Step 5). Acts externally per `L2-F04` §11 and §28.2. |
| Adjustment Publisher | Confirms publication readiness and executes the `Approved → Published → Pending Application → Applied` progression for the Reversal Adjustment (Steps 6–7). |

`L2-F04` §28.3 No Self-Certification Principle applies: the Reversal Authority is never, by that action alone, the Enterprise Approval Authority whose outcome the Reversal Adjustment consumes.

---

# 7. Preconditions and Entry Criteria

- The original Financial Adjustment exists in the `Applied`, `Completed`, or `Archived` operational state (the resolved mapping for `Posted`, per `L2-F04` §29.1).
- The original adjustment, its source, category, reason, institutional reference, financial-position reference, and historical financial effect are available and traceable.
- The original adjustment is immutable and cannot be edited or deleted.
- A business correction requirement exists that calls for reversal rather than correction through a successor record.
- The Reversal Authority (`L2-F04` §28.2) has confirmed the reversal basis before a new Reversal Adjustment is initiated.

---

# 8. Trigger

A governed business correction requirement determines that the financial effect of a posted Financial Adjustment must be reversed without modifying the original adjustment.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the original posted Financial Adjustment, its canonical reference, business basis, and financial effect requiring reversal. | Financial Adjustment Management | The reversal is anchored to one immutable and traceable original adjustment. |
| 2 | Confirm that reversal, rather than historical editing or successor-record correction, is the governed correction requirement, and that the Reversal Authority has established the required business basis. | Reversal Authority (`L2-F04` §28.2) | The reversal basis is established without altering the original record. |
| 3 | Initiate a new Financial Adjustment, Category `Reversal Adjustment`, carrying a `Reversed Adjustment Reference` to the original. | Adjustment Requestor / Reversal Authority (`L2-F04` §28.2) | The Reversal Adjustment enters `Draft` / `Adjustment Initiated` within the canonical Financial Adjustment aggregate; the original is untouched. |
| 4 | Validate the Reversal Adjustment for structural, financial, source, category, reason, and reference completeness (§12 layers). | Financial Adjustment Management | The Reversal Adjustment reaches `Validated`. |
| 5 | Obtain the required positive enterprise approval outcome for the Reversal Adjustment. | Enterprise Approval Authority (`L2-F04` §28.2) | The Reversal Adjustment reaches `Approved`. |
| 6 | Publish the approved Reversal Adjustment as the authoritative enterprise representation of the reversal. | Adjustment Publisher (`L2-F04` §28.2) | The Reversal Adjustment reaches `Published`, publishing `AdjustmentReversalPublished`. |
| 7 | Apply the Reversal Adjustment to the applicable financial position without modifying the original adjustment or historical accounting record. | Adjustment Publisher (`L2-F04` §28.2) | The Reversal Adjustment reaches `Applied`; the original's financial effect is neutralized. The original's own state and lifecycle remain unchanged. |
| 8 | Preserve the original adjustment, reversal basis, authority, reference, resulting financial effect, and complete history as auditable evidence. | Financial Adjustment Management | The reversal outcome is permanently reconstructable and the original adjustment remains immutable. |

This flow, its states, and its event follow `L2-F04` §29.1 (ADR-FIN-ADJ-001).

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

Where the business requirement is correction through a successor record rather than reversal, processing belongs to `L3-F-024` and is outside this scenario.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The original adjustment or its financial lineage cannot be authoritatively established. | A Reversal Adjustment cannot be safely related to the correct original financial effect. | Resolved by `L2-F04` §29.3 Terminal Failure rule: the Reversal Adjustment moves to `Cancelled` with an evidence reason citing the reference exception category (§23.3); the original record remains unchanged. | Financial Adjustment Management |
| The business authority or reason for reversal is absent or unresolved. | The Reversal Adjustment cannot become a governed financial outcome. | Resolved by `L2-F04` §29.3: the Reversal Adjustment is held at its current pre-approval state until the Reversal Authority resolves the basis; no automatic transition occurs. | Financial Adjustment Management |
| Approval of the Reversal Adjustment is rejected. | The reversal cannot become the authoritative published outcome. | Resolved by `L2-F04` §29.3 Rejected Approval rule: the Reversal Adjustment moves to `Cancelled` with evidence reason "Approval Rejected"; resubmission requires a new instance. | Financial Adjustment Management |
| The reversing financial effect would compromise financial consistency. | The Reversal Adjustment cannot validly affect the financial position. | Resolved by `L2-F04` §29.3 Partial Application rule: application is atomic and does not proceed; the Reversal Adjustment remains contained in `Pending Application` while the original adjustment and audit history remain preserved. | Financial Adjustment Management |

Recovery is governed by `L2-F04` §29.3 (ADR-FIN-ADJ-001). No technical recovery mechanism is defined at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Historical financial records shall never be modified, and corrections shall be represented through compensating financial transactions such as reversal entries or adjustment entries. | Finance | Reversal must preserve the original posted adjustment and create a separately traceable financial consequence. | `L2-F00`, Sections 27 and 59.2 |
| Financial Adjustment is the only canonical adjustment aggregate, and external domains may reference but never modify it. | Financial Adjustment Management | The original adjustment remains under exclusive Financial Adjustment ownership throughout reversal. | `L2-F04`, Sections 4 and 24 |
| Historical Financial Adjustments and their published references remain immutable. | Financial Adjustment Management | Neither the original adjustment nor its historical references may be overwritten during reversal. | `L2-F04`, Sections 7.7, 13.6 and 16.4 |
| Equivalent financial contexts shall produce identical adjustment outcomes. | Financial Adjustment Management | Any governed reversal behaviour must remain deterministic once its upstream lifecycle and rule definition is resolved. | `L2-F04`, Section 6 |
| Financial integrity and complete traceability must be preserved. | Financial Adjustment Management | The reversal must be related to the original adjustment and remain permanently reconstructable. | `L2-F04`, Sections 6, 21 and 23 |
| Financial Adjustment shall not create journals or maintain ledgers. | Financial Adjustment Management | Any accounting reversal remains outside this scenario and under Accounting ownership. | `L2-F04`, Section 19.5 |
| The specific Financial Adjustment lifecycle progresses forward only and completed milestones remain immutable. | Financial Adjustment Management | Reversal is represented as a new Financial Adjustment (Category `Reversal Adjustment`) rather than backward movement through the original's lifecycle. | `L2-F04`, Sections 14.5, 14.6 and 29.1 |
| Reversal and Correction shall never mutate the operational state or lifecycle stage of the original Financial Adjustment, and shall always carry an immutable reference to it. | Financial Adjustment Management | The original adjustment's state/lifecycle is unaffected by this scenario; the Reversal Adjustment carries a `Reversed Adjustment Reference` to it. | `L2-F04`, Section 29.5 |

---

# 13. State and Lifecycle Impact

Resolved by `L2-F04` §29.1 (ADR-FIN-ADJ-001), reconciling `L2-F00`'s universal `Posted`/`Reversed` states with `L2-F04`'s forward-only, no-reversal-transition lifecycle: `Reversed` is not a state the original adjustment transitions into — it is a fact evidenced by the existence of a published Reversal Adjustment referencing it.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Original Financial Adjustment | `Applied`, `Completed`, or `Archived` | The original is referenced by a new Reversal Adjustment but never transitions. | Unchanged — remains at its existing state and lifecycle position, immutable. | Financial Adjustment Management | `L2-F04`, Sections 7.7, 14.5 and 29.1 |
| Reversal Adjustment (new instance) | `Draft` / `Adjustment Initiated` | Validation completes for the Reversal Adjustment. | `Validated` | Financial Adjustment Management | `L2-F04`, Sections 12, 14 and 29.1 |
| Reversal Adjustment | `Validated` | Positive enterprise approval participation is obtained. | `Approved` | Financial Adjustment Management | `L2-F04`, Sections 11, 14 and 29.1 |
| Reversal Adjustment | `Approved` | The Reversal Adjustment is published as the authoritative reversal record. | `Published` | Financial Adjustment Management | `L2-F04`, Sections 14-16 and 29.1 |
| Reversal Adjustment | `Published` | The Reversal Adjustment becomes pending application. | `Pending Application` | Financial Adjustment Management | `L2-F04`, Section 15.4 |
| Reversal Adjustment | `Pending Application` | The reversing financial effect is applied, neutralizing the original's effect. | `Applied` | Financial Adjustment Management | `L2-F04`, Sections 14.4, 15.4 and 29.1 |

The original adjustment possesses exactly one active operational state throughout this scenario, and it is never `Reversed` as a literal state value — consistent with `L2-F04` §15.5 Single Active State Principle and §7.7 immutability.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Resolved by `L2-F04` §17.4/§29.1/§29.2 (ADR-FIN-ADJ-001). Single publisher: Financial Adjustment Management.

| Event | Scenario Step | Material Consumers |
|---|---|---|
| `AdjustmentReversalPublished` | Step 6 (`Approved` → `Published`) | Accounting, Reporting, Analytics, Notification, Bank Reconciliation |
| `AdjustmentApplicationCompleted` | Step 7 (`Pending Application` → `Applied`) | Accounting, Reporting, Analytics |
| `AdjustmentFinancialPositionUpdated` | Step 7, concurrent with the neutralized financial effect | Accounting, Bank Reconciliation |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain contract or governed hand-off is established inside the scenario boundary because the frozen Stage 3 row lists only Section F as participating.

The relationship between the original Financial Adjustment and its Reversal Adjustment is the canonical `Reversed Adjustment Reference` defined in `L2-F04` §13.4 and §29.1 (ADR-FIN-ADJ-001).

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Determine that a posted Financial Adjustment requires reversal | Reversal Authority (`L2-F04` §28.2) | A governed correction requirement and explainable business reason must exist. | Financial Adjustment Management |
| Approve the Reversal Adjustment | Enterprise Approval Authority (`L2-F04` §28.2) | The Reversal Adjustment follows the same approval-participation path as any Financial Adjustment; it is not a separate authority track. | Financial Adjustment Management owns the adjustment; enterprise approval execution remains external. |
| Publish and apply the Reversal Adjustment | Adjustment Publisher (`L2-F04` §28.2) | Authorization must be validated before the reversal affects the financial position. | Financial Adjustment Management |

## 15.2 Separation of Duties, Overrides and Exceptions

`L2-F04` §28.3 No Self-Certification Principle applies: the Reversal Authority that determines reversal is necessary is never, by that action alone, the Enterprise Approval Authority whose outcome the Reversal Adjustment consumes. `L2-F00`'s identification of Journal Reversal as a sensitive operation is a separate, Accounting-owned control and is not extended to Financial Adjustment reversal by this scenario.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the original posted adjustment | Immutable original Financial Adjustment, canonical reference, financial effect, and historical status | Financial Adjustment Management |
| Preserve the reversal basis | Explainable business reason and governed correction requirement associated with the original adjustment | Financial Adjustment Management |
| Preserve reversal authority | The applicable authorization or approval outcome once defined upstream | Financial Adjustment Management |
| Preserve reversal lineage | Immutable relationship between the original Financial Adjustment and the governed reversal outcome | Financial Adjustment Management |
| Preserve the financial consequence | Reconstructable reversing effect on the applicable financial position | Financial Adjustment Management |
| Preserve complete history | Original and reversal-related activities remain immutable and permanently auditable | Financial Adjustment Management |

Section A retains ownership of authorization and audit infrastructure; this scenario records only the Finance-owned business obligations and evidence.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The original posted Financial Adjustment remains unchanged and historically available.
- A new Reversal Adjustment, carrying a `Reversed Adjustment Reference` to the original, has completed validation and approval, been published, and reached the `Applied` operational state and lifecycle milestone.
- The reversing financial effect has been applied to the applicable financial position.
- The business reason, authority, original-to-reversal relationship, and resulting financial effect are permanently reconstructable.
- No Accounting-owned journal, ledger, or posting lifecycle has been re-owned by this scenario.

## 16.2 Postconditions

- The original Financial Adjustment remains immutable and under Financial Adjustment Management ownership.
- The applicable financial position reflects the governed reversal outcome.
- The original and reversing financial effects are separately traceable.
- Any accounting consequence is handled by Accounting and General Ledger Management without transferring adjustment ownership.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

None remain for this scenario. All five issues originally recorded here (the `Posted`/`Reversed` state reconciliation, the reversal lifecycle mechanism, missing Actors, illustrative-only events, and undefined recovery paths) were resolved 2026-08-21 by `ADR-FIN-ADJ-001` and the corresponding amendment to `L2-F04` (§29.1–29.5), approved by Dhruv (Lead Architect, lock approval). See Sections 4, 9, 12, 13, 14, and 15 above. This was the most architecturally significant of the five FIN-ADJ scenarios — the frozen title `Posted Adjustment to Reversal` had no valid state-and-lifecycle mapping at all under the pre-amendment `L2-F04`.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-023**
