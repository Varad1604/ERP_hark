# Escort Request Exception to Alternative Security Response

## 1. Specification Metadata


**Specification ID:** ** `L4-G-212`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-212` — Escort Request Exception to Alternative Security Response  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-212_Escort_Request_Exception_to_Alternative_Security_Response.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed resolution of an escort request that cannot be fulfilled through the routine guard pool by establishing an approved alternative protection arrangement.

The behavior begins when the escort supervisor determines that routine escort fulfilment is not possible. It ends when the Escort Detail Aggregate reaches `ALTERNATIVE_PROTECTION_DEPLOYED`.

This specification does not execute the alternative transport/police/shelter operation, define dispatch technology, or transfer Transport/Police ownership to Security Operations.

---

## 3. Required Business Context

- Existing escort requirement.
- Determination that routine guard-pool fulfilment is not possible.
- Security Dispatch Supervisor.
- Campus Transport Manager where shuttle response is used.
- Local Police Liaison where police escort is used.
- Requestor acceptance of the resulting alternative arrangement.
- Parent-defined duty-of-care and non-abandonment rule.
- Security Operations ownership of the Escort Detail Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-212-BH-001` | When routine escort fulfilment is determined to be unavailable, the system shall establish the Escort Detail Aggregate in `EXCEPTION_PENDING`. | L3-G-212 §§4, 9, 13 |
| `L4-G-212-BH-002` | The system shall preserve the reason the routine escort cannot be fulfilled, including parent-supported resource shortage, high-risk threat, or out-of-perimeter conditions. | L3-G-212 §2 |
| `L4-G-212-BH-003` | The system shall enforce the parent-defined duty-of-care and non-abandonment obligation while the escort exception is unresolved. | L3-G-212 §12 |
| `L4-G-212-BH-004` | The system shall support selection of only the parent-listed alternative responses—secured campus shuttle, police escort liaison, or shelter-in-place—unless upstream architecture authorizes another response. | L3-G-212 §§2–3 |
| `L4-G-212-BH-005` | The system shall preserve Security Dispatch Supervisor approval of the alternative protection arrangement. | L3-G-212 §§6, 9, 15 |
| `L4-G-212-BH-006` | The system shall preserve the requestor's acceptance of the verified alternative arrangement before scenario completion. | L3-G-212 §§3–4 |
| `L4-G-212-BH-007` | On successful governed resolution, the system shall transition the Escort Detail Aggregate from `EXCEPTION_PENDING` to `ALTERNATIVE_PROTECTION_DEPLOYED`. | L3-G-212 §§9, 13, 16 |
| `L4-G-212-BH-008` | The system shall preserve which participating domain is responsible for the selected alternative response without transferring that domain's canonical ownership to Security Operations. | L3-G-212 §§5–6 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-212-BH-009` | The system shall preserve the cross-domain handoff to Campus Transport and/or external police liaison where applicable without redefining their responsibilities. | L3-G-212 §§5–6 |
| `L4-G-212-BH-010` | When the Escort Detail Aggregate reaches `ALTERNATIVE_PROTECTION_DEPLOYED`, the system shall produce the canonical `Escort_Exception_Resolved` business event on behalf of Security Operations. | L3-G-212 §§14, 16 |

No shuttle-dispatch integration, police-system interface, shelter technology, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If no safe parent-supported alternative can be established, the exception shall remain unresolved and the requestor shall not be treated as safely handed off.
- If the duty-of-care obligation cannot be satisfied, the system shall expose the unresolved condition for authorised escalation.
- If the selected external/cross-domain arrangement is not accepted or verified, the aggregate shall not reach `ALTERNATIVE_PROTECTION_DEPLOYED`.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- original escort requirement;
- reason for routine-escort exception;
- risk/exception basis;
- selected alternative response;
- Security Dispatch Supervisor decision;
- Transport/Police liaison where applicable;
- requestor acceptance;
- resulting `ALTERNATIVE_PROTECTION_DEPLOYED` position;
- publication of `Escort_Exception_Resolved`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Escort Detail Aggregate is `ALTERNATIVE_PROTECTION_DEPLOYED`;
- duty-of-care handling is attributable;
- the selected alternative protection arrangement is verified and accepted;
- `Escort_Exception_Resolved` has been produced.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-212` — Escort Request Exception to Alternative Security Response  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

**Upstream clarification required:** The Locked parent describes the `Escort Requestor` as executive oversight authority granting exception waivers/policy-level sign-offs. L4 does not silently adopt or reinterpret that authority. The operational outcome and Security Dispatch Supervisor approval remain modeled as supported; any additional waiver authority requires upstream clarification.

Dispatch technology, police interfaces, shuttle systems, persistence, event transport, audit implementation, and UI/UX remain deferred.
