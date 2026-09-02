# Library Resource Issue to Active Borrowing

## 1. Specification Metadata

**Specification ID:** L4-G-131  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-131 — Library Resource Issue to Active Borrowing  
**Primary Business Owner:** Library Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-131; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern issue of an eligible Library Copy to an eligible member through establishment of Active borrowing and the corresponding copy issue condition.

## 3. Required Business Context

- The member and Active Library Membership are authoritative.
- The requested Library Copy is Available and eligible for circulation.
- Borrowing limits, dues/restrictions, reservation priority and borrowing policy can be evaluated.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-131-BH-001 | Validate Active Membership and configured borrowing eligibility. | L3-G-131 §§7,9,12 |
| L4-G-131-BH-002 | Validate the Library Copy's authoritative availability and resource eligibility. | L3-G-131 §§7,9,12 |
| L4-G-131-BH-003 | Evaluate configured borrowing limits, outstanding-dues restrictions, reservation priority and administrative restrictions where applicable. | L3-G-131 §§9,12 |
| L4-G-131-BH-004 | On approved issue, establish the borrowing in `Active`, determine the applicable due date, and move the Library Copy from `Available` into its issued operational condition. | L3-G-131 §§3,9,13,16 |
| L4-G-131-BH-005 | Preserve issue decision, borrower/copy relationship, due date and circulation history. | L3-G-131 §§12,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-131-BH-006 | On successful issue, Library Management shall publish `ResourceIssued`. C/E retain authoritative identity/personnel ownership; Finance retains accounting/payment ownership; H may consume the issue outcome for governed communication. | L3-G-131 §§5,14,16 |

## 6. Failure / Rejection / Continuation

- Inactive membership shall prevent issue.
- Unavailable or restricted copy shall prevent issue.
- Configured borrowing-limit, dues, reservation-priority, or administrative restrictions shall be respected.
- A rejected issue shall not create Active borrowing or move the copy into Issued.

## 7. Audit & Evidence Behavior

Preserve membership validation, copy identity/status, borrowing-policy evaluation, approval, due date, issue event and circulation history.

## 8. Observable Completion Conditions

Completion occurs when an eligible issue establishes Active borrowing with an authoritative due date, the copy is no longer ordinarily Available for another issue, `ResourceIssued` is published, and history is traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-131 — Library Resource Issue to Active Borrowing  
**Primary verification source:** Authoritative Campus Operations L2 workflow architecture.

## 10. Open Issues / Deferred Design

Exact borrowing limits, durations, reservation priority and restriction values remain configuration governed. Technical authentication, APIs, database persistence schemas, UI screens, notification delivery mechanics and queues are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
