# Infrastructure Recovery Condition to Controlled Recovery State

# 1. Scenario Metadata

**Scenario ID:** L3-G-313  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A, B  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Related Infrastructure Management recovery and lifecycle scenarios

# 2. Scenario Purpose

Govern the business response of canonical physical Infrastructure when a disaster or major disruption creates a recovery condition, while preserving G10 ownership of physical Infrastructure and avoiding invention of a disaster-response bounded context, recovery lifecycle, or recovery event that is not defined upstream.

# 3. Business Outcome

Infrastructure recovery status is recorded and affected resources follow the authorised recovery lifecycle

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a disaster or major infrastructure disruption affects physical resources.

## 4.2 End Boundary

The frozen end boundary is reached when Infrastructure recovery status is recorded and the affected resources have followed the authoritative recovery lifecycle applicable to the condition.

## 4.3 In Scope

- Identification of affected G10-owned physical Infrastructure.
- Assessment of source-supported Infrastructure condition, criticality, recovery metadata and availability facts.
- Governed lifecycle/availability restriction of affected Infrastructure where an authoritative transition is established.
- Preservation of InstitutionalCampus context without transferring physical ownership from G10.
- Authorization and audit obligations for any business-state change.

## 4.4 Out of Scope

- Emergency response, crisis management, incident coordination or operational evacuation execution.
- Inventing a Disaster Management aggregate, actor, workflow, lifecycle or event.
- Treating recovery metadata as authority to execute disaster-recovery procedures.
- Modifying InstitutionalCampus governance or identity.
- Choosing a recovery lifecycle transition that authoritative L2 has not assigned to this disaster-recovery scenario.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the affected physical Infrastructure, its canonical lifecycle, availability, criticality and recovery metadata. |
| Platform Foundation / Section A | Owns authorization and audit infrastructure used for governed Infrastructure changes. |
| Institution & Structure / Section B | Owns InstitutionalCampus identity/governance context referenced by the affected physical site. |

Participation does not transfer ownership of the physical Infrastructure lifecycle from G10.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Identifies affected Infrastructure and initiates source-supported governed Infrastructure actions. | May act only within authorized Infrastructure scope. |
| Infrastructure Manager | Reviews physical condition, criticality, recovery metadata and availability implications. | Does not acquire authority over emergency/crisis execution. |
| Auditor | Reviews preserved audit evidence for governed Infrastructure changes. | Read-only evidence role. |

No canonical Disaster Management actor or recovery decision authority is defined by the reviewed locked sources; none is invented.

# 7. Preconditions and Entry Criteria

- The affected physical Infrastructure exists under G10 ownership.
- The disaster or major disruption has created an Infrastructure recovery condition.
- Applicable InstitutionalCampus context is resolvable where required.
- Any Infrastructure state change is subject to Platform authorization and G10 lifecycle rules.

# 8. Trigger

A disaster or major infrastructure disruption affects physical resources

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical physical Infrastructure affected by the disruption. | Infrastructure Management | The affected physical scope is established without duplicating Infrastructure identity. |
| 2 | Evaluate current G10-owned lifecycle, availability, operational status, criticality and recovery metadata. | Infrastructure Management | Authoritative physical facts are available for the recovery decision. |
| 3 | Preserve the relevant InstitutionalCampus reference and governance context. | Institution & Structure / Infrastructure Management | Institutional and physical ownership remain separated. |
| 4 | Determine whether an authoritative G10 lifecycle or availability transition is explicitly applicable to the recovery condition. | Infrastructure Management | Only source-defined transitions may be used. |
| 5 | Record source-supported recovery status and perform only the transition(s) that upstream architecture unambiguously authorizes. | Infrastructure Management using Section A authorization/audit capability | Supported recovery facts are governed and auditable. |
| 6 | Stop before inventing the undefined disaster-recovery procedure or lifecycle required to prove the complete frozen outcome. | Infrastructure Management / Architecture governance | The material upstream gap is preserved for resolution. |

Because the authoritative recovery lifecycle is not defined consistently enough to establish the complete frozen outcome, this Draft cannot describe a complete successful path without inventing architecture.

# 10. Alternate Business Flows

No complete scenario-specific alternate business flow can be established while the authoritative recovery lifecycle remains unresolved.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Recovery condition requires emergency response, crisis management or recovery procedures | G10 recovery metadata is insufficient to execute the requested operational response. | Route the responsibility only after the missing canonical owning bounded context and contract are defined upstream. | Architecture governance |
| Requested recovery transition is not one of G10's permitted lifecycle transitions | The physical Infrastructure cannot be moved to an invented state. | Preserve the current canonical state until an authorized upstream transition is identified or defined. | Infrastructure Management |
| Authorization fails | No Infrastructure state or recovery-status change may proceed. | Preserve existing authoritative facts. | Platform Foundation / Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Criticality provides recovery metadata but does not perform disaster recovery. | Infrastructure Management | Recovery priority, maximum downtime, alternate facility and backup-utility facts do not grant recovery-execution ownership. | L2-G10 §13A.74 |
| Disaster Management is stated to own emergency plans, recovery procedures, incident management and crisis response. | Upstream owner undefined in repository | G10 cannot silently assume those procedures. | L2-G10 §9A.259 |
| Infrastructure supports a disaster-recovery governance flow including assessment, temporary closure, recovery planning and restoration. | Infrastructure Management | The flow exists, but it does not define a canonical recovery lifecycle/state model or ownership for recovery procedures. | L2-G10 §39 |
| Lifecycle transitions must follow the approved lifecycle model and skipped transitions are prohibited. | Infrastructure Management | Recovery cannot use an invented or skipped transition. | L2-G10 BR-LC-01 / BR-LC-02 |
| Every business operation must be authorized before execution. | Platform Foundation — Authorization | Unauthorized recovery-state changes are prohibited. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Infrastructure | Existing G10 lifecycle state | A disruption may require restriction, closure or restoration. | **Unresolved for the complete scenario.** G10 defines `Temporarily Closed → Operational`, but does not establish that transition as the canonical disaster-recovery lifecycle for this frozen outcome. | Infrastructure Management | L2-G10 §11A.53; §39 |
| Infrastructure recovery metadata | Existing source-supported metadata | Recovery status/metadata may be recorded or updated. | Updated authoritative recovery facts where supported. | Infrastructure Management | L2-G10 criticality/recovery metadata sections |
| InstitutionalCampus | Existing B03 state | Referenced only as institutional context. | Unchanged. | Institution & Structure | L2-B03 §§1–4 |

No additional recovery state or lifecycle is introduced.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 defines canonical lifecycle events such as `InfrastructureTemporarilyClosed`, `InfrastructureReopened` and `InfrastructureOperational`, but the reviewed architecture does not define which event/transition is authoritative for this disaster-recovery scenario or define a canonical recovery-status event. No event is invented or renamed.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Institution & Structure | Infrastructure Management | InstitutionalCampus identity/governance context | Preserve institutional context for the affected physical site. | L2-B03; L2-G10 ownership split |
| Platform Foundation | Infrastructure Management | Authorization decision and audit capability | Govern any permitted physical-state change. | L2-A04; L2-A07 |

A mandatory contract to the stated Disaster Management owner of recovery procedures is not defined in the repository and therefore cannot be authored at L3.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Record/update G10-owned recovery facts | Infrastructure Administrator or Infrastructure Manager within authorized scope | Must satisfy G10 rules and Platform authorization. | Infrastructure Management |
| Execute a recovery procedure or select an undefined recovery lifecycle | Not established upstream | L3 cannot assign this authority. | **Unresolved** |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific recovery approval chain or separation-of-duty rule is defined by the reviewed authoritative sources. None is invented.

## 15.3 Business Audit and Evidence Obligations

Any permitted Infrastructure lifecycle or recovery-metadata change must remain auditable. Platform Audit owns immutable audit evidence; Infrastructure Management retains ownership of the underlying business facts and lifecycle decision.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen completion criteria require all of the following:

- The affected physical Infrastructure is authoritatively identified.
- Infrastructure recovery status is recorded.
- The affected resources complete the authorized recovery lifecycle applicable to the disruption.
- Canonical physical and InstitutionalCampus ownership remain separated.

The third criterion cannot currently be proven from the locked upstream architecture without selecting an undefined recovery lifecycle/owner.

## 16.2 Postconditions

- Source-supported G10 recovery facts may be preserved.
- No invented recovery state, event, actor, contract or ownership transfer has been introduced.
- Full scenario completion remains blocked pending upstream recovery-architecture resolution.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G10 §9A.259 assigns recovery procedures, incident management and crisis response to “Disaster Management”, but no canonical Disaster Management bounded context/owner or business contract is defined in the reviewed repository. | Material ownership and mandatory hand-off gap: L3 cannot identify who owns or authorizes the recovery procedure needed by the frozen outcome. | Upstream architecture must define the canonical owner and collaboration contract, or formally reassign the responsibility. |
| L2-G10 §39 describes disaster recovery as assessment → temporary closure → recovery planning → restored, while the canonical Infrastructure lifecycle defines general states/transitions but no disaster-specific “recovery lifecycle” or recovery-status state. | Material lifecycle and end-boundary ambiguity: L3 cannot prove which lifecycle path completes the frozen outcome. | Upstream G10 must define the authoritative recovery lifecycle mapping or explicitly bind disaster recovery to existing lifecycle transitions. |
| No exact canonical event is defined for the recovery-status outcome. | Event publication cannot be asserted safely. | Define/register the event upstream if the scenario requires one. |

**Architecture assessment:** `BLOCKED`

The unresolved recovery-procedure owner/contract and recovery-lifecycle definition materially affect ownership, lifecycle, authority and final-outcome correctness.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions beyond the upstream L3-blocking issues recorded above.
