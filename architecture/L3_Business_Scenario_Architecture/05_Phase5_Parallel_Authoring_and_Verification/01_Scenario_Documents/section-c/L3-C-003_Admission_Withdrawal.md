# Admission Withdrawal

**Scenario ID:** L3-C-003  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked Stage 3 Section C Canonical Scenario Register; L2-C01 — Admissions Workflows; L3_Business_Scenario_Template_v2.0.md  
**Referenced By:** None yet

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-003` |
| Scenario Title | Admission Withdrawal |
| Primary Owner | Admissions |
| Participating Domains | C |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |
| Document Owner | Varad |
| Applicable L2 Workflow | `WF-C01-07 — Admission Withdrawal` |
| Canonical Business Record | Admission Record |
| Canonical Withdrawal State | `Withdrawn` |
| Canonical Business Event | `AdmissionWithdrawn` |
| Canonical Section | C — Student Lifecycle |

The frozen Stage 3 register identifies C-003 as **Admission Withdrawal**, owned by **Admissions**, with **C** as the participating domain, **Simple** scenario type, and **P1** priority.

---

## 2. Scenario Purpose

To formally terminate an initially valid and accepted Admission through voluntary withdrawal initiated by the Applicant or processed administratively on the Applicant's behalf, while preserving the Admission as institutional history and applying the governing withdrawal policy.

Admission Withdrawal is distinct from Admission Cancellation and Admission Revocation.

The L2 Admissions architecture defines Withdrawal as a voluntary termination that assumes the admission was initially valid and accepted.

---

## 3. Business Outcome

The Admission Record reaches the canonical **Withdrawn** state.

At successful completion:

- the Admission is formally withdrawn;
- the withdrawal reason is recorded;
- the withdrawal decision is completed according to applicable policy;
- the Admission history is preserved;
- the canonical `AdmissionWithdrawn` business event is published by Admissions; and
- applicable downstream domains can perform their own responsibilities without acquiring ownership of the Admission Record.

The scenario's singular institutional outcome is:

> **Valid Accepted Admission → Withdrawn Admission**

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when a valid withdrawal request or authorized administrative withdrawal action is raised for an Admission Record that is eligible for withdrawal under the applicable institutional withdrawal policy.

The L2 workflow identifies Applicant Withdrawal Requests, Administrative Withdrawal Requests, Student Portal Requests, and Authorized Institutional Officer participation as possible initiation paths.

### 4.2 End Boundary

The scenario ends when the Admission Record has been transitioned to **Withdrawn**, the withdrawal reason/history has been recorded, the required business audit has been completed, and `AdmissionWithdrawn` has been published.

Downstream financial settlement, academic disengagement, operational closure, and any other consuming-domain consequences occur under the ownership of the respective domains.

### 4.3 In Scope

- Receipt of an Applicant or authorized administrative withdrawal request.
- Validation of the withdrawal request.
- Validation of applicant identity or administrative authorization.
- Resolution of the applicable institutional withdrawal policy.
- Determination of withdrawal eligibility.
- Evaluation of applicable admission status, withdrawal deadline, academic session, financial constraints, and institutional rules where defined by policy.
- Capture of the withdrawal reason.
- Approval of withdrawal according to policy.
- Transition of the Admission Record to `Withdrawn`.
- Preservation of withdrawal history.
- Required business audit evidence.
- Publication of `AdmissionWithdrawn`.
- Downstream recognition of the completed withdrawal without ownership transfer.

### 4.4 Out of Scope

- Admission Cancellation.
- Admission Revocation.
- Admission approval itself.
- Admission creation or initial application processing.
- Readmission.
- Student academic lifecycle management.
- Finance-owned refund, payment, accounting, or ledger processing.
- Academic disengagement owned by downstream academic domains.
- Technical authentication, APIs, databases, services, queues, retries, rollback, infrastructure, or UI design.

### 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Distinct terminal outcome | `L3-C-002 — Admission Cancellation` | Cancellation is a separate Admission termination path and must not be silently treated as Withdrawal. |
| Distinct terminal outcome | `L3-C-004 — Admission Revocation` | Revocation is a separate institutionally controlled termination path. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **Admissions — Section C** | Owns the Admission Record, Admission lifecycle, withdrawal workflow, withdrawal decision, Withdrawn state, withdrawal history, and `AdmissionWithdrawn` event. |

The frozen scenario metadata identifies **C** as the sole participating domain.

The L2 Admissions architecture identifies Admissions as the sole authority over the Admission Record and Admission Lifecycle.

Downstream domains may consume `AdmissionWithdrawn` for their own domain responsibilities, but consumption does not transfer Admission ownership.

Finance retains ownership of payments, refunds, ledger, and accounting consequences.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| **Applicant** | Initiates voluntary withdrawal and provides the withdrawal reason where applicant-initiated withdrawal is used. | May initiate withdrawal subject to institutional policy. |
| **Authorized Institutional Officer** | May process withdrawal on behalf of the Applicant where institutional policy permits administrative withdrawal. | Administrative initiation/processing authority as configured upstream. |
| **Admissions business responsibility** | Validates the request, resolves withdrawal policy, determines eligibility, applies the applicable approval, transitions the Admission, preserves history, and publishes the completed business event. | Canonical owner of the Admission Withdrawal outcome. |
| **Committee / institutional approval authority** | Participates where the applicable withdrawal policy requires committee approval. | Decision authority only where required by configured institutional policy. |

The L2 workflow explicitly supports:

- Applicant Withdrawal Request;
- Administrative Withdrawal Request;
- Student Portal Request; and
- Authorized Institutional Officer.

It also states that withdrawal approval may be automatic, administrative, or committee based.

No new role or technical permission model is introduced here.

---

## 7. Preconditions and Entry Criteria

The following business conditions must be satisfied for normal withdrawal processing:

1. An Admission Record exists.
2. The Admission is eligible for withdrawal under the applicable institutional withdrawal policy.
3. The Admission is not Cancelled.
4. The Admission is not Revoked.
5. The Applicant is authenticated where Applicant-initiated withdrawal is used, or the administrative requester has the applicable institutional authority.
6. The institutional withdrawal policy has been resolved.
7. The withdrawal request contains or permits capture of the withdrawal reason.
8. Any configured withdrawal deadline or eligibility condition is satisfied.

### 7.1 Admission State Entry Condition

The L2 workflow explicitly identifies **Admission Confirmed** as an eligible state and also contains a separate statement that **Student Created** may be eligible where institutional withdrawal policy permits.

However, the L2 canonical lifecycle transition matrix separately defines `Admission Confirmed → Withdrawn` and does not define `Student Created → Withdrawn`.

This unresolved upstream sequencing difference is recorded in Section 17 and is not silently resolved here.

---

## 8. Trigger

**A valid Applicant withdrawal request or authorized administrative withdrawal request is raised for an Admission Record that is eligible for withdrawal under institutional policy.**

The L2 workflow's primary flow is:

`Withdrawal Requested → Validate Request → Resolve Withdrawal Policy → Validate Eligibility → Capture Withdrawal Reason → Approve Withdrawal → Withdraw Admission → Persist Withdrawal → Generate Audit → Publish AdmissionWithdrawn`.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | A withdrawal request is received for an Admission Record. | Applicant / Authorized Institutional Officer | Withdrawal processing begins. |
| 2 | The withdrawal request and the requester's identity/authority are validated. | Admissions | The request is established as a valid business request or rejected. |
| 3 | The applicable institutional withdrawal policy is resolved. | Admissions | The governing withdrawal conditions are known. |
| 4 | Withdrawal eligibility is determined using applicable Admission status, withdrawal deadline, academic session, financial constraints, and institutional rules. | Admissions | The Admission is established as eligible or ineligible for withdrawal. |
| 5 | The Applicant's withdrawal reason is captured and preserved. | Applicant / Admissions | An immutable withdrawal reason is associated with the withdrawal. |
| 6 | Withdrawal approval is applied according to the applicable policy. Approval may be automatic, administrative, or committee based. | Admissions / Configured institutional authority | Withdrawal decision is established. |
| 7 | The Admission Record is transitioned to `Withdrawn`. | Admissions | The Admission lifecycle reaches the Withdrawn outcome. |
| 8 | Withdrawal history is preserved. | Admissions | Institutional Admission history remains complete. |
| 9 | Required business audit evidence is recorded. | Admissions | The withdrawal remains auditable. |
| 10 | Admissions publishes `AdmissionWithdrawn`. | Admissions | The completed withdrawal business fact becomes available to applicable consumers. |
| 11 | The L3 scenario ends. | Admissions | Withdrawn Admission is the completed institutional outcome. |

The Main Business Flow stops after the Withdrawal outcome and its canonical business fact. Downstream financial settlement, academic disengagement, or operational closure is not extended into this scenario.

---

## 10. Alternate Business Flows

### 10.1 Self Withdrawal

The Applicant directly requests withdrawal.

1. Applicant submits a withdrawal request.
2. Admissions validates the request and Applicant identity.
3. Withdrawal policy and eligibility are evaluated.
4. Withdrawal reason is captured.
5. Withdrawal is approved according to policy.
6. Admission becomes `Withdrawn`.
7. Withdrawal history and audit evidence are preserved.
8. `AdmissionWithdrawn` is published.

### 10.2 Administrative Withdrawal

An authorized institutional representative processes the withdrawal on behalf of the Applicant.

The Admissions workflow remains the canonical owner of the resulting Admission state.

### 10.3 Committee Approval

Where institutional policy requires committee approval:

1. The withdrawal request is validated.
2. Eligibility is determined.
3. The required committee decision is obtained.
4. If approved, the Admission becomes `Withdrawn`.
5. Withdrawal history and audit evidence are preserved.
6. `AdmissionWithdrawn` is published.

The exact committee structure is not invented at L3.

### 10.4 Withdrawal Policy Permits the Request

Where the applicable policy permits the withdrawal, the normal flow proceeds to the Withdrawn outcome.

### 10.5 Withdrawal Policy Does Not Permit the Request

The withdrawal does not complete and the Admission remains in its current valid lifecycle position.

The scenario does not invent a replacement lifecycle transition.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Withdrawal window is closed | Withdrawal is rejected. | Admission remains governed by its current lifecycle and applicable institutional policy. | Admissions |
| Requester is unauthorized | Withdrawal request is rejected. | A valid Applicant or authorized institutional request must be established. | Admissions |
| Admission is already Withdrawn | A second withdrawal does not create a new lifecycle outcome. | Existing Withdrawn result is returned/recognized. | Admissions |
| Admission is in an invalid state for withdrawal | Withdrawal workflow terminates without changing the Admission lifecycle. | Continue under the applicable canonical Admission lifecycle. | Admissions |
| Withdrawal eligibility conditions are not satisfied | Withdrawal cannot complete. | Preserve the current Admission state and follow the governing policy/lifecycle path. | Admissions |
| Required approval is not obtained | Withdrawal cannot complete. | Obtain the required policy-governed decision or remain in the current state. | Admissions |

The L2 workflow explicitly identifies:

- `EX-01 — Withdrawal Window Closed`;
- `EX-02 — Unauthorized Request`;
- `EX-03 — Admission Already Withdrawn`; and
- `EX-04 — Invalid Admission State`.

Technical retries and infrastructure recovery are outside L3.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Withdrawal represents voluntary termination of an initially valid and accepted admission. | Admissions | C-003 must remain distinct from Cancellation and Revocation. | L2-C01 |
| Applicant may initiate withdrawal where policy permits. | Admissions | Applicant-initiated withdrawal is a valid path. | L2-C01 |
| An authorized institutional representative may process withdrawal on behalf of the Applicant where permitted. | Admissions | Administrative withdrawal is supported. | L2-C01 |
| Withdrawal eligibility is governed by institutional policy. | Admissions | Deadline, academic session, financial constraints, and institutional rules may affect eligibility where configured. | L2-C01 |
| Withdrawal reason is captured and remains immutable. | Admissions | The completed Withdrawal preserves its stated reason. | L2-C01 |
| Withdrawal approval may be automatic, administrative, or committee based. | Admissions | The applicable configured approval path must be followed. | L2-C01 |
| Withdrawn Admissions preserve institutional history. | Admissions | Withdrawal does not erase the Admission Record. | L2-C01 |
| `AdmissionWithdrawn` represents the completed withdrawal business fact. | Admissions | The event is published only after successful business completion. | L2-C01 |
| Finance owns financial consequences. | Finance | Admissions does not perform refunds/accounting itself. | L2-C01 |
| Cross-domain consumption does not transfer ownership. | Architecture-wide | Downstream consumers retain their own ownership boundaries. | Locked architecture |

### 12.1 Distinction From Cancellation

Cancellation is a distinct scenario and workflow. Withdrawal assumes the admission was initially valid and accepted and is voluntarily terminated.

### 12.2 Distinction From Revocation

Revocation is institutionally controlled and governed by its own L2 workflow. It must not be substituted for voluntary Withdrawal.

---

## 13. State and Lifecycle Impact

### 13.1 Owned Business Record

**Admission Record**  
**Canonical Owner:** Admissions

### 13.2 Canonical Withdrawal State

The resulting lifecycle state is:

`Withdrawn`

The L2 lifecycle classifies Withdrawn as a terminal state for the current Admission Record.

### 13.3 Authoritative Transition

The canonical lifecycle transition matrix explicitly defines:

```text
Admission Confirmed → Withdrawn
```

### 13.4 Upstream State Ambiguity

The L2 Withdrawal workflow's preconditions additionally state:

```text
Admission Confirmed
or
Student Created, where institutional withdrawal policy permits.
```

However, the same L2 document's canonical state transition matrix does **not** define:

```text
Student Created → Withdrawn
```

Instead, it defines:

```text
Student Created → Revoked (Policy Controlled)
```

This is a material upstream lifecycle ambiguity and is therefore not resolved in L3.

### 13.5 Terminal State

`Withdrawn` terminates the lifecycle of the current Admission Record.

The L2 architecture also permits Readmission by creating a new Admission Record subject to institutional policy.

C-003 does not reopen the current Admission Record.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AdmissionWithdrawn` | **Admissions** | Applicable downstream consumers, including Finance for its own financial policy processing | Represents the completed Admission Withdrawal business fact. | L2-C01 |

The L2 Admissions architecture identifies Finance as consuming:

- `AdmissionCancelled`;
- `AdmissionRevoked`; and
- `AdmissionWithdrawn`

for refund policy execution.

### 14.2 Event Publication Boundary

`AdmissionWithdrawn` represents a **completed business fact**, not the withdrawal request.

The scenario therefore distinguishes:

- withdrawal request;
- withdrawal decision;
- Admission state transition; and
- completed `AdmissionWithdrawn` event.

### 14.3 Cross-Domain Collaboration

Finance may consume `AdmissionWithdrawn` and perform its own applicable financial settlement/refund processing.

Admissions does not modify Finance-owned:

- payments;
- refunds;
- ledger;
- accounting;
- receipts.

Any academic or operational downstream consumer remains responsible for its own domain consequences.

### 14.4 Business Contract / Hand-off

The business hand-off is the completed withdrawal fact represented by `AdmissionWithdrawn`.

No technical API, payload, message schema, queue, transport, or service contract is defined at L3.

---

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

Withdrawal may be initiated by:

- the Applicant; or
- an authorized institutional representative.

The L2 workflow states that withdrawal approval may be:

- automatic;
- administrative; or
- committee based.

The exact configured human role is not invented here.

### 15.2 Separation of Duties, Overrides and Exceptions

No additional scenario-specific separation-of-duty rule is established beyond the configured withdrawal authority and approval path.

No local override rule is invented.

### 15.3 Audit and Evidence Obligations

The L2 workflow requires:

1. withdrawal history to be persisted;
2. an immutable audit record to be generated; and
3. `AdmissionWithdrawn` to be published following successful withdrawal.

The L2 workflow identifies audit evidence including:

- correlation information;
- Admission Identifier;
- User Identifier;
- Institution Identifier; and
- Timestamp.

At L3, these are treated as business evidence obligations. Technical audit implementation is not defined.

### 15.4 Withdrawal Reason

The withdrawal reason is captured during the withdrawal workflow and remains immutable.

This reason forms part of the business history of the withdrawal.

---

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

C-003 is complete when:

1. a valid withdrawal request or authorized administrative withdrawal action has been received;
2. the requester's identity/authority has been validated;
3. the applicable withdrawal policy has been resolved;
4. withdrawal eligibility has been determined;
5. the withdrawal reason has been captured;
6. required withdrawal approval has been completed;
7. the Admission Record has transitioned to `Withdrawn`;
8. withdrawal history has been preserved;
9. required business audit evidence has been recorded; and
10. Admissions has published `AdmissionWithdrawn`.

### 16.2 Postconditions

| Postcondition | Result |
|---|---|
| Admission lifecycle | **Withdrawn** |
| Admission owner | Admissions |
| Withdrawal reason | Preserved as immutable business history |
| Withdrawal audit | Completed according to upstream obligation |
| Admission history | Preserved |
| `AdmissionWithdrawn` | Published by Admissions |
| Finance ownership | Unchanged |
| Downstream consequences | Remain owned by respective downstream domains |
| Current Admission Record | Terminal for its current lifecycle |

### 16.3 End Boundary

The scenario ends at the completed Withdrawal outcome.

It does not extend into:

- Finance refund execution;
- accounting;
- academic disengagement;
- operational closure;
- notification delivery;
- readmission.

Those are downstream responsibilities or separate canonical scenarios.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

#### OI-C003-01 — Student Created Withdrawal State Conflict

The applicable L2-C01 document contains two materially different statements:

**Withdrawal preconditions** state that the Admission may be in:

- `Admission Confirmed`; **or**
- `Student Created`, where institutional withdrawal policy permits.

However, the authoritative canonical Admission lifecycle transition matrix defines:

```text
Admission Confirmed → Withdrawn
```

and does **not** define:

```text
Student Created → Withdrawn
```

Instead, it defines:

```text
Student Created → Revoked (Policy Controlled)
```

**Architecture impact:** Material.

This affects the exact start boundary and lifecycle transition of C-003. L3 must not silently choose whether post-Student-Creation withdrawal is permitted.

**Required resolution:** Upstream architecture change-control resolution of the L2-C01 Withdrawal workflow versus canonical Admission lifecycle transition matrix.

**L3 treatment:** The main scenario is bounded to the unambiguous `Admission Confirmed → Withdrawn` transition. The post-Student-Creation interpretation remains explicitly unresolved.

---

#### OI-C003-02 — Exact Configured Human Withdrawal Authority

The L2 workflow supports automatic, administrative, and committee-based approval but does not establish one universal human role applicable to every institution.

**Architecture impact:** Non-blocking for the business outcome, but relevant to authorization traceability.

**Required resolution:** Use the applicable institutional configuration/policy when the scenario is reviewed.

---

### 17.2 Assumptions

1. The frozen Stage 3 semantics for `L3-C-003` are authoritative.
2. `WF-C01-07 — Admission Withdrawal` is the applicable L2 workflow.
3. Withdrawal is distinct from Cancellation and Revocation.
4. The canonical unambiguous lifecycle path for this L3 scenario is `Admission Confirmed → Withdrawn`.
5. `AdmissionWithdrawn` is published by Admissions as the completed business fact.
6. Withdrawal history and audit evidence remain preserved.
7. Finance owns any financial consequences arising from the withdrawal.
8. A future Readmission, where permitted, creates a new Admission Record rather than reopening the current Withdrawn record.

These assumptions do not override OI-C003-01.

---

### 17.3 Deferred Questions

| Question | Target Authority / Level | Reason |
|---|---|---|
| Can an Admission in `Student Created` state legitimately transition to `Withdrawn`? | L2-C01 / architecture change control | L2 workflow preconditions and lifecycle matrix disagree. |
| If post-Student-Creation withdrawal is permitted, does Student Lifecycle require a corresponding business lifecycle transition? | L2-C02 / architecture change control | Ownership transfers after Student creation. |
| Which configured institutional human role provides final withdrawal approval? | Admissions policy/configuration | L2 supports multiple approval modes without one universal role. |
| What financial consequence follows a completed withdrawal? | Finance | Finance owns refunds/accounting/financial settlement. |
| What academic/operational consequences follow a completed withdrawal? | Respective downstream domains | Those domains retain ownership of their business processes. |

---

# Validation and Authoring Classification

**Classification: BLOCKED — Draft**

## Verified

- Final ID: `L3-C-003`
- Title: `Admission Withdrawal`
- Primary Owner: Admissions
- Participating Domain: C
- Scenario Type: Simple
- Priority: P1
- Applicable L2 workflow: `WF-C01-07 — Admission Withdrawal`
- Canonical outcome: `Withdrawn`
- Canonical publisher: Admissions
- Canonical event: `AdmissionWithdrawn`
- Applicant and administrative initiation paths are supported.
- Withdrawal reason is captured and preserved.
- Withdrawal history and business audit are required.
- Technical implementation is excluded.

## Blocking Issue

The applicable locked L2-C01 architecture contains a **material lifecycle ambiguity**:

- the Withdrawal workflow preconditions allow `Student Created` where institutional withdrawal policy permits;
- the canonical lifecycle transition matrix does not permit `Student Created → Withdrawn`;
- the same matrix instead identifies `Student Created → Revoked (Policy Controlled)`.

Because the Phase 5 rules prohibit silently choosing or inventing an upstream lifecycle transition, C-003 must remain **Draft / BLOCKED** until the upstream architecture resolves this conflict.

This is deliberately recorded in Section 17 rather than hidden.

**Repository write:** None. This document is a local authoring artifact only; GitHub has not been modified.
