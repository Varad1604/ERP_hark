# L4-E-068 — Employee Separation to Governed Profile Archive

**Scenario ID:** L4-E-068  
**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** L3-E-068 — Employee Separation to Governed Profile Archive  
**Primary Owner:** Employee Profile  
**Participating Domains:** E  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 1. System Behavior Specification Metadata

This specification derives from locked L3-E-068 and preserves Workforce & Engagement ownership of separation, Employee Profile ownership of profile archival treatment, and the parent's unresolved lifecycle/event vocabulary.

## 2. System Behavior Purpose

Specify observable system behavior for applying the governed EmployeeProfile archival consequence of an authoritative employee separation.

## 3. System Behavior Outcome

After an authoritative workforce separation, EmployeeProfile receives only its governed archival consequence while historical profile information remains preserved.

## 4. System Behavior Scope and Boundary

### 4.1 Start Boundary
An authoritative separation outcome reaches the Employee Profile boundary.

### 4.2 End Boundary
The parent-defined profile archival consequence is authoritative and historical profile information remains preserved.

### 4.3 In Scope
- Recognize only an authoritative workforce separation outcome.
- Apply the EmployeeProfile archival consequence defined upstream.
- Preserve historical EmployeeProfile information.
- Do not approve, reverse, or modify the workforce separation.
- Publish only the applicable canonical profile lifecycle fact.

### 4.4 Out of Scope
- Resignation or separation approval.
- Payroll settlement, exit-clearance ownership, or workforce lifecycle modification.
- Any unresolved state or event name not established upstream.
- APIs, databases, services, queues, retries, rollback, infrastructure, technical access control, and UI design.

## 5. Ownership and Participating Domains

Workforce & Engagement remains the canonical owner of employee separation. Employee Profile owns EmployeeProfile lifecycle and archival treatment.

## 6. Business Actors, Roles and Responsibilities

Use only actors and authorities established by L3-E-068 and authoritative upstream sources. The system recognizes the separation outcome; it does not approve or reverse separation.

## 7. Preconditions and Entry Criteria

- EmployeeProfile exists.
- Workforce & Engagement has completed the authoritative separation outcome.
- The separation applies to the employee represented by the profile.

## 8. Trigger

A canonical employee separation outcome becomes available to Employee Profile.

## 9. Main System Behavior

1. The system recognizes the authoritative separation fact and preserves Workforce & Engagement ownership.
2. The system identifies the corresponding EmployeeProfile.
3. The system prevents archival action when the separation fact is incomplete or cannot be associated with the profile.
4. The system applies only the upstream-defined profile archival consequence.
5. The system preserves historical profile information.
6. The system publishes the applicable canonical Employee Profile lifecycle fact.

## 10. Alternate System Behaviors

### 10.1 Profile cannot be matched to the separation
The system does not apply the archival consequence until the authoritative relationship is resolved.

### 10.2 A later workforce rejoin occurs
The system follows the separately governed rejoin/profile consequence rather than independently reversing the archived position.

## 11. Failure, Interruption and Business Recovery

Where the separation outcome is incomplete or cannot be associated authoritatively with the EmployeeProfile, the system prevents the unsupported archival result and preserves the last authoritative profile position. No technical retry or storage-recovery behavior is specified.

## 12. Business Rules and Decision Constraints

- Workforce & Engagement owns employee separation.
- Employee Profile owns EmployeeProfile lifecycle and archival treatment.
- Historical profile information remains preserved.
- A downstream profile consequence must not alter the upstream separation decision.

## 13. State and Lifecycle Impact

Workforce Engagement remains in its authoritative separated/closed position. EmployeeProfile moves only to the canonical archival position defined upstream; no new state name is introduced.

## 14. Business Events and Cross-Domain Collaboration

The separation event remains Workforce & Engagement-owned and the resulting profile archival event remains Employee Profile-owned. Exact event names are not invented at L4.

## 15. Business Authorization, Approval and Audit Obligations

The system recognizes only an already-authorized separation outcome. Separation evidence remains traceable under Workforce & Engagement and the profile archival consequence remains traceable under Employee Profile.

## 16. Completion Criteria and Postconditions

- The authoritative separation outcome exists.
- The parent-defined profile archival consequence is applied.
- Historical EmployeeProfile information remains preserved.
- Workforce separation ownership remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

The parent defers the exact separation event and EmployeeProfile archival transition vocabulary; L4 preserves that upstream dependency and does not invent those terms.

## Production Verification Record

- [x] L3 identity, title, ownership, and scenario boundary preserved.
- [x] Parent status verified as Locked.
- [x] No implementation architecture introduced.
- [x] No unverified state, event, role, rule, or ownership invented.
- [x] Status retained as Draft.
- [x] Parent open issue preserved rather than silently repaired.
