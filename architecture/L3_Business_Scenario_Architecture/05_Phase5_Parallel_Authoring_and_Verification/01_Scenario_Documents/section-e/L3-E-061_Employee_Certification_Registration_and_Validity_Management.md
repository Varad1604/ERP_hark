# Employee Certification Registration and Validity Management

**Scenario ID:** L3-E-061  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Employee Profile  
**Participating Domains:** E — People & Workforce; H — Institutional Services & Intelligence  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-E07 — Employee Profile Workflows; L2-H10 — External Education Integration Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-E-061 |
| Canonical Title | Employee Certification Registration and Validity Management |
| Primary Owner | Employee Profile |
| Participating Domains | E — People & Workforce; H — Institutional Services & Intelligence |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Maintain a canonical employee professional certification from registration through its applicable verification and validity lifecycle so that the employee profile accurately represents current and historical certifications.

Employee Profile owns the certification record and its lifecycle. External Education Integration participates only when an externally issued certification requires governed external verification; that participation does not transfer ownership of the certification.

## 3. Business Outcome

A professional certification is registered against the employee's canonical EmployeeProfile and, where applicable, its verification and validity outcome is established so that the profile distinguishes a valid verified certification from an expired or otherwise non-verified historical certification.

Expired certifications remain historically visible.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an employee or authorized HR participant presents a professional certification for registration in an existing EmployeeProfile.

### 4.2 End Boundary

The scenario ends when the certification has been registered and the applicable verification/validity outcome has been established, or when the certification reaches its governed expired state while remaining historically visible.

### 4.3 In Scope

- Registration of a professional certification.
- Recording certification name, issuing authority, issue date and expiry date where applicable.
- Verification of a certification where the Employee Profile process requires verification.
- External credential verification coordination where an externally issued certification requires it.
- Updating certification information through its canonical EmployeeProfile lifecycle.
- Expiring certifications when their validity ends under the governed lifecycle.
- Preserving historical certification visibility.
- Publishing canonical certification business events.

### 4.4 Out of Scope

- Academic qualification registration.
- Employee skill profile maintenance.
- Employee language profile maintenance.
- Workforce engagement lifecycle.
- Position assignment.
- Professional certification issuance by an external certifying authority.
- External provider or connector lifecycle.
- External integration configuration.
- Technical credential exchange, APIs, queues, retries or transport mechanics.
- Authentication or authorization infrastructure.

### 4.5 Related Scenarios — Optional

No canonical related scenario is required to define this boundary.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Employee Profile — Section E | Canonical owner of the employee certification record, certification information, verification status, update lifecycle, expiry lifecycle and historical visibility. |
| External Education Integration — Section H | Coordinates external verification when the certification requires authenticity checking against an external education/provider source. |

Participation by H does not make H the owner of the employee certification.

## 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility |
|---|---|
| Employee | Provides certification information and supporting evidence/reference for registration. |
| HR Administrator | Registers and maintains employee certification information within Employee Profile. |
| HR Executive | Supports certification registration and profile administration. |
| Verification Officer | Performs or oversees the business verification activity within the Employee Profile context. |
| Compliance Officer | Participates where certification validity has a supported compliance implication. |
| External Education Integration | Performs external credential verification orchestration when required; it is not an institutional approval role. |

These actors are supported by the locked Employee Profile architecture. No additional scenario-specific actor is invented.

## 7. Preconditions and Entry Criteria

- The employee exists.
- A canonical EmployeeProfile exists.
- The certification is a professional certification appropriate to the employee profile.
- Certification name and issuing authority are available.
- Issue date is available.
- Expiry date is available where the certification has a defined validity period.
- Required supporting evidence or credential reference is available where verification is required.
- The certification is not already registered as the same certification record in the employee profile.
- If external verification is required, the external verification context is governed and available.
- The requested action concerns the EmployeeProfile certification entity and does not attempt to alter an externally owned certification record.

## 8. Trigger

An employee certification is submitted for registration, update, verification, or validity management within Employee Profile.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain / Role | Business Result |
|---|---|---|---|
| 1 | The certification information and supporting evidence/reference are presented for employee-profile registration. | Employee / HR Administrator / HR Executive | A certification registration request exists. |
| 2 | Employee Profile validates the required certification information and confirms that the certification belongs within the EmployeeProfile boundary. | Employee Profile | The certification is eligible for registration. |
| 3 | Employee Profile registers the certification with its name, issuing authority, issue date and applicable expiry date. | Employee Profile | The certification becomes part of the canonical EmployeeProfile. |
| 4 | Employee Profile publishes `CertificationAdded`. | Employee Profile | The enterprise has an authoritative certification-added fact. |
| 5 | Where verification is required, the certification is submitted to the governed verification process. | Employee Profile / Verification Officer | Verification is requested. |
| 6 | Where the certification is externally issued and requires external authenticity verification, External Education Integration coordinates the verification with the relevant external provider. | External Education Integration | The external verification process produces a governed outcome. |
| 7 | Employee Profile records the applicable verification outcome using its certification lifecycle. | Employee Profile / Verification Officer | The certification is represented as verified where verification succeeds. |
| 8 | Where the certification reaches the end of its defined validity period, Employee Profile applies the canonical expiry lifecycle. | Employee Profile | The certification becomes expired while remaining historically visible. |
| 9 | Employee Profile publishes the authoritative event corresponding to the completed certification lifecycle change. | Employee Profile | Dependent consumers can use the current certification fact without acquiring ownership. |

The flow stops once the certification is registered and its applicable verification/validity outcome is established.

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Outcome |
|---|---|---|---|
| A1 | The certification does not require external verification. | Employee Profile completes registration and any locally governed verification without invoking H10. | Certification remains governed entirely within Employee Profile. |
| A2 | The certification is already registered. | Employee Profile does not create a duplicate certification; an update follows the existing certification lifecycle where the change is legitimate. | One canonical certification record remains. |
| A3 | External verification validates the certification. | The verified outcome is recorded against the EmployeeProfile certification. | Certification is represented as verified. |
| A4 | External verification does not validate the certification. | Employee Profile does not represent the certification as verified. The certification remains subject to its governed profile status. | Verification failure is preserved as a business outcome without falsely establishing validity. |
| A5 | A certification reaches its expiry date. | Employee Profile applies `ExpireCertification`. | Certification becomes expired while remaining historically visible. |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution | Owner |
|---|---|---|---|
| Mandatory certification information is missing. | Registration cannot validly complete. | Supply the missing certification information and resubmit. | Employee Profile |
| Duplicate certification is identified. | A second canonical certification record must not be created. | Use the existing certification record and perform an allowed update if appropriate. | Employee Profile |
| Verification cannot establish authenticity. | The certification cannot be represented as verified. | Preserve the governed non-verified outcome and resolve the business evidence issue through the applicable verification process. | Employee Profile / H10 |
| Certification expires. | The certification is no longer current for validity purposes. | Apply the canonical expiry lifecycle while preserving historical visibility. | Employee Profile |
| External provider information is insufficient for verification. | External verification cannot produce a validated outcome. | Follow the governed external verification exception path; do not fabricate verification. | External Education Integration |

Technical retries, queues, transport failures, timeout mechanics and infrastructure recovery are excluded from L3.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| EmployeeProfile owns certifications. | Employee Profile | Certification lifecycle remains within E07. |
| Certification includes certification name, issuing authority, issue date, expiry date, verification status and supporting document reference. | Employee Profile | These are the business attributes relevant to registration and validity. |
| Certifications may be added, updated, verified and expired. | Employee Profile | These are the canonical certification business actions. |
| Expired certifications remain historically visible. | Employee Profile | Expiry does not remove certification history. |
| External Education Integration owns external verification orchestration, not the underlying employee certification. | H10 | H participates without ownership transfer. |
| Verification failure must not be represented as successful verification. | Employee Profile / H10 | The profile must preserve the true business verification outcome. |

## 13. State and Lifecycle Impact

| Concept | Entry Position | Scenario Impact | Result | Owner |
|---|---|---|---|---|
| EmployeeProfile | Existing valid profile | Certification is added or maintained within the aggregate. | Profile remains in its existing lifecycle state. | Employee Profile |
| Certification | Not registered | Certification is added. | Registered certification | Employee Profile |
| Certification verification | Verification required | Verification is completed or fails. | Verified or non-verified governed outcome | Employee Profile |
| Certification validity | Valid | Validity period is maintained until expiry. | Expired when governed expiry is reached | Employee Profile |

The scenario does not introduce a new lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `CertificationAdded` | Employee Profile | Certification has been registered. |
| `CertificationUpdated` | Employee Profile | Certification information has been changed through the canonical profile lifecycle. |
| `CertificationVerified` | Employee Profile | Certification verification has completed successfully. |
| `CertificationExpired` | Employee Profile | Certification validity has expired. |
| `CertificateVerificationSubmitted` | External Education Integration | External credential verification has been submitted where H10 verification applies. |
| `CertificateVerificationValidated` | External Education Integration | External credential authenticity has been validated. |
| `CertificateVerificationFailed` | External Education Integration | External credential verification has failed. |

The H10 verification events remain H10-published events; they do not become Employee Profile events.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Employee Profile | External Education Integration | External credential verification request | Requests authenticity verification for an externally issued certification where required. |
| External Education Integration | Employee Profile | Governed external verification outcome | Supplies the business verification outcome for the certification context. |
| Employee Profile | Dependent consumers | Certification business facts | Allows consumers to use certification information without owning it. |

The exact technical representation of these hand-offs is outside L3.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Business Authority

| Action | Business Actor / Role | Authority |
|---|---|---|
| Register certification | Employee / HR Administrator / HR Executive | May register supported certification information in Employee Profile. |
| Verify certification | Verification Officer | Performs or oversees profile certification verification. |
| Review compliance relevance | Compliance Officer | Participates where supported by business policy. |
| Coordinate external verification | External Education Integration | Owns the external verification orchestration, not the certification decision within Employee Profile. |

### 15.2 Separation of Duties

The authoritative E07 architecture distinguishes profile administration from verification responsibility. No additional scenario-specific separation-of-duty rule is defined.

### 15.3 Audit / Evidence

The certification record, verification outcome and validity history must remain traceable within the Employee Profile business context. External verification evidence remains associated with the governed H10 verification process where applicable.

Section A remains the canonical owner of audit infrastructure; L3 does not define its implementation.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The certification is registered in the canonical EmployeeProfile.
- Required certification information is present.
- No duplicate certification record has been created.
- Where required, verification has produced a governed outcome.
- A successful verification is represented by `CertificationVerified`.
- Expired certifications remain historically visible and are represented by the canonical expiry outcome.
- Employee Profile remains the sole owner of the employee certification.
- External Education Integration remains the owner of external verification orchestration.

### 16.2 Postconditions

- The employee profile contains an authoritative certification record.
- The current verification/validity position is available to authorized profile consumers.
- Subsequent certification updates, verification or expiry are governed by the canonical Employee Profile lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material ownership, lifecycle, event-publisher or scenario-boundary issue was identified.

The exact E07 consumer-side business contract by which an H10 `CertificateVerificationValidated` or `CertificateVerificationFailed` event is applied to a specific EmployeeProfile certification is not explicitly named in the reviewed upstream material. This does not alter ownership or the scenario outcome, but the exact contract should be confirmed before Architecture Approval if L3 is required to prescribe that hand-off.

### 17.2 Assumptions

No assumptions that override locked architecture.

### 17.3 Deferred Questions

Confirm the authoritative E07↔H10 business hand-off if a specific cross-domain contract is required for the verification-result application.
