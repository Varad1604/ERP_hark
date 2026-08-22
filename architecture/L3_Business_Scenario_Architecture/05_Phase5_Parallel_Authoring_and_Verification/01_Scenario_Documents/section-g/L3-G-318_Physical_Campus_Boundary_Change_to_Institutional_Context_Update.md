# Physical Campus Boundary Change to Institutional Context Update

# 1. Scenario Metadata

**Scenario ID:** L3-G-318  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, B, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-B03 — Campus Workflows  
**Referenced By:** Institutional consumers requiring current physical-campus context

# 2. Scenario Purpose

Govern an authorised change to the G10-owned Physical Campus boundary while preserving the separate B03-owned InstitutionalCampus identity and governance context and making the resulting authoritative physical boundary available through an approved publication contract.

# 3. Business Outcome

The canonical physical boundary change is published while B03 retains InstitutionalCampus governance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a physical campus boundary changes under authorised infrastructure governance.

## 4.2 End Boundary

The scenario ends when the changed G10-owned Physical Campus boundary is authoritatively recorded and published without changing B03 InstitutionalCampus ownership or governance.

## 4.3 In Scope

- Identification of the affected G10 Physical Campus.
- Validation and authorised modification of the Physical Campus boundary.
- Preservation of Physical Campus identity and historical traceability.
- Preservation of the B03 InstitutionalCampus identity/governance relationship.
- Publication of the changed physical boundary through a canonical G10 business event or contract.

## 4.4 Out of Scope

- Changing InstitutionalCampus identity, institutional status, or Institution-to-Campus governance.
- Redefining the physical hierarchy beneath the Physical Campus unless separately authorised.
- Inventing a boundary-specific event, read contract, or H-domain consumer contract.
- Dashboard, notification, search, or other H-domain behaviour not defined by an authoritative hand-off for this boundary change.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the Physical Campus, Campus Boundary, physical hierarchy, boundary change, and resulting authoritative physical fact. |
| Institution & Structure / Section B | Owns InstitutionalCampus identity, institutional governance, and the institutional association referenced by the Physical Campus. |
| Institutional Services & Intelligence / Section H | Frozen participating section for consumption of the published outcome; the reviewed upstream sources do not identify the exact H bounded context or mandatory contract for this boundary change. |

Participation does not transfer Physical Campus ownership from G10 or InstitutionalCampus governance from B03.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates or performs the authorised Physical Campus boundary update. | Acts only within authorised Infrastructure scope and G10 business rules. |
| Campus Administrator | Participates where the change falls within the administrator's authorised campus scope. | Does not acquire B03 InstitutionalCampus governance ownership. |
| Auditor | Reviews the preserved history and evidence of the boundary change. | Read-only audit participation. |

No H-domain business actor for boundary-change consumption is defined in the reviewed locked sources.

# 7. Preconditions and Entry Criteria

- The affected Physical Campus exists under G10 ownership.
- The applicable InstitutionalCampus reference exists where institutional context is required.
- The requested boundary change is authorised and satisfies applicable G10 business validation.
- Physical Campus identity remains unchanged by the boundary modification.

# 8. Trigger

A physical campus boundary changes under authorised infrastructure governance

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the canonical G10 Physical Campus whose boundary requires change. | Infrastructure Management | The authoritative physical aggregate is selected. |
| 2 | Confirm the related B03 InstitutionalCampus reference without modifying its institutional governance. | Infrastructure Management / Institution & Structure | Institutional and physical ownership remain separated. |
| 3 | Validate the requested boundary change against Infrastructure identity, hierarchy, policy, and authorization constraints. | Infrastructure Management | An invalid or unauthorised change is prevented. |
| 4 | Record the approved change to the Physical Campus boundary while preserving prior values and historical traceability. | Infrastructure Management | The current canonical physical boundary is updated. |
| 5 | Publish the changed physical boundary through the authoritative G10 publication mechanism required by the frozen outcome. | Infrastructure Management | **Blocked:** the reviewed locked G10 sources do not define a boundary-bearing event or public contract sufficient to prove this outcome. |

The scenario cannot complete its frozen publication outcome without an upstream definition of the boundary publication contract.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Boundary change is rejected | Business validation or authorization fails. | Preserve the existing Physical Campus boundary and its history. | No canonical boundary change occurs. | Infrastructure Management |
| Institutional governance change is also requested | The request includes a B03-owned InstitutionalCampus governance change. | Keep the G10 boundary change separate and route the institutional matter to the B03-owned lifecycle/governance process. | Ownership remains canonical. | Infrastructure Management / Institution & Structure |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Physical Campus cannot be resolved | The boundary cannot be changed authoritatively. | Correct the Infrastructure reference before any boundary update. | Infrastructure Management |
| Boundary publication contract is unavailable | Consumers cannot be guaranteed the authoritative changed boundary required by the frozen outcome. | Preserve the governed G10 change but do not claim scenario completion until the upstream publication contract is defined. | Architecture governance / Infrastructure Management |
| B03 and G10 identities are confused | Ownership could be transferred or duplicated incorrectly. | Preserve the B03 InstitutionalCampus reference and G10 Physical Campus as distinct canonical concepts. | Institution & Structure / Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G10 is the canonical owner of physical institutional infrastructure. | Infrastructure Management | The Physical Campus boundary change remains G10-owned. | L2-G10 §§1–6 |
| Campus Boundary is a child entity of the G10 Physical Campus aggregate. | Infrastructure Management | Boundary change does not create a new aggregate or transfer ownership. | L2-G10 §8.1 |
| Infrastructure identity is immutable and structural changes preserve historical integrity and publish domain events. | Infrastructure Management | Boundary changes must preserve identity/history and require publication. | L2-G10 §§18, 47 |
| B03 owns InstitutionalCampus identity and governance; physical infrastructure is delegated to G10. | Institution & Structure | Institutional governance remains unchanged by the physical boundary update. | L2-B03 §§1–4 |
| Consumers shall use supported G10 contracts/events and shall not infer undocumented business meaning. | Infrastructure Management | L3 cannot invent a boundary-bearing publication contract. | L2-G10 §44A.199 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Physical Campus | Existing source-defined lifecycle state permitting modification | Campus Boundary is changed; aggregate identity is preserved. | Lifecycle remains unchanged unless a separate authorised lifecycle transition is required. | Infrastructure Management | L2-G10 §§8.1, 18 |
| InstitutionalCampus | Existing B03 state | Referenced for institutional context only. | Unchanged. | Institution & Structure | L2-B03 |
| H-domain projection/consumer | Not defined for this boundary hand-off | May consume only an approved published boundary fact. | Cannot be asserted until the contract is defined. | Institutional Services & Intelligence | Frozen participation; upstream gap |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 defines `CampusUpdated` for general Campus updates and requires structural changes to publish canonical events. However, the reviewed sources do not establish that `CampusUpdated` carries the Physical Campus Boundary, nor do they define a canonical `CampusBoundaryChanged` event. No boundary-specific event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The G10 Physical Campus Summary contract exposes identity, code, name, Institution reference, lifecycle, operational status, address and geo coordinates, but the reviewed contract does not expose the Campus Boundary. B03 supplies InstitutionalCampus identity/governance context. No authoritative H-domain hand-off for the physical boundary is defined.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Change Physical Campus boundary | Infrastructure Administrator or Campus Administrator within authorised scope | Must satisfy G10 validation and enterprise authorization. | Infrastructure Management |

## 15.2 Separation of Duties, Overrides and Exceptions

No universal boundary-change approval chain or separation-of-duty rule is defined in the reviewed sources. None is invented.

## 15.3 Business Audit and Evidence Obligations

The changed boundary, previous values where applicable, initiating actor, business reason and resulting G10 state change must remain historically traceable under G10 audit obligations.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The canonical Physical Campus is identified.
- The boundary change is authorised, validated and recorded under G10.
- Physical Campus identity and prior history remain traceable.
- B03 retains InstitutionalCampus governance.
- The changed boundary is published through a canonical boundary-bearing event or contract.

The final criterion is not currently supported by an authoritative upstream contract.

## 16.2 Postconditions

- The governed G10 boundary change may be recorded.
- B03 InstitutionalCampus governance remains unchanged.
- The scenario cannot be considered fully complete until the boundary publication mechanism is defined upstream.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 owns `Campus Boundary` and requires structural changes to publish events, but no exact boundary-specific event is defined. `CampusUpdated` is generic and its documented semantics do not establish a boundary payload. | The frozen final outcome specifically requires the canonical physical boundary change to be published; L3 cannot choose or rename an event silently. | Define or explicitly designate the canonical event for Physical Campus boundary changes. |
| The G10 Physical Campus Summary contract does not expose the Campus Boundary. | A consumer cannot be guaranteed access to the changed authoritative boundary through the documented public contract. | Extend or designate an approved G10 business contract that exposes the physical boundary. |
| Section H is frozen as a participant, but no specific H bounded context or mandatory hand-off for the boundary change is identified upstream. | H participation cannot be described beyond generic consumption without invention. | Clarify the intended H consumer and contract if H participation is mandatory to completion. |

**Architecture assessment:** `BLOCKED`

The missing boundary-bearing publication event/contract directly affects the frozen final outcome and mandatory cross-domain hand-off.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions beyond the blocking issues above.
