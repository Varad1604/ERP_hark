# Former Workforce Member Rejoining

**Scenario ID:** L3-E-008  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E01 — Workforce & Engagement Workflows; L2-E02 — Position & Assignment Workflows where assignment consequences are involved  
**Referenced By:** Future L4 System Behavior Specifications and materially participating downstream workforce consumers

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-008 |
| Canonical Title | Former Workforce Member Rejoining |
| Primary Owner | Workforce & Engagement |
| Participating Domains | E |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
Establish a new Workforce Engagement for a former workforce member while retaining the existing canonical Workforce Profile and all historical Workforce Engagements.

## 3. Business Outcome
An existing Workforce Profile is retained, a new Workforce Engagement is created beneath that profile, the new engagement enters its appropriate lifecycle, and the rejoining business fact is published without altering historical engagements.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorised former workforce member is eligible to rejoin and the existing Workforce Profile is identified.

### 4.2 End Boundary
A new Workforce Engagement has been established for the former workforce member and the rejoining fact has been published.

### 4.3 In Scope
- Rejoining a former workforce member.
- Reuse of the existing Workforce Profile.
- Creation of a new Workforce Engagement.
- Preservation of prior engagement history.
- Establishment of the new engagement's lifecycle.
- Publication of the canonical rejoining event.
- Coordination with Position & Assignment where a new assignment is subsequently required.

### 4.4 Out of Scope
- Recruitment pipeline management.
- Modification or reopening of a closed historical Workforce Engagement.
- Position Assignment ownership.
- Payroll processing.
- Leave administration.
- Employee Profile ownership outside the workforce engagement lifecycle.
- Technical identity/account provisioning.

### 4.5 Related Scenarios
E001 Workforce Engagement Establishment and Activation; E007 Workforce Engagement Closure and Institutional Exit; E011 Workforce Member Assignment to Active Institutional Position.

## 5. Ownership and Participating Domains
Workforce & Engagement owns the re-engagement lifecycle. L2-E01 explicitly states that WF-014 creates a new Workforce Engagement beneath an existing Workforce Profile and preserves historical engagements.

## 6. Business Actors, Roles and Responsibilities
- Workforce & Engagement owns the new Workforce Engagement outcome.
- The applicable institutional authority establishes the authorised rejoining/appointment outcome under the governed policy/workflow process.
- Position & Assignment owns any subsequent Position Assignment lifecycle.
- Downstream consumers use the resulting workforce fact without taking ownership.
- No unsupported technical role is introduced.

## 7. Preconditions and Entry Criteria
- The former workforce member has an existing canonical Workforce Profile.
- Historical Workforce Engagements remain preserved.
- An authorised rejoining/appointment outcome exists.
- Applicable policy/workflow conditions are satisfied.

## 8. Trigger
An authorised former-workforce rejoining outcome becomes effective.

## 9. Main Business Flow
1. Identify the existing Workforce Profile.
2. Confirm that the historical profile is the canonical profile for the returning person.
3. Confirm the authorised rejoining outcome.
4. Execute **WF-014 — Rejoin Workforce**.
5. Retain the existing Workforce Profile.
6. Create a **new Workforce Engagement** beneath that profile.
7. Preserve all previous Workforce Engagement history unchanged.
8. Establish the new engagement's lifecycle according to the applicable Workforce workflow.
9. Publish **`WorkforceRejoined`**.
10. Where organisational placement is required, continue through the authoritative Position & Assignment workflow without transferring ownership to E01.

## 10. Alternate Business Flows
- A former workforce member may rejoin through a new engagement type supported by the workforce architecture; the historical engagements remain separate.
- If the rejoining outcome is not authorised, no new Workforce Engagement is established.
- If the canonical Workforce Profile cannot be confidently identified, the rejoining outcome does not proceed as an authoritative new engagement; the ambiguity must be resolved through the appropriate upstream governance.

## 11. Failure, Interruption and Business Recovery
If creation of the new engagement cannot validly complete, historical Workforce Profiles and prior engagements remain unchanged. A corrected or re-authorised rejoining outcome is processed as a new governed business progression. No technical retry, rollback, or persistence mechanism is specified.

## 12. Business Rules and Decision Constraints
- A closed Workforce Engagement is not reopened.
- Subsequent employment relationships create a new Workforce Engagement.
- A Workforce Profile may contain multiple historical Workforce Engagements.
- Historical engagements remain immutable.
- Workforce Profile continuity is preserved across rejoining.
- Position Assignment lifecycle remains owned by E02.

## 13. State and Lifecycle Impact
The historical Workforce Engagement(s) remain in their existing terminal/historical states. A **new Workforce Engagement** begins its own lifecycle beneath the existing Workforce Profile; rejoining does not reopen the previous engagement.

## 14. Business Events and Cross-Domain Collaboration
| Event | Canonical Publisher | Business Meaning |
|---|---|---|
| `WorkforceRejoined` | Workforce & Engagement | A former workforce member has rejoined through a new Workforce Engagement |

L2-E01 defines `WorkforceRejoined` for WF-014.

## 15. Business Authorization, Approval and Audit Obligations
Rejoining follows the applicable policy-driven appointment/workflow governance. The scenario preserves the distinction between a new engagement and the historical engagement. Business authorization and the rejoining decision must remain auditable; no unsupported approver or evidence requirement is invented.

## 16. Completion Criteria and Postconditions
- Existing Workforce Profile remains canonical.
- A new Workforce Engagement exists beneath it.
- Historical engagements remain preserved and unchanged.
- `WorkforceRejoined` has been published.
- Any new organisational placement is handled through E02's authoritative lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions
No material unresolved issue affecting ownership, final outcome, lifecycle, event publisher, or boundary was identified in the locked L2-E01 material used for this scenario.

**Authoring classification: PASS — source-grounded.**
