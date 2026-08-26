# Institutional Demand to Approved Purchase Requisition Release

# 1. Scenario Metadata

**Scenario ID:** L3-J-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Purchase Requisition & Approval  
**Participating Domains:** F, J, VAR:Requesting Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-J03 — Purchase Requisition & Approval Workflows; Finance budget-validation reference where policy requires; requesting-domain demand context; Platform Foundation authorization, workflow, event and audit architecture  
**Referenced By:** J01 — Procurement Planning & Sourcing; J04 — Purchase Order Management

---

# 2. Scenario Purpose
Convert an institutional demand into an approved Purchase Requisition released for downstream procurement while preserving requesting-domain, Finance and procurement ownership boundaries.

# 3. Business Outcome
A governed Purchase Requisition is submitted, validated, approved and released for procurement as the authoritative J03 demand-control outcome.

# 4. Scenario Boundary and Scope
**Start:** A Requesting Actor / Requesting Domain establishes institutional demand requiring procurement. **End:** J03 releases the approved Purchase Requisition for J01/J04 consumption. Out of scope: supplier selection, sourcing, purchase order creation, receipt, inventory, accounting/payment, UI/API/database design.

# 5. Ownership and Participating Domains
J03 owns the `PurchaseRequisition`, requisition content, approval consequence and release state. The Requesting Domain owns its demand context. Finance owns budget truth and supplies only a budget-validation reference where required. J01/J04 consume released demand without acquiring J03 ownership.

# 6. Business Actors, Roles and Responsibilities
- **Requesting Actor / Requesting Domain:** originates demand and supplies purpose, scope, items and quantities.
- **Budget Validator:** supplies the Finance-owned budget-validation reference where policy requires it.
- **Requisition Reviewer / Approver:** reviews demand, policy, authority and validation evidence and records the approval decision.

`Requester != Final Approver`. `Budget Validator != Requisition Requester` where configured.

# 7. Preconditions and Entry Criteria
The institutional demand is legitimate and identifiable; requesting scope is authorized; required item/quantity/purpose information is available; budget validation can be obtained where policy requires it.

# 8. Trigger
A Requesting Actor / Requesting Domain submits an institutional demand requiring purchase requisition control.

# 9. Main Business Flow
1. The Requesting Actor supplies demand purpose, organizational scope, item/quantity information and required context.
2. J03 establishes the Purchase Requisition and its demand lines.
3. J03 records `PurchaseRequisitionSubmitted` when submission is complete.
4. Where policy requires, the Budget Validator supplies the Finance-owned budget-validation reference; J03 does not rewrite budget truth.
5. J03 coordinates policy and authorization validation through governed platform outcomes while retaining requisition business-state ownership.
6. The Requisition Reviewer / Approver reviews demand, validations, authority and policy compliance under segregation-of-duties rules.
7. On approval, J03 establishes the approved requisition and publishes `PurchaseRequisitionApproved`.
8. After release conditions are satisfied, J03 releases the requisition for downstream procurement and publishes `PurchaseRequisitionReleasedForProcurement`.
9. J01/J04 consume the released requisition identity/state without transfer of Purchase Requisition ownership.

# 10. Alternate Business Flows
Where budget validation is not required by policy, the requisition proceeds through applicable policy/authority review without a Finance validation reference. A returned or rejected requisition follows the corresponding governed J03 path rather than being treated as approved.

# 11. Failure, Interruption and Business Recovery
Missing demand information, failed required budget validation or authorization, or incomplete approval prevents release. The requisition remains within J03 governance until corrected, returned, rejected, withdrawn or cancelled according to its applicable business path.

# 12. Business Rules and Decision Constraints
Requester cannot be Final Approver; Finance retains budget/accounting truth; J03 owns requisition business state resulting from workflow decisions; A05 owns routing, not requisition state; released demand may be consumed by J01/J04 without ownership transfer.

# 13. State and Lifecycle Impact
The source defines requisition progression through `Requesting Domain → Purchase Requisition → Budget / Policy Validation → Approval Review → Released for Procurement`, with Submitted, Approved, Rejected and Released-for-Procurement stages referenced by canonical events/evidence. No extra discrete state machine is invented.

# 14. Business Events and Cross-Domain Collaboration
J03 publishes `PurchaseRequisitionSubmitted`, `PurchaseRequisitionApproved` and `PurchaseRequisitionReleasedForProcurement` when those facts occur. A06 owns event transport. Finance contributes budget-validation references; J01/J04 consume approved/released facts.

# 15. Business Authorization, Approval and Audit Obligations
Authorization may consider institution, requesting scope, category, value band, funding source, actor role, authority/delegation and requisition lifecycle. Audit evidence preserves creation, submission, validation references, approval and release.

# 16. Completion Criteria and Postconditions
An approved Purchase Requisition is released for procurement; required validations and SoD controls are satisfied; J03 remains canonical owner; downstream consumers receive the governed released demand.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains. The scenario deliberately uses the canonical J03 actors instead of generic placeholder roles.

**Architecture assessment:** `PASS`
