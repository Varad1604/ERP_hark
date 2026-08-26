# ADR-G-READINESS-001 — Campus Readiness Decision Authority and Event Semantics

| Field | Value |
|---|---|
| ADR ID | ADR-G-READINESS-001 |
| Change Type | L2 business-authority and event-semantics clarification |
| Target Locked Document | L2-G00 — Campus Operations Review & Governance |
| Affected Downstream Scenario | L3-G-001 — Campus Readiness Assessment to Governance Readiness Decision |
| Status | Approved — Incorporated |
| Decision Effect | Effective through the locked L2-G00 amendment dated 2026-08-26 |
| Domain | Section G — Campus Operations / Campus Operations Governance |
| Date | 2026-08-26 |

---

## 1. Need

Locked L2-G00 defines the Campus Readiness Assessment, its lifecycle, four readiness classifications, the `Executive Reviewer` role, and the events `CampusReadinessAssessmentScheduled`, `CampusReadinessCalculated`, `CampusReady`, and `CampusNotReady`. It does not explicitly assign the final G00 readiness-decision authority, does not state how the two qualified classifications map to event publication, and leaves Section A participation implicit.

L3-G-001 therefore preserved these as upstream issues rather than inventing a final authority or new events.

## 2. Architecture Constraints

The resolution shall:

- preserve G00 ownership of the Campus Readiness Assessment and governance readiness result;
- preserve source-domain ownership of operational readiness indicators;
- preserve admission, examination, academic-delivery, event, emergency, and other associated activity ownership in their respective domains;
- reuse existing G00 business roles where source architecture supports them;
- avoid adding events where existing event semantics are sufficient;
- preserve Platform Foundation ownership of authorization, workflow, event transport, and audit capabilities without treating A as a readiness-data owner.

## 3. Decision

### 3.1 Canonical Readiness Decision Authority

`Executive Reviewer` is the canonical G00 business authority responsible for the final governance readiness decision and closure of a Campus Readiness Assessment after the configured readiness result has been calculated and reviewed.

This authority is limited to the G00 campus-operational readiness assessment. It does not authorize the Executive Reviewer to change the lifecycle of an Examination, admission, academic session, event, emergency process, or any other business aggregate owned outside G00.

`Governance Coordinator` remains responsible for creating/coordinating the readiness assessment. `Campus Operations Manager` remains responsible for operational review where that review is required by the configured governance path.

### 3.2 Canonical Classification and Event Semantics

The four canonical readiness classifications remain:

- `Ready`
- `Ready with Observations`
- `Partially Ready`
- `Not Ready`

Event semantics are:

1. `CampusReadinessAssessmentScheduled` is published when the assessment is scheduled.
2. `CampusReadinessCalculated` is published whenever the configured readiness calculation completes and identifies the resulting canonical classification.
3. `CampusReady` is additionally published only when the resulting classification is exactly `Ready`.
4. `CampusNotReady` is additionally published only when the resulting classification is exactly `Not Ready`.
5. No new event is introduced for `Ready with Observations` or `Partially Ready`; those classifications are communicated through the canonical `CampusReadinessCalculated` fact and the closed assessment's result.

This preserves one event vocabulary while making all four classifications externally interpretable.

### 3.3 Platform Foundation Participation

Section A participation in the G00 readiness scenario is limited to Platform Foundation capabilities used by governed readiness processing, including authorization-policy evaluation, workflow/orchestration capability, event transport, and audit evidence support where applicable.

Section A does not publish Campus Operations readiness indicators, does not own the Campus Readiness Assessment, and does not make the G00 business readiness decision.

## 4. Impact Analysis

### L0 / L1

No change required. Domain ownership remains unchanged.

### Dependency Map

No change required. G00 remains within Section G and may depend on Platform Foundation capabilities without transferring business ownership.

### L2

A locked L2-G00 amendment incorporates decision authority, event semantics, and Section A participation.

### L3

L3-G-001 can name `Executive Reviewer` as final G00 readiness-decision authority, use existing events for all classifications, and state the exact Platform Foundation participation without open issues.

## 5. Rejected Alternatives

1. Invent a new `CampusReadinessDecisionAuthority` role — rejected because `Executive Reviewer` already exists in locked G00 and the workflow includes Executive Review.
2. Add events for every classification — rejected because `CampusReadinessCalculated` can carry/represent the complete calculated classification without proliferating event names.
3. Let G00 approve or cancel Examination/admission/academic lifecycles — rejected because G00 owns operational-readiness governance only.
4. Treat Platform Foundation as a readiness-data owner — rejected because participation does not transfer business ownership.

## 6. Acceptance Criteria

1. The final G00 readiness-decision authority is explicit.
2. All four classifications have unambiguous event semantics.
3. No duplicate classification events are introduced.
4. G00 remains the owner/publisher of readiness governance facts.
5. Section A participation is explicit and non-owning.
6. Associated business domains retain their lifecycles and decision authority.
7. L3-G-001 has no remaining upstream ambiguity from these findings.

## 7. Decision Status

**Approved and incorporated on 2026-08-26 under the explicit repository-change authorization recorded in the architecture working session.**
