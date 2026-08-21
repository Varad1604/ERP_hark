# Learning Provider Onboarding to Governed Active Provider

**Scenario ID:** L3-D-044  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0764 / CAN-L3-D-044

---

# 2. Scenario Purpose

Onboard an institution-approved external learning provider through unique identity, governed profile/capability configuration, connectivity validation and activation.

# 3. Business Outcome

A uniquely coded provider is Active with versioned configuration, validated connectivity and approved capabilities; suspended providers cannot receive new enrollments.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Administrative authorization and provider documentation are available for a new provider or governed provider change.

## 4.2 End Boundary

The provider reaches Active and `LearningProviderActivated` is published after successful validation.

## 4.3 In Scope

- Create unique provider profile and approved capability set.
- Configure authentication/integration and validate connectivity.
- Activate, suspend, reactivate or migrate through governed paths.

## 4.4 Out of Scope

- Treating provider as owner of institutional enrollment or records.
- Activation without institutional approval or connectivity validation.
- Hard-coding provider-specific business semantics.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns institutional Provider configuration, active/suspended availability and integration relationship. |
| External provider registry/accreditation authority | Supplies authoritative provider identity, accreditation and approval evidence. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Affairs | Approves institutional provider use and capabilities. | Academic onboarding authority. |
| Integration Administrator | Configures provider profile, authentication and connectivity. | Technical configuration authority only. |
| Enterprise Administrator | Authorizes provider-level administrative lifecycle changes. | Governed administrative authority. |

# 7. Preconditions and Entry Criteria

- Administrative approval and provider documentation exist.
- Provider code can be unique.
- Connectivity and supported capabilities can be validated.

# 8. Trigger

A new provider is approved for onboarding or an existing provider requires governed lifecycle/configuration change.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create provider onboarding record and validate uniqueness/approval. | Academic Affairs / D06 | Provider candidate established. |
| 2 | Configure profile, capabilities and institutional usage scope. | Academic Affairs / Integration Administrator | Governed provider configuration exists. |
| 3 | Configure authentication and provider integration version. | Integration Administrator | Versioned integration configuration exists. |
| 4 | Validate connectivity, authentication and supported capabilities. | D06 Integration Service | Readiness confirmed or activation blocked. |
| 5 | Approve and activate provider. | Academic Affairs / Enterprise Administrator | Provider becomes Active. |
| 6 | Publish configuration and activation facts. | D06 | LearningProviderConfigured and LearningProviderActivated. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Open Electives & MOOCs |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Open Electives & MOOCs |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Open Electives & MOOCs |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Open Electives & MOOCs | Prevents cross-domain ownership leakage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Open Electives & MOOCs | Prevents governance bypass. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Published or historical business facts remain immutable. | Open Electives & MOOCs | Preserves auditability and lineage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Learning Provider | Onboarding / Configured | Validate and activate approved provider. | Onboarding → Configured → Active; governed alternate Suspended / Reactivated | D06 | L2-D06 W60 and W65 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| LearningProviderConfigured / LearningProviderActivated / ProviderIntegrationConfigured | D06 | Opportunity configuration, enrollment and integration consumers | Records governed provider configuration and activation. | L2-D06 W60, W65; D06-E51/E56 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| External provider registry/accreditation authority | Open Electives & MOOCs | Supplies authoritative provider identity, accreditation and approval evidence. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Approves institutional provider use and capabilities. | Academic Affairs | Academic onboarding authority. | Open Electives & MOOCs |
| Configures provider profile, authentication and connectivity. | Integration Administrator | Technical configuration authority only. | Open Electives & MOOCs |
| Authorizes provider-level administrative lifecycle changes. | Enterprise Administrator | Governed administrative authority. | Open Electives & MOOCs |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Open Electives & MOOCs |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Open Electives & MOOCs |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Open Electives & MOOCs |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A uniquely coded provider is Active with versioned configuration, validated connectivity and approved capabilities; suspended providers cannot receive new enrollments.
- All required authority, validation and audit evidence is retained.
- Applicable completed business facts are published exactly once in business effect.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical decisions and prior states remain immutable.
- Downstream domains consume facts without modifying the owning aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
