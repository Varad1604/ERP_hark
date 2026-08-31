# Approved Candidate to Active Student

**Specification ID:** L4-C-001  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-001  
**Primary Business Owner:** Student Lifecycle  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-08-31  
**Depends On:** L3-C-001; L2-C01 — Admissions Workflows; L2-C02 — Student Lifecycle & Activation Workflows; applicable locked A, B and F sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall support the governed transition from an approved admission through confirmation and Student establishment to an Active StudentContext. The behavior ends when the StudentContext is Active and `StudentActivated` is published. Finance, institutional structure, identity, admission and downstream registration ownership remain outside this specification.

## 3. Required Business Context

- Admission is approved and remaining governed conditions are satisfied.
- Required Finance-owned clearance is available where policy requires it.
- Student Lifecycle owns Student, StudentContext and activation.
- Activation requires authoritative conditions and Registrar authority.
- At most one StudentContext may be Active within an Institute.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-001-BH-001 | When an approved Admission satisfies remaining governed confirmation conditions, the system shall make the confirmed-admission outcome available for Student establishment. | L3-C-001 §§7, 9; L2-C01 |
| L4-C-001-BH-002 | When a confirmed admission is received, the system shall establish the canonical Student without creating a competing identity. | L3-C-001 §§9, 12; L2-C02 |
| L4-C-001-BH-003 | When Student establishment is valid for the applicable Institute, the system shall establish the corresponding StudentContext. | L3-C-001 §§9, 13; L2-C02 |
| L4-C-001-BH-004 | When activation authority and required conditions are satisfied, the system shall transition the StudentContext to `Active`. | L3-C-001 §§6, 12, 13; L2-C02 |
| L4-C-001-BH-005 | The system shall prevent activation when doing so would violate the single Active StudentContext rule for the Institute. | L3-C-001 §§7, 12; L1 locked rule |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-001-BH-006 | On successful Student activation, the system shall publish the canonical `StudentActivated` business event under Student Lifecycle ownership. | L3-C-001 §§3, 14; L2-C02 |

Authoritative Finance clearance and institutional context may be consumed without ownership transfer.

## 6. Failure / Rejection / Continuation

- Prevent progression when required admission verification or Finance-owned clearance is not satisfied.
- Do not establish activation when authoritative activation conditions are not satisfied.
- Preserve the valid pre-activation position when activation cannot proceed.
- Cancellation, revocation, expiry and withdrawal remain governed by their canonical workflows.

## 7. Audit & Evidence Behavior

Preserve attributable evidence of Student establishment and activation, including the authoritative admission basis, activation decision/authority and resulting StudentContext lifecycle position through the applicable audit capability.

## 8. Observable Completion Conditions

The canonical Student exists, the applicable StudentContext is `Active`, required authority/conditions are satisfied, and `StudentActivated` has been published. Academic registration remains downstream.

## 9. Traceability & Provenance

**Parent:** L3-C-001 — Approved Candidate to Active Student  
**Primary verification sources:** L2-C01; L2-C02; applicable locked A, B and F architecture.

## 10. Open Issues / Deferred Design

No L4-specific issue identified. API/interface, persistence, technical authorization, event transport, infrastructure and UI behavior are deferred.
