# Governed Flexible or External Learning Participation to Recorded Learning Completion

# 1. Scenario Metadata

**Scenario ID:** L3-D-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0759 / CAN-L3-D-039 (final L3-D-038)

---

# 2. Scenario Purpose

Track governed flexible/external learning participation, validate institutional completion requirements and record the D06 Learning Completion business fact.

# 3. Business Outcome

Learning completion is institutionally validated and recorded with attributable evidence; completion does not itself grant academic credit.

# 4. Scenario Boundary and Scope

Starts when an Approved enrollment becomes Active and progress can be governed; ends when Learning Completion is recorded and `LearningCompleted` is published. In scope: progress facts, assessment/completion evidence, institutional completion validation and downstream certificate-verification trigger. Out of scope: engines/services, synchronization protocols, technical idempotency, automatic credit award.

# 5. Ownership and Participating Domains

D06 owns Learning Progress, synchronization history, completion validation and recorded Learning Completion. External provider/LMS supplies progress/assessment/provider-completion facts only.

# 6. Business Actors, Roles and Responsibilities

External Learning Provider/LMS supplies provider-side progress/completion evidence where applicable. D06 performs institutional validation/recording as business-domain responsibility; no synchronization/validation/completion `Service` or `Engine` is an L3 business actor.

# 7. Preconditions and Entry Criteria

Enrollment is Active; external provider registration exists where required; applicable progress/completion policy is active.

# 8. Trigger

Governed participation begins and later reaches the institutional completion condition or receives authoritative provider completion evidence.

# 9. Main Business Flow

1. D06 establishes Learning Progress for the Active enrollment — `LearningProgressInitialized`.
2. D06 records/reconciles progress, hours and assessment-status facts — existing D06 progress events remain authoritative.
3. Where external, provider supplies completion evidence — `ExternalCourseCompleted` as defined by D06.
4. D06 evaluates hours, mandatory activities, assessments, provider evidence and institutional completion policy — `LearningCompletionValidated` or `LearningCompletionValidationFailed`.
5. For valid completion, D06 records Learning Completion and publishes `LearningCompleted`.
6. Certificate verification may then begin; no credit is granted by this scenario.

# 10. Alternate Business Flows

No distinct alternate path reaches Learning Completed. Failed completion validation blocks completion and is handled in Section 11.

# 11. Failure, Interruption and Business Recovery

Missing/inconsistent progress or completion evidence blocks validation until resolved. Failed validation retains the non-completed business position with reason. Interrupted progression preserves the last authoritative D06 progress/completion context and continues after business evidence is resolved.

# 12. Business Rules and Decision Constraints

D06 owns institutional Learning Completion; external evidence never becomes institutional ownership; completion must satisfy approved policy; repeated same completion fact must not create duplicate institutional completion outcome; technical idempotency/synchronization design is deferred.

# 13. State and Lifecycle Impact

Learning Progress: Initialized → In Progress → Completed → Verified; then Archived as defined in L2-D06 Aggregate 5; W35–W44.

# 14. Business Events and Cross-Domain Collaboration

D06 event set remains: `LearningProgressInitialized`, `LearningProgressSynchronized`, `LearningProgressUpdated`, `AssessmentStatusSynchronized`, `ExternalCourseCompleted`, `LearningCompletionValidated`, `LearningCompletionValidationFailed`, `LearningCompleted` per L2-D06 W35–W44/Event Catalogue.

# 15. Business Authorization, Approval and Audit Obligations

D06 applies approved completion policy; provider/LMS supplies only external evidence. Audit retains enrollment, progress/evidence, policy version, validation result, completion decision and published facts.

# 16. Completion Criteria and Postconditions

Learning Completion is validly recorded by D06 with evidence; no academic credit is automatically granted; external provider ownership does not displace D06 institutional ownership.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Synchronization engines/services, protocols, polling, retries, exactly-once mechanisms and persistence are deferred to L4/L5.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
