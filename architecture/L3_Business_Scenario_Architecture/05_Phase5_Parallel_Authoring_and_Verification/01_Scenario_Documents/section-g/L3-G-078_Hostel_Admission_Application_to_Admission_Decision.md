# Hostel Admission Application to Admission Decision

**Scenario ID:** L3-G-078  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-22  
**Depends On:** Locked L1 Business Domain Architecture; L2-G03 Hostel Management Workflows  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | `L3-G-078` |
| Canonical Title | Hostel Admission Application to Admission Decision |
| Primary Owner | Hostel Management |
| Participating Domains | G, C, E, H |
| Scenario Type | Cross-domain |
| Priority | P0 |
| Status | Draft |
| Authoritative L2 | `L2-G03` — Hostel Management Workflows |

The final Phase 5 identity is authoritative. Candidate identifiers in earlier canonical-construction stages do not replace the approved final L3 numbering.

## 2. Scenario Purpose

Determine whether a student is granted operational eligibility for hostel residency and conclude the hostel admission request with an authoritative decision, without creating active occupancy or altering the Student Lifecycle.

## 3. Business Outcome

The hostel admission application reaches a governed decision of `Approved`, `Waitlisted`, `Rejected`, or `Withdrawn`, with the corresponding Hostel Admission position recorded and available for the next permitted residency step.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
A hostel admission application is submitted from an approved hostel-admission source.

### 4.2 End Boundary
The Hostel Admission aggregate reaches a governed decision state and the resulting admission fact is available for the next hostel process.

### 4.3 In Scope
- Hostel admission application submission.
- Eligibility validation for hostel residency.
- Business review and decision.
- Approval, waitlisting, rejection, or withdrawal.
- Publication of the canonical hostel admission outcome.

### 4.4 Out of Scope
- Academic admission.
- Student lifecycle state changes.
- Room allocation and occupancy activation.
- Hostel infrastructure ownership.
- Financial accounting.
- Technical authentication, notification delivery, or implementation mechanics.

### 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | `L3-G-079` — Approved Hostel Admission to Active Residency | An approved Hostel Admission creates the business precondition for residency activation. |

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Hostel Management (G) | Owns Hostel Admission, hostel eligibility determination, admission decision, and admission lifecycle. |
| Student Lifecycle (C) | Provides canonical student context; does not own hostel admission. |
| People & Workforce (E) | Provides workforce context for hostel operational roles; does not own hostel admission. |
| Section H | May consume admission outcomes for authorised communication/reporting purposes; does not own admission. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Student | Submits the hostel application where the configured source is student-originated. | Initiates the request. |
| Hostel Administrator | Reviews and governs hostel admission processing. | Business decision authority supported by L2-G03. |
| Hostel Warden | May participate in admission approval under configured hostel authority. | Approval participation supported by L2-G03. |

## 7. Preconditions and Entry Criteria

- Valid student identity/context is available from Student Lifecycle.
- The application concerns institutional hostel residency.
- The admission source is supported by configured Hostel Management policy.
- Hostel Management can evaluate the request.

## 8. Trigger

A hostel admission application is submitted for a student.

## 9. Main Business Flow

1. Register the hostel admission application and establish `Applied`.
2. Consume authoritative student context required for hostel eligibility.
3. Evaluate applicable hostel-admission criteria.
4. Complete the configured review and approval path.
5. Record `Approved`, `Waitlisted`, `Rejected`, or `Withdrawn` as applicable.
6. Publish the applicable canonical hostel-admission event through G03.

## 10. Alternate Business Flows

- Capacity unavailable under policy → `Waitlisted` where permitted.
- Eligibility requirements not satisfied → `Rejected`.
- Applicant withdraws before active residency → `Withdrawn`.

## 11. Failure, Interruption and Business Recovery

If required student context is unavailable, the deficiency is resolved through its owning source before a final admission decision. If the configured approval path is incomplete, Hostel Admission remains in its applicable review position. If capacity is unavailable, the governed waiting-list path applies where permitted.

## 12. Business Rules and Decision Constraints

- Hostel admission governs residency eligibility, not academic admission.
- Eligibility may consider authoritative student status, programme/campus context, hostel category, configured policy, hostel availability, and administrative approval.
- Admission states include `Applied`, `Under Review`, `Approved`, `Waitlisted`, `Rejected`, and `Withdrawn`.
- Hostel Management does not change Student Lifecycle state.
- Infrastructure ownership remains outside Hostel Management.

## 13. State and Lifecycle Impact

| Aggregate | Entry Position | Scenario Impact | Resulting Position | Owner |
|---|---|---|---|---|
| Hostel Admission | `Applied` / `Under Review` | Governed eligibility and decision | `Approved` / `Waitlisted` / `Rejected` / `Withdrawn` | G03 Hostel Management |

No Student Lifecycle transition is owned by this scenario.

## 14. Business Events and Cross-Domain Collaboration

| Business Event | Canonical Publisher | Purpose |
|---|---|---|
| `HostelAdmissionApplied` | G03 Hostel Management | Records application submission. |
| `HostelAdmissionApproved` | G03 Hostel Management | Communicates approved outcome. |
| `HostelAdmissionRejected` | G03 Hostel Management | Communicates rejected outcome. |
| `HostelAdmissionWaitlisted` | G03 Hostel Management | Communicates waiting-list placement. |

Student Lifecycle provides authoritative student context. Downstream consumers consume the G03 outcome without modifying Hostel Admission.

## 15. Business Authorization, Approval and Audit Obligations

Hostel admission follows the configured authority described by L2-G03 and may involve Hostel Administrator and/or Hostel Warden. Admission requests, review, decisions, overrides where permitted, and lifecycle history must remain auditable. No technical access-control design is introduced.

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria
- Hostel Admission application has a governed decision.
- Resulting state is one of the canonical decision positions.
- Applicable G03 event is published.
- Decision history is preserved.

### 16.2 Postconditions
- Approved admissions may proceed to `L3-G-079` under its own boundary.
- Waitlisted admissions remain governed by waiting-list policy.
- Rejected/withdrawn admissions do not create active residency.
- Student Lifecycle remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues
No final-ID, owner, lifecycle, or publisher conflict remains in this scenario after reconciliation with the Phase 5 filename authority and locked L2-G03.

### 17.2 Assumptions
The exact institution-specific approval sequence remains configuration governed under L2-G03.

### 17.3 Deferred Questions
Technical notification delivery and implementation mechanics remain outside L3.
