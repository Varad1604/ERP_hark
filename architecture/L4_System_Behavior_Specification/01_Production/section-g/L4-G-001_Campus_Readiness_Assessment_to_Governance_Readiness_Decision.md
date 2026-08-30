# Campus Readiness Assessment to Governance Readiness Decision

## 1. Specification Metadata

**Specification ID:** L4-G-001  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations Governance  
**Realizes Scenario:** L3-G-001  
**Primary Business Owner:** Campus Operations Governance  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-G-001_Campus_Readiness_Assessment_to_Governance_Readiness_Decision.md`; L2-G00 — Campus Operations Review & Governance; `L2-G00_Campus_Operations_Review_and_Governance_Amendment_2026-08-26.md`; ADR-G-READINESS-001  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern one configured Campus Readiness Assessment from scheduling through source-information collection, calculation, classification, executive review, final G00 governance decision and closure while preserving ownership of all source readiness facts and externally owned institutional lifecycles.

The behavior begins when a configured campus readiness assessment is scheduled and ends when the assessment reaches `Closed` with one canonical readiness classification, the Executive Reviewer decision, required canonical events and immutable governance history preserved. L4 shall not convert the G00 readiness decision into approval or state control over another domain's aggregate.

## 3. Required Business Context

- A supported readiness type, assessment rules and governance scope are configured.
- Required readiness information is available from its canonical G01–G10 owner or is handled through a source-supported governance exception path.
- Governing authorization conditions are satisfied.
- `Executive Reviewer` is the final G00 readiness-decision authority.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-001-BH-001 | When a valid configured readiness assessment is scheduled, the system shall create the G00 assessment at `Scheduled` without changing any source-domain readiness fact. | L3-G-001 §§4–5, 7–9, 13 |
| L4-G-001-BH-002 | The system shall obtain applicable readiness information only from canonical operational owners and shall preserve the source identity and ownership of each input. | L3-G-001 §§2, 5, 9, 12–13 |
| L4-G-001-BH-003 | The system shall prevent readiness calculation when required source information is missing, invalid, out of scope or not attributable to its canonical owner. | L3-G-001 §§7, 9, 11–12 |
| L4-G-001-BH-004 | When collection is valid, the system shall progress the assessment only through `Scheduled → Collecting → Calculated`. | L3-G-001 §§9, 12–13 |
| L4-G-001-BH-005 | On calculation, the system shall record exactly one source-defined classification: `Ready`, `Ready with Observations`, `Partially Ready`, or `Not Ready`. | L3-G-001 §§9–10, 12, 16 |
| L4-G-001-BH-006 | The system shall progress a calculated assessment to `Reviewed` only through the governed review path and shall preserve observations and material exceptions used in that review. | L3-G-001 §§9, 12–13, 15 |
| L4-G-001-BH-007 | The system shall permit the final G00 readiness decision and closure only under the `Executive Reviewer` authority and shall then place the assessment at `Closed`. | L3-G-001 §§3, 6, 9, 12–16 |
| L4-G-001-BH-008 | The system shall not use the G00 readiness decision to change, approve or take ownership of an Examination, admission, academic-delivery, event, emergency or other externally owned lifecycle. | L3-G-001 §§2–6, 10, 12–13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-001-BH-009 | On assessment scheduling, the system shall publish `CampusReadinessAssessmentScheduled` as a G00 fact. | L3-G-001 §§9, 14 |
| L4-G-001-BH-010 | On readiness calculation, the system shall publish `CampusReadinessCalculated` for all four canonical classifications. | L3-G-001 §§9–10, 12, 14 |
| L4-G-001-BH-011 | When and only when the exact classification is `Ready`, the system shall additionally publish `CampusReady`. | L3-G-001 §§10, 12, 14 |
| L4-G-001-BH-012 | When and only when the exact classification is `Not Ready`, the system shall additionally publish `CampusNotReady`. | L3-G-001 §§10, 12, 14 |

Section A may provide platform authorization, workflow, event-transport and audit capabilities. B, C, D, H and I may participate or consume the completed governance result within their own boundaries; no participation transfers readiness-source or institutional-lifecycle ownership to G00.

## 6. Failure / Rejection / Continuation

If required readiness information is missing or invalid, the system shall retain the assessment at its current valid lifecycle position and shall not manufacture a readiness result. Correction remains with the canonical source owner; deferral or escalation may occur only through source-supported governance behavior. If executive review or final decision cannot complete, the assessment shall remain outside `Closed`.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-001-BH-013 | The system shall preserve attributable evidence for assessment scope, source references, calculation inputs, classification, observations/exceptions, executive review, final decision, lifecycle transitions and canonical event publication. | L3-G-001 §§11, 15–16 |

## 8. Observable Completion Conditions

Completion requires the assessment to have progressed through `Scheduled → Collecting → Calculated → Reviewed → Closed`; one canonical classification to be recorded; the Executive Reviewer decision to be preserved; `CampusReadinessAssessmentScheduled` and `CampusReadinessCalculated` to have been published, with `CampusReady` or `CampusNotReady` only where their exact classification rule applies; and all source ownership to remain unchanged.

## 9. Traceability & Provenance

**Parent:** L3-G-001 — Campus Readiness Assessment to Governance Readiness Decision  
**Primary verification sources:** L2-G00 — Campus Operations Review & Governance; L2-G00 Locked Amendment 2026-08-26; ADR-G-READINESS-001

The Locked L2 amendment establishes the Executive Reviewer authority, the four readiness classifications, exact event semantics and Section A participation boundary consumed by the Locked parent.

## 10. Open Issues / Deferred Design

No L4 correctness issue remains. Implementation mechanisms, technical authorization design, event transport, persistence, APIs and UI behavior remain deferred to downstream architecture.