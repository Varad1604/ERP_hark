# Hostel Capacity Constraint to Waiting List Placement

**Scenario ID:** L3-G-080  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, H  
**Scenario Type:** Cross-domain  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-G-080 |
| Canonical Title | Hostel Capacity Constraint to Waiting List Placement |
| Primary Owner | Hostel Management |
| Participating Domains | G, C, H |
| Scenario Type | Cross-domain |
| Priority | P0 |
| Status | Draft |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

Place an approved hostel applicant onto the authoritative hostel waiting list when institutional hostel residential capacity is exhausted, establishing a prioritized queue for room allocation without canceling the student's approved admission position.

---

## 3. Business Outcome

The applicant's hostel residency request is recorded in the **Waitlisted** state with an assigned, transparent queue position. The Student Lifecycle and applicant are formally notified of capacity constraints while retaining eligibility for subsequent room vacancies.

---

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary
The scenario begins when an approved hostel application reaches the room allocation stage, and all eligible rooms meeting the applicant's accommodation profile are fully occupied.

### 4.2 End Boundary
The scenario terminates when the applicant is placed on the governed hostel waiting list and the waitlist status is communicated across participating domains.

### 4.3 In Scope
- Identification of residential capacity exhaustion for requested room category.
- Application of institutional allocation priority criteria (e.g., year of study, distance, special needs).
- Assignment of official waiting list rank and state transition to **Waitlisted**.
- Cross-domain notification to Student Lifecycle.

### 4.4 Out of Scope
- Initial hostel admission eligibility verification (owned by L3-G-078).
- Final room check-in and key issuance upon vacancy (owned by L3-G-079).
- Hostel fee refund or adjustment calculations (owned by Finance).

---

## 5. Ownership and Participating Domains

- **Primary Owner:** Hostel Management (Section G) retains sole canonical authority over hostel room capacity, quota management, and waiting list queue ordering.
- **Participating Domains:**
  - *Student Lifecycle (Section C):* Consumes residency waiting status to reflect campus accommodation standing on the student record.
  - *Institutional Services (Section H):* Routes institutional communication and portal notifications to the student.

---

## 6. Business Actors, Roles and Responsibilities

- **Hostel Warden / Allocation Officer:** Evaluates accommodation capacity, applies quota rules, and authorizes waiting list placement.
- **Student Applicant:** Submits approved application and receives waitlist position.
- **Hostel System Coordinator:** Manages operational queue ordering per established governance policies.

---

## 7. Preconditions and Entry Criteria

1. Applicant holds an approved hostel admission decision (state: **Approved**).
2. Hostel capacity analysis indicates zero available rooms in the target residential facility.
3. Institutional waiting list window is active and accepting queue registrations.

---

## 8. Trigger

An authorized allocation run detects zero unoccupied beds for the applicant's category during the room assignment phase.

---

## 9. Main Business Flow

1. Hostel Allocation Officer initiates room assignment for an approved hostel applicant.
2. System evaluates available inventory across relevant hostel blocks and confirms zero vacancies.
3. Allocation Officer applies institutional priority criteria (e.g., academic cohort, domicile distance, quota tier).
4. System calculates the applicant's sequential waiting list rank.
5. Hostel Management records the application state transition to **Waitlisted**.
6. System publishes the `HostelApplicantWaitlisted` business event.
7. Notification is dispatched to the student and Student Lifecycle domain with queue rank and validity period.

---

## 10. Alternate Business Flows

- **Alternative Hostel Block Offer:** If another compatible hostel block has vacancies, the applicant is offered the alternative option prior to waitlist finalization.
- **Special Accommodation Override:** Under Dean approval, exceptional room configurations may be allocated outside standard quota before waitlisting.

---

## 11. Failure, Interruption and Business Recovery

- **Waitlist Capacity Reached:** If maximum waitlist threshold is exceeded, the application is transitioned to **CapacityClosed**, and the student is advised to seek accredited off-campus options.
- **Application Invalidation:** If student context becomes inactive upstream, the waitlist position is automatically rescinded.

---

## 12. Business Rules and Decision Constraints

- **BR-G03-01 (Strict Priority Ordering):** Waiting list order must strictly follow published institutional priority parameters and timestamp of approval.
- **BR-G03-02 (Queue Transparency):** The student must be granted visibility into their active queue position.
- **BR-G03-03 (Vacancy Promotion):** Vacated rooms must be offered to waitlisted candidates strictly in ascending rank order.

---

## 13. State and Lifecycle Impact

- **Hostel Application Lifecycle:** `Approved` → `Waitlisted`.
- **Student Context Impact:** Retains active student status with accommodation flag set to *Pending Waitlist*.

---

## 14. Business Events and Cross-Domain Collaboration

- **Emits:** `HostelApplicantWaitlisted` (Publisher: Hostel Management).
  - *Payload/Data:* Student ID, Application ID, Waitlist Rank, Facility Category, Timestamp.
  - *Consumers:* Student Lifecycle (Section C), Notification Service (Section H).

---

## 15. Business Authorization, Approval and Audit Obligations

- **Authorization Authority:** Hostel Warden / Accommodation Committee.
- **Audit Obligation:** Immutable logging of capacity evaluation, quota checks, and timestamp of queue assignment.

---

## 16. Completion Criteria and Postconditions

- Applicant holds a verified rank on the authoritative hostel waiting list.
- Application status is recorded as **Waitlisted**.
- Business event `HostelApplicantWaitlisted` is successfully published.

---

## 17. Open Issues, Assumptions and Deferred Questions

- **Assumptions:** It is assumed that hostel capacity counters are updated in real-time by check-in and checkout operations.
- **Open Issues:** None.
