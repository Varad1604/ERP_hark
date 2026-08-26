# Recompute Result Before Institutional Approval

# 1. Scenario Metadata

**Scenario ID:** L3-I-087  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Result Processing & Computation  
**Participating Domains:** I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-I09 — Result Processing & Computation Workflows; Platform Foundation authorization, event and audit architecture  
**Referenced By:** I09 result approval and I10 publication

---

# 2. Scenario Purpose
Govern authorized recomputation of a computed but not yet institutionally approved result.

# 3. Business Outcome
The pre-approval result is recomputed, revalidated and presented for the applicable institutional approval path with full traceability to the prior computation.

# 4. Scenario Boundary and Scope
**Start:** A valid reason for recomputation is established before institutional approval. **End:** A corrected computed result has completed validation and is approval-eligible. No approved result or publication is overwritten.

# 5. Ownership and Participating Domains
I09 retains exclusive ownership of computation, validation and approval-state consequences.

# 6. Business Actors, Roles and Responsibilities
**Result Processing Officer** performs authorized recomputation and validation; **Examination Administrator** supports scope readiness; **Controller of Examination**, **Academic Registrar**, **Department Head**, **Faculty Dean** and **Academic Committee** participate only according to the configured I09 approval path; **Candidate** cannot approve their own result.

# 7. Preconditions and Entry Criteria
The result has been computed but not institutionally approved; the recomputation basis is authorized and traceable; Final Approved Marks and regulations are authoritative.

# 8. Trigger
An authorized pre-approval recomputation requirement is established.

# 9. Main Business Flow
1. Verify recomputation authority and basis.
2. Preserve the prior computed result as historical evidence.
3. Re-run applicable I09 computation using authoritative marks and regulations.
4. Validate the recomputed outcome.
5. Present it to the configured institutional approval path.
6. Preserve recomputation evidence and canonical events where their facts occur.

# 10. Alternate Business Flows
If recomputation confirms the prior result, the validated result proceeds unchanged. If source marks require correction, recomputation pauses until the marks owner completes the authorized correction.

# 11. Failure, Interruption and Business Recovery
Unverified recomputation grounds or unavailable authoritative inputs prevent replacement of the prior computed outcome.

# 12. Business Rules and Decision Constraints
Final Approved Marks are immutable to I09; validation precedes approval; no published result is changed by this pre-publication scenario.

# 13. State and Lifecycle Impact
The result re-enters the applicable computation stage before `Result Approval`, then progresses through the locked I09 lifecycle. No new state is introduced.

# 14. Business Events and Cross-Domain Collaboration
Use only canonical I09 events such as `Subject Result Computed`, `Semester Result Computed`, `CGPA Calculated`, `Result Approved` and `Result Ready for Publication` when applicable; no recomputation-specific event is invented.

# 15. Business Authorization, Approval and Audit Obligations
Recomputation and subsequent approval require appropriate institutional authority; the previous computation, reason, recalculated outcome and approvals remain auditable.

# 16. Completion Criteria and Postconditions
A traceable validated recomputed result is available for institutional approval and the prior computation remains historically reconstructable.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains.

**Architecture assessment:** `PASS`
