# Library Membership Closure to Library Access Termination

**Scenario ID:** L3-G-126  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Library Management  
**Participating Domains:** G, C, E, H  
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

This scenario governs the complete business outcome in which a Library Membership is closed and the member's library access is terminated.

Closure is materially different from temporary suspension. The membership reaches the terminal end of its G05 lifecycle and subsequent library operations must no longer treat the membership as Active.

The scenario preserves the ownership boundary: Library Management closes the membership; C/E remain owners of the member's institutional identity and lifecycle; H may communicate the outcome.

## 3. Business Outcome

A Library Membership reaches the authoritative Closed state after the applicable closure conditions and approvals are satisfied, and library access associated with that membership is terminated.

The scenario does not terminate the person's institutional identity, student status, employment status, or financial account.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An existing Library Membership is identified as eligible or required for closure under the applicable library policy.

### 4.2 End Boundary

The Library Membership reaches the canonical Closed state and its library-service access is terminated.

### 4.3 In Scope

- Identification of membership for closure.
- Closure eligibility review.
- Outstanding library obligation verification where applicable.
- Administrative approval.
- Membership closure.
- Termination of library-service eligibility.
- Preservation of membership history.
- Communication business outcome.

### 4.4 Out of Scope

- Student withdrawal/graduation/identity closure.
- Employee separation.
- Financial ledger closure.
- Payment/refund processing.
- Destruction of historical library records.
- Technical access revocation implementation.

## 5. Ownership and Participating Domains

G05 owns Library Membership and its lifecycle.

C remains owner of student identity/status. E remains owner of employee/personnel status. Finance owns financial accounting/payment. H owns communication delivery.

Closing library membership does not close the upstream identity.

## 6. Business Actors, Roles and Responsibilities

### Librarian / Library Assistant

Performs operational membership review and verifies library obligations.

### Library Administrator / Configured Authority

Approves closure where required.

### Member

Is the subject of the membership closure and may need to resolve outstanding library obligations before closure.

### C / E Authorities

Provide authoritative identity/status information where relevant.

### H

Consumes the closure fact when communication is required.

## 7. Preconditions and Entry Criteria

1. A Library Membership exists.
2. A valid closure condition is present under applicable policy.
3. The membership can be identified authoritatively.
4. Required outstanding library obligations can be reviewed.
5. Required approval authority is available.

## 8. Trigger

A membership reaches a business condition requiring closure, such as the end of its permitted lifecycle or another configured institutional condition.

The scenario does not assume a specific closure reason because the L2 does not prescribe one canonical reason.

## 9. Main Business Flow

### Phase A — Closure Review

1. Identify the membership.
2. Establish its current membership status.
3. Determine whether the membership is eligible for closure.
4. Review outstanding borrowings, fines, replacements or other library obligations where applicable.
5. Obtain authoritative member status context from C/E when required.
6. Determine whether closure can proceed.

### Phase B — Resolve Closure Authority

7. Complete any required operational review.
8. Obtain the configured approval for closure.
9. Record the closure decision and applicable business context.

### Phase C — Close Membership

10. Change the Library Membership to Closed.
11. Terminate library-service eligibility associated with the membership.
12. Ensure subsequent library operations do not treat the closed membership as active.
13. Preserve membership and obligation history.
14. Make the closure fact available to H where communication is required.

### Phase D — Complete

15. Publish `MembershipClosed`.
16. End the scenario with the membership in the canonical Closed state.

## 10. Alternate Business Flows

### 10.1 Outstanding Borrowings Prevent Closure

If policy requires obligations to be cleared before closure, the membership remains non-closed until the relevant obligations are resolved.

The scenario does not invent a new membership lifecycle state.

### 10.2 Outstanding Fine or Replacement Obligation

Outstanding fines or replacements are reviewed as part of library clearance/closure governance. Finance remains owner of accounting/payment processing.

### 10.3 Closure Not Approved

The membership remains in its existing authoritative state.

### 10.4 Student Lifecycle Has Ended

C remains authoritative for the student's institutional lifecycle. G05 closes only the Library Membership.

### 10.5 Employee Lifecycle Has Ended

E remains authoritative for employee lifecycle. G05 closes only the Library Membership.

## 11. Failure, Interruption and Business Recovery

### 11.1 Obligation Status Cannot Be Established

Closure does not proceed until the relevant library obligation information is available.

### 11.2 Closure Authority Rejects the Decision

The membership remains in its current lifecycle state and the decision is recorded.

### 11.3 Historical Records Are Required

Closure does not delete circulation, borrowing, membership, or audit history. Historical information remains preserved.

### 11.4 Communication Is Required

The closure fact is consumed by H for governed communication; delivery is outside G05.

No technical retry, queue, rollback or implementation mechanics are introduced.

## 12. Business Rules and Decision Constraints

- Library Membership lifecycle includes Closed as the terminal state in the G05 state transition matrix.
- Library Membership owns membership status and history.
- Library Clearance owns outstanding borrowings, outstanding fines, pending replacements and clearance approval.
- Student lifecycle remains C-owned.
- Employee lifecycle remains E-owned.
- Finance owns payment processing/accounting.
- Historical library records remain preserved.
- Lifecycle transitions are audited.
- Membership policies remain configuration driven.

No local closure reason, grace period, automatic closure rule, or financial settlement rule is invented.

## 13. State and Lifecycle Impact

The authoritative lifecycle is:

`Applied → Approved → Active → Suspended → Closed`

This scenario reaches:

`Suspended → Closed`

or, where permitted by the upstream lifecycle semantics, an existing eligible membership progresses to Closed through the authorised closure process.

The scenario does not invent `Pending Closure`, `Clearance Pending`, or `Access Terminated` as membership states.

“Library access termination” is the business consequence of the membership reaching Closed; **Closed** remains the canonical lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

G05 defines:

- `MembershipRequested`
- `MembershipApproved`
- `MembershipSuspended`
- `MembershipClosed`

The applicable event is:

**`MembershipClosed` — published by Library Management.**

No `LibraryAccessTerminated` event is invented.

### 14.2 Synchronous Business Collaboration / Hand-offs

- C/E provide authoritative member context where required.
- Finance remains authoritative for accounting/payment.
- H consumes the membership closure fact for communication.
- Library Clearance provides the applicable library-obligation verification outcome where closure depends on clearance.

## 15. Business Authorization, Approval and Audit Obligations

The G05 authorization model identifies Library Administrator and Librarian as typical operational roles and states that authorization policies are configurable.

Closure approval therefore follows the configured library authority.

The closure decision, obligation verification, effective status and history must remain auditable. Closure does not erase historical records.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Membership identified.
- Closure condition established.
- Applicable library obligations reviewed.
- Required approval completed.
- Membership reaches Closed.
- Library access is terminated as a business consequence.
- Historical records preserved.
- `MembershipClosed` published.
- Required communication outcome made available.

### Postconditions

- Membership status is Closed.
- The closed membership is not eligible for ordinary active-member library services.
- Historical borrowing/membership/audit information remains available.
- C/E institutional lifecycle remains unaffected.
- Finance retains ownership of accounting/payment outcomes.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

No material ownership or event-publisher conflict identified.

### Assumptions

- Closure criteria are configuration/policy driven.
- Library Clearance may be required before closure where outstanding obligations exist.
- Closure does not delete historical records.

### Deferred Questions

Any specific automatic closure trigger or mandatory clearance condition must be established by upstream policy before being added to L3.
