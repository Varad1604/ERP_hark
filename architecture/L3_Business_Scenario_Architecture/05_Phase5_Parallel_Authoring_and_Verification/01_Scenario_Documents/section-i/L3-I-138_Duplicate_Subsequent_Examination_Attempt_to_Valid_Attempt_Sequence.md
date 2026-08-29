# Duplicate Subsequent Examination Attempt to Valid Attempt Sequence

**Scenario ID:** L3-I-138  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I04 Examination Registration Workflows; L2-I12 Supplementary / Backlog / Improvement Examination Execution Workflows; applicable Student Lifecycle authoritative sources  
**Referenced By:** L2-I05 Hall Ticket Management; L2-I06 Practical & Viva Examination Conduct; L2-I07 Theory Examination Conduct; L2-I08 Marks Collection & Evaluation; L2-I09 Result Processing & Computation

## 1. Scenario Metadata

This scenario is the final L3-I-138 canonical output. The frozen Stage 3 semantics define it as a Critical P1 scenario owned by Supplementary, Backlog & Improvement Examination, with Student Lifecycle and Examination participation.

The scenario governs detection and resolution of a duplicate subsequent-examination attempt so that one valid attempt sequence remains authoritative and no duplicate attempt is allowed to create a second competing examination outcome.

## 2. Scenario Purpose

Preserve a single authoritative sequence of Supplementary, Backlog, or Improvement examination attempts when duplicate attempt activity is identified.

The scenario reconciles the duplicate against authoritative candidate, examination, registration and prior-attempt context without creating a new local attempt lifecycle or altering the ownership of Student Lifecycle or Examination Registration.

## 3. Business Outcome

A duplicate subsequent-examination attempt is resolved to the valid authoritative attempt sequence, with the duplicate activity prevented from becoming a competing independent examination attempt and the complete business history retained for traceability.

## 4. Scenario Boundary and Scope

**Start Boundary:** A subsequent-examination application, registration or attempt-management activity is identified as duplicating an existing attempt for the same candidate and applicable examination scope.

**In Scope:**
- establish candidate and examination context;
- identify the potentially duplicate attempt;
- compare it with authoritative prior attempt and registration context;
- determine whether the activity represents the same attempt or a legitimately distinct permitted attempt;
- preserve the valid attempt sequence;
- prevent an invalid duplicate from becoming an additional authoritative attempt;
- preserve the resulting reconciliation and audit history.

**Out of Scope:**
- redefining attempt-limit policy;
- changing Student Lifecycle academic status;
- owning Examination Registration;
- computing results;
- changing an authoritative examination outcome without the owning workflow;
- creating a new duplicate-resolution lifecycle state;
- technical duplicate-detection mechanisms.

**End Boundary:** The duplicate activity is conclusively reconciled to the valid attempt sequence or established as a legitimately distinct permitted attempt under the authoritative rules.

## 5. Ownership and Participating Domains

**Supplementary, Backlog & Improvement Examination:** owns attempt lifecycle administration and the business reconciliation of subsequent-examination attempt activity.

**Student Lifecycle:** supplies authoritative candidate and academic context. It does not own the examination attempt.

**Examination Registration (L2-I04):** owns the Examination Registration record and lifecycle for Supplementary, Backlog and Improvement categories. A duplicate registration is resolved through that canonical lifecycle; L2-I12 does not create a parallel registration record.

No duplicate ownership is created.

## 6. Business Actors, Roles and Responsibilities

| Actor / Domain | Responsibility |
|---|---|
| Candidate | Provides the examination application/participation context |
| Examination Administrator | Reviews and resolves duplicate subsequent-examination activity |
| Supplementary Examination Officer | Reconciles attempt history and applies the applicable attempt policy |
| Controller of Examination | Governs exceptional administrative decisions where required |
| Student Lifecycle | Provides authoritative candidate/academic context |
| Examination Registration | Provides the authoritative registration position |

The scenario introduces no new duplicate-resolution authority.

## 7. Preconditions and Entry Criteria

- A subsequent-examination activity exists for a candidate.
- The candidate context can be established authoritatively.
- An existing attempt, registration or relevant examination history can be identified.
- The applicable Supplementary, Backlog or Improvement policy defines the permitted attempt framework.
- The authoritative Examination Registration lifecycle can be consulted where registration identity is relevant.

## 8. Trigger

A subsequent-examination activity is identified as potentially duplicating an existing attempt for the same candidate and examination scope.

## 9. Main Business Flow

1. Identify the candidate and the subsequent-examination activity under review.
2. Establish the applicable examination category, subject/examination scope and Examination Session.
3. Retrieve the authoritative prior attempt and relevant Examination Registration position.
4. Compare the candidate, examination scope, session and attempt history to determine whether the new activity represents the same permitted attempt or a distinct permitted attempt.
5. Where the activity is a duplicate, reconcile it to the existing valid attempt sequence and prevent it from becoming a second authoritative attempt.
6. Where the activity is legitimately distinct and permitted by policy, retain it as the next valid attempt in the existing attempt sequence.
7. Preserve the reconciliation decision and affected attempt history.
8. Where Examination Registration must be corrected, amended or withdrawn, route the change through L2-I04 rather than modifying registration ownership locally.
9. Publish the applicable subsequent-examination lifecycle event(s).
10. End with one authoritative attempt sequence for the candidate and examination scope.

## 10. Alternate Business Flows

### 10.1 Duplicate Application Before Registration

Where duplicate activity is identified before Examination Registration is completed, the valid application/attempt path is retained and the duplicate is not allowed to create a second authoritative attempt.

### 10.2 Duplicate Registration

Where two registration activities represent the same intended examination participation, L2-I04 remains authoritative for Examination Registration. The duplicate registration is reconciled through the existing registration governance.

### 10.3 Legitimate Subsequent Attempt

Where policy permits another attempt and the activity is materially distinct from the prior attempt, it is retained as the next valid attempt in the sequence rather than incorrectly treated as a duplicate.

### 10.4 Duplicate Activity After Scheduling

Where duplicate activity is discovered after scheduling, the duplicate does not automatically produce a second examination schedule or participation path. Existing registration and schedule ownership remain with L2-I04 and L2-I03 respectively.

### 10.5 Duplicate Activity After Examination Conduct

Where duplication is discovered after conduct has occurred, the scenario does not silently replace marks or results. Any consequential academic outcome remains governed by L2-I08, L2-I09 and L2-I10.

## 11. Failure, Interruption and Business Recovery

### 11.1 Candidate Identity Cannot Be Reliably Matched

If the candidate context is unresolved, the duplicate determination is not finalized on assumption. Student Lifecycle/identity authority remains responsible for establishing the authoritative candidate context.

### 11.2 Examination Scope Cannot Be Reconciled

If subject, examination category or session context is conflicting, the attempt remains unresolved until the authoritative examination context is established.

### 11.3 Attempt Limit Is Reached

If the applicable policy indicates that no further attempt is permitted, the activity cannot become a new valid attempt. The existing authoritative sequence remains unchanged except for the governed administrative decision.

### 11.4 Conflicting Registration Positions

If registration records conflict, L2-I04 remains the canonical owner of Examination Registration and must govern the registration correction. L2-I12 does not create a local substitute.

### 11.5 Duplicate Has Downstream Consequences

If the duplicate has already influenced scheduling, Hall Ticket, conduct or evaluation, each downstream consequence is handled by its canonical owning workflow. This scenario records the authoritative attempt reconciliation and does not directly alter downstream owned records.

### 11.6 Reconciliation Decision Is Exceptional

Where institutional policy requires an exceptional administrative override, the existing L2-I12 override governance applies and the decision must remain justified and auditable.

## 12. Business Rules and Decision Constraints

- Every Supplementary, Backlog or Improvement examination represents an independent institutional attempt, and each attempt must remain uniquely identifiable.
- Attempt limits and attempt regulations are governed by institutional examination policy.
- A duplicate activity must not create a second competing authoritative attempt for the same permitted attempt.
- A legitimately distinct permitted attempt must not be incorrectly collapsed into a prior attempt.
- Examination Registration remains owned by L2-I04.
- Student academic context remains owned by Student Lifecycle.
- Downstream marks, results and publication remain owned by their respective Examination workflows.
- Administrative overrides require appropriate authority, recorded justification and complete auditability.
- No duplicate-specific state or event is invented.

## 13. State and Lifecycle Impact

L2-I12 defines the subsequent-examination attempt lifecycle as:

`Eligible → Application Submitted → Fee Verified → Registered → Session Allocated → Scheduled → Hall Ticket Initiated → Examination Conducted → Evaluation Initiated → Result Processing Initiated → Attempt Closed → Archived`

The scenario does not add a `Duplicate` or `Merged` state.

Where duplicate activity is reconciled, the valid attempt remains in the authoritative lifecycle position and the duplicate activity is treated as a governed business reconciliation outcome rather than a new canonical lifecycle.

Examination Registration retains its own L2-I04 lifecycle and is not duplicated in L2-I12.

## 14. Business Events and Cross-Domain Collaboration

L2-I12 publishes canonical subsequent-examination milestones including:
- **Supplementary Eligibility Confirmed**
- **Backlog Eligibility Confirmed**
- **Improvement Eligibility Confirmed**
- **Examination Application Submitted**
- **Examination Registration Completed**
- **Supplementary Session Allocated**
- **Supplementary Examination Scheduled**
- **Hall Ticket Initiation Requested**
- **Supplementary Examination Started**
- **Supplementary Examination Completed**
- **Evaluation Initiated**
- **Result Processing Initiated**
- **Candidate Notification Issued**
- **Examination Attempt Closed**
- **Examination Attempt Archived**

L2-I04 publishes canonical registration milestones including **Registration Initiated**, **Registration Approved**, **Registration Confirmed**, **Registration Amended**, **Registration Withdrawn**, and **Registration Archived**.

The scenario does not invent `DuplicateAttemptResolved` or `AttemptMerged` events. The reconciliation is represented through the authoritative existing lifecycle/registration milestones where an actual state change occurs.

## 15. Business Authorization, Approval and Audit Obligations

- Duplicate determination must use authoritative candidate, examination and attempt context.
- Attempt-policy exceptions follow the applicable institutional approval governance.
- Registration corrections follow L2-I04 authority.
- Any administrative override follows L2-I12's requirement for appropriate authority, business justification and complete audit history.
- The original and duplicate activities must remain distinguishable in the business history.
- Any downstream consequence requiring correction remains attributable to the owning workflow.
- No technical identity-matching or duplicate-detection mechanism is specified.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- Candidate and examination context are established.
- Existing authoritative attempt/registration context is identified.
- Duplicate versus legitimate subsequent attempt is determined under authoritative rules.
- A duplicate activity is prevented from creating a competing authoritative attempt, or a legitimate new attempt is retained in the valid sequence.
- Any required registration correction is routed through L2-I04.
- Attempt history and reconciliation decision are auditable.
- Applicable canonical events are published.

**Postconditions — Duplicate Outcome:**
- One authoritative attempt sequence remains for the affected permitted attempt.
- Duplicate activity does not create a competing examination attempt.
- Existing downstream authoritative outcomes are not silently replaced.

**Postconditions — Legitimate New Attempt:**
- The new attempt remains a distinct, valid position in the existing attempt sequence.
- It is governed by the same subsequent-examination lifecycle and applicable attempt rules.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I12 requires every attempt to be uniquely identifiable and traceable and defines attempt limits as configurable institutional policy. It does not define a dedicated `Duplicate` lifecycle state.
2. L2-I04 owns Examination Registration for Supplementary, Backlog and Improvement categories. Any registration-level duplicate must therefore be reconciled through L2-I04.
3. The authoritative Student Lifecycle source for the exact candidate-context query/contract was not included in the available L2-I12 source. This scenario therefore states the dependency without inventing a contract name.
4. The exact institutional criteria distinguishing a duplicate from a legitimate subsequent attempt remain policy-owned. L3 applies those rules but does not create their substantive values.
