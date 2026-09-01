# Gate Closure to Restricted Access

## 1. Specification Metadata


**Specification ID:** ** `L4-G-190`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-190` — Gate Closure to Restricted Access  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-08-30  
**Depends On:** ** `L3-G-190_Gate_Closure_to_Restricted_Access.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support the governed transition of an operational gate from `GATE_OPEN` to `GATE_RESTRICTED_LOCKED` when the scheduled closure/curfew condition or other parent-supported closure requirement applies.

The behavior begins when a valid closure requirement applies to a gate in `GATE_OPEN`. It ends when the gate reaches `GATE_RESTRICTED_LOCKED`, required closure authority and evidence are attributable, restricted-access operation is established at the business level, and the canonical closure event is produced.

This specification does not define physical locking mechanisms, individual night-pass adjudication, emergency-device operation, gate reopening, or downstream monitoring.

---

## 3. Required Business Context

- Gate currently at parent-defined `GATE_OPEN`.
- Applicable closure/curfew condition or authorized closure requirement.
- Active Security Office and relevant gate/patrol context.
- Eligible operating personnel.
- Parent-defined closure inspection and verification context.
- Security Shift Supervisor closure clearance.
- Parent-defined restricted-access business outcome.
- Required execution/approval separation.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-190-BH-001` | When a governed closure requirement applies to a gate in `GATE_OPEN`, the system shall establish the closure case while retaining `GATE_OPEN` until required closure conditions are satisfied. | L3-G-190 §§3, 7–9, 13 |
| `L4-G-190-BH-002` | The system shall require the parent-defined active Security Office and eligible operating-personnel context before the closure transition is committed. | L3-G-190 §7 |
| `L4-G-190-BH-003` | The system shall require the parent-defined closure verification and applicable night-curfew condition before normal transition to `GATE_RESTRICTED_LOCKED`. | L3-G-190 §§2, 9, 12 |
| `L4-G-190-BH-004` | The system shall require authoritative Security Shift Supervisor clearance before committing the closure outcome. | L3-G-190 §§6, 9, 15 |
| `L4-G-190-BH-005` | The system shall prevent the Gate Guard Officer performing the closure activity from being treated as the approving authority for the same closure decision. | L3-G-190 §15.2 |
| `L4-G-190-BH-006` | If required credential or physical verification fails, the system shall prevent the normal closure progression and retain the last valid gate position pending governed resolution. | L3-G-190 §11 |
| `L4-G-190-BH-007` | On successful authorized closure, the system shall transition the Gate Operational Status Aggregate from `GATE_OPEN` to `GATE_RESTRICTED_LOCKED`. | L3-G-190 §§3, 13, 16 |
| `L4-G-190-BH-008` | After the closure outcome, the system shall represent the gate as restricted-access rather than ordinarily open for transit, without defining the downstream decision for any individual entrant. | L3-G-190 §§2–4, 16 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-190-BH-009` | The system shall use authoritative identity/eligibility facts supplied by their owning domains where required by the closure scenario without re-owning those facts. | L3-G-190 §§5, 14 |
| `L4-G-190-BH-010` | When the gate reaches `GATE_RESTRICTED_LOCKED`, the system shall produce the canonical `Gate_Closed_to_Restricted_Transit` business event on behalf of Security Operations. | L3-G-190 §§14, 16 |

No event transport, physical lock control, intercom mechanism, or identity-technology design is defined.

---

## 6. Failure / Rejection / Continuation

- If required closure verification or authority is absent, the system shall not silently represent the gate as `GATE_RESTRICTED_LOCKED`.
- If a parent-supported supervisory exception is authoritatively granted, the system shall preserve its justification and apply only the approved exception scope.
- If normal digital business recording is unavailable, parent-supported manual business evidence may be recognized without defining technical synchronization or recovery mechanics.
- If a security breach or unresolved dispute interrupts closure, the system shall preserve the last valid gate position and expose the unresolved condition for authorized escalation.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- gate identity and previous `GATE_OPEN` position;
- applicable closure/curfew basis;
- personnel and credential evidence considered;
- closure verification outcome;
- supervisory clearance;
- any governed exception and justification;
- separation between execution and approval;
- resulting `GATE_RESTRICTED_LOCKED` outcome;
- publication of `Gate_Closed_to_Restricted_Transit`.

---

## 8. Observable Completion Conditions

L4 behavior is complete when:

- the gate is `GATE_RESTRICTED_LOCKED`;
- the applicable closure conditions and authoritative clearance are satisfied;
- required evidence is attributable;
- the canonical closure event has been produced;
- individual restricted-entry decisions, physical locking, monitoring, and later reopening remain outside this specification.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-190` — Gate Closure to Restricted Access  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the Locked L3 parent. The parent-defined `GATE_OPEN` → `GATE_RESTRICTED_LOCKED` transition and canonical event are preserved exactly.

---

## 10. Open Issues / Deferred Design

No material L4 architecture issue remains for this draft.

Physical locking/intercom design, device protocols, persistence, event transport, technical access control, audit implementation, and UI/UX remain deferred to downstream architecture.
