# Service Jurisdiction Change or Withdrawal

## 1. Specification Metadata

**Specification ID:** L4-E-021  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-021 — Service Jurisdiction Change or Withdrawal  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-021; applicable locked E architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised change or withdrawal of an existing Service Jurisdiction while preserving the underlying Position Assignment and its organisational placement. L4 begins when an existing Service Jurisdiction and valid governing Position Assignment are available for an authorised change or withdrawal. L4 ends when the revised or withdrawn jurisdictional outcome is authoritative for its applicable period. Position Assignment transfer, Position Definition lifecycle, Workforce Engagement lifecycle, and creation of a new organisational appointment remain outside this specification.

## 3. Required Business Context

- Existing Service Jurisdiction.
- Valid governing Position Assignment.
- Authorised change or withdrawal decision.
- Applicable organisational policy and effective-period context.
- Authoritative organisational and workforce context where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-021-BH-001 | When an authorised Service Jurisdiction change is submitted, the system shall validate the existing Service Jurisdiction and its governing Position Assignment before accepting the change. | L3-E-021 §§4, 7, 9 |
| L4-E-021-BH-002 | The system shall prevent a jurisdiction change when the governing Position Assignment is not valid or the proposed jurisdiction does not satisfy applicable organisational policy. | L3-E-021 §§4.3, 7, 10, 12 |
| L4-E-021-BH-003 | When a valid future-effective jurisdiction change is accepted, the system shall preserve the current service boundary until the approved effective boundary. | L3-E-021 §§9, 10, 13 |
| L4-E-021-BH-004 | When a jurisdiction is withdrawn, the system shall remove the jurisdictional scope without changing the underlying Position Assignment lifecycle. | L3-E-021 §§4.2, 10, 12, 13 |
| L4-E-021-BH-005 | The system shall preserve the prior Service Jurisdiction history when a revised or withdrawn outcome becomes authoritative. | L3-E-021 §§4.3, 9, 11, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-021-BH-006 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Position & Assignment ownership. | L3-E-021 §14 |

## 6. Failure / Rejection / Continuation

- When the requested change or withdrawal cannot be validly established, the system shall retain the existing authoritative Service Jurisdiction and shall not replace it with an invalid outcome. A request requiring a different organisational appointment shall remain outside this behavior and follow the applicable Position Assignment lifecycle.

## 7. Audit & Evidence Behavior

The system shall preserve evidence sufficient to demonstrate: - the existing Service Jurisdiction considered; - the governing Position Assignment context; - the authorisation basis for the change or withdrawal; - the effective period; - the resulting authoritative jurisdictional outcome; - the prior jurisdiction history where a change occurred.

## 8. Observable Completion Conditions

The behavior is complete when: - the revised or withdrawn Service Jurisdiction is authoritative for its applicable period; - the governing Position Assignment remains valid and unchanged by this scenario; - prior jurisdiction history remains preserved; and - no new organisational appointment has been created by this behavior.

## 9. Traceability & Provenance

**Parent:** L3-E-021 — Service Jurisdiction Change or Withdrawal  
**Primary verification source:** Authoritative People & Workforce L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
