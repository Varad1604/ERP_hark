# Hostel Residency End to Cleared and Closed Occupancy

**Scenario ID:** L3-G-081  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Hostel Management  
**Participating Domains:** G, C, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2 Domain Workflow Architecture  
**Referenced By:** None yet  

---

## 2. Scenario Purpose

Complete the institutional end of a student's hostel residency by performing the governed exit-clearance and check-out process, closing the active occupancy while preserving the student's canonical identity, the historical residency record, and Finance's ownership of financial obligations.

## 3. Business Outcome

The resident's hostel occupancy is cleared and closed through the governed exit process, the residency period is concluded, and the completed occupancy remains historically traceable.

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

The scenario begins when an active hostel residency is due to end and the resident is required to complete hostel exit processing.

### 4.2 End Boundary

The scenario ends when required exit clearance is completed and the Hostel Occupancy lifecycle reaches its completed check-out position.

### 4.3 In Scope

- Initiation of hostel exit processing for an active resident.
- Exit-clearance verification.
- Required room/damage inspection where applicable.
- Resolution of hostel-owned clearance matters that prevent closure.
- Completion of check-out.
- Closure of the occupancy lifecycle.
- Communication of the completed residency outcome to participating downstream consumers.

### 4.4 Out of Scope

- Student Lifecycle termination, withdrawal, graduation, or archival.
- Financial accounting, settlement, or adjustment decisions.
- Physical infrastructure ownership or room lifecycle changes.
- Maintenance execution.
- Post-residency student services.
- Technical notification delivery, persistence, APIs, or implementation mechanics.

### 4.5 Related Scenarios

- **L3-G-080 — Approved Hostel Admission to Active Residency** establishes active residency before this scenario.
- Subsequent institutional exit or student-lifecycle processes may consume the completed residency fact but are outside this scenario boundary.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Hostel Management (G) | Owns Hostel Occupancy, Exit Clearance, Check-Out, residency closure, and residency history. |
| Student Lifecycle (C) | Owns canonical Student identity and lifecycle; supplies student context but does not own hostel closure. |
| Finance (F) | Consumes hostel occupancy/residency-period information and owns financial obligations and accounting. |
| Notifications (H) | Consumes approved hostel outcomes for governed communication; does not own the residency decision or lifecycle. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Student / Resident | Participates in the hostel exit and completes resident-facing clearance requirements applicable to the institution. | Participates in residency closure. |
| Hostel Administrator | Coordinates hostel exit processing and verifies completion of hostel clearance requirements. | Operational responsibility within Hostel Management. |
| Hostel Warden | Provides hostel governance/approval participation where required by configured exit policy. | Business authority is configurable under Hostel Management. |

No additional approval or authority role is introduced where L2-G03 does not define one.

## 7. Preconditions and Entry Criteria

- A valid Hostel Occupancy exists for the resident.
- The occupancy is in an active residency position from which check-out may proceed.
- The resident is eligible to begin hostel exit processing under applicable institutional policy.
- The canonical student identity/context remains available from Section C.

## 8. Trigger

The resident's hostel residency reaches a governed end condition and hostel exit processing is initiated.

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Hostel Management identifies the active residency that is to end and begins the exit-clearance process. | Hostel Management | Exit processing is initiated for the active occupancy. |
| 2 | Required exit-clearance items are verified, including the applicable hostel administrative checks. | Hostel Administrator / Hostel Management | Exit requirements are assessed. |
| 3 | Where applicable, the room is inspected for damage or other hostel-owned clearance conditions. | Hostel Management | Inspection outcome is established for the residency closure decision. |
| 4 | Any hostel-owned clearance matter that prevents closure is resolved through the applicable hostel business process. | Hostel Management | Required hostel clearance conditions are satisfied or the closure remains pending. |
| 5 | Hostel Management completes the Exit Clearance lifecycle. | Hostel Management | Exit Clearance reaches `Completed`. |
| 6 | Hostel Management completes check-out and closes the resident's Hostel Occupancy. | Hostel Management | The residency reaches the completed `Checked-Out` position and the active occupancy ends. |

**Scenario End:** The main flow stops when exit clearance is completed and the occupancy is closed. Downstream financial consumption and notifications are consequences of the completed business outcome, not additional steps in this scenario.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture beyond the governed clearance variations captured in the failure/recovery section.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required exit-clearance condition remains unresolved. | Occupancy cannot be closed through normal completion. | Keep exit processing open until the applicable hostel-owned clearance requirement is resolved. | Hostel Management |
| Room inspection identifies a hostel-owned damage or clearance matter requiring resolution. | Final check-out cannot yet be completed. | Resolve the applicable hostel matter before completing exit clearance and occupancy closure. | Hostel Management |
| A financial consequence is identified during hostel closure. | Financial obligation or adjustment may require Finance processing, but residency ownership does not change. | Provide the authoritative hostel occupancy/residency information to Finance; Finance determines and owns the financial outcome. | Hostel Management / Finance |

No technical retry, rollback, queue, or infrastructure-recovery behaviour is defined at L3.

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Hostel Management owns Hostel Check-Out and Exit Clearance. | Hostel Management | Residency closure remains within G03. | L2-G03 §§5, 8, 18–24 |
| Exit Clearance includes checkout verification, clearance items, damage inspection, and final residency closure. | Hostel Management | These are the business checks relevant to closure. | L2-G03 §8, §24 |
| Hostel Occupancy owns residency status, occupancy history, current room, and residency timeline. | Hostel Management | Closure changes the occupancy lifecycle without deleting history. | L2-G03 §8 |
| Historical residency information shall never be deleted. | Hostel Management | Completed occupancy remains historically traceable. | L2-G03 §6, §13 |
| Finance owns billing, deposits, ledger, accounting and fee collection. | Finance | Financial consequences do not transfer ownership of residency. | L2-G03 §4 |
| Student Lifecycle owns Student identity and academic status. | Student Lifecycle | Hostel closure does not alter Student Lifecycle state. | L2-G03 §4, L2-C02 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Exit Clearance | Requested / Verification / Inspection | Required exit checks are completed. | Completed | G03 Hostel Management | L2-G03 §11, §24 |
| Hostel Occupancy | Resident / Leaving | Residency is formally concluded through check-out. | Checked-Out | G03 Hostel Management | L2-G03 §11, §18–§19 |
| StudentContext | Current active institutional context | Student identity/context is consumed for resident identification only. | No Student Lifecycle transition | C02 Student Lifecycle | L2-C02 |

The scenario does not introduce a new occupancy state or a new Student Lifecycle state.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

The authoritative G03 document establishes that significant hostel operational transitions publish immutable domain events and that downstream consumers use hostel outcomes. The exact event name for final occupancy closure is not asserted here unless it is explicitly named in the canonical event catalogue; no new event name is invented.

| Business Event / Fact | Canonical Publisher | Consumer / Interested Domain | Purpose |
|---|---|---|---|
| Completed hostel residency / occupancy outcome | G03 Hostel Management | Finance, Notifications and other approved consumers | Communicates the authoritative closure of the residency without transferring ownership. |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle (C) | Hostel Management (G) | Student Identity / student context | Identifies the resident and preserves Student ownership in C. | L2-G03 §9; L2-C02 |
| Hostel Management (G) | Finance (F) | Hostel Occupancy, Residency Period, Exit Clearance information | Enables Finance to consume the residential business fact while retaining financial ownership. | L2-G03 §9 |
| Hostel Management (G) | Notifications (H) | Hostel business outcome / published event | Enables governed communication of the completed residency outcome. | L2-G03 §§9, 27–28; L2-H02 |

Events remain distinct from synchronous business contracts and hand-offs.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Conduct hostel exit processing | Hostel Administrator / Hostel Warden as applicable | Governed by configured hostel operational policy. | Hostel Management |
| Complete Exit Clearance | Hostel Management authority defined by applicable hostel policy | Exact institution-specific approval sequencing is configurable. | Hostel Management |

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule is explicitly defined in the authoritative G03 source for this closure scenario.

No additional override or exception authority is invented.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Residency activities and closure must remain historically auditable. | Immutable hostel residency and operational audit history. | Hostel Management |
| Financial consequences must remain attributable to the underlying hostel business outcome. | Finance-owned financial records linked to the consumed hostel occupancy/residency information. | Finance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- Required hostel exit-clearance checks are complete.
- Exit Clearance reaches `Completed`.
- Hostel Occupancy reaches the completed `Checked-Out` position.
- The resident no longer has an active hostel occupancy under the closed residency.
- Historical residency information remains available for institutional traceability.

### 16.2 Postconditions

- The hostel residency period is closed.
- Finance may consume the completed residency information for its own financial processing.
- Notifications may communicate the completed outcome through the governed notification architecture.
- Student Lifecycle remains unchanged and continues to own student identity and lifecycle.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The supplied authoritative extracts establish the hostel closure lifecycle and downstream consumers, but the exact final occupancy-closure event name is not asserted in the material used for this draft. | A specific event name must not be fabricated. The scenario can preserve the publisher and business fact without naming an unsupported event. | Confirm the exact canonical G03 event name for the final occupancy/check-out closure before Architecture Approval/Lock. |

### 17.2 Assumptions

No assumptions override locked architecture.

### 17.3 Deferred Questions

No scenario-specific deferred L4/L5/L6 question is required.
