# Escort Request to Escort Assignment

## 1. Specification Metadata


**Specification ID:** ** `L4-G-210`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-210` — Escort Request to Escort Assignment  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-210_Escort_Request_to_Escort_Assignment.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed assessment and assignment of a security escort request from intake through risk categorization, approval, guard assignment, and rendezvous readiness.

The behavior begins when an escort request is submitted by an authorised source. It ends when the Escort Detail Aggregate reaches `ESCORT_ASSIGNED` and the assigned escort detail is ready at the rendezvous point.

This specification does not govern execution of the escort journey, route-navigation technology, physical guard tactics, or downstream incident handling.

---

## 3. Required Business Context

- Escort request.
- Requestor identity and source context.
- Purpose category, including parent-supported late-night safety, VIP, or high-value transfer context.
- Security Dispatch Supervisor.
- Available Security Escort Guards.
- Risk assessment context.
- Parent-defined gender-sensitive night escort composition rule.
- Security Operations ownership of the Escort Detail Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-210-BH-001` | When an escort request is accepted, the system shall establish the Escort Detail Aggregate in `REQUESTED`. | L3-G-210 §§4, 9, 13 |
| `L4-G-210-BH-002` | The system shall preserve the requestor, escort purpose, requested movement context, and other parent-required facts needed for risk assessment. | L3-G-210 §§2–4 |
| `L4-G-210-BH-003` | The system shall assign the parent-supported risk category before the escort is treated as approved and assigned. | L3-G-210 §3 |
| `L4-G-210-BH-004` | For night-escort situations to which the parent rule applies, the system shall enforce the parent-defined gender-sensitive escort-composition constraint. | L3-G-210 §12 |
| `L4-G-210-BH-005` | The system shall require Security Dispatch Supervisor approval before the escort assignment is treated as authorised. | L3-G-210 §§6, 9, 15 |
| `L4-G-210-BH-006` | The system shall preserve the assigned Security Escort Guards and governed route context required by the parent scenario. | L3-G-210 §§2–3 |
| `L4-G-210-BH-007` | On successful governed assignment, the system shall transition the Escort Detail Aggregate from `REQUESTED` to `ESCORT_ASSIGNED`. | L3-G-210 §§9, 13, 16 |
| `L4-G-210-BH-008` | The system shall preserve the parent-defined rendezvous readiness outcome without extending this specification into execution of the escort journey. | L3-G-210 §§3–4 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-210-BH-009` | The system shall recognize authoritative requestor and cross-domain request facts from their owning domains without transferring ownership to Security Operations. | L3-G-210 §§2, 5, 14 |
| `L4-G-210-BH-010` | When the Escort Detail Aggregate reaches `ESCORT_ASSIGNED`, the system shall produce the canonical `Security_Escort_Assigned` business event on behalf of Security Operations. | L3-G-210 §§14, 16 |

No route-engine technology, personnel-dispatch implementation, location tracking, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If requestor identity, risk context, or required escort composition cannot be validated, the system shall prevent normal assignment.
- If suitable authorised guards cannot be assigned, the system shall retain the escort request as unresolved rather than representing it as `ESCORT_ASSIGNED`.
- If a parent-supported supervisory exception is used, the system shall preserve the authority and justification without bypassing non-waived safety constraints.
- Escort execution after assignment remains outside this specification.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- escort request and requestor;
- escort purpose;
- risk category;
- applicable composition rule;
- Security Dispatch Supervisor approval;
- assigned guards;
- governed route/rendezvous context;
- resulting `ESCORT_ASSIGNED` position;
- publication of `Security_Escort_Assigned`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Escort Detail Aggregate is `ESCORT_ASSIGNED`;
- risk category and required approval are attributable;
- assigned guards and relevant composition constraints are recorded;
- rendezvous readiness is established;
- `Security_Escort_Assigned` has been produced;
- escort journey execution remains outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-210` — Escort Request to Escort Assignment  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Route-navigation technology, personnel-dispatch software, location tracking, persistence, event transport, audit implementation, and UI/UX remain deferred.
