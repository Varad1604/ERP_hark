# Visitor Request to Closed Hostel Visit

## 1. Specification Metadata

**Specification ID:** L4-G-084  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-084 — Visitor Request to Closed Hostel Visit  
**Primary Business Owner:** Hostel Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-084; L2-G03 — Hostel Management Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Govern a hostel visitor request from registration through approval, authorised entry, exit and closure while preserving Hostel Management ownership of Visitor Entry.

## 3. Required Business Context

- Sponsoring resident has active hostel residency.
- Visitor and resident context are identifiable.
- Applicable visitor policy and configured hostel approval authority are available.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-084-BH-001 | The system shall register the visitor against an active resident and establish `Registered`. | L3-G-084 §§7–9, 13 |
| L4-G-084-BH-002 | The system shall evaluate the request against configured visitor policy and required approval conditions. | L3-G-084 §§9, 12, 15 |
| L4-G-084-BH-003 | When authorised, the system shall establish `Approved`; an unauthorised request shall not enter the authorised visit lifecycle. | L3-G-084 §§9–13 |
| L4-G-084-BH-004 | On authorised entry, the system shall establish `Checked-In`. | L3-G-084 §§9, 13 |
| L4-G-084-BH-005 | On visitor exit, the system shall establish `Checked-Out`. | L3-G-084 §§9, 13 |
| L4-G-084-BH-006 | After exit is validly recorded, the system shall establish `Closed` and preserve the complete visit history. | L3-G-084 §§3, 9, 13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-084-BH-007 | G03 Hostel Management remains the canonical publisher of visitor approval, entry, exit and closure facts; no exact event name shall be invented because the Locked parent records the event identifiers as unresolved. | L3-G-084 §§14, 17 |
| L4-G-084-BH-008 | Security and Notifications may consume visitor information/outcomes without acquiring ownership of Visitor Entry. | L3-G-084 §§5, 14 |

## 6. Failure / Rejection / Continuation

- Incomplete or inconsistent visitor information shall prevent authoritative approval.
- Missing approval shall prevent authorised entry.
- A visit shall not be falsely closed if expected exit is not established.

## 7. Audit & Evidence Behavior

Preserve visitor registration, resident relationship, policy review, authority decision, entry, exit and closure history.

## 8. Observable Completion Conditions

Completion requires the request to reach a governed terminal outcome and any authorised visit to reach `Closed` with history retained.

## 9. Traceability & Provenance

**Parent:** L3-G-084 — Visitor Request to Closed Hostel Visit  
**Primary verification source:** L3-G-084 §§3, 5, 9, 11–17 and authoritative L2-G03 architecture.

## 10. Open Issues / Deferred Design

Exact canonical Visitor Entry event names remain unresolved upstream. Technical identity verification and notification delivery are deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
