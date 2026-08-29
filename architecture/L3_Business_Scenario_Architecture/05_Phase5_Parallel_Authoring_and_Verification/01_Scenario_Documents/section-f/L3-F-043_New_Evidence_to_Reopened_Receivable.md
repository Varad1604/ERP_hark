# New Evidence to Reopened Receivable

# 1. Scenario Metadata

**Scenario ID:** L3-F-043  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`; applicable authoritative Finance source architecture  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Reopen a previously resolved Receivable only when new authoritative Finance evidence establishes that an outstanding receivable position exists again, while preserving all prior resolution history.

---

# 3. Business Outcome

The same Receivable is reopened under AR ownership with a validated current outstanding position and immutable linkage to the authoritative Finance fact that justified reopening.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A previously resolved Receivable exists and AR receives new authoritative Finance evidence that may justify reopening.

## 4.2 End Boundary

The Receivable is validly reopened, its current outstanding position is established, and `ReceivableReopened` records the completed AR fact.

## 4.3 In Scope

Evidence validation, reopening review, AR state change, current balance interpretation and preservation of prior history.

## 4.4 Out of Scope

Creating or correcting the source Finance fact, rewriting prior settlement/write-off history, or technical reprocessing.

---

# 5. Ownership and Participating Domains

Accounts Receivable owns reopening of the Receivable representation. Payment, Refund, Financial Adjustment, Accounting or another Finance capability retains ownership of the authoritative fact that causes the reopening assessment.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Authority | Responsibility | Boundary |
|---|---|---|
| Reopening Review Authority | Confirms the authoritative Finance fact justifies reopening and that the resulting AR position is valid. | Cannot create or alter the source fact. |
| Receivable Validation Authority | Validates current balance, references and AR integrity. | AR validation only. |

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §1.

---

# 7. Preconditions and Entry Criteria

- The Receivable is previously resolved/closed under AR governance.
- New authoritative Finance evidence exists and is traceable.
- The new evidence has a determinable effect on the Receivable position.
- Prior resolution history remains available.

---

# 8. Trigger

AR receives an authoritative Finance fact indicating that the prior resolved receivable position is no longer complete or current.

---

# 9. Main Business Flow

1. AR identifies the previously resolved Receivable and the new source fact.
2. The Receivable Validation Authority validates the source reference and resulting outstanding position.
3. The Reopening Review Authority confirms that the evidence justifies reopening.
4. AR reopens the same Receivable rather than creating an unrelated replacement.
5. The current outstanding balance and collection context are established from authoritative Finance evidence.
6. Prior settlement, adjustment, write-off and closure history remains unchanged.
7. AR publishes `ReceivableReopened` and preserves complete reopening evidence.

---

# 10. Alternate Business Flows

If the evidence does not justify an outstanding position, the Receivable remains resolved and the review decision is preserved without reopening.

---

# 11. Failure, Interruption and Business Recovery

Uncertain, incomplete or contradictory source evidence prevents reopening. AR retains the prior valid resolved position until the source-owning Finance workflow establishes an authoritative fact. No technical retry/reload mechanism is defined.

---

# 12. Business Rules and Decision Constraints

- Reopening requires an authoritative Finance fact.
- Reopening does not edit prior resolution history.
- AR does not directly modify source Finance aggregates.
- The same Receivable identity and source-obligation lineage remain traceable.

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§1, 3 and 4.

---

# 13. State and Lifecycle Impact

The previously resolved Receivable returns to an active receivable position according to the existing F07 reopening semantics. L3 does not invent an intermediate state.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`, constrained by the reopening authority and ownership rules in the 2026-08-21 amendment §§1, 3 and 4.

---

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Meaning |
|---|---|---|
| `ReceivableReopened` | Accounts Receivable | Authoritative new Finance evidence has caused the Receivable to be validly reopened. |

The triggering source fact remains owned and published by its canonical Finance source.

**Authoritative source:** canonical `L2-F07_Accounts_Receivable_Management.md`; source ownership and event-contract rules are reinforced by the 2026-08-21 amendment §§2–5.

---

# 15. Business Authorization, Approval and Audit Obligations

Evidence must preserve prior resolution state/evidence, new source fact, validation outcome, reopening authority, resulting outstanding position and the `ReceivableReopened` fact.

---

# 16. Completion Criteria and Postconditions

The same Receivable is reopened on authoritative evidence, its current outstanding position is valid, prior history remains immutable, and source ownership is unchanged.

---

# 17. Open Issues, Assumptions and Deferred Questions

No material AR authority blocker remains after the 2026-08-21 F07 amendment.

---

**End of Scenario L3-F-043**