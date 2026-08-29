# Conflicting Completion Inputs to Valid Completion Basis

**Scenario ID:** L3-I-147  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; D — Academic Delivery; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; applicable Academic Completion & Completion Evidence L2 workflow; L2-I09 Result Processing & Computation; authoritative Student Lifecycle and Academic Delivery sources  
**Referenced By:** Downstream student academic-record and completion-evidence consumers, as applicable

## 1. Scenario Metadata

The frozen Stage 3 semantics are preserved unchanged: the canonical owner is Academic Completion & Completion Evidence; participating domains, Type and Priority remain as assigned. The Final L3 identity is derived from the approved Stage 4 mapping.

## 2. Scenario Purpose

Resolve conflicting academic inputs used by completion evaluation so that the completion decision is based on one authoritative set of student, academic-delivery and examination facts.

## 3. Business Outcome

A valid completion basis is established from authoritative, reconciled academic inputs, with conflicting source positions retained historically and prevented from producing competing completion outcomes.

## 4. Scenario Boundary and Scope

**Start Boundary:** Completion evaluation identifies conflicting source facts that materially affect the student's completion determination.

**In Scope:** identify the conflicting inputs; establish their canonical owners; reconcile the conflict through those owners; re-evaluate completion using the authoritative facts; preserve historical conflict evidence.

**Out of Scope:** changing examination marks/results locally, changing Academic Delivery facts locally, changing Student Lifecycle facts locally, or inventing a completion rule to choose a winner.

**End Boundary:** One valid authoritative completion basis exists for the affected student and scope.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns the completion-basis outcome.

Student Lifecycle owns student context.

Academic Delivery owns its academic-delivery outcomes.

Examination owns examination results and related examination facts.

The completion workflow consumes reconciled source facts and does not transfer upstream ownership.

## 6. Business Actors, Roles and Responsibilities

- Completion Administrator: identifies and coordinates conflicting inputs.
- Student Lifecycle authority: resolves student-context conflicts.
- Academic Delivery authority: resolves academic-delivery conflicts.
- Examination authority: resolves examination-result conflicts.
- Completion authority: re-evaluates completion after the authoritative inputs are established.

## 7. Preconditions and Entry Criteria

- Completion evaluation is active or due.
- At least two materially conflicting academic inputs have been identified.
- The affected student and completion scope can be established.
- Each conflicting input has a canonical source owner.

## 8. Trigger

Completion evaluation detects conflicting academic inputs that could produce competing completion positions.

## 9. Main Business Flow

1. Establish the affected student, programme and completion scope.
2. Identify each conflicting academic input and its canonical owner.
3. Determine whether the conflict is historical, derived or genuinely current.
4. Route each source-owned conflict to the owning workflow.
5. Wait for authoritative source positions to be established through their canonical lifecycles.
6. Reassemble the completion basis using only authoritative current facts.
7. Re-evaluate completion requirements.
8. Establish one authoritative completion position.
9. Preserve the conflict and resolution history.
10. Publish applicable completion facts through the canonical completion owner.
11. End when a valid completion basis exists.

## 10. Alternate Business Flows

**Examination result conflict:** resolve through Examination result-processing ownership before re-evaluating completion.

**Academic Delivery conflict:** resolve through Academic Delivery.

**Student context conflict:** resolve through Student Lifecycle.

**Historical versus current difference:** preserve legitimate historical facts while using the current authoritative position required by completion policy.

**Conflict does not affect completion:** document the reconciliation outcome and retain the authoritative completion basis.

## 11. Failure, Interruption and Business Recovery

**No source owner can resolve the conflict:** completion remains unresolved/blocked according to the completion lifecycle.

**Conflicting sources remain simultaneously authoritative:** do not invent a local precedence rule; escalate to the applicable upstream governance.

**Conflict changes after re-evaluation:** re-evaluate from the newly authoritative basis and preserve prior completion decisions.

**Downstream completion evidence already exists:** any consequential evidence action remains within the completion-evidence lifecycle and is not performed locally.

## 12. Business Rules and Decision Constraints

- Each academic fact has one canonical owner.
- Completion evaluation consumes authoritative source facts; it does not arbitrate ownership.
- Historical facts remain traceable and are not deleted to hide a conflict.
- A completion position must not be established from unresolved conflicting mandatory inputs.
- No local precedence rule is invented where upstream architecture is silent.

## 13. State and Lifecycle Impact

No new `Conflicted` or `Reconciled` state is introduced. The scenario operates across the existing source-domain lifecycles and the existing completion lifecycle.

The valid completion basis is the business outcome; exact completion lifecycle labels remain those of the authoritative completion L2 workflow.

## 14. Business Events and Cross-Domain Collaboration

Student Lifecycle, Academic Delivery and Examination remain publishers of their own authoritative facts. The completion-owning workflow publishes the resulting completion outcome.

No `CompletionInputsReconciled` event is invented without an upstream canonical definition.

## 15. Business Authorization, Approval and Audit Obligations

- Source conflicts are resolved by the canonical owner of each fact.
- Completion decisions based on reconciled inputs must be auditable.
- Any exceptional cross-domain decision follows the applicable upstream governance.
- Separation of duties remains with the source and completion authorities.

## 16. Completion Criteria and Postconditions

**Complete when:** every material conflict is classified; source owners establish authoritative current facts; the completion basis is rebuilt from those facts; one valid completion position is established; and the conflict/resolution history is auditable.

**Postconditions:** the completion workflow has one valid authoritative basis; conflicting source positions are not treated as competing current facts; downstream completion processing can proceed from the reconciled basis.

## 17. Open Issues, Assumptions and Deferred Questions

1. The Stage 3 scenario establishes the valid completion-basis outcome but does not define a local precedence hierarchy among Student Lifecycle, Academic Delivery and Examination facts.
2. The accessible repository retrieval did not expose the dedicated Academic Completion L2 document, so exact completion lifecycle/event names are deliberately not invented.
3. L2-I09 explicitly owns examination computation and excludes Academic Completion; examination results therefore remain upstream inputs.
