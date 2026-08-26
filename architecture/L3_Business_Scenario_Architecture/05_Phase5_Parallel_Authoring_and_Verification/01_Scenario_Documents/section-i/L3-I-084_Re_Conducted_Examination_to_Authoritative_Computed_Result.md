# Re-Conducted Examination to Authoritative Computed Result

# 1. Scenario Metadata

**Scenario ID:** L3-I-084  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Result Processing & Computation  
**Participating Domains:** I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-I09 — Result Processing & Computation Workflows; authoritative examination-conduct and marks-evaluation outcome supplying the valid re-conducted occurrence; applicable Platform Foundation authorization, event and audit architecture  
**Referenced By:** L2-I10 — Result Publication Workflows and authorized consumers of institutionally approved results

---

# 2. Scenario Purpose

Govern recomputation after a valid re-conducted examination occurrence so the authoritative result reflects the valid examination outcome without changing evaluation ownership or inventing a parallel result lifecycle.

---

# 3. Business Outcome

An authoritative result is computed using the valid re-conducted examination occurrence and progresses through the applicable I09 validation, approval and finalisation controls.

---

# 4. Scenario Boundary and Scope

**Start Boundary:** A valid re-conducted examination outcome and its Final Approved Marks are available for result processing.  
**End Boundary:** The resulting academic outcome has been recomputed under I09 and is in the applicable validated, approved or publication-ready position required by the frozen outcome.

**In scope:** consuming authoritative re-conducted marks; applying I09 academic regulations; recomputation; validation; approval/finalisation where required; audit evidence.  
**Out of scope:** conducting the examination; changing approved marks; publishing the official result; UI/API/database/technical recovery design.

---

# 5. Ownership and Participating Domains

I09 owns result computation, validation, approval and finalisation. The workflow that establishes the re-conducted examination and I08 marks ownership remain upstream. Participation or event consumption does not transfer ownership.

---

# 6. Business Actors, Roles and Responsibilities

| Actor | Responsibility in this scenario |
|---|---|
| Result Processing Officer | Verifies authoritative inputs and performs authorized recomputation and validation activities. |
| Examination Administrator | Supports examination-scope readiness and source completeness. |
| Controller of Examination | Exercises result approval/finalisation authority where assigned by institutional policy. |
| Academic Registrar | Participates in institutional result governance where the configured policy requires it. |
| Department Head / Faculty Dean / Academic Committee | Participate only where the configured institutional approval path requires their canonical I09 role. |
| Candidate | Is the subject of the result and cannot approve their own result. |

No placeholder `Authorized Initiator`, generic `Business Owner`, `Reviewer or Approver`, `Participating-Domain Provider`, or `Auditor` role is introduced.

---

# 7. Preconditions and Entry Criteria

- The re-conducted examination occurrence is authoritative under its owning workflow.
- Final Approved Marks for that occurrence are available from the marks-evaluation owner.
- Applicable academic regulations are active.
- Candidate and examination scope are valid.

---

# 8. Trigger

A valid re-conducted examination occurrence produces the authoritative Final Approved Marks required for result recomputation.

---

# 9. Main Business Flow

1. The Result Processing Officer verifies the re-conducted occurrence, candidate association and Final Approved Marks.
2. I09 resolves the applicable academic regulations and computation basis.
3. I09 recomputes subject, grade, credit, SGPA/CGPA and programme outcomes only to the extent applicable to the affected result.
4. The recomputed result is validated for computational and policy consistency.
5. The configured institutional approval path is completed using only canonical I09 actors where approval is required.
6. The result is finalised or made approval/publication-ready according to its applicable I09 lifecycle position.
7. I09 preserves traceability to the superseded computation and authoritative re-conducted inputs.

---

# 10. Alternate Business Flows

If the re-conducted occurrence changes only part of the candidate result, only the affected academic computation is recomputed while unaffected authoritative outcomes remain unchanged. If an institution-configured grace or benefit rule applies, it is applied through the normal I09 regulation path.

---

# 11. Failure, Interruption and Business Recovery

If the re-conducted occurrence, candidate association or Final Approved Marks are incomplete or disputed, recomputation does not proceed until the canonical source owner resolves the condition. I09 does not correct source-owned examination or marks facts.

---

# 12. Business Rules and Decision Constraints

- Only Final Approved Marks may enter authoritative result computation.
- I09 cannot modify evaluation history or approved marks.
- Recomputed outcomes must use the same governing academic regulations applicable to the valid examination occurrence.
- Validation precedes institutional approval.
- Publication remains owned by I10.

---

# 13. State and Lifecycle Impact

This scenario re-enters the locked I09 lifecycle at the applicable computation stage and progresses through `Result Validation → Result Approval → Result Finalisation → Ready for Publication` where required. It introduces no new lifecycle state.

---

# 14. Business Events and Cross-Domain Collaboration

I09 publishes only canonical source-defined result-processing events when their corresponding facts occur, including `Subject Result Computed`, `Semester Result Computed`, `CGPA Calculated`, `Result Approved`, and `Result Ready for Publication` as applicable. No new re-conduct-specific event is invented. A06 owns event transport, not result semantics.

---

# 15. Business Authorization, Approval and Audit Obligations

Recomputation and approval require authority for the applicable institution and examination scope. Approval actors follow the institution-configured I09 governance path. Audit evidence preserves the prior computation, authoritative re-conducted input, regulations applied, recomputed outcome, validation and approval/finalisation facts.

---

# 16. Completion Criteria and Postconditions

The scenario completes when the valid re-conducted examination outcome has been incorporated into the authoritative I09 result, required validation/approval has completed, traceability is preserved, and the result is in its governed next lifecycle position.

---

# 17. Open Issues, Assumptions and Deferred Questions

No L3-local architecture issue remains. The exact institutional approval sequence is configurable in locked L2-I09 and is therefore intentionally not hard-coded here.

**Architecture assessment:** `PASS`
