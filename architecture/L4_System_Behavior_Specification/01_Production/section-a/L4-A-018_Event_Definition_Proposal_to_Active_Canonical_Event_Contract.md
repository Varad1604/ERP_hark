# Event Definition Proposal to Active Canonical Event Contract

## 1. Specification Metadata

**Specification ID:** L4-A-018  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Event Infrastructure  
**Realizes Scenario:** L3-A-018  
**Primary Business Owner:** Event Infrastructure  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-018_Event_Definition_Proposal_to_Active_Canonical_Event_Contract.md`; `L2-A06_Event_Infrastructure_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern a proposed completed business fact into one active canonical Event Definition, initial Event Version, Schema and Event Contract while preserving the Event-Owning Domain’s exclusive ownership of the business fact and semantics. Event Instance publication and delivery are outside this specification.

## 3. Required Business Context

- A completed business fact with exactly one authoritative Event-Owning Domain.
- Proposed canonical name, classification, schema, metadata, compatibility and security context.
- Event governance actors and policies identified by the parent.
- Canonical definition/version lifecycle culminating in `Active`.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-018-BH-001 | The system shall reject a proposal that does not represent a completed business fact, lacks exactly one authoritative owner, or duplicates an existing canonical definition. | L3-A-018 §§7, 9, 11–12 |
| L4-A-018-BH-002 | When eligible, the system shall establish the Event Definition in `Draft` with canonical name, classification, ownership and business meaning without transferring fact ownership to Event Infrastructure. | L3-A-018 §§5, 9, 12–13 |
| L4-A-018-BH-003 | The system shall associate the initial Event Schema, Version and Contract and validate required metadata, compatibility, security, Tenant and ownership constraints. | L3-A-018 §§9, 12–13 |
| L4-A-018-BH-004 | The system shall prevent approval/activation until required architectural, business, security, Tenant and compatibility review succeeds. | L3-A-018 §§9, 11–12, 15 |
| L4-A-018-BH-005 | When governance approval succeeds, the system shall preserve the approved schema/version as immutable and establish the authoritative registry entry. | L3-A-018 §§9, 12, 16 |
| L4-A-018-BH-006 | The system shall prevent runtime publication eligibility until the approved Event Definition and default Event Version reach `Active`. | L3-A-018 §§4, 9, 12–13, 16 |
| L4-A-018-BH-007 | When activation succeeds, the system shall make the active canonical Event Contract discoverable to authorized publishers and consumers. | L3-A-018 §§3–4, 9, 14, 16 |
| L4-A-018-BH-008 | The system shall prevent in-place modification of an activated schema/version; governed structural evolution shall use a new version. | L3-A-018 §§10, 12, 16 |

## 5. Cross-Domain & Event Behavior

L2-A06 and the Locked parent define no exact canonical business-event names for Event Definition/Version lifecycle transitions. The system shall not invent lifecycle events for Draft, review, approval, registration or activation. The active registry contract is the observable cross-domain availability outcome.

## 6. Failure / Rejection / Continuation

Invalid completed-fact semantics, disputed ownership, duplicate naming, failed schema/security/compatibility review, denied governance approval or unauthorized activation shall keep the definition outside `Active` and unavailable for canonical publication.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-018-BH-009 | The system shall preserve traceable evidence of canonical name, owner, business meaning, schema/version/contract, review outcomes, approval, registry identity, activation and lifecycle history. | L3-A-018 §15.3 |

## 8. Observable Completion Conditions

Completion requires an authoritative registry entry with Event Definition and default Event Version `Active`, an approved immutable Schema/Contract, discoverability for authorized use, and retained Event-Owning Domain ownership. No Event Instance is published.

## 9. Traceability & Provenance

**Parent:** L3-A-018 — Event Definition Proposal to Active Canonical Event Contract  
**Primary verification source:** L2-A06 — Event Infrastructure Workflows

## 10. Open Issues / Deferred Design

The parent records inconsistent `Review`/`Under Review` wording and missing canonical lifecycle commands/event names. This draft avoids depending on those absent contracts and does not invent lifecycle events.