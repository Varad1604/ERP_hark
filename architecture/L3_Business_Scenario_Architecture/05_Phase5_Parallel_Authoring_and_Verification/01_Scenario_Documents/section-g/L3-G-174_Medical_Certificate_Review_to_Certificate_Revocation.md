# Medical Certificate Review to Certificate Revocation

**Scenario ID:** L3-G-174  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

An issued medical certificate may subsequently require revocation under the institution's authorised healthcare policy. Because medical certificates are healthcare records, revocation must preserve historical traceability while clearly establishing that the certificate is no longer valid for future institutional use.

The locked L2-G06 defines Medical Certificate as a G06 aggregate, includes validity and issuing professional, and explicitly defines `MedicalCertificateRevoked` as a domain event. The L2 state model provides the authoritative certificate lifecycle.

---

## 3. Business Outcome

A previously issued Medical Certificate has been reviewed under authorised healthcare responsibility, the revocation decision has been established, the certificate has been moved to its authoritative revoked outcome, and the revocation is permanently traceable without deleting the historical certificate record.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An issued Medical Certificate is identified as requiring formal review for possible revocation.

### 4.2 End Boundary

The authorised revocation decision has been recorded, the certificate's valid-use status has been withdrawn according to the authoritative lifecycle, and the revocation history is preserved.

### 4.3 In Scope

- Certificate identification.
- Certificate validity review.
- Medical assessment/review supporting revocation.
- Authorised revocation decision.
- Certificate lifecycle progression.
- Revocation reason/evidence.
- Medical history traceability.
- Student/employee context.
- Notification participation where configured.
- Audit.

### 4.4 Out of Scope

- Deletion of historical certificates.
- External legal proceedings.
- External hospital certificate management.
- External employer/government record modification.
- Financial accounting.
- Student identity ownership.
- Employee identity ownership.
- Technical document deletion or storage.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

Owns Medical Certificate, certificate validity, issuing professional, revocation and certificate history.

### Section C — Student Lifecycle

Provides authoritative student context where relevant.

### Section E — People & Workforce

Provides authoritative employee/personnel context and healthcare-professional context.

### Section H — Notifications

May communicate a configured certificate revocation notification but does not own the certificate or revocation decision.

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Reviews the certificate and establishes whether revocation is warranted within medical authority and policy.

### Medical Staff

Supports review and recording of the revocation outcome.

### Patient / Certificate Holder

May receive the resulting revocation communication where policy requires.

### Section C / E

Provide authoritative identity/personnel context.

### Section H

Provides notification delivery where configured.

No independent technical approver or external legal authority is invented.

---

## 7. Preconditions and Entry Criteria

1. A Medical Certificate exists.
2. The certificate has previously been issued.
3. The certificate can be associated with its patient and healthcare history.
4. A reason for review exists.
5. An appropriately authorised healthcare professional can review the certificate.
6. The certificate remains within G06 institutional authority.

---

## 8. Trigger

An issued Medical Certificate is identified for authorised review and possible revocation.

---

## 9. Main Business Flow

### Phase A — Certificate Review

1. Identify the Medical Certificate.
2. Identify the patient and originating Medical Encounter.
3. Confirm the certificate's issuing professional and validity.
4. Establish the reason for review.
5. Review the medical/business evidence relevant to revocation.
6. Determine whether the certificate remains valid under the applicable policy.

### Phase B — Revocation Decision

7. If the certificate remains valid, preserve the certificate as issued and close the review without revocation.
8. If revocation is warranted, establish the authorised revocation decision.
9. Record the reason supporting revocation.
10. Confirm the responsible healthcare professional.

### Phase C — Lifecycle Outcome

11. Advance the Medical Certificate to its authoritative revocation outcome.
12. Preserve the historical issued certificate.
13. Preserve the relationship between the original certificate, review and revocation decision.
14. Ensure subsequent institutional use cannot treat the certificate as currently valid.

### Phase D — Communication and Audit

15. Record the revocation in healthcare history/audit history.
16. Publish the canonical `MedicalCertificateRevoked` business event.
17. Where configured, provide the relevant business fact to H for notification.
18. Preserve the complete historical evidence.
19. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Review Confirms Certificate Remains Valid

No revocation occurs. The certificate remains in its authoritative lifecycle position and the review outcome is recorded.

### 10.2 Certificate Has Already Expired

If the certificate has naturally reached the end of its validity, the review does not falsely represent it as actively revoked unless the authoritative policy requires revocation.

### 10.3 Revocation Reason Is Insufficient

The review remains unresolved until an authorised basis for revocation is established.

### 10.4 Further Medical Assessment Required

The responsible professional performs or obtains the necessary assessment before making the revocation decision.

### 10.5 Student Certificate

C supplies student context; G06 retains certificate ownership.

### 10.6 Employee Certificate

E supplies employee context; G06 retains certificate ownership.

### 10.7 Notification Required

H handles notification delivery according to configured policy. H does not alter the certificate lifecycle.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Certificate cannot be identified | Resolve authoritative certificate identity before review. |
| Issuing professional cannot be established | Preserve the certificate history and resolve the authoritative healthcare context before revocation. |
| Revocation basis is insufficient | Do not revoke; continue review or retain the certificate in its existing valid state. |
| Medical assessment is incomplete | Complete the required authorised assessment before making the revocation decision. |
| Certificate has already reached an expiry/archive outcome | Follow the authoritative lifecycle rather than inventing a second terminal state. |
| Patient identity/context is uncertain | Do not attach revocation to an unverified patient. |
| Notification is not required | Complete the healthcare lifecycle without inventing a notification requirement. |

No technical retry, queue, rollback, infrastructure or document-storage mechanism is specified.

---

## 12. Business Rules and Decision Constraints

1. Medical Certificate is owned by G06.
2. Certificate validity is part of the Medical Certificate aggregate.
3. Medical Certificate history remains traceable.
4. Medical professionals retain healthcare authority.
5. Medical history is not deleted.
6. Student identity remains owned by C.
7. Employee/personnel information remains owned by E.
8. Notification delivery remains H-owned.
9. `MedicalCertificateRevoked` is the canonical revocation event identified by L2-G06.
10. Certificate templates and validity policies remain configuration-driven.

The scenario does not invent legal grounds for revocation or external acceptance rules.

---

## 13. State and Lifecycle Impact

The L2-G06 state transition matrix identifies:

`Medical Certificate: Draft → Approved → Issued → Archived`

The same L2 explicitly identifies `MedicalCertificateRevoked` as a canonical event, but the retrieved state matrix does not expose a distinct `Revoked` state.

Therefore this L3 does **not** silently invent a new lifecycle state.

The revocation outcome must be represented according to the authoritative certificate lifecycle/contract semantics. If the locked upstream architecture defines revocation as a distinct state outside the retrieved matrix, that upstream definition governs. Otherwise the certificate's historical record must clearly record the revocation business outcome without creating an unapproved state.

This is a material upstream verification point before Architecture Approval/Lock.

---

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of the revocation fact because G06 owns Medical Certificate.

The locked L2 explicitly names:

`MedicalCertificateRevoked`

as the canonical Medical Certificate event.

The event communicates the business fact that the certificate has been revoked. It does not transfer certificate ownership to C, E or H.

H may consume the event for configured notification.

C/E provide authoritative identity/personnel context and do not own the certificate lifecycle.

---

## 15. Business Authorization, Approval and Audit Obligations

Revocation must be established by an appropriately authorised healthcare professional under the applicable healthcare authorization model.

Audit evidence should preserve:

- Medical Certificate reference.
- Patient/certificate-holder reference.
- Originating Medical Encounter.
- Original issuing professional.
- Original validity.
- Review reason.
- Revocation reason.
- Responsible revoking professional.
- Previous certificate state.
- Resulting revocation outcome.
- `MedicalCertificateRevoked` event.
- Relevant notification/communication outcome where applicable.

The original certificate must remain historically traceable even after revocation.

Medical confidentiality and need-to-know obligations apply.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Existing Medical Certificate identified.
- Certificate review completed.
- Revocation basis established by authorised healthcare authority.
- Revocation decision recorded.
- Certificate's valid-use status withdrawn according to authoritative lifecycle semantics.
- Historical certificate preserved.
- Revocation evidence audited.
- `MedicalCertificateRevoked` published by G06.
- Any configured notification path is established.

### Postconditions

- The certificate cannot be treated as currently valid for institutional use.
- Historical issuance remains preserved.
- Revocation is traceable to its authorised decision.
- G06 remains certificate owner.
- C/E ownership remains unchanged.
- H remains notification owner where applicable.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The L2 state transition matrix lists `Draft → Approved → Issued → Archived` but does not expose `Revoked` as a state, while the L2 event catalogue explicitly defines `MedicalCertificateRevoked`.
2. The authoritative relationship between revocation and the certificate state machine therefore requires confirmation before the scenario can be considered Architecture Approved/Locked.
3. The L2 does not enumerate universal revocation grounds.

### Assumptions

- `MedicalCertificateRevoked` is the canonical revocation event.
- Revocation is an authoritative healthcare outcome and does not delete historical certificate information.
- The upstream lifecycle semantics, once confirmed, govern the exact state representation.

### Deferred Questions

Confirm whether revocation is:
1. a distinct Medical Certificate lifecycle state,
2. a governed transition to another existing state, or
3. an audited business outcome associated with the issued/archived certificate,

before Architecture Approval/Lock. This issue is deliberately not resolved locally at L3.
