# Course Offering Archive

**Scenario ID:** L3-D-010  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Course & Course Offering  
**Participating Domains:** D  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Architecture Approved  
**Version:** 1.0  
**Owner:** Pallav  
**Last Updated:** 2026-08-20  
**Depends On:** `L2-D01_Course_and_Course_Offering_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0730 / CAN-L3-D-010

---

# 2. Scenario Purpose

Close a completed Course Offering as an immutable historical record after all applicable lifecycle obligations have been resolved.

# 3. Business Outcome

An archival-eligible Completed Course Offering reaches Archived with its identity, versions, lifecycle history and business evidence preserved.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A Course Offering is Completed and an authorized archival requirement confirms that archival preconditions are satisfied.

## 4.2 End Boundary

The offering reaches Archived and `CourseOfferingArchived` is published.

## 4.3 In Scope

- Validate Completed entry state and archival eligibility.
- Confirm no active enrollment, pending assessment or operational dependency prevents archive.
- Apply Completed → Archived and preserve history.

## 4.4 Out of Scope

- Deletion, retention technology or physical disposal.
- Archival of Active, Suspended or Cancelled offerings through an unsupported path.
- Archival of Student, Faculty, Timetable or Assessment records.

## 4.5 Related Scenarios — Optional

No related-scenario cross-reference is required to define this scenario boundary.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Course & Course Offering (D01) | Owns archival eligibility, Completed → Archived transition and historical Course Offering record. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Authorized administrator | Initiates or performs Course Offering archive under policy. | Initiates / performs. |

The Authorized administrator performs archival after eligibility validation; where policy requires approval, the Course Offering Governance Authority provides it before transition.

# 7. Preconditions and Entry Criteria

- The Course Offering exists in Completed.
- No active enrollment, pending assessment or operational dependency prevents archive.
- Historical identity and version lineage are available.

# 8. Trigger

An authorized archival requirement is raised for an archival-eligible Completed Course Offering.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Confirm the Course Offering is Completed. | D01 | The lifecycle entry state is valid. |
| 2 | Validate archival eligibility, including absence of active enrollment, pending assessment and unresolved operational dependencies. | D01 | The offering is eligible or remains Completed. |
| 3 | Authorize and apply archival. | Authorized administrator / D01 | The offering transitions to Archived. |
| 4 | Preserve identity, versions, lifecycle and audit history as immutable historical information. | D01 | Historical integrity is retained. |
| 5 | Publish the completed archival fact. | D01 | `CourseOfferingArchived` records historical closure. |

# 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Offering is not Completed | Archive transition is invalid. | Reject the request and preserve current lifecycle state. | D01 |
| Active enrollment, pending assessment or operational dependency remains | Archival eligibility is not satisfied. | Retain Completed until obligations are resolved. | D01 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Only Completed may transition to Archived. | D01 | Controls archival entry. | L2-D01 §§52,71 |
| Archived offerings are immutable. | D01 | Protects the historical record. | L2-D01 §§12,52 |
| Archive preserves versions, events and audit history. | D01 | Prevents deletion or rewrite. | L2-D01 §§52,55 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Course Offering | Completed | Governed archival. | Archived | D01 | L2-D01 §§52,71 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| CourseOfferingArchived | D01 | Interested D domains | Records historical closure. | L2-D01 §§56,70 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

No synchronous cross-domain business contract or hand-off materially participates in this scenario.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate Course Offering archive | Authorized Course Offering administrator | Offering must be Completed and archival-eligible. | D01 |
| Approve archive where policy requires approval | Course Offering Governance Authority | Approval precedes Completed → Archived. | D01 |
| Apply archival transition | Authorized Course Offering administrator | May act only after eligibility validation and any required approval. | D01 |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty, override or exception authority is established by the authoritative L2 architecture beyond the authorization and policy constraints stated above.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Archival decision | Completed entry state, eligibility evidence, decision and resulting Archived state. | D01 |
| Historical preservation | Identity, version lineage, lifecycle and prior business events retained. | D01 |

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The offering was Completed and archival-eligible.
- The offering is Archived.
- Historical integrity is preserved.
- `CourseOfferingArchived` is published.

## 16.2 Postconditions

- The Archived Course Offering is immutable and unavailable for operational modification.
- Its identity and complete business history remain available as historical evidence.

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
