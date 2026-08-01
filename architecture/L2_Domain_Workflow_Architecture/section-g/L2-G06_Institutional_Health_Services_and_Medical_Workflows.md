# L2-G06 — Medical Workflows

**Repository:** SutraERP  
**Section:** G — Campus Operations  
**Domain:** G06 — Institutional Health Services & Medical Workflows  
**Document ID:** L2-G06  
**Version:** 1.0  
**Status:** Draft  
**Classification:** Level 2 Domain Architecture  
**Owner:** Campus Operations Domain  
**Document Owner:** Devang

**Upstream Dependencies**

- Section A — Platform Foundation
- Section B — Institution & Structure
- Section C — Student Lifecycle
- Section D — Academic Delivery
- Section E — People & Workforce
- G01 — Facilities Management
- G03 — Hostel Management
- Finance Domain

**Downstream Consumers**

- Student Portal
- Employee Portal
- Parent Portal
- Finance
- Security
- Institutional Reporting
- Accreditation
- Notification Services
- Analytics

---

# Table of Contents

1. Purpose
2. Architectural Position
3. Scope
4. Out of Scope
5. Domain Responsibilities
6. Design Principles
7. Domain Terminology
8. Aggregate Design
9. Bounded Context Relationships
10. Domain Model
11. Lifecycle Overview
12. Workflow Catalogue
13. Business Invariants
14. Architectural Constraints
15. Repository Principles
16. Health Centre Management
17. Patient Registration
18. Consultation Management
19. Emergency & First Aid Management
20. Prescription & Medicine Management
21. Observation & Referral Management
22. Health Programs & Medical Certificates
23. Notifications
24. Domain Events
25. Service Contracts
26. State Transition Matrix
27. Authorization Model
28. Reporting
29. Configuration
30. Audit
31. Exception Handling
32. Performance & Scalability
33. Glossary

---

# 1. Purpose

The Institutional Health Services domain governs operational healthcare services provided by educational institutions throughout the complete healthcare lifecycle.

The domain manages health centres, walk-in consultations, emergency treatment, first aid, prescriptions, medicine dispensing, observation care, referrals, medical certificates, vaccination drives, annual health checkups, health programs, and operational medical records.

The architecture is intentionally designed to support institutions with:

- No medical facility
- Basic campus medical rooms
- Full institutional health centres

through enterprise configuration without architectural redesign.

Institutional Health Services operates independently of hospital management systems, pharmacy ERP systems, insurance processing, and financial accounting.

Finance owns billing and accounting.

Student ownership remains within Section C.

Employee ownership remains within Section E.

This document defines:

- Campus Health Services
- Patient Registration
- Walk-in Consultation
- Emergency Treatment
- First Aid
- Prescription Management
- Medicine Dispensing
- Observation Care
- Hospital Referrals
- Medical Certificates
- Health Camps
- Vaccination Programs
- Annual Health Checkups
- Operational Reporting
- Domain Events
- Integration Contracts

Implementation technologies, APIs, databases, deployment architecture, and UI behaviour remain outside the scope of this document.

---

# 2. Architectural Position

Institutional Health Services represents the operational healthcare layer within SutraERP.

Repository Position

```text
Platform Foundation
        │
        ▼
Student / Employee Lifecycle
        │
        ▼
Institutional Health Services
        │
 ┌────────────┼──────────────┬──────────────┐
 ▼            ▼              ▼
Finance   Notifications   Institutional Reporting
```

Institutional Health Services governs campus healthcare operations.

It does **not** replace:

- Hospital Information Systems
- Pharmacy ERP Systems
- Insurance Platforms
- Diagnostic Laboratory Systems
- Electronic Medical Record Platforms used by hospitals

---

# 3. Scope

Institutional Health Services owns:

- Health Centre Operations
- Medical Room Operations
- Patient Registration
- Walk-in Consultation
- Emergency Treatment
- First Aid
- Medical History
- Allergy Records
- Chronic Condition Records
- Prescriptions
- Medicine Dispensing
- Observation Care
- Hospital Referrals
- Medical Certificates
- Sick Leave Recommendations
- Health Camps
- Vaccination Drives
- Annual Health Checkups
- Health Screening Programs
- Operational Dashboards
- Operational Audit

The domain serves as the canonical institutional healthcare engine for the institution.

---

# 4. Out of Scope

The following responsibilities remain outside this domain.

---

## Hospital Management

Examples include:

- Surgery
- Inpatient Wards
- ICU
- Operating Theatre
- Hospital Billing
- Laboratory Information Systems
- Radiology Systems

Institutional Health Services may refer patients to external hospitals.

---

## Financial Accounting

Owned by Finance.

Examples:

- Medical Billing
- Accounting
- Insurance Settlement
- Payment Collection
- Receipts

Health Services records operational medical events only.

---

## Student Lifecycle

Owned by Section C.

Examples:

- Student Identity
- Student Registration
- Student Programme
- Student Status

Institutional Health Services consumes student information.

---

## Employee Management

Owned by Section E.

Examples:

- Doctors
- Nurses
- Medical Officers
- Administrative Staff

Health Services performs operational assignments only.

---

## Biomedical Equipment Maintenance

Owned by G01.

Examples:

- Equipment Servicing
- Preventive Maintenance
- Vendor Repairs
- Calibration

Institutional Health Services may recommend maintenance.

---

## Authentication & Workflow

Authentication, authorization, workflow execution, notifications, audit infrastructure, and document management remain Platform responsibilities.

---

# 5. Domain Responsibilities

Institutional Health Services owns:

- Health Centre Operations
- Consultation Management
- Emergency Care
- First Aid
- Medical History
- Prescriptions
- Medicine Dispensing
- Observation Care
- Referral Management
- Medical Certificates
- Vaccination Programs
- Health Camps
- Annual Checkups
- Health Screening
- Operational Dashboards
- Operational Audit

The domain maintains complete medical operational history throughout the healthcare lifecycle.

---

# 6. Design Principles

## Operational Ownership

Institutional Health Services owns campus healthcare operations.

Hospital ownership remains external.

---

## Configuration over Customisation

Health centre availability, consultation workflows, medicine policies, observation policies, emergency workflows, referral policies, medical certificate templates, notification templates, and healthcare programs shall remain configuration driven.

Institutions without medical facilities shall be able to disable healthcare modules without requiring architectural modification.

---

## Manual First Operations

Institutional healthcare remains human-driven.

Medical professionals retain operational authority.

The ERP supports operational recording, governance, and coordination without replacing clinical judgement.

---

## Event Driven Architecture

Every significant healthcare activity shall publish immutable domain events.

Downstream systems consume medical events without modifying healthcare state.

---

## Audit First

Every healthcare activity shall produce immutable audit history.

Medical records shall remain permanently traceable according to institutional retention policies.

---

## Enterprise Scalability

The architecture shall support:

- Multi Institution
- Multi Campus
- No Medical Facility
- Basic Medical Room
- Full Health Centre
- Future Telemedicine
- Future Wearable Integration
- Future AI Health Advisory
- Future Hospital Integration

without architectural redesign.

# 7. Domain Terminology

| Term | Definition |
|------|------------|
| Health Centre | Institutional medical facility providing healthcare services. |
| Medical Room | Basic healthcare facility providing first aid and primary treatment. |
| Consultation | Medical interaction between healthcare professional and patient. |
| Patient | Student, employee, guest, or authorised visitor receiving healthcare services. |
| Medical Encounter | Complete operational record of a healthcare visit. |
| Emergency Case | Medical incident requiring immediate institutional response. |
| First Aid | Immediate medical assistance provided before advanced treatment. |
| Observation Care | Temporary monitoring of a patient within the institution. |
| Prescription | Medical recommendation for treatment and medicines. |
| Medicine Dispense | Operational record of medicines issued by the institution. |
| Referral | Recommendation directing a patient to an external healthcare provider. |
| Medical Certificate | Institution-issued document certifying medical condition or treatment. |
| Health Camp | Organised institutional healthcare program. |
| Vaccination Program | Organised immunisation activity conducted by the institution. |
| Annual Health Checkup | Scheduled institutional health assessment programme. |

---

# 8. Aggregate Design

## Aggregate Roots

Institutional Health Services consists of the following aggregate roots:

- Health Centre
- Medical Encounter
- Emergency Case
- Prescription
- Medicine Dispense
- Observation Stay
- Referral
- Medical Certificate
- Health Program
- Vaccination Campaign
- Annual Health Checkup

Each aggregate owns its complete operational lifecycle.

---

## Aggregate Responsibilities

### Health Centre

Owns:

- Health Centre Information
- Operating Hours
- Medical Facility Configuration
- Operational Status

---

### Medical Encounter

Owns:

- Patient Registration
- Consultation
- Diagnosis
- Treatment
- Clinical Notes
- Follow-up Recommendation

---

### Emergency Case

Owns:

- Emergency Registration
- Emergency Treatment
- Incident Classification
- Referral Decision
- Emergency Outcome

---

### Prescription

Owns:

- Prescription Details
- Prescribed Medicines
- Dosage
- Duration
- Instructions

---

### Medicine Dispense

Owns:

- Dispensed Medicines
- Quantity
- Dispensing Date
- Dispensing Professional
- Medicine Issue History

Lightweight medicine inventory remains within this domain.

Enterprise pharmacy management remains outside scope.

---

### Observation Stay

Owns:

- Observation Start
- Observation Notes
- Observation Duration
- Observation Outcome

---

### Referral

Owns:

- Referral Reason
- External Healthcare Provider
- Referral Status
- Referral History

---

### Medical Certificate

Owns:

- Certificate Type
- Certificate Validity
- Medical Recommendation
- Issuing Medical Professional

---

### Health Program

Owns:

- Health Camp
- Screening Programme
- Awareness Programme
- Programme Outcome

---

### Vaccination Campaign

Owns:

- Vaccination Drive
- Vaccination Records
- Vaccine Administration
- Campaign Completion

---

### Annual Health Checkup

Owns:

- Checkup Schedule
- Assessment
- Findings
- Recommendations

---

## Aggregate Identity

Every aggregate shall possess:

- Immutable System Identifier
- Configurable Business Code
- Human Readable Reference

Aggregate identity shall remain immutable throughout its lifecycle.

---

# 9. Bounded Context Relationships

## Upstream Domains

### Platform Foundation

Provides:

- Authentication
- Authorization
- Workflow Engine
- Notification Framework
- Audit Framework
- Configuration Framework
- Document Management
- Event Bus

---

### Section C — Student Lifecycle

Provides:

- Student Identity
- Student Status
- Programme
- Guardian Information
- Emergency Contacts

Institutional Health Services consumes student information.

Student ownership remains external.

---

### Section E — People & Workforce

Provides:

- Doctors
- Nurses
- Medical Officers
- Healthcare Staff
- Employee Information

Personnel ownership remains external.

---

### G03 — Hostel Management

Provides:

- Hostel Residency
- Room Allocation
- Resident Information

Institutional Health Services consumes hostel information for healthcare coordination.

---

### G01 — Facilities Management

Provides:

- Biomedical Equipment Maintenance
- Facility Maintenance
- Service Requests

Institutional Health Services recommends maintenance where required.

---

### Finance

Consumes:

- Consultation Charges
- Medicine Charges
- Medical Service References
- Payment Status

Finance owns accounting and billing.

---

## Downstream Consumers

Institutional Health Services publishes operational information to:

### Student Portal

Consumes:

- Medical History
- Medical Certificates
- Vaccination Records
- Health Notifications

---

### Employee Portal

Consumes:

- Medical Certificates
- Occupational Health Records
- Health Notifications

---

### Parent Portal

Consumes:

- Student Emergency Notifications
- Health Alerts
- Medical Certificates (where permitted)

---

### Institutional Reporting

Consumes:

- Health Statistics
- Medical Service Reports
- Health Camp Reports
- Accreditation Reports

---

# 10. Domain Model

Institutional Health Services governs operational healthcare throughout the institution.

```text
Patient Registration
        │
        ▼
Medical Encounter
        │
 ┌──────┼────────────┬────────────┐
 ▼      ▼            ▼            ▼
Consultation Prescription Emergency Observation
        │
        ▼
Medicine Dispense
        │
        ▼
Referral (If Required)
        │
        ▼
Medical Certificate / Follow-up
```

Supporting operational model:

```text
Medical Encounter
        │
        ├── Patient
        ├── Consultation
        ├── Prescription
        ├── Medicine Dispense
        ├── Referral
        ├── Medical Certificate
        └── Audit
```

---

# 11. Lifecycle Overview

## Medical Encounter Lifecycle

```text
Registered
     │
Consultation
     │
Treatment
     │
Completed
     │
Closed
```

---

## Emergency Case Lifecycle

```text
Reported
     │
Assessment
     │
Treatment
     │
Referral (Optional)
     │
Closed
```

---

## Prescription Lifecycle

```text
Created
    │
Issued
    │
Dispensed
    │
Completed
```

---

## Observation Stay Lifecycle

```text
Admitted
    │
Observed
    │
Released
```

---

## Referral Lifecycle

```text
Recommended
      │
Accepted
      │
Completed
```

---

# 12. Workflow Catalogue

Institutional Health Services governs the following workflow families.

## Patient Operations

- Patient Registration
- Medical History Review
- Consultation Registration
- Follow-up Management

---

## Clinical Operations

- Walk-in Consultation
- First Aid
- Emergency Treatment
- Observation Care
- Referral Management

---

## Prescription Operations

- Prescription Creation
- Medicine Dispensing
- Prescription History

---

## Healthcare Programs

- Health Camps
- Vaccination Drives
- Annual Health Checkups
- Health Screening Programmes

---

## Administrative Operations

- Medical Certificates
- Sick Leave Recommendation
- Occupational Health
- Medical Clearance

---

# 13. Business Invariants

The following invariants shall always remain true.

- Every Medical Encounter shall reference exactly one Patient.
- Every Prescription shall belong to one Medical Encounter.
- Every Medicine Dispense shall reference one valid Prescription.
- Medical history shall never be deleted.
- Emergency Cases shall remain permanently auditable.
- Referral shall reference one Medical Encounter.
- Medical certificates shall reference one authorised Medical Encounter.
- Finance owns billing.
- G01 owns biomedical equipment maintenance.
- Every lifecycle transition shall be audited.

Violation of business invariants shall reject the transaction.

---

# 14. Architectural Constraints

Institutional Health Services shall not:

- Replace Hospital Information Systems
- Execute Financial Accounting
- Own Student Identity
- Own Employee Identity
- Manage Biomedical Equipment
- Replace Notification Services
- Replace Authentication Services
- Replace Enterprise Workflow Engine

Healthcare operations shall occur exclusively through published business contracts.

---

# 15. Repository Principles

Institutional Health Services conforms to the following enterprise principles.

- Domain-Driven Design
- Event-Driven Architecture
- CQRS
- Canonical Domain Ownership
- Configuration over Customisation
- Manual First Healthcare Operations
- Effective-Dated Configuration
- Immutable Medical History
- Multi Campus Support
- Multi Institution Support
- Enterprise Scalability
- High Cohesion
- Loose Coupling
- Future Telemedicine Ready
- Future Hospital Integration Ready
```

# 16. Health Centre Management

## Purpose

Health Centre Management governs operational management of institutional healthcare facilities.

The architecture supports institutions with:

- No Health Centre
- Basic Medical Room
- Full Campus Health Centre

through enterprise configuration.

---

## Health Centre Types

Supported facility types include:

- Medical Room
- Campus Clinic
- Health Centre
- First Aid Centre
- Emergency Medical Unit

Additional facility types shall remain configurable.

---

## Health Centre Workflow

```text
Health Centre Created
          │
Configuration
          │
Operational Validation
          │
Activation
          │
Healthcare Services Available
```

---

## Health Centre Information

Typical information includes:

- Health Centre Code
- Health Centre Name
- Campus
- Facility Type
- Operating Hours
- Emergency Availability
- Contact Information
- Operational Status

Additional information shall remain configurable.

---

## Operational Status

Supported statuses include:

- Draft
- Active
- Temporarily Closed
- Under Maintenance
- Emergency Only
- Retired

Status values shall remain configurable.

---

# 17. Patient Registration

## Purpose

Patient Registration governs operational registration of individuals seeking healthcare services.

Institutional Health Services references existing institutional identities.

It never owns person identities.

---

## Supported Patient Types

Healthcare services may support:

- Students
- Faculty
- Employees
- Guests
- Visitors

Patient categories shall remain configurable.

---

## Registration Workflow

```text
Patient Identified
        │
Identity Validation
        │
Medical History Retrieved
        │
Visit Registered
        │
Consultation Ready
```

---

## Registration Validation

Validation may consider:

- Institutional Identity
- Campus
- Active Status
- Emergency Classification
- Healthcare Eligibility

Validation rules shall remain configurable.

---

## Medical History

Patient history may include:

- Previous Visits
- Allergies
- Chronic Conditions
- Vaccinations
- Previous Prescriptions
- Previous Referrals

Historical information shall remain immutable.

---

# 18. Consultation Management

## Purpose

Consultation Management governs routine healthcare consultations conducted within institutional health facilities.

Walk-in consultations remain the default operational model.

Appointment scheduling remains optional through configuration.

---

## Consultation Workflow

```text
Patient Registered
       │
Clinical Assessment
       │
Diagnosis
       │
Treatment
       │
Prescription (If Required)
       │
Consultation Closed
```

---

## Consultation Information

Typical consultation information includes:

- Consultation Number
- Healthcare Professional
- Consultation Type
- Symptoms
- Diagnosis
- Treatment Notes
- Follow-up Recommendation

Additional information shall remain configurable.

---

## Consultation Types

Examples include:

- Walk-in Consultation
- Scheduled Consultation
- Follow-up Consultation
- Occupational Health Consultation
- Preventive Consultation

Consultation types shall remain configurable.

---

## Follow-up

Follow-up recommendations may include:

- Return Visit
- Referral
- Observation
- Medical Certificate
- Health Program Participation

Follow-up policies shall remain configurable.

---

# 19. Emergency & First Aid Management

## Purpose

Emergency & First Aid Management governs immediate institutional response to medical incidents.

The objective is rapid stabilisation and, where required, referral to external healthcare providers.

---

## Emergency Workflow

```text
Incident Reported
       │
Emergency Assessment
       │
Immediate Treatment
       │
Referral Decision
       │
Patient Stabilised
       │
Case Closed
```

---

## Emergency Categories

Examples include:

- Injury
- Accident
- Medical Emergency
- Sports Injury
- Hostel Incident
- Workplace Incident

Categories shall remain configurable.

---

## First Aid

First Aid operations may include:

- Wound Care
- Minor Injury Treatment
- Basic Medication
- Vital Assessment
- Observation Recommendation

Institutional policies shall determine treatment scope.

---

## Ambulance Coordination

Institutional Health Services may record:

- Ambulance Request
- Provider
- Departure Time
- Destination
- Arrival Confirmation

Ambulance fleet management remains outside this domain.

---

## Emergency Notifications

Where configured, emergency events may notify:

- Parents
- Emergency Contacts
- Hostel Administration
- Security
- Institutional Authorities

Notification delivery remains within Section H.

---

# 20. Prescription & Medicine Management

## Purpose

Prescription & Medicine Management governs operational recording of prescribed and dispensed medicines.

This domain intentionally implements only lightweight medicine inventory.

Enterprise pharmacy systems remain outside scope.

---

## Prescription Workflow

```text
Consultation Completed
         │
Prescription Created
         │
Medicine Validation
         │
Medicine Dispensed
         │
Prescription Closed
```

---

## Prescription Information

Typical prescription information includes:

- Prescribing Professional
- Medicines
- Dosage
- Frequency
- Duration
- Instructions

Additional information shall remain configurable.

---

## Medicine Dispensing

Dispensing may record:

- Medicine
- Quantity
- Batch Reference
- Expiry Date
- Dispensing Professional
- Dispensing Time

Historical dispensing records shall remain immutable.

---

## Medicine Inventory

The domain may maintain lightweight operational inventory including:

- Available Quantity
- Reorder Threshold
- Expiry Date
- Operational Availability

Procurement and enterprise pharmacy management remain outside this domain.

---

## Medicine Expiry

Expiry monitoring may identify:

- Near Expiry Medicines
- Expired Medicines
- Restricted Medicines

Notification policies shall remain configurable.

---

# 21. Observation & Referral Management

## Purpose

Observation & Referral Management governs temporary patient monitoring and referral to external healthcare providers.

Observation is intended for short-duration institutional care.

---

## Observation Workflow

```text
Observation Required
        │
Patient Monitored
        │
Condition Improved?
    ┌──────┴──────┐
    │             │
   Yes           No
    │             │
Discharge     Referral
```

---

## Observation Information

Typical information includes:

- Observation Start
- Observation End
- Observation Notes
- Vital Observations
- Medical Review
- Outcome

Observation policies shall remain configurable.

---

## Referral Workflow

```text
Referral Recommended
         │
Healthcare Provider Selected
         │
Referral Issued
         │
Patient Transferred
         │
Referral Closed
```

---

## Referral Types

Examples include:

- General Hospital
- Specialist Consultation
- Diagnostic Laboratory
- Imaging Centre
- Emergency Hospital

Referral categories shall remain configurable.

---

## Referral History

Historical referral information shall preserve:

- Referral Date
- Referral Reason
- Destination
- Medical Summary
- Follow-up Status

Referral history shall remain immutable.

---

# 22. Health Programs & Medical Certificates

## Purpose

Health Programs govern institutional preventive healthcare initiatives.

Medical Certificates provide authorised medical documentation.

---

## Health Programs

Supported programs include:

- Health Camps
- Vaccination Drives
- Annual Health Checkups
- Health Screening
- Awareness Campaigns

Additional programmes shall remain configurable.

---

## Health Program Workflow

```text
Programme Planned
       │
Participant Registration
       │
Programme Conducted
       │
Findings Recorded
       │
Programme Closed
```

---

## Medical Certificates

Medical certificates may include:

- Fitness Certificate
- Medical Leave Recommendation
- Return-to-Campus Certificate
- Health Assessment Certificate

Certificate templates shall remain configurable.

---

## Medical Certificate Workflow

```text
Medical Assessment
        │
Eligibility Verified
        │
Certificate Generated
        │
Authorised Approval
        │
Certificate Issued
```

---

## Occupational Health

Where applicable, occupational health may support:

- Employee Health Assessment
- Workplace Injury Records
- Periodic Health Evaluation
- Return-to-Work Recommendation

Operational policies shall remain configuration driven.

# 23. Operational Notifications

## Purpose

Operational Notifications inform healthcare stakeholders regarding significant healthcare activities requiring awareness or action.

Notification delivery remains within Section H.

Institutional Health Services publishes business events only.

---

## Notification Sources

Notifications may originate from:

- Consultation Registration
- Emergency Case
- Observation Admission
- Referral Issued
- Prescription Generated
- Medicine Dispensed
- Medical Certificate Issued
- Vaccination Reminder
- Annual Health Checkup
- Health Camp
- Medicine Expiry
- Follow-up Reminder

---

## Typical Notifications

Examples include:

- Consultation Registered
- Follow-up Due
- Emergency Case Registered
- Parent Emergency Alert
- Referral Issued
- Observation Completed
- Medical Certificate Available
- Vaccination Reminder
- Health Camp Registration
- Medicine Near Expiry (Medical Staff)

Notification templates shall remain configurable.

---

## Notification Workflow

```text
Business Event
      │
Domain Event Published
      │
Notification Framework
      │
Recipient Resolution
      │
Delivery
```

Institutional Health Services shall never implement notification delivery.

---

# 24. Domain Events

Institutional Health Services publishes immutable business events.

Every significant healthcare activity shall publish a canonical domain event.

---

## Health Centre Events

| Event | Description |
|--------|-------------|
| HealthCentreCreated | Health centre created |
| HealthCentreActivated | Health centre activated |
| HealthCentreClosed | Health centre temporarily closed |

---

## Medical Encounter Events

| Event | Description |
|--------|-------------|
| PatientRegistered | Patient visit registered |
| ConsultationStarted | Consultation initiated |
| ConsultationCompleted | Consultation completed |
| FollowUpRecommended | Follow-up advised |

---

## Emergency Events

| Event | Description |
|--------|-------------|
| EmergencyReported | Emergency case registered |
| EmergencyTreatmentCompleted | Emergency treatment completed |
| AmbulanceRequested | Ambulance requested |
| ReferralRecommended | External referral recommended |

---

## Prescription Events

| Event | Description |
|--------|-------------|
| PrescriptionCreated | Prescription generated |
| MedicineDispensed | Medicine issued |
| PrescriptionCompleted | Prescription closed |

---

## Observation Events

| Event | Description |
|--------|-------------|
| ObservationStarted | Observation initiated |
| ObservationCompleted | Observation completed |

---

## Health Program Events

| Event | Description |
|--------|-------------|
| HealthCampCreated | Health camp scheduled |
| HealthCampCompleted | Health camp completed |
| VaccinationRecorded | Vaccination recorded |
| HealthCheckCompleted | Annual health assessment completed |

---

## Medical Certificate Events

| Event | Description |
|--------|-------------|
| MedicalCertificateIssued | Certificate generated |
| MedicalCertificateRevoked | Certificate withdrawn |

---

# 25. Service Contracts

Institutional Health Services exposes operational healthcare services.

---

## Patient Services

- Register Patient
- Retrieve Medical History
- Register Follow-up
- Query Patient Visits

---

## Consultation Services

- Start Consultation
- Complete Consultation
- Record Diagnosis
- Record Treatment

---

## Prescription Services

- Create Prescription
- Dispense Medicine
- Query Prescription
- Close Prescription

---

## Emergency Services

- Register Emergency
- Record First Aid
- Create Referral
- Close Emergency Case

---

## Health Program Services

- Create Health Camp
- Register Participant
- Record Vaccination
- Record Health Checkup

---

## Medical Certificate Services

- Generate Certificate
- Issue Certificate
- Verify Certificate
- Revoke Certificate

All interactions shall occur through published service contracts.

---

# 26. State Transition Matrix

| Aggregate | Primary Lifecycle |
|-----------|-------------------|
| Health Centre | Draft → Configured → Active → Closed |
| Medical Encounter | Registered → Consultation → Treatment → Completed → Closed |
| Emergency Case | Reported → Assessment → Treatment → Closed |
| Prescription | Created → Issued → Dispensed → Completed |
| Observation Stay | Admitted → Observed → Released |
| Referral | Recommended → Accepted → Completed |
| Medical Certificate | Draft → Approved → Issued → Archived |
| Health Program | Planned → Active → Completed |
| Vaccination Campaign | Planned → Running → Completed |
| Annual Health Checkup | Scheduled → Conducted → Completed |

Lifecycle extensions shall remain configuration driven.

---

# 27. Authorization Model

## Purpose

Institutional Health Services shall enforce enterprise authorization through the Platform Authorization Framework.

Authentication remains the responsibility of the Platform.

Healthcare operations govern business permissions only.

---

## Authorization Principles

Authorization shall support:

- Role Based Access Control (RBAC)
- Campus Scoped Authorization
- Health Centre Scoped Authorization
- Delegated Responsibility
- Temporary Delegation
- Effective-Dated Permissions
- Multi-Institution Isolation

---

## Typical Operational Roles

Examples include:

- Medical Administrator
- Chief Medical Officer
- Doctor
- Nurse
- Medical Assistant
- Pharmacist (Lightweight Dispensing)
- First Aid Officer
- Health Centre Operator
- Read Only Auditor

Institutions may define additional operational roles.

---

## Authorization Matrix

| Activity | Typical Authorization |
|----------|-----------------------|
| Configure Health Centre | Medical Administrator |
| Register Patient | Medical Staff |
| Conduct Consultation | Doctor |
| Dispense Medicines | Doctor / Nurse / Authorised Medical Staff |
| Record Emergency Case | Medical Staff |
| Issue Medical Certificate | Doctor |
| Manage Health Camp | Medical Administrator |
| Record Vaccination | Medical Staff |
| View Reports | Management |

Authorization policies shall remain configurable.

---

## Medical Confidentiality

Access to healthcare records shall be governed by:

- Need-to-Know Principle
- Role-Based Access
- Complete Audit Logging
- Institution Privacy Policies

Medical confidentiality shall never be bypassed.

---

## Delegation

Operational authority may be delegated.

Delegation shall include:

- Delegate User
- Effective Start Date
- Effective End Date
- Delegation Reason
- Automatic Expiry

Delegation history shall remain immutable.

# 28. Reporting Responsibilities

## Purpose

Institutional Health Services provides operational healthcare reporting supporting institutional governance.

Analytical processing remains outside this domain.

---

## Operational Reports

The domain shall support reporting for:

- Daily Consultations
- Walk-in Visits
- Emergency Cases
- First Aid Treatments
- Observation Cases
- Referrals
- Prescriptions
- Medicine Dispensing
- Medical Certificates
- Health Program Participation

---

## Management Reports

Examples include:

- Health Centre Utilisation
- Consultation Statistics
- Emergency Statistics
- Medicine Usage Summary
- Referral Summary
- Observation Summary
- Vaccination Coverage
- Annual Health Checkup Summary
- Occupational Health Summary

---

## Accreditation Reports

Examples include:

- Student Health Statistics
- Institutional Health Services
- Vaccination Program Reports
- Annual Medical Examination Reports
- Health Camp Activities
- Emergency Response Statistics

---

## Executive Dashboards

Executive dashboards may include:

- Active Health Centres
- Daily Patient Count
- Emergency Cases
- Observation Cases
- Referrals
- Medicine Inventory Alerts
- Upcoming Health Camps
- Vaccination Progress

Dashboard composition shall remain configurable.

---

# 29. Configuration Management

## Purpose

Institutional Health Services shall support enterprise configuration without source code modification.

The architecture shall support institutions with:

- No Medical Facility
- Basic Medical Room
- Full Institutional Health Centre

through configuration alone.

---

## Configurable Elements

Institutions may configure:

### Health Centre

- Facility Types
- Operating Hours
- Emergency Availability
- Consultation Policies

---

### Consultation

- Consultation Categories
- Walk-in Policies
- Appointment Behaviour
- Follow-up Policies

---

### Emergency Care

- Emergency Categories
- Referral Policies
- Observation Policies
- Ambulance Coordination

---

### Medicine Management

- Medicine Categories
- Dispensing Policies
- Expiry Warning Thresholds
- Reorder Levels

---

### Health Programs

- Vaccination Programs
- Health Camps
- Annual Health Checkups
- Screening Programmes

---

### Medical Certificates

- Certificate Templates
- Approval Policies
- Validity Rules

---

### Notifications

- Notification Templates
- Reminder Policies
- Escalation Rules
- Recipient Policies

---

## Configuration Principles

Configuration shall support:

- Version Control
- Effective Dating
- Institution Isolation
- Rollback
- Complete Audit History

---

# 30. Audit Responsibilities

## Purpose

Institutional Health Services shall maintain immutable healthcare audit history.

Every significant healthcare activity shall be auditable.

---

## Audit Coverage

Audit records shall include:

- User
- Timestamp
- Business Action
- Previous State
- New State
- Patient Reference
- Medical Encounter Reference
- Prescription Reference
- Business Reason
- Source System

---

## Audited Activities

Examples include:

- Patient Registration
- Consultation
- Prescription Creation
- Medicine Dispensing
- Emergency Registration
- Referral Creation
- Medical Certificate Issuance
- Vaccination Recording
- Health Camp Activities
- Configuration Changes

---

## Audit Principles

Audit history shall:

- Never be deleted
- Never be modified
- Support operational investigations
- Support compliance reviews
- Support accreditation
- Support historical reconstruction

Medical audit history shall remain permanently available according to institutional retention policies.

---

# 31. Exception Handling

## Purpose

Institutional Health Services shall manage operational healthcare exceptions while preserving business consistency.

---

## Typical Exceptions

Examples include:

- Patient Identity Not Found
- Medical Professional Unavailable
- Medicine Out of Stock
- Medicine Expired
- Referral Required
- Emergency Escalation
- Observation Capacity Reached
- Medical Certificate Rejected
- Authorization Failure
- Configuration Error

---

## Exception Workflow

```text
Business Exception
        │
Validation
        │
Recovery Strategy
        │
Audit Recorded
        │
Notification Published
        │
Operational Continuation
```

---

## Recovery Principles

Recovery actions may include:

- Administrative Override
- Alternative Medical Professional
- External Referral
- Alternative Medicine Recommendation
- Emergency Escalation
- Manual Approval
- Parent / Guardian Notification

Recovery behaviour shall remain configuration driven.

---

# 32. Performance & Scalability

Institutional Health Services shall support enterprise-scale educational institutions.

The architecture shall support:

- Multi Institution
- Multi Campus
- Multiple Health Centres
- Institutions Without Medical Facilities
- Basic Medical Rooms
- Full Institutional Health Centres
- Thousands of Medical Encounters
- Long-Term Medical History
- Health Programs
- Vaccination Campaigns
- Future Telemedicine
- Future Wearable Device Integration
- Future AI Health Advisory
- Future Hospital Integration

The architecture shall remain horizontally scalable without domain redesign.

---

# 33. Glossary

| Term | Definition |
|------|------------|
| Health Centre | Institutional facility providing healthcare services. |
| Medical Encounter | Operational healthcare visit between a patient and healthcare provider. |
| Consultation | Medical assessment and treatment activity. |
| Emergency Case | Medical incident requiring immediate institutional response. |
| Observation Stay | Temporary institutional patient monitoring. |
| Prescription | Medical recommendation for treatment and medicines. |
| Medicine Dispense | Operational record of medicines issued. |
| Referral | Recommendation directing a patient to an external healthcare provider. |
| Medical Certificate | Institution-issued medical document. |
| Health Program | Organised institutional healthcare initiative. |

---

# Appendix A — Canonical Operational Flow

```text
Patient Registration
        │
Medical Encounter
        │
Consultation
        │
Prescription
        │
Medicine Dispense
        │
Observation (If Required)
        │
Referral (If Required)
        │
Medical Certificate
        │
Healthcare History
        │
Audit History
```

---

# Appendix B — Canonical Ownership Matrix

| Business Concept | Canonical Owner |
|------------------|-----------------|
| Health Centre | G06 Institutional Health Services |
| Medical Encounter | G06 Institutional Health Services |
| Emergency Case | G06 Institutional Health Services |
| Prescription | G06 Institutional Health Services |
| Medicine Dispense | G06 Institutional Health Services |
| Observation Stay | G06 Institutional Health Services |
| Referral | G06 Institutional Health Services |
| Medical Certificate | G06 Institutional Health Services |
| Health Camp | G06 Institutional Health Services |
| Vaccination Program | G06 Institutional Health Services |
| Annual Health Checkup | G06 Institutional Health Services |
| Student Identity | Section C |
| Employee Identity | Section E |
| Biomedical Equipment Maintenance | G01 Facilities Management |
| Medical Billing | Finance Domain |
| Authentication | Section A |
| Notifications | Section H |

---

# Appendix C — Enterprise Design Principles

Institutional Health Services follows the enterprise architectural principles established by SutraERP.

- Domain-Driven Design (DDD)
- Event-Driven Architecture
- CQRS
- Canonical Domain Ownership
- Configuration over Customisation
- Manual First Healthcare Operations
- Effective-Dated Configuration
- Immutable Medical History
- Multi-Campus Architecture
- Multi-Institution Architecture
- Audit First
- High Cohesion
- Loose Coupling
- Enterprise Scalability
- No Medical Facility Support
- Basic Medical Room Support
- Full Health Centre Support
- Future Telemedicine Ready
- Future Wearable Integration Ready
- Future AI Health Advisory Ready
- Future Hospital Integration Ready

---

**End of Document**

**L2-G06 — Institutional Health Services & Medical Workflows**

**Version:** 1.0  
**Status:** Architecture Draft (Ready for Architectural Review)
