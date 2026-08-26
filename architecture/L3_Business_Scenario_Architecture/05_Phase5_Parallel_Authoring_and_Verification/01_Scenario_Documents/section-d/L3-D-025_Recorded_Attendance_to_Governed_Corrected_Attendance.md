# Recorded Attendance to Governed Corrected Attendance

# 1. Scenario Metadata

**Scenario ID:** L3-D-025  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Teaching Session & Attendance  
**Participating Domains:** C, D  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D04_Teaching_Session_and_Attendance_Workflows.md`  
**Referenced By:** None yet  
**Frozen Stage 3 Source:** P2W-D-0745 / CAN-L3-D-025

---

# 2. Scenario Purpose

Correct finalized attendance through a governed, justified and auditable business process without deleting or rewriting the original attendance fact.

# 3. Business Outcome

An approved attendance correction is recorded and re-finalized as the current official attendance truth, while the original finalized attendance and correction history remain immutable and traceable.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A justified correction request identifies finalized attendance requiring change.

## 4.2 End Boundary

The correction is approved, applied and re-finalized and `AttendanceCorrected` is published; a rejected request retains the existing finalized attendance truth.

## 4.3 In Scope

- Submit a correction request with reason and evidence.
- Validate correction eligibility, policy, evidence and business authority.
- Review and approve or reject the proposed correction.
- Record an approved correction as new governed history and re-finalize attendance.

## 4.4 Out of Scope

- Direct editing of finalized attendance.
- Deletion or replacement of original attendance history.
- Modification of Section C registration facts.
- Technical permissions, RBAC codes, retries or transaction recovery.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-D-024 — Teaching Session Attendance Capture to Attendance Finalization | Provides the finalized attendance truth that may later require governed correction. |

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| D04 | Owns finalized attendance, correction request, correction decision, corrected attendance fact and history. |
| Section C | Provides authoritative registration/eligibility references only and retains ownership of them. |

# 6. Business Actors, Roles and Responsibilities

The reviewed L2-D04 source establishes governed attendance-correction actions but does not establish one complete canonical institutional role mapping for requester, approver and re-finalization authority that can safely be named here without invention.

| Business Actor / Role | Responsibility | Business Authority |
|---|---|---|
| Applicable D04 attendance-correction business authority | Requests, reviews, decides or applies correction only according to the role assignment established by authoritative institutional attendance policy. | Exact canonical institutional role mapping remains unresolved; no technical permission code is defined at L3. |

# 7. Preconditions and Entry Criteria

- Attendance is Finalized.
- The correction request is supported by a reason and evidence.
- The governing D04 correction policy permits the correction path.
- Original attendance and supporting evidence remain available.

# 8. Trigger

An error, authorized exception or evidence-backed discrepancy is identified in finalized attendance.

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create a correction request linked to the finalized attendance record with reason and evidence. | Applicable D04 business authority / D04 | `AttendanceCorrectionRequested`. |
| 2 | Validate correction eligibility, evidence, student reference and applicable business authority. | D04 | Request may proceed to decision or remains blocked/rejected. |
| 3 | Review and approve or reject the proposed correction with an explicit reason. | Applicable D04 decision authority | `AttendanceCorrectionApproved` or `AttendanceCorrectionRejected`. |
| 4 | For an approved request, record the corrected attendance as new governed history without altering the original finalized fact. | D04 | Corrected attendance history is established. |
| 5 | Re-finalize the corrected attendance under the applicable D04 authority. | D04 under applicable business authority | Current official attendance truth is established and `AttendanceCorrected` is published. |

# 10. Alternate Business Flows

## 10.1 Correction Rejected

Where the correction request is validly reviewed but not approved, the existing finalized attendance remains the official truth and `AttendanceCorrectionRejected` records the governed decision. This is a legitimate terminal business outcome of the correction request; no corrected attendance is established.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Owner |
|---|---|---|---|
| Correction eligibility, evidence or authoritative reference cannot be validated | The correction cannot proceed to approval. | Resolve the missing/invalid business evidence or reference before a new valid decision is attempted. | D04 with source-domain participation where applicable |
| Required business authority is unavailable or cannot be validated | The requested correction decision cannot be made. | Retain the existing finalized attendance and obtain a valid authority assignment through institutional governance. | D04 |
| Correction activity is interrupted before re-finalization | The existing finalized attendance remains authoritative until a corrected result is validly completed. | Preserve the last valid correction-business position and continue only when the blocking condition is resolved. | D04 |

# 12. Business Rules and Decision Constraints

| Rule | Owner | Scenario Effect | Source |
|---|---|---|---|
| Only D04 may change D04-owned attendance facts. | D04 | Prevents cross-domain ownership leakage. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |
| A finalized attendance fact is not edited in place; correction follows the governed correction lifecycle. | D04 | Preserves original attendance history. | L2-D04 §26 |
| Correction requires valid authority, reason and evidence. | D04 | Prevents unauthorized or unexplained correction. | L2-D04 correction workflow |
| Published and historical attendance facts remain traceable. | D04 | Preserves auditability and lineage. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 13. State and Lifecycle Impact

| Concept | Entry State | Scenario Impact | Result | Owner | Source |
|---|---|---|---|---|---|
| Attendance | Finalized | Govern request, review, decision, correction and re-finalization. | Finalized → Correction Requested → Review → Approved/Rejected → Attendance Updated → Re-Finalized | D04 | L2-D04 §26 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumers | Purpose | Source |
|---|---|---|---|---|
| `AttendanceCorrectionRequested` / `AttendanceCorrectionApproved` / `AttendanceCorrectionRejected` / `AttendanceCorrected` | D04 | Section C, D05 and authorized consumers | Records the governed attendance-correction lifecycle and corrected fact. | L2-D04 §46.2 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| From | To | Business Hand-off | Purpose | Source |
|---|---|---|---|---|
| Section C | D04 | Authoritative registration/eligibility references where correction validation requires them. | Enable correction validation without transferring Section C ownership. | `L2-D04_Teaching_Session_and_Attendance_Workflows.md` |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

- Attendance correction may be requested, decided and applied only by the business authorities established by the governing D04 attendance policy.
- This scenario does not translate L2 command names into L3 permission codes.
- The exact canonical institutional role mapping for correction request, approval and re-finalization remains an upstream governance question recorded in Section 17.

## 15.2 Separation of Duties, Overrides and Exceptions

Where institutional policy separates correction initiation, decision and re-finalization, those authorities remain distinct. Any override requires business authority, justification, evidence and retained decision history.

## 15.3 Business Audit and Evidence Obligations

The institution must retain the original finalized attendance, correction request, reason/evidence, authority, decision, corrected attendance history, re-finalized outcome and applicable published events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An approved correction is recorded as new governed history and re-finalized as the current attendance truth; or a governed rejection retains the prior finalized truth.
- Original finalized attendance remains historically immutable.
- Applicable D04 correction events are published by D04.

## 16.2 Postconditions

- D04 remains the canonical owner of attendance and correction history.
- Section C retains ownership of registration/eligibility facts.
- No technical authorization or implementation mechanism is defined at L3.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-D04 defines attendance-correction actions but the reviewed source does not establish one complete canonical institutional business-role mapping for correction requester, correction approver and re-finalization authority. | L3 cannot safely retain invented generic role labels or technical permission names. | Confirm the canonical business-role/authority mapping in authoritative upstream architecture or governance before Architecture Approval. |

## 17.2 Assumptions

No assumption overrides locked architecture.

## 17.3 Deferred Questions

Technical RBAC, permission codes, transaction handling, retries, persistence and infrastructure recovery are deferred to later architecture levels.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** Section D canonical register; `L2-D04_Teaching_Session_and_Attendance_Workflows.md`; `L3_Business_Scenario_Template_v2.0.md`; `Architecture_Level_Validation_Standards.md`.
