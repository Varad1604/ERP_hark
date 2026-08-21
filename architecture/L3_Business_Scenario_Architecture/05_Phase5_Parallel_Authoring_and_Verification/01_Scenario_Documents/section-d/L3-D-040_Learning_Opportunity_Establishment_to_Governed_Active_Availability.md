# Learning Opportunity Establishment to Governed Active Availability

**Scenario ID:** L3-D-040  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** B, D  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0761 / CAN-L3-D-041 (final L3-D-040)

---

# 2. Scenario Purpose

Establish an Open Elective or approved MOOC learning opportunity from canonical academic/provider references and make it actively available for governed discovery and enrollment.

# 3. Business Outcome

A fully configured, approved and Active Learning Opportunity is discoverable within its governed audience, capacity and enrollment window.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved D01 Course Offering or approved mapped provider course is designated for flexible-learning use.

## 4.2 End Boundary

The opportunity reaches Active and `LearningOpportunityActivated` is published after publication and readiness validation.

## 4.3 In Scope

- Validate canonical offering/provider and academic-session references.
- Publish Open Elective or MOOC opportunity.
- Configure provider mapping, capacity, waitlist, audience and enrollment window.
- Validate readiness and activate availability.

## 4.4 Out of Scope

- Creating Course, Course Offering, curriculum or provider accreditation masters.
- Student enrollment decision.
- Bypassing academic or provider approval.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Learning Opportunity projection/configuration, flexible-learning availability, capacity and activation. |
| D01 / Section B | Owns Course, Course Offering, curriculum, academic-session and policy references. |
| External Provider Registry | Owns approved provider identity/accreditation facts referenced by D06. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Affairs Administrator | Publishes and activates governed opportunity. | Publication and activation authority. |
| Department Academic Coordinator / MOOC Coordinator | Configures academic audience, mapping and offering details. | Configuration responsibility. |
| Integration Administrator | Configures approved external-provider synchronization where applicable. | Integration configuration only. |

# 7. Preconditions and Entry Criteria

- Canonical offering or provider course is approved.
- Academic session and mapping are valid.
- Enrollment window and capacity can be configured.

# 8. Trigger

An approved academic or provider learning option is authorized for flexible-learning availability.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Classify opportunity as Open Elective or approved external/MOOC. | Academic Affairs | Governed establishment path selected. |
| 2 | Validate canonical offering/provider, academic session, mapping and approval. | D06 using authoritative references | Establishment basis is valid. |
| 3 | Publish the configured learning opportunity for its permitted audience. | Academic Affairs Administrator | OpenElectiveOfferingPublished or MOOCOpportunityPublished. |
| 4 | Configure provider integration where external. | Integration Administrator / Academic Affairs | ProviderOfferingConfigured. |
| 5 | Configure capacity, reserved categories, waitlist and enrollment window. | Academic Affairs / Coordinator | Readiness configuration complete. |
| 6 | Validate published status, capacity and active enrollment window. | D06 | Opportunity is activation-ready. |
| 7 | Activate and update governed discovery. | Academic Affairs | LearningOpportunityActivated. |

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
| Learning Opportunity | Approved source / configured opportunity | Publish, configure and activate governed availability. | Configured → Published → Active | D06 | L2-D06 W01–W05 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| OpenElectiveOfferingPublished / MOOCOpportunityPublished / ProviderOfferingConfigured / ElectiveCapacityConfigured / LearningOpportunityActivated | D06 | Section B, Section C, student discovery and enrollment consumers | Records establishment, configuration and active availability. | L2-D06 W01–W05; Event Catalogue |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| D01 / Section B | Open Electives & MOOCs | Owns Course, Course Offering, curriculum, academic-session and policy references. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| External Provider Registry | Open Electives & MOOCs | Owns approved provider identity/accreditation facts referenced by D06. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Publishes and activates governed opportunity. | Academic Affairs Administrator | Publication and activation authority. | Open Electives & MOOCs |
| Configures academic audience, mapping and offering details. | Department Academic Coordinator / MOOC Coordinator | Configuration responsibility. | Open Electives & MOOCs |
| Configures approved external-provider synchronization where applicable. | Integration Administrator | Integration configuration only. | Open Electives & MOOCs |

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

- A fully configured, approved and Active Learning Opportunity is discoverable within its governed audience, capacity and enrollment window.
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
