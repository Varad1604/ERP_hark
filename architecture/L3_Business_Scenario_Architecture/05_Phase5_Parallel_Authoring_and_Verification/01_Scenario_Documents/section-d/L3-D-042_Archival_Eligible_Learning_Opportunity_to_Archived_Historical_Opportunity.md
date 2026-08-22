# Archival-Eligible Learning Opportunity to Archived Historical Opportunity

**Scenario ID:** L3-D-042  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0762 / CAN-L3-D-042 — split child B

---

# 2. Scenario Purpose

Archive a completed Learning Opportunity after all academic and operational obligations are closed, preserving it as immutable searchable history.

# 3. Business Outcome

The opportunity is removed from the active catalogue and retained as a read-only historical record with complete audit, retention and compliance evidence.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Learning Opportunity is archival-eligible after enrollment closure, completion of credit recognition and resolution of all pending workflows.

## 4.2 End Boundary

The opportunity reaches Archived and `LearningOpportunityArchived` is published.

## 4.3 In Scope

- Validate enrollment, recognition, workflow, legal-hold and retention readiness.
- Lock opportunity and archive metadata/history.
- Remove from active discovery while retaining compliant searchability.

## 4.4 Out of Scope

- Deleting opportunity history or audit evidence.
- Archiving with pending enrollments, recognition or legal hold.
- Reactivating an archived opportunity.

## 4.5 Related Scenarios — Optional

Related scenarios are referenced only where their completed business facts cross this boundary; they do not transfer ownership.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D06 | Owns Learning Opportunity archival, historical catalogue and retained audit evidence. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Academic Affairs | Reviews academic closure and authorizes archival. | Archival authority. |
| Enterprise Administrator | Executes governed preservation and retention controls. | Administrative archival execution. |

# 7. Preconditions and Entry Criteria

- Enrollment is Closed.
- Credit recognition is completed or formally closed.
- No pending workflow, investigation or legal hold blocks archival.

# 8. Trigger

Academic closure makes the Learning Opportunity eligible for archival.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Evaluate archival eligibility and all blocking obligations. | Academic Affairs / D06 | Eligibility confirmed or archival blocked. |
| 2 | Approve archival with retention and compliance reference. | Academic Affairs | Authorization recorded. |
| 3 | Lock operational opportunity state. | D06 | No further operational changes permitted. |
| 4 | Archive metadata and complete historical/audit record. | Enterprise Administrator / D06 | Opportunity becomes read-only. |
| 5 | Remove from active catalogue and publish archival fact. | D06 | LearningOpportunityArchived. |

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
| Learning Opportunity | Archival-eligible Active/Suspended/Closed operational position | Lock and preserve as historical record. | Archived | D06 | L2-D06 W07; Category A transition |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| LearningOpportunityArchived | D06 | Historical catalogue, audit, compliance and reporting consumers | Records completed opportunity archival. | L2-D06 W07; D06-E05 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain hand-off materially participates; completed events may still be consumed under canonical contracts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Reviews academic closure and authorizes archival. | Academic Affairs | Archival authority. | Open Electives & MOOCs |
| Executes governed preservation and retention controls. | Enterprise Administrator | Administrative archival execution. | Open Electives & MOOCs |

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

- The opportunity is removed from the active catalogue and retained as a read-only historical record with complete audit, retention and compliance evidence.
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
