# Organisation Unit Activation to Operational Availability

## 1. Specification Metadata

**Specification ID:** L4-B-007
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Institution & Structure — Organisation
**Realizes Scenario:** L3-B-007
**Primary Business Owner:** Organisation
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-007_Organisation_Unit_Activation_to_Operational_Availability.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall activate an eligible Organisation Unit to Operational without equating Operational with user authority or global ERP readiness.

The behavior covers the source-supported pre-operational position to Operational; it does not use a Draft-to-Operational shortcut or wait for all downstream consumers.

---

## 3. Required Business Context

- Canonical Organisation Unit in a source-supported pre-operational position.
- Effective Organisation Type Version and applicable parent/structure eligibility.
- Authoritative activation decision/evidence.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-007-BH-001 | The system shall prevent direct Draft-to-Operational activation where the parent requires Planned or Provisioned as the eligible pre-operational position. | L3-B-007 §§7,12-13 |
| L4-B-007-BH-002 | The system shall validate the assigned effective Organisation Type Version before activation. | L3-B-007 §§7,9,12 |
| L4-B-007-BH-003 | The system shall validate applicable parent and structural eligibility before Operational where required. | L3-B-007 §§7,9,12 |
| L4-B-007-BH-004 | The system shall prevent activation when mandatory eligibility or authority evidence is unsatisfactory and retain the current valid state. | L3-B-007 §11 |
| L4-B-007-BH-005 | The system shall make the Organisation Unit Operational only through the source-supported activation path. | L3-B-007 §§3,9,13,16 |
| L4-B-007-BH-006 | When activation conditions are not satisfied, the system shall prevent transition to Operational and retain the last valid source-defined lifecycle position. | L3-B-007 §§11,13 |
| L4-B-007-BH-007 | The system shall make the Operational Organisation Unit fact available for governed downstream use without implying authorization or global readiness. | L3-B-007 §§4,14,16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-007-BH-008 | The system shall publish `OrganizationActivated` when the Organisation Unit successfully becomes Operational; the canonical publisher remains Organisation. | L3-B-007 §14; `L2-B02_Organisation_Workflows.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Failed eligibility, type-version, structural, or authority checks prevent Operational activation and preserve the current authoritative state.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Organisation Unit identity; pre-operational state; effective Organisation Type Version; structural eligibility; activation decision; resulting Operational state; OrganizationActivated event.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The Organisation Unit is Operational through the canonical path.
- OrganizationActivated is produced.
- Downstream processing and authorization/readiness remain separate.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-007 — Organisation Unit Activation to Operational Availability
**Primary verification source:** L2-B02 — Organisation Workflows (`L2-B02_Organisation_Workflows.md`)

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No Draft-to-Operational shortcut or global-readiness semantics are introduced.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
