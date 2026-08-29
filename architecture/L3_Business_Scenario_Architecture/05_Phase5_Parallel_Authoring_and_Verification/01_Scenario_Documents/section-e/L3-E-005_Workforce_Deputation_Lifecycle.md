# Workforce Deputation Lifecycle

**Scenario ID:** L3-E-005  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Workforce & Engagement  
**Participating Domains:** E  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata
Frozen Section E identity is preserved. The authoritative workflow is L2-E01 WF-009 / WF-010. E02 remains authoritative for Position Assignment lifecycle changes.

## 2. Scenario Purpose
Temporarily assign a workforce member to an alternate institutional assignment while preserving the existing Workforce Engagement and returning the member to the normal organisational assignment when deputation ends.

## 3. Business Outcome
A Deputation Assignment is initiated, the existing operational assignment is temporarily suspended as required, employment continuity is preserved, and on completion the deputation ends and normal organisational assignment is restored.

## 4. Scenario Boundary and Scope
### 4.1 Start Boundary
An authorised deputation decision exists for an active workforce member.

### 4.2 End Boundary
The deputation is completed and the normal organisational assignment is restored.

### 4.3 In Scope
- Deputation initiation.
- Temporary alternate assignment.
- Preservation of Workforce Engagement continuity.
- Deputation completion.
- Restoration of normal assignment.
- Canonical deputation events.
- Coordination with E02 for assignment-state changes.

### 4.4 Out of Scope
- Creating a new Workforce Engagement.
- Permanent transfer.
- Position Definition ownership.
- Position Assignment lifecycle ownership.
- Responsibility Assignment ownership.
- Payroll or leave accounting.

### 4.5 Related Scenarios
E003 Workforce Transfer Between Organisational Contexts; E004 Workforce Engagement Suspension and Operational Restoration; E006 Workforce Long-Leave Lifecycle and Operational Availability.

## 5. Ownership and Participating Domains
Workforce & Engagement owns the deputation business lifecycle and preserves Workforce Engagement continuity. L2-E01 states that WF-009 creates a Deputation Assignment while temporarily suspending the existing Position Assignment, and WF-010 closes the Deputation Assignment and restores the previous Position Assignment. E02 performs the Position Assignment lifecycle.

## 6. Business Actors, Roles and Responsibilities
- Workforce & Engagement owns the deputation workflow.
- The authorised institutional authority establishes the deputation decision under policy/workflow governance.
- Position & Assignment (E02) owns Position Assignment creation, suspension, closure and restoration.
- No technical role names are invented.

## 7. Preconditions and Entry Criteria
- Active Workforce Engagement exists.
- Authorised deputation decision exists.
- Existing Position Assignment is authoritative.
- Alternate deputation assignment/context is valid.
- Applicable policy/workflow authorization is complete.

## 8. Trigger
An authorised deputation decision becomes effective.

## 9. Main Business Flow
1. Identify the Workforce Engagement and existing organisational assignment.
2. Validate deputation eligibility and authorization.
3. Execute **WF-009 — Initiate Deputation**.
4. Coordinate suspension of the current Position Assignment through E02.
5. Establish/activate the Deputation Assignment.
6. Preserve the Workforce Engagement unchanged.
7. Publish **`WorkforceDeputationStarted`**.
8. When deputation ends, execute **WF-010 — Complete Deputation**.
9. Close the Deputation Assignment through the authoritative assignment lifecycle.
10. Restore the previous Position Assignment through E02.
11. Publish **`WorkforceDeputationEnded`**.
12. Preserve the full historical sequence.

## 10. Alternate Business Flows
- If deputation is not authorised, no deputation state is established.
- If the alternate assignment cannot be established, the existing authoritative assignment remains in force.
- If deputation completion is not yet due/authorised, the Deputation Assignment remains active.
- A permanent organisational move is handled by the transfer workflow, not by deputation.

## 11. Failure, Interruption and Business Recovery
A deputation transition that cannot validly complete leaves the last authoritative business position unchanged. Any assignment inconsistency is resolved through E02's authoritative assignment lifecycle. Workforce Engagement continuity is not broken.

## 12. Business Rules and Decision Constraints
- Deputation preserves employment/Workforce Engagement continuity.
- Deputation temporarily suspends the existing Position Assignment.
- Completion restores the previous Position Assignment.
- Assignment history is preserved and owned by E02.
- Successful workflows publish completed business facts.
- Policy and workflow governance control authorization.

## 13. State and Lifecycle Impact
**Workforce Engagement:** remains in its existing lifecycle.

**Operational/assignment context:** current Position Assignment is temporarily suspended; Deputation Assignment is active; completion ends deputation and restores the previous assignment.

The assignment lifecycle itself is E02-owned.

## 14. Business Events and Cross-Domain Collaboration
| Event | Publisher | Meaning |
|---|---|---|
| `WorkforceDeputationStarted` | Workforce & Engagement | Deputation has begun |
| `WorkforceDeputationEnded` | Workforce & Engagement | Deputation has completed |

These exact events are defined by L2-E01 WF-009 and WF-010.

## 15. Business Authorization, Approval and Audit Obligations
Deputation follows Policy/Workflow governance. Initiation and completion must be auditable with the applicable business authorization and effective dates. E02 maintains the assignment lifecycle evidence for assignment-state changes.

## 16. Completion Criteria and Postconditions
- Deputation start: alternate Deputation Assignment is active, normal assignment is appropriately suspended, and `WorkforceDeputationStarted` is published.
- Deputation completion: Deputation Assignment is closed, previous assignment restored, and `WorkforceDeputationEnded` is published.
- Workforce Engagement continuity is preserved.

## 17. Open Issues, Assumptions and Deferred Questions
No material source gap identified for the core deputation lifecycle in the retrieved L2-E01 material.

**Authoring classification: PASS — source-grounded.**
