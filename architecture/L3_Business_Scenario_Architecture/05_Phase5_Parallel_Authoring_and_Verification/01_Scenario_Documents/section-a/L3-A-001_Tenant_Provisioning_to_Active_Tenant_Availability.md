# Tenant Provisioning to Active Tenant Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Platform & Tenant  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md` L1-01; `L2-A01_Platform_Tenant_Foundation_Architecture.md`; `L2-A01_Tenant_Provisioning_Authority_and_Event_Amendment_2026-08-26.md`; `L2-A06_Event_Infrastructure_Workflows.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** Institution establishment and downstream Tenant-scoped architecture

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

The scenario begins when the Platform Administrator authorizes an eligible Tenant Registration Request to enter the canonical provisioning path and the Tenant is in the `Requested` state.

## 4.2 End Boundary

The scenario ends when provisioning is successfully completed, the Tenant reaches `Active`, and the Active Tenant context becomes available for permitted downstream setup.

## 4.3 In Scope

- Platform Administrator authorization for an eligible Tenant request to enter provisioning.
- Validation of the requested Tenant lifecycle progression.
- Transition from `Requested` to `Provisioning`.
- Tenant registration and identity initialization.
- Platform and configuration initialization.
- Module, context, shared-service and governance initialization.
- Transition from `Provisioning` to `Active`.
- Governed handling of unsuccessful provisioning.
- Publication of canonical Tenant lifecycle events applicable to this scenario.

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
| Platform & Tenant | Owns Tenant provisioning, lifecycle validation, initialization, activation, Tenant event semantics and the final Active Tenant outcome. |

All participating responsibilities remain within Section A. No downstream business domain owns or modifies the Tenant lifecycle.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Platform Administrator | Reviews the Tenant Registration Request and authorizes an eligible request to enter the governed provisioning path. | Canonical business authority for accepting an eligible Tenant request into `Requested → Provisioning`; does not itself own the Tenant lifecycle or downstream Institution lifecycle. |
| Platform & Tenant Domain | Performs canonical Tenant lifecycle validation and owns progression through the Tenant lifecycle. | Canonical owner of Tenant identity, lifecycle, provisioning, activation and Tenant business facts/events. |

No separate second human approval is introduced for `Provisioning → Active`; activation remains governed by the existing Tenant lifecycle and validation conditions.

---

# 7. Preconditions and Entry Criteria

- The Tenant Registration Request, Subscription Plan and Initial Configuration are available.
- The request satisfies the governing Platform/Tenant eligibility conditions for provisioning consideration.
- The Tenant is in the canonical `Requested` state before lifecycle progression begins.
- Institution creation and downstream business execution have not begun for the Tenant.

---

# 8. Trigger

The Platform Administrator authorizes an eligible Tenant Registration Request to enter the provisioning path.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---:|---|---|---|
| 1 | Review the Tenant Registration Request against the governed Platform/Tenant eligibility basis and authorize eligible progression into provisioning. | Platform Administrator | The request is authorized to proceed into the canonical Tenant provisioning path. |
| 2 | Validate that Tenant identity and lifecycle prerequisites are satisfied and that the requested transition is permitted. | Platform & Tenant | The Tenant is eligible to enter `Provisioning`. |
| 3 | Perform the canonical `Requested` to `Provisioning` transition. | Platform & Tenant | The Tenant enters `Provisioning` and remains unavailable for downstream business operations. |
| 4 | Establish and initialize the canonical Tenant environment, including governed Platform configuration, module/context and shared Platform capabilities required by the Tenant boundary. | Platform & Tenant | The Platform-owned Tenant environment is initialized; `TenantCreated` is published when the canonical Tenant creation fact is established under L2-A01. |
| 5 | Validate that the permitted `Provisioning` to `Active` transition can occur under the canonical Tenant lifecycle. | Platform & Tenant | The Tenant satisfies activation conditions. |
| 6 | Perform the `Provisioning` to `Active` transition. | Platform & Tenant | The Tenant becomes operational; `TenantActivated` is published and the Active Tenant context becomes available for permitted downstream setup. |

The Main Business Flow ends when the Tenant reaches `Active`.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The Tenant Registration Request is not eligible or is not authorized to enter provisioning. | The request does not enter the Tenant provisioning lifecycle. | The request remains outside this scenario until a later governed request satisfies the required eligibility and authorization conditions. | Platform Administrator / Platform & Tenant |
| Provisioning terminates unsuccessfully. | The Tenant cannot become operational and enters `Provisioning Failed`. | After the cause of failure is remediated, the Tenant may perform the permitted `Provisioning Failed` to `Provisioning` transition. | Platform & Tenant |
| A requested lifecycle transition is invalid. | The lifecycle progression is rejected and the Tenant does not advance. | Progression may continue only through a transition permitted by the canonical Tenant lifecycle. | Platform & Tenant |
| A required lifecycle validation condition is not satisfied. | Provisioning or activation cannot validly proceed. | The Platform preserves the current lifecycle position until the authoritative validation conditions are satisfied. | Platform & Tenant |

Recovery remains business-level. No technical retry, rollback or infrastructure-recovery behaviour is defined here.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Platform Administrator is the canonical business authority for accepting an eligible Tenant Registration Request into provisioning. | Platform & Tenant | `Requested → Provisioning` may begin only after the governed authorization decision. | L2-A01 locked amendment §1 |
| Authorization to commence provisioning does not itself activate the Tenant. | Platform & Tenant | The Tenant must still satisfy the canonical provisioning and activation lifecycle conditions. | L2-A01 locked amendment §1.1 |
| Every Tenant shall be uniquely identifiable. | Platform & Tenant | Tenant provisioning cannot proceed without a distinct Tenant identity. | L2-A01 §2.14 |
| Tenant provisioning shall be performed only by the Platform. | Platform & Tenant | No downstream domain may provision or activate a Tenant. | L2-A01 §2.14 |
| Tenant isolation shall be preserved across all Platform capabilities. | Platform & Tenant | All initialization remains within the single Tenant boundary. | L2-A01 §§2.6 and 2.14 |
| Every Tenant shall begin in the `Requested` state. | Platform & Tenant | `Requested` is the required lifecycle entry position. | L2-A01 §5.13 |
| Only the Platform may perform Tenant lifecycle transitions. | Platform & Tenant | Lifecycle ownership remains with the canonical owner. | L2-A01 §5.13 |
| Any lifecycle transition not explicitly permitted is invalid. | Platform & Tenant | The scenario uses only transitions supported by the canonical Tenant lifecycle. | L2-A01 §5.8 |
| Institution creation requires an `Active` Tenant. | Platform & Tenant | Institution creation remains outside the scenario until successful completion. | L2-A01; L2-B01 |
| Tenant lifecycle transitions shall be auditable. | Platform & Tenant | The authorization, provisioning and activation progression must remain auditable. | L2-A01; L2-A01 locked amendment §4 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Tenant | `Requested` | Normal provisioning progression after authorized entry | `Provisioning`, then `Active` | Platform & Tenant | L2-A01; L2-A01 locked amendment |
| Tenant | `Provisioning` | Provisioning terminates unsuccessfully | `Provisioning Failed` | Platform & Tenant | L2-A01 §§5.5 and 5.8 |
| Tenant | `Provisioning Failed` | The cause of failure is remediated and provisioning resumes | `Provisioning` | Platform & Tenant | L2-A01 §§5.4 and 5.8 |

The Tenant lifecycle remains exclusively Platform-owned. Business domains may consume the resulting lifecycle state but cannot modify it.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `TenantCreated` | Platform & Tenant / L2-A01 | No mandatory cross-domain subscriber is established by current authoritative architecture. | Communicates that the canonical Tenant has been established with its Tenant identity and has entered the governed Tenant lifecycle. | L1 L1-01; L2-A01 locked amendment §2 |
| `TenantActivated` | Platform & Tenant / L2-A01 | Section B — Institution & Structure is materially interested because L2-B01 requires Active Tenant context; other downstream consumers may consume the fact where authoritatively required. | Communicates that provisioning has completed and the Tenant has reached `Active`. | L1 L1-01; L2-A01 locked amendment §2 |

L2-A06 Event Infrastructure may distribute these events but does not own or republish Tenant business truth.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Platform & Tenant / L2-A01 | Institution & Structure / L2-B01 | Active canonical Tenant context | Establishes the upstream Tenant boundary within which Institution creation/configuration may begin. | L2-A01 locked amendment §3; L2-B01 |

The hand-off preserves Section B's foundational dependency on Section A. It does not transfer Tenant ownership or define an implementation mechanism.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Accept an eligible Tenant Registration Request into the provisioning path | Platform Administrator | May authorize commencement of `Requested → Provisioning` only when governing Platform/Tenant eligibility conditions are satisfied. | Platform & Tenant |
| Perform Tenant lifecycle transitions within the scenario | Platform & Tenant Domain | Only the canonical Tenant owner may perform lifecycle transitions, and only permitted transitions are valid. | Platform & Tenant |

No separate second human approval for `Provisioning → Active` is established by the authoritative architecture.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception requirement is defined by the authoritative L1/L2 architecture beyond the canonical authority and ownership boundaries above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Provisioning-entry authorization must remain auditable. | Preserved business evidence that the Platform Administrator authorized the eligible Tenant request to enter provisioning. | Platform & Tenant |
| Tenant lifecycle transitions must remain auditable. | Auditable Tenant lifecycle history covering the transitions performed by this scenario. | Platform & Tenant |
| Tenant lifecycle events must remain traceable. | Preserved record of applicable `TenantCreated` and `TenantActivated` publication as governed Tenant business facts. | Platform & Tenant |

L2-A07 retains ownership of canonical audit infrastructure and immutable audit evidence. No evidence fields or technical audit mechanism are defined at L3.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Platform Administrator authorized the eligible Tenant request to enter provisioning.
- The Tenant successfully transitioned from `Requested` through `Provisioning` to `Active`.
- The canonical Tenant environment and required Platform context were initialized.
- `TenantCreated` and `TenantActivated` were published at their applicable governed lifecycle points.
- The Active Tenant context is available for permitted downstream setup.

## 16.2 Postconditions

- The Tenant remains the isolated Platform, security, configuration and execution boundary.
- Section B may begin Institution establishment within the Active Tenant context as a separate downstream business outcome.
- Other downstream domains may consume Active Tenant context only according to their authoritative architecture.
- The Tenant lifecycle and Tenant event semantics remain owned exclusively by Platform & Tenant.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.