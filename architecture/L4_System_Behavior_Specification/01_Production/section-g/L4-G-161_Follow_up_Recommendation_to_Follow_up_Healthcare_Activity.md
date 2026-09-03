# Follow up Recommendation to Follow up Healthcare Activity

## 1. Specification Metadata

**Specification ID:** L4-G-161  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-161 — Follow-up Recommendation to Follow-up Healthcare Activity  
**Primary Business Owner:** Institutional Health Services (G06)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-161; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall preserve a G06 follow-up recommendation, determine its configured healthcare path, and hand the recommendation to the appropriate authoritative workflow without creating a universal follow-up lifecycle.

## 3. Required Business Context

- An originating Medical Encounter exists.
- An authorised healthcare professional has established a follow-up recommendation.
- Recommendation type and applicable configurable follow-up policy can be determined.
- C/E provide source-owned context; H owns notification delivery.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-161-BH-001 | Identify the originating Medical Encounter and record the follow-up recommendation and recommendation type. | L3-G-161 §§3,7–9 |
| L4-G-161-BH-002 | Apply the configured follow-up policy without replacing medical-professional judgement. | L3-G-161 §§9,12,15 |
| L4-G-161-BH-003 | Route Return Visit, Referral, Observation, Medical Certificate or Health Program Participation to the corresponding authoritative healthcare workflow. | L3-G-161 §§9–10,13 |
| L4-G-161-BH-004 | Maintain association with the originating Medical Encounter while the selected follow-up workflow owns its own lifecycle. | L3-G-161 §§9,12–13 |
| L4-G-161-BH-005 | Preserve the resulting follow-up outcome in immutable healthcare history and provide configured notification facts to H where applicable. | L3-G-161 §§9,14–16 |
| L4-G-161-BH-006 | Do not create a generic `Follow-up Completed` state or universal completion rule across heterogeneous follow-up types. | L3-G-161 §§12–14,17 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-161-BH-007 | Publish `FollowUpRecommended` when the recommendation is established; G06 is the canonical publisher. No generic `FollowUpCompleted` event shall be invented. Downstream workflows publish only their own canonical events. | L3-G-161 §§9,14,16–17 |

## 6. Failure / Rejection / Continuation

If follow-up type, patient context or a downstream entry condition cannot be established, preserve the recommendation as unresolved. Do not fabricate referral, observation, certificate or programme outcomes.

## 7. Audit & Evidence Behavior

Preserve originating Medical Encounter, recommendation, type, responsible professional, applicable policy, downstream handoff/outcome and relevant lifecycle transitions.

## 8. Observable Completion Conditions

Completion occurs when `FollowUpRecommended` is published, the appropriate authoritative follow-up path is established, and its resulting business outcome or explicit unresolved condition is traceably preserved.

## 9. Traceability & Provenance

**Parent:** L3-G-161 — Follow-up Recommendation to Follow-up Healthcare Activity  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

No universal follow-up selection matrix, generic aggregate lifecycle or `FollowUpCompleted` event exists upstream. Technical scheduling and notification implementation are deferred.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
