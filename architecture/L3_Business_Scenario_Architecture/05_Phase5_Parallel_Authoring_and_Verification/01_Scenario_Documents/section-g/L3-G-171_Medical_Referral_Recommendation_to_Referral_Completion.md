# Medical Referral Recommendation to Referral Completion

**Scenario ID:** L3-G-171  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, E, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-171 |
| Canonical Title | Medical Referral Recommendation to Referral Completion |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional referral outcome in which an authorised healthcare professional determines that a patient requires referral, establishes the Referral with the appropriate external healthcare provider and reason, obtains the applicable acceptance outcome, and completes the institutional referral lifecycle.

The locked L2 defines Referral as a G06 aggregate owning referral reason, external healthcare provider, referral status and referral history. Its authoritative lifecycle is:

`Recommended → Accepted → Completed`.

Institutional Health Services may refer patients to external hospitals, while hospital management remains outside G06.

## 3. Business Outcome

The institutional medical referral has been completed as far as Institutional Health Services is responsible for it, with the referral reason, external healthcare provider, acceptance and completion history recorded.

The scenario does not claim ownership of or completion of the external hospital's subsequent medical treatment.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A Medical Encounter or healthcare assessment establishes that referral to an external healthcare provider is required.

### 4.2 End Boundary

The G06 Referral reaches `Completed` according to the authoritative referral lifecycle and the institutional referral outcome is recorded.

### 4.3 In Scope

- Referral recommendation.
- Referral reason.
- External healthcare provider selection within applicable policy.
- Referral creation.
- Referral acceptance.
- Referral completion.
- Referral history.
- Association with Medical Encounter/patient.
- Coordination with H for configured notifications.
- Healthcare audit continuity.
- Cross-domain personnel context.

### 4.4 Out of Scope

- External hospital admission.
- External hospital treatment.
- External provider clinical lifecycle.
- Insurance processing.
- Financial accounting.
- External hospital records ownership.
- Technical communication or transport mechanisms.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns Referral and the institutional referral lifecycle.

### Section E — People & Workforce

E remains authoritative for healthcare-professional/personnel context.

### Section H — Notifications

H may deliver configured referral notifications. It does not own Referral.

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Determines that referral is required and establishes the clinical/medical referral recommendation.

### Medical Staff

Coordinates the institutional referral process and records the referral outcome within authorised responsibility.

### Patient

Receives the referral and participates in the external-care handoff where applicable.

### External Healthcare Provider

Acts as the receiving external care destination. Its treatment lifecycle remains outside G06.

### Section E

Provides authoritative professional context.

### Section H

Provides notification delivery where configured.

No unsupported technical role or approval authority is introduced.

## 7. Preconditions and Entry Criteria

1. A valid patient and Medical Encounter exist.
2. Referral is determined to be medically required.
3. An authorised medical professional has authority to establish the referral recommendation.
4. An external healthcare provider can be identified according to applicable policy.
5. The referral reason can be established.
6. The patient remains within the institutional healthcare context until the applicable referral handoff outcome is reached.

## 8. Trigger

An authorised medical assessment determines that the patient should be referred to an external healthcare provider.

## 9. Main Business Flow

### Phase A — Referral Recommendation

1. Identify the patient and Medical Encounter.
2. Confirm the responsible healthcare professional.
3. Determine that institutional care alone is insufficient or that external healthcare is otherwise required.
4. Establish the referral reason.
5. Identify the appropriate external healthcare provider according to applicable institutional policy.
6. Create the Referral.
7. Place the Referral in `Recommended`.

### Phase B — Referral Coordination

8. Provide the referral information required for the institutional handoff.
9. Coordinate the referral with the external healthcare provider through the applicable business process.
10. Where the institutional process requires provider acceptance, establish the acceptance outcome.
11. Move the Referral to `Accepted` when the authoritative acceptance condition is satisfied.
12. Preserve the referral history.

The L2 defines the Referral lifecycle as `Recommended → Accepted → Completed`.

### Phase C — Referral Completion

13. Establish that the institutional referral handoff/completion condition has been satisfied.
14. Move the Referral to `Completed`.
15. Record the referral outcome and history.
16. Preserve the association to the Medical Encounter.
17. Where configured, provide relevant notification facts to H.
18. End the scenario.

### Boundary Condition

Completion means completion of the **institutional referral responsibility**, not completion of external hospital treatment.

## 10. Alternate Business Flows

### 10.1 Referral Accepted

The external healthcare provider accepts the referral and the referral proceeds to completion.

### 10.2 External Provider Not Available

The institution may consider another appropriate provider under the applicable policy. The original referral is not falsely completed.

### 10.3 Referral Recommendation Withdrawn Before Acceptance

If authorised medical reassessment establishes that referral is no longer required, the referral follows the applicable business outcome without inventing a new lifecycle state.

### 10.4 Patient Requires Emergency Transfer

Where the referral arises from an emergency case, emergency/ambulance coordination may proceed in parallel under its own scenario while G06 preserves the referral relationship.

### 10.5 Referral Requires Follow-up

Where the referral outcome creates a subsequent institutional follow-up need, that follow-up is governed through the applicable healthcare workflow.

### 10.6 Provider Accepts but Institutional Completion Evidence Is Pending

The Referral remains in the applicable lifecycle position until the business completion condition is satisfied.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Required Business Handling |
|---|---|
| Referral reason cannot be established | Referral is not completed until the authorised medical reason is established. |
| External provider cannot be identified | Preserve the healthcare requirement and resolve provider selection under applicable policy. |
| External provider does not accept | Consider an alternative authorised provider; do not mark the referral accepted/completed without evidence. |
| Patient condition changes | Reassess the healthcare decision under authorised medical judgement and continue through the applicable emergency/medical workflow. |
| Referral completion cannot be established | Preserve the Referral in its current valid lifecycle position until the completion condition is satisfied. |
| External hospital treatment remains unknown | Do not treat external treatment as an institutional completion fact. |

No technical retries, queues, rollback, APIs, database mechanics or infrastructure behaviour are described.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Application |
|---|---|---|
| Referral is a G06 aggregate | G06 | G06 owns referral lifecycle and history. |
| Referral owns referral reason, external healthcare provider, status and history | G06 | These facts remain within the referral aggregate. |
| Referral lifecycle is `Recommended → Accepted → Completed` | G06 | No local referral states are introduced. |
| Hospital management remains outside G06 | Boundary | External provider treatment is not represented as G06-owned. |
| Medical professionals retain operational authority | G06 | Referral decision remains professionally authorised. |
| Student identity remains C-owned | C | G06 consumes identity context only. |
| Employee/personnel information remains E-owned | E | G06 consumes personnel context only. |
| Healthcare activities are auditable | G06 | Referral lifecycle and outcome remain traceable. |

The L2 does not define a universal external-provider selection ranking or mandatory acceptance evidence beyond the lifecycle itself; no such local rule is invented.

## 13. State and Lifecycle Impact

### Referral

Authoritative lifecycle:

`Recommended → Accepted → Completed`

This scenario covers the complete lifecycle.

- `Recommended`: referral has been medically established.
- `Accepted`: receiving provider has accepted the referral according to the applicable business condition.
- `Completed`: the institutional referral responsibility has been fulfilled.

No new referral state is created.

### Medical Encounter

The Referral remains associated with the healthcare encounter that produced the referral recommendation. The scenario does not redefine Medical Encounter lifecycle.

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical publisher for Referral business facts because G06 owns the Referral aggregate.

The locked L2 establishes domain events for significant healthcare activities, but the exact canonical event names for Referral Recommended, Accepted and Completed are not exposed in the authoritative event material retrieved for this drafting pass.

Therefore no event names are invented.

H may consume the applicable G06 referral facts for configured notifications.

E provides authoritative personnel context and does not publish the Referral lifecycle outcome.

## 15. Business Authorization, Approval and Audit Obligations

The referral recommendation must be established by an appropriately authorised healthcare professional.

Business evidence should preserve:

| Obligation | Required Business Evidence | Owner |
|---|---|---|
| Referral recommendation | Referral reason and responsible healthcare decision | G06 |
| Referral destination | External healthcare provider associated with the Referral | G06 |
| Referral acceptance | Authoritative acceptance outcome where required | G06 |
| Referral completion | Institutional referral completion outcome | G06 |
| Referral traceability | Referral linked to patient/Medical Encounter and history | G06 |

Medical confidentiality and need-to-know obligations apply.

The scenario does not invent technical permission structures or external-provider approval mechanisms.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Referral requirement has been established by an authorised medical professional.
- Referral reason has been established.
- External healthcare provider has been identified.
- Referral has entered `Recommended`.
- Applicable provider acceptance has been established.
- Referral has reached `Accepted`.
- Institutional referral completion condition has been satisfied.
- Referral has reached `Completed`.
- Referral history and healthcare audit evidence are preserved.

### Postconditions

- Referral is in `Completed`.
- Referral remains traceable to its Medical Encounter.
- External hospital treatment remains outside G06 ownership.
- Any subsequent institutional follow-up remains separately governed.
- C/E ownership of identity/personnel context remains unchanged.
- H remains notification owner where configured.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Exact canonical Referral lifecycle event names are not exposed in the retrieved L2 event definitions | Prevents authoritative event naming in L3 | Verify the complete locked L2 event catalogue |
| Exact evidence required for external-provider acceptance/completion is not fully enumerated | L3 cannot invent an acceptance evidence model | Confirm authoritative referral contract/event definitions |

### Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| `Completed` represents completion of the institutional referral responsibility, not external hospital treatment | L2 explicitly keeps hospital management outside G06 | Boundary may need refinement if a locked contract defines completion differently |
| Provider selection remains subject to applicable institutional policy | L2 supports external referrals but does not define a universal ranking | A later authoritative policy may constrain selection |

### Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Detailed operational communication behaviour with external healthcare providers | L4/L5 | L3 must remain implementation-independent |
