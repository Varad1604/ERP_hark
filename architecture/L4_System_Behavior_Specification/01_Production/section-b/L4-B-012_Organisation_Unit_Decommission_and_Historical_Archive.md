# Organisation Unit Decommission and Historical Archive

## 1. Specification Metadata

**Specification ID:** L4-B-012  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Organisation  
**Realizes Scenario:** L3-B-012  
**Primary Business Owner:** Organisation  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-012_Organisation_Unit_Decommission_and_Historical_Archive.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall support Organisation Unit decommissioning followed by historical archival while preserving identity, lineage, and cross-domain ownership boundaries.

The canonical lifecycle sequence is Operational to Decommissioned to Archived. Decommissioning and archival do not destructively cascade into child/downstream aggregates.

---

## 3. Required Business Context

- Canonical Organisation Unit in Operational for decommissioning or Decommissioned for archival as applicable.
- Authoritative decommission/archive decision and required structural/dependency evidence.
- Historical Organisation relationships and downstream references.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-012-BH-001 | The system shall make an eligible Operational Organisation Unit Decommissioned only after required decommissioning evidence and decision are satisfactory. | L3-B-012 §§7,9,13,16 |
| L4-B-012-BH-002 | The system shall prevent new operational activity against a Decommissioned Organisation Unit while preserving historical resolution. | L3-B-012 §§3-4,12-13,16 |
| L4-B-012-BH-003 | When an Organisation Unit is decommissioned, the system shall preserve historical references and leave downstream source-owned facts unchanged. | L3-B-012 §§4-5,12,14 |
| L4-B-012-BH-004 | The system shall prevent decommissioning while authoritative active child or structural relationships remain unresolved. | L3-B-012 §§9,11-12 |
| L4-B-012-BH-005 | The system shall make a Decommissioned Organisation Unit Archived only through the source-supported archival step. | L3-B-012 §§9,13,16 |
| L4-B-012-BH-006 | The system shall make Archived read-only and unavailable for new PositionAssignments or other new operational references. | L3-B-012 §§4,12-13,16 |
| L4-B-012-BH-007 | The system shall preserve historical hierarchy/lineage after decommissioning and archival. | L3-B-012 §§11-12,15-16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-012-BH-008 | The system shall publish `OrganizationArchived` when the Organisation Unit successfully reaches Archived; the canonical publisher remains Organisation. | L3-B-012 §14; `L2-B02_Organisation_Workflows.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Unresolved structural/dependency conditions or missing authority/evidence block the requested transition and preserve the current valid lifecycle position.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Organisation Unit identity; decommission/archive decision basis; structural dependencies and governed resolutions; prior/resulting state; historical lineage; OrganizationArchived event when Archived.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- Decommissioning completes at Decommissioned with historical resolution preserved.
- Archival completes at Archived, read-only, with OrganizationArchived produced.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-012 — Organisation Unit Decommission and Historical Archive  
**Primary verification source:** L2-B02 — Organisation Workflows (`L2-B02_Organisation_Workflows.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No distinct decommissioning event is invented where the approved parent does not establish one.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
