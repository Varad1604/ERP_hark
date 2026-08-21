# Approved Write-Off Adjustment to Governed Financial Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-F-022  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Financial Adjustment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md` v1.1.0 (as amended by `ADR-FIN-ADJ-001`); frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs how an approved Write-Off Adjustment becomes an authoritative modification of an existing financial position while preserving the original financial transaction and historical accounting record. It ensures that the write-off is represented through the canonical Financial Adjustment aggregate with controlled publication, application, immutable lineage, and complete auditability.

---

# 3. Business Outcome

The approved Write-Off Adjustment has been published and applied as an immutable Financial Adjustment, and the applicable financial position reflects the governed write-off outcome while the original financial history remains unchanged and traceable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a validated Write-Off Adjustment has received a positive enterprise approval outcome and is in the `Approved` state.

## 4.2 End Boundary

The scenario ends when the approved Write-Off Adjustment has been published, applied to the applicable financial position, and reached the `Applied` state and lifecycle milestone, producing the governed financial outcome defined by the approved adjustment.

## 4.3 In Scope

- Confirmation that the approved Write-Off Adjustment is ready for publication.
- Canonical publication of the approved Financial Adjustment.
- Application of the approved write-off to the applicable financial position.
- Preservation of the original financial record and complete adjustment lineage.
- Recording of the relevant lifecycle, state, approval, publication, and application evidence.

## 4.4 Out of Scope

- Initiation, validation, or approval of the Write-Off Adjustment before it reaches the `Approved` state.
- Determination of the amount or scope of the write-off, which must already be established by the approved adjustment.
- Creation or amendment of the original financial obligation.
- Physical disposal, scrap, inventory write-off business cases, or execution evidence owned by Enterprise Resource Management.
- Editing or replacement of posted accounting entries.
- Creation of journals, ledger postings, or financial statements, which remain owned by Accounting and General Ledger Management.
- Reversal or correction after application, which belongs to separate canonical scenarios.
- Archival of the completed Financial Adjustment.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | `L3-F-023` Posted Adjustment to Reversal | Governs a subsequent reversal of a posted adjustment. |
| Followed by | `L3-F-024` Adjustment Error to Corrected Successor Record | Governs correction where the adjustment is later found to be erroneous. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Adjustment Management / Section F | Owns the Financial Adjustment aggregate, its publication, application milestone, lifecycle, state, references, and the governed financial outcome. |

The frozen Stage 3 row defines only Section F as participating. Enterprise Resource Management owns any physical asset or inventory disposal and write-off business case or execution evidence, while Finance owns the monetary and accounting consequences. Such upstream operational activity is outside this scenario unless already represented in the approved adjustment context.

---

# 6. Business Actors, Roles and Responsibilities

Resolved by `L2-F04` §28.2 (ADR-FIN-ADJ-001) for post-approval publication and application. Within this scenario's boundary (`Approved` → `Applied`):

| Actor | Role in this scenario |
|---|---|
| Adjustment Publisher | Confirms publication readiness (Step 1), publishes the Financial Adjustment (Step 2), and executes the `Pending Application → Applied` progression (Steps 3–4). |
| Enterprise Approval Authority | Produced the positive approval outcome that established the `Approved` entry state before this scenario begins; acts outside this scenario's boundary per `L2-F04` §11 and §28.2. |

**One item remains genuinely open, not resolved:** `L2-F04` §28.4 explicitly carries forward whether "Board" (`L2-F00` §16, Student-Charge write-offs) is a stricter mandatory instance of Enterprise Approval Authority for all write-offs, or a configuration-level constraint scoped to Student Charges only. This scenario does not assume either answer. See Section 17.

---

# 7. Preconditions and Entry Criteria

- A canonical Write-Off Adjustment exists within the Finance-owned Financial Adjustment aggregate.
- The approved write-off scope and affected financial position are established before scenario entry.
- The Financial Adjustment has a canonical source, category, reason, institutional reference, and traceable relationship to the affected financial position.
- Structural, financial, source, category, reason, and publication-readiness validation has completed successfully.
- The enterprise approval capability has produced a positive approval outcome.
- The Financial Adjustment is in the `Approved` operational state and `Approved` lifecycle stage.
- The original financial transaction and posted accounting history remain unchanged.

---

# 8. Trigger

An approved Write-Off Adjustment becomes eligible for authoritative publication and application to the applicable financial position.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm that the Write-Off Adjustment carries the completed validation outcome, positive approval outcome, canonical classification, business reason, source, reference, approved write-off scope, and affected financial-position context required for publication. | Financial Adjustment Management | The approved Write-Off Adjustment is confirmed as publication-ready without reopening the completed approval decision. |
| 2 | Publish the approved Financial Adjustment as the authoritative enterprise representation of the write-off. | Financial Adjustment Management | The Financial Adjustment progresses from `Approved` to `Published` and becomes immutable, enterprise-visible, accounting-ready, traceable, and auditable. |
| 3 | Make the published write-off pending application to the applicable financial position. | Financial Adjustment Management | The Financial Adjustment enters `Pending Application` while preserving its canonical reference and lineage to the original financial position. |
| 4 | Apply the approved write-off to the applicable financial position without altering the original financial transaction or posted accounting history. | Financial Adjustment Management | The Financial Adjustment reaches `Applied`, and the applicable financial position reflects the approved write-off outcome. |
| 5 | Preserve the application milestone, state transition, approval participation, publication history, affected-position reference, and financial lineage as auditable business evidence. | Financial Adjustment Management | The governed financial outcome is permanently reconstructable and ready for downstream consumption without transferring ownership. |

The flow stops when the approved write-off is reflected in the applicable financial position. Downstream journal creation, ledger posting, reporting, notification, completion, and archival are outside this scenario boundary.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Publication readiness is not satisfied despite the prior approval outcome. | The Write-Off Adjustment cannot become the authoritative published adjustment and the financial position remains unchanged. | Resolved by `L2-F04` §29.3 Terminal Failure rule: the Financial Adjustment moves to `Cancelled` with an evidence reason citing the exception category (§23.3); no publication occurs. | Financial Adjustment Management |
| A financial-consistency, reference, or approved-scope exception prevents application. | The published adjustment cannot validly produce the governed write-off outcome. | Resolved by `L2-F04` §29.3: application does not proceed; the same Terminal Failure containment applies, preserving aggregate consistency, approval integrity, and financial integrity. | Financial Adjustment Management |

Recovery is governed by `L2-F04` §29.3 (ADR-FIN-ADJ-001). No technical recovery mechanism is defined at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Write-Off is a canonical Financial Adjustment category used to modify an existing financial position. | Financial Adjustment Management | The approved write-off must be represented through the canonical Financial Adjustment aggregate. | `L2-F04`, Sections 4.1 and 9.4 |
| Every modification to an existing financial position shall be represented by one canonical Financial Adjustment aggregate. | Financial Adjustment Management | No alternative write-off adjustment aggregate may be introduced by this scenario. | `L2-F04`, Sections 4 and 6 |
| Every Financial Adjustment shall have one canonical source, category, reason, and institutional reference. | Financial Adjustment Management | Publication and application require complete classification, justification, origin, identification, and traceability. | `L2-F04`, Sections 8-10 and 13 |
| Every Financial Adjustment shall undergo validation, and validation shall complete before publication. | Financial Adjustment Management | An unvalidated or publication-unready Write-Off Adjustment cannot progress through this scenario. | `L2-F04`, Section 12.6 |
| Only validated and approved Financial Adjustments may be published. | Financial Adjustment Management | The scenario begins after validation and approval and cannot bypass either milestone. | `L2-F04`, Sections 14 and 16 |
| Published Financial Adjustments and their references remain immutable. | Financial Adjustment Management | The applied Write-Off Adjustment and its published lineage cannot be edited after publication. | `L2-F04`, Sections 13.6, 16.4 and 16.6 |
| Historical financial records shall never be modified; corrections shall be represented through compensating financial transactions, including adjustment entries. | Finance | The original financial transaction and posted history remain intact when the write-off changes the current financial position. | `L2-F00`, Section 59.2 |
| A published Student Charge may reach `Written Off` only through an approved write-off. | Finance | Where the affected position is a Student Charge, the write-off outcome must be based on the approved write-off decision and preserve historical read-only treatment. | `L2-F00`, Section 16 |
| Financial Adjustment shall not create journals or maintain ledgers. | Financial Adjustment Management | Accounting consequences remain outside this scenario and under Accounting ownership. | `L2-F04`, Section 19.5 |
| Enterprise Resource Management owns disposal or write-off business cases and execution evidence for physical assets and inventory; Finance owns monetary and accounting consequences. | Enterprise Resource Management and Finance | This scenario governs only the Finance-owned adjustment outcome and must not assume ownership of physical disposal or inventory write-off execution. | `L2-F00`, Section 13, Resource Lifecycle Finance Interface |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Adjustment representing the approved write-off | Operational state `Approved`; lifecycle stage `Approved` | The approved adjustment is published as the authoritative enterprise representation. | Operational state `Published`; lifecycle stage `Published` | Financial Adjustment Management | `L2-F04`, Sections 14-16 |
| Financial Adjustment representing the approved write-off | Operational state `Published` | The published adjustment becomes pending application to the applicable financial position. | Operational state `Pending Application` | Financial Adjustment Management | `L2-F04`, Section 15.4 |
| Financial Adjustment representing the approved write-off | Operational state `Pending Application`; lifecycle stage `Published` | The approved write-off becomes part of the institutional financial position. | Operational state `Applied`; lifecycle stage `Applied` | Financial Adjustment Management | `L2-F04`, Sections 14.4 and 15.4 |

Lifecycle progression remains forward-only, completed milestones remain immutable, and exactly one active operational state exists at a time.

Where the affected financial position is a published Student Charge, `Written Off` is an authoritative alternative terminal charge state under `L2-F00` Section 16. The Financial Adjustment lifecycle remains separately owned by Financial Adjustment Management.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Resolved by `L2-F04` §17.4/§17.5 and §29.2 (ADR-FIN-ADJ-001): the event names are canonical, single publisher Financial Adjustment Management. This scenario publishes:

| Event | Scenario Step | Material Consumers |
|---|---|---|
| `AdjustmentPublicationCompleted` | Step 2 (`Approved` → `Published`) | Accounting, Reporting, Analytics, Notification, Bank Reconciliation |
| `AdjustmentApplicationCompleted` | Step 4 (`Pending Application` → `Applied`) | Accounting, Reporting, Analytics |
| `AdjustmentFinancialPositionUpdated` | Step 4, concurrent with the governed write-off outcome | Accounting, Bank Reconciliation |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain contract or governed hand-off is required inside the scenario boundary because the frozen Stage 3 semantics list only Section F as participating.

Where an approved operational asset or inventory write-off provides context, Enterprise Resource Management remains owner of the physical business case and execution evidence, while Finance owns the monetary consequence. The frozen Stage 3 row does not establish Section J participation in this scenario, so no mandatory J-to-F hand-off is asserted in this Draft.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Approve the Write-Off Adjustment before scenario entry | Enterprise Approval Authority (`L2-F04` §28.2), scope not fully resolved | The scenario may begin only after the required positive approval outcome. `L2-F00` identifies a Board-approved write-off for the Student Charge path; whether "Board" is a mandatory stricter instance of Enterprise Approval Authority for all write-offs or scoped to Student Charges only remains open per `L2-F04` §28.4. | Financial Adjustment Management owns the adjustment; enterprise approval execution remains external. |
| Publish and apply the approved Write-Off Adjustment | Adjustment Publisher (`L2-F04` §28.2) | Authorization must be validated and the approved adjustment must satisfy publication readiness, approved scope, and financial integrity constraints. | Financial Adjustment Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No Write-Off Adjustment-specific separation-of-duty, override, or exception-authority rule is defined by the authoritative L2 architecture. General Finance security and approval principles apply without creating a scenario-specific role or control at L3.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the approved basis and scope for the write-off | Immutable approval participation outcome linked to the Financial Adjustment | Financial Adjustment Management |
| Preserve adjustment identity and financial lineage | Canonical institutional adjustment reference, affected financial-position reference, source, category, and reason | Financial Adjustment Management |
| Preserve validation and publication readiness | Reconstructable validation outcome and publication milestone | Financial Adjustment Management |
| Preserve the governed financial outcome | Application milestone and resulting governed financial-position change | Financial Adjustment Management |
| Preserve lifecycle and operational history | Immutable lifecycle progression and state-transition history through `Applied` | Financial Adjustment Management |
| Preserve original financial history | Original financial transaction and posted accounting history remain unchanged and traceable to the compensating Write-Off Adjustment | Finance |

Section A retains ownership of authorization and audit infrastructure; this scenario records only the Finance-owned business obligations and evidence.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The approved Write-Off Adjustment has been published as the authoritative immutable Financial Adjustment.
- The approved write-off has been applied to the applicable financial position.
- The applicable financial position reflects the governed write-off outcome within the approved scope.
- The Financial Adjustment has reached the `Applied` operational state and lifecycle milestone.
- The original financial transaction and posted accounting history remain unchanged.
- The approval, publication, application, state, lifecycle, reference, and financial-lineage evidence is reconstructable.

## 16.2 Postconditions

- The governed post-write-off financial position is authoritative for subsequent Finance processing.
- The applied Financial Adjustment remains owned by Financial Adjustment Management and is immutable and auditable.
- Accounting-ready adjustment context is available for downstream Accounting consumption without creating journals or ledger entries inside this scenario.
- Any later reversal, correction, completion, or archival follows its separately governed lifecycle or canonical scenario.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

Three of the four issues originally recorded here (missing Actors, illustrative-only events, undefined recovery path) were resolved 2026-08-21 by `ADR-FIN-ADJ-001` and the corresponding amendment to `L2-F04` (§28, §29), approved by Dhruv (Lead Architect, lock approval). One remains open:

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F00` identifies `Written Off` as a Board-approved Student Charge outcome, while `L2-F04` §28.4 (as amended) explicitly leaves open whether "Board" is a mandatory stricter instance of Enterprise Approval Authority for all write-offs, or a configuration-level constraint scoped to Student Charges only. | The canonical approval authority for write-offs outside, and potentially within, the Student Charge path cannot be generalized without changing upstream authority. | Finance owner (Chetan) must confirm the canonical approval authority and scope, or explicitly establish configuration-governed variation, as a scoped follow-up decision within the FIN-ADJ package. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-022**
