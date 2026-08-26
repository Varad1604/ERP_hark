# L2-G00 — Campus Operations Review & Governance — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-G00 — Campus Operations Review & Governance v1.0 |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-G-READINESS-001 |
| Authority | Explicit repository-change authorization recorded 2026-08-26 |

This amendment is part of the canonical L2-G00 architecture. Where this amendment conflicts with L2-G00 v1.0, this amendment is authoritative. Unaffected L2-G00 content remains unchanged.

## 1. Campus Readiness Business Authority

For the Campus Readiness Assessment workflow:

| Business actor | Canonical responsibility |
|---|---|
| Governance Coordinator | Creates and coordinates the Campus Readiness Assessment within authorized scope. |
| Campus Operations Manager | Conducts operational review where required by the configured governance path. |
| Executive Reviewer | Makes the final G00 governance readiness decision and closes the assessment after calculation and review. |

The Executive Reviewer's decision is a Campus Operations governance decision only. It does not change or approve the lifecycle of an Examination, admission, academic-delivery, event, emergency, or other aggregate owned by another domain.

## 2. Readiness Classification Event Semantics

The four existing canonical readiness classifications remain unchanged:

- `Ready`
- `Ready with Observations`
- `Partially Ready`
- `Not Ready`

The event rules are:

| Condition | Canonical event behaviour |
|---|---|
| Assessment scheduled | Publish `CampusReadinessAssessmentScheduled`. |
| Readiness calculation completes with any of the four classifications | Publish `CampusReadinessCalculated` with/for the calculated classification. |
| Classification = `Ready` | Additionally publish `CampusReady`. |
| Classification = `Not Ready` | Additionally publish `CampusNotReady`. |
| Classification = `Ready with Observations` | No new event name; `CampusReadinessCalculated` and the assessment result are authoritative. |
| Classification = `Partially Ready` | No new event name; `CampusReadinessCalculated` and the assessment result are authoritative. |

G00 remains the canonical publisher of these readiness-governance events. Event transport remains owned by Platform Foundation.

## 3. Section A Participation

Where Platform Foundation / Section A participates in Campus Readiness governance, its responsibility is limited to governed platform capabilities such as authorization-policy evaluation, workflow/orchestration support, event transport, and audit evidence support as applicable.

Section A:

- does not own the Campus Readiness Assessment;
- does not publish operational readiness indicators on behalf of G01–G10;
- does not determine the G00 readiness classification;
- does not make the G00 final readiness decision.

## 4. Ownership Preservation

Operational domains retain ownership of their own readiness facts. G00 consumes those facts and owns only the consolidated Campus Readiness Assessment, its classification, governance review, decision and history.

The G00 decision may inform an associated institutional activity but does not take ownership of that activity's lifecycle.

## 5. Downstream Effect

L3-G-001 shall consume this amendment and ADR-G-READINESS-001. The former issues concerning final authority, intermediate-classification events, and Section A participation are resolved.

## 6. Locked Decisions

The `Executive Reviewer` authority, classification-event semantics, Platform Foundation participation boundary, and ownership rules in §§1–5 are Locked L2 decisions under ADR-G-READINESS-001.
