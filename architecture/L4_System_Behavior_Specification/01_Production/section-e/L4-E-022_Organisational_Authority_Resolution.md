# Organisational Authority Resolution

## 1. Specification Metadata

**Specification ID:** L4-E-022  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Position & Assignment  
**Realizes Scenario:** L3-E-022  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-022 — Organisational Authority Resolution; L2-E02 — Position & Assignment Workflows  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

SutraERP shall deterministically resolve the organisational authority applicable to an institutional business action from authoritative Position Assignment context.

L4 begins when an institutional action requires authority resolution and authoritative Position Assignment context exists. L4 ends when the applicable authority outcome is available to the consuming business capability.

Person-based authority, technical IAM, and independent Position Assignment lifecycle changes remain outside this specification.

## 3. Required Business Context

- Relevant active Position Assignment context.
- Organisational placement and reporting context.
- Applicable concurrent, acting, delegated, or future-effective authority context.
- Authoritative Workforce Engagement and Position Definition context where required.
- Historical context where authority is being reconstructed for a prior business condition.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-022-BH-001 | When an institutional business action requires authority resolution, the system shall resolve the applicable Position Assignment context before determining the organisational authority. | L3-E-022 §§4, 8, 9 |
| L4-E-022-BH-002 | The system shall prevent a person-based authority outcome and shall resolve authority from the authoritative Position Assignment context. | L3-E-022 §§3, 12 |
| L4-E-022-BH-003 | The system shall account for applicable concurrent, acting, and delegated authority when resolving the authoritative organisational authority. | L3-E-022 §§4.3, 9, 10, 12 |
| L4-E-022-BH-004 | The system shall apply future-effective organisational reporting context only for the period in which that context is authoritative. | L3-E-022 §§9, 10, 12 |
| L4-E-022-BH-005 | When authoritative historical context is requested, the system shall resolve the organisational authority using reconstructable historical reporting context. | L3-E-022 §§4.3, 10, 12 |
| L4-E-022-BH-006 | The system shall make the resolved authority outcome available to the consuming business capability without transferring ownership of the underlying Position Assignment. | L3-E-022 §§3, 5, 9, 14 |

## 5. Cross-Domain & Event Behavior

The L3 source identifies Position & Assignment as the canonical owner and states that no distinct OrganisationalAuthorityResolved event was identified. The system may make the resolved authority outcome available through the established business collaboration boundary, but no new event name is introduced.

## 6. Failure / Rejection / Continuation

When authority cannot be resolved deterministically from authoritative Position Assignment context, the system shall not make an ambiguous authority outcome operational. The consuming business action shall remain subject to the applicable governance path until a valid authority outcome exists.

The system shall retain the authoritative business position rather than substitute person-based or unsupported authority.

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable the business evidence necessary to demonstrate:

- the Position Assignment context used for resolution;
- applicable organisational/reporting context;
- concurrent, acting, or delegated authority considered where relevant;
- applicable effective-period context;
- the resolved authority outcome; and
- historical authority context when reconstructing a past outcome.

## 8. Observable Completion Conditions

The behavior is complete when:

- the applicable organisational authority has been deterministically resolved;
- the resolution is position-based;
- no person-based authority has been introduced;
- applicable concurrent, acting, delegated, future-effective, or historical context has been respected; and
- the resolved outcome is available to the consuming business capability without ownership transfer.

## 9. Traceability & Provenance

**Parent:** L3-E-022 — Organisational Authority Resolution  
**Primary verification source:** L2-E02 — Position & Assignment Workflows  

Material source rules include ARM-001 through ARM-007 as reproduced by the L3 parent.

## 10. Open Issues / Deferred Design

No L4 business-behavior issue is identified in the approved parent. APIs, persistence, technical IAM, transport, retry/rollback mechanics, infrastructure, and UI remain downstream concerns.

## Canonical Validation Note

This specification is a Draft and requires applicable L4 validation and governance review before Architecture Approval or Locking.
