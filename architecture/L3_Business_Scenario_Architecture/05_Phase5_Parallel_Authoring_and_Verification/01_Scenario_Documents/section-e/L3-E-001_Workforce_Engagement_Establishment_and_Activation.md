# Workforce Engagement Establishment and Activation

**Scenario ID:** L3-E-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** A, B, E  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Authoritative L2:** L2-E01 — Workforce & Engagement Workflows

## 1. Scenario Metadata
Frozen Section E identity is preserved. L2-E01 identifies Workforce Engagement as the primary aggregate and Workforce Management Domain as authoritative owner. E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship and Assignment History.

## 2. Scenario Purpose
Establish and activate an approved institutional workforce engagement so the institution has an authoritative workforce relationship for the person and dependent domains can consume the resulting business fact.

## 3. Business Outcome
An approved workforce engagement becomes authoritative in its applicable active/effective business position, with effective dating and historical continuity preserved.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An approved workforce appointment/engagement decision is available with authoritative Person and organisational context.
### 4.2 End Boundary
The Workforce Engagement reaches its applicable active/effective position and the canonical workforce activation fact is published.
### 4.3 In Scope
- Workforce Engagement establishment and activation.
- Applicable institutional/organisational context.
- Effective dating and historical continuity.
- Publication of the resulting workforce business fact.
### 4.4 Out of Scope
- Recruitment/candidate processing.
- Person ownership.
- Organisation-structure ownership.
- Position Assignment, Responsibility Assignment, Reporting Relationship or Assignment History lifecycle.
- Payroll, leave calculation, IAM or implementation architecture.
### 4.5 Related Scenarios
E002 — Active Workforce Engagement Modification; E003 — Workforce Transfer Between Organisational Contexts; E007 — Workforce Engagement Closure and Institutional Exit.

## 5. Ownership and Participating Domains
Workforce & Engagement owns the Workforce Engagement aggregate. Platform Foundation owns Person; Organisation Management owns organisational structures. E02 owns assignment aggregates. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities
Use only authoritative Workforce/Policy/Workflow business participants. The retrieved L2 material did not expose a sufficiently specific actor name for the appointment authority, so no role title is invented. Workforce & Engagement establishes the engagement; upstream owners supply authoritative facts.

## 7. Preconditions and Entry Criteria
- Approved appointment/engagement exists.
- Person information is authoritative.
- Applicable organisation context is authoritative.
- Applicable workforce governance has been satisfied.

## 8. Trigger
An approved workforce appointment/engagement becomes ready for establishment and activation.

## 9. Main Business Flow
1. Identify the approved appointment/engagement and canonical Person.
2. Resolve applicable institutional and organisational context.
3. Establish the Workforce Engagement under E01.
4. Apply the effective engagement context and applicable business conditions.
5. Establish the engagement in its authoritative active/effective position.
6. Preserve effective dating and history.
7. Publish the canonical activation business fact.
8. Make authoritative workforce information available to dependent domains.

Any E02 assignment outcome is requested/consumed through E02; E01 does not perform the E02 lifecycle.

## 10. Alternate Business Flows
- If appointment approval is absent, activation does not occur.
- If authoritative Person/organisation context is invalid, activation does not complete.
- An unresolved E02 assignment outcome is not silently created by E01.

## 11. Failure, Interruption and Business Recovery
An incomplete activation remains in its applicable E01 business position until the business prerequisite is resolved. Person and organisation corrections remain with their canonical owners. No technical retry, queue or rollback behaviour is defined.

## 12. Business Rules and Decision Constraints
- Workforce Engagement is E01-owned.
- Workforce relationships originate, evolve and conclude in Workforce & Engagement.
- Significant workforce information is effective-dated and historically traceable.
- E02 exclusively owns Position Assignment, Responsibility Assignment, Reporting Relationship and Assignment History.

## 13. State and Lifecycle Impact
The Workforce Engagement follows the canonical E01 lifecycle. Exact establishment/activation state labels are not invented because the retrieved L2 material did not expose the detailed state-matrix entries for this transition.

## 14. Business Events and Cross-Domain Collaboration
L2-E01 establishes event publication after successful business state transitions, but the retrieved source does not expose a verified exact activation-event name. No event name is invented. The canonical event and publisher must be verified from the detailed locked L2-E01 event catalogue before lock.

## 15. Business Authorization, Approval and Audit Obligations
Appointment approval and applicable Workforce/Policy/Workflow governance are prerequisites. Exact approving actor names are not invented. Establishment and activation remain auditable and historically traceable.

## 16. Completion Criteria and Postconditions
- Approved appointment represented as a Workforce Engagement.
- Applicable effective position established.
- Historical continuity preserved.
- Canonical activation fact published.
- Dependent domains can consume authoritative workforce information.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-E001-01 — Event vocabulary:** Exact activation event name requires verification against detailed L2-E01 event catalogue.

**OI-E001-02 — State vocabulary:** Exact establishment/activation states require verification against detailed L2-E01 state matrix.

**Classification: PASS WITH ISSUES — source-grounded authoring; not lock-ready until exact upstream event/state vocabulary is verified.**
