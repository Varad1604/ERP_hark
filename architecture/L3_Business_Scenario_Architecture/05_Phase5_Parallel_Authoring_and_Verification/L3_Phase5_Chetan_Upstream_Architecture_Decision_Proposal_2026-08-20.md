# SutraERP L3 Phase 5 — Chetan Upstream Architecture Decision Proposal

## 1. Document Control

| Field | Value |
|---|---|
| Status | Proposed — Non-Authoritative |
| Date | 2026-08-20 |
| Scope | Finance scenarios L3-F-020–L3-F-032 and L3-F-039–L3-F-061 |
| Purpose | Consolidate upstream decisions required before the affected L3 scenarios can be completed without architectural invention |
| Decision effect | None until approved through repository governance and incorporated into the applicable locked upstream architecture |

This proposal does not amend, reinterpret, or override locked L1 or L2 architecture. It records the minimum upstream decisions needed to unblock the 36 affected Finance scenarios. Until those decisions are approved upstream, the L3 files must retain their blocked or open-issue disposition and must not invent actors, states, events, publishers, contracts, or recovery semantics.

## 2. Authoritative Sources Requiring Governance Action

- `SutraERP_L1_Business_Domain_Architecture_Locked.md`
- applicable locked Finance L2 workflow documents identified by each affected L3 scenario
- `DEPENDENCY_MAP.md`
- `Architecture_Level_Validation_Standards.md`
- `L3_Business_Scenario_Template_v2.0.md`
- `L3_Final_Canonical_Scenario_ID_Register.md`

The Finance architecture owner is the semantic decision owner. Enterprise Architecture must validate ownership, dependency direction, cross-domain boundaries, and consistency before any L2 amendment is locked. Section A owners participate only where canonical authorization, audit, workflow, or event-infrastructure contracts are affected.

## 3. Decision Package FIN-ADJ — Financial Adjustment Governance

**Affected scenarios:** L3-F-020–L3-F-024

The applicable upstream Finance architecture must define:

1. the canonical business actors authorized to request, verify, recommend, approve, reject, reverse, and close each adjustment outcome;
2. the owned aggregate and exact lifecycle states for an adjustment request and its governed outcome;
3. exact canonical event names, one publisher per event, and the material consumers;
4. the business-level distinction between correction, reversal, rejection, cancellation, and terminal failure;
5. the governed recovery path when an adjustment result is uncertain, duplicated, rejected, or only partly applied; and
6. the cross-domain hand-off contract where the adjustment depends on a fact owned outside Finance.

**Required approval evidence:** an L2 actor table, lifecycle transition table, event catalogue, publisher/consumer mapping, and business recovery rules traceable to the owning Finance aggregate.

## 4. Decision Package FIN-GL — General Ledger Posting and Close

**Affected scenarios:** L3-F-025–L3-F-032

The applicable upstream Finance architecture must define:

1. which authoritative business fact initiates each posting, validation, close, lock, reopen, or correction scenario;
2. the actor and authority separation for preparation, review, posting, close, lock, reopen, and exceptional approval;
3. exact canonical General Ledger lifecycle states and valid transitions, including whether `Closed`, `Locked`, and `Reopened` are distinct states or governed dispositions;
4. exact canonical events and publishers for posting, rejection, close, lock, reopen, and correction outcomes;
5. the relationship between journal state, ledger-period state, financial-period state, and financial-year state without merging their ownership; and
6. the minimum business evidence and checklist required to prove a close or reopen decision.

**Required approval evidence:** separate lifecycle definitions for each aggregate, a trigger-to-event map, authority matrix, close/reopen evidence requirements, and explicit cross-aggregate ownership boundaries.

## 5. Decision Package FIN-AR — Accounts Receivable Lifecycle

**Affected scenarios:** L3-F-039–L3-F-043

The applicable upstream Finance architecture must define:

1. the canonical receivable actors and their initiation, review, allocation, approval, dispute, write-off, and closure authority;
2. the owned receivable, allocation, dispute, and settlement aggregates and their exact lifecycle states;
3. exact events and publishers for recognition, allocation, partial settlement, dispute, correction, write-off, and closure;
4. the contract by which non-Finance source facts are accepted without transferring ownership;
5. duplicate-recognition and duplicate-allocation prevention at business level; and
6. governed recovery when source evidence, allocation, settlement, or downstream confirmation is incomplete or contradictory.

**Required approval evidence:** actor/authority definitions, aggregate boundaries, lifecycle tables, event contracts, and a business recovery decision table.

## 6. Decision Package FIN-AP — Accounts Payable and Vendor-Invoice Recognition

**Affected scenarios:** L3-F-044–L3-F-050

The applicable upstream architecture must resolve:

1. the canonical publisher and meaning of `VendorInvoiceApproved`, including the present F08/F14 source ambiguity;
2. whether invoice approval, payable recognition, matching, exception resolution, credit treatment, and payment eligibility are separate owned lifecycle positions;
3. the canonical actors for invoice intake, matching, exception review, payable approval, credit application, and reconciliation;
4. exact lifecycle states and transitions for Vendor Invoice, Payable, Match/Exception, and Credit where those concepts are authoritative;
5. exact event names, publishers, and material consumers for recognition, match outcomes, exceptions, credit, correction, and closure;
6. the business contract with supplier, receipt, purchasing, tax, treasury, and General Ledger owners; and
7. the business recovery path for duplicates, unmatched evidence, disputed outcomes, and uncertain downstream confirmation.

**Required approval evidence:** a single authoritative event publisher decision for `VendorInvoiceApproved`, aggregate ownership map, actor matrix, lifecycle/event tables, and cross-domain contract definitions.

## 7. Decision Package FIN-PERIOD — Financial Year and Period Governance

**Affected scenarios:** L3-F-051–L3-F-055

The applicable upstream Finance architecture must define:

1. the canonical owner of Financial Year and Financial Period master data;
2. exact Financial Year and Financial Period lifecycle states and valid transitions;
3. the distinction between creation, opening, activation, operational close, lock, reopen, final close, and archival;
4. the actors authorized to propose, approve, activate, close, lock, reopen, and archive each aggregate;
5. exact events, their single publishers, and material consumers for each governed transition;
6. cross-domain prerequisites and evidence for opening, closing, and reopening; and
7. the business rule for transactions or adjustments discovered after close or lock.

**Required approval evidence:** owner declaration, separate Year and Period lifecycle tables, authority matrix, event catalogue, prerequisite checklist, and post-close exception rules.

## 8. Decision Package FIN-TREASURY — Treasury Execution, Settlement, and Resolution

**Affected scenarios:** L3-F-056–L3-F-061

The applicable upstream Finance architecture must define:

1. canonical ownership and lifecycles for Treasury Account, Treasury Instruction or Execution, Settlement, Reconciliation, and Resolution records;
2. actors and separation of duties for initiation, verification, approval, release, confirmation, reconciliation, exception resolution, reversal, and closure;
3. what constitutes authoritative business finality for release, external acceptance, settlement, rejection, return, reversal, and unresolved outcome;
4. exact event names, one publisher per event, and material consumers;
5. synchronous business hand-offs with banking or payment providers, without prescribing endpoints, payloads, queues, or technical retry behaviour;
6. business-level duplicate, timeout, uncertain-result, partial-settlement, rejected, returned, and reversed outcome treatment; and
7. the boundary between Treasury authority, Accounts Payable/Receivable authority, General Ledger posting authority, and external financial-institution facts.

**Required approval evidence:** aggregate and ownership map, lifecycle tables, actor/authority matrix, finality decision table, event contracts, and business recovery rules that expressly exclude implementation-level retry design.

## 9. Required Governance Sequence

1. Finance Architecture confirms the intended aggregates and owners for each decision package.
2. Relevant cross-domain owners confirm every consumed or published business contract.
3. Finance Architecture defines exact actors, lifecycle states, transitions, events, publishers, and business recovery rules.
4. Enterprise Architecture checks consistency with L1, dependency direction, Section A platform ownership, and existing locked L2 documents.
5. The approved change is incorporated and locked at the appropriate upstream level.
6. Only after the authoritative sources are updated are the affected L3 scenarios revised, revalidated, and moved out of their blocked/open-issue disposition.

## 10. Acceptance Checklist

An upstream decision package is complete only when all of the following are explicit and mutually consistent:

- canonical aggregate and owner;
- exact state names and transitions;
- exact actor names and decision authority;
- exact event names and one publisher per event;
- consumers and dependency direction;
- synchronous business contracts where required;
- separation-of-duty and evidence obligations;
- business-level rejection, correction, reversal, interruption, and terminal-resolution semantics;
- no implementation-level API, persistence, queue, retry-count, idempotency, or infrastructure design; and
- traceability from the amended L2 material to every affected L3 scenario.

## 11. Non-Decision Statement

This document intentionally does not select state names, invent events, assign publishers, create actors, or settle the F08/F14 publisher ambiguity. Those choices change authoritative architecture and require approval by the named upstream decision owners.
