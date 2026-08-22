# Candidate Examination Registration to Confirmed Registration

**Scenario ID:** L3-I-017  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, D, F, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-I01 Examination Policy & Configuration; L2-I02 Examination Session Management; L2-I03 Examination Planning & Scheduling; L2-I04 Examination Registration; Curriculum Framework; Academic Calendar; Student Lifecycle; Finance  
**Referenced By:** L2-I05 Hall Ticket Management; L2-I06 Practical & Viva Examination Conduct; L2-I07 Theory Examination Conduct; downstream examination workflows

## 2. Scenario Purpose

Establish an authoritative Examination Registration for an eligible candidate so that the candidate is formally registered for the applicable examination subjects within an active Examination Session and the registration reaches the canonical Confirmed position for downstream examination operations.

## 3. Business Outcome

A candidate's examination registration is validated against the active examination context, academic applicability, financial eligibility and institutional registration rules; required review and approval are completed; the registration is confirmed as the authoritative business record for the candidate's examination participation.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an eligible candidate initiates registration during an approved registration window for a published examination schedule.

### 4.2 End Boundary

The scenario ends when the Examination Registration becomes Confirmed and is available as the authoritative input to downstream Hall Ticket and examination-conduct workflows.

### 4.3 In Scope

- Examination Session and Schedule identification.
- Candidate and academic context validation.
- Examination eligibility validation.
- Subject registration.
- Registration category selection.
- Financial eligibility validation.
- Institutional review and approval where required.
- Registration confirmation.
- Publication of canonical registration events.
- Auditability of significant registration decisions.

### 4.4 Out of Scope

- Examination planning and timetable publication.
- Hall Ticket generation.
- Examination conduct.
- Marks collection and result processing.
- Student ownership.
- Curriculum ownership.
- Finance transaction processing.
- Technical APIs, databases, queues, retry mechanisms or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| Examination Registration | Owns the Examination Registration and its lifecycle. | Registration remains I04-owned. |
| Student / Student Lifecycle | Provides candidate academic status and student context. | Student concepts remain C-owned. |
| Curriculum Framework | Provides programme, curriculum and subject applicability. | Curriculum concepts remain D-owned. |
| Finance | Provides financial eligibility/payment status where required. | Financial facts remain F-owned. |

L2-I04 states that registration collaborates with multiple domains while ownership remains with the originating domain. fileciteturn246file0

## 6. Business Actors, Roles and Responsibilities

L2-I04 defines the following business participants:

- Candidate (Student)
- Examination Administrator
- Controller of Examination
- Department Administration
- Academic Administration
- Finance Domain
- Curriculum Framework Domain
- Academic Calendar Domain
- Student Domain
- Audit Infrastructure

The scenario uses these responsibilities as defined upstream and does not invent additional authority. fileciteturn246file0

## 7. Preconditions and Entry Criteria

- Examination Policy is established.
- Examination Session is Active.
- Examination Schedule is Published.
- Registration window is open or an authorised late/special registration condition applies.
- Candidate satisfies applicable eligibility requirements.
- Applicable subjects are valid under the Curriculum Framework.
- Financial eligibility is satisfied where required.
- Required institutional review/approval requirements are available.

L2-I04 explicitly requires an Active Examination Session, Published Examination Schedule and satisfied institutional eligibility before registration proceeds. fileciteturn245file0turn247file0

## 8. Trigger

The candidate initiates an examination registration for a scheduled examination within the applicable registration window.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the active Examination Session and published examination schedule. | Examination Registration | Valid examination context is established. |
| 2 | Identify the candidate's academic status, programme, academic period and curriculum context. | Student / Curriculum domains | Candidate context is available. |
| 3 | Validate registration eligibility, including academic, examination-policy, registration-window, financial and institutional restrictions as applicable. | Examination Registration with participating domains | Candidate is eligible to proceed or registration is stopped. |
| 4 | Register the applicable examination subjects and examination category. | Examination Registration | Examination scope is established. |
| 5 | Submit the registration for institutional review where required. | Candidate / Examination Administration | Registration enters governed review. |
| 6 | Complete registration review and obtain required approval. | Examination Administration / Controller / applicable authority | Registration becomes approved. |
| 7 | Confirm the Examination Registration as the authoritative record for candidate participation. | Examination Registration | Registration becomes Confirmed. |
| 8 | Publish `Registration Approved` and `Registration Confirmed` business events. | Examination Registration | Downstream examination workflows receive authoritative business facts. |
| 9 | Preserve the registration audit history. | Examination Registration / Audit Infrastructure | The registration is historically traceable. |

L2-I04 defines the registration lifecycle through eligibility validation, subject registration, review, approval and confirmation, with only confirmed registrations eligible for Hall Ticket generation. fileciteturn246file0turn247file0

## 10. Alternate Business Flows

### A1 — Late Registration

Where the normal registration window has closed but institutional late-registration policy permits participation, the registration proceeds under the governed late-registration rules.

### A2 — Supplementary, Backlog or Improvement Registration

The registration category is selected according to the institutional examination policy and the candidate's applicable academic context.

### A3 — Financial Eligibility Failure

Where required financial eligibility is not satisfied, registration remains pending or is rejected according to institutional policy. Finance remains owner of the financial eligibility fact. fileciteturn247file0

### A4 — Registration Review Requires Correction

The registration returns for correction before approval. No confirmation occurs until the reviewed registration satisfies the applicable requirements.

### A5 — Approved Special Permission

Where an authorised exception permits registration outside the normal path, the registration is governed under the applicable special-permission rules and remains fully auditable.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Examination Session is not Active | Registration does not proceed. |
| Examination Schedule is not Published | Registration does not proceed. |
| Candidate fails eligibility | Registration does not progress until eligibility requirements are satisfied. |
| Subject is not applicable | Invalid subject is not registered; eligible subjects may continue according to policy. |
| Financial eligibility is not satisfied | Registration remains pending or is rejected according to institutional policy. |
| Duplicate registration is detected | Existing valid registration remains authoritative and duplicate creation is prevented. |
| Approval is rejected | Registration does not become Confirmed. It may return for correction, resubmission or withdrawal according to policy. |
| Registration becomes invalid after review but before confirmation | Revalidate the relevant condition before confirmation; do not confirm an invalid registration. |

L2-I04 explicitly defines controlled handling for eligibility failure, closed windows, subject failure, financial eligibility failure, duplicates and administrative rejection. fileciteturn247file0

## 12. Business Rules and Decision Constraints

- Registration requires an Active Examination Session.
- Registration requires a Published Examination Schedule.
- Candidate eligibility must be satisfied.
- Registration must occur within an approved registration window unless an authorised exception applies.
- Every Examination Registration includes one or more valid examination subjects.
- Subject applicability follows the Curriculum Framework.
- Only approved registrations may become Confirmed.
- Confirmed registrations are the authoritative business input for downstream examination workflows.
- Significant registration activities are auditable.
- Ownership of student, curriculum and financial facts remains with originating domains.
- Registration does not create or own admission, academic, finance or curriculum policy.

fileciteturn245file0turn247file0

## 13. State and Lifecycle Impact

The canonical Examination Registration lifecycle includes:

`Registration Initiated → Eligibility Validation → Subject Registration → Registration Review → Registration Approval → Registration Confirmation → Registration Locking → Operational Consumption → Registration Completion → Registration Archival`

Optional amendment and withdrawal paths exist before applicable completion/locking boundaries. fileciteturn246file0turn247file0

This scenario ends at **Registration Confirmation** and does not introduce new states or transitions.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Business Events

L2-I04 defines:

- `Registration Initiated`
- `Eligibility Validated`
- `Registration Submitted`
- `Registration Approved`
- `Registration Rejected`
- `Registration Confirmed`
- `Registration Amended`
- `Registration Withdrawn`
- `Registration Locked`
- `Registration Archived`

The Examination Registration domain is the canonical publisher of these events. fileciteturn247file0

### 14.2 Business Hand-offs

| From | To | Business Fact | Purpose |
|---|---|---|---|
| Examination Session Management | Examination Registration | Active Session fact | Allows registration to begin only within an active session. |
| Examination Planning & Scheduling | Examination Registration | Published Examination Schedule | Establishes the examinable schedule. |
| Student | Examination Registration | Candidate academic context | Supports candidate eligibility. |
| Curriculum Framework | Examination Registration | Programme/course/subject applicability | Determines valid examination subjects. |
| Finance | Examination Registration | Financial eligibility | Supports financial participation rules where required. |
| Examination Registration | Hall Ticket Management | `Registration Approved` / `Registration Confirmed` | Enables downstream Hall Ticket processing. |

Events remain distinct from synchronous business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

L2-I04 defines Examination Administrator, Controller of Examination, Department Administration and Academic Administration responsibilities. The exact approval authority depends on institutional policy; no new authority is invented.

### 15.2 Separation of Duties

The scenario applies only the upstream requirement for review and approval where required. No additional separation-of-duty rule is introduced.

### 15.3 Audit Evidence

Significant registration activities must remain traceable, including:

- registration creation;
- eligibility validation;
- subject registration;
- review;
- approval;
- confirmation;
- amendments or withdrawal where applicable.

L2-I04 explicitly requires complete auditability for significant registration activities. fileciteturn246file0turn247file0

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Active Examination Session is confirmed.
- Published Examination Schedule is identified.
- Candidate eligibility is satisfied.
- Applicable subjects are registered.
- Required review and approval are complete.
- Registration is Confirmed.
- `Registration Confirmed` is published.
- Registration history is auditable.

### 16.2 Postconditions

- Candidate possesses an authoritative Confirmed Examination Registration.
- Registration is eligible for downstream Hall Ticket and examination-conduct processing.
- Student, Curriculum and Finance ownership remains unchanged.
- No Hall Ticket or examination-conduct state is created by this scenario.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material ownership, lifecycle, event-publisher or business-boundary conflict was identified for the core I017 flow in the accessible locked L2-I04 architecture.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

Institution-specific approval chains remain governed by upstream examination policy and authorization architecture and are not invented locally.
