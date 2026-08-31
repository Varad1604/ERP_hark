# Student Enrollment Update

**Specification ID:** L4-C-008  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-008  
**Primary Business Owner:** Student Lifecycle  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-008; L2-C02 — Student Lifecycle & Activation Workflows, WF-C02-03; applicable Academic Delivery authority  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support an authorized update to enrollment information for an Active Student while preserving prior enrollment history and current-enrollment uniqueness. The behavior ends when the enrollment update is complete and `EnrollmentUpdated` is published.

## 3. Required Business Context

- Student exists and is Active.
- Request is authorized.
- Current enrollment is identifiable.
- Historical enrollment is preserved.
- Academic Delivery may provide academic context without acquiring enrollment ownership.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-008-BH-001 | When an enrollment change, academic-session update or authorized administrative request is received for an Active Student, the system shall validate the request before applying it. | L3-C-008 §§7, 8, 9; L2-C02 WF-C02-03 |
| L4-C-008-BH-002 | When the enrollment change is valid, the system shall update the Student Lifecycle-owned current enrollment information. | L3-C-008 §§3, 9, 12; L2-C02 |
| L4-C-008-BH-003 | The system shall preserve the previous enrollment information as historical enrollment. | L3-C-008 §§3, 9, 12; L2-C02 |
| L4-C-008-BH-004 | The system shall prevent a completed update from establishing contradictory current enrollment. | L3-C-008 §§7, 12, 13; L2-C02 |
| L4-C-008-BH-005 | The system shall not apply a Programme, Batch or Section assignment decision as an enrollment update when the requested change belongs to Academic Assignment. | L3-C-008 §§4, 10, 11; L2-C02 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-008-BH-006 | On successful enrollment update, the system shall publish the canonical `EnrollmentUpdated` business event under Student Lifecycle ownership. | L3-C-008 §§3, 14; L2-C02 |

Academic Delivery information is consumed only as applicable business context.

## 6. Failure / Rejection / Continuation

- Prevent update processing when the Student is not Active.
- Reject an unauthorized request.
- Do not force a request outside enrollment capability into this behavior.
- Preserve historical enrollment when an update cannot complete.
- Programme migration and academic assignment remain governed by their canonical workflows.

## 7. Audit & Evidence Behavior

The system shall preserve evidence of the authorized enrollment change, prior/current enrollment positions and resulting completed outcome through applicable Student Lifecycle/Audit capabilities.

## 8. Observable Completion Conditions

Completion occurs when valid enrollment information is updated, prior history is preserved, current enrollment remains unique, required audit evidence is available and `EnrollmentUpdated` has been published.

## 9. Traceability & Provenance

**Parent:** L3-C-008 — Student Enrollment Update  
**Primary verification source:** L2-C02 — Student Lifecycle & Activation Workflows, WF-C02-03.

## 10. Open Issues / Deferred Design

No L4-specific issue identified. APIs, persistence, technical authorization, event transport, infrastructure and UI behavior are deferred.
