# Laboratory Configuration Change to Effective Operational Policy

**Scenario ID:** L3-G-078  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Laboratory Management  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-G02 — Laboratory Management Workflows; Section A — Platform Foundation; locked L1 Business Domain Architecture; DEPENDENCY_MAP.md  
**Referenced By:** None yet  

## 2. Scenario Purpose

Establish a governed change to laboratory operational configuration so that an institution-defined laboratory rule or policy is available for subsequent operational use while preserving Laboratory Management ownership and the Platform Foundation's infrastructure responsibilities.

## 3. Business Outcome

The requested laboratory configuration change is accepted as an effective operational policy for the governed laboratory-management capability, with the change recorded in the domain's immutable operational history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authorized business configuration change to a Laboratory Management policy, rule, checklist, category, availability policy, safety policy, notification policy, or related configurable laboratory-management element is submitted for governance.

### 4.2 End Boundary

The scenario ends when the approved configuration takes effect for the governed laboratory-management scope and its change is auditable.

### 4.3 In Scope

- Review of a laboratory operational configuration change.
- Governance of the change under institution-configurable policy.
- Effective dating of the configuration.
- Recording the resulting change in immutable laboratory audit history.
- Making the accepted configuration available to subsequent Laboratory Management operations.

### 4.4 Out of Scope

- Creation or modification of laboratory infrastructure, capacity, or rooms, which remain owned by Infrastructure Management.
- Academic timetable changes, which remain owned by Section D.
- Maintenance execution, which remains owned by G01 Facilities Management.
- Technical authorization, persistence, deployment, or configuration implementation mechanics.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to establish this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Laboratory Management (G) | Owns laboratory operational configuration and the resulting effective operational policy within its domain. |
| Platform Foundation (A) | Provides the enterprise authorization, workflow, audit, event, and configuration infrastructure used by the business process; it does not own the laboratory business policy. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Laboratory Administrator | Initiates and manages laboratory configuration changes within the laboratory-management business scope. | Configuration responsibility supported by L2-G02. |

The authoritative L2 source does not define a more specific approval actor for this configuration-change scenario. No additional actor is invented here; the unresolved authority detail is recorded in Section 17.

## 7. Preconditions and Entry Criteria

- A governed Laboratory Management configuration element exists within the institution's operational scope.
- The requested change concerns a configuration element that L2-G02 identifies as configurable, including laboratory types, operational categories, shared laboratory rules, readiness policies, availability policies, session rules, safety controls, equipment operational-status rules, compliance categories, or notification policies.
- The change is submitted within the applicable institution and scope.

## 8. Trigger

A laboratory configuration change is submitted for governance and effective use.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The Laboratory Administrator identifies the laboratory-management configuration element requiring change and submits the proposed business change. | Laboratory Management | A governed configuration-change request exists. |
| 2 | The proposed change is reviewed against the applicable institution-configurable laboratory operational policy and the boundaries of Laboratory Management. | Laboratory Management | The requested change is determined to be within the owned laboratory operational scope or requires correction/escalation. |
| 3 | The change is assigned an effective date in accordance with the domain's effective-dated configuration principle. | Laboratory Management | The change has a governed point at which it becomes applicable. |
| 4 | The approved configuration becomes the effective operational policy for the governed Laboratory Management scope. | Laboratory Management | Subsequent laboratory operations use the effective configuration. |
| 5 | The configuration change is recorded in immutable operational audit history. | Laboratory Management with Platform Audit infrastructure | The business change is historically traceable. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | The proposed change is outside Laboratory Management ownership, such as infrastructure capacity or an academic timetable policy. | The request is not applied within Laboratory Management and is directed to the owning business domain. | Laboratory configuration remains unchanged. | Laboratory Management / owning domain |
| A2 | The applicable institution-configurable policy requires additional business review before effectiveness. | The change remains subject to the governing review rather than becoming effective immediately. | Existing effective policy remains in force until the governed change can take effect. | Laboratory Management |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Configuration request contains an invalid laboratory-management scope or conflicts with an ownership boundary. | The requested change cannot become effective. | Correct the request or route the change to the owning domain without changing the current effective policy. | Laboratory Management |
| Required business approval is not established. | The effective configuration cannot be authoritatively changed. | Hold the proposed change pending clarification of the applicable approval authority; preserve the existing effective policy. | Laboratory Management |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Laboratory operational policies and related controls are configuration driven. | Laboratory Management | Configuration changes are managed as governed business configuration rather than implementation customization. | L2-G02 §6, §32 |
| Configuration supports version control and effective dating. | Laboratory Management | The new configuration must be governed as an effective-dated change rather than an unbounded in-place business change. | L2-G02 §32 |
| Configuration changes require complete audit history. | Laboratory Management / Platform Audit infrastructure | The change must remain historically traceable. | L2-G02 §32, §33 |
| Laboratory Management does not modify infrastructure capacity or academic timetables. | Infrastructure Management / Academic Delivery | The scenario must not use configuration change as a means to alter those canonical domains. | L2-G02 §4, §14 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Laboratory operational configuration | Existing effective configuration | A governed, effective-dated configuration change is applied. | New effective configuration is available for operational use. | G02 Laboratory Management | L2-G02 §32 |

L2-G02 does not define a dedicated lifecycle state machine for laboratory configuration itself; this scenario therefore does not invent one.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

No exact canonical business event for laboratory configuration change is defined in L2-G02. No event name is invented in this scenario.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Laboratory Management | Platform Foundation | Enterprise authorization, workflow, audit, configuration, and event infrastructure usage | Support governed business execution while retaining laboratory policy ownership in G02. | L2-G02 §9, §30, §32–§33 |

No additional synchronous business contract is established specifically for this scenario by the authoritative L2 source.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate laboratory configuration change | Laboratory Administrator | Configuration responsibility is supported by L2-G02. | Laboratory Management |
| Approve configuration change for effectiveness | Not explicitly defined in authoritative L2-G02 | Approval authority is a material upstream gap for this specific scenario. | Laboratory Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule or override authority for configuration changes is explicitly defined in L2-G02.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Configuration change must remain historically auditable. | Immutable audit history for the configuration change, including the business action and resulting state change. | Laboratory Management, with Section A Audit infrastructure |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The proposed configuration change is within Laboratory Management's canonical business scope.
- The change has an established effective date under the governing configuration policy.
- The resulting configuration is effective for the intended laboratory-management scope.
- The change is recorded in immutable audit history.

### 16.2 Postconditions

- The new effective configuration governs subsequent applicable Laboratory Management operations.
- The previous configuration remains historically traceable through the effective-dated configuration history.
- No infrastructure, academic timetable, student lifecycle, workforce, or maintenance ownership has been transferred to Laboratory Management by the change.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G02 defines configuration as effective-dated and auditable but does not define a dedicated configuration-change lifecycle or an explicit approval role for this scenario. | The scenario can express the business outcome and boundaries, but the canonical approval authority and lifecycle semantics cannot be asserted without inventing architecture. | Confirm the canonical business approval authority and, if required, the authoritative configuration-change lifecycle upstream. |

### 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| The institution uses an existing governed configuration-review path before an effective-dated change becomes authoritative. | L2-G02 requires configurable governance, authorization, effective dating, and complete audit history. | The precise approval path would differ and must be clarified upstream. |

### 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| How configuration is technically stored, versioned, published, or evaluated is deferred. | L5 | Implementation architecture is outside L3. |

