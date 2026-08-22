# L3-C-049 — Qualification Completion Deferral and Remediation

**Architecture Level:** L3 — Business Scenario Architecture  
**Final L3 ID:** L3-C-049  
**Canonical Title:** Qualification Completion Deferral and Remediation  
**Primary Owner:** Qualification Completion & Student Exit  
**Participating Domains:** C, F, G, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Derivation:** KEEP  
**Status:** Draft  
**Version:** 1.0  
**Author:** Varad  
**Last Updated:** 2026-08-20  

## 1. Scenario Metadata
Frozen Section C identity is preserved. No ownership, participating-domain, type, priority, title, or lifecycle reclassification is introduced.

## 2. Scenario Purpose
Manage a policy-governed deferral of qualification completion when one or more qualification requirements remain unresolved, preserving the remediation position without prematurely approving or conferring the qualification.

## 3. Business Outcome
The qualification-completion decision is authoritatively Deferred, outstanding conditions are preserved, and the Student remains in a governed remediation position until resolution or another approved outcome.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
Qualification completion evaluation identifies an outstanding academic, clearance, approval, policy or other C05-recognized blocking condition.

### 4.2 End Boundary
The qualification-completion deferral is recorded and the outstanding remediation position is preserved.

### 4.3 In Scope
- Identification of the authoritative blocking condition.
- Policy evaluation for deferral.
- Recording Deferred.
- Preservation of remediation requirements.
- Coordination with canonical owners.
- Re-evaluation when authoritative evidence changes.
- Resumption of qualification completion when requirements are satisfied.

### 4.4 Out of Scope
- Re-deciding Examination results.
- Rewriting progression rules.
- Overriding Finance/Library/Hostel decisions.
- Degree conferment.
- Student Exit unless separately approved.
- Technical workflow/retry mechanics.

### 4.5 Related Scenarios
C039 — Qualification Completion and Degree Conferment; C036 — Qualification Eligibility Determination; C046 — Institutional Exit Clearance Completion; C050 — Conferred Qualification Correction or Revocation.

## 5. Ownership and Participating Domains
**Canonical owner:** Qualification Completion & Student Exit (C).

F, G and I retain ownership of their authoritative facts and decisions.

## 6. Business Actors, Roles and Responsibilities
- C05 owns the qualification-completion deferral decision and remediation coordination.
- F/G/I own their respective financial, institutional or academic/examination facts and decisions.

## 7. Preconditions and Entry Criteria
- Qualification completion evaluation has begun.
- At least one authoritative condition prevents immediate completion.
- Institutional policy permits deferral.
- No incompatible terminal outcome exists.

## 8. Trigger
Qualification evaluation, degree audit, clearance or approval review identifies a blocking condition.

## 9. Main Business Flow
1. Identify the authoritative blocking condition.
2. Determine whether policy permits deferral.
3. Record Qualification Completion as Deferred.
4. Preserve remediation requirements.
5. Coordinate resolution with canonical owners.
6. Receive authoritative resolution evidence.
7. Re-evaluate qualification completion.
8. Resume the canonical completion path when requirements are satisfied.

## 10. Alternate Business Flows
- Degree-audit failure: defer and retain outstanding requirements.
- Clearance unresolved: retain deferral until the owning domain resolves it.
- Approval returned/rejected: follow the governed review path.
- Authorized policy exception: re-evaluate under that decision.
- Remediation succeeds: resume normal completion.

## 11. Failure, Interruption and Business Recovery
Outstanding conditions remain owned by canonical domains. Deferral never confers a qualification. Historical academic facts are not overwritten. Re-evaluation uses authoritative current evidence.

## 12. Business Rules and Decision Constraints
- BR-010: qualification completion cannot proceed with incomplete mandatory academic requirements.
- BR-012: complete degree audit is required.
- BR-015: academic standing must satisfy requirements.
- BR-021/022: institutional clearance prerequisites apply.
- BR-030/031: configured approval and permanent audit apply.

## 13. State and Lifecycle Impact
- **Qualification Completion:** Under Evaluation/Eligible → Deferred.
- **Degree Award:** remains not conferred while deferred.

## 14. Business Events and Cross-Domain Collaboration
`QualificationCompletionDeferred` is the canonical C05 exception event. Subsequent evaluation may publish the standard qualification-evaluation events. C05 remains the publisher of its own qualification business facts.

## 15. Business Authorization, Approval and Audit Obligations
Deferral requires policy-governed authority and immutable audit evidence. Participating domains retain authority over their own facts.

## 16. Completion Criteria and Postconditions
- Blocking condition identified.
- Deferral recorded.
- Remediation requirements preserved.
- Authoritative resolution received.
- Qualification completion re-evaluated.
- No degree conferred while deferred.

## 17. Open Issues, Assumptions and Deferred Questions
**OI-C049-01:** C05 defines deferral/exception handling but no universal remediation taxonomy; none is invented.

**OI-C049-02:** F/G/I retain ownership of authoritative facts and decisions.

**OI-C049-03:** Deferral is not silently converted into cancellation; cancellation is a separate governed outcome.

**Classification:** PASS WITH ISSUES — source-grounded, with institution-specific remediation policy remaining upstream.

## Production Verification Record
- [x] Frozen identity preserved.
- [x] C05 ownership boundary preserved.
- [x] Cross-domain ownership preserved.
- [x] Canonical rules/lifecycle/events applied.
- [x] No implementation leakage.
- [x] All 17 sections present.
- [x] Material gaps recorded in Section 17.
- [x] Status is Draft per Phase 5 governance.
