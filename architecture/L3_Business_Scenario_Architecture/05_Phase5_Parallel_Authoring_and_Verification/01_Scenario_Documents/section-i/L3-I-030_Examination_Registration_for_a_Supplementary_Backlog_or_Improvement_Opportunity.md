# Examination Registration for a Supplementary, Backlog or Improvement Opportunity

**Scenario ID:** L3-I-030  
**Participating Domains:** C, D, F, I  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-I01 Examination Policy & Configuration Workflows; L2-I02 Examination Session Management Workflows; L2-I03 Examination Planning & Scheduling Workflows; L2-I04 Examination Registration Workflows; L2-I12 Supplementary / Backlog / Improvement Examination Execution Workflows; applicable Student, Academic Delivery and Finance architecture  
**Referenced By:** L2-I05 Hall Ticket Management; L2-I12 Supplementary / Backlog / Improvement Examination Execution; downstream examination processing

## 2. Scenario Purpose

Govern examination registration for an eligible candidate who is participating in a supplementary, backlog, or improvement examination opportunity, establishing the authoritative examination participation scope without transferring ownership of academic history, policy, or financial information.

## 3. Business Outcome

The candidate is registered for the authorised supplementary, backlog, or improvement examination opportunity and the confirmed registration becomes the authoritative input for downstream examination execution.

## 4. Scenario Boundary and Scope

**Start Boundary:** A policy-permitted supplementary, backlog, or improvement examination opportunity exists and a candidate seeks or is identified for registration.

**In Scope:** opportunity applicability; candidate eligibility; category selection; subject registration; registration review and approval; confirmation; audit; and downstream hand-off.

**Out of Scope:** creation of the candidate's academic failure/backlog/improvement condition; examination policy definition; examination conduct; marks/evaluation; result computation; financial settlement; and technical implementation.

**End Boundary:** A valid registration is confirmed for the authorised opportunity, or the candidate is prevented from registering because policy or eligibility requirements are not satisfied.

## 5. Ownership and Participating Domains

**Examination (I):** owns the examination registration and examination participation outcome.

**Student Lifecycle (C):** owns student academic context and progression/backlog status.

**Academic Delivery (D):** provides curriculum/course/subject applicability.

**Finance (F):** provides financial eligibility where required.

No participating domain takes ownership of the Examination Registration.

## 6. Business Actors, Roles and Responsibilities

- **Candidate (Student):** initiates or participates in registration.
- **Examination Administrator:** manages registration and category validation.
- **Controller of Examination:** governs registration policy and approval.
- **Academic Administration:** validates academic applicability where required.
- **Department Administration:** supports departmental verification where applicable.
- **Student Domain:** supplies authoritative academic status.
- **Finance Domain:** supplies financial eligibility where required.

## 7. Preconditions and Entry Criteria

- An applicable Examination Session is Active.
- An approved Examination Schedule exists.
- The supplementary/backlog/improvement opportunity is permitted by examination policy.
- The candidate has the relevant academic context.
- The requested subject(s) are applicable to the candidate.
- Required financial eligibility is available where policy requires it.
- The registration window or authorised exception permits the registration.

## 8. Trigger

A candidate or authorised institutional process initiates registration for a supplementary, backlog, or improvement examination opportunity.

## 9. Main Business Flow

1. Identify the candidate and applicable examination opportunity.
2. Establish whether the opportunity is supplementary, backlog, or improvement.
3. Validate the candidate's academic status and applicability of the requested subject(s).
4. Apply the governing examination-policy rules for the selected category.
5. Establish the permitted subject registration scope.
6. Evaluate any required financial eligibility.
7. Complete registration review and approval where required.
8. Confirm the Examination Registration.
9. Preserve the registration category and business basis in the registration history.
10. Publish the applicable registration events.
11. Make the confirmed registration available to downstream examination workflows, including the applicable supplementary/backlog/improvement execution workflow.

## 10. Alternate Business Flows

### 10.1 Supplementary Registration

The candidate is registered for the policy-permitted supplementary opportunity and follows the corresponding examination execution path.

### 10.2 Backlog Registration

The candidate is registered for an eligible previously unsuccessful subject or subjects, subject to the applicable policy.

### 10.3 Improvement Registration

The candidate is registered for an authorised improvement opportunity subject to the institution's improvement-examination rules.

### 10.4 Multiple Eligible Subjects

Where policy permits, multiple eligible subjects may be included in the same registration while preserving subject-level applicability.

## 11. Failure, Interruption and Business Recovery

### 11.1 Opportunity Not Applicable

If the candidate does not qualify for the requested opportunity category, registration does not proceed under that category.

### 11.2 Subject Not Eligible

A subject that is not applicable under the candidate's academic and examination context is not registered.

### 11.3 Financial Eligibility Not Satisfied

Where financial eligibility is mandatory, the registration remains pending or is rejected according to the applicable policy; Finance retains ownership of the financial condition.

### 11.4 Registration Window Closed

If the registration window is closed, the candidate follows the permitted late, extended, or exceptional registration path only where policy allows it.

### 11.5 Opportunity Withdrawn or Changed

If the examination opportunity is withdrawn or materially changed before confirmation, the registration does not become authoritative against the obsolete opportunity.

## 12. Business Rules and Decision Constraints

- Supplementary, backlog, and improvement registration is supported by L2-I04.
- Category eligibility is governed by Examination Policy.
- Registration occurs only within an applicable Active Examination Session and approved schedule.
- Candidate and subject eligibility must be established before confirmation.
- Academic ownership of progression/backlog status remains with Student Lifecycle.
- Curriculum ownership remains with the academic/curriculum domain.
- Finance owns financial eligibility and transactions.
- Confirmed Examination Registration is the authoritative input for downstream examination workflows.
- Registration history is auditable.

## 13. State and Lifecycle Impact

The scenario uses the existing Examination Registration lifecycle.

The registration category does not create separate registration states. A supplementary, backlog, or improvement registration progresses through the same governed registration lifecycle while carrying its applicable examination category.

Downstream execution is governed by L2-I12 and does not transfer registration ownership.

## 14. Business Events and Cross-Domain Collaboration

Examination Registration publishes the canonical registration events defined by L2-I04, including:

- **Registration Created**
- **Registration Approved**
- **Registration Confirmed**
- **Registration Withdrawn**
- **Registration Locked**

No new event is introduced for the category itself.

L2-I12 consumes the approved/confirmed registration for supplementary, backlog, or improvement examination execution.

## 15. Business Authorization, Approval and Audit Obligations

- Registration must comply with the applicable examination-policy category rules.
- Required registration approval must be obtained.
- The category and subject scope must remain traceable.
- Significant registration activity must be auditable.
- Any financial eligibility decision remains Finance-owned.
- No technical authorization model is defined at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- Opportunity category is established.
- Candidate and subject eligibility are validated.
- Required financial eligibility is satisfied where applicable.
- Required registration review/approval is complete.
- Registration is confirmed.
- Registration events are published.
- The registration is available to the appropriate downstream examination execution workflow.

**Postconditions:**
- The candidate has an authoritative registration for the approved supplementary, backlog, or improvement scope.
- The prior academic condition remains owned by Student Lifecycle.
- Downstream examination execution can proceed using the confirmed registration.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 explicitly supports Regular, Backlog, Supplementary, Improvement, and Special registration categories; this scenario does not create new category semantics.
2. L2-I12 owns execution of supplementary/backlog/improvement examinations; this scenario stops at authoritative registration and does not duplicate conduct/execution.
3. The exact frozen Stage 3 Type/Priority row was not independently retrievable from the uploaded register during this run; the metadata above should be mechanically rechecked against the frozen Stage 3 row before Review.
