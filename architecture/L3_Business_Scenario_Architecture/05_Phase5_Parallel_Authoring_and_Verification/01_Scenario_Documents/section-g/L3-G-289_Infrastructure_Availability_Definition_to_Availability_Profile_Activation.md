# Infrastructure Availability Definition to Availability Profile Activation

# 1. Scenario Metadata

**Scenario ID:** L3-G-289  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A04; L2-A06; L2-A07; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and Architecture Approval review

# 2. Scenario Purpose

Establish the authoritative Availability Profile that determines whether a physical resource may be consumed operationally while keeping availability distinct from the infrastructure lifecycle.

# 3. Business Outcome

An Availability Profile becomes authoritative for the applicable infrastructure.

# 4. Scenario Boundary and Scope

The scenario begins when availability conditions must be established for a physical resource and ends when the first authoritative Availability Profile is in effect, remains distinct from physical lifecycle, is auditable and is represented by the exact canonical G10 availability-establishment fact.

In scope: availability definition, effective conditions, authorization, policy validation, scheduled/partial closure, event publication and auditability. Out of scope: consumer scheduling decisions, maintenance execution, invented Availability Profile lifecycle states and implementation design.

# 5. Ownership and Participating Domains

- **G10:** owns Availability Profile and the authoritative current usability fact.
- **A:** supplies authorization, event-governance and audit capabilities without owning availability.

# 6. Business Actors, Roles and Responsibilities

**Infrastructure Administrator** administers the governed availability definition where authorized.

# 7. Preconditions and Entry Criteria

- Physical resource exists.
- Its lifecycle permits the availability definition.
- Requested availability condition is supported by institutional policy/configuration.
- The action is authorized.

# 8. Trigger

Availability conditions must be established for a physical resource.

# 9. Main Business Flow

1. Identify the physical resource and availability conditions to establish.
2. Validate authorization, lifecycle compatibility and governing availability rules.
3. Apply source-defined effective dates, reasons, authority and partial-scope conditions where applicable.
4. Establish the first authoritative Availability Profile independently from physical lifecycle.
5. Publish `InfrastructureAvailabilityEstablished`.

# 10. Alternate Business Flows

- Scheduled closure: govern the applicable start/end period and reason.
- Partial availability: limit the condition to the governed physical scope.

# 11. Failure, Interruption and Business Recovery

Invalid lifecycle, policy or authorization conditions prevent establishment; the resource retains its prior authoritative business condition until a later valid action.

# 12. Business Rules and Decision Constraints

- Availability is independent from physical-infrastructure lifecycle.
- No Availability Profile lifecycle state named `Active` is required; the frozen word “activation” means an authoritative Availability Profile is in effect.
- `InfrastructureAvailabilityEstablished` is the canonical event for the first authoritative Availability Profile.
- Later changes use `InfrastructureAvailabilityChanged`.
- Lifecycle events such as `InfrastructureActivated`/`InfrastructureDeactivated` shall not substitute for availability events.

# 13. State and Lifecycle Impact

The authoritative Availability Profile is established without changing the infrastructure lifecycle. No invented `Active` availability state is introduced.

# 14. Business Events and Cross-Domain Collaboration

- `InfrastructureAvailabilityEstablished` — published by G10 for first authoritative availability establishment.

Authorized consumers may consume the resulting availability fact without ownership transfer.

# 15. Business Authorization, Approval and Audit Obligations

Infrastructure Administrator acts under applicable authorization. Evidence preserves the resource, governed availability condition, scope/effective period/reason, responsible authority and `InfrastructureAvailabilityEstablished` publication.

# 16. Completion Criteria and Postconditions

An authoritative Availability Profile is in effect for the resource, remains separate from lifecycle, and G10 has published `InfrastructureAvailabilityEstablished`.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing exact Availability Profile establishment event was resolved by locked `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §5.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
