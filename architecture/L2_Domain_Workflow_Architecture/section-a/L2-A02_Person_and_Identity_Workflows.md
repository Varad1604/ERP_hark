# L2-A02 — Person & Identity Workflows

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | Purpose |
| Scope | Scope; Out of Scope; Authorization Scope |
| Actors | Person; User Account administrator; downstream identity-reference consumers |
| Entities | Canonical Entity Relationships |
| Business Rules | Business Rules; Lifecycle Business Rules; Security Business Rules |
| Workflow | L2-A02 — Person & Identity Workflows; Authentication Workflow; Context Resolution Workflow |
| State Machine | Lifecycle State Machine |
| Contracts | Domain Events & Integration Contracts; Integration Contracts; Person Service Contracts |
| Events | Published Events; Domain Events & Integration Contracts; Event Architecture |
| Dependencies | Dependency Direction; Context Resolution Dependencies; Dependency Rules |
| Locked Decisions | Authorization Decision Outcomes; Workflow 4 — Authorization Decision; Architecture Decision Records (ADRs) |

---

## Canonical Ownership Amendment — INT-301

> **Approved and applied 2026-08-13.** The Enterprise Architecture decision for `INT-301` assigns exactly one canonical owner to the previously duplicated security concepts. This amendment is authoritative over any later legacy wording in this document that has not yet been physically removed.

| Concept | Canonical owner | A02 relationship |
|---|---|---|
| Person, Identity, User Account | L2-A02 — Person & Identity | Owns identity and account lifecycles. |
| Identity and active-context references | L2-A02 — Person & Identity | Publishes stable references for consumers. |
| Credential, Authentication Session, authentication factors/MFA enrollment, Authentication Provider | L2-A03 — Authentication | Consumes A02 identity/account references; A02 never creates or governs these lifecycles. |
| Role, Role Assignment, Delegation, authorization-policy lifecycle | L2-A04 — Authorization & Policy | References A02 Person/Identity identifiers; A02 never grants or governs authorization. |

Any later A02 diagram, table, workflow, service, or prose that describes a Credential, Authentication Session, authentication factor/MFA enrollment, Authentication Provider, Role, Role Assignment, Delegation, or authorization-policy lifecycle as A02-owned is superseded and shall be read only as non-authoritative integration context. A03 and A04 published contracts govern those concepts.

---

## Canonical Person–Identity Establishment and Activation Sequencing Decision — 2026-08-20

> **Approved by the Lead Architect on 2026-08-20.** This decision resolves the conflict between the legacy Activation Workflow and the User Account Service Contracts. It is authoritative over later legacy wording that places User Account creation, initial-context assignment, or authentication enablement before successful Person and Identity activation.

The canonical establishment and activation order is:

1. create the canonical Person and publish `PersonCreated`;
2. create the associated Identity in `Pending` and publish `IdentityCreated`;
3. apply the applicable Tenant verification policy and validate the required evidence;
4. transition the Person from `Created` to `Verified` and publish `PersonVerified`;
5. transition the Identity from `Pending` to `Verified` and publish `IdentityVerified`;
6. transition the Person from `Verified` to `Active` and publish `PersonActivated`;
7. transition the Identity from `Verified` to `Active` and publish `IdentityActivated`; and
8. only after successful Person and Identity activation, create a User Account where login capability is required.

Person and Identity activation form one governed institutional outcome. The outcome is incomplete unless both aggregates reach their authoritative `Active` state.

User Account creation is a post-activation A02 lifecycle action. It is not a prerequisite for Person or Identity activation, and a Person may remain active without a User Account.

Credential creation, authentication-factor or MFA enrollment, Authentication Provider lifecycle, Authentication Session lifecycle, and authentication enablement remain owned by L2-A03. Role, Role Assignment, Delegation, authorization-policy lifecycle, and initial authorization-context assignment remain owned by L2-A04.

This decision does not rename or re-own any lifecycle state, event, aggregate, or contract.

---

# Document Information

| Attribute | Value |
|-----------|-------|
| Document ID | L2-A02 |
| Document Owner | Platform Foundation — Identity |
| Domain | Platform Foundation |
| Module | Person & Identity |
| Architecture Level | Level 2 |
| Document Type | Canonical Domain Workflow Specification |
| Owner | Platform Foundation — Identity |
| Status | Locked |
| Version | 1.0 |
| Depends On | L2-A01 — Platform & Tenant Foundation |
| Followed By | All Business Domains |
| Canonical Aggregate | Person |
| Secondary Aggregate | Identity |
| Last Updated | 2026-08-20 — Person–Identity establishment and activation sequencing decision |

---

# Purpose

This document defines the canonical architecture, lifecycle, ownership rules, workflows, business rules, events, state transitions, and integration contracts for the Person & Identity domain within SutraERP.

The Person & Identity domain serves as the single source of truth for every human and system identity that interacts with the platform. It establishes a unified identity model that is shared across all institutions, campuses, organizations, and business domains while maintaining strict tenant isolation and canonical ownership.

This specification ensures that every downstream module references a consistent identity model without creating duplicate person records or conflicting identity information.

---

# Architectural Position

Within the SutraERP architecture, the Person & Identity domain is a Platform Domain.

Unlike business domains such as Admissions, Examination, Finance, or Hostel, this domain provides foundational identity services that are consumed by nearly every module across the platform.

The domain owns:

- Person lifecycle
- Identity lifecycle
- User account lifecycle
- Authentication identity references supplied to L2-A03
- Active Context Resolution References
- Identity resolution
- Duplicate detection
- Identity merge operations
- Identity audit trail
- Workspace reference fields

The domain does **not** own business-specific information such as admissions, enrollments, examinations, payroll, finance, hostel allocations, or library memberships. Those remain under their respective canonical domains.

---

# Scope

This document covers:

- Person Aggregate
- Identity Aggregate
- User Account
- Authentication Identity
- Context Reference
- Person Lifecycle
- Identity Lifecycle
- Context Resolution References
- Identity Resolution
- Duplicate Handling
- Merge & Split Operations
- Security Policies
- Authentication identity-reference contracts
- Multi-Domain Identity Events
- Identity Audit
- Identity Governance

---

# Out of Scope

The following capabilities are intentionally excluded from this domain:

- Student Admission Management
- Academic Registration
- Examination Processing
- Fee Management
- Human Resource Management
- Payroll
- Library Operations
- Hostel Management
- Transport Management
- Timetable Management
- Academic Records
- Learning Management
- Finance Operations
- Workspace ownership
- Workspace lifecycle
- Workspace switching

These domains consume Person entities but remain the canonical owners of their respective business data.

---

# Canonical Ownership

The Person & Identity domain is the exclusive owner of:

- Person
- Identity
- User Account
- Context Reference
- Identity Status
- Account Status
- Identity Events

No other domain may create, directly modify, or delete these aggregates.

Business domains may only reference Person identifiers through published contracts and events.

---

# Architectural Principles

This document inherits all platform-wide architectural principles defined in:

- L2-A01 — Platform & Tenant Foundation
- Architecture Workflow Context Baseline
- Dependency Map
- Architecture Index

Unless explicitly overridden within this document, all architectural rules, governance standards, dependency constraints, layering principles, and platform contracts defined in those documents remain applicable.

This document specifies only the additional rules that are unique to the Person & Identity domain.

---

# Domain Vision

The Person & Identity domain establishes the canonical digital identity framework for every human and system actor interacting with the SutraERP platform.

Its primary objective is to ensure that every individual is represented by a single canonical Person record within a Tenant while allowing that Person to participate across multiple institutions, campuses, departments, programs, and business domains through business-domain associations and authorization references governed by L2-A04.

The domain eliminates identity duplication, enforces centralized identity governance, supplies canonical identity and account references to L2-A03 and L2-A04, and provides a stable identity foundation for every downstream business capability.

Every business domain within SutraERP references the Person aggregate but never owns it.

---

# Domain Objectives

The Person & Identity domain is designed to achieve the following objectives:

- Maintain a single canonical Person record per individual within a Tenant.
- Prevent duplicate identity creation wherever possible.
- Support multi-institution identity sharing within a Tenant.
- Support multiple simultaneous roles for a single Person.
- Separate identity management from business operations.
- Supply canonical identity and account context to the separately owned authentication and authorization domains.
- Maintain complete identity auditability.
- Enable configurable security policies.
- Support future identity federation standards.
- Provide immutable identity references for all downstream domains.

---

# Core Responsibilities

The Person & Identity domain is responsible for:

- Person lifecycle management
- Identity lifecycle management
- User account management
- Authentication identity
- Context resolution
- Organizational context consumption
- Authentication identity-reference publication
- Identity verification
- Duplicate detection
- Identity merge operations
- Identity archival
- Identity anonymization
- Identity deletion approval workflows
- Active-context reference ownership
- Identity audit trail
- Multi-domain identity references

---

# Non-Responsibilities

The following capabilities are intentionally excluded from this domain.

These belong to their respective canonical business domains.

- Student Admissions
- Academic Registration
- Student Academic Records
- Examination Management
- Fee Collection
- Finance & Accounting
- Human Resources
- Payroll
- Hostel Allocation
- Library Membership
- Transport Allocation
- Learning Management
- Timetable Management
- Inventory Management

The Person & Identity domain only provides canonical identity references that these domains consume.

---

# Domain Design Principles

The domain follows the architectural principles inherited from L2-A01 while introducing the following identity-specific principles.

## Principle 1 — One Person, One Canonical Identity

Within a Tenant, every real human shall have exactly one canonical Person record.

Business domains shall never create independent person entities.

---

## Principle 2 — Separation of Identity and Business Data

Identity information shall remain independent of business information.

Examples:

- A student's admission belongs to Admissions.
- A faculty appointment belongs to HR.
- A hostel allocation belongs to Hostel.
- A library membership belongs to Library.

Only the Person entity belongs to this domain.

---

## Principle 3 — Identity Before Business

Every business process requiring a human participant shall first reference a valid Person.

No business workflow may operate using orphan identity information.

---

## Principle 4 — Tenant Isolation

Person identities are unique only within the boundaries of a Tenant.

No Person record shall ever be shared across different Tenants.

---

## Principle 5 — Multiple Roles

A Person may simultaneously perform multiple responsibilities.

Examples include:

- Faculty + Parent
- Student + Employee
- Alumni + Donor
- Trustee + Faculty
- Contractor + Auditor

Roles are additive rather than mutually exclusive.

---

## Principle 6 — Scoped Identity References

Scoped identity context references may be stored for downstream authorization evaluation.

Authorization scope is owned by L2-A04 and determines where a role is effective.

Supported scopes include:

- Tenant
- Institution
- Campus
- Department
- Program
- Batch
- Division
- Hostel
- Library
- Committee
- Project
- Custom Organizational Unit

---

## Principle 7 — Immutable Identity

Once created, the canonical Person identifier shall never change.

Names, contact information, login credentials, and business-domain association may change throughout the lifecycle, but the Person identifier remains permanent.

---

## Principle 8 — Complete Auditability

Every identity creation, modification, authentication event, authorization change, business association changes, archive, anonymization, merge, split, and deletion approval shall be permanently audited.

---

# Ubiquitous Language

The following terminology is used consistently throughout the SutraERP architecture.

| Term | Definition |
|------|------------|
| Person | A real human represented within the Tenant. |
| Identity | The digital representation of a Person used for authentication and authorization. |
| User Account | A login-enabled account associated with a Person. |
| Tenant | The highest isolation boundary within the platform. |
| Institution | An educational institution operating under a Tenant. |
| Authentication | Verification of user identity. |
| Session | A temporary authenticated interaction between a user and the platform. |
| Identity Provider | The component responsible for validating identities. |
| Duplicate Person | Multiple Person records representing the same real human. |
| Merge | Consolidation of duplicate Person records into a single canonical Person. |
| Archive | Permanent deactivation while preserving historical records. |
| Anonymization | Removal or masking of personally identifiable information while preserving business history. |

---

# Domain Success Criteria

The Person & Identity domain shall be considered architecturally successful when:

- Every human within a Tenant has a single canonical Person record.
- No downstream domain owns Person data.
- Duplicate identities are minimized and governed.
- Identity references remain consistent across scoped downstream domains.
- Identity history remains fully auditable.
- Business domains consume Person references without duplicating identity information.
- Future authentication standards can be integrated without redesigning the domain.
- Tenant isolation is preserved at all times.

---

# Domain Model

The Person & Identity domain follows Domain-Driven Design (DDD) principles by establishing clear aggregate boundaries, ownership rules, and lifecycle responsibilities.

This domain serves as the canonical identity provider for every business domain within SutraERP.

Every downstream domain references the Person aggregate through immutable identifiers rather than duplicating identity information.

---

# Bounded Context

The Person & Identity bounded context encapsulates every capability related to identity ownership, identity references, and user lifecycle.

The boundary ends where business-specific information begins.

```
                        Person & Identity Domain

+---------------------------------------------------------------+

    Person
        │
        ├──────── Identity
        │
        ├──────── User Account
        │
        ├──────── Authentication Reference
        │
        ├──────── Identity Verification
        │
        └──────── Audit Trail

+---------------------------------------------------------------+

Business Domains

Admissions
HR
Finance
Hostel
Library
Examination
Academics
Transport
LMS

↓

Reference PersonID only

```

Business domains shall never store duplicate Person entities.

---

# Canonical Aggregates

The following aggregates are owned exclusively by this domain.

| Aggregate | Ownership | Purpose |
|------------|-----------|---------|
| Person | Canonical | Represents a real human within a Tenant |
| Identity | Canonical | Digital identity associated with a Person |
| User Account | Canonical | Login-enabled account |
| Context Reference | Canonical | Scoped downstream identity reference data |
| Identity Verification | Canonical | Verification status and methods |

No downstream module may create or own these A02 aggregates. Credentials, authentication sessions, authentication factors/MFA enrollment and authentication providers are A03 aggregates; roles, role assignments, delegations and authorization policies are A04 aggregates.

---

# Aggregate Relationships

```
                    Person
                       │
     ┌─────────────────┼──────────────────┐
     │                 │                  │
     ▼                 ▼                  ▼

 Identity      User Account      Context References
     │                 │                  │
     │                 │                  │
     ▼                 ▼                  ▼

 A03 Credential/Session Reference     A04 Role/Assignment Reference
                 │                                  │
                 ▼                                  ▼

          Authentication Context            Authorization Context

```

The Person aggregate is the aggregate root for identity ownership.

All other aggregates ultimately reference a single Person.

---

# Aggregate Ownership Rules

## Person

Owns:

- Personal identity
- Core demographic profile
- Permanent identifier
- Lifecycle status

Does NOT own:

- Student admissions
- Employee records
- Academic information
- Financial information
- Library information
- Hostel information

---

## Identity

Owns:

- Login identity
- Authentication identity
- Verification state
- Identity status

Does NOT own:

- Business permissions
- Academic records

---

## User Account

Owns:

- Login enablement
- Username
- Login status
- Lock status
- Identity/account eligibility supplied to authentication

Does NOT own:

- Person demographics

---

## Role Assignment Reference

A02 publishes stable Person and Identity references that L2-A04 uses in its canonical Role Assignment aggregate. A02 does not create, approve, activate, suspend, expire, revoke, or otherwise govern Role Assignment or Delegation.

---

## Authentication Session Reference

A02 supplies canonical Identity and User Account references to L2-A03. L2-A03 exclusively owns Authentication Session creation, state, timestamps, device and network context, activity, expiry, termination and revocation.

---

# Canonical Entity Relationships

```
Person

1

↓

1 Identity

↓

1

↓

1 User Accounts

↓

1

↓

N Sessions

--------------------------------

Person

↓

1

↓

Role Assignments

↓

1

↓

N Role Assignments

```

A Person may have:


- Multiple roles
- Multiple sessions
- Multiple authentication methods

But only one canonical Person record.

---

# Aggregate Invariants

The following rules shall always remain true.

### Person

- Exactly one Person exists for a real human within a Tenant.
- PersonID is immutable.
- Person cannot exist without a Tenant.

---

### Identity

- Identity belongs to exactly one Person.
- Identity cannot exist independently.
- Identity status follows Person lifecycle.

---

### User Account

- User Account belongs to one Person.
- Username must be unique within a Tenant.
- Locked accounts cannot authenticate.

---

### Role Assignment Reference

- A referenced Role Assignment must resolve to the canonical A04 aggregate.
- A02 Person and Identity state may be consumed during authorization but does not grant permissions.

---

### Authentication Session Reference

- A referenced Authentication Session must resolve to the canonical A03 aggregate.
- A02 cannot activate, expire, terminate, or revoke an Authentication Session.

---

# Domain Services

The Person & Identity domain exposes the following logical services.

- Person Service
- Identity Service
- User Account Service
- Context Reference Service
- Authentication Reference Service
- Authorization Reference Service
- Session Management Service
- MFA Service
- Identity Verification Service
- Duplicate Detection Service
- Identity Merge Service
- Identity Archive Service
- Identity Audit Service

These services encapsulate business logic and prevent direct manipulation of aggregates by external domains.

---

# Architectural Constraints

The following constraints are mandatory.

- Person is the root identity aggregate.
- Every aggregate references Person through immutable PersonID.
- Downstream domains consume Person references only.
- Multi-domain writes into Person aggregates are prohibited.
- Authorization decisions shall be scope-aware.
- Aggregate invariants shall never be bypassed.
- Identity integrity takes precedence over business convenience.

---

# Person Aggregate

The Person aggregate is the canonical aggregate root of the Person & Identity domain.

Every human represented within a Tenant is modelled as exactly one Person.

Business domains never create independent person records.

Instead, they reference the canonical Person through immutable PersonID.

The Person aggregate exists independently of any business process.

A Person may later become:

- Applicant
- Student
- Parent
- Guardian
- Faculty
- Employee
- Alumni
- Trustee
- Donor
- Contractor
- Visitor
- Sponsor
- Auditor
- External Examiner
- Patient
- Resident

These are business roles and not separate Person entities.

---

# Aggregate Responsibility

The Person aggregate owns:

- Canonical identity
- Core demographic information
- Identity lifecycle
- Identity status
- Legal identity attributes
- Communication preferences
- Privacy preferences
- Permanent identifiers

The Person aggregate does NOT own:

- Admissions
- Employment
- Examination
- Finance
- Hostel
- Library
- Academic Records
- Payroll
- Attendance

Those belong to their respective domains.

---

# Aggregate Root

```

                    Person
                       │
        ┌──────────────┼───────────────┐
        │              │               │
        ▼              ▼               ▼

   Identity      User Account     Preferences

                       │

                       ▼

                  Authentication

                       │

                       ▼

                  Authorization

```

The Person aggregate acts as the single source of truth for human identity.

---

# Person Identifier

Every Person receives two identifiers.

## Internal Identifier

Example

```
UUID

9ab13aef-7cb1-45d8-9ab3-38f7aa928fd2
```

Characteristics

- Immutable
- Globally unique
- Used internally
- Never exposed to business users

---

## Business Identifier

Example

```
PER-000001245
```

Characteristics

- Human readable
- Displayed in ERP
- Searchable
- Never reused
- Immutable

---

# Person Attributes

## Identity Information

- First Name
- Middle Name
- Last Name
- Preferred Name
- Full Name
- Display Name

---

## Demographics

- Date of Birth
- Gender
- Nationality
- Marital Status

---

## Contact Information

- Mobile Numbers
- Email Addresses
- Emergency Contact

---

## Address

- Permanent Address
- Current Address
- Postal Address

---

## Government Identity

Configurable.

Examples

- Aadhaar
- PAN
- Passport
- Driving Licence
- Employee Number
- ERP Number

---

## Communication Preferences

- Preferred Language
- Preferred Notification Channel
- Time Zone

---

## Privacy Preferences

- Profile Visibility
- Contact Visibility
- Data Sharing Consent
- Communication Consent

---

# Value Objects

The following are immutable Value Objects.

- Person Name
- Address
- Email
- Mobile Number
- Government Identifier
- Emergency Contact

Replacing a Value Object creates a new instance.

Existing audit history remains unchanged.

---

# Person Lifecycle

```
Draft

↓

Created

↓

Verified

↓

Active

↓

Inactive

↓

Archived

↓

Anonymized

↓

Deleted
```

---

# Lifecycle Description

## Draft

Temporary record.

Not visible to business domains.

---

## Created

Canonical Person exists.

No login yet.

---

## Verified

Identity verified through approved verification process.

---

## Active

Person available for all downstream business domains.

---

## Inactive

Temporarily unavailable.

Existing history preserved.

---

## Archived

No active participation.

Business history preserved forever.

---

## Anonymized

Personally identifiable information removed or masked where legally permitted.

Business transactions remain intact.

---

## Deleted

Exceptional state.

Requires approval workflow.

Allowed only when:

- Legal retention satisfied
- Dependency validation passed
- Administrative approval completed

---

# Lifecycle Invariants

The following rules always apply.

A Person

- belongs to exactly one Tenant.
- owns one immutable PersonID.
- may have multiple business roles.
- owns exactly one User Account per Tenant.
- may exist without any login account.
- may exist before admissions.
- may exist before employment.
- can never migrate across Tenants.

---

# Validation Rules

The Person aggregate validates:

- Mandatory attributes
- Name consistency
- Duplicate indicators
- Mobile uniqueness policy
- Username uniqueness
- Government identifier policy
- Required demographic information

Validation rules remain configurable per Tenant.

---

# Duplicate Detection

Possible duplicate indicators include:

- Same mobile number
- Same email
- Same Aadhaar
- Same passport
- High name similarity
- Same date of birth
- Same guardian information

Detection produces warnings.

Automatic blocking is not performed.

Authorized users may override warnings.

Every override is audited.

---

# Person Merge

Duplicate Persons may be merged.

```
Person A

+

Person B

↓

Canonical Person

↓

Redirect obsolete references

↓

Preserve audit history

```

Merge operations never destroy historical information.

---

# Person Split

Rare administrative operation.

Applicable when:

- Incorrect merge occurred
- Identity fraud detected
- Data migration mistake

Split requires administrative approval.

Complete audit history is preserved.

---

# Business Rules

The Person aggregate follows these mandatory rules.

- One canonical Person per Tenant.
- Immutable identifiers.
- Business domains reference PersonID only.
- No duplicate Person creation without warning.
- No multi-domain ownership.
- Full auditability.
- Tenant isolation enforced.
- Archive before deletion.
- Deletion requires approval.

---

# Published Events

The Person aggregate publishes:

- PersonCreated
- PersonVerified
- PersonUpdated
- PersonActivated
- PersonInactivated
- PersonArchived
- PersonAnonymized
- PersonDeleted
- PersonMerged
- PersonSplit

Business domains subscribe to these events.

No downstream domain directly modifies the Person aggregate.

---

# Identity Aggregate

The Identity aggregate represents the digital identity of a Person within a Tenant.

While the Person aggregate represents the real human, the Identity aggregate represents how that human interacts with the SutraERP platform.

Every Identity belongs to exactly one Person.

An Identity cannot exist independently.

---

# Aggregate Responsibility

The Identity aggregate owns:

- Canonical digital identity
- Identity lifecycle
- Identity verification state
- Identity activation state
- Identity linkage to Person

The Identity aggregate does not own:

- User Account lifecycle and login identifiers
- Authentication Sessions
- Credentials
- Authentication factors/MFA enrollment
- Authentication Provider lifecycle
- Roles, Role Assignments, Delegations, or authorization policy

User Account lifecycle and login identifiers belong to A02. Authentication concepts belong to L2-A03 and authorization concepts belong to L2-A04 under the `INT-301` ownership amendment.


---

# Aggregate Relationship

```
Tenant

│

└────────────── Person

│

└────────────── Identity

│

└────────────── User Account

│

├──────── Username / Login Identifier

├──────── Account Status

├──────── A03 Authentication References

└──────── A04 Authorization References
```

---

# Identity Identifier

Each Identity owns:

- Internal IdentityID (UUID)
- Identity Number (Readable)

Example

```
IDN-000000482
```

IdentityID remains immutable throughout the lifecycle.

---

# User Account Model

Each Person owns exactly one User Account within a Tenant.
The User Account is the canonical account record referenced by authentication and authorization.

It owns:

- Username
- Account Lock Status
- Login Enablement
- Account Lifecycle

The User Account does not own the Person or Identity lifecycle.

The User Account provides a stable account reference. L2-A03 owns authentication methods, credentials, factors, providers, and sessions associated with that reference.

Supported authentication methods include:

- Username
- Mobile Number
- Password

Future extensions may include:

- OAuth2
- OpenID Connect
- SAML
- Enterprise Identity Providers

without changing the domain model.

It may reference:

- Available Workspaces
- Preferred Default Workspace

These references are informational only and do not create ownership, lifecycle control, or authorization authority.

---

# User Account Attributes

## Account Information

- Account ID
- Username
- Display Username
- Account Status
- Login Enabled

---

## Authentication References

- Credential Reference (opaque A03 identifier, where required)
- Authentication Provider Reference (opaque A03 identifier, where required)
- Authentication Eligibility Context

---

## Authentication Factor References

- MFA Enrollment Reference (opaque A03 identifier, where required)
- Authentication Assurance Reference

A02 does not store factor secrets, backup codes, recovery mechanisms, or factor lifecycle state.

---

## Security

- Failed Login Count
- Lock Status
- Lock Timestamp
- Unlock Timestamp

---

## Session Policy References

- Authentication Policy Reference
- Tenant Security Configuration Reference

Session limits, timeout, trusted-device behavior and session lifecycle remain owned by L2-A03.

---

# Login Identifiers

The following identifiers may authenticate a User Account.

## Username

Characteristics

- Unique within Tenant
- Immutable after creation unless authorized
- Human readable

---

## Mobile Number

Characteristics

- Configurable uniqueness
- Verified before use
- Can act as login identifier

---

## Credential Boundary

Passwords and all other credentials are authentication proofs, not login identifiers. Their storage, hashing, versioning, history and lifecycle are owned exclusively by L2-A03.

---

# Identity States

```
Pending

↓

Verified

↓

Active

↓

Locked

↓

Suspended

↓

Disabled

↓

Archived
```

---

# State Definitions

## Pending

Identity created.

Verification pending.

---

## Verified

Identity successfully verified.

Authentication permitted.

---

## Active

Fully operational.

Normal authentication allowed.

---

## Locked

Automatically locked.

Reasons include:

- Failed login attempts
- Security policy
- Administrator action

---

## Suspended

Temporary administrative restriction.

Authentication denied.

Historical information preserved.

---

## Disabled

Login permanently disabled.

Usually follows Person archival.

---

## Archived

Identity retained for history.

No authentication permitted.

---

# Account States

```
Created

↓

Enabled

↓

Locked

↓

Disabled

↓

Archived
```

---

# Password Policy

The platform supports configurable password policies.

Examples include:

- Minimum length
- Complexity
- Expiration
- Password reuse prevention
- Maximum failed attempts

Policies are configurable at Tenant level.

---

# Authentication Rules

Authentication requires:

- Valid Identity
- Active User Account
- Valid Password
- Successful MFA (when required)
- Active Person status

Failure of any requirement rejects authentication.

---

# Lock Rules

Accounts may become locked due to:

- Excessive failed logins
- Administrative action
- Security incident
- Suspicious authentication activity

Unlocking may occur through:

- Administrator
- Time-based unlock
- Verified recovery process

---

# Login History

Every successful and failed authentication generates an immutable audit record.

Captured information includes:

- Timestamp
- Device
- Browser
- Operating System
- IP Address
- Geographic Region (if available)
- Success/Failure
- Failure Reason

---

# Authentication Session Reference *(legacy integration context; A03 owns the session)*

Each authenticated User Account may be referenced by multiple concurrent Authentication Sessions owned by L2-A03.

Each session records:

- Login Time
- Last Activity
- Device Identifier
- Session Expiry
- Logout Time
- Logout Reason

Inactive sessions automatically expire according to Tenant policy.

---

# Security Rules

The Identity aggregate enforces:

- Immutable identity ownership
- One User Account per Person
- Secure password storage
- MFA enforcement
- Session timeout
- Device validation
- Login auditing
- Identity verification

---

# Published Events

The Identity aggregate publishes:

- IdentityCreated
- IdentityVerified
- UserAccountCreated
- UserAccountEnabled
- UserAccountLocked
- UserAccountUnlocked
- PasswordChanged
- MFAEnabled
- MFADisabled
- UserAuthenticated
- PersonAuthenticationFailed
- SessionStarted
- PersonSessionExpired
- SessionEnded

Downstream domains consume these events without directly modifying Identity.

---

# Authentication & Authorization *(legacy integration reference; normative ownership is A03/A04)*

> **INT-301 boundary:** Sections from this point that describe authentication or authorization models document how A02 identity/account context is consumed. They are not canonical aggregate, lifecycle, policy, command, event, service, or repository definitions. L2-A03 governs authentication; L2-A04 governs authorization. Where this legacy reference prose says that A02, Identity, or User Account “owns”, “creates”, “publishes”, “assigns”, “grants”, or “manages” an A03/A04 concept, the statement is superseded by the Canonical Ownership Amendment near the start of this document.

Authentication and Authorization are independent architectural capabilities.

Authentication establishes the identity of a user.

Authorization determines the actions that an authenticated identity may perform within a specific organizational scope.

Authentication must always complete successfully before authorization evaluation begins.

---

# Authentication Architecture

Authentication verifies that a login request belongs to a valid Person.

Successful authentication establishes a trusted security context for the current session.

Authentication does not determine business permissions.

Those decisions belong exclusively to Authorization.

---

# Authentication Components

The authentication subsystem consists of:

- Login Service
- Password Validator
- Identity Verification
- MFA Engine
- Session Manager
- Security Policy Engine
- Login Audit Service

Future extensions may include:

- OAuth2
- OpenID Connect
- SAML
- Enterprise Identity Providers
- API Authentication

without changing the domain architecture.

---

# Authentication Workflow

```

User

↓

Enter Username / Mobile

↓

Locate User Account

↓

Verify Account Status

↓

Verify Password

↓

Evaluate MFA Policy

↓

MFA Verification (if required)

↓

Create Session

↓

Publish Authentication Event

↓

Return Security Context

```

---

# Authentication Preconditions

Authentication proceeds only if:

- Person exists.
- Identity exists.
- User Account exists.
- Person is Active.
- Identity is Active.
- Account is Enabled.
- Password is valid.
- MFA requirements are satisfied.

Failure of any validation terminates authentication.

---

# Authentication Failure Reasons

Authentication may fail because of:

- Invalid username
- Invalid password
- Invalid mobile number
- Account disabled
- Account locked
- Person archived
- Identity suspended
- MFA failed
- Session policy violation
- Tenant mismatch

Every failure shall be audited.

---

# Multi-Factor Authentication

MFA policies are configurable.

Recommended defaults:

| User Category | MFA |
|--------------|-----|
| Student | Optional |
| Faculty | Required |
| Staff | Required |
| Administrator | Mandatory |
| Super Administrator | Mandatory |

Supported factors include:

- OTP
- Authenticator Application
- Backup Codes

The architecture allows future authentication factors without structural changes.

---

# Session Creation

Successful authentication creates exactly one authenticated session.

The session contains:

- Session Identifier
- PersonID
- User Account ID
- Tenant ID
- Active Institution
- Active Scope
- Authentication Timestamp
- Last Activity Timestamp
- Device Information

---

# Session Timeout

Sessions terminate automatically after configurable inactivity.

The timeout policy is configurable per Tenant.

Examples:

- Student Portal
- Faculty ERP
- Administrative Console

may each define different timeout policies.

---

# Concurrent Sessions

A User Account may own multiple concurrent sessions.

Each session is independently managed.

Administrative policies may limit:

- Maximum devices
- Maximum active sessions
- Geographic restrictions

---

# Authorization Architecture

Authorization begins only after successful authentication.

Authorization evaluates whether an authenticated identity may perform a requested action.

Authorization decisions are always scope-aware.

---

# Authorization Model

Authorization follows Role-Based Access Control (RBAC) with scoped permissions.

Permission =

Role

+

Scope

+

Permission Set

+

Policy Evaluation

---

# Authorization Scope

Permissions may be assigned at:

- Tenant
- Institution
- Campus
- Department
- Program
- Batch
- Division
- Hostel
- Library
- Committee
- Project
- Custom Organizational Unit

Authorization never assumes global permissions.

---

# Permission Evaluation

Every authorization request evaluates:

```

Authenticated Person

↓

Active Role

↓

Organizational Scope

↓

Permission

↓

Policy Rules

↓

Access Decision

```

---

# Permission Types

Permissions may include:

- View
- Create
- Update
- Delete
- Approve
- Reject
- Publish
- Archive
- Restore
- Export
- Import
- Execute
- Configure

Additional permission types may be introduced without changing the authorization architecture.

---

# Permission Inheritance

Authorization supports hierarchical inheritance.

Example

Tenant Administrator

↓

Institution Administrator

↓   

Campus Administrator

↓

Department Administrator

↓

Faculty

↓

Student

Inheritance may be overridden by explicit deny rules where configured.

---

# Role Assignment

Roles are assigned to Persons through scoped authorization assignments.

A role assignment contains:

- Person
- Role
- Scope
- Effective Date
- Expiry Date
- Status
- Assignment Authority

Expired assignments automatically become inactive.

---

# Authorization Rules

Authorization decisions shall consider:

- Active Person
- Active Identity
- Active User Account
- Active Role
- Valid Organizational Scope
- Effective Assignment Dates
- Permission Availability
- Security Policies

Failure of any rule results in access denial.

---

# Authorization Decision Outcomes

Possible outcomes include:

- Allow
- Deny
- Allow with Restrictions
- Escalate for Approval
- Additional Authentication Required

---

# Security Context

Every authenticated request carries a Security Context containing:

- PersonID
- TenantID
- InstitutionID
- Active Scope
- Assigned Roles
- Permission Set
- SessionID
- Authentication Level

Business domains consume the Security Context rather than implementing independent authorization logic.

---

# Published Events

The Authentication & Authorization subsystem publishes:

- AuthenticationStarted
- AuthenticationSucceeded
- PersonAuthenticationFailed
- PersonSessionCreated
- PersonSessionExpired
- SessionTerminated
- RoleAssigned
- RoleRevoked
- PermissionGranted
- PermissionRevoked
- AuthorizationSucceeded
- AuthorizationDenied

Downstream domains consume these events for auditing and workflow orchestration.

---

# Architectural Constraints

The following constraints are mandatory.

- Authentication shall always precede Authorization.
- Authorization decisions shall always be scope-aware.
- Business domains shall never implement independent authentication.
- Business domains shall never duplicate authorization logic.
- Every access decision shall be traceable through audit records.
- Security policies shall remain configurable at the Tenant level.
- Future authentication standards shall integrate without redesigning the domain.

---

# Context Reference Management

Context Reference Management defines how identity-related references are stored for downstream authorization evaluation without owning roles or permissions.

Context references describe downstream identity associations.

Authorization, roles, permissions, and scopes remain owned by L2-A04.

---

# Authorization Hierarchy

The authorization hierarchy is structured as follows.

```

Permission

↓

Permission Group

↓

Role

↓

Role Assignment

↓

Person

↓

Security Context

↓

Business Operation

```

This layered model minimizes duplication while supporting highly configurable authorization.

---

# Permission

A Permission represents the smallest unit of authorization.

Examples include:

- View Student
- Create Admission
- Update Admission
- Approve Admission
- Cancel Admission
- Generate Hall Ticket
- Approve Fees
- Publish Results
- Manage Timetable
- Manage Hostel Allocation
- Create Employee
- Process Payroll

Permissions are immutable identifiers.

---

# Permission Categories

Permissions are organized into logical categories.

Examples:

- Admissions
- Student Lifecycle
- Academics
- Examination
- Finance
- Library
- Hostel
- Transport
- HR
- Payroll
- Inventory
- Reports
- Administration
- Configuration
- Audit

Categories improve administration but do not affect authorization logic.

---

# Permission Groups

Permission Groups combine related permissions.

Example

```
Admission Operator

↓

Permissions

• Create Admission

• Update Admission

• View Admission

• Print Admission Forms
```

Permission Groups simplify role maintenance.

---

# Role

A Role represents a business responsibility.

Examples include:

- Student
- Applicant
- Faculty
- HOD
- Principal
- Registrar
- Finance Officer
- Librarian
- Hostel Warden
- Controller of Examination
- HR Executive
- Super Administrator

Roles never directly identify a Person.

Roles are reusable templates.

---

# Role Types

The platform supports multiple role classifications.

## System Roles

Delivered by SutraERP.

Examples:

- Student
- Faculty
- Parent
- Administrator

System Roles cannot be deleted.

They may be extended where permitted.

---

## Tenant Roles

Created by Tenant administrators.

Examples:

- Research Coordinator
- Placement Officer
- Scholarship Officer

Tenant Roles exist only inside one Tenant.

---

## Institution Roles

Institution-specific roles.

Examples:

- IQAC Coordinator
- Examination Cell Coordinator
- NAAC Coordinator

Institution Roles remain local to the Institution.

---

## Temporary Roles

Assigned for limited duration.

Examples:

- Acting Principal
- Election Officer
- Flying Squad
- Visiting Faculty

Temporary Roles automatically expire.

---

# Role Assignment

Role Assignment connects:

Person

↓

Role

↓

Scope

↓

Validity

↓

Authority

Role Assignment is the only mechanism through which authorization is granted.

---

# Role Assignment Attributes

Every Role Assignment contains:

- Assignment ID
- PersonID
- RoleID
- Scope
- Effective Date
- Expiry Date
- Status
- Assigned By
- Assigned On
- Approval Reference
- Remarks

---

# Role Scope

Every Role Assignment must define its scope.

Supported scopes include:

- Tenant
- Institution
- Campus
- Department
- Program
- Batch
- Division
- Hostel
- Library
- Committee
- Project
- Custom Organizational Unit

No Role Assignment is considered global.

---

# Delegated Administration

Authorization administration may itself be delegated.

Example

```
Tenant Administrator

↓

Institution Administrator

↓

Campus Administrator

↓

Department Administrator

```

Each administrator may assign only those roles that fall within their delegated authority.

Delegation never exceeds the administrator's own scope.

---

# Role Lifecycle

```
Draft

↓

Approved

↓

Active

↓

Suspended

↓

Expired

↓

Archived

```

---

# Lifecycle Description

## Draft

Role created but unavailable.

---

## Approved

Role validated by authorized administrator.

---

## Active

Role available for assignment.

---

## Suspended

Temporarily unavailable.

Existing assignments remain configurable according to policy.

---

## Expired

Role validity ended.

No further assignments permitted.

---

## Archived

Historical role retained.

No active assignments permitted.

---

# Role Assignment Lifecycle

```
Requested

↓

Approved

↓

Assigned

↓

Active

↓

Suspended

↓

Expired

↓

Revoked

```

---

# Assignment Rules

Role Assignment requires:

- Active Person
- Active Identity
- Existing Role
- Valid Scope
- Effective Dates
- Assignment Authority

Failure of any validation prevents assignment.

---

# Separation of Duties

Certain roles may not coexist.

Examples include:

- Fee Collection + Fee Approval
- Payroll Processing + Payroll Approval
- Examination Paper Setter + Paper Approver
- Inventory Request + Inventory Approval

These conflicts are configurable through authorization policies.

---

# Role Inheritance

Child scopes inherit permissions from parent scopes only when explicitly permitted.

Example

```
Institution Administrator

↓

Campus Administrator

↓

Department Administrator

```

Inheritance never bypasses organizational boundaries.

---

# Custom Roles

Tenant administrators may create custom roles.

Custom Roles may:

- Reuse existing Permission Groups
- Add additional permissions
- Remove optional permissions
- Define custom scopes

System permissions remain protected.

---

# Permission Evaluation

Every request evaluates:

```
Authenticated Person

↓

Active Context References

↓

Permission Groups

↓

Permissions

↓

Scope Validation

↓

Security Policies

↓

Decision

```

---

# Governance Rules

The following rules are mandatory.

- Every Role Assignment must reference one Person.
- Every Role Assignment must define one Scope.
- Roles never bypass Tenant boundaries.
- Expired assignments grant no permissions.
- Revoked assignments immediately terminate authorization.
- Permission Groups remain reusable.
- Direct permission assignment to Persons is prohibited.
- Every authorization change is audited.

---

# Published Events

The Context Reference Management subsystem publishes:

- RoleCreated
- RoleUpdated
- RoleApproved
- RoleActivated
- RoleArchived
- RoleAssigned
- RoleRevoked
- RoleExpired
- PermissionGroupCreated
- PermissionGroupUpdated
- PermissionGranted
- PermissionRevoked

Business domains consume these events through the platform event bus.

---

# Context Resolution

Context Resolution determines the operational environment in which an authenticated Person interacts with the SutraERP platform.

Authentication identifies the Person.

Authorization determines permitted actions.

Context Resolution determines where those permissions apply.

The Person & Identity domain never owns organizational memberships.

Instead, it resolves available operational contexts from the canonical business domains.

---

# Purpose

Context Resolution enables:

- Multi-Institution access
- Multi-Campus access
- Multi-Department access
- Multi-Role access
- Context switching
- Scoped authorization
- Session isolation

Every authenticated session operates inside exactly one Active Context.

---

# Context Providers

Operational contexts originate from business domains.

Examples include:

| Domain | Context Published |
|----------|------------------|
| Institution | Institution Membership |
| Campus | Campus Association |
| HR | Employment Assignment |
| Student Lifecycle | Student Enrollment |
| Hostel | Hostel Allocation |
| Library | Library Membership |
| Committee | Committee Membership |
| Project | Project Assignment |

The Person & Identity domain consumes these contexts but does not own them.

---

# Active Context

An Active Context represents the operational environment currently selected by the authenticated Person.

Example:

Tenant

↓

Institution

↓

Campus

↓

Department

↓

Role

↓

Program (optional)

↓

Batch (optional)

↓

Current Session

Every request executes using this Active Context.

---

# Context Selection

After successful authentication:

If exactly one valid operational context exists:

The system automatically activates it.

If multiple contexts exist:

The user must select one.

Example:

```

Welcome Rahul

Choose Working Context

• ABC Engineering College

Faculty

Computer Engineering

-------------------------

• XYZ Polytechnic

Parent

-------------------------

• ABC Engineering College

Alumni

```

After selection, the Active Context becomes part of the session.

---

# Context Components

Every Active Context contains:

- Tenant
- Institution
- Campus
- Department
- Organizational Unit
- Role
- Security Scope
- Session Identifier

Optional components include:

- Program
- Batch
- Hostel
- Library
- Committee
- Project

---

# Context Resolution Workflow

```

Authentication

↓

Retrieve Person

↓

Collect Contexts

↓

Validate Context Availability

↓

Resolve Security Scope

↓

User Selection (if required)

↓

Create Active Context

↓

Start Session

```

---

# Context Validation

A context is considered valid only if:

- Person is Active
- Identity is Active
- User Account is Enabled
- Context exists
- Role is Active
- Organizational reference exists
- Scope remains valid

Invalid contexts are excluded from selection.

---

# Context Switching

A Person may switch between available contexts without re-authentication.

Example:

```

Current Context

↓

Faculty

ABC Engineering College

↓

Switch

↓

Parent

XYZ Public School

↓

Continue Working

```

Switching contexts creates a new security evaluation.

---

# Context Isolation

Every context operates independently.

Permissions granted in one context shall never automatically transfer to another.

Example:

```

Institution A

Faculty

↓

Can Publish Attendance

------------------------

Institution B

Parent

↓

Cannot Publish Attendance

```

Authorization is evaluated separately for each context.

---

# Session Binding

Each session is permanently associated with one Active Context.

Changing context results in:

- Security Context Refresh
- Permission Recalculation
- Navigation Refresh
- Audit Entry

---

# Context Expiration

A context becomes invalid when:

- Membership removed
- Employment terminated
- Student qualification completed
- Role expired
- Institution access revoked
- Session expired

Invalid contexts cannot be activated.

---

# Context Security Rules

The following rules are mandatory.

- Every authenticated session shall contain exactly one Active Context.
- Contexts shall never cross Tenant boundaries.
- Context switching shall trigger authorization recalculation.
- Business domains remain canonical owners of context information.
- Identity consumes context without owning it.
- Authorization decisions shall always use the Active Context.
- Every context change shall be audited.

---

# Published Events

The Context Resolution subsystem publishes:

- ContextResolved
- ContextActivated
- ContextChanged
- ContextExpired
- ContextRevoked
- SecurityContextRefreshed

Business domains consume these events where appropriate.

---

# Architectural Constraints

The Person & Identity domain shall never own:

- Employment assignments
- Student enrollments
- Academic affiliations
- Hostel allocations
- Library memberships
- Department structures

Those remain canonical responsibilities of their respective domains.

Identity resolves context using published contracts and immutable references.

---

# Domain Events & Integration Contracts

The Person & Identity domain communicates with all downstream business domains through well-defined domain events.

These events notify other domains about changes in identity, authentication, authorization, and security state without exposing internal implementation details.

The Person & Identity domain never directly modifies business domain data.

Similarly, business domains never directly modify Person or Identity aggregates.

All multi-domain communication shall occur through published contracts and platform messaging.

---

# Event Architecture

```

                Person & Identity Domain

                        │

            Domain Event Publisher

                        │

────────────────────────┼────────────────────────

        Event Bus / Message Broker

────────────────────────┼────────────────────────

        │        │         │         │

 Admissions   HR   Examination   Finance

        │        │         │         │

        Subscribe to Required Events

```

---

# Event Categories

Events are grouped into the following categories:

- Person Events
- Identity Events
- User Account Events
- Authentication Events
- Authorization Events
- Session Events
- Security Events
- Context Events
- Audit Events

---

# Person Events

The Person aggregate publishes:

- PersonCreated
- PersonUpdated
- PersonVerified
- PersonActivated
- PersonInactivated
- PersonArchived
- PersonAnonymized
- PersonDeleted
- PersonMerged
- PersonSplit

---

# Identity Events

The Identity aggregate publishes:

- IdentityCreated
- IdentityVerified
- IdentityActivated
- IdentitySuspended
- IdentityDisabled
- IdentityArchived

---

# User Account Events

The User Account aggregate publishes:

- UserAccountCreated
- UserAccountEnabled
- UserAccountDisabled
- UserAccountLocked
- UserAccountUnlocked
- PasswordChanged
- PasswordResetRequested
- PasswordResetCompleted

---

# Authentication Events

Authentication publishes:

- AuthenticationStarted
- AuthenticationSucceeded
- PersonAuthenticationFailed
- MFARequested
- MFASucceeded
- MFAFailed

---

# Authorization Events

Authorization publishes:

- RoleAssigned
- RoleUpdated
- RoleRevoked
- PermissionGranted
- PermissionRevoked
- AuthorizationSucceeded
- AuthorizationDenied

---

# Session Events

Session Management publishes:

- SessionStarted
- PersonSessionExpired
- SessionEnded
- SessionTimedOut
- SessionTerminated
- ContextChanged

---

# Security Events

Security Services publish:

- AccountLocked
- AccountUnlocked
- SuspiciousLoginDetected
- MultipleFailedLogins
- PasswordPolicyViolation
- SecurityPolicyViolation

---

# Audit Events

Audit Services publish:

- IdentityAuditRecorded
- LoginAuditRecorded
- AuthorizationAuditRecorded
- AdministrativeActionRecorded

---

# Event Payload Principles

Every published event shall contain:

Mandatory:

- EventID
- EventType
- EventVersion
- EventTimestamp
- TenantID
- PersonID
- CorrelationID
- SourceSystem

Optional:

- InstitutionID
- UserAccountID
- SessionID
- ContextID
- Metadata

Sensitive information shall never be included in event payloads.

Passwords, authentication secrets, MFA codes, and confidential personal information must never be published.

---

# Event Versioning

Events are immutable.

Breaking changes require a new event version.

Examples:

- PersonCreated v1
- PersonCreated v2

Consumers may migrate independently.

Older versions remain supported according to platform compatibility policy.

---

# Event Delivery Guarantees

The platform guarantees:

- At-least-once delivery
- Ordered delivery within the same aggregate
- Immutable event payloads
- Event replay capability
- Correlation support

Consumers shall implement idempotent processing.

---

# Integration Contracts

Business domains integrate only through published contracts.

Allowed interactions include:

- Subscribe to domain events
- Query published identity APIs
- Resolve Person by PersonID
- Resolve Security Context

Business domains shall not:

- Modify Person directly
- Modify Identity directly
- Bypass authorization
- Access internal aggregates

---

# Event Consumers

Typical event subscriptions include:

Admissions

- PersonCreated
- PersonUpdated
- PersonMerged

HR

- PersonCreated
- IdentityVerified
- RoleAssigned

Examination

- PersonUpdated
- IdentityArchived
- ContextChanged

Finance

- PersonMerged
- PersonArchived

Library

- IdentityDisabled
- SessionEnded

Hostel

- PersonArchived
- ContextChanged

Each domain subscribes only to the events required for its business processes.

---

# Event Reliability Rules

The following rules are mandatory.

- Events are immutable.
- Events shall never contain confidential credentials.
- Every event shall be uniquely identifiable.
- Every event shall support traceability through CorrelationID.
- Failed consumers shall not impact event publication.
- Event processing shall be retryable.
- Duplicate event processing shall be safe.

---

# Published APIs

The Person & Identity domain may expose read-only service contracts for:

- Resolve Person
- Resolve Identity
- Resolve User Account
- Resolve Active Context
- Resolve Authorization
- Validate Session
- Validate Token

Write operations remain internal to the Person & Identity domain.

---

# Architectural Constraints

The Person & Identity domain is the only publisher of canonical identity events.

Downstream domains consume identity events but never redefine them.

Identity events are platform contracts and shall remain backward compatible across releases.

---

# Person Lifecycle

The Person Lifecycle governs the complete existence of a Person within the SutraERP platform.

The lifecycle begins with the creation of a canonical Person record and concludes only after an approved archival, anonymization, or permanent deletion process.

Every lifecycle transition shall be governed by business rules, approval policies, and audit requirements.

---

# Lifecycle Philosophy

A Person represents a real human.

Therefore:

- A Person should normally exist for the lifetime of the relationship with the Tenant.
- Business roles may change.
- Organizational assignments may change.
- Identity credentials may change.
- The Person itself remains the canonical identity.

Deletion is considered an exceptional administrative operation.

---

# Lifecycle State Machine

```

Draft

↓

Created

↓

Verified

↓

Active

↓

Inactive

↓

Archived

↓

Anonymized

↓

Deleted

```

Only approved transitions are permitted.

---

# State Definitions

## Draft

Temporary record.

Characteristics:

- Not visible outside Identity domain
- Editable
- No authentication
- No downstream references

---

## Created

Canonical Person exists.

Characteristics:

- PersonID generated
- Business Identifier generated
- Available for verification
- Authentication not yet enabled

---

## Verified

Identity verification completed.

Characteristics:

- Government documents validated (if required)
- Contact verification completed
- Eligible for activation

---

## Active

Normal operational state.

Characteristics:

- Login permitted
- Roles may be assigned
- Business domains may reference Person
- Sessions permitted

---

## Inactive

Temporary suspension.

Characteristics:

- Login disabled
- Business history preserved
- Re-activation permitted

---

## Archived

Relationship concluded.

Characteristics:

- Authentication disabled
- Business history preserved
- Read-only identity

---

## Anonymized

Personally identifiable information removed or masked according to configured retention policies.

Characteristics:

- Business references preserved
- Audit preserved
- Identity cannot be restored unless policy permits

---

## Deleted

Permanent removal.

Characteristics:

- Exceptional administrative operation
- Requires approval
- Dependency validation mandatory

---

# Allowed State Transitions

| From | To | Allowed |
|-------|----|----------|
| Draft | Created | Yes |
| Created | Verified | Yes |
| Verified | Active | Yes |
| Active | Inactive | Yes |
| Inactive | Active | Yes |
| Active | Archived | Yes |
| Archived | Anonymized | Yes |
| Archived | Deleted | Conditional |
| Inactive | Archived | Yes |

Any other transition shall be rejected.

---

# Person Creation Workflow

```

Create Request

↓

Validate Tenant

↓

Duplicate Detection

↓

Generate PersonID

↓

Generate Business ID

↓

Persist Person

↓

Publish PersonCreated

↓

Created

```

---

# Canonical Person–Identity Establishment and Activation Workflow

```text
Created Person

↓

Create Associated Identity

↓

Publish IdentityCreated

↓

Identity Pending

↓

Determine Tenant Verification Policy

↓

Validate Required Evidence

↓

Approval (if required)

↓

Person: Created → Verified

↓

Publish PersonVerified

↓

Identity: Pending → Verified

↓

Publish IdentityVerified

↓

Person: Verified → Active

↓

Publish PersonActivated

↓

Identity: Verified → Active

↓

Publish IdentityActivated

↓

Active Person and Active Identity
```

Verification policies remain configurable per Tenant.

The Person and its associated Identity must each reach `Verified` before activation. The establishment outcome completes only when both aggregates reach `Active` and their respective canonical activation events have been published.

User Account creation, where required, occurs only after successful Person and Identity activation. It is not part of the Person and Identity establishment outcome and does not enable authentication by itself.

Authentication capability, credentials, factors, providers, and sessions remain governed by L2-A03. Roles, role assignments, delegations, authorization policies, and authorization-context assignment remain governed by L2-A04.

---

# Temporary Deactivation Workflow

```

Active

↓

Administrative Action

or

Policy Trigger

↓

Disable Login

↓

Terminate Sessions

↓

Publish PersonInactivated

↓

Inactive

```

No historical information is removed.

---

# Reactivation Workflow

```

Inactive

↓

Administrative Approval

↓

Policy Validation

↓

Enable Identity

↓

Enable User Account

↓

Publish PersonActivated

↓

Active

```

---

# Archive Workflow

```

Active

↓

Archive Request

↓

Dependency Validation

↓

Disable Authentication

↓

Terminate Sessions

↓

Archive Identity

↓

Publish PersonArchived

↓

Archived

```

---

# Anonymization Workflow

```

Archived

↓

Privacy Request

↓

Retention Validation

↓

Mask Personal Data

↓

Retain Business References

↓

Publish PersonAnonymized

↓

Anonymized

```

Only fields permitted by policy may be anonymized.

---

# Permanent Deletion Workflow

```

Deletion Request

↓

Administrative Approval

↓

Dependency Validation

↓

Legal Retention Validation

↓

Security Confirmation

↓

Delete

↓

Publish PersonDeleted

```

Deletion is allowed only when:

- Legal retention requirements satisfied
- No blocking dependencies exist
- Approval workflow completed

---

# Duplicate Merge Workflow

```

Duplicate Detected

↓

Administrative Review

↓

Select Canonical Person

↓

Transfer References

↓

Archive Duplicate

↓

Publish PersonMerged

```

No business history is lost.

---

# Split Workflow

```

Split Request

↓

Administrative Review

↓

Validate Merge History

↓

Create New Person

↓

Redistribute References

↓

Publish PersonSplit

```

Split operations are exceptional.

---

# Lifecycle Business Rules

The following rules are mandatory.

- A Person must belong to one Tenant.
- PersonID is immutable.
- Active Persons may authenticate.
- Archived Persons may not authenticate.
- Deleted Persons cannot be restored.
- Every transition is audited.
- Every transition publishes a domain event.
- Unauthorized transitions are rejected.

---

# Approval Policies

The following operations require approval.

| Operation | Approval Required |
|------------|-------------------|
| Archive Person | Yes |
| Anonymize Person | Yes |
| Delete Person | Mandatory |
| Merge Person | Yes |
| Split Person | Yes |
| Identity Recovery | Yes |

Approval authority is determined by authorization policies.

---

# Published Events

Lifecycle workflows publish:

- PersonCreated
- PersonVerified
- PersonActivated
- PersonInactivated
- PersonArchived
- PersonAnonymized
- PersonDeleted
- PersonMerged
- PersonSplit

Downstream domains shall synchronize through these events.

---

# Architectural Constraints

- Lifecycle transitions shall occur only through approved workflows.
- Business domains shall never change Person lifecycle directly.
- Identity state shall remain synchronized with Person state.
- Every lifecycle operation shall be fully auditable.
- All destructive operations require approval and policy validation.

---

# Identity Governance

Identity Governance ensures that every Person within a Tenant represents one real human through a single canonical identity.

It establishes policies and workflows for:

- Identity Verification
- Duplicate Detection
- Duplicate Prevention
- Identity Merge
- Identity Split
- Data Quality Monitoring
- Identity Validation
- Identity Integrity

Identity Governance protects the platform from fragmented identities and inconsistent business references.

---

# Governance Objectives

The objectives of Identity Governance are:

- Maintain one canonical Person per individual within a Tenant.
- Prevent duplicate Person creation.
- Ensure identity data quality.
- Verify identity before activation where required.
- Maintain referential integrity during merge and split operations.
- Preserve complete audit history.

---

# Identity Verification

Identity Verification confirms that a Person represents a genuine and verifiable individual according to Tenant policies.

Verification requirements are configurable.

Examples include:

- Mobile Number Verification
- Email Verification
- Government Identity Verification
- Employee Verification
- Student Verification
- Parent Verification
- Manual Administrative Verification

Verification requirements may vary by Person Type.

---

# Verification Levels

The platform supports multiple verification levels.

| Level | Description |
|--------|-------------|
| Level 0 | No Verification |
| Level 1 | Mobile Verified |
| Level 2 | Mobile + Email Verified |
| Level 3 | Government ID Verified |
| Level 4 | Manual Administrative Verification |
| Level 5 | Fully Verified Identity |

Tenants may define additional verification levels.

---

# Verification Workflow

```

Verification Requested

↓

Determine Verification Policy

↓

Validate Required Evidence

↓

Perform Verification

↓

Administrative Review (if required)

↓

Verification Approved

↓

Publish IdentityVerified

```

---

# Duplicate Detection

Duplicate Detection identifies multiple Person records that may represent the same real individual.

Detection occurs during:

- Person Creation
- Bulk Import
- Identity Synchronization
- Administrative Review
- Scheduled Data Quality Scan

Duplicate detection is advisory unless Tenant policy mandates enforcement.

---

# Duplicate Detection Criteria

Matching may consider:

- Full Name
- Date of Birth
- Mobile Number
- Email Address
- Government ID
- Employee Number
- Student Number
- Passport Number
- External Identity Reference

Matching rules are configurable.

---

# Matching Strategy

The platform supports:

- Exact Matching
- Weighted Matching
- Fuzzy Name Matching
- Configurable Rule-Based Matching

Matching algorithms may evolve without changing the domain model.

---

# Duplicate Confidence Levels

| Confidence | Action |
|------------|--------|
| Low | Informational Warning |
| Medium | Administrative Review |
| High | Creation Block or Override |
| Exact Match | Reject or Merge Recommendation |

Thresholds remain configurable.

---

# Duplicate Review Workflow

```

Potential Duplicate

↓

Administrative Review

↓

Confirm Duplicate

↓

No ----------------→ Continue

↓

Yes

↓

Merge Recommendation

↓

Approval

↓

Merge Execution

```

---

# Identity Merge

Merge combines multiple duplicate Person records into one canonical Person.

One Person becomes the survivor.

Remaining Person records become historical merge sources.

Business references are transferred to the surviving Person.

---

# Merge Workflow

```

Duplicate Confirmed

↓

Select Canonical Person

↓

Validate Dependencies

↓

Transfer References

↓

Archive Duplicate Record

↓

Publish PersonMerged

```

No business history shall be lost during merge.

---

# Merge Validation

Before merge:

- Same Tenant
- Administrative Approval
- No conflicting legal restrictions
- Dependency validation complete
- Survivor identified
- Merge policy satisfied

Failure of any validation blocks the merge.

---

# Reference Migration

During merge:

- Admissions references updated.
- Student references updated.
- HR references updated.
- Examination references updated.
- Finance references updated.
- Library references updated.
- Hostel references updated.

Historical audit references remain unchanged.

---

# Identity Split

Split separates one Person into two independent canonical Person records.

Split operations are exceptional and occur only when a previous merge or incorrect identity assignment is identified.

---

# Split Workflow

```

Split Request

↓

Administrative Review

↓

Validate Eligibility

↓

Create New Person

↓

Redistribute References

↓

Publish PersonSplit

```

---

# Split Validation

Split requires:

- Administrative approval
- Complete audit trail
- Reference redistribution plan
- Dependency validation
- Legal compliance validation

---

# Data Quality Monitoring

Identity Governance continuously monitors data quality.

Quality indicators include:

- Duplicate rate
- Missing mandatory attributes
- Invalid contact information
- Unverified identities
- Incomplete demographic data
- Expired verification status

Monitoring may be performed on demand or through scheduled jobs.

---

# Data Quality Score

Each Person may be assigned a configurable data quality score.

Factors influencing the score include:

- Verification completeness
- Mandatory field completion
- Contact validity
- Duplicate confidence
- Identity consistency

The score is informational and does not alter business ownership.

---

# Governance Dashboard

Identity Governance dashboards may report:

- Total Persons
- Active Persons
- Unverified Persons
- Duplicate Candidates
- Pending Merge Requests
- Pending Split Requests
- Verification Backlog
- Data Quality Trends

Dashboards consume published events and read models.

---

# Manual Overrides

Authorized administrators may override:

- Duplicate warnings
- Verification requirements
- Merge recommendations

Every override shall record:

- Approver
- Date and Time
- Reason
- Supporting Remarks

Overrides never bypass audit requirements.

---

# Business Rules

The following rules are mandatory.

- One canonical Person per Tenant.
- Duplicate detection shall execute before Person creation where applicable.
- Merge shall preserve business history.
- Split shall preserve referential integrity.
- Verification policies shall be configurable.
- Data quality checks shall never modify business data automatically.
- All governance operations shall be auditable.

---

# Published Events

Identity Governance publishes:

- IdentityVerificationRequested
- IdentityVerified
- IdentityVerificationFailed
- DuplicateCandidateDetected
- DuplicateOverrideApproved
- PersonMerged
- PersonSplit
- DataQualityAssessmentCompleted

These events enable downstream domains to synchronize identity-related processes.

---

# Architectural Constraints

- Identity Governance owns verification and identity quality policies.
- Business domains remain owners of their own business records.
- Merge and split operations shall never violate referential integrity.
- Duplicate detection algorithms shall be configurable without changing aggregate structures.
- Every governance action shall generate immutable audit records.

---

# Identity Service Architecture

The Person & Identity domain exposes canonical platform services for identity management.

These services provide controlled access to Person, Identity, User Account, Authentication, Authorization, and Context Resolution capabilities.

Business domains shall interact with the Person & Identity domain only through published service contracts or domain events.

Direct database access is prohibited.

---

# Service Architecture

```

                Person & Identity Domain

                         │

              Application Services

                         │

─────────────────────────┼────────────────────────

      Read APIs                Command APIs

─────────────────────────┼────────────────────────

                         │

               Event Publication

                         │

─────────────────────────┼────────────────────────

             Business Domains

```

---

# Service Categories

The domain exposes the following service categories.

- Person Services
- Identity Services
- User Account Services
- Authentication Services
- Authorization Services
- Context Services
- Administration Services
- Integration Services

---

# Command Services

Command services modify domain state.

Examples include:

- Create Person
- Update Person
- Verify Identity
- Activate Person
- Archive Person
- Merge Persons
- Split Person
- Create User Account
- Reset Password
- Assign Role
- Revoke Role

Command services execute business workflows and validations before persisting changes.

---

# Query Services

Query services provide read-only access to identity information.

Examples include:

- Get Person
- Search Persons
- Resolve Identity
- Resolve User Account
- Resolve Active Context
- Resolve Assigned Roles
- Resolve Effective Permissions

Query services shall never modify domain state.

---

# Person Service Contracts

Supported operations include:

Create

- Create Person
- Verify Person
- Activate Person
- Archive Person
- Merge Person
- Split Person

Read

- Get Person
- Search Persons
- Validate Person
- Resolve Person

Delete operations are governed by lifecycle workflows and shall not be exposed as unrestricted APIs.

---

# Identity Service Contracts

Supported operations include:

- Create Identity
- Activate Identity
- Suspend Identity
- Disable Identity
- Verify Identity
- Resolve Identity Status

Identity lifecycle remains synchronized with the Person lifecycle.

---

# User Account Service Contracts

Supported operations include:

- Create User Account
- Enable User Account
- Disable User Account
- Lock User Account
- Unlock User Account
- Change Password
- Reset Password

User Account creation shall occur only after successful Person and associated Identity activation. It is a post-activation A02 lifecycle action, is not a prerequisite for Person or Identity activation, and does not transfer credential, authentication-factor, Authentication Provider, Authentication Session, role, delegation, or authorization-policy ownership into A02.

---

# Authentication Services

Authentication services include:

- Authenticate User
- Validate Credentials
- Perform MFA
- Create Session
- Validate Session
- Terminate Session
- Refresh Session

Authentication services do not expose internal credential storage.

---

# Authorization Services

Authorization services include:

- Evaluate Permission
- Resolve Roles
- Resolve Effective Permissions
- Validate Scope
- Validate Access Request

Authorization responses shall always consider the Active Context.

---

# Context Services

Context services include:

- Resolve Available Contexts
- Activate Context
- Change Active Context
- Resolve Current Context

Context services consume business-domain context providers without owning their data.

---

# Administration Services

Administrative services include:

- Duplicate Review
- Merge Approval
- Split Approval
- Verification Approval
- Identity Audit Review
- Session Management
- Account Recovery

Administrative operations require authorization according to platform policies.

---

# Integration Services

Integration services support interoperability with external systems.

Examples include:

- Identity Synchronization
- Directory Synchronization
- Student Information Exchange
- HR Integration
- Government Identity Integration
- Enterprise Identity Providers

Integration services execute through approved contracts.

---

# API Design Principles

All APIs shall follow these principles.

- Stateless communication
- Explicit versioning
- Idempotent operations where applicable
- Strong validation
- Consistent error responses
- Auditability
- Secure transport
- Tenant isolation

---

# API Versioning

Every public API shall include an explicit version.

Example:

```

/api/v1/persons

/api/v1/authentication

/api/v1/roles

```

Breaking changes require a new API version.

Existing versions remain supported according to platform compatibility policy.

---

# Idempotency

The following operations shall support idempotency where applicable:

- Create Person
- Import Person
- Identity Synchronization
- External User Provisioning

Repeated requests with the same idempotency key shall not create duplicate records.

---

# Concurrency Control

Concurrent updates shall be managed using optimistic concurrency.

Typical process:

```

Read Entity

↓

Modify

↓

Version Validation

↓

Persist

↓

Success

```

If version validation fails, the operation shall be rejected with a concurrency conflict.

---

# Error Handling

Standard response categories include:

- Validation Error
- Authentication Failure
- Authorization Failure
- Resource Not Found
- Duplicate Detected
- Business Rule Violation
- Concurrency Conflict
- Internal Error

Sensitive implementation details shall never be returned to clients.

---

# External Identity Providers

The architecture supports future integration with:

- OAuth2 Providers
- OpenID Connect Providers
- SAML Identity Providers
- LDAP Directories
- Active Directory
- SCIM Provisioning

These integrations extend authentication without modifying the Person aggregate.

---

# SCIM Readiness

The architecture is designed to support System for Multi-domain Identity Management (SCIM).

Potential capabilities include:

- User Provisioning
- User De-provisioning
- Group Synchronization
- Identity Attribute Updates

SCIM integration remains optional and configurable.

---

# Import & Export Contracts

Supported import mechanisms include:

- Bulk File Import
- REST APIs
- Scheduled Synchronization
- Event-Based Synchronization

Supported export mechanisms include:

- REST APIs
- Event Publication
- Scheduled Data Feeds

All import operations execute duplicate detection before persistence.

---

# Security Requirements

All service contracts shall enforce:

- Authentication
- Authorization
- Tenant Isolation
- Scope Validation
- Input Validation
- Audit Logging
- Secure Transport

Public APIs shall never expose internal aggregate structures.

---

# Service Ownership Rules

The Person & Identity domain owns:

- Person APIs
- Identity APIs
- Authentication APIs
- Authorization APIs
- Context APIs

Business domains shall never expose alternative identity management interfaces.

---

# Published Integration Events

External integrations may consume:

- PersonCreated
- PersonUpdated
- PersonActivated
- PersonArchived
- IdentityVerified
- UserAccountCreated
- RoleAssigned
- SessionStarted
- ContextChanged

Consumers shall treat events as immutable integration contracts.

---

# Architectural Constraints

- All write operations shall execute through application services.
- Read services shall remain side-effect free.
- Public APIs shall not expose internal aggregate implementations.
- Integration contracts shall remain backward compatible.
- External systems shall never bypass business workflows.
- Every externally initiated write operation shall be auditable.

---

# Security Architecture *(legacy cross-domain reference; A03/A04 contracts govern)*

> Authentication proofs, sessions, factors, providers, and their policy evaluation below are A03 concepts. Roles, Role Assignments, Delegations, permissions, and authorization policies below are A04 concepts. A02 supplies canonical Person, Identity, User Account, and context references only.

The Person & Identity domain serves as the security foundation for the SutraERP platform.

It establishes centralized security policies governing:

- Identity Protection
- Authentication
- Authorization
- Credential Management
- Session Security
- Account Protection
- Threat Detection
- Compliance
- Security Auditing

Business domains consume these capabilities but shall not implement independent security mechanisms.

---

# Security Principles

The security architecture is based on the following principles.

- Zero Trust
- Least Privilege
- Defense in Depth
- Secure by Default
- Explicit Authorization
- Complete Auditability
- Tenant Isolation
- Privacy by Design

These principles apply across every identity-related workflow.

---

# Security Layers

```

Client Layer

↓

Transport Security

↓

Authentication

↓

Authorization

↓

Context Resolution

↓

Application Services

↓

Business Domains

↓

Audit & Monitoring

```

Each layer provides an independent security boundary.

---

# Identity Protection

Identity protection safeguards Person and User Account information against unauthorized access or modification.

Protection mechanisms include:

- Strong authentication
- MFA enforcement
- Account lock policies
- Credential protection
- Session validation
- Audit logging

---

# Credential Management

Credentials shall never be stored in plaintext.

Supported credential types include:

- Password
- Mobile Authentication
- Email Authentication
- MFA Secret
- Recovery Codes

Future credential types may include:

- Passkeys
- Hardware Security Keys
- Enterprise Identity Tokens

Credential storage shall remain abstracted behind the authentication subsystem.

---

# Password Policies

Tenant administrators may configure password policies.

Supported controls include:

- Minimum length
- Maximum length
- Complexity requirements
- Password history
- Expiration period
- Minimum password age
- Reuse restrictions

Password policies are evaluated during password creation and change operations.

---

# Account Lock Policy

Accounts may be temporarily locked after repeated authentication failures.

Configurable parameters include:

- Maximum failed attempts
- Lock duration
- Automatic unlock
- Administrative unlock

Lock policies are evaluated independently for each Tenant.

---

# Session Security

Every authenticated session shall maintain:

- Session Identifier
- Authentication Level
- Device Information
- IP Address (where available)
- Last Activity Timestamp
- Active Context

Sessions shall terminate automatically according to configured inactivity policies.

---

# Session Protection

The platform supports:

- Session timeout
- Session revocation
- Forced logout
- Concurrent session management
- Administrative session termination

Session identifiers shall be unpredictable and securely generated.

---

# Multi-Factor Authentication

MFA policies are configurable by Tenant.

Authentication factors may include:

- One-Time Password (OTP)
- Authenticator Application
- Backup Recovery Codes

Future support may include:

- Passkeys
- FIDO2/WebAuthn
- Hardware Tokens

MFA requirements may vary by Role, Context, or Security Policy.

---

# Account Recovery

Account recovery workflows shall require identity verification before credential reset.

Supported recovery mechanisms include:

- Verified Mobile Number
- Verified Email Address
- Administrative Recovery

Recovery workflows shall always generate audit records.

---

# Threat Detection

The security subsystem monitors identity-related events for suspicious behavior.

Examples include:

- Repeated failed logins
- Unusual login locations
- Simultaneous active sessions
- Brute-force attempts
- Privilege escalation attempts
- Excessive password reset requests

Threat detection policies are configurable.

---

# Security Policies

Tenant-level policies may govern:

- Password rules
- MFA requirements
- Session timeout
- Login hours
- Allowed IP ranges
- Device restrictions
- Concurrent session limits

Security policies apply uniformly across all business domains.

---

# Privacy Protection

Personally identifiable information (PII) shall be protected according to applicable legal and organizational requirements.

Sensitive attributes include:

- Government identifiers
- Mobile numbers
- Email addresses
- Date of birth
- Residential addresses

Access to sensitive information requires explicit authorization.

---

# Data Encryption

Sensitive data shall be protected using platform-approved encryption mechanisms.

The architecture distinguishes:

- Data in Transit
- Data at Rest
- Credential Storage

Encryption implementation remains an infrastructure responsibility and is not embedded within domain logic.

---

## Privacy Boundary and Cryptographic Governance

The Person & Identity domain shall expose only the minimum identity attributes required by the consuming domain, workflow, or approval context.

The following privacy rules apply:

- Identity references may be shared multi-domain only in scoped, purpose-bound form.
- Government identity data, contact data, and privacy preferences shall not be globally visible by default.
- Downstream domains shall not duplicate raw identity records unless explicitly required by architectural contract.
- Data minimization and purpose limitation shall govern identity propagation and export.
- Anonymized or deleted identity states shall not reintroduce personal data into downstream domains.

The following cryptographic rules apply:

- The Platform Security and Cryptographic Governance model owns shared key, certificate, and secret lifecycle governance.
- The Person & Identity domain shall not independently own shared encryption keys or certificates.
- Identity data that requires encryption shall consume the platform-approved cryptographic governance model.

---

# Security Compliance

The architecture is designed to support compliance with applicable organizational and regulatory requirements.

Examples may include:

- Institutional security policies
- Government data protection regulations
- Internal audit requirements

Compliance implementation remains configurable per deployment.

---

# Security Incident Response

Security-related events may trigger operational workflows.

Examples include:

- Account compromise
- Credential leakage
- Unauthorized access attempts
- Suspicious administrative activity

Incident handling workflows shall integrate with platform monitoring and audit services.

---

# Administrative Security

Administrative operations require elevated authorization.

Examples include:

- Force password reset
- Unlock account
- Disable identity
- Revoke sessions
- Override duplicate detection
- Merge or split Person records

Every administrative security action shall be audited.

---

# Security Events

The security subsystem publishes:

- AccountLocked
- AccountUnlocked
- PasswordChanged
- PasswordResetRequested
- PasswordResetCompleted
- MFAVerified
- MFAFailed
- SuspiciousLoginDetected
- SecurityPolicyViolation
- SessionRevoked

Business domains may subscribe to these events where operationally required.

---

# Security Business Rules

The following rules are mandatory.

- Every authentication request shall be authenticated before authorization.
- Credentials shall never be exposed through APIs or events.
- MFA enforcement shall follow Tenant policy.
- Sensitive attributes require explicit authorization.
- Every security operation shall generate an audit record.
- Security policies shall be centrally managed.
- Business domains shall not implement independent authentication mechanisms.

---

# Architectural Constraints

- Security policies are owned by the Person & Identity domain.
- Business domains consume security capabilities but do not redefine them.
- Security controls shall remain configurable without modifying domain models.
- Identity protection shall apply consistently across all Tenants.
- Every security-relevant action shall support traceability through audit records.

---

# Audit & Observability

The Person & Identity domain shall provide comprehensive observability for all identity-related activities.

Observability consists of four independent capabilities:

- Audit Records
- Operational Logging
- Metrics & Monitoring
- Distributed Tracing

Each capability has a distinct purpose, lifecycle, and retention policy.

---

# Observability Architecture

```

                Person & Identity Domain

                         │

─────────────────────────┼─────────────────────────

        Audit        Logs        Metrics       Traces

─────────────────────────┼─────────────────────────

                         │

            Central Observability Platform

                         │

 Dashboards • Alerts • Compliance • Analytics

```

---

# Audit Architecture

Audit records provide immutable evidence of security and administrative actions.

Audit records exist for:

- Regulatory compliance
- Internal investigations
- Administrative accountability
- Historical reconstruction

Audit records are never used for application debugging.

---

# Audit Principles

Audit records shall be:

- Immutable
- Timestamped
- Digitally traceable
- Tenant isolated
- Tamper evident
- Searchable
- Retainable according to policy

No audit record may be modified after creation.

---

# Audit Categories

The platform records:

- Authentication Audit
- Authorization Audit
- Identity Audit
- Administrative Audit
- Configuration Audit
- Session Audit
- Security Audit
- Data Governance Audit

Each category follows a standardized schema.

---

# Audit Record Structure

Every audit record contains:

Mandatory:

- Audit ID
- Timestamp
- Tenant ID
- Person ID (if applicable)
- User Account ID (if applicable)
- Session ID
- Correlation ID
- Action
- Resource
- Outcome

Optional:

- Institution ID
- Active Context
- Client Device
- Client IP
- Additional Metadata

Sensitive values shall never be stored in plaintext.

---

# Audit Events

Examples include:

- Person Created
- Person Updated
- Identity Verified
- Login Successful
- Login Failed
- Password Changed
- MFA Enabled
- Role Assigned
- Role Revoked
- Context Switched
- Account Locked
- Account Unlocked
- Person Archived
- Person Deleted

Every security-sensitive operation generates an audit record.

---

# Operational Logging

Operational logs support troubleshooting and diagnostics.

Examples include:

- Service startup
- Service shutdown
- API execution
- Validation failures
- Integration failures
- Retry attempts
- Background job execution
- Cache synchronization

Operational logs are not legal evidence.

---

# Log Levels

Supported logging levels:

- TRACE
- DEBUG
- INFO
- WARN
- ERROR
- FATAL

Production logging policies are configurable.

Sensitive information shall never be logged.

---

# Structured Logging

All application logs shall follow a structured format.

Recommended fields include:

- Timestamp
- Service Name
- Tenant ID
- Correlation ID
- Request ID
- Operation
- Severity
- Duration
- Status

Structured logs simplify automated analysis.

---

# Distributed Tracing

Distributed tracing follows requests across multiple platform services.

Each request carries:

- Trace ID
- Span ID
- Correlation ID

Tracing enables end-to-end visibility amulti-domain interactions.

---

# Correlation IDs

Every externally initiated request shall receive a Correlation ID.

The same Correlation ID propagates through:

- API Gateway
- Identity Services
- Business Domains
- Event Bus
- Background Jobs

Correlation IDs enable complete request reconstruction.

---

# Metrics Collection

Operational metrics monitor platform health.

Examples include:

Authentication Metrics

- Successful Logins
- Failed Logins
- MFA Success Rate
- Lockout Rate

Identity Metrics

- Persons Created
- Active Persons
- Archived Persons
- Verification Completion Rate

Authorization Metrics

- Permission Evaluations
- Authorization Failures
- Context Switches

Session Metrics

- Active Sessions
- Session Expirations
- Forced Session Revocations

---

# Performance Metrics

Identity services shall measure:

- API Response Time
- Authentication Latency
- Authorization Latency
- Session Creation Time
- Query Duration
- Event Publication Latency

Performance thresholds remain configurable.

---

# Health Monitoring

Every identity service exposes health indicators.

Typical indicators include:

- Service Availability
- Database Connectivity
- Event Bus Connectivity
- Cache Availability
- Authentication Service Status
- Authorization Engine Status

Health indicators support automated platform monitoring.

---

# Alerting

Operational alerts may be generated for:

- Authentication failures exceeding threshold
- Repeated account lockouts
- High authorization failure rate
- Event publication failures
- Service outages
- Performance degradation
- Integration failures

Alert thresholds are configurable.

---

# Dashboards

Operational dashboards may include:

Identity Dashboard

- Active Persons
- Verification Status
- Duplicate Candidates

Authentication Dashboard

- Login Success Rate
- MFA Adoption
- Failed Login Trend

Authorization Dashboard

- Permission Requests
- Access Denials
- Role Assignment Trends

Security Dashboard

- Locked Accounts
- Security Violations
- Suspicious Login Activity

Operations Dashboard

- Service Health
- Response Time
- Error Rate
- Event Throughput

---

# Retention Policies

Retention periods are configurable.

Typical policy categories include:

- Audit Records
- Operational Logs
- Metrics
- Traces

Each category may have independent archival and deletion policies.

---

# Data Privacy

Observability data shall comply with platform privacy requirements.

The following shall never appear in logs or traces:

- Passwords
- MFA secrets
- Authentication tokens
- Encryption keys
- Recovery codes

Personally identifiable information shall be logged only where explicitly permitted.

---

# Published Events

The observability subsystem publishes:

- PersonAuditRecorded
- LogThresholdExceeded
- HealthStatusChanged
- AlertGenerated
- MetricThresholdExceeded
- TraceCompleted

These events enable centralized monitoring without coupling business domains.

---

# Business Rules

The following rules are mandatory.

- Every security-sensitive operation shall generate an audit record.
- Audit records are immutable.
- Operational logs shall never replace audit records.
- Every external request shall carry a Correlation ID.
- Sensitive credentials shall never be logged.
- Monitoring shall not modify business state.
- Observability data shall remain Tenant isolated.

---

# Architectural Constraints

- Audit, logging, metrics, and tracing remain independent capabilities.
- Business domains consume the centralized observability framework.
- Retention policies shall be configurable without modifying domain logic.
- Observability shall support horizontal scalability.
- Every identity workflow shall be traceable from request initiation to completion.

---

# Identity Configuration & Administrative Governance

The Person & Identity domain provides a configurable identity management framework while preserving platform-wide architectural consistency.

Configuration enables Tenants to adapt identity behavior without modifying canonical domain models or violating architectural invariants.

The platform distinguishes between:

- Platform Invariants
- Tenant Policies
- Tenant Configuration
- Identity Master Data

Each category has clearly defined ownership and governance.

---

# Configuration Philosophy

The platform follows the principle:

"Configure behavior, never redefine architecture."

Configuration shall influence:

- Validation Rules
- Security Policies
- Identity Workflows
- Verification Policies
- User Experience

Configuration shall never alter:

- Aggregate ownership
- Domain boundaries
- Canonical identifiers
- Event contracts
- Referential integrity

---

# Configuration Hierarchy

```

Platform Defaults

↓

Tenant Policies

↓

Institution Policies (Optional)

↓

Runtime Evaluation

```

Lower levels may override only those settings explicitly designated as overridable.

---

# Platform Invariants

The following rules are architecturally fixed.

- One canonical Person per Tenant
- One User Account per Person per Tenant
- Authentication precedes Authorization
- Authorization requires Active Context
- PersonID is immutable
- Domain Events are immutable
- Business domains do not own Person
- Audit records are immutable
- Tenant isolation is mandatory

These rules cannot be overridden.

---

# Tenant Policies

Tenant administrators may configure:

- Password Policy
- MFA Policy
- Session Timeout
- Login Identifier Types
- Identity Verification Levels
- Duplicate Detection Thresholds
- Account Lock Policy
- Context Selection Rules
- Identity Numbering Strategy
- Data Retention Policy
- Privacy Rules

Tenant policies remain isolated within the Tenant boundary.

---

# Institution Policies

Where permitted by the platform, Institutions may define localized operational policies.

Examples include:

- Faculty MFA Requirements
- Student Verification Requirements
- Session Timeout Overrides
- Login Hours
- Department-specific Administrative Delegation

Institution policies cannot violate Tenant or Platform policies.

---

# Policy Evaluation

Policies are evaluated using the following precedence:

```

Platform Rule

↓

Tenant Policy

↓

Institution Policy

↓

Runtime Decision

```

If multiple policies conflict, the most restrictive applicable policy shall prevail unless explicitly documented otherwise.

---

# Password Configuration

Configurable attributes include:

- Minimum Length
- Maximum Length
- Character Complexity
- Password Expiration
- Password History
- Reuse Restrictions
- Minimum Password Age

Password validation occurs during creation and modification.

---

# Authentication Configuration *(legacy A03 integration reference)*

Tenant administrators may configure:

- Username Login
- Mobile Login
- Email Login (Future)
- MFA Requirement
- Trusted Devices
- Session Timeout
- Concurrent Session Limit

Authentication methods remain extensible.

---

# Identity Verification Configuration

Verification policies may vary by Person Type.

Examples:

Student

- Mobile Verification
- Email Verification

Faculty

- Mobile Verification
- Email Verification
- Government ID Verification

Administrator

- Full Verification
- MFA Mandatory

Verification policies remain declarative and configurable.

---

# Duplicate Detection Configuration

Tenant policies may define:

- Matching Attributes
- Matching Weights
- Similarity Thresholds
- Automatic Warning Threshold
- Manual Review Threshold
- Creation Blocking Threshold

Matching algorithms remain implementation-independent.

---

# Identity Numbering Strategy

Business identifiers may follow configurable formats.

Examples:

```

PER-000000001

PER-2026-000001

EMP-000154

STU-000987

```

Configuration options include:

- Prefix
- Numeric Width
- Financial Year Inclusion
- Institution Code
- Sequence Scope

Internal UUIDs remain immutable and unaffected.

---

# Session Policies

Configurable session settings include:

- Idle Timeout
- Absolute Session Lifetime
- Maximum Concurrent Sessions
- Device Restrictions
- Automatic Logout
- Session Renewal Policy

Session security remains centrally enforced.

---

# Role Governance Policies

Tenant administrators may configure:

- Role Approval Requirement
- Delegated Administration
- Assignment Expiration
- Role Review Frequency
- Separation of Duties Rules

Role structures remain governed by the authorization model.

---

# Administrative Governance

Administrative operations are governed through configurable approval policies.

Examples:

| Operation | Approval Configurable |
|------------|-----------------------|
| Merge Person | Yes |
| Split Person | Yes |
| Archive Person | Yes |
| Delete Person | Yes |
| Reset Password | Yes |
| Unlock Account | Yes |
| Override Duplicate Warning | Yes |

Approval workflows integrate with the platform workflow engine.

---

# Feature Flags

The platform supports controlled feature enablement.

Examples include:

- MFA Enabled
- Passkey Support
- OAuth Login
- SCIM Provisioning
- Identity Synchronization
- Advanced Duplicate Detection
- Context Switching

Feature flags allow staged rollout without architectural changes.

---

# Identity Master Data

The following master data remains configurable.

- Person Types
- Verification Types
- Identity Document Types
- Role Categories
- Permission Categories
- Security Questions (if enabled)
- Login Identifier Types
- Authentication Methods

Master data changes are auditable.

---

# Configuration Versioning

Every configuration change shall maintain:

- Configuration Version
- Effective Date
- Previous Version
- Changed By
- Change Reason

Runtime services evaluate the active configuration version.

---

# Configuration Audit

Every configuration modification shall generate audit records including:

- Configuration Item
- Previous Value
- New Value
- Changed By
- Timestamp
- Approval Reference
- Correlation ID

Configuration history shall be immutable.

---

# Configuration Import & Export

Configuration may be migrated using approved platform mechanisms.

Supported operations include:

- Export Configuration
- Import Configuration
- Validate Configuration
- Compare Configuration Versions
- Rollback to Previous Version (where supported)

Imports shall be validated before activation.

---

# Governance Business Rules

The following rules are mandatory.

- Platform invariants shall never be configurable.
- Tenant policies shall remain Tenant isolated.
- Institution policies shall not weaken Tenant policies.
- Configuration changes shall be versioned.
- Every configuration change shall be audited.
- Runtime services shall evaluate only active configurations.
- Invalid configurations shall not be activated.

---

# Published Events

Configuration services publish:

- PolicyCreated
- PersonPolicyUpdated
- PolicyActivated
- PolicyArchived
- ConfigurationImported
- ConfigurationExported
- FeatureFlagEnabled
- FeatureFlagDisabled
- IdentityPolicyChanged

Business domains may subscribe where operational behavior depends on policy changes.

---

# Architectural Constraints

- Configuration shall never alter aggregate ownership.
- Configuration shall never modify immutable identifiers.
- Configuration shall never redefine event contracts.
- Governance rules shall remain centrally enforced.
- Platform invariants shall remain protected across all deployments.
- Configuration services shall support backward compatibility across platform upgrades.

---

# Bulk Operations, Migration & Identity Synchronization

The Person & Identity domain supports large-scale identity processing while preserving canonical identity integrity, auditability, and platform consistency.

This capability consists of three independent operational areas:

- Bulk Operations
- Data Migration
- Identity Synchronization

Each capability follows its own workflows, validation rules, and operational controls.

---

# Operational Architecture

```

                Source

                   │

───────────────────┼───────────────────

 Bulk Jobs   Migration   Synchronization

───────────────────┼───────────────────

                   │

        Validation Pipeline

                   │

 Duplicate Detection Engine

                   │

 Business Rule Validation

                   │

 Person & Identity Domain

                   │

 Domain Events

```

---

# Bulk Operations

Bulk Operations execute high-volume identity changes initiated within the platform.

Typical operations include:

- Bulk Person Creation
- Bulk Person Update
- Bulk Role Assignment
- Bulk Role Revocation
- Bulk Identity Verification
- Bulk Account Activation
- Bulk Account Deactivation
- Bulk Password Reset
- Bulk Archive

Bulk operations execute through controlled background jobs.

---

# Bulk Processing Workflow

```

Upload Request

↓

Validate File

↓

Validate Schema

↓

Duplicate Detection

↓

Business Rule Validation

↓

Create Processing Job

↓

Execute in Batches

↓

Publish Events

↓

Generate Processing Report

```

---

# Batch Processing

Large datasets shall be processed in configurable batches.

Typical batch activities include:

- Read Batch
- Validate Batch
- Persist Batch
- Publish Events
- Commit Batch

Failures shall be isolated to the affected batch whenever possible.

---

# Bulk Validation

Each record undergoes:

- Mandatory Field Validation
- Tenant Validation
- Duplicate Detection
- Identity Validation
- Business Rule Validation
- Authorization Validation

Records failing validation shall not prevent processing of valid records unless the configured job policy requires fail-fast execution.

---

# Processing Modes

Supported execution modes include:

- Validate Only
- Dry Run
- Execute
- Resume Failed Job
- Retry Failed Records

Dry Run performs all validations without persisting data.

---

# Bulk Error Handling

Every rejected record shall contain:

- Record Number
- Error Code
- Error Description
- Validation Stage
- Suggested Resolution

Error reporting remains available after job completion.

---

# Migration

Migration imports identity data from legacy systems during implementation or consolidation projects.

Migration is intended for controlled onboarding rather than routine operations.

---

# Migration Workflow

```

Extract Legacy Data

↓

Transform

↓

Validate

↓

Duplicate Detection

↓

Preview

↓

Administrative Approval

↓

Migration Execution

↓

Verification

↓

Completion Report

```

---

# Migration Principles

Migration shall:

- Preserve canonical Person identity
- Preserve auditability where available
- Prevent duplicate creation
- Validate Tenant ownership
- Generate new internal identifiers where required

Legacy identifiers may be retained as external references.

---

# Legacy Identifier Mapping

The platform maintains mapping between:

- Legacy Person Identifier
- Legacy Employee Identifier
- Legacy Student Identifier

and

- Canonical PersonID
- Business Identifier

This mapping supports traceability during post-migration activities.

---

# Migration Validation

Migration validation includes:

- Schema Validation
- Data Type Validation
- Mandatory Attributes
- Duplicate Detection
- Reference Validation
- Identity Quality Assessment

Invalid records require correction before migration.

---

# Migration Rollback

Rollback capability depends on migration stage.

Supported options include:

- Cancel Before Execution
- Rollback Current Batch
- Administrative Recovery

Completed migrations are reversed only through approved corrective workflows rather than direct database rollback.

---

# Identity Synchronization

Identity Synchronization exchanges identity information with external systems.

Synchronization may operate in:

- Scheduled Mode
- Event-Driven Mode
- Manual Mode

Synchronization does not bypass canonical business rules.

---

# Synchronization Sources

Potential sources include:

- HR Systems
- Student Information Systems
- Government Identity Services
- Enterprise Directories
- LDAP
- Active Directory
- SCIM Providers

Support depends on deployment configuration.

---

# Synchronization Workflow

```

Receive External Data

↓

Validate Source

↓

Validate Payload

↓

Resolve Identity

↓

Duplicate Detection

↓

Apply Business Rules

↓

Persist Changes

↓

Publish Events

```

---

# Synchronization Strategies

Supported synchronization strategies include:

- Full Synchronization
- Incremental Synchronization
- Delta Synchronization

Selection depends on integration requirements.

---

# Conflict Resolution

Conflicts may occur when:

- Local data differs from external data.
- Multiple systems update the same identity.
- Duplicate identities are detected.

Conflict policies may include:

- Local Wins
- External Wins
- Manual Review
- Attribute-Level Resolution

Conflict handling is configurable.

---

# Idempotency

Bulk operations, migration, and synchronization shall support idempotent processing where applicable.

Repeated processing of the same logical request shall not create duplicate Person records.

Idempotency keys or equivalent mechanisms shall be used where supported.

---

# Job Management

Every processing job maintains:

- Job ID
- Job Type
- Submitted By
- Submission Time
- Current Status
- Progress
- Success Count
- Failure Count
- Completion Time

Jobs remain queryable after completion.

---

# Job Lifecycle

```

Submitted

↓

Validated

↓

Queued

↓

Running

↓

Completed

↓

Archived

```

Alternative terminal states include:

- Failed
- Cancelled
- Partially Completed

---

# Progress Monitoring

Processing jobs expose:

- Percentage Complete
- Records Processed
- Records Remaining
- Current Batch
- Estimated Completion Time

Progress information supports operational dashboards.

---

# Notifications

The platform may notify administrators when:

- Job Completed
- Job Failed
- Validation Failed
- Migration Completed
- Synchronization Interrupted

Notification mechanisms remain configurable.

---

# Business Rules

The following rules are mandatory.

- Every imported Person shall undergo duplicate detection.
- Bulk operations shall execute through background processing.
- Migration shall preserve referential integrity.
- Synchronization shall not bypass domain validation.
- Every processing job shall be auditable.
- Failed records shall remain traceable.
- Canonical Person ownership shall remain unchanged.

---

# Published Events

Operational processing publishes:

- BulkJobSubmitted
- BulkJobStarted
- BulkJobCompleted
- BulkJobFailed
- PersonMigrationStarted
- PersonMigrationCompleted
- PersonSynchronizationStarted
- PersonSynchronizationCompleted
- SynchronizationConflictDetected
- IdentityImported

Business domains consume these events through the platform event bus.

---

# Architectural Constraints

- Bulk processing shall not bypass application services.
- Migration shall never modify immutable identifiers.
- Synchronization shall preserve aggregate boundaries.
- Background processing shall support resumable execution.
- Processing jobs shall remain tenant isolated.
- All operational activities shall be fully auditable.

---

# Exception Handling, Recovery & Administrative Operations

The Person & Identity domain shall provide standardized mechanisms for handling exceptional conditions while preserving data integrity, security, auditability, and business continuity.

Exception handling shall distinguish between business failures, technical failures, and administrative interventions.

Recovery workflows shall restore operational capability without violating architectural invariants.

---

# Exception Architecture

```

Business Request

↓

Validation

↓

Business Exception

↓

Technical Exception

↓

Administrative Exception

↓

Recovery Workflow

↓

Audit

↓

Notification

```

Every exception shall be classified before resolution.

---

# Exception Categories

The platform recognizes the following exception categories.

## Business Exceptions

Business rule violations.

Examples:

- Duplicate Person
- Invalid Lifecycle Transition
- Role Assignment Conflict
- Verification Failure
- Unauthorized Context
- Expired Role
- Invalid Tenant

Business exceptions do not indicate platform failure.

---

## Technical Exceptions

Infrastructure or platform failures.

Examples:

- Database unavailable
- Cache unavailable
- Event Bus unavailable
- External API timeout
- Storage failure
- Network interruption

Technical exceptions require operational recovery mechanisms.

---

## Administrative Exceptions

Controlled administrative interventions.

Examples:

- Force Unlock Account
- Manual Verification Override
- Duplicate Override
- Emergency Password Reset
- Emergency Context Recovery
- Manual Merge Approval

Administrative exceptions always require authorization.

---

# Exception Severity

Each exception shall have an assigned severity.

| Severity | Description |
|----------|-------------|
| Informational | No operational impact |
| Warning | Recoverable business issue |
| Error | Operation failed |
| Critical | Immediate administrative attention required |
| Emergency | Platform continuity or security risk |

Severity influences alerting and escalation.

---

# Business Exception Handling

Business exceptions follow this workflow.

```

Validation Failure

↓

Business Rule Evaluation

↓

Reject Operation

↓

Return Business Error

↓

Audit

↓

Continue Processing

```

Business exceptions shall never corrupt domain state.

---

# Technical Exception Handling

Technical failures follow this workflow.

```

Operation

↓

Infrastructure Failure

↓

Retry Policy

↓

Recovery Attempt

↓

Escalation

↓

Administrative Notification

```

Technical failures shall preserve transactional consistency.

---

# Retry Policies

Retry behavior is configurable.

Supported strategies include:

- Immediate Retry
- Delayed Retry
- Exponential Backoff
- Scheduled Retry
- Manual Retry

Retries shall never violate idempotency guarantees.

---

# Circuit Protection

Repeated failures may trigger circuit protection.

States include:

```

Closed

↓

Open

↓

Half Open

↓

Closed

```

Circuit protection prevents cascading failures across dependent services.

---

# Recovery Workflows

Recovery workflows restore operational capability after exceptional conditions.

Supported recovery processes include:

- Session Recovery
- Identity Recovery
- Account Recovery
- Password Recovery
- Synchronization Recovery
- Background Job Recovery

Recovery actions shall be fully auditable.

---

# Account Recovery

Account recovery may include:

- Password Reset
- Account Unlock
- MFA Re-enrollment
- Credential Replacement
- Identity Re-verification

Recovery workflows require identity validation before execution.

---

# Session Recovery

Session recovery may occur when:

- Session corruption detected
- Context invalidated
- Authentication token expired
- Infrastructure interruption

Recovery shall never bypass authentication requirements.

---

# Background Job Recovery

Bulk processing jobs support recovery through:

- Resume Execution
- Retry Failed Batch
- Restart Job
- Administrative Cancellation

Completed batches shall not be reprocessed unless explicitly requested.

---

# Administrative Operations

Authorized administrators may perform controlled recovery actions.

Examples include:

- Unlock Account
- Force Logout
- Revoke Sessions
- Reset Password
- Activate Identity
- Suspend Identity
- Archive Person
- Restore Configuration
- Resume Synchronization

Every action requires authorization and audit.

---

# Emergency Operations

Emergency operations are exceptional administrative procedures.

Examples:

- Emergency Administrative Access
- Force Session Termination
- Global Session Revocation
- Identity Lockdown
- Tenant-wide Authentication Suspension

Emergency operations require elevated authorization and complete traceability.

---

# Escalation Workflow

```

Exception Detected

↓

Severity Assessment

↓

Automatic Recovery

↓

Yes → Execute Recovery

↓

No

↓

Administrative Escalation

↓

Resolution

↓

Audit Closure

```

Escalation rules remain configurable.

---

# Notifications

Exception handling may notify:

- Tenant Administrator
- Institution Administrator
- Security Administrator
- System Operations Team
- Platform Monitoring Services

Notification channels remain configurable.

---

# Failure Isolation

Failures shall be isolated wherever possible.

Examples:

- Failed bulk record shall not stop unrelated records.
- Failed synchronization shall not block authentication.
- Failed session shall not affect independent sessions.
- Failed integration shall not corrupt Person data.

Isolation minimizes operational impact.

---

# Compensation

Where distributed operations cannot complete atomically, compensation workflows may be executed.

Examples:

- Revoke partially assigned roles
- Reverse incomplete synchronization
- Cancel unfinished provisioning
- Restore previous configuration state

Compensation actions shall be deterministic and auditable.

---

# Manual Override

Certain operations may be manually overridden.

Supported overrides include:

- Duplicate Warning Override
- Verification Override
- Account Unlock Override
- Context Override
- Migration Approval Override

Overrides require:

- Authorized approver
- Recorded justification
- Timestamp
- Audit reference

Overrides shall never violate platform invariants.

---

# Exception Reporting

Operational reporting may include:

- Exception Frequency
- Recovery Success Rate
- Retry Statistics
- Administrative Overrides
- Failed Jobs
- Unresolved Exceptions
- Mean Time to Recovery (MTTR)

Reports support operational governance.

---

# Business Rules

The following rules are mandatory.

- Every exception shall be classified.
- Technical failures shall preserve transactional integrity.
- Administrative overrides shall require authorization.
- Recovery workflows shall not bypass security policies.
- Every recovery action shall be auditable.
- Retry mechanisms shall remain idempotent.
- Platform invariants shall never be overridden.

---

# Published Events

Exception management publishes:

- BusinessExceptionRaised
- TechnicalExceptionRaised
- AdministrativeOverrideRequested
- AdministrativeOverrideApproved
- RecoveryStarted
- RecoveryCompleted
- RecoveryFailed
- PersonRetryScheduled
- PersonRetryCompleted
- CircuitOpened
- CircuitClosed

These events enable centralized operational monitoring and governance.

---

# Architectural Constraints

- Exception handling shall remain independent of business workflows.
- Recovery mechanisms shall preserve canonical identity integrity.
- Administrative operations shall never bypass aggregate boundaries.
- Exception policies shall be configurable without changing domain logic.
- Every exception lifecycle shall be traceable through immutable audit records.
- Recovery workflows shall support horizontal scalability and distributed execution.

---

# Reporting, Search & Analytics

The Person & Identity domain provides standardized capabilities for operational search, management reporting, and analytical insights.

These capabilities consume read models and analytical projections derived from canonical domain events.

Reporting, search, and analytics shall never directly query transactional aggregates for complex or historical analysis.

---

# Reporting Architecture

```

             Person & Identity Domain

                      │

               Domain Events

                      │

──────────────────────┼──────────────────────

      Read Models      Analytics Models

──────────────────────┼──────────────────────

        │                    │

   Operational Search   Reports & Dashboards

```

The reporting architecture separates transactional processing from analytical workloads.

---

# Reporting Principles

Reporting shall follow these principles.

- Read-only access
- Event-driven projections
- Tenant isolation
- Configurable filters
- Historical consistency
- Non-blocking execution
- Scalable query performance

Transactional aggregates remain optimized for writes rather than reporting.

---

# Search Architecture

Search supports operational activities requiring rapid retrieval of identity information.

Typical search operations include:

- Search Person
- Search User Account
- Search Identity
- Search Role Assignment
- Search Active Sessions
- Search Verification Requests

Search results are optimized for interactive user experiences.

---

# Search Criteria

Supported search attributes include:

- PersonID
- Business Identifier
- Full Name
- Mobile Number
- Email Address
- Government Identifier
- Employee Number
- Student Number
- Username
- User Account Status
- Verification Status
- Person Type

Search criteria are configurable according to Tenant policies.

---

# Advanced Search

Advanced search supports combinations of multiple criteria.

Examples include:

- Active Students in Institution
- Faculty with Expired Verification
- Locked User Accounts
- Persons Pending Activation
- Duplicate Candidates
- Archived Persons

Advanced search executes against optimized read models.

---

# Search Result Model

Search results may include:

- Person Summary
- Identity Status
- User Account Status
- Active Roles
- Active Context
- Verification Level
- Last Login
- Data Quality Score

Sensitive information shall be returned only when authorized.

---

# Reporting Categories

The platform supports multiple reporting categories.

## Identity Reports

Examples:

- Total Persons
- Active Persons
- Archived Persons
- New Persons by Period
- Persons by Type

---

## Authentication Reports

Examples:

- Successful Logins
- Failed Logins
- MFA Adoption
- Locked Accounts
- Password Reset Requests

---

## Authorization Reports

Examples:

- Role Assignments
- Role Expirations
- Permission Distribution
- Context Usage
- Access Denials

---

## Verification Reports

Examples:

- Verification Completion Rate
- Pending Verification Requests
- Verification Failures
- Verification by Person Type

---

## Governance Reports

Examples:

- Duplicate Candidates
- Merge Operations
- Split Operations
- Administrative Overrides
- Identity Quality Score Distribution

---

## Security Reports

Examples:

- Suspicious Login Attempts
- Security Policy Violations
- Session Revocations
- Administrative Security Actions

---

# Dashboard Architecture

Dashboards present aggregated operational information.

Typical dashboards include:

Identity Dashboard

- Active Persons
- Verification Status
- Duplicate Candidates

Security Dashboard

- Login Trends
- Account Lockouts
- MFA Adoption
- Security Incidents

Administration Dashboard

- Pending Approvals
- Bulk Jobs
- Synchronization Status
- Failed Operations

Operations Dashboard

- Active Sessions
- Service Health
- Event Throughput
- Recovery Operations

Dashboards consume projection models rather than transactional entities.

---

# Analytics

Analytics provides strategic insights into identity operations.

Examples include:

- Identity Growth Trends
- Authentication Trends
- Verification Trends
- Role Utilization
- Policy Adoption
- Administrative Workload
- Data Quality Trends

Analytics supports planning and governance rather than operational processing.

---

# KPI Framework

Example identity KPIs include:

Identity KPIs

- Total Persons
- Active Identity Ratio
- Identity Growth Rate

Security KPIs

- Authentication Success Rate
- MFA Adoption Rate
- Account Lock Frequency

Governance KPIs

- Duplicate Detection Rate
- Merge Success Rate
- Verification Completion Rate

Operational KPIs

- Average Login Time
- Session Duration
- Recovery Time
- Bulk Job Success Rate

KPIs are configurable according to Tenant requirements.

---

# Scheduled Reports

Reports may execute:

- On Demand
- Daily
- Weekly
- Monthly
- Quarterly
- Annually

Scheduling remains configurable.

---

# Export Capabilities

Supported export formats may include:

- PDF
- CSV
- XLSX
- JSON

Exports shall respect authorization and data privacy policies.

---

# Data Privacy

Reporting shall enforce:

- Tenant isolation
- Role-based visibility
- Sensitive attribute masking
- Audit logging for report generation
- Export authorization

Reports shall never expose unauthorized identity information.

---

# Report Retention

Generated reports may follow configurable retention policies.

Examples include:

- Temporary Reports
- Operational Reports
- Compliance Reports
- Scheduled Reports

Retention policies remain independent from transactional data retention.

---

# Business Rules

The following rules are mandatory.

- Reporting shall consume read models.
- Search shall remain optimized for operational usage.
- Analytics shall not affect transactional performance.
- Every report shall enforce authorization.
- Sensitive information shall be masked where required.
- Generated reports shall be auditable.
- Tenant isolation shall apply to all reporting capabilities.

---

# Published Events

Reporting services publish:

- ReportGenerated
- ReportExported
- DashboardViewed
- SearchExecuted
- AnalyticsCalculated
- KPIThresholdExceeded

These events support operational monitoring and governance.

---

# Architectural Constraints

- Transactional aggregates shall not be queried directly for analytical workloads.
- Reporting models shall remain eventually consistent with domain events.
- Search indexes shall be rebuildable from canonical data.
- Dashboards shall consume projection models.
- Analytics shall remain read-only.
- Reporting services shall support horizontal scalability.

--- 

# Non-Functional Requirements

The Person & Identity domain shall satisfy the platform-wide non-functional requirements defined for the SutraERP architecture.

These requirements establish measurable quality attributes governing performance, scalability, reliability, availability, security, maintainability, interoperability, and operational excellence.

All implementations of this domain shall comply with these architectural constraints.

---

# NFR Philosophy

The Person & Identity domain is a foundational platform capability.

Every business domain depends upon it.

Therefore, identity services shall prioritize:

- Availability
- Reliability
- Security
- Consistency
- Scalability
- Observability

over feature-specific optimizations.

---

# Quality Attribute Model

The domain shall be evaluated against the following quality attributes.

```

Security

↓

Availability

↓

Reliability

↓

Performance

↓

Scalability

↓

Maintainability

↓

Observability

↓

Interoperability

↓

Recoverability

↓

Extensibility

```

Each attribute contributes to overall platform quality.

---

# Availability

Identity services shall remain continuously available to support platform authentication and authorization.

Availability requirements include:

- High service availability
- Redundant deployment support
- Graceful degradation where applicable
- Health monitoring
- Automatic recovery

Availability targets are defined by platform operational policies.

---

# Reliability

Identity operations shall execute predictably and consistently.

Reliability objectives include:

- Deterministic workflow execution
- Transaction integrity
- Event publication guarantees
- Consistent authorization decisions
- Stable identity resolution

Every completed operation shall leave the system in a valid state.

---

# Performance

Performance requirements apply to interactive identity operations.

Typical operations include:

- Authentication
- Authorization
- Person Search
- Context Resolution
- Session Validation

Performance objectives shall be measured through platform monitoring.

Implementation-specific latency targets are deployment responsibilities.

---

# Scalability

The architecture shall support horizontal scaling.

Scalable components include:

- Authentication Services
- Authorization Engine
- Session Management
- Search Services
- Read Models
- Event Consumers

Scaling shall not require changes to domain models.

---

# Consistency

The Person aggregate remains the canonical source of identity information.

Consistency requirements include:

- Strong consistency within aggregate boundaries
- Eventual consistency across read models
- Eventual consistency across business domains

Aggregate invariants shall always be preserved.

---

# Concurrency

Concurrent operations shall preserve identity integrity.

Concurrent updates shall support:

- Optimistic Concurrency
- Version Validation
- Conflict Detection
- Safe Retry

Concurrent writes shall never produce duplicate Person records.

---

# Security

Security requirements include:

- Strong Authentication
- Scoped Authorization
- Tenant Isolation
- Secure Credential Storage
- Auditability
- Encryption Support
- Privacy Protection

Security policies are centrally governed.

---

# Privacy

Personally identifiable information shall be protected according to applicable organizational policies.

Privacy requirements include:

- Data minimization
- Purpose limitation
- Attribute masking
- Controlled access
- Configurable retention
- Auditability

Privacy controls apply throughout the identity lifecycle.

---

# Maintainability

The architecture shall remain maintainable through:

- Modular services
- Clear aggregate boundaries
- Stable service contracts
- Immutable domain events
- Versioned APIs
- Standardized workflows

Changes shall minimize impact on downstream domains.

---

# Extensibility

The architecture shall support future capabilities without redesign.

Examples include:

- Passkeys
- Biometric Authentication
- Additional Identity Providers
- New Verification Methods
- Additional Person Types
- Advanced Security Policies

Extensions shall preserve existing contracts.

---

# Interoperability

Identity services shall integrate with:

- Internal Business Domains
- Enterprise Identity Providers
- Government Services
- External Directories
- Third-party Applications

Integration occurs through approved APIs and events.

---

# Observability

Identity services shall provide:

- Structured Logs
- Metrics
- Traces
- Audit Records
- Health Indicators
- Operational Alerts

Observability shall support operational governance and troubleshooting.

---

# Recoverability

Recovery capabilities include:

- Session Recovery
- Account Recovery
- Job Recovery
- Synchronization Recovery
- Administrative Recovery

Recovery shall preserve transactional consistency.

---

# Fault Tolerance

The platform shall tolerate localized failures through:

- Retry Policies
- Circuit Protection
- Background Processing
- Failure Isolation
- Graceful Degradation

Failures shall not compromise identity integrity.

---

# Deployability

The architecture shall support:

- Independent Service Deployment
- Rolling Upgrades
- Backward-Compatible API Evolution
- Zero-Downtime Deployment (where infrastructure permits)

Deployment strategies remain infrastructure concerns.

---

# Portability

The Person & Identity domain shall remain deployment-independent.

The architecture shall avoid coupling to:

- Specific database vendors
- Specific messaging platforms
- Specific authentication providers
- Specific cloud providers

Infrastructure choices shall not affect domain logic.

---

# Testability

Identity services shall support:

- Unit Testing
- Integration Testing
- Contract Testing
- Event Testing
- Performance Testing
- Security Testing

Business rules shall be independently testable.

---

# Configuration Quality

Configuration mechanisms shall provide:

- Version Control
- Validation
- Rollback Support
- Tenant Isolation
- Auditability

Configuration changes shall never compromise platform invariants.

---

# Compliance

The architecture shall support organizational compliance through:

- Immutable Audit Records
- Configurable Retention Policies
- Identity Traceability
- Administrative Accountability
- Security Governance

Compliance implementation remains deployment-specific.

---

# Capacity Planning

Operational planning shall consider:

- Number of Persons
- Concurrent Sessions
- Authentication Volume
- Authorization Requests
- Event Throughput
- Bulk Processing Load

Capacity planning supports sustainable platform growth.

---

# Engineering Standards

The implementation shall follow platform engineering standards.

Examples include:

- Consistent API Design
- Standard Error Handling
- Versioned Contracts
- Structured Logging
- Immutable Events
- Automated Testing
- Secure Development Practices

These standards apply uniformly across the identity domain.

---

# NFR Validation Matrix

| Quality Attribute | Validation Method |
|-------------------|------------------|
| Availability | Health Monitoring & Uptime Measurement |
| Reliability | Workflow Verification & Failure Analysis |
| Performance | Load & Performance Testing |
| Scalability | Horizontal Scaling Validation |
| Security | Security Assessment & Penetration Testing |
| Privacy | Access Review & Data Protection Audit |
| Maintainability | Architecture Review |
| Extensibility | Backward Compatibility Verification |
| Observability | Monitoring & Alert Validation |
| Recoverability | Disaster Recovery & Recovery Drills |

The validation approach shall be incorporated into the platform quality assurance process.

---

# Business Rules

The following rules are mandatory.

- Canonical identity integrity shall never be compromised for performance optimization.
- Aggregate boundaries shall remain protected under all deployment models.
- All service contracts shall remain backward compatible unless versioned.
- Identity services shall remain observable and auditable.
- Horizontal scaling shall not alter business behavior.
- Infrastructure dependencies shall remain replaceable.
- Platform invariants shall take precedence over implementation optimizations.

---

# Architectural Constraints

- Non-functional requirements are architectural contracts, not implementation recommendations.
- Every downstream domain shall inherit these quality standards.
- Implementation technologies may change without affecting domain behavior.
- Operational characteristics shall be continuously measurable.
- Quality attributes shall be validated throughout the software lifecycle.

---

# Multi-Domain Interaction Matrix

The Person & Identity domain serves as the canonical identity provider for the entire SutraERP platform.

Every business domain interacts with Person & Identity through approved APIs, published events, and read models.

Business domains shall never modify canonical identity data directly.

---

# Multi-Domain Interaction Principles

The following principles govern all domain interactions.

- Person & Identity owns canonical Person records.
- Business domains own business-specific data.
- Multi-domain communication occurs through APIs and domain events.
- Aggregate ownership shall never be violated.
- Circular dependencies are prohibited.
- Downstream domains consume identity services but do not redefine them.

---

# Interaction Types

Interactions are classified as:

- Command
- Query
- Event Subscription
- Event Publication
- Reference Lookup

Each interaction has defined ownership and direction.

---

# Canonical Ownership Matrix

| Data / Capability | Canonical Owner | Consumer Domains |
|-------------------|-----------------|------------------|
| Person | Person & Identity | All Domains |
| Identity | Person & Identity | All Domains |
| User Account | Person & Identity | All Domains |
| Authentication | L2-A03 Authentication | All Domains |
| Authorization | L2-A04 Authorization & Policy | All Domains |
| Security Context | L2-A03 / L2-A04 | All Domains |
| Student Enrollment | Student Lifecycle | Academics, Exams, Finance |
| Employee Assignment | HR | Payroll, Attendance |
| Institution | Institution | All Domains |
| Campus | Campus | All Domains |
| Department | Organization | HR, Academics |
| Course Registration | Academic Registration | Examination, Finance |
| Examination Registration | Examination | Results, Finance |

Ownership is singular and unambiguous.

---

# Domain Interaction Matrix

| Domain | Reads Identity | Sends Commands | Subscribes to Events | Owns Identity Data |
|---------|----------------|----------------|----------------------|--------------------|
| Platform Foundation | Yes | No | Yes | No |
| Institution | Yes | No | Yes | No |
| Organization | Yes | No | Yes | No |
| Campus | Yes | No | Yes | No |
| Student Lifecycle | Yes | Approved APIs | Yes | No |
| Academic Registration | Yes | Approved APIs | Yes | No |
| Examination | Yes | No | Yes | No |
| Finance | Yes | No | Yes | No |
| HR | Yes | Approved APIs | Yes | No |
| Library | Yes | No | Yes | No |
| Hostel | Yes | No | Yes | No |
| Transport | Yes | No | Yes | No |
| Placement | Yes | No | Yes | No |
| Research | Yes | No | Yes | No |
| Alumni | Yes | Approved APIs | Yes | No |

Identity ownership always remains within the Person & Identity domain.

---

# API Consumption Matrix

The following APIs may be consumed by business domains.

| Service | Consumer Examples |
|----------|-------------------|
| Resolve Person | Admissions, HR, Library |
| Search Person | Administration, Student Services |
| Resolve Active Context | All Domains |
| Resolve Roles | Authorization Consumers |
| Validate Session | Web Portal, Mobile Apps |
| Validate Permission | Business Services |

Write operations require application services and workflow validation.

---

# Event Publication Matrix

The Person & Identity domain publishes:

- PersonCreated
- PersonUpdated
- PersonActivated
- PersonArchived
- IdentityVerified
- UserAccountCreated
- RoleAssigned
- RoleRevoked
- ContextChanged
- SessionStarted
- SessionEnded
- AccountLocked
- AccountUnlocked

These events form the canonical integration contract.

---

# Event Subscription Matrix

The Person & Identity domain subscribes only to events required for identity context resolution.

Examples include:

Student Lifecycle

- StudentEnrolled
- QualificationAwarded
- StudentWithdrawn

HR

- EmployeeJoined
- EmployeeTransferred
- EmployeeSeparated

Institution

- InstitutionCreated
- InstitutionArchived

Organization

- DepartmentCreated
- DepartmentArchived

The Person & Identity domain consumes these events without taking ownership of the originating data.

---

# Dependency Direction

Allowed dependency direction:

```

Business Domain

↓

Identity APIs

↓

Person & Identity

↓

Published Events

↓

Business Domains

```

Reverse ownership dependencies are prohibited.

---

# Context Resolution Dependencies

Context Resolution consumes:

- Institution Membership
- Employment Assignment
- Student Enrollment
- Committee Membership
- Project Assignment

These references remain owned by their originating domains.

---

# Data Ownership Rules

Business domains may:

- Reference PersonID
- Resolve Person
- Query Identity Status
- Validate Authorization
- Subscribe to Identity Events

Business domains shall not:

- Modify Person
- Create Identity directly
- Change User Account state
- Assign internal identity identifiers
- Bypass identity workflows

---

# Integration Responsibility Matrix

| Capability | Owner | Consumers |
|------------|-------|-----------|
| Authentication | L2-A03 Authentication | All Domains |
| Authorization | L2-A04 Authorization & Policy | All Domains |
| Session Management | L2-A03 Authentication | All Domains |
| Identity Verification | Person & Identity | Admissions, HR |
| Duplicate Detection | Person & Identity | Import Services |
| Context Resolution | L2-A03 Authentication | All Domains |

---

# Business Scenario Example

Student Admission

```

Admissions

↓

Create Person (API)

↓

Person Created

↓

Publish PersonCreated

↓

Student Lifecycle

↓

Student Enrollment

↓

Publish StudentEnrolled

↓

Context Resolution Updated

```

Each domain executes only its own responsibilities.

---

# Dependency Rules

The following rules are mandatory.

- Every business domain references Person using PersonID.
- Identity data shall never be duplicated as a source of truth.
- Business domains shall consume only published contracts.
- Domain ownership shall remain singular.
- Multi-domain writes shall execute through application services.
- Circular dependencies are prohibited.
- Integration shall remain event-driven wherever practical.

---

# Published Contracts

The Person & Identity domain guarantees:

- Stable API contracts
- Stable event contracts
- Stable identifier semantics
- Stable authorization contracts
- Stable context resolution contracts

Breaking changes require formal versioning.

---

# Architectural Constraints

- Person & Identity is the canonical identity provider.
- Business domains remain autonomous within their own aggregate boundaries.
- Multi-domain interactions shall preserve loose coupling.
- Ownership shall never transfer through integration.
- Every dependency shall be explicitly documented and governed.
- Integration contracts shall remain backward compatible.

---

# End-to-End Sequence Diagrams & Canonical Workflows

This section defines the canonical operational workflows for the Person & Identity domain.

These workflows illustrate the interaction between users, application services, domain services, event publication, and downstream business domains.

The workflows serve as implementation references for all SutraERP modules.

---

# Workflow Principles

Every workflow shall adhere to the following principles.

- Person remains the canonical identity.
- Aggregate boundaries are preserved.
- Business validation precedes persistence.
- Domain events are published after successful state changes.
- Every workflow is auditable.
- Authorization is evaluated before protected operations.
- Failures trigger controlled recovery workflows.

---

# Workflow 1 — Person Creation

```

Administrator

      │

      ▼

Create Person Request

      │

      ▼

Validate Tenant

      │

      ▼

Duplicate Detection

      │

      ▼

Validate Attributes

      │

      ▼

Generate PersonID

      │

      ▼

Persist Person

      │

      ▼

Publish PersonCreated

      │

      ▼

Audit Record

      │

      ▼

Completed

```

---

# Workflow 2 — Identity Activation

```

Verified Person

      │

      ▼

Create Identity

      │

      ▼

Create User Account

      │

      ▼

Assign Initial Role

      │

      ▼

Resolve Available Contexts

      │

      ▼

Activate Identity

      │

      ▼

Publish PersonActivated

      │

      ▼

Completed

```

---

# Workflow 3 — User Authentication

```

User

      │

      ▼

Submit Credentials

      │

      ▼

Locate User Account

      │

      ▼

Validate Password

      │

      ▼

Evaluate MFA Policy

      │

      ▼

MFA Validation

      │

      ▼

Create Session

      │

      ▼

Resolve Active Context

      │

      ▼

Publish AuthenticationSucceeded

      │

      ▼

Access Granted

```

---

# Workflow 4 — Authorization Decision

```

Business Request

      │

      ▼

Resolve Session

      │

      ▼

Resolve Active Context

      │

      ▼

Resolve Roles

      │

      ▼

Resolve Permissions

      │

      ▼

Evaluate Policies

      │

      ▼

Allow / Deny

      │

      ▼

Audit Decision

```

---

# Workflow 5 — Context Switching

```

Authenticated User

      │

      ▼

Request Context Switch

      │

      ▼

Retrieve Available Contexts

      │

      ▼

Validate Selection

      │

      ▼

Recalculate Permissions

      │

      ▼

Refresh Security Context

      │

      ▼

Publish ContextChanged

      │

      ▼

Continue Session

```

---

# Workflow 6 — Duplicate Detection

```

Create Person Request

      │

      ▼

Matching Engine

      │

      ▼

Potential Duplicate

      │

 ┌────┴────┐

 │         │

No        Yes

 │         │

 ▼         ▼

Create   Administrative Review

 │         │

 ▼         ▼

Done   Merge Recommendation

```

---

# Workflow 7 — Person Merge

```

Duplicate Confirmed

      │

      ▼

Select Canonical Person

      │

      ▼

Validate Dependencies

      │

      ▼

Transfer References

      │

      ▼

Archive Duplicate

      │

      ▼

Publish PersonMerged

      │

      ▼

Audit

```

---

# Workflow 8 — Person Archive

```

Archive Request

      │

      ▼

Authorization Check

      │

      ▼

Dependency Validation

      │

      ▼

Terminate Sessions

      │

      ▼

Disable Identity

      │

      ▼

Archive Person

      │

      ▼

Publish PersonArchived

```

---

# Workflow 9 — Bulk Import

```

Upload File

      │

      ▼

Schema Validation

      │

      ▼

Duplicate Detection

      │

      ▼

Business Validation

      │

      ▼

Background Processing

      │

      ▼

Persist Records

      │

      ▼

Publish Events

      │

      ▼

Processing Report

```

---

# Workflow 10 — External Identity Synchronization

```

External Source

      │

      ▼

Receive Payload

      │

      ▼

Validate Source

      │

      ▼

Resolve Person

      │

      ▼

Duplicate Detection

      │

      ▼

Apply Changes

      │

      ▼

Publish IdentityUpdated

```

---

# Workflow 11 — Password Reset

```

Password Reset Request

      │

      ▼

Identity Verification

      │

      ▼

Authorization Policy

      │

      ▼

Generate Reset Token

      │

      ▼

Reset Password

      │

      ▼

Invalidate Sessions

      │

      ▼

Publish PasswordChanged

```

---

# Workflow 12 — Administrative Override

```

Administrator

      │

      ▼

Override Request

      │

      ▼

Authorization Validation

      │

      ▼

Approval Workflow

      │

      ▼

Execute Override

      │

      ▼

Generate Audit Record

      │

      ▼

Publish Override Event

```

---

# Workflow 13 — Exception Recovery

```

Exception Raised

      │

      ▼

Classify Exception

      │

      ▼

Retry Possible

      │

 ┌────┴─────┐

 │          │

Yes        No

 │          │

 ▼          ▼

Retry   Administrative Recovery

 │          │

 └────┬─────┘

      ▼

Audit

      │

      ▼

Completed

```

---

# Workflow 14 — Session Expiration

```

Session Active

      │

      ▼

Timeout Reached

      │

      ▼

Terminate Session

      │

      ▼

Release Security Context

      │

      ▼

Publish PersonSessionExpired

      │

      ▼

User Re-authentication Required

```

---

# Canonical Workflow Catalog

| Workflow | Primary Aggregate | Published Events |
|-----------|-------------------|------------------|
| Person Creation | Person | PersonCreated |
| Identity Activation | Identity | PersonActivated |
| Authentication | User Account | AuthenticationSucceeded |
| Authorization | Security Context | AuthorizationSucceeded / AuthorizationDenied |
| Context Switch | Security Context | ContextChanged |
| Duplicate Detection | Person | DuplicateCandidateDetected |
| Merge | Person | PersonMerged |
| Archive | Person | PersonArchived |
| Bulk Import | Person | IdentityImported |
| Synchronization | Person | IdentityUpdated |
| Password Reset | User Account | PasswordChanged |
| Administrative Override | Identity | AdministrativeOverrideApproved |
| Exception Recovery | Recovery Service | RecoveryCompleted |
| Session Expiration | Session | PersonSessionExpired |

---

# Business Scenario Integration

The following domains participate in selected workflows.

| Workflow | External Domains |
|----------|------------------|
| Person Creation | Student Lifecycle, HR |
| Identity Activation | Authorization |
| Authentication | All Domains |
| Context Switching | Institution, HR, Student Lifecycle |
| Merge | Finance, Library, Examination |
| Synchronization | External Identity Providers |
| Archive | All Referencing Domains |

Each domain participates only through approved APIs or domain events.

---

# Workflow Governance

The following rules are mandatory.

- Every workflow begins with validation.
- Aggregate invariants shall be preserved.
- State changes occur only after successful validation.
- Domain events are published after successful transactions.
- Every workflow generates audit records.
- Authorization is required for protected operations.
- Recovery workflows shall preserve identity integrity.

---

# Architectural Constraints

- These workflows are canonical reference implementations.
- Business domains may extend workflows but shall not violate aggregate boundaries.
- Workflow orchestration shall remain implementation independent.
- Sequence diagrams define logical interactions rather than deployment topology.
- Every downstream identity-related workflow shall align with these reference sequences.

---

# Architecture Governance & Compliance Framework

This section establishes the governance model for the Person & Identity domain.

It defines:

- Architecture Decision Records (ADRs)
- Architectural Principles
- Immutable Platform Invariants
- Compliance Requirements
- Review Process
- Versioning Strategy
- Change Management
- Acceptance Criteria

This chapter serves as the architectural contract governing future evolution of the Person & Identity domain.

---

# Purpose

The objectives of architecture governance are:

- Preserve architectural consistency
- Prevent uncontrolled design drift
- Protect aggregate boundaries
- Maintain backward compatibility
- Standardize architectural reviews
- Enable controlled evolution

All future changes shall comply with this governance framework.

---

# Architecture Decision Records (ADRs)

Major architectural decisions shall be documented using Architecture Decision Records.

Each ADR shall include:

- ADR Identifier
- Decision Title
- Status
- Context
- Decision
- Alternatives Considered
- Consequences
- Related Domains
- Approval Information

ADRs provide historical traceability for architectural evolution.

---

# Canonical ADR Register

| ADR ID | Decision |
|---------|----------|
| ADR-001 | Person is the canonical identity within a Tenant |
| ADR-002 | One User Account per Person per Tenant |
| ADR-003 | Authentication and Authorization are independent capabilities |
| ADR-004 | Authorization uses scoped RBAC |
| ADR-005 | Business domains do not own Person |
| ADR-006 | Context Resolution consumes business memberships but does not own them |
| ADR-007 | Domain Events are immutable integration contracts |
| ADR-008 | Aggregate ownership is singular and explicit |
| ADR-009 | Multi-domain communication uses APIs and Events |
| ADR-010 | Platform invariants are not tenant configurable |

Future ADRs shall extend this register rather than modify historical decisions.

---

# Architectural Principles Summary

The Person & Identity domain is governed by the following principles.

- Canonical ownership
- Domain-driven design
- Aggregate consistency
- Explicit bounded contexts
- Event-driven integration
- Immutable identifiers
- Tenant isolation
- Security by default
- Configuration over customization
- Backward compatibility

These principles are mandatory across all implementations.

---

# Platform Invariants

The following architectural rules are immutable.

- One canonical Person per Tenant.
- PersonID is immutable.
- One User Account per Person per Tenant.
- Authentication precedes Authorization.
- Authorization requires Active Context.
- Business domains shall not own Person.
- Domain Events are immutable.
- Audit records are immutable.
- Aggregate ownership is singular.
- Multi-domain writes occur only through approved application services.

These invariants shall never be overridden by configuration or customization.

---

# Domain Ownership Checklist

Before approving any architectural change, verify:

- Does the Person aggregate remain the canonical owner
- Has ownership moved between domains
- Are aggregate boundaries preserved
- Are responsibilities duplicated
- Are dependencies unidirectional
- Are integration contracts unchanged or versioned

Any violation requires architectural review.

---

# API Governance Checklist

All public APIs shall satisfy:

- Versioned contract
- Authentication required
- Authorization enforced
- Tenant isolation
- Input validation
- Audit generation
- Backward compatibility
- Error standardization

No public API shall expose internal aggregate structures.

---

# Event Governance Checklist

All domain events shall satisfy:

- Immutable payload
- Explicit version
- Correlation ID
- Tenant ID
- Aggregate reference
- Timestamp
- No credential data
- Backward compatibility

Breaking changes require a new event version.

---

# Security Compliance Checklist

Every identity capability shall verify:

- Authentication enforced
- Authorization evaluated
- Active Context resolved
- Sensitive data protected
- Audit generated
- Policy evaluation completed

Security compliance is mandatory before production deployment.

---

# Quality Review Checklist

Architecture reviewers shall evaluate:

| Review Area | Validation |
|-------------|------------|
| Aggregate Design | Complete |
| Ownership | Verified |
| Domain Boundaries | Verified |
| Event Contracts | Stable |
| API Contracts | Stable |
| Security | Compliant |
| Observability | Complete |
| Configuration | Governed |
| Scalability | Verified |
| NFR Compliance | Verified |

Review outcomes shall be documented.

---

# Merge Readiness Checklist

A domain change is merge-ready only when:

- Aggregate boundaries are preserved.
- Platform invariants remain unchanged.
- ADRs updated where necessary.
- APIs remain backward compatible.
- Events remain backward compatible.
- Test coverage updated.
- Documentation synchronized.
- Review approved.

Incomplete changes shall not be merged into the canonical architecture.

---

# Versioning Policy

The architecture follows semantic versioning.

```

Major

↓

Minor

↓

Patch

```

Major Version

- Breaking architectural changes

Minor Version

- New capabilities
- Backward-compatible enhancements

Patch Version

- Documentation corrections
- Clarifications
- Non-functional refinements

---

# Change Management

Every architectural change follows:

```

Proposal

↓

Architecture Review

↓

ADR Creation (if required)

↓

Impact Analysis

↓

Approval

↓

Implementation

↓

Validation

↓

Documentation Update

↓

Release

```

Changes bypassing this workflow are not considered canonical.

---

# Compliance Matrix

| Requirement | Compliance Mechanism |
|-------------|----------------------|
| Canonical Ownership | Aggregate Validation |
| Tenant Isolation | Security Validation |
| Identity Integrity | Duplicate Detection |
| API Stability | Contract Versioning |
| Event Stability | Event Versioning |
| Security | Policy Enforcement |
| Auditability | Immutable Audit Records |
| Scalability | Architecture Review |
| Maintainability | Modular Design |
| Observability | Central Monitoring |

This matrix supports architectural audits and governance reviews.

---

# Architecture Review Gates

Identity-related changes shall pass the following review gates.

Gate 1

Domain Model Review

Gate 2

Aggregate Boundary Review

Gate 3

Security Review

Gate 4

Integration Review

Gate 5

Performance & NFR Review

Gate 6

Architecture Governance Approval

Deployment is permitted only after successful completion of all mandatory gates.

---

# Acceptance Criteria

The Person & Identity architecture is considered complete when:

- All aggregates satisfy invariants.
- Ownership is unambiguous.
- APIs are documented.
- Events are documented.
- Security architecture is complete.
- Observability is complete.
- Configuration governance is complete.
- Operational workflows are defined.
- Multi-domain dependencies are documented.
- Architecture review has been approved.

---

# Future Evolution Guidelines

Future enhancements shall:

- Preserve backward compatibility.
- Extend existing aggregates rather than duplicate them.
- Introduce new ADRs for significant architectural decisions.
- Maintain event compatibility.
- Preserve platform invariants.
- Minimize downstream impact.

Architectural evolution shall remain deliberate, traceable, and governed.

---

# Final Architectural Statement

The Person & Identity domain is the canonical identity foundation of the SutraERP platform.

It establishes:

- Canonical Person ownership
- Identity lifecycle governance
- Authentication and Authorization
- Context Resolution
- Security architecture
- Integration contracts
- Operational governance
- Quality standards

Every downstream domain shall consume these capabilities through the approved architectural contracts defined in this document.

This document is the authoritative reference for all identity-related architecture within the SutraERP ecosystem.

---
