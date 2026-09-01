# Person Gate Access to Recorded Entry Denial or Authorised Override

## 1. Specification Metadata


**Specification ID:** ** `L4-G-191`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-191` — Person Gate Access to Recorded Entry Denial or Authorised Override  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-191_Person_Gate_Access_to_Recorded_Entry_Denial_or_Authorised_Override.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed evaluation of a person's gate-access attempt from presented credential through an authoritative granted, denied, or source-supported authorised-override outcome.

The behavior begins when a person presents an access credential at an operational gate. It ends when the access transaction reaches the parent-defined evaluated outcome and the resulting access decision is recorded.

This specification does not own identity master data, workforce/student status, visitor registration, disciplinary proceedings, physical turnstile control, or downstream security investigation.

---

## 3. Required Business Context

- Gate and associated security context are operationally active.
- A person has presented an access credential.
- Authoritative identity and eligibility facts are available from their owning domains.
- Applicable blacklist, expiry, and access-policy conditions are available.
- Security Shift Supervisor authority exists for parent-supported override decisions.
- Security Operations owns the Gate Access Transaction Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-191-BH-001` | When a person presents a gate-access credential, the system shall establish a Gate Access Transaction in the parent-defined `PRESENTED` position. | L3-G-191 §§4, 9, 13 |
| `L4-G-191-BH-002` | The system shall evaluate the presented credential against authoritative identity, eligibility, blacklist, expiry, and applicable access-policy conditions. | L3-G-191 §§2, 9, 12, 14 |
| `L4-G-191-BH-003` | The system shall prevent normal access approval when required credential or eligibility verification fails. | L3-G-191 §11 |
| `L4-G-191-BH-004` | Where a parent-supported supervisory override is requested, the system shall require an authorised Security Shift Supervisor decision and justification before treating the access attempt as overridden. | L3-G-191 §§10, 15 |
| `L4-G-191-BH-005` | The system shall prevent the Gate Guard Officer executing the access check from being treated as the approving authority for the same supervisory clearance. | L3-G-191 §15.2 |
| `L4-G-191-BH-006` | The system shall preserve whether the final access result was granted, denied, or granted through an authorised override. | L3-G-191 §§3, 9–10, 16 |
| `L4-G-191-BH-007` | On completion of the access decision, the system shall transition the Gate Access Transaction Aggregate to `ACCESS_GRANTED_OR_DENIED`. | L3-G-191 §§9, 13, 16 |
| `L4-G-191-BH-008` | The system shall make the completed access decision available for governed Security Operations use without transferring ownership of source identity facts. | L3-G-191 §§5, 14, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-191-BH-009` | The system shall recognize authoritative identity and eligibility facts supplied by their owning domains for the access evaluation without re-owning those facts. | L3-G-191 §§5, 14 |
| `L4-G-191-BH-010` | When the access transaction reaches its authoritative evaluated outcome, the system shall produce the canonical `Person_Gate_Access_Evaluated` business event on behalf of Security Operations. | L3-G-191 §§14, 16 |

No message format, event transport, scanner protocol, or physical barrier behavior is defined.

---

## 6. Failure / Rejection / Continuation

- If credential or required physical verification fails, the system shall preserve the denied outcome and prevent normal progression.
- If an override is not authorised, the system shall preserve the denial result.
- If a source-supported manual fallback is used, the system may recognize the resulting business evidence without defining technical synchronization or recovery mechanics.
- If an unresolved security dispute or breach interrupts the access decision, the system shall preserve the last valid business position and expose the unresolved condition for authorised escalation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- the presented access transaction;
- authoritative identity and eligibility facts considered;
- blacklist, expiry, and access-policy basis considered;
- normal grant or denial outcome;
- any supervisory override decision and justification;
- separation between execution and approval;
- the resulting `ACCESS_GRANTED_OR_DENIED` position;
- publication of `Person_Gate_Access_Evaluated`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the access transaction is `ACCESS_GRANTED_OR_DENIED`;
- the granted, denied, or authorised-override result is attributable;
- required evidence is preserved;
- `Person_Gate_Access_Evaluated` has been produced;
- physical transit mechanisms and downstream investigation remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-191` — Person Gate Access to Recorded Entry Denial or Authorised Override  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent. L2/L1 sources verify semantics only.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Credential-reader integration, physical barrier actuation, persistence, event transport, technical access control, audit implementation, and UI/UX remain deferred to downstream architecture.
