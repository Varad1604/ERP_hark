# Internship Opportunity to Verified Completion

# 1. Scenario Metadata

**Scenario ID:** L3-H-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, D, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; applicable locked Section D Academic Delivery sources; L2-E01 — Workforce & Engagement Workflows; `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md`  
**Referenced By:** Internship analytics and optional placement conversion; L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern an Internship opportunity through eligible Student participation, start, completion evidence, evaluation, verified completion and optional conversion while keeping Student, academic and workforce facts with their canonical owners.

# 3. Business Outcome

Participation, evaluation, and completion evidence produce a governed internship outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved internship opportunity is assigned to an eligible student.

## 4.2 End Boundary

The scenario ends when the Internship reaches the applicable governed outcome, including `Verified` for successful completion, or a governed `Withdrawn`/`Cancelled` exception before verified completion where policy permits.

## 4.3 In Scope

Internship publication/participation, configured recruitment, start, completion evidence, evaluation, verification, withdrawal/cancellation before verification, reopening where permitted and optional conversion.

## 4.4 Out of Scope

Student/academic/workforce lifecycle changes, official result ownership transfer, and implementation mechanisms.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns Student identity/lifecycle. |
| Academic Delivery / D | Supplies D-owned academic facts where applicable; does not own official examination results. |
| People & Workforce / E | Owns workforce context for Placement staff. |
| Placement / H | Owns Internship aggregate, participation/start/completion/evaluation/verification history and H04 Internship events. |

# 6. Business Actors, Roles and Responsibilities

Placement Officer is the primary business authority for Internship decisions under H04. Existing investigators/reviewers are not introduced here; the governed H04 process records the applicable evaluation evidence and verification decision without inventing a new cross-domain authority.

# 7. Preconditions and Entry Criteria

- Internship opportunity exists and is approved for Placement use.
- Student is eligible using authoritative source facts.
- Required recruitment is complete or available.
- Internship references one employer and one Student assignment.

# 8. Trigger

An approved internship opportunity is assigned to an eligible student

# 9. Main Business Flow

1. Publish the Internship opportunity and register the eligible candidate.
2. Complete configured recruitment and generate the Internship offer.
3. Start the Internship and publish `InternshipStarted`.
4. Preserve operational completion evidence and publish `InternshipCompleted` when the activity has operationally completed.
5. Evaluate the completion evidence and publish `InternshipEvaluated`.
6. Verify the governed completion outcome and publish `InternshipCompletionVerified`.
7. Move the Internship to the final successful `Verified` outcome.
8. If permitted, perform optional conversion to Placement after the governed Internship outcome and preserve conversion history.

# 10. Alternate Business Flows

- Internship may be `Withdrawn` or `Cancelled` before Verified completion where policy permits, publishing `InternshipWithdrawn` or `InternshipCancelled`.
- A Withdrawn/Cancelled or interrupted Internship may be reopened only where the H04 recovery rule permits and prior completion has not been Verified, publishing `InternshipReopened`.
- A Verified Internship is immutable historical evidence and is not reopened.

# 11. Failure, Interruption and Business Recovery

Incomplete or invalid evidence prevents verification. Withdrawal/cancellation preserves prior participation history. Reopening is governed and cannot erase prior interruption evidence. Verified completion cannot be reversed by local L3 inference.

# 12. Business Rules and Decision Constraints

- Internship belongs to one employer and one Student assignment.
- Operational `Completed` is distinct from `Evaluated` and final `Verified`.
- `InternshipCompleted` does not by itself mean verified completion.
- Verified completion is final for the successful H04 internship-completion outcome.
- Withdrawal/cancellation may occur only before Verified where policy permits.
- C/D/E source facts remain owned by their canonical domains.

# 13. State and Lifecycle Impact

Canonical Internship successful lifecycle:

`Published → Candidate Registered → Offered → Accepted → Started → Completed → Evaluated → Verified`

Governed exceptional outcomes before Verified include `Withdrawn` and `Cancelled`. Reopening is permitted only where the prior completion has not been Verified.

# 14. Business Events and Cross-Domain Collaboration

Canonical H04 events used in this scenario include:

- `InternshipStarted`
- `InternshipCompleted`
- `InternshipEvaluated`
- `InternshipCompletionVerified`
- `InternshipWithdrawn`
- `InternshipCancelled`
- `InternshipReopened`

Placement remains semantic publisher; C/D/E facts are consumed by reference only.

# 15. Business Authorization, Approval and Audit Obligations

Placement Officer is the primary H04 business authority for Internship decisions. Evidence must preserve Internship, employer, Student, participation, completion evidence, evaluation, verification, exception/reopening decisions and optional conversion history.

# 16. Completion Criteria and Postconditions

Successful completion requires operational completion evidence, governed evaluation and final `Verified` completion. Alternative governed outcomes may end in Withdrawn or Cancelled before verification. Source-domain ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing Internship evaluation/verification lifecycle, events and withdrawal/cancellation/reopening semantics were resolved by locked `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

The earlier stale H04 dependency wording does not transfer official examination-result ownership from Section I.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
