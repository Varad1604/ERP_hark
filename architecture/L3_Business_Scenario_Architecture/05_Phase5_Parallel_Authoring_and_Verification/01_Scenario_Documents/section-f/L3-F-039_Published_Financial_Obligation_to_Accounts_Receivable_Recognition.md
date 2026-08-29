# Published Financial Obligation to Accounts Receivable Recognition

# 1. Scenario Metadata

**Scenario ID:** L3-F-039  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounts Receivable  
**Participating Domains:** C, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F07_Accounts_Receivable_Management.md`; `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md`; applicable source-obligation L2 architecture  
**Referenced By:** L3-F-040; L3-F-041  

---

# 2. Scenario Purpose

Recognize a Finance-owned Receivable representation after an authoritative source domain has established a financial obligation, without transferring ownership of that obligation into Accounts Receivable.

---

# 3. Business Outcome

A validated Receivable exists with an immutable reference to the originating financial obligation, its opening receivable amount and classification are established, and the source obligation remains canonically owned by its source domain.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authoritative source-owned obligation-established business fact is available to Accounts Receivable.

## 4.2 End Boundary

The corresponding Receivable representation is registered and validated under AR ownership.

## 4.3 In Scope

Obligation-reference validation, Receivable registration, opening balance/classification establishment, source lineage and recognition evidence.

## 4.4 Out of Scope

Creating or changing the source obligation, collecting payment, accounting posting, write-off approval, or technical integration design.

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Accounts Receivable / F07 | Owns the Receivable representation, balance, classification and AR lifecycle. |
| Source obligation owner | Owns the originating obligation and publishes the obligation-established fact. |

For Student Charges, Student Charge Management remains the source owner.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Authority | Responsibility | Boundary |
|---|---|---|
| Receivable Registration Authority | Registers the Receivable after a valid source event is received. | Cannot originate the source obligation. |
| Receivable Validation Authority | Validates obligation reference, amount, classification, institution and AR integrity. | Cannot alter the source obligation. |
| Receivable Review Authority | Reviews recognition exceptions. | Corrective source action remains with the source owner. |

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §1.

---

# 7. Preconditions and Entry Criteria

- The originating obligation exists under its canonical owner.
- The source event/fact is authoritative and traceable.
- Amount, institution, classification and source reference are sufficient for AR validation.
- No duplicate Receivable already represents the same originating obligation.

---

# 8. Trigger

Accounts Receivable receives the canonical obligation-established fact from the source owner.

---

# 9. Main Business Flow

1. AR receives the source-owned obligation fact without assuming source ownership.
2. The Receivable Validation Authority verifies source reference, amount, institution and classification.
3. The Receivable Registration Authority creates the Receivable representation linked to exactly one originating obligation.
4. AR establishes the opening balance and relevant AR classification.
5. The Receivable passes aggregate integrity validation.
6. AR publishes `ReceivableCreated` and preserves source-to-receivable lineage.

---

# 10. Alternate Business Flows

## 10.1 Student Charge Source

For Student Charge recognition, the exact canonical trigger is `ChargeEstablished`, published by Student Charge Management.

## 10.2 Other Obligation Sources

For another obligation class, AR consumes only the exact obligation-established event defined by that source owner. Where the source L2 has not named that event, the event name remains unresolved and is not invented by this scenario.

---

# 11. Failure, Interruption and Business Recovery

Invalid, incomplete, duplicate or untraceable source evidence prevents Receivable recognition. The matter is reviewed and, where correction is needed, returned to the source-owning workflow. AR does not directly correct the source aggregate.

---

# 12. Business Rules and Decision Constraints

- AR never creates the originating obligation.
- Every Receivable references exactly one originating obligation.
- Recognition does not transfer source ownership.
- Registration and validation are independently governable responsibilities.
- Only source-authoritative event names may be used as recognition triggers.

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§2–4.

---

# 13. State and Lifecycle Impact

The Receivable lifecycle begins after valid source obligation establishment. This scenario establishes the initial AR representation; it does not redefine source-domain lifecycle states.

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§2–3 and unaffected canonical L2-F07 lifecycle definitions.

---

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Meaning |
|---|---|---|
| `ChargeEstablished` | Student Charge Management | Exact canonical Student-Charge obligation-establishment fact. |
| `ReceivableCreated` | Accounts Receivable | AR Receivable representation has been established. |

Other source-event names remain source-owned and must be cited only when canonically defined upstream.

**Authoritative source:** `L2-F07_Accounts_Receivable_Management_Amendment_2026-08-21.md` §§2–3 and canonical L2-F07 Receivable event definitions.

---

# 15. Business Authorization, Approval and Audit Obligations

Evidence must preserve source owner, source obligation reference, exact consumed event/fact, registration authority, validation authority, opening balance/classification and Receivable identity.

---

# 16. Completion Criteria and Postconditions

- one Receivable represents the source obligation;
- source ownership is unchanged;
- AR validation is complete;
- opening AR position is established;
- `ReceivableCreated` and lineage evidence are preserved.

---

# 17. Open Issues, Assumptions and Deferred Questions

The exact obligation-established event remains unresolved only for source classes whose authoritative L2 architecture has not named one. This is intentionally not filled by L3.

---

**End of Scenario L3-F-039**