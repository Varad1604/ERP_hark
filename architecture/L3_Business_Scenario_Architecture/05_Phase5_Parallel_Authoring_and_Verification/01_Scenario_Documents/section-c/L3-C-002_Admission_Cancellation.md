# L3-C-002 — Admission Cancellation

**Scenario ID:** L3-C-002  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Author:** Varad  
**Last Updated:** 2026-08-22  
**Canonical Filename:** `L3-C-002_Admission_Cancellation.md`  
**Depends On:** `L2-C01_Admissions_Workflows.md`, especially `WF-C01-06 — Admission Cancellation`

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-002` |
| Canonical Title | Admission Cancellation |
| Primary Owner | Admissions |
| Participating Domains | C |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |
| Applicable L2 | `L2-C01_Admissions_Workflows.md` |
| Applicable Workflow | `WF-C01-06 — Admission Cancellation` |

The frozen Phase 5 identity and classification are unchanged.

---

## 2. Scenario Purpose

Govern the permanent termination of an eligible Admission Record before successful Student creation when cancellation is permitted by the authoritative Admissions lifecycle and applicable institutional policy.

The scenario preserves the Admission Record as institutional history and prevents the cancelled admission from progressing to Student creation.

---

## 3. Business Outcome

The Admission Record reaches the canonical `Cancelled` state and the current admission attempt is terminated without creating a Student.

Admissions remains the canonical owner of the Admission Record and cancellation outcome. Historical admission information remains preserved. The completed cancellation is represented by the canonical `AdmissionCancelled` business event.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario starts when an existing Admission Record is eligible for cancellation before Student creation and a valid cancellation basis is raised.

The basis may arise from an applicant request, institutional decision, applicable institutional policy, or regulatory requirement as supported by L2-C01.

### 4.2 End Boundary

The scenario ends when the Admission Record has reached `Cancelled`, the cancellation is preserved in institutional history, and the canonical cancellation business fact has been published.

### 4.3 In Scope

- Identify the Admission Record to be cancelled.
- Validate cancellation eligibility under the authoritative Admission lifecycle.
- Apply the applicable institutional cancellation policy and authority.
- Record the cancellation basis/reason required by the upstream workflow.
- Transition the Admission Record to `Cancelled` where permitted.
- Preserve the historical Admission Record.
- Publish `AdmissionCancelled` after the cancellation outcome is established.

### 4.4 Out of Scope

- Application processing, merit preparation, reservation, or seat allocation.
- Admission approval or confirmation as separate business outcomes.
- Student creation or Student Lifecycle processing.
- Admission Withdrawal, Revocation, Expiry, or Readmission as separate scenarios.
- Finance-owned refunds, payments, ledger, or accounting adjustments.
- Technical authentication/authorization design.
- APIs, databases, DTOs, services, queues, screens, infrastructure, retry mechanics, rollback mechanics, or other implementation design.

---

## 5. Ownership and Participating Domains

Admissions is the canonical owner of the Admission Record, Admission Lifecycle, Admission Status, and Admission Cancellation outcome.

The frozen participating-domain value for this scenario is Section C.

Finance may consume the completed cancellation fact for its own domain-owned financial consequences where applicable. Such consumption does not transfer ownership of the Admission Record, cancellation decision, or `Cancelled` lifecycle state.

Admissions does not directly modify Finance-owned records.

---

## 6. Business Actors, Roles and Responsibilities

| Actor / Business Responsibility | Responsibility | Authority Boundary |
|---|---|---|
| Applicant | May initiate cancellation where institutional policy permits applicant-initiated cancellation. | Does not own the Admission lifecycle or cancellation decision. |
| Admissions | Evaluates cancellation eligibility, applies the governing Admissions policy/authority, establishes the cancellation outcome, preserves history, and publishes the completed business fact. | Canonical owner of the Admission Record and cancellation outcome. |

L2-C01 makes admission authority and approval chains configurable and does not establish one universal human role for every institution. This L3 scenario therefore does not invent a role title for the cancellation authority.

---

## 7. Preconditions and Entry Criteria

- A valid Admission Record exists.
- Successful Student creation has not occurred for the Admission Record.
- The Admission Record is in a state from which the authoritative L2 cancellation workflow permits cancellation.
- A valid cancellation basis exists.
- Applicable institutional cancellation policy conditions are satisfied.
- Any required business authority is satisfied according to upstream governance.

Cancellation must preserve the Admission Record and its institutional history.

---

## 8. Trigger

A valid applicant cancellation request or institutional cancellation decision is raised for an Admission Record that is eligible for cancellation before Student creation.

The trigger itself does not change the Admission state; the state changes only when the governed cancellation decision completes.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Responsibility | Business Result |
|---|---|---|---|
| 1 | Identify the Admission Record and cancellation basis. | Admissions / Applicant where applicant-initiated | A specific Admission Record and cancellation basis are established. |
| 2 | Confirm that Student creation has not occurred. | Admissions | The scenario remains within the pre-Student-creation cancellation boundary. |
| 3 | Validate the current Admission state against the authoritative cancellation workflow. | Admissions | Cancellation eligibility is established. |
| 4 | Apply the applicable institutional cancellation policy and required authority. | Admissions | The cancellation decision is governed by upstream policy. |
| 5 | Record the required cancellation reason/basis and establish the cancellation decision. | Admissions | The business decision is attributable and auditable. |
| 6 | Transition the Admission Record to `Cancelled`. | Admissions | The current Admission lifecycle reaches its cancellation outcome. |
| 7 | Preserve the completed cancellation in Admission history. | Admissions | Historical integrity is maintained. |
| 8 | Publish the canonical `AdmissionCancelled` event representing the completed cancellation. | Admissions | Approved downstream consumers can react to the completed business fact. |
| 9 | End the scenario. | Admissions | The Admission Record remains terminated in `Cancelled`. |

The scenario does not extend into downstream refund, accounting, notification-delivery, or Student Lifecycle outcomes.

---

## 10. Alternate Business Flows

### 10.1 Applicant-Initiated Cancellation

The Applicant raises a cancellation request. Admissions validates lifecycle eligibility and applicable policy. If cancellation is permitted, the Admission Record reaches `Cancelled` and `AdmissionCancelled` is published.

### 10.2 Institution-Initiated Cancellation

The institution establishes a valid cancellation basis. Admissions applies the authoritative cancellation workflow and policy. If permitted, the Admission Record reaches `Cancelled` and `AdmissionCancelled` is published.

### 10.3 Eligible-State Variants

The authoritative L2 cancellation workflow supports cancellation from the eligible pre-Student-creation Admission states defined by L2-C01. The scenario behaviour remains the same regardless of which eligible state is the starting state: eligibility is validated, the governed decision is completed, and the Admission Record reaches `Cancelled`.

---

## 11. Failure, Interruption and Business Recovery

| Condition | Business Result | Business Handling |
|---|---|---|
| Current Admission state is not eligible for cancellation | Cancellation does not complete. | Continue through the applicable authoritative Admission lifecycle path. |
| Student creation has already occurred | Admission Cancellation is prohibited for this scenario. | Use the applicable post-creation Revocation or Student Lifecycle process defined upstream. |
| Cancellation policy condition is not satisfied | Admission remains in its current valid state. | Resolve through the applicable governed policy/path; do not force cancellation. |
| Required business authority is not satisfied | Cancellation cannot complete. | Obtain the required upstream-governed decision/authority before progressing. |
| Cancellation basis is invalid | Admission remains unchanged. | Follow the applicable canonical alternative lifecycle path. |

No technical retry, transaction rollback, exception-handler, queue, infrastructure-failover, or database-recovery behaviour is defined at L3.

---

## 12. Business Rules and Decision Constraints

The scenario applies the following upstream-owned constraints without assigning new L3 rule identifiers:

- Admission Cancellation terminates an admission before successful Student creation.
- An admission that has already resulted in Student creation shall not be cancelled through this scenario.
- Cancellation may be initiated by the institution or applicant subject to applicable institutional policy.
- Cancelled admissions remain preserved as institutional history.
- Admissions retains ownership of the Admission Record and cancellation outcome.
- Finance retains ownership of payments, refunds, ledger, and accounting consequences.
- Cross-domain consumption of `AdmissionCancelled` does not transfer Admission ownership.
- `AdmissionCancelled` represents the completed cancellation business fact and is published only after the cancellation outcome is established.
- Readmission, where permitted, creates a new Admission Record rather than reopening the cancelled Admission Record.

These constraints are derived from `L2-C01_Admissions_Workflows.md`, including its Admission lifecycle, cancellation workflow, ownership boundaries, and event model. L3 does not create a parallel rule namespace.

---

## 13. State and Lifecycle Impact

The canonical lifecycle owner remains Admissions.

The applicable cancellation outcome is:

`Eligible pre-Student-creation Admission state → Cancelled`

L2-C01 `WF-C01-06 — Admission Cancellation` and the canonical transition architecture govern which current states are eligible for that transition.

`Cancelled` remains historical for the current Admission Record. Where institutional policy permits Readmission, the upstream architecture creates a new Admission Record linked to the same Person rather than reopening the cancelled record.

No new lifecycle state or transition is introduced by this L3 scenario.

---

## 14. Business Events and Cross-Domain Collaboration

| Business Fact | Canonical Event | Canonical Publisher | Scenario Relevance |
|---|---|---|---|
| Admission cancellation completed | `AdmissionCancelled` | Admissions | Represents the completed `Cancelled` Admission outcome. |

L2-C01 identifies downstream consumers such as Finance for domain-owned consequences. Consumers do not acquire ownership of the Admission Record or cancellation lifecycle.

The scenario does not define transport protocols, topics, queues, payloads, APIs, or direct cross-domain writes.

---

## 15. Business Authorization, Approval and Audit Obligations

The cancellation must be attributable to a valid applicant request or institutional cancellation basis and must satisfy the applicable upstream cancellation policy and business authority.

The cancellation reason/basis, decision, lifecycle transition, and resulting business fact must remain traceable within the Admission history.

L2-C01 treats institutional authority and approval chains as configurable. This L3 scenario therefore does not invent a universal human role or define technical RBAC/IAM behaviour.

---

## 16. Completion Criteria and Postconditions

The scenario is complete when:

- cancellation eligibility has been validated;
- applicable policy and required business authority have been satisfied;
- the Admission Record has reached `Cancelled`;
- successful Student creation has not occurred for the cancelled Admission Record;
- historical Admission information remains preserved; and
- `AdmissionCancelled` has been published by Admissions as the completed business fact.

Postcondition: the current Admission Record is terminated in `Cancelled`; any later financial or other downstream consequences remain owned by their respective domains.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

**OI-C002-01 — Institution-specific cancellation authority:** L2-C01 establishes configurable admission authority/approval chains rather than one universal named institutional role. The exact configured cancellation authority must remain an upstream policy/configuration concern unless a more specific authoritative role is established.

### 17.2 Assumptions

- `WF-C01-06 — Admission Cancellation` remains the authoritative L2 workflow for this scenario.
- Admissions remains the canonical publisher of `AdmissionCancelled`.
- Downstream domains consume the completed cancellation fact without modifying the Admission-owned lifecycle.

### 17.3 Deferred Questions

Institution-specific approval routing and policy configuration remain outside this L3 scenario unless established by higher-authority business architecture.
