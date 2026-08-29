# External Credential Verification Request to Verified or Rejected Outcome

# 1. Scenario Metadata
**Scenario ID:** L3-H-079  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** C, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.2  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** locked L1; `DEPENDENCY_MAP.md`; L2-H10; `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md`; `L2-H10_Enrollment_Cancellation_and_Verification_Result_Amendment_2026-08-27.md`  
**Referenced By:** L3 formal Architecture Approval re-review; targeted remediation current-state governance

# 2. Scenario Purpose
Coordinate a governed external credential-verification request and preserve immutable external verification evidence without H10 becoming owner of the source credential.

# 3. Business Outcome
Provider evidence produces a verified, rejected, expired, or inconclusive verification result with immutable lineage

The locked H10 architecture now supports all four frozen verification-result outcomes: `Verified`, `Rejected`, `Expired`, and `Inconclusive`.

# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
An authorized process requests verification of an externally issued credential.
## 4.2 End Boundary
The External Credential Verification reaches `Verified`, `Rejected`, `Expired`, or `Inconclusive` and immutable evidence is preserved.
## 4.3 In Scope
Verification request, provider evidence, all four governed verification outcomes, immutable lineage, successor verification semantics and source-ownership preservation.
## 4.4 Out of Scope
Credential ownership, mutation of the source credential, or authentication-provider/credential lifecycle ownership.

# 5. Ownership and Participating Domains
H10 owns the external verification request/result evidence only. C/I retain ownership of their credential/result references where applicable; authentication credentials remain owned by their canonical source domain.

# 6. Business Actors, Roles and Responsibilities
Authorized H10 integration/verification participants operate under source-defined governance. H10 records the external verification outcome but does not acquire source-credential authority. The External Education Integration Governance Authority governs H10 verification exception decisions where institutional authority is required.

# 7. Preconditions and Entry Criteria
- Credential reference/identifier is available.
- Trusted external verification context/provider is available.
- The verification request is authorized.
- A governed verification request/result window is known where expiry applies.

# 8. Trigger
An authorized process requests verification of an externally issued credential

# 9. Main Business Flow
1. Create the verification request in `Requested`.
2. Move to `Verification Pending` and obtain/evaluate governed provider evidence.
3. If authenticity is established under the external verification evidence, move to `Verified` and publish `ExternalCredentialVerificationVerified`.
4. If the governed verification decision is rejection, move to `Rejected` and publish `ExternalCredentialVerificationRejected`.
5. If the governed verification request/result window ends before a conclusive result can be established, move to `Expired` and publish `ExternalCredentialVerificationExpired`.
6. If governed evidence is available but insufficient or contradictory such that neither Verified nor Rejected can be asserted, move to `Inconclusive` and publish `ExternalCredentialVerificationInconclusive`.
7. Preserve immutable verification evidence and result lineage.
8. A later verification creates a successor request/result rather than overwriting the prior result.

# 10. Alternate Business Flows
- `Verification Pending → Expired` when the governed verification window ends without a conclusive result.
- `Verification Pending → Inconclusive` when governed evidence is available but insufficient or contradictory.
- Source-credential expiry remains a separate source-domain fact and shall not be used as a substitute for H10 verification-result `Expired`.
- A later attempt after any terminal verification result creates a successor request/result.

# 11. Failure, Interruption and Business Recovery
Provider unavailability or transport failure alone does not create an `Inconclusive` result. Such interruption remains unresolved until governed evidence exists or the governed verification window expires. No recovery branch mutates the source credential or prior verification result.

# 12. Business Rules and Decision Constraints
- Canonical verification lifecycle: `Requested → Verification Pending → Verified / Rejected / Expired / Inconclusive`.
- Verification results are immutable evidence for that request.
- `Expired` means the H10 verification request/result window ended; it is distinct from source-credential expiry.
- `Inconclusive` requires governed evidence that is insufficient or contradictory.
- Later verification creates a successor request/result.
- H10 verification does not change source-credential lifecycle.

# 13. State and Lifecycle Impact
Only the H10 verification-request/result lifecycle changes. Source credential lifecycle is unchanged by H10.

# 14. Business Events and Cross-Domain Collaboration
H10 publishes:
- `ExternalCredentialVerificationVerified`
- `ExternalCredentialVerificationRejected`
- `ExternalCredentialVerificationExpired`
- `ExternalCredentialVerificationInconclusive`

Source credentials/results retain their canonical owners and publishers.

# 15. Business Authorization, Approval and Audit Obligations
Preserve request authority, credential reference, external provider evidence, governed verification window where applicable, result, decision provenance and immutable successor lineage. H10 cannot rewrite source credentials.

# 16. Completion Criteria and Postconditions
Completion requires one of the four authoritative H10 verification-result outcomes—Verified, Rejected, Expired, or Inconclusive—with immutable evidence and complete lineage preserved. Source-credential ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions
## 17.1 Resolved Historical Blocker

The former missing `Expired` and `Inconclusive` verification-result semantics were resolved by locked `L2-H10_Enrollment_Cancellation_and_Verification_Result_Amendment_2026-08-27.md`, which defines the complete four-outcome verification lifecycle and exact H10 events.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions
No assumptions.
## 17.3 Deferred Questions
No additional scenario-specific questions.
