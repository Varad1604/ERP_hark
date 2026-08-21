# Governed Template-Based Course Offering Generation

**Scenario ID:** L3-D-012  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** B, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0732 / CAN-L3-D-012

---

# 2. Scenario Purpose

Generate independent Draft Course Offerings from an approved reusable template without bypassing authoritative academic references or offering approval governance.

# 3. Business Outcome

Valid independent Draft Course Offerings are generated from an approved template with generation evidence and retained template provenance.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An approved active template and valid generation requirement exist for an authoritative academic period and course context.

## 4.2 End Boundary

Eligible Draft Course Offerings exist and `CourseOfferingGenerationCompleted` records the governed generation outcome.

## 4.3 In Scope

- Validate template, Course, Academic Period and institutional context.
- Generate one or more independent Draft Course Offerings.
- Record per-offering validation outcomes and provenance.

## 4.4 Out of Scope

- Treating the template as an operational offering.
- Automatic publication or activation.
- Ownership of Section B academic masters.

## 4.5 Related Scenarios — Optional

No additional scenario reference is required to define this boundary.

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D01 | Owns templates, generated Course Offerings, validation outcomes and generation evidence. |
| Section B | Owns Course, curriculum, academic calendar and structural references. |

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority |
|---|---|---|
| Authorized Course Offering administrator | Initiates template-based generation. | Initiates. |
| Course Offering Governance Authority | Approves templates and any governed generation policy. | Decides. |

# 7. Preconditions and Entry Criteria

- Template is approved and active.
- Course and Academic Period are authoritative and eligible.
- Required configuration is complete.

# 8. Trigger

A manual, bulk or scheduled generation requirement becomes due.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Select approved template and target academic scope. | Authorized administrator | Generation basis identified. |
| 2 | Validate template, Course, Academic Period, duplicates and policy. | D01 using Section B references | Eligible records identified. |
| 3 | Generate each eligible Course Offering as an independent Draft. | D01 | Draft aggregates exist without publication bypass. |
| 4 | Record failures independently unless atomic policy applies. | D01 | Detailed governed outcome retained. |
| 5 | Publish generation completion. | D01 | `CourseOfferingGenerationCompleted` is available. |

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
| Course Offering | No generated offering | Create independent offering from approved template. | Draft | D01 | L2-D01 §§57–60 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| CourseOfferingGenerationCompleted | D01 | Section B and interested D domains | Records generation outcome. | L2-D01 §§60,68,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| B | Course & Course Offering | Authoritative reference facts or completed business facts | Preserve ownership while enabling the scenario. | `L2-D01_Course_and_Course_Offering_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Action / Decision | Authorized Role | Constraint | Owner |
|---|---|---|---|
| Initiates template-based generation. | Authorized Course Offering administrator | Initiates. | Course & Course Offering |
| Approves templates and any governed generation policy. | Course Offering Governance Authority | Decides. | Course & Course Offering |

## 15.2 Separation of Duties, Overrides and Exceptions

Initiation, validation and approval remain distinct wherever the source or configured governance requires separation; an automated actor never makes a governance decision.

## 15.3 Business Audit and Evidence Obligations

| Obligation | Evidence | Owner |
|---|---|---|
| Scenario decision and lifecycle | Actor, reason, prior position, validation evidence, decision and resulting position. | Course & Course Offering |
| Cross-domain collaboration | Consumed references and published facts with ownership preserved. | Course & Course Offering |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Valid independent Draft Course Offerings are generated from an approved template with generation evidence and retained template provenance.
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
