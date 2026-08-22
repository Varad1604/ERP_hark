# Infrastructure Availability Definition to Availability Profile Activation

# 1. Scenario Metadata

**Scenario ID:** L3-G-289  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** None yet

# 2. Scenario Purpose

Establish the authoritative Availability Profile that determines whether a physical resource may be consumed operationally while keeping availability distinct from the infrastructure lifecycle.

# 3. Business Outcome

An Availability Profile becomes authoritative for the applicable infrastructure.

G10 requires the availability workflow to publish a domain event but does not establish one exact canonical event name for this outcome. The Draft is therefore **BLOCKED**.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when availability conditions must be established for a physical resource.

## 4.2 End Boundary

The scenario ends when the authoritative Availability Profile is established independently from lifecycle and the required canonical availability business event can be identified and published.

## 4.3 In Scope

- Availability definition for G10-owned infrastructure.
- Authorization, lifecycle and business validation.
- Source-defined availability conditions, scheduled closures and partial availability.
- Preservation of the distinction between availability and lifecycle.
- Required canonical business-event publication and auditability.

## 4.4 Out of Scope

- Academic scheduling or other consumer decisions.
- Maintenance execution.
- Creation of an `Active` Availability Profile lifecycle state.
- Any implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G | Owns Availability Profile and the authoritative current usability fact. |
| Platform Foundation / A | Supplies authorization, event governance and audit capabilities without owning availability. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers availability definition for the physical resource. | Acts only where the applicable authorization decision permits the G10 action. |

No universal named availability approver is defined upstream.

# 7. Preconditions and Entry Criteria

- The physical resource exists.
- Its lifecycle permits the availability definition.
- The requested availability condition is supported by institutional configuration and policy.
- The business action is authorized.

# 8. Trigger

Availability conditions must be established for a physical resource.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the physical resource and the availability conditions to be established. | Infrastructure Administrator / Infrastructure Management | The availability definition is bounded to one canonical resource. |
| 2 | Validate authorization, lifecycle compatibility and governing availability rules. | Infrastructure Management | Invalid or unauthorized availability definitions do not proceed. |
| 3 | Apply any source-defined dates, closure reason, responsible authority or partial-scope condition that materially governs availability. | Infrastructure Management | The availability definition reflects the applicable business conditions. |
| 4 | Establish the Availability Profile as the authoritative operational usability fact independently from lifecycle. | Infrastructure Management | The frozen business outcome is achieved. |
| 5 | Publish the canonical availability business event required by the G10 workflow. | Infrastructure Management | **Blocked:** no exact canonical event name is established upstream for this outcome. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Scheduled closure | Availability changes at a future governed period. | Record the source-defined start, end, reason and responsible authority. | Current availability remains governed until the effective closure period. | Infrastructure Management |
| Partial availability | Only a campus, building, floor or selected spaces are affected. | Apply the availability condition only to the governed scope. | Unrelated infrastructure is not implicitly restricted. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Lifecycle or availability-policy validation fails. | No authoritative new availability condition is established. | Correct the business condition before a later authorized availability action. | Infrastructure Management |
| Authorization is not granted. | Availability cannot be changed. | Proceed only after a later valid authorization decision. | Platform Foundation / Infrastructure Management |
| Required canonical availability event is undefined. | The successful outcome cannot satisfy G10's canonical publication rule. | Define the exact event through upstream architecture change control. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Availability is maintained independently from infrastructure lifecycle. | Infrastructure Management | This scenario changes usability facts without inventing a lifecycle transition. | L2-G10 §25 |
| Availability may reflect lifecycle, operational status, closure status and maintenance state. | Infrastructure Management | The authoritative availability fact must remain consistent with its business inputs. | L2-G10 availability rules |
| Manual availability override requires authorization. | Infrastructure Management / Platform Authorization | Unauthorized override cannot become authoritative. | L2-G10 availability rule; L2-A04 AP-01 |
| Every successful G10 workflow publishes at least one canonical business event. | Infrastructure Management | A missing exact event name prevents source-complete completion. | L2-G10 WDP-02 |
| Business domains own their events; Event Infrastructure owns event governance and distribution. | Infrastructure Management / Platform Event Infrastructure | Event participation does not transfer availability ownership. | L2-A06 EIP-01, EIP-02 |

# 13. State and Lifecycle Impact

The source lists availability states such as `Available`, `Reserved`, `Under Maintenance`, `Under Renovation`, `Temporarily Closed`, `Emergency Closed`, and `Operationally Restricted`. It defines no Availability Profile lifecycle state named `Active`.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Availability Profile | No authoritative profile or an existing profile requiring definition | Establishes the current usability conditions | Authoritative Availability Profile; no invented lifecycle state | Infrastructure Management | L2-G10 §25 |
| Physical infrastructure | Existing lifecycle position | Referenced for lifecycle validation only | Infrastructure lifecycle unchanged | Infrastructure Management | L2-G10 §25 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10's Availability Workflow ends with “Publish Domain Event,” and WDP-02 requires a canonical event for every successful workflow. No exact canonical event name for Availability Profile establishment is defined in the locked source. L3 therefore does not invent one.

## 14.2 Business Contracts and Cross-Domain Hand-offs

G10 exposes authoritative availability information for authorized consumers. No ownership transfers through that access.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish infrastructure availability | Infrastructure Administrator | Must satisfy the applicable authorization and G10 availability rules. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule is defined. Manual availability override may occur only where authorization permits it.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve availability governance history | The governed availability condition, effective period/reason where applicable, and source-defined responsible authority. | Infrastructure Management |
| Preserve immutable audit evidence | Audit evidence derived from the completed canonical business fact. | Platform Foundation — Audit |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An Availability Profile is authoritative for the applicable infrastructure.
- Availability remains distinct from lifecycle.
- The exact required canonical availability event is established and published by Infrastructure Management.

## 16.2 Postconditions

- Authorized consumers may rely on the current G10 usability fact.
- Physical-resource ownership remains with Infrastructure Management.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 mandates event publication for the availability workflow but defines no exact canonical event name for Availability Profile establishment. | L3 cannot name the mandatory event without invention, preventing source-complete completion. | Upstream G10/event governance must define the exact event and retain Infrastructure Management as publisher. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
