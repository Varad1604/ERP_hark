# Infrastructure Capacity Definition to Capacity Profile Activation

# 1. Scenario Metadata

**Scenario ID:** L3-G-288  
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

Establish an authoritative Capacity Profile for physical infrastructure so institutional consumers can rely on governed capacity facts without acquiring ownership of the infrastructure or its capacity.

# 3. Business Outcome

A Capacity Profile becomes authoritative for the applicable infrastructure.

# 4. Scenario Boundary and Scope

The scenario begins when capacity information is established or changed for a physical infrastructure aggregate and ends when the current Capacity Profile is authoritative, prior history is preserved, and the exact G10 capacity fact is published.

In scope: configured capacity types, validation, effective dating, temporary overrides, history, authorization, audit and event publication. Out of scope: utilisation/scheduling/examination decisions, infrastructure lifecycle change and implementation design.

# 5. Ownership and Participating Domains

- **G10:** owns Capacity Profile, capacity rules, history and event semantics.
- **A:** supplies authorization, event-governance and audit capabilities without owning the capacity fact.

# 6. Business Actors, Roles and Responsibilities

**Infrastructure Administrator** performs the governed capacity definition/change where authorized.

# 7. Preconditions and Entry Criteria

- Applicable infrastructure exists and permits capacity modification.
- Capacity type is supported by configuration.
- Proposed capacity value is valid and non-negative.
- Required authorization exists.

# 8. Trigger

Capacity information is established for a physical infrastructure aggregate.

# 9. Main Business Flow

1. Identify the infrastructure and capacity type.
2. Validate authorization, infrastructure eligibility, capacity type, value and governing policy.
3. Determine the current effective Capacity Profile, including bounded temporary override where permitted.
4. Preserve prior capacity history and make the new Capacity Profile authoritative.
5. Publish `CapacityUpdated` as the sole canonical G10 capacity-establishment/change event.

# 10. Alternate Business Flows

- Temporary override: apply for the governed period and preserve the configured/current capacity lineage.
- Multiple configured capacity types: govern each supported capacity fact independently.

# 11. Failure, Interruption and Business Recovery

Invalid value/type, policy or authorization prevents a new authoritative capacity fact. The prior profile remains current until a later valid change.

# 12. Business Rules and Decision Constraints

- Infrastructure Management owns capacity.
- Capacity values cannot be negative.
- Capacity supports configurable profiles, multiple types, effective dating, temporary overrides, history and auditability.
- `CapacityUpdated` is the sole canonical event for establishment or change of an authoritative Capacity Profile.
- `CapacityChanged` and `RoomCapacityChanged` are legacy vocabulary and shall not be treated as separate events.
- No Capacity Profile lifecycle state named `Active` is required; the frozen word “activation” means the effective profile has become authoritative.

# 13. State and Lifecycle Impact

The Capacity Profile becomes the authoritative current capacity fact; no invented `Active` lifecycle state is introduced. The physical infrastructure lifecycle remains unchanged.

# 14. Business Events and Cross-Domain Collaboration

- `CapacityUpdated` — published by G10 for establishment or change of the authoritative Capacity Profile.

Authorized consumers may consume the current capacity fact without acquiring ownership.

# 15. Business Authorization, Approval and Audit Obligations

Infrastructure Administrator must satisfy applicable authorization and G10 capacity rules. Evidence preserves prior/new capacity, effective dates, reason, override history, actor/authority and `CapacityUpdated` publication.

# 16. Completion Criteria and Postconditions

A valid Capacity Profile is authoritative, prior history is preserved, and G10 has published `CapacityUpdated`.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former `CapacityChanged` / `RoomCapacityChanged` / `CapacityUpdated` vocabulary conflict was resolved by locked `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §4.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
