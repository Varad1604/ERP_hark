# Resident Leave Request to Verified Return

**Scenario ID:** L3-G-082  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-082 |
| Canonical Title | Resident Leave Request to Verified Return |
| Primary Owner | Hostel Management |
| Participating Domains | G, C, H |
| Scenario Type | Cross-domain |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern a resident's temporary absence from the hostel from leave request through approved departure and verified return, while preserving Hostel Management ownership of leave and residency operations and Student Lifecycle ownership of student identity.

## 3. Business Outcome

The resident's leave is governed through approval, the resident departs under the approved leave conditions, and the resident's return is verified and the leave lifecycle is closed.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an active hostel resident submits a leave request.

### 4.2 End Boundary

The scenario ends when the resident's return to the hostel is verified and the Leave Request reaches `Closed`.

### 4.3 In Scope

- Leave request submission.
- Hostel review and approval.
- Recording approved leave duration.
- Resident departure under the approved leave.
- Return confirmation.
- Verification of return.
- Closure of the leave lifecycle.
- Communication of material leave outcomes through governed notification participation.

### 4.4 Out of Scope

- Changes to Student Lifecycle status.
- Permanent residency termination.
- Curfew-exception or disciplinary scenarios after the leave boundary.
- Visitor management.
- Financial accounting.
- Technical notification delivery or implementation mechanics.

### 4.5 Related Scenarios

- **L3-G-081 — Hostel Residency End to Cleared and Closed Occupancy** is a separate permanent residency-closure outcome.
- **L3-G-083 — Curfew Exception to Authorised Late Entry** is a separate scenario and is not absorbed into this leave scenario.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Hostel Management (G) | Owns Leave Request, leave lifecycle, resident movement during leave, and return verification. |
| Student Lifecycle (C) | Supplies canonical student identity/context; does not own hostel leave state. |
| Notifications (H) | Supports governed communication of leave-related business outcomes; does not own leave state. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student / Resident | Submits leave request, departs under approved leave, and confirms return. | Initiates and participates in the business process. |
| Hostel Warden | Reviews and governs leave approval under applicable hostel policy. | Business approval participation supported by L2-G03. |
| Hostel Administrator | Performs hostel operational review and return verification as applicable. | Operational responsibility within Hostel Management. |

The exact institution-specific approval chain remains configuration driven where G03 does not prescribe a fixed sequence.

## 7. Preconditions and Entry Criteria

- The resident has an active Hostel Occupancy.
- The resident is eligible to request leave under applicable hostel policy.
- The canonical student identity/context is available from Section C.
- The requested leave contains sufficient business information for hostel review under the applicable policy.

## 8. Trigger

An active hostel resident submits a Leave Request.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The resident submits a Leave Request. | Student / Resident | Leave Request enters `Applied`. |
| 2 | Hostel Management reviews the request against applicable leave rules, duration, resident status, and hostel policy. | Hostel Administrator / Hostel Warden | Request reaches the governed review decision. |
| 3 | The request is approved where the applicable hostel authority permits the leave. | Hostel Management | Leave Request reaches `Approved`. |
| 4 | The resident departs under the approved leave period. | Student / Resident | Leave is operationally commenced. |
| 5 | Hostel Management records the resident's return when the resident returns to the hostel. | Hostel Management | Return is identified for verification. |
| 6 | Hostel Management verifies the resident's return and closes the leave lifecycle. | Hostel Management | Leave Request reaches `Closed` and the resident is again present under the existing active occupancy. |

**Scenario End:** The flow stops when return is verified and the leave is closed. Any later curfew exception, disciplinary action, or permanent residency closure is outside this scenario.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture beyond approval/rejection and return-verification outcomes.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Leave request does not satisfy applicable hostel policy. | The request cannot be approved as submitted. | Reject or otherwise resolve the request under the configured hostel leave process; no leave is created from an unapproved request. | Hostel Management |
| Required approval is not completed. | The resident cannot proceed under approved leave conditions. | Keep the request in the applicable review position until the governed decision is completed. | Hostel Management |
| Resident does not return as expected under the approved leave. | Return cannot be verified and the leave cannot be closed normally. | Apply the applicable hostel movement/curfew/discipline process rather than falsely closing the leave. | Hostel Management |

The source does not define a separate technical recovery path and none is invented.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Leave Request belongs to an active resident. | Hostel Management | Leave cannot exist independently of active residency. | L2-G03 §13 |
| Leave lifecycle is `Applied → Under Review → Approved → Resident Left → Resident Returned → Closed`. | Hostel Management | The scenario follows the canonical leave lifecycle without adding states. | L2-G03 §11, §20, §30 |
| Leave rules and approval workflows are configuration driven. | Hostel Management | Institution-specific leave policy governs decision handling. | L2-G03 §6, §20, §33 |
| Student identity and academic lifecycle remain owned by Section C. | Student Lifecycle | Leave does not modify student lifecycle. | L2-G03 §§1, 4, 9; L2-C02 |
| Significant hostel operational transitions are auditable and event driven. | Hostel Management | Leave outcomes remain traceable and available to downstream consumers. | L2-G03 §§6, 27–28 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Leave Request | Applied | Review and governed approval occur. | Under Review / Approved | G03 Hostel Management | L2-G03 §11, §20 |
| Leave Request | Approved | Resident departs under approved leave. | Resident Left | G03 Hostel Management | L2-G03 §11, §20 |
| Leave Request | Resident Left | Return is verified. | Resident Returned | G03 Hostel Management | L2-G03 §11, §20 |
| Leave Request | Resident Returned | Leave process is completed. | Closed | G03 Hostel Management | L2-G03 §11, §20 |

Hostel Occupancy remains active; temporary leave does not create a new occupancy lifecycle or transfer ownership to another domain.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03 establishes event-driven hostel operations and downstream notification consumption. The exact event catalogue names for each leave transition are not asserted in the source material used for this draft, so no unsupported event names are invented.

| Business Event / Fact | Canonical Publisher | Consumer / Interested Domain | Purpose |
|---|---|---|---|
| Leave approval / resident movement outcome | G03 Hostel Management | Notifications and other approved consumers | Communicates an authoritative hostel leave fact without transferring leave ownership. |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle (C) | Hostel Management (G) | Student Identity / student context | Establishes the canonical resident identity. | L2-G03 §9; L2-C02 |
| Hostel Management (G) | Notifications (H) | Hostel leave business outcome / published event | Supports governed notification of leave outcomes. | L2-G03 §§9, 27–28; L2-H02 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Submit Leave Request | Student / Resident | Must be an active resident. | Hostel Management |
| Review / approve leave | Hostel Warden / Hostel Administrator as configured | Leave approval is governed by configurable hostel policy. | Hostel Management |
| Verify return | Hostel Management operational authority | Return must be verified before closure. | Hostel Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty requirement is explicitly defined by G03.

No additional override authority is invented.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Leave lifecycle must remain auditable. | Immutable hostel operational history covering the leave decision and movement outcome. | Hostel Management |
| Resident return must be verifiable before closure. | Business evidence of return verification sufficient to establish the `Resident Returned` and `Closed` lifecycle positions. | Hostel Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Leave Request has passed the applicable approval path.
- Resident departure occurred under the approved leave.
- Resident return has been verified.
- Leave Request reaches `Closed`.
- The resident's active Hostel Occupancy remains intact.

### 16.2 Postconditions

- The temporary leave is closed.
- The resident is again operationally present within the existing hostel occupancy.
- Student Lifecycle remains unchanged.
- Relevant downstream notification consumers may act on the completed leave outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The supplied G03 source defines the leave lifecycle and event-driven model but the exact canonical names of individual leave events are not asserted in the material used for this draft. | Specific event names must not be fabricated. Publisher ownership remains unambiguous as G03. | Confirm the exact canonical G03 leave-event names before Architecture Approval/Lock. |

### 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Hostel Warden and Hostel Administrator are the relevant operational authorities where the institution's configured leave workflow assigns those responsibilities. | G03 defines these hostel operational roles and configurable approval workflows. | Institution-specific approval sequencing may differ. |

### 17.3 Deferred Questions

No scenario-specific deferred L4/L5/L6 question is required.
