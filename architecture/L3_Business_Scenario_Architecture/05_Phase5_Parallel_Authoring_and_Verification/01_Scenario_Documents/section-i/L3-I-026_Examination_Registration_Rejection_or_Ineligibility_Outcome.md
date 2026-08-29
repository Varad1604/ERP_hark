# Examination Registration Rejection or Ineligibility Outcome

**Scenario ID:** L3-I-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, D, F, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I01 Examination Policy & Configuration; L2-I04 Examination Registration Workflows; applicable Student, Curriculum/Academic Delivery, Finance, Examination Session and Examination Schedule architecture  
**Referenced By:** Downstream examination workflows consuming approved/confirmed registrations

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-026 |
| Canonical Title | Examination Registration Rejection or Ineligibility Outcome |
| Primary Owner | Examination Registration |
| Participating Domains | C, D, F, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the business outcome when an Examination Registration cannot proceed because the candidate or requested registration scope is not eligible, or because a required institutional registration condition is not satisfied.

The scenario ensures that an ineligible or rejected registration does not become an authoritative confirmed examination participation record, while preserving the reasoned business outcome and audit history.

## 3. Business Outcome

The Examination Registration receives an authoritative rejection or ineligibility outcome, the affected registration does not progress to confirmation, the applicable business reason is preserved, and the candidate or authorised institutional process has a clear path for correction or subsequent eligible resubmission where policy permits.

## 4. Scenario Boundary and Scope

**Start Boundary:** A candidate registration has entered Examination Registration and one or more eligibility, policy, academic, financial, or institutional conditions prevents it from progressing.

**In Scope:**
- Establishing the affected registration and candidate context.
- Evaluating registration eligibility and required conditions.
- Identifying the failed or unsatisfied business requirement.
- Applying the appropriate rejection or ineligibility outcome.
- Preserving the registration history and audit evidence.
- Communicating the business outcome through the canonical registration event.
- Allowing correction and resubmission where upstream policy permits.

**Out of Scope:**
- Defining or changing examination eligibility policy.
- Determining academic eligibility owned by Student/Curriculum domains.
- Determining financial clearance owned by Finance.
- Changing examination session or schedule governance.
- Hall ticket generation or examination conduct.
- Technical error handling, retry mechanisms, queues, or implementation mechanics.

**End Boundary:** The registration is rejected or remains unable to progress under the applicable business outcome, with the authoritative reason and history preserved.

## 5. Ownership and Participating Domains

**Examination Registration (I) — Primary Owner:** Owns the registration workflow and authoritative rejection/ineligibility outcome.

**Student Lifecycle (C):** Supplies candidate academic status and enrolment information; it retains ownership of those records.

**Academic Delivery / Curriculum Context (D):** Supplies programme, course, subject, and academic applicability information where applicable; it does not own the Examination Registration.

**Finance (F):** Supplies financial eligibility or payment status where institutional policy requires it; Finance retains ownership of financial obligations and clearance.

No participating domain receives ownership of the Examination Registration because it supplies information used in the decision.

## 6. Business Actors, Roles and Responsibilities

- **Candidate (Student):** Provides or corrects registration information and may resubmit when permitted.
- **Examination Administrator:** Reviews registration eligibility outcomes and manages administrative registration processing.
- **Controller of Examination:** Governs examination registration policies and approvals where required.
- **Department Administration:** Verifies departmental registration requirements where applicable.
- **Academic Administration:** Validates academic eligibility where applicable.
- **Finance Domain:** Provides financial eligibility and payment status without owning the registration decision.

## 7. Preconditions and Entry Criteria

- An Examination Registration has been initiated or submitted.
- The applicable Examination Session is Active.
- An approved Examination Schedule has been published.
- Candidate and registration context are identifiable.
- The applicable examination, academic, and financial eligibility information is available.
- Registration is being evaluated within the applicable registration window or authorised exception.

## 8. Trigger

Eligibility validation or institutional registration review identifies a condition that prevents the candidate or requested registration scope from proceeding.

## 9. Main Business Flow

1. The Examination Registration and candidate context are established.
2. The registration is evaluated against applicable Examination Policy requirements.
3. Student, academic, curriculum, and financial information required for eligibility is considered from the owning domains.
4. Any failed or unsatisfied eligibility condition is identified.
5. Examination Registration determines the appropriate business disposition under the applicable registration policy.
6. Where the outcome is rejection, the registration is moved to the source-defined rejected business outcome and is not progressed to confirmation.
7. The rejection or ineligibility reason is preserved as part of the registration's business history.
8. The **Registration Rejected** event is published by Examination Registration.
9. The candidate or authorised institutional process may correct the underlying condition and resubmit where institutional policy permits.
10. A later successful resubmission follows the normal registration lifecycle rather than bypassing validation.

## 10. Alternate Business Flows

### 10.1 Academic Eligibility Failure

Where Student or academic information does not satisfy the applicable examination eligibility policy, the registration is rejected or prevented from progressing. Academic ownership remains with the originating domain.

### 10.2 Subject Eligibility Failure

Where one or more requested subjects are not applicable, those subjects are not accepted into the authoritative registration. Eligible scope may continue only where institutional policy explicitly permits partial registration.

### 10.3 Financial Eligibility Failure

Where financial eligibility is required and is not satisfied, the registration remains pending or is rejected according to the applicable institutional policy. Finance retains ownership of the financial condition.

### 10.4 Registration Window Failure

Where the registration is outside the approved registration window and no permitted exception applies, the registration is rejected.

### 10.5 Administrative Rejection

Where institutional review determines that the submitted registration does not satisfy a required administrative condition, the registration is rejected and may be corrected and resubmitted where policy permits.

## 11. Failure, Interruption and Business Recovery

### 11.1 Required Eligibility Information Unavailable

If a required eligibility outcome cannot be established, the registration shall not be confirmed until the authoritative information is available.

### 11.2 Rejection Disputed

A disputed rejection follows the applicable institutional review or correction process. The existing rejection remains the authoritative business outcome until a valid review changes it.

### 11.3 Underlying Condition Corrected

When the failed condition is corrected by its owning domain or through an authorised institutional process, the candidate may resubmit where policy permits. The new submission follows the standard eligibility and registration lifecycle.

### 11.4 Rejection After Partial Subject Selection

If only part of the requested registration scope is invalid, the outcome follows institutional subject-registration policy. No local rule is introduced to force partial acceptance.

## 12. Business Rules and Decision Constraints

- Only eligible candidates may register for examinations.
- An Active Examination Session and officially published Examination Schedule are prerequisites for registration.
- Eligibility may consider academic status, programme applicability, course/subject applicability, examination rules, financial eligibility, and institutional restrictions.
- Failed validations prevent registration from progressing.
- Registration outside approved windows requires an authorised institutional exception.
- Financial eligibility remains owned by Finance.
- Academic and curriculum eligibility remain owned by the originating academic domains.
- Rejected registrations shall not become confirmed registrations.
- Significant registration activities, including rejection, are auditable.
- A rejection does not create a new examination policy or transfer ownership of any upstream rule.

## 13. State and Lifecycle Impact

The scenario uses the Examination Registration lifecycle defined by L2-I04.

The registration may remain in a validation/review position while eligibility is unresolved. When the business decision is rejection, the registration receives the source-defined rejected outcome and cannot progress to **Confirmed** unless a later permitted correction and resubmission establishes eligibility.

The scenario does not invent a new lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

**Canonical publisher:** Examination Registration.

Relevant events include:

- **Registration Initiated** — identifies creation of the registration request.
- **Eligibility Validated** — records completion of eligibility verification.
- **Registration Submitted** — records submission for institutional processing.
- **Registration Rejected** — published when the authoritative rejection outcome is established.
- **Registration Confirmed** — may occur only on a later successful registration path.

The Examination Registration domain consumes academic, student, financial, policy, session, and schedule information from their canonical owners. No new cross-domain event is invented for the rejection scenario.

## 15. Business Authorization, Approval and Audit Obligations

- Rejection shall follow the applicable Examination Policy and institutional registration rules.
- Where institutional review is required, the authorised examination administration process determines the outcome.
- The reason for rejection or ineligibility must remain traceable.
- Significant registration activity, including rejection and administrative decisions, must be auditable.
- The originating domain retains ownership of the rule or business information that caused the outcome.
- No technical IAM/RBAC or enforcement design is defined at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The affected registration and candidate are identified.
- Applicable eligibility conditions have been evaluated.
- The failed condition or rejection basis is established.
- The registration receives the authoritative rejection/ineligibility outcome.
- The reason and business history are preserved.
- **Registration Rejected** is published when rejection is established.

**Postconditions:**
- The rejected registration is not a confirmed examination participation record.
- Downstream operational examination workflows do not consume it as an approved/confirmed registration.
- A corrected and permitted resubmission must re-enter the governed registration lifecycle.
- Ownership of academic, financial, policy, and student information remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 explicitly defines **Registration Rejected** as a published event and defines administrative rejection outcomes, but it allows some institutional behaviour such as pending versus rejected financial eligibility to be configured. This scenario therefore does not impose a universal local disposition where the source leaves it configurable.
2. The exact reason-code taxonomy for rejection is not defined at L2; no new local reason taxonomy is introduced.
3. Partial subject acceptance following mixed eligibility is governed by institutional subject-registration policy; no additional rule is invented here.
