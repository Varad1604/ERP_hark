# Authorized Theory Re Conduct Decision to Completed Re Examination

# 1. Scenario Metadata

**Scenario ID:** `L3-I-061`  
**Architecture Level:** L3 – Business Scenario Architecture  
**Primary Owner:** Theory Examination Conduct (I07)  
**Participating Domains:** I (Examination), G (Campus Operations), C (Student Lifecycle), A (Platform Foundation)  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.0  
**Owner:** Devang  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-I07_Theory_Examination_Conduct_Workflows.md` §8, §11, §14; `DEPENDENCY_MAP.md`  
**Referenced By:** `L3-I-070`; `L3-I-187`; Future L4 System Behavior Specifications  

---

# 2. Scenario Purpose

Executes re-conduct of an examination following authorized invalidation (e.g. natural disaster, paper leak), scheduling fresh session and preparing new question paper.

---

# 3. Business Outcome

Re-examination successfully conducted under fresh QP code, attendance reconciled, and new conduct record linked to invalidation decree.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

CoE & Academic Council issue formal Re-Examination Authorization Order.

## 4.2 End Boundary

Re-examination concluded, answer scripts vaulted, results processing initiated.

## 4.3 In Scope

- Comprehensive execution of authorized theory re conduct decision to completed re examination at examination centres under locked university conduct rules.
- Verification of candidate eligibility, custody tracking of question papers/scripts, and observer certification.
- Immutable conduct logging and secure script bundle sealing.

## 4.4 Out of Scope

- Physical security perimeter patrols at main campus gates (governed by G07).
- Post-examination evaluation and marks entry (governed by I08).

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Governed by | `L3-I-001` | Operates under active Examination Policy |
| Predecessor | `L3-I-055` | Standard conduct baseline for all theory sessions |
| Successor | `L3-I-187` | Conduct records feed into Session Closure & Finalization |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Theory Examination Conduct (I07) | Primary Owning Domain — manages hall administration, script custody, attendance muster, and conduct sealing. |
| Campus Operations (G07/G01) | Examination centre perimeter security, physical hall readiness, and emergency logistics. |
| Student Lifecycle (Section C) | Candidate registration verification, hall ticket validity, and photo identity records. |
| Platform Foundation (Section A) | Biometric verification, tamper-evident audit ledger, and event messaging. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Controller of Examinations (CoE) | Executive head of examination centre; oversees conduct, unseals packets, signs muster | Directs / Signs |
| Chief Superintendent | Independent institutional monitor verifying compliance, witnessing seals, certifying records | Verifies / Attests |
| University Observer | Executes candidate verification, muster attendance, distribution, script collection | Performs / Conducts |
| Academic Council | Statutory examination executive overseeing all centres and emergency directives | Approves / Governs |

---

# 7. Preconditions and Entry Criteria

- Examination centre, halls, and seating rosters initialized in `ACTIVE_SCHEDULED` state.
- Chief Superintendent and University Observer appointments verified and active.
- Question paper custody packets in verified secure storage.

---

# 8. Trigger

Operational requirement for authorized theory re conduct decision to completed re examination is initiated at the examination centre.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Examination centre docket initialized and operational parameters verified for authorized theory re conduct decision to completed re examination | Controller of Examinations (CoE) / Centre Administration | Session docket created in `RECONDUCT_AUTHORIZED` state. |
| 2 | Question paper / identity / script prerequisite inspection and dual-witness verification | Controller of Examinations (CoE) & Chief Superintendent | Inspection checklist attested and digitally signed. |
| 3 | Hall execution, invigilation supervision, attendance muster, and candidate processing | University Observer / Hall Invigilator | Candidates processed in strict accordance with L2-I07 rules. |
| 4 | Answer script collection, physical count reconciliation, and discrepancy audit | Controller of Examinations (CoE) & University Observer | Script count verified 100% against attendance muster. |
| 5 | Tamper-evident bundle sealing, observer certification, and formal conduct closure | Controller of Examinations (CoE) & Chief Superintendent | Bundle sealed with tamper-evident barcode; state updated to `RE_EXAMINATION_COMPLETED`. |
| 6 | State transition committed in Examination Master Registry and canonical event published | Theory Examination Conduct | `Theory_Re_Conduct_Completed` event broadcast to CoE command center. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Alt-1 | Authorized Observer Exception Protocol | Minor non-disruptive hall anomaly or late arrival waiver | Chief Superintendent and Observer execute joint exception record with recorded justification | Controller of Examinations (CoE) & Chief Superintendent |
| Alt-2 | Manual Offline Muster Fallback | Centre biometric / barcode scanner offline | Execute physical paper attendance sheet with candidate physical signatures and ink thumbprints | University Observer |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|---|
| Fail-1 | Critical custody breach or packet seal tampering detected | Session suspended; emergency escalation triggered | Impound breached materials; notify CoE immediately; activate emergency reserve protocol | Controller of Examinations (CoE) / Academic Council |
| Fail-2 | Unreconciled answer script count discrepancy post-exam | Bundle sealing halted | Initiate immediate locked-hall physical search; record sworn invigilator statements | Controller of Examinations (CoE) |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-I07-007: Mandatory Fresh Question Paper & Versioning Rule | Theory Examination Conduct | Strict compliance required to maintain examination integrity | `L2-I07_Theory_Examination_Conduct_Workflows.md` §13 |
| BR-I07-CUST: Dual-Witness Tamper-Evident Sealing Invariant | Theory Examination Conduct | Question paper packet opening and answer script bundle sealing must be witnessed and certified by Observer | `L2-I07_Theory_Examination_Conduct_Workflows.md` §13 |
| BR-I07-MUST: 100% Attendance-Script Match Rule | Theory Examination Conduct | The number of collected answer scripts plus marked absentees must exactly equal total registered candidates | `L2-I07_Theory_Examination_Conduct_Workflows.md` §13 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Re-Examination Session Aggregate | `RECONDUCT_AUTHORIZED` | Executed, reconciled, attested, and sealed | `RE_EXAMINATION_COMPLETED` | Theory Examination Conduct | `L2-I07_Theory_Examination_Conduct_Workflows.md` §8, §11 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

**Published Business Events:**
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `Theory_Re_Conduct_Completed` | Theory Examination Conduct | Controller of Examinations, Evaluation (I08), Archival (I16) | Broadcasts completion of examination and readiness of sealed scripts | `L2-I07_Theory_Examination_Conduct_Workflows.md` §6.4 |
| `Examination_Attendance_Finalized` | Theory Examination Conduct | Result Processing (I09), Student Lifecycle (Section C) | Communicates finalized candidate attendance and absentee records | `L2-I07_Theory_Examination_Conduct_Workflows.md` §6.4 |

**Consumed Business Events:**
| Business Event | Source Domain / Publisher | Consumption Effect | Handling Domain / Role | Authoritative Source |
|---|---|---|---|---|
| `Examination_Session_Scheduled` | Session Management (I02) | Authorizes centre opening and conduct execution | Chief Superintendent | `L2-I02_Examination_Session_Management_Workflows.md` |
| `Hall_Ticket_Verified` | Hall Ticket Management (I05) | Confirms candidate entry eligibility | Hall Invigilator | `L2-I05_Hall_Ticket_Management_Workflows.md` |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle (Section C) | Theory Examination Conduct | VerifyCandidateEligibilityContract | Synchronous verification of candidate registration and debarment status | `L2-I07_Theory_Examination_Conduct_Workflows.md` §9 |
| Platform Foundation (Section A) | Theory Examination Conduct | VerifyBiometricIdentityContract | Synchronous candidate photo/biometric authentication at hall entry | `L2-I07_Theory_Examination_Conduct_Workflows.md` §9 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Centre Conduct Intake & Hall Opening | Controller of Examinations (CoE) | Operational command of examination centre | Theory Examination Conduct |
| Compliance & Seal Attestation | Chief Superintendent | Independent verification of exam integrity | Theory Examination Conduct |
| Statutory Directive / Exception Clearance | Academic Council | Highest executive authority for examination conduct | Controller of Examinations |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Centre Superintendent vs Independent Observer | Chief Superintendent vs University Observer | Independent Observer must not belong to the host institution department conducting the exam. |
| Invigilation vs Script Custody Audit | Hall Invigilator vs Chief Superintendent | Invigilator submits collected scripts; Superintendent and Observer perform independent count audit. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Master Conduct Report & Observer Certificate | Signed certificate confirming unsealing, attendance tally, and sealing timestamps | Theory Examination Conduct |
| Attendance & Absentee Roll | Physical signature sheets and digital attendance ledger sealed | Theory Examination Conduct |
| Tamper-Evident Bundle Security Tag | Unique serialized barcode tag attached to sealed script bag | Theory Examination Conduct |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Re-Examination Session reached `RE_EXAMINATION_COMPLETED` in Examination Master Registry.
- Observer compliance certificate digitally and physically sealed.
- Script bundles sealed and ready for secure custody transit.
- `Theory_Re_Conduct_Completed` event emitted and acknowledged.

## 16.2 Postconditions

- Sealed answer scripts ready for handover to Central Evaluation Camp (I08).
- Candidate attendance and absentee records locked for result computation (I09).
- Centre conduct docket archived under permanent exam records (I16).

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open architectural issues.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Observer Independence | Appointed observers have no conflicts of interest with examinees | Integrity challenge to conduct records |
| Secure Custody Courier | Armed/secure transit available immediately post-session | Delay in script transfer to evaluation vault |

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Barcode scanner firmware and optical resolution standards | L4 / L5 | Scanner hardware specification is an implementation detail |
