# Transport Pass Renewal to Continued Transport Authorisation

## 1. Specification Metadata

**Specification ID:** L4-G-114  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-114 — Transport Pass Renewal to Continued Transport Authorisation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-114; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern renewal of an existing Transport Pass so that eligible transport authorisation continues without changing canonical ownership.

## 3. Required Business Context

- Existing Transport Pass is identifiable.
- Continued eligibility and validity policy are authoritative.
- Transport Management owns the pass lifecycle.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-114-BH-001 | Identify the existing Transport Pass and its renewal context. | L3-G-114 §§3, 7, 9 |
| L4-G-114-BH-002 | Evaluate continued eligibility and applicable renewal conditions. | L3-G-114 §§9, 12 |
| L4-G-114-BH-003 | Record the governed renewal decision after required approval. | L3-G-114 §§9, 15 |
| L4-G-114-BH-004 | Establish the authorised renewed validity period and continued transport authorisation. | L3-G-114 §§3, 9, 16 |
| L4-G-114-BH-005 | Preserve prior validity and renewal history. | L3-G-114 §§12, 15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-114-BH-006 | On successful renewal, Transport Management shall publish `PassRenewed`. | L3-G-114 §§14, 16 |

## 6. Failure / Rejection / Continuation

- Failed eligibility or approval shall prevent continued authorisation.
- Missing authoritative context shall prevent final renewal.
- A replacement rather than renewal requirement shall follow the replacement-pass scenario.

## 7. Audit & Evidence Behavior

Preserve renewal request, eligibility assessment, authority, validity change and prior-pass history.

## 8. Observable Completion Conditions

The authorised renewal is effective, continued transport authorisation is established for the renewed period, `PassRenewed` is published, and historical validity remains traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-114 — Transport Pass Renewal to Continued Transport Authorisation  
**Primary verification source:** L3-G-114 §§3, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

No unresolved event identifier remains for the core renewal outcome. Institution-specific policy and technical implementation remain upstream/downstream governed respectively.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
