# L4-C-030 — Academic Probation Management and Resolution

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-030` — Academic Probation Management and Resolution  
**Primary Owner:** Academic Progression  
**Participating Domains:** C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad

## 1. Purpose
Define behavior for managing an Academic Probation lifecycle from assignment through authoritative review and resolution.

## 2. Preconditions and Trigger
Academic Progression has authoritatively assigned Academic Probation. Applicable probation conditions and progression policy are established. Trigger occurs when probation review becomes due under institutional policy.

## 3. Main Behavior
1. Establish active probation conditions and review basis.
2. Obtain authoritative academic evidence, including official Examination outcomes where relevant.
3. Evaluate probation requirements against the approved progression policy.
4. Determine review outcome.
5. If requirements are satisfied, establish `Progressed`.
6. If requirements fail, establish `Detained` under the canonical C04 lifecycle.
7. Preserve probation history and review evidence.
8. Publish the applicable canonical progression outcome.

## 4. Alternate Behavior
- Evidence unavailable: defer final outcome where policy permits; probation remains unresolved.
- Authoritative evidence changes: perform a governed fresh evaluation and preserve prior history.

## 5. Ownership
Academic Progression owns probation conditions, review and resolution. Examination owns official examination evidence. Student Lifecycle participates as a consumer/context provider without owning probation.

## 6. Lifecycle and Events
Probation resolves to `Progressed` or `Detained`. Canonical events include `AcademicProbationAssigned`, `StudentProgressed`, and `AcademicDetentionApplied` as applicable. Under `ADR-STU-PROG-EVT-001`, superseded aliases are not parallel canonical events.

## 7. Rules
Probation is policy governed; only authoritative academic evidence may determine the outcome; historical progression decisions remain preserved; L4 introduces no new probation rule or approver.

## 8. Authorization and Audit
Review authority remains governed by L2-C04 and institutional policy. Review basis, evidence, decision, lifecycle result and applicable authority are auditable.

## 9. Business Failure Handling
Unavailable evidence may defer resolution; failed requirements lead to canonical detention; changed authoritative evidence leads to fresh governed evaluation. Technical monitoring/scheduling/retry mechanisms are excluded.

## 10. Traceability
L3-C-030; L2-C04 and 2026-08-22 amendment; `ADR-STU-PROG-EVT-001`; related C027, C029 and C031.

## 11. Open Issues
Exact probation review cadence and universal human approval role remain institution/policy specific where upstream architecture does not fix them. Canonical probation/detention event vocabulary is resolved upstream.

## 12. Implementation Boundary
No API, scheduler, database, service, queue, IAM, UI or infrastructure design is defined.

**Classification: PASS WITH ISSUES — Draft**
