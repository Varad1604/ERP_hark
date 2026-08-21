# Payable Change to Approved Adjustment or Reversal

# 1. Scenario Metadata
**Scenario ID:** L3-F-047  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Payable  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F04_Financial_Adjustment_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`; `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md`; `L3_Business_Scenario_Template_v2.0.md`

---
# 2. Scenario Purpose
Govern a change to an existing Payable through authoritative source correction, approved Financial Adjustment or reversal evidence rather than direct destructive modification.

---
# 3. Business Outcome
The Payable reflects the approved authoritative change while prior liability, procurement and accounting history remains traceable and separately owned.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
A valid business fact indicates that an existing Payable position requires change.
## 4.2 End Boundary
AP has incorporated the authoritative approved correction/adjustment/reversal fact and established the resulting Payable position.
## 4.3 In Scope
Source-reference validation, AP interpretation, liability update and evidence.
## 4.4 Out of Scope
Creating the source correction, Financial Adjustment approval, editing procurement facts or accounting history.

---
# 5. Ownership and Participating Domains
F08 owns the Payable representation. F04 owns Financial Adjustments. F14 owns procurement-finance liability-recognition facts. Section J owns procurement/supplier facts.

---
# 6. Business Actors, Roles and Responsibilities
AP reviews and incorporates authoritative completed Finance facts. Adjustment/reversal approval remains with the owning Finance capability and its approved authority; no AP role substitutes for that decision.

---
# 7. Preconditions and Entry Criteria
An existing Payable is traceable to the recognized liability; an authoritative approved correction/adjustment/reversal fact exists; amount and affected scope are determinable.

---
# 8. Trigger
AP receives an authoritative completed fact that changes the payable liability position.

---
# 9. Main Business Flow
1. AP identifies the affected Payable and source-liability lineage.
2. AP validates the authoritative change fact and approved scope.
3. AP records the correction/adjustment/reversal reference without taking source ownership.
4. AP updates the outstanding liability representation according to the authoritative fact.
5. AP reassesses payment readiness and due/ageing consequences where applicable.
6. Historical Payable and source evidence remains unchanged.
7. AP preserves the resulting liability position and audit lineage.

---
# 10. Alternate Business Flows
A fact that does not validly apply to the Payable is rejected from AP interpretation and returned to the owning business process for clarification.

---
# 11. Failure, Interruption and Business Recovery
An uncertain or incomplete source fact does not alter the Payable. Recovery requires an authoritative corrected business fact, not technical replay or direct mutation.

---
# 12. Business Rules and Decision Constraints
- Payable history is not destructively rewritten.
- F04 adjustments remain F04-owned.
- Procurement-finance liability facts remain F14-owned.
- Section J facts remain Section J-owned.
- AP changes only its Payable representation from authoritative completed facts.

---
# 13. State and Lifecycle Impact
AP recalculates the Payable liability/readiness position using F08 semantics; no source-domain lifecycle state is redefined.

---
# 14. Business Events and Cross-Domain Collaboration
Source events remain with their canonical publishers. AP publishes only established Payable lifecycle/update facts and does not treat F14 events as aliases for AP events.

---
# 15. Business Authorization, Approval and Audit Obligations
Evidence must identify original Payable, authoritative change fact, owning approval/correction reference, prior/resulting liability and AP review outcome.

---
# 16. Completion Criteria and Postconditions
The Payable reflects the approved authoritative change; historical lineage is intact; no ownership transferred; downstream AP activity uses the corrected current liability.

---
# 17. Open Issues, Assumptions and Deferred Questions
No material publisher/ownership blocker remains after the 2026-08-21 amendments.

---
**End of Scenario L3-F-047**