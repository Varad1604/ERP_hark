# Internal Assessment Requirement to Published Assessment Blueprint

**Scenario ID:** L3-D-051  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D  
**Scenario Type:** Simple  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0771 / CAN-L3-D-051

---

# 2. Scenario Purpose

Transform an internal-assessment requirement into a governed Published blueprint defining component structure, weightage and policy binding for an assessment cycle.

# 3. Business Outcome

A validated Internal Assessment Blueprint reaches Published and becomes the authoritative D07 structure required before any Assessment Window may open.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved Course Offering or academic-term context requires an internal or continuous assessment structure.

## 4.2 End Boundary

The blueprint reaches Published and `InternalAssessmentBlueprintPublished` is published.

## 4.3 In Scope

- Reference authoritative Course Offering, curriculum and calendar context.
- Define assessment components, weightage and policy binding.
- Validate completeness and publish the blueprint.

## 4.4 Out of Scope

- Course/Course Offering creation.
- Assessment window execution.
- Examination result or grade computation.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns InternalAssessmentBlueprint, component structure, weightage, policy binding and publication. |
| D01 / Section B | Own authoritative Course Offering, curriculum and calendar references consumed by D07. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Policy-configured D07 blueprint administrator | Creates and validates the assessment blueprint. | CreateInternalAssessmentBlueprint authority. |
| Policy-configured D07 publication authority | Approves publication after validation. | PublishInternalAssessmentBlueprint authority. |

# 7. Preconditions and Entry Criteria

- Published Course Offering and applicable curriculum/calendar references exist.
- Assessment-cycle policy and required component information are available.

# 8. Trigger

An authorized academic requirement calls for an internal-assessment structure.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create Draft blueprint for the governed assessment cycle. | D07 blueprint administrator | Draft blueprint exists. |
| 2 | Define components, weightage and policy binding. | D07 blueprint administrator | Blueprint structure complete. |
| 3 | Validate source references, component completeness, weightage and policy. | D07 | Blueprint valid or returned for correction. |
| 4 | Authorize publication. | D07 publication authority | Publication decision recorded. |
| 5 | Publish immutable business fact. | D07 | Published and InternalAssessmentBlueprintPublished. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Internal Assessment |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Internal Assessment |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Internal Assessment |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Internal Assessment | Prevents cross-domain ownership leakage. | `L2-D07_Internal_Assessment_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Internal Assessment | Prevents governance bypass. | `L2-D07_Internal_Assessment_Workflows.md` |
| Published or historical business facts remain immutable. | Internal Assessment | Preserves auditability and lineage. | `L2-D07_Internal_Assessment_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Internal Assessment Blueprint | Draft | Define, validate and publish assessment structure. | Draft → Published | D07 | L2-D07 §§5–8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| InternalAssessmentBlueprintPublished | D07 | Assessment Window, Academic Monitoring and authorized consumers | Records governed blueprint publication. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D01 / Section B | Internal Assessment | Own authoritative Course Offering, curriculum and calendar references consumed by D07. | Preserve ownership while enabling the scenario. | `L2-D07_Internal_Assessment_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Creates and validates the assessment blueprint. | Policy-configured D07 blueprint administrator | CreateInternalAssessmentBlueprint authority. | Internal Assessment |
| Approves publication after validation. | Policy-configured D07 publication authority | PublishInternalAssessmentBlueprint authority. | Internal Assessment |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Internal Assessment |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Internal Assessment |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Internal Assessment |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A validated Internal Assessment Blueprint reaches Published and becomes the authoritative D07 structure required before any Assessment Window may open.
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
**Validation Basis:** Section D canonical register; `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
