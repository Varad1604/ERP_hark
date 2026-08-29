# Deferred Preventive Maintenance to Controlled Rescheduling

## 1. Scenario Metadata

**Scenario ID:** L3-G-024  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Facilities Management  
**Participating Domains:** G, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
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

Govern exceptional deferral of a scheduled preventive-maintenance occurrence and establish an auditable replacement schedule without silently discarding the original obligation.

## 3. Business Outcome

The preventive-maintenance occurrence has an authorised deferral reason and a controlled replacement schedule, with original and successor history preserved.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A scheduled preventive-maintenance occurrence cannot proceed because of a source-supported exceptional circumstance.

### 4.2 End Boundary

An authorised reason and replacement schedule are recorded and the maintenance obligation remains visible for execution.

### 4.3 In Scope

- deferral eligibility and authority
- business reason and audit history
- replacement scheduling
- workforce/resource context and supported notification need

### 4.4 Out of Scope

- inventing a Deferred lifecycle state
- scheduling implementation design
- silent cancellation or deletion of the original schedule

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Facilities Management (G01) | Owns the deferral decision, rescheduling and maintenance history. |
| People & Workforce (E) | Provides authoritative resource/availability context where relevant. |
| Institutional Services (H) | Owns supported notification delivery. |

Participation does not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Maintenance Supervisor | Identifies the exceptional circumstance and proposes rescheduling. | Initiates/reviews. |
| Facilities Manager | Authorises the deferral under institutional policy. | Decides. |

## 7. Preconditions and Entry Criteria

- A valid scheduled Preventive Maintenance occurrence exists.
- A supported reason exists: infrastructure closure, emergency activity, resource constraint, or administrative approval context.
- Deferral policy is effective.

## 8. Trigger

An exceptional business circumstance prevents a scheduled Preventive Maintenance occurrence from proceeding as planned.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Record and validate the exceptional circumstance. | Maintenance Supervisor | Deferral eligibility is known. |
| 2 | Assess operational, resource and maintenance impact. | Facilities Management / E | Controlled rescheduling constraints are known. |
| 3 | Authorise the deferral with a business reason. | Facilities Manager | Deferral is governed and auditable. |
| 4 | Establish the replacement schedule under effective maintenance policy. | Facilities Management | Controlled rescheduling is complete. |
| 5 | Preserve the original occurrence and provide supported notification need. | Facilities Management / H | Historical continuity and awareness are maintained. |

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Deferral reason or authority invalid | Scheduled obligation cannot be deferred. | Retain the original schedule or escalate under maintenance policy. | Facilities Management |
| No valid replacement schedule | Controlled rescheduling is incomplete. | Escalate and keep the maintenance obligation visible. | Facilities Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Deferral is allowed only under exceptional circumstances. | Facilities Management | Constrains scenario entry. | L2-G01 §20 |
| Deferred maintenance requires an authorised reason and complete audit history. | Facilities Management | Makes the decision governed and traceable. | L2-G01 §20 |
| Historical schedules must not be silently discarded. | Facilities Management | Preserves maintenance continuity. | L2-G01 §20 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Preventive Maintenance | Scheduled | Authorised deferral and controlled rescheduling; no new Deferred state is introduced. | Scheduled (replacement occurrence established) | Facilities Management | L2-G01 §§20, 33 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

No scenario-specific event publication or consumption is required by the locked L2 source or by a confirmed downstream consumer contract. The scenario therefore completes through the source-supported owned lifecycle and immutable audit record; no event is invented.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| E | Facilities Management | Authoritative resource/availability context where the reason is resource constraint | Supports the decision without workforce ownership transfer. | L2-G01 §§19–20 |
| Facilities Management | H | Supported awareness requirement for the rescheduled occurrence | Enables delivery where configured. | L2-G01 §20 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Propose deferral | Maintenance Supervisor | Must state a supported exceptional circumstance. | Facilities Management |
| Authorise deferral and replacement schedule | Facilities Manager | Authorised reason and audit history are required. | Facilities Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty obligation is defined by the authoritative L2 architecture. Any override stated above requires the source-supported authority, business reason, and audit history.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Deferral decision | Original schedule, exceptional reason, decision actor and outcome. | Facilities Management |
| Controlled rescheduling | Replacement schedule and relationship to the original occurrence. | Facilities Management |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The exceptional circumstance and authority are valid.
- A controlled replacement schedule exists.
- Original and successor maintenance history is preserved.

### 16.2 Postconditions

- The maintenance obligation remains visible and executable.
- No unsupported lifecycle state was introduced.
- Affected participants may receive supported notification delivery.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No open issues.

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No predetermined questions. Any further question must be triggered by new source evidence or a validation failure.
