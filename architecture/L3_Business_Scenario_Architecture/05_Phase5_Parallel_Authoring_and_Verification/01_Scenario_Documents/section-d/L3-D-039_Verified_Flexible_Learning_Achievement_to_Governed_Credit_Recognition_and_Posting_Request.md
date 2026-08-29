# Verified Flexible Learning Achievement to Governed Credit Recognition and Posting Request

# 1. Scenario Metadata

**Scenario ID:** L3-D-039  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** B, C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0760 / CAN-L3-D-040 (final L3-D-039)

---

# 2. Scenario Purpose

Verify flexible-learning achievement, govern institutional credit recognition and hand the approved posting request to the canonical Academic Records owner without D06 mutating the transcript.

# 3. Business Outcome

Approved credit recognition becomes an immutable D06 decision and `TranscriptPostingRequested` is published as the governed hand-off to Academic Records; D06 never writes the transcript directly.

# 4. Scenario Boundary and Scope

Starts when Learning Completion and certificate/evidence are available; ends when recognition is Approved/Partially Approved/Rejected/Returned and, for approved recognition, a posting request is issued to Academic Records. In scope: certificate verification, recognition case, curriculum mapping, academic review, recognition decision and posting hand-off. Out of scope: transcript mutation, technical integration/idempotency, result computation.

# 5. Ownership and Participating Domains

D06 owns Certificate Verification, Credit Recognition, mapping recommendation, approval and posting request. Section C/Academic Records owns permanent academic record/transcript posting. Section B/Curriculum owns curriculum/course/Learning Outcome definitions used for equivalency.

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Advisor / Committee / HoD / Dean, where assigned by policy | Conducts academic recognition review. | Review authority only where established. |
| Academic Affairs / configured Credit Recognition Authority | Approves, partially approves or rejects D06 recognition. | D06 recognition decision authority. |
| Academic Records | Receives the approved D06 posting request and governs transcript posting under its own ownership. | No D06 mutation authority. |

Verification/mapping/posting-hand-off are D06 business responsibilities, not `Verification Service`, `Mapping Engine` or `Integration Service` actors.

# 7. Preconditions and Entry Criteria

Learning Completion exists; certificate/evidence is linked to the learner/enrollment/provider; recognition/mapping policy and authoritative curriculum references are available.

# 8. Trigger

Completed learning produces evidence requiring verification and possible credit recognition.

# 9. Main Business Flow

1. D06 receives certificate/evidence — `CertificateReceived`.
2. D06 verifies authenticity, learner identity and provider status using authoritative evidence — existing D06 verification events.
3. D06 establishes a Credit Recognition request — `CreditRecognitionRequested`.
4. D06 evaluates curriculum/Learning Outcome mapping — `CreditMappingEvaluated`.
5. Assigned academic reviewers complete business review — `AcademicCreditReviewCompleted`.
6. Authorized D06 recognition authority Approves/Partially Approves/Rejects or requests additional evidence — canonical D06 recognition events.
7. For approved recognition, D06 publishes `TranscriptPostingRequested`; Academic Records retains transcript ownership.

# 10. Alternate Business Flows

## 10.1 Additional Evidence Required

Where review cannot decide because evidence is insufficient, `AdditionalEvidenceRequested` keeps the recognition case open until authoritative evidence is supplied; it does not create the approved outcome.

# 11. Failure, Interruption and Business Recovery

Unverifiable certificate/identity/provider evidence blocks recognition. Invalid decision authority blocks approval. Interrupted recognition preserves the last authoritative D06 recognition position/evidence. Transcript-posting acknowledgement uncertainty is resolved by business reconciliation with Academic Records; no technical retry semantics are defined here.

# 12. Business Rules and Decision Constraints

D06 never owns or mutates transcript; recognition requires verified evidence and approved mapping/policy; recommendation cannot become automatic approval unless policy explicitly establishes that business path; historical recognition decisions remain traceable; duplicate institutional posting requests/outcomes must not be created from the same business fact.

# 13. State and Lifecycle Impact

Certificate Verification: Received → Validating → Verified → Accepted; governed alternatives Rejected/Fraud Suspected. Credit Recognition: Initiated → Academic Review → Approved → Credit Posted Request; governed alternatives Rejected/Returned. Owner D06. Source: L2-D06 Aggregates 6–7; W45–W58.

# 14. Business Events and Cross-Domain Collaboration

D06 retains canonical publication of the verification and recognition events listed in L2-D06 W45–W58, including `CertificateReceived`, `CertificateAccepted`, `CreditRecognitionRequested`, `CreditMappingEvaluated`, `AcademicCreditReviewCompleted`, `CreditRecognitionApproved`, `CreditRecognitionRejected`, `AdditionalEvidenceRequested`, and `TranscriptPostingRequested`.

# 15. Business Authorization, Approval and Audit Obligations

Academic review/recognition authority follows D06 policy. Academic Records alone owns transcript-posting execution. Audit retains evidence, mapping/policy versions, review/decision authority, recognition decision/reason and posting hand-off.

# 16. Completion Criteria and Postconditions

Recognition has a governed decision; approved recognition produces the posting hand-off without transcript mutation; ownership of D06, Section B and Academic Records remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Certificate-verification services, mapping engines, integration APIs, idempotency/retries and persistence are deferred to later levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
