# Admission Revocation

**Scenario ID:** L3-C-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Critical  
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
| Final L3 ID | `L3-C-004` |
| Scenario Title | Admission Revocation |
| Primary Owner | Admissions |
| Participating Domains | C |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |
| Document Owner | Varad |
| Applicable L2 Workflow | `WF-C01-08 — Admission Revocation` |
| Canonical Business Record | Admission Record |
| Canonical Lifecycle Outcome | `Revoked` |
| Canonical Business Event | `AdmissionRevoked` |
| Canonical Section | C — Student Lifecycle |

The frozen Stage 3 Section C register identifies C-004 as **Admission Revocation**, Primary Owner **Admissions**, Participating Domain **C**, Scenario Type **Critical**, and Priority **P1**. fileciteturn72file0L2-L2

---

## 2. Scenario Purpose

The purpose of this scenario is to permanently invalidate an Admission Record that was previously approved or confirmed when an authorized institutional basis exists, including fraud, regulatory violation, administrative error, policy violation, eligibility violation, duplicate admission, court order, or government directive.

Admission Revocation is an exceptional institutional action. It terminates the Admission lifecycle while preserving the complete historical record required for institutional accountability, audit, legal defence, and regulatory compliance.

Unlike Cancellation or Withdrawal, Revocation asserts that the admission is no longer institutionally valid because of an authorized exceptional basis.

---

## 3. Business Outcome

The Admission Record reaches the canonical **Revoked** state.

Successful completion establishes that:

1. the institution has determined that the Admission must be revoked under an authorized basis;
2. the applicable revocation authority and policy have been satisfied;
3. supporting evidence has been validated;
4. the revocation reason has been recorded as immutable business history;
5. the Admission Record is in `Revoked`;
6. remaining Admission processing is terminated;
7. the revocation history and required audit evidence are preserved; and
8. Admissions publishes the canonical `AdmissionRevoked` business event.

Where revocation occurs after Student Context creation, the downstream Student Lifecycle domain may consume `AdmissionRevoked` and apply its own policy-controlled Student Context archival workflow. Admissions does not modify the Student Context.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authorized revocation basis or revocation request exists for an Admission Record.

The L2 workflow identifies the following possible triggers:

- Fraud Investigation Completed;
- Administrative Decision;
- Court Order;
- Government Directive; and
- Internal Audit Finding;
- Eligibility Violation Detected.

### 4.2 End Boundary

The Admissions portion of the scenario ends when the Admission Record has been transitioned to `Revoked`, the revocation history and required audit evidence have been recorded, and `AdmissionRevoked` has been published.

Where the revocation affects a previously created Student Context, downstream Student Lifecycle processing remains a separate domain-owned consequence.

### 4.3 In Scope

- Identification of a valid revocation basis.
- Validation of revocation authority.
- Resolution of institutional revocation policy.
- Validation of the Admission.
- Validation of supporting evidence.
- Recording of the revocation reason.
- Transition of the Admission Record to `Revoked`.
- Preservation of revocation history.
- Generation of required business audit evidence.
- Publication of `AdmissionRevoked`.
- Downstream recognition of post-creation revocation by Student Lifecycle without ownership transfer.

### 4.4 Out of Scope

- Legal proceedings themselves.
- Investigation execution itself.
- Financial recovery or refund processing.
- Academic record management.
- Certificate cancellation.
- Direct modification of Student Context.
- Direct modification of Finance-owned records.
- Technical authorization mechanisms.
- APIs, databases, services, Business Data Structures, queues, infrastructure, technical retries, rollback, or UI design.

### 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Distinct terminal outcome | `L3-C-002 — Admission Cancellation` | Cancellation is not an exceptional invalidation based on revocation authority/evidence. |
| Distinct terminal outcome | `L3-C-003 — Admission Withdrawal` | Withdrawal represents voluntary termination of an initially valid and accepted admission. |
| Downstream consequence | L2-C02 Student Lifecycle & Activation | Post-creation `AdmissionRevoked` may cause Student Lifecycle to archive the Student Context under its own policy. |

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **Admissions — Section C** | Owns the Admission Record, revocation decision, Admission lifecycle transition to `Revoked`, revocation history, and publication of `AdmissionRevoked`. |

The frozen scenario metadata identifies **C** as the participating domain.

### 5.1 Canonical Ownership

Admissions owns:

- Admission Record;
- Admission Status;
- Admission Lifecycle;
- Admission Revocation;
- Admission Audit Trail.

Admissions remains the canonical owner of the revocation decision even when other domains consume the completed event.

### 5.2 Post-Creation Boundary

Where `AdmissionRevoked` is published after Student Context creation:

- Admissions retains ownership of the revocation decision.
- Student Lifecycle retains ownership of Student Context.
- Student Lifecycle consumes the event and applies its own policy-controlled archival workflow.
- Admissions does not modify Student Context directly.

The L2 source explicitly records this post-creation consequence as an approved architecture decision.

### 5.3 Finance Boundary

Finance retains ownership of:

- payments;
- refunds;
- ledger;
- accounting; and
- financial recovery.

Admissions does not perform financial recovery directly.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| **Institution / authorized revocation authority** | Initiates and/or authorizes revocation under applicable institutional policy. | Holds the business authority required for revocation. |
| **Admissions business responsibility** | Validates authority, policy, Admission eligibility, supporting evidence, records the reason, revokes the Admission, preserves history, and publishes the completed event. | Canonical owner of the Admission Revocation outcome. |
| **Fraud / investigation responsibility** | Provides investigation findings or evidence where fraud is the revocation basis. | Supplies supporting evidence; does not own the Admission lifecycle. |
| **Compliance / audit responsibility** | May provide or validate supporting evidence where applicable. | Evidence/governance participation; does not own Admission. |
| **Judicial / Government authority** | May provide a court order or government directive forming the revocation basis. | External authoritative basis where applicable. |

The exact institutional human role/title is not invented because the L2 architecture explicitly allows institutions to configure revocation authorities.

---

## 7. Preconditions and Entry Criteria

The following conditions must be satisfied for normal revocation:

1. An Admission Record exists.
2. The Admission is active/eligible for revocation under the applicable Admission lifecycle.
3. The Admission has not already been revoked.
4. A valid revocation authority is available.
5. The institutional revocation policy has been resolved.
6. Supporting evidence is available.
7. A revocation reason can be documented.
8. The revocation basis corresponds to an authorized institutional circumstance.

Possible supporting evidence includes:

- fraud reports;
- investigation findings;
- legal orders;
- government directives; and
- administrative decisions.

The L2 workflow explicitly requires supporting evidence and documented reason.

---

## 8. Trigger

The business trigger is:

> **An authorized revocation basis or revocation request is raised for an Admission Record, requiring institutional invalidation of the admission.**

The L2 workflow identifies these trigger categories:

- Fraud Investigation Completed;
- Administrative Decision;
- Court Order;
- Government Directive;
- Internal Audit Finding; and
- Eligibility Violation Detected.

The trigger establishes the need for revocation consideration; it does not itself change the Admission state.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | A revocation request or authorized revocation basis is received. | Institution / authorized authority | Revocation processing begins. |
| 2 | Revocation authority is validated. | Admissions | The request is established as institutionally authorized or rejected. |
| 3 | The applicable institutional revocation policy is resolved. | Admissions | Governing revocation conditions are established. |
| 4 | The Admission is validated for revocation. | Admissions | The Admission is confirmed as eligible for the revocation workflow. |
| 5 | Supporting evidence is validated. | Admissions / relevant evidence responsibility | The revocation basis is supported by required evidence. |
| 6 | The revocation reason is recorded. | Admissions | An immutable business reason is associated with the revocation. |
| 7 | The Admission Record is transitioned to `Revoked`. | Admissions | The Admission lifecycle is terminated through Revocation. |
| 8 | Revocation history is preserved. | Admissions | Complete institutional Admission history remains available. |
| 9 | Required business audit evidence is recorded. | Admissions | Revocation remains auditable. |
| 10 | Admissions publishes `AdmissionRevoked`. | Admissions | The completed revocation business fact becomes available to applicable consumers. |
| 11 | Where Student Context already exists, Student Lifecycle applies its own policy-controlled archival workflow after consuming the event. | Student Lifecycle | Student Context ownership remains with Student Lifecycle. |
| 12 | The L3 scenario ends. | Admissions | The Admission Revocation outcome is complete. |

The Main Business Flow stops once the Admissions revocation outcome and canonical event are established. Downstream corrective actions remain outside Admissions ownership.

---

## 10. Alternate Business Flows

### 10.1 Administrative Revocation

An institution revokes an Admission based on an authorized administrative decision.

1. Revocation authority is validated.
2. Applicable institutional policy is resolved.
3. Admission eligibility is validated.
4. Required evidence is validated.
5. Revocation reason is recorded.
6. Admission becomes `Revoked`.
7. History and audit evidence are preserved.
8. `AdmissionRevoked` is published.

### 10.2 Fraud-Based Revocation

Where a fraud investigation establishes a valid revocation basis:

1. Investigation findings are supplied as supporting evidence.
2. Revocation authority and policy are validated.
3. Admission is validated.
4. Evidence is accepted as sufficient.
5. Revocation reason is recorded.
6. Admission becomes `Revoked`.
7. History and audit evidence are preserved.
8. `AdmissionRevoked` is published.

### 10.3 Court-Ordered Revocation

Where a court order provides the authorized basis:

1. The order is received as the relevant external authority.
2. Revocation authority and institutional policy are validated.
3. Admission is validated.
4. Supporting legal evidence is validated.
5. Revocation reason is recorded.
6. Admission becomes `Revoked`.
7. History and audit evidence are preserved.
8. `AdmissionRevoked` is published.

### 10.4 Government-Directed Revocation

Where a government directive provides the basis:

1. The directive is received.
2. Authority and applicable policy are validated.
3. Admission is validated.
4. Supporting evidence is validated.
5. Revocation reason is recorded.
6. Admission becomes `Revoked`.
7. History and audit evidence are preserved.
8. `AdmissionRevoked` is published.

These alternate flows differ in the source of the revocation basis but converge on the same canonical `Revoked` Admission outcome.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Revocation request is unauthorized | Revocation is rejected. | Establish valid revocation authority before re-submission. | Admissions |
| Admission is already revoked | No new revocation transition is required. | Recognize the existing revoked outcome. | Admissions |
| Supporting evidence is missing | Revocation workflow is suspended. | Supply the required supporting evidence before proceeding. | Admissions / evidence responsibility |
| Admission is invalid for revocation | Revocation workflow terminates without changing the Admission. | Follow the applicable canonical Admission lifecycle path. | Admissions |
| Revocation policy does not permit the action | Revocation does not complete. | Follow the applicable institutional policy/lifecycle. | Admissions |
| Evidence does not establish the revocation basis | Revocation does not complete. | Resolve the evidentiary deficiency under the governing process. | Admissions / relevant evidence responsibility |

The L2 workflow explicitly defines:

- `EX-01 — Unauthorized Request`;
- `EX-02 — Admission Already Revoked`;
- `EX-03 — Evidence Missing`; and
- `EX-04 — Invalid Admission`.

### 11.1 Business Irreversibility

The L2 architecture describes Revocation as irreversible for the current Admission lifecycle.

A later Readmission, if permitted by institutional policy, is a new Admission Record rather than reopening the revoked record.

### 11.2 Post-Creation Business Consequence

When a revoked Admission has already resulted in Student Context creation, Student Lifecycle consumes `AdmissionRevoked` and routes the Student Context through its existing policy-controlled archival workflow.

This is not a direct mutation by Admissions.

### 11.3 Explicit Technical Exclusion

No technical retry, queue, transaction rollback, infrastructure failure, business exception workflows, or server recovery is defined.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Revocation requires institutional authority. | Admissions | An unauthorized revocation cannot complete. | L2-C01 |
| Supporting evidence is mandatory. | Admissions / evidence responsibility | Revocation cannot complete without required evidence. | L2-C01 |
| Revocation reason becomes immutable. | Admissions | The completed revocation preserves its business basis. | L2-C01 |
| Revocation history is immutable. | Admissions | Historical evidence remains preserved. | L2-C01 |
| Admission records are never deleted. | Admissions | Revocation terminates the lifecycle without destroying history. | L2-C01 |
| Every revocation generates audit history. | Admissions | Revocation is institutionally auditable. | L2-C01 |
| Revocation terminates remaining Admission workflows. | Admissions | No further normal Admission processing continues. | L2-C01 |
| Revocation is idempotent. | Admissions | Repeated recognition of an already revoked Admission does not create another lifecycle outcome. | L2-C01 |
| Finance owns financial recovery. | Finance | Admissions does not perform financial recovery. | L2-C01 |
| Legal proceedings remain outside the revocation workflow. | Admissions / Legal boundary | The scenario records the business basis but does not conduct legal proceedings. | L2-C01 |
| Student Lifecycle owns Student Context after Student creation. | Student Lifecycle | Post-creation consequences are performed by Student Lifecycle. | L2-C01 / L2-C02 architecture |

### 12.1 Revocation vs Cancellation

Cancellation is a separate workflow for terminating an Admission before Student creation.

Revocation is an exceptional institutional action declaring that an admission is no longer institutionally valid.

### 12.2 Revocation vs Withdrawal

Withdrawal is voluntary termination of an initially valid and accepted admission.

Revocation is an institutionally authorized invalidation based on exceptional circumstances.

---

## 13. State and Lifecycle Impact

### 13.1 Owned Aggregate

**Admission Record**  
**Canonical Owner:** Admissions

### 13.2 Resulting State

The Admission Record transitions to:

`Revoked`

### 13.3 Canonical Lifecycle Position

The L2 lifecycle identifies `Revoked` as a terminal state for the current Admission Record.

The lifecycle transition matrix identifies:

```text
Student Created → Revoked (Policy Controlled)
```

and the Revocation workflow describes the business transition as:

```text
Any Valid Admission
        ↓
Revoked
```

### 13.4 Post-Creation Revocation

A specific approved architecture decision in the L2 source states that when `AdmissionRevoked` is published after Student Context creation:

1. L2-C02 consumes `AdmissionRevoked`;
2. Student Lifecycle routes the Student Context through its existing `WF-C02-10` / `ArchiveStudent()` path according to institutional policy;
3. Admissions retains ownership of the revocation decision;
4. Admissions does not modify Student Context;
5. Student Lifecycle retains Student Context ownership; and
6. archived history is preserved.

This is a cross-domain consequence, not an ownership transfer.

### 13.5 Lifecycle Termination

Revocation permanently terminates the current Admission lifecycle.

If Readmission is later permitted, it creates a new Admission Record.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AdmissionRevoked` | **Admissions** | Student Lifecycle and other applicable consumers | Represents the completed revocation business fact. | L2-C01 |

The L2 Revocation workflow also identifies the following business inputs/events:

- `RevocationRequested`;
- `FraudDetected`;
- `CourtOrderReceived`; and
- `GovernmentDirectiveReceived`.

These are triggers/consumed facts for the Revocation workflow and are distinct from the completed `AdmissionRevoked` event.

### 14.2 Event Publisher

The canonical publisher of `AdmissionRevoked` is **Admissions**.

No participating consumer becomes the publisher.

### 14.3 Student Lifecycle Collaboration

Where Student Context has already been created:

`AdmissionRevoked`

is consumed by Student Lifecycle.

Student Lifecycle applies its own policy-controlled Student Context archival workflow.

Admissions does not directly change Student Context.

### 14.4 Finance Collaboration

Finance owns financial recovery.

The Revocation workflow explicitly excludes financial recovery and refund processing from Admissions.

Any financial consequences are therefore processed under Finance ownership.

### 14.5 Compliance / Audit Collaboration

Supporting evidence and investigation history may be supplied by compliance/audit responsibilities.

These responsibilities provide evidence and governance context; they do not acquire ownership of the Admission Record.

### 14.6 Business Contract vs Event

The revocation request, authority validation, evidence validation, and policy decision are business interactions preceding the completed event.

`AdmissionRevoked` represents the completed business fact.

No technical API, event payload, queue, transport, or service contract is defined at L3.

---

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Revocation Authority

Revocation requires institutional authority.

The exact authority may be configured by the institution.

No unsupported human role title is invented in this scenario.

### 15.2 Supporting Evidence

Supporting evidence is mandatory.

Possible evidence includes:

- fraud reports;
- investigation findings;
- legal orders;
- government directives;
- administrative decisions.

The evidence must support the revocation basis.

### 15.3 Revocation Reason

The revocation reason is captured during the workflow and becomes immutable.

### 15.4 Audit Evidence

The L2 workflow requires audit information including:

- Revoking Authority;
- Revocation Reason;
- Supporting Evidence Reference;
- Previous Status;
- New Status;
- Timestamp; and
- Policy Version.

These are treated here as business evidence obligations.

### 15.5 Historical Integrity

Admission records are never deleted.

Revocation preserves the historical record required for:

- governance;
- legal defence;
- compliance audits; and
- institutional accountability.

### 15.6 Separation of Duties

The L2 source does not establish a universal additional separation-of-duty matrix for C-004.

Any configured institutional approval or investigation separation remains governed by upstream policy.

### 15.7 Overrides

No local override is introduced.

Any exception to normal revocation authority must be governed by the authoritative institutional policy.

---

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

C-004 is complete when:

1. a valid revocation basis/request exists;
2. revocation authority has been validated;
3. applicable revocation policy has been resolved;
4. the Admission has been validated;
5. supporting evidence has been validated;
6. the revocation reason has been captured;
7. the Admission Record has transitioned to `Revoked`;
8. revocation history has been preserved;
9. required business audit evidence has been recorded; and
10. Admissions has published `AdmissionRevoked`.

### 16.2 Postconditions

| Postcondition | Result |
|---|---|
| Admission lifecycle | **Revoked** |
| Admission owner | Admissions |
| Revocation reason | Immutable |
| Supporting evidence | Preserved/referenced as required by upstream governance |
| Revocation history | Preserved |
| Audit evidence | Recorded |
| `AdmissionRevoked` | Published by Admissions |
| Remaining Admission workflows | Terminated |
| Finance ownership | Unchanged |
| Student Context ownership | Remains with Student Lifecycle |
| Post-creation Student consequence | Student Lifecycle applies policy-controlled archival workflow |
| Current Admission Record | Revoked / terminal |

### 16.3 End Boundary

The scenario ends when the Admission Revocation outcome is complete and represented by the canonical `AdmissionRevoked` event.

The scenario does not extend into:

- legal proceedings;
- financial recovery;
- Student Context archival execution;
- academic record management;
- certificate cancellation.

Those remain outside Admissions' direct L3 business outcome.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

#### OI-C004-01 — Exact Institutional Revocation Authority

The L2 architecture requires institutional revocation authority and allows institutions to configure revocation authorities, but it does not establish one universal human role applicable to all institutions.

**Impact:** Non-blocking for the core business outcome; relevant to final authority traceability.

**Disposition:** Preserve the authority requirement at L3 and defer exact configured role/authority to institutional policy/configuration.

---

### 17.2 Assumptions

1. The frozen Stage 3 semantics for `L3-C-004` are authoritative.
2. `WF-C01-08 — Admission Revocation` is the applicable L2 workflow.
3. Revocation is distinct from Cancellation and Withdrawal.
4. Supporting evidence is mandatory.
5. The revocation reason is immutable.
6. `Revoked` is the canonical terminal Admission state for the current Admission Record.
7. `AdmissionRevoked` is published by Admissions.
8. Finance remains owner of financial recovery.
9. Student Lifecycle remains owner of Student Context.
10. Post-creation revocation causes Student Lifecycle to apply its approved policy-controlled archival path rather than Admissions directly modifying Student Context.
11. A later Readmission, if permitted, creates a new Admission Record.

---

### 17.3 Deferred Questions

| Question | Target Authority / Level | Reason |
|---|---|---|
| Which configured institutional role/authority performs the final revocation decision? | Admissions policy/configuration | L2 defines configurable revocation authorities. |
| What exact financial recovery follows revocation? | Finance | Finance owns financial recovery. |
| What exact Student Context archival rule applies after post-creation revocation? | L2-C02 | Student Lifecycle owns the Student Context and its archival workflow. |
| What exact legal process surrounds court/government-driven revocation? | Legal / governance | Legal proceedings are explicitly outside the L2 Revocation workflow. |

---

# Validation and Authoring Classification

**Classification: PASS WITH ISSUES — Draft**

## Identity Verification

- Final ID: `L3-C-004`
- Title: `Admission Revocation`
- Primary Owner: Admissions
- Participating Domain: C
- Scenario Type: Critical
- Priority: P1

The frozen Section C register confirms these values. fileciteturn72file0L2-L2

## L2 Verification

The applicable workflow is:

`WF-C01-08 — Admission Revocation`

The L2 source defines:

- revocation purpose and context;
- authorized trigger categories;
- preconditions;
- evidence requirements;
- main workflow;
- alternate flows;
- exception flows;
- `Revoked` outcome;
- `AdmissionRevoked` event;
- audit obligations; and
- cross-domain boundaries. fileciteturn73file0turn73file1 fileciteturn73file0turn73file2

## Lifecycle Verification

The canonical lifecycle identifies `Revoked` as a terminal state and permits policy-controlled revocation after Student creation. fileciteturn73file2turn73file4

## Post-Creation Ownership Verification

The L2 architecture explicitly states that after `AdmissionRevoked` where Student Context exists, Student Lifecycle consumes the event and routes Student Context through its existing archival path; Admissions retains revocation ownership and does not modify Student Context. fileciteturn73file0

## Event Verification

Canonical completed business event:

`AdmissionRevoked`

Canonical publisher:

**Admissions**

The Revocation workflow separately identifies incoming/trigger facts such as `RevocationRequested`, `FraudDetected`, `CourtOrderReceived`, and `GovernmentDirectiveReceived`. fileciteturn73file0

## Business Rule Verification

The L2 rules establish:

- institutional authority required;
- supporting evidence mandatory;
- immutable revocation history;
- no deletion;
- audit generation;
- termination of remaining Admission workflows; and
- idempotent revocation. fileciteturn73file0

## Implementation Leakage Check

No API, database, Business Data Structure, service, framework, queue, technical access control/IAM, deployment, infrastructure, technical retry, or rollback design has been introduced.

## Final Status

**PASS WITH ISSUES — Draft**

The only identified non-blocking issue is that the exact configured human revocation authority is institution-specific and is not invented here.

> **Governance note:** This is a complete L3 authoring artifact, not an Architecture Approved or Locked document. Phase 5 AI authoring remains `Draft` until human architecture review.
