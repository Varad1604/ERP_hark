# L2-J03 — Purchase Requisition & Approval Workflows

**Repository:** SutraERP
**Section:** J — Enterprise Resource Management
**Domain:** J03 — Purchase Requisition & Approval
**Document ID:** L2-J03
**Version:** 1.1 — INT-018 structural maintenance
**Status:** Locked
**Classification:** Level 2 Domain Workflow Architecture
**Parent Architecture:** L2-J00 — Enterprise Resource Management Architecture & Governance
**Canonical Owner:** Purchase Requisition & Approval
**Document Owner:** Enterprise Resource Management — Requisition & Approval

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §14 Scope |
| Actors | §15 Canonical Actors |
| Entities | §4 Primary Aggregate |
| Business Rules | Rules / Invariants |
| Workflow | §12 Event and Workflow Integration; Workflows |
| State Machine | States |
| Contracts | §16 Domain Contracts |
| Events | §6 Domain Events; §12 Event and Workflow Integration |
| Dependencies | §7 Dependencies |
| Locked Decisions | §17 Locked Decisions |

> **INT-018 maintenance:** The four former structural gaps were closed on 2026-08-13 by consolidating semantics already present in §§1–13. No canonical ownership, lifecycle, event, or rule was added.

---

# 1. Purpose

J03 defines the canonical architecture for converting an institutional demand into an approved purchase requisition.

J03 governs requisition creation, amendment, review, validation, approval, rejection, and release for downstream procurement planning.

The canonical lifecycle is:

```text
Requesting Domain
        |
        v
Purchase Requisition
        |
        v
Budget / Policy Validation
        |
        v
Approval Review
        |
        v
Released for Procurement
        |
        v
J01 Procurement Planning & Sourcing
```

---

# 2. Domain Mission

J03 provides the authoritative demand-control boundary for resources that are not fulfilled directly from available stock.

It ensures purchase demand is visible, approvable, auditable, and ready for procurement planning without duplicating supplier, order, inventory, or payment ownership.

---

# 3. Explicit Non-Ownership

J03 does not own:

| Capability | Canonical Owner |
|---|---|
| Supplier Master | J02 — Vendor & Supplier Management |
| Procurement planning and sourcing | J01 — Procurement Planning & Sourcing |
| Purchase Order | J04 — Purchase Order Management |
| Goods Receipt | J05 — Goods Receipt, Inspection & Acceptance |
| Inventory Stock and Warehouse Operations | J06 — Inventory & Warehouse Management |
| Stock Movement | J07 — Stock Operations & Distribution |
| Asset Lifecycle | J08 — Asset Lifecycle Management |
| Accounting Entries, Budget, Invoice, Payment | Section F — Finance |
| Authentication | L2-A03 — Authentication |
| Authorization Policy and SoD | L2-A04 — Authorization and Policy |
| Workflow Routing | L2-A05 — Workflow Engine |
| Event Transport | L2-A06 — Event Infrastructure |
| Enterprise Audit Persistence | L2-A07 — Audit Infrastructure |

---

# 4. Primary Aggregate

| Aggregate | Responsibility |
|-----------|----------------|
| PurchaseRequisition | Governs requisition content, approval state, and release for procurement |

---

# 5. Core Responsibilities

- Create purchase requisitions.
- Manage requisition lines and quantities.
- Coordinate policy and budget validation references.
- Capture approval and rejection decisions.
- Publish requisition release facts for J01 and J04.

---

# 6. Domain Events

J03 publishes:

- PurchaseRequisitionSubmitted
- PurchaseRequisitionApproved
- PurchaseRequisitionRejected
- PurchaseRequisitionReleasedForProcurement

---

# 7. Dependencies

## Depends On

Upstream capabilities J03 requires in order to function:

- Requesting domains for demand origin.
- Finance for budget-validation references where policy requires it.

## Referenced By (Downstream Consumers)

Domains that consume J03 output. Per J00, "depends on" means requiring an
upstream capability; J03 produces approved demand that these domains consume,
so they are not dependencies of J03:

- J01 consumes released requisitions for downstream sourcing.
- J04 consumes released requisitions for order creation after release.

---

# 8. Completion Criteria

J03 is complete when requisitions, approvals, release conditions, authorization semantics, audit evidence, tenant isolation, integration contracts, and domain events are defined without overlap with supplier, order, inventory, asset, or finance ownership.

---

# 9. Authorization & Segregation of Duties

J03 defines requisition authorization semantics while L2-A04 owns policy evaluation.

Authorization may consider:

- Tenant and institution.
- Requesting campus, department, programme, office, or operational unit.
- Item category and controlled-resource classification.
- Estimated value band.
- Funding source and budget-validation reference.
- Requesting actor role.
- Approval authority and delegated authority.
- Requisition lifecycle state.

Representative SoD rules include:

```text
Requester != Final Approver
Budget Validator != Requisition Requester where configured
Emergency Requisition Approver requires explicit emergency authority
```

---

# 10. Audit & Evidence

J03 produces domain evidence for:

- Requisition creation and submission.
- Line addition, amendment, cancellation, or withdrawal.
- Budget-validation request and response reference.
- Policy-validation result.
- Approval, rejection, return, release, and emergency override.
- Downstream sourcing or purchase-order release reference.

L2-A07 owns enterprise audit persistence, retention governance, access governance, and evidence integrity.

---

# 11. Tenant and Institution Isolation

Every requisition command, aggregate, event, read model, and audit reference shall preserve tenant and institution context.

A Requisition Identifier alone shall not authorize access.

Cross-institution requisition processing shall require explicit governed authorization.

---

# 12. Event and Workflow Integration

J03 owns the business meaning and timing of requisition events.

L2-A06 owns event transport, routing, retry, replay, and event registry governance.

L2-A05 owns workflow routing, approval tasks, escalation, timeout, and orchestration. J03 owns the requisition business state that results from workflow decisions.

---

# 13. Privacy, Retention, and Recovery

J03 shall minimize copied requester, student, employee, department, and financial information by referencing canonical identifiers from owning domains.

Requisition history shall remain reconstructable according to enterprise and institutional retention policy.

Retried commands shall be idempotent for submission, approval, rejection, release, cancellation, and amendment.

---

# 14. Scope

J03 owns creation and controlled change of the `PurchaseRequisition`, its demand lines and quantities, validation references, approval/return/rejection decisions, emergency authority, withdrawal/cancellation, and release as authoritative procurement demand. The explicit non-ownership table in §3 remains the boundary for supplier, sourcing, purchase order, receipt, inventory, movement, asset and Finance lifecycles.

# 15. Canonical Actors

| Actor | Existing responsibility |
|---|---|
| Requesting Actor / Requesting Domain | Originates institutional demand and supplies the required purpose, scope and item/quantity information. |
| Budget Validator | Supplies the Finance-owned budget-validation reference where policy requires it. |
| Requisition Reviewer / Approver | Reviews demand, policy, authority and validation evidence and records approval, return, or rejection. |
| Emergency Requisition Approver | Applies explicit emergency authority to the emergency requisition path. |

# 16. Domain Contracts

| Contract direction | Existing contract |
|---|---|
| Consumes | Demand context from the requesting domain and Finance-owned budget-validation references where required. |
| Consumes platform outcomes | A03 authentication context, A04 authorization decisions, A05 workflow outcomes, A06 event transport and A07 audit persistence. |
| Publishes | Approved/released Purchase Requisition identity and state to J01 for sourcing and J04 for downstream order creation. |
| Publishes events | The requisition facts enumerated in §6; J03 owns meaning and timing while A06 owns transport. |

# 17. Locked Decisions

1. J03 is the authoritative demand-control boundary for resources not fulfilled directly from stock.
2. J03 owns the Purchase Requisition business state produced by workflow decisions; A05 owns routing and human-task orchestration.
3. Finance retains budget and accounting truth; a budget-validation reference does not transfer Finance ownership to J03.
4. Released demand is consumed by J01 and J04 without transferring Purchase Requisition ownership.
5. Every requisition command, aggregate, event, read model and audit reference preserves tenant and institution context.

---

## Lifecycle

The requisition lifecycle is described as a flow in Section 1 (Purpose): Requesting Domain -> Purchase Requisition -> Budget/Policy Validation -> Approval Review -> Released for Procurement -> J01 Procurement Planning & Sourcing.

## Workflows

Requisition workflow steps are captured in Section 5 (Core Responsibilities): requisition creation, line/quantity management, budget and policy validation coordination, approval/rejection capture, and release publication to J01 and J04.

## States

This document does not define a discrete requisition state machine separate from the lifecycle flow in Section 1; requisition progression is expressed through the stages Submitted, Approved, Rejected, and Released for Procurement referenced in the Domain Events (Section 6) and Audit & Evidence (Section 10).

## Rules / Invariants

Requisition rules are stated across the document rather than consolidated: SoD rules (Section 9), tenant/institution isolation rules (Section 11), and idempotency guarantees for submission, approval, rejection, release, cancellation, and amendment (Section 13).
