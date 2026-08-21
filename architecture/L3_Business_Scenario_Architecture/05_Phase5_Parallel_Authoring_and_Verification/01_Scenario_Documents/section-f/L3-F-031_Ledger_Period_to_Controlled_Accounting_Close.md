# Ledger Period to Controlled Accounting Close

# 1. Scenario Metadata

**Scenario ID:** L3-F-031  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounting & General Ledger  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs controlled closure of an Accounting Period after completion of required ledger activity, pending-Journal resolution, balance validation, reconciliation review, Trial Balance verification, and business approval. It prevents further posting into a closed period, preserves historical accounting integrity, and establishes an auditable locked-period outcome for reporting and institutional oversight.

---

# 3. Business Outcome

The ledger period has completed its governed accounting close, reached `Closed` and then `Locked`, rejected further posting or modification, and retained complete validation, approval, Trial Balance, reconciliation, event, and audit evidence.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an `Open` Accounting Period reaches its governed closing point and Accounting initiates the period-closing workflow for the applicable Ledger and Fiscal Year.

## 4.2 End Boundary

The scenario ends when all closing conditions are satisfied, the Accounting Period reaches `Closed` and then `Locked`, posting is disabled for that period, `AccountingPeriodClosed` and `AccountingPeriodLocked` represent the completed lifecycle facts, and the complete closing evidence is auditable.

## 4.3 In Scope

- Initiation of Accounting Period closing.
- Identification and resolution of pending Journals.
- Validation of unbalanced entries, ledger consistency, reconciliation status, and audit exceptions.
- Verification of the applicable Trial Balance.
- Review and approval of the period-closing decision.
- Transition of the Accounting Period from `Open` through `Closing` to `Closed` and `Locked`.
- Prevention of further posting or modification after closure and locking.
- Preservation of closing, approval, event, and audit evidence.

## 4.4 Out of Scope

- Creation, approval, posting, or reversal of individual Journal Entries except as separately governed prerequisites.
- Ledger reconciliation, which belongs to `L3-F-029`.
- Trial Balance generation, which belongs to `L3-F-030`.
- Reopening a closed Accounting Period.
- Fiscal Year closing, carry-forward, and opening-balance generation.
- Financial statement preparation, statutory reporting, or external publication.
- Archival of closed-ledger evidence, which belongs to `L3-F-032`.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-029` Accounting Source and Ledger to Reconciled Position | Provides reconciliation evidence relevant to closing readiness. |
| Preceded by | `L3-F-030` Accounting Period Activity to Trial Balance | Provides the verified Trial Balance required before closure. |
| Followed by | `L3-F-032` Closed Ledger Evidence to Auditable Financial History | Governs preservation and historical reconstruction of the closed-period evidence. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounting & General Ledger / Section F | Owns the Accounting Period lifecycle, closing validations, pending-Journal condition, Trial Balance verification requirement, ledger protection, closing decision, period events, and final controlled close. |
| Contributing Finance capabilities / Section F | Retain ownership of source-side exceptions or unresolved financial activity that must be governed before Accounting can complete the close. |

Accounting owns the period-closing outcome but cannot directly modify source-owned transactions merely to satisfy a closing condition. Each unresolved source matter remains with its canonical Finance owner until governed resolution.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Accountant | Prepares the period-closing population, identifies pending Journals and exceptions, and assembles closing evidence. | Performs closing preparation under the source-defined typical Accounting role. |
| Senior Accountant | Reviews Journal resolution, ledger validation, reconciliation status, Trial Balance verification, and closing evidence. | Reviews and supervises the closing preparation. |
| Finance Manager | Governs Accounting Period and Fiscal Year activities and oversees the closing decision. | Reviews or decides under the source-defined typical period-governance role. |
| Internal Auditor | Reviews closing history and evidence without modifying Accounting records. | Receives and examines the completed close and exceptions. |

`L2-F05` defines these as typical, institution-configurable roles. It requires approval in the closing workflow but does not assign every closing, locking, or exception-acceptance action to one immutable canonical role. This is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- The Accounting Period exists in `Open` and belongs to the applicable Fiscal Year and Accounting Ledger.
- The closing boundary and institutional scope are defined.
- Posted Journal activity for the period is available and traceable in the General Ledger.
- Pending Journals, unbalanced entries, reconciliation status, and audit exceptions can be identified.
- A Trial Balance Snapshot exists for the applicable period boundary and can be verified before closure.
- Any required correction must use its canonically owned adjustment, Journal, reversal, reconciliation, or source workflow rather than direct historical modification.

---

# 8. Trigger

Accounting initiates the governed closing process when an Open Accounting Period reaches its approved closing point.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Accounting Period closing scope, Ledger, Fiscal Year, institutional boundary, and closing evidence population. | Accountant / Accounting & General Ledger | A bounded and traceable period-close population is established. |
| 2 | Begin the governed period-closing workflow. | Accounting & General Ledger | The Accounting Period transitions from `Open` to `Closing`. |
| 3 | Identify all pending Journals and confirm that each is resolved through its governed posting, rejection, cancellation, correction, or other authoritative outcome. | Accountant / Accounting & General Ledger | No unresolved pending Journal remains as a closing blocker. |
| 4 | Validate the period for unbalanced entries, ledger inconsistency, unresolved reconciliation differences, and outstanding audit exceptions. | Accountant and Senior Accountant / Accounting & General Ledger | The period satisfies the defined accounting-completeness checks. |
| 5 | Verify the applicable Trial Balance and confirm total debit equals total credit for the closing boundary. | Senior Accountant / Accounting & General Ledger | The verified Trial Balance supports the closing decision. |
| 6 | Review the complete closing evidence and approve the Accounting Period for closure. | Finance Manager / Accounting & General Ledger | The period is authorized to move from closing review to the closed position. |
| 7 | Close the Accounting Period and preserve the closing decision and audit evidence. | Accounting & General Ledger | The Accounting Period reaches `Closed`, rejects new postings, and publishes `AccountingPeriodClosed`. |
| 8 | Lock the closed Accounting Period against further modification. | Accounting & General Ledger | The Accounting Period reaches `Locked`, historical integrity is protected, and `AccountingPeriodLocked` records the completed lock. |

The flow stops at the controlled close and lock. Reopening, Fiscal Year closing, reporting, and archival are separate outcomes.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow toward the same controlled close is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| One or more pending Journals remain unresolved. | `BR-033` prohibits period closure. | Resolve each Journal through its governed lifecycle, then repeat the closing validation. The Accounting Period remains outside `Closed`. | Accounting & General Ledger |
| An unbalanced entry or Trial Balance inequality is identified. | Financial integrity and the closing verification requirement are not satisfied. | Investigate through the governed reconciliation and correction workflows, generate or verify a valid Trial Balance, and repeat closing review. | Accounting & General Ledger |
| Reconciliation status is incomplete or a discrepancy remains unresolved. | The period cannot demonstrate a reconciled ledger position for closing. | Retain the closing interruption until the discrepancy is governed through `L3-F-029` or its owning correction workflow. | Accounting & General Ledger and affected Finance capability |
| An audit exception remains unresolved. | Closing evidence is incomplete or contested. | Investigate and resolve the exception under its canonical owner; do not approve the period close while the required condition remains outstanding. | Accounting & General Ledger and affected owner |
| Closing approval is absent or rejected. | The Accounting Period cannot transition to `Closed`. | Preserve the review outcome, resolve the stated business deficiency, and present the period for a new governed decision where permitted. | Accounting & General Ledger |
| A posting or modification is attempted after `Closed` or `Locked`. | The attempted activity violates period protection. | Reject the posting or modification. Any reopening requires a separate authorized workflow and is outside this scenario. | Accounting & General Ledger |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Closed Accounting Periods reject postings and Locked periods reject modifications. | Accounting & General Ledger | Closure and locking protect the historical ledger period from new activity or alteration. | `L2-F05`, Section 31 |
| Reopening requires authorization and Accounting Periods belong to one Fiscal Year. | Accounting & General Ledger | The close cannot be bypassed or reopened informally, and its fiscal relationship remains fixed. | `L2-F05`, Section 31 |
| Period closing validates pending Journals, unbalanced entries, reconciliation status, and audit exceptions. | Accounting & General Ledger | Each condition must be examined before the close can complete. | `L2-F05`, Section 35.6 |
| `BR-029`: Posting is permitted only within an Open Accounting Period. | Accounting & General Ledger | The move out of `Open` removes ordinary posting eligibility. | `L2-F05`, Section 36.4 |
| `BR-030`: Closed Accounting Periods reject postings. | Accounting & General Ledger | No Journal may be posted into the period after closure. | `L2-F05`, Section 36.4 |
| `BR-031`: Locked Accounting Periods reject modifications. | Accounting & General Ledger | Historical period records cannot be changed after locking. | `L2-F05`, Section 36.4 |
| `BR-032`: Period reopening requires authorized approval. | Accounting & General Ledger | A later reopening must use a separate governed decision. | `L2-F05`, Section 36.4 |
| `BR-033`: All pending Journals shall be resolved before closing. | Accounting & General Ledger | Any unresolved Journal blocks the transition to `Closed`. | `L2-F05`, Section 36.4 |
| `BR-034`: Trial Balance shall be verified before closure. | Accounting & General Ledger | A verified balanced Trial Balance is mandatory closing evidence. | `L2-F05`, Section 36.4 |
| `BR-035`: Period closing shall generate audit records. | Accounting & General Ledger | The complete closing decision and validation history must be preserved. | `L2-F05`, Section 36.4 |
| `BR-059`, `BR-060`, and `BR-064`: Financial operations generate audit events, audit history is retained, and historical Accounting records remain immutable. | Accounting & General Ledger | Closing and locking evidence must remain permanently reconstructable. | `L2-F05`, Section 36.9 |
| `BR-065`, `BR-068`, `BR-069`, and `BR-070`: Financial integrity takes precedence, operations are deterministic, results reproducible, and the Accounting Ledger remains the financial source of truth. | Accounting & General Ledger | The same validated period position must support a reproducible controlled close. | `L2-F05`, Section 36.10 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Accounting Period | `Open` | The governed closing workflow begins for the defined Ledger and Fiscal Year boundary. | `Closing` | Accounting & General Ledger | `L2-F05`, Sections 31 and 35.6 |
| Accounting Period | `Closing` | Pending Journals, balance conditions, reconciliation status, audit exceptions, Trial Balance, and approval are validated. | `Closed` | Accounting & General Ledger | `L2-F05`, Sections 31, 35.6 and 36.4 |
| Accounting Period | `Closed` | Posting is prohibited and the closed period is protected against further modification. | `Locked` | Accounting & General Ledger | `L2-F05`, Sections 31 and 36.4 |
| Trial Balance Snapshot | `Verified` or `Published` | The verified debit-credit position is used as closing evidence without changing the snapshot. | Unchanged | Accounting & General Ledger | `L2-F05`, Sections 33, 35.6 and 36.4 |
| Accounting Ledger and Ledger Accounts | Active with period-specific posting history | Closing and locking protect the completed period but do not rewrite ledger balances or historical postings. | Active ledger with the Accounting Period closed and locked | Accounting & General Ledger | `L2-F05`, Sections 23, 25, 31 and 35.6 |

The authoritative lifecycle also permits `Closed -> Reopened -> Open` and `Locked -> Archived`. Those transitions are outside this controlled-close boundary.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AccountingPeriodClosingStarted` | Accounting & General Ledger | Accounting & General Ledger; Audit & Compliance | Records entry into the governed closing lifecycle. | `L2-F05`, Sections 31 and 37.5 |
| `AccountingPeriodClosed` | Accounting & General Ledger | Financial Reporting, Notification Services, Audit & Compliance | Records successful completion of the period-close decision. | `L2-F05`, Sections 31, 37.5 and 37.8 |
| `AccountingPeriodLocked` | Accounting & General Ledger | Audit & Compliance; other consumers not authoritatively defined | Records that modification of the closed Accounting Period is prohibited. | `L2-F05`, Sections 31 and 37.5 |

Events represent completed lifecycle facts and do not authorize consumers to modify the Accounting Period or Ledger.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Accounting & General Ledger | Financial Reporting | Closed-period accounting position, verified Trial Balance reference, and `AccountingPeriodClosed` fact | Provides the authoritative period-close basis for reporting without transferring Accounting ownership. | `L2-F05`, Sections 35.6, 37.5 and 37.8 |
| Contributing Finance capability | Accounting & General Ledger | Governed resolution of a source-side closing exception | Confirms that an exception owned outside Accounting no longer blocks period closure. | Required by `L2-F05`, Sections 34.7 and 35.6; exact contracts vary by owning workflow. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Prepare the Accounting Period close | Accountant | May assemble and validate closing evidence under institution-configured Accounting authority. | Accounting & General Ledger |
| Review Journal, reconciliation, Trial Balance, and exception readiness | Senior Accountant | May perform supervisory review under the source-defined typical Accounting role. | Accounting & General Ledger |
| Approve the Accounting Period close | Finance Manager | Performs period governance and financial oversight under the source-defined typical role; exact institutional authority remains configurable. | Accounting & General Ledger |
| Close and lock the Accounting Period | Not assigned as a separate canonical role in `L2-F05` | The action requires completed validations and approval and must preserve period lifecycle and audit evidence. | Accounting & General Ledger |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Preparation and approval review | Accountant, Senior Accountant, and Finance Manager | Closing preparation and the final decision follow the institution's configured review and approval authority. |
| Journal resolution | Closing participants and Journal authorities | The closing workflow cannot bypass Journal validation, approval, posting, rejection, correction, or reversal rules. |
| Period reopening | Closing authority and authorized reopening authority | Reopening requires a separate explicit approval and is not an informal override of the completed close. |
| Audit independence | Internal Auditor | Audit users shall not modify Accounting Period, Journal, Trial Balance, or Ledger records. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve closing scope | Accounting Period, Ledger, Fiscal Year, institution, and closing boundary | Accounting & General Ledger |
| Preserve Journal readiness | Pending-Journal population and each governed resolution outcome | Accounting & General Ledger |
| Preserve financial validation | Unbalanced-entry checks, ledger consistency, reconciliation status, and audit-exception outcome | Accounting & General Ledger and affected source owner |
| Preserve Trial Balance evidence | Applicable verified Trial Balance Snapshot and debit-credit equality | Accounting & General Ledger |
| Preserve approval and lifecycle | Review, approval, `Closing`, `Closed`, and `Locked` lifecycle evidence | Accounting & General Ledger |
| Preserve completed events and history | `AccountingPeriodClosingStarted`, `AccountingPeriodClosed`, `AccountingPeriodLocked`, and complete audit lineage | Accounting & General Ledger |

Section A retains ownership of authorization and audit infrastructure; Accounting owns the period-closing decision, validations, evidence obligations, and financial records.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The closing scope identifies one Accounting Period, Ledger, Fiscal Year, institution, and closing boundary.
- All pending Journals are resolved through their governed lifecycles.
- No unresolved unbalanced entry, reconciliation discrepancy, or required audit exception remains.
- The applicable Trial Balance is verified and total debit equals total credit.
- The controlled close has received the required business review and approval.
- The Accounting Period reaches `Closed` and then `Locked`.
- The closing lifecycle, validation, approval, events, and audit evidence are reconstructable.

## 16.2 Postconditions

- New postings are rejected for the closed Accounting Period.
- Modifications are rejected for the locked Accounting Period.
- Historical Journals, Trial Balance, Ledger Account balances, and General Ledger history remain immutable.
- `AccountingPeriodClosed` is available to Financial Reporting and other defined consumers as a read-only Accounting fact.
- Any later reopening requires a separately authorized Accounting Period reopening workflow.
- The completed close is available for Fiscal Year governance, reporting, audit, and historical preservation.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The closing workflow ends with â€œClose Period -> Lock Posting,â€ while the Accounting Period lifecycle defines distinct `Closed` and `Locked` states and also permits `Closed -> Reopened`. | The source does not clarify whether locking must occur immediately as part of every close, whether a governed interval may exist in `Closed`, or whether reopening is permitted before locking only. | Accounting & General Ledger owner must define the business timing and decision boundary between `Closed`, `Locked`, and permitted reopening. |
| `L2-F05` requires approval in the period-closing workflow and defines typical configurable roles, but it does not explicitly assign final closure, locking, exception acceptance, or reopening authority. | Section 15 can use the typical Finance Manager governance role but cannot establish an immutable enterprise authority mapping. | Accounting & General Ledger owner must confirm the configuration-governed approval and execution authorities for closing, locking, exceptions, and reopening. |
| The closing checks name pending Journals, unbalanced entries, reconciliation status, and audit exceptions but do not define canonical completeness criteria, acceptable exception outcomes, materiality or tolerance rules, or required evidence for each check. | Sections 9, 11, 12, and 16 can require resolution but cannot determine when every closing condition is authoritatively satisfied. | Accounting & General Ledger owner must define or reference the governed closing checklist and exception-decision rules upstream. |
| `BR-034` requires a verified Trial Balance before closure, while the Trial Balance lifecycle separately defines `Verified` and `Published`. | It is unclear whether `Verified` alone is sufficient closing evidence or whether the snapshot must also be `Published`. | Accounting & General Ledger owner must define the minimum Trial Balance lifecycle position required for period closure. |
| `AccountingPeriodLocked` is canonical, but interested consumers beyond Audit & Compliance and its relationship to the General Ledger's period-specific protection are not fully defined. | Section 14 can publish the fact but cannot establish all downstream business obligations arising from the lock. | Accounting & General Ledger owner must define the canonical consumers and business scope of `AccountingPeriodLocked`. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-031**
