# Approved Candidate to Active Student

**Scenario ID:** L3-C-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Lifecycle — Section C  
**Participating Domains:** A — Platform Foundation; B — Institution & Structure; C — Student Lifecycle; F — Finance  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** L2-C01 — Admissions Workflows; L2-C02 — Student Lifecycle & Activation Workflows; Section A Platform Foundation; Section B Institution & Structure; Section F Finance  
**Referenced By:** Downstream student lifecycle, academic, finance, examination and campus-service scenarios that require an Active Student

## 1. Scenario Metadata

This scenario preserves the frozen Phase 4 identity: canonical title `Approved Candidate to Active Student`, Primary Owner `Student Lifecycle`, participating domain set `A, B, C, F`, Scenario Type `Cross-domain`, and Priority `P0`.

Authoritative sources are the Phase 4 Section C Canonical Register, `L2-C01 — Admissions Workflows`, `L2-C02 — Student Lifecycle & Activation Workflows`, `DEPENDENCY_MAP.md`, and the applicable Section A, B and F architecture governing identity, institutional context and financial clearance.

## 2. Scenario Purpose

Transform an approved admission candidate into an Active Student through the governed Admissions-to-Student-Lifecycle hand-off without transferring ownership of Finance, institutional structure or platform identity concerns.

The scenario spans the remaining admission conditions after approval, successful admission confirmation, canonical Student establishment and Student activation.

## 3. Business Outcome

The approved candidate has completed the required admission conditions, the Admissions responsibility has handed off the confirmed admission to Student Lifecycle, the canonical Student and StudentContext have been established under Student Lifecycle ownership, and the StudentContext is **Active**.

The authoritative `StudentActivated` event is published by Student Lifecycle.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins with an **Admission Approved** candidate whose remaining confirmation conditions must be completed before Student Lifecycle activation can validly occur.

### 4.2 End Boundary

The scenario ends when Student Lifecycle has established the canonical Student/StudentContext and the StudentContext has reached the **Active** lifecycle state, with `StudentActivated` published by Student Lifecycle.

### 4.3 In Scope

- Completion of remaining governed admission conditions after admission approval.
- Finance-owned financial-clearance contribution where required by admission policy.
- Admission confirmation under Admissions ownership.
- Admissions-to-Student-Lifecycle hand-off.
- Canonical Student establishment by Student Lifecycle.
- StudentContext establishment and activation by Student Lifecycle.
- Publication of the canonical Student Lifecycle activation outcome.
- Preservation of institutional, identity and financial ownership boundaries.

### 4.4 Out of Scope

- Applicant CRM, application processing, merit preparation or seat allocation.
- Definition of institutional or academic structure.
- Finance-owned payment execution, ledger or accounting activity.
- Academic registration after activation.
- Academic progression, examination, hostel, transport or library operations.
- Technical authentication, APIs, persistence, queues, event transport or deployment mechanisms.

### 4.5 Related Scenarios

Subsequent Student Lifecycle and Academic Registration scenarios consume the Active Student outcome but do not redefine this activation lifecycle.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario | Canonical Ownership Preserved |
|---|---|---|
| A — Platform Foundation | Provides the platform identity, authorization and shared foundation required by the participating business domains. | Platform foundation concepts remain A-owned. |
| B — Institution & Structure | Provides the authoritative institution and structural context into which the Student relationship is established. | Institution and structure remain B-owned. |
| C — Student Lifecycle / Admissions | Admissions owns the admission lifecycle through confirmation and the governed hand-off; Student Lifecycle owns Student, StudentContext and activation. | Student and StudentContext remain C-owned; admission ownership does not transfer to another section. |
| F — Finance | Provides authoritative financial-clearance facts required by the admission policy. | Finance owns monetary transactions and financial truth; it does not activate the Student. |

The cross-domain scenario does not create shared ownership. Student Lifecycle remains the frozen Primary Owner of the final Active Student outcome.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Authority Boundary |
|---|---|---|
| Approved candidate | Completes the business conditions required for admission confirmation and Student establishment. | Does not authorize institutional lifecycle transitions. |
| Admissions business authority / Admission Officer where applicable under L2-C01 | Governs the remaining admission processing and confirmation within Admissions. | Does not own StudentContext activation. |
| Registrar | Authorizes Student activation under L2-C02. | Activation authority applies to the Student Lifecycle transition. |
| Student Lifecycle | Establishes the canonical Student/StudentContext and performs the activation workflow. | Owns the resulting Student lifecycle state. |
| Finance | Provides authoritative financial-clearance outcome required by the admission workflow. | Does not create or activate StudentContext. |

No new scenario-specific role is invented beyond roles and domain responsibilities supported by the locked L2 architecture.

## 7. Preconditions and Entry Criteria

- The candidate's Admission is in the approved position under L2-C01.
- The candidate is associated with the canonical institutional/person context required by the governed architecture.
- Required admission documentation and verification conditions can be completed under Admissions governance.
- Required Finance-owned financial-clearance fact can be obtained where policy requires it.
- The target institution context is valid and authoritative.
- Student creation and activation can proceed without violating Student Lifecycle invariants, including the single Active StudentContext rule for an Institute.

## 8. Trigger

**An Admission reaches the approved position and enters the governed path toward confirmation, Student creation and activation.**

The cross-domain hand-off into Student Lifecycle occurs when Admissions reaches its authoritative confirmed-admission outcome as defined by L2-C01.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | The approved Admission continues through the remaining governed confirmation conditions. | Admissions | The approved candidate enters the final admission-completion path. |
| 2 | Required admission verification conditions are confirmed according to L2-C01. | Admissions | Admission verification requirements are satisfied. |
| 3 | Required financial-clearance truth is obtained from Finance and applied to the admission decision path without transferring financial ownership. | Finance / Admissions | The financial condition required for confirmation is satisfied. |
| 4 | Admissions confirms the Admission and publishes the authoritative confirmed-admission hand-off fact defined by L2-C01. | Admissions | Student Lifecycle receives a valid confirmed-admission basis for Student establishment. |
| 5 | Student Lifecycle establishes the canonical Student from the confirmed Admission. | Student Lifecycle | The canonical Student exists under C ownership. |
| 6 | Student Lifecycle establishes the StudentContext for the applicable Institute. | Student Lifecycle with B institutional context | The institutional Student relationship exists under Student Lifecycle ownership. |
| 7 | The Registrar-authorized Student Activation workflow validates activation eligibility and transitions the StudentContext to **Active**. | Registrar / Student Lifecycle | StudentContext reaches Active. |
| 8 | Student Lifecycle publishes `StudentActivated`. | Student Lifecycle | The authoritative Active Student business fact becomes available to downstream consumers. |

## 10. Alternate Business Flows

### A1 — Policy-Permitted Financial Clearance Condition

Where institutional policy permits a governed financial-clearance condition other than full payment, Admissions consumes the authoritative Finance outcome allowed by that policy and may proceed toward confirmation if all other conditions are satisfied.

### A2 — Existing Historical Student Identity

Where the person already has historical Student information, Student Lifecycle preserves the canonical Student identity and applies the locked StudentContext rules rather than creating a competing Student identity.

No alternate path may bypass Admissions confirmation or Student Lifecycle activation authority.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required admission verification condition is not satisfied | Admission cannot validly proceed to confirmation. | Resolve the Admissions-owned condition before hand-off. | Admissions |
| Required financial clearance is not satisfied | Admission cannot reach the required confirmation condition where policy requires clearance. | Resolve the Finance-owned financial condition; Admissions consumes the resulting fact. | Finance / Admissions |
| Admission is revoked, cancelled, withdrawn or expires before successful hand-off | The successful activation outcome cannot be reached through this scenario. | Follow the applicable canonical Admissions scenario/lifecycle. | Admissions |
| Student establishment would violate Student Lifecycle identity or context invariants | Canonical Student/StudentContext establishment cannot proceed. | Resolve the governing Student Lifecycle condition without creating duplicate identity or context. | Student Lifecycle |
| Student activation is not authorized or activation conditions are not satisfied | StudentContext does not reach Active. | Remain in the governed pre-activation lifecycle position until the C02 condition is satisfied. | Registrar / Student Lifecycle |

No technical retry, queue, rollback or infrastructure recovery is defined at L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Admissions owns the admission transition and does not own Finance, academic structure or downstream Student lifecycle. | Admissions / Section C | Remaining admission processing stays within C01 boundaries. | L2-C01 |
| Finance owns monetary transactions and provides financial-clearance truth. | Finance | Finance contributes a prerequisite fact but does not activate a Student. | L2-C01 / Finance architecture |
| Student Lifecycle is the canonical owner of the Student Aggregate and StudentContext. | Student Lifecycle | Student establishment and activation remain C02-owned. | L2-C02 |
| A Student shall originate only from a successful Admission. | Student Lifecycle | Student establishment cannot bypass the confirmed Admission hand-off. | L2-C02 SP-002 |
| A Student has one canonical Student Aggregate; historical StudentContexts are allowed but at most one StudentContext may be Active per Institute. | Student Lifecycle | Activation must preserve the single-active-context invariant. | L2-C02 SP-004 |
| Student lifecycle transitions are deterministic and auditable. | Student Lifecycle | Activation follows the canonical lifecycle and records the transition. | L2-C02 SP-007 / SP-008 |
| Student Activation is authorized by the Registrar. | Student Lifecycle | The activation decision cannot be assigned to an invented actor. | L2-C02 WF-C02-02 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry Position | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Admission | Admission Approved | Remaining governed admission conditions are completed and Admission reaches confirmation/completion hand-off. | Confirmed / completed admission hand-off position defined by L2-C01 | Admissions | L2-C01 |
| Student Aggregate | Not yet established for this admission, or existing canonical Student identity where applicable | Student Lifecycle establishes or reuses the canonical Student according to its invariants. | Student established | Student Lifecycle | L2-C02 |
| StudentContext | Pre-activation | Registrar-authorized Student Activation is performed. | **Active** | Student Lifecycle | L2-C02 WF-C02-02 |

The scenario does not redefine any Admission or Student Lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event / Fact | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `AdmissionConfirmed` | Admissions | Student Lifecycle | Provides the authoritative admission-completion hand-off into Student establishment. | L2-C01 |
| `StudentCreationRequested` | Admissions, where applicable in the locked C01 event flow | Student Lifecycle | Requests downstream canonical Student establishment without transferring Student ownership. | L2-C01 |
| `StudentCreated` | Student Lifecycle | Student Lifecycle and interested downstream consumers | Records canonical Student creation. | L2-C02 |
| `StudentActivated` | Student Lifecycle | Academic Registration and other authorized downstream consumers | Records completion of the Active Student outcome. | L2-C02 |

`StudentActivated` is published exclusively by Student Lifecycle. Admissions does not publish the Active Student outcome.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| Finance | Admissions | Financial-clearance business fact | Allows Admissions to evaluate the governed financial admission condition. |
| Admissions | Student Lifecycle | Confirmed Admission / Student-creation hand-off | Establishes the valid basis for Student establishment. |
| B — Institution & Structure | Student Lifecycle | Authoritative institutional context | Provides the Institute/structure context for StudentContext establishment without transferring ownership. |
| A — Platform Foundation | Participating domains | Canonical identity / authorization foundation | Supports identity and authority without owning Student lifecycle. |

Cross-domain interaction remains through governed facts, contracts and events; no direct cross-domain data ownership is introduced.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Admission confirmation | Admissions authority defined by L2-C01 and institutional policy | Must satisfy the locked Admission lifecycle and configured admission authority. | Admissions |
| Student activation | Registrar | Must satisfy WF-C02-02 activation conditions before transition to Active. | Student Lifecycle |

### 15.2 Separation of Duties, Overrides and Exceptions

Admissions confirmation and Student activation remain distinct business responsibilities. Finance provides financial truth but does not perform either lifecycle transition. Any configured exception remains governed by its owning L2 architecture and is not invented in this scenario.

### 15.3 Business Audit and Evidence Obligations

Admissions and Student Lifecycle preserve their authoritative lifecycle history. L2-C02 requires immutable audit records for Student lifecycle transitions, including activation. Platform audit capability remains A-owned and is not re-owned here.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The approved Admission has satisfied the governed conditions required for confirmation.
- Admissions has completed the authoritative hand-off to Student Lifecycle.
- The canonical Student and StudentContext exist under Student Lifecycle ownership.
- The StudentContext is **Active**.
- `StudentActivated` has been published by Student Lifecycle.

### 16.2 Postconditions

- The candidate is an operational Active Student within the applicable Institute context.
- Student and StudentContext remain canonically owned by Student Lifecycle.
- Admission remains a distinct upstream historical responsibility under Admissions.
- Finance, institutional structure and platform identity ownership remain unchanged.
- Downstream domains may consume the Active Student business fact through their governed contracts or events.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

None identified for this scenario after direct reconciliation with locked L2-C01 and L2-C02.

### 17.2 Assumptions

None. No assumption overrides the locked architecture.

### 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| What system behavior realizes the approved business flow? | L4 | Implementation behavior is outside L3. |

**Authoring Classification:** PASS — directly reconciled with locked L2-C01 and L2-C02; remains Draft pending formal Phase 5 review.
