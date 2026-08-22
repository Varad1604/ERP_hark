# Transport Pass Renewal to Continued Transport Authorisation

**Scenario ID:** L3-G-114  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose
This scenario governs renewal of an existing Transport Pass so that an eligible transport user can continue authorised use of institutional transport services without an unintended break in operational authorisation.

## 3. Business Outcome
The eligible Transport Pass is renewed, its validity is extended according to authoritative policy, and continued transport authorisation is established with complete historical traceability.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An active or renewal-eligible Transport Pass is approaching the end of its authorised validity and continued transport use is required.
### 4.2 End Boundary
The renewal becomes effective and the user has continued transport authorisation for the newly established validity period.
### 4.3 In Scope
- Identify the existing Transport Pass and holder.
- Validate continued eligibility and allocation context.
- Determine the renewal period and applicable transport service.
- Apply configured approval where required.
- Renew the pass.
- Establish the renewed validity/operational status.
- Preserve prior and renewed pass history.
- Make the continued authorisation fact available to participating consumers.
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

The frozen participating-domain set is **G, C, E, F, H**. Participation does not transfer ownership.

G04 owns Transport Pass Management. The Transport Pass aggregate owns pass information, validity, assigned transport service and operational status. G04 remains separate from student/personnel ownership and Finance's accounting ownership.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Coordinator | Coordinates operational pass renewal. |
| Transport Manager / configured authority | Reviews or approves renewal where policy requires. |
| Student / Employee / authorised transport user | Holds the pass and receives continued authorisation. |
| Section C authority | Owns student identity/status where applicable. |
| Section E authority | Owns employee identity/status where applicable. |
| Finance | May consume transport information for its own financial process; it does not own the pass. |
| H / communication consumer | May consume the renewed authorisation outcome. |

## 7. Preconditions and Entry Criteria
- An existing Transport Pass is identifiable.
- The holder remains eligible for the relevant transport service.
- The underlying allocation/service context remains valid or can be renewed under policy.
- Renewal policy and validity rules can be evaluated.

## 8. Trigger
An existing Transport Pass approaches expiry or enters a policy-defined renewal point.

## 9. Main Business Flow
1. Identify the existing Transport Pass, holder and assigned transport service.
2. Validate continued eligibility using the authoritative holder and transport allocation context.
3. Determine whether the pass may be renewed under configured transport policy.
4. Establish the applicable renewed validity period and pass information.
5. Obtain required operational approval where configured.
6. Renew the Transport Pass.
7. Establish continued operational authorisation for the renewed period.
8. Preserve the previous validity and renewal history.
9. Publish/use the renewed authorisation business outcome for relevant consumers.

## 10. Alternate Business Flows
- Holder is no longer eligible: renewal is not granted and the existing pass follows its authoritative expiry/restriction outcome.
- Allocation has changed: update/resolve the transport allocation through the applicable allocation workflow before renewal.
- Required approval is rejected: renewal does not become effective.
- Existing pass information requires correction: resolve the authoritative pass record before completing renewal.
- A lost/damaged pass is being replaced rather than renewed: use the replacement-pass scenario.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Holder no longer eligible | Continued authorisation cannot be established | Resolve eligibility/allocation through its owner |
| Renewal approval rejected | Renewal does not become effective | Retain current authorised state until policy expiry/restriction |
| Allocation/service context changed | Renewal cannot be safely completed | Resolve allocation change first |
| Pass record requires correction | Renewal remains unresolved | Correct authoritative pass information |

All recovery is expressed as business decisions. No technical retry, queue, rollback or infrastructure mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule / Constraint | Effect |
|---|---|
| Transport Pass owns validity and operational status. | Renewal changes the pass's authorised validity without transferring ownership. |
| Pass lifecycle includes `Draft`, `Issued`, `Active`, `Suspended`, `Expired`, `Cancelled`, and `Replaced`. | Renewal continues an eligible authorisation; it does not invent a new lifecycle state. |
| Pass formats, validity and transport policies are configuration driven. | Renewal periods are not hard-coded by L3. |
| Historical transport operations shall never be deleted. | Previous pass validity remains traceable. |
| Student/personnel ownership remains C/E. | Renewal does not alter identity ownership. |
| Finance owns accounting. | Renewal does not create a financial accounting outcome. |

## 13. State and Lifecycle Impact
The authoritative Transport Pass lifecycle includes `Draft → Issued → Active → Suspended / Expired`, with `Cancelled` and `Replaced` also defined. This scenario preserves the pass as an operational authorisation and establishes continued validity; no new `Renewed` lifecycle state is invented.

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
The locked G04 event catalogue explicitly defines **`PassRenewed` — Pass renewed**. G04 is the canonical publisher because Transport Pass Management is G04-owned. The scenario does not invent an alternative event name.

### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participants are G, C, E, F and H. G04 owns the pass renewal. C/E provide authoritative identity/status context; F may consume transport information for its own financial process; H may consume the renewed authorisation for governed communication. These relationships do not transfer G04 ownership.

## 15. Business Authorization, Approval and Audit Obligations
Pass renewal authority follows the configured G04 operational authorization model. Where approval is configured, renewal cannot become effective before that business decision. Renewal decision, validity period, holder, transport service and historical changes remain auditable.

Transport Management maintains immutable operational history and lifecycle transitions are audited.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Existing pass identified.
- Continued eligibility validated.
- Renewal period determined.
- Required approval completed.
- Pass renewed and continued authorisation established.
- `PassRenewed` business event published by G04.
- Historical validity preserved.
### 16.2 Postconditions
- The named Transport Pass outcome is authoritative within G04.
- Pass history remains traceable.
- Underlying student/personnel ownership remains with its canonical owner.
- No financial or maintenance outcome is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. The Stage 3/Stage 4 mapping is important: final ID G114 corresponds to the frozen Renewal scenario under the authoritative Stage 4 mapping, not the sequential row numbering visible in the Stage 3 register.

### 17.2 Assumptions
- Pass policies, formats, validity rules and approval requirements remain configuration driven.
- Manual operational decisions remain available where G04 permits manual coordination.
- No actor, rule, lifecycle state, event, publisher, contract, owner or dependency is invented.

### 17.3 Deferred Questions
Any material unresolved upstream vocabulary must be resolved through architecture governance before Architecture Approval/Lock.
