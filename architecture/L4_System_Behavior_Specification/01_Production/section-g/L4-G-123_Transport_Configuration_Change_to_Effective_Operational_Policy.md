# L4-G-123 — Transport Configuration Change to Effective Operational Policy

## 1. Specification Metadata
**Specification ID:** L4-G-123  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Transport Management  
**Realizes Scenario:** L3-G-123 — Transport Configuration Change to Effective Operational Policy  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern a transport configuration change through validation, authorisation, effective adoption, and historical traceability without defining technical configuration mechanisms.

**Start:** A governed transport configuration change is proposed.  
**End:** The authorised configuration becomes effective operational policy or reaches a governed rejection outcome.

## 3. Required Business Context
- An existing authoritative transport configuration context exists.
- Applicable transport governance and approval authority are available.
- Transport Management owns the configuration lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-123-BH-001 | Register the proposed transport configuration change against its authoritative context. | L3-G-123 |
| L4-G-123-BH-002 | Validate the proposed change against applicable transport policy and operational constraints. | L3-G-123 |
| L4-G-123-BH-003 | Apply the governed authorisation and approval path. | L3-G-123 |
| L4-G-123-BH-004 | When approved and effective, establish the changed configuration as the applicable operational policy. | L3-G-123 |
| L4-G-123-BH-005 | Preserve the superseded configuration and change history for traceability. | L3-G-123 |

## 5. Cross-Domain & Event Behavior
The effective transport policy outcome shall be made available to authorised consumers without transferring Transport Management ownership. No unsupported event name is introduced.

## 6. Failure / Rejection / Continuation
- Invalid configuration changes shall not become effective.
- Missing approval shall retain the change in its applicable governance position.
- Effective adoption shall not erase historical configuration context.

## 7. Audit & Evidence Behavior
Preserve attributable proposal, validation, approval, effective date, resulting policy, and superseded-policy evidence.

## 8. Observable Completion Conditions
Completion occurs when the authorised configuration is effective as operational policy with traceable history, or a governed terminal rejection is recorded.

## 9. Traceability & Provenance
**Parent:** L3-G-123 — Transport Configuration Change to Effective Operational Policy

## 10. Open Issues / Deferred Design
Technical configuration storage, interfaces, deployment, and transport mechanisms remain outside L4.