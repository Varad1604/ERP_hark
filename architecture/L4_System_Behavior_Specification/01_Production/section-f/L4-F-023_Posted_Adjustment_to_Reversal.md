# Posted Adjustment to Reversal

## 1. Specification Metadata

**Specification ID:** L4-F-023  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Financial Adjustment Management  
**Realizes Scenario:** L3-F-023  
**Primary Business Owner:** Financial Adjustment Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-29  
**Depends On:** `L3-F-023_Posted_Adjustment_to_Reversal.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md` as amended by `ADR-FIN-ADJ-001`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall represent reversal of a posted Financial Adjustment by creating and progressing a separate Reversal Adjustment that neutralizes the original adjustment's financial effect while preserving the original adjustment, its lifecycle position, and its historical evidence unchanged.

For this scenario, the frozen term `Posted` means an original Financial Adjustment in `Applied`, `Completed`, or `Archived`. The behavior begins when such an adjustment requires reversal under an established business correction requirement. It ends when a new Reversal Adjustment carrying a `Reversed Adjustment Reference` to the original has completed validation and approval, been published, reached `Applied`, and neutralized the original financial effect.

This specification does not edit, delete, overwrite, or backward-transition the original Financial Adjustment. It does not govern successor-record correction, Journal Entry reversal, Payment reversal, Refund reversal, or accounting implementation of the reversal consequence.

---

## 3. Required Business Context

The behavior requires:

- one existing original Financial Adjustment in `Applied`, `Completed`, or `Archived`;
- authoritative original adjustment identity, source, category, reason, institutional reference, financial-position reference, and historical financial effect;
- the original Financial Adjustment remaining immutable;
- an established business correction requirement that calls for reversal rather than successor-record correction;
- a reversal basis confirmed by the Reversal Authority;
- the canonical category `Reversal Adjustment`;
- a canonical `Reversed Adjustment Reference` relating the new Reversal Adjustment to the original;
- the canonical Financial Adjustment validation, approval, publication, application, and event model.

Financial Adjustment Management retains ownership of both the original Financial Adjustment and the new Reversal Adjustment. Enterprise approval execution remains external.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-023-BH-001 | When reversal is requested, the system shall accept the original adjustment as eligible for this scenario only when it is in `Applied`, `Completed`, or `Archived` and its authoritative identity and financial lineage are available. | L3-F-023 §§2, 4.1, 7, 9 step 1 |
| L4-F-023-BH-002 | The system shall prevent reversal behavior from editing, deleting, overwriting, or changing the operational state or lifecycle position of the original Financial Adjustment. | L3-F-023 §§2, 4.3-4.4, 12, 13 |
| L4-F-023-BH-003 | When the governed correction requirement is reversal and the reversal basis is established, the system shall create a separate Financial Adjustment instance classified as `Reversal Adjustment` with a `Reversed Adjustment Reference` to the original. | L3-F-023 §§4.2, 8, 9 steps 2-3, 12-13 |
| L4-F-023-BH-004 | On initiation of the Reversal Adjustment, the system shall place the new instance in `Draft` / `Adjustment Initiated` without altering the original adjustment. | L3-F-023 §§9 step 3, 13 |
| L4-F-023-BH-005 | When the Reversal Adjustment satisfies structural, financial, source, category, reason, and reference validation, the system shall move it to `Validated`. | L3-F-023 §§9 step 4, 13 |
| L4-F-023-BH-006 | When a positive enterprise approval outcome is received for the Reversal Adjustment, the system shall move it from `Validated` to `Approved`. | L3-F-023 §§6, 9 step 5, 13, 15 |
| L4-F-023-BH-007 | When publication readiness is satisfied, the system shall publish the approved Reversal Adjustment and move it to `Published`. | L3-F-023 §§9 step 6, 13 |
| L4-F-023-BH-008 | After publication, the system shall move the Reversal Adjustment to `Pending Application`. | L3-F-023 §13 |
| L4-F-023-BH-009 | When the reversing financial effect is valid for application, the system shall apply the Reversal Adjustment and move it from `Pending Application` to `Applied`. | L3-F-023 §§4.2, 9 step 7, 13, 16 |
| L4-F-023-BH-010 | On successful application, the system shall expose the neutralized financial effect while retaining the original Financial Adjustment and historical financial record unchanged. | L3-F-023 §§3, 4.2, 9 step 7, 12, 16 |
| L4-F-023-BH-011 | The system shall not represent reversal by moving the original Financial Adjustment into a literal `Reversed` state. | L3-F-023 §§2, 12, 13 |
| L4-F-023-BH-012 | The system shall not create or reverse Accounting journals or ledger postings as part of this behavior. | L3-F-023 §§4.4, 5, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-023-BH-013 | When the Reversal Adjustment reaches `Published`, the system shall publish `AdjustmentReversalPublished` with Financial Adjustment Management retained as the canonical publisher. | L3-F-023 §§9 step 6, 14.1 |
| L4-F-023-BH-014 | When the Reversal Adjustment reaches `Applied`, the system shall publish `AdjustmentApplicationCompleted` with Financial Adjustment Management retained as the canonical publisher. | L3-F-023 §§9 step 7, 14.1 |
| L4-F-023-BH-015 | When application neutralizes the original financial effect, the system shall publish `AdjustmentFinancialPositionUpdated` with Financial Adjustment Management retained as the canonical publisher. | L3-F-023 §§9 step 7, 14.1 |
| L4-F-023-BH-016 | The system shall preserve the canonical `Reversed Adjustment Reference` as the business relationship between the new Reversal Adjustment and the original adjustment. | L3-F-023 §§4.2, 9 step 3, 14.2 |
| L4-F-023-BH-017 | The system shall make accounting-ready reversal context available for downstream consumption without transferring Financial Adjustment ownership or performing an Accounting reversal. | L3-F-023 §§5, 14.2 |

No synchronous cross-domain contract or governed hand-off is required inside this scenario boundary.

---

## 6. Failure / Rejection / Continuation

- If the original adjustment or its authoritative financial lineage cannot be established, the system shall prevent valid reversal completion and move the Reversal Adjustment to `Cancelled` with the source-supported reference-exception evidence reason.
- If the reversal authority or business reason is unresolved, the system shall retain the Reversal Adjustment at its current pre-approval position and shall not automatically advance it.
- If enterprise approval is rejected, the system shall move the Reversal Adjustment to `Cancelled` with evidence reason `Approval Rejected`; any resubmission requires a new instance.
- If the reversing financial effect cannot be applied consistently, the system shall prevent partial application and retain the Reversal Adjustment in `Pending Application`.
- In all failure cases, the original Financial Adjustment and its historical evidence shall remain unchanged.
- If the correction requirement is successor-record correction rather than reversal, continuation belongs to `L3-F-024`.

No technical retry, rollback, queue recovery, transaction compensation, or infrastructure recovery is specified here.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to reconstruct:

- the identity and lifecycle position of the original Financial Adjustment;
- the original adjustment's source, category, reason, institutional reference, financial-position reference, and historical financial effect;
- the business correction requirement and reversal basis;
- the Reversal Authority participation;
- the `Reversed Adjustment Reference`;
- the new Reversal Adjustment identity, category, validation result, and approval outcome;
- progression of the Reversal Adjustment through `Draft`, `Validated`, `Approved`, `Published`, `Pending Application`, and `Applied`, or an applicable governed failure state;
- publication and application milestones;
- the resulting neutralized financial effect;
- the unchanged state, lifecycle position, and historical evidence of the original Financial Adjustment;
- the occurrence of `AdjustmentReversalPublished`, `AdjustmentApplicationCompleted`, and `AdjustmentFinancialPositionUpdated` where applicable.

The system shall preserve evidence that the Reversal Authority and Enterprise Approval Authority are distinct business authority concepts under the upstream no-self-certification principle.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. the original Financial Adjustment remains unchanged and historically available;
2. a separate Financial Adjustment classified as `Reversal Adjustment` exists with a `Reversed Adjustment Reference` to the original;
3. the Reversal Adjustment has completed validation and received the required positive approval outcome;
4. the Reversal Adjustment has been published;
5. the Reversal Adjustment has reached `Applied`;
6. the original adjustment's financial effect has been neutralized;
7. the original adjustment has not transitioned backward or into a literal `Reversed` state;
8. the required reversal, authority, reference, lifecycle, financial-effect, and event evidence is reconstructable.

Successor-record correction, Accounting reversal, Payment reversal, Refund reversal, and downstream accounting implementation are outside this completion boundary.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-023` — Posted Adjustment to Reversal  
**Primary verification source:** `L2-F04` — Financial Adjustment Management  
**Additional verification source:** `L2-F00` — Finance Domain Model and Foundation  

Behavioral provenance is anchored in `L3-F-023`.

`L2-F04` verifies the resolved `Posted` mapping, the forward-only Financial Adjustment lifecycle, the canonical `Reversal Adjustment` category, the `Reversed Adjustment Reference`, authority participation, governed recovery behavior, and canonical reversal/application events.

`L2-F00` verifies the Finance-level rule that historical financial records are not rewritten and corrections are represented through compensating financial outcomes.

No L4 behavior in this specification creates a new reversal state, lifecycle stage, authority, event, publisher, contract, or accounting responsibility.

---

## 10. Open Issues / Deferred Design

No behavior-specific L4 issue remains from the parent semantics. The prior ambiguity around `Posted` / `Reversed` terminology and the reversal mechanism was resolved upstream by `ADR-FIN-ADJ-001` and incorporated into `L2-F04`.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/rollback/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to downstream architecture levels.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
