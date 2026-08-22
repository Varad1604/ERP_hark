# Incomplete Academic Record to Resolved or Blocked Completion Position

**Scenario ID:** L3-I-146  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; D — Academic Delivery; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; applicable Academic Completion & Completion Evidence L2 workflow; L2-I09 Result Processing & Computation; authoritative Student Lifecycle and Academic Delivery sources  
**Referenced By:** Downstream student academic-record and completion-evidence consumers, as applicable

## 1. Scenario Metadata

The frozen Stage 3 semantics are preserved unchanged: the canonical owner is Academic Completion & Completion Evidence; participating domains, Type and Priority remain as assigned. The Final L3 identity is derived from the approved Stage 4 mapping.

## 2. Scenario Purpose

Resolve an incomplete academic record encountered during completion evaluation, or establish a governed blocked completion position when the missing information cannot yet be authoritatively resolved.

## 3. Business Outcome

The incomplete academic record is either resolved with authoritative source information sufficient for completion evaluation, or the student remains in a governed blocked completion position with the unresolved dependency clearly identified.

## 4. Scenario Boundary and Scope

**Start Boundary:** Completion evaluation identifies missing, incomplete or unavailable academic information required to determine completion eligibility.

**In Scope:** identify the missing completion input; determine its canonical owner; obtain/consume the authoritative resolution; re-evaluate completion where resolved; or establish a blocked position where it remains unresolved.

**Out of Scope:** creating substitute academic facts, altering examination/academic-delivery records directly, inventing completion requirements, or issuing completion evidence.

**End Boundary:** The missing requirement is resolved and completion can be re-evaluated, or a governed blocked position is authoritative.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns the completion outcome and the business handling of incomplete completion inputs.

Student Lifecycle, Academic Delivery and Examination remain owners of their respective source facts.

The completion workflow coordinates resolution without taking ownership of upstream records.

## 6. Business Actors, Roles and Responsibilities

- Completion Administrator: identifies and coordinates the incomplete record.
- Student Lifecycle authority: resolves student-context gaps.
- Academic Delivery authority: resolves academic-delivery gaps.
- Examination authority: resolves examination-result gaps.
- Completion authority: determines whether the resolved record is sufficient for completion evaluation.

## 7. Preconditions and Entry Criteria

- Completion evaluation has identified a required missing/incomplete input.
- The affected student and completion scope are established.
- The canonical source owner of the missing input can be identified.
- A valid completion lifecycle position exists for the incomplete evaluation.

## 8. Trigger

Completion evaluation encounters an incomplete academic record that prevents a conclusive completion determination.

## 9. Main Business Flow

1. Identify the student, programme and completion scope.
2. Identify the missing or incomplete academic requirement.
3. Determine the canonical owner of that requirement.
4. Route the resolution request/business dependency to that owning workflow.
5. Receive the authoritative resolved input when available.
6. Re-run completion evaluation using the resolved source fact.
7. If the input cannot yet be resolved, establish the governed blocked completion position.
8. Preserve the missing-input history and resolution basis.
9. Publish applicable completion facts through the canonical completion owner.
10. End when the record is resolved for evaluation or the blocked position is authoritative.

## 10. Alternate Business Flows

**Examination input incomplete:** resolve through Examination result-processing/marks lifecycle before completion is re-evaluated.

**Academic Delivery input incomplete:** resolve through the owning Academic Delivery workflow.

**Student context incomplete:** resolve through Student Lifecycle.

**Input resolved after a blocked position:** re-open/re-evaluate only through the existing completion lifecycle; do not create a new local restoration state.

## 11. Failure, Interruption and Business Recovery

**Source owner cannot resolve the record:** completion remains blocked; no substitute value is invented.

**Conflicting source facts:** the owning domain must establish one authoritative fact before completion evaluation resumes.

**Resolution arrives after a prior decision:** the completion workflow applies its governed re-evaluation process; historical decisions remain traceable.

**Required approval unavailable:** the record remains in the applicable governed position and is not treated as complete.

## 12. Business Rules and Decision Constraints

- Missing completion inputs must be resolved by their canonical owners.
- Completion cannot be treated as eligible when a mandatory requirement is unresolved.
- No source-owned academic fact may be invented or overwritten by the completion workflow.
- Historical incomplete/blocked positions remain traceable.
- Re-evaluation uses the newly authoritative source fact once available.

## 13. State and Lifecycle Impact

The scenario uses the existing completion lifecycle positions defined by the applicable completion architecture. It does not introduce `IncompleteRecord`, `Resolved`, or `Blocked` states unless those states are already defined upstream.

Where the frozen scenario outcome is resolved versus blocked, the exact lifecycle labels remain authoritative to the completion L2 workflow.

## 14. Business Events and Cross-Domain Collaboration

The canonical source workflow publishes the resolution of its own academic fact. The completion-owning workflow publishes the resulting completion-evaluation position.

No generic `AcademicRecordResolved` or `CompletionBlocked` event is invented without an upstream definition.

## 15. Business Authorization, Approval and Audit Obligations

- Source-domain resolution follows the authority of Student Lifecycle, Academic Delivery or Examination.
- Completion blocking and subsequent re-evaluation follow the completion workflow's approval rules.
- Missing-input identification, resolution basis and resulting completion position must be auditable.

## 16. Completion Criteria and Postconditions

**Complete when:** the missing requirement is either authoritatively resolved and completion re-evaluation can proceed, or a governed blocked position is established with the dependency identified and traceable.

**Postconditions:** no unresolved mandatory input is silently treated as satisfied; resolved records can proceed to completion evaluation; blocked records remain governed until the dependency is resolved.

## 17. Open Issues, Assumptions and Deferred Questions

1. The Stage 3 title specifies a resolved-or-blocked completion outcome but does not define the exact completion lifecycle labels. Those remain upstream-owned.
2. The accessible repository retrieval did not expose the dedicated Academic Completion L2 document, so exact completion event names and state names are not asserted here.
3. Examination result inputs are consumed from L2-I09; L2-I09 explicitly does not own Academic Completion.
