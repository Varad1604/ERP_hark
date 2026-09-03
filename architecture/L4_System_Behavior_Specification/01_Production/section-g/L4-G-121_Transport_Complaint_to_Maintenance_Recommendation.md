# Transport Complaint to Maintenance Recommendation

## 1. Specification Metadata

**Specification ID:** L4-G-121  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-121 — Transport Complaint to Maintenance Recommendation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-121; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall assess a maintenance-relevant transport complaint and establish the authoritative G04 maintenance recommendation and G01 business hand-off without claiming maintenance execution.

## 3. Required Business Context

- A registered transport complaint and relevant operational vehicle context are identifiable.
- G04 owns transport-side assessment and maintenance recommendation.
- G01 owns subsequent maintenance execution.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-121-BH-001 | Identify the maintenance-relevant complaint and affected transport/vehicle context. | L3-G-121 §§3, 7–9 |
| L4-G-121-BH-002 | Assess the available business evidence and determine whether maintenance attention is required. | L3-G-121 §§9, 11–12 |
| L4-G-121-BH-003 | Establish the G04 maintenance recommendation only when supported by the operational assessment and required business review. | L3-G-121 §§3, 9, 15–16 |
| L4-G-121-BH-004 | Make the authoritative recommendation and supporting transport context available to the G01 maintenance process without representing diagnosis, repair, work-order or maintenance completion as a G04 outcome. | L3-G-121 §§3–5, 9, 12, 16 |
| L4-G-121-BH-005 | Preserve complaint, assessment, recommendation, review and hand-off history. | L3-G-121 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-121-BH-006 | When the maintenance recommendation is established, Transport Management shall publish `MaintenanceRecommended`; G01 remains the owner of subsequent maintenance execution. | L3-G-121 §§14, 16 |

## 6. Failure / Rejection / Continuation

- Insufficient evidence shall prevent an unsupported maintenance recommendation.
- Where maintenance is not required, no maintenance recommendation shall be fabricated; the complaint returns to its applicable G04 operational-resolution path.
- Rejection during configured G04 review shall prevent the recommendation from becoming authoritative.
- G01 non-completion shall never be represented as G04 maintenance completion.

## 7. Audit & Evidence Behavior

Preserve complaint evidence, affected transport/vehicle context, operational assessment, recommendation basis, review decision, `MaintenanceRecommended` publication and G01 hand-off evidence.

## 8. Observable Completion Conditions

The authoritative G04 maintenance recommendation is established, `MaintenanceRecommended` is published, the recommendation is available to G01, and no downstream maintenance completion is claimed.

## 9. Traceability & Provenance

**Parent:** L3-G-121 — Transport Complaint to Maintenance Recommendation  
**Primary verification source:** L3-G-121 §§3, 5, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No material event or ownership gap remains for the recommendation outcome. G01 maintenance execution remains outside this L4 boundary. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
