# Governed Flexible or External Learning Participation to Recorded Learning Completion

**Scenario ID:** L3-D-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0759 / CAN-L3-D-039 (final L3-D-038)

---

# 2. Scenario Purpose

Track governed flexible or external learning participation, validate institutional completion requirements and record one immutable Learning Completion.

# 3. Business Outcome

Learning completion is institutionally validated and recorded once with immutable timestamp and evidence; completion does not itself grant academic credit.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved enrollment becomes Active and progress tracking can be initialized.

## 4.2 End Boundary

Learning Completion is recorded, progress is locked, `LearningCompleted` is published and certificate verification is triggered where applicable.

## 4.3 In Scope

- Initialize one progress record per enrollment.
- Synchronize/update progress and assessment/completion facts.
- Validate hours, activities, assessments, provider confirmation and policy.
- Record immutable completion and begin downstream verification.

## 4.4 Out of Scope

- Institutional grade calculation.
- Automatic credit award.
- Unverified progress regression.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Learning Progress, synchronization history, completion validation and recorded learning completion. |
| External Learning Provider / LMS | Supplies progress, assessment and provider-completion facts; does not own institutional completion. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Progress Synchronization Engine / Internal LMS | Supplies and validates progress updates. | Execution under active enrollment. |
| Completion Validation Service | Applies versioned institutional completion rules. | Produces reproducible validation decision. |
| Completion Service | Records validated completion exactly once. | Authorized completion recording. |

# 7. Preconditions and Entry Criteria

- Enrollment is Active.
- Provider registration exists for external learning.
- Applicable progress and completion policies are active.

# 8. Trigger

Enrollment activation initializes progress; later progress reaches the configured completion threshold or provider reports completion.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Initialize one idempotent Learning Progress record at activation. | D06 Progress Service | LearningProgressInitialized. |
| 2 | Synchronize and update percentage, hours and assessment status with version history. | D06 / provider or LMS | LearningProgressUpdated/Synchronized. |
| 3 | Receive provider completion confirmation where external. | Completion Synchronization Engine | ExternalCourseCompleted. |
| 4 | Validate hours, mandatory activities, assessments, provider confirmation and institutional policy. | Completion Validation Service | LearningCompletionValidated or failed. |
| 5 | Record completion exactly once and lock progress. | Completion Service | LearningCompleted with immutable timestamp. |
| 6 | Trigger certificate verification and hold credit recognition until verification succeeds. | D06 | Downstream verification starts; no credit granted. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Open Electives & MOOCs |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Open Electives & MOOCs |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Open Electives & MOOCs |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Open Electives & MOOCs | Prevents cross-domain ownership leakage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Open Electives & MOOCs | Prevents governance bypass. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Published or historical business facts remain immutable. | Open Electives & MOOCs | Preserves auditability and lineage. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Learning Progress | Initialized | Track governed progress and validate terminal completion. | Initialized → In Progress → Completed → Verified; then Archived | D06 | L2-D06 Aggregate 5; W35–W44 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| LearningProgressInitialized / LearningProgressSynchronized / LearningProgressUpdated / AssessmentStatusSynchronized / ExternalCourseCompleted / LearningCompletionValidated / LearningCompletionValidationFailed / LearningCompleted | D06 | Certificate verification, monitoring and authorized academic consumers | Records progress and institutionally validated completion. | L2-D06 W35–W44; Event Catalogue |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| External Learning Provider / LMS | Open Electives & MOOCs | Supplies progress, assessment and provider-completion facts; does not own institutional completion. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Supplies and validates progress updates. | Progress Synchronization Engine / Internal LMS | Execution under active enrollment. | Open Electives & MOOCs |
| Applies versioned institutional completion rules. | Completion Validation Service | Produces reproducible validation decision. | Open Electives & MOOCs |
| Records validated completion exactly once. | Completion Service | Authorized completion recording. | Open Electives & MOOCs |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Open Electives & MOOCs |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Open Electives & MOOCs |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Open Electives & MOOCs |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Learning completion is institutionally validated and recorded once with immutable timestamp and evidence; completion does not itself grant academic credit.
- All required authority, validation and audit evidence is retained.
- Applicable completed business facts are published exactly once in business effect.

## 16.2 Postconditions

- Canonical ownership remains unchanged.
- Historical decisions and prior states remain immutable.
- Downstream domains consume facts without modifying the owning aggregate.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D06_Open_Electives_and_MOOCs_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
