# Hostel Discipline Incident to Disciplinary Resolution

**Scenario ID:** L3-G-085  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Govern a hostel discipline incident from recording through review and an authorised disciplinary resolution while preserving Hostel Management ownership of hostel conduct records and Student Lifecycle ownership of student status.

## 3. Business Outcome

A hostel discipline incident is reviewed through the applicable institutional hostel disciplinary process and reaches a governed resolution that is recorded in the resident's disciplinary history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a hostel conduct incident is recorded against a resident.

### 4.2 End Boundary

The scenario ends when the Hostel Discipline process reaches its authorised resolution and the disciplinary history is recorded.

### 4.3 In Scope

- Incident recording and classification.
- Review and evidence consideration.
- Hostel disciplinary decision.
- Warning or other configured hostel disciplinary action.
- Recording the final resolution.
- Appropriate cross-domain communication to Student Lifecycle, workforce participants, and Notifications where defined by the governing architecture.

### 4.4 Out of Scope

- Student Lifecycle status transition unless a separate canonical C02 workflow is invoked.
- Employee/workforce lifecycle changes.
- Notification delivery mechanics.
- Legal proceedings or external disciplinary bodies.
- Technical evidence storage or implementation mechanics.

### 4.5 Related Scenarios

- Curfew Violation to Disciplinary Action is a separate canonical scenario.
- Student Lifecycle status changes remain owned by Section C.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Hostel Management (G) | Owns Hostel Discipline incident, warning, action, and disciplinary history. |
| Student Lifecycle (C) | Owns canonical student identity/status and receives hostel disciplinary facts where architecture requires downstream consumption. |
| People & Workforce (E) | Provides workforce/personnel context where hostel staff/authority participate; does not own the student discipline outcome. |
| Notifications (H) | Provides governed communication capability; does not own the disciplinary decision. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Resident / Student | Is the subject of the hostel incident and participates in the applicable review process. |
| Hostel Administrator | Records and manages operational disciplinary processing. |
| Hostel Warden | Provides hostel governance and disciplinary authority according to configured hostel policy. |
| Relevant Hostel Staff | May provide incident information or participate where institution policy assigns responsibility. |

No new disciplinary authority is invented beyond the configurable hostel governance model.

## 7. Preconditions and Entry Criteria

- The subject is an active hostel resident or otherwise within Hostel Management's disciplinary scope.
- A conduct incident has sufficient information to be recorded.
- The incident falls within the hostel discipline policy.

## 8. Trigger

A hostel discipline incident is recorded for a resident.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Result |
|---|---|---|---|
| 1 | The incident is recorded and associated with the relevant resident and hostel context. | Hostel Management | Hostel Discipline incident exists. |
| 2 | Hostel Management reviews the incident and available business evidence under configured discipline policy. | Hostel Administrator / Hostel Warden | The incident reaches a governed review position. |
| 3 | The resident is considered under the applicable institutional disciplinary process. | Hostel Management | A disciplinary decision basis is established. |
| 4 | The authorised hostel authority determines the applicable disciplinary action or closure. | Hostel Management | A governed disciplinary outcome is reached. |
| 5 | The action and resolution are recorded in disciplinary history. | Hostel Management | The incident reaches its resolved/closed business position. |
| 6 | Where the architecture requires downstream consumption, the authoritative hostel disciplinary outcome is made available to Student Lifecycle, Notifications, or other approved consumers. | Hostel Management | Cross-domain consumers receive the hostel-owned fact without ownership transfer. |

**Scenario End:** The scenario ends at the governed hostel disciplinary resolution. It does not absorb any separate Student Lifecycle status change.

## 10. Alternate Business Flows

| Condition | Business Path | Outcome |
|---|---|---|
| Incident does not substantiate a disciplinary action. | Close the incident through the applicable hostel review outcome. | No unsupported disciplinary action is imposed. |
| Configured policy requires warning or another corrective action. | Record the authorised hostel disciplinary action. | Disciplinary history reflects the action. |
| Matter requires a separate student-lifecycle decision. | Preserve Hostel Management's incident outcome and hand off the authoritative business fact to the Student Lifecycle domain. | C02 decides any student-lifecycle consequence under its own authority. |

## 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Business Resolution | Owner |
|---|---|---|---|
| Incident evidence is insufficient for a decision. | Disciplinary resolution cannot be concluded authoritatively. | Obtain the required business evidence through the applicable hostel review process. | Hostel Management |
| Required hostel authority has not completed the decision. | Incident remains unresolved. | Continue through the configured disciplinary decision path. | Hostel Management |
| A potential student-lifecycle consequence is identified. | Hostel Management cannot directly change Student Lifecycle state. | Publish/hand off the hostel-owned disciplinary outcome through the canonical cross-domain contract; C02 determines its own lifecycle action. | Hostel Management / Student Lifecycle |

No technical retry, rollback, queue, or exception mechanics are introduced.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect | Source |
|---|---|---|---|
| Hostel Discipline owns incident, warning, action, and disciplinary history. | G03 | Disciplinary ownership remains with Hostel Management. | L2-G03 Aggregate Design |
| Discipline policies are configuration driven. | G03 | Institutional policy governs the disciplinary decision. | L2-G03 Design Principles / Configuration |
| Student Lifecycle owns Student identity and lifecycle. | C02 | G03 cannot directly change student lifecycle state. | L2-G03 Scope / C02 Ownership |
| Workforce Management owns workforce engagement/personnel information. | E01 | Workforce participation does not transfer ownership of hostel discipline. | L2-E01 |
| Notifications owns notification orchestration. | H02 | Notifications consumes the business outcome rather than deciding it. | L2-H02 |

## 13. State and Lifecycle Impact

The authoritative G03 source identifies Hostel Discipline as an aggregate owning incident, warning, action, and disciplinary history, but the material available for this authoring pass does not expose a dedicated canonical state-transition matrix for the discipline aggregate.

Therefore, this scenario does **not** invent new discipline lifecycle state names.

| Business Aggregate | Impact | Owner |
|---|---|---|
| Hostel Discipline | Incident is reviewed and reaches an authorised disciplinary resolution recorded in history. | G03 |
| StudentContext | No automatic C02 lifecycle transition is asserted by this scenario. | C02 |
| Workforce Engagement | Personnel context may be consumed where applicable; no E01 lifecycle transition is asserted. | E01 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03 requires significant hostel transitions to publish immutable domain events. The exact canonical discipline event names are not exposed in the authoritative material available for this draft.

| Business Fact | Publisher | Consumers |
|---|---|---|
| Authoritative hostel disciplinary outcome | G03 Hostel Management | Student Lifecycle, Notifications, and other approved consumers where contractually applicable |

No unsupported event name is invented.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| Student Lifecycle | Hostel Management | Canonical Student Identity / Student Context | Establish subject identity without transferring ownership. |
| Workforce Management | Hostel Management | Workforce/personnel context for participating hostel staff where required | Provide authoritative workforce context without transferring E ownership. |
| Hostel Management | Student Lifecycle | Hostel disciplinary business outcome | Allow C02 to determine any separate student-lifecycle consequence. |
| Hostel Management | Notifications | Authoritative disciplinary outcome/event | Support governed communication. |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

| Decision | Authority | Constraint |
|---|---|---|
| Record incident | Hostel Administrator / designated hostel operational role | Must be within hostel discipline scope. |
| Determine disciplinary resolution | Hostel Warden / configured hostel authority | Must follow institution-configured discipline policy. |
| Student lifecycle consequence | Student Lifecycle authority | G03 cannot decide or mutate C02 lifecycle state. |

### 15.2 Separation of Duties / Overrides

The supplied G03 source establishes configurable approval/authority but does not expose a dedicated discipline-specific separation-of-duty rule. None is invented.

### 15.3 Audit Evidence

The incident, review, authorised action, and resolution must remain in immutable hostel disciplinary history.

Any Student Lifecycle consequence must be evidenced within C02's own authoritative lifecycle records.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The incident is recorded against the appropriate resident/context.
- Applicable hostel review is completed.
- An authorised disciplinary resolution or non-disciplinary closure is reached.
- The outcome is recorded in Hostel Discipline history.
- Required downstream business facts are made available without transferring ownership.

### 16.2 Postconditions

- Hostel Management has a completed disciplinary history entry.
- Any Student Lifecycle consequence remains separately governed by C02.
- Workforce and notification participation remains consumption/collaboration rather than ownership transfer.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| The authoritative material available for this pass does not expose the exact discipline state machine or canonical discipline event names. | New states/events cannot be safely invented. | Confirm exact G03 discipline lifecycle and event catalogue before Architecture Approval/Lock. |

### 17.2 Assumptions

The configured Hostel Management discipline policy identifies the applicable hostel authority and review path.

### 17.3 Deferred Questions

Any technical evidence-management and notification-delivery behaviour remains outside L3.
