# Available Asset to Governed Assignment and Custody

## 1. Specification Metadata

**Specification ID:** L4-J-075
**Architecture Level:** L4 — System Behavior Specification
**Domain:** Section J — Enterprise Resource Management / Asset Lifecycle Management
**Realizes Scenario:** L3-J-075
**Primary Business Owner:** Asset Lifecycle Management (J08)
**Version:** 1.1.0
**Status:** Review
**Owner:** Pallav
**Last Updated:** 2026-09-03
**Depends On:** `L3-J-075_Available_Asset_to_Governed_Assignment_and_Custody.md`; `architecture/L2_Domain_Workflow_Architecture/section-j/L2-J08_Asset_Lifecycle_Management.md`; locked L4 governance/template/validation baseline
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP must realize **Available Asset to Governed Assignment and Custody** as observable, implementation-independent system behavior derived from locked L3-J-075. The behavior begins only when the parent trigger is authoritative and ends only when the parent-defined governed outcome (or a parent-supported rejection/alternate terminal outcome) is objectively recorded.

Asset, AssetAssignment, AssetTransfer and AssetLifecycleEvent facts remain J08-owned; receipt, inventory, stock movement, audit conclusions, disposal execution and Finance accounting remain with their canonical owners. This specification does not convert a human/business action into a system decision or physical act and does not define implementation design.

## 3. Required Business Context

The behavior requires:

- the authoritative trigger: **An available Asset has an authorised assignment requirement.**;
- the current parent-supported aggregate/lifecycle context owned by Asset Lifecycle Management (J08);
- authoritative tenant/institution identity and all parent-required source-domain references;
- attributable human decisions, approvals, confirmations, inspection/verification evidence, custody evidence or execution evidence where L3-J-075 requires them;
- the parent-defined cross-domain references needed to demonstrate completion without transferring ownership.

No local threshold, actor, event, publisher, lifecycle state, business rule or authority model is introduced by this specification.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-075-BH-001 | When an available Asset has an authorised assignment requirement, the system shall open this behavior only against the authoritative L3-J-075 context and the same tenant/institution scope. | L3-J-075 §§7–8 |
| L4-J-075-BH-002 | Before recording the scenario outcome, the system shall validate Asset availability, canonical holder/organisation references, assignment scope, custodian, location and controlled-Asset context. | L3-J-075 §§7, 9, 12 |
| L4-J-075-BH-003 | Where business authority or human evidence is material, the system shall require and record applicable assignment authority and the Asset Custodian / Holder's acceptance of responsibility. | L3-J-075 §§6, 9, 15 |
| L4-J-075-BH-004 | After the required authoritative facts, decisions and evidence are present, the system shall record AssetAssignment and resulting custody only after the required authority and holder acceptance are authoritative, while preserving prior assignment history. | L3-J-075 §§9, 13, 16 |
| L4-J-075-BH-005 | For cross-domain collaboration, the system shall make the governed assignment fact available without changing person, organisation, stock-movement or Finance-owned records. | L3-J-075 §§5, 9, 12, 14 |
| L4-J-075-BH-006 | The system shall block assignment completion when Asset availability, authority, holder acceptance or required references are invalid; it shall retain already-confirmed evidence and shall not claim the successful L3-J-075 outcome while that condition persists. | L3-J-075 §§10–12, 16 |
| L4-J-075-BH-007 | When a business fact listed in L3-J-075 §14 occurs, event behavior shall use only `AssetAssigned` with the canonical publisher(s) stated in Section 5; the system shall not rename, duplicate, or transfer publication ownership. | L3-J-075 §14 |

## 5. Cross-Domain & Event Behavior

Canonical event behavior is limited to the business facts explicitly listed in L3-J-075 §14:

| Business Event | Canonical Publisher | L4 Obligation |
|---|---|---|
| `AssetAssigned` | Asset Lifecycle Management (J08) | Publish or consume only when the corresponding L3-J-075 §14 business fact occurs; preserve the named publisher and do not rename or duplicate the event. |

Synchronous validations, acknowledgements, approval outcomes and governed hand-offs remain distinct from these events. No transport, payload, queue, retry or protocol design is defined at L4.

Asset, AssetAssignment, AssetTransfer and AssetLifecycleEvent facts remain J08-owned; receipt, inventory, stock movement, audit conclusions, disposal execution and Finance accounting remain with their canonical owners.

## 6. Failure / Rejection / Continuation

The system shall not claim the successful L3-J-075 outcome when a required authoritative fact, business decision, human evidence, lifecycle eligibility condition, or mandatory cross-domain reference is absent or contradictory. Already-confirmed business evidence shall remain attributable and intact while the matter follows only a parent-supported return, referral, correction, continuation, cancellation, exception or alternate outcome.

A participating-domain interruption shall not silently rewrite an already-established canonical owner outcome. Technical retry, rollback and transport recovery remain outside L4.

## 7. Audit & Evidence Behavior

The system shall preserve business evidence sufficient to demonstrate:

- the authoritative trigger and source references considered;
- the actor-supplied decision/approval/confirmation/evidence where required;
- the exact owned record or lifecycle consequence established by this behavior;
- prior history that must remain non-destructively traceable;
- each material cross-domain reference consumed or made available;
- each parent-defined event actually published or consumed and its canonical publisher;
- the successful, blocked, returned, rejected, cancelled, exceptional or otherwise parent-supported outcome actually reached.

Audit infrastructure remains owned by Section A; this section specifies business evidence only.

## 8. Observable Completion Conditions

Completion is objectively observable only when the authoritative trigger, required validation, applicable human/business authority, owned record/evidence consequence, and mandatory cross-domain references required by L3-J-075 have been established. Where the parent defines event behavior, the applicable canonical event fact must correspond to the actual business outcome; where the parent defines no event, no event is required or invented.

Completion shall remain traceable to L3-J-075 and shall not claim completion of a lifecycle owned by another domain.

## 9. Traceability & Provenance

**Parent:** L3-J-075 — Available Asset to Governed Assignment and Custody
**Primary verification source:** `L2-J08_Asset_Lifecycle_Management.md`
**Higher-level constraints:** Locked L1 ownership; Dependency Map; L4 Governance and Architecture Standard; L4 Validation Standard

BH provenance is limited to the locked parent scenario. L2/L1 are verification sources only and are not used to silently repair or extend L3.

## 10. Open Issues / Deferred Design


No API/interface design, persistence/data model, service/component design, event transport, technical access control, retry/rollback/transaction mechanics, infrastructure/deployment, or UI/UX design is specified here.

---

