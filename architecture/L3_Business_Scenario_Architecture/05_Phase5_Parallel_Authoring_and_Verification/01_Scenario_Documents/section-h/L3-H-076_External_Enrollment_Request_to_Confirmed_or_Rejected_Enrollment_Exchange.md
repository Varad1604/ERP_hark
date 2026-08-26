# External Enrollment Request to Confirmed or Rejected Enrollment Exchange

# 1. Scenario Metadata
**Scenario ID:** L3-H-076  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** C, D, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** locked L1; `DEPENDENCY_MAP.md`; L2-H10

# 2. Scenario Purpose
Coordinate an approved external enrollment exchange while preserving student/academic enrollment authority outside H10.

# 3. Business Outcome
The external enrollment is submitted and confirmed, rejected, cancelled, or expired while the owning academic domain retains enrollment authority

# 4. Scenario Boundary and Scope
## 4.1 Start Boundary
An eligible learner requests enrollment through an approved external provider.
## 4.2 End Boundary
H10 explicitly supports submitted, confirmed, rejected and pending-cancellation behavior, but no canonical `Expired` enrollment outcome/event/lifecycle is defined. Full frozen outcome is therefore blocked.
## 4.3 In Scope
Submission, provider acknowledgement, confirmation/rejection, pending cancellation, audit evidence.
## 4.4 Out of Scope
Academic enrollment decision ownership and inventing an expired H10 enrollment state.

# 5. Ownership and Participating Domains
H10 owns External Enrollment orchestration; C/D retain student/academic enrollment authority.

# 6. Business Actors, Roles and Responsibilities
Academic Administrator participates in enrollment commands; Integration Administrator/Operator coordinate H10 exchange. Owning C/D authority remains upstream.

# 7. Preconditions and Entry Criteria
Owning student/academic workflow has approved external enrollment participation; provider/connector are governed active.

# 8. Trigger
An eligible learner requests enrollment through an approved external provider

# 9. Main Business Flow
1. H10 receives the approved enrollment request.
2. H10 validates the external exchange context.
3. H10 submits the external enrollment.
4. Provider acknowledgement is recorded.
5. H10 records confirmed or rejected external outcome.
6. Pending enrollment may follow the source-defined cancellation action.
7. Stop before asserting an `Expired` outcome because H10 defines none.

# 10. Alternate Business Flows
Confirmed, rejected and pending cancellation are supported; expired is not.

# 11. Failure, Interruption and Business Recovery
Exchange failures remain H10 exceptions/reconciliation issues and never become academic enrollment decisions.

# 12. Business Rules and Decision Constraints
Academic ownership remains outside H10; provider acknowledgement plus canonical owner confirmation is required for completion.

# 13. State and Lifecycle Impact
External Enrollment lifecycle details are incomplete in locked H10; confirmation/rejection events are explicit, cancellation command exists, expiry is undefined.

# 14. Business Events and Cross-Domain Collaboration
## 14.1 Published / Consumed Business Events
`ExternalEnrollmentSubmitted`, `ExternalEnrollmentConfirmed`, `ExternalEnrollmentRejected`, `ExternalEnrollmentCompensationRequested`; H10 also defines enrollment confirmation/rejection events. No `EnrollmentExpired` event exists.
## 14.2 Business Contracts and Cross-Domain Hand-offs
C/D retain enrollment authority; H10 synchronizes external status only.

# 15. Business Authorization, Approval and Audit Obligations
## 15.1 Initiation and Decision Authority
Academic approval originates in the owning student/academic workflow; H10 does not approve academic enrollment.
## 15.2 Separation of Duties, Overrides and Exceptions
Provider confirmation cannot transfer academic authority to H10.
## 15.3 Business Audit and Evidence Obligations
Preserve originating approval, provider submission/acknowledgement, confirmation/rejection/cancellation evidence.

# 16. Completion Criteria and Postconditions
## 16.1 Completion Criteria
Supported branches are traceable; expired branch is unresolved.
## 16.2 Postconditions
No unsupported expiry outcome is asserted.

# 17. Open Issues, Assumptions and Deferred Questions
## 17.1 Open Issues
Frozen outcome requires `expired`; locked H10 defines no expiration state, rule or event for External Enrollment. **Architecture assessment:** `BLOCKED`
## 17.2 Assumptions
No assumptions.
## 17.3 Deferred Questions
Upstream H10 must define External Enrollment expiration semantics.
