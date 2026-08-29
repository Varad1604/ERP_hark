# Learning Opportunity Establishment to Governed Active Availability

# 1. Scenario Metadata

**Scenario ID:** L3-D-040  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** B, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0761 / CAN-L3-D-041 (final L3-D-040)

---

# 2. Scenario Purpose

Establish an Open Elective or approved external/MOOC Learning Opportunity from canonical academic/provider references and make it actively available for governed discovery and enrollment.

# 3. Business Outcome

A fully governed, approved Learning Opportunity reaches Active and is discoverable within its authorized audience, capacity and enrollment window.

# 4. Scenario Boundary and Scope

Starts when an approved D01 Course Offering or approved mapped provider course is designated for flexible-learning use; ends when the opportunity reaches Active and `LearningOpportunityActivated` is published by D06.

In scope: validate authoritative offering/provider and session references, establish opportunity configuration, capacity/waitlist/audience/window, confirm institutional readiness and activate. Out of scope: source-master creation, enrollment decision, technical provider synchronization/authentication/connectivity configuration.

# 5. Ownership and Participating Domains

D06 owns Learning Opportunity configuration, flexible-learning availability, capacity and activation. D01/Section B owns Course/Course Offering/curriculum/session/policy references. Approved external provider identity/accreditation facts remain owned by their source authority.

# 6. Business Actors, Roles and Responsibilities

Academic Affairs Administrator may publish/activate the governed opportunity where supported by policy. Department Academic Coordinator/MOOC Coordinator may prepare academic audience/mapping/offering details. Technical `Integration Administrator` is removed as an L3 actor; external-provider relationship readiness is a D06 business condition.

# 7. Preconditions and Entry Criteria

Canonical offering/provider course is approved; session/mapping is valid; governed audience/capacity/enrollment window can be established.

# 8. Trigger

An approved academic or provider learning option is authorized for flexible-learning availability.

# 9. Main Business Flow

1. D06/Academic Affairs classifies the opportunity as Open Elective or approved external/MOOC.
2. D06 validates authoritative offering/provider, academic session, mapping and approval.
3. D06 publishes the configured opportunity — `OpenElectiveOfferingPublished` or `MOOCOpportunityPublished`.
4. Where external, D06 records the approved provider relationship/configuration business fact — `ProviderOfferingConfigured` as defined upstream.
5. D06 establishes capacity, reserved categories, waitlist, audience and enrollment window — `ElectiveCapacityConfigured` as applicable.
6. D06 validates business readiness.
7. Authorized D06 business action activates the opportunity — `LearningOpportunityActivated`.

# 10. Alternate Business Flows

No distinct alternate path reaches Active. Invalid source/provider/readiness conditions block activation and belong in Section 11.

# 11. Failure, Interruption and Business Recovery

Invalid offering/provider/session/mapping references block activation until authoritative business conditions are resolved. Missing activation authority blocks the Active outcome. Interrupted establishment preserves the last authoritative D06 opportunity position and continues once blocking conditions are resolved.

# 12. Business Rules and Decision Constraints

D06 alone changes Learning Opportunity state; source academic/provider facts remain externally owned; activation requires approved/readiness conditions; technical provider integration mechanisms are outside L3.

# 13. State and Lifecycle Impact

Learning Opportunity: Configured → Published → Active. Owner D06. Source L2-D06 W01–W05.

# 14. Business Events and Cross-Domain Collaboration

D06 publishes `OpenElectiveOfferingPublished`, `MOOCOpportunityPublished`, `ProviderOfferingConfigured`, `ElectiveCapacityConfigured`, and `LearningOpportunityActivated` as defined in L2-D06 W01–W05/Event Catalogue. Section B/provider authorities retain source ownership.

# 15. Business Authorization, Approval and Audit Obligations

Only institutional roles established by D06 policy may publish/activate. Audit retains source references, opportunity scope, capacity/audience/window, approval/readiness evidence, authority and activation fact.

# 16. Completion Criteria and Postconditions

Opportunity is Active/discoverable under governed audience/capacity/window; source ownership remains unchanged; D06 owns availability.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Provider authentication, synchronization, connectivity, APIs and infrastructure readiness are deferred to later levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
