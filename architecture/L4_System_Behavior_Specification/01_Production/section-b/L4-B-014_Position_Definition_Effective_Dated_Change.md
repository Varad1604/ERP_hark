# Position Definition Effective Dated Change

## 1. Specification Metadata

**Specification ID:** L4-B-014  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Organisation  
**Realizes Scenario:** L3-B-014  
**Primary Business Owner:** Organisation  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-014_Position_Definition_Effective_Dated_Change.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall apply an effective-dated PositionDefinition change without overwriting the Active predecessor or re-owning PositionAssignments.

The scenario changes the PositionDefinition structure; Section E owns assignment resolution and occupant changes.

---

## 3. Required Business Context

- Active PositionDefinition and authoritative current baseline.
- Approved effective-dated successor/change proposal.
- Material impact evidence, including Section E-owned assignment context where relevant.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-014-BH-001 | The system shall identify the authoritative active PositionDefinition baseline before applying a material change. | L3-B-014 §§7,9,12 |
| L4-B-014-BH-002 | The system shall prevent direct destructive overwrite of an Active PositionDefinition for a material effective-dated change. | L3-B-014 §§9,12-13 |
| L4-B-014-BH-003 | The system shall keep the current PositionDefinition authoritative until the approved successor reaches its permitted effective point. | L3-B-014 §§9,12,16 |
| L4-B-014-BH-004 | The system shall preserve predecessor/successor PositionDefinition lineage and historical interpretation. | L3-B-014 §§11-13,15-16 |
| L4-B-014-BH-005 | The system shall prevent historical PositionAssignments from being rewritten to the new PositionDefinition merely because the definition changes. | L3-B-014 §§4-5,12 |
| L4-B-014-BH-006 | The system shall apply the PositionDefinition change without automatically reassigning existing occupants or modifying Section E-owned PositionAssignments. | L3-B-014 §§4-5,14 |
| L4-B-014-BH-007 | The system shall prevent stale approved changes from being applied when a newer authoritative baseline invalidates them. | L3-B-014 §11 |

---

## 5. Cross-Domain & Event Behavior

No additional exact canonical public event is established by the approved parent for this outcome. The system shall expose only the source-supported authoritative business fact or governed hand-off required by the scenario, without inventing an event name or publisher.

---

## 6. Failure / Rejection / Continuation

- Invalid or stale change proposals leave the current PositionDefinition authoritative; Section E-owned assignment issues are exposed for governed resolution rather than modified locally.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: PositionDefinition identity; predecessor/successor representations; effective date; validation/decision basis; material assignment-impact evidence; preserved lineage.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The successor PositionDefinition is effective at the permitted point.
- The predecessor and historical PositionAssignments remain resolvable without ownership transfer.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-014 — Position Definition Effective Dated Change  
**Primary verification source:** L2-B02 — Organisation Workflows (`L2-B02_Organisation_Workflows.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No exact canonical PositionDefinition-change event is established; none is invented.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
