# Examination Exception Resolution to Post-Resolution Computed Result

# 1. Scenario Metadata

**Scenario ID:** L3-I-096  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Result Processing & Computation  
**Participating Domains:** I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-I09 — Result Processing & Computation Workflows; authoritative examination-exception resolution; Platform Foundation authorization, event and audit architecture  
**Referenced By:** I09 approval and I10 publication

---

# 2. Scenario Purpose
Resume or recompute result processing after an authoritative examination exception has been resolved.

# 3. Business Outcome
A post-resolution computed result reflects the authoritative exception resolution and applicable academic regulations.

# 4. Scenario Boundary and Scope
**Start:** The owning examination workflow establishes a conclusive exception resolution. **End:** I09 has incorporated the permitted consequence into a validated computed result. Exception resolution itself remains out of scope.

# 5. Ownership and Participating Domains
I09 owns result computation; the source examination workflow owns exception resolution.

# 6. Business Actors, Roles and Responsibilities
**Result Processing Officer** verifies and applies the resolution consequence; **Examination Administrator** validates scope; **Controller of Examination**, **Academic Registrar**, **Department Head**, **Faculty Dean**, **Academic Committee** participate only where required; **Candidate** has no self-approval authority.

# 7. Preconditions and Entry Criteria
A conclusive authoritative exception resolution exists and result processing can determine its academic consequence.

# 8. Trigger
An examination exception affecting result processing is conclusively resolved by its canonical owner.

# 9. Main Business Flow
1. Verify the resolution and affected candidate/examination scope.
2. Determine its permitted effect on computation.
3. Resume/recompute the affected result from authoritative inputs.
4. Validate the post-resolution result.
5. Complete applicable approval/finalisation.
6. Preserve traceability to the exception resolution.

# 10. Alternate Business Flows
If the resolution has no academic-result impact, normal I09 processing resumes unchanged.

# 11. Failure, Interruption and Business Recovery
An unresolved or contradictory exception keeps affected result processing from final approval.

# 12. Business Rules and Decision Constraints
I09 does not decide the exception; only authoritative resolutions are consumed; source marks remain immutable; publication remains I10-owned.

# 13. State and Lifecycle Impact
The result resumes from its last valid I09 lifecycle position and proceeds through validation/approval as applicable.

# 14. Business Events and Cross-Domain Collaboration
Only canonical I09 events are published for established result facts; exception-resolution events remain source-owned.

# 15. Business Authorization, Approval and Audit Obligations
Evidence links exception, resolution, computation consequence, validation and approval.

# 16. Completion Criteria and Postconditions
The authoritative computed result reflects the resolved exception and is positioned for its next governed I09 step.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains.

**Architecture assessment:** `PASS`
