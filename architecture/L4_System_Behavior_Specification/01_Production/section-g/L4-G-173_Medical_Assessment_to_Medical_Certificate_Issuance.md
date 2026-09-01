# Medical Assessment to Medical Certificate Issuance

## 1. Specification Metadata

**Specification ID:** L4-G-173  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institutional Health Services  
**Realizes Scenario:** L3-G-173 — Medical Assessment to Medical Certificate Issuance  
**Primary Business Owner:** Institutional Health Services  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-01  
**Depends On:** Approved L3-G-173; locked L2-G06 healthcare workflow semantics  
**Referenced By:** None yet  

## 2. Behavioral Objective & Boundary

The system shall support the governed transition from an authorised medical assessment to an institution-issued Medical Certificate. The boundary begins when sufficient assessment information exists to consider issuance and ends when the certificate is authoritatively issued or issuance is declined. Certificate generation technology and external acceptance are outside this specification.

## 3. Required Business Context

- Patient and Medical Encounter context must be authoritative.
- Certificate type and validity are configuration-governed.
- Issuance requires an authorised medical professional.
- Medical Certificate is G06-owned.
- Student/workforce context remains owned by C/E where applicable.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-173-BH-001 | When an authorised assessment supports certificate issuance, the system shall allow the applicable configured certificate type, recommendation, validity and issuing professional to be established. | L3-G-173 §§2,4,9,12 |
| L4-G-173-BH-002 | The system shall prevent certificate issuance when the patient/encounter context, medical authority, certificate type, or required validity information is not sufficiently established. | L3-G-173 §§7,11 |
| L4-G-173-BH-003 | On authorised issuance, the system shall record the Medical Certificate as an issued G06 healthcare outcome associated with the relevant patient and Medical Encounter. | L3-G-173 §§9,13,16 |
| L4-G-173-BH-004 | The system shall preserve the issuing professional and certificate validity as attributable business evidence. | L3-G-173 §§9,15 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-173-BH-005 | On authoritative certificate issuance, the system shall publish the canonical `MedicalCertificateIssued` business fact with G06 as publisher. | L3-G-173 §14 |
| L4-G-173-BH-006 | Where configured, the system shall make the issued certificate fact available to H without transferring Medical Certificate ownership. | L3-G-173 §§5,9,14 |

## 6. Failure / Rejection / Continuation

- If assessment evidence is insufficient, the system shall retain the certificate as not-issued and permit continuation of the authorised assessment process.
- If certificate eligibility or validity cannot be established, the system shall prevent issuance until the governed condition is resolved.
- If the assessment does not justify a certificate, the system shall not produce an issued certificate.
- The system shall not represent downstream notification or external acceptance as certificate issuance.

## 7. Audit & Evidence Behavior

The system shall preserve attributable evidence of the patient/Medical Encounter, assessment basis, certificate type, recommendation, validity, issuing professional, issuance outcome and relevant lifecycle position.

## 8. Observable Completion Conditions

Completion occurs when either: (a) a valid Medical Certificate has reached the authoritative issued position and `MedicalCertificateIssued` is produced by G06; or (b) issuance has been authoritatively declined without an issued certificate being represented.

## 9. Traceability & Provenance

**Parent:** L3-G-173 — Medical Assessment to Medical Certificate Issuance  
**Primary verification source:** L2-G06 — Institutional Health Services workflow/state/event definitions  

## 10. Open Issues / Deferred Design

Certificate-generation/storage mechanisms, portal presentation, technical authorisation, APIs, persistence and infrastructure remain L4/L5 implementation concerns and are not defined here. The precise configured certificate approval policy remains upstream-governed.