# Accounting Source and Ledger to Reconciled Position

# 1. Scenario Metadata

**Scenario ID:** L3-F-029  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Compare an authoritative accounting source with the Accounting Ledger and close the comparison as a governed Ledger Reconciliation Record.

---

# 3. Business Outcome

The accounting source and corresponding ledger position are reconciled, with comparison scope, differences, resolution references and closure evidence preserved in a Finance-owned Ledger Reconciliation Record.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authoritative accounting source and corresponding ledger population are available for comparison.

## 4.2 End Boundary

The Ledger Reconciliation Record reaches `Reconciled` and `LedgerReconciled` records the closed reconciliation fact.

## 4.3 In Scope

Comparison, difference identification, exception recording, reference to corrective outcomes, re-comparison and reconciliation closure.

## 4.4 Out of Scope

Ownership of source-domain facts, direct correction of source aggregates, correction-policy design and technical reconciliation implementation.

---

# 5. Ownership and Participating Domains

Accounting & General Ledger owns the Ledger Reconciliation Record. Source facts remain owned by their originating Finance capabilities; corrective transactions remain owned by the capability that canonically owns each correction.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Accountant | Performs reconciliation and records identified differences. | Comparison responsibility. |
| Senior Accountant | Reviews evidence and closes the reconciliation. | Reconciliation closure authority. |
| Internal Auditor | Examines reconciliation evidence. | Read-only. |

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §2.

---

# 7. Preconditions and Entry Criteria

- Accounting source identity, amount, classification and chronology are available.
- Corresponding ledger population is identifiable.
- Reconciliation scope and period/as-of boundary are explicit.

---

# 8. Trigger

A governed reconciliation requirement is initiated for the identified accounting source and ledger scope.

---

# 9. Main Business Flow

1. Accounting establishes a Ledger Reconciliation Record and comparison scope.
2. The Accountant compares source facts with the corresponding ledger position.
3. If no unresolved difference exists, the comparison is prepared for closure.
4. If a difference exists, the record moves to `Exception` and the difference and supporting evidence are recorded.
5. The owning Finance capability resolves any required corrective business action outside the reconciliation aggregate.
6. The reconciliation returns to `Comparing` and the affected scope is compared again.
7. The Senior Accountant reviews the final evidence and closes the record as `Reconciled`.
8. `LedgerReconciled` is published and closure evidence is preserved.

---

# 10. Alternate Business Flows

## 10.1 Reconciliation Exception

`Comparing → Exception → Comparing` is used until differences are either resolved or authoritatively explained. The reconciliation does not claim success while an unresolved material difference remains.

---

# 11. Failure, Interruption and Business Recovery

An unresolved difference is a business reconciliation exception, not a technical failure. `ReconciliationExceptionRaised` records the exception fact; corrective action occurs through the relevant owning workflow before re-comparison.

---

# 12. Business Rules and Decision Constraints

- The reconciliation record never owns the source transaction or correction.
- Historical accounting records are not overwritten to force reconciliation.
- Institution-governed tolerance/materiality policies may influence review but are not invented by L3.
- Closure requires review by the Senior Accountant.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§2 and 6.

---

# 13. State and Lifecycle Impact

`Comparing → Reconciled`

`Comparing → Exception → Comparing`

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §6.

---

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Meaning |
|---|---|---|
| `ReconciliationExceptionRaised` | Accounting & General Ledger | An unresolved reconciliation difference has been formally recorded. |
| `LedgerReconciled` | Accounting & General Ledger | Reconciliation has been reviewed and closed successfully. |

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §6.

---

# 15. Business Authorization, Approval and Audit Obligations

Evidence must preserve source scope, ledger scope, comparison result, differences, corrective references, performer, reviewer and closure decision.

---

# 16. Completion Criteria and Postconditions

The reconciliation record is `Reconciled`; any differences are resolved or authoritatively accounted for; source and ledger ownership remain unchanged; closure evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

Institution-specific tolerance/materiality policy remains configurable and is not fixed by this scenario. No material ownership or lifecycle blocker remains.

---

**End of Scenario L3-F-029**