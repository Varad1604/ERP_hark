# Curfew Exception to Authorised Late Entry

**Scenario ID:** L3-G-083  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
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
| Final L3 ID | L3-G-083 |
| Canonical Title | Curfew Exception to Authorised Late Entry |
| Primary Owner | Hostel Management |
| Participating Domains | G |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern an exception to an institution-defined hostel curfew so that a resident who cannot meet the normal return condition can receive an authorised late-entry outcome where hostel policy permits it.

## 3. Business Outcome

A resident's curfew exception is evaluated and, where authorised under hostel policy, the resident is permitted a governed late entry without treating the exception as an ungoverned curfew violation.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a resident requires an exception to the hostel's applicable curfew return condition.

### 4.2 End Boundary

The scenario ends when the curfew exception is authorised and the resident's late entry is accepted under the applicable hostel policy.

### 4.3 In Scope

- Identification of the curfew exception.
- Review against configured curfew/exception policy.
- Hostel authority decision.
- Recording the authorised late-entry outcome.
- Maintaining the distinction between authorised exception and disciplinary curfew violation.

### 4.4 Out of Scope

- General leave approval.
- Permanent residency closure.
- Disciplinary action for an unauthorised curfew violation.
- Visitor management.
- Student Lifecycle changes.
- Notification delivery mechanics.

### 4.5 Related Scenarios

- **L3-G-082 — Resident Leave Request to Verified Return** governs approved temporary leave and verified return.
- **L3-G-086 — Curfew Violation to Disciplinary Action** governs an unauthorised curfew violation and is a separate scenario.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Hostel Management (G) | Owns curfew operations, exception handling, and the authorised late-entry outcome. |

No cross-domain business ownership is transferred by the exception decision.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student / Resident | Requests or presents the business reason for a curfew exception and participates in the resulting late-entry process. | Initiates / participates. |
| Hostel Warden | Reviews and authorises the curfew exception under hostel governance. | Business authority for hostel governance. |
| Hostel Administrator | May perform the operational processing of the exception under configured hostel policy. | Operational responsibility where assigned. |

If a specific exception authority is not fixed by institution configuration, no additional role is invented.

## 7. Preconditions and Entry Criteria

- The resident is subject to an applicable hostel curfew.
- The resident has a valid active hostel residency.
- A business circumstance exists that requires consideration of a late-entry exception.
- The exception is within the scope of Hostel Management policy.

## 8. Trigger

A resident requires a late-entry exception to the applicable hostel curfew.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The resident presents the need for a curfew exception under the applicable hostel process. | Student / Resident | An exception request/condition is presented for hostel review. |
| 2 | Hostel Management assesses the circumstance against the institution's configured curfew and exception policy. | Hostel Administrator / Hostel Warden | The exception is determined to be eligible or not eligible for authorisation. |
| 3 | The authorised hostel authority approves the exception where policy permits. | Hostel Warden / configured hostel authority | The late-entry exception becomes an authorised hostel outcome. |
| 4 | The resident's late entry is accepted under the authorised exception. | Hostel Management | The resident is permitted to enter without the event being treated as an unauthorised curfew violation. |

**Scenario End:** The main flow stops when authorised late entry is established. Any later discipline or violation scenario is outside this boundary.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | The resident's circumstance satisfies the configured exception policy without requiring additional hostel escalation. | Apply the configured exception path and authorise late entry. | Authorised late entry. | Hostel Management |

No other scenario-specific alternate path is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| The requested exception does not satisfy applicable hostel policy. | Late entry cannot be authorised under the exception scenario. | Treat the matter according to the applicable hostel curfew/violation process rather than granting an unsupported exception. | Hostel Management |
| Required hostel authority has not approved the exception. | The exception cannot become an authorised late-entry outcome. | Keep the matter subject to the applicable hostel decision path; do not record it as authorised without the required decision. | Hostel Management |

No technical failure or recovery mechanism is introduced.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Curfew is an institution-defined operational return time for hostel residents. | Hostel Management | The exception is evaluated against the applicable curfew policy. | L2-G03 §7 |
| Curfew rules and operational constraints are configuration driven. | Hostel Management | Exception handling follows configured institutional policy. | L2-G03 §6, §33 |
| Hostel Warden is an operational authority responsible for hostel governance. | Hostel Management | Warden participation is supported for hostel exception governance. | L2-G03 §7, §31 |
| Unauthorised curfew violations are handled separately from an authorised exception. | Hostel Management | An authorised exception must not be represented as a curfew violation. | L2-G03 §§20, 22 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Curfew exception / hostel movement outcome | Applicable curfew condition | The resident's late return is evaluated and authorised. | Authorised late entry | G03 Hostel Management | L2-G03 §§6–7, §20 |

L2-G03 does not define a separate canonical state machine for a curfew-exception request in the material used for this scenario. No new lifecycle state is invented.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

No exact scenario-specific curfew-exception event name is asserted in the source material used for this draft. No event name is invented.

The authoritative G03 architecture nevertheless requires significant hostel operational transitions to be auditable and event driven.

### 14.2 Business Contracts and Cross-Domain Hand-offs

No cross-domain business contract is required for this G-only scenario.

The scenario remains wholly within Hostel Management's canonical operational boundary.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Present curfew exception | Student / Resident | Must relate to an applicable hostel curfew condition. | Hostel Management |
| Authorise late entry | Hostel Warden / configured hostel authority | Must be permitted by applicable hostel exception policy. | Hostel Management |

### 15.2 Separation of Duties, Overrides and Exceptions

The scenario itself represents an exception to the normal curfew condition. No additional separation-of-duty or override rule is defined by the authoritative L2 source.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Hostel operational exceptions must remain traceable. | Immutable hostel operational/audit history associated with the curfew exception and authorised outcome. | Hostel Management |

No unsupported evidence fields or technical audit mechanisms are defined.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The curfew exception has been evaluated under applicable hostel policy.
- The authorised hostel authority has approved the exception where approval is required.
- Late entry is accepted as an authorised exception.
- The resident is not treated as being in an unauthorised curfew-violation path for the authorised exception.

### 16.2 Postconditions

- The authorised late-entry outcome is part of the resident's hostel operational history.
- Any subsequent unrelated curfew violation or disciplinary matter is handled by its own scenario.
- Hostel Management remains the sole owner of the curfew-exception outcome.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The authoritative G03 material used for this draft does not expose a dedicated named lifecycle or canonical event name for a curfew-exception aggregate. | A new state or event cannot be safely invented at L3. The scenario therefore expresses the business outcome without asserting a new lifecycle or event identity. | Confirm whether G03 defines a dedicated curfew-exception state/event elsewhere in its locked catalogue before Architecture Approval/Lock. |

### 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Curfew exceptions are governed through the configured hostel policy and appropriate hostel authority. | G03 defines curfew as a Hostel Management responsibility and states that operational policies/approval workflows are configuration driven. | Exact institution-specific exception sequencing may differ. |

### 17.3 Deferred Questions

No scenario-specific deferred L4/L5/L6 question is required.
