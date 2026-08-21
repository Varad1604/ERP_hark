# Financial Year and Period Establishment to Open Processing Window

# 1. Scenario Metadata

**Scenario ID:** L3-F-051  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Financial Period & Closing  
**Participating Domains:** B, F  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-A02_Person_and_Identity_Workflows.md`; `L2-A03_Authentication_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-B01_Institution_Workflow.md`; `L2-B02_Organisation_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs establishment of the Financial Year context and its Financial Periods, followed by the controlled transition of an eligible prepared period into an open processing window. It ensures that institutional identity, fiscal calendar, date range, Financial Year association, predecessor requirements, policy, governance approval, aggregate invariants, and publication conditions are satisfied before participating Finance domains may independently accept financial activity.

---

# 3. Business Outcome

An institution has a valid Financial Year context and one or more uniquely identified Financial Period aggregates associated with it. The selected period has progressed from `Registered` through `Prepared` to `Open`, `FinancialPeriodOpened` has been published as a completed business fact, and participating Finance domains may apply their own business rules to accept activity within the open window without receiving ownership of the Financial Period.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins after the Institution is established and an authoritative Fiscal Calendar and governed Financial Year context are available for creation of the required accounting-period structure.

## 4.2 End Boundary

The scenario ends when the selected Financial Period successfully enters `Open`, its aggregate version and immutable opening event are established, and participating domains receive the period-governance fact; or when establishment/preparation/opening fails and the period remains in its last valid state with blocking evidence preserved.

## 4.3 In Scope

- Validation of active institutional and organisational context.
- Reference to the authoritative Fiscal Calendar and governed Financial Year context.
- Definition of Financial Year dates and the required Financial Period structure under institutional configuration.
- Registration of each Financial Period aggregate with immutable identity, one Institution, one Financial Year, one accounting-period date range, initial governance state, lock state, closing readiness, and version.
- Association of each period with its governing Financial Year and external Fiscal Calendar.
- Administrative preparation and validation of the selected period.
- Evaluation of predecessor dependencies, institutional opening policy, authorization, and governance approval.
- Transition of the selected period from `Registered` to `Prepared` and from `Prepared` to `Open`.
- Publication of establishment, association, preparation, and opening facts supported by the locked event catalogue.
- Notification of participating Finance domains that they may independently evaluate financial activity for the open period.

## 4.4 Out of Scope

- Creation, activation, suspension, or modification of the Institution or Organisation.
- Ownership or modification of the external Fiscal Calendar definition.
- Creation of Journals, Receivables, Payables, payments, budgets, financial reports, tax calculations, or bank transactions.
- Period locking, closing-readiness evaluation, closing validation, closure, reopening, or archival.
- Direct activation of transaction processing inside neighbouring domains.
- Definition of local Finance-domain posting, receipt, payment, budget, or source-transaction acceptance rules.
- Lower-level calendar calculation, storage, communication, interface, or execution design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Precedes | `L3-F-052` Open Financial Period to Locked Close | Governs the next Financial Period lifecycle stage from the open processing window toward controlled lock. |
| Related | `L3-F-031` Ledger Period to Controlled Accounting Close | Accounting close operates within the Financial Period governance boundary. |
| Related | `L3-F-049` Payable Population to Reconciled Period Outcome | Accounts Payable reconciliation uses the established Financial Period and later publishes close-readiness facts. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Institution Management / Section B | Owns Institution identity, lifecycle, status, and institutional configuration used as the governance boundary. |
| Organisation Management / Section B | Owns Organisation identity, hierarchy, and status referenced by period governance where applicable. |
| Fiscal Calendar Management | Owns calendar definition, fiscal structure, calendar configuration, and fiscal rules; Financial Period & Closing references these facts. |
| Enterprise Governance | Owns governance approval, hierarchy, delegated authority, escalation, and immutable decision outcomes. |
| Authorization & Policy / Section A | Owns user authorization, policy evaluation, access decisions, and approval authority; the period aggregate consumes outcomes only. |
| Financial Period & Closing / Section F | Owns Financial Period identity, Financial Year association/context, date range, lifecycle, status, lock state, closing readiness, validation, version, and period events. |
| Accounting & General Ledger / Section F | Independently determines whether Accounting activity is accepted after consuming the open-period fact. |
| Accounts Receivable / Section F | Independently applies its own receivable-period rules after opening. |
| Accounts Payable / Section F | Independently applies its own payable-period rules after opening. |
| Treasury & Banking / Section F | Independently applies its own Treasury-period rules after opening. |
| Budget Management / Section F | Independently applies its own budget-period rules after opening. |
| Financial Reporting and Audit | Consume immutable period facts for read-only reporting, governance evidence, and audit reconstruction. |

Opening communicates governance state. It does not create transactions, transfer ownership, or command a neighbouring domain to accept a particular business operation.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F09` does not define canonical actors for establishing a Financial Year, registering/preparing a period, validating predecessor dependencies, approving opening, applying the transition, or reviewing the result. It describes financial administration responsibilities and governance committees only as configurable concepts. This Draft therefore assigns responsibilities to canonical domains and records the missing actor and authority model in Section 17.

---

# 7. Preconditions and Entry Criteria

- An authoritative Institution exists, is eligible for Finance governance, and is isolated from other institutions/tenants.
- An authoritative Organisation context exists where the institution's model requires it.
- An authoritative Fiscal Calendar definition and fiscal rules exist for the Institution.
- The proposed Financial Year context has a valid identifier, start/end boundary, and governing Fiscal Calendar reference.
- Financial governance approval required for calendar initialization is available.
- Proposed Financial Period date ranges and classifications are defined under institution configuration.
- Each period will belong to exactly one Institution and one Financial Year and represent exactly one accounting period.
- Duplicate period identity or reuse is absent.
- The selected period's predecessor dependencies and opening policy can be evaluated.
- Authorization and governance decision outcomes can be recorded without embedding authority logic in the aggregate.

---

# 8. Trigger

An institution requires initialization of a new governed Financial Year/period structure or requires an already registered and prepared Financial Period to become operational for participating Finance-domain activity.

The exact canonical event that establishes the Financial Year prerequisite is not defined. `FinancialYearAssigned` and `FiscalCalendarAssociated` are administrative period events rather than a clearly defined Financial Year establishment lifecycle.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Institution, Organisation context where applicable, authoritative Fiscal Calendar, proposed Financial Year boundary, and institutional period configuration. | Institution, Organisation, Fiscal Calendar, and Financial Period & Closing | The governance context for establishment is traceable to canonical owners. |
| 2 | Validate Institution status, calendar authority, Financial Year dates, fiscal rules, institutional configuration, and required financial-governance approval. | Financial Period & Closing with Enterprise Governance | Invalid or unauthorized calendar initialization is stopped. |
| 3 | Establish the governed Financial Year context and determine its required period structure, classifications, and date ranges according to the authoritative calendar. | Financial Period & Closing referencing Fiscal Calendar | The fiscal grouping and proposed accounting-period population are defined. |
| 4 | Validate each proposed period for Institution, Financial Year association, accounting-period identity, date range, classification, and consistency with the fiscal structure. | Financial Period & Closing | Each period candidate is valid for registration or is rejected. |
| 5 | Register each valid Financial Period with immutable aggregate identity, `Registered` state, governance state, lock state, closing-readiness state, version, and required references. | Financial Period & Closing | Canonical Financial Period aggregates become available for governance. |
| 6 | Publish `FinancialPeriodRegistered`, associate the external Fiscal Calendar and governing Financial Year, and preserve `FiscalCalendarAssociated` / `FinancialYearAssigned` facts where applicable. | Financial Period & Closing | Period establishment and association are historically traceable. |
| 7 | Prepare the selected period by validating that the Financial Year is established, its Period Date Range is valid, and aggregate creation/invariants have succeeded. | Financial Period & Closing | The period progresses from `Registered` to `Prepared`. |
| 8 | Publish `FinancialPeriodPrepared` after successful preparation and preserve the current aggregate version and administrative evidence. | Financial Period & Closing | The period becomes eligible for opening-governance evaluation. |
| 9 | Evaluate predecessor dependencies, institutional opening policy, opening schedule, authorization, and governance approval for the selected `Prepared` period. | Financial Period & Closing, Authorization & Policy, and Enterprise Governance | The opening decision is complete and auditable. |
| 10 | If all guards succeed, transition the selected Financial Period from `Prepared` to `Open`, establish its operational status, and advance the aggregate version. | Financial Period & Closing | The open processing window becomes authoritative. |
| 11 | Publish `FinancialPeriodOpened` only after lifecycle, policy, invariants, persistence, version, and transaction success. | Financial Period & Closing | Participating domains receive an immutable completed opening fact. |
| 12 | Each participating Finance domain independently applies its own business rules to determine whether a proposed transaction may be accepted within the open period. | Accounting, AR, AP, Treasury, Budget, and other Finance owners | Opening enables governed processing without transferring transaction ownership. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Multiple periods established | The Financial Year contains monthly, quarterly, semester, or custom institutional periods. | Register and prepare each period separately with unique identity and date range; open only the selected eligible period. | Other periods remain in their own valid lifecycle states. | Financial Period & Closing |
| Registration rejected | Institution, Financial Year, date range, identity, or invariants are invalid. | Preserve rejection evidence and correct the authoritative input before a new registration attempt. | Financial Period & Closing and input owner |
| Preparation blocked | Financial Year establishment, date range, or aggregate creation requirements fail. | Retain `Registered`, resolve the blocking context through its canonical owner, and re-evaluate. | Financial Period & Closing |
| Predecessor incomplete | A predecessor-period dependency has not been satisfied. | Retain `Prepared`, preserve the dependency exception, and do not open until policy-defined completion occurs. | Financial Period & Closing and dependency owner |
| Opening approval rejected | Governance denies the opening decision. | Preserve the rejection and keep the period `Prepared`. | Enterprise Governance and Financial Period & Closing |
| Opening scheduled for later | Policy and approval are satisfied but the configured opening boundary has not arrived. | Retain `Prepared` until the authoritative schedule permits transition. | Financial Period & Closing |
| Participating domain not ready | The period is Open but a Finance domain's own prerequisites fail. | The domain rejects its specific activity under its own rules; the period remains Open unless separate governance changes it. | Participating Finance domain |
| Reopened prior period | An exceptional correction requires a previously Closed period to resume. | Use the separate governed `Closed -> Reopened -> Open` path; do not treat it as initial establishment. | Financial Period & Closing and Enterprise Governance |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Institution or Organisation context is inactive, inconsistent, or missing. | Period establishment would lack a valid enterprise boundary. | Withhold registration/opening until the canonical owner publishes a valid context. | Institution/Organisation owner |
| Fiscal Calendar is missing, contradictory, or changes during establishment. | Financial Year and period dates may be invalid or non-reproducible. | Preserve the attempt, resolve calendar governance, and restart validation from the authoritative version. | Fiscal Calendar Management and Financial Period & Closing |
| Duplicate Financial Period or identity reuse is detected. | Canonical period state and audit lineage would be ambiguous. | Reject registration and preserve the existing authoritative aggregate. | Financial Period & Closing |
| Period date range is invalid or inconsistent with the Financial Year. | Transactions could be assigned to an invalid accounting window. | Retain the last valid state and correct the governed period definition before retry. | Financial Period & Closing |
| Required predecessor dependency cannot be proven. | Opening policy cannot be satisfied. | Keep `Prepared`, record the blocking dependency, and re-evaluate only after authoritative completion. | Financial Period & Closing and dependency owner |
| Authorization or governance outcome is missing, ambiguous, or untraceable. | The opening transition lacks authority. | Do not open; obtain a valid immutable outcome from the canonical owner. | Authorization & Policy / Enterprise Governance |
| Concurrent period change advances aggregate version. | Opening evaluation is based on stale lifecycle or governance facts. | Re-read the aggregate, revalidate guards, and submit a new attempt if still eligible. | Financial Period & Closing |
| Period opens but event publication is not confirmed. | Participating domains may not recognize the open window. | Preserve the committed aggregate and recover publication without reopening or duplicating the transition. | Financial Period & Closing |
| A neighbouring domain accepts activity before `FinancialPeriodOpened`. | Enterprise period governance is bypassed. | Reject or contain the activity through that domain's rules and preserve the governance breach for resolution. | Participating Finance domain and Governance |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Financial Period & Closing is the sole canonical owner of Financial Period identity, status, lifecycle, lock state, closing readiness, validation, version, and period events. | Financial Period & Closing | No neighbouring domain may open or modify period state. | `L2-F09`, Aggregate Responsibilities and Canonical Ownership |
| Financial Period creation requires established Institution, Fiscal Calendar, Financial Year, and financial-governance approval. | Financial Period & Closing with context owners | Registration cannot occur from incomplete or unapproved calendar context. | `L2-F09`, Aggregate Instantiation Rules |
| Every Financial Period has one immutable identity, belongs to one Institution and one Financial Year, represents one accounting period, begins `Registered`, and satisfies invariants. | Financial Period & Closing | Duplicate, cross-institution, or unassociated periods are invalid. | `L2-F09`, Aggregate Creation Constraints |
| A Financial Year may contain multiple periods and the number/duration remain institution configurable. | Financial Period & Closing | Monthly, quarterly, semester, or custom structures are supported without changing ownership. | `L2-F09`, Financial Year Association |
| `Registered -> Prepared` requires established Financial Year, valid Period Date Range, and successful aggregate creation. | Financial Period & Closing | Preparation cannot skip establishment validation. | `L2-F09`, State Transition Rules |
| `Prepared -> Open` requires satisfied predecessor dependencies, governance approval, and institutional policy. | Financial Period & Closing and Enterprise Governance | Opening is prohibited if any guard fails. | `L2-F09`, State Transition Rules |
| Only an Open Financial Period may accept financial activity from participating Finance domains. | Financial Period & Closing / participating domains | Non-open periods cannot be treated as ordinary processing windows. | `L2-F09`, Financial Period Opening |
| Activation communicates governance state and does not create financial transactions. | Financial Period & Closing | Opening cannot originate or post business transactions. | `L2-F09`, Financial Period Activation |
| `FinancialPeriodOpened` means the period is operational; each Finance domain determines its own response. | Financial Period & Closing and consumers | Open state permits evaluation but does not guarantee acceptance of every transaction. | `L2-F09`, Event Catalogue |
| Authorization and approval authority remain external; the aggregate consumes outcomes only. | Authorization & Policy / Enterprise Governance | Period behaviour cannot invent user authority or approval hierarchy. | `L2-F09`, Authorization & Policy Integration |
| Aggregate consistency precedes persistence and event publication; partial updates are prohibited. | Financial Period & Closing | Failed establishment/opening leaves no partially valid period fact. | `L2-F09`, Aggregate Consistency Boundary |
| Multi-domain collaboration uses immutable events and never transfers ownership. | All participating domains | Consumers cannot directly manipulate the Financial Period aggregate. | `L2-F09`, Integration Principles |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Financial Year context | Must be established before period creation; no independent lifecycle is defined | Supplies fiscal grouping, dates, calendar relationship, and reporting hierarchy for multiple periods. | Established/assigned context; exact state undefined | Financial Period & Closing according to `L2-F09`, with ownership ambiguity noted below | `L2-F09`, Instantiation and Financial Year sections |
| Financial Period - registration | Does not yet exist | Create immutable aggregate identity, associate Institution/Financial Year, initialize governance/lock/readiness/version. | `Registered` | Financial Period & Closing | `L2-F09`, Financial Period Registration |
| Financial Period - preparation | `Registered` | Validate Financial Year, Period Date Range, and successful creation. | `Prepared` | Financial Period & Closing | `L2-F09`, State Transition Rules |
| Financial Period - opening | `Prepared` | Validate predecessor dependencies, governance approval, and policy; establish operational status. | `Open` | Financial Period & Closing | `L2-F09`, State Transition Rules |
| Fiscal Calendar | Authoritative external calendar | Referenced and associated; never modified by the Financial Period aggregate. | Existing Fiscal Calendar-owned state | Fiscal Calendar Management | `L2-F09`, Fiscal Calendar Integration |
| Participating Finance aggregates | Domain-owned states | Consume `FinancialPeriodOpened` and independently determine their operational response. | Unchanged until each owner performs valid business behaviour | Respective Finance domain | `L2-F09`, Downstream Integration |

No separate `Active` lifecycle state exists. The relationship between Opening and Activation, and the exact Financial Year ownership/lifecycle, remain open in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `InstitutionCreated` / `InstitutionActivated` | Institution Management | Financial Period & Closing | Supplies authoritative institutional context for period establishment. | `L2-F09`, Institution Management Integration |
| `FinancialPeriodRegistered` | Financial Period & Closing | Audit, Reporting, Administration | Confirms successful creation and initialization of a new period aggregate. | `L2-F09`, Event Catalogue |
| `FiscalCalendarAssociated` | Financial Period & Closing | Reporting, Audit, Administration | Records association to the externally owned Fiscal Calendar. | `L2-F09`, Administrative Events |
| `FinancialYearAssigned` | Financial Period & Closing | Reporting, Audit, Administration | Records the Financial Period's governing Financial Year association. | `L2-F09`, Administrative Events |
| `FinancialPeriodPrepared` | Financial Period & Closing | Enterprise Governance, Administration, Audit | Confirms administrative preparation and eligibility for governance evaluation. | `L2-F09`, Event Catalogue |
| `GovernanceApprovalGranted` / `GovernanceApprovalRejected` | Financial Period & Closing records the externally owned outcome | Financial Period & Closing, Audit, Administration | Communicates the immutable approval result used by the lifecycle decision. | `L2-F09`, Event Catalogue |
| `FinancialPeriodOpened` | Financial Period & Closing | General Ledger, AR, AP, Treasury, Budget, Reporting, Audit | Announces that the period is operational and consumers may apply their own rules. | `L2-F09`, Event Catalogue |
| `PeriodStatusChanged` | Financial Period & Closing | Governance, Reporting, Audit, participating Finance domains | Communicates operational status change where used. | `L2-F09`, Operational Events |

No canonical Financial Year registered/prepared/opened event, predecessor-satisfied event, opening-requested event, or separate FinancialPeriodActivated event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institution Management | Financial Period & Closing | Institution identity, lifecycle/status, and configuration | Establishes the tenant/institution boundary without ownership transfer. | `L2-F09`, Institution Management Integration |
| Organisation Management | Financial Period & Closing | Organisation identity, hierarchy, status, and reference | Supplies organisational context where required. | `L2-F09`, Organisation Integration |
| Fiscal Calendar Management | Financial Period & Closing | Calendar definition, fiscal structure, configuration, rules, and authoritative reference | Enables Financial Year/period structure while calendar ownership remains external. | `L2-F09`, Fiscal Calendar Integration |
| Authorization & Policy / Enterprise Governance | Financial Period & Closing | Authorization and immutable approved/rejected opening outcome | Determines whether the transition may proceed without embedding authority logic. | `L2-F09`, Authorization & Policy Integration |
| Financial Period & Closing | Participating Finance domains | `FinancialPeriodOpened` with period identity, Institution, Financial Year/date boundary, status, version, and correlation | Communicates the open processing window for independent domain evaluation. | `L2-F09`, Event and Integration Architecture; exact content incomplete |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate Financial Year/period establishment | No canonical actor defined | Requires established Institution, Fiscal Calendar, Financial Year context, and financial-governance approval. | Financial Period & Closing / Enterprise Governance |
| Define fiscal calendar | No canonical actor defined in `L2-F09` | Calendar definition remains externally owned and cannot be changed by this scenario. | Fiscal Calendar Management |
| Register and prepare Financial Period | No canonical actor defined | Must satisfy creation and `Registered -> Prepared` guards. | Financial Period & Closing |
| Validate predecessor dependencies | No canonical actor or dependency owner mapping defined | Every required predecessor must be authoritatively complete. | Financial Period & Closing with dependency owners |
| Approve or reject opening | No canonical role, hierarchy, matrix, or threshold defined | Governance approval and institutional policy are mandatory. | Enterprise Governance |
| Apply `Prepared -> Open` | No canonical actor defined | Only Financial Period & Closing may modify period lifecycle after all guards succeed. | Financial Period & Closing |
| Accept domain transaction in open period | Role defined by each participating domain | Open status is necessary but each domain retains its own authorization and business rules. | Respective Finance domain |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Calendar ownership versus period lifecycle | Fiscal Calendar Management and Financial Period & Closing | Period governance references the calendar and never modifies its definition. |
| Period preparation versus approval | Financial Period & Closing and Enterprise Governance | Administrative readiness cannot self-create approval authority. |
| Authorization versus aggregate behaviour | Authorization & Policy and Financial Period & Closing | External authorization outcome precedes local lifecycle transition. |
| Period opening versus transaction creation | Financial Period & Closing and participating Finance domains | Opening does not create or approve Accounting, AR, AP, Treasury, or Budget activity. |
| Institution isolation | Institution Management and Financial Period & Closing | Cross-institution Financial Year or period ownership is prohibited. |
| Governance override | Enterprise Governance and Financial Period & Closing | No override may bypass invalid date range, missing predecessor, lifecycle, identity, or aggregate invariants. |
| Audit independence | Audit and operational owners | Audit examines evidence and does not register, prepare, open, or modify the period. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove institutional context | Institution identity/status/configuration, Organisation reference where applicable, and tenant isolation | Institution/Organisation owners and Financial Period & Closing references |
| Prove calendar and Financial Year basis | Fiscal Calendar identity/version, fiscal rules, Financial Year identifier, dates, structure, and governance approval | Fiscal Calendar Management, Financial Period & Closing, Enterprise Governance |
| Prove period registration | Period identity, Institution, Financial Year, date range, classification, initial states, version, invariants, and `FinancialPeriodRegistered` | Financial Period & Closing |
| Prove associations | Fiscal Calendar and Financial Year references plus association events and effective sequence | Financial Period & Closing and Fiscal Calendar owner |
| Prove preparation | Valid Financial Year, date range, aggregate creation, prior/resulting state, version, and `FinancialPeriodPrepared` | Financial Period & Closing |
| Prove opening authority | Predecessor results, policy, authorization outcome, governance decision, reason, conditions, and immutable approval evidence | Enterprise Governance / Authorization & Policy |
| Prove open window | Prior `Prepared` state, resulting `Open`, aggregate version, `FinancialPeriodOpened`, date boundaries, and consumer availability | Financial Period & Closing |
| Prove domain autonomy | Each participating domain's independent acceptance/rejection evidence for its own activity | Respective Finance domain |

---

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- **Canonical period identity:** Every accounting period has one immutable aggregate identity, one Institution, one Financial Year, and one period boundary.
- **Calendar authority:** Every period structure traces to an authoritative Fiscal Calendar without ownership duplication.
- **Financial Year completeness:** Financial Year identifier, dates, fiscal structure, period population, and approval evidence are available.
- **Registration integrity:** No duplicate/reused identity exists and all initialization/invariants succeed before `FinancialPeriodRegistered`.
- **Preparation integrity:** `Registered -> Prepared` occurs only after Financial Year, date range, and creation requirements succeed.
- **Opening guard completeness:** Predecessor dependencies, institutional policy, authorization, governance approval, lifecycle, and version are valid.
- **Event integrity:** `FinancialPeriodOpened` is published only after successful `Prepared -> Open` completion.
- **Domain ownership:** The period domain changes only period governance; participating domains independently control their transactions.
- **Institution isolation:** No period, Financial Year association, or transaction-governance fact crosses institution/tenant boundaries.
- **Historical traceability:** Registration, association, preparation, approval, opening, versions, and events remain reconstructable and immutable.
- **Failure containment:** Failed establishment or opening leaves the aggregate in its previous valid state and publishes no false completion fact.
---

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- Canonical period identity acceptance evidence is recorded and auditable.
- Calendar authority acceptance evidence is recorded and auditable.
- Financial Year completeness acceptance evidence is recorded and auditable.
- Registration integrity acceptance evidence is recorded and auditable.
- Preparation integrity acceptance evidence is recorded and auditable.
- Opening guard completeness acceptance evidence is recorded and auditable.
- Event integrity acceptance evidence is recorded and auditable.
- Domain ownership acceptance evidence is recorded and auditable.
- Institution isolation acceptance evidence is recorded and auditable.
- Historical traceability acceptance evidence is recorded and auditable.
- Failure containment acceptance evidence is recorded and auditable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Conflict | Architectural Impact | Required Upstream Decision / Owner |
|---|---|---|---|
| F051-01 | Financial Year establishment is a prerequisite for period creation, but `L2-F09` defines no independent Financial Year aggregate, lifecycle, registration command, or establishment event. | The start of the scenario and authoritative Financial Year state cannot be represented precisely. | Financial Period architecture must define the Financial Year business model or identify its external canonical owner. |
| F051-02 | `L2-F09` calls Financial Year an internal entity owned by each Financial Period aggregate while also stating one Financial Year contains multiple Financial Periods. | The same Financial Year may be duplicated across multiple aggregate ownership boundaries. | Domain architecture governance must correct the aggregate/entity ownership model. |
| F051-03 | Fiscal Calendar is externally owned, while Financial Year is internally owned and Fiscal Calendar Governance is also listed as a period-domain capability. | Calendar, Financial Year, and period responsibilities overlap. | Fiscal Calendar and Financial Period owners must define exact contracts and governance boundaries. |
| F051-04 | Financial Period Opening and Financial Period Activation are separate capabilities, but the state model contains only `Prepared -> Open`, `FinancialPeriodOpened` already means operational, and no Active state/event exists. | The final outcome and required sequence are ambiguous or duplicated. | Financial Period architecture must merge or distinguish Opening and Activation with explicit semantics. |
| F051-05 | `Registered -> Prepared` and `Prepared -> Open` requirements are defined, but no canonical command/event content or negative outcome is defined for failed preparation/opening. | Failure handling and audit contracts remain incomplete. | Financial Period and Event Governance must define completed and rejected business facts. |
| F051-06 | â€œPredecessor dependenciesâ€ and â€œpredecessor period requirementsâ€ are mandatory but not enumerated, owned, or contracted. | Opening decisions cannot be deterministic across institutions. | Financial Period & Closing with participating Finance domains must define dependency types and authoritative completion facts. |
| F051-07 | No explicit rules prohibit overlapping periods, gaps, duplicate date ranges, periods outside the Financial Year, or inconsistent period sequences beyond a general valid date-range guard. | Invalid calendars could be registered while passing incomplete validation. | Fiscal Calendar and Financial Period architecture must define structural invariants. |
| F051-08 | The source does not define whether multiple Financial Periods may be Open concurrently for the same Institution, Financial Year, or organisation. | Processing-window exclusivity and transaction-period selection are ambiguous. | Enterprise Finance Governance must define whether simultaneous open periods are permitted. |
| F051-09 | No canonical actors, financial administrator role, preparer/reviewer, opening authority, approval hierarchy, delegation, or separation-of-duty rules are defined. | Authorization and audit acceptance cannot be completed. | Enterprise Governance and Financial Period & Closing must define the role model. |
| F051-10 | Opening schedule, automatic activation eligibility, and approval requirements are configurable, but exact mandatory evidence and invariant limits are absent. | Configuration may produce inconsistent or insufficiently governed openings. | Financial Period and Enterprise Governance must define the configuration schema and non-overridable controls. |
| F051-11 | `FinancialPeriodOpened` contract content, correlation with Financial Year/calendar version, effective time, timezone, consumer acknowledgement, and version semantics are not fully defined. | Participating domains may interpret the open window differently. | Enterprise Event Governance must define the integration contract. |
| F051-12 | â€œOnly an Open Financial Period may accept financial activityâ€ is stated, but each domain independently determines operational response; mandatory minimum rejection behaviour for non-open periods is not defined across all domains. | Finance domains may inconsistently enforce the same period status. | Cross-domain Finance architecture must define the invariant acceptance boundary. |
| F051-13 | No authoritative timezone, inclusive/exclusive date-boundary, cut-off-time, holiday, leap-year, or calendar-correction rules are defined. | Opening windows and transaction assignment may differ across locations or domains. | Fiscal Calendar, Institution Governance, and Financial Period owners must define temporal semantics. |
| F051-14 | No governed administrative-correction path is defined for an incorrect Financial Year association or period definition after registration/opening. | Immutable identity/history and necessary calendar correction may conflict. | Financial Period architecture must define correction, replacement, and event-history behaviour. |
| F051-15 | The exact relationship between Period Status, lifecycle state, lock state, and operational availability is not defined. | A period could have internally inconsistent governance indicators. | Financial Period architecture must define one authoritative state model and invariants. |
| F051-16 | No measurable targets exist for establishment, validation, approval, opening, publication, consumer recognition, or exception resolution. | Operational control cannot be assessed against canonical expectations. | Business Governance must define measurable objectives without weakening correctness. |

This scenario remains **Draft** until the required upstream decisions are incorporated into the locked architecture and the scenario is revalidated.

## 17.2 Assumptions

- The scenario operates under the current locked L1/L2 business architecture baseline.
- All external references to institutional policies, organizational units, and accounts are authoritative.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-051**
