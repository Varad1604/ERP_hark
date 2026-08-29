# Institution-Assisted or Bulk Examination Registration

**Scenario ID:** L3-I-018  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** C, D, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-I01 Examination Policy & Configuration; L2-I02 Examination Session Management; L2-I03 Examination Planning & Scheduling; L2-I04 Examination Registration; Student; Curriculum Framework; Academic Calendar  
**Referenced By:** L2-I05 Hall Ticket Management; downstream examination workflows; Audit

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-018 |
| Canonical Title | Institution-Assisted or Bulk Examination Registration |
| Primary Owner | Examination Registration |
| Participating Domains | C, D, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern institution-assisted or bulk creation of Examination Registrations for a defined candidate population while preserving the canonical registration ownership and applying the same eligibility, subject, policy, approval, confirmation and audit requirements that govern individual candidate registration.

## 3. Business Outcome

A governed candidate population is transformed into authoritative Examination Registrations for eligible candidates, with each individual registration remaining separately identifiable, validated, auditable and Confirmed where all required conditions are satisfied.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an institutionally defined population is identified for assisted or bulk examination registration during an approved registration window.

### 4.2 End Boundary

The scenario ends when eligible candidates in the population have authoritative Confirmed Examination Registrations or an explicitly governed non-confirmed outcome, while individual registration history remains traceable.

### 4.3 In Scope

- Candidate population definition.
- Assisted/bulk registration initiation.
- Per-candidate eligibility validation.
- Subject and examination-category applicability.
- Institutional review/approval where required.
- Confirmation of eligible registrations.
- Partial population outcomes.
- Duplicate prevention.
- Audit and traceability.
- Publication of canonical registration events for completed individual registrations.

### 4.4 Out of Scope

- Technical batch processing.
- File imports, APIs, queues or implementation mechanics.
- Creation of a bulk-registration aggregate not defined by L2-I04.
- Examination scheduling.
- Hall Ticket generation.
- Examination conduct.
- Student or curriculum ownership.

## 5. Ownership and Participating Domains

| Domain | Responsibility | Canonical Ownership |
|---|---|---|
| Examination Registration | Owns each Examination Registration and its lifecycle. | I04-owned. |
| Student | Provides student context and academic status. | Student remains C-owned. |
| Curriculum Framework | Provides subject applicability. | Curriculum remains D-owned. |
| Examination Administration | Operates institution-assisted registration within I04 responsibilities. | Registration business operations remain I04-owned. |

L2-I04 defines the Examination Registration domain as the authoritative source for candidate examination participation and includes Examination Administration as a participating business actor.

## 6. Business Actors, Roles and Responsibilities

L2-I04 identifies:

- Examination Administrator
- Department Administration
- Academic Administration
- Candidate (Student)
- Controller of Examination
- supporting Student and Curriculum domains

No separate “Bulk Operator” role is invented. Institution-assisted activity uses the existing Examination Administrator/administrative roles defined by I04, subject to institutional authority.

## 7. Preconditions and Entry Criteria

- Examination Session is Active.
- Examination Schedule is Published.
- Registration window is open or an approved exception applies.
- Candidate population is defined by authoritative institutional context.
- Applicable eligibility information is available for each candidate.
- Subject applicability is available from the Curriculum Framework.

## 8. Trigger

An authorized institution-assisted registration activity begins for a defined candidate population.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Establish the candidate population and the examination context. | Examination Administration / Registration | The population is ready for governed registration. |
| 2 | Validate the active Examination Session and Published Examination Schedule. | Examination Registration | The registration context is valid. |
| 3 | Evaluate each candidate against registration eligibility and applicable restrictions. | Examination Registration with Student/Curriculum inputs | Each candidate receives an eligibility outcome. |
| 4 | Determine applicable examination subjects and category for each eligible candidate. | Examination Registration / Curriculum | Registration scope is established. |
| 5 | Create/register the examination participation record for each eligible candidate. | Examination Registration | Individual registrations exist within the canonical Registration lifecycle. |
| 6 | Conduct required institutional review and approval. | Examination Administration / authorised authority | Eligible registrations become approved where required. |
| 7 | Confirm each successful registration. | Examination Registration | Individual registrations become Confirmed. |
| 8 | Record non-confirmed candidates with the applicable governed reason. | Examination Registration | Ineligible/rejected/exception cases remain traceable without false confirmation. |
| 9 | Publish registration events for each completed registration transition. | Examination Registration | Downstream workflows receive authoritative facts. |
| 10 | Preserve population-level operational evidence and individual registration audit history. | Examination Registration / Audit Infrastructure | Bulk activity is reconstructable without creating a competing aggregate. |

L2-I04 establishes candidate-level registration governance and treats the Examination Registration as the authoritative business record.

## 10. Alternate Business Flows

### A1 — All Candidates Eligible

All members of the defined population complete registration and confirmation subject to required approvals.

### A2 — Mixed Eligibility

Eligible candidates become Confirmed while ineligible candidates remain outside confirmation with recorded business reasons.

### A3 — Existing Registration Detected

An existing valid registration remains authoritative and duplicate registration is prevented.

### A4 — Candidate-Specific Exception

A candidate requiring special permission proceeds through the institution's approved exception process without changing the standard registration architecture.

### A5 — Registration Window Closes During Population Processing

Candidates not yet validly registered cannot be treated as Confirmed outside the approved window unless a governed exception applies.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Resolution |
|---|---|
| Some candidates fail eligibility | Continue valid candidate registrations where policy permits; failed candidates retain their individual exception outcome. |
| Duplicate registration detected | Preserve the existing valid registration and do not create another. |
| Subject applicability fails | Do not register the invalid subject; process eligible subjects according to policy. |
| Approval is unavailable for a candidate | Candidate remains non-confirmed until the required approval is completed. |
| A bulk activity contains inconsistent candidate inputs | Resolve candidates individually using authoritative Student/Curriculum context; do not create a population-level workaround. |
| A candidate requires late/special registration | Apply the authorised institutional exception for that candidate. |

L2-I04 requires duplicates to be prevented, eligibility failures to stop progression and exceptional conditions to remain institutionally governed.

## 12. Business Rules and Decision Constraints

- Bulk assistance does not bypass individual Examination Registration eligibility.
- Each Examination Registration belongs to one candidate.
- An Active Examination Session and Published Schedule are required.
- Subject applicability follows the Curriculum Framework.
- Only valid/approved registrations may become Confirmed.
- Duplicate registrations are prevented.
- Registration history is auditable.
- Ownership remains with the originating domains.
- Bulk/assisted activity does not create a new canonical bulk Registration aggregate.

## 13. State and Lifecycle Impact

Each candidate follows the canonical Registration lifecycle:

`Registration Initiated → Eligibility Validation → Subject Registration → Registration Review → Registration Approval → Registration Confirmation → Registration Locking → Operational Consumption → Registration Completion → Registration Archival`

The scenario does not introduce a bulk-registration state. The population is an operational grouping of individual Examination Registrations.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Canonical Events

The Examination Registration domain publishes the same canonical Registration events for each individual registration:

- `Registration Initiated`
- `Eligibility Validated`
- `Registration Submitted`
- `Registration Approved`
- `Registration Rejected`
- `Registration Confirmed`
- `Registration Amended`
- `Registration Withdrawn`
- `Registration Locked`
- `Registration Archived`

No `BulkRegistrationCompleted` event is invented because L2-I04 does not define a population-level event.

### 14.2 Business Hand-offs

| From | To | Business Fact | Purpose |
|---|---|---|---|
| Student | Examination Registration | Candidate/academic context | Supports individual eligibility. |
| Curriculum Framework | Examination Registration | Subject applicability | Determines valid subjects. |
| Examination Registration | Hall Ticket Management | `Registration Confirmed` | Supports downstream Hall Ticket population. |
| Examination Registration | Audit | Individual lifecycle events | Preserves business traceability. |

Events remain distinct from business hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Authority

Institution-assisted registration operates under the Examination Administrator/Controller/academic administration responsibilities defined by I04. No new bulk-specific authority is created.

### 15.2 Separation of Duties

Where institutional approval is required, the same approval requirements applicable to individual registration remain applicable.

### 15.3 Audit Evidence

The bulk activity must remain traceable through:

- candidate population context;
- individual registrations;
- eligibility outcomes;
- approvals;
- confirmations;
- failures/exceptions;
- event history.

L2-I04 requires complete auditability for significant registration activities.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Candidate population is identified.
- Registration context is valid.
- Each candidate receives an authoritative eligibility outcome.
- Eligible candidates receive valid individual Examination Registrations.
- Required approvals are completed.
- Confirmed registrations are published as authoritative facts.
- Non-confirmed candidates retain traceable outcomes.

### 16.2 Postconditions

- Individual confirmed registrations are available to downstream workflows.
- No bulk Registration aggregate has been introduced.
- Candidate ownership remains in the Student domain.
- Curriculum ownership remains in the Curriculum Framework.
- Registration ownership remains with Examination Registration.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

L2-I04 defines candidate-level Registration but does not define a separate bulk-registration aggregate, batch state, or population-level event. This L3 therefore models bulk/assisted registration as coordinated creation of individual canonical registrations, not as a new bulk aggregate.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

Any future requirement for population-level bulk-registration state or governance must be established upstream before a new L3 aggregate/state/event is introduced.
