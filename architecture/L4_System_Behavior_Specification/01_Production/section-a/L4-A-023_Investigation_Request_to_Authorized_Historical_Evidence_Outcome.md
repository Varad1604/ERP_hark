# Investigation Request to Authorized Historical Evidence Outcome

## 1. Specification Metadata

**Specification ID:** L4-A-023  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Audit Infrastructure  
**Realizes Scenario:** L3-A-023  
**Primary Business Owner:** Audit Infrastructure  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-023_Investigation_Request_to_Authorized_Historical_Evidence_Outcome.md`; `L2-A07_Audit_Infrastructure_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L3-A-022_Canonical_Business_Event_to_Immutable_Audit_Evidence_Availability.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall reconstruct an authorized historical business activity from canonical Audit Evidence references, assemble an immutable investigation-scoped Evidence Package and required derived views, and make a read-only Investigation View available without duplicating or modifying canonical evidence or source-domain facts.

## 3. Required Business Context

- Legitimate investigation purpose, requester identity and Tenant/institution scope.
- Authoritative investigation-access and scope decision.
- Relevant immutable Audit Evidence, including restored availability where separately required.
- Evidence relationships, correlation/causation and classification context.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-023-BH-001 | The system shall prevent investigation retrieval unless requester identity, authorization, Tenant/institution, classification, purpose and requested scope are valid. | L3-A-023 §§7, 9, 11, 15 |
| L4-A-023-BH-002 | Within the authorized scope, the system shall locate canonical Audit Evidence by reference using permitted entity, actor, workflow, time, correlation and causation relationships. | L3-A-023 §§9, 12–14 |
| L4-A-023-BH-003 | When chronology is required, the system shall retrieve or generate a Timeline Projection derived from canonical evidence without giving the projection authority over that evidence. | L3-A-023 §§9–10, 12–13 |
| L4-A-023-BH-004 | The system shall resolve required correlation and direct causation relationships without modifying their canonical historical meaning. | L3-A-023 §§9, 12 |
| L4-A-023-BH-005 | The system shall assemble an immutable investigation-scoped Evidence Package containing canonical evidence references rather than duplicated evidence records. | L3-A-023 §§2–4, 9, 12–13, 16 |
| L4-A-023-BH-006 | When Evidence Package assembly succeeds, the system shall publish `Evidence Package Created`; when a required Timeline Projection is newly generated, it shall publish `Timeline Projection Read Model Generated`. | L3-A-023 §§9, 14 |
| L4-A-023-BH-007 | The system shall construct and make available a read-only Investigation View from the authorized package and derived views without changing the investigated domain’s business facts. | L3-A-023 §§3–5, 9, 12, 16 |

## 5. Cross-Domain & Event Behavior

`Evidence Package Created` and `Timeline Projection Read Model Generated` retain Audit Infrastructure as canonical publisher. No canonical Investigation View completion event is defined; L4-A-023 shall not invent one. Investigated domains retain ownership of their business facts and semantics.

## 6. Failure / Rejection / Continuation

Unauthorized scope shall prevent disclosure. Archived evidence shall require separately governed restored access. Missing evidence shall produce a governed incomplete outcome rather than fabricated evidence. A derived view conflicting with canonical evidence shall not be treated as authoritative.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-023-BH-008 | The system shall preserve traceable requester, purpose, authorization, Tenant/institution, scope, evidence references, package/view identities, access and returned outcome. | L3-A-023 §15.3 |

## 8. Observable Completion Conditions

Completion requires valid authorization/scope, relevant canonical evidence references, required derived relationship/timeline views, an immutable Evidence Package and an authorized read-only Investigation View. Original evidence and domain facts remain unchanged.

## 9. Traceability & Provenance

**Parent:** L3-A-023 — Investigation Request to Authorized Historical Evidence Outcome  
**Primary verification source:** L2-A07 — Audit Infrastructure Workflows

## 10. Open Issues / Deferred Design

The parent does not define a universal investigation requestor/approver or Investigation View completion event, and the initiating request boundary remains ambiguous. This draft requires authoritative access/scope decisions without inventing those missing actors/events/contracts.