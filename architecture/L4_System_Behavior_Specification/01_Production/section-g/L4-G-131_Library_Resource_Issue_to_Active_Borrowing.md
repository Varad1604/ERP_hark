# L4-G-131 — Library Resource Issue to Active Borrowing

## 1. Specification Metadata
**Specification ID:** L4-G-131  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Library Management  
**Realizes Scenario:** L3-G-131 — Library Resource Issue to Active Borrowing  
**Primary Business Owner:** Library Management  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01

## 2. Behavioral Objective & Boundary
SutraERP shall govern issue of an eligible library resource to an authorised borrower through establishment of active borrowing.

**Start:** An eligible library resource is requested for issue.  
**End:** Borrowing becomes Active or the issue reaches a governed rejection outcome.

## 3. Required Business Context
- An Active Library Membership exists.
- The requested copy is eligible for circulation.
- Applicable borrowing policy is authoritative.
- Library Management owns borrowing lifecycle.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-131-BH-001 | Validate borrower membership and borrowing eligibility. | L3-G-131 |
| L4-G-131-BH-002 | Validate the requested copy's circulation availability. | L3-G-131 |
| L4-G-131-BH-003 | Apply applicable borrowing limits and authorisation rules. | L3-G-131 |
| L4-G-131-BH-004 | On successful issue, establish the borrowing as Active and associate the issued copy with the borrowing. | L3-G-131 |
| L4-G-131-BH-005 | Preserve issue transaction and borrowing history. | L3-G-131 |

## 5. Cross-Domain & Event Behavior
The active borrowing outcome shall be available to authorised library consumers without transferring Library Management ownership. No unsupported event name is invented.

## 6. Failure / Rejection / Continuation
- Ineligible membership shall prevent issue.
- Unavailable or restricted copy shall prevent issue.
- Exceeded borrowing constraints shall prevent creation of Active Borrowing.

## 7. Audit & Evidence Behavior
Preserve borrower eligibility, copy identity, issue authority, issue decision, and active borrowing evidence.

## 8. Observable Completion Conditions
Completion occurs when an eligible copy is issued to an eligible borrower and Active Borrowing is established, or a governed rejection is recorded.

## 9. Traceability & Provenance
**Parent:** L3-G-131 — Library Resource Issue to Active Borrowing

## 10. Open Issues / Deferred Design
Technical circulation interfaces, barcode/RFID operations, persistence, and notifications remain outside L4.