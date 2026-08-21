# Course Offering Enrollment Window Governance

**Scenario ID:** L3-D-014  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** C, D  
**Scenario Type:** Simple  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0734 / CAN-L3-D-014

---

# 2. Scenario Purpose

Govern effective-dated Course Offering enrollment windows and communicate opening and closing facts while Section C retains enrollment-transaction ownership.

# 3. Business Outcome

A valid enrollment window opens and closes according to governed timing and policy, with Section C consuming the authoritative availability facts.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized enrollment-window definition exists for an eligible Course Offering.

## 4.2 End Boundary

The window reaches Closed after its governed open period and exact opening/closing events are published.

## 4.3 In Scope

- Configure window type and effective period.
- Validate lifecycle, timing and policy.
- Open, extend, suspend, resume or close enrollment as authorized.

## 4.4 Out of Scope

- Student-specific registration decisions.
- Eligibility, restriction or payment evaluation owned elsewhere.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D01 | Owns enrollment-window definition and availability state. |
| Section C | Owns Student Academic Registration transactions. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized enrollment administrator | Configures and administers enrollment windows. | Initiates / performs. |
| Course Offering Governance Authority | Approves exceptions or material window changes where required. | Decides. |

# 7. Preconditions and Entry Criteria

- Course Offering is eligible for enrollment scheduling.
- Window dates and policy are valid.

# 8. Trigger

The window is configured or its effective opening time arrives.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate offering lifecycle, window type, dates and policy. | D01 | Window is accepted or rejected. |
| 2 | Record the scheduled window. | D01 | Enrollment availability is Scheduled. |
| 3 | Open enrollment at the effective time or authorized action. | D01 | `EnrollmentWindowOpened` is published. |
| 4 | Apply governed extension, suspension or resumption where authorized. | D01 | Availability remains policy-compliant. |
| 5 | Close enrollment at the effective time or authorized action. | D01 | `EnrollmentWindowClosed` is published. |
| 6 | Consume availability facts for registrations. | Section C | Transactions remain Section C-owned. |

# 10. Alternate Business Flows

The scenario follows only the source-supported alternate outcomes stated in the main flow; rejection or correction does not create the successful final outcome.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Recovery | Owner |
|---|---|---|---|
| Validation or reference failure | Scenario cannot proceed. | Reject or return for correction without changing the authoritative prior state. | Course & Course Offering |
| Authorization or approval failure | Requested outcome is not permitted. | Retain prior state and record the decision reason. | Course & Course Offering |
| Interrupted processing | Completion is uncertain. | Resume from the last authoritative state with audit continuity. | Course & Course Offering |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only the canonical owner may change the owned business state. | Course & Course Offering | Prevents cross-domain ownership leakage. | `L2-D01_Course_and_Course_Offering_Workflows.md` |
| Every transition or effective decision requires valid authority and business validation. | Course & Course Offering | Prevents bypass of governance. | `L2-D01_Course_and_Course_Offering_Workflows.md` |
| Historical business facts remain immutable. | Course & Course Offering | Preserves auditability and lineage. | `L2-D01_Course_and_Course_Offering_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Enrollment availability | Closed / Scheduled | Schedule, open and close governed window. | Scheduled → Open → Closed | D01 | L2-D01 §§31–32,41 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| EnrollmentWindowOpened / EnrollmentWindowClosed | D01 | Section C | Records authoritative availability changes. | L2-D01 §§42,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| C | Course & Course Offering | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D01_Course_and_Course_Offering_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Configures and administers enrollment windows. | Authorized enrollment administrator | Initiates / performs. | Course & Course Offering |
| Approves exceptions or material window changes where required. | Course Offering Governance Authority | Decides. | Course & Course Offering |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Course & Course Offering |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Course & Course Offering |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A valid enrollment window opens and closes according to governed timing and policy, with Section C consuming the authoritative availability facts.
- Required audit evidence is retained.
- Applicable completed business facts are published.

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

No deferred questions beyond recorded open issues.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** Section D canonical register; `L2-D01_Course_and_Course_Offering_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500.
