# Approved Write-Off Adjustment to Governed Financial Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-F-022  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Adjustment Management  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.2  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F04_Financial_Adjustment_Management.md` v1.1.1  
**Governance References:** `ADR-FIN-ADJ-001`; `ADR-FIN-ADJ-002`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-F-023; L3-F-024  

---

# 2. Scenario Purpose

Govern an approved write-off as a canonical Financial Adjustment that changes an existing financial position without rewriting the original transaction or accounting history.

---

# 3. Business Outcome

The approved Write-Off Adjustment is published and applied, the applicable financial position reflects the governed write-off outcome, and the original financial history remains immutable and traceable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A validated Write-Off Adjustment has received the required positive approval outcome and is `Approved`.

## 4.2 End Boundary

The Write-Off Adjustment is `Applied`, the governed financial-position effect is established, and required evidence is preserved.

## 4.3 In Scope

- publication readiness;
- publication and application of the approved adjustment;
- write-off authority compliance;
- immutable linkage to the affected financial position;
- business evidence and downstream Finance facts.

## 4.4 Out of Scope

- creation or approval of the underlying obligation;
- physical inventory/asset disposal;
- journal creation and ledger posting;
- later reversal, correction or archival.

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Adjustment Management / F04 | Owns the Financial Adjustment, its lifecycle, publication, application and write-off adjustment evidence. |
| Source Finance capability | Retains ownership of the underlying financial position or obligation. |

Participation never transfers source ownership to F04.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Enterprise Approval Authority | Provides the positive approval outcome before scenario entry. | Authority instance is determined by the applicable approved Finance rule. |
| Adjustment Publisher | Confirms publication readiness and progresses the approved adjustment through publication and application. | Cannot bypass validation or the required approval outcome. |

For a **Student-Charge write-off**, the mandatory approval authority is the **Board**, as resolved by `ADR-FIN-ADJ-002` and L2-F04 §30. Other write-off classes use the authority required by their canonical source rule.

---

# 7. Preconditions and Entry Criteria

- A canonical Financial Adjustment classified as Write-Off exists.
- Source, reason, affected financial-position reference and approved scope are established.
- Required validation is complete.
- The required approval outcome is positive.
- For Student-Charge write-off, Board approval evidence exists.
- The original financial history remains intact.

---

# 8. Trigger

The approved Write-Off Adjustment becomes eligible for publication and application.

---

# 9. Main Business Flow

1. Financial Adjustment Management confirms the approved adjustment, scope, references and applicable approval authority.
2. The Adjustment Publisher confirms publication readiness.
3. The adjustment progresses `Approved → Published` and becomes the authoritative published write-off fact.
4. The published adjustment progresses to `Pending Application`.
5. The approved write-off is applied to the referenced financial position without editing the original transaction.
6. The adjustment reaches `Applied` and the resulting financial-position fact is published for authorized Finance consumers.
7. Approval, publication, application and lineage evidence are preserved.

---

# 10. Alternate Business Flows

## 10.1 Non-Student-Charge Write-Off

Where the affected financial position is not a Student Charge, the applicable source-owned approval rule determines the required Enterprise Approval Authority. The Student-Charge Board rule is not generalized to unrelated write-offs.

---

# 11. Failure, Interruption and Business Recovery

- If publication readiness is not satisfied, publication does not occur and the governed F04 exception path applies.
- If application cannot validly complete, the financial position is not partially changed; the adjustment remains governed according to F04 recovery rules.
- No technical retry, rollback, persistence or event-delivery mechanism is defined at L3.

---

# 12. Business Rules and Decision Constraints

- Write-Off is represented through the canonical Financial Adjustment aggregate.
- Historical transactions and posted accounting history are not edited.
- Only validated and approved Financial Adjustments may be published.
- Student-Charge write-off requires Board approval.
- The Board requirement is scoped to Student-Charge write-off unless another authoritative source explicitly requires it.
- Financial Adjustment Management owns the adjustment, not the originating obligation.
- Accounting consequences remain owned by Accounting & General Ledger.

---

# 13. State and Lifecycle Impact

| Business Concept | From | To | Owner |
|---|---|---|---|
| Financial Adjustment | Approved | Published | F04 |
| Financial Adjustment | Published | Pending Application | F04 |
| Financial Adjustment | Pending Application | Applied | F04 |

The source financial position reflects only the approved business consequence; its canonical ownership does not move to F04.

---

# 14. Business Events and Cross-Domain Collaboration

| Business Event | Publisher | Meaning |
|---|---|---|
| `AdjustmentPublicationCompleted` | Financial Adjustment Management | The approved write-off adjustment is published. |
| `AdjustmentApplicationCompleted` | Financial Adjustment Management | The write-off adjustment has been applied. |
| `AdjustmentFinancialPositionUpdated` | Financial Adjustment Management | The governed financial position reflects the approved adjustment. |

Consumers may react to these facts but may not modify the Financial Adjustment aggregate.

---

# 15. Business Authorization, Approval and Audit Obligations

- Approval evidence must identify the approved adjustment, scope, authority and decision.
- For Student-Charge write-off, Board approval evidence is mandatory.
- The Adjustment Publisher may not substitute for the approval authority.
- The audit trail must preserve source reference, reason, amount/scope, approval, publication, application and resulting state.
- The original financial transaction remains reconstructable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- the adjustment is validly approved under the applicable authority rule;
- the adjustment is published and applied;
- the Financial Adjustment is `Applied`;
- the financial-position consequence is established;
- original history is unchanged and traceable;
- required business events and evidence exist.

## 16.2 Postconditions

The governed write-off outcome is authoritative for subsequent Finance processing. Any later reversal or correction is handled through its separate canonical scenario.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

None material to this scenario. The former Student-Charge Board-authority ambiguity was resolved by `ADR-FIN-ADJ-002` on 2026-08-21.

## 17.2 Assumptions

No assumptions beyond the authoritative L2 Finance architecture.

## 17.3 Deferred Questions

None.

---

**End of Scenario L3-F-022**