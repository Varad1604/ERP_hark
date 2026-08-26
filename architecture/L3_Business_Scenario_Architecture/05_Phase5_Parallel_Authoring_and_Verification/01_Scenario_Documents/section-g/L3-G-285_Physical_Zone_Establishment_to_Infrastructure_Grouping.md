# Physical Zone Establishment to Infrastructure Grouping

# 1. Scenario Metadata

**Scenario ID:** L3-G-285  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows  
**Referenced By:** Infrastructure consumers using G10 Zone relationships

---

# 2. Scenario Purpose

Establish the G10-owned Zone used to group applicable physical infrastructure for operational administration while preserving the canonical physical Campus → Building → Floor → Space hierarchy.

---

# 3. Business Outcome

A canonical Zone is established and linked to its applicable physical infrastructure.

The locked G10 source conflicts on the Zone-registration event and on the lifecycle position of a newly registered Zone. This Draft is therefore **BLOCKED**.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an institution requires physical areas to be grouped into an operational zone.

## 4.2 End Boundary

The scenario ends when the Zone exists canonically under G10, the applicable infrastructure memberships are established without changing physical hierarchy, the Zone lifecycle position is unambiguous, and all required canonical Zone events have been published.

## 4.3 In Scope

- Zone registration under Infrastructure Management.
- Zone Type and Zone Name validation.
- Assignment of applicable infrastructure to the Zone.
- Preservation of physical hierarchy and infrastructure ownership.
- Required Zone registration and membership events.
- Auditability of Zone establishment and membership.

## 4.4 Out of Scope

- Re-parenting Buildings, Floors or Spaces.
- Creation of physical hierarchy.
- Collection or Portfolio management.
- Security access-control operations or consumer-domain operational decisions.
- Any implementation mechanism.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-G-281 — Infrastructure Hierarchy Creation to Canonical Physical Structure | The Zone groups infrastructure without becoming a level in the canonical physical hierarchy. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / G10 | Sole owner of the Zone aggregate, Zone Membership, Zone lifecycle and grouping rules. |

The frozen title uses “Physical Zone”; the locked G10 definition treats `Zone` as an **operational logical grouping of physical infrastructure**, not as a new physical hierarchy level. This Draft preserves that owner-defined meaning.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Creates the Zone and manages Zone membership. | Performs the source-defined Zone registration and relationship activities within authorized scope. |

---

# 7. Preconditions and Entry Criteria

- The required Zone Type is configured.
- The proposed Zone Name satisfies the source-defined uniqueness requirement.
- The infrastructure to be linked exists canonically in G10.
- The Infrastructure Administrator is authorized for Zone registration and relationship management.

---

# 8. Trigger

An institution requires physical areas to be grouped into an operational zone.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Define the intended Zone and resolve its configured Zone Type. | Infrastructure Administrator / Infrastructure Management | The proposed Zone is grounded in G10 configuration. |
| 2 | Validate Zone metadata and the Zone Name uniqueness rule. | Infrastructure Management | Invalid Zone establishment is prevented. |
| 3 | Establish the canonical Zone under G10 ownership. | Infrastructure Management | The Zone aggregate exists. |
| 4 | Identify the applicable physical infrastructure to be grouped. | Infrastructure Administrator / Infrastructure Management | Membership candidates are resolved without re-parenting them. |
| 5 | Validate and establish Zone Membership for the applicable infrastructure. | Infrastructure Management | The infrastructure is linked to the Zone while retaining its physical hierarchy. |
| 6 | Record the Zone and membership changes as auditable business actions. | Infrastructure Management | Establishment and grouping are traceable. |
| 7 | Publish the canonical Zone-registration and `AssignedToZone` membership events. | Infrastructure Management | Membership event is defined; **Zone-registration event remains blocked by conflicting names**. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Multiple Zone membership | Infrastructure already belongs to another Zone. | Permit an additional valid Zone membership because G10 explicitly allows multiple Zone memberships. | The new Zone link is added without replacing existing memberships or hierarchy. | Infrastructure Management |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Zone Type is not configured or Zone Name is invalid. | Zone establishment cannot proceed. | Correct the source-owned Zone definition before a later authorized attempt. | Infrastructure Management |
| Proposed membership would be treated as a physical hierarchy change. | The scenario would violate Zone relationship rules. | Establish only a logical Zone Membership; do not alter physical hierarchy. | Infrastructure Management |
| Authorization fails. | Zone creation or membership change cannot proceed. | Obtain source-supported authorization before resubmission. | Infrastructure Management |
| Registration event conflict remains unresolved. | Canonical Zone registration cannot be completed with one exact event identity. | Resolve the conflicting locked event definition through architecture change control. | Infrastructure Management |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Zone is an operational grouping and Zone Membership never changes physical hierarchy. | Infrastructure Management | Grouping cannot re-parent physical infrastructure. | L2-G10 §§7, 8.5; BR-RL-01 |
| Infrastructure may belong to multiple Zones. | Infrastructure Management | Existing Zone membership does not prevent another valid membership. | L2-G10 §8.5; ZCP-02 |
| Zone Type must be configured and Zone Name must satisfy the registration rule. | Infrastructure Management | Zone creation is validated before establishment. | L2-G10 Zone Registration Workflow |
| Every membership change publishes a canonical event. | Infrastructure Management | `AssignedToZone` is required for successful linkage. | L2-G10 Zone Membership Workflow; ZCPWP-06 |

---

# 13. State and Lifecycle Impact

The Zone lifecycle is inconsistent across locked G10 sections. The Zone-specific lifecycle states `Proposed → Operational → Archived`, while the generic registration workflow states that newly registered infrastructure begins in `Draft`, and the general canonical lifecycle contains a larger state set.

No initial Zone lifecycle state is selected locally.

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Zone | No canonical Zone exists | Establishes the Zone and its memberships | Exact initial Zone lifecycle position unresolved upstream | Infrastructure Management | L2-G10 §§8.5, 16; Zone Lifecycle and Registration Workflow |
| Zone Membership | Zone and referenced infrastructure exist | Adds valid logical membership | Membership established; physical hierarchy unchanged | Infrastructure Management | L2-G10 Zone Membership Workflow |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AssignedToZone` | Infrastructure Management | Authorized G10 consumers | Confirms successful Zone Membership. | L2-G10 Zone Membership Workflow; §175 |

The exact Zone-registration event is unresolved:

- `ZoneRegistered` in §44.
- `ZoneCreated` in the Zone Registration Workflow and §175.

L3 does not select between them.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain hand-off is required for this G-only scenario. Later consumers may query infrastructure by Zone under G10's published read capabilities.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Establish Zone and manage membership | Infrastructure Administrator | Must act within source-defined relationship-management authority. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override requirement is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Zone creation and membership changes are auditable. | Source-defined evidence of the Zone establishment and infrastructure memberships. | Infrastructure Management |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical Zone exists under G10 ownership.
- Applicable infrastructure is linked through Zone Membership without altering physical hierarchy.
- The initial Zone lifecycle position is consistently defined upstream.
- `AssignedToZone` is published for membership establishment.
- The exact canonical Zone-registration event is resolved and published by Infrastructure Management.

## 16.2 Postconditions

- Infrastructure may retain other Zone memberships.
- Physical ownership and physical hierarchy remain unchanged.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 names Zone registration `ZoneRegistered` in §44 but `ZoneCreated` in the Zone workflow and event summary. | A mandatory registration event has no single exact canonical name. | Architecture governance must resolve the event name while retaining G10 as publisher. |
| L2-G10 gives incompatible initial Zone lifecycle positions (`Draft` under generic registration versus Zone-specific `Proposed → Operational → Archived`). | L3 cannot state the newly established Zone's lifecycle position consistently. | Align the locked Zone registration and lifecycle definitions. |

**Source Assessment:** **BLOCKED**

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
