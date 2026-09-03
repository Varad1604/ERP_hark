# Organisation Unit Registration

## 1. Specification Metadata

**Specification ID:** L4-B-006
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Institution & Structure — Organisation
**Realizes Scenario:** L3-B-006
**Primary Business Owner:** Organisation
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-006_Organisation_Unit_Registration.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B02_Organisation_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall register a canonical Organisation Unit in Draft while preserving Organisation ownership and source-defined structural context.

Registration ends with a Draft Organisation Unit. It does not make the unit Operational; activation belongs to L3-B-007.

---

## 3. Required Business Context

- Active canonical Institution.
- Proposed Organisation Unit identity and applicable Organisation Type Version.
- Authoritative structural/parent context where applicable.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-006-BH-001 | The system shall require an Active Institution before establishing an Organisation Unit. | L3-B-006 §§7,12 |
| L4-B-006-BH-002 | The system shall require an applicable authoritative Organisation Type Version before unit establishment. | L3-B-006 §§7,9,12 |
| L4-B-006-BH-003 | The system shall validate the proposed Organisation Unit business identity and applicable structural context using only the authoritative identity and uniqueness constraints established for that Organisation context. | L3-B-006 §§7,9,12 |
| L4-B-006-BH-004 | The system shall prevent registration when mandatory identity, type-version, or structural evidence is invalid or conflicting. | L3-B-006 §§9,11 |
| L4-B-006-BH-005 | The system shall create the canonical Organisation Unit in Draft only after registration requirements are satisfied. | L3-B-006 §§3,9,13,16 |
| L4-B-006-BH-006 | The system shall expose the registered Organisation Unit as the single authoritative unit master for governed downstream use and reject creation of a competing master for the same canonical unit identity within this scenario. | L3-B-006 §§5,12,14 |
| L4-B-006-BH-007 | The system shall require a parent relationship only when the authoritative organisation model for the proposed unit requires one. | L3-B-006 §§7,12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-006-BH-008 | The system shall publish `OrganizationUnitCreated` after successful canonical Organisation Unit registration; the canonical publisher remains Organisation. | L3-B-006 §14; `L2-B02_Organisation_Workflows.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Invalid Institution, Organisation Type Version, identity, or structural context blocks registration and no canonical Draft unit is established.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Institution context; Organisation Unit identity; Organisation Type Version; structural/parent context where applicable; validation result; resulting Draft unit; OrganizationUnitCreated event.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- A canonical Organisation Unit exists in Draft under Organisation ownership.
- The unit is not yet Operational.
- OrganizationUnitCreated is produced.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-006 — Organisation Unit Registration
**Primary verification source:** L2-B02 — Organisation Workflows (`L2-B02_Organisation_Workflows.md`)

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No universal business uniqueness key or mandatory-parent rule is added beyond authoritative Organisation semantics.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
