# Student Readmission

**Scenario ID:** L3-C-005  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Applicable L2 Workflow:** `WF-C01-09 — Readmission Workflow`  
**Canonical Business Record:** Admission Record  
**Canonical Outcome:** Readmitted  
**Canonical Business Event:** `AdmissionReadmitted`

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-005` |
| Scenario Title | Student Readmission |
| Primary Owner | Admissions |
| Participating Domains | C |
| Scenario Type | Cross-domain |
| Priority | P1 |
| Status | Locked |
| Source Phase 2 Row | `P2W-C-1054` |
| Source Phase 1 Candidate | `CAN-L3-C-005` |
| Applicable L2 Workflow | `WF-C01-09 — Readmission Workflow` |
| Canonical Admission Outcome | `Readmitted` |
| Canonical Event | `AdmissionReadmitted` |
| Canonical Owner | Admissions |

The frozen Section C canonical register identifies C-005 as **Student Readmission**, owned by **Admissions**, participating domain **C**, Type **Cross-domain**, Priority **P1**.

---

## 2. Scenario Purpose

The purpose of this scenario is to provide the institutionally governed business process through which an applicant whose previous admission lifecycle has concluded may be considered for readmission.

Readmission evaluates the historical Admission, validates the applicant's eligibility and applicable institutional policy, determines the appropriate readmission strategy, and completes the resulting admission outcome while preserving all historical relationships.

The L2 architecture explicitly separates **historical admissions** from **active admissions**. Institutions may configure whether an eligible readmission restores an existing admission or creates a new admission lifecycle.

---

## 3. Business Outcome

A previously inactive historical admission is successfully processed through the institution's readmission policy and reaches the canonical **Readmitted** outcome.

The successful outcome consists of:

1. a valid readmission request or institutional readmission action;
2. applicant and institutional authorization validation;
3. applicable readmission policy resolution;
4. eligibility determination;
5. determination of the configured readmission strategy;
6. execution of either restoration or creation of the appropriate admission lifecycle;
7. preservation of the historical admission relationship;
8. generation of required business audit evidence; and
9. publication of `AdmissionReadmitted`.

The scenario does **not** erase, overwrite, or replace the historical admission.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a readmission request or authorized institutional readmission action is raised for a historical Admission Record that is inactive and potentially eligible for readmission.

Possible triggers defined by the L2 workflow include:

- Applicant Readmission Request;
- Administrative Readmission;
- Institutional Restoration;
- Approved Appeal; and
- Government Directive.

### 4.2 End Boundary

The scenario ends when the configured readmission strategy has been successfully executed, readmission history has been preserved, audit evidence has been generated, and `AdmissionReadmitted` has been published.

### 4.3 In Scope

- Readmission request.
- Applicant identity validation.
- Institutional authorization validation.
- Readmission policy resolution.
- Eligibility validation.
- Review of historical admission status.
- Review of withdrawal/cancellation reason where relevant.
- Academic-session validation.
- Programme eligibility evaluation.
- Maximum readmission attempt evaluation.
- Time-limit evaluation.
- Determination of restoration versus new admission.
- Execution of the configured strategy.
- Preservation of historical admission linkage.
- Prevention of duplicate active admissions.
- Readmission audit evidence.
- Publication of `AdmissionReadmitted`.

### 4.4 Out of Scope

- Academic credit transfer.
- Financial settlement.
- Fee calculation.
- Student promotion.
- Academic registration.
- Initial admission processing unrelated to the readmission strategy.
- Technical authentication implementation.
- APIs, databases, services, Business Data Structures, queues, infrastructure, technical retries, rollback, or UI design.

### 4.5 Cross-Domain Boundary

The L2 workflow identifies Academic Delivery as participating for **Session Validation**. Platform participates in authentication, and Audit participates in readmission history. These interactions do not transfer ownership of the Admission lifecycle away from Admissions.

---

## 5. Ownership and Participating Domains

### 5.1 Canonical Owner

**Admissions** remains the canonical owner of:

- Admission Record;
- Admission Status;
- Admission Lifecycle;
- Readmission workflow;
- Readmission strategy decision;
- Readmission history; and
- `AdmissionReadmitted`.

### 5.2 Participating Domains / Responsibilities

| Domain / Responsibility | Participation |
|---|---|
| **Admissions** | Owns readmission decision, eligibility, strategy, Admission lifecycle, and completed outcome. |
| **Academic Delivery** | Provides academic session validation and programme information required by the readmission workflow. |
| **Platform / Identity** | Supports authentication of the Applicant/requester; does not own the Admission. |
| **Audit** | Preserves readmission history/evidence; does not own the Admission lifecycle. |

### 5.3 Ownership Preservation

The fact that Academic Delivery participates in session validation does not make it the owner of Readmission.

Likewise, audit participation does not transfer Admission ownership to Audit.

Admissions remains the canonical owner throughout the readmission business outcome.

---

## 6. Business Actors, Roles and Responsibilities

| Actor / Business Responsibility | Responsibility | Authority Boundary |
|---|---|---|
| **Applicant** | Requests readmission and provides the information required for eligibility assessment. | May initiate readmission subject to institutional policy. |
| **Institution / Admissions responsibility** | Validates the request, resolves policy, determines eligibility, selects/executes the configured strategy, preserves history, and completes the Admission outcome. | Canonical business owner. |
| **Academic Delivery responsibility** | Provides academic session and programme information relevant to eligibility. | Supplies academic business information; does not own Readmission. |
| **Audit responsibility** | Preserves readmission history/evidence. | Evidence/governance responsibility; does not own Admission. |

The L2 source explicitly supports Applicant Readmission Request, Administrative Readmission, and Institutional Restoration.

No unsupported human role title is invented.

---

## 7. Preconditions and Entry Criteria

The normal readmission flow requires:

1. A historical Admission Record exists.
2. The historical Admission is inactive.
3. The historical Admission is eligible for readmission.
4. The Applicant is authenticated where Applicant-initiated.
5. The institution/requester is authorized where institutionally initiated.
6. The readmission policy has been resolved.

The L2 preconditions explicitly identify:

- Historical Admission exists;
- Admission inactive;
- Admission eligible for readmission;
- Applicant authenticated;
- Institution authorized; and
- Readmission policy resolved.

### 7.1 Historical Record Requirement

The historical Admission remains available and must not be overwritten.

### 7.2 Duplicate Active Admission Constraint

The readmission process must prevent duplicate active admissions.

This is a canonical business rule and not an implementation-level uniqueness mechanism.

---

## 8. Trigger

The business trigger is:

> **A readmission request or authorized institutional readmission action is raised for an inactive historical Admission.**

The L2 workflow identifies:

- Applicant Readmission Request;
- Administrative Readmission;
- Institutional Restoration;
- Approved Appeal; and
- Government Directive.

The trigger starts readmission consideration but does not itself establish the final `Readmitted` outcome.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Responsibility / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the readmission request/action. | Applicant / Institution | Readmission processing begins. |
| 2 | Validate Applicant identity and/or institutional authority. | Admissions / Platform participation | Requester is established as eligible to initiate the process. |
| 3 | Resolve institutional readmission policy. | Admissions | Applicable restoration, academic-year, programme, attempt, and time-limit rules are established. |
| 4 | Validate readmission eligibility. | Admissions with Academic Delivery information | Applicant and historical Admission satisfy configured eligibility conditions. |
| 5 | Determine the readmission strategy. | Admissions | Strategy is established as Restore Existing Admission, Create New Admission, or Reject. |
| 6 | Apply the selected strategy. | Admissions | The configured readmission outcome is executed. |
| 7 | Preserve readmission history and historical Admission linkage. | Admissions / Audit | Historical continuity remains intact. |
| 8 | Generate required business audit evidence. | Admissions / Audit | Readmission remains institutionally traceable. |
| 9 | Publish `AdmissionReadmitted`. | Admissions | Completed readmission business fact becomes available. |
| 10 | End the scenario. | Admissions | Readmission outcome is complete. |

The L2 workflow's canonical sequence is:

`Readmission Requested → Validate Request → Resolve Readmission Policy → Validate Eligibility → Determine Readmission Strategy → Restore / Create New Admission / Reject → Persist Readmission → Generate Audit → Publish AdmissionReadmitted`.

---

## 10. Alternate Business Flows

### 10.1 AF-01 — Restore Existing Admission

Where policy and eligibility permit restoration:

1. Validate the historical Admission.
2. Resolve the restoration policy.
3. Determine that restoration is permitted.
4. Restore the applicable Admission lifecycle.
5. Preserve the historical relationship.
6. Generate audit evidence.
7. Publish `AdmissionReadmitted`.

The L2 decision matrix explicitly identifies:

`Eligible = Yes + Policy = Restore + Previous Status = Withdrawn → Restore Admission`.

### 10.2 AF-02 — Create New Admission

Where institutional policy requires a fresh admission lifecycle:

1. Validate historical eligibility.
2. Determine that a new Admission is required.
3. Create the new Admission lifecycle according to the applicable Admissions architecture.
4. Preserve the relationship to the historical admission.
5. Generate audit evidence.
6. Publish `AdmissionReadmitted`.

The L2 workflow explicitly states that every readmission shall preserve historical records and allows the configured strategy to restore or create a new Admission.

### 10.3 AF-03 — Administrative Readmission

Where the institution directly initiates readmission:

1. Validate institutional authority.
2. Resolve readmission policy.
3. Validate eligibility.
4. Determine strategy.
5. Execute configured strategy.
6. Preserve history and audit evidence.
7. Publish `AdmissionReadmitted`.

### 10.4 AF-04 — Approved Appeal

Where an approved appeal establishes a readmission basis:

1. Recognize the approved appeal as the trigger.
2. Validate the historical Admission.
3. Resolve readmission policy.
4. Validate eligibility.
5. Determine and execute the configured strategy.
6. Preserve history.
7. Publish `AdmissionReadmitted`.

### 10.5 AF-05 — Government Directive

Where a government directive establishes the applicable readmission basis, the institution evaluates and executes the readmission according to the governing policy and configured strategy.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution | Responsible Domain |
|---|---|---|---|
| Ineligible Applicant | Readmission rejected. | Preserve historical Admission and current institutional history. | Admissions |
| Policy Restriction | Readmission workflow terminates without readmission. | Follow applicable institutional policy. | Admissions |
| Maximum Attempts Exceeded | Request rejected. | No new active readmission outcome is established. | Admissions |
| Invalid Historical Admission | Workflow terminates. | Historical record remains preserved. | Admissions |
| Duplicate active admission would result | Readmission cannot establish a duplicate active admission. | Apply the applicable policy-governed existing Admission strategy. | Admissions |
| Applicant is not authenticated | Applicant-initiated request cannot proceed. | Establish the required authenticated business identity. | Admissions / Platform |
| Institution is not authorized | Administrative readmission cannot proceed. | Establish applicable institutional authority. | Admissions |

The L2 workflow explicitly identifies:

- `EX-01 — Ineligible Applicant`;
- `EX-02 — Policy Restriction`;
- `EX-03 — Maximum Attempts Exceeded`; and
- `EX-04 — Invalid Historical Admission`.

### 11.1 Historical Preservation

Failure of a readmission request does not delete or overwrite the historical Admission.

### 11.2 No Technical Recovery Mechanics

L3 does not define technical retries, queues, rollback, database recovery, or infrastructure failure handling.

---

## 12. Business Rules and Decision Constraints

| Rule | Business Constraint | Owner | Scenario Effect |
|---|---|---|---|
| BR-C01-051 | Readmission shall preserve historical records. | Admissions | Historical Admission remains intact. |
| BR-C01-052 | Readmission policy shall be configurable. | Admissions | Institution-specific eligibility/strategy can govern outcome. |
| BR-C01-053 | Duplicate active admissions are prohibited. | Admissions | Readmission cannot create an invalid duplicate active Admission. |
| BR-C01-054 | Readmission shall generate audit records. | Admissions / Audit | Completed readmission remains auditable. |
| BR-C01-055 | Readmission may restore or recreate admissions according to policy. | Admissions | Strategy is policy-governed. |
| BR-C01-056 | Workflow execution shall be idempotent. | Admissions | Repeated business execution does not create contradictory duplicate outcomes. |

These rules are explicitly defined by the L2 Readmission workflow.

### 12.1 Eligibility Dimensions

The L2 workflow identifies configurable readmission policy dimensions including:

- restoration rules;
- academic year policy;
- programme eligibility;
- maximum readmission attempts; and
- time limits.

### 12.2 Eligibility Examples

The L2 workflow identifies possible eligibility considerations including:

- previous Admission status;
- withdrawal reason;
- cancellation reason;
- academic regulations; and
- institutional restrictions.

These are policy-governed examples, not new L3 rules.

### 12.3 Historical vs Active Admission

Historical admissions and active admissions must remain distinct.

The readmission process preserves historical relationships regardless of whether the strategy restores or creates a new Admission.

---

## 13. State and Lifecycle Impact

### 13.1 Canonical Business Record

**Admission Record**

**Canonical Owner:** Admissions

### 13.2 L2 State Transition

The L2 architecture presents the readmission transition as:

```text
Withdrawn / Cancelled
        │
        ▼
Readmitted
```

The scenario therefore uses `Readmitted` as the canonical readmission outcome.

### 13.3 Strategy-Dependent Lifecycle Behaviour

The readmission workflow provides two successful strategy paths:

**Restore Existing Admission**

```text
Historical inactive Admission
        ↓
Restore
        ↓
Readmitted
```

**Create New Admission**

```text
Historical inactive Admission
        ↓
New Admission lifecycle
        ↓
Readmitted
```

The selected strategy is determined by institutional policy.

### 13.4 Rejection

If eligibility or policy conditions fail:

```text
Readmission Requested
        ↓
Reject Request
```

No successful `Readmitted` outcome is established.

### 13.5 Ownership

Admissions remains the owner of the Admission lifecycle and readmission outcome.

Academic Delivery participates in session/programme validation but does not acquire Admission ownership.

### 13.6 Historical Integrity

The historical Admission remains immutable and is never overwritten.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published Business Event

| Event | Canonical Publisher | Purpose |
|---|---|---|
| `AdmissionReadmitted` | **Admissions** | Represents the completed readmission business outcome. |

The L2 workflow also defines `ReadmissionRejected` for rejected requests.

### 14.2 Consumed Business Fact

The L2 workflow identifies:

`ReadmissionRequested`

as a consumed event/input to the workflow.

This is distinct from the completed `AdmissionReadmitted` event.

### 14.3 Academic Delivery Collaboration

Academic Delivery provides:

- Academic Session;
- Programme Information; and
- Session Validation.

This supports eligibility determination.

Academic Delivery does not own the Admission or readmission outcome.

### 14.4 Platform / Identity Collaboration

Platform participates in Applicant authentication.

Authentication is a prerequisite/collaboration concern and does not transfer Admission ownership.

### 14.5 Audit Collaboration

Audit preserves readmission history/evidence.

Audit does not become the owner of the Admission Record.

### 14.6 Finance Boundary

Financial settlement and fee calculation are explicitly outside the Readmission workflow.

Finance remains responsible for Finance-owned consequences.

### 14.7 Business Event Boundary

`AdmissionReadmitted` represents the completed business fact.

The following remain distinct business interactions:

- Readmission Request;
- Eligibility Decision;
- Strategy Decision;
- Readmission execution;
- Audit; and
- Completed Readmission event.

No API, payload, queue, transport, or technical event mechanism is defined.

---

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Applicant Authorization

Applicant-initiated readmission requires Applicant authentication.

### 15.2 Institutional Authorization

Administrative or institutional readmission requires applicable institutional authorization.

### 15.3 Approval

The L2 architecture supports configurable approval hierarchy as an extension point.

The exact human approval role is therefore not invented at L3.

### 15.4 Audit Evidence

The L2 workflow requires the following readmission information to be recorded:

- Previous Admission;
- Readmission Reason;
- Decision;
- Strategy Applied;
- Timestamp; and
- Initiating User.

At L3, these are treated as business evidence obligations.

### 15.5 Historical Integrity

Historical admissions are immutable.

Readmission never overwrites historical records.

### 15.6 Separation of Duties

No additional scenario-specific separation-of-duty rule is established in the applicable L2 material.

Any institutional approval hierarchy remains governed by configurable upstream policy.

---

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

C-005 is complete when:

1. a valid readmission request or institutional action exists;
2. Applicant/institutional authority has been validated;
3. readmission policy has been resolved;
4. eligibility has been determined;
5. the readmission strategy has been determined;
6. the configured strategy has been successfully executed;
7. historical Admission relationships have been preserved;
8. required readmission audit evidence has been generated; and
9. Admissions has published `AdmissionReadmitted`.

### 16.2 Successful Postconditions

| Postcondition | Result |
|---|---|
| Readmission outcome | **Readmitted** |
| Admission owner | Admissions |
| Historical Admission | Preserved |
| Duplicate active Admission | Prohibited |
| Strategy | Restore Existing Admission or Create New Admission according to policy |
| Audit history | Generated |
| `AdmissionReadmitted` | Published by Admissions |
| Academic Delivery | Participates in session/programme validation only |
| Finance | Remains owner of financial processing |
| Platform | Authentication responsibility only |

### 16.3 Rejected Postconditions

If eligibility or policy fails:

- Readmission does not complete.
- Historical Admission remains preserved.
- No invalid duplicate active Admission is created.
- `ReadmissionRejected` may represent the rejected business outcome as defined upstream.

### 16.4 End Boundary

The scenario ends after successful readmission execution and publication of `AdmissionReadmitted`, or after the business decision rejects/terminates the request.

Downstream academic registration, fee calculation, and financial settlement remain outside this scenario.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

#### OI-C005-01 — Restoration vs New Admission Lifecycle Detail

The L2 workflow intentionally allows institutions to configure the readmission strategy as either:

- Restore Existing Admission; or
- Create New Admission.

The L2 workflow also presents the canonical outcome as `Readmitted`.

**Architecture impact:** Non-blocking at L3 because the scenario preserves both authoritative strategy paths without inventing institution-specific policy.

**Disposition:** Strategy remains policy-governed. L3 does not select one strategy universally.

---

#### OI-C005-02 — Exact Approval Authority

The L2 workflow allows configurable approval hierarchy.

**Architecture impact:** Non-blocking.

**Disposition:** Do not invent a universal human role. Exact authority remains an institutional policy/configuration concern.

---

### 17.2 Assumptions

1. The frozen Stage 3 semantics for `L3-C-005` are authoritative.
2. `WF-C01-09 — Readmission Workflow` is the applicable L2 workflow.
3. Historical Admissions remain immutable.
4. Duplicate active Admissions are prohibited.
5. Readmission strategy is determined by institutional policy.
6. `AdmissionReadmitted` is published by Admissions.
7. Academic Delivery participates in session/programme validation but does not own Readmission.
8. Platform authentication does not transfer Admission ownership.
9. Audit preserves readmission evidence but does not own the Admission lifecycle.
10. Finance remains outside the Readmission workflow for financial settlement and fee calculation.
11. A restoration strategy preserves the historical relationship rather than destroying the historical record.

---

### 17.3 Deferred Questions

| Question | Target Authority / Level | Reason |
|---|---|---|
| Which institution-specific policy determines Restore versus Create New Admission? | Admissions policy/configuration | L2 intentionally makes strategy configurable. |
| Which exact human role provides approval where approval hierarchy is configured? | Admissions policy/configuration | No universal role is established. |
| What exact academic session/programme rules apply? | Academic Delivery | L2 consumes academic session and programme information. |
| What downstream financial consequences apply after readmission? | Finance | Financial settlement is explicitly outside C-005. |
| What academic registration consequences follow readmission? | Academic Registration | Registration is explicitly outside the Readmission workflow. |

---

# Validation and Authoring Classification

**Classification: PASS WITH ISSUES — Draft**

## Identity Verification

- Final ID: `L3-C-005`
- Title: `Student Readmission`
- Primary Owner: Admissions
- Participating Domain: C
- Scenario Type: Cross-domain
- Priority: P1

The frozen Section C register confirms all of these values.

## L2 Verification

The applicable L2 workflow is:

`WF-C01-09 — Readmission Workflow`

The L2 source defines:

- readmission purpose and context;
- triggers;
- preconditions;
- inputs;
- main workflow;
- restore/new-admission strategy;
- alternate flows;
- exception flows;
- postconditions;
- events;
- cross-domain interactions;
- decision matrix;
- business rules;
- audit requirements; and
- architectural constraints.

## Lifecycle Verification

The L2 state transition is:

`Withdrawn / Cancelled → Readmitted`

with strategy-dependent restoration or new Admission processing.

## Ownership Verification

Admissions retains ownership of the Admission Record and readmission outcome.

Academic Delivery, Platform, and Audit participate without acquiring Admission ownership.

## Event Verification

Canonical completed event:

`AdmissionReadmitted`

Canonical publisher:

**Admissions**

Rejected outcome:

`ReadmissionRejected`

## Rule Verification

The authoritative L2 rules establish:

- historical preservation;
- configurable policy;
- prohibition of duplicate active Admissions;
- audit generation;
- restore/recreate strategy; and
- idempotent workflow execution.

## Implementation Leakage Check

No APIs, databases, Business Data Structures, services, frameworks, queues, deployment, infrastructure, technical access control/IAM, technical retries, or rollback mechanics are specified.

## Final Status

**PASS WITH ISSUES — Draft**

The document is complete across all 17 L3 sections. The remaining issues are policy/configuration details intentionally left to upstream authority rather than invented in L3.

> **Governance note:** Complete document ≠ Architecture Approved/Locked. This remains `Draft` until human architecture review.
