# Physical Campus Boundary Change to Institutional Context Update

# 1. Scenario Metadata

**Scenario ID:** L3-G-318  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows  
**Referenced By:** Authorised institutional consumers requiring current physical-campus context

# 2. Scenario Purpose

Govern an authorised change to the G10-owned Physical Campus boundary so that the changed canonical physical boundary is published while B03 retains InstitutionalCampus identity and governance ownership.

# 3. Business Outcome

The canonical physical boundary change is published while B03 retains InstitutionalCampus governance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a physical campus boundary changes under authorised infrastructure governance.

## 4.2 End Boundary

The scenario ends when the authorised boundary change has been validated and recorded against the canonical G10 Physical Campus, `CampusBoundaryUpdated` has been published by Infrastructure Management, and B03 InstitutionalCampus governance remains unchanged.

## 4.3 In Scope

- Identification of the affected G10 Physical Campus.
- Preservation of the linked B03 InstitutionalCampus identity/governance context.
- Business validation and authorised modification of the Physical Campus boundary.
- Preservation of Physical Campus identity and historical traceability.
- Publication of the exact G10 `CampusBoundaryUpdated` business event.
- Frozen Section H participation as an eligible consumer of the published outcome, without inventing an H-owned boundary fact.

## 4.4 Out of Scope

- Changing InstitutionalCampus identity, institutional status, or Institution-to-Campus association.
- Redefining the physical hierarchy beneath the Physical Campus unless separately authorised.
- Creating a second boundary aggregate or transferring physical ownership to B03 or H.
- Inventing an H-specific business workflow, actor, command, or ownership transfer.
- UI, map, GIS, database, API, or implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the Physical Campus, Campus Boundary, boundary-change validation, historical physical fact and `CampusBoundaryUpdated`. |
| Institution & Structure / Section B | Owns InstitutionalCampus identity, institutional status, Institution association and institutional governance. |
| Institutional Services & Intelligence / Section H | Frozen participating section that may consume the published physical-boundary fact; the exact H bounded context is not identified by the reviewed locked sources. |

Cross-domain participation does not transfer Physical Campus ownership from G10 or InstitutionalCampus governance from B03.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates or records an authorised Physical Campus boundary change within Infrastructure scope. | Must satisfy applicable G10 and enterprise authorization rules. |
| Campus Administrator | Participates where the boundary change falls within authorised campus scope. | Does not acquire B03 institutional-governance ownership. |
| Auditor | Reviews preserved change history and evidence where authorised. | Read-only evidence role; audit ownership remains Section A. |

No Section H business actor is defined for this boundary-change consumption and none is invented.

# 7. Preconditions and Entry Criteria

- The affected Physical Campus exists under G10 ownership.
- The Physical Campus retains its canonical identity throughout the boundary change.
- The corresponding B03 InstitutionalCampus reference is resolvable where institutional context is required.
- The requested boundary change is authorised and satisfies applicable G10 business validation.

# 8. Trigger

A physical campus boundary changes under authorised infrastructure governance

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical G10 Physical Campus whose physical boundary is changing. | Infrastructure Management | The authoritative physical aggregate is established. |
| 2 | Resolve the linked B03 InstitutionalCampus identity without modifying its institutional lifecycle or governance. | Infrastructure Management / Institution & Structure | Institutional and physical concepts remain separated. |
| 3 | Validate the requested boundary change against applicable Infrastructure identity, hierarchy, policy and authorization constraints. | Infrastructure Management | Invalid or unauthorised changes do not proceed. |
| 4 | Record the changed Campus Boundary while preserving Physical Campus identity and prior history. | Infrastructure Management | The canonical current physical boundary is updated with historical traceability. |
| 5 | Publish `CampusBoundaryUpdated` as the completed G10 business fact. | Infrastructure Management | The canonical physical boundary change is published. |
| 6 | Allow authorised Section H consumption of the published fact only through the source-owned outcome; no H ownership of the boundary is created. | Institutional Services & Intelligence / Infrastructure Management | Frozen H participation is preserved without ownership transfer. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Boundary change fails validation | The requested physical change violates applicable G10 constraints. | Reject the change and preserve the current boundary. | No boundary event is published. | Infrastructure Management |
| Institutional governance change is also required | The business need includes a B03-owned institutional status, identity or governance change. | Complete the physical-boundary scenario independently and use the applicable B03 workflow for the institutional change. | Canonical ownership remains separated. | Infrastructure Management / Institution & Structure |
| No H consumer acts on the event | No authorised H capability requires the changed boundary at that time. | The G10 publication remains complete; no H workflow is invented. | Frozen outcome remains satisfied. | Infrastructure Management |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Physical Campus reference is unresolved | The boundary cannot be changed canonically. | Resolve the G10 aggregate before a later authorised change attempt. | Infrastructure Management |
| Authorization or policy validation fails | The boundary change is not permitted. | Preserve the current authoritative boundary. | Infrastructure Management / Platform authorization capability |
| B03 and G10 Campus concepts are conflated | Ownership and lifecycle could be corrupted. | Preserve the B03 InstitutionalCampus reference and G10 Physical Campus as distinct canonical concepts. | Institution & Structure / Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 is the canonical owner of physical institutional Infrastructure, including the Physical Campus site boundary. | Infrastructure Management | Boundary changes remain G10-owned. | L2-G10 §§1–8, 17A.105 |
| B03 owns the institutional Campus identity and governance while G10 owns the physical Campus. | Institution & Structure / Infrastructure Management | The boundary update cannot alter B03 ownership. | L2-B03 §§1–4 |
| Physical-site boundary events are published by G10, not B03. | Infrastructure Management | `CampusBoundaryUpdated` has one canonical publisher. | L2-G10 §17A.105 |
| Infrastructure identity remains immutable and historical integrity must be preserved. | Infrastructure Management | Boundary modification does not create a new Physical Campus identity by default. | L2-G10 §§6, 8, 47 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Campus / Campus Boundary | Existing source-defined state permitting modification | Boundary value is changed; physical identity is preserved. | Current boundary reflects the authorised change; lifecycle is unchanged unless a separate lifecycle action occurs. | Infrastructure Management | L2-G10 §§8.1, 17A.105 |
| InstitutionalCampus | Existing B03 lifecycle state | Referenced for institutional context only. | Unchanged by this physical-boundary scenario. | Institution & Structure | L2-B03 §§1–4 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `CampusBoundaryUpdated` | Infrastructure Management | Authorised enterprise consumers, including the frozen Section H participant where applicable | Publishes the completed canonical change to the Physical Campus boundary. | L2-G10 §17A.105 |

B03 does not publish this physical-boundary event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

B03 provides the authoritative InstitutionalCampus identity/governance reference used by the G10 Physical Campus. G10 publishes the physical-boundary fact. The reviewed locked sources do not identify which specific Section H bounded context must consume `CampusBoundaryUpdated`, so no H-specific contract is invented.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate/apply Physical Campus boundary change | Infrastructure Administrator or Campus Administrator within authorised scope | Must satisfy applicable G10 validation and enterprise authorization. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal boundary-change approver or separation-of-duty rule is defined in the reviewed locked sources. None is invented.

## 15.3 Business Audit and Evidence Obligations

The prior boundary, changed boundary, affected Physical Campus identity, business reason, actor/authority context and resulting published fact must remain historically traceable. Audit evidence does not transfer ownership of the physical boundary from G10.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical Physical Campus is identified.
- The boundary change is authorised and validated.
- The changed physical boundary is recorded under G10 ownership with prior history preserved.
- `CampusBoundaryUpdated` is published by Infrastructure Management.
- B03 InstitutionalCampus identity/governance remains unchanged.

## 16.2 Postconditions

- G10 holds the authoritative current physical boundary.
- B03 remains the authoritative institutional-campus owner.
- Authorised consumers may use the published boundary-change fact without becoming the boundary owner.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Section H is frozen as a participant, but the reviewed locked sources do not identify the exact H bounded context or mandatory H-specific consumption contract for `CampusBoundaryUpdated`. | The G10 publication outcome is fully source-supported, but H's concrete business response cannot be described without invention. | Upstream architecture may identify the intended H consumer/contract if later L3/L4 behaviour requires it. |

**Architecture assessment:** `PASS WITH ISSUES`

The frozen boundary-change and publication outcome is source-grounded by G10's exact `CampusBoundaryUpdated` event. The remaining H-consumer ambiguity does not prevent completion of the G10-owned business outcome.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
