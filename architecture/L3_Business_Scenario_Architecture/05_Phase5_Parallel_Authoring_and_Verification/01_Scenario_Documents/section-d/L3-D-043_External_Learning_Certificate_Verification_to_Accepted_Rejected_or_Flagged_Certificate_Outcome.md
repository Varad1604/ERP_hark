# External Learning Certificate Verification to Accepted, Rejected or Flagged Certificate Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-D-043  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0763 / CAN-L3-D-043

---

# 2. Scenario Purpose

Verify external-learning certificate evidence against authenticity, learner identity and approved-provider evidence and establish one governed Accepted, Rejected or Fraud Suspected D06 outcome.

# 3. Business Outcome

Certificate is Accepted for possible credit recognition, Rejected with explicit reason, or marked Fraud Suspected with recognition blocked and an investigation path initiated.

# 4. Scenario Boundary and Scope

Starts when certificate evidence is received after Learning Completion; ends with Accepted, Rejected or Fraud Suspected and the applicable D06 event. In scope: evidence receipt, authenticity/learner/provider verification, governed terminal outcome, preserved evidence/investigation linkage. Out of scope: credit-recognition approval, transcript posting, technical signature/fingerprint algorithms/services.

# 5. Ownership and Participating Domains

D06 owns Certificate Verification, verification evidence, outcome and fraud-investigation linkage.

# 6. Business Actors, Roles and Responsibilities

Student or approved external provider may supply certificate evidence. Academic Affairs exercises any human academic acceptance/rejection authority established by policy. Compliance Office handles Fraud Suspected investigation action. Verification itself is a D06 business capability; verification/integration/provider `Service` actors are removed.

# 7. Preconditions and Entry Criteria

Learning Completion/enrollment exist; certificate is associated with learner/provider; authoritative provider/identity evidence is available.

# 8. Trigger

Certificate evidence is received after governed learning completion.

# 9. Main Business Flow

1. D06 receives certificate evidence — `CertificateReceived`.
2. D06 validates authenticity evidence — `CertificateAuthenticityValidated` where satisfied.
3. D06 validates learner identity — `LearnerIdentityVerified` where satisfied.
4. D06 validates provider approval/accreditation — `ProviderVerified` where satisfied.
5. If all required checks succeed, D06 records Accepted — `CertificateAccepted`.
6. If verification fails without fraud basis, D06 records Rejected — `CertificateRejected`.
7. If fraud indicators require governed investigation, D06/Compliance records Fraud Suspected — `CertificateFraudSuspected` and `CertificateInvestigationOpened` as defined upstream.

# 10. Alternate Business Flows

Accepted, Rejected and Fraud Suspected are the explicitly governed terminal outcomes of this verification scenario. They are not technical failures and all preserve the same verification-evidence boundary.

# 11. Failure, Interruption and Business Recovery

Missing authoritative identity/provider/evidence prevents a valid terminal verification outcome until resolved. Missing required human authority blocks decision where policy requires it. Interrupted verification preserves the last authoritative D06 evidence/status until continuation is valid.

# 12. Business Rules and Decision Constraints

D06 owns verification outcome; evidence/history is preserved; a Fraud Suspected certificate cannot support recognition until governed resolution; technical verification algorithms and service design are deferred.

# 13. State and Lifecycle Impact

Certificate Verification: Received → Validating → Verified → Accepted; governed alternatives Rejected or Fraud Suspected. Owner D06. Source L2-D06 Aggregate 6; W45–W51.

# 14. Business Events and Cross-Domain Collaboration

D06 retains canonical publication of `CertificateReceived`, `CertificateAuthenticityValidated`, `LearnerIdentityVerified`, `ProviderVerified`, `CertificateAccepted`, `CertificateRejected`, `CertificateFraudSuspected`, and `CertificateInvestigationOpened` under L2-D06 W45–W51/Event Catalogue.

# 15. Business Authorization, Approval and Audit Obligations

Any human academic verification decision follows D06 policy; Compliance Office governs investigation action. Audit retains original evidence, learner/provider references, checks, actor/authority, terminal outcome/reason and investigation linkage.

# 16. Completion Criteria and Postconditions

One governed verification outcome exists; D06 evidence/history is traceable; only Accepted outcome may proceed to recognition; Fraud Suspected remains blocked/investigated.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Cryptographic verification algorithms, integration services, APIs, retries and persistence are deferred.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
