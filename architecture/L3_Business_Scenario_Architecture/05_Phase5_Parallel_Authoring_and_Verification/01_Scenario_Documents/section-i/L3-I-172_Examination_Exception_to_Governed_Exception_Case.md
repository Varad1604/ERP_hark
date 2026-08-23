# Examination Exception to Governed Exception Case

# 1. Scenario Metadata

**Scenario ID:** `L3-I-172`  
**Architecture Level:** L3 – Business Scenario Architecture  
**Primary Owner:** Examination Exception & Contingency (I15)  
**Participating Domains:** I (Examination), G (Campus Operations), A (Platform)  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.0  
**Owner:** Devang  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-I15_Examination_Exception_and_Contingency_Workflows.md` §8.1, §11.1; `DEPENDENCY_MAP.md`  
**Referenced By:** `L3-I-173`; `L3-I-184`; Future L4 System Behavior Specifications  

---

# 2. Scenario Purpose

Captures, catalogs, and initiates formal governed case handling for unexpected examination disruptions (power blackout, hall flooding, civil protest, tech failure).

---

# 3. Business Outcome

Exception case docket created in `EXCEPTION_REGISTERED` state with initial operational impact parameters and assigned priority triage level.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Chief Superintendent or Central Monitoring Cell reports critical operational disruption during examination session.

## 4.2 End Boundary

Exception case formally registered; central incident command triage activated.

## 4.3 In Scope

- Comprehensive execution of examination exception to governed exception case under locked university governance statutes.
- Operational coordination, multi-tier authority sign-offs, and tamper-evident record logging.
- Institutional event publishing and permanent state registry locking.

## 4.4 Out of Scope

- Physical storage warehouse construction or climate control HVAC engineering.
- Civil court trial proceedings outside university jurisdiction.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Governed by | `L3-I-001` | Operates under master Examination Policy |
| Successor | `L3-I-187` | Integrates into permanent session archival lifecycle |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Examination Exception & Contingency (I15) | Primary Owning Domain — executes operational procedures, manages exceptions/archival, and maintains master registers. |
| Governance & Legal (Section B) | Statutory approvals, legal hold mandates, and regulatory oversight. |
| Platform Foundation (Section A) | Cryptographic vault storage, tamper-evident audit ledger, and event transport. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Central Examination Incident Commander | Operational or statutory lead directing workflow and authorizing decisions | Directs / Decides |
| Centre Chief Superintendent | Executive examination authority reviewing and granting final approval | Approves / Signs |
| Controller of Examinations (CoE) | Institutional specialist executing technical inspections, forensics, or auditing | Verifies / Inspects |
| Campus Operations Lead | External authority or institutional stakeholder collaborating in scenario | Collaborates / Attests |

---

# 7. Preconditions and Entry Criteria

- Preceding examination workflows completed with verified audit records.
- Operating actors authenticated with valid institutional credentials in Section E master.
- Cryptographic archival vault and immutable audit logger operational.

---

# 8. Trigger

Operational requirement for examination exception to governed exception case is triggered in the Examination Management System.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Intake and registration of operational docket for examination exception to governed exception case with verified parameters | Central Examination Incident Commander / Owning Domain | Case docket initialized in `DISRUPTION_REPORTED` state. |
| 2 | Compliance verification, physical evidence/script audit, and regulatory checklist review | Controller of Examinations (CoE) / Operational Lead | Prerequisite inspection verified and attested. |
| 3 | Governed operational execution, physical vaulting/contingency deployment, and parameter validation | Central Examination Incident Commander / Owning Domain | Operational action completed in accordance with L2 rules. |
| 4 | Executive review, dual-signoff attestation, and formal authorization granted | Centre Chief Superintendent / Examination Domain | Authorization approved and digitally signed. |
| 5 | State transition committed in Master Registry and canonical event published | Owning Domain | Entity state transitioned to `EXCEPTION_REGISTERED_AND_TRIAGED`; `Examination_Exception_Logged` emitted. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Alt-1 | Authorized Emergency Variance | Critical unforeseen operational variance or urgent deadline | Lead Executive applies governed emergency exception protocol with recorded justification | Centre Chief Superintendent |
| Alt-2 | Manual Fallback Operation | Digital network or automated scanner offline | Execute physical paper protocol with dual-witness signature; sync to digital ledger upon recovery | Central Examination Incident Commander |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|---|
| Fail-1 | Prerequisite verification failure or statutory non-compliance | Progression suspended; authorization withheld | Issue formal rejection docket; preserve last safe state in audit log; route to corrective queue | Centre Chief Superintendent |
| Fail-2 | Custody breach or physical asset mismatch detected | Scenario halted; emergency escalation triggered | Impound materials; initiate formal security inquiry; notify Controller of Examinations | Central Examination Incident Commander / Centre Chief Superintendent |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-I15-001: Mandatory 15-Minute Critical Incident Reporting Rule | Examination Exception & Contingency | Strict compliance required to maintain examination and institutional integrity | `L2-I15_Examination_Exception_and_Contingency_Workflows.md` / `L2-I16` §13 |
| BR-I-AUTH: Dual Executive Sign-off Rule | Examination Exception & Contingency | Critical decisions, closures, and waivers require independent executive approval | `L2-I15` / `L2-I16` §13 |
| BR-I-AUDIT: Immutable Digital Audit Ledger Rule | Examination Exception & Contingency | All examination exceptions, disposals, and archival actions must be permanently hashed | `L2-I16` §6.5 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Examination Exception Case Aggregate | `DISRUPTION_REPORTED` | Executed, verified, and sealed under governed workflow | `EXCEPTION_REGISTERED_AND_TRIAGED` | Examination Exception & Contingency | `L2-I15` / `L2-I16` §8, §11 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

**Published Business Events:**
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `Examination_Exception_Logged` | Examination Exception & Contingency | Examination Administration, Academic Delivery, Audit, Legal | Broadcasts authoritative operational state change | `L2-I15` / `L2-I16` §6.4 |
| `Examination_Audit_Checkpoint_Sealed` | Examination Exception & Contingency | Internal Audit, University Archives | Confirms sealing of tamper-evident verification checkpoint | `L2-I16` §6.4 |

**Consumed Business Events:**
| Business Event | Source Domain / Publisher | Consumption Effect | Handling Domain / Role | Authoritative Source |
|---|---|---|---|---|
| `Operational_Directive_Issued` | University Leadership / CoE | Initiates scenario entry conditions | Examination Exception & Contingency / Central Examination Incident Commander | `L2-I15` / `L2-I16` §9 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Governance (Section B) | Examination Exception & Contingency | VerifyStatutoryAuthorityContract | Synchronous validation of legal and statutory committee authority | `L2-I15` / `L2-I16` §9 |
| Platform Foundation (Section A) | Examination Exception & Contingency | VerifyVaultCryptographicSealContract | Cryptographic verification of immutable archive hash and vault seals | `L2-I16` §9 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Operational Docket Intake | Central Examination Incident Commander | Registers case docket and initiates preliminary checks | Examination Exception & Contingency |
| Technical Compliance Attestation | Controller of Examinations (CoE) | Verifies physical evidence, checksums, and audit trail | Examination Exception & Contingency |
| Executive Order & Final Approval | Centre Chief Superintendent | Highest executive authority approving operational order | Controller of Examinations |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Operational Execution vs Executive Approval | Central Examination Incident Commander vs Centre Chief Superintendent | The operational officer executing the action cannot grant their own executive approval. |
| Independent Archival Witnessing | Central Examination Incident Commander and Controller of Examinations (CoE) | Unsealing, destruction, and permanent vaulting require independent dual-witness signatures. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Master Order & Resolution Record | Digitally signed order signed by Controller of Examinations | Examination Domain |
| Physical Vault Inspection Voucher | Signed checklist with serialized seal barcode and witness signatures | University Archives |
| Immutable Ledger Checksum | Cryptographic hash sealed in permanent institutional blockchain/ledger | Platform / Examination |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Examination Exception Case successfully reached `EXCEPTION_REGISTERED_AND_TRIAGED` in Examination Master Registry.
- Executive digital sign-off verified and sealed.
- `Examination_Exception_Logged` event emitted and acknowledged by campus message broker.

## 16.2 Postconditions

- Updated operational state is immediately queryable across university systems.
- Downstream academic and administrative dependencies enabled to proceed.
- Institutional examination integrity and statutory compliance certified.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open architectural issues.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Statutory Quorum & Legal Mandate | Executive decisions adhere strictly to university acts and statutes | Legal challenge to examination orders |
| Vault Security & Climate Control | Archival vault meets statutory physical and environmental preservation standards | Physical degradation of historical answer scripts |

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Physical document scanner DPI optical resolution and compression codecs | L4 / L5 | Document digitization hardware specifications are an implementation concern |
