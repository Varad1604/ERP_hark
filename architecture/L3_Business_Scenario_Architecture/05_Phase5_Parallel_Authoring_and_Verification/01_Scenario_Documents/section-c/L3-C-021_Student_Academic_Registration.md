# Student Academic Registration

**Scenario ID:** L3-C-021  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Registration  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P0  
**Derivation:** KEEP  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Exact Filename:** `L3-C-021_Student_Academic_Registration.md`

---

| Attribute | Value |
|---|---|
| Final L3 ID | `L3-C021` |
| Canonical Scenario Title | **Student Academic Registration** |
| Primary Owner | **Academic Registration** |
| Participating Domains | **C, D** |
| Scenario Type | **Critical** |
| Priority | **P0** |
| Derivation | **KEEP** |
| Assigned Author | **Varad** |
| Status | **Draft** |

The frozen Section C canonical register identifies this scenario with the exact title, owner, participating domains, type, priority and KEEP derivation. Final Section C IDs are directly positionally aligned, so the frozen canonical sequence maps to the corresponding `L3-C-NNN` identity. The Phase 5 workload assigns all Section C scenarios to Varad. 

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-C-021 |
| Canonical Title | Student Academic Registration |
| Primary Owner | Academic Registration |
| Participating Domains | C, D |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario describes the complete institutional business outcome represented by **Student Academic Registration**.

It applies the locked Academic Registration architecture without transferring ownership to participating domains. The scenario remains implementation-independent and stops when its institutional business outcome and end boundary are reached.

---

## 3. Business Outcome

An eligible Student has a valid Academic Registration for the applicable academic period, containing the institutionally permitted academic offerings and satisfying the authoritative registration validation and approval requirements.

The outcome is owned by **Academic Registration**. Participating domains provide or consume authoritative business information within their own ownership boundaries and do not acquire ownership of the Academic Registration Aggregate.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Successful Student eligibility validation while the approved registration period is active.

## 4.2 End Boundary

The scenario ends when the business outcome stated in Section 3 has been reached and the applicable canonical lifecycle/event outcome has been completed.

## 4.3 In Scope

- The business trigger and required entry conditions.
- Validation and decision-making explicitly supported by locked L2-C03.
- The applicable Academic Registration lifecycle behaviour.
- Cross-domain business participation required by the frozen scenario.
- Canonical business events explicitly defined by L2-C03.
- Business-level audit, approval and exception obligations.
- Clear completion and postconditions.

## 4.4 Out of Scope

- API/interface implementation.
- Database or persistence design.
- Business Data Structures, classes, services or code.
- Technical IAM/RBAC.
- UI/UX design.
- Deployment or infrastructure.
- Technical queues, retries, timeouts or rollback mechanics.
- Redesign of upstream ownership, rules, lifecycle or events.
- Detailed downstream behaviour not defined by authoritative L2 architecture.

---

## 5. Ownership and Participating Domains

| Domain | Role in Scenario | Ownership |
|---|---|---|
| **Academic Registration** | Owns the Academic Registration business lifecycle and the relevant registration outcome. | **Canonical owner** |
| **C — Student Lifecycle** | Provides/participates in Student lifecycle information where applicable. | Retains Student ownership. |
| **D — Academic Delivery** | Participates in downstream academic execution where applicable. | Retains Academic Delivery ownership. |
| **F — Finance** | Participates where finalized registration is consumed or where financial clearance is an authoritative prerequisite. | Retains Finance ownership. |
| **I — Examination** | Participates where examination/academic-result information is relevant to the frozen scenario. | Retains Examination ownership. |

Only the domains listed in the frozen scenario are treated as participating domains. Participation never transfers ownership.

---

## 6. Business Actors, Roles and Responsibilities

The locked L2-C03 architecture supports the following business participation:

| Actor / Role | Responsibility | Boundary |
|---|---|---|
| Student | Initiates or participates in permitted registration activity where applicable. | Cannot bypass institutional registration rules. |
| Academic Registration | Performs the canonical registration workflow and lifecycle transitions. | Sole owner of Academic Registration. |
| Authorised academic/administrative participant | Performs institutionally permitted approval, modification, cancellation or override actions. | Authority is governed by configured institutional policy. |
| Applicable downstream academic domain | Consumes finalized registration business facts where defined. | Cannot directly modify the Registration Aggregate. |

L2-C03 names possible approval participants including Faculty Advisor, Programme Coordinator, Department Office, Registrar and Academic Administration, but approval is configuration-driven. This scenario therefore does not invent a universal approver.

---

## 7. Preconditions and Entry Criteria

The applicable prerequisites are:

- an active academic session;
- an approved/open registration period where required;
- an existing Student for Student-specific registration;
- applicable academic eligibility;
- published academic offerings;
- approved registration configuration;
- applicable curriculum/programme mapping;
- required prerequisites/co-requisites;
- no disqualifying registration hold;
- applicable financial clearance where configured; and
- any required institutional approval.

Only prerequisites actually relevant to this scenario are applied; downstream domain ownership remains unchanged.

---

## 8. Trigger

**Successful Student eligibility validation while the approved registration period is active.**

The trigger is business-level. No technical scheduler, API call, queue, command or implementation mechanism is introduced.

---

## 9. Main Business Flow

WF-C03-02 Student Eligibility Validation → WF-C03-03 Academic Registration Creation → WF-C03-04 Academic Offering Selection → WF-C03-05 Registration Validation → WF-C03-06 Registration Approval Workflow.

1. Confirm that the triggering business condition is valid.
2. Confirm the applicable Student/academic/registration prerequisites.
3. Apply the authoritative registration rules.
4. Execute the applicable Academic Registration business activity.
5. Revalidate the registration wherever the locked workflow requires validation after a modification.
6. Obtain required approval where the configured registration policy requires it.
7. Apply the canonical lifecycle transition supported by L2-C03.
8. Publish only the canonical business events explicitly defined by L2-C03.
9. Create the required audit evidence.
10. Stop when the Business Outcome and End Boundary are reached.

For **C021**, the concrete flow is eligibility validation → registration creation → offering selection → registration validation → approval.

For **C022**, the concrete flow is permitted modification/add-drop → full revalidation → resulting registration state.

For **C023**, the concrete flow is cancellation trigger → reason recording → lifecycle cancellation → `StudentRegistrationCancelled`.

For **C024**, the concrete flow is permitted repeat/backlog selection → validation → approval where required.

For **C025**, the concrete flow is approved registration → freeze → publication.

---

## 10. Alternate Business Flows

## 10.1 Rejection / Ineligibility

If a mandatory eligibility or registration rule fails, the business process does not complete successfully. The condition must be resolved through the applicable institutional process before successful continuation.

## 10.2 Approval Rejection

Where approval is required and rejected, the registration does not reach Approved/Frozen/Published as applicable.

## 10.3 Administrative Exception

Where the locked architecture permits an administrative override or policy exception, it must use the applicable institutional authorization and remain auditable.

## 10.4 Scenario-Specific Alternate

- **C021:** Eligibility may produce Eligible, Ineligible or Manual Review Required.
- **C022:** Add, Drop, elective change, subject replacement or project modification may be performed where permitted.
- **C023:** Cancellation may originate from Student request, administrative action or policy enforcement.
- **C024:** Repeat-course selection may be applied through Academic Offering Selection; detailed examination-side determination remains upstream.
- **C025:** Finalization proceeds only from an approved, validated registration when the freeze condition is reached.

---

## 11. Failure, Interruption and Business Recovery

| Business Failure | Business Resolution |
|---|---|
| Registration window closed | Registration cannot proceed unless a separately authorized late-registration/override policy applies. |
| Student ineligible | Resolve the underlying eligibility condition through the owning business process. |
| Credit limit exceeded | Modify the registration or obtain the applicable authorized exception. |
| Prerequisite/co-requisite not satisfied | Resolve the academic prerequisite condition or apply an authorized policy exception. |
| Duplicate registration detected | Remove/resolve the duplicate before successful completion. |
| Required approval rejected | Resolve the rejection through the institutional approval process. |
| Required configuration missing | Complete the governing academic/registration configuration. |
| Authorization unavailable | Obtain the required business authority before proceeding. |

The L2 source explicitly identifies these as registration workflow failure conditions. Technical retries, queues, infrastructure failures and rollback are intentionally excluded from L3.

---

## 12. Business Rules and Decision Constraints

The authoritative L2-C03 rules applied to these scenarios include:

- registration only during approved registration windows;
- active Student and programme eligibility;
- applicable semester eligibility;
- financial clearance where configured;
- academic standing and registration-hold validation;
- published academic offerings;
- curriculum/programme compliance;
- duplicate prevention;
- maximum/minimum credit limits;
- prerequisite and co-requisite validation;
- complete validation after modification;
- modification only before freeze unless explicitly authorized;
- configured approval workflows;
- required approval before Approved;
- cancellation reason recording;
- frozen registration immutability;
- authorized administrative override;
- complete auditability;
- domain-event publication only after successful business completion.

For C024, repeat-course selection is explicitly supported by the Academic Offering Selection workflow. The L2 source does not provide a complete backlog-specific rule matrix; therefore no examination rule is invented.

---

## 13. State and Lifecycle Impact

The locked L2-C03 lifecycle sequence is:

```text
Window Open
  ↓
Eligibility Validated
  ↓
Registration Created
  ↓
Academic Offerings Selected
  ↓
Registration Validated
  ↓
Approved
  ↓
Frozen
  ↓
Published
```

A cancellation branch exists from the registration lifecycle to **Cancelled**.

Scenario-specific lifecycle application:

- **C021:** progresses through creation/selection/validation/approval.
- **C022:** modifies an existing registration before Freeze.
- **C023:** transitions the applicable registration to Cancelled.
- **C024:** records repeat/backlog offerings within the normal Academic Registration lifecycle.
- **C025:** applies Approved → Frozen → Published.

No additional states are introduced.

---

## 14. Business Events and Cross-Domain Collaboration

## Canonical Events

| Event | Publisher | Scenario Relevance |
|---|---|---|
| `RegistrationEligibilityValidated` | Academic Registration | C021 |
| `RegistrationCreated` | Academic Registration | C021 |
| `CourseRegistered` | Academic Registration | C021/C024/C022 where applicable |
| `SubjectRegistered` | Academic Registration | C021 where applicable |
| `ElectiveRegistered` | Academic Registration | C021/C022 where applicable |
| `RegistrationSubmitted` | Academic Registration | C021/C024 |
| `StudentRegistrationApproved` | Academic Registration | C021/C024 |
| `RegistrationModified` | Academic Registration | C022 |
| `CourseDropped` | Academic Registration | C022 |
| `StudentRegistrationCancelled` | Academic Registration | C023 |
| `RegistrationFrozen` | Academic Registration | C025 |
| `RegistrationCompleted` | Academic Registration | C025 |

These event names and publisher ownership are taken from the locked L2-C03 event architecture. No new event or publisher is introduced.

## Cross-Domain Collaboration

Participating domains exchange business facts or consume governed registration outcomes without modifying the Academic Registration Aggregate directly.

---

## 15. Business Authorization, Approval and Audit Obligations

Approval is configuration-driven in L2-C03. Possible approvers named by the source include:

- Faculty Advisor;
- Programme Coordinator;
- Department Office;
- Registrar; and
- Academic Administration.

The exact approval path remains institutionally configured.

Every workflow execution and transition is auditable. Required business evidence includes the actor, timestamp, prior/new state, business action, outcome and applicable approval/exception evidence.

Administrative overrides and exceptional registration paths must be authorized and auditable.

No technical authorization model is introduced.

---

## 16. Completion Criteria and Postconditions

## Completion Criteria

The scenario is complete only when its frozen business outcome has been reached.

### C021
- eligible Student;
- Academic Registration created;
- permitted offerings selected;
- registration validated;
- required approval completed;
- `StudentRegistrationApproved` published where approval completes.

### C022
- permitted modification/add-drop completed;
- all required validations re-executed;
- resulting registration remains valid and auditable.

### C023
- cancellation condition valid;
- cancellation reason recorded;
- registration cancelled;
- `StudentRegistrationCancelled` published.

### C024
- permitted repeat/backlog offering selected;
- academic registration rules satisfied;
- required approval completed;
- resulting registration remains available for finalization.

### C025
- registration approved;
- validation complete;
- freeze condition reached;
- registration frozen;
- `RegistrationFrozen` and applicable completion/publication event(s) occur;
- finalized registration information is available to downstream consumers.

## Postconditions

The Academic Registration remains the authoritative registration record. Downstream domains consume finalized business facts but do not acquire ownership of the registration lifecycle.

---

## 17. Open Issues, Assumptions and Deferred Questions

## Open Issues

The L2 source defines possible approvers but leaves exact institution-specific approval routing configuration-driven. No single universal approver is invented.

## Scenario-Specific Dependency Notes

Academic Calendar/registration window; Student Lifecycle for Student eligibility; Academic Structure/Curriculum/Course Catalogue for offerings; downstream Academic Delivery consumes approved/frozen registration outcomes.

The dependency direction is deliberately conservative:

- upstream domains provide authoritative information;
- Academic Registration owns Academic Registration;
- downstream domains consume finalized registration outcomes;
- participating domains do not modify one another's aggregates directly.

## Assumptions

1. Locked L2-C03 remains authoritative for Academic Registration workflow/rule/state/event semantics.
2. Participating-domain references do not transfer ownership.
3. Institution-specific configuration does not create a new canonical lifecycle state or event.
4. Any unresolved upstream business rule is recorded rather than invented.

## Deferred Questions

Any detailed contract, exact approval routing, or domain-specific prerequisite not explicitly defined by locked upstream architecture is deferred to L4/change control.

---
