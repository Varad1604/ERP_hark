# Duplicate Completion Evaluation to Single Authoritative Evaluation

**Scenario ID:** L3-I-149
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Varad  
**Last Updated:** 2026-09-20  
**Depends On:** Locked L1 Business Domain Architecture; L2-I13 Academic Completion, CGPA & Completion Evidence Workflows; L2-C05 Qualification Completion & Student Exit; applicable source-domain architecture  
**Amendment Authority:** L3_Controlled_Semantic_Amendment_Decision_I148_I150_I157_I160_2026-09-20.md

## 1. Scenario Metadata

This controlled successor restores the frozen canonical semantics for L3-I-149 without changing its final identity, title, owner, Type, Priority or repository placement.

## 2. Scenario Purpose

Prevent multiple completion evaluations for the same student and completion scope from competing as current authoritative positions.

## 3. Business Outcome

Exactly one current completion evaluation remains authoritative for the affected student and scope; duplicate/superseded evaluations remain historically traceable.

## 4. Scenario Boundary and Scope

**Start Boundary:** Multiple completion-evaluation positions exist or have been initiated for the same student and materially identical completion scope.

**End Boundary:** One current completion evaluation is authoritative and downstream completion processing uses only that position.

**In Scope:** the business actions required to reach the stated outcome while preserving canonical ownership and authoritative source facts.

**Out of Scope:** redefining Person/Identity, Student Lifecycle, Examination results, Qualification Completion, technical implementation, APIs, databases, UI design or new business authority.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns only its completion evaluation/evidence responsibilities under L2-I13. Source domains retain ownership of their authoritative facts. Student Lifecycle alone owns the institutional Qualification Completion decision under L2-C05.

## 6. Business Actors, Roles and Responsibilities

- Academic Completion / Completion Evidence authority governs the I13-owned evaluation/evidence position.
- Applicable Examination authorities establish authoritative examination outcomes under their owning workflows.
- Platform Identity and Student Lifecycle authorities retain their source responsibilities where identity/context facts participate.
- Student is the subject of the completion/evidence outcome.
- Section H consumers, where applicable, consume governed evidence without acquiring ownership.

No new actor or approval authority is introduced.

## 7. Preconditions and Entry Criteria

- The affected student/candidate and completion scope are identifiable.
- Required source facts are available or the scenario explicitly governs their unresolved condition.
- Existing I13 evidence/completion governance applies.
- Source-domain ownership remains unchanged.

## 8. Trigger

Multiple completion-evaluation positions exist or have been initiated for the same student and materially identical completion scope.

## 9. Main Business Flow

1. Identify the student, programme and completion scope.
2. Identify all completion-evaluation positions for that scope.
3. Distinguish legitimate re-evaluation/history from unintended duplicate current evaluations.
4. Apply existing completion governance to determine the current authoritative evaluation.
5. Preserve prior legitimate evaluation history and decision basis.
6. Supersede or otherwise disposition non-authoritative duplicate current positions without deleting history.
7. Ensure downstream completion-evidence processing references only the authoritative current evaluation.

## 10. Alternate Business Flows

### 10.1

A legitimate later re-evaluation may supersede an earlier evaluation while preserving both historically.

### 10.2

If duplicate evaluations are materially inconsistent and no existing rule selects one, escalate through completion governance rather than inventing precedence.

## 11. Failure, Interruption and Business Recovery

No current evaluation is chosen by assumption. Completion processing remains blocked until existing completion governance can establish one authoritative evaluation.

## 12. Business Rules and Decision Constraints

- One current completion evaluation must be authoritative for a student and completion scope.
- Historical evaluations remain traceable.
- Source-owned examination/student facts are not rewritten by duplicate resolution.
- No new duplicate-resolution lifecycle state or event is invented.

## 13. State and Lifecycle Impact

Existing I13 completion-evaluation/evidence lifecycle remains authoritative; no new Duplicate or Consolidated state is introduced.

## 14. Business Events and Cross-Domain Collaboration

Existing completion-domain events are used only when their source-defined facts occur. No DuplicateCompletionEvaluationResolved event is created.

## 15. Business Authorization, Approval and Audit Obligations

Duplicate-evaluation disposition follows existing completion governance/approval authority. The selected authoritative evaluation and disposition basis must be auditable.

## 16. Completion Criteria and Postconditions

One current completion evaluation is authoritative; duplicate current positions cannot compete downstream; historical evaluations remain reconstructable.

## 17. Open Issues, Assumptions and Deferred Questions

No L3-local blocker remains after the controlled semantic amendment. No implementation detail or unsupported business authority is introduced.
