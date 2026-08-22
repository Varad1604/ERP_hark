# Completion Evaluation to Authoritative Completion Eligibility

**Scenario ID:** L3-I-145  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; D — Academic Delivery; I — Examination  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; DEPENDENCY_MAP.md; applicable Academic Completion & Completion Evidence L2 workflow; L2-I09 Result Processing & Computation; L2-I10 Result Publication; authoritative Academic Delivery completion inputs  
**Referenced By:** Downstream student academic-record and completion-evidence consumers, as applicable

## 1. Scenario Metadata

The frozen Stage 3 semantics are preserved unchanged: the canonical owner is Academic Completion & Completion Evidence; participating domains, Type and Priority remain as assigned. The Final L3 identity is derived from the approved Stage 4 mapping.

## 2. Scenario Purpose

Determine whether a student satisfies the institutionally governed academic completion requirements by evaluating authoritative academic completion inputs across Student Lifecycle, Academic Delivery and Examination, and establish one authoritative completion-eligibility position.

## 3. Business Outcome

An authoritative completion-eligibility position is established for the student: the student is eligible for completion when all applicable completion requirements are satisfied, or remains not eligible/blocked when an authoritative requirement is unmet or unresolved.

## 4. Scenario Boundary and Scope

**Start Boundary:** Authoritative academic inputs required for completion evaluation are available for a student and completion evaluation is due.

**In Scope:** identify the student and applicable programme context; assemble authoritative academic completion inputs; evaluate required academic conditions; reconcile examination and academic-delivery outcomes; determine completion eligibility; and make the authoritative eligibility position available to downstream completion-evidence processing.

**Out of Scope:** changing examination marks/results, changing course or academic-delivery ownership, changing Student Lifecycle status directly, issuing completion evidence, or inventing local completion rules.

**End Boundary:** One authoritative completion-eligibility position exists for the student.

## 5. Ownership and Participating Domains

Academic Completion & Completion Evidence owns the completion-eligibility business outcome.

Student Lifecycle supplies and consumes authoritative student context.

Academic Delivery supplies authoritative academic-delivery outcomes.

Examination supplies authoritative examination results and academic outcomes.

Participation does not transfer ownership of student, academic-delivery or examination lifecycles.

## 6. Business Actors, Roles and Responsibilities

- Completion authority/administrator: evaluates completion requirements.
- Student Lifecycle: supplies authoritative student context.
- Academic Delivery: supplies authoritative academic-delivery outcomes.
- Examination: supplies authoritative examination results.
- Appropriate academic approval authority: approves completion where required by upstream policy.

No new authority is introduced.

## 7. Preconditions and Entry Criteria

- Student and programme context is authoritative.
- Required academic-delivery inputs are available or their unresolved position is known.
- Required examination results are authoritative or explicitly identified as unresolved.
- Applicable completion requirements are available from the owning completion architecture.
- The completion evaluation has not already reached an immutable final position for the same scope.

## 8. Trigger

Completion evaluation is initiated for a student whose academic inputs are due for determination of completion eligibility.

## 9. Main Business Flow

1. Establish the authoritative student and programme context.
2. Identify the academic-delivery requirements applicable to completion.
3. Identify authoritative examination requirements and results.
4. Assemble the required completion inputs without changing their source-owned values.
5. Apply the authoritative completion requirements.
6. Determine whether every required condition is satisfied.
7. Where all conditions are satisfied, establish the authoritative completion-eligible position.
8. Where a required condition is unmet or unresolved, establish the corresponding non-eligible or blocked position according to the owning completion lifecycle.
9. Preserve the evaluated evidence basis and contributing source facts.
10. Publish the applicable completion-evaluation/eligibility business facts through the canonical owner.
11. End when one authoritative completion-eligibility position exists.

## 10. Alternate Business Flows

**All requirements satisfied:** establish completion eligibility and make the outcome available for completion-evidence processing.

**Requirement unresolved:** retain the completion position as blocked/pending according to the authoritative completion lifecycle rather than assuming satisfaction.

**Examination result changes before final completion:** re-evaluate using the newly authoritative result through the completion workflow.

**Academic Delivery input changes:** consume the revised authoritative academic-delivery outcome and re-evaluate; do not alter Academic Delivery state locally.

## 11. Failure, Interruption and Business Recovery

**Required input unavailable:** completion cannot be conclusively evaluated; the unresolved dependency is recorded against the authoritative source.

**Conflicting inputs:** do not choose a local winner; route the conflict to the owning source workflow and re-evaluate once authoritative.

**Student context mismatch:** Student Lifecycle remains authoritative for student identity/academic context.

**Completion approval unavailable:** the eligibility outcome remains subject to the required upstream approval and is not represented as final completion evidence.

## 12. Business Rules and Decision Constraints

- Completion eligibility is determined from authoritative upstream academic facts and completion requirements.
- Examination results remain owned by Examination.
- Academic Delivery outcomes remain owned by Academic Delivery.
- Student academic context remains owned by Student Lifecycle.
- Completion evaluation must not mutate upstream facts.
- A blocked or unresolved completion position must not be treated as eligible.
- No local completion rule, threshold or academic requirement is invented.

## 13. State and Lifecycle Impact

The scenario affects the existing completion lifecycle only through its authoritative eligibility outcome. It does not create new Student Lifecycle, Academic Delivery or Examination states.

The precise completion lifecycle states and transitions must remain those defined by the applicable Academic Completion & Completion Evidence L2 architecture.

## 14. Business Events and Cross-Domain Collaboration

Examination publishes its canonical result facts; Academic Delivery publishes its own authoritative academic outcomes; Student Lifecycle publishes/consumes its canonical student facts.

The completion-owning workflow is the canonical publisher of completion eligibility. No new `CompletionEligibilityDetermined` event is invented unless defined by the applicable locked L2 completion workflow.

## 15. Business Authorization, Approval and Audit Obligations

- Completion evaluation and approval follow the authority defined by the completion architecture.
- The evaluated input basis and resulting eligibility position must be auditable.
- Any academic approval or separation-of-duty requirement remains upstream-owned.
- No technical authorization mechanism is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** student/programme context is established; all applicable authoritative inputs are identified; completion requirements are applied; one current eligibility position is established; the evidence basis is traceable; and applicable completion facts are published.

**Postconditions:** completion eligibility is authoritative for the evaluated scope; eligible students may proceed to completion-evidence processing; blocked/non-eligible students do not proceed as if eligible.

## 17. Open Issues, Assumptions and Deferred Questions

1. The frozen Stage 3 semantics identify the business outcome but do not expose the detailed completion-policy thresholds in the accessible sources. Those rules must remain owned by the applicable completion architecture.
2. L2-I09 explicitly owns examination computation and excludes Academic Completion & Completion Evidence; therefore examination results are consumed, not recomputed, here.
3. The exact completion event catalogue was not accessible in the current L2 source retrieval; no event name is invented.
