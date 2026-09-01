# L4-I-142 — Exhausted Examination Attempts to Conclusive Eligibility Determination

## 1. Specification Metadata
- Specification ID: L4-I-142
- Architecture Level: L4 — System Behavior Specification
- Realizes: L3-I-142 — Exhausted Examination Attempts to Conclusive Eligibility Determination
- Primary Owner: Supplementary, Backlog & Improvement Examination
- Participating Domains: C — Student Lifecycle; I — Examination
- Scenario Type: Critical; Priority: P1
- Status: Draft; Version: 1.0

## 2. Behavioral Objective & Boundary
Determine the authoritative eligibility position when the governed examination-attempt opportunities available to a candidate have been exhausted. End at a conclusive eligibility determination, without inventing a new academic or student lifecycle state.

## 3. Required Business Context
- Candidate identity is authoritative.
- Subsequent-examination attempt history is complete enough for determination.
- Applicable attempt and eligibility rules are authoritative.
- Student Lifecycle remains owner of student status consequences.

## 4. Required System Behaviors
1. Identify the candidate and applicable examination opportunity history.
2. Establish the authoritative completed/exhausted attempt sequence.
3. Evaluate remaining eligibility under the applicable governing rules.
4. Determine whether another permitted examination opportunity remains.
5. Establish a conclusive examination-eligibility outcome where attempts are exhausted.
6. Preserve the attempt evidence and determination rationale.
7. Hand any resulting student-status consequence to Student Lifecycle rather than changing it locally.

## 5. Cross-Domain Behavior
Examination owns examination attempt history and the examination eligibility determination within its scope. Student Lifecycle owns any resulting student lifecycle consequence. No ownership transfer occurs.

## 6. Failure / Recovery
Incomplete attempt history, conflicting attempt records, or unresolved eligibility authority prevents a conclusive determination. The existing authoritative position remains until the conflict is resolved.

## 7. Audit & Evidence
Preserve candidate context, attempt sequence, applicable rules, evaluation, determination, authority and any resulting cross-domain handoff evidence.

## 8. Observable Completion
The candidate has a conclusive authoritative examination-eligibility determination, traceable to the exhausted attempt history and governing rules.

## 9. Traceability
Parent: L3-I-142 — Exhausted Examination Attempts to Conclusive Eligibility Determination.

## 10. Deferred Design
Technical calculation, storage, integration and notification mechanisms remain outside L4.