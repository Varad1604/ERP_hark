# Post-Trip Inspection to Operational Follow-Up

**Scenario ID:** L3-G-112  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose
This scenario governs post-trip inspection and operational follow-up after a transport Trip has completed. G04 defines post-trip operational guidance including vehicle inspection, damage observation, fuel recording, lost-and-found review, cleaning requirement and operational remarks.

## 3. Business Outcome
The completed Trip receives its applicable post-trip inspection and the resulting operational follow-up is established, with any maintenance recommendation kept separate from maintenance execution.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A transport Trip has completed and is ready for applicable post-trip operational inspection.
### 4.2 End Boundary
Post-trip findings are recorded, required operational follow-up is established, and the Trip's historical operational record is preserved.
### 4.3 In Scope
- Identify completed Trip and vehicle.
- Perform applicable post-trip inspection/guidance.
- Record material findings.
- Determine operational follow-up.
- Recommend maintenance where the findings require it.
- Preserve inspection and follow-up history.
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

The frozen participating-domain set is **G**.

G04 owns Trip planning/execution and operational vehicle assignment. Driver/personnel ownership remains with Section E; vehicle ownership remains outside G04; maintenance execution remains with G01.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Coordinator / Administrator | Coordinates post-trip inspection and follow-up. |
| Assigned Driver / operational workforce | Provides operational observations where applicable. |
| Transport Supervisor / configured authority | Reviews material findings and follow-up where required. |
| G01 maintenance authority | Owns maintenance execution when G04 establishes a maintenance recommendation/service need. |

## 7. Preconditions and Entry Criteria
- Trip has reached the canonical Completed state.
- Applicable post-trip inspection policy/guidance is available.
- The completed Trip and operational vehicle context are identifiable.

## 8. Trigger
A completed Trip enters the institution's configured post-trip inspection/follow-up process.

## 9. Main Business Flow
1. Identify the completed Trip and associated operational vehicle context.
2. Apply the configured post-trip inspection guidance.
3. Record relevant observations, including vehicle condition, damage observations, fuel information, lost-and-found findings, cleaning requirements and operational remarks where applicable.
4. Assess whether findings require operational follow-up.
5. If a maintenance need is indicated, establish a maintenance recommendation/service need; do not execute maintenance in G04.
6. Establish any other authorised operational follow-up.
7. Preserve inspection findings, decisions and follow-up history against the Trip's operational record.
8. Complete the post-trip follow-up outcome.

## 10. Alternate Business Flows
- No issue is identified: record the inspection outcome and close the follow-up.
- Damage or vehicle condition issue is identified: establish appropriate operational follow-up and maintenance recommendation where indicated.
- Lost-and-found matter is identified: record the operational finding and hand off to the applicable governed process.
- Cleaning requirement is identified: establish the operational follow-up without inventing a facilities ownership transfer.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Inspection identifies no material issue | No corrective follow-up required | Record normal inspection outcome |
| Vehicle issue identified | Follow-up required | Establish operational action and/or maintenance recommendation |
| Maintenance is indicated | G04 stops at recommendation | G01 owns maintenance execution |
| Follow-up requires another domain | G04 preserves its operational finding | Hand off the business outcome without transferring Trip ownership |

All recovery described here is business-level. No technical retry, queue, rollback or infrastructure mechanism is introduced.

## 12. Business Rules and Decision Constraints
| Rule / Constraint | Effect |
|---|---|
| Post-trip guidance may include vehicle inspection, damage observation, fuel recording, lost-and-found review, cleaning requirement and operational remarks. | These are authoritative inspection dimensions where configured. |
| Maintenance execution remains within G01. | G04 only recommends maintenance. |
| Historical trip execution remains permanently available. | Inspection/follow-up does not overwrite or erase Trip history. |
| Every lifecycle transition is audited. | Completion/follow-up remains traceable. |
| Configuration over customisation. | Inspection policy remains institutionally configurable. |

## 13. State and Lifecycle Impact
The Trip lifecycle ends at `Completed` after execution. This scenario does not reopen the Trip or create a new Trip lifecycle state. Post-trip inspection is an operational follow-up applied to the completed Trip history.

No additional lifecycle state is invented.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
The retrieved G04 source establishes immutable transport events but does not expose a canonical event name for post-trip inspection completion or follow-up. No event name is fabricated.

### 14.2 Synchronous Business Collaboration / Hand-offs
Frozen participating domain is G only. G04 remains owner. G01 is an upstream/downstream maintenance boundary, but maintenance execution is outside this scenario and does not transfer ownership.

## 15. Business Authorization, Approval and Audit Obligations
Inspection responsibility and material follow-up decisions remain governed by G04 operational policy. Inspection evidence, findings, recommendations and follow-up decisions remain auditable.

G04 requires immutable operational audit history and historical transport operations are preserved.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Completed Trip identified.
- Applicable post-trip inspection performed.
- Findings recorded.
- Operational follow-up determined.
- Maintenance recommendation established where required, without executing maintenance.
- Historical record preserved.
### 16.2 Postconditions
- The scenario's named operational outcome is established within the owning domain.
- Relevant transport history remains traceable.
- Cross-domain participation does not transfer canonical ownership.
- No later unrelated business outcome is implied.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. Exact canonical post-trip inspection/follow-up event vocabulary is not exposed and is therefore intentionally unresolved.

### 17.2 Assumptions
- Transport policies and checklist policies remain configuration driven.
- Manual operational decisions remain supported where G04 states they are manually overridable.
- No actor, rule, state, event, publisher, contract, owner or dependency is invented.

### 17.3 Deferred Questions
Any unresolved canonical event vocabulary must be confirmed through architecture governance before Architecture Approval/Lock.
