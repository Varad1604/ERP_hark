# Cross-Campus and Cross-Institution Course Offering Enrollment Governance

**Scenario ID:** L3-D-008  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** B, C, D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0728 / CAN-L3-D-008

---

# 2. Scenario Purpose

Govern whether a Course Offering may be made available for cross-campus or cross-institution registration using authoritative institutional, campus, academic and registration contexts.

# 3. Business Outcome

The Course Offering has a governed cross-campus or cross-institution enrollment-availability decision that preserves Section B structural ownership and Section C registration ownership.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An authorized requirement exists to permit or restrict enrollment in a Course Offering across campus or institution boundaries.

## 4.2 End Boundary

The cross-campus/cross-institution availability decision is governed and made available to Section C; no Student registration is completed.

## 4.3 In Scope

- Validate campus/institution context, capacity, academic policy and required institutional approvals.
- Govern offering-level availability across boundaries.
- Provide the decision to Section C.

## 4.4 Out of Scope

- Creating Campus, Institution, Course, Programme or Registration records.
- Physical infrastructure capacity governance.
- Student-specific registration completion.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns offering-level cross-boundary enrollment availability. |
| Section B | Owns Institution, Campus and academic-structure context supplied to the decision. |
| Section C — Student Academic Registration | Owns Student-specific eligibility and registration transactions. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Home Institution Course Offering Governance Authority | Authorizes participation from the student/home-institution side. | Decides. |
| Offering Institution Course Offering Governance Authority | Authorizes cross-boundary availability of the offering. | Decides. |
| Student Academic Registration authority | Determines the individual registration transaction after dual institutional authorization. | Decides within Section C. |

Cross-institution availability requires both home- and offering-institution authorization; Section C remains sole owner of each individual enrollment transaction.

# 7. Preconditions and Entry Criteria

- The Course Offering exists.
- Authoritative Institution, Campus and academic structures are available.
- Section C registration ownership is preserved.

# 8. Trigger

An authorized requirement is raised to govern cross-campus or cross-institution enrollment availability for a Course Offering.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Obtain authoritative Institution, Campus and academic-structure context. | Section B | The structural basis is authoritative. |
| 2 | Evaluate campus/institution eligibility, capacity and applicable policy. | D01 using Section B context | A governed availability proposal exists. |
| 3 | Obtain home-institution authorization. | Home Institution Course Offering Governance Authority | Home-side participation is approved or rejected. |
| 4 | Obtain offering-institution authorization. | Offering Institution Course Offering Governance Authority | Offering-side availability is approved or rejected. |
| 5 | Record the dual-authorized cross-boundary availability decision. | D01 | The offering-level governance position becomes effective without changing Course Offering lifecycle. |
| 6 | Provide the decision to Student Academic Registration. | D01 / Section C | Section C independently determines each registration transaction. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Cross-campus route | Enrollment remains within one Institution but crosses Campus boundaries. | Apply authoritative campus eligibility and policy. | Governed cross-campus availability decision. | B / D01 |
| Cross-institution route | Enrollment crosses Institution boundaries in a multi-institution context. | Apply authoritative shared-structure, federated-offering and institutional-approval policy. | Governed cross-institution availability decision. | B / D01 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Campus or Institution context is invalid | The decision lacks an authoritative structural basis. | Reject or return the requirement for correction. | D01 / B |
| Required institutional approval is absent | Availability cannot become effective. | Retain the current offering availability position. | D01 |
| Capacity or policy is not satisfied | Cross-boundary enrollment cannot be permitted. | Record the governed rejection/restriction outcome when supported. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Cross-campus behavior is institution-configurable and may validate campus eligibility, capacity and policy. | D01 with Section B context | Controls cross-campus governance. | L2-D01 §38 |
| Cross-institution behavior is configuration-driven and may require shared structures, federated offerings and institutional approvals. | D01 with Section B context | Controls cross-institution governance. | L2-D01 §39 |
| Section C owns academic registration transactions. | Section C | Prevents D01 from completing Student registration. | L2-C03 §§5.6,13.7 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Cross-boundary availability position | Current governed position | Apply authorized cross-campus/cross-institution governance; Course Offering lifecycle state is unchanged. | Permitted or restricted governed position | D01 | L2-D01 §§38–39 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

No separate canonical business event is required for this governed configuration outcome; completion is evidenced by the effective business decision and retained audit record.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Section B | D01 | Authoritative Institution, Campus and academic-structure context | Validate the cross-boundary scope. | L2-B03; L2-B04; L2-D01 §§38–39 |
| D01 | Section C | Governed cross-boundary offering-availability decision | Support Student-specific registration governance. | L2-D01 §§38–39; L2-C03 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Authorize home-institution participation | Home Institution Course Offering Governance Authority | Required for cross-institution participation. | Relevant Section B/D governance |
| Authorize offering-institution availability | Offering Institution Course Offering Governance Authority | Required for cross-boundary offering availability. | D01 |
| Decide individual enrollment | Student Academic Registration authority | Requires effective dual authorization and all student-specific checks. | Section C |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Cross-boundary decision | Authoritative structural context, applicable policy, approval where required, and final availability position. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The authoritative campus/institution context was evaluated.
- Any required approval was obtained.
- A governed offering-level availability decision exists for Section C.

## 16.2 Postconditions

- Section B retains Institution/Campus/academic-structure ownership.
- D01 owns offering availability.
- Section C owns Student-specific registration.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**Draft Validation Verdict:** PASS  
**Validation Basis:** `Section_D_Canonical_Register.md` and `L2-D01_Course_and_Course_Offering_Workflows.md`; Template v2.0; Architecture Level Validation Standards; accepted Section D decisions 001–500; accepted Batch 1 resolution decisions 1–7 (2026-08-20).
