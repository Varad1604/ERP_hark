# Vehicle Operational Issue to Maintenance Recommendation

**Scenario ID:** L3-G-110  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Transport Management  
**Participating Domains:** G  
**Scenario Type:** Critical  
**Priority:** P0  
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
| Final L3 ID | L3-G-110 |
| Canonical Title | Vehicle Operational Issue to Maintenance Recommendation |
| Primary Owner | Transport Management |
| Participating Domains | G |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose
This scenario governs identification and assessment of a vehicle operational issue until Transport Management establishes a maintenance recommendation for the issue. G04 explicitly states that Transport Management recommends maintenance while G01 executes maintenance.

## 3. Business Outcome
A vehicle operational issue is assessed and a business maintenance recommendation is established for follow-up by the maintenance-owning domain, without Transport Management executing maintenance.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
A vehicle operational issue is identified during transport operations or operational vehicle assessment.
### 4.2 End Boundary
The issue has been assessed and the maintenance recommendation/service need has been established for the appropriate maintenance process.
### 4.3 In Scope
- Register/identify the operational issue.
- Assess operational impact.
- Determine whether maintenance is indicated.
- Establish the maintenance recommendation.
- Preserve operational evidence and history.
- Hand the maintenance need to the maintenance-owning process without taking ownership of maintenance execution.
### 4.4 Out of Scope
- Vehicle asset ownership, procurement, registration, disposal or depreciation.
- Maintenance execution/work orders/vendor repair.
- Driver/personnel master ownership.
- Financial accounting.
- Technical APIs, databases, services, queues, deployment, infrastructure, UI or IAM/RBAC.
- Independent notification delivery.
- Any later business outcome not required to reach the frozen scenario outcome.

## 5. Ownership and Participating Domains
Transport Management (G04) owns the operational vehicle assignment outcome. Vehicle ownership remains outside G04 and maintenance execution remains with G01.

The frozen participating-domain set is **G**. Participation does not transfer canonical ownership.

G04 owns operational vehicle assignment, availability and replacement history; vehicle asset ownership remains outside Transport Management. Maintenance execution remains with G01. Transport Management may recommend maintenance but does not execute it.

## 6. Business Actors, Roles and Responsibilities
| Actor / Role | Responsibility |
|---|---|
| Transport Administrator / Coordinator | Records and assesses the operational issue. |
| Transport Supervisor / configured authority | Reviews operational significance where required. |
| Driver / operational workforce | May identify or report the vehicle issue during operation. |
| G01 maintenance authority | Owns maintenance execution after the recommendation/service need is established. |

## 7. Preconditions and Entry Criteria
- A vehicle is within Transport Management's operational context.
- A material operational issue has been identified.
- Sufficient business information exists to assess whether maintenance is indicated.

## 8. Trigger
A vehicle operational issue is identified.

## 9. Main Business Flow
1. Identify the affected vehicle and operational context.
2. Record the observed operational issue and supporting business evidence.
3. Assess the operational impact on vehicle suitability.
4. Determine whether the issue indicates a maintenance requirement.
5. If maintenance is indicated, establish the maintenance recommendation/service need.
6. Preserve the issue, assessment and recommendation history.
7. Hand the maintenance need to the maintenance-owning process.
8. Stop the scenario at the maintenance recommendation/service-need outcome; do not execute the repair.

## 10. Alternate Business Flows
- Issue does not require maintenance: record the operational assessment and resolve through the applicable operational process.
- Issue makes the vehicle unavailable for a trip: vehicle replacement/assignment is a separate operational outcome.
- Issue is a breakdown: the Vehicle Breakdown to Replacement Vehicle Assignment scenario governs replacement, while maintenance remains separate.
- G01 determines the actual maintenance execution outcome: that is outside this L3 scenario.

## 11. Failure, Interruption and Business Recovery
| Situation | Consequence | Business Recovery |
|---|---|---|
| Issue cannot be assessed from available business evidence | Recommendation cannot be established | Obtain the required operational evidence |
| Issue does not require maintenance | No maintenance recommendation | Apply the appropriate transport operational resolution |
| Issue makes vehicle unsuitable for current trip | Vehicle assignment may be affected | Use the vehicle replacement/assignment workflow |
| Maintenance is required | G04 stops at recommendation/service need | G01 owns maintenance execution |

Recovery is business-level only. No technical retries, queues, rollback or infrastructure failure mechanics are introduced.

## 12. Business Rules and Decision Constraints
| Rule | Effect |
|---|---|
| Transport Management recommends maintenance where operational issues are identified. | G04 owns the recommendation outcome. |
| Maintenance execution remains within G01. | G04 cannot execute repair/work-order activity. |
| Vehicle ownership remains outside G04. | Assessment does not create asset ownership. |
| Complaint/operational evidence and audit history are preserved. | Issue assessment remains traceable. |
| Configuration over customisation. | Operational assessment policies remain configurable. |

## 13. State and Lifecycle Impact
This scenario does not create or alter a vehicle asset lifecycle. It operates on the G04 operational vehicle context and ends with a maintenance recommendation/service need. The G04 source does not define a new Vehicle Issue lifecycle state, so none is invented.

No lifecycle state is invented beyond the authoritative G04 terminology.

## 14. Business Events and Cross-Domain Collaboration
### 14.1 Business Events
The G04 workflow describes `Vehicle Issue → Operational Assessment → Maintenance Recommendation → Administrative Review → Manual G01 Service Request`, but the retrieved event catalogue does not expose a canonical event name for the recommendation. No `MaintenanceRecommended` event is invented.
### 14.2 Synchronous Business Collaboration / Hand-offs
The frozen participating-domain set is G only. G01 remains the maintenance owner/dependency but is not a participating domain in the frozen scenario semantics. The L3 boundary therefore stops at the G04 maintenance recommendation/service-need outcome.

## 15. Business Authorization, Approval and Audit Obligations
Operational issue assessment and recommendation remain within configured G04 transport responsibilities. Issue evidence, vehicle context, assessment, recommendation and audit history must be traceable. Maintenance approval/execution belongs to the maintenance-owning process.

G04 requires immutable operational audit history and historical operational information is not deleted.

## 16. Completion Criteria and Postconditions
### 16.1 Completion Criteria
- Vehicle operational issue identified.
- Issue assessed.
- Maintenance requirement determined.
- Maintenance recommendation/service need established where required.
- Operational evidence preserved.
- Scenario stops before maintenance execution.
### 16.2 Postconditions
- The named operational vehicle outcome is authoritative within G04.
- Assignment/replacement/recommendation history remains traceable.
- Vehicle asset ownership remains outside G04.
- Maintenance execution remains outside G04.

## 17. Open Issues, Assumptions and Deferred Questions
### 17.1 Open Issues
No material ownership conflict identified. The source explicitly separates G04 maintenance recommendation from G01 maintenance execution. Exact canonical recommendation event vocabulary is not exposed and is therefore not fabricated.
### 17.2 Assumptions
- Vehicle operational policies remain configuration driven.
- G04 performs operational decisions and recommendations only.
- No actor, rule, state, event, publisher, owner or dependency is invented.
### 17.3 Deferred Questions
Material unresolved event vocabulary must be confirmed through architecture governance before Architecture Approval/Lock.
