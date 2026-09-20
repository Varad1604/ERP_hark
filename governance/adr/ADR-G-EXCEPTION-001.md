# ADR-G-EXCEPTION-001 — G00 Governance Exception Recovery Authority and Event Semantics

| Field | Value |
|---|---|
| ADR ID | ADR-G-EXCEPTION-001 |
| Change Type | L2 G00 business-authority and event clarification |
| Target | L2-G00 — Campus Operations Review & Governance, §28 Exception Handling |
| Directly Resolves | L3/L4-G-013 |
| Status | Approved — Incorporated |
| Date | 2026-09-20 |
| Authority | User-authorized controlled upstream remediation in SutraERP_Level_3 project |

## 1. Problem

L2-G00 defines a governance-exception workflow and permissible recovery strategies but does not assign decision authority for deferral, administrative override or manual verification and does not identify the canonical non-escalation recovery event required before governance continuation.

L3/L4-G-013 therefore cannot complete the frozen corrected/deferred/escalated outcome without invention.

## 2. Decision

### 2.1 No new exception aggregate

The phrase **Governance Exception** remains a workflow condition affecting an existing G00 governance aggregate. This decision does not create a separate Governance Exception aggregate or lifecycle.

### 2.2 Canonical responsibilities

| Business role | Canonical exception responsibility |
|---|---|
| Governance Coordinator | Identifies and coordinates the exception, obtains corrected source evidence and records recovery evidence. |
| Campus Operations Manager | Canonical decision authority for ordinary G00 recovery selection, including source correction acceptance, review rescheduling, action reassignment, risk reassessment, governance-review extension, manual verification and time-bounded deferral where permitted by policy. |
| Executive Reviewer | Required decision authority for Administrative Override and for any recovery explicitly escalated beyond Campus Operations Manager authority by configured governance policy. |

No role may use exception recovery to mutate another domain's aggregate or fabricate missing source facts.

### 2.3 Recovery guards

- Source-owned facts must be corrected by their canonical source owner.
- Manual verification may confirm only a G00-owned governance condition or verify received source evidence; it may not substitute for a missing authoritative source fact.
- Deferral must be time/condition bounded and preserve the affected aggregate at a valid existing lifecycle position.
- Administrative Override may alter only a G00-owned governance-process requirement that policy explicitly permits to be overridden. It may not override another domain's lifecycle, statutory obligation, mandatory ownership boundary or unavailable authoritative fact.
- If ordinary recovery is not authorized or is insufficient, the matter must use the existing Executive Governance Resolution / escalation path.

### 2.4 Canonical events

For non-escalation recovery, G00 / Campus Operations Review is the sole canonical publisher of:

- `GovernanceExceptionRecoveryRecorded`

The event means that an identified G00 governance exception has a source-supported recovery decision recorded, with the affected governance item and recovery strategy traceable in governance history.

For escalation recovery, the existing canonical event remains:

- `GovernanceEscalationRaised`

No separate event is required for each recovery strategy.

### 2.5 Continuation semantics

The affected G00 governance journey may continue only when:
- the recovery decision is authorized;
- any required source correction has been supplied by its canonical owner;
- the current aggregate remains at a valid lifecycle position;
- required recovery evidence has been recorded; and
- the applicable recovery event has been published.

A deferred journey may remain at its current valid lifecycle position until the governed deferral condition is satisfied.

## 3. Ownership Preservation

G00 owns the recovery decision and governance history only. G01–G10 retain ownership of operational source facts and operational lifecycle transitions.

## 4. Rejected Alternatives

1. Create a generic Governance Exception aggregate — rejected because existing G00 architecture treats exception handling as a workflow around existing aggregates.
2. Let Governance Coordinator approve all overrides — rejected because it collapses coordination and material decision authority.
3. Let G00 correct source-domain facts — rejected because it violates canonical ownership.
4. Create one event per recovery strategy — rejected as unnecessary duplication; one G00 recovery fact plus the existing escalation event is sufficient.
5. Make all exceptions Executive Reviewer decisions — rejected because it would over-centralize routine governance recovery.

## 5. Decision Status

**Approved and incorporated on 2026-09-20.**
