# Published Assessment Blueprint to Governed Assessment Window

**Scenario ID:** L3-D-052  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D  
**Scenario Type:** Simple  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0772 / CAN-L3-D-052

---

# 2. Scenario Purpose

Create and govern an Assessment Window from a Published blueprint through opening, authorized extension and closure.

# 3. Business Outcome

A blueprint-bound Assessment Window is opened and controlled under its release, deadline, extension and closure rules, enabling submissions only while permitted.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Published Internal Assessment Blueprint and valid calendar/course-offering context are available.

## 4.2 End Boundary

The governed window is Closed after its permitted activity period; `AssessmentWindowOpened` records opening and closure remains auditable.

## 4.3 In Scope

- Create window bound to Published blueprint.
- Validate release, deadline and academic context.
- Open, extend when authorized and close the window.

## 4.4 Out of Scope

- Submission assessment or moderation.
- Changing the Published blueprint through window operations.
- Timetable or Academic Calendar ownership.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns AssessmentWindow release, deadline, extension and closure control. |
| Section B / D01 | Own calendar and Course Offering references consumed by D07. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Policy-configured D07 window administrator | Creates, opens, extends and closes the window under policy. | Open/Extend/CloseAssessmentWindow authority. |
| Policy-configured D07 exception authority | Approves exceptional extension where required. | Extension decision authority. |

# 7. Preconditions and Entry Criteria

- Blueprint is Published.
- Release and deadline align with authoritative academic context.

# 8. Trigger

The Published blueprint is ready for assessment activity scheduling.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create Assessment Window linked to Published blueprint. | D07 window administrator | Governed window exists. |
| 2 | Validate release, deadline, course context and policy. | D07 | Window is valid or rejected. |
| 3 | Open the window at authorized release. | D07 window administrator | Published → Open; AssessmentWindowOpened. |
| 4 | Accept activity while open and track In Progress state. | D07 | Window-controlled assessment proceeds. |
| 5 | Extend deadline only through authorized decision. | D07 exception authority | New deadline and reason audited. |
| 6 | Close window at deadline or authorized closure. | D07 window administrator | Window Closed; new submissions prohibited. |

# 10. Alternate Business Flows

A failed validation, rejection, configured optional approval path, escalation or compensation follows only the alternatives explicitly supported above; it never creates the successful final outcome or rewrites prior facts.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the prior authoritative state. | Internal Assessment |
| Authorization or approval failure | Requested outcome is not permitted. | Retain the prior state and record the decision reason. | Internal Assessment |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity and idempotency. | Internal Assessment |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change its owned business state. | Internal Assessment | Prevents cross-domain ownership leakage. | `L2-D07_Internal_Assessment_Workflows.md` |
| Each transition requires valid lifecycle position, authority, policy and evidence. | Internal Assessment | Prevents governance bypass. | `L2-D07_Internal_Assessment_Workflows.md` |
| Published or historical business facts remain immutable. | Internal Assessment | Preserves auditability and lineage. | `L2-D07_Internal_Assessment_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Assessment Window | Published blueprint / configured window | Open, operate, optionally extend and close. | Published → Open → In Progress → Closed | D07 | L2-D07 §§5–8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| AssessmentWindowOpened | D07 | Submission, Notification and authorized consumers | Records governed window opening. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section B / D01 | Internal Assessment | Own calendar and Course Offering references consumed by D07. | Preserve ownership while enabling the scenario. | `L2-D07_Internal_Assessment_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Creates, opens, extends and closes the window under policy. | Policy-configured D07 window administrator | Open/Extend/CloseAssessmentWindow authority. | Internal Assessment |
| Approves exceptional extension where required. | Policy-configured D07 exception authority | Extension decision authority. | Internal Assessment |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation, review, approval and execution remain distinct wherever source or configured policy requires separation. Automated services execute rules but never make governance decisions. Any override requires authority, justification, evidence and audit history.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario lifecycle and decision | Actor, authority, reason, prior state, validation evidence, decision and resulting state. | Internal Assessment |
| Source and cross-domain traceability | Source identifiers, versions, evidence references, consumed events and published facts. | Internal Assessment |
| Failure and recovery | Rejected validations, interruptions, retries, escalation or compensation outcome. | Internal Assessment |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A blueprint-bound Assessment Window is opened and controlled under its release, deadline, extension and closure rules, enabling submissions only while permitted.
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
**Validation Basis:** Section D canonical register; `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
