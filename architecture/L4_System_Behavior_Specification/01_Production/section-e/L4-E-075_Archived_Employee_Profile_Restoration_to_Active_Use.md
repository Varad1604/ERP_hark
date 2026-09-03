# Archived Employee Profile Restoration to Active Use

## 1. Specification Metadata

**Specification ID:** L4-E-075  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-075 — Archived Employee Profile Restoration to Active Use  
**Primary Business Owner:** Employee Profile  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-075; L2-E07 — Employee Profile; L2-E01 — Workforce & Engagement  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Restore the existing archived EmployeeProfile through the canonical `Archived → Restored → Active` path while preserving its historical identity and prior archival history. Restoration does not recreate Person, Identity, Workforce Engagement or Position Assignment.

## 3. Required Business Context

- Existing EmployeeProfile in `Archived`.
- Valid institutional business need for restoration.
- Applicable verification/compliance evidence where required.
- Confirmation that restoration will not create a duplicate EmployeeProfile.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-075-BH-001 | The system shall identify the existing archived canonical EmployeeProfile and establish the governed restoration request against that same profile. | L3-E-075 §§3, 7–9, 16 |
| L4-E-075-BH-002 | The system shall validate that restoration is permitted under Employee Profile governance and that any applicable verification or compliance conditions are satisfied. | L3-E-075 §§7, 9–12, 15 |
| L4-E-075-BH-003 | The system shall not create a second EmployeeProfile, Person, Identity, Workforce Engagement or Position Assignment as part of restoration. | L3-E-075 §§4.4, 5, 11–12, 16 |
| L4-E-075-BH-004 | When restoration conditions are satisfied, the system shall transition the profile from `Archived` to `Restored`. | L3-E-075 §§3, 9, 12–13 |
| L4-E-075-BH-005 | The system shall then transition the restored profile to `Active` for operational Employee Profile use. | L3-E-075 §§3, 9, 13, 16 |
| L4-E-075-BH-006 | The system shall preserve the prior archival position, restoration history and version lineage. | L3-E-075 §§9, 12–13, 15–16 |
| L4-E-075-BH-007 | The Active restored profile shall be available as the same canonical EmployeeProfile to dependent consumers without transferring Employee Profile ownership. | L3-E-075 §§3–5, 9, 14.2, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-075-BH-008 | Workforce & Engagement and other dependent capabilities may consume the restored canonical profile fact but shall remain owners of their own lifecycles. | L3-E-075 §§4.4–5, 14.2, 16 |
| L4-E-075-BH-009 | The system shall not invent a restoration event name because the Locked parent explicitly states that the accessible E07 event catalogue does not expose one. | L3-E-075 §§14.1, 17 |

## 6. Failure / Rejection / Continuation

- Ineligible archived profiles remain archived.
- Incomplete required verification prevents Active restoration.
- Compliance conditions that prevent restoration preserve the archived position.
- Restoration must not silently recreate or alter Workforce Engagement if that separate lifecycle is not active.

## 7. Audit & Evidence Behavior

The restoration request, eligibility decision, verification/compliance evidence, prior `Archived` position, `Restored` transition, resulting `Active` position and version lineage shall remain traceable.

## 8. Observable Completion Conditions

The same canonical EmployeeProfile has moved through `Archived → Restored → Active`, prior history remains preserved, no duplicate profile/identity has been created, and separately owned workforce/assignment lifecycles remain unchanged.

## 9. Traceability & Provenance

**Parent:** L3-E-075 — Archived Employee Profile Restoration to Active Use  
**Primary verification source:** L3-E-075 §§3, 5, 9, 11–17 and authoritative L2-E07 architecture.

## 10. Open Issues / Deferred Design

The exact restoration event name remains unavailable in the accessible authoritative E07 event vocabulary. L4 preserves this upstream gap. Technical archive restoration and persistence mechanics remain deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
