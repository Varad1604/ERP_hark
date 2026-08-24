# Facility Service Request to Operational Resolution

## 1. Scenario Metadata

**Scenario ID:** L3-G-016  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.3  
**Owner:** Pallav  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-G01_Facilities_Management_Workflows.md; L3_Business_Scenario_Template_v2.0.md  
**Referenced By:** None yet  
**Architecture Approval:** ACR-G-2026-08-21-002  
**Approved By:** Arjun Mehta, Chief Enterprise Architect  
**Approval Date:** 21 August 2026  
**Approval Decision:** Approved — No Conditions  

## 2. Scenario Purpose

Provide the governed institutional path from a submitted need for facility support to an accepted and operationally resolved Facilities service request, while preserving the ownership of requester identity and notification delivery.

## 3. Business Outcome

A valid Facility Service Request has been classified, accepted, resolved through an authoritative configured route, closed, and retained in Facilities operational history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A source-supported requester or operational source submits a demand for maintenance, inspection, utility support, facility assistance, or operational intervention.

### 4.2 End Boundary

The Service Request has followed its valid conversion or administrative-resolution route and is closed with its operational reference and history preserved.

### 4.3 In Scope

- request authorization, category, priority, severity, and infrastructure validation
- creation and classification of the Service Request
- conversion to the applicable Facilities workflow or administrative closure
- cross-domain requester context and supported notification need

### 4.4 Out of Scope

- execution of a resulting Complaint, Inspection, Preventive Maintenance activity, or Emergency Work Order beyond the hand-off
- Student or workforce master-data changes
- notification delivery mechanics

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns request validation, classification, conversion, operational resolution, closure, and history. |
| Student Lifecycle (C) | Provides authoritative student identity/context when the requester is a student. |
| People & Workforce (E) | Provides authoritative faculty or employee identity/context when the requester is workforce. |
| Institutional Services (H) | Owns supported notification delivery where awareness is required. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Service Requester | Submits the facility-service demand and supporting business context. | Initiates. |
| Maintenance Supervisor | Validates and classifies the request under Facilities policy. | Reviews and directs the operational route. |
| Facilities Manager | Governs an applicable administrative resolution or operational disposition. | Decides under configured Facilities authority. |

## 7. Preconditions and Entry Criteria

- The requester or operational source is valid for the institution and scope.
- The referenced facility infrastructure can be validated against authoritative infrastructure information.
- Applicable request categories, priorities, severity and conversion policies are effective.

## 8. Trigger

A valid business demand for facility service is submitted by a supported requester or operational source.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the facility-service demand and requester context. | Service Requester / Facilities Management | Request enters validation. |
| 2 | Validate authority, category, priority, severity, and infrastructure reference. | Maintenance Supervisor | Request is valid and classified. |
| 3 | Create and accept the Service Request. | Facilities Management | Authoritative request exists. |
| 4 | Determine the configured operational route: Complaint, Inspection, Preventive Maintenance, Emergency Work Order, or administrative resolution. | Facilities Manager / Facilities Management | Valid resolution route is selected. |
| 5 | Record the conversion or administrative resolution and close the Service Request at its boundary. | Facilities Management | Operationally resolved request and history are available. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Administrative resolution | No operational work is required and policy permits administrative closure. | Record the authorised reason and close the request without creating downstream work. | Request is operationally resolved. | Facilities Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Invalid infrastructure reference | The request cannot progress. | Correct the authoritative reference or reject the request under policy; preserve the failed validation. | Facilities Management |
| Invalid category or authority | The request cannot be accepted. | Correct/resubmit or terminate under configured request policy. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every maintenance activity originates from a valid operational trigger. | Facilities Management | Prevents unsupported work initiation. | L2-G01 §16 |
| Request priority is independent from severity. | Facilities Management | Both are assessed under configuration. | L2-G01 §16 |
| Conversion rules remain configuration-driven. | Facilities Management | Only valid configured resolution routes may be selected. | L2-G01 §16 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Service Request | Draft / Submitted | Validation, acceptance, conversion, and closure. | Accepted → Converted → Closed | Facilities Management | L2-G01 §33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ServiceRequestCreated | Facilities Management | C, E, H as materially interested | Establishes the accepted request. | L2-G01 §31 |
| ServiceRequestConverted | Facilities Management | Owning downstream Facilities workflow; H where supported | Records conversion to the governed operational route. | L2-G01 §31 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| C / E | Facilities Management | Authoritative requester identity/context | Validates the business requester. | L2-G01 §16; DEPENDENCY_MAP.md |
| Facilities Management | H | Supported notification need based on exact Facilities event | Enables awareness without transferring request ownership. | L2-G01 §31 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Submit facility-service demand | Service Requester | Must be an authorised supported requester. | Facilities Management |
| Classify and route request | Maintenance Supervisor / Facilities Manager | Must follow effective Facilities policy. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Request validation and classification | Requester, category, priority, severity, infrastructure reference, action and business reason. | Facilities Management |
| Conversion or administrative resolution | Resolution route, authoritative actor, reason, and request history. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- A valid Service Request exists and was classified under effective policy.
- The request followed one authoritative configured resolution route.
- The request is closed with operational and audit history preserved.

### 16.2 Postconditions

- Facilities retains the authoritative Service Request history.
- Any downstream Facilities activity has its own independent lifecycle and ownership.
- Supported interested domains may consume the resulting event or notification outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
