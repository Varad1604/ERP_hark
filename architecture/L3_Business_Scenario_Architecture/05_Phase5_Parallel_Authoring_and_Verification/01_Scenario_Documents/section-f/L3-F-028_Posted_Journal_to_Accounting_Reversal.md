# Posted Journal to Accounting Reversal

# 1. Scenario Metadata

**Scenario ID:** L3-F-028  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Correct a posted Journal through an authorized compensating Journal while preserving the original accounting history.

---

# 3. Business Outcome

The approved reversal amount is represented by a new posted compensating Journal linked to the original, with the General Ledger corrected and historical Journal lines unchanged.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A `Posted` Journal requires full or partial reversal under approved accounting governance.

## 4.2 End Boundary

The compensating Journal is `Posted`, the ledger reflects the approved reversal amount, and original-to-reversal traceability is preserved.

## 4.3 In Scope

Reversal authorization, compensating Journal creation, maker-checker approval, posting and reversal evidence.

## 4.4 Out of Scope

Editing the original Journal, source-domain correction, Financial Period lifecycle ownership and technical rollback mechanisms.

---

# 5. Ownership and Participating Domains

Accounting & General Ledger owns both the original Journal and compensating Journal. F09 owns posting-period lifecycle governance.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Finance Manager | Authorizes a partial reversal and applicable governed reversal scope. | Required before compensating Journal creation for partial reversal. |
| Accountant | Prepares the compensating Journal. | Maker role. |
| Senior Accountant | Reviews and approves/rejects the compensating Journal. | Checker role. |
| Authorized posting actor | Posts the approved compensating Journal. | Existing posting authority. |

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §2.

---

# 7. Preconditions and Entry Criteria

- The original Journal is `Posted` and immutable.
- Reversal business basis and scope are established.
- Required reversal authorization exists.
- The governing Financial Period is eligible for the compensating posting.

---

# 8. Trigger

An authorized accounting reversal requirement is established for a posted Journal.

---

# 9. Main Business Flow

1. Accounting identifies the original posted Journal and required reversal scope.
2. Where the reversal is partial, Finance Manager authorization is recorded before the compensating Journal is created.
3. The Accountant creates a new compensating Journal referencing the original.
4. The compensating Journal follows normal validation and maker-checker approval.
5. An authorized posting actor posts the compensating Journal.
6. The ledger reflects the compensating effect while original Journal lines remain unchanged.
7. Full/partial reversal evidence and lineage are preserved.

---

# 10. Alternate Business Flows

## 10.1 Full Reversal

After the compensating Journal is successfully posted, the original may be represented as `Reversed` because its entire financial effect has been neutralized.

## 10.2 Partial Reversal

The original remains `Posted` because part of its financial effect remains authoritative. No `Partially Reversed` state is introduced.

---

# 11. Failure, Interruption and Business Recovery

If authorization, validation, approval or posting cannot complete, no partial reversal effect is recognized. The original posted Journal remains unchanged. L3 defines no technical rollback or retry mechanism.

---

# 12. Business Rules and Decision Constraints

- Posted Journals are immutable.
- Every reversal uses a new compensating Journal referencing the original.
- Partial reversal requires Finance Manager authorization.
- The compensating Journal follows the ordinary maker-checker and posting lifecycle.
- Partial reversal does not move the original to a new partial-reversal state.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§2, 3 and 5; period eligibility remains governed by F09 through F05 amendment §8.

---

# 13. State and Lifecycle Impact

Compensating Journal: `Draft → Validated → Approved → Posted`.

Original Journal:
- full reversal: `Posted → Reversed` after compensating posting;
- partial reversal: remains `Posted`.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§3 and 5.

---

# 14. Business Events and Cross-Domain Collaboration

`JournalPosted` records the compensating Journal posting. `JournalReversed` may record the full-reversal outcome for the original when its entire effect has been neutralized. Accounting retains ownership of both facts.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§3 and 5 and unaffected canonical L2-F05 event definitions.

---

# 15. Business Authorization, Approval and Audit Obligations

Evidence must identify original Journal, reversal reason, approved scope, reversal authority, compensating Journal, preparer, reviewer, posting actor and resulting ledger effect.

---

# 16. Completion Criteria and Postconditions

The compensating Journal is posted, the approved reversal effect is reflected in the ledger, and immutable original-to-reversal lineage is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

No material upstream blocker remains after the 2026-08-21 F05 locked amendment.

---

**End of Scenario L3-F-028**