# Lost or Damaged Transport Pass to Replacement Pass

**Scenario ID:** L3-G-115  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G, C, E, H  
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
This scenario governs replacement of a Transport Pass that has been lost or damaged, establishing a new operational pass while preserving the historical relationship to the affected pass.

## 3. Business Outcome
The lost or damaged Transport Pass is superseded through the authoritative replacement process and a replacement pass is issued with valid operational authorisation and preserved history.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A Transport Pass holder reports that the active pass is lost or damaged and a replacement is required.
### 4.2 End Boundary
A replacement Transport Pass is issued and becomes the authoritative operational pass for the holder, subject to its applicable validity and status.
### 4.3 In Scope
- Identify holder and affected pass.
- Establish lost/damaged condition.
- Determine replacement eligibility.
- Apply required approval and pass policy.
- Issue replacement pass.
- Establish replacement status/validity.
- Preserve original and replacement history.
- Make replacement authorisation available to relevant consumers.
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

The frozen participating-domain set is **G, C, E, H**. Participation does not transfer ownership.

G04 owns Transport Pass Management. The Transport Pass aggregate owns pass information, validity, assigned transport service and operational status. G04 remains separate from student/personnel ownership and Finance's accounting ownership.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Coordinator | Manages replacement-pass processing. |
| Transport Manager / configured authority | Reviews replacement where policy requires. |
| Student / Employee / authorised transport user | Reports the lost or damaged pass and receives replacement. |
| Section C authority | Owns student identity/status where applicable. |
| Section E authority | Owns employee identity/status where applicable. |
| H / communication consumer | May consume replacement authorisation information. |

## 7. Preconditions and Entry Criteria
- An identifiable Transport Pass exists or a holder can be authoritatively established.
- Lost/damaged status is reported.
- The holder remains eligible for transport authorisation or can be evaluated under policy.
- Replacement policy can be applied.

## 8. Trigger
A Transport Pass is reported lost or damaged and replacement is requested.

## 9. Main Business Flow
1. Identify the holder and affected Transport Pass.
2. Record the lost or damaged condition and relevant business evidence.
3. Validate the holder's continued eligibility for transport authorisation.
4. Determine replacement conditions under configured pass policy.
5. Apply required operational approval.
6. Establish the replacement pass information and validity.
7. Issue the replacement pass.
8. Establish the replacement as the authoritative operational pass.
9. Preserve the original pass history and the replacement relationship.
10. Publish/use the replacement authorisation outcome for relevant consumers.

## 10. Alternate Business Flows
- Holder is not eligible for replacement: do not issue a replacement; resolve the underlying eligibility condition.
- Pass is damaged but still operationally valid: apply the configured replacement decision without assuming immediate suspension unless policy requires it.
- Original pass is later recovered: apply the authoritative pass replacement/cancellation policy; do not create duplicate active authorisation.
- Required approval is rejected: replacement does not become effective.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Holder not eligible | Replacement cannot be issued | Resolve eligibility/allocation issue |
| Replacement approval rejected | New pass does not become effective | Resolve the business decision |
| Original pass recovered | Duplicate authorisation risk | Apply configured replacement/cancellation policy |
| Required pass information unavailable | Replacement remains unresolved | Obtain authoritative context before issuance |

All recovery is expressed as business decisions. No technical retry, queue, rollback or infrastructure mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule / Constraint | Effect |
|---|---|
| Transport Pass owns operational status and validity. | Replacement is a G04 pass-management outcome. |
| Pass lifecycle includes `Replaced`. | Replacement is represented by the authoritative lifecycle model rather than a fabricated state. |
| Historical transport operations shall never be deleted. | Original and replacement history remain available. |
| Pass formats and policies remain configuration driven. | No local replacement fee/validity rule is invented. |
| Identity ownership remains C/E. | Replacement does not alter the holder's canonical identity. |

## 13. State and Lifecycle Impact
The authoritative Transport Pass lifecycle includes `Replaced`. This scenario moves from an affected existing pass through the replacement outcome. It does not invent a separate `Lost` or `Damaged` lifecycle state unless the authoritative policy defines one.

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
The locked G04 event catalogue explicitly defines **`PassReplaced` — Replacement issued**. G04 is the canonical publisher. No alternative replacement event name is introduced.

### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participants are G, C, E and H. G04 owns the Transport Pass and replacement. C/E provide authoritative holder context; H may consume the replacement outcome. Participation does not transfer pass ownership.

## 15. Business Authorization, Approval and Audit Obligations
Replacement authority follows configured G04 transport policy. The reported condition, affected pass, holder, replacement decision, replacement pass, effective validity/status and historical relationship must be auditable.

Transport Management maintains immutable operational history and lifecycle transitions are audited.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Affected pass and holder identified.
- Lost/damaged condition recorded.
- Replacement eligibility validated.
- Required decision/approval completed.
- Replacement pass issued and effective.
- `PassReplaced` business event published by G04.
- Original and replacement history preserved.
### 16.2 Postconditions
- The named Transport Pass outcome is authoritative within G04.
- Pass history remains traceable.
- Underlying student/personnel ownership remains with its canonical owner.
- No financial or maintenance outcome is implicitly completed.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. The exact final ID/filename is confirmed by the Phase 5 workload and Stage 4 identity mapping.

### 17.2 Assumptions
- Pass policies, formats, validity rules and approval requirements remain configuration driven.
- Manual operational decisions remain available where G04 permits manual coordination.
- No actor, rule, lifecycle state, event, publisher, contract, owner or dependency is invented.

### 17.3 Deferred Questions
Any material unresolved upstream vocabulary must be resolved through architecture governance before Architecture Approval/Lock.
