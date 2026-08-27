# Floor Registration to Building Hierarchy Extension

# 1. Scenario Metadata

**Scenario ID:** L3-G-283  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md`  
**Referenced By:** L3-G-281; L3-G-284; L3 targeted revalidation and Architecture Approval review

# 2. Scenario Purpose

Establish a canonical Floor under exactly one valid Building while applying G10-owned hierarchy, uniqueness, lifecycle, authorization, event and audit requirements.

# 3. Business Outcome

A canonical Floor is created under the correct Building.

# 4. Scenario Boundary and Scope

The scenario begins when a valid Floor must be established within a Building and ends when one canonical Floor exists under the correct Building, starts at the canonical initial lifecycle position, preserves registration evidence and publishes the exact canonical G10 establishment fact.

In scope: parent Building validation, Floor-number uniqueness, Floor establishment, immutable identity, initial lifecycle, audit evidence and canonical event. Out of scope: Building creation, Space creation, later Floor lifecycle progression, downstream consumer operations and implementation mechanisms.

# 5. Ownership and Participating Domains

Infrastructure Management solely owns the Floor aggregate, parent relationship, lifecycle, registration rules and events.

# 6. Business Actors, Roles and Responsibilities

**Infrastructure Administrator** performs Floor establishment within authorized infrastructure scope.

# 7. Preconditions and Entry Criteria

- Parent Building exists and satisfies applicable G10 parent conditions.
- Floor Number is unique within the Building.
- Required Floor information is valid.
- The action is authorized.

# 8. Trigger

A valid floor must be established within a Building.

# 9. Main Business Flow

1. Resolve the parent Building.
2. Validate parent eligibility and Floor registration rules.
3. Validate Floor Number uniqueness and required information.
4. Establish the Floor under exactly one Building and assign immutable identity.
5. Place the new Floor at canonical initial lifecycle position `Proposed`; registration does not make it Operational.
6. Preserve registration evidence.
7. Publish `FloorCreated` as the canonical completed establishment fact.

# 10. Alternate Business Flows

Invalid parent, duplicate Floor Number or invalid authorization prevents establishment and leaves existing infrastructure unchanged.

# 11. Failure, Interruption and Business Recovery

A later attempt may proceed only after the authoritative parent, uniqueness or authorization condition is corrected.

# 12. Business Rules and Decision Constraints

- Every Floor belongs to exactly one Building.
- Floor Number must be unique within its Building.
- Floor lifecycle cannot exceed the parent Building lifecycle.
- `FloorCreated` is the sole canonical completed structural establishment fact; earlier `FloorRegistered` wording is legacy vocabulary.
- Establishment begins at `Proposed`, not Operational.

# 13. State and Lifecycle Impact

The newly established Floor begins at `Proposed`. Later G10 lifecycle progression is outside this scenario.

# 14. Business Events and Cross-Domain Collaboration

- `FloorCreated` — published by Infrastructure Management.

No cross-domain hand-off is required for completion. Authorized later consumers may use the G10 Floor Summary contract without acquiring ownership.

# 15. Business Authorization, Approval and Audit Obligations

The Infrastructure Administrator must be authorized. Evidence preserves parent Building, Floor identity/number, validated input, actor, authorization and `FloorCreated` publication.

# 16. Completion Criteria and Postconditions

A unique canonical Floor exists under exactly one correct Building, is initially `Proposed`, remains G10-owned, and has auditable establishment evidence with `FloorCreated` published.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former `FloorRegistered` / `FloorCreated` event conflict and initial-lifecycle ambiguity were resolved by locked `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` §2.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
