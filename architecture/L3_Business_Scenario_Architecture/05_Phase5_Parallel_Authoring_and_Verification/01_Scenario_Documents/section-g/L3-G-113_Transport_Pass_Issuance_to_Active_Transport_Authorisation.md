# Transport Pass Issuance to Active Transport Authorisation

**Scenario ID:** L3-G-113  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-113 |
| Canonical Title | Transport Pass Issuance to Active Transport Authorisation |
| Primary Owner | Transport Management |
| Participating Domains | G, C, E, F, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
This scenario governs issuance of a transport pass after transport entitlement/allocation has reached the point where an operational pass can be issued, through establishment of the pass as active transport authorisation.

## 3. Business Outcome
An eligible transport user receives an issued and active Transport Pass that authorises use of the applicable institutional transport service, with pass history and cross-domain operational consequences traceable.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An approved/eligible transport allocation exists and requires a Transport Pass for operational authorisation.
### 4.2 End Boundary
The Transport Pass is active and represents the user's current operational transport authorisation.
### 4.3 In Scope
- Identify the eligible transport allocation/user context.
- Validate pass issuance conditions.
- Establish pass information and validity.
- Issue the Transport Pass.
- Establish Active pass status.
- Preserve pass history.
- Make the operational authorisation fact available to relevant participating domains/consumers.
### 4.4 Out of Scope
- Vehicle asset ownership, procurement, registration, disposal or depreciation.
- Maintenance execution and work-order management.
- Driver/personnel master ownership.
- Financial accounting.
- Technical APIs, databases, services, queues, infrastructure, deployment, UI or technical IAM/RBAC.
- Notification delivery as an implementation process.
- Any downstream business outcome outside the frozen scenario title.

## 5. Ownership and Participating Domains
Transport Management (G04) owns the Trip and its operational execution. The scenario does not transfer ownership of drivers, vehicles, maintenance, student records or communication services.

The frozen participating-domain set is **G, C, E, F, H**.

G04 owns Trip planning/execution and operational vehicle assignment. Driver/personnel ownership remains with Section E; vehicle ownership remains outside G04; maintenance execution remains with G01.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Administrator / Coordinator | Manages operational pass issuance. |
| Transport Manager / configured authority | Performs configured review/approval where required. |
| Student/Employee/authorised transport user | Receives the operational transport authorisation. |
| Section C / Section E authority | Owns underlying student/personnel identity/status. |
| Finance | May consume transport allocation/pass information for financial processes; it does not own the Transport Pass. |
| H / Parent Communication or relevant consumer | May consume the active transport authorisation where governed. |

## 7. Preconditions and Entry Criteria
- An eligible/approved transport allocation exists.
- The underlying user identity/status is available from its canonical owner.
- Required route/stop/service context exists where applicable.
- Pass issuance conditions can be evaluated under configured transport policy.

## 8. Trigger
An eligible transport allocation reaches the point at which a Transport Pass must be issued.

## 9. Main Business Flow
1. Identify the eligible transport user and authoritative transport allocation.
2. Validate that the allocation and user context satisfy pass-issuance conditions.
3. Determine the applicable transport category, route, stop, effective date and expiry/validity information.
4. Establish the Transport Pass information.
5. Issue the Transport Pass.
6. Establish the pass as Active for the applicable transport authorisation period.
7. Preserve issuance, validity and operational history.
8. Make the active authorisation available to relevant participating consumers through governed business collaboration/events.

## 10. Alternate Business Flows
- User/allocation is not eligible: do not issue the pass; resolve the underlying eligibility/allocation issue.
- Required allocation information is incomplete: pass issuance remains unresolved until authoritative allocation context is available.
- Pass issuance requires configured approval: obtain that approval before making the pass Active.
- Existing active pass conflicts with issuance: use the applicable pass replacement/change workflow rather than creating duplicate active authorisation.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Allocation not eligible | Pass cannot be issued | Resolve allocation/eligibility |
| Required user context unavailable | Issuance cannot be validated | Obtain authoritative context from C/E |
| Required approval rejected | Pass does not become Active | Resolve or retain non-issued outcome |
| Active-pass conflict | New authorisation cannot safely become active | Apply pass change/replacement governance |

All recovery described here is business-level. No technical retry, queue, rollback or infrastructure mechanism is introduced.

## 12. Business Rules and Decision Constraints
| Rule / Constraint | Effect |
|---|---|
| Transport Pass represents operational authorisation to use institutional transport services. | Active Pass is the operational authorisation outcome. |
| Pass information includes pass number, holder, transport category, assigned route/stop, effective date, expiry date and operational status. | Issuance establishes the applicable pass context. |
| Pass lifecycle includes `Draft`, `Issued`, `Active`, `Suspended`, `Expired`, `Cancelled` and `Replaced`. | This scenario covers issuance through Active. |
| Transport Management owns Transport Pass Management. | Pass ownership remains G04-owned. |
| Student ownership remains Section C; employee ownership remains Section E. | Pass issuance does not transfer identity/lifecycle ownership. |
| Finance owns financial accounting. | Pass issuance does not become a financial accounting outcome. |
| Historical pass information remains permanently available. | Issuance and later changes remain traceable. |

## 13. State and Lifecycle Impact
The authoritative Transport Pass lifecycle includes `Draft → Issued → Active → Suspended/Expired`, with additional `Cancelled` and `Replaced` states. This scenario covers the business progression through `Issued → Active`; no new pass state is introduced.

No additional lifecycle state is invented.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
G04 establishes immutable domain events for significant transport activities, but the retrieved material does not expose the exact canonical event name for Transport Pass issuance/activation. No `TransportPassIssued` or `TransportPassActivated` event is fabricated.

### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participants are G, C, E, F and H. G04 owns the Transport Pass. C/E provide authoritative student/personnel context; F may consume transport allocation/pass information for finance; H may consume operational authorisation information. None of these collaborations transfers pass ownership.

## 15. Business Authorization, Approval and Audit Obligations
Pass issuance authority and configured approval remain within G04 transport policy. Pass holder, allocation, route/stop, validity, issuance decision and active authorisation must remain auditable. Separation of ownership across C/E/F/H is preserved.

G04 requires immutable operational audit history and historical transport operations are preserved.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Eligible transport allocation identified.
- Issuance conditions validated.
- Pass information established.
- Pass issued.
- Pass becomes Active.
- Operational authorisation is established.
- Pass history preserved and relevant consumers can use the business outcome.
### 16.2 Postconditions
- The scenario's named operational outcome is established within the owning domain.
- Relevant transport history remains traceable.
- Cross-domain participation does not transfer canonical ownership.
- No later unrelated business outcome is implied.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. Exact canonical Transport Pass issuance/activation event names are not exposed in the retrieved G04 material and are intentionally not invented.

### 17.2 Assumptions
- Transport policies and checklist policies remain configuration driven.
- Manual operational decisions remain supported where G04 states they are manually overridable.
- No actor, rule, state, event, publisher, contract, owner or dependency is invented.

### 17.3 Deferred Questions
Any unresolved canonical event vocabulary must be confirmed through architecture governance before Architecture Approval/Lock.
