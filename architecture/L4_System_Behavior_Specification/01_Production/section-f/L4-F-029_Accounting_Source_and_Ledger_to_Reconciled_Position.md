# Accounting Source and Ledger to Reconciled Position

## 1. Specification Metadata

**Specification ID:** L4-F-029  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Accounting & General Ledger  
**Realizes Scenario:** L3-F-029  
**Primary Business Owner:** Accounting & General Ledger  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-29  
**Depends On:** `L3-F-029_Accounting_Source_and_Ledger_to_Reconciled_Position.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall compare an authoritative accounting source with the corresponding Accounting Ledger population and govern the comparison through a Finance-owned Ledger Reconciliation Record until the position is closed as reconciled.

The behavior begins when an authoritative accounting source and corresponding ledger population are available for comparison. It ends when the Ledger Reconciliation Record reaches `Reconciled`, the closed reconciliation fact is published, and the supporting evidence is reconstructable.

This specification does not transfer ownership of source-domain facts, directly correct source aggregates, own corrective transactions, define institution-specific tolerance/materiality policy, or prescribe technical reconciliation implementation.

---

## 3. Required Business Context

The behavior requires:

- an authoritative accounting source;
- identifiable corresponding ledger population;
- explicit reconciliation scope;
- explicit period or as-of boundary;
- source identity, amount, classification, and chronology sufficient for comparison;
- an Accountant performing the reconciliation;
- a Senior Accountant holding reconciliation closure authority;
- the canonical Ledger Reconciliation Record lifecycle.

Accounting & General Ledger owns the Ledger Reconciliation Record. Source facts remain owned by their originating Finance capabilities. Any corrective transaction remains owned by the capability that canonically owns that correction.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-F-029-BH-001 | When a governed reconciliation begins, the system shall establish a Ledger Reconciliation Record with explicit source scope, ledger scope, and period/as-of boundary. | L3-F-029 §§4.1, 7-9 step 1 |
| L4-F-029-BH-002 | The system shall preserve source-fact ownership outside the Ledger Reconciliation Record while permitting the record to reference the source facts required for comparison. | L3-F-029 §§4.4, 5, 12 |
| L4-F-029-BH-003 | While the record is in `Comparing`, the system shall permit the Accountant to compare authoritative source facts with the corresponding ledger position and record the comparison result. | L3-F-029 §§6, 9 step 2, 13 |
| L4-F-029-BH-004 | When comparison identifies no unresolved difference, the system shall make the reconciliation eligible for Senior Accountant closure review without claiming closure before that review completes. | L3-F-029 §§9 steps 3 and 7, 12 |
| L4-F-029-BH-005 | When an unresolved difference is identified, the system shall move the Ledger Reconciliation Record from `Comparing` to `Exception`. | L3-F-029 §§9 step 4, 10.1, 13 |
| L4-F-029-BH-006 | On entry to `Exception`, the system shall preserve the identified difference and supporting reconciliation evidence. | L3-F-029 §§9 step 4, 11, 15 |
| L4-F-029-BH-007 | The system shall not modify source-domain facts or create corrective transactions inside the Ledger Reconciliation Record. | L3-F-029 §§4.4, 5, 9 step 5, 12 |
| L4-F-029-BH-008 | After the canonically owning Finance capability resolves or authoritatively explains the relevant difference, the system shall permit the reconciliation to move from `Exception` back to `Comparing` for re-comparison. | L3-F-029 §§9 steps 5-6, 10.1, 13 |
| L4-F-029-BH-009 | The system shall prevent reconciliation closure while an unresolved material difference remains. | L3-F-029 §§10.1, 12, 16 |
| L4-F-029-BH-010 | When the final comparison evidence is complete and Senior Accountant review confirms closure, the system shall move the Ledger Reconciliation Record from `Comparing` to `Reconciled`. | L3-F-029 §§6, 9 step 7, 12-13, 16 |
| L4-F-029-BH-011 | On reconciliation closure, the system shall preserve source scope, ledger scope, comparison result, differences, corrective references, performer, reviewer, and closure decision as auditable evidence. | L3-F-029 §§3, 9 step 8, 15-16 |
| L4-F-029-BH-012 | The system shall not overwrite historical accounting records merely to force a reconciled result. | L3-F-029 §12 |
| L4-F-029-BH-013 | The system shall consume institution-governed tolerance/materiality policy only where supplied by authoritative governance and shall not create or fix such policy within this behavior. | L3-F-029 §§12, 17 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-F-029-BH-014 | When an unresolved reconciliation difference is formally recorded, the system shall publish `ReconciliationExceptionRaised` with Accounting & General Ledger retained as the canonical publisher. | L3-F-029 §§11, 14 |
| L4-F-029-BH-015 | When Senior Accountant review closes the Ledger Reconciliation Record as `Reconciled`, the system shall publish `LedgerReconciled` with Accounting & General Ledger retained as the canonical publisher. | L3-F-029 §§4.2, 9 step 8, 14 |
| L4-F-029-BH-016 | The system shall preserve references to corrective outcomes owned by other Finance capabilities without assuming ownership of those corrective transactions. | L3-F-029 §§3, 4.3-4.4, 5, 9 step 5 |

Cross-domain correction references do not transfer ownership to Accounting & General Ledger.

---

## 6. Failure / Rejection / Continuation

- An unresolved difference shall be treated as a governed reconciliation exception, not as a technical failure.
- When a difference is unresolved, the system shall retain the Ledger Reconciliation Record in `Exception` and prevent successful closure.
- Corrective business action shall occur through the canonically owning Finance workflow outside the reconciliation aggregate.
- After a relevant corrective outcome or authoritative explanation exists, the reconciliation may return to `Comparing`.
- The system shall not create technical retry, rollback, or automated correction behavior within this specification.

---

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to reconstruct:

- reconciliation record identity;
- authoritative source scope and source references;
- ledger scope;
- period/as-of boundary;
- each comparison result;
- identified differences;
- supporting evidence for each exception;
- references to corrective or explanatory outcomes;
- the Accountant who performed the comparison;
- the Senior Accountant who reviewed and closed the reconciliation;
- lifecycle progression through `Comparing`, `Exception`, and `Reconciled`;
- the occurrence of `ReconciliationExceptionRaised` and `LedgerReconciled` where applicable.

Internal Auditor access remains read-only and does not alter the reconciliation outcome.

---

## 8. Observable Completion Conditions

The behavior is complete when:

1. the Ledger Reconciliation Record has reached `Reconciled`;
2. source and ledger scope remain identifiable and traceable;
3. all material differences are resolved or authoritatively accounted for;
4. Senior Accountant closure review has completed;
5. `LedgerReconciled` has been published;
6. source facts remain owned by their originating capabilities;
7. corrective transactions remain owned by their canonical capabilities;
8. complete reconciliation and closure evidence is reconstructable.

A record in `Exception` is not a successful completion state.

---

## 9. Traceability & Provenance

**Parent:** `L3-F-029` — Accounting Source and Ledger to Reconciled Position  
**Primary verification source:** `L2-F05` — Accounting & General Ledger Management  
**Material amendment:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  

Behavioral provenance is anchored in `L3-F-029`.

The locked F05 amendment verifies the Ledger Reconciliation Record as an Accounting-owned supporting aggregate, its lifecycle `Comparing → Reconciled` and `Comparing → Exception → Comparing`, Accountant/Senior Accountant responsibilities, `ReconciliationExceptionRaised`, `LedgerReconciled`, and the rule that source facts and corrective transactions remain externally owned.

No L4 behavior in this specification creates a new tolerance policy, corrective authority, reconciliation state, event, publisher, or ownership assignment.

---

## 10. Open Issues / Deferred Design

Institution-specific tolerance/materiality policy remains externally governed and configurable; this specification does not fix or infer those thresholds.

API/interface design, persistence/data model, service/component design, comparison algorithms, technical matching logic, event transport, technical access control, retry/rollback mechanics, infrastructure/deployment, and UI/UX remain deferred to downstream architecture levels.

---

**Draft Validation Classification:** Prepared as an L4 Draft for architecture review.
