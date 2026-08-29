# Student Communication Preference Update

**Scenario ID:** L3-C-015  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L2-C02 — Student Lifecycle & Activation Workflows, especially `WF-C02-14 — Student Communication Preference Management Workflow`  

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-015` |
| Canonical Title | Student Communication Preference Update |
| Primary Owner | Student Lifecycle |
| Participating Domains | C |
| Scenario Type | Simple |
| Priority | P2 |
| Status | Locked |
| Applicable L2 Workflow | `WF-C02-14 — Student Communication Preference Management Workflow` |

Frozen identity fields are preserved without reclassification.

## 2. Scenario Purpose

Govern the controlled update of Student-owned communication preferences while preserving Student Lifecycle ownership, institutional policy, auditability, and the boundary between preference ownership and downstream communication delivery.

## 3. Business Outcome

The Student's permitted communication preferences are updated in the authoritative Student Aggregate, applicable policy and approval conditions are satisfied, the change is auditable, and `StudentCommunicationPreferencesUpdated` is published by Student Lifecycle.

The scenario does not itself deliver a communication.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A Student or institutionally permitted Registrar context requests a change to Student-owned communication preferences.

### 4.2 End Boundary
The scenario ends when the permitted preferences are updated, any required approval is complete, audit evidence is established, and the canonical update event is published.

### 4.3 In Scope
- Preferred language.
- Notification-channel preferences.
- Communication consent/preferences managed by the Student Aggregate.
- Business validation of the requested preference values.
- Institutional-policy validation.
- Approval where policy requires it.
- Auditability.
- Publication of `StudentCommunicationPreferencesUpdated`.

### 4.4 Out of Scope
- Message delivery or orchestration.
- Notification templates.
- Provider/channel technical execution.
- Student identity correction.
- General profile changes unrelated to communication preferences.
- APIs, persistence design, queues, read models, UI, deployment, or infrastructure.

### 4.5 Related Scenarios
| Relationship | Scenario |
|---|---|
| Related | L3-C-014 — Student Profile Update |
| Distinct boundary | L3-C-016 — Student Identity Correction |

The frozen canonical set keeps C014, C015, and C016 as separate outcomes.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| C — Student Lifecycle | Canonical owner of the Student Aggregate and Student communication preferences; validates and applies the preference change and publishes the canonical outcome. |

Locked L2-C02 explicitly places Communication Preferences inside the Student Aggregate. Downstream notification/communication capabilities may consume the result without acquiring ownership of Student preference data.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority |
|---|---|---|
| Student | Maintains permitted communication preferences. | May initiate/update preferences within institutional policy. |
| Registrar | Modifies or governs preferences where institutional policy permits or requires administrative intervention. | Administrative authority defined by locked C02 policy boundary. |
| Student Lifecycle | Validates and applies the change and preserves canonical Student ownership. | Canonical business owner. |

Technical/supporting participants exposed by L2-C02 are not modeled as L3 business actors.

## 7. Preconditions and Entry Criteria

- Student exists.
- Student belongs to the applicable institutional context.
- Requested communication-preference values are supported by institutional configuration/policy.
- Caller/business context is authorized according to institutional policy.
- Any approval requirement applicable to the requested change can be evaluated.
- The request does not attempt to alter immutable Student identity or another separately owned concept.

## 8. Trigger

A permitted request is made to create or update the Student's communication preferences.

The trigger is expressed at business level; L3 does not model the portal, command, API, or other technical initiation mechanisms described by lower-level implementation concerns.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Result |
|---|---|---|---|
| 1 | Receive the communication-preference change request. | Student Lifecycle | Request enters governed evaluation. |
| 2 | Validate the Student and the requested preference scope. | Student Lifecycle | Request is within Student-owned preference boundary. |
| 3 | Validate requested preferences against institutional policy/configuration. | Student Lifecycle | Business-valid preferences established. |
| 4 | Determine whether institutional approval is required. | Student Lifecycle | Approval path established. |
| 5 | Where required, obtain the applicable approval. | Registrar / applicable governed authority | Approval satisfied or request rejected. |
| 6 | Update the permitted communication preferences. | Student Lifecycle | Authoritative preference state changes. |
| 7 | Record the completed change as auditable business history. | Student Lifecycle | Traceability preserved. |
| 8 | Publish `StudentCommunicationPreferencesUpdated`. | Student Lifecycle | Downstream consumers may synchronize their own behavior. |
| 9 | Complete the scenario. | Student Lifecycle | C015 outcome achieved. |

## 10. Alternate Business Flows

| Condition | Business Path | Result |
|---|---|---|
| No approval required | Complete validation and update directly. | Preference update completed. |
| Approval required | Complete the governed approval before applying the change. | Approved update completed or rejected. |
| Multiple permitted preferences changed together | Validate the combined request under the same policy boundary. | One authoritative updated preference outcome. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Student does not exist | Do not change preferences. |
| Unsupported communication channel/preference is requested | Reject the unsupported value and preserve the existing preference state. |
| Institutional policy validation fails | Do not complete the change until a valid request is established. |
| Required approval is rejected | Preserve existing preferences; no update event is published. |
| Request attempts to modify Student identity or unrelated data | Route the matter to the appropriate canonical scenario; C015 does not perform the change. |

No technical retry/rollback/infrastructure recovery is introduced.

## 12. Business Rules and Decision Constraints

- Student Lifecycle owns the Student Aggregate and Communication Preferences component.
- Communication preferences may be changed only within institutional policy/configuration.
- Approval is conditional where policy requires it; L3 does not impose a universal approval requirement.
- Student identity remains unchanged.
- Completed preference changes must be auditable.
- Communication delivery remains outside Student Lifecycle ownership.

These constraints apply locked L2-C02 semantics and are not new L3-owned rules.

## 13. State and Lifecycle Impact

| Concept | Scenario Impact | Owner |
|---|---|---|
| Student Aggregate | Communication Preferences component is updated. | Student Lifecycle |
| Student lifecycle state | No lifecycle transition is inherently caused by the preference update. | Student Lifecycle |
| Student identity | Unchanged. | Student Lifecycle |

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

| Business Event | Canonical Publisher | Purpose |
|---|---|---|
| `StudentCommunicationPreferencesUpdated` | Student Lifecycle | Announces successful governed modification of Student communication preferences. |

Locked L2-C02 `WF-C02-14` and its event catalogue explicitly define this event. Downstream notification/communication capabilities may consume it, but they do not become owners of Student preference state.

## 15. Business Authorization, Approval and Audit Obligations

- Student may maintain permitted preferences.
- Registrar may modify/govern preferences where institutional policy permits.
- Approval occurs where policy requires it; no universal approval chain is invented.
- Every completed modification must remain auditable.
- L3 does not define technical IAM/RBAC controls.

## 16. Completion Criteria and Postconditions

C015 is complete when:

- the Student and requested preference scope are valid;
- institutional policy/configuration is satisfied;
- required approval, if any, is complete;
- permitted preferences are updated;
- Student identity and lifecycle ownership remain unchanged;
- audit evidence exists; and
- `StudentCommunicationPreferencesUpdated` is published by Student Lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified for the C015 actor, workflow, ownership, or event semantics after direct reconciliation with locked L2-C02 `WF-C02-14` and the C02 event catalogue.

### 17.2 Assumptions

Institution-specific permitted values and approval conditions remain governed by upstream configuration/policy; this L3 scenario does not hard-code them.

### 17.3 Deferred Questions

Technical delivery synchronization and implementation mechanics remain outside L3.

## Production Verification Record

- Frozen identity preserved.
- Direct locked L2-C02 source verification completed.
- Student and Registrar actor roles aligned to `WF-C02-14`.
- `StudentCommunicationPreferencesUpdated` restored as the exact canonical event.
- Student Lifecycle publisher/ownership preserved.
- No implementation mechanics imported from L2 technical wording.
- All 17 mandatory sections present.
- Status remains **Draft** pending architecture review.
