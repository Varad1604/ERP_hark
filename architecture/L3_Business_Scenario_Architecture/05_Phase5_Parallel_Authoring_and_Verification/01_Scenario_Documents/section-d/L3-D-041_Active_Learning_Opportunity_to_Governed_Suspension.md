# Active Learning Opportunity to Governed Suspension

# 1. Scenario Metadata

**Scenario ID:** L3-D-041  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0762 / CAN-L3-D-042 — split child A

---

# 2. Scenario Purpose

Temporarily suspend new enrollment into an Active Learning Opportunity through an authorized, reasoned and auditable D06 decision while preserving the opportunity and existing approved enrollments.

# 3. Business Outcome

Learning Opportunity becomes Suspended for the governed effective period; new enrollments are prohibited and existing Approved enrollments remain valid unless separately revoked.

# 4. Scenario Boundary and Scope

Starts when a governed administrative/provider/policy condition requires suspension; ends when D06 reaches Suspended and `LearningOpportunitySuspended` is published. In scope: reason/authority/effective period, suspension, new-enrollment prohibition, existing-enrollment preservation, participant notification obligation. Out of scope: archival/deletion, automatic revocation, technical service execution.

# 5. Ownership and Participating Domains

D06 owns Learning Opportunity availability, suspension state and suspension history.

# 6. Business Actors, Roles and Responsibilities

Academic Affairs requests/authorizes governed suspension where established by policy. Applying the D06 state change is a domain responsibility, not a `D06 Learning Opportunity Service` actor.

# 7. Preconditions and Entry Criteria

Learning Opportunity is Active; suspension reason, authority and effective period are available.

# 8. Trigger

An authorized administrative decision, provider condition or policy condition requires temporary suspension.

# 9. Main Business Flow

1. Academic Affairs/D06 establishes suspension request/reason/scope/effective period.
2. D06 validates Active state, authority, policy and affected enrollments.
3. D06 transitions Active → Suspended and prohibits new enrollment.
4. Existing Approved enrollments remain valid unless separately governed.
5. D06 records/publishes `LearningOpportunitySuspended`; required participant communication follows business policy.

# 10. Alternate Business Flows

No alternate route reaches Suspended without the same governed decision. Invalid/rejected suspension is handled in Section 11.

# 11. Failure, Interruption and Business Recovery

Invalid state/reason/authority blocks suspension; rejected authority retains Active state; interrupted progression preserves last authoritative opportunity state until resolved.

# 12. Business Rules and Decision Constraints

Only D06 changes Learning Opportunity state; suspension does not implicitly revoke existing Approved enrollments; historical facts remain traceable; technical execution/retry is deferred.

# 13. State and Lifecycle Impact

Learning Opportunity: Active → Suspended. Owner D06. Source L2-D06 W06/Category A transition.

# 14. Business Events and Cross-Domain Collaboration

D06 publishes `LearningOpportunitySuspended` under L2-D06 W06/D06-E04. No cross-domain ownership transfer occurs.

# 15. Business Authorization, Approval and Audit Obligations

Academic Affairs may authorize only under governing policy. Audit retains reason, authority, effective period, prior state, affected-enrollment assessment and suspension fact.

# 16. Completion Criteria and Postconditions

Opportunity is Suspended; new enrollment is prohibited; existing Approved enrollments are preserved unless separately governed; D06 ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Service implementation, notification transport and technical recovery are deferred.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
