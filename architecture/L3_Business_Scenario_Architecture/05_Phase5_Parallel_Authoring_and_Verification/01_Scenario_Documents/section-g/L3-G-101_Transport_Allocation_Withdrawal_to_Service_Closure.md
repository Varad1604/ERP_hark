# Transport Allocation Withdrawal to Service Closure

**Scenario ID:** L3-G-101  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-101 |
| Canonical Title | Transport Allocation Withdrawal to Service Closure |
| Primary Owner | Transport Management |
| Participating Domains | G, C, E, F, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
This scenario governs withdrawal of an existing Transport Allocation and closure of its transport service relationship. G04 defines Allocation Withdrawal and the terminal Withdrawn allocation state.

## 3. Business Outcome
The authorised Transport Allocation reaches **Withdrawn**, its service relationship is closed, and its history remains permanently traceable.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An existing active Transport Allocation has a legitimate business reason for withdrawal.
### 4.2 End Boundary
The allocation reaches the canonical **Withdrawn** state.
### 4.3 In Scope
- Identify allocation.
- Establish withdrawal basis.
- Validate configured withdrawal policy.
- Obtain required approval.
- Withdraw allocation.
- Record withdrawal date/history.
- Publish the canonical withdrawal event.
### 4.4 Out of Scope
- Vehicle ownership or maintenance execution.
- Student/workforce lifecycle ownership.
- Financial accounting.
- Notification delivery mechanics.
- APIs, databases, services, queues, UI, deployment or infrastructure.
- Separate downstream transport-pass, trip, financial or communication outcomes.

## 5. Ownership and Participating Domains
Transport Management (G04) is the canonical owner of the Transport Allocation outcome. Participation does not transfer ownership.

G04 defines Transport Allocation as owning Student Allocation, Staff Allocation, Guest Allocation, Allocation Status and Allocation History. Student information remains externally owned by Section C; workforce information remains externally owned by Section E. Finance consumes transport allocation but does not modify transport operations. Parent Communication consumes transport information. None of these relationships transfers allocation ownership.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Administrator | Manages allocation withdrawal. |
| Transport Manager / configured authority | Performs approval responsibility where required. |
| Student / Staff / authorised consumer | May be affected by the service withdrawal but does not own the allocation lifecycle. |

## 7. Preconditions and Entry Criteria
- An active Transport Allocation exists.
- A legitimate withdrawal basis exists.
- Applicable withdrawal/approval policy can be evaluated.

## 8. Trigger
A withdrawal of an active Transport Allocation is requested or authorised.

## 9. Main Business Flow
1. Identify the active allocation.
2. Establish the business withdrawal reason.
3. Validate the request against configured withdrawal policy.
4. Complete required approval.
5. Withdraw the allocation.
6. Record withdrawal date and administrative/business remarks.
7. Preserve complete allocation history.
8. Publish `AllocationWithdrawn`.
9. Make the authoritative withdrawal fact available to participating consumers.

## 10. Alternate Business Flows
- Withdrawal is rejected: allocation remains active.
- Withdrawal information is incomplete: allocation remains active until the business decision can be completed.
- Allocation is already Withdrawn: no second lifecycle outcome is created.
- A transport-pass consequence exists: the separate pass lifecycle scenario remains authoritative.
- A financial consequence exists: Finance consumes the allocation outcome; financial closure remains outside G04.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Withdrawal basis unresolved | Allocation remains active | Complete the business decision |
| Withdrawal rejected | Service relationship remains active | Retain or revise request |
| Allocation already Withdrawn | No new lifecycle transition | Preserve existing historical outcome |
| Financial consequence follows | G04 does not own accounting closure | Provide authoritative allocation outcome to Finance |
| Pass/communication consequence follows | Separate downstream outcome | Allow the relevant canonical scenario to consume the withdrawal fact |

No technical retries, queues, rollback or infrastructure recovery mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule | Effect |
|---|---|
| Transport Allocation owns allocation status/history. | Withdrawal is G04-owned. |
| Allocation lifecycle ends in Withdrawn. | Withdrawal completes the allocation lifecycle. |
| Historical allocation information includes withdrawal date and change history. | Closure preserves the prior service relationship. |
| Finance consumes transport allocation but does not modify transport operations. | Financial processing does not transfer ownership. |
| Every lifecycle transition is audited. | Withdrawal is permanently traceable. |
| Student/workforce lifecycle remains externally owned. | Withdrawal does not close student/employee lifecycle. |

## 13. State and Lifecycle Impact
Canonical lifecycle: `Applied → Approved → Allocated → Active → Withdrawn`. This scenario covers `Active → Withdrawn`. Later pass, financial, student/workforce or communication consequences are separate workflows.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
**`AllocationWithdrawn`** — allocation withdrawn. **Canonical publisher: Transport Management (G04).** No separate `TransportServiceClosed` event is invented.
### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participating domains are G, C, E, F and H. G04 owns withdrawal; C/E may consume the outcome for student/workforce context; F consumes transport allocation for financial processing; H may consume it for governed communication. None modifies G04 allocation state.

## 15. Business Authorization, Approval and Audit Obligations
G04 identifies Transport Administrator as a typical allocation authority; approval policies remain configurable. Withdrawal reason, decision, effective date, remarks and allocation history are subject to immutable operational audit.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Active allocation identified.
- Withdrawal basis validated.
- Required approval completed.
- Allocation reaches **Withdrawn**.
- `AllocationWithdrawn` is published.
- Withdrawal date/history are preserved.
### 16.2 Postconditions
- The named allocation outcome is authoritative within G04.
- Allocation history remains traceable.
- No unrelated domain lifecycle is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material source conflict identified. The scenario intentionally stops at allocation withdrawal and does not absorb pass closure, financial settlement or communication delivery.
### 17.2 Assumptions
- G04 configuration-driven policies remain authoritative.
- No additional actor, rule, state, event, publisher, owner or dependency is invented.
### 17.3 Deferred Questions
Any material upstream ambiguity must be resolved through architecture governance before Architecture Approval/Lock.
