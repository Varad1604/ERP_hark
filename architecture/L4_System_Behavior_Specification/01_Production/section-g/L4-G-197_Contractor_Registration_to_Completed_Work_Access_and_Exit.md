# Contractor Registration to Completed Work Access and Exit

## 1. Specification Metadata


**Specification ID:** ** `L4-G-197`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-197` — Contractor Registration to Completed Work Access and Exit  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-197_Contractor_Registration_to_Completed_Work_Access_and_Exit.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed contractor access from verified work permit and labour muster through daily campus work access and verified end-of-day exit.

The behavior begins when a contractor team arrives with an approved Work Permit. It ends when the Contractor Access Pass Aggregate reaches `CONTRACTOR_SHIFT_CLOSED`.

This specification does not govern work execution, civil/maintenance delivery, contract administration, payroll, or physical safety-device implementation.

---

## 3. Required Business Context

- Active Security Office and service-gate context.
- Approved Work Permit.
- Authoritative contractor/labour identity context.
- Estate contract/project context supplied by the owning Campus Operations source.
- Parent-defined labour safety and insurance clearance.
- Required PPE/safety compliance evidence.
- Security Operations ownership of the Contractor Access Pass Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-197-BH-001` | When a contractor team arrives with an approved Work Permit, the system shall establish the access case in `PERMIT_VERIFIED`. | L3-G-197 §§4, 9, 13 |
| `L4-G-197-BH-002` | The system shall evaluate the contractor labour muster against the approved estate-contract/work-permit context before daily access is granted. | L3-G-197 §§2–3 |
| `L4-G-197-BH-003` | The system shall require the parent-defined statutory labour-safety and insurance clearance before normal contractor access. | L3-G-197 §12 |
| `L4-G-197-BH-004` | The system shall require the parent-defined PPE/safety compliance conditions before access is treated as valid. | L3-G-197 §2 |
| `L4-G-197-BH-005` | The system shall prevent normal access when required identity, permit, insurance, safety, or physical verification fails. | L3-G-197 §11 |
| `L4-G-197-BH-006` | The system shall preserve the authorised daily contractor-access disposition and the labour roster admitted under it. | L3-G-197 §§2–3, 16 |
| `L4-G-197-BH-007` | Before shift closure, the system shall require evidence that the contractor labour team has been checked out and that the site-supervisor attestation required by the parent has been completed. | L3-G-197 §§3–4 |
| `L4-G-197-BH-008` | On successful governed checkout, the system shall transition the Contractor Access Pass Aggregate from `PERMIT_VERIFIED` to `CONTRACTOR_SHIFT_CLOSED`. | L3-G-197 §§9, 13, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-197-BH-009` | The system shall recognize authoritative estate/project and identity facts from their owning domains without transferring ownership to Security Operations. | L3-G-197 §§5, 14 |
| `L4-G-197-BH-010` | When the contractor shift reaches `CONTRACTOR_SHIFT_CLOSED`, the system shall produce the canonical `Contractor_Work_Access_Closed` business event on behalf of Security Operations. | L3-G-197 §§14, 16 |

No workforce-management implementation, scanner technology, interface protocol, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If required work permit, identity, labour-safety, insurance, PPE, or physical verification fails, the system shall prevent normal access.
- If end-of-day personnel accountability is incomplete, the system shall not represent the contractor shift as closed.
- If a parent-supported exception is authorised, the system shall preserve its authority and justification.
- If an unresolved security breach interrupts the scenario, the system shall preserve the last valid business position for authorised escalation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- work permit and contractor access case;
- labour muster considered;
- estate/project context used;
- safety, insurance, and PPE verification;
- daily access disposition;
- contractor checkout evidence;
- site-supervisor personnel-accountability attestation;
- resulting `CONTRACTOR_SHIFT_CLOSED` position;
- publication of `Contractor_Work_Access_Closed`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Contractor Access Pass Aggregate is `CONTRACTOR_SHIFT_CLOSED`;
- labour entry and exit are attributable;
- required safety/permit evidence is preserved;
- zero-missing-personnel attestation is recorded;
- `Contractor_Work_Access_Closed` has been produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-197` — Contractor Registration to Completed Work Access and Exit  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Badge technology, attendance devices, worksite systems, persistence, event transport, audit implementation, and UI/UX remain deferred.
