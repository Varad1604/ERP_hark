# L2-F13 — KPI Definition and Analytical Model Governance — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F13 — Financial Reporting & Analytics Workflows and locked amendments |
| Amendment Version | 1.3.0 |
| Status | Locked |
| Effective Date | 2026-09-20 |
| Approved Through | ADR-FIN-ANALYTICS-001 |

This amendment is part of the canonical L2-F13 architecture. Where it conflicts with earlier F13 text concerning KPI Definition or Analytical Model governance, this amendment is authoritative. Unaffected F13 content remains unchanged.

## 1. Canonical configuration lifecycle

KPI Definition and Analytical Model are F13-owned Finance configurations and consume the canonical L2-F00 Finance configuration lifecycle:

```text
Draft → Validation → Approval → Published → Active → Superseded → Archived
```

Rules:

1. A changed KPI Definition or Analytical Model is a distinct successor version; the predecessor is not edited in place.
2. A candidate must complete Validation and Approval before it can become Published or Active.
3. Only an `Active` version may govern new eligible KPI calculations or analytical projections for its applicable scope/effective boundary.
4. When an Active successor replaces the current Active predecessor for the same governed scope, the predecessor becomes `Superseded`.
5. A Superseded version may progress to `Archived` under retention governance.
6. Historical KPI results, analytical projections and governed reports retain the definition/model version under which they were produced.
7. Activation of a successor does not rewrite or re-evaluate historical results.

## 2. Canonical business authority

**Financial Analytics Definition Approval Authority** is the canonical F13 business authority for approving new or revised KPI Definition and Analytical Model versions.

The authority is institution-independent and is mapped by institutional policy to the applicable level of the L2-F00 Finance approval hierarchy. It does not create a new platform role or transfer authorization-policy ownership from Platform Foundation.

F13-authorized business participants may prepare and review candidate definitions/models under institutional policy. Existing Finance segregation-of-duties controls apply where configured; approval shall not be inferred merely from preparation or validation.

## 3. Canonical governance events

### KPI Definition Aggregate / F13

The KPI Definition Aggregate is the sole canonical publisher of:

- `KPIDefinitionApproved` — the candidate has received the governed approval required to progress toward effective use.
- `KPIDefinitionActivated` — the approved/published version has become Active for new eligible KPI calculations.
- `KPIDefinitionSuperseded` — an Active predecessor has been replaced as the current effective version by an Active successor.
- `KPIDefinitionArchived` — a Superseded version has reached governed historical archival.

### Analytical Model Aggregate / F13

The Analytical Model Aggregate is the sole canonical publisher of:

- `AnalyticalModelApproved`
- `AnalyticalModelActivated`
- `AnalyticalModelSuperseded`
- `AnalyticalModelArchived`

Each event records only a completed business transition of its owning aggregate.

Existing `FinancialKPICalculated` remains the KPI calculation-completion event. Existing `FinancialDashboardProjectionUpdated` remains the analytical projection-completion event. They do not represent definition/model approval or activation.

## 4. Validation and failure behavior

A proposed successor shall not progress to Active when:

- ownership/scope cannot be established;
- canonical source references are invalid or would transfer source-fact ownership;
- version consistency or deterministic calculation/model behavior cannot be validated;
- required approval is absent; or
- activation would destroy historical reproducibility.

No additional failure lifecycle state is introduced by this amendment. The candidate remains outside Active until corrected and governed through the canonical lifecycle.

## 5. Audit and historical obligations

Governance evidence shall preserve:

- definition/model identity and version;
- governed scope and owner;
- predecessor/successor lineage;
- validation evidence;
- approving authority and approval reference;
- Published and Active effective boundaries;
- Superseded/Archived disposition where applicable; and
- historical calculation/projection/report references to the exact governing version.

## 6. Ownership preservation

- F13 retains ownership of KPI Definition, Analytical Model and their governance semantics.
- Operational Finance domains retain ownership of source financial facts.
- H07 retains ownership of dashboard identity, composition, layout, widgets, audience assignment and publication.
- Platform Foundation retains technical authorization/workflow/audit infrastructure ownership.
- This amendment defines business architecture only.

## 7. Downstream effect

This amendment resolves the lifecycle, approval-authority and event-publisher gaps recorded by `L3-F-076` and `L4-F-076`.

L3-F-076 shall consume this amendment and ADR-FIN-ANALYTICS-001. L4-F-076 shall derive its observable behavior from the amended L3 parent and this locked L2 decision.

## 8. Locked Decisions

The lifecycle binding, Financial Analytics Definition Approval Authority, effective-version semantics, event names, event publishers, ownership safeguards and historical-version rules in §§1–7 are **Locked L2 decisions** under ADR-FIN-ANALYTICS-001.
