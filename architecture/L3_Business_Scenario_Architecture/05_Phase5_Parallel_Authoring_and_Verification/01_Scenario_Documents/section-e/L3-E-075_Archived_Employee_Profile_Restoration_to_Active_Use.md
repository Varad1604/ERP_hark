# Archived Employee Profile Restoration to Active Use

**Scenario ID:** L3-E-075  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile — People & Workforce  
**Participating Domains:** E — People & Workforce  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-E07 — Employee Profile Workflows; L2-E01 — Workforce & Engagement; L3 Phase 5 Template v2.0  
**Referenced By:** None yet

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-075 |
| Canonical Title | Archived Employee Profile Restoration to Active Use |
| Primary Owner | Employee Profile — People & Workforce |
| Participating Domains | E — People & Workforce |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Restore an archived EmployeeProfile to active operational use when the institutional business need for the profile's continued use is established and the governed restoration path is applicable.

The scenario restores the profile within Employee Profile ownership while preserving its historical identity and prior archival history.

## 3. Business Outcome

The archived EmployeeProfile is restored through its governed lifecycle and becomes **Active** for operational employee-profile use, with its prior history preserved and its identity continuity maintained.

Restoration does not recreate the employee, transfer Person or Identity ownership, or create a new EmployeeProfile.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an archived EmployeeProfile is identified as requiring restoration to active institutional use.

### 4.2 End Boundary

The scenario ends when the profile has completed the authoritative restoration path and is Active and available for operational profile use.

### 4.3 In Scope

- Identification of an archived EmployeeProfile requiring restoration.
- Validation that restoration is permitted.
- Restoration of the archived profile.
- Preservation of historical profile information and prior lifecycle history.
- Re-establishment of Active profile use.
- Publication/consumption of profile business facts where the authoritative E07 architecture defines them.

### 4.4 Out of Scope

- Creating a new Person.
- Creating or changing Identity ownership.
- Re-establishing Workforce Engagement.
- Recreating Position Assignment.
- Payroll or Leave lifecycle changes.
- Technical archive restoration, storage operations, database recovery, APIs, or implementation mechanics.

## 5. Ownership and Participating Domains

Employee Profile is the sole canonical owner of EmployeeProfile and its lifecycle.

The locked E07 architecture identifies Employee Profile as the canonical provider of employee profile information and explicitly excludes position management, payroll, leave, identity authentication and workforce onboarding from its ownership.

No participating workforce domain acquires ownership of EmployeeProfile through restoration.

## 6. Business Actors, Roles and Responsibilities

The authoritative E07 architecture identifies the following business actors:

| Actor / Role | Responsibility |
|---|---|
| HR Administrator | Administers employee profile lifecycle and restoration. |
| HR Executive | Supports workforce profile administration. |
| Employee | May participate in profile-related business activity where the governed process permits. |
| Compliance Officer | Participates where compliance obligations require profile governance. |
| Verification Officer | Participates where verification is required. |

No additional actor is introduced for the restoration scenario.

## 7. Preconditions and Entry Criteria

- The EmployeeProfile exists in the `Archived` state.
- The profile remains historically preserved.
- The restoration is a valid business requirement.
- The restoration can be performed without transferring EmployeeProfile ownership.
- Any applicable verification or compliance requirement can be satisfied.

## 8. Trigger

An authorized HR business process determines that an archived EmployeeProfile must return to active operational use.

## 9. Main Business Flow

| Step | Business Behaviour | Result |
|---|---|---|
| 1 | Identify the archived EmployeeProfile and the institutional need for restoration. | A restoration request exists. |
| 2 | Validate that the profile is eligible for restoration under its governed lifecycle. | Restoration is eligible or is rejected. |
| 3 | Perform any applicable profile verification or compliance review required by the authoritative business process. | Required business conditions are satisfied. |
| 4 | Restore the archived EmployeeProfile through the governed lifecycle. | Profile enters `Restored`. |
| 5 | Return the restored profile to operational `Active` use. | EmployeeProfile is Active. |
| 6 | Preserve the prior archival history and restored version lineage. | Historical traceability remains intact. |
| 7 | Make the active profile available as the canonical employee profile for dependent business use. | Downstream consumers can use the restored profile fact without creating a duplicate profile. |

## 10. Alternate Business Flows

### A1 — Restoration Requires Verification

Where the authoritative profile process requires verification before active use, the restoration remains subject to the applicable verification condition before the profile can be operationally Active.

### A2 — Restoration Cannot Be Completed

If a required business condition is not satisfied, the EmployeeProfile remains in its existing governed state and no active profile outcome is declared.

### A3 — Archived Profile Is Already the Canonical Historical Record

The existing EmployeeProfile is restored rather than creating a new profile. Historical identity and lineage are preserved.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Governed Resolution |
|---|---|
| Archived profile is not eligible for restoration | Do not activate it; preserve the archived state. |
| Required verification is incomplete | Do not declare the profile Active until the required business condition is satisfied. |
| Compliance concern prevents restoration | Preserve the archived profile and route the business issue through the applicable compliance process. |
| Restoration would create a duplicate employee profile | Do not create a second profile; resolve through the canonical Employee Profile ownership model. |
| Workforce Engagement is not active | Do not silently recreate or alter Workforce Engagement; resolve that separate workforce lifecycle through E01 if required. |

Technical retry, storage recovery, database rollback and infrastructure handling are outside L3.

## 12. Business Rules and Decision Constraints

The scenario applies the authoritative E07 profile architecture:

- EmployeeProfile is the canonical owner of employee profile information.
- Employee profiles are governed through a defined lifecycle.
- `Archived` is a historical profile state.
- `Restored` represents return from archival before `Active`.
- Restored profiles preserve history and increment the profile version.
- No child profile entity becomes independently owned through restoration.
- Employee Profile does not own workforce engagement, position assignment, payroll, leave or identity authentication.
- Profile restoration must not create duplicate employee identities/profiles.

No new local lifecycle state or restoration rule is introduced.

## 13. State and Lifecycle Impact

The authoritative E07 lifecycle is:

`Draft → PendingVerification → Verified → Active`

with the governed restoration path:

`Archived → Restored → Active`

Relevant states:

| State | Scenario Meaning |
|---|---|
| Archived | EmployeeProfile is preserved as a historical profile. |
| Restored | Archived profile has been returned through the restoration lifecycle. |
| Active | Restored profile is operationally available. |

The scenario does not redefine or rename any E07 lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The authoritative E07 source defines a domain event catalogue, but the accessible authoritative source material does not expose a specific restoration event name in the section used for this scenario.

Accordingly, this L3 does not invent an event name.

The business outcome remains the governed EmployeeProfile lifecycle transition from `Archived` through `Restored` to `Active`.

### 14.2 Cross-Domain Collaboration

Employee Profile remains the canonical provider of employee profile facts.

Where Workforce & Engagement or other People-domain capabilities consume the restored profile, they consume the canonical profile outcome without acquiring ownership of EmployeeProfile.

No new cross-domain contract is introduced by this scenario.

## 15. Business Authorization, Approval and Audit Obligations

Restoration is a governed Employee Profile business action administered by supported HR roles.

Where verification or compliance is required by the authoritative profile process, Verification Officer or Compliance Officer participation is retained.

The restoration decision, relevant verification/compliance evidence, lifecycle transition and resulting profile history must remain traceable under the Employee Profile governance model.

Technical IAM/RBAC and audit infrastructure remain outside L3.

## 16. Completion Criteria and Postconditions

- The archived EmployeeProfile has completed the governed restoration path.
- EmployeeProfile is `Active`.
- The same canonical EmployeeProfile remains in use; no duplicate profile is created.
- Historical archival information remains preserved.
- The restored version and lifecycle history remain traceable.
- Person and Identity ownership remain unchanged.
- Workforce Engagement and Position Assignment remain owned by their respective domains.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The locked E07 source establishes the `Archived → Restored → Active` lifecycle and states that restoration increments the version and preserves history. The accessible source material does not expose the exact named restoration event. This L3 therefore deliberately avoids inventing one.

No material ownership or lifecycle conflict has been identified.

### 17.2 Assumptions

No assumption overrides locked architecture.

### 17.3 Deferred Questions

If the authoritative E07 event catalogue is later amended or its restoration event vocabulary is explicitly governed, the exact canonical event name and publisher should be reflected here during review without changing the scenario outcome or ownership.
