# Adjustment Error to Corrected Successor Record

# 1. Scenario Metadata

**Scenario ID:** L3-F-024  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Financial Adjustment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md` v1.1.0 (as amended by `ADR-FIN-ADJ-001`); frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs correction when an existing Financial Adjustment is found to contain an error. It preserves the erroneous adjustment as immutable historical evidence and establishes a separately traceable corrected successor record rather than editing, deleting, or overwriting the original adjustment.

The frozen scenario requires a corrected successor record. The canonical successor relationship, supersession treatment, and correction lifecycle are resolved by `L2-F04` §29.1 (ADR-FIN-ADJ-001, approved 2026-08-21 by Dhruv, Lead Architect): the successor is a new Financial Adjustment, Category `Correction Adjustment`, carrying a `Corrected Adjustment Reference` to the erroneous original; no `Superseded` state is added to the original.

---

# 3. Business Outcome

A corrected successor Financial Adjustment record has been authoritatively established and linked to the immutable erroneous adjustment, preserving complete historical and financial traceability without altering the original record.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed business finding confirms that an existing Financial Adjustment contains an error requiring correction through a successor record.

## 4.2 End Boundary

The scenario ends when the corrected successor record has completed validation and approval, has been published as the authoritative correction, and retains an immutable relationship to the original erroneous adjustment.

Resolved by `L2-F04` §29.1: the end boundary is `Published`, not `Applied`. Application of the corrected successor to the financial position follows the standard `Pending Application → Applied` path as a normal continuation of the same scenario shape used by `L3-F-020`–`022`, and is recorded here as a postcondition (§16.2) rather than a separate unresolved boundary.

## 4.3 In Scope

- Identification of the erroneous Financial Adjustment and the confirmed correction requirement.
- Preservation of the original adjustment as immutable historical evidence.
- Establishment of a corrected successor Financial Adjustment record.
- Validation, approval participation, and publication of the corrected successor.
- Preservation of the original-to-successor relationship, correction reason, and complete audit lineage.

## 4.4 Out of Scope

- Editing, deleting, overwriting, or silently replacing the erroneous adjustment.
- Reversal of a posted adjustment under `L3-F-023`.
- Correction of Accounting-owned journals or ledger entries.
- Creation of an alternative correction aggregate outside Financial Adjustment Management.
- Definition of a new supersession state, successor contract, correction event, approval role, or recovery mechanism.
- Technical propagation of the corrected record to consumers.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | `L3-F-023` Posted Adjustment to Reversal | Governs reversal rather than correction through a successor record. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Adjustment Management / Section F | Owns the erroneous Financial Adjustment, the corrected successor record, their lifecycle and state, correction lineage, publication, and audit obligations. |

The frozen Stage 3 row defines only Section F as participating. Downstream consumers may use the corrected published adjustment, but consumption does not transfer ownership or authorize modification of either the original or successor record.

---

# 6. Business Actors, Roles and Responsibilities

Resolved by `L2-F04` §28.2 (ADR-FIN-ADJ-001):

| Actor | Role in this scenario |
|---|---|
| Correction Authority | Confirms the error finding on the Published/Applied original and directs correction via a successor record (Steps 1–2). Cannot edit or withdraw the erroneous original. |
| Adjustment Requestor | Establishes the corrected successor record carrying the corrected information and the `Corrected Adjustment Reference` (Step 3). |
| Enterprise Approval Authority | Produces the positive approval outcome for the corrected successor (Step 5). Acts externally per `L2-F04` §11 and §28.2. |
| Adjustment Publisher | Confirms publication readiness and publishes the corrected successor (Steps 4 and 6). |

`L2-F04` §28.3 No Self-Certification Principle applies throughout.

---

# 7. Preconditions and Entry Criteria

- An existing canonical Financial Adjustment is available with its source, category, reason, institutional reference, financial-position reference, lifecycle, state, and audit history.
- A governed business finding confirms that the existing adjustment contains an error.
- The original adjustment remains immutable and historically available.
- The correction requirement belongs to Financial Adjustment Management and calls for a corrected successor record rather than reversal.
- The correction basis and relationship to the original adjustment are available for validation and audit.

---

# 8. Trigger

A governed adjustment-error finding requires the erroneous Financial Adjustment to be corrected through a separately traceable successor record.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the erroneous Financial Adjustment, its canonical reference, error finding, affected financial context, and immutable historical record. | Correction Authority (`L2-F04` §28.2) | The correction is anchored to one authoritative original adjustment. |
| 2 | Confirm that correction through a successor record is the governed path and preserve the business reason and correction basis. | Correction Authority | The original adjustment remains unchanged and the successor requirement is explainable and auditable. |
| 3 | Establish a new Financial Adjustment, Category `Correction Adjustment`, carrying a `Corrected Adjustment Reference` to the erroneous original. | Adjustment Requestor | A separately identifiable corrected successor record exists within the canonical Financial Adjustment ownership boundary. |
| 4 | Validate the corrected successor for structural, financial, source, category, reason, reference, and publication readiness. | Adjustment Publisher | The successor reaches the authoritative `Validated` state and lifecycle milestone if all required validation succeeds. |
| 5 | Obtain the required positive enterprise approval outcome for the corrected successor. | Enterprise Approval Authority | The successor reaches the authoritative `Approved` state and lifecycle milestone without transferring adjustment ownership. |
| 6 | Publish the approved corrected successor as the authoritative correction, publishing `AdjustmentCorrectionPublished`, while preserving the original erroneous record and correction lineage. | Adjustment Publisher | The corrected successor reaches `Published`; both records remain immutable, distinguishable, and historically reconstructable. |
| 7 | Preserve the error finding, correction basis, approval participation, publication outcome, and original-to-successor relationship as auditable evidence. | Financial Adjustment Management | The corrected successor outcome is permanently traceable without alteration of the original adjustment. |

The flow stops at the frozen outcome of a corrected successor record (`Published`, per `L2-F04` §29.1). The successor's own subsequent progression to `Applied` follows the standard path used by `L3-F-020`–`022` and is recorded as a postcondition (§16.2).

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

Where the governed correction requirement is reversal rather than a corrected successor record, processing belongs to `L3-F-023` and is outside this scenario.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The error finding or original adjustment cannot be authoritatively established. | A successor cannot be safely related to a verified erroneous adjustment. | Resolved by `L2-F04` §29.3 Terminal Failure rule: the successor moves to `Cancelled` with an evidence reason citing the reference exception category (§23.3); the original record remains unchanged. | Financial Adjustment Management |
| The corrected successor fails validation. | The successor cannot progress to approval or publication. | Resolved by `L2-F04` §29.3 Terminal Failure rule: the successor moves to `Cancelled`; the original adjustment remains unchanged; resubmission requires a new instance. | Financial Adjustment Management |
| Approval is rejected. | The successor cannot become the authoritative published correction. | Resolved by `L2-F04` §29.3 Rejected Approval rule: the successor moves to `Cancelled` with evidence reason "Approval Rejected". | Financial Adjustment Management |
| The proposed correction would compromise financial consistency or lineage. | The successor cannot validly correct the erroneous adjustment. | Resolved by `L2-F04` §29.3: publication is contained while preserving all existing records and audit evidence. | Financial Adjustment Management |

Recovery is governed by `L2-F04` §29.3 (ADR-FIN-ADJ-001). No technical recovery mechanism is defined at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial corrections shall not modify historical transactions. | Finance | The erroneous adjustment must remain unchanged and the correction must be represented separately. | `L2-F00`, Sections 4 and 11, Adjustment |
| Historical financial records shall never be modified; corrections use compensating financial transactions. | Finance | The successor must preserve the original record and provide separately traceable correction evidence. | `L2-F00`, Section 59.2 |
| Financial Adjustment is the only canonical adjustment aggregate, and future financial modifications extend it rather than introduce independent adjustment entities. | Financial Adjustment Management | The corrected successor remains within Financial Adjustment Management ownership and does not create a competing aggregate. | `L2-F04`, Sections 4 and 24 |
| Historical Financial Adjustments and published references remain immutable. | Financial Adjustment Management | Neither the erroneous record nor its published references may be overwritten by the correction. | `L2-F04`, Sections 7.7, 13.6 and 16.4 |
| Every Financial Adjustment shall possess a canonical source, category, reason, and institutional reference. | Financial Adjustment Management | The corrected successor must be independently identifiable, justified, classified, and traceable. | `L2-F04`, Sections 8-10 and 13 |
| Every Financial Adjustment shall undergo validation before publication. | Financial Adjustment Management | The corrected successor cannot be approved for publication without completing validation. | `L2-F04`, Section 12.6 |
| Only validated and approved Financial Adjustments may be published. | Financial Adjustment Management | The corrected successor must complete the applicable validation and approval milestones before authoritative publication. | `L2-F04`, Sections 14 and 16 |
| Events represent completed business facts and corrections generate new events. | Finance | Correction must not alter a historical event; the exact successor-correction event remains to be defined upstream. | `L2-F00`, Section 22 |
| Financial Adjustment shall not create journals or maintain ledgers. | Financial Adjustment Management | Accounting correction consequences remain outside this scenario and under Accounting ownership. | `L2-F04`, Section 19.5 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Erroneous Financial Adjustment | Existing authoritative state and lifecycle position | The error finding creates a correction requirement but does not alter the historical adjustment. | Existing state and lifecycle history remain unchanged and immutable. | Financial Adjustment Management | `L2-F04`, Sections 14-16, 21 and 29.1 |
| Corrected successor Financial Adjustment (Category `Correction Adjustment`) | `Draft` / `Adjustment Initiated` | The successor is established and undergoes structural, financial, source, category, reason, and reference validation. | `Validated` state and lifecycle milestone | Financial Adjustment Management | `L2-F04`, Sections 12, 14 and 15 |
| Corrected successor Financial Adjustment | `Validated` | Positive enterprise approval participation is obtained. | `Approved` state and lifecycle milestone | Financial Adjustment Management | `L2-F04`, Sections 11, 14 and 15 |
| Corrected successor Financial Adjustment | `Approved` | The corrected successor becomes the authoritative published correction record; scenario end boundary. | `Published` state and lifecycle milestone | Financial Adjustment Management | `L2-F04`, Sections 14-16 and 29.1 |

Resolved by `L2-F04` §29.1 (ADR-FIN-ADJ-001): no `Superseded`, `Corrected`, or equivalent state is added to the erroneous original — supersession is evidenced entirely by the successor's `Corrected Adjustment Reference` pointing back to it, consistent with §7.7 constraint 4 (historical adjustments remain immutable). The successor's own subsequent progression to `Pending Application`/`Applied` is a postcondition of this scenario (§16.2), not part of its state table, matching the resolved `Published` end boundary in Section 4.2.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Resolved by `L2-F04` §17.4/§29.1/§29.2 (ADR-FIN-ADJ-001). Single publisher: Financial Adjustment Management.

| Event | Scenario Step | Material Consumers |
|---|---|---|
| `AdjustmentCorrectionPublished` | Step 6 (`Approved` → `Published`) | Accounting, Reporting, Analytics, Notification |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain contract or governed hand-off is established inside the scenario boundary because the frozen Stage 3 row lists only Section F as participating.

The original-to-successor relationship is the canonical `Corrected Adjustment Reference` defined in `L2-F04` §13.4 and §29.1 (ADR-FIN-ADJ-001).

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Confirm the adjustment error and select successor-record correction | Correction Authority (`L2-F04` §28.2) | A governed, explainable error finding and correction requirement must exist. Cannot edit or withdraw the erroneous original. | Financial Adjustment Management |
| Approve the corrected successor | Enterprise Approval Authority (`L2-F04` §28.2) | The successor may be published only after a positive enterprise approval outcome. | Financial Adjustment Management owns the adjustment; enterprise approval execution remains external. |
| Publish the corrected successor | Adjustment Publisher (`L2-F04` §28.2) | Authorization must be validated and publication readiness must be satisfied. | Financial Adjustment Management |

## 15.2 Separation of Duties, Overrides and Exceptions

`L2-F04` §28.3 No Self-Certification Principle applies: the Correction Authority that confirms the error is never, by that action alone, the Enterprise Approval Authority whose outcome the successor consumes.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the erroneous adjustment | Immutable original Financial Adjustment, canonical reference, lifecycle, state, financial context, and history | Financial Adjustment Management |
| Preserve the error finding | Governed and explainable business finding establishing the correction requirement | Financial Adjustment Management |
| Preserve successor identity | Canonical institutional reference, source, category, reason, and correction relationship | Financial Adjustment Management |
| Preserve validation and approval | Reconstructable successor validation outcome and approval participation outcome | Financial Adjustment Management |
| Preserve publication | Immutable publication milestone for the corrected successor | Financial Adjustment Management |
| Preserve correction lineage | Permanent relationship between the erroneous adjustment and corrected successor | Financial Adjustment Management |

Section A retains ownership of authorization and audit infrastructure; this scenario records only the Finance-owned business obligations and evidence.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The erroneous Financial Adjustment remains unchanged and historically available.
- A separately identifiable corrected successor Financial Adjustment has been established.
- The corrected successor has completed validation and received a positive approval outcome.
- The corrected successor has reached the `Published` state and lifecycle milestone.
- The correction basis and original-to-successor relationship are permanently reconstructable.
- No Accounting-owned journal, ledger, or posting lifecycle has been re-owned by this scenario.

## 16.2 Postconditions

- The original erroneous adjustment remains immutable and distinguishable from the corrected successor; supersession is evidenced solely by the successor's `Corrected Adjustment Reference` (`L2-F04` §29.1).
- The corrected successor is the published authoritative correction record.
- Downstream consumers may use the corrected published adjustment without modifying either record or acquiring ownership.
- The corrected successor's subsequent progression through `Pending Application` to `Applied` (applying the correction to the financial position) follows the standard path used by `L3-F-020`–`022` as a separate continuation of the same Financial Adjustment instance, not a new scenario. Any Accounting consequence remains owned by Accounting and General Ledger Management.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

None remain for this scenario. All six issues originally recorded here (the successor/supersession relationship, treatment of the erroneous original, the `Published` vs `Applied` end boundary, missing Actors, illustrative-only events, and undefined recovery paths) were resolved 2026-08-21 by `ADR-FIN-ADJ-001` and the corresponding amendment to `L2-F04` (§29.1–29.5), approved by Dhruv (Lead Architect, lock approval). See Sections 4, 6, 9, 11, 13, 14, and 15 above.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-024**
