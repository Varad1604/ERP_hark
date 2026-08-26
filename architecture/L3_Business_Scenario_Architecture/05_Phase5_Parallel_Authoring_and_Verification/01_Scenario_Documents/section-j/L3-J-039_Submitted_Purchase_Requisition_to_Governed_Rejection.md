# Submitted Purchase Requisition to Governed Rejection

# 1. Scenario Metadata

**Scenario ID:** L3-J-039  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Purchase Requisition & Approval  
**Participating Domains:** F, J, VAR:Requesting Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-J03 — Purchase Requisition & Approval Workflows; Finance budget-validation reference where required; requesting-domain demand context; Platform Foundation authorization, workflow, event and audit architecture  
**Referenced By:** requesting-domain demand governance and downstream procurement eligibility

---

# 2. Scenario Purpose
Govern rejection of a submitted Purchase Requisition when demand, validation, policy or authority requirements are not satisfied.

# 3. Business Outcome
The submitted requisition reaches a governed rejection outcome, remains auditable, and is not released for procurement.

# 4. Scenario Boundary and Scope
**Start:** A submitted Purchase Requisition enters J03 review. **End:** The Requisition Reviewer / Approver records a conclusive rejection and J03 publishes the rejection fact. Out of scope: correcting Finance truth, sourcing, PO creation or technical workflow mechanics.

# 5. Ownership and Participating Domains
J03 owns rejection and Purchase Requisition state. Finance owns any budget-validation truth; Requesting Domain owns originating demand context.

# 6. Business Actors, Roles and Responsibilities
- **Requesting Actor / Requesting Domain:** supplied the submitted demand and receives the governed rejection consequence.
- **Budget Validator:** supplies Finance-owned budget-validation reference where policy requires it.
- **Requisition Reviewer / Approver:** reviews the requisition and records rejection when requirements are not satisfied.

Requester cannot be Final Approver; budget validation does not transfer Finance ownership.

# 7. Preconditions and Entry Criteria
A Purchase Requisition has been submitted and sufficient review/validation evidence exists to make a governed decision.

# 8. Trigger
Review identifies grounds requiring rejection of the submitted Purchase Requisition.

# 9. Main Business Flow
1. Requisition Reviewer / Approver reviews the submitted demand, policy, authority and applicable validation evidence.
2. Where Finance validation is required, use the authoritative Budget Validator reference without modifying Finance truth.
3. Determine that approval criteria are not satisfied and rejection is the governed outcome.
4. J03 records the rejection decision and reason/evidence.
5. J03 publishes `PurchaseRequisitionRejected`.
6. Prevent the rejected requisition from being released to J01/J04 as approved demand.
7. Preserve the Requesting Domain context and complete audit history.

# 10. Alternate Business Flows
If the review outcome is return for clarification/amendment rather than rejection, follow the applicable J03 return/amendment path and do not publish the rejection event.

# 11. Failure, Interruption and Business Recovery
If decision authority or validation evidence is incomplete, do not infer rejection or approval; keep the requisition under governed review until a valid business decision is available.

# 12. Business Rules and Decision Constraints
Only J03 owns the requisition decision; requester cannot approve their own demand; Finance validation is a reference; rejected requisitions cannot be released for procurement.

# 13. State and Lifecycle Impact
The requisition reaches the source-referenced Rejected stage. No additional rejected sub-state is invented.

# 14. Business Events and Cross-Domain Collaboration
J03 publishes `PurchaseRequisitionRejected`; it does not publish `PurchaseRequisitionApproved` or `PurchaseRequisitionReleasedForProcurement` for the rejected outcome.

# 15. Business Authorization, Approval and Audit Obligations
Review authority, validation references, rejection reason and decision evidence remain auditable.

# 16. Completion Criteria and Postconditions
The requisition is conclusively rejected, not released downstream, and fully traceable to its demand and validation context.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains.

**Architecture assessment:** `PASS`
