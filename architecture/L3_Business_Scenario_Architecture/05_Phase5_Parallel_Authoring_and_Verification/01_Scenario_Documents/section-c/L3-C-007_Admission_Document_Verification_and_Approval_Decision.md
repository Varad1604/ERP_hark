# L3-C-007 — Admission Document Verification and Approval Decision

**Architecture Level:** L3 — Business Scenario Architecture  
**Scenario ID:** `L3-C-007`  
**Scenario Title:** Admission Document Verification and Approval Decision  
**Primary Owner:** Admissions  
**Participating Domains:** A, B, C  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Last Updated:** 2026-08-20  
**Applicable L2 Workflows:** `WF-C01-02 — Document Verification Workflow`; `WF-C01-03 — Admission Approval Workflow`  
**Canonical Business Record:** Admission Record  
**Canonical Successful Lifecycle Outcome:** `Admission Approved`  
**Canonical Verification Transition:** `Documents Submitted → Verification Completed`  
**Canonical Approval Transition:** `Verification Completed → Admission Approved`

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-007` |
| Title | Admission Document Verification and Approval Decision |
| Primary Owner | Admissions |
| Participating Domains | A, B, C |
| Scenario Type | Critical |
| Priority | P0 |
| Source Phase 2 Row | `P2W-C-1056` |
| Source Phase 1 Candidate | `CAN-L3-C-007` |
| Derivation | KEEP |
| Status | Draft |
| Applicable L2 Workflow 1 | `WF-C01-02 — Document Verification Workflow` |
| Applicable L2 Workflow 2 | `WF-C01-03 — Admission Approval Workflow` |
| Canonical Owner | Admissions |
| Successful Business Outcome | Admission Approved |
| Verification Event | `DocumentsVerified` |
| Approval Event | `AdmissionApproved` |

The frozen Section C canonical register identifies C-007 as **Admission Document Verification and Approval Decision**, Primary Owner **Admissions**, Participating Domains **A, B, C**, Type **Critical**, Priority **P0**, with derivation **KEEP**. fileciteturn80file0L2-L2

---

## 2. Scenario Purpose

The purpose of this scenario is to establish whether an Admission Record has satisfied the institution's documentary verification requirements and, following successful verification, whether the institution formally approves the admission.

This is a single institutional governance outcome spanning two tightly coupled L2 gates:

1. **Document Verification** establishes that the required admission documents satisfy the applicable verification policy.
2. **Admission Approval** establishes the institutional decision to approve the verified admission.

The scenario therefore represents:

> **Documents Submitted → Verification Completed → Admission Approved**

The scenario does **not** perform Financial Clearance, Admission Confirmation, or Student Creation.

The L2 architecture explicitly states that Document Verification must complete before Admission Approval, and Admission Approval must precede Financial Clearance. fileciteturn89file0

---

## 3. Business Outcome

The successful business outcome is:

**Admission Approved following successful document verification.**

Successful completion means:

- required documents have been evaluated under the applicable policy;
- verification has reached the successful `Verification Completed` outcome;
- the Admission is eligible for approval;
- institutional approval policy and authority have been applied;
- an approval decision has been recorded;
- the Admission Record reaches `Admission Approved`;
- verification and approval history remain preserved;
- required audit evidence is recorded; and
- the canonical `AdmissionApproved` business event is published.

The scenario ends at **Admission Approved**.

It does not continue into Financial Clearance, Admission Confirmation, or Student Creation.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an Admission Record reaches the document-verification stage with required documents available for verification and the Admission is eligible to undergo the verification/approval governance path.

### 4.2 End Boundary

The primary successful path ends when:

1. document verification has completed successfully;
2. the Admission has been approved according to applicable institutional policy;
3. approval history and audit evidence have been recorded; and
4. `AdmissionApproved` has been published.

### 4.3 In Scope

- Resolution of institutional document policy.
- Determination of required/optional documents.
- Programme/category/country-specific document requirements where configured.
- Document completeness validation.
- Document authenticity verification.
- Manual, automated, hybrid, conditional, and waiver verification paths where supported upstream.
- Verification decision.
- Verification history preservation.
- Verification audit.
- `DocumentsVerified` publication.
- Admission approval policy resolution.
- Admission eligibility validation.
- Approval hierarchy resolution.
- Approval execution.
- Approval decision.
- Approval history preservation.
- Approval audit.
- `AdmissionApproved` publication.
- Cross-domain collaboration with Platform, Institution/Structure, Academic Delivery, Document Platform, Workflow Engine, and Audit within their defined ownership boundaries.

### 4.4 Out of Scope

- Application processing.
- Merit calculation.
- Reservation/seat allocation.
- Financial Clearance.
- Payment collection.
- Refunds.
- Admission Confirmation.
- Student creation.
- Academic Registration.
- Student Lifecycle processing.
- Modification of Document Platform-owned document storage.
- Modification of Finance-owned records.
- APIs.
- Database design.
- Business Data Structures.
- Services.
- Queues.
- Technical retries.
- Rollback.
- Infrastructure.
- UI/UX design.

### 4.5 Boundary Principle

Verification is a prerequisite to Approval.

Approval is a prerequisite to Financial Clearance.

Therefore C-007 must not claim that an approved Admission is already financially cleared, confirmed, or a Student.

---

## 5. Ownership and Participating Domains

### 5.1 Canonical Owner

**Admissions / Section C** is the canonical owner of:

- Admission Record;
- Admission Lifecycle;
- Admission Status;
- Admission Verification Workflow;
- Admission Approval Workflow;
- verification decision;
- approval decision;
- verification history;
- approval history; and
- associated canonical Admission events.

The L2 architecture explicitly states that Admissions owns the Admission Record, Admission Lifecycle, Admission Verification Workflow, and Admission Approval Workflow. fileciteturn87file0

### 5.2 Participating Domains

The frozen scenario metadata identifies **A, B, C**.

| Domain | Business Participation | Ownership Boundary |
|---|---|---|
| **A — Platform Foundation** | Authentication, authorization, workflow policy, audit/event capabilities | Does not own Admission or verification/approval decisions. |
| **B — Institution & Structure** | Supplies institutional context such as Institution, College, Department, Campus, Academic Calendar, Academic Year, and Programme Catalog references | Does not own Admission. |
| **C — Admissions / Student Lifecycle Section** | Owns Admission verification and approval | Canonical owner. |

The L2 document additionally identifies Academic Delivery and Document Platform participation in verification/approval, but those interactions do not transfer ownership. fileciteturn89file0

### 5.3 Document Platform Boundary

Document Platform owns document storage.

Admissions owns the verification workflow and verification decision.

Admissions does not modify document storage ownership.

### 5.4 Academic Delivery Boundary

Academic Delivery may provide Programme validation/reference information.

Admissions does not create or modify Programme or academic structure.

### 5.5 Ownership Rule

Cross-domain participation is collaboration, not ownership transfer.

---

## 6. Business Actors, Roles and Responsibilities

### 6.1 Applicant

The Applicant supplies required admission documents and may be required to resubmit documents where the verification outcome is Pending or documents are invalid/expired.

The Applicant does not own the verification or approval decision.

### 6.2 Verification Responsibility

The L2 verification workflow supports:

- manual verification;
- automated verification;
- hybrid verification;
- conditional verification; and
- authorized document waivers.

The exact human verification role is not made universal at L3.

### 6.3 Approval Authority

The L2 Approval workflow supports configurable:

- single-level approval;
- multi-level approval;
- parallel approval;
- committee approval; and
- automatic approval.

The exact institutional human role is therefore policy/configuration dependent.

### 6.4 Admissions

Admissions owns the business decisions:

- verification outcome;
- approval outcome;
- Admission lifecycle transition;
- verification/approval history; and
- canonical business events.

### 6.5 Platform / Workflow Engine

Platform and Workflow Engine provide supporting capabilities for:

- authentication;
- authorization;
- policy resolution;
- workflow orchestration.

They do not own the Admission decision.

### 6.6 Audit

Audit participates in preserving business audit evidence.

Audit does not own the Admission lifecycle.

---

## 7. Preconditions and Entry Criteria

### 7.1 Verification Preconditions

The applicable Admission must:

- exist;
- be in `Documents Submitted`;
- have required documents available for evaluation;
- be active;
- not be cancelled;
- not be revoked;
- have applicable document policy resolved.

### 7.2 Approval Preconditions

Approval requires:

- Admission exists;
- Admission is in `Verification Completed` / verified state;
- Admission is active;
- Admission is not cancelled;
- Admission is not revoked;
- required documents are verified;
- verification outcome is successful;
- approval workflow is configured;
- approval hierarchy is resolved;
- institutional policy is loaded; and
- applicable approval authority is available.

These conditions are directly supported by the L2 verification and approval workflows. fileciteturn89file0

### 7.3 Lifecycle Entry Constraint

The canonical lifecycle transition is:

```text
Documents Submitted
        ↓
Verification Completed
        ↓
Admission Approved
```

No other C-007 lifecycle transition is introduced.

---

## 8. Trigger

The scenario may begin when:

- document verification is ready for execution; and/or
- the Admission is submitted for approval after successful verification.

The L2 verification workflow identifies `AdmissionInitiated`, `DocumentsUploaded`, and `DocumentUpdated` as consumed inputs/events.

The L2 approval workflow identifies:

- Document Verification Completed;
- Admission Submitted for Approval;
- Workflow Engine Trigger;
- Committee Review Scheduled; and
- Government Allocation Imported

as possible approval triggers.

For the successful C-007 outcome, the critical business sequence remains:

> **Document verification success → Admission approval decision**

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Responsibility | Business Result |
|---|---|---|---|
| 1 | Resolve institutional document policy. | Admissions / Workflow Policy | Applicable document requirements are established. |
| 2 | Determine mandatory and optional documents for the Admission. | Admissions | Required documentary obligations are known. |
| 3 | Evaluate the submitted document set for completeness. | Admissions | Document completeness is determined. |
| 4 | Verify document authenticity using the institutionally configured verification approach. | Admissions / configured verification responsibility | Authenticity is established. |
| 5 | Determine the verification outcome. | Admissions | Documents are Verified, Pending, Rejected, Conditionally accepted, or Waived according to policy. |
| 6 | For successful verification, persist verification history and audit evidence. | Admissions / Audit | Verification history is preserved. |
| 7 | Publish `DocumentsVerified`. | Admissions | Completed verification fact is established. |
| 8 | Resolve institutional approval policy and hierarchy. | Admissions / Workflow Policy | Approval authority and sequence are established. |
| 9 | Validate Admission eligibility for approval. | Admissions | Admission satisfies approval conditions. |
| 10 | Execute the configured approval decision process. | Admissions / authorized approval authority | Approval decision is established. |
| 11 | Record the approval decision and immutable approval history. | Admissions | Institutional approval evidence is preserved. |
| 12 | Publish `AdmissionApproved`. | Admissions | Completed approval fact is established. |
| 13 | End C-007. | Admissions | Admission is now `Admission Approved`. |

### 9.1 Successful Lifecycle Result

```text
Documents Submitted
        │
        ▼
Verification Completed
        │
        ▼
Admission Approved
```

### 9.2 Downstream Boundary

After `AdmissionApproved`, the next canonical Admission lifecycle stage is Financial Clearance.

C-007 stops before that stage.

---

## 10. Alternate Business Flows

### 10.1 AF-01 — Manual Verification

Where manual verification is configured:

1. Required documents are identified.
2. Completeness is evaluated.
3. Authenticity is evaluated by the authorized institutional verification responsibility.
4. Verification is approved.
5. Verification history is preserved.
6. `DocumentsVerified` is published.
7. Approval proceeds under the configured approval policy.

### 10.2 AF-02 — Automated Verification

Where configured automated verification is used:

1. Document requirements are resolved.
2. Completeness is evaluated.
3. Configured verification outcomes are considered.
4. Verification is completed where policy conditions are satisfied.
5. `DocumentsVerified` is published.
6. Approval proceeds.

The L2 architecture states that Admissions remains responsible for the verification decision even when verification consumes outcomes from external verification services.

### 10.3 AF-03 — Hybrid Verification

Some documents may be verified through configured digital mechanisms while remaining documents are verified manually.

The overall business outcome remains a single Admissions-owned verification decision.

### 10.4 AF-04 — Conditional Verification

Where institutional policy permits temporary conditional acceptance:

1. Specific pending requirements are identified.
2. The condition is recorded.
3. Verification remains governed by the applicable policy.
4. Approval cannot bypass mandatory verification requirements.

### 10.5 AF-05 — Authorized Waiver

Where an authorized institution policy permits a document waiver:

1. The applicable requirement is identified.
2. Authorized waiver approval is established.
3. Waiver evidence is preserved.
4. Verification proceeds according to the waived requirement.
5. Audit evidence records the waiver.

### 10.6 AF-06 — Single-Level Approval

A single authorized approval level completes the approval decision.

### 10.7 AF-07 — Multi-Level Approval

Multiple configured approval authorities complete the required approval chain.

### 10.8 AF-08 — Parallel Approval

Where institutional policy supports parallel approvals, required authorities participate according to the configured policy.

### 10.9 AF-09 — Committee Approval

A committee collectively provides the required approval decision.

### 10.10 AF-10 — Automatic Approval

Where institutional policy permits automatic approval, the configured policy may establish the approval outcome without a manually executed approval chain.

---

## 11. Failure, Interruption and Business Recovery

### 11.1 Missing Document

**Condition:** A required document is missing.

**Business result:** Verification remains `Pending`.

**Business handling:** Applicant action/resubmission is required according to policy.

### 11.2 Expired Document

**Condition:** A submitted document is expired.

**Business result:** Verification cannot complete successfully.

**Business handling:** Resubmission is required.

### 11.3 Invalid Document

**Condition:** A document fails validity requirements.

**Business result:** Verification is rejected.

**Business handling:** Reason is recorded and the applicable correction/resubmission path is followed.

### 11.4 Suspected Fraud

**Condition:** Fraud is suspected during verification.

**Business result:** Admission processing is suspended.

**Business handling:** Institution is notified and the applicable fraud/governance process takes control.

### 11.5 External Verification Failure

**Condition:** A configured external verification source is unavailable or does not produce the required business outcome.

**Business result:** Where configured, fallback to manual verification may occur; otherwise verification remains Pending.

No technical retry mechanism is specified at L3.

### 11.6 Duplicate Upload

**Condition:** Duplicate document upload occurs.

**Business result:** Latest applicable document version is considered while historical versions remain preserved.

### 11.7 Approval Rejected

**Condition:** Approval authority rejects the Admission.

**Business result:** Admission enters the canonical rejected outcome defined by upstream architecture.

**Business handling:** Rejection reason is recorded and the Admission does not continue to Financial Clearance.

### 11.8 Approver Unavailable

**Condition:** Required approval authority is unavailable.

**Business result:** Approval remains Pending or is escalated according to configured policy.

### 11.9 Approval Timeout

**Condition:** Configured approval time condition is reached.

**Business result:** Approval remains Pending and applicable escalation policy is invoked.

### 11.10 Policy Resolution Failure

**Condition:** Required institutional policy cannot be resolved.

**Business result:** Workflow is suspended.

### 11.11 Admission Modified During Approval

**Condition:** Admission information materially changes during approval.

**Business result:** Existing approval may be invalidated according to the upstream approval policy.

The L2 workflow states that the approval workflow may restart after such invalidation.

### 11.12 Technical Failure Boundary

Technical retry, rollback, queue, server failure, persistence recovery, or infrastructure behaviour is outside L3.

---

## 12. Business Rules and Decision Constraints

### 12.1 Document Verification Rules

| Rule ID | Business Rule | Owner |
|---|---|---|
| `BR-C01-009` | Required documents shall be policy-driven. | Admissions |
| `BR-C01-010` | Verification history shall be immutable. | Admissions |
| `BR-C01-011` | Latest document version shall not overwrite historical versions. | Admissions / Document boundary |
| `BR-C01-012` | Waivers require authorized approval. | Admissions |
| `BR-C01-013` | Verification outcome shall be deterministic. | Admissions |
| `BR-C01-014` | Fraud detection shall suspend further processing. | Admissions |
| `BR-C01-015` | Verification must complete before Admission Approval. | Admissions |
| `BR-C01-016` | Verification policies shall support programme-specific rules. | Admissions |

These rules are explicitly defined in the L2 verification workflow. fileciteturn89file0

### 12.2 Admission Approval Rules

| Rule ID | Business Rule | Owner |
|---|---|---|
| `BR-C01-017` | Admission shall not bypass approval. | Admissions |
| `BR-C01-018` | Approval hierarchy shall be configurable. | Admissions |
| `BR-C01-019` | Approval history shall be immutable. | Admissions |
| `BR-C01-020` | Approval authority shall be authenticated. | Admissions / Platform |
| `BR-C01-021` | Rejected admissions shall not continue. | Admissions |
| `BR-C01-022` | Every approval generates audit history. | Admissions |
| `BR-C01-023` | Approval shall precede Financial Clearance. | Admissions |

These rules are defined in the L2 Approval workflow. fileciteturn89file0

### 12.3 Ownership Constraints

- Admissions owns verification.
- Document Platform owns storage.
- Admissions owns approval.
- Academic Delivery provides programme validation/reference.
- Admissions does not modify Finance-owned data.
- Approval does not create a Student.

---

## 13. State and Lifecycle Impact

### 13.1 Canonical Admission Record

**Owner:** Admissions

### 13.2 Verification Transition

The authoritative L2 verification workflow performs:

```text
Documents Submitted
        ↓
Verification Completed
```

No other lifecycle transition is introduced by this scenario. fileciteturn89file0

### 13.3 Approval Transition

The authoritative L2 Approval workflow performs:

```text
Verification Completed
        ↓
Admission Approved
```

No other successful lifecycle transition is introduced by this scenario. fileciteturn89file0

### 13.4 Overall C-007 Lifecycle

```text
Documents Submitted
        │
        ▼
Verification Completed
        │
        ▼
Admission Approved
```

### 13.5 Rejected Verification

A rejected verification outcome does not permit the Admission Approval transition.

### 13.6 Pending Verification

A Pending verification outcome does not permit Approval.

### 13.7 Rejected Approval

A rejected approval does not permit Financial Clearance or Admission Confirmation.

### 13.8 Terminal Boundary

`Admission Approved` is **not** the terminal state of the complete Admission lifecycle.

The next authoritative lifecycle stage is Financial Clearance.

C-007 ends before that stage.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Verification Events

| Event | Canonical Publisher | Meaning |
|---|---|---|
| `DocumentsVerified` | Admissions | Document verification successfully completed. |
| `DocumentsRejected` | Admissions | Document verification failed. |
| `DocumentsPending` | Admissions | Verification awaits required action. |
| `VerificationWaived` | Admissions | Policy-authorized verification waiver granted. |

The L2 verification workflow defines these published events. fileciteturn89file0

### 14.2 Approval Events

| Event | Canonical Publisher | Meaning |
|---|---|---|
| `AdmissionApproved` | Admissions | Admission approval successfully completed. |
| `AdmissionRejected` | Admissions | Admission approval rejected. |
| `ApprovalPending` | Admissions | Approval remains pending. |
| `ApprovalEscalated` | Admissions | Approval escalation initiated. |

The L2 Approval workflow defines these published events. fileciteturn89file0

### 14.3 Consumed Verification Inputs

The verification workflow consumes:

- `AdmissionInitiated`;
- `DocumentsUploaded`; and
- `DocumentUpdated`.

### 14.4 Consumed Approval Inputs

The approval workflow consumes:

- `DocumentsVerified`; and
- `AdmissionSubmitted`.

### 14.5 Cross-Domain Collaboration

| Domain | Business Collaboration | Ownership |
|---|---|---|
| Platform | Authentication, authorization, policy/workflow support | Platform-owned |
| Institution & Structure | Institutional context | Section B-owned |
| Document Platform | Document metadata/storage | Document Platform-owned |
| Academic Delivery | Programme validation/reference | Section D-owned |
| Audit | Verification/approval audit evidence | Audit-owned |
| Workflow Engine | Policy and workflow resolution | Platform capability |
| Admissions | Verification and approval decisions | **Canonical owner** |

### 14.6 Event Boundary

Events represent completed business facts.

The scenario does not equate an event with:

- an API;
- a technical message;
- a queue;
- a database transaction;
- an implementation callback.

---

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Verification Authorization

Verification policy determines:

- required documents;
- optional documents;
- programme-specific documents;
- category-specific documents;
- country-specific requirements.

Where a waiver is used, authorized approval is mandatory.

### 15.2 Approval Authorization

Approval authority must be authenticated.

The approval hierarchy is configurable and may include:

- single-level approval;
- multi-level approval;
- parallel approval;
- committee approval; or
- automatic approval.

The exact human role is not universal and is therefore not invented.

### 15.3 Verification Audit Evidence

The L2 verification workflow identifies additional audit attributes including:

- Verifier;
- Verification Method;
- Verification Decision;
- Document Version;
- Waiver Reference;
- Fraud Flag; and
- Verification Timestamp. fileciteturn89file0

### 15.4 Approval Audit Evidence

The L2 approval workflow identifies:

- Approver;
- Approval Level;
- Decision;
- Remarks;
- Approval Timestamp; and
- Workflow Version.

### 15.5 Historical Integrity

Verification and approval histories are immutable.

Historical document versions are preserved.

### 15.6 Separation of Duties

No additional C-007-specific separation-of-duty rule is invented beyond the configured approval authority and policy.

### 15.7 Override

A document waiver is permitted only where authorized by policy.

No local override mechanism is introduced.

---

## 16. Completion Criteria and Postconditions

### 16.1 Successful Completion Criteria

C-007 is complete when:

1. the Admission Record exists and is eligible for document verification;
2. applicable document policy has been resolved;
3. required documents have been evaluated;
4. authenticity/completeness requirements have been satisfied or validly waived;
5. verification reaches `Verification Completed`;
6. verification history and audit evidence are preserved;
7. `DocumentsVerified` is published;
8. approval policy and hierarchy have been resolved;
9. Admission eligibility for approval has been validated;
10. required approval authority/chain has completed successfully;
11. approval history and audit evidence are preserved;
12. Admission reaches `Admission Approved`; and
13. `AdmissionApproved` is published.

### 16.2 Successful Postconditions

| Postcondition | Result |
|---|---|
| Verification lifecycle | `Verification Completed` |
| Approval lifecycle | `Admission Approved` |
| Admission owner | Admissions |
| Verification history | Immutable/preserved |
| Approval history | Immutable/preserved |
| Verification audit | Recorded |
| Approval audit | Recorded |
| `DocumentsVerified` | Published by Admissions |
| `AdmissionApproved` | Published by Admissions |
| Financial Clearance | **Not yet performed** |
| Admission Confirmation | **Not yet performed** |
| Student Creation | **Not performed** |

### 16.3 Non-Success Postconditions

If verification remains Pending:

- Approval does not complete.

If verification is Rejected:

- Approval does not proceed.

If Approval is Pending:

- Financial Clearance does not begin through C-007.

If Approval is Rejected:

- the Admission follows the canonical rejection outcome and does not continue through the successful path.

### 16.4 End Boundary

The scenario ends at the completed **Admission Approved** outcome.

The next Admission lifecycle stage, Financial Clearance, belongs to the subsequent workflow and is outside C-007.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

#### OI-C007-01 — Combined L3 Outcome Spans Two L2 Workflows

The frozen scenario title intentionally combines:

- Document Verification; and
- Approval Decision.

The L2 architecture defines these as two separate workflows with a strict dependency:

```text
Documents Submitted
        ↓
Verification Completed
        ↓
Admission Approved
```

This does not create an ownership conflict because both workflows are Admissions-owned and the frozen Stage 3 scenario explicitly combines them into C-007.

**Disposition:** Preserve the frozen scenario as one L3 institutional outcome while retaining the two authoritative L2 workflow boundaries.

---

#### OI-C007-02 — Approval Role Names Are Institution-Specific

The L2 approval workflow permits configurable approval hierarchy.

Examples are given in the source, but the architecture does not establish one universal role applicable to all institutions.

**Disposition:** Do not hard-code a human role at L3.

---

### 17.2 Assumptions

1. The frozen Stage 3 identity for C-007 is authoritative.
2. `WF-C01-02 — Document Verification Workflow` and `WF-C01-03 — Admission Approval Workflow` are the applicable L2 sources.
3. Verification must complete before Approval.
4. Approval must complete before Financial Clearance.
5. Admissions remains the canonical owner of both verification and approval decisions.
6. Document Platform retains document-storage ownership.
7. Cross-domain collaboration does not transfer Admission ownership.
8. `DocumentsVerified` and `AdmissionApproved` are canonical business events published by Admissions.
9. Financial Clearance, Confirmation, and Student Creation remain outside C-007.
10. No local institutional policy is invented where the L2 architecture makes policy configurable.

---

### 17.3 Deferred Questions

| Question | Target Authority | Reason |
|---|---|---|
| Which exact human verification role is used at a particular institution? | Institutional configuration | L2 supports multiple verification modes. |
| Which exact approval hierarchy applies? | Institutional configuration | Approval hierarchy is explicitly configurable. |
| Which programme/category/country document requirements apply to a specific institution? | Document policy | L2 makes these configurable. |
| Which downstream process consumes `AdmissionApproved` for the next stage? | Admissions / Financial Clearance architecture | C-007 ends before Financial Clearance. |

---

# Validation and Authoring Classification

**Classification: PASS WITH ISSUES — Draft**

## Identity Verification

- Final ID: `L3-C-007`
- Title: `Admission Document Verification and Approval Decision`
- Primary Owner: Admissions
- Participating Domains: A, B, C
- Scenario Type: Critical
- Priority: P0

These frozen values are confirmed by the Section C canonical register. fileciteturn80file0L2-L2

## L2 Verification

The applicable L2 source defines:

- Document Verification;
- Verification outcomes;
- verification state transition;
- verification events;
- verification business rules;
- approval workflow;
- approval decision paths;
- approval state transition;
- approval events;
- approval business rules; and
- audit obligations. fileciteturn89file0

## Lifecycle Verification

The authoritative lifecycle defines:

```text
Documents Submitted
        ↓
Verification Completed
        ↓
Admission Approved
```

and separately establishes Financial Clearance after Approval.

Therefore C-007 stops at `Admission Approved`.

## Ownership Verification

Admissions owns both verification and approval.

Document Platform, Platform, Institution & Structure, Academic Delivery, and Audit participate without taking ownership of the Admission lifecycle.

## Event Verification

Canonical successful events:

- `DocumentsVerified`
- `AdmissionApproved`

Both are Admissions-owned business facts. fileciteturn89file0

## Business Rule Verification

The document carries forward the authoritative verification rules `BR-C01-009` through `BR-C01-016` and approval rules `BR-C01-017` through `BR-C01-023`. fileciteturn89file0

## Implementation Leakage Check

No APIs, databases, Business Data Structures, services, queues, deployment, infrastructure, technical access control/IAM, technical retries, rollback, or UI design have been introduced.

## Final Status

**PASS WITH ISSUES — Draft**

The document is complete across all 17 mandatory sections. The only non-blocking issues concern institution-specific verification and approval configuration, which are deliberately left to upstream policy rather than invented.

> **Governance note:** Complete authoring artifact ≠ Architecture Approved/Locked. This remains `Draft` until human architecture review.
