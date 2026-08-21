# Posted Journal to Accounting Reversal

# 1. Scenario Metadata

**Scenario ID:** L3-F-028  
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

This scenario governs correction of a posted Journal Entry through an authorized compensating Journal rather than modification or deletion of historical accounting records. It preserves the original posting, reverses its required financial effect, updates the General Ledger consistently, and maintains complete original-to-reversal traceability and audit evidence.

---

# 3. Business Outcome

The posted Journal has received a governed accounting reversal through a separately traceable compensating Journal, with the original Journal preserved unchanged, the authorized reversing ledger consequence completed, and the full accounting history remaining auditable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an existing `Posted` Journal Entry is determined to require an accounting reversal and a reversal request is presented for governed authorization.

## 4.2 End Boundary

The scenario ends when the authorized compensating Journal is posted, all affected Ledger Accounts and the General Ledger reflect the reversing consequence, the original Journal remains immutable and linked to the reversal, and the original Journal reaches its authoritative reversal lifecycle position where applicable.

## 4.3 In Scope

- Identification of the original posted Journal and the business reason for reversal.
- Determination of the supported reversal treatment.
- Reversal authorization and maker-checker protection.
- Creation, validation, approval, and posting of the compensating Journal Entry.
- Update of affected Ledger Accounts and the General Ledger.
- Preservation of the original Journal, reversal reference, financial consequence, events, and audit history.
- Full, partial, correction-entry, accrual, and closing reversal forms to the extent defined by authoritative L2 architecture.

## 4.4 Out of Scope

- Editing, deleting, overwriting, or reopening the original posted Journal.
- Reversal of a Financial Adjustment, which belongs to `L3-F-023`.
- Correction of a Financial Adjustment through a successor record, which belongs to `L3-F-024`.
- Reversal or chargeback of operational Payments, Refunds, or other Finance-owned transactions.
- Reopening or unlocking an Accounting Period.
- Definition of reversal accounting policy, Ledger Accounts, or authority configuration.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-025` Financial Transaction to Balanced Ledger Posting | May produce the posted Journal that later requires reversal. |
| Preceded by | `L3-F-026` Manual Journal Request to Approved Posting or Rejection | May produce a posted Manual Journal that later requires reversal. |
| Related | `L3-F-023` Posted Adjustment to Reversal | Governs Financial Adjustment reversal under a different canonical owner and lifecycle. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounting & General Ledger / Section F | Owns the original and compensating Journal Entries, reversal authorization requirement, Journal lifecycle, accounting validation, posting, Ledger Account and General Ledger consequences, accounting events, and complete reversal audit history. |

The originating operational transaction and any operational correction remain owned by their canonical Finance capability. This Accounting reversal changes only the recorded accounting consequence and does not reverse, cancel, or re-own the source transaction.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Accountant | Identifies and prepares the accounting reversal and compensating Journal with reference to the original posted Journal. | Performs reversal preparation under the source-defined typical Accounting role. |
| Senior Accountant | Reviews the reversal basis and compensating Journal under maker-checker separation. | Performs review and approval under the source-defined typical Accounting role. |
| Internal Auditor | Examines reversal lineage and evidence without modifying accounting records. | Receives and reviews the auditable reversal outcome. |

`L2-F05` defines these as typical, institution-configurable roles. It requires authorization for reversal but does not assign final reversal execution or partial-reversal authorization to one canonical business role. The authority gap is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- The original Journal Entry exists in `Posted` and is immutable.
- The original Journal, source business-transaction reference, posting reference, Journal lines, Ledger Accounts, Accounting Period, Fiscal Year, currency, and audit history are traceable.
- A governed business reason exists for reversal.
- The required reversal authorization can be obtained before a compensating financial consequence is posted.
- The Accounting Period selected for the compensating Journal is `Open`, the Fiscal Year is active, and every affected Ledger Account is active and posting-eligible.
- The proposed compensating Journal can preserve double-entry balance and reference the original Journal.

---

# 8. Trigger

A governed accounting correction requirement initiates a request to reverse the financial effect of an existing posted Journal without modifying the original record.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the original `Posted` Journal, its source reference, posting history, affected Ledger Accounts, and the business reason requiring reversal. | Accountant / Accounting & General Ledger | The reversal request is anchored to one immutable and traceable original Journal. |
| 2 | Determine the applicable reversal treatment and confirm that reversal, rather than historical editing, is required. | Accounting & General Ledger | The required compensating accounting treatment is established under the supported reversal forms. |
| 3 | Obtain the governed business authorization for the reversal under applicable separation of duties. | Senior Accountant / Accounting & General Ledger | The reversal is authorized to proceed; unauthorized reversal is prohibited. |
| 4 | Establish a new compensating Journal Entry that references the original Journal and represents the required opposite accounting consequence. | Accountant / Accounting & General Ledger | A separately traceable `Draft` compensating Journal exists while the original Journal remains unchanged. |
| 5 | Validate the compensating Journal for double-entry balance, active Ledger Accounts, Open Accounting Period, active Fiscal Year, currency, mandatory references, chronology, and duplicate protection. | Accounting & General Ledger | The compensating Journal reaches `Validated`; any failed validation prevents reversal posting. |
| 6 | Independently approve the validated compensating Journal for posting. | Senior Accountant / Accounting & General Ledger | The compensating Journal reaches `Approved` and becomes posting-eligible. |
| 7 | Post the complete balanced compensating Journal and update every affected Ledger Account and the General Ledger. | Accounting & General Ledger | The compensating Journal reaches `Posted`, and the authorized reversing financial consequence is recognized without changing the original Journal. |
| 8 | Complete the original Journal's reversal lifecycle impact and preserve the original-to-reversal relationship, authorization, posting consequence, events, and audit evidence. | Accounting & General Ledger | The original Journal reaches `Reversed` for the authoritative full-reversal path; `JournalReversed`, the compensating `JournalPosted`, and `LedgerUpdated` record the completed accounting facts. |

The main path represents a full reversal because that path maps unambiguously to the defined `Posted -> Reversed` transition. Partial-reversal lifecycle ambiguity is governed in Section 10 and recorded in Section 17.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Partial reversal | Only part of the original Journal's financial effect requires authorized reversal. | Establish and post a balanced compensating Journal for the authorized partial amount while retaining full reference to the original posted Journal. | The authorized portion is reversed and the financial history remains traceable; the resulting lifecycle state of the original Journal is undefined upstream. | Accounting & General Ledger |
| Accrual reversal | A posted accrual requires its governed reversing entry. | Establish, authorize, validate, approve, and post the compensating accrual Journal with reference to the original. | The accrual's authorized reversing consequence is recorded without modifying the original Journal. | Accounting & General Ledger |
| Closing reversal | A posted closing entry requires an authorized compensating treatment. | Establish and post the governed closing-reversal Journal in an eligible Accounting Period. | The closing-entry consequence is reversed through a separate traceable Journal. | Accounting & General Ledger |
| Correction entry | The accounting correction is represented by a linked compensating correction Journal. | Establish, validate, approve, and post the correction Journal with reference to the original posted Journal. | The corrected accounting position is represented through immutable linked history. | Accounting & General Ledger |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The original Journal or its accounting lineage cannot be established. | A reversal cannot be safely related to the correct historical posting. | Stop progression, preserve the original Journal unchanged, and resolve the lineage deficiency before presenting a new reversal request. | Accounting & General Ledger |
| Reversal authorization is absent, invalid, or conflicts with separation of duties. | The compensating Journal cannot become eligible for posting. | Reject or withhold the reversal request until valid independent authorization exists; no ledger change occurs. | Accounting & General Ledger |
| The compensating Journal is unbalanced or references an invalid or ineligible Ledger Account. | Accounting validation fails. | Correct the unposted compensating Journal and repeat validation; the original Journal and ledger history remain unchanged. | Accounting & General Ledger |
| The selected Accounting Period is closed or locked, or the Fiscal Year is inactive. | Reversal posting is prohibited in that period. | Do not post the compensating Journal. Any period reopening or fiscal-governance action belongs to its separately governed workflow. | Accounting & General Ledger |
| The complete balanced compensating posting cannot be achieved. | A partial or inconsistent reversing ledger consequence would result. | Do not recognize the reversal as completed; retain the original Journal in its prior lifecycle position and preserve unchanged ledger balances. | Accounting & General Ledger |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| `BR-001` and `BR-002`: Every Journal Entry maintains double-entry accounting and total debit equals total credit. | Accounting & General Ledger | The compensating Journal must be balanced before posting. | `L2-F05`, Section 36.1 |
| `BR-006`: Every Journal Entry shall reference a valid business transaction. | Accounting & General Ledger | The compensating Journal retains the business and accounting lineage of the original posting. | `L2-F05`, Section 36.1 |
| `BR-007` and `BR-008`: Posted Journals are immutable and Journals are never physically deleted. | Accounting & General Ledger | Reversal cannot edit, overwrite, or delete the original posted Journal. | `L2-F05`, Section 36.1 |
| `BR-009`: Journal reversal shall create compensating Journal Entries. | Accounting & General Ledger | The reversing financial effect must be represented by a separate Journal. | `L2-F05`, Section 36.1 |
| `BR-010`: Every Journal Entry shall generate an audit record. | Accounting & General Ledger | Both original and compensating Journals remain auditable. | `L2-F05`, Section 36.1 |
| `BR-011` and `BR-012`: Ledger balances change only through successful Journal posting; manual balance modification is prohibited. | Accounting & General Ledger | The reversal affects balances only through successful posting of the compensating Journal. | `L2-F05`, Section 36.2 |
| `BR-029` and `BR-030`: Posting is permitted only in an Open Accounting Period; closed periods reject posting. | Accounting & General Ledger | The compensating Journal requires an eligible posting period. | `L2-F05`, Section 36.4 |
| `BR-041` through `BR-046`: Posting requires approved and validated Journals, updates all affected accounts and the General Ledger, preserves chronology, and completely withdraws failed posting effects. | Accounting & General Ledger | The compensating Journal must complete the standard governed posting path. | `L2-F05`, Section 36.6 |
| `BR-049` through `BR-052`: Historical transactions are never edited; corrections use reversal Journals; every reversal references the original Journal and preserves complete financial history. | Accounting & General Ledger | These rules define the central reversal method and evidence obligations. | `L2-F05`, Section 36.7 |
| `BR-053`: Partial reversals require financial authorization. | Accounting & General Ledger | A partial reversal cannot proceed without the required business authority. | `L2-F05`, Section 36.7 |
| `BR-055` through `BR-057`: Posting follows governed role authority, maker and checker differ where applicable, and overrides are fully audited. | Accounting & General Ledger | Authorization and separation controls apply to the compensating Journal and reversal. | `L2-F05`, Section 36.8 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Original Journal Entry | `Posted` | A completed full reversal is related to the original without changing its historical content. | `Reversed` | Accounting & General Ledger | `L2-F05`, Sections 28.5-28.6 and 35.4 |
| Compensating Journal Entry | New authorized reversal requirement | A separate Journal is established with mandatory reference to the original Journal. | `Draft` | Accounting & General Ledger | `L2-F05`, Sections 28.5-28.6, 34.4 and 36.7 |
| Compensating Journal Entry | `Draft` | Accounting, balance, account, period, fiscal, currency, reference, and duplicate conditions are validated. | `Validated` | Accounting & General Ledger | `L2-F05`, Sections 28.4-28.6 and 34.2 |
| Compensating Journal Entry | `Validated` | Independent review produces a positive posting approval. | `Approved` | Accounting & General Ledger | `L2-F05`, Sections 28.6 and 36.6 |
| Compensating Journal Entry | `Approved` | The complete balanced compensating entry is posted. | `Posted` and immutable | Accounting & General Ledger | `L2-F05`, Sections 28.5-28.6, 34.1 and 36.6 |
| Accounting Ledger and affected Ledger Accounts | Active and posting-eligible | Successful compensating-Journal posting updates balances and posting history while retaining the original posting. | Active ledger with updated mathematically consistent balances | Accounting & General Ledger | `L2-F05`, Sections 23, 25, 34.3 and 35.4 |

For partial reversal, authoritative L2 defines the supported treatment and authorization requirement but does not state whether the original Journal remains `Posted`, reaches `Reversed`, or requires another lifecycle position. No state is invented in this Draft.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `JournalPosted` | Accounting & General Ledger | Procurement Finance, Financial Analytics, Audit & Compliance | Records successful posting of the compensating Journal. | `L2-F05`, Sections 28.8, 37.2 and 37.8 |
| `JournalReversed` | Accounting & General Ledger | Audit & Compliance | Records completion of the original Journal's governed reversal. | `L2-F05`, Sections 28.8, 37.2 and 37.8 |
| `LedgerUpdated` | Accounting & General Ledger | Budget Management, Treasury Management, Financial Analytics, Audit & Compliance | Records the General Ledger consequence of the successful compensating posting. | `L2-F05`, Sections 25.8, 37.3 and 37.8 |

Only completed business facts are published. A rejected, invalid, or incomplete reversal request does not publish `JournalReversed`, `JournalPosted`, or `LedgerUpdated`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain synchronous contract or hand-off materially participates in the frozen Section F boundary. The original-to-compensating-Journal reference is an Accounting-owned aggregate relationship, not a transfer of ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Prepare the reversal request and compensating Journal | Accountant | May prepare the reversal under institution-configured Accounting authority while preserving the original Journal. | Accounting & General Ledger |
| Review and approve the compensating Journal | Senior Accountant | May independently review the reversal basis and approve the compensating Journal under maker-checker separation. | Accounting & General Ledger |
| Authorize a partial reversal | Not assigned to one canonical business role in `L2-F05` | Explicit financial authorization is mandatory under `BR-053`. | Accounting & General Ledger |
| Execute the authorized reversal and post the compensating Journal | Not assigned to one canonical business role in `L2-F05` | Reversal and posting require governed authority, successful validation, approval, and an eligible accounting period. | Accounting & General Ledger |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Maker-checker separation | Journal Creator and Journal Approver | The creator of the compensating Journal shall not approve the same Journal where maker-checker applies. |
| Unauthorized reversal prevention | Journal Approver and reversal authority | A Journal Approver shall not execute an unauthorized reversal. |
| Partial-reversal authorization | Financial authorization and reversal participants | A partial reversal requires explicit financial authorization before posting. |
| Period protection | Reversal authority and period-governance authority | Closed or locked periods cannot be bypassed; reopening requires separate authorized approval. |
| Audit independence | Internal Auditor | Audit users shall not modify accounting records. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve the original posting | Immutable original Journal, source reference, Journal lines, posting reference, period, accounts, and audit history | Accounting & General Ledger |
| Preserve the reversal basis and authority | Reversal request, business reason, supported reversal treatment, and authorization or approval outcome | Accounting & General Ledger |
| Preserve reversal lineage | Mandatory immutable reference from the compensating Journal to the original Journal | Accounting & General Ledger |
| Preserve compensating-Journal integrity | Unique Journal number, balanced lines, validation and approval results, period, Fiscal Year, currency, and account references | Accounting & General Ledger |
| Preserve the financial consequence | Compensating posting, affected Ledger Accounts, General Ledger update, and resulting original-Journal lifecycle effect | Accounting & General Ledger |
| Preserve completed facts | Immutable `JournalPosted`, `JournalReversed`, and `LedgerUpdated` events and complete audit lineage | Accounting & General Ledger |

Section A retains ownership of authorization and audit infrastructure; Accounting owns the reversal decision, accounting evidence, and financial records.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The original `Posted` Journal remains unchanged, immutable, and fully traceable.
- An authorized, balanced, validated, approved, and separately identified compensating Journal references the original Journal.
- The compensating Journal reaches `Posted`, and all affected Ledger Accounts and the General Ledger reflect the complete reversing consequence.
- For the full-reversal path, the original Journal reaches `Reversed`.
- Reversal authorization, original-to-reversal lineage, posting evidence, and accounting audit history are reconstructable.
- `JournalPosted`, `JournalReversed`, and `LedgerUpdated` represent the completed full-reversal facts.

## 16.2 Postconditions

- The original and compensating Journals are retained as separate immutable accounting records.
- The Accounting Ledger reflects both the original posting and the compensating reversal in chronological history.
- Any further correction uses another governed accounting entry rather than modification of either posted Journal.
- The originating operational transaction remains owned by its canonical Finance capability and is not automatically reversed by this accounting outcome.
- The lifecycle postcondition for an original Journal after partial reversal remains unresolved as recorded in Section 17.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F05` supports partial reversals and requires their financial authorization, but the Journal Entry lifecycle provides only `Posted -> Reversed` and no partial-reversal state or lifecycle rule. | Section 13 cannot determine whether a partially reversed original Journal remains `Posted`, becomes `Reversed`, or requires a distinct lifecycle position. This materially affects the supported alternate outcome and prevents locking. | Accounting & General Ledger owner and Lead Architect must define the original Journal's lifecycle treatment after partial reversal. |
| `L2-F05` requires reversal authorization and defines typical configurable Accounting roles, but it does not assign reversal execution or partial-reversal authorization to a canonical business role. | Sections 6 and 15 cannot establish complete authority or separation responsibilities for every reversal action. | Accounting & General Ledger owner must define or confirm the configuration-governed business authority mapping for full and partial reversals. |
| The reversal workflow places authorization before creation of the Reversal Journal, while the general Journal lifecycle requires validation and approval before posting; the relationship between reversal authorization and compensating-Journal approval is not explicitly defined. | The Draft can preserve both controls but cannot determine whether they are one decision or separate governed decisions. | Accounting & General Ledger owner must clarify the business decision sequence and authority relationship in authoritative L2 architecture. |
| The Reversal Service supports correction-entry, accrual, and closing reversals but does not define scenario-specific eligibility, decision rules, or lifecycle differences for those forms. | Section 10 can preserve the supported forms but cannot establish when each is selected or whether their original-Journal lifecycle effects differ. | Accounting & General Ledger owner must define the applicable business decision rules or explicitly confirm common reversal treatment. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

## Draft Validation Classification

**BLOCKED** - The document is complete as a Template v2.0 Draft and the full-reversal path is strongly grounded, but authoritative L2 leaves the supported partial-reversal lifecycle, reversal authority, approval sequence, and reversal-form decision rules unresolved. These material lifecycle and authorization gaps prevent Architecture Approval and locking.

---

**End of Scenario L3-F-028**
