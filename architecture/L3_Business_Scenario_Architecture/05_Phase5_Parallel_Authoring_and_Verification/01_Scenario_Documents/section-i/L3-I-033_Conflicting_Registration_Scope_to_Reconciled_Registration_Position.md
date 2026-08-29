# Conflicting Registration Scope to Reconciled Registration Position

**Scenario ID:** L3-I-033  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Participating Domains:** Examination, Student Lifecycle, Academic Delivery, Finance  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I01 Examination Policy & Configuration Workflows; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows; applicable Student Lifecycle, Academic Delivery, Finance and Platform Foundation architecture  
**Referenced By:** L2-I05 through L2-I16 where the reconciled Examination Registration is consumed

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-033 |
| Canonical Title | Conflicting Registration Scope to Reconciled Registration Position |
| Primary Owner | Examination |
| Participating Domains | Examination, Student Lifecycle, Academic Delivery, Finance |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Resolve conflicting examination-registration scope so that the candidate's Examination Registration represents one authoritative, policy-valid participation position without allowing inconsistent academic, subject, category or registration information to create competing downstream examination participation.

## 3. Business Outcome

The conflicting registration information is reconciled into one authoritative Examination Registration position that is consistent with the applicable candidate, examination, subject and policy context, while the prior conflicting position remains historically traceable.

## 4. Scenario Boundary and Scope

**Start Boundary:** Authoritative or potentially authoritative sources identify inconsistent registration scope for the same candidate and examination context.

**In Scope:** identification of the conflicting registration positions; comparison against the applicable Examination Session, Schedule, candidate context, curriculum/subject applicability, registration category and policy; reconciliation; authorised amendment or withdrawal where required; preservation of history; audit; and downstream use of the reconciled registration.

**Out of Scope:** changing upstream Student, Curriculum, Examination Policy, Examination Session or Schedule ownership; financial transaction processing; Hall Ticket generation; examination conduct; marks; results; and implementation mechanics.

**End Boundary:** One reconciled Examination Registration position is established and is the only current registration position to be consumed for the affected examination scope.

## 5. Ownership and Participating Domains

**Examination:** owns Examination Registration and the reconciliation outcome.

**Student Lifecycle:** owns authoritative student context and academic status.

**Academic Delivery / Curriculum:** owns programme, curriculum, course and subject applicability.

**Finance:** owns financial eligibility where applicable.

The reconciliation does not transfer ownership of any participating-domain information to Examination.

## 6. Business Actors, Roles and Responsibilities

- **Examination Administrator:** analyses the conflicting registration scope and manages the reconciliation.
- **Controller of Examination:** provides examination governance and required approval for material reconciliation.
- **Candidate (Student):** may provide clarification or participate where required by institutional process.
- **Academic Administration:** supports academic applicability clarification where required.
- **Student Domain:** provides authoritative candidate and academic-status information.
- **Finance Domain:** provides authoritative financial eligibility where required.

Only source-supported business roles are used.

## 7. Preconditions and Entry Criteria

- An Examination Registration exists.
- Conflicting scope is identified for the same candidate and examination context.
- The applicable Examination Session and approved Schedule are identifiable.
- The relevant subject, programme, curriculum and registration category can be evaluated.
- The applicable registration policy and lifecycle remain those defined upstream.

## 8. Trigger

A material conflict is identified between registration scope positions for a candidate, such that more than one interpretation could affect examination participation.

## 9. Main Business Flow

1. Identify the affected candidate, Examination Session, Schedule and current registration position.
2. Identify each conflicting registration-scope representation.
3. Establish which business facts are authoritative for candidate, academic applicability, examination policy and financial eligibility.
4. Determine whether the conflict is a legitimate multi-item registration or an actual inconsistency.
5. Establish the single registration scope permitted by the authoritative business context.
6. Apply the existing L2-I04 amendment, withdrawal or other authorised registration disposition needed to reconcile the position.
7. Preserve the prior conflicting information and the business basis for the resulting position.
8. Obtain required institutional approval where the reconciliation requires an exceptional or post-lock action.
9. Confirm the reconciled Examination Registration when all required conditions are satisfied.
10. Publish the applicable canonical registration lifecycle events.
11. Make the reconciled registration position authoritative for downstream examination workflows.

## 10. Alternate Business Flows

### 10.1 Conflict Is a Legitimate Mixed Registration

Where L2-I04 permits multiple Registration Items or examination categories within one Examination Registration, apparently different scope values may be legitimate. The registration remains one authoritative registration with its permitted item-level scope.

### 10.2 Conflict Caused by Academic Context Change

Where an academic-registration or curriculum change has altered applicability, the authoritative academic domain supplies the changed context and Examination Registration applies the resulting registration consequence through its own lifecycle.

### 10.3 Conflict Caused by Registration Amendment

Where the conflict is the result of a pending amendment, the authorised amendment process resolves the scope before confirmation or locking as applicable.

### 10.4 Conflict Identified After Locking

A locked registration is not routinely amended. Any required change follows the authorised exceptional process.

## 11. Failure, Interruption and Business Recovery

### 11.1 No Authoritative Reconciliation Is Available

If the participating sources cannot establish a single authoritative scope, the registration remains unresolved rather than inventing a compromise.

### 11.2 Candidate Context Is Unclear

If candidate or academic status is conflicting, Examination does not redefine Student Lifecycle state. The relevant owning domain must establish the authoritative context before the registration consequence is finalised.

### 11.3 Subject Applicability Is Unclear

If subject applicability cannot be established, the affected subject remains unresolved and is not treated as valid examination participation until the Curriculum/Academic authority establishes applicability.

### 11.4 Financial Eligibility Is Conflicting

Finance remains authoritative for financial eligibility. Examination does not resolve competing financial positions itself.

### 11.5 Downstream Processing Already Exists

If Hall Ticket or examination operations have consumed the conflicting registration, the reconciliation is coordinated with those owning workflows. This scenario does not redefine their lifecycle.

## 12. Business Rules and Decision Constraints

- Examination Registration must represent an authoritative, policy-valid examination participation scope.
- Candidate, academic, curriculum and financial facts remain owned by their source domains.
- A legitimate multi-item registration must not be incorrectly collapsed merely because its items differ.
- A genuine conflict must not leave multiple competing current registration positions.
- Existing L2-I04 lifecycle states and transitions govern reconciliation.
- Amendments must follow registration policy and locking rules.
- Exceptional post-lock changes require authorised institutional action.
- The resulting registration must remain auditable.
- No new local rule, state or event is created.

## 13. State and Lifecycle Impact

The scenario uses the L2-I04 Registration State Model:

`Draft → Pending Validation → Pending Review → Approved → Confirmed → Locked`

with existing `Amended`, `Withdrawn`, `Completed` and `Archived` positions where the reconciliation requires those existing dispositions.

A conflicting scope does not create a new `Conflict` state. Reconciliation is expressed through existing registration lifecycle transitions.

## 14. Business Events and Cross-Domain Collaboration

Examination Registration remains the canonical publisher for registration events defined by L2-I04:

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

No `RegistrationScopeConflictResolved` event is invented.

Student Lifecycle, Academic Delivery/Curriculum and Finance provide authoritative business information through the repository's existing collaboration mechanisms. Examination owns the resulting registration position.

## 15. Business Authorization, Approval and Audit Obligations

- Reconciliation must be performed by an authorised Examination business actor.
- Required institutional approval must be obtained for material or exceptional registration changes.
- The conflicting position, decision basis and resulting registration must be auditable.
- Separation of ownership must be preserved: source domains decide their own authoritative facts; Examination decides the registration consequence.
- No technical IAM/RBAC behaviour is defined.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The conflict is classified as legitimate scope variation or genuine inconsistency.
- Authoritative upstream business facts are established.
- A single valid Examination Registration position is established.
- Existing registration lifecycle transitions are applied.
- Required approval is complete.
- Registration history and reconciliation rationale are preserved.
- Applicable registration events are published.
- Downstream workflows can consume one authoritative current registration position.

**Postconditions:**
- The candidate has one authoritative current registration scope for the affected examination context.
- Legitimate item-level distinctions remain preserved where permitted.
- Conflicting or superseded registration information remains historically traceable.
- Upstream ownership remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 defines Registration Item as the operational scope and explicitly permits registration categories at Registration Item level. Therefore differing item-level values are not automatically a conflict.
2. L2-I04 does not define a dedicated conflict lifecycle state or conflict-resolution event; this scenario intentionally uses existing amendment/withdrawal/confirmation lifecycle capabilities.
3. The exact frozen Stage 3 Type/Priority values are not exposed in the available canonical-register search results. The values in Section 1 are therefore a Phase 5 classification interpretation and must be mechanically reconciled against the frozen Stage 3 row before the document can advance beyond Draft.
4. If the Stage 3 row defines a different participating-domain set, Type or Priority, the frozen value must be restored without changing the business boundary.
