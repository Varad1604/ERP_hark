# L2-G00 — Governance Exception Recovery — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-G00 — Campus Operations Review & Governance, §28 Exception Handling |
| Amendment Version | 1.3.0 |
| Status | Locked |
| Effective Date | 2026-09-20 |
| Approved Through | ADR-G-EXCEPTION-001 |

This amendment is part of canonical L2-G00. Where it conflicts with earlier G00 text concerning exception-recovery authority or event semantics, this amendment is authoritative. Existing G00 ownership and aggregate lifecycles remain unchanged.

## 1. Exception model

A **Governance Exception** is a workflow condition affecting an existing G00 governance journey. It is not a separate aggregate and has no independent lifecycle.

The affected Campus Readiness Assessment, Operational Review, Operational Risk or Multi-Domain Action Plan remains at its last valid lifecycle position until authorized recovery conditions are satisfied.

## 2. Canonical business responsibilities

| Role | Responsibility |
|---|---|
| Governance Coordinator | Identify/coordinate the exception, obtain corrected source evidence, maintain recovery evidence and support revalidation. |
| Campus Operations Manager | Decide ordinary G00 recovery strategy within configured policy: source-correction acceptance, review rescheduling, action reassignment, risk reassessment, governance-review extension, manual verification, or time-bounded deferral. |
| Executive Reviewer | Decide Administrative Override and any recovery escalated beyond Campus Operations Manager authority by configured governance policy. |

## 3. Recovery constraints

1. G00 shall never correct or mutate an operational fact owned by G01–G10.
2. A required source correction must be completed by the canonical source owner and then revalidated by G00.
3. Manual verification shall not fabricate or replace a missing authoritative source fact.
4. Deferral must be bounded by an explicit time or condition and preserve the affected aggregate at a valid lifecycle position.
5. Administrative Override may apply only to a G00-owned governance-process requirement that institutional policy explicitly permits to be overridden.
6. Administrative Override shall not override statutory obligations, another domain's lifecycle/ownership, or mandatory source evidence.
7. A recovery outside ordinary authority shall use the existing G00 executive-escalation path.
8. Recovery behavior remains configuration driven within these locked authority and ownership constraints.

## 4. Canonical events

G00 / Campus Operations Review is the sole canonical publisher of:

- `GovernanceExceptionRecoveryRecorded` — published after a non-escalation source-supported recovery decision is authorized and recorded.

Existing:

- `GovernanceEscalationRaised` — remains the canonical event when escalation is the selected recovery.

The generic §28 `Domain Event Published` step is satisfied by the applicable event above. No dedicated Governance Exception aggregate is implied.

## 5. Continuation rule

An affected G00 governance journey may continue only when:
- a recovery strategy has been selected by the applicable canonical authority;
- required source facts are available from their canonical owner;
- the affected G00 aggregate remains in a valid lifecycle position;
- recovery evidence is auditable; and
- the applicable recovery event has been published.

A deferred journey may remain at its current valid lifecycle position until the deferral condition is met.

## 6. Downstream resolution

This amendment resolves the authority/event gaps recorded by `L3-G-013` and `L4-G-013`.

## 7. Locked Decisions

Sections 1–6 are **Locked L2 decisions** under ADR-G-EXCEPTION-001.
