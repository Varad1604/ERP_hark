# Moderate Examination Marks Under Approved Policy

## 1. Specification Metadata

**Specification ID:** L4-I-075
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Examination — Marks Collection & Evaluation
**Realizes Scenario:** L3-I-075
**Primary Business Owner:** Marks Collection & Evaluation
**Version:** 1.0.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-03
**Depends On:** L3-I-075 — Locked canonical parent; L2-I08 — Locked primary verification source; `Section_I_L3_Event_Integrity_and_Boundary_Remediation_Amendment_2026-09-03.md` — approved upstream remediation; L4 Governance and Architecture Standard; L4 Validation Standard
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP must realize **Moderate Examination Marks Under Approved Policy** as the observable system behavior required by the Locked parent scenario, while preserving Examination ownership, source-owned cross-domain facts, and the parent scenario boundary.

The behavior begins when **Marks are eligible for policy-governed moderation** and all parent-defined entry conditions exist. It ends when **Policy-governed moderated marks outcome** is established as the governed outcome, including only the failure, rejection, continuation and evidence behavior required by the parent. Adjacent Examination outcomes and other domains' lifecycles remain outside this specification unless the parent explicitly includes them.

## 3. Required Business Context

- Authoritative completed examination/assessment evidence and evaluation context.
- Applicable evaluator assignment, marking/evaluation policy and candidate/assessment identity.
- Source-supported validation, review, moderation and marks-approval conditions where the parent requires them.
- Participating domains identified by the Locked parent retain ownership of their source facts, decisions, lifecycles and published business facts.

Only authoritative business context identified by the parent or its Locked verification source may be used. This specification does not create a new business rule, lifecycle state, event, publisher, actor or authority.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-075-BH-001 | The system shall apply marks moderation only when the governing policy and moderation authority required by the parent scenario are authoritative. | L3-I-075 §§3–4, 7–9, 11–16 |
| L4-I-075-BH-002 | The system shall calculate no locally invented moderation threshold, formula or discretion beyond the source-defined moderation rule. | L3-I-075 §§3–4, 7–9, 11–16 |
| L4-I-075-BH-003 | The system shall preserve the pre-moderation marks and the authoritative moderation basis when moderated marks are established. | L3-I-075 §§3–4, 7–9, 11–16 |
| L4-I-075-BH-004 | The system shall prevent moderated marks from becoming authoritative until the parent-defined moderation conditions are satisfied. | L3-I-075 §§3–4, 7–9, 11–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-075-BH-005 | The system shall treat a collaboration occurrence as a business event only when its name exactly matches the canonical L2-I08 event vocabulary admitted by the approved Section I remediation amendment; workflow, capability, policy, configuration and activity labels shall not be emitted as events. | L3-I-075 §14; proposed L3 Section I remediation amendment §§3–5; L2-I08 §§11–12 |
| L4-I-075-BH-006 | When the source-defined transition represented by `Moderation Started` occurs within this scenario boundary, the system shall publish `Moderation Started` under the canonical L2-I08 publisher and shall not rename or republish it under another owner. | L3-I-075 §§9, 14, 16; proposed L3 Section I remediation amendment §5; L2-I08 §§11–12 |
| L4-I-075-BH-007 | When the source-defined transition represented by `Moderation Completed` occurs within this scenario boundary, the system shall publish `Moderation Completed` under the canonical L2-I08 publisher and shall not rename or republish it under another owner. | L3-I-075 §§9, 14, 16; proposed L3 Section I remediation amendment §5; L2-I08 §§11–12 |

## 6. Failure / Rejection / Continuation

- If a mandatory parent-defined entry condition or authoritative source fact is missing, invalid or unresolved, the system shall prevent only the affected progression and preserve the last valid business position.
- If the parent defines a rejection, conflict, exception, withdrawal, interruption, correction or alternate governed outcome, the system shall permit only that source-supported business disposition.
- The system shall not infer a successful outcome while a material parent-defined blocking condition remains unresolved.
- Continuation shall occur only after the authoritative business condition is resolved or an allowed terminal outcome is established.

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to attribute the governing parent context, authoritative source facts considered, applicable policy/rule basis, responsible business decision or authority where required, prior and resulting business positions, material exception/correction evidence, and the final governed outcome. Evidence preservation shall not redefine ownership of the underlying business facts.

## 8. Observable Completion Conditions

This L4 behavior is complete when:

- the parent-defined entry conditions were satisfied or an allowed terminal alternative was established;
- every applicable BH in this specification has an objectively observable result;
- **Policy-governed moderated marks outcome** is established as the authoritative governed outcome for this scenario scope;
- required business evidence and source-supported collaboration are preserved; and
- no adjacent scenario outcome, source-domain lifecycle or downstream result has been absorbed beyond the Locked parent boundary.

## 9. Traceability & Provenance

**Parent:** L3-I-075 — Moderate Examination Marks Under Approved Policy
**Approved upstream remediation:** `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/Section_I_L3_Event_Integrity_and_Boundary_Remediation_Amendment_2026-09-03.md`
**Primary verification source:** L2-I08 — locked Section I workflow architecture

Every BH retains L3-I-075 as its single parent. The approved upstream amendment corrects the current parent's event classification and the explicit boundary clarifications recorded for affected scenarios. L2-I08 is used only to verify exact event names, publishers and already-established business semantics.

The 2026-09-03 Architecture Approval makes the Section I remediation amendment effective for the scoped parent correction. Repository locking remains a separate explicit governance action.

## 10. Open Issues / Deferred Design

This specification is synchronized with the Architecture-Approved `Section_I_L3_Event_Integrity_and_Boundary_Remediation_Amendment_2026-09-03.md`. Repository lock remains a separate governance action.

Architecture Approval was granted on the amended parent baseline for L3-I-075 by Vikram Desai, Enterprise Architecture Governance Lead, and recorded on 2026-09-03. No `Locked` status is asserted by this approval.

API/interface design, persistence, event transport, technical access control, infrastructure and UI/UX remain deferred.
