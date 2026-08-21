# Financial Year End to Immutable Year Closure

# 1. Scenario Metadata

**Scenario ID:** L3-F-053  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Financial Period & Closing  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-B01_Institution_Workflow.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; relevant Budget and Reporting architecture; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs finalization of institutional Accounting for a Fiscal/Financial Year after all constituent Accounting Periods are closed. It coordinates year-end validation, final-period evidence, closing entries, balance carry-forward, next-year opening balances, executive governance approval, Fiscal Year closure, and permanent read-only historical protection without rewriting Journals, Ledger balances, period history, or prior-year facts.

---

# 3. Business Outcome

The authoritative Fiscal Year has progressed from `Open` through `Closing` to `Closed`. All Accounting Periods are closed, year-end financial integrity is validated, closing balances have been carried forward, next-year opening balances have been generated from prior closing balances, required executive approval is preserved, `FiscalYearClosed` has been published, and the closed year is immutable, read-only, auditable, and historically reconstructable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative Open Fiscal Year reaches its governed year-end boundary and year-closing activity is initiated for one Institution and Ledger context.

## 4.2 End Boundary

The scenario ends when the Fiscal Year successfully enters `Closed`, year-end entries/carry-forward/opening balances and approvals are traceable, closure events are published, and the year becomes read-only; or when any prerequisite fails and the year remains in its last valid state with blocking evidence preserved.

## 4.3 In Scope

- Identification of the Fiscal Year, Institution, Ledger, Fiscal Calendar, period population, lifecycle, and version/context.
- Verification that every Accounting Period belonging to the year is closed.
- Confirmation that the final Financial/Accounting Period is closed under its canonical workflow.
- Validation of pending Journals, balance integrity, reconciliation status, audit exceptions, Trial Balance, and year-end adjustment requirements.
- Initiation and governance of year-end processing.
- Creation and posting of governed year-end closing entries through Accounting-owned Journal behaviour.
- Determination of closing balances from the authoritative Ledger.
- Carry-forward of closing balances under the Fiscal Year aggregate's rules.
- Generation of next Fiscal Year's opening balances from prior closing balances.
- Executive governance approval and segregation of duties for Fiscal Year closure.
- Transition `Open -> Closing -> Closed` and publication of year-end events.
- Read-only protection and historical availability after closure.

## 4.4 Out of Scope

- Closing or modifying Financial Periods directly; period closure belongs to `L3-F-052`.
- Reopening/reclosing a closed period; that belongs to `L3-F-054`.
- Operational AR, AP, Treasury, Budget, Procurement, or source-domain corrections.
- Direct modification of Ledger balances or historical Journals.
- Financial-statement publication, statutory filing, tax determination, audit opinion, or external certification.
- Archival of the closed Fiscal Year, which is a later `Closed -> Archived` transition.
- Creation of the next Fiscal Year's period calendar beyond the opening-balance outcome.
- Lower-level balance-calculation, storage, communication, or execution design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-052` Open Financial Period to Locked Close | Establishes Closed Financial Periods required before year closure. |
| Related | `L3-F-030` Accounting Period Activity to Trial Balance | Supplies verified Trial Balance evidence for year-end validation. |
| Related | `L3-F-029` Accounting Source and Ledger to Reconciled Position | Supplies reconciliation evidence and `LedgerReconciled`. |
| Related | `L3-F-031` Ledger Period to Controlled Accounting Close | Supplies Accounting Period close evidence. |
| Related | `L3-F-054` Closed Period to Controlled Reopening and Reclosure | Governs exceptional period correction that may affect year-end conclusions. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounting & General Ledger / Section F | Owns the Fiscal Year aggregate, annual Accounting lifecycle, Journals, Ledgers, year-end entries, closing balances, carry-forward, opening-balance generation, Fiscal Year state, and year-end events. |
| Financial Period & Closing / Section F | Owns Financial Period lifecycle and supplies authoritative evidence that each period is Closed; it does not close the Accounting Fiscal Year. |
| Enterprise Governance | Owns executive approval workflow, hierarchy, delegation, exceptions, and immutable decision outcome. |
| Authorization & Policy / Section A | Owns authorization and access decisions, including permission to execute Fiscal Year closure. |
| AR, AP, Treasury, and Budget | Retain ownership of their aggregates and provide completed period/year-end facts or resolve owned blockers before year closure. |
| Financial Reporting | Consumes closed-year Accounting facts and produces read-only reports. |
| Audit | Reviews year-end validation, approvals, Journals, balances, carry-forward, opening balances, and closure evidence without modifying Accounting records. |
| Institution Management | Owns the institutional boundary within which the Fiscal Year and Ledger operate. |
| Fiscal Calendar Management | Owns the authoritative calendar definition and year boundary referenced by Accounting. |

No participant may directly modify the Fiscal Year aggregate, Journal, Ledger balance, Accounting Period, or Financial Period owned by another bounded context.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F05` defines typical configurable roles: Accountant, Senior Accountant, Finance Manager, Chief Financial Officer, Internal Auditor, and System Administrator. It assigns Fiscal Year governance/year-end approval to the Chief Financial Officer and states that Fiscal Year Closure requires executive approval. Because role definitions are institution-configurable and â€œexecutive approvalâ€ is not mapped to a universal hierarchy, exact authority remains subject to Section 17.

---

# 7. Preconditions and Entry Criteria

- One authoritative Fiscal Year aggregate exists in `Open` for one Institution and Ledger.
- Only one Fiscal Year is active for the Ledger.
- The Fiscal Calendar and year start/end boundaries are authoritative.
- Every Accounting Period belonging to the Fiscal Year is identifiable.
- Every required Accounting Period is `Closed`; the final period close evidence is available.
- Pending Journals, unbalanced entries, reconciliation exceptions, audit exceptions, and year-end adjustments can be evaluated.
- A verified Trial Balance and closing-balance basis are available.
- Required AR, AP, Treasury, Budget, and source-domain positions are complete or represented by governed resolution facts.
- Executive approval and explicit authorization can be obtained.
- The next Fiscal Year context exists or can validly receive opening balances under the canonical Accounting workflow.

---

# 8. Trigger

The authoritative Fiscal Year reaches its configured year-end boundary and Accounting initiates `FiscalYearClosingStarted` after confirming that the year-end workflow may begin.

The exact initiator, close-request contract, schedule, and relationship between Financial Year terminology in `L2-F09` and Fiscal Year terminology in `L2-F05` remain open in Section 17.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Fiscal Year, Institution, Ledger, Fiscal Calendar, year boundary, lifecycle, period population, and current year-end context. | Accounting & General Ledger | The exact annual Accounting boundary is established. |
| 2 | Verify that only one Fiscal Year is active for the Ledger and that every Accounting Period in the year is identifiable. | Accounting & General Ledger | Fiscal Year and period completeness are confirmed. |
| 3 | Obtain authoritative evidence that all Accounting/Financial Periods, including the final period, are Closed. | Accounting and Financial Period & Closing | The mandatory year-closing prerequisite is satisfied or a blocker is raised. |
| 4 | Initiate year-end processing, transition the Fiscal Year from `Open` to `Closing`, and publish `FiscalYearClosingStarted` after successful state change. | Accounting & General Ledger | The governed annual close is active and traceable. |
| 5 | Validate pending Journals, unbalanced entries, Trial Balance integrity, reconciliation status, audit exceptions, year-end adjustments, and cross-domain financial consistency. | Accounting & General Ledger with participating owners | Every unresolved year-end blocker is detected and assigned. |
| 6 | Resolve each blocker through its canonical workflow and reperform the relevant validation without editing historical records directly. | Canonical owner of each blocker | Year-end evidence becomes complete or closure remains blocked. |
| 7 | Create, validate, approve, and post the required year-end closing entries through the Accounting-owned Journal lifecycle. | Accountant, Senior Accountant, Finance Manager / Accounting | Governed closing effects enter the Ledger with complete history. |
| 8 | Determine final closing balances and verify that the Ledger and Trial Balance remain balanced, reconciled, and mathematically consistent. | Accounting & General Ledger | The authoritative closing position is established. |
| 9 | Carry forward closing balances according to the Fiscal Year aggregate rules and publish `CarryForwardCompleted` after success. | Accounting & General Ledger | Prior-year closing balances are traceably transferred to the next-year context. |
| 10 | Generate next Fiscal Year's opening balances from the prior closing balances and publish `OpeningBalancesGenerated`. | Accounting & General Ledger | The next year receives authoritative opening positions without changing the closed-year history. |
| 11 | Present the complete year-end validation, entries, balances, carry-forward, opening balances, exceptions, and audit evidence for executive governance approval. | Finance Manager / Accounting and Enterprise Governance | The closure decision receives independent governed authority. |
| 12 | If approval and all Fiscal Year invariants succeed, transition the Fiscal Year from `Closing` to `Closed`. | Accounting & General Ledger | The year becomes an immutable historical Accounting boundary. |
| 13 | Publish `FiscalYearClosed`, enforce read-only protection, and make the closed-year evidence available to Reporting and Audit. | Accounting & General Ledger | Immutable year closure is authoritative and historically reconstructable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Period remains open | One or more Accounting/Financial Periods are not Closed. | Retain the Fiscal Year `Open` or `Closing` as source-defined, complete the canonical period-close path, and restart year validation. | Fiscal Year closure is blocked. | Accounting and Financial Period & Closing |
| Pending Journal or unbalanced entry | Year-end validation identifies incomplete or invalid Accounting activity. | Complete, reject, correct, reverse, or post through the governed Journal workflow; regenerate Trial Balance and revalidate. | No manual balance adjustment occurs. | Accounting & General Ledger |
| Reconciliation/audit exception | A Ledger, source, control, or audit difference remains. | Preserve the exception, route it to the canonical owner, and do not close until resolved or upstream policy explicitly governs treatment. | Year remains unclosed. | Accounting, Audit, and affected owner |
| Executive approval rejected | Governance denies closure. | Preserve rejection and keep the Fiscal Year in its current valid state; do not publish `FiscalYearClosed`. | No unauthorized closure occurs. | Enterprise Governance and Accounting |
| Carry-forward fails | Closing balances cannot be transferred consistently. | Preserve the prior closing position, prevent opening-balance completion and year closure, correct the governed cause, and retry safely. | No partial year-end outcome is accepted. | Accounting & General Ledger |
| Opening balances fail validation | Generated opening balances do not derive exactly from prior closing balances. | Reject the generated outcome, preserve prior-year balances, correct the Accounting workflow, and regenerate. | The next year does not begin from an invalid position. | Accounting & General Ledger |
| Closed-period correction required | A constituent period requires legitimate correction during year-end. | Use governed period reopening/reclosure and then rerun affected year-end validations before approval. | Historical facts remain immutable. | Financial Period & Closing, Accounting, and Governance |
| Future archival | Retention and archival approval are later satisfied. | Apply the separate `Closed -> Archived` Fiscal Year transition. | Closed year becomes archived while remaining historically available. | Accounting & General Ledger |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Fiscal Year, Ledger, Institution, calendar, or period population is inconsistent. | The wrong annual boundary could be closed. | Reject initiation and correct authoritative context before retry. | Accounting and context owner |
| Period closure evidence is missing, stale, or contradictory. | BR-037 and Fiscal Year invariant cannot be proven. | Retain the year unclosed and obtain current authoritative period facts. | Accounting and Financial Period & Closing |
| Journal/Trial Balance/Ledger validation fails. | Closing balances are unreliable. | Preserve existing history, correct through governed Accounting behaviour, and regenerate validation evidence. | Accounting & General Ledger |
| Cross-domain blocker lacks a canonical owner or resolution. | Year-end consistency cannot be established. | Retain the exception, assign governance ownership, and block closure. | Enterprise Governance and affected domains |
| Carry-forward or opening balances are duplicated. | Next-year balances may be overstated. | Reject duplicate outcome using immutable correlation/evidence and preserve the single authoritative transfer. | Accounting & General Ledger |
| Approval is absent, ambiguous, expired, or untraceable. | Closure lacks executive authority. | Keep `Closing` and obtain a valid immutable governance outcome. | Enterprise Governance |
| Concurrent year-end change invalidates the evaluated position. | Approval or closure may rely on stale balances. | Re-establish the authoritative version/position and reperform impacted validation. | Accounting & General Ledger |
| State change succeeds but event publication is not confirmed. | Consumers may not recognize closure. | Preserve committed `Closed` state and recover publication without repeating balances or closure. | Accounting & General Ledger |
| Direct modification of closed-year records is attempted. | Immutability and auditability would be violated. | Reject the modification and route legitimate correction through governed period/future-year compensating behaviour. | Accounting & General Ledger |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Fiscal Year is the highest Accounting time boundary and governs annual closing, carry-forward, opening balances, and historical fiscal records. | Accounting & General Ledger | Year-end closure belongs primarily to Accounting. | `L2-F05`, Fiscal Year Aggregate |
| Only one Fiscal Year may remain active per Ledger. | Accounting & General Ledger | Annual close must preserve a single active-year boundary. | `L2-F05`, Fiscal Year invariant / BR-036 |
| Fiscal Year closing requires all Accounting Periods to be closed. | Accounting & General Ledger | Any open period blocks year closure. | `L2-F05`, Fiscal Year invariant / BR-037 |
| Fiscal Year lifecycle is `Planned -> Open -> Closing -> Closed -> Archived`. | Accounting & General Ledger | Closure may not skip mandatory states. | `L2-F05`, Fiscal Year Lifecycle |
| Opening balances derive from prior closing balances. | Accounting & General Ledger | Next-year opening position must exactly trace to the closed year. | `L2-F05`, Fiscal Year Invariants |
| Carry-forward balances and opening balances are generated by governed Accounting behaviour. | Accounting & General Ledger | Manual balance creation or direct Ledger editing is prohibited. | `L2-F05`, BR-038 and BR-039 |
| Fiscal closure requires executive approval and independent authorization. | Enterprise Governance / Accounting | Closure cannot be self-authorized by the preparer. | `L2-F05`, Fiscal Year Governance and Segregation of Duties |
| Year Closing is irreversible after governance approval. | Accounting & General Ledger | Historical year facts cannot be reopened or rewritten through an undefined local path. | `L2-F05`, Fiscal Year Closing Workflow |
| Closed Fiscal Years are immutable and read-only. | Accounting & General Ledger | New posting or modification is prohibited after closure. | `L2-F05`, Fiscal Year Invariants / BR-040 |
| Historical records, Journals, Ledger balances, and audit history remain protected. | Accounting & General Ledger | Corrections cannot overwrite the closed year's evidence. | `L2-F05`, Accounting immutability and data-protection rules |
| Financial Period & Closing owns Financial Period state; it does not create Journals or modify Ledger balances. | Financial Period & Closing | Accounting consumes period closure facts without transferring period ownership. | `L2-F09`, Scope and Ownership Constraints |
| Events represent completed business facts and preserve audit traceability. | Accounting & General Ledger | Closing, carry-forward, and opening-balance events publish only after successful outcomes. | `L2-F05`, Fiscal Year Events and Audit Rules |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Fiscal Year Aggregate | `Open` | Initiate governed year-end after all periods close. | `Closing` | Accounting & General Ledger | `L2-F05`, Fiscal Year Aggregate/Lifecycle |
| Fiscal Year Aggregate | `Closing` | Complete validation, entries, carry-forward, opening balances, and approval. | `Closed` | Accounting & General Ledger | `L2-F05`, Fiscal Year Workflow/Invariants |
| Accounting Periods | `Closed` required | Supply immutable closure evidence; are not modified by Fiscal Year aggregate. | Remain `Closed`/source-owned protected states | Accounting & General Ledger | `L2-F05`, Accounting Period and Fiscal Year rules |
| Financial Periods | `Closed` under period governance | Supply corresponding period-governance evidence; remain externally owned. | Remain `Closed` | Financial Period & Closing | `L2-F09` |
| Closing balances | Authoritative final Ledger position | Determined and preserved as the source for carry-forward. | Immutable prior-year closing position | Accounting & General Ledger | `L2-F05`, Fiscal Year Invariants |
| Next-year opening balances | Not yet generated/current proposal | Generated from prior closing balances after governed year-end behaviour. | Authoritative opening-balance position | Accounting & General Ledger | `L2-F05`, BR-038/BR-039 and Workflow |
| Fiscal Year archival | `Closed` after later retention approval | Not performed in this scenario. | Remains `Closed`; later `Archived` | Accounting & General Ledger | `L2-F05`, Fiscal Year Lifecycle |

`L2-F09` defines Financial Year as an internal Financial Period entity, while `L2-F05` defines a separate Fiscal Year aggregate. This ownership conflict remains in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AccountingPeriodClosed` | Accounting & General Ledger | Fiscal Year workflow, Reporting, Audit | Confirms an Accounting Period is closed. | `L2-F05`, Accounting Period Events |
| `FinancialPeriodClosed` | Financial Period & Closing | Accounting, Finance domains, Reporting, Audit | Confirms the governed Financial Period Closed state. | `L2-F09`, Event Catalogue |
| `FiscalYearClosingStarted` | Accounting & General Ledger | Governance, Reporting, Audit, Finance participants | Announces initiation of year-end processing. | `L2-F05`, Fiscal Year Events |
| `JournalPosted` / `LedgerUpdated` / Trial Balance events | Accounting & General Ledger | Fiscal Year workflow, Reporting, Audit | Supply completed Accounting facts required by year-end validation. | `L2-F05`, Domain Event Catalogue |
| `CarryForwardCompleted` | Accounting & General Ledger | Next-year Accounting, Reporting, Audit | Confirms successful transfer of closing balances. | `L2-F05`, Fiscal Year Events |
| `OpeningBalancesGenerated` | Accounting & General Ledger | Next-year Accounting, Reporting, Audit | Confirms creation of next-year opening balances. | `L2-F05`, Fiscal Year Events |
| `FiscalYearClosed` | Accounting & General Ledger | Financial Period & Closing, Finance domains, Reporting, Audit, Governance | Confirms successful immutable year closure. | `L2-F05`, Fiscal Year Events |

No canonical FiscalYearCloseBlocked, FiscalYearValidationCompleted, FiscalYearApprovalGranted/Rejected, year-close-invalidated, or carry-forward-reversed event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Financial Period & Closing | Accounting & General Ledger | Complete Financial Period population and immutable `FinancialPeriodClosed` facts | Supports proof that every governed period is closed. | `L2-F09`; exact year-level contract absent |
| Accounting Period / GL / Trial Balance / Reconciliation | Fiscal Year workflow | Closed-period, posted-Journal, Ledger, verified-balance, and reconciled-position facts | Supplies authoritative Accounting year-end inputs. | `L2-F05` |
| AR, AP, Treasury, Budget and source owners | Accounting & General Ledger | Completed year-end positions and resolved exception facts | Supports cross-domain Accounting consistency without ownership transfer. | Implied by Finance dependencies; exact contracts incomplete |
| Accounting & General Ledger | Enterprise Governance | Year-end validation, entries, balances, carry-forward, opening balances, exceptions, and approval context | Enables required executive approval. | `L2-F05`, Fiscal Year Governance |
| Enterprise Governance | Accounting & General Ledger | Immutable approved/rejected executive decision | Determines whether Fiscal Year closure may complete. | `L2-F05`, Governance/SoD; exact event absent |
| Accounting & General Ledger | Reporting, Audit, Finance domains | `FiscalYearClosed` plus carry-forward/opening-balance references and audit lineage | Communicates immutable year closure for independent consumption. | `L2-F05`, Fiscal Year Events |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Prepare year-end validation and entries | Accountant | Typical configurable role; no final closure authority. | Accounting & General Ledger |
| Review Journals, balances, reconciliation, and year-end evidence | Senior Accountant / Finance Manager | Typical configurable supervision and financial-control responsibilities. | Accounting & General Ledger |
| Execute Fiscal Year closure | Authorized holder of `Close Fiscal Year` permission | Explicit authorization and all invariant/approval conditions are required. | Accounting & General Ledger / Authorization & Policy |
| Grant year-end approval | Chief Financial Officer / executive approval authority | Source assigns fiscal governance/year-end approval to this typical configurable role and requires executive approval. | Enterprise Governance |
| Review year-end audit evidence | Internal Auditor | Read-only access; cannot modify Accounting records. | Audit |
| Perform technical administration | System Administrator | Must not perform financial posting solely through technical privileges. | Platform Administration / Accounting control |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Year-end preparation versus approval | Accountant/Senior Accountant/Finance Manager and executive approver | Fiscal Year Closure requires independent executive authorization. |
| Journal creation versus approval | Journal Creator and Journal Approver | Creator cannot approve the same Journal. |
| Closing entries versus unauthorized reversal | Journal Approver and reversal authority | Approver cannot execute unauthorized reversals. |
| Fiscal Year closure versus period ownership | Accounting and Financial Period & Closing | Accounting cannot alter Financial Period state; period governance cannot alter Ledger balances. |
| Audit independence | Internal Auditor and Accounting operators | Audit users remain read-only. |
| Technical privilege versus financial authority | System Administrator and financial approvers | Technical access does not grant posting or year-close authority. |
| Override | Executive Governance and Accounting | No override may bypass unclosed periods, unbalanced Ledger, invalid carry-forward, missing opening balances, or immutable history. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove Fiscal Year scope | Fiscal Year identity, Institution, Ledger, Fiscal Calendar, date range, lifecycle, and active-year status | Accounting & General Ledger |
| Prove all periods closed | Complete period population, each AccountingPeriodClosed/FinancialPeriodClosed fact, final-period evidence, and exceptions | Accounting and Financial Period & Closing |
| Prove year-end validation | Pending Journal checks, balance/Trial Balance verification, reconciliation status, audit exceptions, and resolution lineage | Accounting & General Ledger and affected owners |
| Prove closing entries | Journal identity, entries, supporting reason, approval, posting, Ledger effect, and immutable references | Accounting & General Ledger |
| Prove carry-forward/opening | Prior closing balances, transfer mapping, `CarryForwardCompleted`, derived opening balances, and `OpeningBalancesGenerated` | Accounting & General Ledger |
| Prove executive approval | Approval context, authority, outcome, reason, conditions, sequence, and immutable decision evidence | Enterprise Governance |
| Prove closure | Prior `Closing`, resulting `Closed`, invariants, approval, event, and read-only enforcement | Accounting & General Ledger |
| Prove historical protection | Immutable Journals/Ledgers/events, report/audit access, and absence of direct modification after close | Accounting, Reporting, and Audit |

---

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- **Single active year:** Exactly one Fiscal Year is active per Ledger before transition; closure does not create overlapping active years.
- **Complete period closure:** Every Accounting Period in the year has authoritative Closed evidence.
- **Balanced Accounting:** Journals, Ledger, Trial Balance, closing entries, and year-end balances are valid and balanced.
- **Reconciliation and audit clearance:** No unresolved blocking reconciliation or audit exception remains.
- **Governed closing entries:** Every year-end entry follows the Journal validation, approval, posting, and traceability rules.
- **Carry-forward integrity:** `CarryForwardCompleted` maps exactly from authoritative prior closing balances without duplication.
- **Opening-balance integrity:** Next-year opening balances derive from prior closing balances and pass Accounting validation.
- **Executive approval:** A valid independent executive outcome exists before `Closing -> Closed`.
- **Immutable closure:** Closed Fiscal Year is read-only and historical records remain unaltered and reconstructable.
- **Event integrity:** `FiscalYearClosingStarted`, carry-forward, opening-balance, and `FiscalYearClosed` facts follow successful business outcomes.
- **Ownership preservation:** Period, Accounting, AR, AP, Treasury, Budget, Reporting, Governance, and Audit concepts remain with their canonical owners.
- **Failure containment:** Any failed prerequisite leaves the Fiscal Year unclosed and preserves the prior valid Accounting position.
---

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- Single active year acceptance evidence is recorded and auditable.
- Complete period closure acceptance evidence is recorded and auditable.
- Balanced Accounting acceptance evidence is recorded and auditable.
- Reconciliation and audit clearance acceptance evidence is recorded and auditable.
- Governed closing entries acceptance evidence is recorded and auditable.
- Carry-forward integrity acceptance evidence is recorded and auditable.
- Opening-balance integrity acceptance evidence is recorded and auditable.
- Executive approval acceptance evidence is recorded and auditable.
- Immutable closure acceptance evidence is recorded and auditable.
- Event integrity acceptance evidence is recorded and auditable.
- Ownership preservation acceptance evidence is recorded and auditable.
- Failure containment acceptance evidence is recorded and auditable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Conflict | Architectural Impact | Required Upstream Decision / Owner |
|---|---|---|---|
| F053-01 | `L2-F05` defines Fiscal Year as an independent Accounting aggregate, while `L2-F09` defines Financial Year as an internal entity owned by each Financial Period aggregate. | Canonical ownership, identity, lifecycle, and event authority conflict across locked Finance documents. | Finance Architecture Governance must establish one canonical year model and explicit cross-domain contract. |
| F053-02 | Fiscal Year and Financial Year terminology is used without an explicit equivalence or distinction. | The scenario may close a different business boundary than the period architecture references. | Finance Foundation and Accounting/Period owners must define canonical terminology. |
| F053-03 | `L2-F05` requires all Accounting Periods closed, while `L2-F09` separately owns Financial Period closure; mapping between Accounting Period and Financial Period is not defined. | Period-completeness proof and ownership may be duplicated or contradictory. | Accounting and Financial Period architecture must define identity/reference mapping and precedence. |
| F053-04 | Year-end workflow orders closing entries, carry-forward, opening balances, and Fiscal Year lock/close, but exact lifecycle transition points and all-or-none business completion are not specified. | Partial year-end completion could leave balances and lifecycle inconsistent. | Accounting architecture must define state/transaction boundaries and recovery semantics. |
| F053-05 | â€œYear Closing is irreversible after governance approval,â€ but no governed correction model is defined for a material error discovered after Fiscal Year closure. | Legitimate correction may conflict with immutable closure. | Accounting, Financial Period, Audit, and Governance must define future-period compensation or exceptional recovery policy. |
| F053-06 | The relationship between FiscalYearClosed, opening-balance generation, carry-forward completion, and next Fiscal Year activation is not fully ordered by invariant. | Next-year processing may begin before authoritative prior-year completion. | Accounting architecture must define mandatory event/state ordering. |
| F053-07 | No detailed year-end validation standard defines required Trial Balance, reconciliation, audit, source-domain, statutory, tax, grant, fund, or multi-ledger checks. | Closure completeness cannot be consistently evaluated for higher-education institutions. | Accounting, Compliance, Audit, Tax, Grants/Funds, and Finance Governance must define the checklist. |
| F053-08 | Only one active Fiscal Year per Ledger is required, but the Ledger-to-Institution/organisation/multi-campus year model and parallel-book cases are not defined. | Active-year enforcement may be wrong across multiple ledgers or reporting bases. | Accounting and Institution architecture must define scope and multiplicity. |
| F053-09 | `Close Fiscal Year` permission and typical roles exist, but exact executive authority, hierarchy, delegation, value/risk thresholds, and approval evidence are configurable/undefined. | Authorization and separation-of-duty acceptance remain incomplete. | Enterprise Governance and Accounting must define the authority matrix. |
| F053-10 | Cross-domain year-end readiness facts for AR, AP, Treasury, Budget, grants, assets, payroll, tax, and other sources are not defined in `L2-F05`. | Accounting may close using incomplete operational positions. | Cross-domain Finance Architecture must define mandatory year-end contracts. |
| F053-11 | Carry-forward mapping, account eligibility, retained earnings/fund balance treatment, temporary/permanent account rules, dimensions, currencies, and rounding are not canonically specified. | Generated closing/opening balances may be financially incorrect. | Accounting Domain Architecture and Finance Policy must define carry-forward rules. |
| F053-12 | No canonical events exist for validation completion, blocked year close, approval granted/rejected, closing-entry completion, or closure invalidation. | Workflow observability and audit sequencing are incomplete. | Accounting and Enterprise Event Governance must complete the event catalogue. |
| F053-13 | Closed Fiscal Years are read-only, but archival timing, retention duration, legal hold, privacy interaction, disposal, and historical-access authority are not fully defined. | Immutable historical governance cannot be operationally or legally completed. | Records Governance, Legal/Compliance, Privacy, Audit, and Accounting must define retention policy. |
| F053-14 | No rule defines late-arriving transactions or facts received after final period/year close. | Institutions may inconsistently reopen periods, post to the next year, or ignore valid facts. | Accounting, Financial Period, and source-domain owners must define late-fact policy. |
| F053-15 | No explicit business rules for simultaneous changes and repeated requests protect repeated carry-forward, opening-balance generation, or duplicate FiscalYearClosed attempts at the business level. | Year-end balances may be duplicated or contradictory. | Accounting architecture must define authoritative business references and repeat-safe outcomes. |
| F053-16 | No measurable targets exist for year-end validation, exception resolution, closing entries, carry-forward, opening balances, approval, closure, or audit availability. | Operational control cannot be assessed against canonical expectations. | Business Governance must define measurable objectives without weakening correctness. |

This scenario remains **Draft** until the required upstream decisions are incorporated into the locked architecture and the scenario is revalidated.

## 17.2 Assumptions

- The scenario operates under the current locked L1/L2 business architecture baseline.
- All external references to institutional policies, organizational units, and accounts are authoritative.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-053**
