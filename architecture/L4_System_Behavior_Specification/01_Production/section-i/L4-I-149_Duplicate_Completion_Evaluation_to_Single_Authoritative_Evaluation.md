# L4-I-149 — Duplicate Completion Evaluation to Single Authoritative Evaluation

## 1. Specification Metadata
**Specification ID:** L4-I-149  
**Architecture Level:** L4 — System Behavior Specification  
**Realizes Scenario:** L3-I-149 — Duplicate Completion Evaluation to Single Authoritative Evaluation  
**Primary Business Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 2. Behavioral Objective & Boundary
Govern identification and resolution of duplicate completion evaluations so one authoritative evaluation remains while all superseded evidence remains traceable.

**Start:** Multiple completion evaluations are identified for the same applicable completion context.  
**End:** One authoritative evaluation is established, or an explicit unresolved position is retained.

## 3. Required Business Context
- Completion-evaluation context is identifiable.
- Candidate/student context is authoritative.
- Potential duplicate evaluations are attributable to their source and basis.
- Academic Completion & Completion Evidence owns the evaluation outcome within its scope.

## 4. Required System Behaviors
1. Identify potentially duplicate completion evaluations.
2. Compare their student, programme, completion-context, source, timing, and evaluation basis.
3. Determine the authoritative evaluation using governing rules and authority.
4. Supersede duplicate evaluations without silently deleting historical evidence.
5. Establish one current authoritative completion evaluation.
6. Preserve the duplicate-resolution rationale and source history.
7. Prevent superseded evaluations from being treated as current authoritative evidence.

## 5. Cross-Domain Behavior
Student Lifecycle and Examination retain ownership of their source contexts and facts. Academic Completion & Completion Evidence owns completion-evaluation resolution and does not redefine upstream ownership.

## 6. Failure / Recovery
Ambiguous duplicate evaluations shall not be silently merged. Where authoritative selection cannot be determined, the evaluations remain governed and unresolved until an authorised determination exists.

## 7. Audit & Evidence
Preserve candidate/context identity, duplicate candidates, comparison basis, selected evaluation, superseded evaluations, authority, and resolution history.

## 8. Observable Completion
Exactly one evaluation is authoritative for the applicable completion context, with duplicate resolution fully traceable, or the unresolved condition remains explicit.

## 9. Traceability
Parent: L3-I-149 — Duplicate Completion Evaluation to Single Authoritative Evaluation.

## 10. Deferred Design
Technical deduplication algorithms, storage, reconciliation utilities, and interfaces remain outside L4.