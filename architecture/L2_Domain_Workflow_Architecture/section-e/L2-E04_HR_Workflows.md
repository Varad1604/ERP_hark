# L2-E04 --- HR Workflows

## Document Metadata

  Attribute            Value
  -------------------- --------------------------------------
  Document Name        L2-E04 --- HR Workflows
  Domain               HR Operations Workflows
  Section              Section E --- People & Workforce
  Architecture Layer   Level 2 Domain Workflow Architecture
  Architecture Style   Domain Driven Design (DDD)
  Domain Type          Workflow-Oriented Bounded Context
  Status               Architecture Specification
  Version              1.0
  Owner                SutraERP Architecture Team

------------------------------------------------------------------------

# 1. Purpose

## 1.1 Purpose Statement

The purpose of the **L2-E04 --- HR Workflows** bounded context is to
define and govern the operational workflows required for managing
employee-related human resource processes within SutraERP.

The domain acts as the enterprise HR process orchestration layer
responsible for executing, coordinating, and governing HR operations
while maintaining strict ownership boundaries with specialized domains.

HR Workflows SHALL provide controlled lifecycle management for:

-   Employee onboarding.
-   Probation management.
-   Confirmation workflows.
-   Employee requests.
-   HR case management.
-   Leave workflows.
-   Absence workflows.
-   Employee document workflows.
-   Certificate workflows.
-   Employment change workflows.
-   Promotion workflows.
-   Transfer workflows.
-   Contract renewal workflows.
-   Separation and exit workflows.
-   HR compliance workflows.
-   Retention case workflows.

------------------------------------------------------------------------

## 1.2 Architectural Intent

The HR Workflows domain exists to ensure that HR business processes are:

-   Consistent across institutions.
-   Configurable according to institutional requirements.
-   Traceable through complete audit history.
-   Integrated through service contracts and domain events.
-   Securely controlled through authorization policies.
-   Independently evolvable without violating domain boundaries.

The domain SHALL NOT become a central employee database.

``` mermaid
flowchart LR

HR["L2-E04 HR Workflows"]

Employee["E07 Employee Profile"]

Payroll["Payroll Domain"]

Organization["Organization Domain"]

Identity["Identity Domain"]

Documents["Document Platform"]

HR -->|Workflow Decisions| Employee
HR -->|Business Events| Payroll
HR -->|Assignment Requests| Organization
HR -->|Access Requirements| Identity
HR -->|Document Requests| Documents
```

------------------------------------------------------------------------

# 2. Scope

## 2.1 Scope Statement

The scope of L2-E04 HR Workflows includes operational HR processes
required to manage employee lifecycle events and HR service operations.

The domain focuses on:

-   Workflow orchestration.
-   Business approvals.
-   HR case management.
-   Employee requests.
-   Lifecycle transitions.
-   Compliance workflows.
-   HR document processes.
-   Integration with workforce-related domains.

------------------------------------------------------------------------

# 2.2 In Scope Capabilities

## Employee Lifecycle Workflows

-   Employee onboarding.
-   Probation workflows.
-   Confirmation workflows.
-   Employment status changes.
-   Promotion workflows.
-   Transfer workflows.
-   Contract renewal workflows.
-   Separation workflows.
-   Exit clearance workflows.

## HR Case Management

-   Employee HR requests.
-   Manager requests.
-   HR administrative cases.
-   Employee grievances.
-   SLA management.
-   Escalation workflows.
-   Resolution tracking.

## Leave and Absence Workflows

-   Leave requests.
-   Leave approvals.
-   Leave cancellation.
-   Leave policy execution.
-   Absence case management.

## HR Document Workflows

-   Certificate requests.
-   Employment document workflows.
-   Appointment letters.
-   Experience certificates.
-   Relieving certificates.
-   Document verification workflows.

## HR Compliance Workflows

-   Compliance cases.
-   Required document tracking.
-   Policy acknowledgement workflows.
-   Verification workflows.

------------------------------------------------------------------------

# 2.3 Out of Scope

  Capability                      Owning Domain
  ------------------------------- ------------------------------
  Employee profile data           E07 Employee Profile
  Faculty capability management   E03 Faculty Workforce
  Authentication                  Identity Domain
  Organization hierarchy          Organization Domain
  Payroll calculation             Payroll Domain
  Attendance capture              Attendance Domain
  Document storage engine         Document Platform
  Analytics database              Reporting & Analytics Domain

------------------------------------------------------------------------

# 2.4 Scope Boundary Principle

``` text
HR Workflow Domain

Owns:
Business Process Execution

Does Not Own:
External Domain Data Ownership
```

------------------------------------------------------------------------

# 2.5 Scope Summary

  Area                     Status
  ------------------------ ----------
  HR Lifecycle Workflows   Included
  HR Case Management       Included
  Employee Requests        Included
  Leave Workflows          Included
  HR Documents             Included
  HR Compliance            Included
  Employee Profile Data    Excluded
  Payroll Processing       Excluded
  Attendance Processing    Excluded



# 3. Upstream Dependencies

## 3.1 Dependency Philosophy

L2-E04 --- HR Workflows operates as a workflow orchestration bounded
context.

The domain SHALL NOT directly own or duplicate upstream domain data.

Upstream dependencies provide authoritative information required for HR
process execution.

The dependency model follows:

``` text
Upstream Domain

        ↓

Business Context / Data / Event

        ↓

HR Workflow Execution
```

------------------------------------------------------------------------

# 3.2 Primary Upstream Dependencies

## E07 --- Employee Profile Domain

### Dependency Purpose

E04 depends on Employee Profile for authoritative employee identity and
profile information.

E07 owns:

-   Employee identity.
-   Employee profile record.
-   Employment profile representation.
-   Personal information.
-   Employee identifiers.

E04 consumes:

-   Employee identity references.
-   Employment context required for workflow execution.
-   Employee lifecycle updates after approved HR decisions.

E04 SHALL NOT directly modify employee profile records.

Integration Pattern:

``` mermaid
sequenceDiagram

participant HR as E04 HR Workflow
participant EP as E07 Employee Profile

HR->>HR: Approve HR Process

HR->>EP: Publish Employee Update Event

EP->>EP: Update Employee Profile

EP-->>HR: Confirmation Event
```

------------------------------------------------------------------------

## Recruitment Domain

### Dependency Purpose

Recruitment provides approved hiring outcomes that initiate employee
onboarding workflows.

Recruitment owns:

-   Candidate lifecycle.
-   Selection process.
-   Hiring decision.

E04 consumes:

-   Hiring approved events.
-   Candidate conversion information.
-   Joining initiation information.

Example:

``` text
HiringApproved Event

        ↓

Employee Onboarding Workflow

        ↓

Employee Activation
```

------------------------------------------------------------------------

## Organization Domain

### Dependency Purpose

Organization provides organizational structures required for HR
workflows.

Organization owns:

-   Departments.
-   Divisions.
-   Reporting hierarchy.
-   Organizational assignments.

E04 consumes:

-   Department references.
-   Approval hierarchy information.
-   Organizational relationships.

E04 SHALL NOT create or modify organizational structures.

------------------------------------------------------------------------

## Identity Domain

### Dependency Purpose

Identity provides authentication and access management capabilities.

Identity owns:

-   User accounts.
-   Authentication.
-   Access provisioning.

E04 consumes:

-   Access provisioning requirements.
-   Identity lifecycle events.

Example:

``` text
Employee Onboarding Completed

        ↓

Access Provisioning Request

        ↓

Identity Domain
```

------------------------------------------------------------------------

## Policy Configuration Capability

### Dependency Purpose

Policy capabilities provide configurable rules required for HR workflow
execution.

Examples:

-   Leave policies.
-   Compliance rules.
-   Institutional HR policies.

E04 executes policy decisions but SHALL NOT hardcode institutional
rules.

------------------------------------------------------------------------

# 3.3 Secondary Upstream Dependencies

## Attendance Domain

Provides:

-   Attendance information.
-   Absence information.
-   Attendance conflict validation.

E04 uses attendance information for:

-   Leave validation.
-   Absence workflows.

Attendance remains the source of attendance truth.

------------------------------------------------------------------------

## Payroll Domain

Provides:

-   Payroll status information.
-   Compensation-related workflow dependencies.

E04 does not calculate payroll.

Example:

``` text
Leave Approved

        ↓

Leave Event

        ↓

Payroll Processing
```

------------------------------------------------------------------------

## Document Platform

Provides:

-   Document storage.
-   Document versioning.
-   Document retrieval.
-   Document generation infrastructure.

E04 provides:

-   Document business context.
-   Document workflow decisions.

------------------------------------------------------------------------

# 3.4 Upstream Dependency Summary

  Dependency             Purpose                       Ownership
  ---------------------- ----------------------------- -------------------
  Employee Profile       Employee profile information  External
  Recruitment            Hiring decisions              External
  Organization           Structure and hierarchy       External
  Identity               Authentication and access     External
  Policy Configuration   Rules and policies            Shared Capability
  Attendance             Attendance validation         External
  Payroll                Financial processing          External
  Document Platform      Document infrastructure       External

------------------------------------------------------------------------

# 4. Downstream Dependencies

## 4.1 Dependency Philosophy

Downstream domains consume HR workflow outcomes through controlled
contracts.

E04 SHALL communicate using:

-   Domain events.
-   Service contracts.
-   Approved integration interfaces.

E04 SHALL NOT expose internal workflow storage.

------------------------------------------------------------------------

# 4.2 Primary Downstream Consumers

## E07 --- Employee Profile Domain

Consumes:

-   Employee lifecycle decisions.
-   Approved employment changes.
-   Status transitions.

Example events:

``` text
EmployeeActivated

EmployeeTransferred

EmployeeSeparated
```

------------------------------------------------------------------------

## Payroll Domain

Consumes:

-   Employment changes.
-   Leave decisions.
-   Separation completion.

Examples:

``` text
CompensationChangeApproved

LeaveApproved

EmployeeSeparated
```

------------------------------------------------------------------------

## Attendance Domain

Consumes:

-   Approved leave outcomes.
-   Absence decisions.

Examples:

``` text
LeaveApproved

LeaveCancelled
```

------------------------------------------------------------------------

## Notification Service

Consumes:

-   HR workflow events requiring communication.

Examples:

``` text
LeaveApproved

CertificateGenerated

ApprovalPending
```

Notification Service owns delivery.

------------------------------------------------------------------------

## Analytics and Reporting Domain

Consumes:

-   HR business events.
-   Workflow metrics.
-   Operational statistics.

E04 SHALL NOT maintain analytics databases.

------------------------------------------------------------------------

# 4.3 Downstream Communication Model

``` mermaid
flowchart LR

HR["L2-E04 HR Workflows"]

Events["HR Domain Events"]

Employee["Employee Profile"]

Payroll["Payroll"]

Attendance["Attendance"]

Notify["Notification Service"]

Analytics["Analytics"]


HR --> Events

Events --> Employee

Events --> Payroll

Events --> Attendance

Events --> Notify

Events --> Analytics
```

------------------------------------------------------------------------

# 4.4 Downstream Dependency Summary

  Consumer               Receives
  ---------------------- -------------------------------
  Employee Profile       Lifecycle updates
  Payroll                Financial-impacting HR events
  Attendance             Leave and absence outcomes
  Notification Service   Communication triggers
  Analytics              HR operational events

------------------------------------------------------------------------

# 5. Domain Ownership Boundaries

## 5.1 Ownership Principle

L2-E04 SHALL follow:

``` text
HR owns process execution.

Specialized domains own specialized data.
```

The domain boundary exists to prevent:

-   Duplicate employee records.
-   Conflicting lifecycle states.
-   Shared database ownership.
-   Multi-domain coupling.

------------------------------------------------------------------------

# 5.2 E04 Ownership

L2-E04 owns:

## Workflow Ownership

-   HR processes.
-   Approvals.
-   State transitions.
-   SLA management.
-   Escalations.

## HR Operational Cases

-   Employee requests.
-   HR cases.
-   Grievances.
-   Compliance cases.

## Lifecycle Processes

-   Onboarding.
-   Probation.
-   Confirmation.
-   Promotion workflow.
-   Transfer workflow.
-   Separation workflow.

## HR Documents

-   Certificate workflows.
-   Verification workflows.
-   Document requests.

------------------------------------------------------------------------

# 5.3 Non-Owned Capabilities

  Capability                      Owner
  ------------------------------- ----------------------
  Employee profile data            E07 Employee Profile
  Organization hierarchy          Organization Domain
  Authentication                  Identity Domain
  Salary calculation              Payroll Domain
  Attendance capture              Attendance Domain
  File storage                    Document Platform
  Analytics processing            Analytics Domain
  Legal contract interpretation   Legal Domain

------------------------------------------------------------------------

# 5.4 Ownership Boundary Diagram

``` mermaid
flowchart TB

HR["E04 HR Workflows"]

E07["Employee Profile"]

ORG["Organization"]

PAY["Payroll"]

ATT["Attendance"]

DOC["Document Platform"]

ID["Identity"]

ANA["Analytics"]


HR -->|Workflow Decisions| E07

HR -->|Assignment Requests| ORG

HR -->|Financial Events| PAY

HR -->|Leave Events| ATT

HR -->|Document Requests| DOC

HR -->|Access Requests| ID

HR -->|Business Events| ANA
```

------------------------------------------------------------------------

# 5.5 Boundary Enforcement Rules

The following rules SHALL apply:

1.  E04 SHALL never directly modify external domain databases.
2.  E04 SHALL communicate through contracts and events.
3.  External domains SHALL remain authoritative for owned data.
4.  HR workflows SHALL maintain business process history.
5.  Employee profile state SHALL remain outside E04 ownership.
6.  Document storage SHALL remain outside E04 ownership.

------------------------------------------------------------------------

# 6. Domain Model / Aggregate / Lifecycle

## 6.1 Domain Model Overview

L2-E04 --- HR Workflows is modeled as a workflow-oriented bounded
context responsible for coordinating HR business processes.

The domain model is intentionally designed around **business process
aggregates** rather than employee data ownership.

The core principle:

``` text
Employee Data Ownership
        |
        ↓
E07 Employee Profile

HR Process Ownership
        |
        ↓
E04 HR Workflows
```

E04 manages:

-   HR process state.
-   Workflow execution.
-   Approval decisions.
-   Business rules.
-   Process history.
-   Domain events.

E04 does not manage:

-   Employee profile identity.
-   Organizational structure.
-   Payroll records.
-   Attendance records.

------------------------------------------------------------------------

# 6.2 HR Workflow Domain Model

``` mermaid
classDiagram

class HRCaseAggregate {
+caseId
+caseType
+status
+priority
+createdDate
+resolve()
}

class EmployeeOnboardingAggregate {
+onboardingId
+employeeReference
+status
+initiate()
+complete()
}

class LeaveRequestAggregate {
+leaveRequestId
+employeeReference
+leaveType
+status
+approve()
+reject()
}

class CertificateRequestAggregate {
+certificateRequestId
+certificateType
+status
+generate()
}

class EmploymentChangeAggregate {
+changeId
+changeType
+approvalStatus
+execute()
}

class SeparationAggregate {
+separationId
+exitType
+status
+completeExit()
}

class ComplianceCaseAggregate {
+complianceId
+requirementType
+status
+verify()
}


HRCaseAggregate --> ComplianceCaseAggregate
EmployeeOnboardingAggregate --> EmploymentChangeAggregate
LeaveRequestAggregate --> HRCaseAggregate
CertificateRequestAggregate --> HRCaseAggregate
SeparationAggregate --> HRCaseAggregate
```

------------------------------------------------------------------------

# 6.3 Aggregate Design Principles

## Principle 1 --- Workflow Ownership

Each aggregate represents a business process boundary.

Example:

A leave request is not only a record.

It contains:

-   Request intent.
-   Approval state.
-   Policy validation.
-   Decision history.
-   Business outcome.

------------------------------------------------------------------------

## Principle 2 --- No Shared Ownership

E04 aggregates SHALL reference external entities through identifiers.

Example:

Correct:

``` text
LeaveRequest

employeeId = EMP-12345
```

Incorrect:

``` text
LeaveRequest

Employee {
    Name
    Address
    Department
    Salary
}
```

------------------------------------------------------------------------

## Principle 3 --- Event-Based Synchronization

Aggregate completion produces domain events.

Example:

``` text
PromotionApproved

        ↓

Employee Profile updates role information

Payroll updates compensation

Analytics updates reporting data
```

------------------------------------------------------------------------

# 6.4 Core Aggregates

## 6.4.1 HR Case Aggregate

### Responsibility

Represents a generic HR operational case.

Examples:

-   Employee request.
-   HR grievance.
-   Compliance issue.
-   Administrative request.

### Owns

-   Case lifecycle.
-   Priority.
-   SLA.
-   Assignment.
-   Resolution.

### Does Not Own

-   Employee profile data.
-   Documents.
-   Notification delivery.

------------------------------------------------------------------------

## 6.4.2 Employee Onboarding Aggregate

### Responsibility

Controls employee joining workflow after approved hiring.

Lifecycle:

``` text
Hiring Approved

↓

Onboarding Initiated

↓

Documentation Pending

↓

Verification Pending

↓

Ready For Joining

↓

Completed
```

------------------------------------------------------------------------

## 6.4.3 Leave Request Aggregate

### Responsibility

Controls leave business process execution.

Owns:

-   Leave request.
-   Approval state.
-   Policy evaluation result.
-   Cancellation workflow.

Does not own:

-   Attendance records.
-   Payroll deductions.

------------------------------------------------------------------------

## 6.4.4 Certificate Request Aggregate

### Responsibility

Controls HR certificate lifecycle.

Examples:

-   Experience certificate.
-   Employment certificate.
-   Relieving letter.
-   NOC.

Owns:

-   Request.
-   Approval.
-   Generation trigger.
-   Delivery status.

Does not own:

-   File storage.

------------------------------------------------------------------------

## 6.4.5 Employment Change Aggregate

### Responsibility

Controls employee-related change workflows.

Examples:

-   Promotion.
-   Transfer.
-   Designation change.
-   Contract renewal.

Owns:

-   Change request.
-   Approval process.
-   Effective date.
-   Business decision.

------------------------------------------------------------------------

## 6.4.6 Separation Aggregate

### Responsibility

Controls employee exit workflows.

Owns:

-   Resignation workflow.
-   Exit clearance.
-   Completion state.
-   Exit events.

Does not own:

-   Final settlement calculation.

------------------------------------------------------------------------

## 6.4.7 Compliance Case Aggregate

### Responsibility

Controls HR compliance execution.

Examples:

-   Missing documents.
-   Policy acknowledgement.
-   Mandatory verification.

------------------------------------------------------------------------

# 6.5 Entity Model

## HR Case Entity

  Attribute   Description
  ----------- -------------------
  Case ID     Unique identifier
  Case Type   Business category
  Status      Lifecycle state
  Priority    Business urgency
  Assignee    Responsible actor
  Timeline    Business history

------------------------------------------------------------------------

## Leave Request Entity

  Attribute            Description
  -------------------- ----------------------------
  Leave Request ID     Unique identifier
  Employee Reference   External employee identity
  Leave Type           Policy category
  Requested Period     Duration
  Approval State       Workflow state

------------------------------------------------------------------------

## Certificate Request Entity

  Attribute           Description
  ------------------- --------------------
  Request ID          Unique identifier
  Certificate Type    Requested document
  Approval Status     Processing state
  Generation Status   Document state

------------------------------------------------------------------------

# 6.6 Value Objects

## Employee Reference

Represents a reference to an employee owned by E07.

Example:

``` text
EmployeeReference {
    employeeId
}
```

------------------------------------------------------------------------

## Workflow Status

Represents process state.

Examples:

``` text
INITIATED
PENDING_APPROVAL
APPROVED
REJECTED
COMPLETED
CANCELLED
```

------------------------------------------------------------------------

## Approval Decision

Represents an approval outcome.

Contains:

-   Decision.
-   Actor.
-   Timestamp.
-   Reason.

------------------------------------------------------------------------

## Document Reference

Represents a document managed by Document Platform.

Contains:

-   Document ID.
-   Version.
-   Type.

------------------------------------------------------------------------

# 6.7 Lifecycle Architecture

``` mermaid
stateDiagram-v2

[*] --> Initiated

Initiated --> PendingApproval

PendingApproval --> Approved

PendingApproval --> Rejected

Approved --> Processing

Processing --> Completed

Processing --> Failed

Failed --> Processing

Completed --> Archived

Rejected --> Closed

Archived --> [*]
```

------------------------------------------------------------------------

# 6.8 Employee Lifecycle Workflow Model

``` mermaid
stateDiagram-v2

[*] --> HiringApproved

HiringApproved --> OnboardingInitiated

OnboardingInitiated --> DocumentationPending

DocumentationPending --> VerificationPending

VerificationPending --> ReadyForJoining

ReadyForJoining --> ActiveEmployment

ActiveEmployment --> Probation

Probation --> Confirmed

Probation --> Extended

Confirmed --> EmploymentChange

EmploymentChange --> ActiveEmployment

ActiveEmployment --> SeparationInitiated

SeparationInitiated --> ExitClearance

ExitClearance --> SeparationCompleted

SeparationCompleted --> Archived
```

------------------------------------------------------------------------

# 6.9 Aggregate Interaction Model

``` mermaid
flowchart LR

Onboarding["Employee Onboarding"]

Employment["Employment Change"]

Leave["Leave Request"]

Certificate["Certificate Request"]

Separation["Separation"]

Case["HR Case"]

Events["Domain Events"]


Onboarding --> Events

Employment --> Events

Leave --> Events

Certificate --> Events

Separation --> Events

Case --> Events
```

------------------------------------------------------------------------

# 6.10 Lifecycle Ownership Rules

  Lifecycle Element            Owner
  ---------------------------- ----------------------
  HR workflow state            E04 HR Workflows
  Employee identity state      E07 Employee Profile
  Organization assignment      Organization Domain
  Payroll impact               Payroll Domain
  Attendance impact            Attendance Domain
  Document storage lifecycle   Document Platform

------------------------------------------------------------------------

# 6.11 Domain Model Summary

The E04 domain model follows:

``` text
Multiple Workflow Aggregates

        +

External Entity References

        +

Event Driven Integration

        +

Strict Ownership Boundaries
```

This ensures HR workflows remain scalable, configurable, and independent
from other enterprise domains.

# 7. Domain Workflows

## 7.1 Workflow Architecture Overview

L2-E04 HR Workflows is organized around employee lifecycle and HR
operational workflows.

The workflow architecture follows:

``` mermaid
flowchart LR

Trigger["Business Trigger"]

Workflow["E04 HR Workflow Engine"]

Approval["Approval Process"]

Decision["Business Decision"]

Events["Domain Events"]

Consumers["External Domains"]


Trigger --> Workflow
Workflow --> Approval
Approval --> Decision
Decision --> Events
Events --> Consumers
```

The domain SHALL manage:

-   Workflow initiation.
-   Workflow state transitions.
-   Approval orchestration.
-   SLA tracking.
-   Escalation handling.
-   Business decision recording.
-   Event publication.

------------------------------------------------------------------------

# 7.2 Workflow Catalogue

  -----------------------------------------------------------------------
  Workflow                Purpose                 Aggregate
  ----------------------- ----------------------- -----------------------
  Employee Onboarding     Execute joining         Employee Onboarding
                          lifecycle               Aggregate

  Probation Management    Manage probation        Employment Change
                          evaluation              Aggregate

  Confirmation Workflow   Confirm employment      Employment Change
                          status                  Aggregate

  Employee Request        Handle HR requests      HR Case Aggregate
  Workflow                                        

  Leave Workflow          Manage leave approvals  Leave Request Aggregate

  Certificate Workflow    Generate HR             Certificate Request
                          certificates            Aggregate

  Promotion Workflow      Manage promotion        Employment Change
                          decisions               Aggregate

  Transfer Workflow       Manage employee         Employment Change
                          movement                Aggregate

  Contract Renewal        Extend employment       Employment Change
  Workflow                contracts               Aggregate

  Separation Workflow     Execute exit process    Separation Aggregate

  Compliance Workflow     Manage HR compliance    Compliance Case
                          activities              Aggregate

  Grievance Workflow      Handle employee         HR Case Aggregate
                          concerns                
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 7.3 Employee Onboarding Workflow

## Purpose

The onboarding workflow manages employee activation after approved
hiring.

## Trigger

``` text
HiringApproved Event
```

## Actors

-   HR Administrator
-   Hiring Manager
-   Employee
-   Verification Team

## Workflow

``` mermaid
flowchart TD

A["Hiring Approved"]

B["Create Onboarding Case"]

C["Collect Employee Documents"]

D["Verify Information"]

E["Create Employment Record"]

F["Request System Access"]

G["Complete Onboarding"]

H["Publish EmployeeOnboardingCompleted"]


A --> B
B --> C
C --> D
D --> E
E --> F
F --> G
G --> H
```

------------------------------------------------------------------------

# 7.4 Leave Workflow

## Purpose

Manage employee leave requests and approval processes.

## Trigger

``` text
Employee Leave Request Submitted
```

## Workflow

``` mermaid
stateDiagram-v2

[*] --> Submitted

Submitted --> PolicyValidation

PolicyValidation --> PendingApproval

PendingApproval --> Approved

PendingApproval --> Rejected

Approved --> LeaveRecorded

LeaveRecorded --> Completed

Rejected --> Closed
```

------------------------------------------------------------------------

# 7.5 HR Case Workflow

## Purpose

Manage employee-related HR operational requests.

Examples:

-   HR queries.
-   Grievances.
-   Administrative requests.
-   Compliance issues.

## Workflow

``` mermaid
flowchart TD

Request["Request Created"]

Classification["Case Classification"]

Assignment["Assign HR Owner"]

Investigation["Investigation"]

Resolution["Resolution Decision"]

Closure["Case Closed"]


Request --> Classification

Classification --> Assignment

Assignment --> Investigation

Investigation --> Resolution

Resolution --> Closure
```

------------------------------------------------------------------------

# 7.6 Promotion Workflow

## Purpose

Manage employee promotion decisions.

## Workflow

``` mermaid
flowchart LR

Request["Promotion Request"]

Review["Eligibility Review"]

Approval["Approval Chain"]

Decision["Promotion Decision"]

Event["PromotionApproved Event"]


Request --> Review

Review --> Approval

Approval --> Decision

Decision --> Event
```

------------------------------------------------------------------------

# 7.7 Transfer Workflow

## Purpose

Manage employee movement between organizational assignments.

## Workflow

``` mermaid
sequenceDiagram

participant Employee
participant Manager
participant HR
participant Organization

Employee->>Manager: Transfer Request

Manager->>HR: Submit Recommendation

HR->>HR: Validate Policy

HR->>Organization: Update Assignment Request

Organization-->>HR: Assignment Confirmation

HR->>Employee: Transfer Completed
```

------------------------------------------------------------------------

# 7.8 Certificate Workflow

## Purpose

Manage official HR certificate generation.

Examples:

-   Experience Certificate.
-   Employment Certificate.
-   Relieving Certificate.
-   NOC.

## Workflow

``` mermaid
flowchart TD

Request["Certificate Request"]

Validation["Validate Eligibility"]

Approval["HR Approval"]

Generation["Document Generation Request"]

Storage["Document Platform Storage"]

Delivery["Certificate Available"]


Request --> Validation

Validation --> Approval

Approval --> Generation

Generation --> Storage

Storage --> Delivery
```

------------------------------------------------------------------------

# 7.9 Separation Workflow

## Purpose

Manage employee exit lifecycle.

## Workflow

``` mermaid
stateDiagram-v2

[*] --> ResignationSubmitted

ResignationSubmitted --> ExitApproval

ExitApproval --> ClearanceProcess

ClearanceProcess --> FinalVerification

FinalVerification --> SeparationCompleted

SeparationCompleted --> EmployeeSeparatedEvent

EmployeeSeparatedEvent --> Archived
```

------------------------------------------------------------------------

# 7.10 Compliance Workflow

## Purpose

Manage HR compliance activities.

## Workflow

``` mermaid
flowchart TD

Requirement["Compliance Requirement"]

Case["Compliance Case Created"]

Evidence["Evidence Collection"]

Verification["Verification"]

Completed["Compliance Completed"]


Requirement --> Case

Case --> Evidence

Evidence --> Verification

Verification --> Completed
```

------------------------------------------------------------------------

# 7.11 Workflow Execution Rules

The following rules SHALL apply:

1.  Every workflow SHALL have a defined lifecycle.
2.  Every approval SHALL record actor, timestamp, and decision.
3.  Every completed business workflow SHALL publish relevant events.
4.  Workflow history SHALL be immutable.
5.  External domain updates SHALL occur through contracts/events.
6.  Manual intervention SHALL be supported for exceptional cases.

------------------------------------------------------------------------

# 7.12 Workflow Ownership Matrix

  Workflow                 Owner
  ------------------------ ----------------------
  Onboarding               E04 HR Workflows
  Leave Approval           E04 HR Workflows
  HR Cases                 E04 HR Workflows
  Certificates             E04 HR Workflows
  Promotion Process        E04 HR Workflows
  Transfer Process         E04 HR Workflows
  Separation Process       E04 HR Workflows
  Payroll Calculation      Payroll Domain
  Employee Profile Update   E07 Employee Profile
  Organization Update      Organization Domain

------------------------------------------------------------------------

# 7.13 Domain Workflow Summary

The HR Workflow domain provides a controlled execution layer for
employee lifecycle operations.

Architecture principle:

``` text
Business Process Ownership

        +

Domain Event Integration

        +

Strict Data Ownership Boundaries

        =

Scalable HR Workflow Architecture
```
# 8. Workflow Specifications

## 8.1 Workflow Specification Model

Each HR workflow SHALL be documented using a standard specification
model.

Every workflow specification contains:

  Attribute          Description
  ------------------ --------------------------------------
  Workflow ID        Unique workflow identifier
  Workflow Name      Business workflow name
  Purpose            Business objective
  Trigger            Workflow initiation event
  Actors             Participants involved
  Preconditions      Conditions required before execution
  Inputs             Required information
  Processing Steps   Workflow execution sequence
  States             Lifecycle states
  Outputs            Result of workflow completion
  Domain Events      Events generated
  Failure Handling   Exception management

------------------------------------------------------------------------

# 8.2 HR-WF-001 --- Employee Onboarding Workflow

## Purpose

Execute employee joining lifecycle after approved hiring.

## Trigger

``` text
HiringApproved Event
```

## Actors

  Actor               Responsibility
  ------------------- ------------------------
  HR Administrator    Workflow owner
  Hiring Manager      Joining coordination
  Employee            Information submission
  Verification Team   Document verification

------------------------------------------------------------------------

## Preconditions

The following conditions SHALL be satisfied:

-   Hiring decision approved.
-   Employee reference available.
-   Position assignment available.
-   Required onboarding policy exists.

------------------------------------------------------------------------

## Workflow Steps

``` mermaid
flowchart TD

A["Hiring Approved"]

B["Create Onboarding Workflow"]

C["Collect Employee Information"]

D["Verify Documents"]

E["Complete HR Validation"]

F["Request Identity Provisioning"]

G["Activate Employee"]

H["Publish EmployeeOnboardingCompleted"]


A --> B
B --> C
C --> D
D --> E
E --> F
F --> G
G --> H
```

------------------------------------------------------------------------

## States

``` text
INITIATED

PENDING_INFORMATION

PENDING_VERIFICATION

PENDING_APPROVAL

COMPLETED

FAILED
```

------------------------------------------------------------------------

## Outputs

-   Completed onboarding workflow.
-   Employee activation request.
-   Audit history.
-   Domain events.

------------------------------------------------------------------------

# 8.3 HR-WF-002 --- Leave Request Workflow

## Purpose

Manage employee leave requests through policy validation and approvals.

## Trigger

``` text
Leave Request Submitted
```

## Actors

  Actor      Responsibility
  ---------- ------------------
  Employee   Submit request
  Manager    First approval
  HR         Policy oversight

------------------------------------------------------------------------

## Preconditions

-   Employee exists.
-   Leave policy exists.
-   Requested dates are valid.
-   Required approvals are configured.

------------------------------------------------------------------------

## Workflow

``` mermaid
stateDiagram-v2

[*] --> Submitted

Submitted --> Validation

Validation --> PendingApproval

PendingApproval --> Approved

PendingApproval --> Rejected

Approved --> LeaveRecorded

LeaveRecorded --> Completed

Rejected --> Closed
```

------------------------------------------------------------------------

## Outputs

-   Leave decision.
-   Leave event publication.
-   Employee notification trigger.

------------------------------------------------------------------------

# 8.4 HR-WF-003 --- HR Case Workflow

## Purpose

Manage HR operational cases.

Examples:

-   Employee requests.
-   Grievances.
-   Administrative cases.

------------------------------------------------------------------------

## Trigger

``` text
HR Case Created
```

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

Create["Case Created"]

Classify["Classify Case"]

Assign["Assign HR Owner"]

Process["Process Case"]

Decision["Resolution Decision"]

Close["Close Case"]


Create --> Classify

Classify --> Assign

Assign --> Process

Process --> Decision

Decision --> Close
```

------------------------------------------------------------------------

## Outputs

-   Case resolution.
-   Case history.
-   Audit record.

------------------------------------------------------------------------

# 8.5 HR-WF-004 --- Promotion Workflow

## Purpose

Manage employee promotion decisions.

## Trigger

``` text
Promotion Request Submitted
```

------------------------------------------------------------------------

## Workflow

``` mermaid
sequenceDiagram

participant Manager
participant HR
participant Organization
participant Payroll

Manager->>HR: Submit Promotion Recommendation

HR->>HR: Validate Eligibility

HR->>Organization: Validate Position Structure

HR->>Payroll: Request Compensation Impact

HR->>HR: Complete Approval

HR-->>Manager: Promotion Decision
```

------------------------------------------------------------------------

## Outputs

-   Promotion decision.
-   Employee update event.
-   Compensation update request.

------------------------------------------------------------------------

# 8.6 HR-WF-005 --- Transfer Workflow

## Purpose

Manage employee organizational movement.

## Trigger

``` text
Transfer Request Submitted
```

------------------------------------------------------------------------

## Workflow Steps

``` mermaid
flowchart TD

Request["Transfer Request"]

Validation["Validate Transfer Rules"]

Approval["Approval Workflow"]

Assignment["Update Organizational Assignment"]

Event["EmployeeTransferred Event"]


Request --> Validation

Validation --> Approval

Approval --> Assignment

Assignment --> Event
```

------------------------------------------------------------------------

# 8.7 HR-WF-006 --- Certificate Generation Workflow

## Purpose

Generate official HR certificates.

Supported certificates:

-   Experience Certificate.
-   Employment Certificate.
-   Relieving Certificate.
-   NOC.

------------------------------------------------------------------------

## Trigger

``` text
Certificate Request Submitted
```

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart LR

Request["Certificate Request"]

Validation["Eligibility Validation"]

Approval["HR Approval"]

Generate["Generate Document"]

Sign["Digital Signature"]

Deliver["Employee Delivery"]


Request --> Validation

Validation --> Approval

Approval --> Generate

Generate --> Sign

Sign --> Deliver
```

------------------------------------------------------------------------

# 8.8 HR-WF-007 --- Contract Renewal Workflow

## Purpose

Manage employment contract renewal process.

------------------------------------------------------------------------

## Workflow

``` mermaid
stateDiagram-v2

[*] --> RenewalInitiated

RenewalInitiated --> Review

Review --> Approval

Approval --> ContractUpdated

ContractUpdated --> Completed

Approval --> Rejected

Rejected --> Closed
```

------------------------------------------------------------------------

# 8.9 HR-WF-008 --- Separation Workflow

## Purpose

Execute employee exit lifecycle.

## Trigger

``` text
Resignation Submitted
```

------------------------------------------------------------------------

## Workflow

``` mermaid
flowchart TD

Resignation["Resignation Request"]

Approval["Exit Approval"]

Clearance["Exit Clearance"]

Verification["Final Verification"]

Complete["Separation Completed"]

Event["EmployeeSeparated Event"]


Resignation --> Approval

Approval --> Clearance

Clearance --> Verification

Verification --> Complete

Complete --> Event
```

------------------------------------------------------------------------

# 8.10 HR-WF-009 --- Compliance Workflow

## Purpose

Manage HR compliance activities.

Examples:

-   Mandatory documents.
-   Policy acknowledgements.
-   Verification requirements.

------------------------------------------------------------------------

## Workflow

``` mermaid
stateDiagram-v2

[*] --> RequirementCreated

RequirementCreated --> EvidenceCollection

EvidenceCollection --> Verification

Verification --> Completed

Verification --> Failed

Failed --> Remediation

Remediation --> Verification
```

------------------------------------------------------------------------

# 8.11 Workflow Specification Rules

The following rules SHALL apply to all HR workflows:

1.  Every workflow SHALL have a unique identifier.
2.  Every workflow SHALL maintain lifecycle history.
3.  Every approval SHALL capture actor, timestamp, and decision.
4.  Every completed workflow SHALL publish required domain events.
5.  Every failure SHALL create traceable failure information.
6.  Every workflow SHALL support authorization validation.
7.  Workflow definitions SHALL support institutional configuration.

------------------------------------------------------------------------

# 8.12 Workflow Specification Summary

  -----------------------------------------------------------------------------
  Workflow                Aggregate               Primary Event
  ----------------------- ----------------------- -----------------------------
  Employee Onboarding     Employee Onboarding     EmployeeOnboardingCompleted
                          Aggregate               

  Leave Request           Leave Request Aggregate LeaveApproved

  HR Case                 HR Case Aggregate       HRCaseResolved

  Promotion               Employment Change       PromotionApproved
                          Aggregate               

  Transfer                Employment Change       EmployeeTransferred
                          Aggregate               

  Certificate             Certificate Request     CertificateGenerated
                          Aggregate               

  Contract Renewal        Employment Change       ContractRenewed
                          Aggregate               

  Separation              Separation Aggregate    EmployeeSeparated

  Compliance              Compliance Case         ComplianceCompleted
                          Aggregate               
  -----------------------------------------------------------------------------


# 9. Business Rules

## 9.1 Business Rule Philosophy

L2-E04 HR Workflows SHALL enforce business rules required to maintain
consistent, compliant, and auditable HR operations.

Business rules define:

-   Allowed workflow transitions.
-   Approval requirements.
-   Data validation rules.
-   Ownership boundaries.
-   Compliance controls.
-   Institutional configuration rules.

The principle:

``` text
Workflow Execution

+

Business Rules

+

Authorization

+

Audit

=

Controlled HR Operations
```

------------------------------------------------------------------------

# 9.2 General HR Workflow Rules

## BR-HR-001 --- Workflow Ownership Rule

Every HR workflow SHALL have exactly one owning bounded context.

E04 owns:

-   Workflow execution.
-   Approval process.
-   Decision history.
-   Business events.

E04 does not own:

-   External domain master data.
-   External domain calculations.

------------------------------------------------------------------------

## BR-HR-002 --- State Transition Rule

A workflow SHALL move only through valid lifecycle states.

Invalid transitions SHALL be rejected.

Example:

``` text
Completed

X

Pending Approval
```

A completed workflow cannot return to an earlier approval state.

------------------------------------------------------------------------

## BR-HR-003 --- Approval Rule

Every approval action SHALL record:

-   Approver identity.
-   Decision.
-   Timestamp.
-   Reason.
-   Approval context.

------------------------------------------------------------------------

## BR-HR-004 --- Audit Rule

Every significant HR business action SHALL create an immutable audit
record.

Examples:

-   Approval.
-   Rejection.
-   Cancellation.
-   Status change.
-   Certificate generation.
-   Separation completion.

------------------------------------------------------------------------

# 9.3 Employee Lifecycle Rules

## BR-HR-005 --- Onboarding Completion Rule

Employee onboarding SHALL NOT complete until:

-   Required information is collected.
-   Required verification is completed.
-   Required approvals are completed.

------------------------------------------------------------------------

## BR-HR-006 --- Employee Activation Rule

Employee activation SHALL update employee lifecycle status through E07
Employee Profile.

E04 SHALL publish an activation event.

------------------------------------------------------------------------

## BR-HR-007 --- Separation Completion Rule

Employee separation SHALL NOT complete until:

-   Exit approvals are completed.
-   Required clearance workflows are completed.
-   Final workflow validation succeeds.

------------------------------------------------------------------------

# 9.4 Leave Workflow Rules

## BR-HR-008 --- Leave Eligibility Rule

Leave approval SHALL validate:

-   Employee eligibility.
-   Leave policy applicability.
-   Requested duration.
-   Institutional rules.

------------------------------------------------------------------------

## BR-HR-009 --- Leave Approval Rule

Leave approval workflow SHALL follow configured approval hierarchy.

Example:

``` mermaid
flowchart TD

Employee["Employee"]

Manager["Manager Approval"]

HR["HR Review"]

Decision["Leave Decision"]


Employee --> Manager
Manager --> HR
HR --> Decision
```

------------------------------------------------------------------------

# 9.5 Employment Change Rules

## BR-HR-010 --- Promotion Rule

Promotion approval SHALL validate:

-   Eligibility criteria.
-   Required approvals.
-   Effective date.
-   Organizational constraints.

Promotion SHALL publish:

``` text
PromotionApproved
```

------------------------------------------------------------------------

## BR-HR-011 --- Transfer Rule

Transfer workflow SHALL NOT directly modify organizational structure.

The Organization Domain SHALL execute assignment changes.

------------------------------------------------------------------------

## BR-HR-012 --- Contract Renewal Rule

Contract renewal SHALL validate:

-   Contract validity.
-   Approval authority.
-   Effective dates.
-   Institutional policy.

------------------------------------------------------------------------

# 9.6 Certificate Rules

## BR-HR-013 --- Certificate Generation Rule

Official certificates SHALL only be generated after:

-   Eligibility validation.
-   Approval completion.
-   Template availability.

------------------------------------------------------------------------

## BR-HR-014 --- Document Ownership Rule

E04 owns certificate workflow.

Document Platform owns:

-   File storage.
-   Versioning.
-   Retrieval.

------------------------------------------------------------------------

# 9.7 Multi-Institution Rules

## BR-HR-015 --- Configuration Rule

Institutions SHALL customize:

-   Approval chains.
-   Policies.
-   Templates.
-   SLA definitions.

Institutions SHALL NOT modify core domain ownership rules.

------------------------------------------------------------------------

## BR-HR-016 --- Policy Override Rule

Institution-specific overrides SHALL follow governance rules.

------------------------------------------------------------------------

# 9.8 Compliance Rules

## BR-HR-017 --- Compliance Tracking Rule

Compliance workflows SHALL maintain:

-   Requirement status.
-   Evidence status.
-   Verification history.
-   Resolution history.

------------------------------------------------------------------------

# 9.9 Domain Events

# 10. Domain Events

## 10.1 Event Philosophy

L2-E04 uses domain events to communicate completed business decisions.

Events represent:

-   Something meaningful happened.
-   A business state changed.
-   Other domains may react.

Events SHALL NOT represent:

-   Database operations.
-   UI actions.
-   Temporary processing states.

------------------------------------------------------------------------

# 10.2 Event Publishing Model

``` mermaid
flowchart LR

Aggregate["E04 Aggregate"]

Event["Domain Event"]

Bus["Event Bus"]

Consumers["External Consumers"]


Aggregate --> Event

Event --> Bus

Bus --> Consumers
```

------------------------------------------------------------------------

# 10.3 Employee Lifecycle Events

  Event                         Description
  ----------------------------- -------------------------------
  EmployeeOnboardingStarted     Onboarding workflow initiated
  EmployeeOnboardingCompleted   Employee onboarding completed
  EmployeeActivated             Employee became active
  EmployeeProbationStarted      Probation started
  EmployeeConfirmed             Employment confirmed
  EmployeeSeparated             Separation completed

------------------------------------------------------------------------

# 10.4 Leave Events

  Event            Description
  ---------------- -------------------------
  LeaveRequested   Leave request submitted
  LeaveApproved    Leave approved
  LeaveRejected    Leave rejected
  LeaveCancelled   Leave cancelled

------------------------------------------------------------------------

# 10.5 Employment Change Events

  Event                 Description
  --------------------- ------------------------------
  PromotionRequested    Promotion workflow initiated
  PromotionApproved     Promotion approved
  EmployeeTransferred   Transfer completed
  ContractRenewed       Contract renewal completed

------------------------------------------------------------------------

# 10.6 Certificate Events

  Event                  Description
  ---------------------- -----------------------------
  CertificateRequested   Certificate request created
  CertificateApproved    Certificate approved
  CertificateGenerated   Certificate generated
  CertificateDelivered   Certificate delivered

------------------------------------------------------------------------

# 10.7 HR Case Events

  Event            Description
  ---------------- -----------------
  HRCaseCreated    HR case created
  HRCaseAssigned   Case assigned
  HRCaseResolved   Case resolved
  HRCaseClosed     Case closed

------------------------------------------------------------------------

# 10.8 Compliance Events

  Event                             Description
  --------------------------------- ------------------------------
  ComplianceCaseCreated             Compliance case initiated
  ComplianceVerificationCompleted   Verification completed
  ComplianceCompleted               Compliance process completed

------------------------------------------------------------------------

# 10.9 Event Ownership Rules

The following rules SHALL apply:

1.  E04 publishes only events related to HR-owned business processes.
2.  Events SHALL contain business meaning, not technical details.
3.  Consumers SHALL not modify E04 state directly.
4.  Event contracts SHALL be version controlled.
5.  Failed event delivery SHALL support retry and recovery mechanisms.

------------------------------------------------------------------------

# 10.10 Domain Event Summary

``` mermaid
flowchart TD

Onboarding["Onboarding Events"]

Leave["Leave Events"]

Employment["Employment Change Events"]

Certificate["Certificate Events"]

Cases["HR Case Events"]

Compliance["Compliance Events"]

Bus["Event Bus"]


Onboarding --> Bus
Leave --> Bus
Employment --> Bus
Certificate --> Bus
Cases --> Bus
Compliance --> Bus
```


# 11. Service Contracts

## 11.1 Service Contract Philosophy

L2-E04 HR Workflows exposes controlled service contracts for:

-   Workflow commands.
-   Workflow queries.
-   External domain interactions.
-   Business process integration.

The domain SHALL NOT expose internal aggregate storage.

Communication SHALL follow:

``` text
Commands
    |
    ↓
E04 Workflow Domain
    |
    ↓
Domain Events
    |
    ↓
External Consumers
```

------------------------------------------------------------------------

# 11.2 Command Contracts

Commands represent business intentions.

## CMD-HR-001 --- Create HR Case

### Purpose

Creates a new HR operational case.

### Input

  Field               Description
  ------------------- ---------------------
  caseType            HR case category
  employeeReference   Employee identifier
  priority            Business priority
  description         Case details

### Output

  Result   Description
  -------- -------------------------
  caseId   Created case identifier
  status   Initial workflow state

------------------------------------------------------------------------

## CMD-HR-002 --- Submit Leave Request

### Purpose

Creates a leave workflow.

### Input

  Field               Description
  ------------------- ---------------------
  employeeReference   Employee identifier
  leaveType           Leave category
  startDate           Leave start
  endDate             Leave end

### Output

-   Leave request identifier.
-   Workflow status.

------------------------------------------------------------------------

## CMD-HR-003 --- Approve HR Workflow

### Purpose

Approves a pending HR workflow.

### Input

  Field        Description
  ------------ ---------------------
  workflowId   Workflow identifier
  approverId   Decision maker
  decision     Approval decision
  remarks      Decision reason

------------------------------------------------------------------------

## CMD-HR-004 --- Initiate Employee Onboarding

### Purpose

Starts employee onboarding after approved hiring.

Input:

-   Hiring reference.
-   Employee reference.
-   Joining information.

Output:

-   Onboarding workflow identifier.

------------------------------------------------------------------------

## CMD-HR-005 --- Submit Employment Change

Supports:

-   Promotion.
-   Transfer.
-   Designation change.
-   Contract renewal.

------------------------------------------------------------------------

# 11.3 Query Contracts

Queries provide read access to workflow information.

## QUERY-HR-001 --- Get Workflow Status

Returns:

-   Current state.
-   Pending actions.
-   Approval status.

------------------------------------------------------------------------

## QUERY-HR-002 --- Get Employee HR History

Returns:

-   Completed HR workflows.
-   Decisions.
-   Audit references.

------------------------------------------------------------------------

## QUERY-HR-003 --- Get Pending Approvals

Returns:

-   Pending approvals.
-   Assigned approvers.
-   SLA status.

------------------------------------------------------------------------

## QUERY-HR-004 --- Get Certificate Status

Returns:

-   Request status.
-   Generation status.
-   Delivery status.

------------------------------------------------------------------------

# 11.4 External Service Contracts

## Employee Profile Integration

Purpose:

Synchronize approved employee lifecycle changes.

Contract:

``` text
E04 → E07

EmployeeLifecycleUpdate
```

------------------------------------------------------------------------

## Payroll Integration

Purpose:

Communicate financial-impacting HR decisions.

Examples:

-   Compensation changes.
-   Leave outcomes.
-   Separation completion.

Contract:

``` text
E04 → Payroll

HRBusinessChangeEvent
```

------------------------------------------------------------------------

## Organization Integration

Purpose:

Validate organizational assignments.

Contract:

``` text
E04 → Organization

AssignmentValidationRequest
```

------------------------------------------------------------------------

## Document Platform Integration

Purpose:

Generate and manage HR documents.

Contract:

``` text
E04 → Document Platform

DocumentGenerationRequest
```

------------------------------------------------------------------------

# 11.5 Event Contract Rules

All events SHALL contain:

  Attribute      Description
  -------------- ----------------------
  Event ID       Unique identifier
  Event Type     Business event name
  Aggregate ID   Source aggregate
  Timestamp      Creation time
  Version        Contract version
  Payload        Business information

------------------------------------------------------------------------

# 12. State Transition Matrix

## 12.1 Workflow State Principles

State transitions SHALL:

-   Follow defined lifecycle rules.
-   Be authorized.
-   Be auditable.
-   Generate events where applicable.

------------------------------------------------------------------------

# 12.2 HR Case State Transition

  Current State   Action    Next State
  --------------- --------- -------------
  Created         Assign    Assigned
  Assigned        Process   In Progress
  In Progress     Resolve   Resolved
  Resolved        Close     Closed

------------------------------------------------------------------------

# 12.3 Leave Request State Transition

  Current State      Action     Next State
  ------------------ ---------- ------------------
  Submitted          Validate   Pending Approval
  Pending Approval   Approve    Approved
  Pending Approval   Reject     Rejected
  Approved           Complete   Completed

------------------------------------------------------------------------

# 12.4 Onboarding State Transition

  Current State          Action         Next State
  ---------------------- -------------- ----------------------
  Initiated              Collect Data   Information Pending
  Information Pending    Verify         Verification Pending
  Verification Pending   Approve        Ready For Joining
  Ready For Joining      Activate       Completed

------------------------------------------------------------------------

# 12.5 Separation State Transition

  Current State        Action               Next State
  -------------------- -------------------- --------------------
  Requested            Approve              Exit Clearance
  Exit Clearance       Complete Clearance   Final Verification
  Final Verification   Complete             Separated

------------------------------------------------------------------------

# 12.6 Generic Workflow Lifecycle

``` mermaid
stateDiagram-v2

[*] --> Initiated

Initiated --> PendingApproval

PendingApproval --> Approved

PendingApproval --> Rejected

Approved --> Processing

Processing --> Completed

Processing --> Failed

Failed --> Processing

Completed --> Archived
```

------------------------------------------------------------------------

# 13. Decision Matrix

## 13.1 Decision Ownership Philosophy

Decision ownership SHALL remain aligned with domain ownership.

E04 decides:

-   HR workflow progression.
-   Approval outcomes.
-   HR case resolution.

Other domains decide:

-   Their own authoritative data changes.

------------------------------------------------------------------------

# 13.2 Business Decision Matrix

  Decision                  Owner               Supporting Domain
  ------------------------- ------------------- -------------------
  Start onboarding          E04 HR Workflows    Recruitment
  Approve leave             E04 HR Workflows    Policy/Attendance
  Promotion approval        E04 HR Workflows    Organization
  Salary calculation        Payroll             E04
  Department assignment     Organization        E04
  Employee profile update   E07                 E04
  Document storage          Document Platform   E04
  Authentication access     Identity            E04

------------------------------------------------------------------------

# 13.3 Conflict Resolution Rules

If ownership conflict occurs:

1.  Authoritative data owner decides.
2.  E04 manages workflow coordination.
3.  Events communicate completed decisions.
4.  Direct database modification is prohibited.

------------------------------------------------------------------------

# 13.4 Service Contract Summary

  Capability              Contract Type
  ----------------------- -------------------
  Workflow Commands       Command API
  Workflow Queries        Query API
  Multi-domain updates    Domain Events
  Document generation     Service Contract
  Employee updates        Event Integration
  Payroll communication   Event Integration

------------------------------------------------------------------------

# 13.5 Architecture Principle

``` text
Services expose capabilities.

Events communicate business outcomes.

Domains own their data.
```

# 14. Failure Matrix

## 14.1 Failure Handling Philosophy

L2-E04 HR Workflows SHALL treat failures as controlled business and
technical states.

Failures SHALL NOT result in:

-   Lost workflow history.
-   Silent data inconsistency.
-   Untraceable decisions.
-   Direct unauthorized corrections.

The failure handling principle:

``` text
Detect

↓

Record

↓

Recover

↓

Resume or Escalate
```

------------------------------------------------------------------------

# 14.2 Workflow Failure Categories

  Category                Description
  ----------------------- -----------------------------------
  Business Failure        Business rule validation failure
  Authorization Failure   User lacks permission
  Integration Failure     External service unavailable
  Data Failure            Invalid or incomplete information
  Processing Failure      Internal execution failure
  Document Failure        Document generation issue

------------------------------------------------------------------------

# 14.3 Failure Matrix

  -----------------------------------------------------------------------
  Failure Scenario  Impact            Detection         Recovery
  ----------------- ----------------- ----------------- -----------------
  Invalid leave     Workflow rejected Business          Request
  request                             validation        correction

  Missing           Onboarding        Document          Submit missing
  onboarding        blocked           validation        information
  document                                              

  Approval timeout  Workflow delayed  SLA monitoring    Escalation
                                                        workflow

  Payroll           External update   Integration       Retry mechanism
  integration       pending           monitoring        
  unavailable                                           

  Document          Certificate       Generation status Regenerate
  generation        unavailable       check             document
  failure                                               

  Event delivery    Consumer not      Event monitoring  Retry and
  failure           updated                             reconciliation

  Unauthorized      Security          Authorization     Reject and audit
  access attempt    violation         layer             

  Invalid workflow  State             State validation  Reject transition
  transition        inconsistency                       
                    prevented                           
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 14.4 Integration Failure Handling

External integrations SHALL support:

-   Retry policies.
-   Failure tracking.
-   Compensation workflows.
-   Manual intervention.

Example:

``` mermaid
flowchart TD

Request["External Request"]

Success["Successful Response"]

Failure["Failure Detected"]

Retry["Retry Processing"]

Escalate["Manual Intervention"]

Resolved["Resolved"]


Request --> Success

Request --> Failure

Failure --> Retry

Retry --> Success

Retry --> Escalate

Escalate --> Resolved
```

------------------------------------------------------------------------

# 14.5 Manual Intervention Model

Manual intervention SHALL be supported for exceptional situations.

Examples:

-   Payroll unavailable.
-   External service outage.
-   Policy exception.
-   Document correction.

Manual actions SHALL require:

-   Authorized user.
-   Reason capture.
-   Audit record.
-   Timestamp.

------------------------------------------------------------------------

# 14.6 Failure State Management

Workflows SHALL support failure states.

Example:

``` mermaid
stateDiagram-v2

[*] --> Processing

Processing --> Completed

Processing --> Failed

Failed --> RetryPending

RetryPending --> Processing

Failed --> ManualReview

ManualReview --> Completed
```

------------------------------------------------------------------------

# 14.7 Recovery Principles

1.  Failures SHALL be recoverable whenever possible.
2.  Failed workflows SHALL preserve original context.
3.  Recovery SHALL not bypass authorization.
4.  Manual corrections SHALL be audited.
5.  External failures SHALL not corrupt internal workflow state.

------------------------------------------------------------------------

# 15. Validation Rules

## 15.1 Validation Philosophy

Validation rules ensure that HR workflows execute only with:

-   Valid data.
-   Authorized actors.
-   Applicable policies.
-   Correct lifecycle state.

Validation occurs at:

``` text
Request Level

↓

Business Rule Level

↓

Authorization Level

↓

Integration Level
```

------------------------------------------------------------------------

# 15.2 Identity Validation Rules

## VR-HR-001 --- Employee Reference Validation

The system SHALL validate:

-   Employee exists.
-   Employee reference is valid.
-   Employee status allows requested operation.

------------------------------------------------------------------------

## VR-HR-002 --- Actor Validation

The system SHALL validate:

-   User identity.
-   Role permission.
-   Workflow authority.

------------------------------------------------------------------------

# 15.3 Workflow Validation Rules

## VR-HR-003 --- State Validation

Every workflow action SHALL verify:

-   Current state.
-   Allowed transition.
-   Required approvals.

------------------------------------------------------------------------

## VR-HR-004 --- Approval Validation

Approval actions SHALL validate:

-   Approver authority.
-   Approval sequence.
-   Delegation rules.
-   Decision reason.

------------------------------------------------------------------------

# 15.4 Leave Validation Rules

## VR-HR-005 --- Leave Eligibility

Leave requests SHALL validate:

-   Applicable policy.
-   Employee eligibility.
-   Requested dates.
-   Approval requirement.

------------------------------------------------------------------------

## VR-HR-006 --- Attendance Conflict Validation

Leave workflow MAY validate attendance conflicts through Attendance
Domain.

Attendance remains the source of attendance truth.

------------------------------------------------------------------------

# 15.5 Onboarding Validation Rules

## VR-HR-007 --- Required Information Validation

Onboarding SHALL validate:

-   Required employee information.
-   Mandatory documents.
-   Verification completion.

------------------------------------------------------------------------

# 15.6 Employment Change Validation

## VR-HR-008 --- Promotion Validation

Promotion workflow SHALL validate:

-   Eligibility criteria.
-   Approval authority.
-   Effective date.
-   Organizational compatibility.

------------------------------------------------------------------------

## VR-HR-009 --- Transfer Validation

Transfer workflow SHALL validate:

-   Source assignment.
-   Target assignment.
-   Approval completion.

------------------------------------------------------------------------

# 15.7 Document Validation Rules

## VR-HR-010 --- Certificate Generation Validation

Certificate generation SHALL validate:

-   Request eligibility.
-   Approval completion.
-   Template availability.
-   Authorized signer.

------------------------------------------------------------------------

# 15.8 Compliance Validation Rules

## VR-HR-011 --- Compliance Evidence Validation

Compliance workflows SHALL validate:

-   Required evidence exists.
-   Verification completed.
-   Compliance status updated.

------------------------------------------------------------------------

# 15.9 Multi-Institution Validation Rules

Institution-specific configurations SHALL validate:

-   Allowed workflow customization.
-   Applicable policies.
-   Approval definitions.
-   Template availability.

------------------------------------------------------------------------

# 15.10 Validation Summary

  Validation Area            Purpose
  -------------------------- -------------------------------------
  Identity Validation        Ensure correct employee and actor
  Authorization Validation   Prevent unauthorized actions
  Workflow Validation        Maintain lifecycle integrity
  Policy Validation          Apply institutional rules
  Integration Validation     Ensure external consistency
  Document Validation        Ensure valid certificates/documents
  Compliance Validation      Maintain regulatory readiness


# 16. Authorization Model

## 16.1 Authorization Philosophy

L2-E04 HR Workflows operates on highly sensitive employee-related
business processes.

Authorization SHALL ensure:

-   Only authorized users can execute HR actions.
-   Sensitive information is protected.
-   Workflow decisions are traceable.
-   Access follows organizational responsibility.
-   Institutional boundaries are respected.

The authorization principle:

``` text
Role

+

Data Scope

+

Sensitivity Classification

+

Workflow Authority

=

Authorized HR Access
```

------------------------------------------------------------------------

# 16.2 Authorization Model Overview

E04 uses a multi-dimensional authorization model.

``` mermaid
flowchart TD

User["User"]

Role["Role Based Access Control"]

Scope["Data Scope Control"]

Sensitivity["Data Sensitivity Rules"]

Workflow["Workflow Authority"]

Decision["Authorization Decision"]


User --> Role
User --> Scope
User --> Sensitivity
User --> Workflow

Role --> Decision
Scope --> Decision
Sensitivity --> Decision
Workflow --> Decision
```

------------------------------------------------------------------------

# 16.3 Role Based Access Control (RBAC)

## HR Administrator

Permissions:

-   Create HR cases.
-   Manage employee requests.
-   Execute HR workflows.
-   View authorized employee information.

------------------------------------------------------------------------

## HR Manager

Permissions:

-   Approve HR workflows.
-   Review employee lifecycle changes.
-   Manage escalated cases.

------------------------------------------------------------------------

## Department Manager

Permissions:

-   Submit employee requests.
-   Approve assigned workflows.
-   View permitted team information.

------------------------------------------------------------------------

## Employee

Permissions:

-   Submit HR requests.
-   View own HR information.
-   Track workflow status.
-   Request certificates.

------------------------------------------------------------------------

## System Administrator

Permissions:

-   Configure system settings.
-   Manage technical access.

System administrators SHALL NOT automatically receive business data
access.

------------------------------------------------------------------------

# 16.4 Data-Level Authorization

HR access SHALL be restricted using organizational scope.

Examples:

``` text
Campus HR Manager

↓

Only Campus Employees
```

``` text
Institution HR Director

↓

All Institution Employees
```

------------------------------------------------------------------------

# 16.5 Sensitivity Classification

HR information SHALL be classified.

  Classification        Examples
  --------------------- -------------------------------------------
  Public                General HR policies
  Internal              Workflow status
  Confidential          Employee records
  Highly Confidential   Medical, disciplinary, salary information

------------------------------------------------------------------------

# 16.6 Workflow Authorization Rules

The system SHALL validate:

-   User role.
-   Workflow ownership.
-   Approval authority.
-   Delegation rules.
-   Separation of duties.

Example:

``` mermaid
flowchart LR

Request["Workflow Request"]

Authority["Check Authority"]

Decision["Approval Allowed"]

Reject["Reject Action"]


Request --> Authority

Authority --> Decision

Authority --> Reject
```

------------------------------------------------------------------------

# 16.7 Separation of Duties

The system SHALL prevent conflicts of interest.

Examples:

-   Request creator cannot approve own request.
-   Employee cannot approve own promotion.
-   Unauthorized users cannot modify completed workflows.

------------------------------------------------------------------------

# 16.8 Authorization Summary

  Area                   Control
  ---------------------- -----------------------------
  User Access            RBAC
  Organizational Scope   Data-level security
  Sensitive Data         Classification-based access
  Workflow Actions       Authority validation
  Approvals              Separation of duties

------------------------------------------------------------------------

# 17. Audit Requirements

## 17.1 Audit Philosophy

HR workflows require complete business auditability.

Audit SHALL capture:

-   What happened.
-   Who performed the action.
-   When it happened.
-   Why it happened.
-   Previous state.
-   New state.

------------------------------------------------------------------------

# 17.2 Audited Activities

The following SHALL create audit records:

## Workflow Actions

-   Creation.
-   Approval.
-   Rejection.
-   Cancellation.
-   Completion.

------------------------------------------------------------------------

## Employee Lifecycle Events

-   Onboarding completion.
-   Confirmation.
-   Promotion.
-   Transfer.
-   Separation.

------------------------------------------------------------------------

## Document Activities

-   Certificate generation.
-   Document verification.
-   Document access.

------------------------------------------------------------------------

## Security Activities

-   Authorization failures.
-   Sensitive data access.
-   Permission changes.

------------------------------------------------------------------------

# 17.3 Audit Record Model

``` mermaid
classDiagram

class AuditRecord {

+auditId

+actorId

+action

+timestamp

+entityId

+previousState

+newState

+reason

}
```

------------------------------------------------------------------------

# 17.4 Audit Lifecycle

``` mermaid
flowchart LR

Action["Business Action"]

Capture["Capture Audit Data"]

Store["Immutable Audit Storage"]

Review["Audit Review"]


Action --> Capture

Capture --> Store

Store --> Review
```

------------------------------------------------------------------------

# 17.5 Audit Rules

1.  Audit records SHALL be immutable.
2.  Audit history SHALL not be deleted by normal users.
3.  Sensitive actions SHALL include reason capture.
4.  Administrative overrides SHALL be audited.
5.  Audit records SHALL support compliance review.

------------------------------------------------------------------------

# 18. Security Considerations

## 18.1 Security Philosophy

HR workflows handle highly sensitive organizational information.

Security SHALL protect:

-   Employee privacy.
-   Business confidentiality.
-   Workflow integrity.
-   Document confidentiality.
-   Regulatory compliance.

------------------------------------------------------------------------

# 18.2 Data Security Controls

The domain SHALL support:

-   Encryption in transit.
-   Encryption at rest.
-   Secure API communication.
-   Access logging.
-   Sensitive field protection.

------------------------------------------------------------------------

# 18.3 Workflow Security

Workflow execution SHALL enforce:

``` mermaid
flowchart TD

Action["Requested Action"]

Authentication["Authentication"]

Authorization["Authorization"]

Validation["Business Validation"]

Execution["Workflow Execution"]

Audit["Audit Record"]


Action --> Authentication

Authentication --> Authorization

Authorization --> Validation

Validation --> Execution

Execution --> Audit
```

------------------------------------------------------------------------

# 18.4 Document Security

HR document workflows SHALL enforce:

-   Permission-controlled access.
-   Document sensitivity classification.
-   Authorized signing.
-   Access auditing.

E04 SHALL not directly manage document storage security.

Document Platform owns storage security controls.

------------------------------------------------------------------------

# 18.5 Privacy Protection

Sensitive employee information SHALL follow:

-   Least privilege access.
-   Need-to-know principle.
-   Controlled visibility.
-   Audit tracking.

------------------------------------------------------------------------

# 18.6 Security Boundary Rules

The following rules SHALL apply:

1.  Authentication SHALL be owned by Identity Domain.
2.  Authorization decisions SHALL follow enterprise security policies.
3.  HR data SHALL never be exposed through uncontrolled access.
4.  Sensitive workflow actions SHALL require authorization.
5.  All privileged actions SHALL be audited.

------------------------------------------------------------------------

# 18.7 Security Summary

  Security Area        Approach
  -------------------- ---------------------------------
  Authentication       Identity Domain
  Authorization        RBAC + Data Scope + Sensitivity
  Audit                Immutable Business Audit
  Documents            Controlled Document Security
  Privacy              Least Privilege
  Workflow Integrity   Authorized State Transitions


# 19. Performance & Scalability

## 19.1 Performance Philosophy

L2-E04 HR Workflows SHALL be designed to support enterprise-scale HR
operations across multiple institutions, campuses, and employee
populations.

Performance SHALL be achieved through:

-   Workflow optimization.
-   Asynchronous processing.
-   Event-driven communication.
-   Independent scaling.
-   Efficient data access patterns.
-   Controlled workload distribution.

The principle:

``` text
High Volume Workflows

+

Independent Processing

+

Event Driven Integration

=

Scalable HR Operations
```

------------------------------------------------------------------------

# 19.2 Scalability Requirements

The domain SHALL support:

-   Multiple institutions.
-   Multiple campuses.
-   Large employee populations.
-   High-volume HR requests.
-   Bulk onboarding operations.
-   Large document workflows.
-   Concurrent approval processes.

------------------------------------------------------------------------

# 19.3 Workflow Execution Scalability

Workflow execution SHALL be separated from user interaction.

Example:

``` mermaid
flowchart LR

User["User Action"]

API["HR Workflow API"]

Engine["Workflow Engine"]

Queue["Async Processing Queue"]

Workers["Processing Workers"]


User --> API

API --> Engine

Engine --> Queue

Queue --> Workers
```

Benefits:

-   Reduced response time.
-   Better workload distribution.
-   Improved failure recovery.

------------------------------------------------------------------------

# 19.4 Asynchronous Processing

The following operations SHOULD support asynchronous execution:

-   Bulk employee onboarding.
-   Certificate generation.
-   Document processing.
-   Notification triggering.
-   External system synchronization.
-   Large compliance checks.

------------------------------------------------------------------------

# 19.5 Event Processing Scalability

E04 SHALL use event-driven processing for multi-domain communication.

Example:

``` mermaid
flowchart LR

HR["HR Workflow"]

Event["Domain Event"]

Bus["Event Bus"]

Payroll["Payroll"]

Analytics["Analytics"]

Notification["Notification"]


HR --> Event

Event --> Bus

Bus --> Payroll

Bus --> Analytics

Bus --> Notification
```

------------------------------------------------------------------------

# 19.6 Data Access Optimization

The domain SHALL support:

-   Efficient workflow queries.
-   Indexed workflow identifiers.
-   Optimized approval searches.
-   Pagination for large datasets.
-   Read optimization for dashboards.

------------------------------------------------------------------------

# 19.7 Caching Strategy

Caching MAY be used for:

-   Workflow definitions.
-   Institution configurations.
-   Approval hierarchy information.
-   Policy configurations.

Caching SHALL NOT store:

-   Sensitive employee information without protection.
-   Authoritative workflow state.

------------------------------------------------------------------------

# 19.8 Multi-Institution Scalability

The architecture SHALL support:

``` mermaid
flowchart TD

Platform["SutraERP Platform"]

InstitutionA["Institution A"]

InstitutionB["Institution B"]

InstitutionC["Institution C"]

Configuration["Configuration Layer"]

Platform --> Configuration

Configuration --> InstitutionA

Configuration --> InstitutionB

Configuration --> InstitutionC
```

Each institution may configure:

-   Approval chains.
-   Leave policies.
-   Certificate templates.
-   SLA rules.

Core domain behavior remains consistent.

------------------------------------------------------------------------

# 19.9 Bulk Processing Capability

E04 SHALL support controlled bulk operations.

Examples:

-   Bulk onboarding.
-   Bulk employee status updates.
-   Bulk compliance verification.

Bulk operations SHALL:

-   Execute asynchronously.
-   Provide progress tracking.
-   Maintain audit history.
-   Support failure recovery.

------------------------------------------------------------------------

# 19.10 Performance Monitoring

The domain SHALL monitor:

-   Workflow execution time.
-   Approval turnaround time.
-   Failed workflows.
-   Event processing latency.
-   Integration failures.

------------------------------------------------------------------------

# 20. Extension Points

## 20.1 Extension Philosophy

L2-E04 SHALL support future HR capability expansion without violating
domain boundaries.

Extensions SHALL be introduced through:

-   New workflow aggregates.
-   New domain events.
-   New service contracts.
-   Configurable workflow definitions.

------------------------------------------------------------------------

# 20.2 Workflow Extension Model

``` mermaid
flowchart TD

Core["E04 Core Workflow Engine"]

Extension["Extension Capability"]

Contract["Service Contract"]

Events["Domain Events"]


Extension --> Contract

Extension --> Events

Contract --> Core

Core --> Events
```

------------------------------------------------------------------------

# 20.3 Future Extension Areas

## Performance Management

Potential future capability:

-   Goal workflows.
-   Review cycles.
-   Evaluation processes.

This SHOULD be a separate bounded context.

------------------------------------------------------------------------

## Recruitment Integration

Potential capability:

-   Advanced recruitment lifecycle.
-   Candidate management.
-   Hiring analytics.

This SHOULD remain separate from HR operations.

------------------------------------------------------------------------

## Learning and Development

Potential capability:

-   Training workflows.
-   Certification tracking.
-   Skill development.

This SHOULD integrate through contracts/events.

------------------------------------------------------------------------

## Workforce Analytics

Potential capability:

-   Workforce insights.
-   Attrition analysis.
-   Planning models.

Analytics SHALL consume HR events.

------------------------------------------------------------------------

# 20.4 Custom Workflow Extensions

Institutions MAY configure:

-   Approval sequences.
-   SLA rules.
-   Templates.
-   Notification preferences.

Institutions SHALL NOT modify:

-   Aggregate ownership.
-   Core business invariants.
-   Security rules.

------------------------------------------------------------------------

# 20.5 Integration Extension Points

New integrations SHALL use:

``` mermaid
flowchart LR

NewDomain["New Domain"]

Contract["Service Contract"]

Event["Domain Event"]

HR["E04 HR Workflow"]


NewDomain --> Contract

HR --> Event

Event --> NewDomain
```

------------------------------------------------------------------------

# 20.6 Extension Governance Rules

Every extension SHALL define:

1.  Ownership boundary.
2.  Data ownership.
3.  Integration contract.
4.  Security requirements.
5.  Audit requirements.
6.  Failure handling strategy.

------------------------------------------------------------------------

# 20.7 Performance and Extension Summary

  Area                Approach
  ------------------- --------------------------------
  Workflow Scaling    Independent workflow execution
  Large Operations    Async processing
  Integration         Event-driven architecture
  Multi-Institution   Configuration-driven model
  Future Features     Separate bounded contexts
  Customization       Controlled extension points


# 21. Architecture Visual Models

## 21.1 Visual Model Philosophy

Architecture visual models provide a complete representation of L2-E04
HR Workflows from multiple architectural perspectives.

The visual models cover:

-   Domain boundary.
-   Aggregate relationships.
-   Workflow orchestration.
-   Event communication.
-   Service interaction.
-   Authorization model.
-   Security boundaries.
-   Multi-domain integration.

All diagrams SHALL represent ownership boundaries clearly.

------------------------------------------------------------------------

# 21.2 Bounded Context Context Map

``` mermaid
flowchart LR

HR["L2-E04 HR Workflows"]

Employee["E07 Employee Profile"]

Faculty["E03 Faculty Workforce"]

Organization["Organization Domain"]

Payroll["Payroll Domain"]

Attendance["Attendance Domain"]

Identity["Identity Domain"]

Document["Document Platform"]

Analytics["Analytics Domain"]


Recruitment["Recruitment Domain"]


Recruitment --> HR

HR --> Employee

HR --> Faculty

HR --> Organization

HR --> Payroll

HR --> Attendance

HR --> Identity

HR --> Document

HR --> Analytics
```

------------------------------------------------------------------------

# 21.3 E04 Domain Boundary Model

``` mermaid
flowchart TB

subgraph E04["L2-E04 HR Workflows Bounded Context"]

Cases["HR Case Aggregate"]

Onboarding["Employee Onboarding Aggregate"]

Leave["Leave Request Aggregate"]

Certificate["Certificate Request Aggregate"]

Employment["Employment Change Aggregate"]

Separation["Separation Aggregate"]

Compliance["Compliance Case Aggregate"]

end


External["External Domains"]


E04 --> External
```

------------------------------------------------------------------------

# 21.4 Aggregate Relationship Model

``` mermaid
flowchart LR

HRCase["HR Case"]

Onboarding["Onboarding"]

Leave["Leave Request"]

Certificate["Certificate Request"]

Employment["Employment Change"]

Separation["Separation"]

Compliance["Compliance Case"]


HRCase --> Compliance

Onboarding --> Employment

Leave --> HRCase

Certificate --> HRCase

Separation --> HRCase
```

------------------------------------------------------------------------

# 21.5 Employee Lifecycle Model

``` mermaid
stateDiagram-v2

[*] --> HiringApproved

HiringApproved --> Onboarding

Onboarding --> Verification

Verification --> ActiveEmployment

ActiveEmployment --> Probation

Probation --> Confirmed

Confirmed --> EmploymentChange

EmploymentChange --> ActiveEmployment

ActiveEmployment --> Separation

Separation --> ExitClearance

ExitClearance --> Separated

Separated --> Archived
```

------------------------------------------------------------------------

# 21.6 HR Workflow Orchestration Model

``` mermaid
flowchart LR

Trigger["Business Trigger"]

Workflow["HR Workflow Engine"]

Rules["Business Rules"]

Approval["Approval Engine"]

Decision["Business Decision"]

Event["Domain Event"]


Trigger --> Workflow

Workflow --> Rules

Rules --> Approval

Approval --> Decision

Decision --> Event
```

------------------------------------------------------------------------

# 21.7 Domain Event Architecture Model

``` mermaid
flowchart LR

Aggregate["E04 Aggregates"]

Events["Domain Events"]

Bus["Event Bus"]

Consumers["Consumers"]


Aggregate --> Events

Events --> Bus

Bus --> Consumers
```

------------------------------------------------------------------------

# 21.8 Service Interaction Model

``` mermaid
sequenceDiagram

participant User

participant HR as E04 HR Workflow

participant Employee as E07 Employee Profile

participant Payroll

participant Organization

User->>HR: Submit HR Request

HR->>HR: Execute Workflow

HR->>Employee: Publish Employee Update

HR->>Payroll: Publish Financial Event

HR->>Organization: Request Assignment Validation

HR-->>User: Workflow Result
```

------------------------------------------------------------------------

# 21.9 Authorization Architecture Model

``` mermaid
flowchart TD

User["User"]

Role["Role"]

Scope["Organizational Scope"]

Sensitivity["Data Sensitivity"]

Workflow["Workflow Authority"]

Decision["Access Decision"]


User --> Role

User --> Scope

User --> Sensitivity

User --> Workflow

Role --> Decision

Scope --> Decision

Sensitivity --> Decision

Workflow --> Decision
```

------------------------------------------------------------------------

# 21.10 Audit Architecture Model

``` mermaid
flowchart LR

Action["Business Action"]

Audit["Audit Capture"]

Storage["Immutable Audit Store"]

Review["Compliance Review"]


Action --> Audit

Audit --> Storage

Storage --> Review
```

------------------------------------------------------------------------

# 21.11 Security Execution Model

``` mermaid
flowchart TD

Request["Requested Action"]

Authentication["Authentication"]

Authorization["Authorization"]

Validation["Business Validation"]

Execution["Workflow Execution"]

Audit["Audit"]


Request --> Authentication

Authentication --> Authorization

Authorization --> Validation

Validation --> Execution

Execution --> Audit
```

------------------------------------------------------------------------

# 21.12 Multi-Institution Architecture Model

``` mermaid
flowchart TD

Platform["SutraERP Platform"]

Configuration["Configuration Layer"]

InstitutionA["Institution A"]

InstitutionB["Institution B"]

InstitutionC["Institution C"]


Platform --> Configuration

Configuration --> InstitutionA

Configuration --> InstitutionB

Configuration --> InstitutionC
```

------------------------------------------------------------------------

# 21.13 Complete E04 Architecture View

``` mermaid
flowchart TB

HR["L2-E04 HR Workflows"]

subgraph Processes["HR Processes"]

Onboarding["Onboarding"]

Leave["Leave"]

Cases["HR Cases"]

Documents["Certificates"]

Changes["Employment Changes"]

Exit["Separation"]

end


Events["Domain Events"]

Consumers["Enterprise Domains"]


HR --> Processes

Processes --> Events

Events --> Consumers
```

------------------------------------------------------------------------

# 21.14 Visual Model Summary

The architecture visualization set establishes:

``` text
Domain Boundaries

+

Aggregate Ownership

+

Workflow Execution

+

Event Communication

+

Security Controls

+

Enterprise Integration
```

The models collectively define the complete L2-E04 HR Workflow
architecture.




# 22. Architecture Decision Records (ADR)

# ADR-001 --- HR Workflow Domain Does Not Own Employee Profile Data

## Status

Accepted

## Context

Employee information is consumed across multiple enterprise domains.

Duplicating employee ownership inside HR would create inconsistent
records.

## Decision

E04 owns HR processes.

E07 Employee Profile owns employee profile data.

## Consequences

Positive:

-   Clear ownership.
-   No duplicate employee records.
-   Better domain autonomy.

Negative:

-   Requires integration contracts.

------------------------------------------------------------------------

# ADR-002 --- Workflow-Based Aggregate Design

## Status

Accepted

## Context

HR operations contain independent business processes.

A single employee aggregate would create excessive coupling.

## Decision

Create workflow-specific aggregates.

Examples:

-   Leave Request.
-   HR Case.
-   Onboarding.
-   Separation.

## Consequences

Positive:

-   Better scalability.
-   Independent lifecycle management.

Negative:

-   Requires event coordination.

------------------------------------------------------------------------

# ADR-003 --- Event Driven Multi-Domain Communication

## Status

Accepted

## Context

HR decisions affect multiple domains.

Direct database synchronization creates coupling.

## Decision

Use:

-   Service contracts.
-   Domain events.

## Consequences

Positive:

-   Loose coupling.
-   Independent evolution.

Negative:

-   Requires event management.

------------------------------------------------------------------------

# ADR-004 --- Payroll Remains External to HR Workflow Ownership

## Status

Accepted

## Context

Payroll involves financial calculations and compliance.

## Decision

HR initiates business decisions.

Payroll calculates financial outcomes.

## Consequences

Positive:

-   Correct ownership.
-   Reduced financial risk.

------------------------------------------------------------------------

# ADR-005 --- Document Storage Remains External

## Status

Accepted

## Context

Documents are used across many domains.

## Decision

E04 manages document workflows.

Document Platform manages storage.

## Consequences

Positive:

-   Shared enterprise capability.
-   Reduced duplication.

------------------------------------------------------------------------

# ADR-006 --- Configurable Multi-Institution Workflow Model

## Status

Accepted

## Context

Institutions require different approval processes.

## Decision

Support configuration-driven workflows.

Core domain rules remain protected.

## Consequences

Positive:

-   Multi-tenant flexibility.
-   Reduced customization cost.

------------------------------------------------------------------------

# ADR-007 --- RBAC Plus Data Scope Authorization

## Status

Accepted

## Context

HR information contains sensitive data.

Simple role access is insufficient.

## Decision

Use:

-   Role-based access.
-   Organizational scope.
-   Sensitivity classification.

## Consequences

Positive:

-   Strong privacy protection.
-   Better compliance.

------------------------------------------------------------------------

# ADR-008 --- Immutable HR Business Audit

## Status

Accepted

## Context

HR operations require traceability.

## Decision

Maintain immutable audit history for business actions.

## Consequences

Positive:

-   Compliance readiness.
-   Dispute resolution capability.

------------------------------------------------------------------------


# 23. Glossary

## 23.1 Purpose

This glossary defines canonical terminology used throughout the L2-E04
HR Workflows architecture document.

The terminology SHALL be used consistently across:

-   Domain models.
-   Workflow definitions.
-   Service contracts.
-   Domain events.
-   Architecture decisions.

------------------------------------------------------------------------

# 23.2 Domain Terms

## HR Workflow

A business process executed by the HR domain to manage employee-related
operational activities.

Examples:

-   Onboarding.
-   Leave approval.
-   Separation.

------------------------------------------------------------------------

## HR Case

A controlled business process instance created to manage an HR request,
issue, or administrative activity.

Examples:

-   Employee request.
-   Grievance.
-   Compliance issue.

------------------------------------------------------------------------

## Workflow Aggregate

A domain-driven design aggregate responsible for maintaining the
lifecycle and business rules of a specific HR process.

Examples:

-   Leave Request Aggregate.
-   Separation Aggregate.

------------------------------------------------------------------------

## Employee Reference

A reference identifier pointing to employee information owned by the
Employee Profile domain.

E04 stores references, not employee profile data.

------------------------------------------------------------------------

## Business Event

A meaningful occurrence in the business lifecycle that may be consumed
by other domains.

Examples:

-   EmployeeOnboardingCompleted.
-   LeaveApproved.

------------------------------------------------------------------------

## Domain Event

A technical representation of a completed business event published by a
bounded context.

------------------------------------------------------------------------

## Service Contract

A defined communication interface between domains.

Examples:

-   Command APIs.
-   Query APIs.
-   Integration contracts.

------------------------------------------------------------------------

## Approval Workflow

A controlled sequence where authorized actors evaluate and decide on a
business request.

------------------------------------------------------------------------

## Workflow State

The current lifecycle condition of a business process.

Examples:

-   Initiated.
-   Pending Approval.
-   Completed.

------------------------------------------------------------------------

## Workflow Transition

A permitted movement from one workflow state to another.

------------------------------------------------------------------------

## Aggregate Root

The primary entity responsible for maintaining consistency rules within
an aggregate boundary.

------------------------------------------------------------------------

## Value Object

An object defined by its attributes rather than identity.

Examples:

-   Employee Reference.
-   Approval Decision.

------------------------------------------------------------------------

# 23.3 HR Lifecycle Terms

## Onboarding

The workflow responsible for converting an approved hiring decision into
an active employee lifecycle state.

------------------------------------------------------------------------

## Probation

A temporary employment evaluation period managed through HR workflows.

------------------------------------------------------------------------

## Confirmation

The workflow that transitions an employee from probationary status to
confirmed employment.

------------------------------------------------------------------------

## Promotion

A workflow managing employee advancement decisions.

------------------------------------------------------------------------

## Transfer

A workflow managing employee movement between organizational
assignments.

------------------------------------------------------------------------

## Contract Renewal

A workflow managing extension or continuation of employment agreements.

------------------------------------------------------------------------

## Separation

The workflow responsible for employee exit processing.

------------------------------------------------------------------------

## Exit Clearance

A process ensuring required organizational clearances are completed
before separation.

------------------------------------------------------------------------

# 23.4 Security Terms

## RBAC

Role-Based Access Control.

Authorization model where permissions are assigned according to user
roles.

------------------------------------------------------------------------

## Data Scope Control

Restriction of information visibility based on organizational
boundaries.

Example:

Campus HR Manager sees only campus employees.

------------------------------------------------------------------------

## Sensitivity Classification

Classification of information according to confidentiality level.

Examples:

-   Internal.
-   Confidential.
-   Highly Confidential.

------------------------------------------------------------------------

## Separation of Duties

Security principle preventing conflicting responsibilities from being
performed by the same actor.

------------------------------------------------------------------------

# 23.5 Integration Terms

## Event Bus

Infrastructure mechanism responsible for distributing domain events to
consumers.

------------------------------------------------------------------------

## Event Consumer

A domain or service that reacts to published business events.

------------------------------------------------------------------------

## Upstream Dependency

A domain that provides information or capabilities required by E04.

------------------------------------------------------------------------

## Downstream Consumer

A domain that consumes E04 outputs.

------------------------------------------------------------------------

## Bounded Context

A clearly defined domain boundary with its own model and ownership
rules.

------------------------------------------------------------------------

# 24. Domain Summary

## 24.1 Domain Identity

  Attribute                Description
  ------------------------ -----------------------------------
  Domain                   L2-E04 HR Workflows
  Domain Type              Workflow-Oriented Bounded Context
  Primary Responsibility   HR operational process execution
  Architecture Style       DDD + Event-Driven Architecture

------------------------------------------------------------------------

# 24.2 Core Responsibility

L2-E04 HR Workflows is responsible for executing and governing
employee-related HR processes.

The domain manages:

-   Workflow execution.
-   Approvals.
-   HR cases.
-   Employee requests.
-   Lifecycle processes.
-   HR documents workflow.
-   Compliance workflows.
-   Business audit history.

------------------------------------------------------------------------

# 24.3 Ownership Model

``` mermaid
flowchart LR

HR["E04 HR Workflows"]

Employee["E07 Employee Profile"]

Payroll["Payroll Domain"]

Organization["Organization Domain"]

Documents["Document Platform"]


HR -->|Workflow Decisions| Employee

HR -->|Business Events| Payroll

HR -->|Assignment Requests| Organization

HR -->|Document Requests| Documents
```

------------------------------------------------------------------------

# 24.4 What E04 Owns

## Business Processes

-   Onboarding.
-   Leave workflows.
-   HR cases.
-   Promotion workflows.
-   Transfer workflows.
-   Separation workflows.

## Business Governance

-   Approvals.
-   State transitions.
-   SLA tracking.
-   Escalations.
-   Audit history.

------------------------------------------------------------------------

# 24.5 What E04 Does Not Own

  Capability               Owner
  ------------------------ ----------------------
  Employee profile data    E07 Employee Profile
  Salary calculation       Payroll Domain
  Attendance records       Attendance Domain
  Organization hierarchy   Organization Domain
  Authentication           Identity Domain
  Document storage         Document Platform
  Analytics                Analytics Domain

------------------------------------------------------------------------

# 24.6 Final Architecture Principle

``` text
HR owns employee processes.

Specialized domains own specialized capabilities.

Events connect enterprise workflows.

Ownership boundaries protect scalability.
```

------------------------------------------------------------------------

# 24.7 Domain Readiness Status

  Area                 Status
  -------------------- --------
  Domain Boundary      Locked
  Aggregates           Locked
  Workflows            Locked
  Events               Locked
  Service Contracts    Locked
  Security Model       Locked
  Extension Strategy   Locked

------------------------------------------------------------------------

The L2-E04 HR Workflows architecture is ready for final governance and
repository integration.


# 25. Document Completion & Governance

## 25.1 Document Completion Status

This document defines the complete Level 2 domain workflow architecture
for:

``` text
L2-E04 — HR Workflows
```

The document establishes:

-   Domain boundaries.
-   Aggregate ownership.
-   Workflow definitions.
-   Service contracts.
-   Domain events.
-   Security model.
-   Audit requirements.
-   Scalability principles.
-   Extension strategy.

------------------------------------------------------------------------

# 25.2 Architecture Completion Checklist

  Area                              Status
  --------------------------------- -----------
  Domain Purpose Defined            Completed
  Scope Defined                     Completed
  Upstream Dependencies Defined     Completed
  Downstream Dependencies Defined   Completed
  Ownership Boundaries Defined      Completed
  Domain Model Defined              Completed
  Aggregates Defined                Completed
  Lifecycle Models Defined          Completed
  Workflows Defined                 Completed
  Workflow Specifications Defined   Completed
  Business Rules Defined            Completed
  Domain Events Defined             Completed
  Service Contracts Defined         Completed
  State Transitions Defined         Completed
  Decision Matrix Defined           Completed
  Failure Handling Defined          Completed
  Validation Rules Defined          Completed
  Authorization Defined             Completed
  Audit Requirements Defined        Completed
  Security Model Defined            Completed
  Performance Model Defined         Completed
  Extension Points Defined          Completed
  Visual Models Defined             Completed
  ADRs Defined                      Completed
  Glossary Defined                  Completed

------------------------------------------------------------------------

# 25.3 Governance Ownership

The L2-E04 architecture SHALL be governed by the SutraERP Architecture
Governance process.

Governance responsibilities:

  Responsibility            Owner
  ------------------------- -------------------------
  Domain Boundary Changes   Architecture Team
  Workflow Changes          HR Domain Owner
  Security Changes          Security Governance
  Integration Changes       Enterprise Architecture
  Event Contract Changes    Integration Governance

------------------------------------------------------------------------

# 25.4 Change Management Rules

Any modification to L2-E04 SHALL evaluate:

-   Impact on domain boundaries.
-   Impact on external integrations.
-   Impact on published events.
-   Impact on security controls.
-   Impact on audit requirements.
-   Impact on institutional configurations.

------------------------------------------------------------------------

# 25.5 Version Governance

Changes SHALL follow:

``` text
Change Request

↓

Architecture Review

↓

Impact Analysis

↓

Approval

↓

Version Update
```

------------------------------------------------------------------------

# 25.6 Architecture Review Triggers

A review SHALL be required when:

-   New HR capabilities are introduced.
-   Existing ownership boundaries change.
-   New external integrations are added.
-   New domain events are published.
-   Security model changes.
-   Workflow lifecycle changes.

------------------------------------------------------------------------

# 25.7 Documentation Maintenance

The document SHALL remain synchronized with:

-   Domain implementation.
-   Service contracts.
-   Event schemas.
-   Security policies.
-   Enterprise architecture standards.

------------------------------------------------------------------------

# 26. Appendices

# Appendix A --- Domain Ownership Reference

  Capability                     Owner
  ------------------------------ -----------------------
  HR Workflow Execution          L2-E04 HR Workflows
  Employee Profile Data           E07 Employee Profile
  Faculty Workforce Capability   E03 Faculty Workforce
  Payroll Processing             Payroll Domain
  Attendance Processing          Attendance Domain
  Organization Structure         Organization Domain
  Identity Management            Identity Domain
  Document Storage               Document Platform
  Analytics                      Analytics Domain

------------------------------------------------------------------------

# Appendix B --- Workflow Reference

  Workflow               Aggregate
  ---------------------- -------------------------------
  Employee Onboarding    Employee Onboarding Aggregate
  Leave Management       Leave Request Aggregate
  HR Cases               HR Case Aggregate
  Promotion              Employment Change Aggregate
  Transfer               Employment Change Aggregate
  Certificate Requests   Certificate Request Aggregate
  Contract Renewal       Employment Change Aggregate
  Separation             Separation Aggregate
  Compliance             Compliance Case Aggregate

------------------------------------------------------------------------

# Appendix C --- Event Reference

  Event Category       Examples
  -------------------- ------------------------------------------------
  Employee Lifecycle   EmployeeOnboardingCompleted, EmployeeSeparated
  Leave                LeaveApproved, LeaveCancelled
  Employment Change    PromotionApproved, EmployeeTransferred
  Certificates         CertificateGenerated
  HR Cases             HRCaseResolved
  Compliance           ComplianceCompleted

------------------------------------------------------------------------

# Appendix D --- Integration Reference

  Integration         Pattern
  ------------------- ----------------------------
  Employee Profile    Domain Events
  Payroll             Domain Events
  Organization        Service Contracts + Events
  Attendance          Service Contracts + Events
  Identity            Service Contracts
  Document Platform   Service Contracts
  Analytics           Event Consumption

------------------------------------------------------------------------

# Appendix E --- Final Architecture Principles

The L2-E04 HR Workflows domain follows these principles:

## Principle 1 --- Process Ownership

``` text
HR owns business processes.

Specialized domains own specialized capabilities.
```

------------------------------------------------------------------------

## Principle 2 --- Data Ownership

``` text
No duplicate ownership of enterprise data.
```

------------------------------------------------------------------------

## Principle 3 --- Event Communication

``` text
Business outcomes are communicated through domain events.
```

------------------------------------------------------------------------

## Principle 4 --- Security

``` text
HR access requires role, scope, sensitivity, and authority validation.
```

------------------------------------------------------------------------

## Principle 5 --- Extensibility

``` text
Future capabilities integrate through contracts and events.
```

------------------------------------------------------------------------

# Appendix F --- Final Domain Statement

``` text
L2-E04 HR Workflows is a workflow-oriented bounded context responsible for executing, governing, and coordinating HR operational processes across SutraERP while maintaining strict ownership boundaries with employee, payroll, organization, identity, document, and analytics domains.
```

------------------------------------------------------------------------

# Document Final Status

  Attribute                    Status
  ---------------------------- -----------
  Architecture Discovery       Completed
  Architecture Decisions       Locked
  Workflow Design              Locked
  Integration Model            Locked
  Security Model               Locked
  Governance Model             Defined
  Ready for Repository Merge   Yes


