# Approved Credit Note to Reduced Financial Liability

# 1. Scenario Metadata

**Scenario ID:** L3-F-020  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Financial Adjustment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md` v1.1.1  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs how an approved Credit Note becomes an authoritative reduction of an existing financial liability without editing the historical financial transaction or posted accounting entry that established that liability. It preserves Finance ownership, financial integrity, immutability, and complete traceability while applying the approved modification to the institutional financial position.

---

# 3. Business Outcome

The approved Credit Note has been published and applied as an immutable Financial Adjustment, and the applicable financial liability is authoritatively reduced while the original financial history remains unchanged and traceable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a validated Credit Note has received a positive enterprise approval outcome and is in the `Approved` state.

## 4.2 End Boundary

The scenario ends when the approved Credit Note has been published, applied to the applicable financial position, and reached the `Applied` state and lifecycle milestone, producing the reduced financial liability defined by the canonical scenario outcome.

## 4.3 In Scope

- Confirmation that the approved Credit Note is ready for publication.
- Canonical publication of the approved Financial Adjustment.
- Application of the approved credit modification to the applicable financial position.
- Preservation of the original financial record and complete adjustment lineage.
- Recording of the relevant lifecycle, state, approval, publication, and application evidence.

## 4.4 Out of Scope

- Creation, validation, or approval of the Credit Note before it reaches the `Approved` state.
- Creation or amendment of the original financial obligation.
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
| Financial Adjustment Management / Section F | Owns the Financial Adjustment aggregate, its publication, application milestone, lifecycle, state, references, and the final reduced-liability outcome. |

The Stage 3 canonical row defines only Section F as participating. Accounting may consume accounting-ready adjustment information after publication, but that downstream consumption does not form part of this scenario outcome and does not transfer ownership.

---

# 6. Business Actors, Roles and Responsibilities

Resolved by `L2-F04` §28.2 (ADR-FIN-ADJ-001). Within this scenario's boundary (`Approved` → `Applied`):

| Actor | Role in this scenario |
|---|---|
| Adjustment Publisher | Confirms publication readiness (Step 1), publishes the Financial Adjustment (Step 2), and executes the `Pending Application → Applied` progression (Steps 3–4). |
| Enterprise Approval Authority | Produced the positive approval outcome that established the `Approved` entry state before this scenario begins; acts outside this scenario's boundary per `L2-F04` §11 and §28.2. |

No separation-of-duty or override rule beyond `L2-F04` §28.3 (Actor Boundary Principle, No Self-Certification Principle) applies to this scenario.

---

# 7. Preconditions and Entry Criteria

- A canonical Credit Note exists as a Finance-owned, immutable record representing a reduction of financial liability.
- The corresponding Financial Adjustment has a canonical source, category, reason, institutional reference, and traceable relationship to the affected financial position.
- Structural, financial, source, category, reason, and publication-readiness validation has completed successfully.
- The enterprise approval capability has produced a positive approval outcome.
- The Financial Adjustment is in the `Approved` operational state and `Approved` lifecycle stage.
- The original financial transaction and posted accounting history remain unchanged.

---

# 8. Trigger

An approved Credit Note becomes eligible for authoritative publication and application to the applicable financial liability.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm that the Credit Note carries the completed validation outcome, positive approval outcome, canonical classification, business reason, source, reference, and affected financial-position context required for publication. | Financial Adjustment Management | The approved Credit Note is confirmed as publication-ready without reopening the completed approval decision. |
| 2 | Publish the approved Financial Adjustment as the authoritative enterprise representation of the Credit Note. | Financial Adjustment Management | The Financial Adjustment progresses from `Approved` to `Published` and becomes immutable, enterprise-visible, accounting-ready, traceable, and auditable. |
| 3 | Make the published credit modification pending application to the applicable financial position. | Financial Adjustment Management | The Financial Adjustment enters `Pending Application` while preserving its canonical reference and lineage to the original liability. |
| 4 | Apply the approved credit modification to the applicable financial position without altering the original financial transaction or posted accounting history. | Financial Adjustment Management | The Financial Adjustment reaches `Applied`, and the applicable financial liability is reduced by the governed Credit Note outcome. |
| 5 | Preserve the application milestone, state transition, approval participation, publication history, affected-position reference, and financial lineage as auditable business evidence. | Financial Adjustment Management | The reduced-liability outcome is permanently reconstructable and ready for downstream consumption without transferring ownership. |

The flow stops at the reduced-liability outcome. Downstream journal creation, ledger posting, reporting, notification, completion, and archival are outside this scenario boundary.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Publication readiness is not satisfied despite the prior approval outcome. | The Credit Note cannot become the authoritative published adjustment and the liability remains unchanged. | Resolved by `L2-F04` §29.3 Terminal Failure rule: the Financial Adjustment moves to `Cancelled` with an evidence reason citing the exception category (§23.3); no publication occurs. | Financial Adjustment Management |
| A financial-consistency or reference exception prevents application. | The published adjustment cannot validly reduce the financial liability. | Resolved by `L2-F04` §29.3: application does not proceed; the same Terminal Failure containment applies, preserving aggregate consistency and financial integrity. | Financial Adjustment Management |

Recovery is governed by `L2-F04` §29.3 (ADR-FIN-ADJ-001). No technical recovery mechanism is defined at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Credit Note represents a reduction of student liability and is used instead of editing posted accounting entries. | Finance | The outcome must reduce the liability through a separate immutable record while preserving original posted history. | `L2-F00`, Section 11, Credit Note |
| Every modification to an existing financial position shall be represented by one canonical Financial Adjustment aggregate. | Financial Adjustment Management | The Credit Note outcome is governed through the canonical Financial Adjustment aggregate rather than an alternative adjustment aggregate. | `L2-F04`, Sections 4 and 6 |
| Every Financial Adjustment shall have one canonical source, category, reason, and institutional reference. | Financial Adjustment Management | Publication and application require complete classification, justification, origin, identification, and traceability. | `L2-F04`, Sections 8-10 and 13 |
| Every Financial Adjustment shall undergo validation, and validation shall complete before publication. | Financial Adjustment Management | An unvalidated or publication-unready Credit Note cannot progress through this scenario. | `L2-F04`, Section 12.6 |
| Only validated and approved Financial Adjustments may be published. | Financial Adjustment Management | The scenario begins after validation and approval and cannot bypass either milestone. | `L2-F04`, Sections 14 and 16 |
| Published Financial Adjustments and their references remain immutable. | Financial Adjustment Management | The applied Credit Note and its published lineage cannot be edited after publication. | `L2-F04`, Sections 13.6, 16.4 and 16.6 |
| Historical financial records shall never be modified; corrections use compensating financial transactions. | Finance | The original liability history and posted entries remain intact when the Credit Note reduces the current financial position. | `L2-F00`, Section 59.2 |
| Financial Adjustment shall not create journals or maintain ledgers. | Financial Adjustment Management | Accounting consequences remain outside this scenario and under Accounting ownership. | `L2-F04`, Section 19.5 |
| Financial Adjustments shall never generate or apply Late Fees, but may waive or remove Late Fees through an Administrative Credit. | Financial Adjustment Management | A Credit Note may reduce an existing late-fee liability but cannot create a late fee. | `L2-F04`, Section 24, Late Fee Constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Adjustment representing the approved Credit Note | Operational state `Approved`; lifecycle stage `Approved` | The approved adjustment is published as the authoritative enterprise representation. | Operational state `Published`; lifecycle stage `Published` | Financial Adjustment Management | `L2-F04`, Sections 14-16 |
| Financial Adjustment representing the approved Credit Note | Operational state `Published` | The published adjustment becomes pending application to the applicable financial position. | Operational state `Pending Application` | Financial Adjustment Management | `L2-F04`, Section 15.4 |
| Financial Adjustment representing the approved Credit Note | Operational state `Pending Application`; lifecycle stage `Published` | The credit modification becomes part of the institutional financial position and reduces the applicable liability. | Operational state `Applied`; lifecycle stage `Applied` | Financial Adjustment Management | `L2-F04`, Sections 14.4 and 15.4 |

Lifecycle progression remains forward-only, completed milestones remain immutable, and exactly one active operational state exists at a time.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Resolved by `L2-F04` §17.4/§17.5 and §29.2 (ADR-FIN-ADJ-001): the event names are canonical, single publisher Financial Adjustment Management. This scenario publishes:

| Event | Scenario Step | Material Consumers |
|---|---|---|
| `AdjustmentPublicationCompleted` | Step 2 (`Approved` → `Published`) | Accounting, Reporting, Analytics, Notification, Bank Reconciliation |
| `AdjustmentApplicationCompleted` | Step 4 (`Pending Application` → `Applied`) | Accounting, Reporting, Analytics |
| `AdjustmentFinancialPositionUpdated` | Step 4, concurrent with the reduced-liability outcome | Accounting, Bank Reconciliation |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain contract or governed hand-off is required inside the scenario boundary because the frozen Stage 3 semantics list only Section F as participating.

The published Financial Adjustment may subsequently provide accounting-ready context to Accounting and may be consumed by Reporting, Analytics, Notification, and Bank Reconciliation. Those downstream consumptions occur after this scenario's reduced-liability outcome and do not transfer ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Approve the Credit Note before scenario entry | Enterprise Approval Authority (`L2-F04` §28.2) | Credit Notes may require configurable enterprise approval; this scenario may begin only after a positive approval outcome. Acts before this scenario's boundary. | Financial Adjustment Management owns the adjustment; enterprise approval execution remains external. |
| Publish and apply the approved Credit Note | Adjustment Publisher (`L2-F04` §28.2) | Authorization must be validated and the approved adjustment must satisfy publication readiness and financial integrity constraints. | Financial Adjustment Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No Credit Note-specific separation-of-duty, override, or exception-authority rule is defined by the authoritative L2 architecture. General Finance security and approval principles apply without creating a scenario-specific role or control at L3.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the approved basis for the Credit Note | Immutable approval participation outcome linked to the Financial Adjustment | Financial Adjustment Management |
| Preserve adjustment identity and financial lineage | Canonical institutional adjustment reference, affected financial-position reference, source, category, and reason | Financial Adjustment Management |
| Preserve validation and publication readiness | Reconstructable validation outcome and publication milestone | Financial Adjustment Management |
| Preserve the reduced-liability outcome | Application milestone and the resulting governed financial-position change | Financial Adjustment Management |
| Preserve lifecycle and operational history | Immutable lifecycle progression and state-transition history through `Applied` | Financial Adjustment Management |
| Preserve original financial history | Original financial transaction and posted accounting history remain unchanged and traceable to the compensating Credit Note | Finance |

Section A retains ownership of authorization and audit infrastructure; this scenario records only the Finance-owned business obligations and evidence.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The approved Credit Note has been published as the authoritative immutable Financial Adjustment.
- The Credit Note has been applied to the applicable financial position.
- The applicable financial liability has been reduced by the governed credit modification.
- The Financial Adjustment has reached the `Applied` operational state and lifecycle milestone.
- The original financial transaction and posted accounting history remain unchanged.
- The approval, publication, application, state, lifecycle, reference, and financial-lineage evidence is reconstructable.

## 16.2 Postconditions

- The reduced financial liability is the current authoritative financial position for subsequent Finance processing.
- The applied Financial Adjustment remains owned by Financial Adjustment Management and is immutable and auditable.
- Accounting-ready adjustment context is available for downstream Accounting consumption without creating journals or ledger entries inside this scenario.
- Any later reversal, correction, completion, or archival follows its separately governed lifecycle or canonical scenario.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

None remain for this scenario. All three issues originally recorded here (missing Actors, illustrative-only events, undefined recovery path) were resolved 2026-08-21 by `ADR-FIN-ADJ-001` and the corresponding amendment to `L2-F04` (§28, §29), approved by Dhruv (Lead Architect, lock approval). See Sections 6, 11, and 14 above.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-020**
