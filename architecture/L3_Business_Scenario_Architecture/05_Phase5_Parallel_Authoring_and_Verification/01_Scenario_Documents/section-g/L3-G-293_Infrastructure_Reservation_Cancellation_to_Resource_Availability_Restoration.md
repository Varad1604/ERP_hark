# Infrastructure Reservation Cancellation to Resource Availability Restoration

# 1. Scenario Metadata

**Scenario ID:** L3-G-293  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Authorized consumers of Infrastructure reservation and availability facts

# 2. Scenario Purpose

Govern the cancellation of an existing Infrastructure Reservation so that the reserved physical resource is released in accordance with Infrastructure Management policy, its resulting availability is determined by the authoritative Availability Profile, and reservation history remains preserved.

# 3. Business Outcome

The reservation is closed and the applicable resource becomes available according to its availability policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised reservation is cancelled.

## 4.2 End Boundary

The scenario ends when the reservation cancellation has released the applicable infrastructure commitment and the resource's current availability has been restored or otherwise determined according to its authoritative availability policy, without deleting reservation history.

## 4.3 In Scope

- Cancellation of an existing Infrastructure Reservation under G10 reservation governance.
- Release of the physical resource from the cancelled reservation commitment.
- Re-evaluation of resource availability according to the authoritative Availability Profile and applicable policy.
- Preservation of reservation history.
- Publication of the canonical completed cancellation fact.
- Business audit evidence generated from the completed business fact.

## 4.4 Out of Scope

- Creation or modification of a Reservation before cancellation.
- Redefinition of infrastructure capacity, lifecycle, classification, or availability policy.
- Any consuming domain's downstream scheduling or operational decision after the resource availability fact changes.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-G-291 — Infrastructure Reservation Request to Controlled Reservation | Establishes a Reservation for the applicable resource and period. |
| Related | L3-G-292 — Infrastructure Reservation Change to Controlled Rescheduling | Governs permitted changes to an existing Reservation without cancelling it. |
| Related | L3-G-289 — Infrastructure Availability Definition to Availability Profile Activation | Establishes the authoritative availability policy used when release is evaluated. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the Reservation, reservation cancellation, physical-resource release and resulting Infrastructure Availability fact. |
| Platform Foundation / Section A | Owns authorization decisions, event-governance capability and immutable audit evidence used by the scenario; it does not own the Reservation or availability outcome. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Administers Infrastructure reservation governance within authorized infrastructure scope. | May perform governed Infrastructure management actions where authorization permits. |
| Campus Administrator | Administers Infrastructure within the governed campus scope. | May participate where the Reservation concerns infrastructure within that campus scope. |
| Auditor | Reviews preserved business evidence. | Read-only review of historical evidence; does not alter Reservation or availability facts. |

L2-G10 defines these logical roles but does not designate one unique canonical role as the initiator of every reservation cancellation. No additional actor is invented by this scenario.

# 7. Preconditions and Entry Criteria

- The Reservation exists under Infrastructure Management ownership.
- The Reservation identifies an Infrastructure resource governed by G10.
- The cancellation action is authorized before the business change proceeds.
- An authoritative availability policy or Availability Profile exists for determining the resource's usable state after release.

# 8. Trigger

An authorised reservation is cancelled

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the existing Reservation and the Infrastructure resource committed by it. | Infrastructure Management | The exact reservation commitment to be cancelled is established. |
| 2 | Confirm that the cancellation is authorized and governed by the applicable reservation policy. | Infrastructure Management, using the canonical authorization decision owned by Section A | The cancellation is eligible to proceed. |
| 3 | Cancel the reservation commitment without deleting its historical record. | Infrastructure Management | The resource is released from that Reservation while reservation history remains preserved. |
| 4 | Determine the resource's current availability under its authoritative Availability Profile and applicable policy. | Infrastructure Management | The resource becomes available where policy permits, or retains another source-defined availability condition where a separate restriction still applies. |

The Main Business Flow ends when the frozen Business Outcome has been established. Event publication and audit evidence are addressed separately in Sections 14 and 15.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Availability remains constrained | The cancelled Reservation is removed as a cause of reservation, but another authoritative availability condition still restricts the resource. | Release the Reservation and determine the resource's current availability from the remaining applicable Availability Profile and policy conditions. | The Reservation is cancelled without falsely declaring the resource usable contrary to its authoritative availability policy. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Authorization does not permit cancellation | The Reservation remains unchanged. | The cancellation does not proceed unless a later authorized business action satisfies the governing policy. | Platform Foundation / Authorization; Infrastructure Management retains the Reservation. |
| The Reservation or referenced Infrastructure resource cannot be validly established | The intended business cancellation cannot be applied to an authoritative Reservation/resource pair. | Resolve the authoritative business reference before a new cancellation attempt; no cancellation outcome is asserted meanwhile. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns reservation governance; operational domains consume reservation capabilities without taking ownership. | Infrastructure Management | Cancellation and resource release remain G10-owned business facts. | L2-G10 §26 Infrastructure Reservation Workflows |
| Reservation cancellation shall release Infrastructure, restore availability, publish a domain event, preserve audit history, and shall never delete the Reservation. | Infrastructure Management | The scenario cannot treat cancellation as destructive deletion and must preserve historical traceability. | L2-G10 §26 Reservation Cancellation |
| Infrastructure availability is a governed operational state distinct from the Infrastructure lifecycle. | Infrastructure Management | Releasing a Reservation does not bypass other availability restrictions or create a new lifecycle state. | L2-G10 Availability governance; §11A lifecycle definitions |
| Every business operation must be authorized before execution. | Platform Foundation — Authorization | An unauthorized cancellation cannot change the Reservation. | L2-A04 AP-01 and business-domain boundary |
| Events represent completed business facts and the originating business domain retains ownership of the fact. | Platform Foundation — Event Infrastructure; Infrastructure Management | `ReservationCancelled` describes the completed G10 cancellation; Event Infrastructure does not become its business owner. | L2-A06 EIP-01, EIP-02; L2-G10 §44 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reservation | Requested → Confirmed → Active → Completed → Archived are the canonical states defined by L2-G10; the exact cancellable entry position is not separately specified. | The frozen scenario requires business cancellation/closure and preservation of history. | **Unresolved:** L2-G10 does not define a `Cancelled` or `Closed` Reservation lifecycle state, so this Draft does not invent one. | Infrastructure Management | L2-G10 §11A canonical lifecycle; §26 Reservation Cancellation |
| Infrastructure Availability | Source-defined availability states include Available, Reserved, Under Maintenance, Under Renovation, Temporarily Closed, Emergency Closed and Operationally Restricted. | The Reservation is released as a reservation constraint and availability is determined under the applicable policy. | Existing source-defined availability state appropriate to the remaining business conditions; `Available` only where policy permits. | Infrastructure Management | L2-G10 Availability States; §26 Reservation Cancellation |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `ReservationCancelled` | Infrastructure Management | No specific consumer is named by L2-G10 for this event. | Publishes the completed fact that the Infrastructure Reservation has been cancelled. | L2-G10 §44 Reservation Events; L2-A06 EIP-02 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain business hand-off is required to transfer ownership of the cancellation outcome. Section A participates through its canonical authorization, event-governance and audit capabilities; those capabilities do not transfer ownership of the Reservation or Infrastructure Availability facts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Cancel an Infrastructure Reservation | Infrastructure Administrator or Campus Administrator within the applicable governed scope | The action proceeds only when the canonical authorization decision permits it and G10 reservation policy is satisfied. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, approval, override, or exception requirement for ordinary Reservation cancellation is defined by the authoritative L2 architecture.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve cancellation history | Evidence that the Reservation was cancelled rather than deleted and that the affected resource was released under G10 governance. | Infrastructure Management for the business fact; Platform Audit Infrastructure for immutable audit evidence. |
| Preserve availability determination | Evidence of the resulting authoritative availability fact following release. | Infrastructure Management for the availability fact; Platform Audit Infrastructure for immutable audit evidence. |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Reservation is no longer an active reservation commitment against the applicable resource.
- The resource's current availability has been determined according to its authoritative availability policy.
- Reservation history has not been deleted.

## 16.2 Postconditions

- Authorized consumers may rely on the current Infrastructure Availability fact under G10 ownership.
- `ReservationCancelled` represents the completed cancellation fact under Infrastructure Management ownership.
- Immutable audit evidence may be generated from the canonical completed business fact under Section A ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome states that the Reservation is “closed,” and L2-G10 defines Reservation Cancellation, but the canonical Reservation lifecycle lists only Requested → Confirmed → Active → Completed → Archived and defines no `Closed` or `Cancelled` state. | A canonical resulting Reservation lifecycle position cannot be asserted without inventing or renaming a state. This materially affects lifecycle correctness. | Upstream L2-G10 architecture authority must clarify the canonical lifecycle treatment of cancellation/closure. |

**Architecture assessment:** `BLOCKED`

The Draft preserves all unambiguous frozen semantics and source-defined behaviour, but it cannot be promoted as source-complete until the Reservation cancellation lifecycle position is resolved upstream.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
