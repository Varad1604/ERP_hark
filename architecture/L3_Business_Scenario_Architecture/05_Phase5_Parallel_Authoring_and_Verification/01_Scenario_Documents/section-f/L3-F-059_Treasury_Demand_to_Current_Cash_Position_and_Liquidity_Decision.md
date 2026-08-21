# Treasury Demand to Current Cash Position and Liquidity Decision

# 1. Scenario Metadata

**Scenario ID:** L3-F-059  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Treasury & Banking  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F07_Accounts_Receivable_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F09_Financial_Period_and_Closing.md`; `L2-F10_Treasury_and_Banking.md`; `L3-F-057_Approved_Payment_Population_to_Bank_Execution_Outcome.md`; `L3-F-058_Bank_Statement_to_Reconciled_Cash_Position.md`; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs evaluation of an institutional Treasury demand against the current authoritative Cash Position, Liquidity Position, reserved and restricted cash, expected payment/settlement activity, forecast cash movements, operational restrictions, and institution-configured liquidity policies. It produces a traceable liquidity decision without approving the underlying obligation, executing payment, changing budgets, or treating forecast values as actual cash.

---

# 3. Business Outcome

The requested Treasury demand is classified through current authoritative evidence as supportable, supportable with governed conditions or sequencing, deferred, blocked, or requiring review. The decision preserves actual-versus-forecast separation, cash and liquidity integrity, currency and Institution scope, reserved/restricted funds, policy thresholds, and immutable evidence. Any subsequent reservation, scheduling, execution, settlement, or accounting occurs through separately owned behaviour.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when Treasury receives a current or future institutional cash demand with identifiable Institution, Organisation context where applicable, business source, amount, currency, required date/window, priority, purpose, obligation/forecast reference, and approval/eligibility evidence where required.

## 4.2 End Boundary

The scenario ends when Treasury records a current cash/liquidity decision with contributing facts, policy, conditions, restrictions, threshold effects, version, and correlation; or retains a pending/failed evaluation because authoritative inputs are incomplete or conflicting. The scenario does not itself execute or settle the demand.

## 4.3 In Scope

- Intake and validation of Treasury demand identity, source, purpose, amount, currency, timing, priority, and correlation.
- Consumption of externally owned approved obligations, expected receipts, budget facts, Financial Period facts, and institutional context.
- Evaluation of current Cash Position, including current, available, reserved, and restricted cash.
- Evaluation of current/available/reserved and forecast Liquidity Position.
- Consideration of scheduled payments, settlements, expected receipts, cash forecasts, reserves, restrictions, minimum liquidity, and configured thresholds.
- Classification of the demand decision and preservation of conditions/exceptions.
- Publication of source-supported Cash Position, liquidity, availability, reserve, forecast, or threshold events only after completed owned changes.
- Read-only decision communication to downstream Treasury operations, Reporting, dashboards, and Audit.

## 4.4 Out of Scope

- Creating or approving Payables, Receivables, refunds, grants, payroll, taxes, purchases, budgets, or other demands.
- Executing, batching, settling, or reconciling payments.
- Treating a forecast as actual cash or changing Cash Position because demand merely exists.
- Creating Journal Entries, modifying Ledger balances, or generating financial statements.
- Implicit currency conversion or inventing exchange-rate ownership.
- Defining forecasting algorithms, data pipelines, APIs, databases, calculation engines, or implementation technology.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-058` Bank Statement to Reconciled Cash Position | Strengthens the current authoritative Cash Position used by the decision. |
| Related | `L3-F-057` Approved Payment Population to Bank Execution Outcome | Consumes liquidity decisions before payment execution. |
| Related | `L3-F-045` Approved Payable to Financial Settlement | Supplies payable demand without transferring liability ownership. |
| Related | `L3-F-049` Payable Population to Reconciled Period Outcome | Provides AP-owned population evidence. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Treasury & Banking / Section F | Solely owns Cash Position, Liquidity Position, Cash Forecast, Treasury demand evaluation, Treasury restrictions, aggregate version, and Treasury events. |
| Accounts Payable | Owns Payable liabilities, approval, due dates, and outgoing demand facts. |
| Accounts Receivable | Owns Receivables and expected receipt/collection facts. |
| Budget Management | Owns budget availability, commitments, and consumption facts; Treasury cannot allocate budgets. |
| Financial Period & Closing | Owns Financial Period lifecycle and restrictions consumed by Treasury. |
| Authorization & Policy / Enterprise Governance | Own authorization, thresholds, exception approval, escalation, and governance outcomes. |
| Accounting & General Ledger | Owns Journals and Ledgers and independently consumes completed Treasury facts. |
| Financial Reporting / Audit | Consume read-only decision, cash, liquidity, forecast, threshold, and evidence facts. |
| Institution and Organisation Management | Own institutional and organisational context. |

Treasury owns institutional cash and liquidity conclusions. It consumes demand and eligibility facts but never assumes ownership of the underlying business obligation.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F10` defines no canonical actors for submitting demand, maintaining forecasts, evaluating liquidity, setting reserves/thresholds, approving exceptions, reviewing decisions, or authorizing subsequent execution. No actor or job title is invented. Required role and segregation definitions remain open in Section 17.

---

# 7. Preconditions and Entry Criteria

- An authoritative operational Treasury Aggregate exists for exactly one Institution.
- Current Cash Position and Liquidity Position are available with their versions and effective sequence.
- Cash Position distinguishes current, available, reserved, and restricted cash where applicable.
- Liquidity Position distinguishes current, forecast, available, and reserved liquidity.
- The demand identifies amount, currency, timing, source, purpose, priority, and correlation.
- Upstream obligation/receipt/budget/period facts remain owned and traceable to their source domains.
- Current Treasury restrictions, account/banking availability, reserves, minimum-liquidity rules, thresholds, schedules, and policies can be evaluated.
- Relevant scheduled payments, settlements, expected receipts, and forecasts are available without being treated as actual cash.
- Duplicate or conflicting demands can be detected.
- No implicit currency conversion or cross-Institution pooling is assumed.

---

# 8. Trigger

A payment population, settlement need, reserve request, institutional cash requirement, planned activity, or treasury review requires a current decision on cash availability and liquidity sufficiency.

No canonical TreasuryDemandSubmitted, LiquidityDecisionRequested, or LiquidityDecisionCompleted event or demand state model is defined upstream.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the Treasury demand with Institution, source, reference, purpose, amount, currency, required timing, priority, approval/eligibility evidence, and correlation. | Demand owner / Treasury & Banking | A traceable evaluation request exists. |
| 2 | Validate tenant/Institution scope, source ownership, demand completeness, currency, timing, duplicates, and authoritative references. | Treasury & Banking | Invalid or conflicting demand is stopped. |
| 3 | Obtain the latest valid Cash Position, including current, available, reserved, and restricted cash for the applicable scope and currency. | Treasury & Banking | Actual cash baseline is established. |
| 4 | Obtain the latest Liquidity Position, including current, available, reserved, and forecast liquidity, with effective sequence/version. | Treasury & Banking | Liquidity baseline is established separately from cash. |
| 5 | Collect current scheduled payment, execution, settlement, reserve, expected receipt, bank activity, and forecast facts from canonical owners. | Treasury & Banking / source owners | Known current/future inflows and outflows are traceable. |
| 6 | Validate Treasury status, restrictions, Financial Period facts, account/banking availability, authorization, and governance conditions. | Treasury & Banking / external owners | Operational eligibility is established. |
| 7 | Apply Cash Management, Liquidity, Payment Execution, Settlement, and Treasury Governance policies, including minimum liquidity, reserve thresholds, operational liquidity, and configured limits. | Treasury & Banking | Policy-compliant decision conditions are evaluated. |
| 8 | Test demand against available cash and liquidity without consuming restricted/reserved amounts unless an authoritative policy permits it. | Treasury & Banking | Cash and reserve integrity are preserved. |
| 9 | Evaluate timing and forecast effect separately from current affordability; never convert forecast receipt into actual available cash. | Treasury & Banking | Current and future decision bases remain distinct. |
| 10 | Classify the outcome as supportable, conditionally supportable, deferred, blocked, or requires review and record reason, amount/currency, timing, conditions, exceptions, and source facts. | Treasury & Banking | A deterministic current decision exists. |
| 11 | If the decision includes a source-supported reserve/availability/liquidity modification, revalidate aggregate invariants, advance version, and publish the applicable completed event only after success. | Treasury & Banking | Owned state changes remain consistent and auditable. |
| 12 | Publish `LiquidityThresholdReached` when configured threshold crossing is completed/detected under policy and route governance response. | Treasury & Banking | Material liquidity condition is visible. |
| 13 | Communicate the decision to payment scheduling/execution, demand owner, Reporting, dashboards, and Audit as a business fact, not payment approval or settlement. | Treasury & Banking | Downstream owners independently decide their next behaviour. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Sufficient current cash/liquidity | Available amounts exceed demand and mandatory reserves. | Mark supportable subject to current eligibility and subsequent execution revalidation. | Demand may progress; no payment executed here. | Treasury & Banking |
| Sufficient forecast, insufficient current cash | Expected future inflow may cover later demand. | Mark deferred/conditionally supportable for a future window; do not treat forecast as cash. | Demand cannot execute immediately. | Treasury & Banking |
| Partial capacity | Only part of a divisible demand is supportable. | Route conditional/partial decision only if upstream policy permits; preserve remaining blocked amount. | No invented split approval. | Treasury / demand owner |
| Insufficient liquidity | Demand would breach availability or reserves. | Block/defer or route governance review and preserve threshold evidence. | Cash integrity maintained. | Treasury & Banking |
| Restricted cash only | Gross cash exists but is not available for this purpose. | Exclude restricted amounts and reject/defer demand. | Restricted funds remain protected. | Treasury & Banking |
| Treasury restricted/suspended | Operational controls prohibit progression. | Mark blocked regardless of apparent cash availability. | No bypass of governance. | Treasury & Banking / Governance |
| Stale inputs | Cash, liquidity, forecast, obligation, or receipt facts are not current. | Keep decision pending and refresh authoritative inputs. | No decision from stale evidence. | Relevant owner |
| Cross-currency demand | Demand currency differs from available position. | Evaluate only source-defined compatible liquidity; perform no implicit conversion. | Requires governed currency handling. | Treasury & Banking |
| Threshold crossed | Demand or changed facts breach configured threshold. | Publish threshold fact and route escalation/review. | Governance response initiated. | Treasury & Banking / Governance |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Demand identity/source/Institution is inconsistent. | Cash could be allocated to an invalid need. | Reject and obtain corrected authoritative context. | Treasury & Banking / demand owner |
| Cash Position conflicts with bank/settlement facts. | Affordability decision may be wrong. | Block decision, reconcile through `L3-F-058`, and re-evaluate. | Treasury & Banking |
| Liquidity/forecast facts are stale or contradictory. | Future capacity may be overstated. | Preserve pending outcome and obtain current authoritative facts. | Treasury & Banking / source owner |
| Reserved/restricted cash is counted as available. | Institutional safeguards may be breached. | Reject/contain decision and recalculate under policy. | Treasury & Banking |
| Concurrent aggregate change advances version. | Decision or reserve change is stale. | Reload current aggregate, re-evaluate all facts, and submit a new attempt. | Treasury & Banking |
| Aggregate modification partially succeeds. | Cash, liquidity, reserve, or history may diverge. | Roll back, preserve prior version, and publish no false event. | Treasury & Banking |
| Decision commits but event publication fails. | Downstream operations may use an outdated decision. | Recover publication without repeating the owned state change. | Treasury & Banking |
| Forecast receipt fails to materialize. | Previously conditional future decision is no longer supportable. | Publish/update authoritative forecast/liquidity fact and re-evaluate before execution. | Treasury & Banking / receipt owner |
| Demand changes after decision. | Amount, currency, timing, or priority basis becomes invalid. | Invalidate the old decision and perform a new governed evaluation. | Demand owner / Treasury & Banking |
| Negative liquidity would result without policy permission. | Treasury invariant is violated. | Block modification and route governance resolution. | Treasury & Banking |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Treasury exclusively owns Cash Position and Liquidity Position. | Treasury & Banking | External demand owners cannot directly modify cash/liquidity state. | `L2-F10`, Scope and Ownership |
| Cash Position represents current, available, reserved, and restricted institutional cash. | Treasury & Banking | Gross cash cannot be treated as fully available. | `L2-F10`, Cash Position Entity |
| Liquidity Position represents current, forecast, available, and reserved liquidity. | Treasury & Banking | Forecast and current liquidity remain distinguishable. | `L2-F10`, Liquidity Position Entity |
| Cash Position changes only after successful settlement completion or another explicitly valid Treasury behaviour. | Treasury & Banking | Demand evaluation alone cannot change actual cash. | `L2-F10`, Settlement and Cash Invariants |
| Payment Execution Policy evaluates readiness, restrictions, liquidity, and operational schedule. | Treasury & Banking | Decision is input to execution, not execution itself. | `L2-F10`, Payment Execution Policy |
| Liquidity Policy governs minimum liquidity, reserve thresholds, and operational liquidity. | Treasury & Banking | Threshold and reserve protection are mandatory. | `L2-F10`, Liquidity Policy |
| Liquidity must not become negative unless explicitly permitted by institutional policy. | Treasury & Banking / Governance | Invalid demand/reserve changes are blocked. | `L2-F10`, Liquidity Invariants |
| Forecasts are projections and do not modify actual cash state. | Treasury & Banking | Expected receipts cannot fund immediate demand as actual cash. | `L2-F10`, Cash Forecasting |
| Treasury never approves obligations, allocates budgets, creates Journal Entries, or governs periods. | External domain owners | Demand decision does not transfer upstream ownership. | `L2-F10`, Boundaries |
| No implicit currency conversion occurs within Treasury aggregate behaviour. | Treasury & Banking | Cross-currency availability cannot be assumed. | `L2-F10`, Currency Constraints |
| Every successful modification advances version; failure preserves version/history. | Treasury & Banking | Cash/liquidity changes are protected from conflicting simultaneous decisions. | `L2-F10`, Version and Failure Constraints |
| Events publish only after completed consistent facts. | Treasury & Banking | Decision intent cannot masquerade as state change. | `L2-F10`, Event Constraints |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Treasury Demand | No canonical state model | Validate and classify against current cash/liquidity. | Supportable, conditional, deferred, blocked, or review as scenario outcomes | Source owner / Treasury evaluation | Canonical register; L2 gap |
| Cash Position | Current authoritative position | Read and, only through valid completed owned behaviour, update availability/reserve/current cash. | Current consistent position | Treasury & Banking | `L2-F10`, Cash Position Management |
| Liquidity Position | Current/forecast position | Evaluate or update current, available, reserved, and forecast liquidity. | Current policy-compliant position | Treasury & Banking | `L2-F10`, Liquidity Management |
| Cash Forecast | Current projection | Incorporate expected demand/inflow under forecast policy. | Updated projection if valid; actual cash unchanged | Treasury & Banking | `L2-F10`, Cash Forecasting |
| Treasury Aggregate | `Operational` unless independently restricted | Enclose consistent owned changes and advance version. | Remains valid current state | Treasury & Banking | `L2-F10`, Aggregate State Model |
| Upstream obligation/budget/receipt | Externally owned | Consumed as decision facts only. | Unchanged by Treasury | Respective owners | `L2-F10`, Integration Matrix |

`L2-F10` defines Cash and Liquidity positions but no formal Liquidity Decision aggregate or decision lifecycle. The scenario outcomes therefore remain L3 business classifications pending Section 17 resolution.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `CashPositionUpdated` | Treasury & Banking | GL, Reporting, dashboards, Audit | Announces completed actual Cash Position change. | `L2-F10`, Cash Management Events |
| `CashReserveUpdated` | Treasury & Banking | Treasury operations, Reporting, Audit | Announces completed reserve allocation change. | `L2-F10`, Cash Management Events |
| `CashAvailabilityChanged` | Treasury & Banking | Payment scheduling/execution, Reporting, dashboards | Announces completed change in available operational cash. | `L2-F10`, Cash Management Events |
| `LiquidityPositionUpdated` | Treasury & Banking | Treasury operations, Reporting, dashboards, Audit | Announces completed Liquidity Position change. | `L2-F10`, Liquidity Events |
| `LiquidityForecastUpdated` | Treasury & Banking | Treasury planning, Reporting, dashboards | Announces updated projection, not actual cash. | `L2-F10`, Liquidity Events |
| `LiquidityThresholdReached` | Treasury & Banking | Governance, dashboards, Notification, Audit | Announces configured threshold condition. | `L2-F10`, Liquidity Events |
| Upstream demand/receipt/budget/period facts | Respective domain owners | Treasury & Banking | Supply externally owned decision inputs. | `L2-F10`, Integration Architecture; exact events incomplete |

No canonical TreasuryDemandSubmitted/Changed/Withdrawn, LiquidityDecisionRequested/Completed/Expired, DemandSupported/Blocked/Deferred, or decision-invalidated event exists.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| AP/AR/Budget/other demand owner | Treasury & Banking | Demand/reference, source state, amount, currency, timing, priority, approval/eligibility, and correlation | Supplies decision need without transferring ownership. | `L2-F10`, Upstream Relationships; exact contract incomplete |
| Financial Period / Authorization & Policy | Treasury & Banking | Current period, restriction, authorization, and governance outcomes | Constrains Treasury decision. | `L2-F10`, Integration Architecture |
| Treasury & Banking | Payment scheduling/execution / demand owner | Decision classification, effective time, amount/currency, conditions, restrictions, policy, evidence, version, and expiry where later defined | Enables independent downstream action. | Canonical scenario; exact contract absent |
| Treasury & Banking | Reporting / dashboards / Audit | Current Cash Position, Liquidity Position, forecast, threshold, and decision facts | Supports visibility without state modification. | `L2-F10`, Reporting/Audit Integration |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Submit demand | No canonical actor defined | Must supply authoritative source, amount, currency, timing, and purpose. | Demand owner |
| Maintain cash/liquidity/forecast inputs | No canonical actor defined | Only Treasury aggregate behaviour may modify owned state. | Treasury & Banking |
| Evaluate liquidity decision | No canonical actor defined | Must apply current evidence, restrictions, reserves, and policy. | Treasury & Banking |
| Approve threshold/reserve exception | No canonical role/hierarchy defined | Governance outcome remains externally owned. | Enterprise Governance |
| Authorize downstream execution | No canonical actor defined | A liquidity decision alone is not business payment approval. | Treasury / upstream approval owner |
| Review decision evidence | No canonical reviewer defined | Read-only review cannot alter facts. | Audit |

## 15.2 Segregation of Duties

- Demand creation, underlying approval, cash/liquidity maintenance, forecast maintenance, decision evaluation, exception approval, payment execution, and audit review should remain independently governed; exact rules are absent.
- Treasury cannot approve the obligation whose cash impact it evaluates.
- Forecast maintenance must not permit conversion of projections into actual cash.
- Exception authority must remain separate from normal decision execution where policy requires.
- Reporting and Audit remain read-only consumers.

## 15.3 Mandatory Audit Evidence

- Demand identity/source, Institution, Organisation context, purpose, amount, currency, timing, priority, eligibility/approval, and correlation.
- Cash Position and Liquidity Position values, composition, effective sequence/version, and source facts used.
- Reserved/restricted cash, scheduled payments, settlements, expected receipts, forecasts, accounts, restrictions, period, budget, and authorization facts.
- Policy/configuration version, minimum liquidity, reserve/availability thresholds, operational schedule, and decision rules.
- Decision classification, reason, conditions, exceptions, effective time, version, and downstream communication.
- Any Cash Position, reserve, availability, liquidity, or forecast change with prior/resulting values and event.
- Threshold crossing, escalation, approval/rejection, failure, stale-fact detection, simultaneous-change conflict, and recovery evidence.
- Proof that no forecast was treated as actual cash and no upstream obligation/budget/accounting state was modified.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

| Measure | Business Meaning | Expected Direction / Acceptance Intent |
|---|---|---|
| Decision input completeness | Decisions supported by current demand, cash, liquidity, restriction, and policy facts | 100% |
| Unauthorized negative-liquidity count | Decisions/modifications breaching policy | Zero |
| Restricted/reserved cash misuse count | Protected cash counted as freely available without authority | Zero |
| Forecast-as-actual violations | Projected inflow treated as current cash | Zero |
| Decision traceability rate | Demand-to-input-to-policy-to-outcome histories reconstructable | 100% |
| Stale decision execution count | Downstream action based on invalidated/outdated decision | Zero |
| Threshold escalation completeness | Threshold events with governed response/evidence | 100% |

No canonical minimum liquidity amount, reserve percentage, forecast horizon, confidence standard, decision validity period, SLA, priority matrix, or tolerance is defined upstream.

## 16.2 Postconditions

- Filename and Scenario ID match the frozen canonical register.
- Demand remains externally owned and is not mistaken for payment approval.
- Current, available, reserved, restricted, and forecast values remain distinguishable.
- Decision uses current Institution/currency-scoped facts and configured policies.
- Forecast values never modify actual Cash Position.
- Restricted/reserved cash is protected unless an authoritative policy permits use.
- Cash/liquidity modifications advance aggregate version and publish events only after consistency.
- Decision outcome is traceable and does not execute or settle payment.
- Failed/stale/conflicting evaluations preserve prior valid state and evidence.
- Treasury does not modify Payables, Receivables, budgets, periods, Journals, or Ledgers.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Open Issue / Gap | Architectural Impact | Required Resolution Owner |
|---|---|---|---|
| OI-F059-001 | No Treasury Demand aggregate/entity, state model, canonical owner, or event contract exists. | Demand intake, change, withdrawal, expiry, and disposition cannot be standardized. | Treasury / Finance Architecture Governance |
| OI-F059-002 | No Liquidity Decision entity, state model, classifications, transition guards, or completion event is defined. | The title's terminal decision cannot be mechanically validated. | Treasury & Banking |
| OI-F059-003 | Cash Position and Liquidity Position calculation formulae, scope, aggregation level, and effective-time semantics are incomplete. | Different implementations may produce different current positions. | Treasury & Banking |
| OI-F059-004 | Liquidity reservation creation, release, expiry, priority, overcommitment, and simultaneous-demand precedence rules are undefined. | Multiple demands may consume the same apparent liquidity. | Treasury & Banking |
| OI-F059-005 | Minimum liquidity, reserve thresholds, operational liquidity, negative-liquidity permission, and exception authority are configurable but no canonical values/guards exist. | Decisions remain institution-specific and not fully testable. | Treasury / Enterprise Governance |
| OI-F059-006 | Forecast inputs, horizons, confidence, scenario versions, approval, stale-data treatment, and actual-versus-forecast reconciliation are incomplete. | Conditional decisions may rely on unreliable projections. | Treasury & Banking / source owners |
| OI-F059-007 | No canonical demand-supported, conditional, deferred, blocked, review, invalidated, or expired events exist. | Downstream execution may not know decision status/finality. | Treasury & Banking |
| OI-F059-008 | Decision validity period and mandatory revalidation point before payment execution are undefined. | Stale liquidity decisions may be executed. | Treasury & Banking |
| OI-F059-009 | Cross-currency demand, exchange-rate owner, conversion timing, currency pools, and implicit-conversion prohibition hand-offs are incomplete. | Liquidity may be overstated across currencies. | Treasury / Accounting Governance |
| OI-F059-010 | Relationship between budget availability, payment approval, liquidity supportability, and execution authority lacks an exact contract. | A positive decision may be misread as budget or payment approval. | Budget / AP / Treasury / Governance |
| OI-F059-011 | Actors, forecast owner, decision maker, reserve authority, threshold exception approver, and segregation rules are absent. | Authorization and accountability cannot be fully accepted. | Enterprise Governance / Authorization & Policy |
| OI-F059-012 | Exact demand, cash, liquidity, forecast, reserve, decision, threshold, and downstream business contract content are incomplete. | End-to-end decision evidence cannot be mechanically validated. | Relevant contract owners |
| OI-F059-013 | Duplicate demands, simultaneous evaluations, late facts, stale versions, repeat-request handling, and published-event outcome confirmation rules are incomplete. | Conflicting decisions or double reservation may occur. | Treasury & Banking |
| OI-F059-014 | Multi-account/institution/organisation liquidity aggregation and restricted-fund isolation rules are incomplete. | Funds may be pooled across prohibited boundaries. | Treasury & Banking / Institutional Governance |
| OI-F059-015 | Decision retention, supersession, correction, authorized access, and audit-reconstruction requirements are incomplete. | Historical decision accountability may be lost. | Treasury / Audit / Compliance |
| OI-F059-016 | No canonical SLA, forecast horizon/confidence, decision validity, reserve threshold, priority rule, or acceptance tolerance exists. | Measures remain directional rather than fully testable. | Treasury & Banking / Enterprise Governance |

This scenario remains **Draft** until Treasury Demand and Liquidity Decision models, position calculations, reserve and simultaneous-demand precedence rules, forecast governance, authorization, events, contracts, and measurable acceptance criteria are defined. The draft never treats forecast liquidity as current cash or a liquidity decision as payment approval.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-059**
