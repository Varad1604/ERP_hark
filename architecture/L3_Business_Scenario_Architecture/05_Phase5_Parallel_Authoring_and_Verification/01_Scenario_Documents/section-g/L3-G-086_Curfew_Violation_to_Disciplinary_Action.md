# Curfew Violation to Disciplinary Action

**Scenario ID:** L3-G-086  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, H  
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

Govern an unauthorised curfew violation from identification through the applicable hostel disciplinary action while preserving the distinction between an authorised curfew exception and a violation.

## 3. Business Outcome

A resident's unauthorised curfew violation is reviewed under hostel policy and reaches an authorised disciplinary outcome that is recorded in Hostel Management history.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a resident is identified as having violated the applicable hostel curfew without an authorised exception.

### 4.2 End Boundary

The scenario ends when the hostel disciplinary action or governed closure for the curfew violation is recorded.

### 4.3 In Scope

- Identification/recording of the curfew violation.
- Review against the applicable curfew and discipline policy.
- Hostel disciplinary decision.
- Recording the resulting action or closure.
- Communication of the authoritative hostel outcome to approved downstream consumers where required.

### 4.4 Out of Scope

- Authorised curfew exceptions.
- General leave approval.
- Permanent residency closure.
- Direct Student Lifecycle status mutation.
- Technical notification delivery, APIs, persistence, or implementation.

### 4.5 Related Scenarios

- **L3-G-084 — Curfew Exception to Authorised Late Entry** is the separate authorised-exception scenario.
- **L3-G-085 — Hostel Discipline Incident to Disciplinary Resolution** is the broader hostel discipline scenario.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Hostel Management (G) | Owns curfew operations, Hostel Discipline, disciplinary action, and disciplinary history. |
| Student Lifecycle (C) | Owns student identity and lifecycle; may consume the hostel outcome without receiving ownership of the violation. |
| Notifications (H) | Provides notification orchestration for authorised business communications. |

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Resident / Student | Subject of the curfew violation and participant in the applicable review. |
| Hostel Administrator | Records and manages the operational violation process. |
| Hostel Warden | Provides hostel governance and disciplinary authority under configured policy. |

No additional actor or disciplinary authority is invented.

## 7. Preconditions and Entry Criteria

- The resident has an active hostel residency.
- An applicable hostel curfew exists.
- The resident's late return is not covered by an authorised exception.
- The violation falls within Hostel Management's curfew/discipline scope.

## 8. Trigger

An unauthorised curfew violation is identified for a resident.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Result |
|---|---|---|---|
| 1 | Hostel Management records the curfew violation against the resident and applicable curfew condition. | Hostel Administrator / Hostel Management | A curfew violation incident is established. |
| 2 | The incident is reviewed against the applicable curfew and disciplinary policy, including whether a valid exception existed. | Hostel Management | The violation is confirmed or otherwise resolved. |
| 3 | Where the violation is confirmed, the authorised hostel authority determines the applicable disciplinary action. | Hostel Warden / configured hostel authority | A governed disciplinary decision is reached. |
| 4 | Hostel Management records the disciplinary action and preserves the history. | Hostel Management | The curfew violation reaches its governed outcome. |
| 5 | Where required, the authoritative hostel outcome is made available to Student Lifecycle and Notifications through approved cross-domain collaboration. | Hostel Management | Downstream consumers receive the hostel-owned business fact. |

**Scenario End:** The scenario ends at the recorded disciplinary outcome. Any separate Student Lifecycle consequence is governed by C02.

## 10. Alternate Business Flows

| Condition | Business Path | Outcome |
|---|---|---|
| A valid authorised curfew exception is established. | Do not classify the event as an unauthorised violation. | Route to the authorised exception outcome instead. |
| Violation is not substantiated. | Close the incident under the applicable hostel review outcome. | No unsupported disciplinary action. |

## 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Business Resolution | Owner |
|---|---|---|---|
| Evidence does not establish an unauthorised curfew violation. | Disciplinary action cannot be justified. | Resolve the incident through the applicable review process. | Hostel Management |
| Required hostel disciplinary authority has not decided. | Outcome remains pending. | Continue through the configured disciplinary decision path. | Hostel Management |
| Potential student-lifecycle consequence is identified. | G03 cannot directly alter C02 lifecycle state. | Make the hostel-owned outcome available to C02; C02 determines any separate lifecycle action. | Hostel Management / Student Lifecycle |

No technical recovery mechanics are introduced.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect | Source |
|---|---|---|---|
| Curfew is an institution-defined operational return time. | Hostel Management | Violation is measured against applicable curfew policy. | L2-G03 Terminology |
| Curfew rules and operational constraints are configuration driven. | Hostel Management | Institution policy governs violation determination. | L2-G03 Design Principles |
| Authorised exceptions are distinct from violations. | Hostel Management | A valid exception prevents classification as an unauthorised curfew violation. | L2-G03 Curfew/exception architecture |
| Hostel Discipline owns incidents, warnings, actions, and disciplinary history. | Hostel Management | Disciplinary resolution remains within G03. | L2-G03 Aggregate Design |
| Student Lifecycle owns Student identity and lifecycle. | Student Lifecycle | Any student-lifecycle consequence remains C02-owned. | L2-G03 / L2-C02 |

## 13. State and Lifecycle Impact

The supplied G03 material does not expose a dedicated canonical state-transition matrix specifically for the curfew-violation aggregate.

| Business Aggregate | Impact | Owner |
|---|---|---|
| Hostel Discipline / curfew violation record | Violation is reviewed and reaches a governed disciplinary outcome. | G03 |
| StudentContext | No automatic lifecycle transition is asserted. | C02 |

No new curfew-violation lifecycle states are invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

G03 establishes immutable events for significant hostel operational transitions, but the exact canonical event name for curfew violation/disciplinary action is not exposed in the authoritative material available for this draft.

| Business Fact | Publisher | Consumers |
|---|---|---|
| Authoritative curfew-violation / disciplinary outcome | G03 Hostel Management | Student Lifecycle, Notifications, and other approved consumers where applicable |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| Student Lifecycle | Hostel Management | Canonical Student Identity / Student Context | Identify the resident without transferring ownership. |
| Hostel Management | Student Lifecycle | Authoritative hostel disciplinary outcome | Allow C02 to determine any separate lifecycle consequence. |
| Hostel Management | Notifications | Hostel violation/disciplinary outcome | Support governed communication without transferring decision ownership. |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

| Decision | Authority | Constraint |
|---|---|---|
| Record curfew violation | Hostel Administrator / designated hostel operational role | Must be within hostel curfew scope. |
| Determine disciplinary action | Hostel Warden / configured hostel authority | Must follow configured hostel discipline policy. |
| Any Student Lifecycle consequence | Student Lifecycle authority | C02 remains sole owner of student lifecycle state. |

### 15.2 Separation of Duties / Overrides

No curfew-violation-specific separation-of-duty rule is exposed in the supplied G03 material. No additional rule is invented.

### 15.3 Audit Evidence

The curfew violation, review, decision, action, and closure must remain in immutable Hostel Management operational/disciplinary history.

Any Student Lifecycle action remains evidenced in C02.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The resident and curfew condition are identified.
- Any claimed exception is considered.
- The violation is confirmed or closed without unsupported disciplinary action.
- If confirmed, the authorised disciplinary action is recorded.
- The hostel outcome remains historically traceable.
- Required downstream consumers can consume the authoritative hostel outcome.

### 16.2 Postconditions

- The curfew violation has a governed hostel outcome.
- A valid authorised exception is not misclassified as a violation.
- Student Lifecycle remains unchanged unless C02 independently determines a separate lifecycle action.
- Notification participation does not transfer ownership from G03.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Impact | Resolution |
|---|---|---|
| Exact canonical curfew-violation/disciplinary event names and a dedicated curfew-violation lifecycle are not exposed in the authoritative material available for this draft. | Inventing states or event names would violate the L3 source discipline. | Confirm the canonical G03 event/lifecycle definitions before Architecture Approval/Lock. |

### 17.2 Assumptions

The institution's configured hostel policy distinguishes an authorised curfew exception from an unauthorised violation and identifies the applicable hostel disciplinary authority.

### 17.3 Deferred Questions

Technical notification delivery and implementation mechanics remain outside L3.
