# External Enrollment Request to Confirmed or Rejected Enrollment Exchange

## 1. Specification Metadata
**Specification ID:** L4-H-076  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** External Education Integration  
**Realizes Scenario:** L3-H-076  
**Primary Business Owner:** External Education Integration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-076; L2-H10 — External Education Integration Workflows; `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md`; `L2-H10_Enrollment_Cancellation_and_Verification_Result_Amendment_2026-08-27.md`; applicable Locked Student/Academic architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern the H10 Enrollment Exchange through validation, acceptance, rejection, governed pre-terminal cancellation or bounded-validity expiry without allowing H10 to create, approve or mutate canonical Student/Academic enrollment truth.

## 3. Required Business Context
Owning Student/Academic governance permits the exchange; provider/trust/connector context is governed; bounded validity is known where expiry applies; C/D retain canonical enrollment authority; H10 owns only the external Enrollment Exchange lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-076-BH-001 | The system shall create the H10 Enrollment Exchange in `Requested` only when the originating learner/provider request and required provider/trust/source context are valid. | L3-H-076 §§7–9 |
| L4-H-076-BH-002 | The system shall transition `Requested → Validation Pending` and validate only the external exchange context; H10 shall not perform or assert the C/D-owned academic enrollment decision. | L3-H-076 §§5,9,12–13 |
| L4-H-076-BH-003 | When the exchange is valid for external processing, the system shall transition `Validation Pending → Accepted` and publish `ExternalEnrollmentExchangeAccepted`; this H10 acceptance shall not create canonical enrollment. | L3-H-076 §§3,9,12–16 |
| L4-H-076-BH-004 | When the exchange is rejected during validation, the system shall transition `Validation Pending → Rejected` and publish `ExternalEnrollmentExchangeRejected` without altering canonical enrollment truth. | L3-H-076 §§9–14 |
| L4-H-076-BH-005 | Before H10 reaches Accepted or Rejected, an eligible governed cancellation request may transition `Requested` or `Validation Pending → Cancellation Requested` and publish `ExternalEnrollmentExchangeCancellationRequested`. | L3-H-076 §§6–10,12–14 |
| L4-H-076-BH-006 | After the governed H10 cancellation decision, the system shall transition `Cancellation Requested → Cancelled` and publish `ExternalEnrollmentExchangeCancelled`; an Accepted exchange shall not be locally moved to Cancelled. | L3-H-076 §§6,10–15 |
| L4-H-076-BH-007 | When an Accepted exchange has a bounded validity period and the owning C/D domain has not established canonical enrollment before that period ends, the system shall transition `Accepted → Expired` and publish `ExternalEnrollmentExchangeExpired`. | L3-H-076 §§7,10,12–14 |
| L4-H-076-BH-008 | H10 exchange outcomes `Accepted`, `Rejected`, `Cancelled` and `Expired` shall never create, replace or mutate the canonical Student/Academic enrollment fact; only its owning C/D domain may do so through its own workflow. | L3-H-076 §§3–5,11–13,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-076-BH-009 | H10 shall publish `ExternalEnrollmentExchangeAccepted`, `ExternalEnrollmentExchangeRejected`, `ExternalEnrollmentExchangeExpired`, `ExternalEnrollmentExchangeCancellationRequested`, and `ExternalEnrollmentExchangeCancelled` only for H10 Enrollment Exchange facts. | L3-H-076 §14 |
| L4-H-076-BH-010 | C/D shall retain enrollment ownership and their own canonical publishers; an originating C/D cancellation request does not make C/D owner of the H10 exchange lifecycle. | L3-H-076 §§5–6,14–16 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-076-BH-011 | Invalid cancellation authority or a cancellation request after H10 terminal acceptance/rejection shall leave the current H10 exchange outcome unchanged. | L3-H-076 §11 |
| L4-H-076-BH-012 | Exchange failure, rejection, expiry or cancellation shall remain an H10 exchange outcome and shall not be represented as an academic enrollment decision. | L3-H-076 §§10–12 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-076-BH-013 | The system shall preserve originating request, provider/exchange context, source references, validation, cancellation request/decision where applicable, H10 outcome, validity/expiry evidence and any separately owned canonical enrollment reference. | L3-H-076 §15 |

## 8. Observable Completion Conditions
The behavior is complete when the H10 Enrollment Exchange reaches `Accepted`, `Rejected`, `Cancelled` or `Expired` with complete evidence and canonical Student/Academic enrollment ownership remains entirely with C/D.

## 9. Traceability & Provenance
**Parent:** L3-H-076 — External Enrollment Request to Confirmed or Rejected Enrollment Exchange  
**Primary verification source:** L2-H10 — External Education Integration Workflows  
**Cancellation/expiry verification:** locked H10 amendments dated 2026-08-26 and 2026-08-27.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Protocol, API, persistence and UI design remain deferred.
