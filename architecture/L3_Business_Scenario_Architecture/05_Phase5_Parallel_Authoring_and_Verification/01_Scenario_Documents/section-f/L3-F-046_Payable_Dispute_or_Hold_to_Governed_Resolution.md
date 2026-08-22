# Payable Dispute or Hold to Governed Resolution

# 1. Scenario Metadata
**Scenario ID:** L3-F-046  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Payable  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F08_Accounts_Payable_Management.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`; `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J04_Purchase_Order_Management.md`; `L2-J05_Goods_Receipt_Inspection_and_Acceptance.md`

---
# 2. Scenario Purpose
Govern a Payable dispute or hold to a traceable AP resolution while preserving ownership of procurement evidence and any corrective source fact.

---
# 3. Business Outcome
The Payable dispute/hold is governed to a resolved AP position: payment readiness is restored, remains blocked/suspended, or a source-owned correction is required, with reasons and evidence preserved.

---
# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
A validated Payable has a material dispute, hold or exception affecting payment readiness.
## 4.2 End Boundary
AP has recorded the governed resolution and resulting payment-readiness/liability position.
## 4.3 In Scope
AP exception review, evidence assessment, hold/suspension governance, release or referral to the owning correction workflow.
## 4.4 Out of Scope
Changing procurement facts, executing Treasury settlement, or inventing source corrections.

---
# 5. Ownership and Participating Domains
F08 owns Payable state/readiness. Section J owns procurement and supplier evidence. F14 owns Finance invoice/liability-recognition facts. Corrective facts remain with their canonical owners.

---
# 6. Business Actors, Roles and Responsibilities
AP review/decision responsibilities operate under the authorities already defined by F08 and institutional policy. Section J participants may supply authoritative procurement evidence but cannot directly change the Payable.

---
# 7. Preconditions and Entry Criteria
A Payable exists; the dispute/hold reason is identifiable; relevant procurement/Finance evidence is available; no settlement should proceed while the blocking condition remains effective.

---
# 8. Trigger
A dispute, hold or material exception is recorded against the Payable.

---
# 9. Main Business Flow
1. AP records the dispute/hold and protects the Payable from inappropriate settlement progression.
2. AP identifies the governing reason and required evidence.
3. Section J/F14/source owners provide authoritative facts where the issue originates outside AP.
4. AP reviews the evidence without modifying those source facts.
5. If resolved in favor of payment readiness, AP removes the AP-owned hold/block under approved governance.
6. If unresolved, the Payable remains blocked/suspended according to F08 policy.
7. If correction is required, AP references the owning correction workflow and updates only after authoritative corrected facts are received.
8. Decision, evidence and resulting Payable position are preserved.

---
# 10. Alternate Business Flows
Resolution may release the Payable, maintain the hold, or require an externally owned correction. None transfers procurement/source ownership to AP.

---
# 11. Failure, Interruption and Business Recovery
Unresolved or contradictory evidence keeps the Payable in its prior safe governed position. No technical retry, message-recovery or rollback behavior is defined.

---
# 12. Business Rules and Decision Constraints
AP controls payment readiness and AP hold/suspension state only. Procurement facts remain Section J-owned; F14 liability-recognition facts remain F14-owned; Treasury must not settle a Payable that AP has not made eligible.

**Authoritative sources:** `L2-F08_Accounts_Payable_Management.md`; `L2-F08_Accounts_Payable_Management_Amendment_2026-08-21.md`; `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md`; applicable supplier, purchase-order and receipt evidence from `L2-J02`, `L2-J04`, and `L2-J05`.

---
# 13. State and Lifecycle Impact
Only F08-supported Payable readiness/suspension/hold semantics are used. No new state is invented.

**Authoritative source:** `L2-F08_Accounts_Payable_Management.md` as amended on 2026-08-21.

---
# 14. Business Events and Cross-Domain Collaboration
AP may publish its existing Payable update/lifecycle facts where defined by F08. This scenario does not rename or republish Section J/F14 events.

**Authoritative sources:** `L2-F08_Accounts_Payable_Management.md` as amended on 2026-08-21; `L2-F14_Procurement_Finance_Consumption_Workflows_Amendment_2026-08-21.md`.

---
# 15. Business Authorization, Approval and Audit Obligations
Evidence must preserve dispute/hold reason, source facts, reviewing authority, decision, prior/resulting Payable readiness and any correction reference.

---
# 16. Completion Criteria and Postconditions
A governed AP resolution exists; Payable readiness correctly reflects the evidence; external ownership is preserved; no unauthorized settlement occurred.

---
# 17. Open Issues, Assumptions and Deferred Questions
No material event-ownership blocker remains after the 2026-08-21 F08/F14 amendments.

---
**End of Scenario L3-F-046**