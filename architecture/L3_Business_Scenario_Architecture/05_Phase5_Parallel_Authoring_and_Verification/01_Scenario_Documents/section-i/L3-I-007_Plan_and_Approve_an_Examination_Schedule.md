# 1. Scenario Metadata

# Plan and Approve an Examination Schedule

**Scenario ID:** L3-I-007  
**Primary Owner:** Examination Planning & Scheduling  
**Participating Domains:** Institution and Academic Structure, Academic Delivery, Examination  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Author:** Pallav  
**Previous Architecture Approval:** ACR-I-2026-08-21-002 — Approved by Vikram Desai, Enterprise Architecture Governance Lead, 21 August 2026; Conditions: None  
**Previous Post-Approval Lock:** Authorized 21 August 2026 under ACR-I-2026-08-21-002; superseded by corrected v1.1  
**Architecture Re-review:** Architecture Approved on 2026-08-26 after correction of actor, rule, lifecycle-entry and event mappings; repository lock remains a separate governance action  
**Canonical Source Candidate:** CAN-L3-I-007  
**Primary L2 Source:** L2-I03  
**Last Updated:** 2026-08-26

# 2. Scenario Purpose

This scenario governs **plan and approve an examination schedule** as one complete institutional business outcome. It applies frozen semantics and locked L2-I03 without redefining upstream architecture.

# 3. Business Outcome

The scenario concludes when **plan and Approve an Examination Schedule** is established as the authoritative governed outcome with required business evidence and hand-offs.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario starts when an **Active Examination Session** establishes a valid planning context and a governed need for an examination schedule exists.

## 4.2 End Boundary

The scenario ends when **plan and Approve an Examination Schedule** is authoritatively determined and recorded for downstream business use.

## 4.3 In Scope

- Verification of source-defined context and entry conditions.
- Application of L2-I03 rules and authority.
- Frozen cross-domain collaboration.
- Establishment and preservation of the final outcome.
- Source-supported events, hand-offs, evidence, and audit obligations.

## 4.4 Out of Scope

- Redefinition of upstream policy, eligibility, academic, workforce, facility, finance, or identity rules.
- Ownership of upstream facts or downstream lifecycles.
- Outcomes beyond this scenario end boundary.
- Technical solution, data, interface, access-control, infrastructure, or recovery design.

## 4.5 Related Scenarios — Optional

Related scenarios provide prerequisites or consume this outcome under locked dependency direction without ownership transfer.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Examination Planning & Scheduling | Owns the governed final outcome and applicable aggregate lifecycle. |
| Institution and Academic Structure | Provides only canonically owned facts, decisions, resources, assignments, or confirmations. |
| Academic Delivery | Provides only canonically owned contributions required for planning. |

The Primary Owner retains the outcome; participating domains retain their canonical responsibilities.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility |
|---|---|
| Examination Administrator | Prepares and manages examination plans and schedules. |
| Controller of Examination | Reviews, approves and governs examination schedules. |
| Academic Administration | Validates academic applicability and examination sequencing. |
| Department Administration | Coordinates departmental examination planning. |
| Campus Administration | Coordinates venue availability and campus resources. |
| Infrastructure Administration | Provides authoritative examination venue and facility information. |
| Faculty Administration | Coordinates invigilator and supervisory resource planning. |

Only responsibilities supported by locked L2-I03 apply. Supporting domains retain ownership of their facts and resources.

# 7. Preconditions and Entry Criteria

- The governing Examination Session is **Active**, as required by L2-I03.
- Approved policy and source-owned facts are authoritative.
- Required upstream academic-calendar, curriculum, campus, infrastructure and resource information is available where applicable.
- Initiation and decision responsibilities are valid.
- No unresolved ownership, identity, scope or lifecycle conflict prevents progression.

# 8. Trigger

**Governed need for plan and approve an examination schedule within an Active Examination Session.**

This is a business occurrence or authoritative condition, not an implementation action.

# 9. Main Business Flow

1. The responsible participant recognizes the authoritative trigger within an Active Examination Session.
2. The Primary Owner verifies source-defined preconditions and upstream facts.
3. Participating domains provide only their canonically owned contributions.
4. Examination Planning prepares the schedule, including applicable timetable, venue and resource planning.
5. Scheduling conflicts are validated and resolved before approval.
6. Examination participants assess the completed schedule under approved policy and L2-I03.
7. The authorized institutional decision establishes the approved Examination Schedule.
8. Required evidence and source-supported events or hand-offs are preserved.
9. The scenario stops at approval; publication remains a separate downstream lifecycle action under L2-I03.

# 10. Alternate Business Flows

## 10.1 Source-Defined Conflict Resolution

Where conflict validation identifies an academic-calendar, venue, resource, capacity or timetable conflict, the schedule returns to governed planning and correction before approval can proceed.

## 10.2 Approval Rejection

Where institutional reviewers reject the Examination Schedule, it may return for modification, replanning, cancellation or replacement as permitted by L2-I03. A rejected schedule is not published.

## 10.3 Authoritative Input Requires Clarification

The owning domain resolves an incomplete or inconsistent input; Examination Planning does not redefine it.

# 11. Failure, Interruption and Business Recovery

- Planning does not begin without an Active Examination Session.
- Schedule approval remains blocked while material academic-calendar, venue, capacity, resource or timetable conflicts remain unresolved.
- Progression is suspended if required authority, evidence or authoritative input is unavailable.
- Disputed facts return to their canonical owner.
- The last valid business position and evidence are preserved.
- Recovery occurs only after the missing condition is satisfied or an authorized terminal decision is reached.
- Technical exception and recovery mechanisms are outside L3.

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Scenario Effect | Owner / Source |
|---|---|---|
| Planning begins only within an Active Examination Session. | The scenario cannot validly begin before the governing session is Active. | L2-I03 §§4.1, 8.2, 10.1 |
| Scheduling conflicts must be validated and resolved before approval. | Approval remains blocked while material conflicts remain unresolved. | L2-I03 §§5.6, 10.2–10.5 |
| Schedule approval requires completed planning, successful validation, confirmed resource availability and institutional governance compliance. | Only a sufficiently governed schedule can reach Approved. | L2-I03 §§5.7, 9.5 |
| Only approved schedules may proceed to publication. | This scenario stops at the approved outcome and does not absorb publication. | L2-I03 §§5.7–5.8, 9.5–9.6 |
| Resource, calendar, curriculum and infrastructure facts remain owned by their originating domains. | Examination Planning consumes authoritative facts without transferring ownership. | L2-I03 §§4.5, 4.9, 13 |

No local threshold, scheduling algorithm, authority, state or rule is created.

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State / Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Examination Schedule | Planning initiated within an Active Examination Session | Prepare, validate, review and authorize the schedule. | Approved | Examination Planning & Scheduling | L2-I03 §§4.7, 8, 9.5 |

This scenario does not publish the schedule or create downstream execution states. Activities are not lifecycle states, and no hidden transition is introduced.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

### Published Events

| Event | Canonical Publisher | Published When | Material Consumers |
|---|---|---|---|
| Examination Planning Initiated | Examination Planning & Scheduling | Planning begins. | I04 |
| Examination Schedule Created | Examination Planning & Scheduling | Initial schedule is prepared. | I04, I05 |
| Schedule Updated | Examination Planning & Scheduling | Governed planning changes are completed. | I04, I05 |
| Conflict Validation Completed | Examination Planning & Scheduling | Conflict validation concludes. | Governance Workflows |
| Schedule Approved | Examination Planning & Scheduling | Institutional approval is granted. | I04 |

`Schedule Published` is not produced by this scenario because publication occurs after the approved outcome and remains governed by L2-I03's publication lifecycle.

### Consumed Events

| Event | Canonical Publisher | Business Purpose |
|---|---|---|
| Examination Session Activated | L2-I02 | Establishes the Active Examination Session required for planning. |
| Examination Policy Updated | L2-I01 | Supplies authoritative planning policy. |
| Academic Calendar Updated | Academic Calendar Domain | Supplies authoritative examination-window context. |
| Curriculum Updated | Curriculum Framework Domain | Supplies authoritative programme/course applicability. |
| Campus Infrastructure Updated | Campus & Infrastructure Domain | Supplies authoritative venue/facility context. |
| Resource Availability Updated | Organisation / Infrastructure Domains | Supplies authoritative resource-readiness context. |
| Administrative Approval Granted | Governance Processes | Supplies authoritative governance approval where applicable. |

Event names and publisher direction are taken exactly from locked L2-I03; no workflow identifier is treated as an event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Hand-off |
|---|---|---|
| Examination Session Management | Examination Planning & Scheduling | Active Examination Session and governing session context. |
| Institution and Academic Structure | Examination Planning & Scheduling | Canonically owned academic-calendar, curriculum, organisational and institutional facts required for planning. |
| Academic Delivery / collaborating resource domains | Examination Planning & Scheduling | Canonically owned academic-delivery, venue, facility and resource information where applicable. |

These are business hand-offs only, with no technical contract design and no ownership transfer.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Examination Administrator prepares and manages planning; Controller of Examination reviews, approves and governs examination schedules; other named L2-I03 actors contribute only within their source-defined responsibilities. Institutional configuration may govern the detailed approval route without changing canonical ownership.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional separation-of-duty, override or exceptional authority is created locally. Any exceptional approval or rescheduling follows authoritative institutional governance.

## 15.3 Business Audit and Evidence Obligations

The institution must demonstrate the governing Active Examination Session, planning context, schedule content, source-owned contributions, conflict-validation result, responsible participants, approval decision, prior planning position where changed and final approved outcome.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An Active Examination Session governed the planning context.
- Required schedule content and source-owned contributions were available.
- Material conflicts were validated and resolved.
- Required institutional review and approval were completed.
- `Schedule Approved` records the approved Examination Schedule outcome.
- Required evidence and hand-offs are preserved.
- Publication and downstream execution outcomes are not absorbed.

## 16.2 Postconditions

- The Examination Schedule is authoritative in its approved position for its scope.
- Examination Planning & Scheduling retains ownership of the schedule.
- Source domains retain ownership of calendar, curriculum, infrastructure, resource and other contributed facts.
- Publication may proceed only through its separately governed L2-I03 lifecycle.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No scenario-specific material conflict remains after the 2026-08-26 correction of actor, rule, lifecycle-entry and event mappings.

## 17.2 Assumptions

No assumption overrides locked architecture. Referenced locked sources are assumed unchanged.

## 17.3 Deferred Questions

System behaviour, solution, integration, security, data and user-experience design are deferred to L4–L6. Scheduling algorithms, optimization techniques, technical access control and implementation recovery are not defined at L3.
