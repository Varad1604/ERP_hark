# Student Communication Preference Update

**Specification ID:** L4-C-015  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-015  
**Primary Business Owner:** Student Lifecycle  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-015; L2-C02 — Student Lifecycle & Activation Workflows, WF-C02-14  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support governed updates to Student communication preferences. The behavior ends when valid preferences are updated, audit evidence is preserved and `StudentCommunicationPreferencesUpdated` is published. Communication delivery remains outside this specification.

## 3. Required Business Context

- Student exists.
- Requested preference values are permitted by institutional policy/configuration.
- Student Lifecycle owns communication preferences.
- Required approval, if applicable, is satisfied.
- Notification delivery is a downstream concern.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-015-BH-001 | The system shall validate the Student and requested communication-preference scope. | L3-C-015 §§7, 9; L2-C02 WF-C02-14 |
| L4-C-015-BH-002 | The system shall validate requested preference values against applicable institutional policy/configuration. | L3-C-015 §§3, 9, 12; L2-C02 |
| L4-C-015-BH-003 | The system shall apply required approval before completing a restricted preference change where policy requires it. | L3-C-015 §§6, 9, 15; L2-C02 |
| L4-C-015-BH-004 | The system shall update Student-owned communication preferences without changing Student identity or lifecycle state. | L3-C-015 §§3, 13; L2-C02 |
| L4-C-015-BH-005 | The system shall not treat preference update as message delivery. | L3-C-015 §§4, 14; L2-C02 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-015-BH-006 | On successful preference update, the system shall publish `StudentCommunicationPreferencesUpdated` under Student Lifecycle ownership. | L3-C-015 §§3, 14; L2-C02 |

## 6. Failure / Rejection / Continuation

- Reject unsupported preference values.
- Do not apply restricted changes before required approval.
- Preserve existing preferences when validation or approval fails.
- Route unrelated profile or identity changes to their canonical scenarios.

## 7. Audit & Evidence Behavior

Preserve evidence of the requested preference change, applicable authorization/approval and resulting preference outcome.

## 8. Observable Completion Conditions

Valid preferences are updated, identity/lifecycle remain unchanged, audit evidence is available and `StudentCommunicationPreferencesUpdated` has been published.

## 9. Traceability & Provenance

**Parent:** L3-C-015 — Student Communication Preference Update  
**Primary verification source:** L2-C02 WF-C02-14 — Student Communication Preference Management Workflow.

## 10. Open Issues / Deferred Design

No L4-specific issue identified. Message delivery, provider selection, APIs, persistence, event transport and UI are deferred.
