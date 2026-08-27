# External Enrollment Request to Confirmed or Rejected Enrollment Exchange

# 1. Scenario Metadata
**Scenario ID:** L3-H-076  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** C, D, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.2  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** locked L1; `DEPENDENCY_MAP.md`; L2-H10; `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md`; `L2-H10_Enrollment_Cancellation_and_Verification_Result_Amendment_2026-08-27.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose
Coordinate a governed external enrollment exchange while preserving Student/Academic enrollment authority outside H10.

# 3. Business Outcome
The external enrollment is submitted and confirmed, rejected, cancelled, or expired while the owning academic domain retains enrollment authority

# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
An eligible learner requests enrollment through an approved external provider.
## 4.2 End Boundary
The H10 Enrollment Exchange reaches Accepted, Rejected, Cancelled, or Expired; none of those exchange outcomes creates or mutates the canonical Student/Academic enrollment fact.
## 4.3 In Scope
External exchange request/validation, acceptance/rejection, governed pre-terminal cancellation, bounded-validity expiry, and audit evidence.
## 4.4 Out of Scope
Student/Academic enrollment decision ownership or mutation of canonical Student enrollment by H10.

# 5. Ownership and Participating Domains
H10 owns only the external Enrollment Exchange. C/D retain canonical Student/Academic enrollment authority and decide their own enrollment outcome.

# 6. Business Actors, Roles and Responsibilities
The **External Education Integration Governance Authority** governs H10 exchange cancellation and other governed exchange exceptions where institutional approval is required. The originating C/D domain may request exchange cancellation according to its own business context but does not become owner of the H10 exchange lifecycle. H10 does not approve academic enrollment.

# 7. Preconditions and Entry Criteria
- Owning Student/Academic governance permits the exchange.
- Provider/trust/connector context is governed.
- Bounded validity is known where expiry applies.
- Cancellation may occur only before H10 reaches Accepted or Rejected.

# 8. Trigger
An eligible learner requests enrollment through an approved external provider

# 9. Main Business Flow
1. Record the Enrollment Exchange in `Requested`.
2. Move to `Validation Pending` and validate provider, learner-reference and exchange context.
3. If valid for external exchange, move to `Accepted` and publish `ExternalEnrollmentExchangeAccepted`.
4. The owning C/D domain independently determines whether to establish its canonical enrollment; H10 acceptance does not create it.
5. Preserve complete exchange and source-domain decision references.

# 10. Alternate Business Flows
- `Validation Pending → Rejected`, publishing `ExternalEnrollmentExchangeRejected`.
- `Requested / Validation Pending → Cancellation Requested` when a governed pre-terminal cancellation request is accepted for review; publish `ExternalEnrollmentExchangeCancellationRequested`.
- `Cancellation Requested → Cancelled` after the governed H10 cancellation decision; publish `ExternalEnrollmentExchangeCancelled`.
- If an accepted exchange has a bounded validity period and the owning Student/Academic domain has not established the canonical enrollment before that period ends, move `Accepted → Expired` and publish `ExternalEnrollmentExchangeExpired`.
- An Accepted exchange is not locally moved to Cancelled; any later academic consequence remains governed by the owning C/D domain.

# 11. Failure, Interruption and Business Recovery
Exchange failure/rejection/expiry/cancellation remains an H10 exchange outcome and never becomes an academic enrollment decision. Invalid cancellation authority or a cancellation request after H10 terminal acceptance/rejection does not alter the current exchange outcome. Later exchange activity creates governed successor/current records rather than altering source-domain history.

# 12. Business Rules and Decision Constraints
- Canonical Enrollment Exchange lifecycle: `Requested → Validation Pending → Accepted / Rejected`.
- Cancellation branch: `Requested / Validation Pending → Cancellation Requested → Cancelled`.
- Accepted bounded-validity exchange may become `Expired` under the locked amendment condition.
- H10 exchange acceptance, rejection, cancellation or expiry never creates/mutates canonical Student enrollment.
- C/D retain enrollment ownership.

# 13. State and Lifecycle Impact
Only the H10 exchange lifecycle changes. Student/Academic enrollment lifecycle remains unchanged unless its canonical owner acts through its own workflow.

# 14. Business Events and Cross-Domain Collaboration
H10 publishes:
- `ExternalEnrollmentExchangeAccepted`
- `ExternalEnrollmentExchangeRejected`
- `ExternalEnrollmentExchangeExpired`
- `ExternalEnrollmentExchangeCancellationRequested`
- `ExternalEnrollmentExchangeCancelled`

C/D source facts retain their publishers and ownership.

# 15. Business Authorization, Approval and Audit Obligations
External Education Integration Governance Authority owns governed H10 exchange cancellation/exception decisions where required; academic enrollment authority remains C/D-owned. Preserve originating request, provider/exchange context, cancellation request/decision where applicable, H10 outcome, validity/expiry evidence, and any separate canonical enrollment reference.

# 16. Completion Criteria and Postconditions
Completion requires a governed H10 exchange outcome—Accepted, Rejected, Cancelled, or Expired—with complete evidence and no transfer of canonical enrollment ownership to H10.

# 17. Open Issues, Assumptions and Deferred Questions
## 17.1 Resolved Historical Blocker
The former missing H10 Enrollment Exchange cancellation state/transition/authority/events were resolved by locked `L2-H10_Enrollment_Cancellation_and_Verification_Result_Amendment_2026-08-27.md`. Existing expiry semantics remain governed by the locked 2026-08-26 H10 amendment.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`
## 17.2 Assumptions
No assumptions.
## 17.3 Deferred Questions
No additional scenario-specific questions.
