# Published Assessment Blueprint to Governed Assessment Window

# 1. Scenario Metadata

**Scenario ID:** L3-D-052  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Internal Assessment  
**Participating Domains:** D  
**Scenario Type:** Simple  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D07_Internal_Assessment_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0772 / CAN-L3-D-052

---

# 2. Scenario Purpose

Create and govern an Assessment Window from a Published blueprint through opening, any authorized extension and closure.

# 3. Business Outcome

A blueprint-bound Assessment Window is governed under its release, deadline, extension and closure rules, enabling assessment activity only while permitted.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Published Internal Assessment Blueprint and valid academic context are available.

## 4.2 End Boundary

The governed Assessment Window reaches Closed after its permitted activity period; `AssessmentWindowOpened` records the authoritative opening fact and closure remains auditable.

## 4.3 In Scope

- Establish the window against a Published blueprint.
- Validate release, deadline and academic context.
- Open the window, apply an authorized extension where permitted, and close it.

## 4.4 Out of Scope

- Assessment submission/moderation.
- Changing the Published blueprint.
- Timetable or Academic Calendar ownership.
- Command/API/permission design.

## 4.5 Related Scenarios — Optional

L3-D-051 provides the Published blueprint prerequisite; downstream submission scenarios may consume the Open window state.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D07 | Owns AssessmentWindow release, deadline, extension and closure control. |
| Section B / D01 | Own calendar and Course Offering references consumed by D07. |

# 6. Business Actors, Roles and Responsibilities

Locked L2-D07 does not define canonical actors. This scenario therefore assigns business behaviour to D07 and records the unresolved institutional role mapping in Section 17 rather than inventing window-administrator or exception-authority roles.

# 7. Preconditions and Entry Criteria

- Blueprint is Published.
- Release and deadline align with authoritative academic context.

# 8. Trigger

The Published blueprint is ready for governed assessment activity scheduling.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish an Assessment Window linked to the Published blueprint. | D07 | Governed window exists. |
| 2 | Validate release, deadline, course context and policy. | D07 | Window is valid or remains blocked. |
| 3 | Open the window under applicable institutional authority. | D07 | Window becomes Open and `AssessmentWindowOpened` is published. |
| 4 | Govern assessment activity while the window remains Open/In Progress. | D07 | Activity occurs only within the permitted window. |
| 5 | Where policy permits, approve and record an extension with reason and revised deadline. | D07 under applicable business authority | Governed extended deadline is established. |
| 6 | Close the window at the governed deadline or authorized closure point. | D07 | Window becomes Closed and new submissions are prohibited. |

# 10. Alternate Business Flows

## 10.1 Authorized Extension

Where institutional policy permits and the required authority approves an extension, the deadline is revised with reason and audit evidence before the window later reaches Closed. This path still reaches the same Closed business outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Blueprint/context/deadline validation fails | Window cannot validly open. | Resolve the invalid business context before opening. | D07 |
| Required extension or closure authority cannot be validated | Requested change cannot proceed. | Retain the current window state until a valid governed decision exists. | D07 |
| Window progression is interrupted | The next lifecycle outcome has not occurred. | Preserve the last valid business position and continue when the blocking condition is resolved. | D07 |

# 12. Business Rules and Decision Constraints

- A blueprint must be Published before an Assessment Window can open. Source: L2-D07 §5.
- D07 owns AssessmentWindow release, deadline, extension and closure. Source: L2-D07 §6.2.
- D07 does not own calendar/Course Offering facts. Source: L2-D07 §§2,9.
- No new lifecycle state or approval rule is introduced by this scenario.

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Assessment Window | Published blueprint / configured window | Open, operate, optionally extend and close. | Published → Open → In Progress → Closed | D07 | L2-D07 §§5–8 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AssessmentWindowOpened` | D07 | Submission, Notification and authorized consumers | Records governed window opening. | L2-D07 §7 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section B/D01 provide authoritative calendar and Course Offering context; D07 retains AssessmentWindow ownership.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Exact institutional role names for opening, extending and closing Assessment Windows are not defined in locked L2-D07. L3 therefore records D07 responsibility without converting L2 commands into permissions.

## 15.2 Separation of Duties, Overrides and Exceptions

Any institutional separation between window administration and exceptional extension approval must be established upstream; L3 does not invent the role mapping.

## 15.3 Business Audit and Evidence Obligations

Retain blueprint reference, academic context, release/deadline, extension reason/decision where applicable, window-state history and opening fact.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The window has followed the D07 lifecycle to Closed.
- Any extension was governed under applicable institutional authority.
- Opening and lifecycle evidence remain auditable.

## 16.2 Postconditions

- D07 retains AssessmentWindow ownership.
- New assessment submissions are not permitted after closure.
- Source-domain ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L2-D07 has no Actors section. | Canonical institutional roles for opening, extending and closing an Assessment Window are not established upstream. | Define/approve D07 actor-authority mapping before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Commands, APIs, technical RBAC, scheduling mechanisms and implementation recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** `L2-D07_Internal_Assessment_Workflows.md`; Template v2.0; Architecture Level Validation Standards.
