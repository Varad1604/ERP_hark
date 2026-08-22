# Medical Configuration Change to Effective Healthcare Policy

**Scenario ID:** L3-G-183  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Institutional Health Services  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P1  
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
| Final L3 ID | L3-G-183 |
| Canonical Title | Medical Configuration Change to Effective Healthcare Policy |
| Primary Owner | Institutional Health Services |
| Participating Domains | G, A |
| Scenario Type | Simple |
| Priority | P1 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Institutional healthcare must operate according to approved institutional policies and configuration, while different institutions and campuses may require different healthcare operating arrangements. Configuration allows the same G06 capability to support institutions with no medical facility, a basic medical room, or a full health centre.

This scenario provides the end-to-end business path from an authorised healthcare configuration change through validation and effective use by G06 healthcare operations.

---

## 3. Business Outcome

An authorised healthcare configuration change has been reviewed under the applicable governance rules, determined to be valid for G06 healthcare operations, and made effective as the current healthcare policy/configuration for the applicable scope.

If the proposed change is not authorised or valid, it is not made effective.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

A proposed change to a G06-owned healthcare configuration or policy is submitted for governed consideration.

### 4.2 End Boundary

The authorised configuration change is effective for the applicable G06 healthcare scope and subsequent healthcare operations use the effective policy.

### 4.3 In Scope

- Healthcare configuration change request.
- Identification of affected healthcare capability/policy.
- Scope of the configuration.
- Governance review.
- Validation against applicable healthcare constraints.
- Effective activation of the approved change.
- Preservation of configuration history/audit.
- Impact on subsequent healthcare operations.

### 4.4 Out of Scope

- Technical configuration storage.
- APIs, databases, classes, services or deployment.
- Infrastructure configuration.
- technical access control/IAM.
- UI configuration screens.
- General institutional policy ownership outside G06.
- Financial policy ownership.
- Student or workforce policy ownership.

---

## 5. Ownership and Participating Domains

### Institutional Health Services — G06

G06 owns healthcare operational configuration and the healthcare policy semantics used by medical workflows.

### Section A — Platform Foundation

A provides platform-level configuration/governance capability and supporting policy mechanisms.

A does not acquire ownership of the medical policy simply because it provides configuration capability.

The cross-domain relationship is therefore:

**A provides platform configuration capability; G06 owns the healthcare meaning and operational effect.**

---

## 6. Business Actors, Roles and Responsibilities

### Authorised Healthcare Administrator / Medical Administrator

Initiates or coordinates a healthcare configuration change within G06 business responsibility.

### Healthcare Governance Authority

Reviews and authorises the healthcare policy/configuration change where the applicable governance process requires such review.

### Platform Configuration Function

Provides the platform-level configuration capability without owning the healthcare policy semantics.

No fixed approval chain is invented where the authoritative architecture has not specified one.

---

## 7. Preconditions and Entry Criteria

1. A healthcare configuration or policy change requirement exists.
2. The affected configuration belongs within G06 healthcare scope.
3. The requested change can be expressed using the existing G06 configuration capability.
4. The applicable governance authority is identifiable.
5. The change can be evaluated against existing healthcare rules and constraints.
6. The change does not silently redefine ownership of another domain.

---

## 8. Trigger

An authorised requirement for a change to G06 healthcare configuration or policy is raised.

---

## 9. Main Business Flow

### Phase A — Define the Proposed Change

1. Identify the healthcare operational requirement.
2. Identify the affected G06 configuration or policy.
3. Record the intended change and its business rationale.
4. Identify the institutional/campus scope to which the change applies.
5. Determine the expected healthcare operational effect.

### Phase B — Validate the Change

6. Review the proposed change against existing G06 healthcare rules and constraints.
7. Confirm that the change remains within G06 ownership.
8. Confirm that the change does not modify C-owned student identity, E-owned workforce identity, Finance-owned billing, or another domain's canonical policy.
9. Determine whether the change requires governance approval under the applicable policy.
10. Resolve any identified business conflict before making the change effective.

### Phase C — Authorise and Activate

11. Obtain the required business authorization where applicable.
12. Establish the approved configuration as the effective healthcare policy/configuration.
13. Preserve the effective scope and business meaning of the configuration.
14. Ensure subsequent affected G06 healthcare operations use the effective configuration.

### Phase D — Record the Outcome

15. Record the configuration change and its effective business outcome.
16. Preserve prior configuration history where required by the authoritative governance policy.
17. Publish an applicable canonical configuration-change event only if one is explicitly defined upstream.
18. End the scenario.

---

## 10. Alternate Business Flows

### 10.1 Change Approved Without Modification

The proposed configuration is valid as submitted and becomes effective.

### 10.2 Change Requires Correction Before Approval

The proposed change is returned for correction. It does not become effective until the corrected change satisfies the applicable governance requirements.

### 10.3 Change Rejected

The change is not made effective. The current valid healthcare configuration remains in force.

### 10.4 Change Applies to a Limited Scope

The approved change becomes effective only within its governed institutional/campus scope. It does not implicitly become a global healthcare policy.

### 10.5 Change Has Cross-Domain Implications

If the proposed healthcare configuration would change another domain's canonical ownership or policy, the issue is escalated to the appropriate upstream architecture rather than silently embedding the other domain's rule in G06.

### 10.6 Change Affects Active Healthcare Operations

Where an approved configuration change affects active operations, its effective business point must be governed so that existing healthcare outcomes are not retrospectively reinterpreted without an explicit authoritative rule.

---

## 11. Failure, Interruption and Business Recovery

| Business Situation | Business Recovery |
|---|---|
| Proposed configuration is outside G06 scope | Do not activate it; route the matter to the canonical owning domain. |
| Configuration violates an existing healthcare rule | Correct or reject the proposed change before activation. |
| Required governance authority is unavailable | Keep the current effective configuration and defer activation until governance is resolved. |
| Change has ambiguous cross-domain ownership | Record and escalate the ownership issue; do not activate a locally invented compromise. |
| Approved change cannot safely affect active operations at the proposed point | Retain the current effective configuration until an authorised effective point is established. |
| Configuration change is later found to be incorrect | Apply the authoritative correction/supersession process; do not silently rewrite historical healthcare outcomes. |

No technical rollback, deployment failure, queue, retry, database or infrastructure behaviour is defined.

---

## 12. Business Rules and Decision Constraints

1. G06 owns healthcare operational configuration and healthcare policy semantics within its capability.
2. Platform Foundation provides configuration capability but does not become owner of G06 healthcare policy.
3. A configuration change must remain within the canonical ownership boundary.
4. An unauthorised configuration must not become effective.
5. Configuration changes must be traceable.
6. Scope must be explicit where the change is not institution-wide.
7. Existing healthcare outcomes must not be silently reinterpreted by a later configuration change.
8. Configuration must support institutional variation without requiring architectural redesign.
9. Student, workforce, financial, academic and other domain-owned policies must not be recreated inside G06.
10. Healthcare professionals retain operational healthcare authority; configuration does not replace clinical judgement.

The scenario does not invent approval roles, policy precedence rules, configuration version identifiers, effective-date mechanics, or technical enforcement.

---

## 13. State and Lifecycle Impact

The scenario governs the business status of a proposed configuration change through approval and effective use but does not invent a new canonical G06 configuration state machine.

The authoritative upstream architecture identifies configuration as configuration-driven, but the retrieved L2-G06 document does not provide a dedicated named lifecycle for Medical Configuration Change.

Accordingly, this L3 scenario uses business outcomes—proposed, reviewed, authorised, effective, rejected/returned—only as descriptive flow conditions, not as a new canonical lifecycle model.

The lifecycle of each affected healthcare aggregate remains unchanged. The new policy governs applicable future healthcare operations from its effective business point and does not retroactively rewrite completed healthcare history without an authoritative rule.

---

## 14. Business Events and Cross-Domain Collaboration

The retrieved L2-G06 event catalogue does not establish a dedicated canonical event name for a generic medical configuration change.

Therefore this scenario does **not** invent an event such as:

`MedicalConfigurationChanged`

or

`HealthcarePolicyActivated`.

If an authoritative platform configuration event exists and is applicable, its canonical name and publisher must be used exactly as defined upstream. Platform-level configuration event ownership remains with the platform capability; G06 remains owner of the healthcare meaning and operational effect.

The scenario does not treat configuration consumption as ownership transfer.

---

## 15. Business Authorization, Approval and Audit Obligations

A healthcare configuration change must follow the applicable business governance authority.

Audit/evidence should preserve:

- Proposed healthcare configuration change.
- Business rationale.
- Affected G06 capability/policy.
- Scope.
- Review outcome.
- Approval/rejection outcome.
- Effective business outcome.
- Configuration history required by applicable governance.
- Impacted healthcare capability.

The scenario does not invent a mandatory approval chain because the retrieved L2-G06 source does not define a fixed named approver sequence for generic medical configuration.

Technical IAM or access-control enforcement is outside L3.

---

## 16. Completion Criteria and Postconditions

### Completion Criteria

- Healthcare configuration requirement is identified.
- Affected G06 policy/configuration is identified.
- Scope and intended business effect are understood.
- Ownership remains within the correct architectural boundary.
- Required governance review is completed.
- The change is approved and made effective, or explicitly rejected/returned.
- Effective configuration is available for subsequent G06 healthcare operations.
- Audit/history is preserved.

### Postconditions

For an approved change:

- The new healthcare configuration/policy is effective for its governed scope.
- G06 healthcare operations use the effective configuration.
- Existing healthcare history remains intact.
- Other domain ownership remains unchanged.

For a rejected/returned change:

- The existing effective healthcare configuration remains authoritative.
- The proposed change is not treated as active policy.

---

## 17. Open Issues, Assumptions and Deferred Questions

### Open Issues

1. L2-G06 establishes that healthcare configuration is configuration-driven but does not define a dedicated lifecycle or canonical event for generic medical configuration changes.
2. The retrieved source does not specify a universal approval chain for healthcare configuration changes.

### Assumptions

- G06 owns the healthcare meaning and operational policy represented by its configuration.
- Platform Foundation provides the generic configuration capability.
- Configuration changes are governed according to applicable institutional/platform policy.
- Configuration changes do not retrospectively rewrite completed medical history.

### Deferred Questions

Before Architecture Approval/Lock, confirm the authoritative source for:
- the exact lifecycle/status model of generic G06 configuration changes;
- the canonical configuration-change event, if one is required;
- the approval authority for healthcare policy/configuration changes;
- effective-scope and effective-date governance.

These are deliberately not invented at L3.
