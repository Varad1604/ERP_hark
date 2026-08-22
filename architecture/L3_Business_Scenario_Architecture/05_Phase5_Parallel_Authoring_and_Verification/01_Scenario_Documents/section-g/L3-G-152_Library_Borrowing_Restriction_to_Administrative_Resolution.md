# Library Borrowing Restriction to Administrative Resolution

**Scenario ID:** L3-G-152  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

This scenario governs the institutional resolution of a restriction affecting a library borrowing relationship. The locked L2 states that borrowing validation may consider Active Membership, Borrowing Limit, Outstanding Dues, Resource Availability, Reservation Priority and Administrative Restrictions, and that validation rules remain configurable. fileciteturn221file0

The L2 also states that Finance owns accounting/payment while Library Management owns operational fines and outstanding operational dues. fileciteturn220file0turn215file0

The scenario applies the locked L2-G05 ownership and workflow boundaries. Library Management remains the canonical operational owner of library services, while external domains retain ownership of their own information and downstream outcomes. fileciteturn215file0turn220file0

## 3. Business Outcome

A borrowing restriction has been reviewed and an authoritative administrative outcome is established: the restriction remains applicable, is resolved/removed under the applicable rule, or the borrowing request remains unable to proceed.

The scenario does not invent a new restriction lifecycle or assume that payment automatically removes every restriction.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario starts when an administrative, financial, membership, borrowing-limit, availability or other authoritative condition prevents normal borrowing validation from succeeding.

### 4.2 End Boundary
The scenario ends when the restriction receives its authoritative administrative resolution, without executing an unrelated downstream process.

### 4.3 In Scope
- Restriction identification.
- Borrowing validation.
- Review of applicable membership, borrowing-limit, dues, resource availability, reservation-priority and administrative-restriction context.
- Cross-domain financial/student context.
- Administrative resolution.
- Governed notification.

### 4.4 Out of Scope
- Technical APIs, databases, Business Data Structures, queues, retries, rollback, infrastructure and deployment.
- UI behaviour.
- technical access control/IAM.
- Any lifecycle, rule, event or ownership not defined by authoritative upstream architecture.
- Unrelated downstream institutional outcomes.

## 5. Ownership and Participating Domains

### Library Management — G
G is the canonical owner of the library business outcome and lifecycle described by this scenario.

### Participating Domains
G, C, F, H

Participation by another domain supplies/consumes information within its own authority; it does not transfer ownership from G.

## 6. Business Actors, Roles and Responsibilities

- Library Management owns borrowing and restriction decisions.
- C provides student context where required.
- F provides authoritative financial/payment context.
- H handles notification delivery.

Only source-supported business responsibilities are used. No technical role is introduced.

## 7. Preconditions and Entry Criteria

1. A borrowing request or active borrowing context exists.
2. The applicable resource and member context are identifiable.
3. Authoritative validation information is available.
4. The restriction has not already been resolved.

## 8. Trigger

A borrowing validation condition prevents normal borrowing progression.

## 9. Main Business Flow

### Phase A — Identify Restriction
1. Identify the affected borrowing request/context.
2. Identify the authoritative condition preventing normal borrowing.
3. Preserve the restriction context.

### Phase B — Validate
4. Check active membership.
5. Check borrowing limit.
6. Check outstanding dues.
7. Check resource availability.
8. Check reservation priority.
9. Check administrative restrictions.
10. Apply configurable validation rules. fileciteturn221file0

### Phase C — Cross-Domain Review
11. Consume student context from C where applicable.
12. Consume financial/payment status from F where applicable.
13. Do not reinterpret Finance-owned accounting/payment information.

### Phase D — Administrative Resolution
14. Determine whether the restriction remains applicable.
15. Establish the authoritative administrative resolution.
16. If the borrowing condition is satisfied, allow the normal borrowing workflow to continue under its own boundary.
17. If it remains unsatisfied, preserve the restriction outcome.

### Phase E — Communication
18. Where governed notification is required, provide the outcome to H.
19. End the scenario.

## 10. Alternate Business Flows

### 10.1 Outstanding Dues
Outstanding dues are considered according to configurable borrowing validation. F remains authoritative for accounting/payment.

### 10.2 Membership Restriction
An inactive/restricted membership may prevent borrowing according to configured policy.

### 10.3 Borrowing Limit
A borrowing-limit condition remains until the applicable rule permits progression.

### 10.4 Resource Unavailable
The resource availability conflict is not silently converted into a restriction-resolution outcome; it follows the applicable reservation/availability workflow.

### 10.5 Administrative Restriction
An administrative restriction is reviewed under its authoritative policy.

### 10.6 Restriction Resolved
Normal borrowing may resume through the separate borrowing workflow. This scenario does not duplicate the borrowing lifecycle.

## 11. Failure, Interruption and Business Recovery

### 11.1 Missing Validation Information
The restriction is not resolved using invented information.

### 11.2 Financial Status Unclear
G does not invent payment status; F remains authoritative.

### 11.3 Student Context Unclear
C remains authoritative for student lifecycle information.

### 11.4 Administrative Decision Pending
The restriction remains unresolved.

### 11.5 Notification Pending
H remains responsible for delivery.

### 11.6 Recovery
When the authoritative business condition is established, administrative resolution proceeds.

No technical retry, queue, server failure, rollback, database recovery, API or infrastructure mechanics are specified.

## 12. Business Rules and Decision Constraints

Borrowing validation may consider Active Membership, Borrowing Limit, Outstanding Dues, Resource Availability, Reservation Priority and Administrative Restrictions; validation rules are configurable. fileciteturn221file0

Borrowing policies may define Maximum Borrowings, Borrowing Duration, Renewal Eligibility, Reservation Priority and Restricted Resources. fileciteturn221file0

Finance owns accounting/payment; Library Management owns operational fine calculation and outstanding operational dues. fileciteturn220file0turn215file0

Rules remain upstream-owned and configurable where L2 states that configuration is permitted. No local rule is invented.

## 13. State and Lifecycle Impact

No dedicated `BorrowingRestriction` lifecycle is defined in the retrieved L2. This scenario therefore treats restriction as a business condition within borrowing validation rather than inventing a new aggregate lifecycle.

The existing borrowing workflow remains authoritative for any subsequent issue/return/renewal action.

No new lifecycle states or transitions are created.

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2 does not define a dedicated borrowing-restriction event.

Therefore no new restriction event is invented. Existing circulation events such as `ResourceIssued`, `ResourceReturned`, `ResourceRenewed` and `ResourceOverdue` are used only when their actual business transitions occur. fileciteturn223file0

C, F and H participate according to ownership boundaries.

Only canonical events actually defined by L2 are named. Cross-domain collaboration does not transfer the canonical publisher/owner.

## 15. Business Authorization, Approval and Audit Obligations

Administrative resolution must follow applicable Library Management policy and configurable borrowing validation. Audit evidence should include the affected request, restriction reason, validation findings, cross-domain information used, decision and notification outcome.

No technical IAM/RBAC is introduced.

The scenario preserves auditable business history. Technical authorization mechanisms are outside L3.

## 16. Completion Criteria and Postconditions

1. Restriction identified.
2. Applicable validation completed.
3. Required cross-domain context consumed.
4. Administrative resolution established.
5. Normal borrowing may continue only through its authoritative workflow.
6. Restriction history preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues
1. The L2 does not define a dedicated borrowing-restriction lifecycle.
2. It does not provide a universal matrix mapping each restriction cause to a resolution.

### Assumptions
- Restriction remains a business validation condition.
- Configurable borrowing policies determine the applicable outcome.

### Deferred Questions
Confirm any dedicated upstream restriction policy/event before Architecture Approval/Lock.

Unresolved upstream matters are recorded rather than fabricated.
