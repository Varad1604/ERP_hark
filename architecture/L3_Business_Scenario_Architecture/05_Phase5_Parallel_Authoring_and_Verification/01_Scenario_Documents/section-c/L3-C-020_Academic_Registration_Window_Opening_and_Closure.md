# Academic Registration Window Opening and Closure

**Scenario ID:** L3-C-020  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Registration  
**Participating Domains:** B, C, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Derivation:** KEEP  
**Status:** Locked  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  
**Exact Filename:** `L3-C-020_Academic_Registration_Window_Opening_and_Closure.md`

---

| Attribute | Value |
|---|---|
| Final L3 ID | `L3-C-020` |
| Canonical Scenario Title | **Academic Registration Window Opening and Closure** |
| Primary Owner | **Academic Registration** |
| Participating Domains | **B, C, D** |
| Scenario Type | **Cross-domain** |
| Priority | **P0** |
| Derivation | **KEEP** |
| Assigned Author | **Varad** |
| Exact Phase 5 Filename | `L3-C-020_Academic_Registration_Window_Opening_and_Closure.md` |
| Status | **Draft** |

The frozen Section C canonical register identifies C020 as **Academic Registration Window Opening and Closure**, with Academic Registration as Primary Owner, participating domains B, C and D, Cross-domain type, P0 priority and KEEP derivation.

The Final L3 register establishes direct positional alignment for Section C, so C020 is `L3-C-020`.

Phase 5 allocates the entire Section C range `L3-C-001`–`L3-C-075` to Varad. The exact filename is mechanically derived from the frozen title and must not be renamed.

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-C-020 |
| Canonical Title | Academic Registration Window Opening and Closure |
| Primary Owner | Academic Registration |
| Participating Domains | B, C, D |
| Scenario Type | Cross-domain |
| Priority | P0 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the institutional opening and closure of an academic registration window.

The scenario establishes when academic registration becomes available to eligible Students and when the registration period is no longer open for ordinary registration activity, using the approved academic calendar, registration configuration and institutional registration policies.

The scenario provides the business boundary required by downstream academic registration activities. It does not perform Student eligibility determination, individual registration creation, course selection, registration approval or registration freeze; those are separate workflows within the Academic Registration capability.

---

## 3. Business Outcome

The institution has established a governed academic registration window for a defined academic period:

1. the registration opening conditions have been validated;
2. the registration window has become available for eligible Students; and
3. the registration period subsequently reaches its governed closure condition, after which new ordinary registration requests are not accepted.

The registration window therefore provides a clear institutional availability boundary for academic registration.

The authoritative L2 architecture explicitly defines the opening outcome as **Registration Window Opened** and **Registration Availability Published** and defines closed registration windows as rejecting new registration requests. It does **not** define a separate canonical `RegistrationWindowClosed` event; that gap is recorded in Section 17 rather than invented.

---

## 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when the configured registration opening condition is reached or an authorized administrative opening is initiated.

The locked L2-C03 workflow identifies these triggers for Registration Window Initialization:

- Registration schedule reaches configured start date.
- Administrative opening of registration.

It also defines the detailed workflow trigger as:

- Scheduled registration opening.
- Administrative initiation.

## 4.2 End Boundary

The business scenario covers the registration window through its opening and governed closure condition.

The opening portion completes when:

- the academic calendar is validated;
- registration configuration is validated;
- the registration window is activated;
- eligible Students may begin registration; and
- `RegistrationWindowOpened` has been published.

The closure portion completes when the configured registration period is no longer open for ordinary registration. The L2 source explicitly establishes that a closed registration window rejects new registration requests, but does not provide a separate canonical closure event or detailed closure workflow. That distinction is preserved rather than invented.

## 4.3 In Scope

- Academic session/calendar validation relevant to opening.
- Validation of approved registration configuration.
- Confirmation that course offerings are published/available before opening.
- Activation of the academic registration window.
- Publication of registration availability.
- Publication of the canonical `RegistrationWindowOpened` event.
- Availability of academic registration to eligible Students.
- Enforcement of the closed-window rule.
- Recognition of the registration-window closure condition.
- Business-level coordination with Student Lifecycle and Academic Delivery where required by registration dependencies.

## 4.4 Out of Scope

- Student eligibility validation.
- Academic Registration Aggregate creation.
- Course/subject/elective selection.
- Registration validation for an individual Student.
- Registration approval.
- Registration modification.
- Registration cancellation.
- Registration freeze.
- Registration publication.
- Curriculum definition.
- Course definition.
- Student lifecycle ownership.
- Academic delivery execution.
- APIs, database design, classes, Business Data Structures, technical services or code.
- UI design.
- Technical IAM/RBAC.
- Deployment/infrastructure.
- Technical timers, schedulers, queues or retry mechanics.

The L2-C03 architecture explicitly separates Registration Window Initialization from Student Eligibility Validation, Registration Creation, Offering Selection, Registration Validation, Approval, Modification, Cancellation, Freeze and Publication.

---

## 5. Ownership and Participating Domains

| Domain | Participation | Ownership |
|---|---|---|
| **Academic Registration** | Owns the academic registration window and its business availability boundary. | **Canonical owner** |
| **B — Institution & Structure** | Participates through the institutional/academic structure and calendar/configuration dependencies required for the registration period. | Does not own the Registration Window. |
| **C — Student Lifecycle** | Provides Student lifecycle information required by downstream eligibility validation and consumes/participates in registration-related business outcomes as defined by its own ownership. | Does not own the Registration Window. |
| **D — Academic Delivery** | Participates downstream once registrations are approved/frozen/published and consumes finalized registration information as defined by the registration architecture. | Does not own the Registration Window. |

The L2-C03 architecture states that Academic Registration is the sole canonical owner of academic registration transactions and that it references, rather than owns, Student, Programme, Curriculum, Course, Academic Session and other upstream business objects.

The L2 architecture also defines Registration Window Initialization as depending on the Academic Calendar, while later registration publication feeds Academic Delivery and other downstream consumers.

Participation does not transfer ownership.

---

## 6. Business Actors, Roles and Responsibilities

The authoritative L2 workflow identifies the following business contexts for registration-window initialization:

| Business Actor / Context | Responsibility | Authority Boundary |
|---|---|---|
| **Academic Registration Domain** | Govern the Registration Window, validate opening conditions and make registration available. | Canonical owner. |
| **Academic Administration / authorised administrative participant** | Initiate an administrative opening where institutional policy permits. | May initiate the governed opening; does not independently own the Registration Window lifecycle. |
| **Academic Calendar authority / owning domain** | Provides the authoritative academic calendar condition on which registration opening depends. | Owns the calendar, not the Registration Window. |
| **Eligible Student** | Becomes able to perform academic registration while the window is open and all individual eligibility rules are satisfied. | Does not control institutional window opening/closure. |

The detailed C03 workflow defines the opening triggers as scheduled registration opening and administrative initiation. It also defines the broader registration domain as operating under configured academic calendars and policies.

No additional named role is invented.

---

## 7. Preconditions and Entry Criteria

Before the registration window may be opened:

- the academic session is active;
- registration configuration is approved;
- course offerings are published;
- the academic calendar is released;
- registration policies are configured before execution;
- the applicable academic period has reached the configured opening condition, or an authorized administrative opening has been initiated.

The authoritative WS-C03-01 explicitly defines:

- Academic session active.
- Registration configuration approved.
- Course offerings published.
- Academic calendar released.

as preconditions.

For ordinary Student registration after opening, the separate Student Eligibility Validation workflow additionally requires an existing Student and an open registration window.

---

## 8. Trigger

The canonical opening triggers are:

1. **Registration schedule reaches configured start date / scheduled registration opening.**
2. **Administrative opening of registration / administrative initiation**, where authorized.

The L2 architecture does not define a separate named business event as the trigger for opening.

The closure condition is the configured end of the approved registration period. The authoritative rules state that registration is permitted only during approved registration windows and that closed windows reject new registration requests. No separate canonical closure event is defined in the available L2 source.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Owner / Participant | Business Result |
|---|---|---|---|
| 1 | The approved academic registration schedule reaches its configured opening condition, or an authorised administrative opening is initiated. | Academic Registration / authorised administration | Registration opening process begins. |
| 2 | The academic calendar is validated. | Academic Registration against Academic Calendar authority | Opening condition is institutionally valid. |
| 3 | Registration configuration is validated. | Academic Registration | Approved registration configuration is confirmed. |
| 4 | Course offerings required for the registration period are confirmed as published. | Academic Registration / Course Catalogue / Academic Delivery dependencies | Required academic offerings are available for registration. |
| 5 | The registration window is activated. | Academic Registration | The institutional registration availability boundary becomes open. |
| 6 | Registration availability is published. | Academic Registration | Eligible Students can begin the separate registration process. |
| 7 | `RegistrationWindowOpened` is published as the canonical business fact that the window became available. | Academic Registration | Downstream interested domains can respond to the opening. |
| 8 | During the open period, ordinary Student registration is permitted subject to the separate eligibility, academic and policy validations. | Academic Registration | Students may enter the registration lifecycle. |
| 9 | The configured registration end condition is reached. | Academic Registration / governing academic calendar | The registration window ceases to be open for ordinary registration. |
| 10 | New ordinary registration requests are rejected because the registration window is closed. | Academic Registration | Registration availability boundary is closed. |
| 11 | The scenario reaches its closure outcome. | Academic Registration | The institutional registration period is closed for ordinary registration. |

The authoritative workflow defines opening as validating the calendar, loading/validating registration policies/configuration, enabling eligible Student access, and publishing registration availability.

The authoritative business rules explicitly state that registration may occur only during approved registration windows and that closed registration windows reject new registration requests.

---

## 10. Alternate Business Flows

## 10.1 Administrative Opening

| Condition | Alternate Flow | Outcome |
|---|---|---|
| Authorized administration initiates registration before/independent of the scheduled opening condition where institutional policy permits | Validate academic calendar and approved configuration, then activate the registration window. | Registration becomes available under the governed administrative opening. |

The L2 architecture explicitly permits administrative opening as a trigger.

## 10.2 Scheduled Opening

| Condition | Alternate Flow | Outcome |
|---|---|---|
| Registration schedule reaches configured start date | Validate calendar/configuration and activate registration availability. | Registration window opens. |

## 10.3 Late Registration Policy

| Condition | Alternate Flow | Outcome |
|---|---|---|
| Ordinary registration window has closed but institutional policy separately permits late registration | Apply the separately configured late-registration policy and its required authorization. | A permitted exception path may allow registration under the applicable policy. |

The L2 architecture explicitly identifies late registration as an exceptional workflow path and states that late registration follows separately configured institutional policies.

C020 does not invent the detailed late-registration workflow.

## 10.4 Administrative Override

| Condition | Alternate Flow | Outcome |
|---|---|---|
| Registration is outside the approved window and an authorized administrative override is permitted | Apply the configured override authorization and audit requirements. | Registration may proceed only through the authorized exception path. |

The L2 rule states that registration outside approved windows requires authorized administrative override.

---

## 11. Failure, Interruption and Business Recovery

| Business Condition | Consequence | Business Resolution | Outcome |
|---|---|---|---|
| Academic calendar is not released/valid | Window cannot be opened. | Resolve the governing academic-calendar condition. | Registration remains unavailable. |
| Registration configuration is not approved | Window cannot be opened. | Complete the required configuration approval. | Registration remains unavailable. |
| Required course offerings are not published | Window cannot be opened under the defined opening preconditions. | Publish/resolve required offerings according to their owning architecture. | Registration remains unavailable. |
| Academic session is not active | Window cannot be opened. | Resolve the academic-session condition. | Registration remains unavailable. |
| Registration opening condition has not been reached | Scheduled opening does not occur. | Wait for the governing business opening condition; no technical timing mechanism is defined here. | Registration remains unavailable. |
| Registration window is closed | New ordinary registration requests are rejected. | Use an applicable late-registration or authorized exception policy if one exists. | Ordinary registration remains closed. |
| Late-registration policy does not permit the request | Registration cannot proceed through the exception path. | Resolve according to institutional policy. | Registration remains unavailable for that request. |
| Administrative override is not authorized | Out-of-window registration cannot proceed. | Obtain required business authorization if policy permits. | Registration remains unavailable for that request. |
| Mandatory registration configuration is missing | Opening cannot complete. | Complete the required configuration. | Registration remains unavailable. |

The L2 workflow states that invalid registration-window conditions prevent registration and that business validation failures require business/user/administrator resolution rather than an invented technical recovery process. C020 therefore defines business recovery only.

Technical retry, queue, timeout, infrastructure failure and rollback behaviour are deliberately excluded from this L3 scenario.

---

## 12. Business Rules and Decision Constraints

| Rule | Business Effect | Canonical Owner / Source |
|---|---|---|
| Registration shall only occur during approved registration windows. | Ordinary registration is unavailable outside the approved window. | Academic Registration — BR-C03-007. |
| Registration windows shall be configuration driven. | Window boundaries are governed by approved institutional configuration. | Academic Registration — BR-C03-008. |
| Late registration follows separately configured institutional policies. | Late registration is an exception path rather than ordinary registration. | Academic Registration — BR-C03-009. |
| Registration outside approved windows requires authorized administrative override. | Out-of-window registration cannot bypass institutional authority. | Academic Registration — BR-C03-010. |
| Closed registration windows reject all new registration requests. | Closure creates a hard ordinary-registration boundary. | Academic Registration — BR-C03-011. |
| Academic session must be active before opening. | An inactive academic session cannot host an open registration window. | WS-C03-01. |
| Registration configuration must be approved. | Unapproved configuration cannot establish the registration period. | WS-C03-01. |
| Course offerings must be published. | Required offerings must exist before opening. | WS-C03-01. |
| Academic calendar must be released. | Opening depends on an authoritative calendar condition. | WS-C03-01. |
| Registration policies are configured before execution. | Window opening and subsequent registration are policy governed. | Domain assumptions. |
| Events represent completed business facts. | `RegistrationWindowOpened` is published only after successful opening. | Event architecture. |
| Domain events are immutable. | Published opening fact is historically stable. | Event principles. |

The authoritative C03 architecture explicitly identifies these registration-window rules and the Registration Window Initialization workflow.

---

## 13. State and Lifecycle Impact

## 13.1 Registration Window Lifecycle

The available locked L2 architecture establishes the following business condition sequence:

```text
Registration Window Closed / Unavailable
                │
                ▼
        Opening Conditions Validated
                │
                ▼
          Window Open
                │
                ▼
       Registration Available
                │
                ▼
        Window Closure Condition
                │
                ▼
          Window Closed
```

The L2 workflow explicitly defines **Window Open** as the registration availability condition and the business rules define the closed condition as rejecting new registration requests.

## 13.2 Canonical Event

`RegistrationWindowOpened` is the only explicit registration-window event defined in the locked L2 event catalogue.

The event producer mapping explicitly associates **Open Registration Window → `RegistrationWindowOpened`**.

## 13.3 Closure State

The L2 architecture clearly establishes the business effect of a closed registration window, but the available source does not define:

- a separate `RegistrationWindowClosed` event;
- a dedicated closure workflow specification;
- a named closure transition operation; or
- a closure-specific event publisher beyond the Academic Registration domain's general ownership.

C020 therefore does not invent any of these.

---

## 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Event

| Event | Publisher | Business Meaning |
|---|---|---|
| `RegistrationWindowOpened` | **Academic Registration** | The academic registration window has become available. |

The locked event catalogue identifies `RegistrationWindowOpened` as `EVT-C03-001`, describes it as the registration window becoming available, and identifies **Open Registration Window** as its producer.

## 14.2 Closure Event

**No canonical `RegistrationWindowClosed` event is defined in the available locked L2-C03 event catalogue.**

C020 therefore does not create or rename one.

The closure business fact is expressed through the authoritative business rule that a closed registration window rejects new registration requests.

## 14.3 Cross-Domain Collaboration

| Domain | Collaboration | Ownership |
|---|---|---|
| **Academic Calendar / Institution & Structure** | Provides the academic calendar condition used to determine the registration schedule. | Calendar/structure domain retains ownership of its own concepts. |
| **Student Lifecycle (C)** | Provides Student lifecycle information consumed by the separate eligibility-validation workflow. | Student Lifecycle retains Student ownership. |
| **Academic Delivery (D)** | Participates in downstream academic processing after registration is approved/frozen/published. | Academic Delivery retains its own academic execution ownership. |

The registration architecture states that Registration Window Initialization depends on Academic Calendar, Eligibility Validation depends on Student Lifecycle, and later Registration Publication feeds Academic Delivery and other downstream domains.

Events remain business facts; cross-domain business dependencies are not redefined as technical APIs.

---

## 15. Business Authorization, Approval and Audit Obligations

## 15.1 Opening Authority

The authoritative L2 source supports:

- scheduled opening according to configured registration timing; and
- administrative opening.

Where administrative initiation is used, the initiation must remain within the institution's authorized business governance.

No specific administrative role is invented because the locked workflow does not provide a canonical named role for the opening decision itself.

## 15.2 Closure Authority

The available L2 source establishes closure through the configured registration period and the rule that closed windows reject new requests, but does not specify a distinct named closure authority.

C020 therefore does not invent one.

## 15.3 Exception Authorization

The L2 rules explicitly require authorized administrative override for registration outside approved windows. Late registration also follows separately configured institutional policy.

## 15.4 Audit Obligations

The Academic Registration architecture requires auditability for registration operations and states that every workflow transition shall generate an audit record.

For C020, the business evidence includes:

- opening condition/configuration;
- academic-calendar validation;
- registration configuration approval;
- opening outcome;
- `RegistrationWindowOpened` event;
- administrative opening evidence where applicable;
- exception/override justification where applicable; and
- closure-related business evidence where required by institutional policy.

The L2 architecture states that every workflow transition shall generate an audit record and that audit records are immutable.

No technical audit schema is defined here.

---

## 16. Completion Criteria and Postconditions

## 16.1 Opening Completion

The opening portion is complete when:

- the academic session is active;
- the academic calendar is released and validated;
- registration configuration is approved;
- required course offerings are published;
- the registration window is activated;
- registration availability is published;
- `RegistrationWindowOpened` is published; and
- eligible Students can enter the separate academic-registration workflow.

These conditions are grounded in WS-C03-01.

## 16.2 Closure Completion

The closure portion is complete when:

- the approved registration period has reached its governing end condition; and
- new ordinary registration requests are no longer accepted because the registration window is closed.

The available L2 source defines this closure business effect through BR-C03-011 but does not define a dedicated closure event.

## 16.3 Postconditions

After successful opening:

- registration is available within the approved window;
- the opening business fact has been published as `RegistrationWindowOpened`;
- downstream consumers may respond to the opening according to their own architecture;
- individual Student registration remains subject to eligibility and registration rules.

After closure:

- ordinary new registration requests are rejected;
- late registration/administrative override is available only where separately authorized by institutional policy;
- the registration window is no longer an ordinary registration availability boundary.

---

## 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| **OI-C020-01 — No Canonical Registration Window Closure Event** | The scenario title explicitly includes closure, and the L2 rules define closed-window behaviour, but the locked event catalogue contains `RegistrationWindowOpened` and no `RegistrationWindowClosed`. Creating one would invent a canonical event. | Confirm through architecture change control whether closure is intentionally represented only as a business state/availability condition or whether a canonical closure event must be added upstream. |
| **OI-C020-02 — No Dedicated Closure Workflow Specification** | WS-C03-01 explicitly specifies opening, while no equally explicit closure workflow was found in the locked C03 source. | Confirm the authoritative closure workflow/state transition before L4 specifies detailed closure behaviour. |
| **OI-C020-03 — Exact Closure Authority Not Named** | The L2 source establishes scheduled/administrative opening and the closed-window rule but does not name a specific closure decision authority. | Confirm the authoritative academic-calendar/policy ownership if detailed authorization is required. |
| **OI-C020-04 — Exact B-D Business Contract for Window Availability Not Enumerated** | Participating domains B and D are part of the frozen C020 semantics, while C03 provides dependency/consumer context rather than a C020-specific business-contract definition. | Resolve the exact cross-domain contract/event consumption relationships at L4 using the authoritative dependency architecture. |

These issues are recorded because the L3 scenario cannot legitimately invent missing closure semantics.

## 17.2 Assumptions

| Assumption | Basis | Constraint |
|---|---|---|
| Academic Registration owns the Registration Window. | L2-C03 identifies Academic Registration as the sole canonical owner of academic registration transactions and explicitly owns Registration Window Initialization. | No ownership is transferred to B, C or D. |
| The registration window's opening is represented by `RegistrationWindowOpened`. | Locked event catalogue. | No new opening event is created. |
| Closure means ordinary registration is no longer accepted. | BR-C03-011. | No new closure event is inferred. |
| Late registration and administrative override are separate policy-controlled exceptions. | BR-C03-009 and BR-C03-010. | Their detailed workflows are not invented. |
| Student eligibility is a separate downstream/adjacent workflow after the window is open. | WF-C03-02. | C020 does not absorb Student eligibility. |

## 17.3 Deferred Questions

| Question | Deferred To | Reason |
|---|---|---|
| Should closure publish a canonical business event? | Architecture change control / L4 | Current locked L2 does not define one. |
| What exact business transition represents closure? | L4 | Current L2 defines the effect but not a detailed closure state machine. |
| Which exact role authorizes administrative opening? | L4 / governance policy | Current L2 gives the trigger but no canonical named role. |
| Which exact business consumers require window-opening information? | L4 / dependency review | Current L2 provides the event and broad event-consumer architecture but not a C020-specific consumer contract. |
| What institution-specific exception applies after closure? | L4 / configured policy | C03 only establishes late registration and authorized override as policy-controlled exceptions. |

---
