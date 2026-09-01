# Service Jurisdiction Change or Withdrawal

## 1. Specification Metadata

**Specification ID:** L4-E-021  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Position & Assignment  
**Realizes Scenario:** L3-E-021  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-021 — Service Jurisdiction Change or Withdrawal; L2-E02 — Position & Assignment Workflows  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall govern an authorised change or withdrawal of an existing Service Jurisdiction while preserving the underlying Position Assignment and its organisational placement.

L4 begins when an existing Service Jurisdiction and valid governing Position Assignment are available for an authorised change or withdrawal. L4 ends when the revised or withdrawn jurisdictional outcome is authoritative for its applicable period.

Position Assignment transfer, Position Definition lifecycle, Workforce Engagement lifecycle, and creation of a new organisational appointment remain outside this specification.

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

The L3 source identifies Service Jurisdiction as a Position & Assignment-owned outcome and does not define a distinct canonical event name for jurisdiction change or withdrawal. No event name is introduced here. Ownership of Workforce Engagement and organisational context remains with their canonical domains.

## 6. Failure / Rejection / Continuation

When the requested change or withdrawal cannot be validly established, the system shall retain the existing authoritative Service Jurisdiction and shall not replace it with an invalid outcome. A request requiring a different organisational appointment shall remain outside this behavior and follow the applicable Position Assignment lifecycle.

## 7. Audit & Evidence Behavior

The system shall preserve evidence sufficient to demonstrate:

- the existing Service Jurisdiction considered;
- the governing Position Assignment context;
- the authorisation basis for the change or withdrawal;
- the effective period;
- the resulting authoritative jurisdictional outcome;
- the prior jurisdiction history where a change occurred.

## 8. Observable Completion Conditions

The behavior is complete when:

- the revised or withdrawn Service Jurisdiction is authoritative for its applicable period;
- the governing Position Assignment remains valid and unchanged by this scenario;
- prior jurisdiction history remains preserved; and
- no new organisational appointment has been created by this behavior.

## 9. Traceability & Provenance

**Parent:** L3-E-021 — Service Jurisdiction Change or Withdrawal  
**Primary verification source:** L2-E02 — Position & Assignment Workflows  

The L3 parent explicitly states that no separate canonical Service Jurisdiction change/withdrawal event name was identified; L4 does not invent one.

## 10. Open Issues / Deferred Design

No L4 business-behavior issue is identified in the approved parent. Technical interface, persistence, transport, access-control implementation, retry/rollback mechanics, and UI remain downstream design concerns.

## Canonical Validation Note

This specification is a Draft and requires applicable L4 validation and governance review before Architecture Approval or Locking.
