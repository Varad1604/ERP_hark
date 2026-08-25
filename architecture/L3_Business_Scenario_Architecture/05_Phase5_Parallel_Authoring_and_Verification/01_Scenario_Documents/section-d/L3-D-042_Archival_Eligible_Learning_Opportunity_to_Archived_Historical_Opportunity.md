# Archival-Eligible Learning Opportunity to Archived Historical Opportunity

# 1. Scenario Metadata

**Scenario ID:** L3-D-042  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0762 / CAN-L3-D-042 — split child B

---

# 2. Scenario Purpose

Archive a completed Learning Opportunity after its academic/operational obligations are closed, preserving it as governed historical truth.

# 3. Business Outcome

The opportunity reaches Archived, is removed from active availability and remains historically traceable with required audit/retention evidence.

# 4. Scenario Boundary and Scope

Starts when the opportunity is archival-eligible after enrollment/recognition obligations and holds are resolved; ends at Archived with `LearningOpportunityArchived` published. In scope: eligibility/hold checks, archival authorization where required, transition to historical state and preservation of evidence. Out of scope: deletion, archival with pending obligations, technical storage/search implementation.

# 5. Ownership and Participating Domains

D06 owns Learning Opportunity archival and historical opportunity evidence.

# 6. Business Actors, Roles and Responsibilities

Academic Affairs reviews academic closure and authorizes archival where policy requires. Historical preservation is a D06 business responsibility; `Enterprise Administrator` is not retained as an L3 technical/administrative execution actor unless separately established upstream.

# 7. Preconditions and Entry Criteria

Enrollment obligations are closed; recognition is completed/formally closed; no pending workflow/investigation/legal hold blocks archival.

# 8. Trigger

Academic/operational closure makes the Learning Opportunity archival-eligible.

# 9. Main Business Flow

1. D06 evaluates archival eligibility and blocking obligations.
2. Academic Affairs provides the required archival business decision where policy requires.
3. D06 preserves the complete opportunity/history and removes it from active availability.
4. D06 transitions to Archived and publishes `LearningOpportunityArchived`.

# 10. Alternate Business Flows

No alternate path reaches Archived if a blocking obligation exists; blocked/rejected archival belongs to Section 11.

# 11. Failure, Interruption and Business Recovery

Pending enrollment/recognition/hold blocks archival until resolved. Missing authority retains prior state. Interruption preserves last authoritative opportunity position/history until continuation is valid.

# 12. Business Rules and Decision Constraints

D06 alone owns archival state; historical evidence is not deleted; active obligations/holds prevent archival; technical storage/retention mechanisms are deferred.

# 13. State and Lifecycle Impact

Learning Opportunity: archival-eligible operational position → Archived. Owner D06. Source L2-D06 W07/Category A.

# 14. Business Events and Cross-Domain Collaboration

D06 publishes `LearningOpportunityArchived` under L2-D06 W07/D06-E05.

# 15. Business Authorization, Approval and Audit Obligations

Required archival decision follows D06 policy. Audit retains eligibility assessment, outstanding-obligation/hold check, authority/decision, prior state and archival fact.

# 16. Completion Criteria and Postconditions

Opportunity is Archived and absent from active availability; historical truth remains traceable; D06 ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Storage, indexing, retention implementation and infrastructure recovery are deferred.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
