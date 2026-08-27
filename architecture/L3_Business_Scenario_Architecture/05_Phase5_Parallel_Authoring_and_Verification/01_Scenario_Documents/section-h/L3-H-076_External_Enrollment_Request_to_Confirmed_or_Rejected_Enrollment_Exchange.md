# External Enrollment Request to Confirmed or Rejected Enrollment Exchange

# 1. Scenario Metadata
**Scenario ID:** L3-H-076  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** C, D, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** locked L1; `DEPENDENCY_MAP.md`; L2-H10; `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose
Coordinate a governed external enrollment exchange while preserving Student/Academic enrollment authority outside H10.

# 3. Business Outcome
The external enrollment is submitted and confirmed, rejected, cancelled, or expired while the owning academic domain retains enrollment authority

# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
An eligible learner requests enrollment through an approved external provider.
## 4.2 End Boundary
The H10 Enrollment Exchange reaches Accepted, Rejected, Expired, or the existing governed cancellation outcome for a pending exchange; none of those exchange outcomes creates the canonical Student enrollment fact.
## 4.3 In Scope
External exchange request/validation, acceptance/rejection, bounded-validity expiry, existing pending-cancellation behavior, and audit evidence.
## 4.4 Out of Scope
Student/Academic enrollment decision ownership or mutation of canonical Student enrollment by H10.

# 5. Ownership and Participating Domains
H10 owns only the external Enrollment Exchange. C/D retain canonical Student/Academic enrollment authority and decide their own enrollment outcome.

# 6. Business Actors, Roles and Responsibilities
The **External Education Integration Governance Authority** governs H10 exchange exceptions. Existing Academic/Integration participants operate under their source-defined responsibilities; H10 does not approve academic enrollment.

# 7. Preconditions and Entry Criteria
Owning Student/Academic governance permits the exchange; provider/trust/connector context is governed; bounded validity is known where expiry applies.

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
- If an accepted exchange has a bounded validity period and the owning Student/Academic domain has not established the canonical enrollment before that period ends, move the H10 exchange to `Expired` and publish `ExternalEnrollmentExchangeExpired`.
- Existing source-defined pending-cancellation behavior may cancel an exchange before terminal completion; cancellation remains an H10 exchange outcome and does not cancel a canonical Student enrollment unless its owner separately decides so.

# 11. Failure, Interruption and Business Recovery
Exchange failure/rejection/expiry/cancellation remains an H10 transaction outcome and never becomes an academic enrollment decision. Later exchange activity creates governed successor/current records rather than altering source-domain history.

# 12. Business Rules and Decision Constraints
- Canonical Enrollment Exchange lifecycle: `Requested → Validation Pending → Accepted / Rejected`.
- Accepted bounded-validity exchange may become `Expired` under the locked amendment condition.
- H10 exchange acceptance never creates/mutates canonical Student enrollment.
- C/D retain enrollment ownership.

# 13. State and Lifecycle Impact
Only the H10 exchange lifecycle changes. Student/Academic enrollment lifecycle remains unchanged unless its canonical owner acts through its own workflow.

# 14. Business Events and Cross-Domain Collaboration
H10 publishes `ExternalEnrollmentExchangeAccepted`, `ExternalEnrollmentExchangeRejected`, and `ExternalEnrollmentExchangeExpired`; existing cancellation facts remain governed by the base H10 workflow. C/D source facts retain their publishers.

# 15. Business Authorization, Approval and Audit Obligations
H10 governance owns exchange exception decisions; academic enrollment authority remains C/D-owned. Preserve originating permission, provider/exchange context, H10 outcome, validity/expiry evidence, cancellation evidence where applicable, and any separate canonical enrollment reference.

# 16. Completion Criteria and Postconditions
Completion requires a governed H10 exchange outcome with complete evidence and no transfer of canonical enrollment ownership to H10.

# 17. Open Issues, Assumptions and Deferred Questions
## 17.1 Resolved Historical Blocker
The former missing exchange-expiry state/event was resolved by locked `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`
## 17.2 Assumptions
No assumptions.
## 17.3 Deferred Questions
No additional scenario-specific questions.
