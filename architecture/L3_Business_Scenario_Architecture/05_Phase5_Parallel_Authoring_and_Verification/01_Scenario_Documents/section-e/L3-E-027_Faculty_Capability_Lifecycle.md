# Faculty Capability Lifecycle

**Scenario ID:** L3-E-027  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Faculty Workforce  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Date:** 2026-08-20  
**Depends On:** L2-E03 — Faculty Workforce Workflows; HR Domain Architecture; Position & Assignment context where organisational eligibility is relevant  
**Referenced By:** Future L4 System Behavior Specification

## 1. Scenario Metadata
This scenario preserves the frozen canonical identity, title, owner, participating domains, Scenario Type and Priority. It applies the locked upstream architecture without transferring ownership or introducing new architecture.

## 2. Scenario Purpose
Manage the governed lifecycle of academic capability associated with a Faculty Workforce so that capability changes become institutionally usable only after the required review and approval.

## 3. Business Outcome
The Faculty Capability reaches its authoritative approved/current state, with capability history preserved and `FacultyCapabilityUpdated` published for completed capability changes.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A Faculty Workforce requires a new capability, a change to existing capability, or a governed capability-state decision.

### 4.2 End Boundary
The applicable Faculty Capability state is authoritative and its completed change is published.

### 4.3 In Scope
- Capture or update academic capability information.
- Review capability against authoritative qualification, expertise, specialization and approval context.
- Approve the capability where required.
- Update the capability lifecycle state.
- Preserve capability history.
- Publish the canonical capability-change event.

### 4.4 Out of Scope
Implementation mechanics, APIs, databases, services, UI, infrastructure, technical access control/IAM, queues, technical retries and other L4+ concerns are outside L3.

### 4.5 Related Scenarios
Only canonical dependencies relevant to this outcome are referenced; no new scenario ownership is created.

## 5. Ownership and Participating Domains
**Canonical owner:** Faculty Workforce.

Participating domains remain authoritative for their own concepts and provide business context only within those ownership boundaries. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
- Faculty Workforce owns Faculty Capability.
- Qualification/credential information remains governed by its authoritative source; E03 maintains qualification association/relevance rather than becoming the credential authority.
- Institutional academic authority participates in capability review/approval where required.

No technical permission model is introduced.

## 7. Preconditions and Entry Criteria
- Faculty Workforce exists.
- The capability subject is identifiable.
- Required capability information is available.
- Applicable review and approval conditions can be evaluated.

## 8. Trigger
An authorised capability request or an authoritative capability change requires the Faculty Capability lifecycle to be evaluated.

## 9. Main Business Flow
1. Identify the Faculty Workforce and capability under change.
2. Capture the capability information.
3. Review the capability against the applicable academic criteria.
4. Apply the required capability approval decision.
5. Update the Faculty Capability state.
6. Preserve the capability history.
7. Publish **`FacultyCapabilityUpdated`**.
8. Stop when the capability outcome is authoritative.

The flow stops when the Business Outcome and End Boundary are reached.

## 10. Alternate Business Flows
- A capability change that fails review does not become an approved capability outcome.
- A capability associated with a non-Active Faculty Workforce cannot remain operational for academic participation.
- Qualification association and capability are related but distinct: E03 maintains the capability consequence while credential authority remains outside E03 where defined upstream.

## 11. Failure, Interruption and Business Recovery
If capability review or approval fails, the prior authoritative capability state remains in force where applicable and the rejected request follows the governed business path. No technical retry or rollback is described.

Only business-level recovery is described.

## 12. Business Rules and Decision Constraints
- Faculty Capability is an authoritative E03 business capability.
- Capability decisions consider qualification, expertise, specialization and institutional approval.
- Capability SHALL remain separate from Teaching Eligibility.
- Capability history is preserved.
- Capability changes are auditable.
- `FacultyCapabilityUpdated` is the canonical event for completed capability information changes.

The rules remain owned by the authoritative upstream architecture.

## 13. State and Lifecycle Impact
Faculty Capability follows its own lifecycle. The locked L2 explicitly identifies capability request/capture, review, approval and capability-state update as the governed sequence; the scenario does not invent additional lifecycle state names not explicitly defined in the source.

No new lifecycle state or transition is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `FacultyCapabilityUpdated` | Faculty Workforce | Faculty capability information has changed. |

### 14.2 Business Contracts / Hand-offs
Qualification, expertise and specialization provide authoritative inputs to capability decisions. Faculty Workforce retains ownership of the capability outcome and does not acquire ownership of upstream credential sources.

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations
Capability review and approval follow the institutional academic governance defined by L2-E03. No unsupported fixed approver or technical permission model is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Capability review is completed.
- Applicable approval is satisfied.
- Faculty Capability reaches its authoritative resulting state.
- Capability history is preserved.
- `FacultyCapabilityUpdated` is published.

### 16.2 Postconditions
The completed business outcome remains authoritative for subsequent institutional behaviour and preserves required historical traceability.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
None identified.

### 17.2 Assumptions
None.

### 17.3 Deferred Questions
None.

**Authoring Classification:** PASS — source-grounded Draft.
