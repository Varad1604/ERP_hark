# Credit Mapping Rule Configuration

**Scenario ID:** L3-D-046  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0766 / CAN-L3-D-046

---

# 2. Scenario Purpose

Define, validate, approve and publish versioned institutional equivalency rules between flexible/external learning and the canonical curriculum.

# 3. Business Outcome

Exactly one active mapping exists for the applicable curriculum version, becomes available for future recognition cases, and does not retroactively alter completed recognition decisions.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A curriculum revision, new approved opportunity or equivalency-governance change requires new credit-mapping rules.

## 4.2 End Boundary

The governed mapping version is published and `CreditMappingRulesPublished` is available to future Credit Recognition workflows.

## 4.3 In Scope

- Select canonical curriculum and learning outcomes.
- Define equivalency, credit values, scope and effective period.
- Validate duplicates, limits, policy and approval.
- Publish version and preserve predecessor/history.

## 4.4 Out of Scope

- Credit-recognition decision for an individual student.
- Direct transcript posting.
- Retroactive change to completed recognitions.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Credit Mapping rule versions and external-to-internal equivalency configuration. |
| Section C / Academic Records | Owns permanent student academic records and transcript facts. |
| Section B / Curriculum domain | Owns curriculum, course and learning-outcome definitions referenced by the mapping. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Curriculum Committee | Evaluates academic equivalency and proposed credit values. | Academic recommendation authority. |
| Academic Affairs | Authors and validates mapping configuration. | Configuration responsibility. |
| Authorized academic governance authority | Approves mapping publication. | Final mapping authority. |

# 7. Preconditions and Entry Criteria

- Canonical curriculum version is Published.
- Academic approval path and candidate learning opportunity/provider evidence are available.

# 8. Trigger

Curriculum or equivalency policy changes require a new mapping version.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create mapping version against canonical curriculum and opportunity. | Academic Affairs | Draft mapping exists. |
| 2 | Define outcome equivalency, credit values and permitted recognition scope. | Curriculum Committee | Academic mapping proposal complete. |
| 3 | Validate duplicate mapping, curriculum availability, credit limits and policy. | D06 | Mapping valid or returned. |
| 4 | Review and approve mapping. | Academic governance authority | Approval recorded. |
| 5 | Publish as the sole active mapping for that curriculum version. | D06 | CreditMappingRulesPublished. |
| 6 | Preserve prior versions and apply new version only to future recognitions. | D06 | Historical decisions remain unchanged. |

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
| Credit Mapping Rule | Draft / existing active mapping | Govern new version and effective position. | Validated → Approved → Published/Active; prior version retained | D06 | L2-D06 W62; ADR-007/008 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CreditMappingRulesPublished | D06 | Credit Recognition, Section C and authorized curriculum consumers | Records effective equivalency rule version. | L2-D06 W62; D06-E53 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C / Academic Records | Open Electives & MOOCs | Owns permanent student academic records and transcript facts. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| Section B / Curriculum domain | Open Electives & MOOCs | Owns curriculum, course and learning-outcome definitions referenced by the mapping. | Preserve ownership while enabling the scenario. | `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Evaluates academic equivalency and proposed credit values. | Curriculum Committee | Academic recommendation authority. | Open Electives & MOOCs |
| Authors and validates mapping configuration. | Academic Affairs | Configuration responsibility. | Open Electives & MOOCs |
| Approves mapping publication. | Authorized academic governance authority | Final mapping authority. | Open Electives & MOOCs |

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

- Exactly one active mapping exists for the applicable curriculum version, becomes available for future recognition cases, and does not retroactively alter completed recognition decisions.
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
