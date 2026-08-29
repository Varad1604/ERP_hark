# Workforce Transfer Between Organisational Contexts

**Scenario ID:** L3-E-003  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Authoritative L2:** L2-E01 — Workforce & Engagement Workflows; L2-E02 where assignment outcomes are required; Organisation Management; applicable Policy/Workflow governance

## 1. Scenario Metadata
Frozen Section E identity is preserved. Workforce Engagement remains E01-owned; organisational structures remain Organisation Management-owned; E02 owns assignment aggregates.

## 2. Scenario Purpose
Govern an authorised workforce movement from one organisational context to another while preserving Workforce Engagement ownership and canonical organisational/assignment boundaries.

## 3. Business Outcome
The Workforce Engagement has an authoritative effective target organisational context, with prior context preserved historically and required E02 assignment changes handled by E02.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorised workforce transfer is available with authoritative source and target organisational context.
### 4.2 End Boundary
The Workforce Engagement reflects the authorised target organisational context from its effective boundary and the canonical transfer business fact is available to consumers.
### 4.3 In Scope
- Workforce organisational-context transfer.
- Effective dating.
- Historical context preservation.
- Coordination of separately owned E02 assignment outcomes.
### 4.4 Out of Scope
- Creation/modification of organisational structures.
- E02 assignment lifecycle itself.
- Person ownership.
- Payroll, leave, IAM or technical implementation.
### 4.5 Related Scenarios
E001 — Workforce Engagement Establishment and Activation; E010 — Bulk Workforce Organisational Transfer; E011/E012 — Position Assignment scenarios where assignment outcomes are required.

## 5. Ownership and Participating Domains
Workforce & Engagement owns the Workforce Engagement. Organisation Management owns organisation/department/campus structures. E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship and Assignment History.

## 6. Business Actors, Roles and Responsibilities
Authorised workforce and organisational business participants act under applicable governance. Exact approving role names are not invented where not exposed by authoritative sources.

## 7. Preconditions and Entry Criteria
- Active Workforce Engagement exists.
- Source and target organisational contexts are authoritative.
- Transfer is authorised.
- Required E02 assignment changes are separately governed by E02.

## 8. Trigger
An authorised organisational transfer decision becomes ready for execution.

## 9. Main Business Flow
1. Identify Workforce Engagement and current context.
2. Resolve authoritative target organisation/department/campus.
3. Validate transfer under applicable governance.
4. Establish the new effective organisational context.
5. Preserve prior context and effective history.
6. Coordinate required E02 assignment changes through E02.
7. Publish canonical transfer business fact.
8. Make resulting workforce context available to consumers.

## 10. Alternate Business Flows
- Invalid source/target context prevents transfer.
- Unauthorised transfer is not applied.
- Future-effective transfer is not current before its effective boundary.
- E01 does not create or mutate E02-owned assignment aggregates.
- Organisation Management remains authoritative for organisational structures.

## 11. Failure, Interruption and Business Recovery
An incomplete transfer leaves the existing Workforce Engagement organisational context authoritative until the business prerequisite is resolved. Organisation corrections remain Organisation-owned; assignment corrections remain E02-owned. No technical recovery is described.

## 12. Business Rules and Decision Constraints
- Workforce Engagement remains E01-owned.
- Organisational structure remains Organisation Management-owned.
- Transfer changes are effective-dated.
- Historical organisational context remains traceable.
- E02 owns assignment-state changes arising from the transfer.

## 13. State and Lifecycle Impact
The Workforce Engagement remains within its E01 lifecycle while its effective organisational context changes. Any E02 assignment transitions are separate E02-owned lifecycle changes.

## 14. Business Events and Cross-Domain Collaboration
The retrieved L2-E01 material confirms event publication and the E01/E02 ownership boundary but does not expose a verified exact transfer-event name. No event is invented. Verify the detailed L2-E01 event catalogue before lock.

## 15. Business Authorization, Approval and Audit Obligations
Transfer authorisation follows applicable Workforce/Organisation/Policy/Workflow governance. Exact approving role names are not invented. Transfer and effective boundary remain auditable.

## 16. Completion Criteria and Postconditions
- Authorised target context established.
- Effective transfer recorded.
- Prior context preserved.
- Required E02 outcomes remain E02-owned.
- Canonical transfer fact published.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-E003-01 — Event vocabulary:** Exact transfer event requires verification against detailed L2-E01 event catalogue.

**OI-E003-02 — Policy criteria:** Transfer eligibility/approval criteria are upstream policy/workflow concerns and are not invented here.

**Classification: PASS WITH ISSUES — source-grounded authoring; not lock-ready until exact upstream event/state/rule vocabulary is verified.**
