# Employee Qualification Registration and Verification

**Scenario ID:** L3-E-060  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce; H — Institutional Services & Intelligence  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E07 — Employee Profile Architecture; L2-H10 — External Education Integration Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-060 |
| Canonical Title | Employee Qualification Registration and Verification |
| Primary Owner | Employee Profile |
| Participating Domains | E — People & Workforce; H — Institutional Services & Intelligence |
| Scenario Type | Cross-domain |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Register an employee's academic qualification in the canonical Employee Profile and, where verification of an externally issued credential is required, coordinate the governed external verification process so that the qualification's verification outcome is available without transferring ownership of the employee profile or the external integration capability.

The scenario preserves the distinction between Employee Profile ownership of the qualification record and Institutional Services & Intelligence ownership of external education integration orchestration.

## 3. Business Outcome

The employee's qualification is registered in the canonical EmployeeProfile and the qualification verification process reaches a governed outcome, with verified external credentials reflected through the Employee Profile's verification context where the authoritative business process permits that outcome.

A verification failure or inability to establish authenticity does not create an unverified qualification as if it were verified.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a valid employee qualification is presented for registration in Employee Profile and the qualification requires registration and, where applicable, external verification.

### 4.2 End Boundary

The scenario ends when the qualification is registered and the applicable verification process has produced a governed verification outcome.

### 4.3 In Scope

- Registration of an academic qualification in Employee Profile.
- Validation of mandatory qualification information.
- Prevention of duplicate qualification registration.
- Preservation of qualification history.
- Initiation of external credential verification where required.
- Coordination with External Education Integration for externally issued credential verification.
- Recording the governed verification outcome within the employee qualification/profile context where supported by the authoritative profile process.
- Publication of the authoritative qualification and external-verification business facts.

### 4.4 Out of Scope

- Workforce engagement lifecycle.
- Employee personal/contact profile maintenance.
- Professional certification lifecycle.
- Skill and competency lifecycle.
- Language profile lifecycle.
- Profile document lifecycle except for evidence directly required by the qualification verification process.
- Academic qualification ownership outside the employee profile.
- External provider or connector lifecycle.
- External integration configuration.
- Academic equivalency or institutional academic recognition decisions.
- Technical credential transport, provider APIs, queues, retries, or integration implementation.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to clarify this boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Section E — Employee Profile | Owns the Qualification record within EmployeeProfile, its registration, historical preservation, duplicate prevention, and profile-level qualification information. |
| Section H — External Education Integration | Owns the orchestration of external credential verification and the integration/provider interaction required to determine authenticity of an externally issued credential. |

External Education Integration does not become the owner of the employee qualification. Employee Profile does not become the owner of external integration orchestration.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Employee | Provides the qualification information and supporting evidence/reference required for registration. | Initiates or supplies qualification information. |
| HR Administrator | Registers and maintains employee qualification information within Employee Profile. | Performs qualification administration. |
| HR Executive | Supports qualification registration and profile administration. | Performs qualification administration. |
| Verification Officer | Performs or oversees the business verification activity supported by Employee Profile. | Reviews verification evidence/outcome. |
| External Education Integration | Coordinates authenticity verification with the relevant external education provider. | Performs external verification orchestration as the canonical H-domain capability. |

L2-E07 identifies Verification Officer as a business actor. L2-H10 defines the external verification capability and its provider-facing orchestration but does not establish a new institutional approval role.

## 7. Preconditions and Entry Criteria

- The employee exists.
- A canonical EmployeeProfile exists.
- The qualification type is available and valid.
- The institution associated with the qualification is identified.
- The qualification is not already registered as a duplicate in the EmployeeProfile.
- Required qualification information is available.
- Where external verification is required, the credential reference/evidence necessary for verification is available.
- The external verification capability and relevant provider context are governed and available for the business verification request.
- The qualification belongs to the EmployeeProfile context and does not represent a request to modify another canonical academic record.

## 8. Trigger

An employee or authorized HR participant submits an academic qualification for registration in the Employee Profile, with external verification required where the qualification's evidence or institutional policy calls for it.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The employee qualification is presented for registration with the required qualification information and evidence/reference. | Employee / HR Administrator / HR Executive | A qualification registration request exists. |
| 2 | Employee Profile validates that qualification type and institution information are present and valid and checks for an existing duplicate qualification. | Employee Profile | The qualification is eligible for registration if the business rules are satisfied. |
| 3 | Employee Profile registers the qualification as a historical employee qualification record. | Employee Profile | The qualification becomes part of the canonical EmployeeProfile. |
| 4 | Employee Profile publishes the authoritative `QualificationAdded` business event. | Employee Profile | The enterprise has an authoritative fact that the qualification was added. |
| 5 | Where external verification is required, the qualification's credential reference is submitted for governed verification through External Education Integration. | Employee Profile / External Education Integration | An external credential verification process is initiated. |
| 6 | External Education Integration coordinates the authenticity check with the authorized external education provider and evaluates the returned business verification result according to its governed verification workflow. | External Education Integration | A verification outcome is established: validated or failed. |
| 7 | External Education Integration publishes the canonical verification outcome for the external credential verification process. | External Education Integration | The verification outcome becomes an authoritative H-domain business fact. |
| 8 | The employee profile verification context is updated to reflect the governed verification outcome where the Employee Profile process supports the qualification verification linkage. | Employee Profile | The qualification's verification status is aligned with the governed verification outcome. |
| 9 | The completed registration and verification outcome are available to authorized employee-profile consumers without transferring ownership to External Education Integration. | Employee Profile / External Education Integration | The qualification is registered and its verification outcome is governed and traceable. |

The Main Business Flow stops after the qualification registration and applicable verification outcome have been established.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | The qualification is internally evidenced and does not require external verification. | Employee Profile registers the qualification and completes the scenario without invoking external credential verification. | The qualification is registered with no external verification path required. | Employee Profile |
| A2 | The same qualification is already registered. | Employee Profile does not create a duplicate qualification record. | The existing historical qualification remains authoritative. | Employee Profile |
| A3 | The qualification information is valid but the institution or qualification reference is incomplete. | Registration does not proceed until the required qualification information is complete. | No incomplete qualification is treated as a completed registration. | Employee Profile |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Qualification type is missing. | The qualification cannot be validly registered. | Correct the qualification information and resubmit. | Employee Profile |
| Institution information is missing. | The qualification cannot be validly registered. | Supply the required institution information and resubmit. | Employee Profile |
| Duplicate qualification is identified. | A second canonical qualification record must not be created. | Retain and use the existing qualification record. | Employee Profile |
| External verification cannot establish authenticity. | The qualification cannot be represented as externally verified. | Record the governed verification failure/unverified outcome and preserve the registered qualification without falsely marking it verified. | External Education Integration / Employee Profile |
| External provider verification requires business exception handling. | The verification outcome remains unresolved until the governed external verification process reaches an outcome. | Follow the governed external verification exception path. | External Education Integration |

Technical retry, queue, timeout, transport, or infrastructure recovery details are excluded from L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Qualification type is mandatory. | Employee Profile | Registration cannot complete without qualification type. | L2-E07 |
| Institution is mandatory. | Employee Profile | Registration cannot complete without institution information. | L2-E07 |
| Historical qualifications cannot be physically deleted. | Employee Profile | Qualification history is preserved after registration. | L2-E07 |
| Duplicate qualifications are prevented. | Employee Profile | Existing qualification records remain canonical. | L2-E07 |
| External Education Integration does not own the underlying credential or employee qualification. | External Education Integration | H10 performs verification orchestration only. | L2-H10 |
| External credential verification establishes authenticity or failure and publishes a canonical verification result. | External Education Integration | The scenario distinguishes verified from unverified outcomes. | L2-H10 — Certificate Verification |
| Employee Profile owns qualification information within EmployeeProfile. | Employee Profile | The qualification remains an E07 business fact even when H10 participates. | L2-E07 |
| Academic decisions outside employee profile qualification verification remain with their canonical academic domains. | Relevant owning domain | This scenario does not create academic equivalency or academic recognition decisions. | L2-H10 boundary; L1 ownership |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| EmployeeProfile / Qualification | Existing EmployeeProfile | A qualification is added as a historical employee qualification record. | Qualification registered within EmployeeProfile | Employee Profile | L2-E07 |
| External Certificate Verification | Verification request | External credential authenticity is assessed. | Verified/validated outcome or failed/unverified outcome | External Education Integration | L2-H10 |
| EmployeeProfile verification context | Existing profile verification context | Where supported, the governed external verification outcome is reflected in the profile context. | Verification status aligned to the governed outcome | Employee Profile | L2-E07 verification capability |

No new EmployeeProfile lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| QualificationAdded | Employee Profile | Profile consumers and dependent domains | Establishes that an employee qualification has been registered. | L2-E07 |
| CertificateVerificationSubmitted | External Education Integration | Interested integration/profile participants | Establishes that external credential verification has been submitted. | L2-H10 |
| CertificateVerificationValidated | External Education Integration | Interested integration/profile participants | Establishes that external credential authenticity has been validated. | L2-H10 |
| CertificateVerificationFailed | External Education Integration | Interested integration/profile participants | Establishes that external credential verification failed. | L2-H10 |

The exact downstream subscription mapping from H10's certificate-verification events into the E07 qualification verification record is not explicitly named in the authoritative sources reviewed; the business outcome is therefore expressed without inventing a new consumer contract or event.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Employee Profile | External Education Integration | External credential verification request | Requests authenticity verification for an externally issued employee qualification. | L2-H10 Certificate Verification workflow |
| External Education Integration | Employee Profile | Governed credential verification outcome | Makes the external verification result available for the employee qualification context. | L2-H10; exact E07 consumer contract not explicitly named |
| Employee Profile | Dependent employee-profile consumers | Registered qualification fact | Makes the employee qualification available as canonical EmployeeProfile information. | L2-E07 |

No API, payload, transport, provider protocol, queue, or implementation contract is defined here.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register employee qualification | Employee / HR Administrator / HR Executive | Qualification information must satisfy Employee Profile rules. | Employee Profile |
| Review qualification verification | Verification Officer | Verification must follow the governed verification process. | Employee Profile |
| Coordinate external credential verification | External Education Integration | H10 owns the external verification orchestration and provider interaction. | External Education Integration |

### 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty or override rule is explicitly defined beyond the distinct Employee Profile verification responsibility and H10's separate external verification orchestration responsibility.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Qualification registration | Canonical historical qualification record in EmployeeProfile | Employee Profile |
| External credential verification | Verification outcome and associated verification evidence/reference governed by the verification process | External Education Integration |
| Profile-level verification history | Employee Profile verification context where applicable | Employee Profile |

Platform audit infrastructure remains owned by Section A and is not redefined here.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The employee qualification is registered in the canonical EmployeeProfile.
- Qualification type and institution requirements are satisfied.
- No duplicate qualification record has been created.
- Where external verification is required, the external credential verification process has reached a governed validated or failed outcome.
- A validated external credential is not represented as failed, and a failed verification is not represented as verified.
- The Employee Profile retains ownership of the employee qualification.
- External Education Integration retains ownership of external verification orchestration.
- Canonical qualification and verification events have been published where defined.

### 16.2 Postconditions

- The employee's qualification remains a preserved historical EmployeeProfile record.
- The applicable external verification outcome is available as a governed business fact.
- Any subsequent qualification correction or verification change follows its own canonical Employee Profile or External Education Integration workflow rather than reopening this scenario implicitly.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The authoritative E07 architecture defines qualification registration and profile verification, while H10 defines external certificate verification and its canonical events, but the exact cross-domain consumer contract/event mapping that applies the H10 verification result to an E07 qualification record is not explicitly named. | The business boundary and ownership are clear, but the exact L3 collaboration mechanism for carrying the external verification result into the employee qualification context is not explicitly fixed upstream. | Confirm the canonical E07↔H10 business contract or event-consumption mapping before Architecture Approval if the scenario is required to prescribe that specific hand-off. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
