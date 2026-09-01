# Employee Grievance to Investigated Resolution

## 1. Specification Metadata
**Specification ID:** L4-E-036  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** HR  
**Realizes Scenario:** L3-E-036  
**Primary Business Owner:** HR  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** L3-E-036 — Employee Grievance to Investigated Resolution; authoritative HR grievance/investigation workflows

## 2. Behavioral Objective & Boundary
SutraERP shall govern an employee grievance from receipt through appropriate investigation and resolution, preserving protected evidence and the authoritative outcome. L4 does not define implementation mechanisms.

## 3. Required Business Context
The grievance is an HR-owned business outcome. The case requires protected handling, applicable investigation authority and sufficient evidence/context. Any downstream consequence remains owned by its canonical domain.

## 4. Behavioural Specification
1. Register the employee grievance and validate required information.
2. Establish the governed grievance/investigation state.
3. Determine the applicable investigation responsibility.
4. Gather and preserve relevant evidence.
5. Conduct the authorised investigation.
6. Record findings and applicable decision/outcome.
7. Coordinate required downstream consequences with their canonical owners.
8. Close the grievance only when the authoritative resolution criteria are satisfied.
9. Preserve protected evidence and historical case information.

## 5. Alternate and Exceptional Behaviour
Incomplete information may place the grievance into a governed pending/correction path. If investigation cannot proceed, the case remains unresolved until a valid business path is established. Confidential/protected evidence remains governed throughout.

## 6. Business Rules
- Grievances require governed investigation and resolution.
- Evidence relevant to the grievance remains protected and historically preserved.
- Downstream employment consequences do not transfer ownership to HR unless already canonical.
- No unsupported lifecycle state is invented.

## 7. Events and Collaboration
Use canonical HR grievance events/contracts defined by the upstream architecture. Cross-domain consequences are handed to their canonical owners; no undefined event name is invented.

## 8. Authorization, Audit & Traceability
Investigation authority, decision reason and evidence obligations follow HR governance. The investigation and resolution remain auditable while protecting sensitive business evidence.

## 9. Completion & Validation
Completion requires an authoritative investigated resolution, applicable downstream handoffs, protected evidence preservation and traceable case history.

## 10. Out of Scope
APIs, databases, code, UI, IAM implementation, infrastructure, deployment and technical retry/rollback behaviour.