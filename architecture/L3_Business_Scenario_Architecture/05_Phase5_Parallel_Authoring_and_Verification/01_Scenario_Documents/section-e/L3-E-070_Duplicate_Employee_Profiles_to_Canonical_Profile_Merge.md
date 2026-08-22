# Duplicate Employee Profiles to Canonical Profile Merge

**Scenario ID:** L3-E-070  
**Status:** Draft  
**Primary Owner:** Employee Profile  
**Participating Domains:** A — Platform Foundation; E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P1  
**Architecture Level:** L3 — Business Scenario Architecture  
**Author:** Varad

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-070 |
| Canonical Title | Duplicate Employee Profiles to Canonical Profile Merge |
| Primary Owner | Employee Profile |
| Participating Domains | A — Platform Foundation; E — People & Workforce |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Resolve multiple EmployeeProfile records that represent the same employee into one canonical EmployeeProfile while preserving authoritative ownership, identity integrity, and historical traceability.

The merge is a business resolution of duplicate profile records. It does not transfer Person or Identity ownership from Platform Foundation to Employee Profile, and it does not create a new independent employee.

## 3. Business Outcome

Exactly one canonical EmployeeProfile remains authoritative for the employee, duplicate profile records are resolved according to the governed merge process, historical information is preserved as required by Employee Profile, and downstream consumers have a single canonical profile reference.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when duplicate EmployeeProfile records are identified as representing the same employee and a governed merge decision is required.

### 4.2 End Boundary

The scenario ends when the canonical EmployeeProfile has been established, the duplicate profile records have been resolved according to the authoritative lifecycle, and the resulting business facts are available to dependent consumers.

### 4.3 In Scope

- Identification of duplicate EmployeeProfile records.
- Validation that the candidate profiles represent the same employee.
- Selection of the canonical EmployeeProfile.
- Preservation of authoritative information and required history.
- Resolution of the duplicate profile lifecycle.
- Reconciliation of child profile information within the canonical EmployeeProfile boundary.
- Publication of canonical profile lifecycle/change events.
- Preservation of Person and Identity ownership.

### 4.4 Out of Scope

- Creating or merging Person records directly.
- Merging Identity records directly.
- Workforce Engagement merge.
- Position Assignment merge.
- Payroll or leave record consolidation.
- Technical deduplication algorithms, database operations, rollback or storage mechanics.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Employee Profile | Owns EmployeeProfile, duplicate-profile resolution, canonical profile selection within its bounded context, and profile history. |
| Platform Foundation | Owns Person and Identity; Employee Profile must preserve those canonical identities and references. |

Participation does not transfer ownership of Person or Identity.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| HR Administrator | Initiates or administers duplicate-profile resolution. |
| HR Executive | Supports investigation and profile resolution. |
| Compliance Officer | Participates where policy or evidence requires compliance review. |
| Verification Officer | Participates where identity/profile verification is required to resolve the duplicate. |

No additional role is introduced.

## 7. Preconditions and Entry Criteria

- At least two EmployeeProfile records exist.
- There is evidence that the profiles represent the same employee.
- A valid canonicalization decision is required.
- Person and Identity ownership remain resolvable under Platform Foundation.
- The affected records are accessible within the Employee Profile lifecycle.

## 8. Trigger

Duplicate EmployeeProfile records are identified and submitted for governed canonical resolution.

## 9. Main Business Flow

| Step | Business Behaviour | Owner | Result |
|---|---|---|---|
| 1 | Candidate duplicate EmployeeProfile records are identified. | Employee Profile | A duplicate-resolution case exists. |
| 2 | The records are reviewed to establish that they represent the same employee. | HR / Verification Officer | Duplicate relationship is confirmed or rejected. |
| 3 | The authoritative canonical EmployeeProfile is selected. | Employee Profile | One profile is designated canonical. |
| 4 | Profile information that is valid and applicable is reconciled into the canonical profile. | Employee Profile | Canonical profile contains the accepted authoritative information. |
| 5 | Duplicate profile records are moved to their governed resolved lifecycle treatment without destroying required historical evidence. | Employee Profile | Duplicate records no longer compete as canonical profiles. |
| 6 | The canonical profile remains linked to the existing Person/Identity context. | Employee Profile / Platform Foundation boundary | Identity ownership remains unchanged. |
| 7 | Employee Profile publishes the canonical business events associated with the merge/resolution. | Employee Profile | Dependent consumers can reconcile their profile references. |

## 10. Alternate Business Flows

### A1 — Duplicate Not Confirmed

If the records are found to represent different employees, no merge occurs and both remain independently governed.

### A2 — Conflicting Profile Information

If duplicate profiles contain conflicting information, only information supported by the authoritative profile rules and resolution evidence is incorporated. Unsupported assumptions are not silently selected.

### A3 — Identity Ambiguity

If the Person/Identity relationship cannot be established confidently, the merge is not completed until the canonical identity boundary is resolved.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Duplicate evidence is insufficient | Keep profiles separate until the duplicate relationship is established. |
| Canonical profile cannot be determined | Route for governed business review; do not invent a canonical choice. |
| Conflicting information cannot be resolved | Preserve the existing authoritative information and defer unresolved data rather than inventing a winner. |
| Merge would change Person/Identity ownership | Stop the profile merge and route the identity issue to Platform Foundation. |

Technical retry, rollback and database mechanics are outside L3.

## 12. Business Rules and Decision Constraints

- EmployeeProfile is the canonical owner of employee profile information.
- Exactly one EmployeeProfile must remain authoritative for the resolved employee.
- Person and Identity remain owned by Platform Foundation.
- Profile history must remain traceable.
- Duplicate resolution must not silently delete historical business evidence.
- Child profile entities remain governed by EmployeeProfile and do not become independently owned records.

## 13. State and Lifecycle Impact

| Concept | Impact | Owner |
|---|---|---|
| Canonical EmployeeProfile | Remains in its valid lifecycle state with consolidated authoritative information. | Employee Profile |
| Duplicate EmployeeProfile | Moves to the lifecycle treatment defined by the authoritative profile architecture for resolved duplicates. | Employee Profile |
| Person | Remains canonical and unchanged by the profile merge. | Platform Foundation |
| Identity | Remains canonical and unchanged by the profile merge. | Platform Foundation |

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

Employee Profile publishes the canonical events corresponding to the completed profile resolution.

The exact frozen merge event vocabulary must be taken from the authoritative Stage 3/L2 event mapping; no new event name is invented here.

### 14.2 Business Hand-offs

Employee Profile → downstream consumers: canonical profile identity and resolved profile facts.

Platform Foundation remains the owner of Person and Identity references.

## 15. Business Authorization, Approval and Audit Obligations

Duplicate resolution must be performed by authorized HR business actors within the Employee Profile governance.

Where identity verification or compliance evidence is required, the Verification Officer or Compliance Officer participates according to the applicable business process.

The duplicate-resolution decision, supporting evidence, canonical selection and resulting profile history must remain traceable.

## 16. Completion Criteria and Postconditions

- Duplicate EmployeeProfile records are confirmed as representing the same employee.
- One EmployeeProfile is canonical.
- Profile information is reconciled under Employee Profile ownership.
- Duplicate profile records no longer compete as canonical.
- Person and Identity ownership remain unchanged.
- Historical resolution evidence remains traceable.
- Canonical profile events have been published where defined.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material ownership conflict identified.

The exact frozen merge transition/event vocabulary should be checked against the authoritative Stage 3 and E07 event mapping before Architecture Approval.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

Confirm exact canonical duplicate-profile merge events and terminal treatment of the duplicate profile record.
