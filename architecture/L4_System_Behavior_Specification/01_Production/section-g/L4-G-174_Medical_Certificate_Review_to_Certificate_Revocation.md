# Medical Certificate Review to Certificate Revocation

## 1. Specification Metadata

**Specification ID:** L4-G-174  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-174 — Medical Certificate Review to Certificate Revocation  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Approved L3-G-174; locked L2-G06 healthcare workflow/event semantics  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

The system shall support authorised review of an issued Medical Certificate and, where the authoritative decision requires it, record the revocation outcome while preserving the historical certificate. The system shall not invent a new certificate lifecycle state where upstream semantics have not established one.

## 3. Required Business Context

- The certificate must have been issued and remain identifiable.
- Review and revocation require authorised healthcare responsibility.
- Certificate validity and history are G06-owned.
- Historical certificate information must remain traceable.
- C/E provide identity/personnel context where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-174-BH-001 | When an issued certificate is selected for review, the system shall make its validity, issuing professional, patient/encounter context and relevant review evidence available for the authorised decision. | L3-G-174 §§7,9 |
| L4-G-174-BH-002 | The system shall prevent revocation when the authorised revocation basis is insufficient or the certificate context cannot be established. | L3-G-174 §§10,11 |
| L4-G-174-BH-003 | When revocation is authorised, the system shall record the revocation outcome without deleting the historical certificate. | L3-G-174 §§9,12,15 |
| L4-G-174-BH-004 | The system shall prevent subsequent institutional use from treating a revoked certificate as currently valid according to authoritative lifecycle semantics. | L3-G-174 §§3,9,16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-174-BH-005 | On authoritative revocation, the system shall publish `MedicalCertificateRevoked` with G06 as canonical publisher. | L3-G-174 §14 |
| L4-G-174-BH-006 | Where configured, the system shall make the revocation fact available to H without transferring certificate ownership. | L3-G-174 §§5,14 |

## 6. Failure / Rejection / Continuation

- If review confirms continued validity, the system shall leave the certificate's valid business position unchanged and record the review outcome.
- If revocation evidence is insufficient, the system shall not revoke the certificate.
- If the certificate is already expired/archived, the system shall follow the authoritative lifecycle rather than create a second terminal state.
- The system shall not delete historical certificate information as a revocation mechanism.

## 7. Audit & Evidence Behavior

The system shall preserve the certificate reference, original validity and issuing professional, review basis, revocation reason, responsible professional, prior business position, resulting revocation outcome and `MedicalCertificateRevoked` event evidence.

## 8. Observable Completion Conditions

Completion occurs when the review is authoritatively resolved: either the certificate remains valid without revocation, or an authorised revocation outcome is recorded and `MedicalCertificateRevoked` is published. Historical issuance remains traceable in both cases.

## 9. Traceability & Provenance

**Parent:** L3-G-174 — Medical Certificate Review to Certificate Revocation  
**Primary verification source:** L2-G06 — Medical Certificate lifecycle and event definitions  

## 10. Open Issues / Deferred Design

The authoritative relationship between `MedicalCertificateRevoked` and the L2 state model (`Draft → Approved → Issued → Archived`) remains an upstream verification item. Technical document deletion, storage, APIs, IAM and infrastructure are deferred to later implementation levels.