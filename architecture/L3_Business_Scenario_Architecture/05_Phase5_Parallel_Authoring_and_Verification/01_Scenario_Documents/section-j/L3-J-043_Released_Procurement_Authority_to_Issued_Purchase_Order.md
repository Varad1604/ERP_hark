# Released Procurement Authority to Issued Purchase Order

## 1. Scenario Metadata

**Scenario ID:** L3-J-043  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Purchase Order Management  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-24  
**Depends On:** `architecture/L2_Domain_Workflow_Architecture/section-j/L2-J04_Purchase_Order_Management.md`; L2-J00; applicable L2-F06 and L2-F14 Finance references; frozen Stage 3 Section J canonical row  
**Referenced By:** J05 receiving and applicable Finance processing as identified by locked L2-J04; exact L3 references only where canonically registered  

## 2. Scenario Purpose

This scenario defines the complete institutional business outcome represented by **Released Procurement Authority to Issued Purchase Order** while preserving the locked Section J ownership, lifecycle, authority, evidence, event, and dependency boundaries.

## 3. Business Outcome

An authorised Purchase Order is issued as J04's canonical institutional supplier commitment.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

Released procurement authority is available with valid sourcing, supplier, requisition-release, and policy-required Finance validation references.

### 4.2 End Boundary

The scenario ends when the Purchase Order is issued as the authoritative supplier commitment and the source-supported issue fact is available to receiving and Finance, or when a source-defined blocking condition prevents issuance.

### 4.3 In Scope

- Purchase Order draft creation against released procurement authority.
- Validation of sourcing, supplier, requisition-release, and policy-required Finance references.
- Source-defined purchase-order approval and segregation of duties.
- Purchase Order issue and publication of the resulting canonical business fact.
- Material cross-domain hand-offs required by locked L2-J04.

### 4.4 Out of Scope

- Purchase requisition and sourcing decisions, supplier identity, goods receipt, inventory, assets, invoicing, accounting, and payment remain with their canonical owners.
- Purchase Order amendment, cancellation, and closure are owned by J04 but are separate lifecycle outcomes beyond this scenario's issued-order boundary.
- APIs, data design, software components, UI behaviour, technical access control, deployment, transport, and technical recovery.

## 5. Ownership and Participating Domains

| Domain / Capability | Responsibility in Scenario |
|---|---|
| Purchase Order Management (J04) | Owns `PurchaseOrder`, its approval/issue state, purchase-order evidence, and the final issued supplier-commitment outcome. |
| J01 — Procurement Planning & Sourcing | Owns the sourcing recommendation/decision consumed by J04. |
| J02 — Vendor & Supplier Management | Owns supplier identity and supplier status consumed by J04. |
| J03 — Purchase Requisition & Approval | Owns the released requisition/procurement authority consumed by J04. |
| Finance (F) | Owns Finance validation references where policy requires them and later Finance processing; it does not own Purchase Order state. |
| J05 — Goods Receipt, Inspection & Acceptance | Consumes issued Purchase Order facts for its own receiving lifecycle; it does not own Purchase Order state. |

Participation and consumption do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Purchase Order Creator | Creates the order draft from released requisition and sourcing references. | Cannot act as Purchase Order Approver for the same governed approval where the J04 maker-checker rule applies. |
| Purchase Order Approver | Applies value-, scope-, supplier-, and authority-sensitive approval. | Exercises only the source-supported approval authority defined by J04. |

Supplier acknowledgement exists in the wider J04 lifecycle but is not made a required scenario actor unless it is material to the institution's source-defined issuance path.

## 7. Preconditions and Entry Criteria

- Tenant and institution context is established.
- J03 has supplied authoritative requisition-release / procurement-authority context.
- J01 sourcing recommendation is authoritative where applicable.
- J02 supplier identity and status are authoritative and eligible for the intended commitment.
- Finance validation reference is available where policy requires it.
- The Purchase Order Creator and Purchase Order Approver can satisfy J04's applicable authorization and segregation-of-duty constraints.

## 8. Trigger

Released procurement authority is available with the authoritative upstream references required by locked L2-J04.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the Purchase Order draft from released requisition/procurement authority and applicable sourcing references. | Purchase Order Creator / J04 | A traceable Purchase Order draft exists. |
| 2 | Validate supplier identity/status, sourcing recommendation, requisition-release context, tenant/institution context, and Finance validation reference where policy requires it. | J04 using canonical source-domain facts | The draft is eligible for approval or is blocked. |
| 3 | Evaluate purchase-order authority for the value, scope, supplier, controlled-resource classification, funding/budget references, and delegated authority where applicable. | Purchase Order Approver / J04 | An attributable approval decision exists. |
| 4 | Enforce the source-defined segregation of duties, including `Purchase Order Creator != Purchase Order Approver`. | J04 / applicable authorization governance | Maker-checker integrity is preserved. |
| 5 | Record approval and issue the Purchase Order as J04's canonical institutional supplier commitment. | J04 | The Purchase Order reaches the issued lifecycle position. |
| 6 | Publish the issued Purchase Order fact for J05 receiving and Finance processing without starting or owning either downstream lifecycle. | J04 | Downstream consumers have authoritative Purchase Order context. |

## 10. Alternate Business Flows

### 10.1 Upstream Reference Validation Fails

If the J01 sourcing recommendation, J02 supplier identity/status, J03 requisition-release context, or required Finance validation reference is missing, stale, or invalid, J04 does not issue the Purchase Order. The relevant canonical owner resolves its own fact before J04 revalidates the draft.

### 10.2 Purchase Order Approval Is Not Granted

If the Purchase Order Approver does not grant approval, the Purchase Order remains unissued. Any permitted correction or resubmission stays within J04's governed pre-issue lifecycle; the scenario cannot claim its issued-order outcome.

### 10.3 Segregation of Duties Cannot Be Satisfied

If the required independent approval cannot be achieved, issue is blocked until an authorized source-supported approver is available. No local override is assumed.

### 10.4 Post-Issue Change Is Required

If a change is required after issue, this scenario ends at the issued Purchase Order. J04's governed amendment, cancellation, or closure lifecycle applies separately and the issued order history is not destructively rewritten.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Mandatory upstream fact or validation is unavailable | Issue cannot be authorized. | Preserve the Purchase Order's last valid pre-issue position and obtain an authoritative correction from the owning domain. | J04 and source-owning domain |
| Approval authority or SoD condition is not satisfied | The order cannot be issued. | Obtain a source-supported authorized approver or leave the order unissued; do not infer an override. | J04 / applicable authorization governance |
| A source fact changes before issue | Earlier validation may no longer be reliable. | Revalidate the affected source reference before approval/issue. | J04 and source-owning domain |
| Downstream receiving or Finance processing is unavailable after issue | The issued Purchase Order remains authoritative; downstream processing is incomplete. | Preserve the issued-order fact and allow J05/Finance to resume under their own business lifecycle. | J05 / Finance respectively |

No API retry, message replay, database rollback, or other technical recovery behaviour is defined at L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Scenario Effect | Owner / Source |
|---|---|---|
| J04 is the canonical institutional commitment boundary for external suppliers. | Only J04 may establish the issued Purchase Order outcome. | J04 §17 |
| J04 owns Purchase Order state; J03 owns requisition state, J01 owns sourcing decisions, and J02 owns supplier identity. | Cross-domain validation never transfers or duplicates ownership. | J04 §3, §17 |
| `Purchase Order Creator != Purchase Order Approver`. | Independent approval is required where the J04 maker-checker rule applies. | J04 §9 |
| Supplier Selector != Final PO Approver where configured. | The configured segregation-of-duty control remains conditional and is not generalized beyond J04. | J04 §9 |
| Finance validation references are consumed where policy requires them. | Finance participation is conditional and does not transfer Purchase Order ownership. | J04 §7, §16 |
| Goods receipt and financial settlement are downstream consequences and never destructively modify issued order history. | This scenario ends before those downstream lifecycles. | J04 §17 |
| Tenant and institution context must be preserved. | The Purchase Order outcome is valid only within its governed institutional context. | J04 §11, §17 |

No local threshold, state, actor, event, publisher, or contract is introduced.

## 13. State and Lifecycle Impact

| Owned Aggregate | Relevant Lifecycle | Scenario Impact | Canonical Owner | Authoritative Source |
|---|---|---|---|---|
| PurchaseOrder | Draft → Approval / Validation → Issued | Establishes and approves the Purchase Order, then reaches the issued supplier-commitment position. | Purchase Order Management (J04) | J04 §1, §4, §14 |

Amendment, cancellation, and closure remain J04-owned lifecycle capabilities but are outside this scenario's end boundary. Goods Receipt and financial settlement are downstream and do not become Purchase Order states.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Published When | Scenario Use | Authoritative Source |
|---|---|---|---|---|
| `PurchaseOrderDrafted` | Purchase Order Management (J04) | The Purchase Order draft is established. | Records the canonical draft milestone. | J04 §6 |
| `PurchaseOrderApproved` | Purchase Order Management (J04) | The Purchase Order receives its governed approval. | Records the canonical approval milestone. | J04 §6 |
| `PurchaseOrderIssued` | Purchase Order Management (J04) | The Purchase Order is issued. | Communicates the authoritative supplier-commitment fact. | J04 §6 |

Validations, acknowledgements, approval-task routing, and business hand-offs remain distinct from these business events. J04 owns their business meaning and timing; platform capabilities do not acquire Purchase Order ownership.

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| J01 — Procurement Planning & Sourcing | J04 — Purchase Order Management | Sourcing recommendation / sourcing outcome reference | Provide the authoritative sourcing basis consumed by the Purchase Order. | J04 §7, §16 |
| J02 — Vendor & Supplier Management | J04 — Purchase Order Management | Supplier identity and supplier-status reference | Establish authoritative supplier context without transferring supplier ownership. | J04 §7, §16 |
| J03 — Purchase Requisition & Approval | J04 — Purchase Order Management | Requisition-release / released procurement-authority context | Authorize creation of the Purchase Order against an approved procurement requirement. | J04 §7, §16 |
| Finance (F) | J04 — Purchase Order Management | Finance validation reference where policy requires it | Supply the conditional Finance validation consumed by J04. | J04 §7, §16 |
| J04 — Purchase Order Management | J05 — Goods Receipt, Inspection & Acceptance | Issued Purchase Order fact | Provide receiving authority/context for J05's independent goods-receipt lifecycle. | J04 §16–§17 |
| J04 — Purchase Order Management | Finance (F) | Issued Purchase Order fact | Provide authoritative Purchase Order context for downstream Finance processing. | J04 §16–§17 |

The table expresses business contracts and hand-offs only. It does not define APIs, messages, schemas, synchronous calls, or transport mechanisms.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Authority Constraint | Authoritative Source |
|---|---|---|---|
| Create Purchase Order draft | Purchase Order Creator | Must act from released requisition/procurement authority and authoritative sourcing/supplier context. | J04 §15 |
| Approve Purchase Order | Purchase Order Approver | Must have authority for the value, scope, supplier, and applicable controlled-resource context. | J04 §9, §15 |
| Issue Purchase Order | J04 following governed approval | Issue occurs only after applicable validation, authority, and SoD requirements are satisfied. | J04 §§1, 9, 14 |

### 15.2 Separation of Duties, Overrides and Exceptions

- `Purchase Order Creator != Purchase Order Approver`.
- `Supplier Selector != Final PO Approver` only where configured by the governing policy.
- No local administrator override is introduced.
- Platform authorization and workflow-routing capabilities may support the process but do not own Purchase Order business state.

### 15.3 Business Audit and Evidence Obligations

J04 preserves evidence of draft creation, supplier/sourcing/requisition validation, applicable Finance validation, purchase-order approval, issue, actor authority, segregation of duties, and the receiving/Finance hand-off references. Enterprise audit persistence remains owned by A07.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Released requisition/procurement authority is authoritative.
- Required sourcing, supplier, institutional, and conditional Finance references have been validated.
- Applicable Purchase Order authority and segregation-of-duty requirements are satisfied.
- The Purchase Order has been approved and issued by J04.
- `PurchaseOrderIssued` and the source-supported J05/Finance hand-offs are available.

### 16.2 Postconditions

- The issued Purchase Order is J04's authoritative institutional supplier commitment.
- J03 retains requisition ownership, J01 sourcing ownership, J02 supplier ownership, J05 receiving ownership, and Finance its financial lifecycles.
- Goods receipt and Finance processing may proceed independently without rewriting Purchase Order history.
- Amendment, cancellation, and closure remain governed by J04 outside this scenario boundary.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

No material scenario-specific architecture issue is identified against locked L2-J04 after the contracts and alternate/recovery paths are made explicit.

### 17.2 Assumptions

No assumption overrides locked architecture. Conditional Finance validation and configured segregation-of-duty requirements retain their upstream qualifiers.

### 17.3 Deferred Questions

API, data, user-interface, workflow-engine, event-transport, audit-persistence, deployment, and other implementation/technical design concerns remain deferred to L4–L5.
