# Transport Pass Suspension to Restricted Transport Access

**Scenario ID:** L3-G-116  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-116 |
| Canonical Title | Transport Pass Suspension to Restricted Transport Access |
| Primary Owner | Transport Management |
| Participating Domains | G, C, F, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
This scenario governs suspension of a Transport Pass when an authoritative business condition requires transport access to be restricted, through establishment of the pass's Suspended state and the resulting restricted operational access.

## 3. Business Outcome
The affected Transport Pass is suspended under authorised transport policy and the holder's operational transport access is restricted for the applicable period, with the suspension remaining auditable.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An active Transport Pass exists and an authoritative business condition requires temporary restriction of its transport authorisation.
### 4.2 End Boundary
The Transport Pass is in the authoritative Suspended state and the corresponding operational transport access is restricted.
### 4.3 In Scope
- Identify the active pass and holder.
- Establish the suspension reason/basis.
- Validate suspension against configured policy.
- Obtain required approval.
- Suspend the pass.
- Establish restricted operational access.
- Preserve suspension history.
- Make the business suspension outcome available to relevant consumers.
### 4.4 Out of Scope
- Student/personnel identity ownership.
- Transport asset ownership.
- Maintenance execution.
- Financial accounting or payment processing.
- Notification delivery implementation.
- Technical APIs, databases, services, queues, deployment, infrastructure, UI or technical IAM/RBAC.
- Any later or separate transport lifecycle outcome not required by the scenario title.

## 5. Ownership and Participating Domains
Transport Management (G04) is the canonical owner of Transport Pass Management. The Transport Pass aggregate owns pass information, validity, assigned transport service and operational status. Student identity remains owned by Section C; employee identity remains owned by Section E; financial accounting remains owned by Finance.

The frozen participating-domain set is **G, C, F, H**. Participation does not transfer ownership.

G04 owns Transport Pass Management. The Transport Pass aggregate owns pass information, validity, assigned transport service and operational status. G04 remains separate from student/personnel ownership and Finance's accounting ownership.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Coordinator | Coordinates pass suspension. |
| Transport Manager / configured authority | Makes/reviews the operational suspension decision where required. |
| Student / authorised transport user | Is affected by the restriction. |
| Section C authority | Owns student identity/status where applicable. |
| Finance | May consume transport status for its own governed process; it does not own the pass. |
| H / Parent Communication or relevant consumer | May consume the suspension outcome. |

## 7. Preconditions and Entry Criteria
- An active Transport Pass exists.
- A business basis for restriction exists.
- Suspension policy and authority can be evaluated.
- Holder/pass context is authoritative.

## 8. Trigger
An authorised business condition requires an active Transport Pass to be suspended.

## 9. Main Business Flow
1. Identify the affected active Transport Pass and holder.
2. Establish the business basis for suspension.
3. Validate the suspension against configured transport/pass policy.
4. Complete required operational review or approval.
5. Move the Transport Pass to the authoritative `Suspended` state.
6. Establish the corresponding restricted transport-access outcome.
7. Preserve the suspension reason, decision, effective period and pass history.
8. Publish/use the suspension business outcome for relevant consumers.

## 10. Alternate Business Flows
- Suspension is rejected: the pass remains active and authorised according to its existing validity.
- The pass has already expired/cancelled/replaced: suspension is no longer the applicable lifecycle outcome; apply the current authoritative state instead.
- Suspension should be permanent rather than temporary: use the authoritative cancellation/closure policy rather than extending this scenario.
- The suspension is later lifted: apply the authoritative reactivation workflow; this scenario ends at restricted access.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Suspension basis not accepted | Access remains active | Resolve the business decision |
| Required approval rejected | Suspension does not take effect | Retain active authorisation |
| Pass already expired/cancelled/replaced | Suspension is not applicable | Follow the current lifecycle state |
| Suspension period ends | This scenario is complete | Use authoritative restoration/reactivation workflow if defined |

All recovery is expressed as business decisions. No technical retry, queue, rollback or infrastructure mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule / Constraint | Effect |
|---|---|
| Transport Pass lifecycle includes `Suspended`. | Suspension is an authoritative pass lifecycle outcome. |
| Pass validity and operational status are owned by the Transport Pass aggregate. | G04 owns suspension. |
| Pass formats, validity and suspension policies remain configuration driven. | No local suspension duration is invented. |
| Historical transport operations shall never be deleted. | Suspension history remains traceable. |
| Student/personnel ownership remains C/E and Finance owns accounting. | Suspension does not transfer identity or financial ownership. |

## 13. State and Lifecycle Impact
The authoritative Transport Pass lifecycle includes `Suspended`. This scenario covers the business transition from an active transport authorisation to `Suspended` and restricted access. It does not invent a separate `Restricted` pass lifecycle state; 'restricted transport access' is the business outcome of the Suspended state.

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
The locked G04 event catalogue explicitly defines **`PassSuspended` — Pass suspended**. G04 is the canonical publisher. No alternative suspension event name is introduced.

### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participants are G, C, F and H. G04 owns the pass suspension. C may provide student context; F may consume transport information for its own process; H may consume the operational restriction outcome. None of these collaborations transfers Transport Pass ownership.

## 15. Business Authorization, Approval and Audit Obligations
Suspension authority follows the configured G04 authorization model. The suspension basis, decision, effective period, pass status and resulting restricted-access outcome remain auditable. Any financial consequence is owned by Finance and is not authored as part of this scenario.

Transport Management maintains immutable operational history and lifecycle transitions are audited.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Active pass identified.
- Suspension basis established.
- Policy and authority validated.
- Required approval completed.
- Pass moved to `Suspended`.
- Restricted transport access established.
- `PassSuspended` business event published by G04.
- Suspension history preserved.
### 16.2 Postconditions
- The named Transport Pass outcome is authoritative within G04.
- Pass history remains traceable.
- Underlying student/personnel ownership remains with its canonical owner.
- No financial or maintenance outcome is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. The canonical event `PassSuspended` and lifecycle state `Suspended` are explicitly defined in locked G04.

### 17.2 Assumptions
- Pass policies, formats, validity rules and approval requirements remain configuration driven.
- Manual operational decisions remain available where G04 permits manual coordination.
- No actor, rule, lifecycle state, event, publisher, contract, owner or dependency is invented.

### 17.3 Deferred Questions
Any material unresolved upstream vocabulary must be resolved through architecture governance before Architecture Approval/Lock.
