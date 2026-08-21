# External Learning Certificate Verification to Accepted, Rejected or Flagged Certificate Outcome

**Scenario ID:** L3-D-043  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0763 / CAN-L3-D-043

---

# 2. Scenario Purpose

Verify an external-learning certificate against authenticity, learner identity and approved-provider evidence and produce exactly one governed Accepted, Rejected or Fraud Suspected outcome.

# 3. Business Outcome

The certificate is Accepted for possible credit recognition, Rejected with explicit institutional reasons, or flagged Fraud Suspected with recognition suspended and an investigation opened.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Recorded learning completion produces a certificate from an approved provider, student upload or institutional platform.

## 4.2 End Boundary

Verification reaches Accepted, Rejected or Fraud Suspected and the applicable completed outcome event is published.

## 4.3 In Scope

- Receive and immutably reference certificate.
- Validate authenticity, fingerprint, issuer, learner identity and provider approval.
- Accept verified certificate, reject failed verification or flag suspected fraud.
- Preserve immutable verification and investigation evidence.

## 4.4 Out of Scope

- Credit-recognition approval.
- Transcript posting.
- Allowing a flagged certificate to support recognition.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Certificate Verification, verification evidence, outcome and fraud-investigation linkage. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Certificate Integration Service / Student | Supplies certificate through an authorized channel. | Submission only. |
| Certificate Verification Service | Validates authenticity, learner and provider evidence. | Verification execution. |
| Academic Affairs | Records governed rejection/acceptance where human authority is required. | Academic decision authority. |
| Compliance Office | Owns investigation action for Fraud Suspected outcome. | Authorized compliance handling. |

# 7. Preconditions and Entry Criteria

- Learning Completion and enrollment exist.
- Certificate is associated with the learner and provider.
- Provider/identity reference services are available.

# 8. Trigger

A certificate is received after confirmed learning completion.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive certificate, preserve original reference and create unique verification identifier. | D06 | CertificateReceived. |
| 2 | Validate signature/fingerprint, issuer and metadata integrity. | Certificate Verification Service | Authenticity result recorded. |
| 3 | Verify certificate learner against canonical student/enrollment identity. | Certificate Verification Service | Learner identity result recorded. |
| 4 | Verify provider approval/accreditation for the relevant period. | Provider Verification Service | Provider result recorded. |
| 5 | If all checks succeed, accept and lock evidence. | D06 / Academic Affairs | CertificateAccepted. |
| 6 | If verification fails without fraud basis, reject with explicit reason. | Academic Affairs | CertificateRejected. |
| 7 | If fraud indicators exist, flag, suspend recognition, preserve evidence and open investigation. | Compliance Office / D06 | CertificateFraudSuspected and investigation opened. |

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
| Certificate Verification | Received | Validate authenticity, learner and provider and determine one terminal outcome. | Received → Validating → Verified → Accepted; alternate Rejected or Fraud Suspected | D06 | L2-D06 Aggregate 6; W45–W51 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CertificateReceived / CertificateAuthenticityValidated / LearnerIdentityVerified / ProviderVerified / CertificateAccepted / CertificateRejected / CertificateFraudSuspected / CertificateInvestigationOpened | D06 | Credit Recognition, compliance and authorized academic consumers | Records verification and exact governed outcome. | L2-D06 W45–W51; D06-E36–E42 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates; completed events may still be consumed under canonical contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Supplies certificate through an authorized channel. | Certificate Integration Service / Student | Submission only. | Open Electives & MOOCs |
| Validates authenticity, learner and provider evidence. | Certificate Verification Service | Verification execution. | Open Electives & MOOCs |
| Records governed rejection/acceptance where human authority is required. | Academic Affairs | Academic decision authority. | Open Electives & MOOCs |
| Owns investigation action for Fraud Suspected outcome. | Compliance Office | Authorized compliance handling. | Open Electives & MOOCs |

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

- The certificate is Accepted for possible credit recognition, Rejected with explicit institutional reasons, or flagged Fraud Suspected with recognition suspended and an investigation opened.
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
