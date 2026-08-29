# Hostel Configuration Change to Effective Residency Policy

**Scenario ID:** L3-G-092  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P2  
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
| Final L3 ID | L3-G-092 |
| Canonical Title | Hostel Configuration Change to Effective Residency Policy |
| Primary Owner | Hostel Management |
| Participating Domains | G, A |
| Scenario Type | Simple |
| Priority | P2 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern a Hostel Management operational configuration change from proposal and review through effective application within the authorised hostel operational boundary.

This scenario applies the locked architecture principle that hostel operational constraints are configuration driven while preserving the higher-level ownership of institutional business policy.

It does not create a new policy authority in Hostel Management and does not allow platform configuration capability to redefine institutional policy.

## 3. Business Outcome

An authorised Hostel Management operational configuration change becomes effective for its permitted scope and effective period, and subsequent hostel operations use the revised configuration.

Where the proposed change would alter institution-wide business policy rather than hostel operational configuration, the scenario does not activate it as a G03-owned policy. The matter remains with the authoritative upstream policy owner.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a change to an operational Hostel Management configuration is proposed.

### 4.2 End Boundary

The scenario ends when the authorised configuration is effective for its permitted hostel operational scope.

### 4.3 In Scope

- Identification of the proposed hostel operational configuration change.
- Validation of the proposed scope.
- Review against authoritative hostel governance.
- Required operational approval.
- Effective application of the approved configuration.
- Effective period/scope traceability.
- Audit of the configuration outcome.

### 4.4 Out of Scope

- Definition of institution-wide business policy.
- Creation of a new institutional authority.
- Technical configuration storage.
- technical access control/IAM.
- APIs, databases, services, deployment or infrastructure.
- Redefinition of upstream L1/L2 rules.
- Historical alteration of completed hostel business outcomes.

### 4.5 Related Scenarios

This scenario is separate from individual residency, leave, room, curfew, mess, complaint and discipline outcomes. Those scenarios consume the applicable effective hostel operational configuration.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Hostel Management (G03) | Owns hostel operational configuration within its authorised domain boundary. |
| Institution / upstream policy ownership | Owns institution-wide business policy when the proposed change is a policy-definition decision rather than G03 operational configuration. |
| Platform configuration capability (A) | Provides configuration capability where authorised; it does not become the owner of hostel business policy. |

The frozen Stage 3 participating-domain set is **G, A**. Participation does not transfer ownership.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Hostel Administrator | Proposes and administers the operational configuration change. |
| Hostel Warden / configured authority | Reviews or approves the change where the configured business workflow requires it. |
| Authoritative upstream policy authority | Decides institution-wide policy matters when the proposed change exceeds G03's operational configuration boundary. |

No technical access-control role is introduced.

## 7. Preconditions and Entry Criteria

- The proposed change is identifiable as an operational hostel configuration matter.
- The proposed scope is known.
- The change can be evaluated against the existing authoritative architecture.
- Any required business approval path is known.
- The change does not silently redefine an upstream rule or ownership boundary.

## 8. Trigger

A proposed change to an effective Hostel Management operational configuration is submitted for business review.

## 9. Main Business Flow

### 9.1 Identify the Configuration Change

Hostel Management identifies the configuration item, intended scope, intended effective period and business reason for the proposed change.

The proposal remains non-effective until authorised.

### 9.2 Validate the Ownership Boundary

Hostel Management determines whether the requested change is an operational configuration matter within G03.

If the request would establish or change institution-wide business policy, it cannot be silently activated as a G03 configuration.

### 9.3 Evaluate Operational Effect

The proposed configuration is evaluated against the existing hostel operational architecture and applicable configured constraints.

No new local business rule is created merely to justify the change.

### 9.4 Conduct Required Review

The proposal is reviewed by the configured Hostel Management authority.

Where the matter is outside G03's authority, it is routed to the appropriate upstream policy process rather than being reclassified locally.

### 9.5 Authorise the Change

Where the change is within G03's authority and satisfies the applicable review requirements, the authorised business decision permits the configuration to become effective.

### 9.6 Make Configuration Effective

The approved operational configuration becomes effective for the approved scope and effective period.

Subsequent hostel business scenarios apply the effective configuration.

### 9.7 Preserve Audit Evidence

The proposal, review, decision, scope, effective period and effective outcome remain historically traceable.

### 9.8 End Boundary

The scenario ends when the approved operational configuration is effective.

It does not execute every downstream residency or accommodation scenario affected by the new configuration.

## 10. Alternate Business Flows

### 10.1 Request Is Outside G03 Authority

If the requested change is actually an institution-wide policy change:

1. G03 does not activate it as local hostel configuration.
2. The request is routed to the authoritative upstream policy process.
3. G03 continues using the currently effective configuration until an authoritative outcome exists.

### 10.2 Approval Not Granted

If the configured authority does not approve the change:

1. The proposed configuration remains non-effective.
2. The existing effective configuration remains authoritative.
3. The decision remains auditable.

### 10.3 Effective Period Is Deferred

If the change is approved but its effective period is future-dated:

1. The current configuration remains effective until the approved effective point.
2. The future configuration becomes applicable only within its authorised effective period.
3. No premature operational effect is asserted.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Consequence | Business Resolution | Owner |
|---|---|---|---|
| Proposed change conflicts with authoritative upstream policy | Change cannot be safely activated | Resolve through architecture/policy governance | Upstream authority / G03 |
| Proposed scope exceeds G03 authority | Local activation would violate ownership | Route to authoritative policy owner | G03 |
| Required business review is incomplete | New configuration cannot become effective | Complete the review process | G03 |
| Approval is rejected | Existing configuration remains effective | Revise or close the proposal | G03 |
| Effective period is not established | Operational applicability is ambiguous | Establish an authorised effective period before activation | G03 |

No technical rollback, queue, retry or infrastructure failure handling is described.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Hostel operational constraints and policies are configuration driven. | G03 | Operational hostel behaviour may be configured within G03 authority. |
| Institution-wide business policy remains upstream. | Institution / B01 boundary | G03 cannot redefine institutional policy. |
| Platform configuration capability does not own business policy. | A / upstream policy boundary | A provides capability without becoming policy owner. |
| L3 applies upstream architecture; it does not redesign it. | Architecture governance | Scenario cannot create a local rule to resolve an upstream conflict. |
| Effective configuration must remain auditable. | G03 | Configuration decisions and applicability remain traceable. |

## 13. State and Lifecycle Impact

The authoritative material establishes configuration-driven hostel operations but does not expose a dedicated canonical configuration-change state machine.

Accordingly, no synthetic state names are introduced.

The business lifecycle represented by this scenario is:

`Proposed configuration → Authorised configuration → Effective configuration`

These are business conditions, not newly invented canonical aggregate state labels.

The scenario ends at effective configuration.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

The authoritative G03 material establishes event-driven architecture for significant operational transitions, but the exact canonical event name for effective configuration change is not exposed in the material retrieved for this authoring pass.

| Business Fact | Canonical Publisher | Consumer |
|---|---|---|
| Effective hostel operational configuration change | G03 | Authorised downstream hostel consumers |

No event name is fabricated.

### 14.2 Synchronous Business Contracts / Hand-offs

The frozen participating-domain set includes G and A.

The business boundary is:

- G03 owns the hostel operational configuration outcome.
- A may provide configuration capability.
- A does not become owner of hostel business policy.
- Institution-level policy decisions remain upstream.

No technical configuration contract is described at L3.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Hostel Management is the business authority for operational configuration within its scope.

An institution-wide policy change remains subject to the authoritative upstream policy authority.

### 15.2 Separation of Duties

No dedicated configuration-specific SoD rule is exposed in the authoritative material retrieved.

No additional SoD rule is invented.

### 15.3 Audit Evidence

The following business evidence must remain traceable:

- configuration proposal;
- intended scope;
- effective period;
- review;
- approval/rejection;
- effective outcome;
- applicable policy boundary.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Proposed configuration is identified.
- Ownership boundary is validated.
- Required review is completed.
- Required approval is obtained.
- Scope and effective period are established.
- Configuration becomes effective for its authorised scope.
- Configuration decision and effective outcome are auditable.

### 16.2 Postconditions

- The approved G03 operational configuration is effective.
- Subsequent hostel workflows consume the effective configuration.
- Existing historical business outcomes are not silently rewritten.
- Institution-wide policy ownership remains unchanged.
- Platform configuration capability does not acquire hostel policy ownership.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical configuration-change event name is not exposed in the retrieved authoritative material. | Event naming cannot safely be asserted. | Confirm the applicable canonical event catalogue before Architecture Approval/Lock. |
| A dedicated canonical configuration lifecycle state machine is not exposed. | Synthetic state names would be architectural invention. | Preserve the business progression without introducing new lifecycle states. |

### 17.2 Assumptions

- G03 operational configuration remains subordinate to authoritative institutional policy.
- Effective dates/scope are governed by the applicable upstream configuration model.

### 17.3 Deferred Questions

Exact configuration event/state names remain source-resolution items for the appropriate authoritative catalogue.
