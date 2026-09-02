# Library Membership Suspension to Restricted Library Access

## 1. Specification Metadata

**Specification ID:** L4-G-125  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-125 — Library Membership Suspension to Restricted Library Access  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-125; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern suspension of an Active Library Membership and the resulting restricted library access without altering upstream student or workforce lifecycle ownership.

## 3. Required Business Context

- An Active Library Membership exists.
- Suspension basis, policy and configured authority can be evaluated.
- C/E retain authoritative member identity/status where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-125-BH-001 | Identify the Active Library Membership and suspension business condition. | L3-G-125 §§7–9 |
| L4-G-125-BH-002 | Evaluate the suspension basis against configured membership eligibility/restriction policy. | L3-G-125 §§9,12 |
| L4-G-125-BH-003 | Apply the required operational approval and record the suspension decision. | L3-G-125 §§9,15 |
| L4-G-125-BH-004 | On effective suspension, transition the membership `Active → Suspended` and ensure subsequent library operations respect the resulting restriction. | L3-G-125 §§3,9,13,16 |
| L4-G-125-BH-005 | Preserve suspension reason, decision, effective scope and membership history. | L3-G-125 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-125-BH-006 | On effective suspension, Library Management shall publish `MembershipSuspended`. C/E retain identity/personnel ownership and H may consume the suspension fact for governed communication without acquiring membership ownership. | L3-G-125 §§5,14,16 |

## 6. Failure / Rejection / Continuation

- Insufficient suspension basis shall leave the membership Active.
- Missing or rejected authority shall prevent suspension from becoming effective.
- Suspension shall not be converted into closure.
- No `Restricted`, `Blocked`, or `Pending Suspension` membership state shall be invented.

## 7. Audit & Evidence Behavior

Preserve suspension basis, authority, decision, effective restriction, lifecycle transition and membership history.

## 8. Observable Completion Conditions

Completion occurs when the membership is `Suspended`, applicable library access is restricted according to policy, `MembershipSuspended` is published, and history is traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-125 — Library Membership Suspension to Restricted Library Access  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact suspension criteria and duration remain configuration governed; no automatic reactivation rule is introduced. Technical access-control enforcement, APIs, database persistence schemas, UI screens, notification delivery mechanics and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
