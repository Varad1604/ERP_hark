# Manual Journal Request to Approved Posting or Rejection

# 1. Scenario Metadata

**Scenario ID:** L3-F-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md`; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md`  
**Governance References:** `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Govern an exceptional manual Journal request through independent accounting review to either a valid posted Journal or a conclusive rejection with no ledger effect.

---

# 3. Business Outcome

The manual Journal request reaches one governed accounting disposition: a balanced approved Journal is posted, or the Journal is `Rejected` with the decision preserved and no General Ledger consequence.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized business need for a manual accounting Journal has been established with supporting justification.

## 4.2 End Boundary

The Journal is either `Posted` or terminally `Rejected`.

## 4.3 In Scope

Preparation, validation, maker-checker review, approval/rejection, posting of approved Journals and decision evidence.

## 4.4 Out of Scope

Source-business correction, Financial Period lifecycle governance, later reversal, and technical accounting execution design.

---

# 5. Ownership and Participating Domains

Accounting & General Ledger owns the Journal and its disposition. F09 owns the governing Financial Period lifecycle. Any referenced source transaction remains owned by its source capability.

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Accountant | Prepares the manual Journal and supporting justification. | Maker role. |
| Senior Accountant | Independently reviews and approves or rejects. | Checker/decision role. |
| Authorized Accountant or Senior Accountant | Posts an approved Journal. | Existing posting authority only. |

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §2.

---

# 7. Preconditions and Entry Criteria

- A valid business reason and accounting references exist.
- Ledger Accounts and amount/currency context are available.
- The governing Financial Period is `Open` when posting is required.
- The requester is authorized to prepare the manual Journal.

---

# 8. Trigger

An authorized manual accounting requirement is submitted for Journal preparation.

---

# 9. Main Business Flow

1. The Accountant prepares the Journal with business justification and source references.
2. Accounting validates double-entry balance, account eligibility and period eligibility.
3. The Senior Accountant independently reviews the validated Journal.
4. If approved, the Journal progresses `Validated → Approved`.
5. An authorized posting actor posts the approved Journal and the ledger is updated.
6. If the reviewer declines the Journal, it progresses `Validated → Rejected` and no ledger consequence occurs.
7. The approval/rejection decision and evidence are preserved.

---

# 10. Alternate Business Flows

## 10.1 Rejection

A rejected Journal is terminal for that Journal instance. A later attempt is a new Journal Entry rather than resurrection of the rejected one.

---

# 11. Failure, Interruption and Business Recovery

Validation failure prevents the Journal from entering approval. A business interruption preserves the current governed Journal state and evidence. No technical retry, rollback or persistence design is defined at L3.

---

# 12. Business Rules and Decision Constraints

- Debit must equal credit.
- Maker-checker separation applies.
- Only approved Journals may be posted.
- `Rejected` is a terminal pre-posting outcome.
- A rejected Journal has no ledger effect.
- Posted Journals are immutable.
- Posting eligibility depends on the F09-owned Financial Period being `Open`.

**Authoritative sources:** `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §§2–3 and 9; `L2-F09_Financial_Period_and_Closing_Amendment_2026-08-21.md` §§1, 3 and 5.

---

# 13. State and Lifecycle Impact

`Draft → Validated → Approved → Posted`

`Validated → Rejected`

The `Rejected` state and `JournalRejected` event are canonical under the 2026-08-21 F05 amendment.

**Authoritative source:** `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §3.

---

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Meaning |
|---|---|---|
| `JournalValidated` | Accounting & General Ledger | Validation completed. |
| `JournalApproved` | Accounting & General Ledger | Journal approved for posting. |
| `JournalRejected` | Accounting & General Ledger | Journal conclusively rejected with no ledger effect. |
| `JournalPosted` | Accounting & General Ledger | Approved Journal posted. |

**Authoritative sources:** `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F05_Accounting_and_General_Ledger_Management_Amendment_2026-08-21.md` §3.

---

# 15. Business Authorization, Approval and Audit Obligations

The Accountant who prepares a Journal cannot satisfy the independent approval responsibility by that act alone. Evidence must preserve Journal identity, business reason, preparer, reviewer, approval/rejection, posting actor where applicable, and governing period.

---

# 16. Completion Criteria and Postconditions

For the posted path, the Journal is `Posted` and the ledger consequence is complete. For the rejection path, the Journal is `Rejected`, the decision is preserved, and the General Ledger is unchanged.

---

# 17. Open Issues, Assumptions and Deferred Questions

No material upstream blocker remains after the 2026-08-21 F05 locked amendment.

---

**End of Scenario L3-F-026**