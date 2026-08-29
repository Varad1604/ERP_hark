# Approved Marks Correction to Recomputed Authoritative Result

# 1. Scenario Metadata

**Scenario ID:** L3-I-088  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Result Processing & Computation  
**Participating Domains:** I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-I09 — Result Processing & Computation Workflows; authoritative approved-marks correction from the owning examination workflow; Platform Foundation authorization, event and audit architecture  
**Referenced By:** I09 approval and I10 publication

---

# 2. Scenario Purpose
Govern I09 recomputation after an authorized upstream correction establishes new Final Approved Marks.

# 3. Business Outcome
The affected authoritative result is recomputed from the corrected Final Approved Marks while prior computation history remains traceable.

# 4. Scenario Boundary and Scope
**Start:** Corrected Final Approved Marks are authoritative. **End:** I09 has recomputed and validated the affected result and completed required approval/finalisation. I09 does not perform the marks correction itself.

# 5. Ownership and Participating Domains
The marks owner retains evaluation ownership; I09 owns only the resulting recomputation and result lifecycle.

# 6. Business Actors, Roles and Responsibilities
**Result Processing Officer** verifies corrected marks and recomputes; **Examination Administrator** validates scope; **Controller of Examination**, **Academic Registrar**, **Department Head**, **Faculty Dean**, **Academic Committee** participate as required by the configured I09 approval path; **Candidate** has no approval authority.

# 7. Preconditions and Entry Criteria
A prior computed result exists; corrected Final Approved Marks have completed their owning authorization; regulations and candidate association are valid.

# 8. Trigger
Authoritative corrected Final Approved Marks become available to I09.

# 9. Main Business Flow
1. Verify corrected marks and source authority.
2. Preserve the prior computation and its source-mark references.
3. Recompute only affected academic outcomes under applicable regulations.
4. Revalidate resulting grade, credit and aggregate outcomes.
5. Complete applicable institutional approval/finalisation.
6. Preserve full correction-to-recomputation traceability.

# 10. Alternate Business Flows
If corrected marks do not alter the result, retain the authoritative result while recording the verified no-change recomputation outcome.

# 11. Failure, Interruption and Business Recovery
If corrected marks are not authoritative or remain disputed, I09 does not replace the result.

# 12. Business Rules and Decision Constraints
I09 consumes only Final Approved Marks; it cannot alter evaluation history; publication remains I10-owned; historical computations are not silently overwritten.

# 13. State and Lifecycle Impact
The affected result re-enters the appropriate I09 computation stage and proceeds through validation, approval and finalisation as applicable.

# 14. Business Events and Cross-Domain Collaboration
Only canonical I09 events are emitted when their facts occur; L3 introduces no `MarksCorrected` or `ResultRecomputed` event unless defined by its canonical source.

# 15. Business Authorization, Approval and Audit Obligations
Evidence links old and corrected marks references, prior and recomputed results, validation and approval/finalisation decisions.

# 16. Completion Criteria and Postconditions
The authoritative result reflects corrected Final Approved Marks and remains traceable to both prior computation and correction authority.

# 17. Open Issues, Assumptions and Deferred Questions
No L3-local issue remains.

**Architecture assessment:** `PASS`
