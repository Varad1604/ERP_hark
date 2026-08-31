# Admission Cancellation

**Specification ID:** L4-C-002  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-002  
**Primary Business Owner:** Admissions  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-002; L2-C01 — Admissions Workflows, WF-C01-06  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support cancellation of an eligible Admission Record before successful Student creation. The behavior ends when the Admission reaches `Cancelled`, history is preserved and `AdmissionCancelled` is published. Finance-owned settlement and Student Lifecycle actions remain outside this specification.

## 3. Required Business Context

- A valid Admission Record exists.
- Student creation has not occurred for the Admission.
- Cancellation requires authorized initiation and eligible state.
- Admissions owns the Admission lifecycle and `AdmissionCancelled`.
- L3-C-002 records an unresolved L2 conflict for cancellation from `Admission Approved` and `Financial Clearance`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-002-BH-001 | When an authorized cancellation request is received for an eligible Admission, the system shall validate cancellation eligibility before changing the lifecycle. | L3-C-002 §§7, 9, 12; L2-C01 |
| L4-C-002-BH-002 | When cancellation is eligible, the system shall record the required reason and transition the Admission Record to `Cancelled`. | L3-C-002 §§9, 12, 13; L2-C01 |
| L4-C-002-BH-003 | The system shall preserve the completed cancellation as Admission history. | L3-C-002 §§3, 11, 15; L2-C01 |
| L4-C-002-BH-004 | The system shall prevent cancellation when successful Student creation has already occurred. | L3-C-002 §§3, 7, 11; L2-C01 |
| L4-C-002-BH-005 | The system shall preserve the current valid Admission position when cancellation authority or eligibility is not satisfied. | L3-C-002 §§11, 13; L2-C01 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-002-BH-006 | On successful cancellation, the system shall publish the canonical `AdmissionCancelled` business event under Admissions ownership. | L3-C-002 §§3, 14; L2-C01 |

No cross-domain ownership is transferred.

## 6. Failure / Rejection / Continuation

- Reject cancellation when the requester is unauthorized.
- Do not transition an ineligible Admission to `Cancelled`.
- Preserve an existing `Cancelled` terminal outcome.
- If Student creation has occurred, use the applicable revocation or Student Lifecycle process instead.
- Do not locally resolve the disputed `Admission Approved` / `Financial Clearance` cancellation transition.

## 7. Audit & Evidence Behavior

Preserve attributable cancellation evidence, including the cancellation reason/basis and resulting `Cancelled` lifecycle outcome, through the authoritative Admissions audit capability.

## 8. Observable Completion Conditions

An eligible Admission reaches `Cancelled`, the reason/history is preserved, no Student has been created from that Admission, and `AdmissionCancelled` has been published.

## 9. Traceability & Provenance

**Parent:** L3-C-002 — Admission Cancellation  
**Primary verification source:** L2-C01 — Admissions Workflows, WF-C01-06.

## 10. Open Issues / Deferred Design

L3-C-002 identifies conflicting L2-C01 cancellation-state matrices for `Admission Approved` and `Financial Clearance`. L4 does not resolve this upstream conflict. Technical authorization, persistence, transport and UI behavior are deferred.
