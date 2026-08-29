# Malpractice Case Closure and Archival

# 1. Scenario Metadata

**Scenario ID:** `L3-I-171`  
**Architecture Level:** L3 – Business Scenario Architecture  
**Primary Owner:** Malpractice & Examination Committee (I14)  
**Participating Domains:** I (Examination), C (Student Lifecycle), B (Governance), A (Platform Foundation)  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Devang  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §8, §11, §13; `DEPENDENCY_MAP.md`  
**Referenced By:** `L3-I-171`; `L3-I-187`; Future L4 System Behavior Specifications  

---

# 2. Scenario Purpose

Formally closes malpractice case after penalty expiry or restitution, sealing all evidence, hearing transcripts, CCTV footage, and signed orders in permanent legal archive.

---

# 3. Business Outcome

Malpractice case transitioned to `CASE_CLOSED_AND_ARCHIVED`, physical evidence vault sealed, digital case file locked with cryptographic legal seal.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Penalty term served or appeal cycle exhausted with zero pending litigation.

## 4.2 End Boundary

Case dossier permanently archived under 10-year statutory legal retention.

## 4.3 In Scope

- Comprehensive execution of malpractice case closure and archival under statutory University Malpractice Ordinance.
- Adherence to principles of Natural Justice, evidence custody management, and multi-tier committee hearings.
- Immutable disciplinary logging and formal sanction enforcement.

## 4.4 Out of Scope

- Criminal court trials or police penal investigations outside university jurisdiction.
- General campus security gate patrols (governed by G07).

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Predecessor | `L3-I-055` | Malpractice incidents originate during theory/practical exam conduct |
| Core Standard | `L3-I-170` | Central adjudication baseline for all suspected malpractice cases |
| Successor | `L3-I-171` | Terminal case closure and permanent evidence archival |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Malpractice & Examination Committee (I14) | Primary Owning Domain — conducts inquiries, evaluates seized evidence, and determines statutory penalties. |
| Student Lifecycle (Section C) | Updates student conduct record, debarment flags, and examination eligibility holds. |
| Governance & Legal (Section B) | Regulatory compliance, legal review, and appellate board oversight. |
| Platform Foundation (Section A) | Evidence hashing, digital hearing record sealing, and notification dispatch. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Malpractice Committee Secretary | Statutory committee chairing formal hearings and determining findings | Adjudicates / Decides |
| University Legal Officer | Chief executive approving committee penalty recommendations and issuing orders | Approves / Signs |
| Controller of Examinations (CoE) | Primary witness or appellant presenting evidence or petition | Testifies / Petitions |
| University Archivist | Legal or academic authority ensuring procedural compliance | Advises / Audits |

---

# 7. Preconditions and Entry Criteria

- Malpractice incident report or appeal petition formally registered with valid booking number.
- Seized physical evidence (unauthorized notes, devices) sealed in committee vault.
- Statutory committee quorum convened.

---

# 8. Trigger

Operational requirement for malpractice case closure and archival is initiated before the Malpractice Committee.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Intake and registration of case docket for malpractice case closure and archival with verified evidence index | Committee Secretary / Malpractice Domain | Case docket initialized in `PENALTY_SERVED` state. |
| 2 | Notice issued to candidate with specific charges, seized evidence summary, and hearing date | Committee Secretary / Malpractice Domain | Formal show-cause notice served with recorded delivery. |
| 3 | Formal committee hearing conducted: evidence examined, candidate testimony recorded | Malpractice Committee Secretary / Malpractice Committee | Hearing minutes and candidate defense recorded. |
| 4 | Committee deliberation, violation categorization under Malpractice Ordinance, and penalty recommendation | Malpractice Committee Secretary / Malpractice Committee | Adjudication report and penalty recommendation formulated. |
| 5 | Controller of Examinations review, executive approval, and statutory sanction order issuance | University Legal Officer / Examination Domain | Sanction order approved and digitally signed. |
| 6 | State transition committed in Malpractice Master Registry and canonical event published | Malpractice Domain | Case state updated to `CASE_CLOSED_AND_ARCHIVED`; `Malpractice_Case_Archived` emitted. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Alt-1 | Candidate Fails to Appear (Ex-Parte Hearing) | Candidate refuses or fails to appear after two valid show-cause notices | Committee proceeds with ex-parte review based on available physical evidence and invigilator testimony | Malpractice Committee Secretary |
| Alt-2 | Expert Technical / Forensic Analysis | Disputed electronic device or handwriting requires forensic verification | Committee refers seized exhibit to university forensic examiner; hearing adjourned pending report | Malpractice Committee Secretary |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|---|
| Fail-1 | Procedural defect identified in original booking (missing invigilator statement) | Adjudication suspended | Quash defective booking; order formal administrative inquiry against reporting officer | Malpractice Committee Secretary / University Legal Officer |
| Fail-2 | Loss or tampering of physical evidence in custody vault | Case compromised | Declare mistrial; initiate severe disciplinary/criminal probe into evidence custodian | University Legal Officer |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-I14-010: Statutory 10-Year Disciplinary Record Retention Invariant | Malpractice & Examination Committee | Strict compliance required to protect institutional and student rights | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §13 |
| BR-I14-NAT: Principle of Natural Justice Invariant | Malpractice & Examination Committee | No penalty may be awarded without serving written charges and providing fair hearing opportunity | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §13 |
| BR-I14-SEAL: Evidence Tamper-Proof Custody Rule | Malpractice & Examination Committee | Seized materials must remain in dual-key vault custody with signed chain-of-custody ledger | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §8.2 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Malpractice Case Dossier Aggregate | `PENALTY_SERVED` | Adjudicated, approved, and enforced under statutory ordinance | `CASE_CLOSED_AND_ARCHIVED` | Malpractice & Examination Committee | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §8, §11 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

**Published Business Events:**
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `Malpractice_Case_Archived` | Malpractice & Examination Committee | Student Lifecycle (Section C), Result Processing (I09), Administration | Broadcasts authoritative malpractice decision and penalty status | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §6.4 |
| `Student_Disciplinary_Hold_Applied` | Malpractice & Examination Committee | Student Portal, Registration, Transcripts | Imposes examination/graduation hold on penalized candidate | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §6.4 |

**Consumed Business Events:**
| Business Event | Source Domain / Publisher | Consumption Effect | Handling Domain / Role | Authoritative Source |
|---|---|---|---|---|
| `Malpractice_Incident_Reported` | Theory Conduct (I07) / Practical Conduct (I06) | Initiates malpractice case intake | Committee Secretary | `L2-I07_Theory_Examination_Conduct_Workflows.md` |
| `Appeal_Petition_Lodged` | Student Portal (Section C) | Initiates appellate review sequence | Appellate Secretary | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Student Lifecycle (Section C) | Malpractice Committee | ValidateStudentEnrollmentContract | Synchronous validation of candidate enrollment and prior disciplinary history | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §9 |
| Platform Foundation (Section A) | Malpractice Committee | SealEvidenceHashContract | Cryptographic timestamping and hashing of digital evidence files | `L2-I14_Malpractice_and_Examination_Committee_Workflows.md` §9 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Case Booking & Intake | Committee Secretary | Registers case and verifies evidence custody | Malpractice Committee |
| Adjudication & Penalty Finding | Malpractice Committee Secretary | Evaluates evidence and determines penalty category | Malpractice Committee |
| Executive Sanction Approval | University Legal Officer | Highest executive authority approving penalty enforcement | Controller of Examinations |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Reporting Invigilator vs Committee Adjudicator | Reporting Invigilator vs Committee Chair | The invigilator who reported the incident cannot be a member of the adjudicating bench. |
| Original Adjudication vs Appellate Bench | Original Committee vs Standing Appeals Committee | Appeals bench must consist of independent senior faculty not involved in original decision. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Master Case Dossier & Hearing Minutes | Signed hearing transcript with candidate signature and committee votes | Malpractice Committee |
| Seized Physical Evidence Ledger | Itemized evidence receipt with photograph and vault locker barcode | Malpractice Committee |
| Statutory Penalty Order | Digitally signed order signed by Controller of Examinations | Examination Domain |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Malpractice Case Dossier reached `CASE_CLOSED_AND_ARCHIVED` in Malpractice Master Registry.
- Candidate served with formal signed order and appeal guidance.
- `Malpractice_Case_Archived` event emitted and disciplinary hold bound to student master.

## 16.2 Postconditions

- Penalized student record updated across student portal and hall ticket issuance system.
- Result processing division notified to withhold or cancel marks as per penalty.
- Full case dossier archived in permanent legal vault (I16).

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open architectural issues.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Service of Notice Legality | Notice delivered via registered institutional email and postal tracking | Student claims lack of natural justice notice |
| Committee Quorum | Statutory quorum (minimum 3 members including external academic) present | Invalidation of committee proceedings |

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Evidence vault CCTV streaming protocol and bitrate | L4 / L5 | Video storage compression and network streaming is an IT implementation concern |
