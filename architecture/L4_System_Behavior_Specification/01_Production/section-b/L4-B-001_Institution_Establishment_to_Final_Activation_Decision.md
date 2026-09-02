# Institution Establishment to Final Activation Decision

## 1. Specification Metadata

**Specification ID:** L4-B-001  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Institution & Structure — Institution  
**Realizes Scenario:** L3-B-001  
**Primary Business Owner:** Institution  
**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Pallav  
**Last Updated:** 2026-09-02  
**Depends On:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-b/L3-B-001_Institution_Establishment_to_Final_Activation_Decision.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md`; `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Activation_Authority_Amendment_2026-08-24.md`; `governance/adr/ADR-B-INST-ACT-001.md`  
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall establish the canonical Institution and enforce the governed activation decision so that a valid Institution becomes Active without creating a global operational-readiness state.

The behavior starts with an identifiable authorized Institution establishment/activation request and ends when the Institution-owned activation decision is effective, the Institution is Active, required evidence is attributable, and the canonical activation fact is available. It does not wait for downstream processing or establish Organisation, Campus, Curriculum, Calendar, or capability-readiness outcomes.

---

## 3. Required Business Context

- Existing authoritative Tenant context and Institution-to-Tenant association.
- Canonical Institution identity, Tenant-scoped Institution Code, mandatory master information, and assigned Institution Administrator.
- Permitted Institution lifecycle position and authoritative activation validation/dependency evidence.
- Institution Administrator submission and attributable Institution Activation Authority decision.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-B-001-BH-001 | The system shall require the Institution to resolve to exactly one authoritative Tenant association before establishment can progress and shall leave the source-owned Tenant identity unchanged. | L3-B-001 §§5,7,12,14 |
| L4-B-001-BH-002 | The system shall prevent activation progression when mandatory Institution master information is incomplete, the Institution Administrator is unassigned, the Institution Code is not unique within its Tenant, or another mandatory activation validation fails. | L3-B-001 §§7,9,11-12 |
| L4-B-001-BH-003 | The system shall recognize the Institution Administrator's activation submission without treating submission as activation. | L3-B-001 §§6,9,15 |
| L4-B-001-BH-004 | The system shall recognize only a source-valid Institution Activation Authority decision for final activation progression and preserve Approve, Return, and Reject as distinct governed outcomes. | L3-B-001 §§6,9-12,15 |
| L4-B-001-BH-005 | The system shall prevent silent application of an approved activation when a newer authoritative baseline invalidates the approval before effectiveness and require source-supported revalidation, replacement, cancellation, or renewed approval. | L3-B-001 §11 |
| L4-B-001-BH-006 | The system shall make the Institution Active only when approval exists, mandatory conditions remain satisfactory, and the permitted effective point is reached. | L3-B-001 §§3-4,9,13,16 |
| L4-B-001-BH-007 | The system shall prevent a Rejected activation outcome from transitioning directly to Active. | L3-B-001 §§10,12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-B-001-BH-008 | The system shall publish `InstitutionActivated` after the Institution has successfully reached Active; the canonical publisher remains Institution. | L3-B-001 §§14,16; `L2-B01_Institution_Workflow.md` event semantics |

---

## 6. Failure / Rejection / Continuation

- Invalid entry identity, Tenant context, lifecycle position, or mandatory evidence prevents progression and retains the last valid business position.
- Correctable requests may follow the existing Returned path; Rejected cannot transition directly to Active.
- A stale approved outcome is not silently applied against a newer authoritative baseline.

Technical retry, rollback, transaction, exception-handler, queue-recovery, or infrastructure-recovery mechanisms are outside L4.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable business evidence sufficient to demonstrate the governed outcome, including: Institution identity and Tenant context; activation submission; material validation/dependency results; Institution Activation Authority decision; prior/resulting lifecycle position; effective context; successful activation event where applicable.

No database, log format, storage engine, message schema, or technical retention mechanism is prescribed.

---

## 8. Observable Completion Conditions

- The canonical Institution and Tenant association are unambiguous.
- Mandatory activation validations are satisfactory and an attributable approval exists.
- The Institution is Active at the permitted effective point.
- Required evidence is attributable and InstitutionActivated is produced.
- No downstream global-readiness result is implied.

Completion does not wait for unrelated downstream scenarios unless the approved parent explicitly requires their outcome.

---

## 9. Traceability & Provenance

**Parent:** L3-B-001 — Institution Establishment to Final Activation Decision  
**Primary verification source:** L2-B01 — Institution Workflows (`L2-B01_Institution_Workflow.md`)  

Every BH is anchored to material sections of the Locked L3 parent. L2 is used only to verify exact ownership, lifecycle, rule, event, authority, or dependency semantics and does not silently expand the approved L3 outcome.

---

## 10. Open Issues / Deferred Design

No material L4 issue remains. Institution Activation Authority stays an abstract business authority; local role mapping and technical authorization are deferred.

API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/transaction mechanics, infrastructure/deployment, and UI/UX remain deferred to later architecture levels.
