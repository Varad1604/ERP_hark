# Medical Clearance Request to Institutional Clearance Decision

**Scenario ID:** L3-G-185  
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

Institutional activities may require a determination that an individual has received the necessary medical assessment for a defined institutional purpose. The medical clearance decision must therefore be grounded in authoritative institutional healthcare information and remain within the medical authority of G06.

The scenario provides an end-to-end business path from a legitimate medical clearance request through healthcare assessment, decision and recording of the resulting clearance outcome. It does not create employment, academic, identity, financial or security decisions merely because a medical clearance is required by another institutional process.

---

## 3. Business Outcome

A legitimate medical clearance request has been assessed using the authoritative healthcare information and applicable medical judgement/process, and an institutional medical clearance decision has been established as **cleared, not cleared, or another authoritative outcome if explicitly defined by the governing healthcare architecture**.

The medical clearance decision remains a G06-owned healthcare outcome.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A medical clearance request requiring Institutional Health Services assessment is received.

### 4.2 End Boundary

The applicable G06 medical clearance decision is established and recorded, with any permitted downstream institutional availability handled through the relevant business collaboration.

### 4.3 In Scope

- Receipt and validation of a medical clearance request.
- Identification of the person requiring clearance.
- Retrieval/review of relevant authoritative healthcare information.
- Medical assessment required for the clearance purpose.
- Determination of the medical clearance outcome.
- Recording of the outcome within G06 ownership.
- Permitted communication or downstream institutional use of the clearance outcome.
- Correction/reassessment where the authoritative healthcare process requires it.
- Audit/evidence of the clearance decision.

### 4.4 Out of Scope

- Student identity ownership.
- Workforce identity or employment decisions.
- Academic eligibility decisions.
- Leave, payroll or disciplinary decisions.
- Security access decisions.
- Financial decisions.
- Technical authorization or access control.
- Portal/UI design.
- APIs, databases, services, queues or infrastructure.
- Clinical rules not defined by authoritative healthcare architecture.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the medical assessment, medical clearance decision and associated healthcare record.

### Section C — Student Lifecycle

C remains authoritative for student identity and student lifecycle information when the clearance subject is a student.

### Section E — People & Workforce

E remains authoritative for employee/workforce information when the clearance subject is an employee.

### Section H — Institutional Services & Intelligence

H may provide downstream institutional communication/service capability where applicable.

No participating domain acquires ownership of the medical clearance decision merely through request, consumption, or downstream use.

---

## 6. Business Actors, Roles and Responsibilities

### Requesting Institutional Party

Initiates or submits a legitimate medical clearance requirement.

### Healthcare Professional / Medical Staff

Performs or coordinates the medical assessment within G06 healthcare authority.

### Institutional Health Services

Determines and records the medical clearance outcome.

### Student / Employee / Other Authorised Subject

Provides or participates in the healthcare assessment as applicable.

The authoritative L2-G06 source does not define a universal named role for every clearance use case. Therefore this scenario uses business-level roles and does not invent a specific clearance officer, committee or approving authority.

---

## 7. Preconditions and Entry Criteria

1. A legitimate medical clearance requirement exists.
2. The person requiring clearance can be identified through the canonical identity/workforce/student context applicable to that person.
3. G06 healthcare services are available for the relevant assessment.
4. The clearance purpose is known sufficiently to determine the required healthcare assessment.
5. Relevant authoritative healthcare information exists or can be established through the applicable G06 medical workflow.
6. The requested decision falls within G06 healthcare authority.

---

## 8. Trigger

A medical clearance request requiring Institutional Health Services assessment is submitted.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the medical clearance request and identify its institutional purpose. | G06 / Requesting Institutional Party | Clearance requirement is established. |
| 2 | Identify the subject using the applicable canonical student or workforce context. | G06 with C or E context | Correct person is associated with the request. |
| 3 | Validate that the requested clearance falls within G06 healthcare responsibility. | G06 | Request is accepted for medical assessment. |
| 4 | Review relevant authoritative healthcare information available within G06. | G06 Healthcare Professional | Existing medical context is available for assessment. |
| 5 | Perform the medical assessment required for the clearance purpose under applicable healthcare practice and institutional policy. | G06 Healthcare Professional | Medical assessment is completed. |
| 6 | Determine the medical clearance outcome supported by the authoritative assessment. | G06 Healthcare Professional / G06 | Medical clearance decision is established. |
| 7 | Record the medical clearance outcome within the G06 healthcare record and preserve its relationship to the clearance request. | G06 | Authoritative clearance outcome exists. |
| 8 | Provide the permitted clearance outcome to the relevant institutional context where required by the governed business process. | G06 / Applicable Participating Domain | Permitted institutional use of the clearance outcome is enabled. |
| 9 | Preserve evidence of the clearance assessment and decision. | G06 | Clearance decision is auditable and traceable. |

### Scenario End Discipline

The scenario ends when the G06 medical clearance decision is established and recorded. Any subsequent academic, employment, security, hostel, transport, financial or other institutional decision belongs to the owning downstream business scenario and must not be absorbed into this scenario.

---

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| A1 | The subject already has sufficient authoritative healthcare information for the clearance purpose. | Use the relevant existing G06 medical information where the applicable healthcare process permits. | Clearance assessment proceeds without unnecessary duplication of healthcare history. | G |
| A2 | The clearance requires additional medical assessment. | Perform the additional assessment within the applicable G06 healthcare workflow before determining the outcome. | Clearance decision is based on the required assessment. | G |
| A3 | The subject is a student. | Use authoritative C student context to identify the subject while retaining medical ownership in G06. | Clearance is associated with the correct student without transferring identity ownership. | G, C |
| A4 | The subject is an employee. | Use authoritative E workforce context to identify the subject while retaining medical ownership in G06. | Clearance is associated with the correct employee without transferring workforce ownership. | G, E |

These alternatives remain within the same medical-clearance outcome.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Clearance request does not identify the subject sufficiently for a reliable medical assessment. | Assessment cannot safely proceed. | Resolve the identity/context requirement through the canonical owning domain before continuing. | G with C or E |
| Requested clearance falls outside G06 healthcare authority. | G06 cannot establish the requested institutional decision. | Return or route the request to the appropriate canonical business authority; do not manufacture a medical decision outside G06 scope. | G / Owning Domain |
| Required medical assessment is incomplete. | Clearance decision cannot be reliably established. | Complete the applicable healthcare assessment before finalising the clearance outcome. | G |
| Medical evidence does not support clearance. | The requested clearance outcome cannot be issued as cleared. | Record the authoritative medical outcome and apply any separately governed institutional consequence through its owning domain. | G |
| Medical information requires correction or reassessment. | Existing clearance information may no longer represent the authoritative medical position. | Apply the applicable G06 correction/reassessment process and ensure downstream use reflects the authoritative outcome. | G |
| A downstream institutional party requests information beyond the permitted clearance outcome. | Disclosure would exceed the medical information required for the institutional purpose. | Provide only the information permitted by applicable healthcare/institutional policy. | G |

No technical failure handling, retries, queues, rollback or infrastructure recovery is described.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Institutional Health Services owns medical assessments and operational medical outcomes. | G06 | The medical clearance decision is owned by G06. | L2-G06 — Medical Workflows |
| Student ownership remains within Section C. | C | Student context may identify the subject but does not transfer medical ownership. | L2-G06 / L1 |
| Employee ownership remains within Section E. | E | Workforce context may identify the subject but does not transfer medical ownership. | L2-G06 / L1 |
| Medical records remain authoritative within Institutional Health Services. | G06 | Clearance must be based on authoritative healthcare information. | L2-G06 |
| Financial accounting and billing remain Finance responsibilities. | F | Medical clearance is not a financial decision. | L2-G06 / L1 |
| Downstream consumption does not transfer ownership. | G06 / Participating Domains | C, E or H consumption cannot become a second medical owner. | DEPENDENCY_MAP.md |

The exact medical criteria for granting or denying a particular clearance are not invented here; they must come from the applicable authoritative healthcare policy and professional assessment process.

---

## 13. State and Lifecycle Impact

The authoritative L2-G06 material reviewed for this scenario does not establish a dedicated generic Medical Clearance lifecycle with named canonical states.

Therefore this scenario does not invent a new clearance state machine.

The medical assessment and any resulting healthcare record remain governed by the lifecycle of the underlying G06 aggregate, such as the relevant Medical Encounter or Medical Certificate where applicable.

Student and workforce lifecycles remain owned independently by C and E.

The clearance decision is a business outcome derived from the applicable G06 medical assessment, not a redefinition of the lifecycle of the student or employee.

---

## 14. Business Events and Cross-Domain Collaboration

The reviewed G06 architecture establishes specific healthcare domain events for significant medical outcomes, including events such as:

- `MedicalCertificateIssued`
- `MedicalCertificateRevoked`
- `VaccinationRecorded`
- `HealthCheckCompleted`
- `HealthCampCompleted`

No dedicated canonical generic event for a Medical Clearance Decision is established in the authoritative material reviewed.

Accordingly, this scenario does **not** invent:

- `MedicalClearanceRequested`
- `MedicalClearanceGranted`
- `MedicalClearanceRejected`

If the clearance outcome is represented through an existing G06-owned medical aggregate, that aggregate's canonical lifecycle and event vocabulary remain authoritative.

Where C or E context is consumed, C/E remain authoritative for their own identity/workforce information. The collaboration does not transfer ownership.

H may participate in downstream institutional communication or service availability where the applicable business process requires it, but H does not become the medical owner.

---

## 15. Business Authorization, Approval and Audit Obligations

The medical clearance decision must be made within the healthcare authority established by G06 and applicable institutional healthcare policy.

Evidence should preserve, where applicable:

- clearance request;
- purpose of clearance;
- subject context;
- relevant healthcare information;
- medical assessment;
- resulting medical clearance decision;
- responsible healthcare authority;
- correction/reassessment history where applicable;
- permitted downstream disclosure/use;
- audit trail connecting the decision to the authoritative medical record.

The scenario does not invent a separate institutional approval chain for medical clearance.

Where the clearance is later used for an academic, employment, security, hostel, transport or other decision, that downstream domain remains responsible for its own decision and authority.

Platform Authorization and Audit infrastructure remains owned by Section A.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- The clearance request is legitimate and within G06 scope.
- The subject is correctly identified through the applicable canonical context.
- Required healthcare information has been reviewed.
- Required medical assessment has been completed.
- An authoritative medical clearance outcome has been established.
- The outcome is recorded within G06 ownership.
- Permitted downstream institutional use is enabled where required.
- Relevant evidence is preserved.

### Postconditions

- The medical clearance decision is authoritative within G06.
- Student identity remains C-owned where applicable.
- Workforce identity remains E-owned where applicable.
- No downstream institutional domain becomes owner of the medical decision.
- Any subsequent institutional decision remains with its canonical owning domain.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. The reviewed L2-G06 source does not define a dedicated generic Medical Clearance aggregate, lifecycle, or canonical event vocabulary.
2. The exact medical clearance criteria and the authoritative approval/decision role for different institutional clearance purposes are not enumerated in the retrieved L2-G06 source.
3. The canonical business contract/read-model for exposing a permitted clearance outcome to downstream institutional services is not explicitly defined in the reviewed material.

### Assumptions

- G06 owns the medical clearance decision as an institutional healthcare outcome.
- C and E provide canonical subject context where applicable.
- The medical criteria for a particular clearance purpose come from authoritative healthcare policy and professional judgement, not from this L3 scenario.
- Downstream institutional consequences remain owned by their respective domains.

### Deferred Questions

Before Architecture Approval/Lock, confirm the authoritative sources for:
- Medical Clearance lifecycle/state definitions;
- clearance-specific medical criteria;
- clearance decision authority;
- canonical clearance event(s), if required;
- permitted downstream clearance disclosure contract.

These gaps are intentionally recorded rather than fabricated.
