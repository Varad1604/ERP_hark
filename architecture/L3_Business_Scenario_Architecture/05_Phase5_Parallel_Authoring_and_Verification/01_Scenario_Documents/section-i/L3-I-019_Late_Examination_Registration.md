# Late Examination Registration

**Scenario ID:** L3-I-019  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, F, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-I01 Examination Policy & Configuration; L2-I02 Examination Session Management; L2-I03 Examination Planning & Scheduling; L2-I04 Examination Registration; Student; Finance  
**Referenced By:** L2-I05 Hall Ticket Management; downstream examination workflows

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-019 |
| Canonical Title | Late Examination Registration |
| Primary Owner | Examination Registration |
| Participating Domains | C, F, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern an examination registration submitted after the standard registration window while preserving institutional policy, approval authority, financial eligibility and the canonical Examination Registration lifecycle.

The scenario establishes a valid late registration only when the applicable institutional late-registration policy or authorised exception permits it.

## 3. Business Outcome

A late examination registration is either accepted and becomes a Confirmed Examination Registration under the applicable late-registration governance, or is rejected without creating an unauthorised examination participation record.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a candidate seeks examination registration after the standard registration window has closed.

### 4.2 End Boundary

The scenario ends when the late registration is Confirmed under an authorised exception path or is rejected, with the decision and business history preserved.

### 4.3 In Scope

- Detection of late registration condition.
- Determination of late-registration policy applicability.
- Eligibility validation.
- Financial eligibility/payment status where applicable.
- Exceptional approval.
- Subject registration.
- Registration confirmation.
- Audit and canonical business events.

### 4.4 Out of Scope

- Examination policy authoring.
- Schedule creation.
- Hall Ticket generation.
- Examination conduct.
- Finance transaction processing.
- Technical deadlines, timers or implementation mechanisms.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| Examination Registration | Owns the Examination Registration and late-registration lifecycle. | I04-owned. |
| Student | Provides candidate status and context. | Student remains C-owned. |
| Finance | Provides financial eligibility/payment status where required. | Finance facts remain F-owned. |
| Examination Policy | Defines institutional late-registration policy. | Policy remains I01-owned. |

L2-I04 explicitly identifies late registration as a capability and states that registrations outside approved windows require authorised institutional exceptions.

## 6. Business Actors, Roles and Responsibilities

The upstream I04 actor catalogue includes Candidate (Student), Examination Administrator, Controller of Examination, Department Administration, Academic Administration and Finance.

The scenario does not invent a distinct “Late Registration Officer” role.

## 7. Preconditions and Entry Criteria

- Examination Session is Active.
- Examination Schedule is Published.
- Standard registration window has closed.
- An institutional late-registration rule, extended/special registration window, or authorised exception exists.
- Candidate remains academically eligible.
- Financial eligibility is satisfied where required.
- Required late-registration approval authority is available.

## 8. Trigger

A candidate or authorised institution submits an examination registration after the normal registration deadline.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Identify the candidate and applicable examination context. | Examination Registration | Candidate and examination context are established. |
| 2 | Confirm that the standard registration window has closed. | Examination Registration | The request is classified as late. |
| 3 | Determine whether an authorised late, extended or special registration rule applies. | Examination Registration / Examination Policy | A governed late-registration path is selected. |
| 4 | Revalidate candidate eligibility, subject applicability and financial eligibility where required. | Examination Registration with Student/Curriculum/Finance inputs | The late request is eligible or rejected. |
| 5 | Register applicable examination subjects and category. | Examination Registration | Examination scope is established. |
| 6 | Obtain the required institutional exception/approval. | Examination Administration / Controller / authorised authority | Late registration is authorised or refused. |
| 7 | Confirm the Examination Registration. | Examination Registration | Candidate has an authoritative late registration. |
| 8 | Publish the canonical registration events and preserve the complete audit history. | Examination Registration / Audit | Downstream workflows receive an authoritative business fact. |

L2-I04 states that registrations outside approved windows require authorised institutional exceptions and includes Late Registration as a core capability.

## 10. Alternate Business Flows

### A1 — Late Registration Window Open

An institution may configure a Late Registration or Extended Registration window. The request proceeds under the configured policy without requiring a discretionary exception outside the window.

### A2 — Special Registration Permission

Where an authorised special-registration policy applies, the candidate proceeds through the permitted exception path.

### A3 — Financial Eligibility Failure

Where financial eligibility is required and is not satisfied, registration remains pending or is rejected according to institutional policy.

### A4 — Late Registration Rejected

The registration is not Confirmed and the rejection remains auditable.

### A5 — Valid Registration Already Exists

If an existing valid registration is found, the late request does not create a duplicate registration.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| No late-registration policy or authorised exception exists | Reject the late registration request. |
| Candidate fails eligibility | Do not confirm the late registration. |
| Subject is not applicable | Do not register the invalid subject. |
| Financial eligibility fails | Keep pending or reject according to financial/registration policy. |
| Required exception approval is denied | Registration does not become Confirmed. |
| Duplicate registration detected | Existing valid registration remains authoritative. |
| Registration becomes invalid before confirmation | Revalidate the affected condition before confirmation. |

L2-I04 explicitly requires closed-window registrations to be rejected unless late/special policy permits them and prevents duplicate registration.

## 12. Business Rules and Decision Constraints

- Registrations occur only in approved registration windows unless an authorised exception applies.
- Late registration must remain subject to Examination Session, Schedule, eligibility, subject and institutional policy requirements.
- Required approvals must be obtained.
- Only approved registrations may become Confirmed.
- Duplicate registrations are not permitted.
- Financial eligibility remains owned by Finance.
- Registration audit history must be preserved.
- Late registration does not create a separate Registration aggregate.

## 13. State and Lifecycle Impact

The late request follows the canonical Examination Registration lifecycle:

`Registration Initiated → Eligibility Validation → Subject Registration → Registration Review → Registration Approval → Registration Confirmation`

The scenario does not introduce a new `Late` state.

The late nature is a governed business condition determined from the registration window and policy context.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Events

Relevant I04 events include:

- `Registration Initiated`
- `Eligibility Validated`
- `Registration Submitted`
- `Registration Approved`
- `Registration Rejected`
- `Registration Confirmed`
- `Registration Archived`

There is no separate `LateRegistrationApproved` event in the authoritative I04 event catalogue; none is invented.

### 14.2 Business Hand-offs

| From | To | Business Fact | Purpose |
|---|---|---|---|
| Examination Policy & Configuration | Examination Registration | Late/Extended/Special registration policy | Determines whether late registration is permitted. |
| Student | Examination Registration | Candidate academic context | Supports eligibility validation. |
| Finance | Examination Registration | Financial eligibility/payment status | Supports financial participation rules. |
| Examination Registration | Hall Ticket Management | `Registration Confirmed` | Enables downstream Hall Ticket processing. |

Events remain distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

I04 identifies Controller of Examination, Examination Administrator, Department Administration and Academic Administration as relevant actors. The exact late-registration authority is governed by institutional policy; it is not invented here.

### 15.2 Separation of Duties

No additional late-registration separation-of-duty requirement is defined in I04.

### 15.3 Audit Evidence

Late-registration evidence must preserve:

- standard registration window context;
- late-registration policy/exception basis;
- candidate eligibility outcome;
- financial eligibility where applicable;
- subjects/category;
- approval outcome;
- confirmation/rejection;
- relevant registration events.

L2-I04 requires significant registration activities to be auditable.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

For an approved late registration:

- standard registration window closure is established;
- authorised late/extended/special permission exists;
- eligibility is satisfied;
- required approval is complete;
- registration is Confirmed;
- `Registration Confirmed` is published;
- complete audit history is retained.

For a rejected late registration:

- the rejection is authoritative and auditable;
- the registration does not become Confirmed.

### 16.2 Postconditions

- An approved late registration is a normal canonical Examination Registration distinguished by its governed late-registration basis.
- Downstream examination workflows may consume the Confirmed registration.
- No new late-registration state or aggregate is introduced.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

I04 defines the existence of Late, Extended and Special Registration windows and authorised exceptions, but does not prescribe one universal institutional approval hierarchy for every late-registration case.

Therefore this L3 preserves the requirement for authorised exception/approval without inventing an institution-specific authority chain.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

Institution-specific late-registration fees, approval thresholds or special permissions remain configuration/policy concerns and must be resolved in the relevant upstream architecture rather than created locally in L3.
