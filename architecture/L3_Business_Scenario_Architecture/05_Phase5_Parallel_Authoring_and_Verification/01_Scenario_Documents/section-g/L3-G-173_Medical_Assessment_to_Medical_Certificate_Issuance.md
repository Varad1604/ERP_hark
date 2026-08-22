# Medical Assessment to Medical Certificate Issuance

**Scenario ID:** L3-G-173  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Medical certificates are formal institutional healthcare documents derived from an authorised medical assessment. Their issuance must remain connected to the underlying healthcare encounter and professional authority while respecting the institution's configurable certificate policies.

This scenario covers the business decision and issuance outcome only. It does not define document-generation technology, document storage implementation, or external legal/administrative acceptance.

---

## 3. Business Outcome

An authorised medical professional has completed the required medical assessment, determined that a medical certificate is appropriate, established the applicable certificate type and validity, and issued the certificate as an authoritative G06 healthcare outcome.

The issued certificate is traceable to the relevant healthcare assessment and professional authority.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A medical assessment has produced sufficient information for an authorised healthcare professional to determine that an institution-issued medical certificate is required or appropriate.

### 4.2 End Boundary

The Medical Certificate has been authorised and issued, with its validity and issuing professional recorded.

### 4.3 In Scope

- Medical assessment.
- Patient and Medical Encounter association.
- Certificate type selection from configured types.
- Medical recommendation.
- Certificate validity.
- Issuing medical professional.
- Certificate issuance.
- Healthcare history.
- Student/employee context.
- Configured notification participation.
- Audit.

### 4.4 Out of Scope

- External government certificates.
- External hospital certification.
- Legal adjudication of certificate acceptance.
- Financial accounting.
- Student identity ownership.
- Employee identity ownership.
- Technical document generation/storage.
- UI presentation.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

Owns Medical Certificate and the healthcare decision supporting issuance.

### Section C — Student Lifecycle

Provides authoritative student identity/status where the certificate concerns a student.

### Section E — People & Workforce

Provides authoritative employee/personnel context and healthcare-professional identity.

### Section H — Notifications

May deliver configured notification that a certificate is available or has been issued. H does not own the certificate.

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Performs the medical assessment and determines whether the certificate can be issued.

### Medical Staff

Supports the operational healthcare process within authorised responsibility.

### Patient

Receives the certificate where issuance is authorised.

### Section C / Section E

Provide authoritative identity/personnel context.

### Section H

Provides notification delivery where configured.

The scenario does not invent an independent certificate approver if the L2 does not require one.

---

## 7. Preconditions and Entry Criteria

1. A valid patient and Medical Encounter exist.
2. A medical assessment has occurred.
3. An authorised medical professional is responsible for the medical decision.
4. The applicable certificate type/template is available under configuration.
5. Required certificate validity information can be established.
6. The certificate is within institutional healthcare authority.

---

## 8. Trigger

The authorised medical assessment determines that an institution-issued medical certificate should be issued.

---

## 9. Main Business Flow

### Phase A — Medical Assessment

1. Identify the patient and Medical Encounter.
2. Confirm the responsible medical professional.
3. Review the relevant healthcare assessment.
4. Establish the medical finding/recommendation supporting certificate issuance.
5. Determine whether the certificate falls within institutional healthcare authority.

### Phase B — Certificate Definition

6. Identify the applicable configured certificate type.
7. Establish the medical recommendation represented by the certificate.
8. Establish the certificate validity.
9. Associate the certificate with the responsible medical professional.
10. Associate the certificate with the relevant healthcare encounter/patient context.

### Phase C — Authorization and Issuance

11. Confirm that the responsible professional is authorised to issue the certificate.
12. Establish the Medical Certificate in its authoritative business lifecycle.
13. Issue the certificate.
14. Record the issuing medical professional.
15. Preserve the certificate's validity information.
16. Record the issuance outcome in healthcare history.

The L2 identifies `Medical Certificate` as a G06 aggregate owning certificate type, validity, medical recommendation and issuing medical professional.

### Phase D — Communication and Completion

17. Where configured, expose the issuance fact to H for notification.
18. Where the certificate supports a student or employee business process, preserve the relationship to C/E context without transferring ownership.
19. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Certificate Not Clinically Justified

If the medical assessment does not support issuance, the certificate is not issued. The underlying healthcare assessment remains part of the medical history.

### 10.2 Certificate Type Not Applicable

If the assessment does not correspond to a configured certificate type, issuance does not proceed until the appropriate authorised healthcare outcome is established.

### 10.3 Further Assessment Required

The medical professional continues or schedules the appropriate healthcare assessment before certificate issuance.

### 10.4 Student Certificate

C supplies student identity/context. G06 remains owner of the certificate.

### 10.5 Employee Certificate

E supplies employee/personnel context. G06 remains owner of the certificate.

### 10.6 Certificate Requires Subsequent Follow-up

A follow-up recommendation may be established as part of the Medical Encounter; the follow-up lifecycle remains separately governed.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Patient/encounter association is invalid | Resolve authoritative healthcare context before certificate issuance. |
| Medical authority cannot be established | Do not issue the certificate until the authorised professional is established. |
| Required medical assessment is incomplete | Continue assessment or establish the applicable healthcare outcome before issuance. |
| Certificate type/validity cannot be established | Resolve the applicable configured certificate policy before issuance. |
| Certificate should not be issued | Preserve the medical assessment without creating an issued certificate. |
| Patient requires further care | Route to the applicable consultation, observation, referral or follow-up workflow. |
| Issuance outcome cannot be established | Do not represent the certificate as issued until the business issuance fact is authoritative. |

No technical retries, queues, rollback, document-generation mechanics or infrastructure recovery are described.

---

## 12. Business Rules and Decision Constraints

1. Medical Certificate is a G06-owned aggregate.
2. Medical Certificate owns certificate type, validity, medical recommendation and issuing medical professional.
3. Medical certificate templates remain configuration-driven.
4. Certificate validity rules remain configuration-driven.
5. Medical professionals retain operational authority.
6. Student identity remains owned by C.
7. Employee/personnel information remains owned by E.
8. Medical history remains auditable and traceable.
9. Notification delivery remains H-owned.

No local validity duration, certificate eligibility matrix, clinical rule, or legal acceptance criterion is invented.

---

## 13. State and Lifecycle Impact

The L2-G06 state transition matrix defines:

`Medical Certificate: Draft → Approved → Issued → Archived`

This scenario establishes the certificate through the assessment and issuance boundary.

### Draft

The certificate business information is being established but is not yet issued.

### Approved

The certificate has reached the authoritative approval/authorisation position defined by the healthcare workflow.

### Issued

The certificate is formally issued as the institutional healthcare outcome.

### Archived

Archival is outside the normal issuance completion boundary and belongs to subsequent lifecycle management.

The scenario does not invent additional certificate states.

---

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of Medical Certificate business facts because it owns the Medical Certificate aggregate.

The L2 explicitly identifies `MedicalCertificateIssued` as the canonical medical-certificate event and `MedicalCertificateRevoked` for the later revocation scenario.

The issuance fact may be consumed by configured downstream notification or institutional services.

C and E provide authoritative identity/personnel context; they do not publish or own the Medical Certificate outcome.

---

## 15. Business Authorization, Approval and Audit Obligations

Certificate issuance must remain within the authority of the responsible medical professional and the configured healthcare authorization model.

Audit evidence should preserve:

- Patient.
- Medical Encounter.
- Medical assessment.
- Certificate type.
- Medical recommendation.
- Validity.
- Issuing medical professional.
- Certificate lifecycle position.
- Issuance outcome.
- Relevant business reason.

Medical confidentiality and need-to-know obligations apply.

Every lifecycle transition must be audited.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Patient and Medical Encounter identified.
- Medical assessment completed.
- Certificate requirement established.
- Applicable certificate type identified.
- Medical recommendation established.
- Validity established.
- Authorised medical professional established.
- Certificate reaches the authoritative issuance position.
- `MedicalCertificateIssued` business fact is produced by G06.
- Healthcare history and audit evidence are preserved.

### Postconditions

- An issued Medical Certificate exists under G06 ownership.
- Certificate validity and issuing professional are traceable.
- Student/employee ownership remains with C/E.
- Notification delivery, if configured, remains H-owned.
- No external legal or hospital certificate is claimed as institutionally issued.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 defines configurable certificate validity and templates but does not enumerate every certificate type/validity rule.
2. The exact business meaning of the `Approved` certificate state relative to the medical professional's issuance authority should be confirmed against the complete locked workflow/contract definitions.

### Assumptions

- G06 owns the institution-issued Medical Certificate.
- `MedicalCertificateIssued` is the canonical issuance event identified by the locked L2.
- Certificate validity remains configuration-driven.

### Deferred Questions

Confirm the complete configured certificate approval policy before Architecture Approval/Lock if additional institutional approval is required.
