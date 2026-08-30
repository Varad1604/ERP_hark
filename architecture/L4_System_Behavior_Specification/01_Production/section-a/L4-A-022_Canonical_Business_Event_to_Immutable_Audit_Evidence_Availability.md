# Canonical Business Event to Immutable Audit Evidence Availability

## 1. Specification Metadata

**Specification ID:** L4-A-022  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Audit Infrastructure  
**Realizes Scenario:** L3-A-022  
**Primary Business Owner:** Audit Infrastructure  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-022_Canonical_Business_Event_to_Immutable_Audit_Evidence_Availability.md`; `L2-A07_Audit_Infrastructure_Workflows.md`; `L2-A06_Event_Infrastructure_Workflows.md`; `L3-A-019_Committed_Business_Fact_to_Reliable_Consumer_Delivery.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall convert one audit-eligible canonical completed-fact event into one complete immutable Audit Evidence aggregate with governed context, actor attribution, logical change attribution, correlation, causation and classification while preserving the originating domain’s business ownership.

## 3. Required Business Context

- One immutable canonical auditable event.
- Tenant and applicable institutional/workflow/session/source/security references.
- Actor/delegation and affected-entity references.
- Evidence Generation and Classification Policy outcomes.
- Correlation and, where applicable, direct causation context.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-022-BH-001 | The system shall prevent Audit Evidence creation unless the source event is canonical, represents a completed activity and is audit-eligible under the applicable policy. | L3-A-022 §§7, 9, 11–12 |
| L4-A-022-BH-002 | The system shall resolve the authoritative Tenant, institution, workflow, session, source/channel and security references required by the evidence context without re-owning those facts. | L3-A-022 §§9, 12, 14 |
| L4-A-022-BH-003 | The system shall attribute the accountable actor using authoritative identity, actor-type, delegation, authentication and authorization references. | L3-A-022 §§6, 9, 12, 15 |
| L4-A-022-BH-004 | The system shall record affected business-entity references and permitted logical changes without duplicating complete source-domain aggregates. | L3-A-022 §§9, 11–12 |
| L4-A-022-BH-005 | The system shall associate the authoritative correlation identifier and at most one direct causation reference where applicable. | L3-A-022 §§9–10, 12 |
| L4-A-022-BH-006 | The system shall apply the governed Audit Classification without transferring policy ownership from Authorization & Policy. | L3-A-022 §§9, 12, 15 |
| L4-A-022-BH-007 | When all required evidence components are complete, the system shall create one immutable Audit Evidence aggregate and prevent duplicate evidence creation for the same source event. | L3-A-022 §§9, 11–13, 16 |
| L4-A-022-BH-008 | After complete evidence creation, the system shall publish `Audit Evidence Generated` on behalf of Audit Infrastructure. | L3-A-022 §§9, 14, 16 |
| L4-A-022-BH-009 | The system shall make the immutable evidence available for authorized investigation and eligible timeline projection without changing source business ownership. | L3-A-022 §§3–5, 9, 14, 16 |

## 5. Cross-Domain & Event Behavior

The source canonical event retains its Event-Owning Domain as business-fact owner and Event Infrastructure as distributor. `Audit Evidence Generated` is published by Audit Infrastructure after complete evidence creation. No source aggregate or event is modified.

## 6. Failure / Rejection / Continuation

Invalid audit eligibility, missing Tenant/context/actor references, prohibited full-aggregate duplication, duplicate evidence identity or incomplete evidence shall prevent evidence completion and `Audit Evidence Generated` publication. No partial evidence is treated as complete.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-022-BH-010 | The system shall preserve immutable evidence identity, source event, context, actor/delegation, affected references, logical changes, correlation, causation, classification and policy references sufficient for historical reconstruction. | L3-A-022 §§15–16 |

## 8. Observable Completion Conditions

Completion requires one complete immutable Audit Evidence aggregate, `Audit Evidence Generated`, authorized evidence availability, and preservation of source-domain business ownership and original event immutability.

## 9. Traceability & Provenance

**Parent:** L3-A-022 — Canonical Business Event to Immutable Audit Evidence Availability  
**Primary verification source:** L2-A07 — Audit Infrastructure Workflows

## 10. Open Issues / Deferred Design

No L4 correctness issue remains. Persistence, storage, query mechanisms and UI remain deferred.