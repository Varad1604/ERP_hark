# Medical Event to Employee Portal Health Information

**Scenario ID:** L3-G-181  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E, H  
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
| Final L3 ID | L3-G-181 |
| Canonical Title | Medical Event to Employee Portal Health Information |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E, H |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Institutional healthcare may generate medical information that an employee is permitted to receive through the institution's employee-facing services. The institution therefore needs a governed business path from an authoritative G06 medical outcome to permitted employee-facing health information.

The purpose of this scenario is to coordinate that cross-domain outcome without changing ownership of the underlying healthcare record, workforce record, or student/employee identity.

This scenario is business-level only. It does not prescribe portal screens, technical access controls, APIs, databases, event infrastructure, or delivery mechanisms.

---

## 3. Business Outcome

A permitted employee-related medical event has been identified, associated with the authoritative workforce context, and made available as permitted employee-facing health information through the Employee Portal service context while the underlying medical record remains owned by G06.

Where the medical event is not eligible for employee-facing disclosure, the scenario ends with the medical record retained under G06 ownership and no prohibited employee-facing publication.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A G06 medical event or medical outcome exists for an employee and is eligible for consideration for employee-facing health information.

### 4.2 End Boundary

The permitted information is available through the Employee Portal context, or a governed decision determines that no employee-facing information may be provided.

### 4.3 In Scope

- Recognition of an employee-related G06 medical event.
- Association with authoritative workforce context from E.
- Determination of permitted employee-facing information.
- Cross-domain hand-off to H.
- Employee Portal business availability.
- Traceability from employee-facing information back to G06 medical truth.
- Healthcare confidentiality and business-purpose constraints.
- Correction/supersession handling at business level.

### 4.4 Out of Scope

- Employee identity ownership.
- Workforce lifecycle ownership.
- Medical record ownership outside G06.
- Portal UI design.
- technical access control/IAM.
- APIs, databases, queues, deployment or infrastructure.
- Payroll, leave, disciplinary or employment decisions.
- Unauthorised disclosure of medical information.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the authoritative medical event, healthcare record and medical outcome.

### Section E — People & Workforce

E owns employee/workforce information and supplies authoritative workforce context required to associate the healthcare outcome with the correct employee.

### Section H — Institutional Services & Intelligence

H provides downstream institutional services and projections, including the Employee Portal context. H does not become owner of the healthcare record.

Ownership is not transferred by the cross-domain consumption.

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional / Medical Staff

Records and maintains the authoritative healthcare outcome within G06.

### Employee

Receives permitted employee-facing health information.

### Institutional Health Services

Determines the authoritative medical fact and the business information eligible for downstream use according to applicable healthcare/institutional policy.

### Workforce / People Function

Provides authoritative employee/workforce context through E.

### Employee Portal / Institutional Services

Makes permitted information available through the employee-facing institutional service context.

No technical role, permission code, IAM group, portal implementation role, or unsupported disclosure authority is invented.

---

## 7. Preconditions and Entry Criteria

1. A G06 medical event or medical outcome exists.
2. The event concerns an employee/workforce participant.
3. The employee can be associated with authoritative E context.
4. The medical information is sufficiently complete for the permitted downstream outcome.
5. The event is eligible for employee-facing consideration under applicable healthcare/institutional policy.
6. H provides the applicable employee-facing service capability.

---

## 8. Trigger

A material G06 medical event or medical outcome becomes eligible for permitted employee-facing health information.

---

## 9. Main Business Flow

### Phase A — Establish the Authoritative Medical Outcome

1. G06 records the medical event or healthcare outcome.
2. G06 remains the canonical source of the medical fact.
3. Identify the affected employee through authoritative E workforce context.
4. Confirm the medical event is associated with the correct employee.
5. Preserve the relationship between the medical event and the workforce context.

### Phase B — Determine Permitted Employee Information

6. Determine whether the medical event is eligible for employee-facing availability.
7. Identify the business information that may be made available.
8. Preserve the distinction between the complete G06 medical record and the permitted employee-facing representation.
9. If the event is not eligible for employee-facing disclosure, retain it within G06 and end the scenario.

### Phase C — Provide Employee-Facing Information

10. Provide the permitted business information to H through the applicable business collaboration.
11. H makes the permitted information available through the Employee Portal context.
12. Preserve traceability from the employee-facing information to the authoritative G06 medical outcome.
13. H does not alter the G06 medical record.
14. E remains authoritative for employee/workforce identity and context.

### Phase D — Complete

15. Confirm the permitted employee health information is available through the Employee Portal context.
16. Preserve applicable business audit/evidence.
17. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Medical Event Not Eligible for Employee-Facing Disclosure

The medical event remains within G06 healthcare records. No employee-facing information is made available.

### 10.2 Limited Information Permitted

Only the permitted business information is exposed. The existence of an employee portal relationship does not authorise publication of the complete medical record.

### 10.3 Employee Context Requires Resolution

If the medical event cannot be reliably associated with an employee, E remains the authoritative source for workforce context. G06 does not create or become owner of a duplicate employee record.

### 10.4 Medical Event Is Corrected or Superseded

Where the authoritative G06 medical fact is corrected or superseded, the employee-facing representation must be governed by the applicable business correction/supersession process so that it does not remain inconsistent with the authoritative medical outcome.

### 10.5 Medical Certificate

Where the event concerns a G06-owned Medical Certificate, the certificate remains a G06 healthcare outcome. Only the permitted employee-facing information is made available through H.

### 10.6 Occupational Health Outcome

An occupational health outcome remains G06-owned. Any employment consequence is outside this scenario unless separately governed by E-owned architecture.

### 10.7 Confidential Medical Outcome

If the medical information is subject to heightened confidentiality, the permitted employee-facing information is limited accordingly. No disclosure rule is invented locally.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Employee identity/context cannot be reliably established | Resolve authoritative E workforce context before making employee-facing information available. |
| Medical information is incomplete | Do not expose an incomplete or unsupported medical outcome. |
| Event is not eligible for disclosure | Retain the medical information within G06 and do not publish it to the Employee Portal context. |
| Employee-facing information is inconsistent with G06 truth | Reconcile the downstream representation with the authoritative G06 medical outcome through the applicable business correction process. |
| Downstream service availability is interrupted | Preserve G06 medical ownership and resolve the business availability condition before treating the employee-facing outcome as complete. |
| Workforce context changes | E remains authoritative for workforce information; G06 does not duplicate or locally redefine the employee lifecycle. |

No technical retry, queue, rollback, server-failure, or infrastructure recovery behaviour is defined.

---

## 12. Business Rules and Decision Constraints

1. G06 owns medical events and healthcare records.
2. E owns employee/workforce information.
3. H provides institutional service/projection capability.
4. Downstream consumption does not transfer ownership.
5. Only permitted medical information may be exposed through the Employee Portal.
6. The Employee Portal representation is not a second canonical medical record.
7. E remains authoritative for employee identity/context.
8. Medical confidentiality and applicable institutional healthcare policy constrain disclosure.
9. H must not alter the authoritative G06 medical outcome.
10. Employment, payroll, leave and disciplinary decisions are not implied by this scenario.

No technical access control, IAM, permission code, disclosure matrix, or UI rule is invented.

---

## 13. State and Lifecycle Impact

This scenario does not create a new lifecycle for the medical event.

The originating G06 aggregate retains its authoritative lifecycle.

Employee Portal availability is a downstream business information outcome, not a new state of the underlying medical record.

If the source event arises from a Medical Encounter, Medical Certificate, Vaccination Campaign, Annual Health Checkup, Referral, or another G06 aggregate, that aggregate retains its own authoritative lifecycle.

E's workforce lifecycle remains independently governed.

---

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher of the originating medical event because G06 owns the healthcare outcome.

The exact event is selected from the authoritative event vocabulary of the originating G06 workflow. The L2-G06 catalogue contains specific events such as:

- `MedicalCertificateIssued`
- `MedicalCertificateRevoked`
- `VaccinationRecorded`
- `HealthCheckCompleted`
- `HealthCampCompleted`

This scenario does not invent a generic `MedicalEventPublishedToEmployeePortal` event.

E provides authoritative employee/workforce context.

H consumes the permitted business fact and provides the Employee Portal service context.

Published domain events remain distinct from synchronous business context hand-offs.

---

## 15. Business Authorization, Approval and Audit Obligations

G06 retains authority over the healthcare fact and the business basis for any permitted downstream disclosure.

Audit evidence should preserve, where applicable:

- Source medical event.
- Employee/workforce context.
- Permitted employee-facing information.
- Business basis for downstream availability where defined by policy.
- Source-to-consumer traceability.
- Relevant correction/supersession outcome.
- Applicable canonical G06 event.
- Business date/time evidence where part of the authoritative medical record.

Medical confidentiality must be preserved.

No technical authorization mechanism or unsupported approval chain is introduced.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- A valid G06 medical event exists.
- The event is associated with the correct employee through E.
- Employee-facing eligibility is determined.
- Permitted information is identified.
- Permitted information is made available through the Employee Portal business context.
- The information remains traceable to G06.
- No ownership transfer occurs.

### Postconditions

- Permitted employee health information is available through the Employee Portal context.
- G06 remains owner of the underlying medical record.
- E remains owner of employee/workforce information.
- H remains a downstream institutional service/projection provider.
- Restricted medical information remains outside the employee-facing representation.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. L2-G06 identifies Employee Portal as a downstream consumer but does not provide a complete event-by-event disclosure matrix for employee-facing medical information.
2. The exact business contract/read-model vocabulary governing Employee Portal health information is not fully enumerated in the retrieved L2-G06 document.

### Assumptions

- G06 remains the sole canonical owner of medical facts.
- E supplies authoritative employee/workforce context.
- H provides the employee-facing service/projection capability.
- Employee-facing information is limited to what applicable healthcare/institutional policy permits.

### Deferred Questions

Before Architecture Approval/Lock, confirm the authoritative policy or contract defining which medical event categories and information may be made available through the Employee Portal. This L3 document does not invent that disclosure policy.
