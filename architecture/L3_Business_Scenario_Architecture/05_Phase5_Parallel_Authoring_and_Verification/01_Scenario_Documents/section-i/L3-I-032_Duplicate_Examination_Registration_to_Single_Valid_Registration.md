# Duplicate Examination Registration to Single Valid Registration

**Scenario ID:** L3-I-032  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Participating Domains:** Examination, Student Lifecycle, Academic Delivery, Finance  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I01 Examination Policy & Configuration Workflows; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows; applicable Student Lifecycle, Academic Delivery, Finance and Platform Foundation architecture  
**Referenced By:** L2-I05 through L2-I16 where confirmed Examination Registration is consumed

## 2. Scenario Purpose

Govern the resolution of duplicate Examination Registrations for the same candidate and examination participation scope so that exactly one valid registration remains authoritative while the duplicate registration does not become an independent downstream examination participation record.

## 3. Business Outcome

A single valid Examination Registration remains authoritative for the affected candidate and examination scope, the duplicate registration is prevented from becoming an independent valid participation record, and the registration history remains traceable for downstream examination operations and audit.

## 4. Scenario Boundary and Scope

**Start Boundary:** More than one Examination Registration is identified as representing the same candidate's participation in the same applicable examination scope.

**In Scope:** identification of the duplicate condition; comparison of the candidate, Examination Session, registration scope and applicable registration context; determination of the valid registration; preservation of the authoritative registration; controlled disposition of the duplicate; audit; and downstream consumption of the single valid registration.

**Out of Scope:** changing examination policy; changing the Examination Session or Schedule; altering Student Lifecycle academic status; changing curriculum ownership; financial transaction processing; Hall Ticket generation; examination conduct; and technical duplicate-detection mechanisms.

**End Boundary:** One Examination Registration is established as the valid authoritative registration for the affected participation scope and the duplicate no longer participates as an independent valid registration.

## 5. Ownership and Participating Domains

**Examination:** owns Examination Registration, registration lifecycle and the authoritative resolution.

**Student Lifecycle:** provides authoritative candidate/student context where required; it does not own the registration.

**Academic Delivery / Curriculum:** provides programme, curriculum, course or subject applicability where required; it does not own the registration.

**Finance:** provides financial eligibility where policy requires it; it does not own the registration.

Platform Foundation may provide authorization and audit capabilities without owning the business outcome.

## 6. Business Actors, Roles and Responsibilities

- **Examination Administrator:** identifies and manages duplicate registrations and establishes the valid registration position.
- **Controller of Examination:** provides examination governance and approval where the resolution requires institutional authority.
- **Candidate (Student):** may provide clarification or participate in correction where required.
- **Student Domain:** supplies authoritative candidate context.
- **Academic Administration:** supports academic applicability checks where required.
- **Finance Domain:** supplies financial eligibility where required.

No new actor or authority is introduced by this scenario.

## 7. Preconditions and Entry Criteria

- An Examination Registration exists for the candidate.
- A second registration is identified as potentially duplicating the same examination participation scope.
- The relevant Examination Session and approved Examination Schedule are identifiable.
- The candidate identity and applicable registration scope can be established.
- The applicable registration lifecycle and duplicate-registration rules remain those of L2-I04.

## 8. Trigger

A duplicate Examination Registration is detected or reported for the same candidate and examination participation scope.

## 9. Main Business Flow

1. Identify all Examination Registrations associated with the candidate and affected examination scope.
2. Establish the candidate, Examination Session, subject/registration-item scope and registration category represented by each registration.
3. Determine whether the registrations represent the same business participation or legitimately distinct registration items permitted by policy.
4. If the registrations are duplicates, identify the existing valid registration that is authoritative under the Examination Registration lifecycle.
5. Preserve the authoritative registration and its current valid lifecycle position.
6. Prevent the duplicate registration from becoming an additional valid participation record.
7. Apply the L2-I04 registration disposition permitted for the duplicate registration, preserving its historical traceability.
8. Record the duplicate-resolution activity and business justification.
9. Publish the applicable canonical registration event when the duplicate registration reaches the corresponding governed business state.
10. Ensure downstream examination workflows consume only the single valid registration for the affected participation scope.

## 10. Alternate Business Flows

### 10.1 Registrations Are Not True Duplicates

If the registrations represent legitimately different subjects, categories, or registration items permitted by policy, they remain distinct and no duplicate resolution is applied.

### 10.2 Duplicate Detected Before Confirmation

Where the duplicate is identified while registrations are still pending, the duplicate is prevented from progressing while the valid registration continues through its normal lifecycle.

### 10.3 Duplicate Detected After Confirmation

Where a duplicate has already been confirmed, the duplicate is handled through the authorised amendment/withdrawal or other applicable L2-I04 business disposition. The authoritative registration remains the valid participation record.

### 10.4 Duplicate Detected After Locking

Where the duplicate is already locked, the resolution follows the exceptional institutional process applicable to locked registrations. No routine amendment is assumed.

## 11. Failure, Interruption and Business Recovery

### 11.1 Valid Registration Cannot Be Determined

If the authoritative registration cannot be established from the available business evidence, the registrations remain unresolved pending authorised examination review rather than selecting an arbitrary survivor.

### 11.2 Candidate or Academic Context Is Conflicting

If authoritative candidate or academic context conflicts, the affected registration resolution is held until the owning domain establishes the authoritative context.

### 11.3 Duplicate Has Downstream Consequences

If a duplicate registration has already been consumed by downstream examination workflows, the resulting business correction is coordinated with those workflows. This scenario does not take ownership of Hall Ticket, conduct, marks or result lifecycles.

### 11.4 Duplicate Was Created Through an Approved Exception

If the registrations are both legitimate under an authorised exceptional policy, they are not treated as duplicates merely because their surface details are similar.

## 12. Business Rules and Decision Constraints

- A candidate's examination participation must not be represented by multiple independent valid registrations for the same business scope.
- Existing valid registration remains authoritative when a duplicate is identified.
- Duplicate resolution must preserve registration history and auditability.
- A duplicate condition does not transfer ownership of Student, Curriculum, Academic, Finance or Examination Policy information.
- Registration disposition follows the existing L2-I04 lifecycle and approved institutional policy.
- Downstream examination workflows must consume the authoritative registration rather than an obsolete duplicate.
- No new registration state or duplicate-specific event is created by this scenario.

## 13. State and Lifecycle Impact

The scenario applies the existing L2-I04 Registration State Model:

`Draft → Pending Validation → Pending Review → Approved → Confirmed → Locked`

with the existing governed amendment/withdrawal outcomes where applicable, and subsequent `Completed` / `Archived` lifecycle positions.

Duplicate resolution does not create a new lifecycle state. The duplicate registration receives the existing authorised disposition supported by L2-I04, while the valid registration retains its authoritative lifecycle position.

## 14. Business Events and Cross-Domain Collaboration

L2-I04 is the canonical publisher for registration lifecycle events.

Relevant events include:

- **Registration Initiated**
- **Eligibility Validated**
- **Registration Submitted**
- **Registration Approved**
- **Registration Rejected**
- **Registration Confirmed**
- **Registration Amended**
- **Registration Withdrawn**
- **Registration Locked**
- **Registration Archived**

This scenario does not invent a `DuplicateRegistrationResolved` event.

Student, Academic Delivery/Curriculum and Finance information is consumed according to their existing ownership and contracts. No participating domain directly modifies Examination Registration.

## 15. Business Authorization, Approval and Audit Obligations

- Duplicate resolution must be performed by an authorised Examination business actor.
- Where exceptional action is required after locking or confirmation, the applicable institutional approval must be obtained.
- The business reason for the duplicate disposition must be traceable.
- The prior and resulting registration positions must remain auditable.
- Platform authorization and audit capabilities remain owned by Platform Foundation.
- No technical access control/IAM mechanism is defined.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The duplicate condition is established.
- The valid registration is identified.
- The duplicate receives an authorised existing lifecycle disposition.
- The valid registration remains authoritative.
- Registration history and business justification are preserved.
- Applicable canonical registration events are published.
- Downstream workflows have a single valid registration position for the affected scope.

**Postconditions:**
- One Examination Registration is authoritative for the affected candidate and participation scope.
- The duplicate cannot independently authorise downstream examination processing.
- Historical evidence of the duplicate condition and its resolution remains available.

## 17. Open Issues, Assumptions and Deferred Questions

1. The authoritative L2-I04 source explicitly requires duplicate registration prevention and states that an existing valid registration remains authoritative; it does not define a separate duplicate-specific lifecycle state or event. This scenario therefore uses only existing lifecycle dispositions and canonical registration events.
2. The exact frozen Stage 3 Type/Priority values are not exposed in the available canonical-register search results. The values in Section 1 are therefore a Phase 5 classification interpretation and must be mechanically reconciled against the frozen Stage 3 row before the document can advance beyond Draft.
3. If Stage 3 assigns different frozen values, those values must replace Section 1 without changing the scenario's business semantics.
