# Practical Session Preparation to Completion

## 1. Specification Metadata

**Specification ID:** L4-G-053  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Laboratory Management (G02)  
**Realizes Scenario:** L3-G-053 — Practical Session Preparation to Completion  
**Primary Business Owner:** Laboratory Management  
**Version:** 1.0.0  
**Status:** Architecture Approved  
**Approved By:** Dhruv / Lead Architect  
**Approval Date:** 2026-09-03  
**Approval Provenance:** Approval reported by Pallav in the SutraERP ChatGPT Project conversation; no ACR/reference number invented.  
**Owner:** Pallav  
**Last Updated:** 2026-09-03  
**Depends On:** `L3-G-053_Practical_Session_Preparation_to_Completion.md`; `L2-G02_Laboratory_Management_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`  
**Referenced By:** None yet  

---

## 2. Behavioral Objective & Boundary

SutraERP shall support **Practical Session Preparation to Completion** strictly from the parent-defined start boundary **Practical Session Preparation** to the parent-defined end boundary **Completion**.

This L4 specification defines only observable, implementation-independent system behavior. It does not define APIs, database/persistence design, software components, message transport, technical IAM, transactions/retries, infrastructure/deployment or UI/UX.

---

## 3. Required Business Context

- `L3-G-053` is the sole Locked L3 parent for this specification.
- **Laboratory Management** remains the canonical business owner of the scenario outcome and applicable owned lifecycle.
- The initiating boundary is **Practical Session Preparation** and is governed by `L3-G-053 §§4.1, 7–8`.
- The final boundary is **Completion** and is governed by `L3-G-053 §§4.2 and 16`.
- The exact parent lifecycle evidence used here is **Practical Session: Scheduled → Preparing → Ready → In Progress → Completed** from `L3-G-053 §13`.
- Participating-domain context (D, C, E, H) remains source-owned and read-only to this scenario except for the business hand-offs explicitly allowed by the parent.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-053-BH-001` | When the exact parent-defined initiating condition for **Practical Session Preparation** is presented, the system shall validate the required business references/evidence before admitting the scenario; if validation fails, progression shall be withheld and the validation result preserved. | L3-G-053 §§4.1, 7–8, 11 |
| `L4-G-053-BH-002` | The system shall preserve the parent-defined **Practical Session** lifecycle boundary exactly as **Scheduled → Preparing → Ready → In Progress → Completed** and shall prevent any transition not supported by the Locked parent. | L3-G-053 §13 |
| `L4-G-053-BH-003` | When the exact parent-defined event condition is satisfied, the system shall publish/record `PracticalSessionPrepared` on behalf of the canonical publisher **Laboratory Management** and shall not rename the event or transfer publisher ownership. | L3-G-053 §14.1 |
| `L4-G-053-BH-004` | When the exact parent-defined event condition is satisfied, the system shall publish/record `PracticalSessionStarted` on behalf of the canonical publisher **Laboratory Management** and shall not rename the event or transfer publisher ownership. | L3-G-053 §14.1 |
| `L4-G-053-BH-005` | When the exact parent-defined event condition is satisfied, the system shall publish/record `PracticalSessionCompleted` on behalf of the canonical publisher **Laboratory Management** and shall not rename the event or transfer publisher ownership. | L3-G-053 §14.1 |
| `L4-G-053-BH-006` | Where the parent requires a business authorization or decision, the system shall verify that a decision/evidence reference from the parent-authorized business role is present and shall record that authority basis; the system shall not act as the business approver. | L3-G-053 §15.1 |
| `L4-G-053-BH-007` | For material cross-domain facts or hand-offs required by the parent, the system shall use only authoritative source-owned context, preserve the external domain as canonical owner, and withhold the affected outcome when a required authoritative fact is unavailable. | L3-G-053 §§5, 11, 14.2 |
| `L4-G-053-BH-008` | The system shall establish **Completion** only after every material completion criterion in the Locked parent is objectively evidenced; otherwise it shall expose a non-complete/recovery position and shall not broaden completion into downstream outcomes. | L3-G-053 §16 |

---

## 5. Cross-Domain & Event Behavior

| Event | Canonical Publisher | Required L4 Treatment | Parent |
|---|---|---|---|
| `PracticalSessionPrepared` | Laboratory Management | Produce/recognize only under: Parent-defined condition; preserve canonical name and publisher. | L3-G-053 §14.1 |
| `PracticalSessionStarted` | Laboratory Management | Produce/recognize only under: Parent-defined condition; preserve canonical name and publisher. | L3-G-053 §14.1 |
| `PracticalSessionCompleted` | Laboratory Management | Produce/recognize only under: Parent-defined condition; preserve canonical name and publisher. | L3-G-053 §14.1 |

No event transport, payload schema, broker, retry/delivery mechanism or other implementation design is defined.

Cross-domain contracts/hand-offs are limited to those explicitly defined by `L3-G-053 §14.2`; this specification does not create an L4-local business contract or transfer canonical ownership.

---

## 6. Failure / Rejection / Continuation

- If the exact parent entry condition for **Practical Session Preparation** is invalid, unsupported or lacks mandatory evidence, the system shall reject/withhold progression and preserve the failure reason.
- If a parent-required authoritative context or business decision is unavailable, the system shall retain the last valid business position and expose the unresolved condition rather than infer a replacement fact or approval.
- If the lifecycle/evidence required by `L3-G-053 §16` is incomplete, the system shall not establish **Completion**.
- Only correction, alternative, escalation, reassignment, rescheduling, continuation or other recovery paths explicitly supported by `L3-G-053` may be exposed.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- the initiating reference and validation result for **Practical Session Preparation**;
- the applicable parent-defined business rule/policy basis actually used;
- authoritative external context materially relied upon, with source ownership preserved;
- the previous and resulting lifecycle position when a transition occurs;
- the parent-authorized actor/decision reference when authorization is required;
- each parent-defined event/handoff actually produced or consumed, when applicable;
- the evidence that establishes **Completion**, or the reason completion was withheld.

The implementation mechanism for audit storage, retention or logging is outside L4.

---

## 8. Observable Completion Conditions

L4 behavior is complete only when:

- the exact parent entry/progression conditions applicable to **Practical Session Preparation** have been satisfied;
- the parent-defined **Practical Session** lifecycle has not been corrupted or expanded;
- all applicable exact parent events are evidenced with canonical names/publisher ownership;
- **Completion** is objectively evidenced under `L3-G-053 §16`;
- **Laboratory Management** retains canonical business ownership and participating domains retain ownership of their canonical information;
- no downstream result beyond the parent end boundary has been absorbed.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-053` — Practical Session Preparation to Completion  
**Primary L2 verification source:** `L2-G02_Laboratory_Management_Workflows.md`  
**Higher-level ownership/dependency verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Pinned repository evidence:** `4e98dc1075ef6e6e1990f9287d82ddf1185448f4` (`main`)  
**Canonical parent path:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-g/L3-G-053_Practical_Session_Preparation_to_Completion.md`  

Every admitted BH is anchored to the exact Locked L3 parent. L2 is used as a semantic verification source only; this L4 does not silently repair an upstream omission.

---

## 10. Open Issues / Deferred Design

No unresolved L4 business-behavior issue was identified during the author self-review against the pinned parent baseline. Independent Architecture Approval remains required by repository governance.

Implementation architecture and technical design—including API/interface form, persistence model, service/component design, event transport, technical IAM, retry/rollback/transaction mechanics, infrastructure/deployment and UI/UX—remain deferred to later architecture levels.
