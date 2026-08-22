# Attempt Policy Change to Authoritative Candidate Eligibility

**Scenario ID:** L3-I-143  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Supplementary, Backlog & Improvement Examination  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** L2-I01 Examination Policy & Configuration; L2-I12 Supplementary / Backlog / Improvement Examination Execution; applicable Student Lifecycle academic context  
**Referenced By:** Downstream Examination and Student Lifecycle workflows as applicable

## 1. Scenario Metadata

The frozen Stage 3 canonical semantics are preserved without reclassification. This scenario is owned by Supplementary, Backlog & Improvement Examination and is Critical/P1.

The final ID and exact Phase 5 filename are preserved.

## 2. Scenario Purpose

Re-evaluate a candidate's Supplementary, Backlog, or Improvement examination eligibility when an authoritative attempt policy changes, ensuring the candidate's current eligibility reflects the effective policy without rewriting historical attempts.

## 3. Business Outcome

The affected candidate receives one authoritative current eligibility position under the effective examination attempt policy, while prior eligibility decisions and examination attempts remain historically traceable.

## 4. Scenario Boundary and Scope

**Start Boundary:** An authoritative change to the examination attempt/eligibility policy becomes effective and affects the eligibility basis for an existing or prospective subsequent-examination candidate.

**In Scope:** identify affected candidates/scopes; establish the effective policy; re-evaluate eligibility; preserve prior eligibility and attempt history; and route changed eligibility to the subsequent-examination lifecycle.

**Out of Scope:** creating policy, modifying historical examination attempts, changing official results, or changing Student academic status directly.

**End Boundary:** The affected candidate's current eligibility position is authoritative under the effective policy.

## 5. Ownership and Participating Domains

L2-I01 owns Examination Policy & Configuration.

L2-I12 owns Supplementary/Backlog/Improvement eligibility and attempt administration and applies the effective policy.

Student Lifecycle owns student academic context and consumes applicable examination outcomes.

A policy change does not transfer policy ownership to L2-I12.

## 6. Business Actors, Roles and Responsibilities

- Examination Policy authority: establishes the effective policy.
- Supplementary Examination Officer: identifies affected candidate eligibility.
- Examination Administrator: coordinates re-evaluation.
- Student Lifecycle: provides authoritative academic context where required.

No new policy authority is introduced.

## 7. Preconditions and Entry Criteria

- An examination attempt/eligibility policy is already defined.
- A governed policy change has become effective.
- The affected candidate population and examination scope can be determined.
- Prior attempt and eligibility history remains available.
- The effective date and applicable policy version/position are authoritative.

## 8. Trigger

An authoritative examination attempt-policy change becomes effective and requires affected candidate eligibility to be re-evaluated.

## 9. Main Business Flow

1. Identify the effective policy change and the examination categories/scopes it affects.
2. Identify candidates whose current eligibility depends on the changed attempt policy.
3. Establish each candidate's authoritative attempt history and academic context.
4. Apply the effective policy to each affected candidate.
5. Determine the candidate's new authoritative eligibility position.
6. Preserve the prior eligibility decision as historical context rather than rewriting history.
7. Where eligibility changes, route the candidate into or out of the applicable L2-I12 lifecycle position.
8. Publish applicable eligibility facts through the canonical owning workflow.
9. Notify/route downstream examination processing where the changed eligibility affects an already progressing opportunity.
10. End when every affected candidate has an authoritative current eligibility position.

## 10. Alternate Business Flows

**No eligibility change:** preserve the existing current eligibility position while recording that the effective policy was evaluated.

**Eligibility becomes valid:** the candidate may proceed through the normal subsequent-examination lifecycle.

**Eligibility becomes invalid:** the candidate is prevented from proceeding to a new attempt under the effective policy; existing historical attempts remain unchanged.

**Candidate already scheduled:** the changed eligibility is reconciled with the existing schedule through the owning examination workflows; no direct schedule mutation is invented.

## 11. Failure, Interruption and Business Recovery

**Policy effective date is unclear:** no local effective date is assumed; the policy authority must establish it.

**Affected population cannot be determined:** re-evaluation remains incomplete until the canonical policy scope and candidate population are known.

**Historical attempt conflict:** resolve the underlying attempt/registration issue through the owning workflow before applying the changed policy.

**Downstream impact already exists:** schedule, Hall Ticket or conduct consequences remain owned by their respective L2 workflows.

**Exceptional override:** any override follows the upstream policy/exception authority rather than a local L3 rule.

## 12. Business Rules and Decision Constraints

- Policy changes are owned by Examination Policy & Configuration.
- L2-I12 applies the effective policy to subsequent-examination eligibility.
- Historical attempts and prior eligibility decisions remain traceable.
- A policy change affects current eligibility according to its authoritative effective scope; it does not silently rewrite historical examination facts.
- Student academic state remains owned by Student Lifecycle.

## 13. State and Lifecycle Impact

The subsequent-examination attempt lifecycle remains unchanged. The scenario changes the candidate's current eligibility position only where the effective policy requires it.

No new `PolicyChangedEligibility` or `Requalified` lifecycle state is introduced.

## 14. Business Events and Cross-Domain Collaboration

L2-I01 is the canonical owner of examination policy change facts. L2-I12 is the canonical owner of subsequent-examination eligibility facts resulting from application of the policy.

The accessible evidence does not establish a dedicated event name for policy-driven eligibility re-evaluation; no new event is invented.

## 15. Business Authorization, Approval and Audit Obligations

- Policy change approval and effective-date authority remain with L2-I01.
- Re-evaluation decisions must be auditable against the effective policy and candidate attempt history.
- Where a changed eligibility position affects an active examination opportunity, the consequence must remain attributable to the owning downstream workflow.
- No technical authorization or access-control mechanism is specified.

## 16. Completion Criteria and Postconditions

**Complete when:** the effective policy is authoritative; affected candidates/scopes are identified; attempt history is established; eligibility is re-evaluated; each candidate has one current authoritative eligibility position; prior history remains preserved; and applicable downstream facts are published.

**Postconditions:** current eligibility reflects the effective policy; historical attempts and prior decisions remain traceable; subsequent examination processing proceeds only from the current authoritative eligibility.

## 17. Open Issues, Assumptions and Deferred Questions

1. The Stage 3 semantics define policy change → authoritative candidate eligibility, but do not define retroactivity. The scenario therefore does not assume retroactive rewriting of historical attempts.
2. The exact policy effective-date and scope rules remain owned by L2-I01.
3. A dedicated policy-change-to-eligibility event name is not established in the accessible evidence; none is invented.
