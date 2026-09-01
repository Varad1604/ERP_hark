# Visitor Registration to Authorised Visit and Governed Pass

## 1. Specification Metadata


**Specification ID:** ** `L4-G-192`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-192` — Visitor Registration to Authorised Visit and Governed Pass  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-192_Visitor_Registration_to_Authorised_Visit_and_Governed_Pass.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed visitor registration from identity intake through host authorisation and activation of the visitor pass.

The behavior begins when a visitor presents for registration at campus reception. It ends when the Visitor Pass Aggregate reaches `PASS_ACTIVE` and the authorised visit is established.

This specification does not govern the subsequent visitor check-in/checkout lifecycle, physical badge production technology, disciplinary matters, or identity-master implementation.

---

## 3. Required Business Context

- Active security command context.
- Visitor identity information required by the parent scenario.
- Authoritative host employee/department context.
- Host pre-authorisation or consent outcome.
- Applicable visitor security policy.
- Visitor Reception Officer and Security Post Supervisor authority.
- Security Operations ownership of the Visitor Pass Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-192-BH-001` | When a visitor registration request is initiated, the system shall establish the visitor-pass case in `INTAKE_DRAFT`. | L3-G-192 §§4, 9, 13 |
| `L4-G-192-BH-002` | The system shall require the visitor identity information and verification conditions defined by the parent before an authorised visit may be established. | L3-G-192 §§2, 9, 12 |
| `L4-G-192-BH-003` | The system shall require authoritative host approval before a visitor pass may become active. | L3-G-192 §§2–3, 12 |
| `L4-G-192-BH-004` | The system shall prevent visitor-pass activation when required credential, host-authorisation, or security verification conditions fail. | L3-G-192 §11 |
| `L4-G-192-BH-005` | The system shall require the parent-defined supervisory clearance before transitioning the Visitor Pass Aggregate to `PASS_ACTIVE`. | L3-G-192 §§9, 15 |
| `L4-G-192-BH-006` | The system shall prevent the operational actor performing the visitor intake from being treated as the approving authority for the same supervisory clearance. | L3-G-192 §15.2 |
| `L4-G-192-BH-007` | On successful authorisation, the system shall transition the Visitor Pass Aggregate from `INTAKE_DRAFT` to `PASS_ACTIVE`. | L3-G-192 §§3, 13, 16 |
| `L4-G-192-BH-008` | The system shall associate the active visitor pass with the approved visit and host context required by the parent scenario. | L3-G-192 §§2–3, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-192-BH-009` | The system shall recognize authoritative host and identity facts supplied by their owning domains without transferring ownership of those facts to Security Operations. | L3-G-192 §§5, 14 |
| `L4-G-192-BH-010` | When the Visitor Pass Aggregate reaches `PASS_ACTIVE`, the system shall produce the canonical `Visitor_Pass_Issued` business event on behalf of Security Operations. | L3-G-192 §§14, 16 |

No badge-printing design, identity interface, message format, or event transport is specified.

---

## 6. Failure / Rejection / Continuation

- If required visitor identity verification fails, the system shall prevent `PASS_ACTIVE`.
- If host authorisation is absent or rejected, the system shall prevent visitor-pass activation.
- If a parent-supported supervisory exception is authorised, the system shall preserve the decision and justification and apply only its approved scope.
- If an unresolved security issue halts registration, the system shall retain the last valid visitor-pass position and expose the unresolved condition for authorised escalation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- visitor registration request;
- identity information and verification basis considered;
- host identity and host authorisation;
- security verification result;
- supervisory clearance;
- any approved exception and justification;
- previous and resulting Visitor Pass Aggregate position;
- publication of `Visitor_Pass_Issued`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the Visitor Pass Aggregate is `PASS_ACTIVE`;
- visitor identity and host approval requirements are satisfied;
- supervisory clearance is attributable;
- `Visitor_Pass_Issued` has been produced;
- subsequent visitor movement, checkout, and overstay resolution remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-192` — Visitor Registration to Authorised Visit and Governed Pass  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Badge-generation technology, identity integrations, persistence, event transport, technical access control, audit implementation, and UI/UX remain deferred.
