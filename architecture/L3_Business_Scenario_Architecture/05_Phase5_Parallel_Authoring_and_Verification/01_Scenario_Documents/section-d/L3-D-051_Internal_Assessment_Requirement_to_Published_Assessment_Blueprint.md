# Internal Assessment Requirement to Published Assessment Blueprint

# 1. Scenario Metadata

**Scenario ID:** L3-D-051  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D  
**Scenario Type:** Simple  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
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

The blueprint reaches Published and `InternalAssessmentBlueprintPublished` is published by D07.

## 4.3 In Scope

- Reference authoritative Course Offering, curriculum and calendar context.
- Define assessment components, weightage and policy binding.
- Validate completeness and authorize publication under institutional policy.
- Publish the governed blueprint.

## 4.4 Out of Scope

- Course or Course Offering creation.
- Assessment-window execution.
- Examination result or grade computation.
- Command/API/permission design.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where completed business facts cross this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns InternalAssessmentBlueprint, component structure, weightage, policy binding and publication. |
| D01 / Section B | Own authoritative Course Offering, curriculum and calendar references consumed by D07. |

# 6. Business Actors, Roles and Responsibilities

The locked L2-D07 source explicitly records that its required Actors section is absent. This L3 scenario therefore does not invent a blueprint administrator or publication authority. Exact institutional business-role mapping is recorded as an open issue in Section 17.

# 7. Preconditions and Entry Criteria

- Published Course Offering and applicable curriculum/calendar references exist.
- Assessment-cycle policy and required component information are available.

# 8. Trigger

An authorized academic requirement calls for an internal-assessment structure.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish a Draft blueprint for the governed assessment cycle. | D07 | Draft blueprint exists. |
| 2 | Define components, weightage and policy binding. | D07 under applicable institutional authority | Blueprint structure is complete. |
| 3 | Validate source references, component completeness, weightage and policy. | D07 | Blueprint is valid or returned for correction. |
| 4 | Obtain the required institutional publication decision. | Applicable business authority — role unresolved upstream | Publication is approved or rejected. |
| 5 | Publish the approved blueprint. | D07 | Blueprint becomes Published and `InternalAssessmentBlueprintPublished` is published. |

# 10. Alternate Business Flows

No distinct alternate path is asserted. Validation or publication rejection blocks the Published outcome and is handled under Section 11.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Source reference, component or weightage validation fails | Blueprint cannot be published. | Correct the business definition/evidence and resubmit for validation. | D07 |
| Required publication authority is unavailable or rejects publication | Blueprint remains unpublished. | Retain the current governed state and follow the institutional review/resubmission path. | D07 |
| Blueprint preparation is interrupted | Published outcome has not occurred. | Preserve the last valid business position and continue when the blocking condition is resolved. | D07 |

# 12. Business Rules and Decision Constraints

- D07 owns the InternalAssessmentBlueprint lifecycle. Source: L2-D07 §§1–6.
- A blueprint must be Published before an Assessment Window can open. Source: L2-D07 §5.
- D07 does not own Course Offering, curriculum/calendar or examination result facts. Source: L2-D07 §§2,9.
- Published/historical assessment facts remain governed and auditable; no local rule is invented.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Internal Assessment Blueprint | Draft | Define, validate and publish assessment structure. | Draft → Published | D07 | L2-D07 §§5–8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `InternalAssessmentBlueprintPublished` | D07 | Assessment Window, Academic Monitoring and authorized consumers | Records governed blueprint publication. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

D01/Section B provides authoritative Course Offering, curriculum and calendar context to D07 without transferring ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The exact institutional business roles that prepare and authorize publication are not defined in locked L2-D07. D07 owns the business actions; role mapping must be established upstream before this scenario can be Architecture Approved.

## 15.2 Separation of Duties, Overrides and Exceptions

Where institutional policy separates blueprint preparation, validation and publication approval, those authorities must remain distinct. L3 does not define technical RBAC or command permissions.

## 15.3 Business Audit and Evidence Obligations

Retain blueprint version, source references, component/weightage definition, policy binding, validation evidence, publication decision and `InternalAssessmentBlueprintPublished` fact.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The blueprint is valid and Published under applicable institutional authority.
- `InternalAssessmentBlueprintPublished` is published by D07.
- Required business evidence and decision history are retained.

## 16.2 Postconditions

- D07 retains blueprint ownership.
- Upstream source domains retain ownership of referenced academic facts.
- A governed Assessment Window may be established only after publication.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-D07 explicitly records that its Actors section is absent. | L3 cannot safely name a canonical blueprint administrator/publication authority or use L2 command names as permissions. | Define or approve the D07 institutional actor/authority mapping in upstream architecture/governance before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Commands, APIs, technical RBAC, persistence and implementation recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards.
