# Duplicate Completion Evaluation to Single Authoritative Evaluation

## 1. Specification Metadata

**Specification ID:** L4-I-149  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-149 — Duplicate Completion Evaluation to Single Authoritative Evaluation  
**Primary Business Owner:** Academic Completion & Completion Evidence  
**Version:** 1.1.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-20  
**Depends On:** corrected Locked L3-I-149; L2-I13 Academic Completion, CGPA & Completion Evidence; L2-C05 Qualification Completion & Student Exit; applicable canonical source-domain architecture  
**Amendment Basis:** L3_Controlled_Semantic_Amendment_Decision_I148_I150_I157_I160_2026-09-20.md

## 2. Behavioral Objective & Boundary

Realize the corrected Locked L3 parent as observable, implementation-independent system behavior without redefining upstream ownership, states, events, rules or authority.

The system shall support only the business outcome defined by the corrected parent. No new duplicate-resolution state, event or precedence rule may be invented.

## 3. Required Business Context

- Academic Completion & Completion Evidence owns only its I13 completion evaluation/evidence responsibilities.
- Authoritative Identity, Student Lifecycle and Examination facts remain source-owned.
- L2-C05 alone owns the institutional Qualification Completion decision.
- Historical evidence and prior positions remain reconstructable where the scenario changes, supersedes, withdraws, restores, de-duplicates or reassociates an I13 position.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-149-BH-001 | Detect competing current completion evaluations for the same student and completion scope. | Corrected L3-I-149; L2-I13 / applicable canonical owner |
| L4-I-149-BH-002 | Distinguish legitimate historical/re-evaluation positions from unintended competing current evaluations using existing completion governance. | Corrected L3-I-149; L2-I13 / applicable canonical owner |
| L4-I-149-BH-003 | Establish exactly one current authoritative completion evaluation without deleting prior evaluation history. | Corrected L3-I-149; L2-I13 / applicable canonical owner |
| L4-I-149-BH-004 | Prevent non-authoritative duplicate evaluations from driving completion-evidence or downstream qualification processing. | Corrected L3-I-149; L2-I13 / applicable canonical owner |
| L4-I-149-BH-005 | Preserve the disposition basis and evaluation lineage as reconstructable evidence. | Corrected L3-I-149; L2-I13 / applicable canonical owner |

## 5. Cross-Domain & Event Behavior

Cross-domain collaboration shall preserve canonical ownership. The system shall consume source-owned authoritative facts rather than reproducing them in I13. Existing canonical events may be used only with their source-defined meaning and publisher.

Where a current I13 completion-evidence position must be supplied to Student Lifecycle, `ExaminationCompletionEvidencePrepared` remains the existing evidence boundary. It does not represent a Qualification Completion decision.

## 6. Failure / Rejection / Continuation

When an authoritative prerequisite or source-owned fact is unresolved, the system shall preserve the last valid I13 position, expose the unresolved condition, and prevent the scenario from reaching a false authoritative outcome. Recovery resumes only after the applicable canonical owner establishes the required business fact.

## 7. Audit & Evidence Behavior

The system shall preserve enough business evidence to reconstruct the prior position, current position, authoritative source facts, governing decision/disposition, owner-preserving handoffs, and completion of the scenario.

## 8. Observable Completion Conditions

The corrected L3 business outcome is observable; conflicting/non-current positions can no longer act as current authority; source-domain ownership remains intact; and any resulting current I13 evidence position is traceable.

## 9. Traceability & Provenance

**Parent:** corrected Locked L3-I-149  
**Canonical source:** frozen Phase-4/Stage-3 semantic mapping recorded by the controlled L3 amendment.  
**Ownership:** L2-I13 for examination-owned completion evidence; L2-C05 for institutional Qualification Completion; source domains for their own authoritative facts.

## 10. Open Issues / Deferred Design

No scenario-specific upstream semantic blocker remains after the 2026-09-20 controlled L3 correction.

Implementation choices remain deferred to L5/L6 and may not redefine this behavior.

## Canonical Validation Note

The prior blocker-only L4 draft is superseded by this corrected Draft derivation. Independent L4 review remains required before Architecture Approval.
