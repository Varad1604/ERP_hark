# Workforce Member Establishment as Active Faculty

## 1. Specification Metadata

**Specification ID:** L4-E-026  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Faculty Workforce  
**Realizes Scenario:** L3-E-026  
**Primary Business Owner:** Faculty Workforce  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-026 — Workforce Member Establishment as Active Faculty; L2-E03 — Faculty Workforce Workflows; L2-E02 — Position & Assignment Workflows where organisational context is required  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish an eligible Faculty Workforce representation and activate it only after required faculty verification and approval conditions are satisfied.

L4 begins when an authoritative workforce/employee context exists and a faculty representation is required. L4 ends when Faculty Workforce is Active or the governed non-active/rejected outcome remains authoritative.

HR employment ownership and Position & Assignment ownership remain outside this specification.

## 3. Required Business Context

- Authoritative workforce/employee context.
- Faculty information required for establishment.
- Duplicate Faculty Workforce condition.
- Faculty verification criteria.
- Required institutional approval.
- Organisational assignment context where required.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-026-BH-001 | Before creating a Faculty Workforce representation, the system shall validate the authoritative workforce context and prevent duplicate Faculty Workforce representation for the applicable workforce member. | L3-E-026 §§7, 9, 10, 12 |
| L4-E-026-BH-002 | The system shall establish the Faculty Workforce representation in its initial lifecycle position before faculty activation is attempted. | L3-E-026 §§4.3, 9, 13 |
| L4-E-026-BH-003 | The system shall require successful faculty verification and the applicable institutional approval outcome before activating the Faculty Workforce. | L3-E-026 §§3, 7, 9, 15 |
| L4-E-026-BH-004 | When faculty verification is rejected, the system shall prevent the Faculty Workforce from reaching Active and shall preserve the governed rejection outcome. | L3-E-026 §§10, 11 |
| L4-E-026-BH-005 | On successful activation, the system shall establish Faculty Workforce as Active and recognise `FacultyWorkforceActivated` as the canonical business fact. | L3-E-026 §§3, 9, 13, 14 |
| L4-E-026-BH-006 | The system shall preserve the Faculty Workforce lifecycle history and activation evidence. | L3-E-026 §§9, 11, 16 |

## 5. Cross-Domain & Event Behavior

The system shall recognise the applicable canonical Faculty Workforce business facts:

- `FacultyWorkforceCreated`
- `FacultyWorkforceActivated`
- `FacultyVerificationRejected`

Faculty Workforce remains the canonical publisher; HR and Position & Assignment provide authoritative context without ownership transfer. fileciteturn177file0L2-L2

## 6. Failure / Rejection / Continuation

If required faculty information, verification, or approval conditions are not satisfied, the system shall not establish an Active Faculty Workforce outcome. The applicable non-active or rejection position remains authoritative until a governed correction or subsequent valid progression occurs.

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- authoritative workforce context used;
- duplicate check result;
- faculty information considered;
- verification outcome;
- approval outcome where required;
- resulting Faculty Workforce state; and
- canonical activation or rejection fact.

## 8. Observable Completion Conditions

The behavior is complete when:

- the Faculty Workforce representation exists;
- required verification and approval conditions are satisfied for activation;
- Faculty Workforce is Active, or the governed rejection/non-active outcome is authoritative; and
- applicable lifecycle history and canonical business facts are preserved.

## 9. Traceability & Provenance

**Parent:** L3-E-026 — Workforce Member Establishment as Active Faculty  
**Primary verification source:** L2-E03 — Faculty Workforce Workflows  

HR employee identity/employment and Position & Assignment remain upstream authoritative contexts.

## 10. Open Issues / Deferred Design

No L4 business-behavior issue is identified in the approved parent. Technical interfaces, persistence, IAM, event transport, retry/rollback mechanics, infrastructure, and UI remain downstream concerns.

## Canonical Validation Note

This specification is a Draft and requires applicable L4 validation and governance review before Architecture Approval or Locking.
