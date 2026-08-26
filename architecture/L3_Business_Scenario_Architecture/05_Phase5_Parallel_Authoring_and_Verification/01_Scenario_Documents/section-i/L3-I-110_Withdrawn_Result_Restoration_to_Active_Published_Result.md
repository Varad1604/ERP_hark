# Withdrawn Result Restoration to Active Published Result

# 1. Scenario Metadata

**Scenario ID:** L3-I-110  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Result Publication  
**Participating Domains:** I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-I10 — Result Publication Workflows; L3-I-109; Platform Foundation authorization, event and audit architecture  
**Referenced By:** authorized publication consumers

---

# 2. Scenario Purpose
Restore governed official availability of a previously withdrawn Published Result when restoration is authorized and the underlying approved academic result remains valid.

# 3. Business Outcome
The withdrawn result again becomes the active official publication without recomputing academic outcomes or erasing the withdrawal history.

# 4. Scenario Boundary and Scope
**Start:** An authorized restoration decision applies to a withdrawn published result. **End:** I10 has restored official availability after readiness and authorization checks. If academic content changed during withdrawal, replacement/republish applies instead.

# 5. Ownership and Participating Domains
I10 owns publication restoration; I09 retains academic-result ownership.

# 6. Business Actors, Roles and Responsibilities
**Institutional Publication Officer** coordinates restoration; **Examination Administrator** verifies the source result and publication scope; **Controller of Examination / Academic Registrar** authorize where policy requires; **Department Head / Faculty Dean** participate only where configured; **Candidate** is an authorized consumer, not a restoration authority.

# 7. Preconditions and Entry Criteria
A historically published result is currently withdrawn; restoration authority is valid; the underlying approved academic result remains authoritative and unchanged.

# 8. Trigger
An authorized institutional decision restores the withdrawn published result.

# 9. Main Business Flow
1. Verify restoration authority, prior withdrawal and affected publication scope.
2. Confirm the underlying I09 approved result is still the current academic outcome.
3. Reapply publication readiness, visibility and authorization checks.
4. Restore official availability of the governed published result.
5. Preserve withdrawal-to-restoration history and evidence.

# 10. Alternate Business Flows
If the academic result changed while the publication was withdrawn, do not restore stale content; use the corrected-result replacement/republish path.

# 11. Failure, Interruption and Business Recovery
If current academic validity or restoration authority cannot be established, the result remains withdrawn.

# 12. Business Rules and Decision Constraints
Restoration does not recompute academic content; prior withdrawal history remains immutable; visibility remains policy-governed.

# 13. State and Lifecycle Impact
The publication returns to current official availability under I10 governance without creating a new academic-result lifecycle state.

# 14. Business Events and Cross-Domain Collaboration
No restoration-specific event is invented absent a canonical L2 event. The prior `ExaminationResultPublished` fact remains historical evidence of original publication.

# 15. Business Authorization, Approval and Audit Obligations
Restoration authority, source-result validity, prior withdrawal and restored availability are auditable.

# 16. Completion Criteria and Postconditions
The correct approved result is again actively published and the full publication-withdrawal-restoration history remains reconstructable.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains.

**Architecture assessment:** `PASS`
