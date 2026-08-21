# Accounting Period Activity to Trial Balance

# 1. Scenario Metadata

**Scenario ID:** L3-F-030  
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

This scenario governs derivation of a Trial Balance Snapshot from posted Accounting Ledger activity for a defined accounting point or period boundary. It verifies debit-credit equality, preserves account-level balance evidence, and establishes an immutable published Trial Balance for reconciliation, period closing, financial reporting, and audit without allowing manual maintenance of the Trial Balance.

---

# 3. Business Outcome

Accounting period activity has been represented by a verified and published Trial Balance Snapshot derived exclusively from posted General Ledger transactions, with total debit equal to total credit and the result preserved as immutable financial evidence.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Accounting initiates Trial Balance generation for a defined ledger, institution, Accounting Period or point-in-time boundary, and applicable financial scope after the relevant Journal postings are available in the General Ledger.

## 4.2 End Boundary

The scenario ends when the Trial Balance Snapshot reaches `Published`, its debit and credit totals are verified as equal, its account balances remain traceable to posted ledger activity, and the completed Trial Balance events and audit evidence are preserved.

## 4.3 In Scope

- Definition of the Trial Balance generation scope and accounting boundary.
- Derivation of account balances exclusively from posted General Ledger transactions.
- Consolidation of debit and credit balances for the selected scope.
- Verification that aggregate debit equals aggregate credit.
- Lifecycle progression of the Trial Balance Snapshot through generation, verification, and publication.
- Preservation of immutable snapshot, event, and audit evidence.
- Availability of the published Trial Balance for downstream reconciliation, closing, reporting, and audit.

## 4.4 Out of Scope

- Creation, approval, posting, or reversal of Journal Entries.
- Manual entry or maintenance of Trial Balance values.
- Correction of ledger discrepancies; applicable reconciliation or reversal scenarios govern those outcomes.
- Ledger reconciliation, which belongs to `L3-F-029`.
- Accounting-period closing and locking, which belong to `L3-F-031`.
- Financial statement preparation, statutory reporting, analytics, or external publication.
- Recalculation or alteration of a historical Trial Balance Snapshot.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-025` Financial Transaction to Balanced Ledger Posting | Produces posted ledger activity from which the Trial Balance is derived. |
| Related | `L3-F-029` Accounting Source and Ledger to Reconciled Position | May reconcile or verify the ledger position represented by the Trial Balance. |
| Precedes | `L3-F-031` Ledger Period to Controlled Accounting Close | A verified Trial Balance is required before Accounting Period closure. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounting & General Ledger / Section F | Owns the General Ledger, Ledger Account balances, Trial Balance generation and verification, Trial Balance Snapshot lifecycle, publication decision, canonical Trial Balance events, and final published outcome. |

Financial Reporting, Audit, and other consumers may use the completed Trial Balance as a read-only Accounting fact. Consumption does not transfer ownership or permit independent generation, recalculation, or modification.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Accountant | Defines the generation scope, initiates Trial Balance preparation, and examines the derived account balances. | Performs Trial Balance preparation under the source-defined typical Accounting role. |
| Senior Accountant | Reviews debit-credit verification and supervises the Trial Balance outcome. | Reviews and supervises under the source-defined typical Accounting role. |
| Internal Auditor | Examines the published Trial Balance and its ledger lineage without modifying Accounting records. | Receives and reviews the completed evidence. |

`L2-F05` defines these as typical, institution-configurable roles. It does not canonically assign generation, verification, publication, or archival authority for Trial Balance Snapshots. The authority gap is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- The applicable Accounting Ledger and Ledger Accounts exist for the selected institutional scope.
- The Accounting Period or point-in-time generation boundary is defined.
- The applicable posted Journal Entries and their Ledger Account effects are available in the General Ledger.
- Ledger balances and posting history remain mathematically consistent and traceable.
- The Trial Balance will be derived only from posted ledger transactions and not from pending, draft, validated, or approved-but-unposted Journals.
- A historical Trial Balance Snapshot for a prior boundary will not be recalculated or overwritten.

---

# 8. Trigger

Accounting requests generation of a Trial Balance for a defined ledger and Accounting Period or point-in-time boundary.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the Trial Balance scope, including ledger, institution, Accounting Period or point-in-time boundary, and applicable account population. | Accountant / Accounting & General Ledger | A bounded and reproducible Trial Balance generation basis exists. |
| 2 | Initiate a Trial Balance Snapshot from the posted General Ledger activity within the defined scope. | Accounting & General Ledger | The Trial Balance Snapshot enters `Generating`. |
| 3 | Calculate every included Ledger Account balance and consolidate debit and credit positions without manually maintaining snapshot values. | Accounting & General Ledger | The snapshot contains the derived account balance population and aggregate totals. |
| 4 | Verify that every balance is traceable to posted ledger activity and that total debit equals total credit. | Senior Accountant / Accounting & General Ledger | The Trial Balance Snapshot reaches `Verified`; an imbalance prevents progression. |
| 5 | Review the verified snapshot for publication under the institution's governed Accounting authority. | Senior Accountant / Accounting & General Ledger | The verified Trial Balance is accepted for publication without altering its derived values. |
| 6 | Publish the verified Trial Balance Snapshot as immutable financial evidence. | Accounting & General Ledger | The Trial Balance Snapshot reaches `Published` and becomes available as a read-only Accounting fact. |
| 7 | Preserve the snapshot lineage, verification, publication, events, and audit history. | Accounting & General Ledger | `TrialBalanceGenerated`, `TrialBalanceVerified`, and `TrialBalancePublished` represent the completed lifecycle facts, subject to the event-sequencing issue in Section 17. |

The flow stops at Trial Balance publication. Financial Reporting, reconciliation, Accounting Period closure, and later archival are distinct downstream uses or lifecycle outcomes.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow toward the same verified and published Trial Balance outcome is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Total debit does not equal total credit. | The Trial Balance violates its aggregate invariant and cannot reach `Verified` or `Published`. | Treat the imbalance as a financial discrepancy requiring investigation through the governed reconciliation workflow; do not alter the snapshot or ledger manually. The Trial Balance failure lifecycle is undefined upstream. | Accounting & General Ledger |
| A balance cannot be traced to posted Journal activity. | The snapshot cannot establish authoritative ledger lineage. | Stop verification and investigate the missing or inconsistent accounting lineage through the governed Accounting workflow. No publication occurs. | Accounting & General Ledger |
| The selected scope or accounting boundary is incomplete, inconsistent, or not reproducible. | The derived Trial Balance would not represent a reliable financial position. | Correct the generation scope before initiating a new snapshot; do not publish the incomplete result. | Accounting & General Ledger |
| Pending or unposted Journals are incorrectly included. | The snapshot would no longer be derived exclusively from posted ledger transactions. | Exclude ineligible activity and initiate generation using the governed posted-ledger population. | Accounting & General Ledger |
| Verification or publication authority is absent. | The snapshot cannot complete its governed lifecycle. | Retain the outcome outside `Published` until the required institution-configured authority acts; no downstream publication fact is produced. | Accounting & General Ledger |

The source defines no failed, rejected, cancelled, or superseded Trial Balance Snapshot state. No recovery transition is invented in this Draft.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Trial Balance Snapshots are generated only from posted Ledger transactions. | Accounting & General Ledger | Draft, validated, approved-but-unposted, failed, or rejected Journals are excluded. | `L2-F05`, Section 33 |
| Trial Balance total debit shall equal total credit. | Accounting & General Ledger | An imbalanced snapshot cannot be verified or published. | `L2-F05`, Sections 33 and 35.8 |
| Trial Balance Snapshots are immutable and historical snapshots are never recalculated. | Accounting & General Ledger | Each completed snapshot preserves its original boundary and derived balances. | `L2-F05`, Section 33 |
| Trial Balance Snapshots are read-only and generation occurs after posting completion. | Accounting & General Ledger | Snapshot values cannot be manually maintained or derived from incomplete postings. | `L2-F05`, Section 33 |
| Trial Balance shall never be manually maintained. | Accounting & General Ledger | All values must be derived directly from the General Ledger. | `L2-F05`, Section 35.8 |
| `BR-016`, `BR-017`, and `BR-020`: Running balances remain consistent, historical balances are not overwritten, and ledger history remains auditable. | Accounting & General Ledger | The snapshot must preserve mathematical integrity and permanent traceability. | `L2-F05`, Section 36.2 |
| `BR-034`: Trial Balance shall be verified before Accounting Period closure. | Accounting & General Ledger | The verified snapshot is a required input to the separately governed closing outcome. | `L2-F05`, Section 36.4 |
| `BR-059`, `BR-060`, `BR-061`, and `BR-062`: Financial operations generate audit events; audit history is retained; Journals and ledger updates remain traceable. | Accounting & General Ledger | Snapshot derivation, verification, and publication require reconstructable Accounting evidence. | `L2-F05`, Section 36.9 |
| `BR-064`, `BR-068`, `BR-069`, and `BR-070`: Historical records are immutable, operations deterministic, results reproducible, and the Accounting Ledger is the single source of truth. | Accounting & General Ledger | The same ledger scope and boundary must yield the same Trial Balance without a competing source. | `L2-F05`, Sections 36.9 and 36.10 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Trial Balance Snapshot | New generation requirement | Posted ledger activity is selected for the defined accounting boundary and account population. | `Generating` | Accounting & General Ledger | `L2-F05`, Section 33 |
| Trial Balance Snapshot | `Generating` | Account balances and aggregate debit and credit totals are derived and verified. | `Verified` | Accounting & General Ledger | `L2-F05`, Sections 33, 34.5 and 35.8 |
| Trial Balance Snapshot | `Verified` | The verified snapshot is made available as immutable Accounting evidence. | `Published` | Accounting & General Ledger | `L2-F05`, Section 33 |
| Trial Balance Snapshot | `Published` | A later governed retention action may remove the snapshot from active use without deleting history. | `Archived` | Accounting & General Ledger | `L2-F05`, Section 33 |
| Accounting Ledger and Ledger Accounts | Existing active positions | The scenario reads posted balances and history to derive the snapshot; it does not change ledger values or lifecycle states. | Unchanged | Accounting & General Ledger | `L2-F05`, Sections 23, 25, 33 and 35.8 |

No lifecycle transition is defined for a snapshot that fails debit-credit verification or cannot establish ledger lineage. That material recovery gap is recorded in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `TrialBalanceGenerated` | Accounting & General Ledger | Financial Reporting; Audit & Compliance | Records successful generation of the Trial Balance from posted ledger activity. | `L2-F05`, Sections 33, 37.7 and 37.8 |
| `TrialBalanceVerified` | Accounting & General Ledger | Audit & Compliance; other consumers not authoritatively defined | Records successful debit-credit verification of the generated snapshot. | `L2-F05`, Sections 33 and 37.7 |
| `TrialBalancePublished` | Accounting & General Ledger | Audit & Compliance; other consumers not authoritatively defined | Records publication of the verified immutable snapshot. | `L2-F05`, Sections 33 and 37.7 |

The event catalog and consumer table do not establish whether Financial Reporting should act on `TrialBalanceGenerated`, `TrialBalanceVerified`, or `TrialBalancePublished`. The sequencing and consumption ambiguity is recorded in Section 17.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Accounting & General Ledger | Financial Reporting | Read-only Trial Balance Snapshot derived from the General Ledger | Provides the Accounting-owned balance position for reporting without permitting independent generation or modification. | `L2-F05`, Sections 33, 35.8 and 37.8 |

The exact lifecycle position at which the Trial Balance becomes eligible for Financial Reporting consumption remains unresolved in Section 17.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Define and initiate Trial Balance generation | Accountant | May prepare the Trial Balance under the institution's configured Accounting authority. | Accounting & General Ledger |
| Verify debit-credit equality and ledger lineage | Senior Accountant | May review and verify under the source-defined typical supervisory role. | Accounting & General Ledger |
| Publish or archive the Trial Balance Snapshot | Not assigned to one canonical business role in `L2-F05` | Publication and archival must preserve the defined lifecycle, immutable history, and institution-configured authority. | Accounting & General Ledger |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Preparation and verification review | Accountant and Senior Accountant | The prepared Trial Balance is subject to supervisory verification under the institution's configured Accounting authority. |
| Ledger-source protection | Trial Balance participants and Journal-posting authority | Trial Balance preparation cannot modify ledger balances or bypass Journal posting. |
| Imbalance handling | Trial Balance participants and reconciliation authority | An imbalance cannot be overridden into a verified or published result; it requires governed investigation. |
| Audit independence | Internal Auditor | Audit users shall not modify the Trial Balance, Journals, or Ledger records. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve generation scope | Ledger, institution, Accounting Period or point-in-time boundary, account population, and currency context | Accounting & General Ledger |
| Preserve derivation lineage | Posted Journal and Ledger Account references supporting each derived balance | Accounting & General Ledger |
| Preserve balance verification | Account balances, aggregate debit, aggregate credit, equality result, and review outcome | Accounting & General Ledger |
| Preserve publication | Snapshot lifecycle position, publication outcome, and immutable published content | Accounting & General Ledger |
| Preserve completed events and history | `TrialBalanceGenerated`, `TrialBalanceVerified`, `TrialBalancePublished`, and complete audit lineage | Accounting & General Ledger |

Section A retains ownership of authorization and audit infrastructure; Accounting owns the Trial Balance decisions, evidence, and financial record.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Trial Balance Snapshot represents a defined ledger and Accounting Period or point-in-time boundary.
- Every snapshot balance is derived exclusively from posted General Ledger activity and remains traceable to authoritative Accounting records.
- Aggregate debit equals aggregate credit.
- The Trial Balance Snapshot reaches `Verified` and then `Published`.
- The published snapshot is immutable and cannot be manually maintained or historically recalculated.
- Generation, verification, publication, events, and audit history are reconstructable.

## 16.2 Postconditions

- The published Trial Balance is available as a read-only Accounting fact for reconciliation, period closing, financial reporting, and audit.
- The General Ledger and Ledger Account lifecycles and balances remain unchanged by snapshot generation.
- Historical Trial Balance Snapshots remain immutable and are not recalculated when later accounting activity occurs.
- Any identified imbalance or lineage discrepancy remains outside successful completion and requires governed investigation and correction before a new valid snapshot can be published.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The Trial Balance Snapshot lifecycle defines only `Generating -> Verified -> Published -> Archived` and provides no failed, invalid, rejected, cancelled, or superseded position when debit and credit do not agree or ledger lineage fails. | Section 11 can prevent publication but cannot map the invalid generation attempt or its recovery to an authoritative lifecycle. This materially affects failure handling and locking. | Accounting & General Ledger owner and Lead Architect must define the invalid-generation and recovery lifecycle or explicitly govern non-creation of a snapshot until validation succeeds. |
| `L2-F05` defines typical configurable roles but does not assign canonical generation, verification, publication, or archival authority for Trial Balance Snapshots. | Sections 6 and 15 cannot establish complete business decision authority or separation for the final published outcome. | Accounting & General Ledger owner must define or confirm the configuration-governed authority mapping for the Trial Balance lifecycle. |
| `TrialBalanceGenerated` is listed as the event consumed by Financial Reporting, while the snapshot lifecycle separately requires verification and publication and also defines `TrialBalanceVerified` and `TrialBalancePublished`. | Section 14 cannot determine whether Financial Reporting may consume an unverified generated snapshot or must wait for verified or published status. This materially affects event sequencing and downstream correctness. | Accounting & General Ledger and Financial Reporting owners must define the authoritative consumption event and lifecycle eligibility point. |
| The Trial Balance Snapshot is defined for a point in time and period-end reporting, but `L2-F05` does not fully define canonical snapshot identity and scope dimensions such as ledger, institution or campus, Accounting Period versus as-of boundary, currency treatment, and account-population rules. | Sections 4, 7, 9, and 16 can state a governed scope but cannot establish a complete canonical generation contract. | Accounting & General Ledger owner must define the mandatory Trial Balance scope and identity in authoritative L2 architecture. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

## Draft Validation Classification

**BLOCKED** - The document is complete as a Template v2.0 Draft and the successful Trial Balance lifecycle is strongly grounded, but authoritative L2 leaves the invalid-generation lifecycle, business authority, downstream event eligibility, and canonical snapshot scope unresolved. These material lifecycle, authority, event, and outcome-contract gaps prevent Architecture Approval and locking.

---

**End of Scenario L3-F-030**
