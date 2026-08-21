# Accounting Source and Ledger to Reconciled Position

# 1. Scenario Metadata

**Scenario ID:** L3-F-029  
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

This scenario governs comparison of an authoritative Accounting Source with the Accounting Ledger to establish a verified reconciled financial position. It ensures that differences are detected, investigated, resolved through governed Finance workflows, and closed with complete traceability and audit evidence without overwriting historical accounting records.

---

# 3. Business Outcome

The selected Accounting Source and corresponding ledger position have been compared and brought to an auditable reconciled position, with every identified difference either governed to resolution or retained as an unresolved exception that prevents reconciliation closure.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a defined accounting population and period are selected for reconciliation between an authoritative Accounting Source and the corresponding General Ledger, Ledger Account, control account, or other Accounting-owned ledger position.

## 4.2 End Boundary

The scenario ends when the comparison is complete, no unresolved difference remains in the selected scope, the reconciliation outcome has completed audit closure, and `LedgerReconciled` represents the completed business fact.

## 4.3 In Scope

- Definition of the reconciliation population, accounting period, and comparison boundary.
- Identification and validation of the authoritative Accounting Source and corresponding ledger position.
- Comparison of source and ledger financial facts.
- Detection, classification, and investigation of differences.
- Governed resolution through the appropriate source-owned or Accounting-owned workflow.
- Reperformance of comparison after correction.
- Reconciliation closure, audit evidence, and publication of the completed reconciliation fact.
- Ledger, bank, control-account, Journal, Trial Balance, and opening-versus-closing balance reconciliation where supported by `L2-F05`.

## 4.4 Out of Scope

- Creation or modification of the originating operational transaction.
- Direct editing of historical Journals, Ledger Accounts, ledger balances, or source records.
- Definition of bank-statement processing or Treasury-owned Bank Account lifecycles.
- Journal reversal, which belongs to `L3-F-028` when required as a governed correction.
- Trial Balance generation, which belongs to `L3-F-030`.
- Accounting-period closure, which belongs to `L3-F-031`.
- Financial reporting or statutory-statement preparation.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | `L3-F-028` Posted Journal to Accounting Reversal | May govern an Accounting correction required to resolve a reconciliation difference. |
| Related | `L3-F-030` Accounting Period Activity to Trial Balance | Provides a derived balance position that may be reconciled or verified. |
| Precedes | `L3-F-031` Ledger Period to Controlled Accounting Close | Reconciliation status is materially relevant to period-closing readiness. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounting & General Ledger / Section F | Owns the Accounting Ledger, reconciliation workflow, ledger comparison, difference investigation, accounting correction requirement, reconciliation conclusion, `LedgerReconciled`, and reconciliation audit closure. |
| Accounting Source-owning Finance capability / Section F | Retains ownership of the source financial fact, supplies the authoritative comparison position, and resolves any confirmed source-side discrepancy through its governed workflow. |

Participation of a source-owning Finance capability does not transfer ownership of its operational record to Accounting. Accounting may compare and reference the source fact but cannot modify it directly.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Accountant | Defines the reconciliation scope, performs comparison, investigates differences, and prepares the reconciliation evidence. | Performs reconciliation under the source-defined typical Accounting role. |
| Senior Accountant | Supervises the reconciliation and reviews the proposed resolution and closure. | Reviews and supervises under the source-defined typical Accounting role. |
| Internal Auditor | Reviews reconciliation history and audit evidence without modifying accounting records. | Receives and examines the completed or unresolved reconciliation evidence. |

`L2-F05` defines these as typical, institution-configurable roles. It does not assign authoritative approval of discrepancy resolution or audit closure to one canonical business role. That authority gap is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- A defined reconciliation population, accounting period, institution, currency context, and comparison boundary exist.
- The authoritative Accounting Source and corresponding ledger position are identifiable and available for the same reconciliation scope.
- Source references and Accounting references permit traceability between the compared financial facts.
- The relevant Journals and ledger history are available and immutable.
- Any correction required to resolve a difference must use the workflow owned by the affected source or Accounting aggregate rather than direct historical modification.

---

# 8. Trigger

Accounting initiates a governed reconciliation of a defined Accounting Source population against its corresponding ledger position.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Define the reconciliation scope, including the Accounting Source, corresponding ledger position, accounting period, population, institution, and currency context. | Accountant / Accounting & General Ledger | A bounded and reproducible reconciliation population is established. |
| 2 | Obtain the authoritative source position and corresponding Accounting Ledger facts while preserving each record's canonical owner. | Accountant / Accounting & General Ledger | The comparison inputs are traceable and aligned to the defined scope. |
| 3 | Compare the source and ledger positions using their business references, amounts, classifications, and accounting chronology. | Accountant / Accounting & General Ledger | Matching positions and any differences are identified. |
| 4 | Confirm that no unexplained difference remains within the selected reconciliation scope. | Accountant / Accounting & General Ledger | The source and ledger positions agree for the defined population. |
| 5 | Review the comparison outcome and supporting evidence for reconciliation closure. | Senior Accountant / Accounting & General Ledger | The completed comparison is independently reviewed under the institution's configured authority. |
| 6 | Close the reconciliation with its comparison basis, result, review evidence, and audit history. | Accounting & General Ledger | The selected source and ledger population reaches an auditable reconciled position. |
| 7 | Publish the completed reconciliation fact. | Accounting & General Ledger | `LedgerReconciled` communicates that the governed reconciliation has completed. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Difference detected and resolved | The initial comparison identifies a source-to-ledger difference. | Investigate the difference, determine its authoritative cause, resolve it through the affected source-owned or Accounting-owned workflow, repeat the comparison, and proceed to closure only when no unresolved difference remains. | The corrected source and ledger positions reach the same governed reconciled outcome without direct historical modification. | Accounting & General Ledger and affected Finance capability |
| Valid timing difference | The compared positions differ because the same valid financial fact belongs to different recognized comparison points. | Establish the source-supported timing basis and perform reconciliation at the appropriate common comparison boundary. | The reconciliation proceeds only when the aligned comparison demonstrates agreement; no local timing rule is invented. | Accounting & General Ledger |

The authoritative L2 workflow supports investigation and resolution but does not define detailed discrepancy categories or decision rules. Those gaps are recorded in Section 17.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The authoritative Accounting Source, ledger position, or common reconciliation scope cannot be established. | Comparison would not produce a reliable or reproducible outcome. | Do not close the reconciliation; establish the missing source, ledger, or scope definition before performing a new comparison. | Accounting & General Ledger and affected Finance capability |
| Source-to-ledger references are missing or cannot be traced. | Differences cannot be related to the correct financial facts. | Preserve the unresolved exception and resolve the traceability deficiency through the owning workflow before reconciliation resumes. | Owner of the deficient record |
| A difference is detected but its cause or canonical owner cannot be determined. | The financial position cannot be declared reconciled. | Retain the discrepancy as an exception requiring investigation; no `LedgerReconciled` event or audit closure occurs. | Accounting & General Ledger |
| A correction is identified but lacks the required business authorization. | The difference remains unresolved and historical records cannot be changed. | Obtain the governed authority through the applicable correction, adjustment, or reversal workflow; reconciliation remains open. | Owner of the correction workflow |
| Reperformance still identifies a difference. | The selected population has not reached a reconciled position. | Continue governed investigation and resolution or retain the unresolved exception; do not close the reconciliation. | Accounting & General Ledger and affected Finance capability |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Ledger reconciliation covers the General Ledger, Bank Ledger, control accounts, suspense accounts, receivables, and payables. | Accounting & General Ledger | These are supported reconciliation contexts, subject to preservation of each source owner's boundary. | `L2-F05`, Section 35.9 |
| Reconciliation proceeds through comparison, difference detection, investigation, resolution, and audit closure. | Accounting & General Ledger | A difference prevents closure until governed resolution and successful reperformance. | `L2-F05`, Section 35.9 |
| Any reconciliation discrepancy shall generate an exception requiring investigation. | Accounting & General Ledger | An unexplained difference cannot be silently accepted as reconciled. | `L2-F05`, Section 34.7 |
| `BR-011` and `BR-012`: Ledger balances change only through successful Journal posting; manual balance modification is prohibited. | Accounting & General Ledger | Reconciliation cannot directly alter a ledger balance. | `L2-F05`, Section 36.2 |
| `BR-016` and `BR-017`: Running balances remain mathematically consistent and historical balances are never overwritten. | Accounting & General Ledger | Resolution must preserve consistency and immutable history. | `L2-F05`, Section 36.2 |
| `BR-020`: Ledger history remains permanently auditable. | Accounting & General Ledger | Comparison and correction lineage must remain reconstructable. | `L2-F05`, Section 36.2 |
| `BR-049` through `BR-052`: Historical transactions are never edited; Accounting corrections use reversal Journals with original references and complete history. | Accounting & General Ledger | A ledger-side correction identified by reconciliation must use the governed reversal path where applicable. | `L2-F05`, Section 36.7 |
| `BR-059`, `BR-060`, `BR-061`, and `BR-062`: Financial operations generate audit events; audit history is retained; Journals and Ledger updates remain traceable. | Accounting & General Ledger | Reconciliation and all resolving accounting actions require permanent traceability. | `L2-F05`, Section 36.9 |
| `BR-064`, `BR-068`, and `BR-069`: Historical records remain immutable, financial operations are deterministic, and workflows produce reproducible results. | Accounting & General Ledger | The same governed population and comparison basis must produce a reproducible reconciliation conclusion. | `L2-F05`, Sections 36.9 and 36.10 |

---

# 13. State and Lifecycle Impact

No authoritative Reconciliation aggregate or reconciliation lifecycle is defined by `L2-F05`. The workflow establishes a completed reconciled business position and the exact `LedgerReconciled` event, but it does not name operational reconciliation states or an aggregate that owns them.

The Accounting Ledger and affected Ledger Accounts remain in their existing active lifecycle positions. Reconciliation verifies their consistency; it does not itself create a new Ledger lifecycle state. Any source correction, compensating Journal, or other resolving action follows the lifecycle owned by that separate aggregate.

The absence of a canonical reconciliation record and lifecycle is recorded in Section 17 because it affects how the final reconciled position and audit closure are authoritatively represented.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `LedgerReconciled` | Accounting & General Ledger | Audit & Compliance | Communicates that the governed source-to-ledger reconciliation has completed. | `L2-F05`, Sections 25.8 and 37.3 |

Resolving workflows may publish their own canonical events, such as `JournalReversed`, `JournalPosted`, or `LedgerUpdated`, when their independently governed outcomes complete. Those events are not re-published or re-owned by this reconciliation scenario.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Accounting Source-owning Finance capability | Accounting & General Ledger | Authoritative source financial position for the defined reconciliation population and comparison boundary | Provides the source-owned facts required for comparison without transferring ownership or permitting Accounting modification. | The generic source-to-Accounting contract is implied by `L2-F05`, Sections 34.7 and 35.9, but not fully defined. |
| Accounting & General Ledger | Accounting Source-owning Finance capability | Confirmed source-side discrepancy requiring governed investigation or correction | Returns a difference to its canonical owner for resolution without prescribing changes to the source record. | `L2-F05`, Sections 34.7 and 35.9 |

The exact source contract, mandatory comparison context, and provider mapping are not defined upstream and remain open issues.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Define and perform the reconciliation | Accountant | May perform reconciliation under the institution's configured Accounting authority. | Accounting & General Ledger |
| Review difference resolution and reconciliation closure | Senior Accountant | May supervise and review under the source-defined typical Accounting role; final closure authority is not canonically assigned. | Accounting & General Ledger |
| Correct a source-side discrepancy | Role defined by the source-owning workflow | Correction must occur only under the source owner's governed authority; this scenario cannot assign that role generically. | Accounting Source-owning Finance capability |
| Correct an Accounting-side discrepancy | Role defined by the applicable Journal, adjustment, or reversal workflow | Historical Accounting records cannot be edited directly and required authorization must precede correction. | Accounting & General Ledger |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Reconciliation review | Accountant and Senior Accountant | The prepared comparison and proposed closure are subject to supervisory review under institution-configured authority. |
| Correction authority | Reconciliation participants and correction-workflow authority | Detecting a difference does not authorize direct correction; the owning workflow's approval and separation rules apply. |
| Audit independence | Internal Auditor | Audit users shall not modify Accounting or source records. |
| Historical protection | Reconciliation and correction participants | No override may overwrite Journal, Ledger, or source history merely to force agreement. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve reconciliation scope | Accounting Source, ledger position, population, accounting period, institution, currency context, and comparison boundary | Accounting & General Ledger |
| Preserve comparison basis | Source references, Accounting references, compared positions, matching result, and reproducible comparison evidence | Accounting & General Ledger; source owner retains source facts |
| Preserve differences and investigation | Identified discrepancy, traceability, investigation outcome, canonical owner, and unresolved-exception status | Accounting & General Ledger and affected source owner |
| Preserve resolution lineage | Governed source correction, adjustment, Journal reversal, or other owning-workflow outcome and its authorization | Owner of the resolving workflow |
| Preserve closure | Reperformed comparison, review outcome, reconciled conclusion, `LedgerReconciled`, and audit closure evidence | Accounting & General Ledger |

Section A retains ownership of authorization and audit infrastructure; Accounting and each source capability retain ownership of their respective business decisions and evidence.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Accounting Source and corresponding ledger position are aligned to one defined reconciliation population and comparison boundary.
- The comparison is complete and reproducible from authoritative source and Accounting references.
- Every detected difference has been investigated and resolved through its canonically owned workflow.
- Reperformance confirms that no unresolved difference remains in the selected scope.
- The reconciliation has completed supervisory review and audit closure under the institution's governed authority.
- `LedgerReconciled` has been published as the completed Accounting fact.

## 16.2 Postconditions

- The selected Accounting Source and ledger position have a traceable reconciled relationship for the defined comparison boundary.
- Historical source, Journal, Ledger Account, and General Ledger records remain immutable.
- Corrections remain represented through the separately governed records and lifecycles of their canonical owners.
- The reconciliation evidence is available for Trial Balance verification, period closing, financial reporting, and audit.
- Any later difference requires a new governed reconciliation rather than alteration of the completed evidence.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F05` defines reconciliation services, workflow, and `LedgerReconciled` but no Reconciliation aggregate, canonical reconciliation record, lifecycle, or operational states. | Section 13 cannot map the final reconciled position or audit closure to an authoritative owned lifecycle or record. This materially affects final-outcome representation and locking. | Accounting & General Ledger owner and Lead Architect must define the canonical representation and lifecycle of a reconciliation outcome or explicitly confirm an event-only model. |
| The frozen title uses generic â€œAccounting Source,â€ while `L2-F05` lists several reconciliation contexts without defining the authoritative provider, mandatory source contract, common comparison boundary, or ownership mapping for each. | Sections 5, 7, 9, and 14 can preserve generic ownership but cannot establish a complete reusable source-to-ledger contract. | Finance domain owners must define the canonical source contracts and provider mapping for each supported reconciliation context. |
| `L2-F05` requires difference investigation and resolution but does not define discrepancy categories, ownership decision rules, tolerance treatment, timing-difference rules, acceptance conditions, or escalation outcomes. | Section 10 cannot authoritatively determine how a detected difference progresses to resolution or whether any governed exception may close without exact agreement. | Accounting & General Ledger owner must define reconciliation decision rules and governed exception outcomes upstream. |
| `L2-F05` defines typical configurable roles but does not assign final discrepancy-resolution acceptance or reconciliation audit-closure authority to a canonical role. | Section 15 cannot establish complete decision authority or approval separation for the final reconciled outcome. | Accounting & General Ledger owner must define or confirm the configuration-governed closure and discrepancy-acceptance authorities. |
| `LedgerReconciled` is defined, but the reconciliation scope represented by one event and its interested consumers beyond Audit & Compliance are not established. | Section 14 cannot define whether the event refers to a Ledger, Ledger Account, period, source population, or another reconciliation boundary. | Accounting & General Ledger owner must define the canonical business scope and consumers of `LedgerReconciled`. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

## Draft Validation Classification

**BLOCKED** - The document is complete as a Template v2.0 Draft, but authoritative L2 does not define the reconciliation aggregate or outcome lifecycle, the generic Accounting Source contract, discrepancy-resolution rules, final closure authority, or the scope of `LedgerReconciled`. These material outcome, contract, rule, event, and authority gaps prevent Architecture Approval and locking.

---

**End of Scenario L3-F-029**
