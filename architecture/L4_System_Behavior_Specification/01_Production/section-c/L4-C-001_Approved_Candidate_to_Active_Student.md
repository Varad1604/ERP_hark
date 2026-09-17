# Approved Candidate to Active Student

## 1. Specification Metadata

**Specification ID:** L4-C-001  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Student Lifecycle  
**Realizes Scenario:** L3-C-001 — Approved Candidate to Active Student  
**Primary Business Owner:** Student Lifecycle  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Varad  
**Last Updated:** 2026-09-17  
**Depends On:** L3-C-001; applicable locked C architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

The system shall realize the governed transition from an approved Admission through authoritative Admissions confirmation, canonical Student establishment and StudentContext activation. The behavior ends when the StudentContext is `Active` and `StudentActivated` is published. Admissions retains confirmation ownership; Student Lifecycle retains Student and StudentContext establishment and activation ownership. Finance, institutional structure, identity and downstream academic registration remain outside this specification.

## 3. Required Business Context

- Admission is approved and remaining governed conditions are satisfied.
- Required Finance-owned clearance is available where policy requires it.
- Student Lifecycle owns Student, StudentContext and activation.
- Activation requires authoritative conditions and Registrar authority.
- At most one StudentContext may be Active within an Institute.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-C-001-BH-001 | When Admissions reaches its authoritative confirmation outcome, the system shall recognize the Admissions-published `AdmissionConfirmed` fact as the valid basis for Student establishment. | L3-C-001 §§8, 9, 14; L2-C01 |
| L4-C-001-BH-002 | When a confirmed admission is received, the system shall establish the canonical Student without creating a competing identity. | L3-C-001 §§9, 12; L2-C02 |
| L4-C-001-BH-003 | When Student establishment is valid for the applicable Institute, the system shall establish the corresponding StudentContext. | L3-C-001 §§9, 13; L2-C02 |
| L4-C-001-BH-004 | When activation authority and required conditions are satisfied, the system shall transition the StudentContext to `Active`. | L3-C-001 §§6, 12, 13; L2-C02 |
| L4-C-001-BH-005 | The system shall prevent activation when doing so would violate the single Active StudentContext rule for the Institute. | L3-C-001 §§7, 12; L1 locked rule |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-C-001-BH-006 | On successful Student activation, the system shall publish the canonical `StudentActivated` business event under Student Lifecycle ownership. | L3-C-001 §§3, 14; L2-C02 |
| L4-C-001-BH-007 | When the canonical Student has been established, the system shall publish `StudentCreated` under Student Lifecycle ownership without changing Admissions ownership of the confirmed-admission hand-off. | L3-C-001 §§9, 14; L2-C02 |

Where the locked Admissions flow also uses `StudentCreationRequested`, that fact remains Admissions-published and is consumed only as an authorised Student-establishment request; this specification does not republish it under Student Lifecycle ownership.

## 6. Failure / Rejection / Continuation

- Prevent progression when required admission verification or Finance-owned clearance is not satisfied.
- Do not establish activation when authoritative activation conditions are not satisfied.
- Preserve the valid pre-activation position when activation cannot proceed.
- Cancellation, revocation, expiry and withdrawal remain governed by their canonical workflows.

## 7. Audit & Evidence Behavior

Preserve attributable evidence of Student establishment and activation, including the authoritative admission basis, activation decision/authority and resulting StudentContext lifecycle position through the applicable audit capability.

## 8. Observable Completion Conditions

The canonical Student exists, the applicable StudentContext is `Active`, required authority/conditions are satisfied, `StudentCreated` has been published for canonical Student establishment, and `StudentActivated` has been published for the completed activation outcome. Academic registration remains downstream.

## 9. Traceability & Provenance

**Parent:** L3-C-001 — Approved Candidate to Active Student  
**Primary verification source:** Authoritative Student Lifecycle L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is Architecture Approved under the W-C02 parent-set validation decision and is not Locked.
