# L3-C-006 — Admission Offer Expiry

**Architecture Level:** L3 — Business Scenario Architecture  
**Scenario ID:** `L3-C-006`  
**Scenario Title:** Admission Offer Expiry  
**Primary Owner:** Admissions  
**Participating Domains:** C  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad  
**Version:** 1.0  
**Last Updated:** 2026-08-20  
**Applicable L2 Workflow:** Admissions / L2-C01  
**Canonical Business Record:** Admission Record  
**Canonical Outcome:** Expired  
**Canonical Publisher:** Admissions

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-C-006` |
| Title | Admission Offer Expiry |
| Primary Owner | Admissions |
| Participating Domains | C |
| Scenario Type | Simple |
| Priority | P1 |
| Source Phase 2 Row | `P2W-C-1055` |
| Source Phase 1 Candidate | `CAN-L3-C-006` |
| Derivation | KEEP |
| Canonical Owner | Admissions |
| Canonical Business Record | Admission Record |
| Canonical Outcome | `Expired` |
| Status | Draft |

The frozen Section C canonical register identifies C-006 as **Admission Offer Expiry**, owned by **Admissions**, participating domain **C**, Type **Simple**, Priority **P1**, with derivation **KEEP**. fileciteturn80file0L2-L2

### Source-resolution note

The retrieved L2-C01 material supplied for this authoring pass contains the complete Readmission workflow and surrounding Admissions workflows, but the specific **Admission Offer Expiry** workflow text was not present in the retrieved L2 excerpts.

Accordingly, this document preserves the frozen C-006 identity but does **not** invent an L2 workflow, state transition, business event, actor authority, deadline rule, or publisher that the available authoritative source does not explicitly provide.

---

## 2. Scenario Purpose

The purpose of this scenario is to govern the institutional conclusion of an Admission Offer when the offer reaches its authoritative expiry condition without being completed within the applicable validity period.

The scenario is intended to establish one complete business outcome:

> **An Admission Offer reaches its authoritative expiry condition and the Admission Record reaches the canonical expiry outcome.**

The precise validity period, expiry trigger, lifecycle state, transition, notice requirements, authority, and business event must come from the authoritative Admissions L2 architecture or an applicable locked upstream rule.

No local L3 expiry rule is created here.

---

## 3. Business Outcome

The intended canonical institutional outcome is:

**Admission Offer Expired.**

For C-006, the business outcome is complete only when the authoritative Admissions lifecycle establishes that the offer has expired and the Admission Record reflects that canonical outcome.

The outcome must:

1. identify the affected Admission Record;
2. establish that the authoritative offer-validity condition has been reached;
3. apply the upstream Admission lifecycle transition;
4. preserve the Admission's institutional history;
5. preserve any required business evidence; and
6. publish the authoritative expiry business event if one is defined upstream.

### Critical source constraint

The available authoritative source material does **not** establish, in the retrieved material:

- the exact canonical state name for expiry;
- the exact Admission state from which expiry occurs;
- the exact expiry trigger;
- the exact expiry deadline rule;
- the canonical expiry event;
- the canonical expiry-event publisher;
- an exact expiry actor/role; or
- the downstream consumers of the expiry event.

Therefore these items are intentionally not invented.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an Admission Offer exists and the authoritative offer-validity condition reaches its defined expiry point without a completed acceptance/confirmation outcome.

The exact trigger and validity rule remain upstream-owned and unresolved from the currently retrieved source.

### 4.2 End Boundary

The scenario ends when the authoritative Admissions lifecycle records the Admission Offer as expired and all required business history/evidence for that outcome has been preserved.

If an authoritative downstream event exists, publication of that event is part of completion.

### 4.3 In Scope

- Identification of an Admission Offer approaching or reaching its authoritative expiry condition.
- Application of the upstream offer-validity rule.
- Determination that the offer has expired.
- Application of the canonical Admission lifecycle transition.
- Preservation of Admission history.
- Recording of the expiry business outcome.
- Publication of the canonical expiry event where defined by authoritative architecture.

### 4.4 Out of Scope

- Admission approval.
- Admission confirmation.
- Admission cancellation.
- Admission withdrawal.
- Admission revocation.
- Readmission.
- Student creation.
- Financial settlement.
- Refund processing.
- Academic registration.
- Student Lifecycle processing after a separate downstream outcome.
- Technical scheduling.
- Technical timers.
- APIs.
- Databases.
- Services.
- Queues.
- Retry mechanisms.
- Rollback.
- Infrastructure.
- UI/UX.

### 4.5 Boundary Rule

C-006 must not be used to represent Cancellation, Withdrawal, or Revocation.

Those are separate canonical Admission outcomes and must retain their own lifecycle semantics.

---

## 5. Ownership and Participating Domains

### 5.1 Canonical Ownership

The frozen C-006 register assigns **Admissions** as the Primary Owner.

Accordingly, Admissions owns:

- the Admission Record;
- the Admission Offer lifecycle;
- the business determination that the offer has expired;
- the canonical expiry outcome; and
- any canonical expiry event that the authoritative L2 architecture assigns to Admissions.

### 5.2 Participating Domain

The frozen register identifies **C** as the participating domain.

No additional participating domain is added because the available authoritative material does not establish one for C-006.

### 5.3 Ownership Boundary

A future downstream consumer of an authoritative expiry event would retain its own business ownership.

Event consumption must not transfer ownership of the Admission Record away from Admissions.

### 5.4 No Ownership Invention

The available source material establishes ownership at the scenario/register level but does not establish a specific cross-domain dependency for C-006.

Therefore no cross-domain ownership is invented.

---

## 6. Business Actors, Roles and Responsibilities

### 6.1 Admissions

Admissions is the canonical business owner of the Admission Record and the Admission lifecycle.

Admissions is responsible for applying the authoritative expiry rule and completing the corresponding lifecycle outcome.

### 6.2 Applicant

The Applicant is associated with the Admission Offer but no specific Applicant action for expiry is asserted by the available authoritative source.

Therefore this document does not invent an Applicant notification, acknowledgement, or decision requirement.

### 6.3 Institutional Authority

No specific human authority/role for Admission Offer Expiry is established in the retrieved authoritative material.

Therefore no role such as Admissions Officer, Registrar, Committee, or other named authority is invented.

### 6.4 Business Responsibility Boundary

The exact actors and authorities for expiry remain an upstream architecture/policy question if they are required for the completed L3 definition.

---

## 7. Preconditions and Entry Criteria

The following conditions are supported at the business level without introducing unsupported implementation detail:

1. An Admission Record exists.
2. An Admission Offer exists within that Admission lifecycle.
3. The offer remains subject to an authoritative validity condition.
4. The authoritative expiry condition has been reached.
5. The Admission has not already reached a contradictory terminal outcome such as Cancellation, Withdrawal, or Revocation.

### 7.1 Unsupported Local Preconditions

The following are **not** asserted because the available authoritative source does not define them for C-006:

- a specific number of days;
- a specific calendar deadline;
- a specific time of day;
- a technical timer;
- a particular notification requirement;
- a particular acceptance-state prerequisite.

Those values must be taken from the authoritative upstream source.

---

## 8. Trigger

The business trigger is:

> **The authoritative validity period or expiry condition for an Admission Offer is reached before the offer completes its applicable acceptance/confirmation outcome.**

The exact trigger mechanism is deliberately left unresolved because the retrieved authoritative L2 material does not provide the C-006 expiry trigger.

No technical scheduler, timer, job, or event mechanism is introduced.

---

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Responsibility | Business Result |
|---|---|---|---|
| 1 | An Admission Offer exists within an Admission Record. | Admissions | A candidate offer exists for expiry evaluation. |
| 2 | The authoritative offer-validity condition is evaluated. | Admissions | The offer's validity is determined under upstream policy. |
| 3 | The authoritative expiry condition is found to have been reached. | Admissions | The offer is eligible for expiry. |
| 4 | Admissions applies the canonical expiry lifecycle outcome defined upstream. | Admissions | The Admission Offer reaches its expiry state/outcome. |
| 5 | The expiry outcome is preserved as Admission history. | Admissions | Institutional history remains intact. |
| 6 | Required business evidence is recorded where mandated upstream. | Admissions | The expiry remains traceable. |
| 7 | The canonical expiry business event is published if and only if one is defined by authoritative architecture. | Admissions | Applicable downstream consumers can recognize the completed expiry fact. |
| 8 | The C-006 scenario ends. | Admissions | The Admission Offer expiry outcome is complete. |

### 9.1 Main Flow Boundary

The scenario stops at the completed Admission Offer Expiry outcome.

Any subsequent readmission, new application, financial consequence, or downstream business response belongs to its own canonical process.

---

## 10. Alternate Business Flows

### 10.1 Offer Accepted Before Expiry

If the Applicant completes the authoritative acceptance/confirmation outcome before expiry, C-006 does not complete.

The Admission continues through the applicable Admission lifecycle.

No expiry outcome is asserted.

### 10.2 Offer Already Expired

If the Admission Offer has already reached the authoritative expiry state, the business result is the existing expiry outcome.

The scenario does not create a second lifecycle transition.

### 10.3 Offer Already Cancelled

If the Admission Record has already been cancelled, C-006 does not replace Cancellation with Expiry.

The existing canonical terminal outcome remains authoritative.

### 10.4 Offer Already Withdrawn

If the Admission Record has already been withdrawn, C-006 does not replace Withdrawal with Expiry.

The existing canonical terminal outcome remains authoritative.

### 10.5 Offer Already Revoked

If the Admission Record has already been revoked, C-006 does not replace Revocation with Expiry.

The existing canonical terminal outcome remains authoritative.

### 10.6 Expiry Condition Not Yet Reached

If the authoritative expiry condition has not been reached, the Admission Offer remains governed by its existing lifecycle.

No premature expiry is established.

---

## 11. Failure, Interruption and Business Recovery

### 11.1 Expiry Eligibility Not Established

**Condition:** The authoritative source does not establish that the offer has expired.

**Business Result:** The Admission Offer does not transition to an expiry outcome.

**Business Handling:** Continue under the existing Admission lifecycle.

### 11.2 Conflicting Terminal Outcome

**Condition:** The Admission has already reached Cancellation, Withdrawal, or Revocation.

**Business Result:** C-006 does not overwrite the existing terminal outcome.

**Business Handling:** Preserve the authoritative existing lifecycle state.

### 11.3 Missing Upstream Expiry Rule

**Condition:** The exact validity/expiry rule is unavailable.

**Business Result:** L3 cannot legitimately determine expiry.

**Business Handling:** Escalate to upstream architecture/policy resolution.

### 11.4 Missing Canonical Expiry State

**Condition:** The exact lifecycle state is not defined by the authoritative source.

**Business Result:** L3 cannot invent the state.

**Business Handling:** Resolve the upstream Admission lifecycle before approving C-006.

### 11.5 Missing Canonical Event

**Condition:** No authoritative expiry event is defined.

**Business Result:** L3 must not invent an event name or publisher.

**Business Handling:** Resolve the event contract at the authoritative upstream architecture level.

### 11.6 Technical Recovery Exclusion

The following are explicitly outside L3:

- technical retries;
- queue handling;
- timer implementation;
- infrastructure failures;
- database rollback;
- service exceptions;
- API failures.

---

## 12. Business Rules and Decision Constraints

### 12.1 Source-Supported Constraints

The following architectural constraints apply to the scenario generally:

- Admission ownership remains with Admissions.
- Admission history must remain institutionally preserved.
- C-006 must remain distinct from Cancellation, Withdrawal, and Revocation.
- L3 must apply upstream lifecycle rules rather than create local lifecycle rules.

### 12.2 C-006-Specific Rules Requiring Upstream Evidence

The following must be defined by authoritative source before C-006 can be treated as fully source-complete:

| Required Rule | Current Source Status |
|---|---|
| Offer validity period | Not established in retrieved L2 source |
| Expiry deadline | Not established |
| Eligible source state | Not established |
| Canonical expiry state | Not established |
| Expiry transition | Not established |
| Expiry event | Not established |
| Event publisher | Not established |
| Applicant notification obligation | Not established |
| Administrative override | Not established |

### 12.3 No Local Rules

This document intentionally does not create local deadline, grace-period, notification, override, or acceptance rules.

---

## 13. State and Lifecycle Impact

### 13.1 Canonical Owner

**Admission Record — Admissions**

### 13.2 Intended Outcome

The scenario title and canonical register establish **Admission Offer Expiry** as the business outcome.

However, the retrieved authoritative L2 source does not expose the exact canonical lifecycle state or transition for this outcome.

### 13.3 Required Lifecycle Evidence

Before C-006 can become Architecture Approved, the authoritative source must establish:

```text
<canonical pre-expiry Admission state>
        ↓
<canonical expiry state>
```

The placeholders above are deliberately not converted into invented state names.

### 13.4 Lifecycle Integrity

C-006 must not:

- reopen a terminal Admission;
- overwrite Cancellation;
- overwrite Withdrawal;
- overwrite Revocation;
- transfer Admission ownership.

### 13.5 Readmission Boundary

If an expired Admission is later eligible for Readmission, that is governed by the Readmission workflow.

The L2 Readmission architecture states that historical admissions remain preserved and that readmission strategy is institutionally governed. fileciteturn82file0

C-006 does not itself perform Readmission.

---

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Event

The canonical C-006 expiry event is **not established in the retrieved authoritative material**.

Therefore no event name is invented.

### 14.2 Publisher

The canonical publisher of an expiry event is likewise not established in the retrieved source.

Admissions remains the scenario owner, but ownership alone is not sufficient evidence to invent a canonical event publisher.

### 14.3 Cross-Domain Consumers

No authoritative C-006 consumer is established in the retrieved material.

Therefore no downstream consumer is invented.

### 14.4 Business Fact Boundary

If the authoritative architecture later defines an expiry event, that event must represent the completed business fact of Admission Offer Expiry.

The event must remain distinct from:

- expiry evaluation;
- expiry eligibility determination;
- notification;
- downstream response.

### 14.5 No Technical Contracts

No API, payload, message schema, queue, transport, service, or implementation contract is defined.

---

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authorization

The retrieved authoritative source does not establish a C-006-specific human authority.

No unsupported authority is invented.

### 15.2 Approval

The retrieved source does not establish a C-006-specific approval requirement.

No approval step is invented.

### 15.3 Audit

Admission history must remain institutionally preserved as an architectural principle.

However, the exact C-006 audit fields, if any, are not established in the retrieved C-006 source material.

Therefore no C-006-specific audit schema is invented.

### 15.4 Evidence

If the upstream expiry rule requires evidence of the offer's validity period or expiry condition, that evidence must be defined upstream.

L3 will apply it once authoritative.

### 15.5 Separation of Duties

No C-006-specific separation-of-duty rule is established.

No local rule is introduced.

---

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

C-006 can be considered business-complete when the authoritative Admissions architecture provides sufficient evidence to establish:

1. the Admission Offer exists;
2. the offer is subject to a defined validity condition;
3. that validity condition has expired;
4. the canonical Admission lifecycle transition is applied;
5. the resulting expiry state/outcome is established;
6. Admission history is preserved;
7. required audit/evidence obligations are fulfilled; and
8. the canonical expiry event is published if one is defined.

### 16.2 Postconditions

The intended postconditions are:

| Postcondition | Expected Result |
|---|---|
| Admission Offer | Expired |
| Admission owner | Admissions |
| Admission history | Preserved |
| Cancellation | Not substituted |
| Withdrawal | Not substituted |
| Revocation | Not substituted |
| Readmission | Separate downstream/canonical process |
| Audit | According to authoritative policy |
| Event | According to authoritative L2 definition |

### 16.3 End Boundary

The scenario ends when the Admission Offer expiry outcome is complete.

No downstream process is silently absorbed into C-006.

---

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Blocking Issue — Missing Authoritative C-006 L2 Semantics

The retrieved L2-C01 material available for this authoring pass does not contain the specific Admission Offer Expiry workflow.

The available L2 material contains the adjacent:

- Admission Withdrawal;
- Admission Revocation; and
- Readmission

workflows, but the specific C-006 expiry workflow was not present in the retrieved source sections. fileciteturn83file0turn82file0

**Architecture impact:** Material and blocking.

The missing workflow prevents authoritative determination of:

- exact expiry trigger;
- offer-validity rule;
- expiry deadline;
- eligible Admission state;
- canonical expiry state;
- lifecycle transition;
- expiry event;
- event publisher;
- audit obligations;
- authority/approval;
- exception flows;
- downstream consumers.

**Required action:** Retrieve the authoritative L2 workflow/rules for Admission Offer Expiry before approving C-006.

---

### 17.2 Assumptions

No assumption in this document overrides locked architecture.

The only bounded working assumptions are:

1. The frozen Stage 3 identity for C-006 is authoritative.
2. Admissions remains the canonical owner.
3. Admission Offer Expiry is distinct from Cancellation, Withdrawal, and Revocation.
4. Any expiry lifecycle state and transition must be taken from upstream architecture.
5. Any expiry event and publisher must be taken from upstream architecture.
6. No local deadline or policy is introduced.

---

### 17.3 Deferred Questions

| Question | Target Authority | Impact |
|---|---|---|
| What is the exact validity period of an Admission Offer? | Locked Admissions L2 / policy | Blocking |
| What exact state is an offer in before expiry? | Admissions L2 | Blocking |
| What exact state represents expiry? | Admissions L2 | Blocking |
| What exact lifecycle transition represents expiry? | Admissions L2 | Blocking |
| What event represents completed expiry? | Admissions L2 | Blocking |
| Who publishes the expiry event? | Admissions L2 | Blocking |
| What actors/authority are involved? | Admissions policy/L2 | Blocking if required |
| What audit evidence is mandatory? | Admissions governance/L2 | Blocking for final approval |
| Are applicants notified, and under what rule? | Admissions policy/L2 | Deferred |
| Can an expired offer be restored directly? | Admissions policy/L2 | Deferred |
| What is the relationship between expiry and later Readmission? | Admissions L2 | Deferred |

---

# Validation and Authoring Classification

**Classification: BLOCKED — Draft**

## Identity Verification

- Final ID: `L3-C-006`
- Title: `Admission Offer Expiry`
- Primary Owner: Admissions
- Participating Domain: C
- Scenario Type: Simple
- Priority: P1

These frozen values are confirmed by the Section C canonical register. fileciteturn80file0L2-L2

## Source Verification

The available L2-C01 material was inspected around the Admissions workflow definitions.

It provides authoritative material for adjacent workflows including Withdrawal, Revocation, and Readmission, but the specific C-006 Admission Offer Expiry workflow was not present in the retrieved material. fileciteturn83file0turn82file0

## Why BLOCKED

The Phase 5 governance explicitly prohibits inventing:

- lifecycle states;
- lifecycle transitions;
- business events;
- event publishers;
- business rules;
- actor authority;
- dependencies; and
- terminology.

Because the exact C-006 upstream semantics are unavailable in the retrieved authoritative source, supplying those details would violate repository correctness.

Therefore this is a **complete 17-section authoring artifact**, but it is correctly classified:

> **BLOCKED — Draft**

The blocking issue is explicitly preserved in Section 17 rather than silently filled with assumptions.

## Implementation Leakage Check

No APIs, databases, services, Business Data Structures, frameworks, queues, technical retries, rollback, infrastructure, deployment, or UI design are introduced.

## Repository Safety

No GitHub repository files were created, modified, or committed.
