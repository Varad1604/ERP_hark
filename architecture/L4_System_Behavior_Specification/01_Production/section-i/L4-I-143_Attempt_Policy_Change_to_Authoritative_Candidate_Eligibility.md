# L4-I-143 — Attempt Policy Change to Authoritative Candidate Eligibility

## 1. Specification Metadata
**Specification ID:** L4-I-143  
**Architecture Level:** L4 — System Behavior Specification  
**Realizes Scenario:** L3-I-143 — Attempt Policy Change to Authoritative Candidate Eligibility  
**Primary Business Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 2. Behavioral Objective & Boundary
Govern re-evaluation of candidate examination eligibility when an authoritative attempt policy change affects the permitted attempt position.

**Start:** An authoritative attempt-policy change becomes applicable to an existing candidate examination context.  
**End:** A current authoritative candidate-eligibility position is established, or a governed unresolved/rejected outcome is retained.

Out of scope: defining examination policy, changing Student Lifecycle ownership, altering historical attempts, or prescribing technical implementation.

## 3. Required Business Context
- Candidate identity and examination context are authoritative.
- Existing attempt history is available.
- The policy change is authorised and applicable.
- Student Lifecycle remains owner of student-status consequences.
- Examination owns examination eligibility within its scope.

## 4. Required System Behaviors
1. Identify candidates and examination contexts affected by the policy change.
2. Establish the authoritative policy version and effective applicability.
3. Re-evaluate existing attempt history against the changed policy.
4. Determine the resulting candidate examination-eligibility position.
5. Preserve the prior eligibility basis and the policy-change rationale.
6. Prevent historical attempts from being rewritten merely because eligibility changes.
7. Hand any resulting student-status consequence to Student Lifecycle through its canonical ownership.

## 5. Cross-Domain Behavior
Student Lifecycle provides authoritative candidate context and owns downstream student lifecycle consequences. Examination retains ownership of examination attempt history and eligibility determination. No ownership transfer occurs.

## 6. Failure / Recovery
Unclear policy authority, incomplete attempt history, or unresolved effective-date applicability shall prevent a conclusive eligibility determination. Conflicting policy interpretations shall be resolved through the policy-owning authority rather than by local assumption.

## 7. Audit & Evidence
Preserve prior policy basis, new policy basis, applicability decision, candidate attempt history considered, resulting eligibility determination, authority, and effective context.

## 8. Observable Completion
A current eligibility position is authoritative and traceable, or a governed unresolved/rejected position remains explicit.

## 9. Traceability
Parent: L3-I-143 — Attempt Policy Change to Authoritative Candidate Eligibility.

## 10. Deferred Design
Technical policy evaluation, persistence, rule engines, interfaces, and notification mechanisms remain outside L4.