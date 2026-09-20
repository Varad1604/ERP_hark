# ADR-FIN-ANALYTICS-001 — KPI Definition and Analytical Model Governance

| Field | Value |
|---|---|
| ADR ID | ADR-FIN-ANALYTICS-001 |
| Change Type | L2 business-lifecycle, authority and event clarification |
| Target Locked Document | L2-F13 — Financial Reporting & Analytics Workflows |
| Blocked Downstream Scenario | L3-F-076 — Financial Definition Change to Approved KPI or Analytical Model |
| Status | Approved — Incorporated |
| Decision Effect | Effective through the locked L2-F13 amendment dated 2026-09-20 |
| Domain | Section F — Finance / Financial Reporting & Analytics |
| Date | 2026-09-20 |
| Authority | User-authorized controlled upstream remediation in SutraERP_Level_3 project |

---

## 1. Need

Locked L2-F13 owns KPI Definitions and Analytical Models and requires reporting configuration to be version controlled, but it does not explicitly bind those two configuration types to an authoritative lifecycle, does not define the business authority that approves a successor definition/model for effective use, and does not define aggregate-specific governance events for approval, activation, supersession or archival.

L3-F-076 and L4-F-076 therefore stop before the intended approved/effective successor outcome. L6 Stage-2 Group 5 correctly records that gap as a material upstream blocker.

## 2. Architecture Constraints

The resolution shall:

- preserve Financial Reporting & Analytics ownership of KPI Definitions and Analytical Models;
- preserve canonical source-financial-fact ownership in the existing Finance domains;
- reuse the existing L2-F00 Finance configuration lifecycle rather than create a competing F13 lifecycle;
- preserve historical calculation/reporting context and prevent in-place rewriting of prior definitions or historical results;
- use the existing L2-F00 institution-configurable Finance approval hierarchy and SoD principles;
- preserve H07 ownership of dashboard composition, layout, widgets, audience assignment and publication;
- assign every new business event exactly one canonical publisher;
- introduce no API, database, UI, technical RBAC, queue, deployment, persistence or transport design.

## 3. Decision

### 3.1 Canonical configuration classification

KPI Definition and Analytical Model are Finance-owned reporting configurations under F13. For governance purposes, both consume the canonical L2-F00 Finance configuration lifecycle:

```text
Draft → Validation → Approval → Published → Active → Superseded → Archived
```

Only an `Active` KPI Definition or Analytical Model may govern new eligible financial KPI calculations or analytical projections.

A changed definition/model is represented by a distinct successor version. The predecessor is never edited in place.

### 3.2 Canonical approval authority

**Financial Analytics Definition Approval Authority** is the canonical F13 business authority for approval of a new or revised KPI Definition or Analytical Model version.

This is an institution-independent business authority, not a universal job title. Institutional policy maps it to the applicable level of the existing L2-F00 Finance approval hierarchy.

Preparation/review may be performed by F13-authorized business participants under institutional policy. Approval remains distinct from preparation where the configured Finance SoD policy requires separation.

### 3.3 Effective-version and supersession semantics

- A candidate that has not completed Validation and Approval shall not become Published or Active.
- `Published` means the approved version is available for governed effective use but is not yet the effective version until activation.
- `Active` is the only version eligible to govern new calculations/projections for its applicable scope and effective boundary.
- When an Active successor replaces an Active predecessor for the same governed scope, the predecessor becomes `Superseded`.
- A Superseded version may later become `Archived` under retention governance.
- Historical KPI results, analytical projections and governed reports retain the exact definition/model version used when they were produced.
- Activation of a successor does not trigger rewriting or re-evaluation of historical results.

### 3.4 Canonical governance events and publishers

KPI Definition Aggregate / F13 is the sole publisher of:

- `KPIDefinitionApproved`
- `KPIDefinitionActivated`
- `KPIDefinitionSuperseded`
- `KPIDefinitionArchived`

Analytical Model Aggregate / F13 is the sole publisher of:

- `AnalyticalModelApproved`
- `AnalyticalModelActivated`
- `AnalyticalModelSuperseded`
- `AnalyticalModelArchived`

Existing `FinancialKPICalculated` remains the KPI calculation-completion event. Existing `FinancialDashboardProjectionUpdated` remains the analytical projection-completion event. Neither event is a substitute for definition/model governance events.

Each governance transition publishes its corresponding completed business fact only after that transition succeeds.

### 3.5 Ownership and downstream use

F13 definition/model governance may reference canonical Finance facts but may not modify them. H07 may consume F13-owned KPI/projection outputs for dashboard presentation but does not own KPI Definition, Analytical Model or their lifecycle.

## 4. Impact Analysis

### L0 / L1

No change required. Finance ownership remains unchanged.

### Dependency Map

No dependency change is introduced. F13 continues to consume canonical Finance facts and supply governed financial information products/projections to authorized consumers including H07.

### L2

A locked L2-F13 amendment incorporates the configuration lifecycle binding, approval authority, effective-version semantics and event vocabulary defined here.

### L3

L3-F-076 can now complete its frozen outcome without inventing lifecycle, authority or event semantics. Scenario identity, owner, participating domains, priority and title remain unchanged.

### L4

L4-F-076 can now derive observable approval, activation, supersession and historical-version behavior from L2/L3 rather than stopping at a validated candidate.

### L6

Stage-2 Group 5 may classify the F076 normalized workflow as a governed review/approval workflow with mock interaction, subject to successful L3/L4 revalidation.

## 5. Rejected Alternatives

1. Create an independent F13-only lifecycle — rejected because L2-F00 already defines the canonical lifecycle for every Finance configuration.
2. Reuse Financial Report lifecycle states for KPI Definitions/Analytical Models — rejected because reports and definitions/models are different aggregates with different business meaning.
3. Reuse Financial Report Approval Authority — rejected because report publication approval and analytical-definition approval are distinct governance responsibilities.
4. Use one shared event name for KPI Definition and Analytical Model — rejected because one event must have exactly one canonical publisher.
5. Treat `FinancialKPICalculated` or `FinancialDashboardProjectionUpdated` as definition activation — rejected because they represent calculation/projection completion, not configuration governance.
6. Leave the authority/configuration semantics to L3/L4/L6 — rejected because downstream levels may not invent upstream business lifecycle or event ownership.

## 6. Acceptance Criteria

1. KPI Definition and Analytical Model each have an authoritative lifecycle.
2. The lifecycle reuses, rather than duplicates, L2-F00 Finance configuration governance.
3. One canonical business approval authority governs successor definition/model approval.
4. Only Active versions govern new eligible calculations/projections.
5. Historical results preserve their governing version.
6. Every new event has one canonical publisher.
7. No source Finance ownership or H07 dashboard ownership changes.
8. L3-F-076 and L4-F-076 can complete without an upstream blocker.
9. No implementation architecture is introduced.

## 7. Decision Status

**Approved and incorporated on 2026-09-20 under the user's explicit request to resolve the Stage-2 Finance upstream issue.**
