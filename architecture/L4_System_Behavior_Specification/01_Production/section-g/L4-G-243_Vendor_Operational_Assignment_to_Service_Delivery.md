# Vendor Operational Assignment to Service Delivery

## 1. Specification Metadata

**Specification ID:** L4-G-243
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Campus Operations — Housekeeping Operations (G08)
**Realizes Scenario:** L3-G-243 — Vendor Operational Assignment to Service Delivery
**Primary Business Owner:** Housekeeping Operations
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-03
**Depends On:** `L3-G-243_Vendor_Operational_Assignment_to_Service_Delivery.md`; `L2-G08_Housekeeping_Operations_and_Service_Governance_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`; `L4_Governance_and_Architecture_Standard.md`; `L4_Validation_Standard.md`
**Referenced By:** None yet

---

## 2. Behavioral Objective & Boundary

SutraERP shall support **Vendor Operational Assignment to Service Delivery** strictly from the parent-defined start boundary **Vendor Operational Assignment** to the parent-defined end boundary **Service Delivery**.

This L4 specification defines only observable, implementation-independent system behavior. It does not define APIs, database/persistence design, software components, message transport, technical IAM, transactions/retries, infrastructure/deployment or UI/UX.

---

## 3. Required Business Context

- `L3-G-243` is the sole Locked L3 parent for this specification.
- **Housekeeping Operations** remains the canonical business owner of the scenario outcome and applicable owned lifecycle.
- The initiating boundary is **Vendor Operational Assignment** and is governed by `L3-G-243 §§4.1, 7–8`.
- The final boundary is **Service Delivery** and is governed by `L3-G-243 §§4.2 and 16`.
- The exact parent lifecycle evidence used here is **Service Quality Assessment: Initiated → Evaluated → Approved → Closed (only the applicable portion is traversed)** from `L3-G-243 §13`.
- Participating-domain context (E, J) remains source-owned and read-only to this scenario except for the business hand-offs explicitly allowed by the parent.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-243-BH-001` | When the exact parent-defined initiating condition for **Vendor Operational Assignment** is presented, the system shall validate the required business references/evidence before admitting the scenario; if validation fails, progression shall be withheld and the validation result preserved. | L3-G-243 §§4.1, 7–8, 11 |
| `L4-G-243-BH-002` | The system shall preserve the parent-defined **Service Quality Assessment** lifecycle boundary exactly as **Initiated → Evaluated → Approved → Closed (only the applicable portion is traversed)** and shall prevent any transition not supported by the Locked parent. | L3-G-243 §13 |
| `L4-G-243-BH-003` | When the exact parent-defined event condition is satisfied, the system shall publish/record `QualityInspectionCompleted` on behalf of the canonical publisher **Housekeeping Operations** and shall not rename the event or transfer publisher ownership. | L3-G-243 §14.1 |
| `L4-G-243-BH-004` | Where the parent requires a business authorization or decision, the system shall verify that a decision/evidence reference from the parent-authorized business role is present and shall record that authority basis; the system shall not act as the business approver. | L3-G-243 §15.1 |
| `L4-G-243-BH-005` | For material cross-domain facts or hand-offs required by the parent, the system shall use only authoritative source-owned context, preserve the external domain as canonical owner, and withhold the affected outcome when a required authoritative fact is unavailable. | L3-G-243 §§5, 11, 14.2 |
| `L4-G-243-BH-006` | The system shall establish **Service Delivery** only after every material completion criterion in the Locked parent is objectively evidenced; otherwise it shall expose a non-complete/recovery position and shall not broaden completion into downstream outcomes. | L3-G-243 §16 |

---

## 5. Cross-Domain & Event Behavior

| Event | Canonical Publisher | Required L4 Treatment | Parent |
|---|---|---|---|
| `QualityInspectionCompleted` | Housekeeping Operations | Produce/recognize only under: Parent-defined condition; preserve canonical name and publisher. | L3-G-243 §14.1 |

No event transport, payload schema, broker, retry/delivery mechanism or other implementation design is defined.

Cross-domain contracts/hand-offs are limited to those explicitly defined by `L3-G-243 §14.2`; this specification does not create an L4-local business contract or transfer canonical ownership.

---

## 6. Failure / Rejection / Continuation

- If the exact parent entry condition for **Vendor Operational Assignment** is invalid, unsupported or lacks mandatory evidence, the system shall reject/withhold progression and preserve the failure reason.
- If a parent-required authoritative context or business decision is unavailable, the system shall retain the last valid business position and expose the unresolved condition rather than infer a replacement fact or approval.
- If the lifecycle/evidence required by `L3-G-243 §16` is incomplete, the system shall not establish **Service Delivery**.
- Only correction, alternative, escalation, reassignment, rescheduling, continuation or other recovery paths explicitly supported by `L3-G-243` may be exposed.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- the initiating reference and validation result for **Vendor Operational Assignment**;
- the applicable parent-defined business rule/policy basis actually used;
- authoritative external context materially relied upon, with source ownership preserved;
- the previous and resulting lifecycle position when a transition occurs;
- the parent-authorized actor/decision reference when authorization is required;
- each parent-defined event/handoff actually produced or consumed, when applicable;
- the evidence that establishes **Service Delivery**, or the reason completion was withheld.

The implementation mechanism for audit storage, retention or logging is outside L4.

---

## 8. Observable Completion Conditions

L4 behavior is complete only when:

- the exact parent entry/progression conditions applicable to **Vendor Operational Assignment** have been satisfied;
- the parent-defined **Service Quality Assessment** lifecycle has not been corrupted or expanded;
- all applicable exact parent events are evidenced with canonical names/publisher ownership;
- **Service Delivery** is objectively evidenced under `L3-G-243 §16`;
- **Housekeeping Operations** retains canonical business ownership and participating domains retain ownership of their canonical information;
- no downstream result beyond the parent end boundary has been absorbed.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-243` — Vendor Operational Assignment to Service Delivery
**Primary L2 verification source:** `L2-G08_Housekeeping_Operations_and_Service_Governance_Workflows.md`
**Higher-level ownership/dependency verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`
**Pinned repository evidence:** `4e98dc1075ef6e6e1990f9287d82ddf1185448f4` (`main`)
**Canonical parent path:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-g/L3-G-243_Vendor_Operational_Assignment_to_Service_Delivery.md`

Every admitted BH is anchored to the exact Locked L3 parent. L2 is used as a semantic verification source only; this L4 does not silently repair an upstream omission.

---

## 10. Open Issues / Deferred Design

No unresolved L4 business-behavior issue was identified during the author self-review against the pinned parent baseline. Independent Architecture Approval remains required by repository governance.

Implementation architecture and technical design—including API/interface form, persistence model, service/component design, event transport, technical IAM, retry/rollback/transaction mechanics, infrastructure/deployment and UI/UX—remain deferred to later architecture levels.
