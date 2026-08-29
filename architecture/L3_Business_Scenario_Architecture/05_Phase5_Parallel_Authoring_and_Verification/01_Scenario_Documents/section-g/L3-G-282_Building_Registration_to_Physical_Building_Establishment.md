# Building Registration to Physical Building Establishment

# 1. Scenario Metadata

**Scenario ID:** L3-G-282  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** Floor, Space and downstream physical-context scenarios; L3 targeted revalidation and Architecture Approval review

# 2. Scenario Purpose

Register one canonical G10 Building beneath exactly one valid Physical Campus while preserving immutable identity, parent-child integrity, audit history and the separation between establishment and later operational lifecycle transitions.

# 3. Business Outcome

A canonical Building is established under the applicable Physical Campus.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid building must be represented within a Physical Campus.

## 4.2 End Boundary

The scenario ends when the Building exists canonically under one valid Physical Campus, satisfies Building registration invariants, begins at the canonical initial lifecycle position, preserves registration evidence and publishes the exact canonical G10 establishment fact.

## 4.3 In Scope

Parent Physical Campus validation; Building code/metadata validation; Building aggregate establishment; immutable identity; parent relationship; initial lifecycle position; registration evidence; canonical establishment event.

## 4.4 Out of Scope

Building activation, later lifecycle transitions, Floor/Space registration, consumer-domain operations and implementation mechanisms.

# 5. Ownership and Participating Domains

Infrastructure Management is the sole participating domain and owns Building identity, characteristics, lifecycle, parent Physical Campus relationship and establishment event.

# 6. Business Actors, Roles and Responsibilities

**Infrastructure Administrator** performs governed Building establishment within authorized infrastructure scope. Other administrative participants do not bypass G10 registration rules.

# 7. Preconditions and Entry Criteria

- Parent Physical Campus exists and satisfies applicable G10 establishment preconditions.
- Building code is unique within the Physical Campus.
- Required Building information is valid.
- Gross Area is greater than zero where applicable.
- Construction Year is not in the future where applicable.
- The requester is authorized.

# 8. Trigger

A valid building must be represented within a Physical Campus.

# 9. Main Business Flow

1. Resolve the parent Physical Campus.
2. Validate parent eligibility and Building registration rules.
3. Validate Building code uniqueness and required metadata.
4. Establish the Building aggregate under exactly one Physical Campus and assign immutable identity.
5. Place the newly established Building at canonical initial lifecycle position `Proposed`; registration does not make it Operational.
6. Preserve registration evidence.
7. Publish `BuildingCreated` as the canonical completed structural establishment fact.

# 10. Alternate Business Flows

- Invalid parent condition: reject/defer establishment.
- Duplicate Building code: reject the proposal and preserve existing canonical records.

# 11. Failure, Interruption and Business Recovery

Invalid parent, metadata, invariant or authorization conditions prevent establishment. A later attempt proceeds only after the authoritative business condition is corrected.

# 12. Business Rules and Decision Constraints

- A Building belongs to exactly one Physical Campus.
- Building Code is unique within Campus.
- Aggregate identity is immutable.
- Establishment does not imply Operational lifecycle state.
- `BuildingCreated` is the sole canonical event for the completed Building establishment/registration fact; earlier `BuildingRegistered` wording is legacy vocabulary.

# 13. State and Lifecycle Impact

The new Building enters `Proposed`. Any stricter pre-Proposed preparation position is not externally authoritative. Later progression toward Operational occurs only through the separate governed G10 lifecycle.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

- `BuildingCreated` — published by Infrastructure Management for the completed Building establishment fact.

`BuildingRegistered` is legacy vocabulary for the same fact and is not treated as a separate semantic event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

After establishment, authorized consumers may use the G10 Building Summary contract without acquiring ownership.

# 15. Business Authorization, Approval and Audit Obligations

The Infrastructure Administrator must possess applicable authority. Registration evidence preserves parent reference, Building identity, code, validated metadata, actor, authorization and `BuildingCreated` publication.

# 16. Completion Criteria and Postconditions

Completion requires a unique canonical Building under exactly one valid Physical Campus, initial lifecycle `Proposed`, preserved evidence and publication of `BuildingCreated`.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former `BuildingRegistered` / `BuildingCreated` event conflict and initial-lifecycle ambiguity were resolved by locked `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §2.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
