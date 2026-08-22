# Approved Candidate to Active Student

**Scenario ID:** L3-C-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle — Section C  
**Participating Domains:** Student Lifecycle, Finance, Identity  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-C01 Admissions Workflows; L2-C02 Student Lifecycle & Activation Workflows; Section A Identity/Authorization architecture; Section F Finance architecture  
**Referenced By:** None yet

---

## 2. Scenario Purpose

Convert an approved admission candidate into an operational student by completing the business conditions required for Student Lifecycle activation.

The scenario establishes the candidate's Student Context and reaches the **Active** Student state while preserving canonical ownership across Student Lifecycle, Finance, and Identity.

---

## 3. Business Outcome

An approved candidate has a Student Context in the **Active** state and is recognized as an operational student within the institution.

The Student Lifecycle domain remains the canonical owner of the Student Context and Active Student outcome.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an **approved admission** is available for activation.

Admission approval itself belongs to the Admissions capability and does not itself activate a Student.

### 4.2 End Boundary

The scenario ends when the Student Lifecycle domain has established the Student Context in the **Active** state.

Subsequent academic registration is outside this scenario's completion boundary unless required as part of the authoritative activation workflow.

### 4.3 In Scope

- Receipt/recognition of an approved admission for activation.
- Required document verification.
- Required financial clearance.
- Student Context creation.
- Transition of the Student Context to Active.
- Publication of the canonical `StudentActivated` business event.
- Preservation of canonical ownership across participating domains.

The locked L1 activation sequence is:

`AdmissionApproved → VerifyDocuments → VerifyFinance → CreateStudentContext → ActivateStudent`.

### 4.4 Out of Scope

- Admission decision-making.
- Admission approval or rejection.
- Academic registration and course enrollment beyond what is strictly required to establish activation.
- Academic progression.
- Examination.
- Finance-owned financial processing.
- Technical authentication, authorization, persistence, or event-delivery mechanisms.

### 4.5 Related Scenarios

No authoritative related final L3 scenario has been established for inclusion here.

---

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| **Section C — Student Lifecycle** | Owns Student Context, activation, and the final Active Student outcome. |
| **Section F — Finance** | Provides the required financial clearance/status needed for activation; does not activate the student. |
| **Section A — Platform Foundation / Identity** | Provides canonical identity context required by the institutional student lifecycle; does not own Student Context. |

Student Lifecycle is explicitly the canonical owner of Student Context. Finance does not activate students.

---

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Applicant / approved candidate | Provides the institutional subject of activation and required information/evidence. | Participates in activation prerequisites. |
| Student Lifecycle business responsibility | Performs the Student Context activation process. | Establishes the Student Context and Active state. |

The locked L1 architecture identifies **Applicant**, **Admission Officer**, and **Registrar** as admission actors, but it does not provide a canonical activation-specific human role for C-001. Therefore this draft does **not** invent an activation officer/administrator role.

---

## 7. Preconditions and Entry Criteria

- An admission has been approved for the candidate.
- Required documents for activation are available for verification.
- Required financial clearance is available.
- The candidate can be associated with the canonical Person identity.
- No existing Student Context for the Institute violates the single-Active-Context rule.

The L1 architecture requires Approved Admission, Required Documents, and Financial Clearance as inputs to Student Activation.

---

## 8. Trigger

**An approved admission becomes eligible for Student Lifecycle activation.**

The authoritative L1 trigger/follow-on fact is `AdmissionApproved`.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | An approved admission is made available for Student Lifecycle activation. | Admissions / Student Lifecycle | Approved candidate enters the activation scenario. |
| 2 | Required documents are verified for activation. | Student Lifecycle | Required activation evidence is satisfied. |
| 3 | Required financial clearance is verified. | Finance / Student Lifecycle | Financial activation prerequisite is satisfied. |
| 4 | Student Lifecycle establishes the Student Context for the approved candidate. | Student Lifecycle | Student Context exists in its initial activation position. |
| 5 | Student Lifecycle activates the Student Context. | Student Lifecycle | Student Context transitions to **Active**. |
| 6 | Student Lifecycle publishes the canonical `StudentActivated` business fact. | Student Lifecycle | The institutional Active Student outcome is available to consuming domains. |

The underlying L1 workflow explicitly defines the sequence from `AdmissionApproved` through document verification, financial verification, Student Context creation, and activation.

---

## 10. Alternate Business Flows

No scenario-specific alternate business flow is established by the authoritative material currently available.

The scenario therefore does not introduce additional governed alternatives merely to populate this section.

---

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required documents are not satisfactorily available for activation | Activation cannot validly reach the Active Student outcome. | Activation remains incomplete until the required activation condition is satisfied. | Student Lifecycle |
| Required financial clearance is not available | Activation cannot validly reach the Active Student outcome. | Activation remains incomplete until the required financial condition is satisfied. | Finance / Student Lifecycle |
| Existing Student Context prevents establishment of the required Active state under the single-Active-Context rule | A new Active Student Context cannot validly be established. | Activation remains incomplete pending the governing Student Lifecycle condition. | Student Lifecycle |

No technical retry, queue, rollback, or infrastructure recovery is defined.

The locked architecture establishes that only one Student Context may remain Active within an Institute.

---

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Admission approval does not activate a Student. | Student Lifecycle / Admissions boundary | C-001 must perform a distinct activation outcome after admission approval. | Locked L1 |
| Canonical Person lookup is mandatory during admission. | Identity / Admissions boundary | Activation must preserve canonical person identity rather than establish a duplicate identity. | Locked L1 |
| Duplicate Person creation is prohibited. | Identity | Activation cannot create a competing person identity. | Locked L1 |
| Student Lifecycle owns Student Context. | Student Lifecycle | Student Context creation and activation remain within Section C ownership. | Locked L1 |
| Only one Student Context may remain Active within an Institute. | Student Lifecycle | Activation must respect the existing Active-Context gate. | Locked L1 |
| Finance does not activate students. | Finance / Student Lifecycle boundary | Financial clearance is a prerequisite/contribution, not the activation decision or lifecycle transition. | Locked L1 |

---

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Student Context | Pending Activation | Activation prerequisites are satisfied and Student Lifecycle activates the context. | **Active** | Student Lifecycle | Locked L1 |

The canonical Student Context lifecycle includes `PendingActivation → Active`. Active is non-terminal; subsequent lifecycle transitions include suspension and terminal student-exit states.

The scenario does not redefine any Student Context state or transition.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AdmissionApproved` | Admissions | Student Lifecycle | Establishes the approved-admission business fact that begins activation. | Locked L1 |
| `StudentActivated` | Student Lifecycle | Interested downstream domains | Records completion of the Active Student business outcome. | Locked L1 |

The L1 canonical event catalogue identifies `AdmissionApproved` and `StudentActivated` as immutable business events.

**Important:** `StudentActivated` is published by Student Lifecycle. Admission does not publish the Active Student outcome.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Finance | Student Lifecycle | Financial clearance/status required for activation | Establish whether the financial activation prerequisite is satisfied. | Locked L1 |
| Identity | Student Lifecycle | Canonical identity context | Preserve canonical Person identity during Student Lifecycle activation. | Locked L1 / platform architecture |

The repository's communication model distinguishes synchronous business contracts from completed business-fact events.

No API, payload, transport, database, or implementation mechanism is defined here.

---

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Activate approved candidate as Student | **Activation-specific canonical business role is not established in the currently accessible authoritative source material.** | Activation must occur only after the required activation prerequisites are satisfied. | Student Lifecycle |

The repository explicitly requires authors to record an undefined actor/authority rather than inventing a role.

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override, or exception requirement is established in the authoritative material currently accessible for C-001.

### 15.3 Business Audit and Evidence Obligations

No additional scenario-specific business evidence obligation is established beyond the authoritative business facts and activation prerequisites already defined upstream.

Platform audit infrastructure remains owned by Section A and is not re-owned by Student Lifecycle.

---

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The approved candidate's required activation prerequisites have been satisfied.
- A Student Context has been established under Student Lifecycle ownership.
- The Student Context is in the **Active** state.
- The institutional `StudentActivated` business event is published by Student Lifecycle.

### 16.2 Postconditions

- The candidate is an operational Active Student.
- The Student Context is owned and governed by Student Lifecycle.
- The Active Student outcome is available for subsequent institutional processes.
- Admission remains a distinct completed upstream business responsibility; its approval has not been re-owned by Student Lifecycle.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **Full authoritative L2-C01 Admissions and L2-C02 Student Lifecycle & Activation documents were not retrievable through the current file-search results; only their locked existence and relevant L1-governed semantics are currently exposed.** | Prevents full verification of L2-specific workflow identifiers, exact business contracts, actor/authority definitions, detailed rules, alternate paths, and any L2-specific event publisher detail before the scenario can be considered fully source-verified. | Read and reconcile the complete locked L2-C01 and L2-C02 documents before moving C-001 beyond Draft. |

### 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| The L1 activation sequence accurately represents the governing C-001 business path pending direct L2 verification. | Locked L1 explicitly defines the Student Activation workflow. | L2 may contain additional materially relevant business behaviour that must be incorporated or recorded before approval. |

### 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| What exact system behaviour realizes the approved business activation flow? | L4 | Implementation-independent L3 must not prescribe system behaviour. |

---

## Validation Result

**Classification: PASS WITH ISSUES — Draft**

The scenario preserves the verified Final ID, canonical title, exact filename, owner, Type, Priority, business outcome, lifecycle boundary, ownership, and implementation-independent L3 scope.

The remaining issue is source completeness: the full locked L2-C01 and L2-C02 documents must be directly read and reconciled before this Draft can progress to Review.
