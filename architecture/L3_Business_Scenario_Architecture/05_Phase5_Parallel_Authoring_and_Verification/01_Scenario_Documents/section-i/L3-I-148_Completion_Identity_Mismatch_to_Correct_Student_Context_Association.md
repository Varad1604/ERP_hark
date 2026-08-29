# Completion Identity Mismatch to Correct Student Context Association

**Scenario ID:** L3-I-148
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-C05 Qualification Completion & Student Exit; applicable Examination completion-evidence workflow

## 1. Scenario Metadata

The canonical Stage 3 semantics and Stage 4 final identity are preserved. The scenario remains owned by Academic Completion & Completion Evidence. Cross-domain participation does not transfer ownership of examination or student lifecycle facts.

## 2. Scenario Purpose

Ensure that duplicate completion evaluations for the same student and completion scope do not produce competing current completion positions, and that one authoritative evaluation remains current while historical evaluations remain traceable.

## 3. Business Outcome

A single authoritative completion evaluation is established for the affected student and scope. Duplicate evaluations are recognized as historical or superseded and cannot compete as the current completion basis.

## 4. Scenario Boundary and Scope

**Start Boundary:** More than one completion evaluation exists or is initiated for the same student and materially identical completion scope.

**In Scope:** identify the student/scope; distinguish legitimate re-evaluation from duplicate evaluation; determine the canonical current evaluation; preserve evaluation history; and establish one authoritative completion-evaluation position.

**Out of Scope:** recomputing examination results, changing Student Lifecycle facts, creating a new completion rule, or deleting historical evaluations.

**End Boundary:** One current completion evaluation is authoritative for the scope.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns the completion evaluation and authoritative completion basis.

Student Lifecycle owns student identity and academic lifecycle context.

Examination owns examination results/evidence that may be consumed as completion inputs.

The duplicate-resolution outcome remains within the completion owner.

## 6. Business Actors, Roles and Responsibilities

- Completion Administrator/authority: identifies and resolves duplicate completion evaluations.
- Student Lifecycle: provides authoritative student context.
- Examination: provides authoritative examination evidence where relevant.
- Required academic approval authority: applies existing completion governance.

No new authority is created.

## 7. Preconditions and Entry Criteria

- A student and completion scope are identifiable.
- More than one evaluation position exists or has been initiated.
- The completion lifecycle and its authoritative evaluation context are available.
- Source facts used by the evaluations remain traceable.

## 8. Trigger

A duplicate or competing completion evaluation is identified for the same student and completion scope.

## 9. Main Business Flow

1. Identify the student, programme and completion scope.
2. Identify all evaluation positions associated with that scope.
3. Determine whether each position is a legitimate re-evaluation or an unintended duplicate.
4. Establish the evaluation that represents the current authoritative completion position under the existing completion lifecycle.
5. Preserve legitimate historical evaluations and their decision basis.
6. Supersede or otherwise disposition the duplicate current position using the existing completion lifecycle.
7. Reconcile any downstream completion-evidence processing with the single authoritative evaluation.
8. Publish applicable completion facts through the canonical completion owner.
9. End when exactly one evaluation position is authoritative for the current scope.

## 10. Alternate Business Flows

**Legitimate re-evaluation:** retain the latest/current evaluation as authoritative while preserving the prior evaluation history.

**Duplicate with identical basis:** consolidate current use around one authoritative evaluation without deleting historical evidence.

**Evaluation basis changed:** the newer authoritative evaluation supersedes the prior current position under existing completion governance.

## 11. Failure, Interruption and Business Recovery

**No canonical evaluation can be selected:** do not invent a precedence rule; escalate through completion governance.

**Student context differs:** Student Lifecycle remains authoritative and the evaluations must be associated with the correct context before completion is finalized.

**Underlying examination evidence differs:** Examination remains the owner of examination evidence; completion waits for the authoritative evidence position.

**Downstream evidence already produced:** completion-evidence consequences remain governed by the existing completion-evidence lifecycle.

## 12. Business Rules and Decision Constraints

- One current completion evaluation must be authoritative for a given student and completion scope.
- Legitimate re-evaluation is distinct from unintended duplicate evaluation.
- Historical evaluation decisions remain traceable.
- Source-owned examination/student facts are not rewritten by duplicate resolution.
- No local rule is invented for choosing between materially different authoritative evaluations.

## 13. State and Lifecycle Impact

No new `Duplicate` or `Resolved` completion state is introduced. The existing completion-evaluation lifecycle remains authoritative. Duplicate handling is a business disposition within that lifecycle.

## 14. Business Events and Cross-Domain Collaboration

The completion-owning workflow publishes the authoritative evaluation/completion facts. Student Lifecycle and Examination publish their own source facts.

No `DuplicateCompletionEvaluationResolved` event is invented. Existing completion-domain event names must be used where defined by L2-C05.

## 15. Business Authorization, Approval and Audit Obligations

- Duplicate evaluation disposition follows the existing completion approval/governance authority.
- The selected authoritative evaluation and disposition basis must be auditable.
- Historical evaluations must remain traceable.
- Any correction to examination evidence remains subject to Examination authority.

## 16. Completion Criteria and Postconditions

**Complete when:** all duplicate/competing evaluations are identified; legitimate re-evaluation is distinguished from duplication; one current evaluation is authoritative; historical evaluations remain traceable; and applicable completion facts are published.

**Postconditions:** downstream completion processing has one current evaluation basis and cannot treat duplicate positions as competing authoritative outcomes.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-C05 is locked and explicitly owns qualification-completion eligibility, degree audit, completion approval and completion events.
2. The accessible source does not expose a dedicated duplicate-evaluation event or state; none is invented.
3. Exact duplicate-selection precedence remains completion-governance owned rather than created at L3.
