# Material Movement Request to Completed or Rejected Gate Pass

## 1. Specification Metadata


**Specification ID:** ** `L4-G-195`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-195` — Material Movement Request to Completed or Rejected Gate Pass  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-195_Material_Movement_Request_to_Completed_or_Rejected_Gate_Pass.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed evaluation of an institutional material movement request from `MGP_REQUESTED` through authorised dispatch/closure or the parent-supported rejected gate-pass outcome.

The behavior begins when material arrives for gate clearance with the applicable departmental Material Gate Pass request. It ends when the material movement is authoritatively cleared and closed or rejected under the parent scenario.

This specification does not own asset master valuation, procurement, accounting, physical impoundment operations, or downstream disciplinary investigation.

---

## 3. Required Business Context

- Material Gate Pass request in `MGP_REQUESTED`.
- Authoritative asset/material identification required by the parent.
- Departmental or asset-custodian authorisation.
- Applicable invoice/challan or supporting movement evidence.
- Physical material inspection outcome.
- Relevant Finance/Asset facts where required.
- Security Operations ownership of the Material Gate Pass Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-195-BH-001` | When a material movement request reaches the cargo gate, the system shall establish or recognize the Material Gate Pass case in `MGP_REQUESTED`. | L3-G-195 §§4, 9, 13 |
| `L4-G-195-BH-002` | The system shall evaluate the material movement against the authorised asset-custodian decision and the parent-required supporting movement evidence. | L3-G-195 §§2–3, 12, 15 |
| `L4-G-195-BH-003` | The system shall require the parent-defined physical verification of the material against the supporting asset/invoice/challan context before dispatch may be approved. | L3-G-195 §§2–3, 9 |
| `L4-G-195-BH-004` | The system shall prevent material dispatch when required credential, custody, asset, or physical verification fails. | L3-G-195 §11 |
| `L4-G-195-BH-005` | The system shall preserve whether the gate-pass outcome is authorised dispatch or a parent-supported rejection. | L3-G-195 §§3–4, 11, 16 |
| `L4-G-195-BH-006` | The system shall require the parent-defined authorised asset-custodian sign-off before a valid dispatch outcome is established. | L3-G-195 §§12, 15 |
| `L4-G-195-BH-007` | On successful governed dispatch, the system shall transition the Material Gate Pass Aggregate from `MGP_REQUESTED` to `MGP_DISPATCHED_AND_CLOSED`. | L3-G-195 §§9, 13, 16 |
| `L4-G-195-BH-008` | When the movement is rejected, the system shall preserve the rejection decision and prevent the rejected request from being represented as dispatched. | L3-G-195 §§3, 11 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-195-BH-009` | The system shall recognize authoritative Finance/Asset and identity facts used in gate-pass governance without transferring ownership of those facts to Security Operations. | L3-G-195 §§5, 14–15 |
| `L4-G-195-BH-010` | When the Material Gate Pass reaches the successful dispatch outcome, the system shall produce the canonical `Material_Gate_Pass_Dispatched` business event on behalf of Security Operations. | L3-G-195 §§14, 16 |

---

## 6. Failure / Rejection / Continuation

- If required identity, custody, material, or supporting-document verification fails, the system shall prevent dispatch and preserve the rejection or unresolved outcome supported by the parent.
- If required asset-custodian sign-off is absent, the system shall prevent successful closure as dispatched.
- If a parent-supported supervisory exception is authorised, the system shall preserve the authority and justification and apply only its approved scope.
- If a security breach or dispute interrupts the movement, the system shall retain the last valid business position and expose the unresolved condition for authorised escalation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- Material Gate Pass request;
- material/asset identity considered;
- supporting invoice/challan or movement evidence considered;
- physical inspection result;
- asset-custodian authorisation;
- relevant Finance/Asset facts used;
- dispatch or rejection decision;
- previous and resulting Material Gate Pass position;
- publication of `Material_Gate_Pass_Dispatched` when successful.

---

## 8. Observable Completion Conditions

L4 behavior is complete when either:

- the Material Gate Pass reaches `MGP_DISPATCHED_AND_CLOSED` with required authorisation and verification evidence and `Material_Gate_Pass_Dispatched` has been produced; or
- the parent-supported rejection outcome is preserved and the material movement is not represented as dispatched.

Physical impoundment, asset-accounting updates, and downstream investigation remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-195` — Material Movement Request to Completed or Rejected Gate Pass  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent. No rejection event has been added because the parent defines only `Material_Gate_Pass_Dispatched` as canonical published event.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Asset-scanning technology, document interfaces, persistence, event transport, technical access control, audit implementation, and UI/UX remain deferred.
