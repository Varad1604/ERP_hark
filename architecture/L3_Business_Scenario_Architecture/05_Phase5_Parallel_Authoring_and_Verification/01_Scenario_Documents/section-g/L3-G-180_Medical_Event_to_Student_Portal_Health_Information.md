# Medical Event to Student Portal Health Information

**Scenario ID:** L3-G-180  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, C, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-180 |
| Canonical Title | Medical Event to Student Portal Health Information |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, C, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Students may need access to permitted healthcare information resulting from institutional medical activity. The institution therefore needs a governed business pathway from a G06-owned medical event to appropriate student-facing health information.

The scenario is deliberately bounded at business information availability. It does not describe screens, page layouts, APIs, data stores, notification technology, portal implementation or technical access control.

---

## 3. Business Outcome

A permitted student-related medical event has been recognised by G06, the applicable student context is associated through authoritative C information, and the permitted health information is made available through the institution's Student Portal service context without transferring ownership of the medical record from G06.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A G06 medical event or medical outcome exists for a student and is eligible for permitted downstream student-facing information.

### 4.2 End Boundary

The permitted health information has been made available through the Student Portal business context, or the event has been determined not to be eligible for student-facing publication.

### 4.3 In Scope

- Recognition of a student-related G06 medical event.
- Student identity/context association.
- Determination of permitted student-facing information.
- Publication/availability through H Student Portal services.
- Preservation of G06 ownership.
- Student-facing health information traceability.
- Confidentiality/business-purpose constraints.

### 4.4 Out of Scope

- Student identity ownership.
- Medical record ownership outside G06.
- Portal UI design.
- Technical access control.
- APIs, databases, integration protocols or infrastructure.
- Unauthorised disclosure of medical information.
- Independent modification of medical facts by H or C.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the medical event, medical record and healthcare business truth.

### Section C — Student Lifecycle

C owns student identity and student lifecycle context used to associate the medical information with the correct student.

### Section H — Institutional Services & Intelligence

H provides the Student Portal service/projection capability and makes permitted health information available to the student-facing context. H does not become owner of the underlying medical record.

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional / Medical Staff

Owns the healthcare decision and medical information within G06 authority.

### Student

Receives permitted student-facing health information.

### Institutional Health Services

Determines which healthcare facts are business-authorised for downstream student-facing use.

### Section C

Provides authoritative student context.

### Student Portal / H Services

Makes permitted information available through the institution's student-facing service context.

No technical role, permission code, IAM group or portal implementation role is invented.

---

## 7. Preconditions and Entry Criteria

1. A G06 medical event or healthcare outcome exists.
2. The event is associated with a student.
3. Student identity/context is authoritative in C.
4. The medical information is eligible for student-facing use under applicable healthcare/institutional policy.
5. H provides the applicable Student Portal service capability.
6. The medical fact is sufficiently complete for the permitted downstream information.

---

## 8. Trigger

A material G06 medical event or medical outcome becomes eligible for permitted student-facing health information.

---

## 9. Main Business Flow

### Phase A — Recognise the Medical Event

1. G06 records the medical event or healthcare outcome.
2. G06 determines the event's relevant business meaning and ownership.
3. Identify the affected student through authoritative C context.
4. Confirm that the event belongs to the student context.

### Phase B — Determine Student-Facing Information

5. Determine whether the medical fact is eligible for student-facing availability under applicable healthcare policy.
6. Determine the permitted information to be exposed to the student-facing service context.
7. Preserve the distinction between the underlying medical record and the permitted downstream information.
8. Where the event is not permitted for student-facing use, end the scenario without publishing the restricted medical information.

### Phase C — Make Information Available

9. Provide the permitted business information to H through the approved business collaboration mechanism.
10. H makes the permitted information available through the Student Portal service context.
11. The portal-facing information remains traceable to the G06 medical source.
12. H does not alter the underlying G06 medical record.
13. C remains the authority for student identity.

### Phase D — Complete

14. Confirm the permitted student health information is available in the Student Portal context.
15. Preserve the source and business traceability.
16. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Medical Event Not Eligible for Student-Facing Disclosure

The event remains within G06 healthcare records and is not made available through the Student Portal.

### 10.2 Limited Information Permitted

Only the permitted business information is made available. The complete medical record is not implicitly exposed merely because the student portal consumes a health-related fact.

### 10.3 Student Identity Context Requires Resolution

If the medical event cannot be reliably associated with the correct student, C remains the authoritative source for resolving student identity. G06 does not create a duplicate student identity.

### 10.4 Medical Event Updated Before Student-Facing Availability

The downstream information reflects the authoritative G06 medical fact according to the applicable business update/supersession policy. H does not independently rewrite the medical outcome.

### 10.5 Medical Certificate

Where the student-facing information concerns a medical certificate, the certificate remains a G06-owned medical artefact/outcome while the permitted student-facing representation is provided through H.

### 10.6 Vaccination or Health Check Outcome

Where the student-facing information arises from vaccination or health-check activity, the relevant G06 healthcare event/outcome remains authoritative. H exposes only the permitted information.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Student identity cannot be reliably associated | Resolve the authoritative C student context before making student-facing information available. |
| Medical information is not eligible for disclosure | Keep the information within G06 healthcare records and do not expose it through the Student Portal. |
| Medical outcome is incomplete | Do not publish an incomplete or unsupported student-facing outcome. |
| Downstream availability is interrupted | Preserve G06 medical ownership and resolve the business availability issue before treating the information as successfully available. |
| Published information is found to be inconsistent with G06 truth | Reconcile the student-facing representation with the authoritative G06 medical fact under the applicable business correction/supersession process. |
| Student context changes | C remains authoritative for student lifecycle/identity context; G06 does not take ownership of C data. |

No technical retries, queue mechanics, rollback or infrastructure recovery are described.

---

## 12. Business Rules and Decision Constraints

1. G06 owns medical events and healthcare records.
2. C owns student identity and lifecycle context.
3. H provides student-facing service/projection capability.
4. Downstream consumption does not transfer ownership.
5. Only permitted medical information may become student-facing health information.
6. H must not alter the authoritative G06 medical record.
7. Student identity must come from C's authoritative context.
8. Medical confidentiality and applicable institutional healthcare policy constrain disclosure.
9. A portal representation is not a second canonical medical record.
10. Material medical information remains traceable to G06.

The scenario does not invent specific disclosure matrices, consent categories, technical permissions or UI controls.

---

## 13. State and Lifecycle Impact

This scenario does not create a new medical-event lifecycle.

The originating healthcare aggregate retains its authoritative G06 lifecycle.

The Student Portal availability is a downstream business information outcome and must not be treated as a new lifecycle state of the underlying medical record.

If the originating event is associated with a Medical Certificate, Vaccination Campaign, Annual Health Checkup, Medical Encounter or another G06 aggregate, that aggregate's authoritative lifecycle remains unchanged.

---

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of the originating medical event because G06 owns the medical outcome.

The exact canonical event depends on the originating healthcare activity. The retrieved L2-G06 event catalogue explicitly identifies events such as `HealthCampCompleted`, `VaccinationRecorded`, `HealthCheckCompleted`, `MedicalCertificateIssued` and `MedicalCertificateRevoked`.

The scenario does **not** invent a generic `MedicalEventPublishedToStudentPortal` event.

C provides authoritative student identity/context.

H consumes the permitted medical fact and makes the permitted information available through the Student Portal service context.

A published domain event remains distinct from any synchronous business contract used to resolve student context.

---

## 15. Business Authorization, Approval and Audit Obligations

G06 retains authority over the healthcare fact and its permitted disclosure classification.

Audit evidence should preserve:

- Source G06 medical event.
- Student context.
- Permitted student-facing information.
- Business basis for availability where defined by policy.
- Source-to-consumer traceability.
- Any correction or supersession outcome.
- Relevant canonical event.
- Date/time evidence where part of the source healthcare record.

Medical confidentiality must be preserved.

The scenario does not invent a technical authorization mechanism. Business authorization remains governed by applicable healthcare and institutional policy.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- A valid G06 medical event exists.
- The event is associated with the correct student through C.
- Eligibility for student-facing information is determined.
- Permitted information is identified.
- The information is made available through the H Student Portal business context.
- The source remains traceable to G06.
- No ownership transfer occurs.

### Postconditions

- Permitted student health information is available through the Student Portal context.
- G06 remains owner of the medical record.
- C remains owner of student identity.
- H remains a downstream service/projection provider.
- Restricted medical information remains outside the student-facing representation.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. L2-G06 identifies Student Portal as a downstream consumer but does not enumerate a complete medical-information disclosure matrix for every medical event type.
2. The exact business contract/read-model vocabulary for student-facing health information is not fully enumerated in the retrieved L2 source.

### Assumptions

- G06 remains the sole canonical owner of the medical fact.
- C supplies authoritative student identity/context.
- H provides the student-facing service/projection capability.
- Student-facing availability is limited to information permitted by applicable policy.

### Deferred Questions

Before Architecture Approval/Lock, confirm the authoritative policy or contract defining which medical event categories and attributes may be made available to students through the Student Portal. No such disclosure matrix is invented in this L3 document.
