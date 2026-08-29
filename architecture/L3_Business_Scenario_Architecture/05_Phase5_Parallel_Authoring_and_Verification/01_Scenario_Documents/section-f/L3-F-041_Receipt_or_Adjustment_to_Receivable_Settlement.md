# Receipt or Adjustment to Receivable Settlement

# 1. Scenario Metadata

**Scenario ID:** L3-F-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F02_Payment_Management_Architecture.md`; `L2-F04_Financial_Adjustment_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Incorporate authoritative settlement or adjustment facts into an existing Receivable while preserving separate ownership of Payment, Financial Adjustment, the originating obligation and Accounts Receivable.

---

# 3. Business Outcome

The Receivable reflects the authoritative partial or full settlement position, settlement references and remaining outstanding balance, with complete lineage to the source Finance facts.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Accounts Receivable receives an authoritative completed payment-allocation, settlement or approved Financial Adjustment fact affecting an existing Receivable.

## 4.2 End Boundary

The Receivable representation incorporates the source fact and accurately reflects the remaining outstanding position; when fully satisfied, the Receivable reaches its fully-settled outcome.

## 4.3 In Scope

Settlement/adjustment reference validation, Receivable balance update, settlement reconciliation and AR evidence.

## 4.4 Out of Scope

Payment execution, Financial Adjustment approval/application, source-obligation mutation, Treasury settlement or accounting posting.

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Accounts Receivable / F07 | Owns Receivable balance, settlement references and AR settlement position. |
| Payment / Financial Adjustment / other source Finance capability | Owns the completed source financial fact. |
| Source obligation owner | Continues to own the originating obligation. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Authority | Responsibility | Boundary |
|---|---|---|
| Settlement Reconciliation Authority | Confirms authoritative settlement facts are correctly incorporated into AR. | Does not execute payment or Treasury settlement. |
| Receivable Review Authority | Reviews settlement/adjustment interpretation exceptions. | Cannot alter source-owned facts. |

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §1.

---

# 7. Preconditions and Entry Criteria

- The Receivable exists and is traceable to its source obligation.
- An authoritative completed settlement/payment-allocation or adjustment fact exists.
- Source reference, amount and applicable Receivable relationship are identifiable.

---

# 8. Trigger

AR receives an authoritative Finance fact that changes the outstanding Receivable position.

---

# 9. Main Business Flow

1. AR identifies the affected Receivable and validates the incoming source reference.
2. The Settlement Reconciliation Authority confirms the source amount and relationship to the Receivable.
3. AR records the settlement/adjustment reference without taking ownership of the source fact.
4. AR recalculates the remaining receivable position according to the authoritative source fact.
5. If an amount remains outstanding, AR preserves the updated active Receivable position.
6. If the obligation is fully satisfied, AR records the fully settled outcome.
7. AR publishes the applicable settlement/receivable fact and preserves end-to-end lineage.

---

# 10. Alternate Business Flows

## 10.1 Partial Settlement

The Receivable remains outstanding for the authoritative residual amount.

## 10.2 Full Settlement

The Receivable becomes fully settled and eligible for subsequent governed closure where applicable.

---

# 11. Failure, Interruption and Business Recovery

A source-reference, amount or allocation inconsistency prevents AR from applying an uncertain settlement position. The Receivable Review Authority reviews the exception; correction remains with the source-owning Finance capability. No technical retry or rollback is defined.

---

# 12. Business Rules and Decision Constraints

- AR updates only from authoritative completed Finance facts.
- Payment, Adjustment and Treasury ownership do not transfer to AR.
- Settlement references remain traceable and immutable as historical evidence.
- AR must not independently infer a settlement that the source owner has not established.

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§3–4; source facts remain governed by their owning `L2-F02` / `L2-F04` architecture.

---

# 13. State and Lifecycle Impact

The Receivable balance/settlement representation is updated. A fully satisfied Receivable may reach the existing fully-settled outcome; no new AR state is invented.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`, constrained by the 2026-08-21 amendment §§3–4.

---

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Meaning |
|---|---|---|
| `ReceivableSettlementRecorded` | Accounts Receivable | Authoritative settlement information has been incorporated into AR. |
| `ReceivableFullySettled` | Accounts Receivable | The Receivable has been completely satisfied. |

Source Finance events remain published by their canonical owners.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`; ownership separation is reinforced by the 2026-08-21 amendment §§3–4.

---

# 15. Business Authorization, Approval and Audit Obligations

Evidence must preserve Receivable identity, source fact/event, amount applied, prior and resulting outstanding amount, settlement reference, reconciliation authority and any exception decision.

---

# 16. Completion Criteria and Postconditions

The Receivable accurately reflects the authoritative settlement/adjustment fact; source ownership remains unchanged; the remaining balance is correct; and AR settlement evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

No material authority or ownership blocker remains after the 2026-08-21 F07 amendment.

---

**End of Scenario L3-F-041**