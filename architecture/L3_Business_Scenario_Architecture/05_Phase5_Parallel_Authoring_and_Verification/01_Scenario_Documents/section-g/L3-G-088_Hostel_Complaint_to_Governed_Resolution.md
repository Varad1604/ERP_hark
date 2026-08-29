# Hostel Complaint to Governed Resolution

**Scenario ID:** L3-G-088  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** C, G  
**Scenario Type:** Critical  
**Priority:** P0  
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
| Final L3 ID | L3-G-088 |
| Canonical Title | Hostel Complaint to Governed Resolution |
| Primary Owner | Hostel Management |
| Participating Domains | C, G |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern a hostel complaint from registration through review and an authorised operational resolution while preserving Hostel Management ownership of hostel complaints and keeping Student Lifecycle and Facilities responsibilities separate.

## 3. Business Outcome

A hostel complaint is reviewed, resolved through the applicable hostel operational path, and closed with an auditable outcome; where maintenance is required, the maintenance responsibility remains with Facilities.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a hostel complaint is registered.

### 4.2 End Boundary

The scenario ends when the complaint reaches its governed resolution and closure.

### 4.3 In Scope

- Complaint registration and classification.
- Hostel operational review.
- Resolution through Hostel Management where the issue is within hostel operational scope.
- Maintenance recommendation/hand-off where Facilities execution is required.
- Verification of the business resolution.
- Complaint closure and historical retention.

### 4.4 Out of Scope

- Physical infrastructure ownership.
- Facilities work-order execution.
- Student lifecycle changes.
- Financial accounting.
- Technical evidence or notification implementation.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Hostel Management (G03) | Owns Hostel Complaint registration, classification, operational resolution, and complaint lifecycle. |
| Student Lifecycle (C02) | Provides student identity/context where the complaint concerns a resident; does not own the complaint. |
| Facilities Management (G01) | Owns maintenance execution when a complaint requires maintenance. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Resident / Student | Raises or is associated with the hostel complaint. |
| Hostel Administrator | Reviews and manages the complaint operationally. |
| Hostel Warden | Provides hostel governance where escalation or decision authority requires it. |
| Facilities operational role | Handles maintenance execution only when a maintenance path is required. |

## 7. Preconditions and Entry Criteria

- The complaint is within Hostel Management scope.
- The complaint identifies the relevant hostel/resident context.
- A valid student identity/context is available where the complaint concerns a resident.

## 8. Trigger

A hostel complaint is registered.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The complaint is registered and classified against the relevant hostel context. | Resident / Hostel Management | Complaint reaches `Registered`. |
| 2 | Hostel Management reviews the complaint and determines the appropriate operational resolution path. | Hostel Administrator / Warden | Complaint reaches `Reviewed`. |
| 3 | Where the issue is within hostel operational responsibility, Hostel Management resolves it through its own operational process. | G03 | A hostel-owned resolution is established. |
| 4 | Where the complaint requires maintenance execution, Hostel Management makes the maintenance recommendation/hand-off to Facilities. | G03 → G01 | Facilities becomes responsible for maintenance execution. |
| 5 | The applicable resolution is verified and the complaint is closed. | G03 | Complaint reaches `Closed`. |
| 6 | Hostel Management retains the complaint and resolution history. | G03 | Complete historical complaint record is available. |

**Scenario End:** The flow stops at complaint closure.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| 1 | Complaint concerns maintenance rather than hostel operational governance. | Route the maintenance requirement to Facilities while retaining the complaint as a Hostel Management record. | Complaint can reach governed closure after the applicable resolution is established. | G03 / G01 |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Complaint cannot be substantiated or classified sufficiently. | A governed resolution cannot yet be concluded. | Obtain the required business information through the hostel complaint process. | G03 |
| Complaint requires maintenance execution. | G03 cannot itself execute the maintenance work. | Maintain the hostel complaint boundary and route the maintenance requirement to G01. | G03 / G01 |
| Resident identity/context is invalid. | Resident-linked complaint cannot be resolved against the correct student context. | Resolve the authoritative student reference before concluding the resident-linked outcome. | C02 / G03 |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Hostel Complaint owns complaint registration, classification, operational resolution, and maintenance recommendation. | G03 | Complaint ownership remains in Hostel Management. | L2-G03 §8 |
| Hostel complaints shall never directly create maintenance work orders. | G03 | Maintenance execution remains outside the complaint aggregate. | L2-G03 §13 |
| Maintenance execution remains within Facilities. | G01 | G03 can recommend/hand off but does not execute maintenance. | L2-G03 §4; L2-G01 |
| Student identity remains owned by C02. | C02 | Complaint does not create or modify student identity. | L2-G03 §4; L2-C02 |

## 13. State and Lifecycle Impact

G03 explicitly defines the Hostel Complaint lifecycle:

`Registered → Reviewed → Recommendation → Resolved → Closed`

| Aggregate | Entry | Outcome | Owner |
|---|---|---|---|
| Hostel Complaint | Registered | Closed after governed resolution | G03 |

Facilities maintenance has its own lifecycle and remains separate; G03 does not re-own it.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03 requires significant operational transitions to publish immutable domain events. The exact canonical event names for Hostel Complaint transitions were not exposed in the authoritative material retrieved.

| Business Fact | Canonical Publisher | Consumer / Participant |
|---|---|---|
| Hostel complaint resolution outcome | G03 | G01 where maintenance collaboration is required; H/other consumers where governed |

### 14.2 Synchronous Business Contracts / Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C02 | G03 | Canonical student identity/context | Identify the resident. |
| G03 | G01 | Maintenance recommendation / requirement | Transfer maintenance execution responsibility without transferring complaint ownership. |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Hostel complaint review and resolution authority remains with Hostel Management under its configured operational policy.

### 15.2 Separation of Duty / Override

No complaint-specific separation-of-duty rule is exposed in the retrieved authoritative material. No additional rule is invented.

### 15.3 Audit / Evidence

Complaint registration, review, recommendation, resolution, verification, and closure history must remain auditable within Hostel Management.

Facilities retains its own maintenance execution history when involved.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Complaint is registered and classified.
- Appropriate resolution path is determined.
- Required hostel resolution or maintenance hand-off is completed.
- Resolution is verified.
- Complaint reaches `Closed`.
- Complaint history is retained.

### 16.2 Postconditions

- Hostel Management owns the completed complaint history.
- Any maintenance execution remains G01-owned.
- Student identity remains C02-owned.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical G03 complaint event names were not exposed in the retrieved authoritative material. | Event naming cannot safely be asserted. | Confirm the canonical G03 event catalogue before Architecture Approval/Lock. |

### 17.2 Assumptions

The configured hostel complaint policy identifies the applicable review authority and escalation path.

### 17.3 Deferred Questions

Technical maintenance work-order mechanics and notification delivery remain outside L3.
