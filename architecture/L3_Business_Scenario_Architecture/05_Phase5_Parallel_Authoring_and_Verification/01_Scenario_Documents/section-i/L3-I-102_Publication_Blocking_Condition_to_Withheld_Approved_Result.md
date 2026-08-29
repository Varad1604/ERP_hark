# Publication Blocking Condition to Withheld Approved Result

# 1. Scenario Metadata

**Scenario ID:** L3-I-102  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Result Publication  
**Participating Domains:** C, F, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-I10 — Result Publication Workflows; authoritative C/F blocking facts where applicable; Platform Foundation authorization, event and audit architecture  
**Referenced By:** L3-I-103

---

# 2. Scenario Purpose
Prevent official publication of an approved result while a valid publication-blocking condition remains unresolved.

# 3. Business Outcome
The approved academic result remains unchanged but is withheld from official publication with the blocking basis preserved.

# 4. Scenario Boundary and Scope
**Start:** I10 identifies an authoritative condition that prevents publication. **End:** Publication is conclusively held for the affected scope. C/F source facts remain owned outside I10; I09 result computation is unchanged.

# 5. Ownership and Participating Domains
I10 owns the publication consequence. C/F retain ownership of any student or finance condition supplied to the decision.

# 6. Business Actors, Roles and Responsibilities
**Institutional Publication Officer** verifies and applies the hold; **Examination Administrator** validates scope; **Controller of Examination / Academic Registrar** exercise publication-governance authority where required; **Department Head / Faculty Dean** participate only per configured I10 policy; **Candidate** cannot waive the hold.

# 7. Preconditions and Entry Criteria
An approved publication-ready result exists and the blocking condition can be authoritatively verified.

# 8. Trigger
A publication-blocking condition becomes effective for an approved result or publication scope.

# 9. Main Business Flow
1. Verify the condition and canonical source.
2. Determine its affected publication scope under institutional policy.
3. Prevent official publication for that scope without changing the approved result.
4. Preserve blocking reason and authority evidence.
5. Wait for authoritative resolution before release.

# 10. Alternate Business Flows
If the condition affects only selected candidates/scopes, unaffected authorized results may continue under the publication plan.

# 11. Failure, Interruption and Business Recovery
Unclear source evidence results in no assumed release; the affected publication remains held pending authoritative clarification.

# 12. Business Rules and Decision Constraints
I10 cannot modify approved academic outcomes; only authorized publication may proceed; source-owned C/F facts are referenced, not rewritten.

# 13. State and Lifecycle Impact
Publication remains at its last valid pre-publication I10 lifecycle position. No discrete withheld state is invented if L2-I10 does not define one.

# 14. Business Events and Cross-Domain Collaboration
`ExaminationResultPublished` is not emitted for withheld scope. No new `PublicationWithheld` event is invented.

# 15. Business Authorization, Approval and Audit Obligations
The hold, source evidence and later release decision are auditable.

# 16. Completion Criteria and Postconditions
The approved result remains academically authoritative but is not officially published while the blocker is effective.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains.

**Architecture assessment:** `PASS`
