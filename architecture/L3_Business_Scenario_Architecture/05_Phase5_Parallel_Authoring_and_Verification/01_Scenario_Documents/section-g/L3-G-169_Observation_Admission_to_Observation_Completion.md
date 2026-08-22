# Observation Admission to Observation Completion

**Scenario ID:** L3-G-169  
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
| Final L3 ID | L3-G-169 |
| Canonical Title | Observation Admission to Observation Completion |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, E, H |
| Scenario Type | Critical |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs the complete institutional healthcare outcome in which a patient is determined to require observation care, is admitted into the institutional observation workflow, receives the required observation and monitoring within institutional scope, reaches an authorised observation outcome, and is released from observation.

The locked L2-G06 explicitly includes Observation Care within Institutional Health Services, defines Observation Stay as a G06 aggregate, and establishes the Observation Stay lifecycle as:

`Admitted → Observed → Released`.

The scenario therefore begins only after the business decision to place the patient under observation and ends when the observation stay reaches its release outcome. It does not become hospital inpatient management or external clinical care.

## 3. Business Outcome

The patient has completed the required institutional observation period and has been released from observation with the resulting healthcare outcome recorded and traceable to the relevant Medical Encounter.

The scenario supports either an observation outcome that permits release or a governed transition to another healthcare path where observation establishes that further care is required.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A healthcare professional determines that observation care is required and an Observation Stay is to be admitted.

### 4.2 End Boundary

The patient is released from the institutional Observation Stay and the observation outcome has been recorded.

### 4.3 In Scope

- Observation admission decision.
- Association with patient and Medical Encounter.
- Observation Stay establishment.
- Observation activity within institutional healthcare scope.
- Observation notes/outcome where supported by L2.
- Release decision.
- Referral/follow-up escalation where observation establishes need.
- Healthcare history and audit continuity.
- Cross-domain professional context.
- Configured notification participation.

### 4.4 Out of Scope

- Hospital inpatient admission.
- ICU or advanced hospital monitoring.
- External hospital treatment.
- Diagnostic laboratory ownership.
- Financial accounting.
- Workforce ownership.
- Technical monitoring, scheduling or infrastructure implementation.

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns the Observation Stay, Medical Encounter and institutional observation-care outcome.

The L2 defines Observation Stay as an aggregate owned by Institutional Health Services.

### Section E — People & Workforce

E remains authoritative for healthcare-professional/personnel identity and workforce context. It does not own the Observation Stay.

### Section H — Notifications

H may deliver configured notifications relating to healthcare outcomes. H does not own observation state or clinical outcome.

## 6. Business Actors, Roles and Responsibilities

### Authorised Medical Professional

Determines whether observation is required, assesses the patient during observation, and determines the appropriate observation outcome within professional authority.

### Medical Staff

Performs the institutional observation activities and records the applicable healthcare outcome within the authorised operational process.

### Patient

Receives observation care and participates in release/follow-up arrangements where applicable.

### Section E

Provides authoritative personnel context for healthcare professionals and staff.

### Section H

Provides notification delivery where configured.

No technical actor or role is invented.

## 7. Preconditions and Entry Criteria

1. A valid patient and Medical Encounter exist.
2. Observation care has been determined to be required by an authorised healthcare professional.
3. An institutional observation capability is available.
4. The Observation Stay can be associated with the relevant Medical Encounter.
5. An appropriately authorised healthcare professional is responsible for the observation decision.
6. Applicable observation policy is available where configuration governs the observation process.

## 8. Trigger

An authorised healthcare professional determines that the patient requires institutional observation care.

## 9. Main Business Flow

### Phase A — Observation Admission

1. Identify the patient and Medical Encounter.
2. Confirm that the patient requires observation care.
3. Establish the Observation Stay.
4. Associate the Observation Stay with the relevant healthcare encounter.
5. Enter the Observation Stay into `Admitted`.

The L2 defines the Observation Stay lifecycle as `Admitted → Observed → Released`.

### Phase B — Observation

6. Begin the observation period.
7. Perform the observation activities required within institutional healthcare scope.
8. Maintain the patient's observation status and relevant clinical context.
9. Record the observation outcome necessary for the release decision.
10. Keep the Observation Stay associated with the patient and originating Medical Encounter.
11. Move the Observation Stay to `Observed` when the authoritative observation phase begins.

### Phase C — Observation Outcome

12. Assess whether the patient can be released from observation.
13. If release is appropriate, establish the release outcome.
14. If additional institutional care, referral, or follow-up is required, route to the applicable healthcare workflow without incorrectly closing the patient's healthcare need.
15. Preserve the observation outcome as part of the healthcare history.

### Phase D — Release

16. Establish the authorised release decision.
17. Move the Observation Stay to `Released`.
18. Record the resulting healthcare outcome.
19. Where configured, make the relevant business fact available to H for notification.
20. End the scenario.

## 10. Alternate Business Flows

### 10.1 Observation Leads to Further Institutional Care

If observation establishes that the patient requires further institutional treatment, the Observation Stay reaches its applicable release outcome while the subsequent healthcare activity continues through the appropriate Medical Encounter workflow.

### 10.2 Observation Leads to External Referral

If the patient requires care outside institutional capability, the observation outcome routes into the Referral workflow. External hospital care remains outside G06.

### 10.3 Follow-up Recommended

If observation concludes with a follow-up requirement, the recommendation is carried into the appropriate follow-up healthcare activity.

### 10.4 Patient Released Without Additional Care

Where observation establishes that no further immediate institutional care is required, the patient is released and the Observation Stay reaches `Released`.

### 10.5 Student or Employee Patient

C or E supplies authoritative identity/personnel context where applicable; G06 retains ownership of the healthcare outcome.

## 11. Failure, Interruption and Business Recovery

| Business Situation | Required Business Handling |
|---|---|
| Observation admission cannot be established against a valid Medical Encounter | Do not create an unassociated Observation Stay; resolve the authoritative encounter context first. |
| Observation capability is unavailable | Do not falsely represent the patient as admitted; route to the applicable alternative-care decision. |
| Responsible healthcare authority is unavailable | Preserve the observation condition and obtain the required authorised medical decision before completing the observation outcome. |
| Observation establishes need for external care | Route to the authoritative referral workflow; do not represent external hospital treatment as completed by G06. |
| Release decision cannot yet be established | Keep the Observation Stay in its applicable active observation lifecycle position until the authorised outcome is determined. |
| Patient requires another institutional healthcare activity | Preserve observation history and continue through the owning healthcare workflow. |

Recovery remains business-level. No technical retries, queues, rollback, API failure handling, database recovery or infrastructure mechanics are defined.

## 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Application |
|---|---|---|
| Observation Care is an Institutional Health Services capability | G06 | Observation is governed within G06. |
| Observation Stay is a G06 aggregate | G06 | G06 retains lifecycle ownership. |
| Observation Stay lifecycle is `Admitted → Observed → Released` | G06 | No additional local lifecycle states are introduced. |
| Medical professionals retain operational authority | G06 | Clinical observation/release decisions remain with authorised professionals. |
| Student identity remains owned by C | C | G06 consumes student context without re-owning it. |
| Employee/personnel information remains owned by E | E | G06 consumes workforce context without re-owning it. |
| External hospital management remains outside G06 | Boundary | Referral may occur, but hospital treatment is not included. |
| Healthcare activity is auditable | G06 | Observation lifecycle and outcome remain traceable. |

No local observation-duration, clinical-threshold, diagnosis or release criterion is invented because the locked L2 does not define such values.

## 13. State and Lifecycle Impact

### Observation Stay

Authoritative lifecycle:

`Admitted → Observed → Released`

This scenario covers the complete lifecycle.

- `Admitted`: observation care has been accepted as the institutional care path.
- `Observed`: active observation is being provided.
- `Released`: the observation stay has reached its release outcome.

No additional Observation Stay states are created.

### Medical Encounter

The Observation Stay contributes to the existing Medical Encounter healthcare history. It does not redefine the Medical Encounter lifecycle.

## 14. Business Events and Cross-Domain Collaboration

G06 is the canonical owner and therefore the canonical publisher of business facts concerning the Observation Stay.

The locked L2 establishes that significant healthcare activities publish immutable domain events and identifies domain events as an architectural mechanism. However, the exact canonical event names for Observation Stay admission, observation completion, or release are not exposed in the authoritative content retrieved for this drafting pass.

Therefore no event names are fabricated.

H may consume the applicable G06 healthcare fact for configured notification delivery.

E provides healthcare-professional/personnel context but does not publish or own the Observation Stay outcome.

## 15. Business Authorization, Approval and Audit Obligations

Observation admission and release decisions must remain within the authority of appropriately authorised healthcare professionals under the configurable healthcare authorization model.

The business evidence must preserve the healthcare outcome and lifecycle history required by the L2 audit principle.

Scenario-specific evidence includes:

| Business Obligation | Required Business Evidence | Owner |
|---|---|---|
| Observation admission | Observation Stay admission outcome associated with the patient/Medical Encounter | G06 |
| Observation completion | Observation outcome sufficient to establish the next healthcare path | G06 |
| Observation release | Authorised release outcome | G06 |
| Healthcare traceability | Observation history linked to the Medical Encounter | G06 |

Medical confidentiality and need-to-know obligations apply.

No technical access control/IAM or audit implementation is defined.

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Observation Stay has been validly admitted.
- Patient remains associated with the correct Medical Encounter.
- Observation care has been completed according to the applicable healthcare decision.
- An authorised observation outcome has been established.
- Observation Stay reaches `Released`.
- Any required subsequent healthcare path has been established.
- Healthcare history and applicable audit evidence are preserved.

### Postconditions

- The Observation Stay is in `Released`.
- The observation outcome is traceable to the relevant healthcare encounter.
- Any follow-up or referral requirement remains governed by its own healthcare workflow.
- G06 remains the owner of the observation outcome.
- C/E ownership of identity and workforce context remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Exact canonical Observation Stay event names are not exposed in the retrieved L2 event definitions | Prevents authoritative event naming in L3 | Verify against the complete locked L2 event catalogue before approval/lock |
| L2 does not define universal clinical release thresholds or observation duration | L3 cannot define local clinical criteria | Preserve professional authority and configuration-driven policy |

### Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Observation Stay remains wholly G06-owned | Locked L2 aggregate design | Ownership conflict if upstream architecture changes |
| Observation release can lead to referral/follow-up without changing Observation Stay ownership | L2 separates Observation Care and Referral/Follow-up capabilities | Boundary may require refinement if a locked workflow states otherwise |

### Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Exact system behaviour for observation monitoring and recording | L4 | Implementation-independent L3 does not define system behaviour |
