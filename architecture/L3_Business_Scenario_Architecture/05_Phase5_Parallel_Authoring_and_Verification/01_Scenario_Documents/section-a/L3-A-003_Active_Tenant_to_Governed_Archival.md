# Active Tenant to Governed Archival

# 1. Scenario Metadata

**Scenario ID:** L3-A-003  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Platform & Tenant  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md` L1-01; `L2-A01_Platform_Tenant_Foundation_Architecture.md` Sections 5.2 and 5.4–5.14; `DEPENDENCY_MAP.md`  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Govern the permanent operational retirement of an Active Tenant while preserving its required historical information and preventing further normal business execution.

The scenario ensures that Tenant retirement occurs only through the canonical Platform-owned lifecycle and does not become deletion, uncontrolled loss of history or a transfer of Tenant ownership to another domain.

---

# 3. Business Outcome

The Tenant is archived, becomes read-only and unavailable for normal business execution, and its required historical information is preserved.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an `Active` Tenant has been approved for permanent operational retirement.

## 4.2 End Boundary

The scenario ends when the Tenant reaches the terminal `Archived` state, becomes read-only and unavailable for normal business execution, and its required historical information has been preserved.

## 4.3 In Scope

- Validation of the approved permanent operational-retirement decision.
- Validation of the `Active` to `Archived` lifecycle transition.
- Transition of the Tenant to `Archived`.
- Termination of normal business execution within the Tenant.
- Establishment of read-only historical availability.
- Preservation of required historical information and audit history.
- Enforcement of the terminal and immutable nature of the archived Tenant.

## 4.4 Out of Scope

- Temporary Tenant suspension and restoration.
- Controlled maintenance or Platform upgrade.
- Physical deletion of the Tenant.
- Destruction of Tenant history.
- Institution or business-domain archival lifecycles.
- Definition of retention periods or institutional retention policies.
- Technical archival storage or restoration mechanisms.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-A-001 — Tenant Provisioning to Active Tenant Availability | Establishes the Active Tenant required by this scenario. |
| Related | L3-A-002 — Active Tenant Governance Concern to Controlled Restriction and Restored Availability | Governs temporary restriction and restoration rather than permanent retirement. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform & Tenant | Owns the Tenant lifecycle, retirement transition, archival outcome, historical preservation and terminal lifecycle position. |

All participating responsibilities remain within Section A.

Institutional and business domains may depend on the Tenant lifecycle state but do not own, perform or redefine Tenant archival.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Platform Administrator | Participates in Platform-owned Tenant lifecycle administration. | Primary Platform actor identified by locked L1; specific authority to approve permanent retirement is not defined upstream. |

The scenario does not infer an approval authority that is absent from the authoritative architecture.

---

# 7. Preconditions and Entry Criteria

- The Tenant exists and is in the canonical `Active` state.
- The Tenant identity remains valid and uniquely identifiable.
- The Tenant has been approved for permanent operational retirement.
- The Tenant has not already entered `Archived`.
- Required Tenant historical information and audit history remain available for preservation.

---

# 8. Trigger

An `Active` Tenant is approved for permanent operational retirement.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---:|---|---|---|
| 1 | Receive the approved permanent operational-retirement decision for the Active Tenant. | Platform & Tenant | The Tenant enters governed retirement processing. |
| 2 | Validate that the Tenant identity exists, the `Active` to `Archived` transition is valid, Platform integrity is preserved, lifecycle policies are satisfied and governance requirements are met. | Platform & Tenant | The Tenant is eligible for archival. |
| 3 | Confirm that the required Tenant historical information and audit history remain preservable under applicable governance and retention obligations. | Platform & Tenant | Historical-preservation obligations remain supportable. |
| 4 | Perform the canonical `Active` to `Archived` lifecycle transition. | Platform & Tenant | The Tenant reaches its terminal operational state. |
| 5 | Make the archived Tenant read-only and unavailable for normal business execution. | Platform & Tenant | Further normal Tenant business activity is prevented. |
| 6 | Preserve the required Tenant historical information and audit history and retain governed historical availability according to applicable retention policies. | Platform & Tenant | Governed Tenant archival is complete. |

The Main Business Flow ends when the terminal archival outcome and required historical preservation are established.

---

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

Temporary restriction through `Suspended` is governed by L3-A-002 and is not an alternate path to this scenario’s permanent archival outcome.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Permanent operational retirement has not been approved. | The scenario cannot validly begin. | Preserve the Tenant in `Active` until an authoritative retirement approval exists. | Platform & Tenant |
| The requested `Active` to `Archived` transition is invalid. | The Tenant cannot enter the terminal archival state. | Reject the invalid transition and preserve the existing Tenant lifecycle state. | Platform & Tenant |
| Applicable lifecycle or governance validation conditions are not satisfied. | Archival cannot validly proceed. | Preserve the Tenant in `Active` until the authoritative conditions are satisfied. | Platform & Tenant |
| Required historical information cannot be preserved as required. | The archival outcome cannot be considered complete. | Do not declare successful completion until the authoritative preservation obligations can be satisfied. | Platform & Tenant |

No recovery transition from `Archived` is defined. `Archived` is the terminal Tenant lifecycle state.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| The Platform exclusively owns the lifecycle of every Tenant. | Platform & Tenant | No downstream domain may archive or otherwise modify a Tenant lifecycle. | L2-A01 §5.2 |
| Only the Platform may perform Tenant lifecycle transitions. | Platform & Tenant | The archival transition remains Platform-owned. | L2-A01 §5.13 |
| `Active` to `Archived` is the permitted Tenant archival transition. | Platform & Tenant | The scenario cannot introduce an alternative archival transition. | L2-A01 §5.8 |
| Any transition not listed in the canonical Tenant lifecycle is invalid. | Platform & Tenant | No recovery or reactivation transition from `Archived` may be inferred. | L2-A01 §5.8 |
| `Archived` is the terminal Tenant lifecycle state. | Platform & Tenant | The Tenant cannot return to normal operational availability after archival. | L2-A01 §§5.5, 5.7 and 5.14 |
| Archived Tenants shall remain immutable. | Platform & Tenant | The archived Tenant cannot be changed through normal business execution. | L2-A01 §5.13 |
| Archived Tenants become read-only. | Platform & Tenant | Historical access cannot permit normal modification. | L2-A01 §5.7 |
| Archived Tenants cannot participate in normal business execution. | Platform & Tenant | Normal Tenant operations end after archival. | L2-A01 §5.7 |
| Tenant deletion is prohibited through normal Platform operations. | Platform & Tenant | Archival must not be implemented as Tenant deletion. | L2-A01 §5.13 |
| Archived Tenants remain historically accessible according to retention policies. | Platform & Tenant | Required historical availability continues under applicable retention governance. | L2-A01 §5.7 |
| Tenant lifecycle transitions shall be auditable. | Platform & Tenant | The archival transition must remain traceable. | L2-A01 §5.13 |
| Business domains shall treat the Tenant lifecycle as read-only. | Platform & Tenant | Downstream domains cannot change or reverse the archival outcome. | L2-A01 §5.13 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Tenant | `Active` | Governed permanent operational retirement | `Archived` | Platform & Tenant | L2-A01 §§5.4, 5.5 and 5.8 |
| Tenant | `Archived` | Historical preservation with normal business execution prohibited | Remains `Archived` as the terminal immutable lifecycle state | Platform & Tenant | L2-A01 §§5.7, 5.13 and 5.14 |

No transition from `Archived` to another Tenant lifecycle state is defined.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No exact canonical business event for Tenant archival is defined by the authoritative L1/L2 sources reviewed.

This scenario does not invent events such as `TenantArchived`, `TenantRetired` or `TenantDeactivated`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

No scenario-specific synchronous business contract or governed hand-off is defined by the authoritative L2 architecture.

Downstream domains consume the Tenant lifecycle state as a read-only foundational dependency. The `Archived` state establishes that normal business execution is no longer permitted.

This dependency does not transfer Tenant lifecycle ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Approve permanent operational retirement of an Active Tenant | Canonical business role not defined upstream | An approved permanent-retirement decision is required before the scenario begins. | Platform & Tenant |
| Perform the `Active` to `Archived` transition | Canonical business role not defined in L2-A01 | Only the Platform may perform a permitted Tenant lifecycle transition. | Platform & Tenant |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception requirement is defined by the authoritative L1/L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Permanent Tenant retirement must be approved before archival begins. | Approved permanent operational-retirement decision. | Platform & Tenant |
| The Tenant archival transition must remain auditable. | Auditable Tenant lifecycle history covering the `Active` to `Archived` transition. | Platform & Tenant |
| Tenant audit history must be preserved after archival. | Preserved Tenant audit history within the governed historical Tenant record. | Platform & Tenant |
| Required historical information must remain available according to applicable retention policies. | Preserved required Tenant historical information. | Platform & Tenant |

No evidence fields or technical archival mechanism are defined at L3.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A valid approved permanent operational-retirement decision exists.
- The permitted `Active` to `Archived` transition has been validated and completed.
- The Tenant is in the canonical `Archived` state.
- The Tenant is read-only.
- The Tenant is unavailable for normal business execution.
- Required Tenant historical information and audit history remain preserved.
- Historical availability remains governed by applicable retention policies.
- The archival lifecycle transition remains auditable.

## 16.2 Postconditions

- The Tenant remains permanently in the terminal `Archived` state.
- Normal business execution cannot resume within the archived Tenant.
- The archived Tenant remains immutable and read-only.
- Tenant deletion has not occurred.
- Required historical information and audit history remain available according to applicable retention governance.
- Downstream domains cannot modify or reverse the Tenant lifecycle outcome.
- Tenant lifecycle ownership remains exclusively with Platform & Tenant.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The authoritative sources require approval for permanent Tenant retirement but do not define the canonical business actor or role authorized to grant that approval. | The owner, trigger, lifecycle transition, terminal outcome and preservation obligations are clear, so the Draft is usable. Actor-level approval authority cannot be finalized for later approval and access-control specification. | Platform & Tenant architecture ownership must define or confirm the canonical business role and authority for permanent Tenant-retirement approval. |

This issue is non-blocking for this Draft but must be resolved before the scenario can become `Locked`.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.