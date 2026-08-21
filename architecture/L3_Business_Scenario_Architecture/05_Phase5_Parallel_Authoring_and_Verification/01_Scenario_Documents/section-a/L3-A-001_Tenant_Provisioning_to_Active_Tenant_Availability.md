# Tenant Provisioning to Active Tenant Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Platform & Tenant  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md` L1-01; `L2-A01_Platform_Tenant_Foundation_Architecture.md` Sections 2.8 and 5.4–5.13; `L2-A06_Event_Infrastructure_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Establish a secure, isolated and governed Tenant environment for a customer organization before institutional or business-domain operations begin.

The scenario ensures that the Platform-owned Tenant boundary progresses through its canonical provisioning lifecycle and becomes operationally available without transferring Tenant ownership to any downstream domain.

---

# 3. Business Outcome

The Tenant is provisioned, initialized, activated and available for institution creation and downstream setup.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved Tenant exists in the `Requested` state and the Platform commences the permitted transition to `Provisioning`.

## 4.2 End Boundary

The scenario ends when provisioning is successfully completed, the Tenant reaches `Active`, and the Tenant becomes available for downstream business execution.

## 4.3 In Scope

- Validation of the requested Tenant lifecycle progression.
- Transition from `Requested` to `Provisioning`.
- Tenant registration and identity initialization.
- Platform and configuration initialization.
- Module, context, shared-service and governance initialization.
- Transition from `Provisioning` to `Active`.
- Governed handling of unsuccessful provisioning.

## 4.4 Out of Scope

- Institution creation, which begins only after the Tenant becomes `Active`.
- Organisation, academic or other business-domain setup.
- Tenant maintenance, suspension, upgrade and restoration.
- Tenant archival.
- Technical provisioning mechanisms or infrastructure design.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-A-002 — Active Tenant Governance Concern to Controlled Restriction and Restored Availability | Governs later restriction and restored availability of an Active Tenant. |
| Related | L3-A-003 — Active Tenant to Governed Archival | Governs the later terminal archival outcome. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform & Tenant | Owns Tenant provisioning, lifecycle validation, initialization, activation and the final Active Tenant outcome. |

All participating responsibilities remain within Section A. No downstream business domain owns or modifies the Tenant lifecycle.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Platform Administrator | Participates in the Platform-owned Tenant provisioning workflow. | Primary actor identified by locked L1; no additional approval or lifecycle-transition authority is inferred. |

L2-A01 does not define a canonical business actor for approving Tenant creation or authorizing provisioning initiation. The scenario therefore assigns lifecycle behaviour to the owning domain without inventing an authority.

---

# 7. Preconditions and Entry Criteria

- The Tenant registration has been accepted and approved for creation.
- The Tenant is in the canonical `Requested` state.
- The Tenant identity exists.
- The Tenant Registration Request, Subscription Plan and Initial Configuration are available.
- Institution creation and downstream business execution have not begun for the Tenant.

---

# 8. Trigger

An approved Tenant request is accepted for provisioning.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---:|---|---|---|
| 1 | Validate that the Tenant identity exists, the requested transition is valid, Platform integrity is preserved, required Platform Services are available, lifecycle policies are satisfied and governance requirements are met. | Platform & Tenant | The Tenant is eligible to enter provisioning. |
| 2 | Perform the canonical `Requested` to `Provisioning` transition. | Platform & Tenant | The Tenant enters `Provisioning` and remains unavailable for business operations. |
| 3 | Complete Tenant registration, Tenant identity creation, Platform initialization, configuration initialization, module registration, context initialization, shared-service initialization and governance registration. | Platform & Tenant | The Platform-owned Tenant environment is initialized. |
| 4 | Validate that the permitted `Provisioning` to `Active` transition can occur. | Platform & Tenant | The Tenant satisfies the conditions for activation. |
| 5 | Perform the `Provisioning` to `Active` transition. | Platform & Tenant | The Tenant becomes fully operational and available for permitted downstream execution. |

The Main Business Flow ends when the Tenant reaches `Active`.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Provisioning terminates unsuccessfully. | The Tenant cannot become operational and enters `Provisioning Failed`. | After the cause of failure is remediated, the Tenant may perform the permitted `Provisioning Failed` to `Provisioning` transition. | Platform & Tenant |
| A requested lifecycle transition is invalid. | The lifecycle progression is rejected and the Tenant does not advance. | Progression may continue only through a transition permitted by the canonical Tenant lifecycle. | Platform & Tenant |
| A required lifecycle validation condition is not satisfied. | Provisioning or activation cannot validly proceed. | The Platform preserves the current lifecycle position until the authoritative validation conditions are satisfied. | Platform & Tenant |

Recovery remains business-level. No technical retry, rollback or infrastructure-recovery behaviour is defined here.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Tenant shall be uniquely identifiable. | Platform & Tenant | Tenant provisioning cannot proceed without a distinct Tenant identity. | L2-A01 §2.14 |
| Tenant provisioning shall be performed only by the Platform. | Platform & Tenant | No downstream domain may provision or activate a Tenant. | L2-A01 §2.14 |
| Tenant isolation shall be preserved across all Platform capabilities. | Platform & Tenant | All initialization remains within the single Tenant boundary. | L2-A01 §§2.6 and 2.14 |
| Every Tenant shall begin in the `Requested` state. | Platform & Tenant | `Requested` is the required lifecycle entry position. | L2-A01 §5.13 |
| Only the Platform may perform Tenant lifecycle transitions. | Platform & Tenant | Lifecycle authority remains with the canonical owner. | L2-A01 §5.13 |
| Any lifecycle transition not explicitly permitted is invalid. | Platform & Tenant | The scenario may use only `Requested` to `Provisioning`, `Provisioning` to `Active`, `Provisioning` to `Provisioning Failed`, and `Provisioning Failed` to `Provisioning`. | L2-A01 §5.8 |
| Institution creation requires an `Active` Tenant. | Platform & Tenant | Institution creation remains outside the scenario until successful completion. | L2-A01 §§5.7 and 5.13 |
| Tenant lifecycle transitions shall be auditable. | Platform & Tenant | The provisioning and activation lifecycle progression must remain auditable. | L2-A01 §5.13 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Tenant | `Requested` | Normal provisioning progression | `Provisioning`, then `Active` | Platform & Tenant | L2-A01 §§5.4, 5.5 and 5.8 |
| Tenant | `Provisioning` | Provisioning terminates unsuccessfully | `Provisioning Failed` | Platform & Tenant | L2-A01 §§5.5 and 5.8 |
| Tenant | `Provisioning Failed` | The cause of failure is remediated and provisioning resumes | `Provisioning` | Platform & Tenant | L2-A01 §§5.4 and 5.8 |

The Tenant lifecycle remains exclusively Platform-owned. Business domains may consume the resulting lifecycle state but cannot modify it.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `TenantCreated` | Platform Domain | Not specified in the authoritative sources reviewed | Communicates the canonical fact that the Tenant has been created during provisioning. | Locked L1, L1-01 — Multi-Domain Contracts |
| `TenantActivated` | Platform Domain | Not specified in the authoritative sources reviewed | Communicates the canonical fact that the Tenant has reached operational activation. | Locked L1, L1-01 — Multi-Domain Contracts |

The event names and publisher are preserved from locked L1. No consumer or event-contract detail is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain contract or governed hand-off is defined within this scenario.

The resulting `Active` Tenant lifecycle fact becomes an upstream dependency for institution creation and downstream business execution. Consumption of that fact does not transfer Tenant ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Perform Tenant lifecycle transitions within this scenario | Canonical business role not defined in L2-A01 | Only the Platform may perform lifecycle transitions, and only permitted transitions are valid. | Platform & Tenant |

The scenario begins after the Tenant has been approved for creation. The authoritative sources do not define the actor or role responsible for that approval.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception requirement is defined by the authoritative L1/L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Tenant lifecycle transitions must remain auditable. | Auditable Tenant lifecycle history covering the transitions performed by this scenario. | Platform & Tenant |
| Tenant audit history must be preserved throughout the lifecycle. | Preserved Tenant lifecycle audit history. | Platform & Tenant |

No evidence fields or technical audit mechanism are defined at L3.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Tenant has successfully transitioned from `Requested` through `Provisioning` to `Active`.
- Tenant identity, Platform configuration, module registry, context framework, shared Platform Services and governance registration have been initialized.
- The Tenant is fully operational.
- The Tenant is available for permitted downstream business execution.
- Institution creation is permitted to begin as a separate downstream outcome.

## 16.2 Postconditions

- The Tenant remains the isolated Platform, security, configuration and execution boundary.
- Downstream domains may execute only within the resolved Active Tenant context.
- Institution creation may begin but has not been performed by this scenario.
- The Tenant lifecycle remains owned exclusively by Platform & Tenant and read-only to downstream business domains.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-A01 does not define the business actor responsible for approving Tenant creation or authorizing provisioning initiation. Locked L1 identifies `Platform Administrator` as a primary actor but does not explicitly assign this authority. | The Draft can preserve Platform ownership, but canonical business authority cannot be finalized for approval or later access-control specification. | Platform & Tenant architecture ownership must define or confirm the canonical business role and authority, or confirm that approval belongs to a separate upstream scenario. |
| Locked L1 defines `TenantCreated` and `TenantActivated` as Platform-published events, but the applicable locked L2 sources do not provide their canonical event definitions or identify their consumers. | Exact event names and publisher are preserved, but event-registry traceability and consumer identification remain incomplete. | Platform & Tenant and Event Infrastructure architecture ownership must confirm the canonical event definitions and interested consumers without changing the publisher. |

These issues are non-blocking for this Draft but must be resolved before the scenario can become `Locked`.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.