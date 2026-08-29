# Purchase Requisition Withdrawal or Cancellation to Governed Termination

# 1. Scenario Metadata

**Scenario ID:** L3-J-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Purchase Requisition & Approval  
**Participating Domains:** J, VAR:Requesting Domain  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-J03 — Purchase Requisition & Approval Workflows; requesting-domain demand context; Platform Foundation authorization, workflow, event and audit architecture  
**Referenced By:** procurement eligibility and requisition history

---

# 2. Scenario Purpose
Govern withdrawal or cancellation of a Purchase Requisition so terminated demand cannot proceed to procurement while full requisition history remains reconstructable.

# 3. Business Outcome
The Purchase Requisition reaches a governed termination outcome and is no longer eligible for release/procurement under that demand request.

# 4. Scenario Boundary and Scope
**Start:** A valid withdrawal request or cancellation decision applies to an active requisition. **End:** J03 has conclusively terminated the requisition's procurement eligibility and preserved history. Out of scope: cancelling downstream purchase orders or reversing Finance/accounting facts.

# 5. Ownership and Participating Domains
J03 owns withdrawal/cancellation consequence and Purchase Requisition history. The Requesting Domain owns its original demand context.

# 6. Business Actors, Roles and Responsibilities
- **Requesting Actor / Requesting Domain:** may originate a governed withdrawal request for its demand where permitted.
- **Requisition Reviewer / Approver:** applies the authorized J03 review/decision responsibility when the lifecycle position or policy requires approval of termination.

No new `Cancellation Officer` or similar role is invented.

# 7. Preconditions and Entry Criteria
A Purchase Requisition exists; the requested termination is valid for its governed position; authority can be established.

# 8. Trigger
A governed withdrawal request or authorized cancellation decision requires termination of the Purchase Requisition.

# 9. Main Business Flow
1. Identify the requisition, termination basis and requesting/decision authority.
2. Verify that withdrawal/cancellation is permitted for the current J03 position.
3. Preserve all prior requisition, validation and approval evidence.
4. Apply the governed termination consequence under J03 ownership.
5. Prevent subsequent release as active procurement demand.
6. Preserve withdrawal/cancellation reason and authority for audit/reconstruction.

# 10. Alternate Business Flows
If the requisition has already progressed to downstream owned procurement/order commitments beyond a J03-only termination boundary, this scenario does not invent reversal of those downstream lifecycles; their owning workflows govern any required action.

# 11. Failure, Interruption and Business Recovery
If authority or lifecycle eligibility for termination is unresolved, the last valid requisition position remains authoritative until governance resolves the request.

# 12. Business Rules and Decision Constraints
J03 owns requisition termination; requester authority is governed and does not imply unrestricted cancellation; history cannot be silently deleted; termination does not reverse downstream domain-owned facts.

# 13. State and Lifecycle Impact
L2-J03 records cancellation/withdrawal as governed requisition history but does not define a separate canonical state machine for them. This scenario records conclusive termination without inventing a new state name.

# 14. Business Events and Cross-Domain Collaboration
L2-J03 defines no canonical withdrawal/cancellation event. This scenario therefore invents none. `PurchaseRequisitionReleasedForProcurement` is not published after governed termination.

# 15. Business Authorization, Approval and Audit Obligations
Termination basis, request/decision authority, prior lifecycle evidence and final procurement-ineligibility consequence are auditable.

# 16. Completion Criteria and Postconditions
The terminated requisition cannot be newly released for procurement under J03 and its complete history remains reconstructable.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains. Downstream reversals, if ever required, remain with their canonical owners.

**Architecture assessment:** `PASS`
