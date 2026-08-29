# Active Purchase Requisition to Governed Amended Demand

# 1. Scenario Metadata

**Scenario ID:** L3-J-040  
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
**Referenced By:** J03 approval/release and downstream procurement when subsequently released

---

# 2. Scenario Purpose
Govern an authorized amendment to an active Purchase Requisition while preserving J03 ownership, demand provenance and required validation/approval controls.

# 3. Business Outcome
The Purchase Requisition contains the governed amended demand and remains positioned for any validation, approval or release required by the amendment.

# 4. Scenario Boundary and Scope
**Start:** The Requesting Actor / Requesting Domain identifies an authorized amendment to an active requisition. **End:** J03 has incorporated the amendment and applied the required governance consequence. Out of scope: modifying source-domain demand truth, sourcing/PO changes, technical versioning implementation.

# 5. Ownership and Participating Domains
J03 owns Purchase Requisition lines, quantities and amendment history. The Requesting Domain owns the originating/changed business demand context.

# 6. Business Actors, Roles and Responsibilities
- **Requesting Actor / Requesting Domain:** supplies the amendment request, rationale and changed demand information.
- **Requisition Reviewer / Approver:** reviews the amended requisition where policy or lifecycle position requires renewed approval.
- **Budget Validator:** participates only if the amended demand requires a Finance-owned budget-validation reference; Finance is not a frozen participant here, so no Finance participation is asserted unless invoked by the applicable canonical contract outside this scenario boundary.

# 7. Preconditions and Entry Criteria
An active Purchase Requisition exists under J03; amendment authority and the changed demand can be verified.

# 8. Trigger
An authorized change to active institutional demand requires amendment of the Purchase Requisition.

# 9. Main Business Flow
1. Requesting Actor supplies the amendment rationale and changed item/quantity/purpose context.
2. J03 verifies that the amendment is permitted for the requisition's current governed position.
3. J03 preserves the prior requisition content as reconstructable history.
4. J03 updates the Purchase Requisition demand content under its own ownership.
5. Reapply policy/authorization and any required validation appropriate to the amended demand.
6. Where renewed review/approval is required, the Requisition Reviewer / Approver makes that decision under J03 SoD rules.
7. The amended requisition proceeds only to the next source-supported lifecycle stage after required governance completes.

# 10. Alternate Business Flows
If the requested change is not a valid amendment for the current lifecycle position, do not mutate the requisition; use the applicable withdrawal/cancellation or new-demand path.

# 11. Failure, Interruption and Business Recovery
Incomplete amendment context or missing authority prevents the amendment from becoming effective. The last valid requisition content remains authoritative.

# 12. Business Rules and Decision Constraints
J03 owns amendment history and Purchase Requisition content; requester cannot become Final Approver merely by requesting the amendment; downstream ownership is not implied.

# 13. State and Lifecycle Impact
L2-J03 does not define a discrete amendment state. The amended requisition remains within the existing J03 lifecycle and must satisfy any renewed validation/approval required by its governed position.

# 14. Business Events and Cross-Domain Collaboration
L2-J03 defines no canonical amendment event. This L3 scenario therefore does not invent one. Canonical submitted/approved/released events are published only when those defined facts occur after the amendment.

# 15. Business Authorization, Approval and Audit Obligations
Audit evidence preserves prior content, amendment request, authority, amended content and any renewed validation/approval.

# 16. Completion Criteria and Postconditions
The requisition accurately reflects the governed amended demand, prior content is reconstructable, and no downstream release occurs without required governance.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains. No unsupported amendment state or event has been introduced.

**Architecture assessment:** `PASS`
