# Student Charge to Cancelled, Suspended, or Resumed Obligation

**Scenario ID:** L3-F-003  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Student Charge & Fee Management  
**Participating Domains:** C — Student Lifecycle; F — Finance; G — Institutional Services; I — Engagement, Communication & Experience  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-20  
**Depends On:** L2-F00 — Finance Domain Model & Foundation; L2-F01 — Student Charge & Fee Management; authoritative source-domain business events for cancellation/suspension/resumption; applicable downstream Financial Adjustment / Refund architecture where consequences arise  
**Referenced By:** Payment, Receivable, Refund, Accounting and Reporting consumers as applicable

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-F-003 |
| Canonical Title | Student Charge to Cancelled, Suspended, or Resumed Obligation |
| Primary Owner | Student Charge & Fee Management |
| Participating Domains | C — Student Lifecycle; F — Finance; G — Institutional Services; I — Engagement, Communication & Experience |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Govern the business lifecycle of a Student Charge when the underlying obligation is cancelled, suspended, or resumed, while preserving financial history and keeping the Student Charge as the canonical financial obligation.

The scenario coordinates source-domain business outcomes with the Finance lifecycle without allowing operational domains to mutate the financial aggregate directly.

## 3. Business Outcome

The Student Charge reaches the authoritative financial state corresponding to the underlying business outcome:

- **Cancelled** when the obligation is no longer valid;
- **Suspended** when the obligation is temporarily prevented from normal progression; or
- **Resumed** when a previously suspended obligation is validly returned to active financial use.

The transition is traceable to its authoritative business cause and does not erase historical financial state.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an authoritative business event indicates that an existing Student Charge should be cancelled, suspended, or resumed.

### 4.2 End Boundary

The scenario ends when Student Charge & Fee Management has applied the applicable canonical charge state and the resulting financial fact is available to downstream Finance consumers.

### 4.3 In Scope

- Receipt of authoritative cancellation, suspension or resumption business context.
- Validation that the affected Student Charge exists.
- Determination of the applicable charge state transition.
- Application of the canonical Student Charge state.
- Preservation of historical lifecycle/state traceability.
- Publication of the resulting Finance event.
- Coordination with downstream adjustment/refund/settlement processes where a financial consequence follows.

### 4.4 Out of Scope

- Deciding whether the originating operational condition should occur.
- Payment cancellation or reversal as a separate Payment Management responsibility.
- Refund authorization/execution.
- Accounting reversal or journal management.
- Direct modification of Student Charge by source domains.
- Technical state mutation, queues, retries or infrastructure.

## 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| C — Student Lifecycle | May originate the business event that causes a student-related charge consequence. |
| F — Student Charge & Fee Management | Owns Student Charge state and lifecycle. |
| G — Institutional Services | May originate service-related obligation changes affecting an existing charge. |
| I — Engagement, Communication & Experience | May participate where its authoritative business event has a financial consequence. |

The source domain remains the owner of the underlying business condition. Finance owns the Student Charge and its financial state.

## 6. Business Actors, Roles and Responsibilities

L2-F01 does not provide an authoritative Student Charge actor catalogue. Therefore this scenario does not invent new Finance actors.

The source domain's authorized business actors remain responsible for generating the underlying business outcome under the source domain architecture. Finance applies the resulting financial state within its canonical ownership boundary.

## 7. Preconditions and Entry Criteria

- A canonical Student Charge exists.
- The Student Charge is in a state from which the requested transition is permitted.
- An authoritative source-domain business event or governed financial decision exists.
- The business reason for cancellation, suspension or resumption is traceable.
- The requested transition does not require another domain to directly alter the Student Charge.
- Any downstream financial consequence can be handled by its canonical owning Finance capability.

## 8. Trigger

An authoritative business event indicates that an existing Student Charge must be cancelled, suspended, or resumed.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Source domain establishes the business outcome that affects the obligation. | C / G / I as applicable | A valid source-domain financial consequence exists. |
| 2 | The corresponding business event/context is made available to Student Charge & Fee Management. | Source domain | Finance can identify the affected charge. |
| 3 | Student Charge & Fee Management resolves the canonical Student Charge. | F | The affected financial obligation is identified. |
| 4 | Finance validates that the requested state transition is permitted by the Student Charge lifecycle/state model. | F | The transition is either eligible or rejected. |
| 5 | Finance applies the requested state transition: Cancelled, Suspended or Resumed as applicable. | F | Student Charge reflects the current financial condition. |
| 6 | Historical lifecycle and state information remains traceable. | F | Prior financial state is preserved. |
| 7 | The canonical Finance event for the completed state change is published. | F | Downstream Finance domains can consume the new financial fact. |
| 8 | Where the state change creates a downstream refund, adjustment, settlement or accounting consequence, the relevant canonical Finance domain process takes ownership of that consequence. | F | Secondary financial consequences are handled by their canonical owners. |

## 10. Alternate Business Flows

### A1 — Cancellation

The Student Charge moves to the canonical `Cancelled` state where the source-domain business outcome invalidates the obligation.

### A2 — Suspension

The Student Charge moves to `Suspended`. The obligation remains historically identifiable and may later be Resumed when the governing business condition permits.

### A3 — Resumption

A previously Suspended Student Charge is returned to the applicable active financial position when the authoritative business event permits resumption.

### A4 — Invalid Transition

The requested cancellation, suspension or resumption is not compatible with the current charge state. The transition is rejected and the current canonical state remains unchanged.

### A5 — Downstream Financial Consequence

A cancellation or other lifecycle change creates a financial adjustment, refund or settlement consequence. The Student Charge lifecycle change is completed by F01, while the secondary consequence is handled by the applicable downstream Finance domain.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Resolution |
|---|---|
| Student Charge cannot be identified | Do not change financial state; resolve the source-to-charge relationship. |
| Requested transition is not permitted | Preserve the current canonical state and reject the invalid transition. |
| Source business event is ambiguous | Do not infer the financial outcome; await an authoritative business fact. |
| Cancellation creates a refund consequence | Route the refund to Refund Management. |
| State change requires financial adjustment | Route the adjustment to Financial Adjustment Management. |
| State change affects accounting | Accounting applies its canonical downstream treatment; F01 does not create journal entries. |

Technical failure handling is outside L3.

## 12. Business Rules and Decision Constraints

Authoritative L2-F01 rules applied by this scenario include:

- Every Student Charge has one canonical operational state.
- State transitions must be deterministic.
- Historical states remain preserved.
- External domains shall not manipulate canonical Student Charge state directly.
- Published Student Charges remain historically immutable.
- Financial corrections are handled through downstream adjustment mechanisms.
- Student Charge remains the canonical financial obligation.
- Payment and accounting remain separate responsibilities from Student Charge lifecycle.

No new state or transition is invented.

## 13. State and Lifecycle Impact

L2-F01 provides the following canonical state vocabulary, including:

`Draft`, `Pending Validation`, `Validated`, `Published`, `Active`, `Suspended`, `Cancelled`, `Waived`, `Written Off`, `Disputed`, `Closed`, `Archived`.

This scenario uses the relevant states:

- `Active → Suspended`
- `Suspended → Active` for governed resumption
- `Active → Cancelled` or another permitted cancellation path according to the authoritative transition mapping

The exact permitted state-transition matrix for each source condition remains owned by L2-F01 and is not expanded beyond the published architecture here.

No new lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Business Events

The source domain publishes the authoritative business event that causes the financial consequence.

Student Charge & Fee Management publishes canonical Student Charge lifecycle events. L2-F01 identifies lifecycle events including:

- `ChargeIdentificationInitiated`
- `ChargeGenerationCompleted`
- `ChargeValidationCompleted`
- `ChargePublicationCompleted`
- `ChargeActivationCompleted`
- `ChargeClosureCompleted`
- `ChargeArchivalCompleted`

The accessible L2-F01 source does not define a dedicated cancellation/suspension/resumption event name in its canonical event list. Therefore this scenario does not invent one.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| C / G / I | F — Student Charge | Authoritative business event causing financial lifecycle change | Identifies why the charge must change state. |
| F — Student Charge | Financial Adjustment / Refund / Accounting | Resulting financial consequence | Enables downstream Finance ownership where required. |
| F — Student Charge | Payment / Receivable / Reporting | Updated Student Charge state | Allows downstream financial consumers to reflect the current obligation. |

Participation does not transfer Student Charge ownership.

## 15. Business Authorization, Approval and Audit Obligations

The source domain retains authority for the originating business decision.

Finance applies the resulting state transition under Student Charge governance.

The state change, source business cause, resulting financial state and downstream financial consequence must remain traceable.

L2-F01 explicitly includes lifecycle/state changes in its audit participation and requires historical financial information to remain immutable and reconstructable.

No Finance-specific actor/approval role is invented because L2-F01 does not establish one.

## 16. Completion Criteria and Postconditions

The scenario is complete when:

- the authoritative source business outcome has been identified;
- the correct canonical Student Charge has been identified;
- a permitted state transition has been applied;
- the Student Charge now reflects the authoritative cancelled, suspended or resumed business state;
- prior states remain historically traceable;
- the relevant downstream financial consequence is routed to its canonical owner where required;
- no source domain has directly modified the Student Charge.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

The L2-F01 state architecture defines the canonical state vocabulary but does not expose a complete explicit transition matrix for every cancellation/suspension/resumption condition in the accessible source material. This scenario therefore preserves the named states and uses only transitions clearly consistent with the locked lifecycle rather than inventing a full new transition table.

The L2-F01 event catalogue also does not expose a dedicated named cancellation/suspension/resumption event.

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

Confirm the exact F01 state-transition and event mapping for each source-domain cancellation, suspension and resumption condition during architecture review.
