# Verified Flexible Learning Achievement to Governed Credit Recognition and Posting Request

**Scenario ID:** L3-D-039  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0760 / CAN-L3-D-040 (final L3-D-039)

---

# 2. Scenario Purpose

Verify flexible-learning achievement, govern institutional credit recognition and issue an idempotent posting request to the canonical Academic Records owner.

# 3. Business Outcome

Approved credit recognition becomes immutable and a traceable `TranscriptPostingRequested` hand-off is issued; D06 never writes the transcript directly.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Learning is completed and a provider or institutional certificate/evidence is received for verification.

## 4.2 End Boundary

Credit recognition is approved and the posting request is acknowledged, or recognition is rejected/returned with an explicit governed reason.

## 4.3 In Scope

- Verify certificate authenticity, learner identity and provider approval.
- Create recognition case and evaluate curriculum mapping.
- Conduct academic review and approve, partially approve, reject or request evidence.
- Request transcript posting from Academic Records.

## 4.4 Out of Scope

- Direct transcript mutation.
- Recognition before verified completion.
- Automatic approval from a mapping recommendation.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Certificate Verification, Credit Recognition, mapping recommendation, approval and posting request. |
| Section C / Academic Records | Owns permanent academic record and transcript posting decision/execution. |
| Curriculum domain / Section B | Owns curriculum and learning-outcome references used for equivalency. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Certificate Verification Service | Verifies authenticity, learner identity and provider approval. | Verification execution only. |
| Credit Mapping Engine / Academic Affairs | Produces equivalency recommendation under approved rules. | Recommendation; no automatic approval. |
| Academic Advisor / Committee / HoD / Dean | Conducts academic credit review. | Configured review authority. |
| Credit Recognition Authority / Academic Affairs | Approves, partially approves or rejects recognition. | Final D06 recognition authority. |
| Academic Records Integration Service | Receives idempotent posting request. | No D06 transcript-write authority. |

# 7. Preconditions and Entry Criteria

- Learning Completion is recorded.
- Certificate/evidence is linked to enrollment and provider.
- Recognition and mapping policies are active.

# 8. Trigger

Completed learning produces certificate/evidence requiring verification and possible credit recognition.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive certificate, preserve original reference and create verification request. | D06 | CertificateReceived. |
| 2 | Verify authenticity, learner identity and provider approval. | Certificate Verification Service | Verified evidence or governed rejection/fraud path. |
| 3 | Accept verified certificate and lock verification evidence. | D06 | CertificateAccepted; recognition may begin. |
| 4 | Create one immutable recognition request for the completed learning. | Credit Recognition Service | CreditRecognitionRequested. |
| 5 | Evaluate curriculum/learning-outcome mapping under versioned rules. | Credit Mapping Engine / Academic Affairs | CreditMappingEvaluated recommendation. |
| 6 | Conduct academic review of mapping, evidence and policy. | Academic reviewers | AcademicCreditReviewCompleted. |
| 7 | Approve/partially approve or reject with explicit quantities and reason; request evidence when insufficient. | Credit Recognition Authority | CreditRecognitionApproved or Rejected. |
| 8 | For approval, lock recognition and submit idempotent posting request to Academic Records. | D06 / Academic Records Integration | TranscriptPostingRequested; transcript remains externally owned. |

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
| Certificate Verification | Received | Verify authenticity, learner and provider, then accept. | Received → Validating → Verified → Accepted | D06 | L2-D06 Aggregate 6; W45–W49 |
| Credit Recognition | Initiated | Map, review and decide recognition, then request posting. | Initiated → Academic Review → Approved → Credit Posted Request; alternate Rejected / Returned | D06 | L2-D06 Aggregate 7; W52–W58 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CertificateReceived / CertificateAuthenticityValidated / LearnerIdentityVerified / ProviderVerified / CertificateAccepted | D06 | Credit Recognition and authorized verification consumers | Records verified flexible-learning achievement. | L2-D06 W45–W49; Event Catalogue |
| CreditRecognitionRequested / CreditMappingEvaluated / AcademicCreditReviewCompleted / CreditRecognitionApproved / CreditRecognitionRejected / AdditionalEvidenceRequested / TranscriptPostingRequested | D06 | Section C / Academic Records and authorized academic consumers | Records recognition lifecycle and posting hand-off. | L2-D06 W52–W58; Event Catalogue |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C / Academic Records | Open Electives & MOOCs | Owns permanent academic record and transcript posting decision/execution. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Curriculum domain / Section B | Open Electives & MOOCs | Owns curriculum and learning-outcome references used for equivalency. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Verifies authenticity, learner identity and provider approval. | Certificate Verification Service | Verification execution only. | Open Electives & MOOCs |
| Produces equivalency recommendation under approved rules. | Credit Mapping Engine / Academic Affairs | Recommendation; no automatic approval. | Open Electives & MOOCs |
| Conducts academic credit review. | Academic Advisor / Committee / HoD / Dean | Configured review authority. | Open Electives & MOOCs |
| Approves, partially approves or rejects recognition. | Credit Recognition Authority / Academic Affairs | Final D06 recognition authority. | Open Electives & MOOCs |
| Receives idempotent posting request. | Academic Records Integration Service | No D06 transcript-write authority. | Open Electives & MOOCs |

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

- Approved credit recognition becomes immutable and a traceable `TranscriptPostingRequested` hand-off is issued; D06 never writes the transcript directly.
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
