# L2-G00 — Governance Resolution and Operational Maturity — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-G00 — Campus Operations Review & Governance and locked 2026-08-26 amendment |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Multi-Domain SLA governance completion

G00 does not create a separate SLA Deviation aggregate. The SLA deviation remains a source-owned performance fact.

A G00 SLA governance matter is complete when:

- the applicable Operational Review has reached `Closed`; and
- every Multi-Domain Action Plan required by that review has reached its canonical verified/closed outcome.

If the review determines that no corrective Action Plan is required, closure of the Operational Review is sufficient. `GovernanceReviewClosed`, correlated to the SLA matter, is the canonical completion fact. Existing Action Plan events remain authoritative for corrective work.

## 2. Operational Maturity Assessment

G00 owns **Operational Maturity Assessment**.

Canonical lifecycle:

`Planned → Evidence Collection → Evaluated → Reviewed → Closed`

A maturity assessment uses an institutionally governed, versioned maturity framework containing:

- ordered maturity levels or classifications;
- criteria and required evidence;
- optional weights or thresholds where institutionally configured;
- the rule for deriving the recorded maturity result;
- effective dates/version identity.

The architecture does not mandate universal maturity-level names. Every assessment records the framework version and evidence used so the result is reproducible.

Canonical roles:

- Governance Coordinator — creates/schedules the assessment and coordinates evidence.
- Campus Operations Manager — conducts the evaluation against the active framework.
- Executive Reviewer — confirms the final G00 maturity result and closes the assessment.

Canonical events published by G00:

- `OperationalMaturityAssessmentScheduled`
- `OperationalMaturityEvaluated`
- `OperationalMaturityAssessmentClosed`

Operational source domains retain ownership of contributing evidence.

## 3. Executive Governance Resolution

G00 owns **Executive Governance Resolution**, which records the institutionally binding G00 resolution of an escalated governance matter without taking ownership of the originating aggregate.

Lifecycle:

`Raised → Under Executive Review → Decided → Closed`

The **Executive Reviewer** is the canonical decision authority for the G00 Executive Governance Resolution. The decision records disposition, rationale, required follow-up and references to the originating governance item.

A G00 executive decision does not directly change another aggregate's lifecycle. Any required Risk, Action Plan, readiness, SLA, or source-domain transition must be performed by that aggregate's canonical owner through its own workflow.

G00 publishes `GovernanceExecutiveResolutionRecorded` when the resolution reaches `Decided`. Closure occurs after the resolution record and required hand-offs are complete.

## 4. Infrastructure risk and recovery governance contracts

G00 accepts governed referrals from G10 for material Infrastructure risk. G00 alone registers the resulting Operational Risk and publishes `OperationalRiskRegistered` when applicable.

For a disaster or major disruption crossing the configured campus-governance threshold, G00 owns the cross-campus governance review/coordination record; G10 remains owner of physical Infrastructure state and recovery facts.

## 5. Downstream effect

This amendment resolves the upstream gaps recorded by L3-G-007, L3-G-010 and L3-G-012 and supplies the governance side of L3-G-303 and L3-G-313.
