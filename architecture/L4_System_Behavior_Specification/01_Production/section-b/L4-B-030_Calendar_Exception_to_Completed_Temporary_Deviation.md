# Calendar Exception to Completed Temporary Deviation

## 1. Specification Metadata

**Specification ID:** L4-B-030
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Institution & Structure — Academic Calendar
**Realizes Scenario:** L3-B-030
**Primary Business Owner:** Academic Calendar
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-02
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-030_Calendar_Exception_to_Completed_Temporary_Deviation.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B05_Academic_Calendar_Workflows.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall govern a temporary Calendar Exception as a separate deviation from the immutable base Calendar Version and ensure its effect ends when the exception expires.

The exception does not rewrite the base Calendar Version. Approved and Applied are distinct; only an Applied exception governs within its authorized effective context.

---

## 3. Required Business Context

- Authoritative base Calendar Version.
- Calendar Exception identifying affected rule, scope, owner, effective period/expiry, and required evidence.
- Existing exceptions that may overlap the same rule/scope/time.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-030-BH-001 | The system shall require each Calendar Exception to identify its base Calendar Version, affected rule, governed scope, owner, and expiry/effective context. | L3-B-030 §§7,12 |
| L4-B-030-BH-002 | The system shall preserve the base Calendar Version unchanged while the exception is proposed, approved, applied, or expired. | L3-B-030 §§4,12-13 |
| L4-B-030-BH-003 | The system shall prevent an Approved exception from affecting the governing result before it becomes Applied at the authorized effective context. | L3-B-030 §§9,12-13 |
| L4-B-030-BH-004 | The system shall prevent ambiguous overlapping exceptions for the same rule/scope/effective period unless authoritative precedence/resolution exists. | L3-B-030 §§9,11-12 |
| L4-B-030-BH-005 | The system shall make the exception Applied only after required validation, authority, and effective conditions are satisfied. | L3-B-030 §§3,9,13,16 |
| L4-B-030-BH-006 | The system shall end the exception's active governing effect when it becomes Expired while preserving historical evidence/linkage. | L3-B-030 §§11-13,15-16 |

---

## 5. Cross-Domain & Event Behavior

No additional exact canonical public event is established by the approved parent for this outcome. The system shall expose only the source-supported authoritative business fact or governed hand-off required by the scenario, without inventing an event name or publisher.

---

## 6. Failure / Rejection / Continuation

- Missing required exception evidence/authority, unresolved overlap, or invalid effective context blocks application; the base Calendar result remains authoritative except for other valid applied exceptions.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Exception identity; base Calendar Version; affected rule/scope; owner; approval/application basis; effective period/expiry; overlap resolution; resulting temporary deviation and history.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The temporary deviation has been validly Applied for its authorized period and ceases active effect at expiry, with the base Calendar Version unchanged and history preserved.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-030 — Calendar Exception to Completed Temporary Deviation
**Primary verification source:** L2-B05 — Academic Calendar Workflows (`L2-B05_Academic_Calendar_Workflows.md`)

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No specific canonical initiation/final-decision actor name, precedence formula, or exact public exception event is invented where the approved parent does not establish it.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
