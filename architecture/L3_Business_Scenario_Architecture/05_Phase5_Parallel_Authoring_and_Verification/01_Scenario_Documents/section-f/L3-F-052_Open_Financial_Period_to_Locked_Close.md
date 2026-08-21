# Open Financial Period to Locked Close

# 1. Scenario Metadata

**Scenario ID:** L3-F-052  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Financial Period & Closing  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-B01_Institution_Workflow.md`; `L2-B02_Organisation_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; relevant Budget and Reporting architecture; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs the controlled close of an Open Financial Period through operational lock, domain readiness collection, enterprise readiness evaluation, deterministic closing validation, external governance approval, and transition to `Closed`. It protects the period from new financial activity during close preparation while preserving domain autonomy, immutable financial history, auditability, and the Financial Period aggregate's sole ownership of lifecycle state.

---

# 3. Business Outcome

The selected Open Financial Period has been placed under a governed lock, participating Finance-domain readiness facts have been evaluated, closing validation and governance approval have succeeded, and the period has entered `Closed`. New financial activity is prohibited according to participating-domain policies, historical information remains queryable and auditable, and `FinancialPeriodClosed` communicates immutable historical governance without closing or modifying neighbouring aggregates.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative `Open` Financial Period reaches its governed closing milestone and a valid request is made to restrict new period activity and begin controlled close preparation.

## 4.2 End Boundary

The scenario ends when the Financial Period either: completes `Open -> Locked -> Closing Validation -> Closed` with successful readiness, validation, approval, version advancement, and event publication; or remains in its last valid lifecycle state with a blocked, pending, review-required, rejected, or failed outcome fully preserved.

## 4.3 In Scope

- Identification of the Financial Period, Institution, Organisation context, Financial Year, Fiscal Calendar, date range, lifecycle, lock state, and version.
- Evaluation of lock authority, lock policy, lock reason, lock conditions, and closing schedule.
- Transition from `Open` to `Locked` and publication of the completed lock fact.
- Communication of operational restrictions to participating Finance domains.
- Preservation of historical access while locked.
- Collection of domain-owned close-readiness facts from General Ledger, Accounts Receivable, Accounts Payable, Treasury, and Budget Management.
- Financial Period-owned evaluation of enterprise Closing Readiness.
- Transition from `Locked` to `Closing Validation` after closing request and validation initiation.
- Deterministic, repeatable closing validation and preservation of blocking conditions.
- External governance approval or rejection.
- Transition from `Closing Validation` to `Closed` only after readiness, validation, approval, policies, and invariants succeed.
- Publication and audit preservation of lock, readiness, validation, approval, and closure facts.

## 4.4 Out of Scope

- Closing Journals, Receivables, Payables, Treasury operations, budgets, bank activity, or other neighbouring aggregates.
- Creating, posting, reversing, settling, reconciling, adjusting, or reporting a financial transaction.
- Performing each participating domain's internal reconciliation or readiness work.
- Financial Year closure, which belongs to `L3-F-053`.
- Closed-period reopening and reclosure, which belongs to `L3-F-054`.
- Financial Period archival.
- Destructive modification of historical financial, governance, approval, readiness, validation, or event evidence.
- Lower-level orchestration, storage, communication, timing, or execution design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-051` Financial Year and Period Establishment to Open Processing Window | Establishes the `Open` Financial Period that enters this close path. |
| Related | `L3-F-031` Ledger Period to Controlled Accounting Close | General Ledger independently performs its Accounting close/readiness behaviour. |
| Related | `L3-F-049` Payable Population to Reconciled Period Outcome | Supplies Accounts Payable-owned close-readiness evidence. |
| Precedes | `L3-F-053` Financial Year End to Immutable Year Closure | Period close outcomes contribute to Financial Year-end governance. |
| Related | `L3-F-054` Closed Period to Controlled Reopening and Reclosure | Governs exceptional correction after this scenario's Closed outcome. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Period & Closing / Section F | Solely owns Financial Period identity, lifecycle, status, lock state/reason/history, Closing Readiness conclusion, Closing Validation, aggregate version, and period events. |
| Enterprise Governance | Owns approval workflow, hierarchy, delegated authority, escalation, exception decision, and immutable approval outcome. |
| Authorization & Policy / Section A | Owns authorization, policy evaluation, access decisions, and governance authority; the period aggregate consumes outcomes only. |
| Accounting & General Ledger / Section F | Owns Journals, Ledgers, Accounting close/reconciliation, and `GeneralLedgerReadyForClose` / `GeneralLedgerCloseBlocked`. |
| Accounts Receivable / Section F | Owns Receivables and its readiness determination/facts. |
| Accounts Payable / Section F | Owns Payables and its readiness determination/facts. |
| Treasury & Banking / Section F | Owns payments, settlements, bank operations, reconciliations, and its readiness determination/facts. |
| Budget Management / Section F | Owns budgets and its readiness determination/facts. |
| Financial Reporting | Consumes governance facts and produces read-only period/close views. |
| Audit | Reviews immutable lifecycle, readiness, validation, approval, and closure evidence without changing state. |
| Institution and Organisation Management | Retain ownership of institutional/organisational context referenced by the period. |

Financial Period & Closing coordinates enterprise governance only. It never executes or closes the business processes owned by participating Finance domains.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F09` defines no canonical actors for requesting a lock/close, applying a lock, submitting or reviewing readiness, performing validation, granting approval, closing the period, or authorizing exceptions. Governance committees and financial administration responsibilities are configuration examples rather than canonical roles. Domain and authority gaps are therefore recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- One authoritative Financial Period aggregate exists in `Open` for exactly one Institution and one Financial Year.
- The Institution, Organisation reference where applicable, Fiscal Calendar, period date range, status, lock state, version, and event history are available.
- The closing schedule/milestone and requested close scope are identifiable.
- The Financial Period is not already Locked, in Closing Validation, Closed, Reopened, or Archived.
- Lock authority, institutional lock policy, and a traceable lock reason can be evaluated.
- General Ledger, Accounts Receivable, Accounts Payable, Treasury, and Budget domains can determine and publish their own readiness facts.
- Authorization and governance approval outcomes can be obtained and recorded.
- Historical financial activity remains accessible for readiness, validation, reporting, and audit.
- No proposed action would bypass lifecycle order, institutional isolation, aggregate invariants, or ownership boundaries.

---

# 8. Trigger

The Financial Period reaches its institution-defined closing schedule or receives a governed close request requiring the Open processing window to be restricted and evaluated for closure.

The exact canonical close-request event, lock-request event, initiator, and mandatory trigger evidence are not defined upstream and remain open in Section 17.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Financial Period, Institution, Financial Year, Fiscal Calendar, date range, `Open` state, current lock state, closing schedule, and aggregate version. | Financial Period & Closing | The exact close candidate and authoritative baseline are established. |
| 2 | Establish the governed close request, lock reason, requested restrictions, policy basis, and required authority. | Financial Period & Closing and Enterprise Governance | A traceable close-preparation context exists. |
| 3 | Validate `Open -> Locked` guards: lock authority, lifecycle validity, institutional policy, aggregate consistency, and absence of conflicting period action. | Financial Period & Closing / Authorization & Policy | Invalid or unauthorized locking is rejected before state change. |
| 4 | Transition the Financial Period to `Locked`, preserve lock reason/authority/history, advance version, and publish `FinancialPeriodLocked` / `LockStateChanged` after success. | Financial Period & Closing | Operational restrictions are authoritative and financial history remains accessible. |
| 5 | Participating Finance domains consume the lock fact and reject new activity for the affected period according to their own business policies while completing owned close preparation. | GL, AR, AP, Treasury, Budget, and other Finance owners | The period is protected without the period aggregate modifying external business state. |
| 6 | Each required domain reconciles/validates its owned position and publishes its ready-for-close or close-blocked fact with period correlation. | Participating Finance domains | Domain-owned readiness inputs become available. |
| 7 | Collect the required current readiness facts and validate their period, Institution, ownership, completeness, ordering, version, and traceability. | Financial Period & Closing | Only authoritative, in-scope readiness inputs enter evaluation. |
| 8 | Evaluate enterprise Closing Readiness and classify the conclusion as Ready, Blocked, Pending, or Requires Review under institutional policy. | Financial Period & Closing | The aggregate-owned readiness conclusion is established without changing neighbouring domains. |
| 9 | Publish `ClosingReadinessEvaluated` with the completed conclusion and retain all contributing facts and exceptions. | Financial Period & Closing | Readiness evaluation becomes immutable and auditable. |
| 10 | When closing is requested and lock remains active, transition from `Locked` to `Closing Validation` and initiate deterministic, repeatable validation. | Financial Period & Closing | The mandatory validation stage begins in lifecycle order. |
| 11 | Validate enterprise readiness, aggregate invariants, institutional policy, governance requirements, traceability, historical integrity, and all configured mandatory checks. | Financial Period & Closing | Validation succeeds, fails, or requires review with blocking evidence. |
| 12 | Publish `ClosingValidationCompleted` and obtain the externally owned governance approval or rejection outcome. | Financial Period & Closing and Enterprise Governance | Required validation and approval evidence is complete. |
| 13 | If readiness is achieved, validation succeeds, and governance approval is granted, transition `Closing Validation -> Closed`, advance version, and preserve historical governance. | Financial Period & Closing | The Financial Period reaches its canonical Closed state. |
| 14 | Publish `FinancialPeriodClosed`; participating domains independently apply their own closure restrictions and Reporting/Audit retain read-only historical access. | Financial Period & Closing and consumers | The locked close is completed without transfer of external aggregate ownership. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Lock rejected | Authority, lifecycle, policy, or aggregate validation fails. | Preserve rejection/blocking evidence and keep the period `Open`. | No unauthorized processing freeze occurs. | Financial Period & Closing / Governance |
| Emergency lock | Institutional emergency-lock policy is invoked. | Validate emergency authority and conditions, apply the governed lock, preserve exception evidence, and continue readiness/validation only under defined policy. | Financial Period & Closing and Enterprise Governance |
| Readiness pending | Required domain facts are not yet available or current. | Retain `Locked`, classify Pending, and wait for authoritative facts without closing external domains. | Financial Period & Closing and missing domain owner |
| Close blocked | One or more domains publish a close-blocked fact or validation finds a blocking condition. | Retain the valid Locked/Closing Validation state, preserve exceptions, route resolution to canonical owners, and re-evaluate after new facts. | Financial Period & Closing and affected domains |
| Requires review | Policy identifies ambiguity, exception, or governance review requirement. | Preserve the current state and route the evidence to configured governance authority. | Enterprise Governance and Financial Period & Closing |
| Governance rejection | Closing approval is denied. | Preserve rejection and keep the period in its source-defined current state; do not publish `FinancialPeriodClosed`. | Enterprise Governance and Financial Period & Closing |
| Lock release before validation | Close attempt is cancelled or policy permits operational resumption before closure. | Apply governed lock-removal behaviour and restore permitted processing only through an explicitly valid transition. | Financial Period & Closing |
| New domain fact after Ready | A readiness input becomes invalid before closure. | Invalidate/re-evaluate the conclusion under governed rules, retain the period unclosed, and obtain current domain facts. | Financial Period & Closing and fact owner |
| Successful locked close | All readiness, validation, approval, policy, and invariant conditions succeed. | Complete the full lifecycle sequence and publish `FinancialPeriodClosed`. | Financial Period & Closing |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Period identity, Institution, Financial Year, or lifecycle is inconsistent. | Lock/close could affect the wrong accounting window. | Reject the attempt and correct governance context through the canonical owner before retry. | Financial Period & Closing and context owner |
| Lock authority or reason is missing, ambiguous, or untraceable. | Operational restriction lacks governance legitimacy. | Keep `Open` and obtain valid authority/evidence. | Enterprise Governance / Authorization & Policy |
| A participating domain continues accepting prohibited new activity after lock. | Readiness and close validation may become stale or incorrect. | Contain the activity through the owning domain, preserve the breach, refresh readiness, and block closure until resolved. | Participating Finance domain and Governance |
| Readiness facts are missing, duplicated, contradictory, stale, or for another period. | Enterprise readiness cannot be determined reliably. | Classify Pending/Blocked, obtain authoritative corrected facts, and re-evaluate. | Financial Period & Closing and fact owners |
| Closing validation is not deterministic or repeatable. | Closure evidence cannot be trusted or audited. | Retain the period unclosed, correct the authoritative validation inputs/rules, and perform a fresh validation. | Financial Period & Closing |
| Approval outcome is absent, expired, rejected, or untraceable. | `Closing Validation -> Closed` guard fails. | Preserve current state and obtain a valid immutable governance outcome. | Enterprise Governance |
| Concurrent aggregate change advances the version. | Lock, readiness, validation, or close attempt is stale. | Re-read the period, revalidate all current facts, and submit a new attempt if eligible. | Financial Period & Closing |
| State transition succeeds but event publication is not confirmed. | Consumers may not enforce the authoritative period state. | Preserve the committed aggregate and recover publication without repeating the transition. | Financial Period & Closing |
| Partial lifecycle progression is attempted. | Aggregate could occupy an invalid or skipped state. | Reject/roll back the failed modification, preserve the previous valid version, and publish no false event. | Financial Period & Closing |
| Closure later proves incorrect. | New financial activity is prohibited despite a legitimate correction need. | Use `L3-F-054` governed reopening/reclosure; never rewrite the original close history. | Financial Period & Closing and Enterprise Governance |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial Period & Closing solely owns period lifecycle, status, lock state, readiness conclusion, validation, version, and period events. | Financial Period & Closing | No neighbouring domain may directly lock or close the period. | `L2-F09`, Aggregate Responsibilities and Ownership Constraints |
| `Open -> Locked` requires verified lock authority, valid lifecycle, and policy permission. | Financial Period & Closing / Governance | A close process cannot begin with an unauthorized or invalid lock. | `L2-F09`, State Transition Rules |
| While locked, participating Finance domains reject new financial activity according to their own policies; historical information remains accessible. | Participating domains / Financial Period governance | Lock restricts processing without destroying or hiding history. | `L2-F09`, Period Lock Management |
| Locking is reversible, protects financial integrity, and preserves lock reason, authority, and history. | Financial Period & Closing | Lock is a governed restriction, not itself completed closure. | `L2-F09`, Period Lock Management and Lock Constraints |
| Neighbouring domains own readiness facts; the Financial Period aggregate alone owns enterprise Closing Readiness conclusion. | Financial Period & Closing and Finance domains | External readiness facts cannot directly modify aggregate readiness. | `L2-F09`, Closing Readiness Invariants |
| Readiness is an evaluation, not the closing operation. | Financial Period & Closing | Ready status cannot skip validation, approval, or lifecycle transitions. | `L2-F09`, Closing Readiness Evaluation |
| `Locked -> Closing Validation` requires active lock, closing request, and validation initiation. | Financial Period & Closing | Mandatory lifecycle stage cannot be skipped. | `L2-F09`, State Transition Rules |
| Closing Validation is deterministic, repeatable, auditable, and occurs before closure. | Financial Period & Closing | Failed validation prevents lifecycle progression. | `L2-F09`, Validation Constraints |
| `Closing Validation -> Closed` requires enterprise readiness, successful validation, and governance approval. | Financial Period & Closing / Enterprise Governance | Every mandatory guard must succeed. | `L2-F09`, State Transition Rules and Closing Constraints |
| Closure prohibits new financial activity, retains query/report/audit access, and never destroys historical facts. | Financial Period & Closing and consumers | Closed state establishes immutable historical governance. | `L2-F09`, Financial Period Closing and Closed Period Constraints |
| Every successful modification advances version; failed operations preserve version and historical correctness. | Financial Period & Closing | Lock, validation, and close outcomes are protected from conflicting simultaneous decisions. | `L2-F09`, Aggregate Versioning and Failure Constraints |
| Domain events represent immutable completed facts and publish only after aggregate consistency. | Financial Period & Closing | Intent or partially completed close steps cannot be published as success. | `L2-F09`, Event and Aggregate Constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Period - lock | `Open` | Apply governed operational restrictions after authority, lifecycle, and policy checks. | `Locked` | Financial Period & Closing | `L2-F09`, State Model and Period Lock Management |
| Lock State | Unlocked/open operational position | Record lock status, reason, authority, time/sequence, history, and version. | Active lock | Financial Period & Closing | `L2-F09`, Period Lock State and Lock Configuration |
| Closing Readiness | Initialized/current conclusion | Evaluate authoritative domain readiness facts. | Ready, Blocked, Pending, or Requires Review | Financial Period & Closing | `L2-F09`, Closing Readiness Evaluation |
| Financial Period - validation | `Locked` | Begin close validation when lock active, closing requested, and validation initiated. | `Closing Validation` | Financial Period & Closing | `L2-F09`, State Transition Rules |
| Closing Validation | Not completed/current | Deterministically evaluate mandatory close prerequisites and preserve outcome. | Successful, Failed, or Requires Review | Financial Period & Closing | `L2-F09`, Event Catalogue and Validation Constraints |
| Financial Period - closure | `Closing Validation` | Apply readiness, validation, approval, policy, and invariant guards. | `Closed` on success; otherwise remains unclosed | Financial Period & Closing | `L2-F09`, Closing Constraints |
| Participating Finance aggregates | Domain-owned operational/close states | Produce readiness facts and independently apply restrictions; are not transitioned by the period aggregate. | Domain-owned states | Respective Finance domains | `L2-F09`, Enterprise Closing Coordination |

The title's term â€œLocked Closeâ€ is not a canonical lifecycle state. This scenario interprets it as the controlled lock-to-Closed path because the next canonical scenario governs Financial Year closure. The ambiguity is recorded in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `FinancialPeriodLocked` | Financial Period & Closing | GL, AR, AP, Treasury, Budget, Reporting, Audit | Announces that operational restrictions are applied while the aggregate remains active. | `L2-F09`, Event Catalogue |
| `LockStateChanged` | Financial Period & Closing | Participating Finance domains, Governance, Reporting, Audit | Communicates the completed lock-state change and governance context. | `L2-F09`, Operational Events |
| `GeneralLedgerReadyForClose` / `GeneralLedgerCloseBlocked` | Accounting & General Ledger | Financial Period & Closing | Supplies Accounting-owned readiness. | `L2-F09`, General Ledger Integration |
| `AccountsReceivableReadyForClose` / `AccountsReceivableCloseBlocked` | Accounts Receivable | Financial Period & Closing | Supplies AR-owned readiness. | `L2-F09`, Accounts Receivable Integration |
| `AccountsPayableReadyForClose` / `AccountsPayableCloseBlocked` | Accounts Payable | Financial Period & Closing | Supplies AP-owned readiness. | `L2-F09`, Accounts Payable Integration |
| `TreasuryReadyForClose` / `TreasuryCloseBlocked` | Treasury & Banking | Financial Period & Closing | Supplies Treasury-owned readiness. | `L2-F09`, Treasury Integration |
| `BudgetReadyForClose` / `BudgetCloseBlocked` | Budget Management | Financial Period & Closing | Supplies Budget-owned readiness. | `L2-F09`, Budget Integration |
| `ClosingReadinessEvaluated` | Financial Period & Closing | Governance, Finance domains, Reporting, Audit | Communicates the aggregate-owned enterprise readiness conclusion. | `L2-F09`, Event Catalogue |
| `ClosingValidationCompleted` | Financial Period & Closing | Enterprise Governance, Finance domains, Reporting, Audit | Communicates successful, failed, or review-required validation. | `L2-F09`, Event Catalogue |
| `GovernanceApprovalGranted` / `GovernanceApprovalRejected` | Financial Period & Closing records externally owned outcome | Finance domains, Reporting, Audit | Records the governed approval decision used by closure. | `L2-F09`, Event Catalogue |
| `FinancialPeriodClosed` | Financial Period & Closing | GL, AR, AP, Treasury, Budget, Reporting, Audit | Announces Closed state and historical governance. | `L2-F09`, Event Catalogue |

No canonical close-requested, lock-requested/rejected, readiness-invalidated, close-blocked, lock-released, or closure-failed event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Enterprise Governance / Authorization & Policy | Financial Period & Closing | Lock authority, policy result, and immutable close approval/rejection | Enables governed lifecycle changes without embedding authority logic. | `L2-F09`, Authorization & Policy Integration |
| Financial Period & Closing | Participating Finance domains | `FinancialPeriodLocked` with period identity, Institution, date range, lock reason/authority, status, version, and correlation | Enables each owner to enforce operational restrictions. | `L2-F09`, Period Lock and Integration Architecture; exact contract incomplete |
| GL, AR, AP, Treasury, Budget | Financial Period & Closing | Ready-for-close or close-blocked fact with period correlation, current business conclusion, exceptions, and evidence reference | Supplies domain-owned facts for aggregate-owned readiness evaluation. | `L2-F09`, Consumed Readiness Facts; exact content incomplete |
| Financial Period & Closing | Enterprise Governance | Readiness and validation outcome with contributing facts, exceptions, policy, and traceability | Supports external approval decision. | `L2-F09`, Governance Integration |
| Financial Period & Closing | Participating domains, Reporting, Audit | `FinancialPeriodClosed` with identity, prior/resulting state, date boundary, version, approval, and history references | Communicates completed close for independent downstream behaviour. | `L2-F09`, Event and Integration Architecture |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Request period lock/close | No canonical actor defined | Must identify the period, reason, schedule, restrictions, and policy basis. | Financial Period & Closing context |
| Authorize normal or emergency lock | No canonical role/hierarchy defined | Requires verified lock authority and institutional policy. | Enterprise Governance / Authorization & Policy |
| Apply `Open -> Locked` | No canonical actor defined | Only the period aggregate may change lifecycle after guards succeed. | Financial Period & Closing |
| Determine domain readiness | No canonical actor defined by `L2-F09` | Each domain owns its factual readiness conclusion and exceptions. | GL, AR, AP, Treasury, Budget owners |
| Evaluate enterprise readiness | No canonical actor defined | Must use authoritative current domain facts and cannot override them. | Financial Period & Closing |
| Perform/review closing validation | No canonical validator/reviewer defined | Must be deterministic, repeatable, auditable, and complete before closure. | Financial Period & Closing |
| Approve/reject closure | No canonical role, committee, matrix, or threshold defined | Governance approval is mandatory and externally owned. | Enterprise Governance |
| Apply `Closing Validation -> Closed` | No canonical actor defined | Requires readiness, validation, approval, policy, and invariants. | Financial Period & Closing |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Lock request versus lock authority | Requesting responsibility and Enterprise Governance | Request does not authorize restriction. |
| Period lock versus domain restriction | Financial Period & Closing and participating domains | Period publishes governance state; each domain enforces its own business restrictions. |
| Domain readiness versus enterprise conclusion | Finance-domain owners and Financial Period & Closing | The aggregate evaluates facts but cannot rewrite domain conclusions. |
| Validation versus approval | Financial Period & Closing and Enterprise Governance | Successful validation does not self-create approval authority. |
| Period close versus domain close | Financial Period & Closing and domain owners | Period closure does not directly close Journals, Receivables, Payables, Treasury, or Budgets. |
| Override/emergency decision | Enterprise Governance and all affected owners | No override may conceal blockers or bypass lifecycle, invariants, immutable facts, validation, or approval. |
| Audit independence | Audit and operational owners | Audit reviews immutable evidence and cannot change period or domain state. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove open baseline | Period identity, Institution, Financial Year, date range, `Open` status, lock state, version, and prior events | Financial Period & Closing |
| Prove lock | Request, reason, authority, policy, conditions, previous/resulting state, version, `FinancialPeriodLocked`, and `LockStateChanged` | Financial Period & Closing and Governance |
| Prove domain restrictions/readiness | Each domain's restriction response, reconciliation/validation evidence, readiness fact, blocker, period correlation, and version/effective boundary | Respective Finance domain |
| Prove readiness evaluation | Required fact set, completeness/currentness checks, exceptions, classification, policy, conclusion, and `ClosingReadinessEvaluated` | Financial Period & Closing |
| Prove closing validation | Inputs, deterministic rules, validation run/result, blockers, reviewer evidence, and `ClosingValidationCompleted` | Financial Period & Closing |
| Prove approval | Approval context, authority, result, reason, conditions, sequence, and immutable granted/rejected fact | Enterprise Governance |
| Prove closure | Prior `Closing Validation`, resulting `Closed`, invariants, version, `FinancialPeriodClosed`, and downstream availability | Financial Period & Closing |
| Prove history/access | Immutable governance/financial history plus query, report, and audit accessibility after close | Financial Period & Closing, domain owners, Reporting, Audit |

---

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- **Correct period scope:** One immutable Financial Period identity, Institution, Financial Year, date range, lifecycle, and version are used throughout.
- **Authorized lock:** `Open -> Locked` occurs only under verified authority, valid lifecycle, and permitted policy.
- **Processing restriction:** Participating domains demonstrate rejection/containment of prohibited new activity after lock.
- **Historical accessibility:** Locked and Closed states retain queryable, reportable, auditable, reconstructable history.
- **Readiness completeness:** Every mandatory domain has supplied one current authoritative ready/blocked fact or the conclusion remains Pending/Blocked.
- **Readiness ownership:** Domain facts remain externally owned; the period aggregate alone records enterprise conclusion.
- **Mandatory lifecycle sequence:** No transition skips `Locked` or `Closing Validation`.
- **Deterministic validation:** Repeated validation against the same authoritative facts produces the same result and evidence.
- **Governance approval:** Closure has a valid immutable approval after successful readiness and validation.
- **Atomic closure:** Successful close advances version and publishes `FinancialPeriodClosed`; failure preserves the prior valid state/version.
- **Domain autonomy:** Period closure does not directly modify or claim closure of neighbouring aggregates.
- **Audit reconstruction:** Lock, readiness, validation, approval, closure, exceptions, versions, and events are fully reconstructable.
---

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- Correct period scope acceptance evidence is recorded and auditable.
- Authorized lock acceptance evidence is recorded and auditable.
- Processing restriction acceptance evidence is recorded and auditable.
- Historical accessibility acceptance evidence is recorded and auditable.
- Readiness completeness acceptance evidence is recorded and auditable.
- Readiness ownership acceptance evidence is recorded and auditable.
- Mandatory lifecycle sequence acceptance evidence is recorded and auditable.
- Deterministic validation acceptance evidence is recorded and auditable.
- Governance approval acceptance evidence is recorded and auditable.
- Atomic closure acceptance evidence is recorded and auditable.
- Domain autonomy acceptance evidence is recorded and auditable.
- Audit reconstruction acceptance evidence is recorded and auditable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Conflict | Architectural Impact | Required Upstream Decision / Owner |
|---|---|---|---|
| F052-01 | â€œLocked Closeâ€ is not a canonical lifecycle state; the source defines separate `Locked`, `Closing Validation`, and `Closed` states. | The scenario's intended end boundary could be interpreted as Locked or Closed. | Scenario Governance and Financial Period architecture must confirm the canonical outcome; this Draft uses Closed based on sequence and F-053. |
| F052-02 | The integration overview shows readiness facts leading to approval and closure but omits the mandatory lock and Closing Validation lifecycle stages. | Cross-domain orchestration conflicts with the aggregate state model. | Financial Period architecture must align integration flow, events, and lifecycle transitions. |
| F052-03 | No canonical lock/close request event, initiator, mandatory reason structure, or request lifecycle is defined. | Start boundary and audit evidence cannot be standardized. | Financial Period and Enterprise Event Governance must define the request contract. |
| F052-04 | Lock authority, conditions, duration, emergency procedure, and release are configurable but no canonical role or invariant floor is defined. | Institutions may apply inconsistent or unsafe operational restrictions. | Enterprise Governance and Financial Period & Closing must define lock policy and authority. |
| F052-05 | The source says participating domains reject new activity â€œaccording to their own business policies,â€ but mandatory cross-domain minimum restrictions and treatment of in-flight activity are absent. | A locked period may continue changing differently across domains. | Cross-domain Finance architecture must define lock-effect contracts and cut-off behaviour. |
| F052-06 | No explicit `Locked -> Open` transition or lock-release guard exists despite Lock Management owning lock release and locking being described as reversible. | Cancelled/failed close cannot safely restore processing. | Financial Period architecture must define lock release and lifecycle restoration. |
| F052-07 | Readiness outcomes include Ready, Blocked, Pending, and Requires Review, but their definitions, materiality, precedence, and transition effects are not defined. | Identical fact sets may produce different enterprise conclusions. | Financial Period & Closing with Finance Governance must define deterministic readiness policy. |
| F052-08 | Required readiness domains are described as illustrative rather than mandatory and no dynamic applicability model exists. | Readiness completeness cannot be objectively proven. | Enterprise Finance Governance must define required participants by institution/period context. |
| F052-09 | Domain readiness facts are named only in `L2-F09`; exact content, ownership alignment with source-domain catalogues, validity period, version, withdrawal, and correction semantics are absent. | Stale or contradictory facts may be accepted. | Enterprise Event Governance and each Finance domain must define canonical contracts. |
| F052-10 | No rule defines invalidation/re-evaluation when new financial activity or a revised readiness fact arrives after Ready but before Closed. | Closure may rely on stale enterprise readiness. | Financial Period & Closing must define readiness validity and re-evaluation triggers. |
| F052-11 | Closing Validation mandatory checks, source facts, exception taxonomy, repeatability boundary, and reviewer authority are not defined. | Validation cannot be implemented or audited consistently. | Financial Period, Accounting, Audit, and Enterprise Governance must define the validation standard. |
| F052-12 | `GovernanceApprovalGranted/Rejected` is published by the period event catalogue while approval mechanisms remain externally owned; publisher and evidence ownership are unclear. | Governance outcome may be duplicated or misattributed. | Authorization/Governance and Financial Period owners must define event ownership and recording contract. |
| F052-13 | No canonical actors, preparer/reviewer, domain readiness authority, validator, approver, close authority, delegation, or separation-of-duty rules are defined. | Authorization and audit acceptance cannot be completed. | Enterprise Governance with participating Finance domains must define the role model. |
| F052-14 | The exact relationship between Financial Period closure and independent GL, AR, AP, Treasury, and Budget close/operational states is not defined. | The period may close while a domain remains operational or blocked. | Cross-domain Finance architecture must define required domain outcomes and ordering. |
| F052-15 | No closed-period handling is defined for late-arriving facts discovered immediately after closure; reopening and future-period compensation choice is not specified here. | Financial corrections may be inconsistent across domains. | Financial Period, Accounting, and affected domain owners must define recovery routing to F-054 or compensating paths. |
| F052-16 | No measurable targets exist for lock propagation, domain response, readiness submission, exception resolution, validation, approval, closure, event recognition, or audit availability. | Operational controls cannot be assessed against canonical expectations. | Business Governance must define measurable objectives without weakening correctness. |

This scenario remains **Draft** until the required upstream decisions are incorporated into the locked architecture and the scenario is revalidated.

## 17.2 Assumptions

- The scenario operates under the current locked L1/L2 business architecture baseline.
- All external references to institutional policies, organizational units, and accounts are authoritative.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-052**
