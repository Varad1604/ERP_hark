# Institutional Campus Activation Decision

## 1. Specification Metadata

**Specification ID:** L4-B-018
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Institution & Structure — Institutional Campus
**Realizes Scenario:** L3-B-018
**Primary Business Owner:** Institutional Campus
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-018_Institutional_Campus_Activation_Decision.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall activate an Approved InstitutionalCampus to Operational when B03 requirements and applicable source-owned readiness evidence are satisfactory.

Activation is B03-owned. G10 physical readiness may provide evidence but cannot itself make the InstitutionalCampus Operational, and Operational does not imply academic scheduling/global readiness.

---

## 3. Required Business Context

- Canonical InstitutionalCampus in Approved.
- Mandatory B03 Campus metadata and applicable authoritative readiness evidence, including G10 facts where required.
- Authoritative Campus activation decision.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-018-BH-001 | The system shall require the InstitutionalCampus to be Approved before Operational activation and prevent a Proposed-to-Operational shortcut. | L3-B-018 §§7,12-13 |
| L4-B-018-BH-002 | The system shall validate required B03 Campus metadata before activation. | L3-B-018 §§7,9,12 |
| L4-B-018-BH-003 | Where the parent requires physical-site readiness evidence, the system shall use the authoritative G10 evidence as an input to the B03 Campus activation decision. | L3-B-018 §§5,7,12,14 |
| L4-B-018-BH-004 | The system shall prevent activation when required G10 or other readiness evidence is missing, unsatisfactory, or conflicting. | L3-B-018 §§9,11 |
| L4-B-018-BH-005 | The system shall make the Campus Operational only after the B03-owned authoritative activation decision becomes effective. | L3-B-018 §§3,9,13,16 |
| L4-B-018-BH-006 | The system shall prevent Campus transition to Operational when the B03 activation conditions are not satisfied, even if G10 physical-site availability exists. | L3-B-018 §§5,12,14 |
| L4-B-018-BH-007 | The system shall expose Operational as the B03 Campus lifecycle result only and shall not derive academic scheduling or universal capacity readiness from that state alone. | L3-B-018 §§4,12,16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-018-BH-008 | The system shall publish `CampusActivated` when the InstitutionalCampus successfully becomes Operational; the canonical publisher remains Institutional Campus. | L3-B-018 §14; `L2-B03_Campus_Workflows.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Missing/unsatisfactory Campus metadata, readiness evidence, or activation authority blocks activation and preserves the current valid state.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Campus identity; Institution association; Approved state; B03 metadata; material G10/source-owned readiness evidence; activation decision; resulting Operational state; CampusActivated event.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The InstitutionalCampus is Operational through the Approved-to-Operational path.
- CampusActivated is produced.
- No global capacity/scheduling readiness is implied.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-018 — Institutional Campus Activation Decision
**Primary verification source:** L2-B03 — Campus Workflows (`L2-B03_Campus_Workflows.md`)

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No ownership transfer to G10 or downstream readiness domain is permitted.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
