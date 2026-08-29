# Security Office Configuration to Operational Activation

# 1. Scenario Metadata

**Scenario ID:** `L3-G-186`  
**Architecture Level:** L3 – Business Scenario Architecture  
**Primary Owner:** Security Operations (G07)  
**Participating Domains:** G (Campus Operations), A (Platform Foundation), E (People & Workforce)  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Devang  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-G07_Security_Operations_Workflows.md` §8.2.1; `DEPENDENCY_MAP.md`  
**Referenced By:** `L3-G-188`; `L3-G-213`; Future L4 System Behavior Specifications  

---

# 2. Scenario Purpose

Governs the administrative and operational setup of a campus Security Office, establishing jurisdiction boundaries, assigning duty posts, configuring communication channels, and executing formal activation for 24/7 campus security governance.

---

# 3. Business Outcome

A fully configured and authoritatively activated Security Office operational unit with committed jurisdiction in the Security Office Registry, assigned supervisory personnel, established communication links, and published activation event.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Receipt of an approved institutional directive or campus expansion order requiring the establishment of a new security office or command post.

## 4.2 End Boundary

Security Office operational state transitioned to `ACTIVE`, post-activation muster completed, and operational readiness broadcast across campus domains.

## 4.3 In Scope

- Institutional parameter definition (jurisdiction, perimeter boundaries, emergency contact channels).
- Assignment of Officer-in-Charge (OIC) and supervisor muster rolls.
- Linking of physical gate checkpoints and surveillance zones.
- Formal activation approval by the Chief Security Officer (CSO).

## 4.4 Out of Scope

- Physical building construction and civil engineering works.
- Procurement of physical security hardware assets (radios, uniforms, barriers).
- Underlying network telecom carrier contracts.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Predecessor | `L3-G-213` | Governed by overarching Campus Security Policy |
| Successor | `L3-G-187` | Governs subsequent temporary closures of this office |
| Successor | `L3-G-188` | Governs configuration of physical gates linked to this office |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Security Operations (G07) | Primary Owning Domain — defines office structure, jurisdiction, post roster, and manages operational lifecycle. |
| Campus Operations (G01) | Facilities verification of physical office space, keys, and emergency power backup. |
| People & Workforce (Section E) | Validates employment, deployment credentials, and statutory security clearance of the appointed Officer-in-Charge. |
| Platform Foundation (Section A) | Manages role-based authorization, operational logging, and tamper-evident event transport. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Chief Security Officer (CSO) | Reviews security configuration, jurisdiction coverage, and issues formal activation authorization | Approves / Authorizes |
| Security Post In-Charge (OIC) | Prepares operational setup, configures muster schedules, and verifies radio check links | Initiates / Configures |
| Estate / Facilities Officer | Attests to physical premises readiness, access keys, and safety compliance | Verifies / Attests |
| Platform Security Administrator | Enrolls security office identifiers and cryptographic operational keys | System Enabler |

---

# 7. Preconditions and Entry Criteria

- Approved campus administrative charter establishing the need for the security office.
- Officer-in-Charge (OIC) nominated with active employee master record in Section E.
- Physical office infrastructure inspected and verified ready for occupancy by Facilities (G01).

---

# 8. Trigger

CSO submits formal authorization to create and activate a new campus Security Office command node.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Intake of security office establishment charter and registration of office metadata (name, physical zone, building coordinates, perimeter extent) | Security Post In-Charge / Security Operations | Security Office entity created in `DRAFT_CONFIGURED` state. |
| 2 | Configuration of operational duty roster, shift timings, emergency dispatch channels, and linked gates | Security Post In-Charge / Security Operations | Operational parameters registered and bound to office profile. |
| 3 | Facilities and safety verification check conducted on physical access, communication backup, and holding cells | Facilities Officer / Campus Operations | Facilities readiness clearance attested and attached to docket. |
| 4 | Personnel credentials, deployment eligibility, and supervisory hierarchy verified | People & Workforce (Section E) | Staff deployment attested with zero compliance deficits. |
| 5 | Formal activation review and operational readiness assessment conducted | Chief Security Officer / Security Operations | Activation docket reviewed; formal operational sign-off granted. |
| 6 | State transition committed in Security Office registry and institutional activation event published | Security Operations | Security Office state transitioned to `ACTIVE`; campus event emitted. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Alt-1 | Provisional Activation under Emergency Expansion | CSO grants temporary provisional activation pending final physical inspection checklist | Office activated under `PROVISIONALLY_ACTIVE` state with mandatory 7-day full audit window | Chief Security Officer |
| Alt-2 | Multi-zone Boundary Revision | Boundary conflicts detected with adjacent command post during setup | Joint boundary conciliation conducted between respective OICs and re-demarcated | Security Operations |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|---|
| Fail-1 | Nominated OIC fails statutory background clearance or certification check | Activation halted; office cannot proceed to active duty status | Reject appointment docket; revert to `CONFIG_REJECTED`; notify HR to nominate alternate certified officer | Security Operations / People |
| Fail-2 | Facilities inspection identifies critical safety failure (power/radio blackout) | Office activation suspended | Place docket in `REMEDIATION_HOLD`; trigger priority Facilities ticket; re-inspect upon rectification | Security Operations / Facilities |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-G07-001: Mandatory Security Command Hierarchy | Security Operations | Every operational security office must have exactly one certified Officer-in-Charge and designated deputy at all times | `L2-G07_Security_Operations_Workflows.md` §8.2.1 |
| BR-G07-002: Perimeter Non-Overlapping Invariant | Security Operations | Physical patrol and response jurisdictions between adjacent security offices must be mutually exclusive and exhaustively mapped | `L2-G07_Security_Operations_Workflows.md` §13 |
| BR-G07-003: Statutory 24/7 Logging Obligation | Security Operations | An active security office must maintain uninterrupted digital station logbook continuity | `L2-G07_Security_Operations_Workflows.md` §6.5 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Security Office Aggregate | `NONE` (New) | Configured, verified, attested, and formally activated | `ACTIVE` | Security Operations | `L2-G07_Security_Operations_Workflows.md` §8.2.1 |
| Security Duty Roster | `UNASSIGNED` | Assigned qualified staff and bound to active office | `ROSTER_ACTIVE` | Security Operations | `L2-G07_Security_Operations_Workflows.md` §8.2.1 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

**Published Business Events:**
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `Security_Office_Activated` | Security Operations | Campus Operations, Administration, Facilities, Emergency Management | Broadcasts activation of new command post and operational readiness | `L2-G07_Security_Operations_Workflows.md` §6.4 |
| `Security_Jurisdiction_Mapped` | Security Operations | Patrol Management, Incident Response, Estate Office | Communicates authoritative perimeter responsibility boundaries | `L2-G07_Security_Operations_Workflows.md` §6.4 |

**Consumed Business Events:**
| Business Event | Source Domain / Publisher | Consumption Effect | Handling Domain / Role | Authoritative Source |
|---|---|---|---|---|
| `Office_Premises_Readiness_Certified` | Facilities (G01) | Provides mandatory physical verification clearance for activation | Security Operations / OIC | `L2-G01_Facilities_Management_Workflows.md` |
| `Security_Staff_Assigned` | People & Workforce (Section E) | Binds certified security personnel to the office duty roster | Security Operations / OIC | `L2-E01_Employee_Master_Workflows.md` |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Facilities (G01) | Security Operations | VerifyPremisesReadinessContract | Synchronous verification of physical office infrastructure and emergency utilities | `L2-G07_Security_Operations_Workflows.md` §9 |
| People (Section E) | Security Operations | ValidateStaffEligibilityContract | Synchronous check of security certifications and active duty standing | `L2-G07_Security_Operations_Workflows.md` §9 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Security Office Configuration Intake | Security Post In-Charge (OIC) | Initiates setup and registers jurisdictional parameters | Security Operations |
| Facilities Readiness Attestation | Estate / Facilities Officer | Signs off on physical safety and utility compliance | Campus Operations (G01) |
| Final Operational Activation Order | Chief Security Officer (CSO) | Statutory executive approval to commission security command post | Security Operations |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Separation of Configuration and Activation | Security Post In-Charge vs Chief Security Officer | The configuring officer cannot approve their own office activation order. |
| Independent Facilities Attestation | Facilities Officer vs Security Operations | Safety and power backup clearance must be provided by independent estate officer. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Command Post Activation Record | Immutable digital activation record signed with CSO digital certificate | Security Operations |
| Facilities Readiness Certificate | Signed checklist including fire extinguishers, communication sets, emergency backup | Security Operations / Facilities |
| Master Duty Log Initiation | Sealed digital station logbook entry recording exact activation timestamp | Security Operations |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Security Office record transitioned to `ACTIVE` state in the Security Office master aggregate.
- All required duty posts, emergency channels, and gate links verified and bound.
- Activation sign-off digitally sealed by CSO.
- `Security_Office_Activated` event published and acknowledged.

## 16.2 Postconditions

- Security office is fully visible as an active operational dispatch node across campus systems.
- Shift supervisors and guards can execute duty muster and log gate passes under this office.
- Incident response workflows can route emergency dispatches to this command post.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open architectural issues.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| 24/7 Power and Network Continuity | Campus infrastructure SLA guarantees high availability for command posts | Failover to emergency radio and manual paper logbooks required |
| Single OIC Accountability | Standard operating procedure requires one accountable officer per post | Shift handover gaps may blur operational liability |

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Radio frequency modulation and digital repeater protocol configuration | L4 / L5 | Low-level telecom infrastructure and hardware tuning is an implementation concern |
