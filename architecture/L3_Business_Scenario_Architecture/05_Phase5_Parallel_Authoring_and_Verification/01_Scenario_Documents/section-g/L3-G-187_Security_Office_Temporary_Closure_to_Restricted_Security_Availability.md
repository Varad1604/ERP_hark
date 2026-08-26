# Security Office Temporary Closure to Restricted Security Availability

# 1. Scenario Metadata

**Scenario ID:** `L3-G-187`  
**Architecture Level:** L3 – Business Scenario Architecture  
**Primary Owner:** Security Operations (G07)  
**Participating Domains:** G (Campus Operations), A (Platform Foundation)  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Review  
**Version:** 1.0  
**Owner:** Devang  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-G07_Security_Operations_Workflows.md` §8.2.1; `L3-G-186`  
**Referenced By:** `L3-G-186`; `L3-G-199`; Future L4 System Behavior Specifications  

---

# 2. Scenario Purpose

Governs the temporary operational closure, emergency evacuation, or scheduled maintenance shutdown of a campus Security Office, ensuring secure handover of jurisdiction to an adjacent command post without security coverage gaps.

---

# 3. Business Outcome

Security Office transitioned to `TEMPORARILY_CLOSED` state, jurisdiction temporarily transferred to a designated secondary command post, station logbook sealed, and campus security network notified.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Initiation of a planned maintenance closure request or emergency evacuation order affecting the security post.

## 4.2 End Boundary

Perimeter jurisdiction successfully bound to the relief office, station logbook archived, and closure state sealed.

## 4.3 In Scope

- Scheduled maintenance, deep sanitization, or emergency closure orders.
- Automated diversion of dispatch channels to designated adjacent command post.
- Custody transfer of physical keys, armory/radio lockers, and active incident dossiers.
- Authorization by Chief Security Officer or on-duty Shift Superintendent.

## 4.4 Out of Scope

- Permanent decommissioning or legal dissolution of a security post.
- Physical building repairs or renovation contracts.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Predecessor | `L3-G-186` | Office must be in `ACTIVE` state prior to temporary closure |
| Successor | `L3-G-186` | Re-opening executes activation sequence back to `ACTIVE` |
| Concurrent | `L3-G-199` | Executes shift/custody handover during closure execution |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Security Operations (G07) | Primary Owning Domain — executes closure protocols, re-routes patrol routes, and transfers command jurisdiction. |
| Campus Operations (G01) | Secures premises, keys, and manages maintenance entry permits. |
| Platform Foundation (Section A) | Re-routes automated system alerts, dispatch queues, and access event streams to the relief post. |

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Chief Security Officer (CSO) | Approves planned temporary closure and assigns temporary jurisdiction relief post | Authorizes / Signs |
| Officer-in-Charge (Closing Office) | Executes lockdown checklist, secures physical custody assets, and hands over logbook | Performs / Submits |
| Relief Post Officer-in-Charge | Acknowledges receipt of diverted jurisdiction, radio channels, and active incident files | Accepts / Acknowledges |
| Shift Security Supervisor | Validates physical perimeter seal and verifies emergency diversion channels | Verifies / Inspects |

---

# 7. Preconditions and Entry Criteria

- Security Office is in `ACTIVE` operational state.
- Approved closure justification (scheduled refurbishment, fumigation, or emergency hazard).
- Designated relief security office is currently `ACTIVE` with adequate supervisory capacity.

---

# 8. Trigger

Approved temporary closure order logged in the security operations management system.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Submission of temporary closure request detailing closure reason, expected duration, and designated relief office | Officer-in-Charge / Security Operations | Closure docket created in `CLOSURE_PENDING` state. |
| 2 | Operational impact review and confirmation of relief office coverage capacity | Chief Security Officer / Security Operations | Closure approved; jurisdiction re-routing plan validated. |
| 3 | Active incidents, open gate passes, and emergency call lines transferred to relief office | Officer-in-Charge & Relief OIC | Active queues successfully reassigned; mutual handover signed. |
| 4 | Physical custody items (keys, emergency kits, weapon lockers) sealed in high-security lockbox or transferred | Shift Supervisor / Security Operations | Custody transfer voucher digitally signed by both OICs. |
| 5 | Station logbook formally closed with timestamped summary entry and cryptographic seal | Officer-in-Charge / Security Operations | Active station log archived with checksum. |
| 6 | State transition committed in registry and `Security_Office_Temporarily_Closed` event published | Security Operations | Office state updated to `TEMPORARILY_CLOSED`; emergency dispatch map updated. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Alt-1 | Emergency Evacuation Closure | Immediate hazard (fire, gas leak, structural danger) forces instant abandonment | Fast-track closure executed; physical handover deferred to remote muster station; immediate automated dispatch failover | Security Operations |
| Alt-2 | Closure Duration Extension | Maintenance work exceeds estimated timeframe | OIC submits formal extension request 12 hours prior to scheduled reopening; CSO approves extended relief | Chief Security Officer |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|---|
| Fail-1 | Designated relief office lacks staffing to absorb additional jurisdiction | Closure rejected or suspended | CSO reassigns jurisdiction to central campus security headquarters; deploys mobile patrol unit | Security Operations |
| Fail-2 | Unaccounted key or physical custody asset identified during pre-closure audit | Closure blocked | Initiate immediate lost-asset incident (`L3-G-200`); execute physical search before sealing office | Security Operations |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-G07-004: Zero Campus Coverage Gap Rule | Security Operations | A security office cannot be closed without an active, validated secondary post accepting full jurisdictional liability | `L2-G07_Security_Operations_Workflows.md` §13 |
| BR-G07-005: Maximum Temporary Closure Period | Security Operations | Temporary closures cannot exceed 30 consecutive days without formal executive re-authorization | `L2-G07_Security_Operations_Workflows.md` §8.2.1 |
| BR-G07-006: Dual-Signoff Custody Transfer | Security Operations | Transfer of keys, active passes, and station logs requires independent digital sign-offs from both outgoing and incoming OICs | `L2-G07_Security_Operations_Workflows.md` §13 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Security Office Aggregate | `ACTIVE` | Operations halted, queues diverted, station log sealed | `TEMPORARILY_CLOSED` | Security Operations | `L2-G07_Security_Operations_Workflows.md` §8.2.1 |
| Jurisdiction Assignment Aggregate | `DIRECT_LOCAL` | Re-bound to relief command post | `DIVERTED_TO_RELIEF` | Security Operations | `L2-G07_Security_Operations_Workflows.md` §8.2.1 |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

**Published Business Events:**
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `Security_Office_Temporarily_Closed` | Security Operations | Campus Operations, Administration, Patrol Management, Transport | Informs campus domains of localized office closure and active relief contact | `L2-G07_Security_Operations_Workflows.md` §6.4 |
| `Security_Jurisdiction_Diverted` | Security Operations | Emergency Response, Incident Intake, Gate Checkpoints | Directs incident tickets and patrol telemetry to relief command post | `L2-G07_Security_Operations_Workflows.md` §6.4 |

**Consumed Business Events:**
| Business Event | Source Domain / Publisher | Consumption Effect | Handling Domain / Role | Authoritative Source |
|---|---|---|---|---|
| `Relief_Jurisdiction_Accepted` | Security Operations (Relief Office) | Confirms relief office readiness to assume operational load | Outgoing OIC / Security Operations | `L2-G07_Security_Operations_Workflows.md` §8.2.1 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Relief Security Office | Outgoing Security Office | TransferJurisdictionContract | Synchronous validation of active incident queues, radio channels, and pass registers | `L2-G07_Security_Operations_Workflows.md` §9 |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Temporary Closure Intake | Officer-in-Charge | Initiates closure docket with operational justification | Security Operations |
| Temporary Closure Approval | Chief Security Officer | Executive authority to authorize command post shutdown | Security Operations |
| Relief Jurisdiction Attestation | Relief Post OIC | Formally accepts operational liability for the closed zone | Security Operations |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Outgoing vs Incoming Custody Attestation | Closing OIC vs Relief OIC | Must be two distinct certified officers; self-handover is strictly forbidden. |
| Closure Authorization vs Execution | Chief Security Officer vs Closing OIC | Executive approval required prior to physical lockdown. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Closure Authorization Order | Signed CSO decree specifying closure window, justification, and relief post | Security Operations |
| Physical Asset & Key Handover Voucher | Itemized custody transfer record signed by both outgoing and incoming OICs | Security Operations |
| Final Station Logbook Checksum | Tamper-evident digital hash of the closed office station logbook | Security Operations |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Office aggregate transitioned to `TEMPORARILY_CLOSED` state.
- All active incident dockets and dispatch queues reassigned to relief office.
- Dual-signoff handover completed and verified.
- `Security_Office_Temporarily_Closed` broadcast published.

## 16.2 Postconditions

- Dispatch system routes all 911/emergency and operational alerts for this zone to relief post.
- Physical access to the closed office is restricted to authorized maintenance personnel.
- Status dashboard reflects reduced operational footprint.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open architectural issues.

## 17.2 Assumptions

| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| Relief Office Infrastructure Redundancy | Relief command post has adequate radio channels and workstations | Operational bottleneck during peak incident surges |
| Scheduled Maintenance Timeliness | Maintenance contractors adhere to declared schedule | Extended closures may require successive CSO waivers |

## 17.3 Deferred Questions

| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| Automated physical door lock latching protocols | L4 / L5 | Low-level IoT lock actuation mechanisms are deferred to technical solution architecture |
