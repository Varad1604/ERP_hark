# Flexible Learning Opportunity to Governed Flexible Learning Enrollment

# 1. Scenario Metadata

**Scenario ID:** L3-D-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0755 / CAN-L3-D-035 (final L3-D-034)

---

# 2. Scenario Purpose

Govern an eligible student request for an Active flexible-learning opportunity through D06 validation and applicable institutional approval to an approved Flexible Learning Enrollment.

# 3. Business Outcome

A unique Flexible Learning Enrollment reaches Approved and becomes ready for the applicable D06 seat-allocation or external-provider registration path.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An eligible student selects an Active discoverable learning opportunity during its governed enrollment window.

## 4.2 End Boundary

The enrollment reaches Approved and `EnrollmentApproved` is published, or a governed rejection closes the unsuccessful request with reason.

## 4.3 In Scope

- Submit the enrollment request.
- Validate eligibility, enrollment window, applicable academic policy, capacity/finance references and duplicates as defined by D06.
- Apply the configured approval path.
- Record Approved or Rejected outcome with evidence.

## 4.4 Out of Scope

- Section C Student/core-registration ownership.
- Seat-allocation outcome.
- Provider registration or learning completion.
- Validation-service, approval-service, API, retry or persistence design.

## 4.5 Related Scenarios — Optional

L3-D-035 governs seat allocation after Approved enrollment where applicable; L3-D-037 governs external-provider registration where applicable.

# 5. Ownership and Participating Domains

D06 owns Flexible Learning Enrollment, eligibility/approval evidence and enrollment lifecycle. Section C retains canonical student/program/academic-standing/core-registration ownership.

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Student | Selects the opportunity and submits the student's own enrollment request. | Initiates own enrollment. |
| Academic Advisor / HoD / Dean / Academic Affairs, where assigned by D06 policy | Reviews enrollment when the configured institutional approval path requires human decision. | Approval authority only where established by policy. |

Validation and recording the approved/rejected D06 outcome are domain responsibilities, not `Enrollment Validation Service` or `Approval Service` business actors.

# 7. Preconditions and Entry Criteria

- Opportunity is Active/discoverable and the enrollment window is Open.
- Student and required eligibility references are authoritative.
- No duplicate Active enrollment exists for the governed scope.

# 8. Trigger

The student submits enrollment for the flexible-learning opportunity.

# 9. Main Business Flow

1. Student/D06 establishes the enrollment request — `EnrollmentRequestSubmitted`.
2. D06 validates authoritative eligibility, window, policy, capacity/finance references and duplicates — `EnrollmentRequestValidated` or `EnrollmentValidationFailed`.
3. D06 evaluates the applicable approval policy.
4. Where policy permits approval without human review, D06 records `EnrollmentAutomaticallyApproved` and proceeds to Approved.
5. Where human review is required, the configured academic approver decides — `AcademicApprovalRequested` / `AcademicApprovalCompleted`.
6. D06 records `EnrollmentApproved` for a satisfied approval outcome; a governed rejection records `ElectiveEnrollmentRejected`.

# 10. Alternate Business Flows

## 10.1 Approval Without Human Review

Where authoritative D06 policy permits automatic approval after successful validation, the enrollment reaches the same Approved business outcome without human-review steps.

# 11. Failure, Interruption and Business Recovery

Validation/reference failure blocks approval until authoritative business conditions are corrected. A rejected human approval retains the governed rejection outcome and reason. An interruption preserves the last authoritative D06 enrollment position and evidence until the blocking condition is resolved.

# 12. Business Rules and Decision Constraints

D06 alone owns enrollment state; Section C facts remain externally owned; approval follows authoritative D06 policy; repeated same business fact must not create duplicate institutional enrollment outcomes. Technical idempotency/retries are deferred.

# 13. State and Lifecycle Impact

Flexible Learning Enrollment: Created → Eligibility Validation → Pending Approval → Approved; governed alternate Rejected. Owner D06. Source: L2-D06 Aggregate 1; W17–W22.

# 14. Business Events and Cross-Domain Collaboration

D06 publishes `EnrollmentRequestSubmitted`, `EnrollmentRequestValidated`, `EnrollmentValidationFailed`, `EnrollmentAutomaticallyApproved`, `AcademicApprovalRequested`, `AcademicApprovalCompleted`, `EnrollmentApproved`, and `ElectiveEnrollmentRejected` as defined by L2-D06 W17–W22/Event Catalogue.

Section C supplies authoritative student/program/standing/registration facts without ownership transfer.

# 15. Business Authorization, Approval and Audit Obligations

Student initiates own request. Where D06 policy requires human approval, only the assigned academic business authority may decide. Audit retains eligibility/policy references, approval path, actor/authority, decision/reason and resulting enrollment state.

# 16. Completion Criteria and Postconditions

Approved enrollment is unique and ready for its next D06 allocation/provider-registration path; rejection remains traceable; D06 ownership and Section C ownership remain unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced by this purity correction. Validation/approval services, APIs, technical RBAC, retries and persistence are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
