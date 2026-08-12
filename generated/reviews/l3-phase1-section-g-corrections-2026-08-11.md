# L3 Phase 1 — Section G Correction Manifest

**Date:** 2026-08-11  
**Target:** `architecture/L3_Business_Scenario_Architecture/01_Phase1_Discovery/L3_Phase1_Inventory_Varad.md`  
**Scope:** Section G only. Sections F and H untouched.  
**Status:** Applied, uncommitted. Approved by repository owner 2026-08-11.

No candidate was renumbered, merged, split or deleted. No L1 or L2 content was changed.
No canonical ownership was moved. No repository conflict was resolved.

---

## 1. Summary

| Change | Rows | Kind |
|---|---:|---|
| Invalid `Type` value corrected | 109 | schema |
| Malformed 9-column row repaired | 1 | schema |
| Placeholder Domains resolved | 43 | traceability |
| Non-canonical domain tokens normalised | 59 | traceability |
| Candidates added | 2 | coverage |
| Heading structure corrected | 3 | structure |

Candidate count: **539 -> 541**.

---

## 2. Type corrections (109 rows)

All 109 rows carried the Priority value `P1` in the Type column, which
`L3_Phase1_Discovery_Rules.md` §12 does not permit.

**Rule applied:** `{owner}` or `{owner, A}` + configuration -> `Simple`;
safety / medical / security / compliance / archival / audited-correction /
authority-delegation -> `Critical`; all other multi-domain -> `Cross-domain`.

| Candidate | Scenario | Was | Now |
|---|---|---|---|
| CAN-L3-G-160 | Catalogue Update to Effective Resource Information | `P1` | `Cross-domain` |
| CAN-L3-G-168 | Reservation Expiry to Reservation Closure | `P1` | `Cross-domain` |
| CAN-L3-G-176 | Reading Room Opening to Operational Availability | `P1` | `Simple` |
| CAN-L3-G-181 | Damaged Library Resource During Stock Verification to Administrative Review | `P1` | `Cross-domain` |
| CAN-L3-G-184 | Library Notice Creation to Stakeholder Awareness | `P1` | `Cross-domain` |
| CAN-L3-G-186 | Library Overdue Status to Member Notification | `P1` | `Cross-domain` |
| CAN-L3-G-195 | Health Centre Temporary Closure to Restricted Healthcare Availability | `P1` | `Cross-domain` |
| CAN-L3-G-200 | Follow-up Recommendation to Follow-up Healthcare Activity | `P1` | `Cross-domain` |
| CAN-L3-G-209 | Medicine Inventory Threshold or Expiry to Medical Operations Alert | `P1` | `Cross-domain` |
| CAN-L3-G-213 | Referral Follow-up to Updated Healthcare History | `P1` | `Cross-domain` |
| CAN-L3-G-215 | Medical Certificate Review to Certificate Revocation | `P1` | `Critical` |
| CAN-L3-G-218 | Annual Health Checkup to Recorded Assessment Outcome | `P1` | `Cross-domain` |
| CAN-L3-G-219 | Health Screening or Camp Finding to Follow-up Recommendation | `P1` | `Cross-domain` |
| CAN-L3-G-225 | Medical Professional Unavailability to Alternative Operational Assignment | `P1` | `Cross-domain` |
| CAN-L3-G-227 | Medical Configuration Change to Effective Healthcare Policy | `P1` | `Simple` |
| CAN-L3-G-230 | Security Office Temporary Closure to Restricted Security Availability | `P1` | `Cross-domain` |
| CAN-L3-G-232 | Gate Opening to Operational Availability | `P1` | `Simple` |
| CAN-L3-G-233 | Gate Closure to Restricted Access | `P1` | `Simple` |
| CAN-L3-G-243 | Visitor Pass Expiry to Access Restriction | `P1` | `Simple` |
| CAN-L3-G-246 | Visitor Overstay to Operational Follow-up | `P1` | `Cross-domain` |
| CAN-L3-G-257 | Courier Arrival to Verified Delivery Entry | `P1` | `Cross-domain` |
| CAN-L3-G-258 | Courier Delivery Completion to Exit Verification | `P1` | `Simple` |
| CAN-L3-G-260 | Contractor Work Completion to Exit Verification | `P1` | `Cross-domain` |
| CAN-L3-G-267 | Patrol Completion to Patrol Report | `P1` | `Simple` |
| CAN-L3-G-268 | Security Shift Handover to Operational Continuity | `P1` | `Cross-domain` |
| CAN-L3-G-273 | Routine Security Incident to Operational Resolution | `P1` | `Simple` |
| CAN-L3-G-281 | Lost Property Report to Lost Item Registration | `P1` | `Simple` |
| CAN-L3-G-282 | Recovered Property to Found Item Registration | `P1` | `Simple` |
| CAN-L3-G-283 | Lost or Found Ownership Verification to Property Claim | `P1` | `Simple` |
| CAN-L3-G-284 | Verified Property Claim to Handover | `P1` | `Simple` |
| CAN-L3-G-285 | Unclaimed Property to Administrative Disposition | `P1` | `Simple` |
| CAN-L3-G-286 | Escort Request to Escort Assignment | `P1` | `Cross-domain` |
| CAN-L3-G-287 | Escort Assignment to Completed Escort | `P1` | `Cross-domain` |
| CAN-L3-G-288 | Escort Request Exception to Alternative Security Response | `P1` | `Cross-domain` |
| CAN-L3-G-295 | Security Configuration Change to Effective Security Policy | `P1` | `Simple` |
| CAN-L3-G-296 | Visitor Policy Configuration to Controlled Visitor Access | `P1` | `Simple` |
| CAN-L3-G-297 | Patrol Configuration to Effective Patrol Coverage | `P1` | `Simple` |
| CAN-L3-G-298 | Incident Policy Configuration to Effective Escalation | `P1` | `Simple` |
| CAN-L3-G-299 | Security Delegation to Temporary Operational Authority | `P1` | `Critical` |
| CAN-L3-G-308 | Housekeeping Zone Configuration Change to Effective Operational Policy | `P1` | `Cross-domain` |
| CAN-L3-G-309 | Housekeeping Zone Deactivation to Controlled Service Restriction | `P1` | `Cross-domain` |
| CAN-L3-G-314 | Cleaning Assignment Exception to Alternative Housekeeping Assignment | `P1` | `Cross-domain` |
| CAN-L3-G-317 | Deep Cleaning Plan to Completed Deep Cleaning Cycle | `P1` | `Cross-domain` |
| CAN-L3-G-318 | Event Cleaning Plan to Event-Ready Campus Area | `P1` | `Cross-domain` |
| CAN-L3-G-320 | Special Cleaning Requirement to Completed Special Cleaning | `P1` | `Cross-domain` |
| CAN-L3-G-322 | Invalid Cleaning Complaint to Controlled Rejection or Correction | `P1` | `Cross-domain` |
| CAN-L3-G-327 | Repeat Cleaning Complaint to Corrective Resolution | `P1` | `Cross-domain` |
| CAN-L3-G-328 | SLA Warning to Proactive Cleaning Escalation | `P1` | `Cross-domain` |
| CAN-L3-G-335 | Washroom Consumable Shortage to Restocking or Operational Escalation | `P1` | `Cross-domain` |
| CAN-L3-G-340 | Housekeeping Incident Escalation to Controlled Resolution | `P1` | `Cross-domain` |
| CAN-L3-G-343 | Waste Segregation to Verified Collection | `P1` | `Cross-domain` |
| CAN-L3-G-345 | Housekeeping Consumable Availability Monitoring to Operational Alert | `P1` | `Cross-domain` |
| CAN-L3-G-346 | Consumable Shortage to Operational Continuity Decision | `P1` | `Cross-domain` |
| CAN-L3-G-358 | Housekeeping Configuration Change to Effective Service Policy | `P1` | `Simple` |
| CAN-L3-G-359 | Cleaning Policy Change to Future Schedule Behaviour | `P1` | `Simple` |
| CAN-L3-G-360 | Inspection Policy Change to Effective Hygiene Governance | `P1` | `Simple` |
| CAN-L3-G-361 | SLA Policy Change to Effective Service Governance | `P1` | `Simple` |
| CAN-L3-G-362 | Housekeeping Delegation to Temporary Operational Authority | `P1` | `Critical` |
| CAN-L3-G-374 | Student Status Change to Mentorship Relationship Review | `P1` | `Cross-domain` |
| CAN-L3-G-380 | External Context Unavailability to Mentoring Continuation | `P1` | `Cross-domain` |
| CAN-L3-G-385 | Mentoring Discussion to No-Further-Action Closure | `P1` | `Simple` |
| CAN-L3-G-386 | Mentoring Discussion to Parent Interaction Requirement | `P1` | `Cross-domain` |
| CAN-L3-G-388 | Incomplete Parent Interaction to Controlled Recording Rejection | `P1` | `Simple` |
| CAN-L3-G-389 | Parent Interaction to Follow-up Requirement | `P1` | `Cross-domain` |
| CAN-L3-G-398 | Invalid Referral Destination to Controlled Referral Rejection | `P1` | `Simple` |
| CAN-L3-G-400 | Referral Creation to Student Referral Guidance | `P1` | `Cross-domain` |
| CAN-L3-G-405 | Follow-up Review to Additional Guidance | `P1` | `Simple` |
| CAN-L3-G-407 | Follow-up Review to Further Follow-up | `P1` | `Simple` |
| CAN-L3-G-408 | Follow-up Completion to Mentorship Continuation | `P1` | `Simple` |
| CAN-L3-G-412 | Administrative Mentorship Closure to Archived Mentoring History | `P1` | `Critical` |
| CAN-L3-G-414 | Pending Follow-up During Closure to Controlled Closure Resolution | `P1` | `Simple` |
| CAN-L3-G-416 | Parent Interaction Correction to Audited Historical Amendment | `P1` | `Critical` |
| CAN-L3-G-417 | Referral Correction to Audited Historical Amendment | `P1` | `Critical` |
| CAN-L3-G-418 | Follow-up Correction to Audited Historical Amendment | `P1` | `Critical` |
| CAN-L3-G-421 | Student Mentoring Visibility Request to Policy-Controlled Information Access | `P1` | `Cross-domain` |
| CAN-L3-G-422 | Parent Mentoring Information Request to Policy-Controlled Access Decision | `P1` | `Cross-domain` |
| CAN-L3-G-423 | Department Mentoring Access Request to Controlled Departmental Visibility | `P1` | `Simple` |
| CAN-L3-G-424 | Institutional Administration Mentoring Review to Controlled Historical Access | `P1` | `Simple` |
| CAN-L3-G-429 | Follow-up Creation to Mentoring Reminder | `P1` | `Cross-domain` |
| CAN-L3-G-432 | Mentorship History to Student Mentoring Report | `P1` | `Cross-domain` |
| CAN-L3-G-436 | Follow-up Activity to Follow-up Completion Report | `P1` | `Cross-domain` |
| CAN-L3-G-441 | Financial Context to Privacy-Controlled Mentoring View | `P1` | `Cross-domain` |
| CAN-L3-G-442 | Hostel Context to Mentoring Workspace | `P1` | `Cross-domain` |
| CAN-L3-G-443 | Library Context to Mentoring Workspace | `P1` | `Cross-domain` |
| CAN-L3-G-448 | Mentorship Case Closure to Historical Reconstruction Capability | `P1` | `Cross-domain` |
| CAN-L3-G-449 | Mentorship Extension Configuration to New Mentoring Capability | `P1` | `Simple` |
| CAN-L3-G-454 | Physical Campus Attribute Change to Effective-Dated Infrastructure Update | `P1` | `Simple` |
| CAN-L3-G-460 | Building Attribute Change to Effective-Dated Update | `P1` | `Simple` |
| CAN-L3-G-465 | Space Attribute Change to Effective-Dated Update | `P1` | `Simple` |
| CAN-L3-G-466 | Physical Zone Establishment to Infrastructure Grouping | `P1` | `Simple` |
| CAN-L3-G-468 | Invalid Space Classification to Controlled Rejection | `P1` | `Simple` |
| CAN-L3-G-473 | Infrastructure Capacity Change to Effective Capacity Update | `P1` | `Simple` |
| CAN-L3-G-477 | Infrastructure Availability Restoration to Resource Reinstatement | `P1` | `Simple` |
| CAN-L3-G-481 | Infrastructure Reservation Change to Controlled Rescheduling | `P1` | `Simple` |
| CAN-L3-G-482 | Infrastructure Reservation Cancellation to Resource Availability Restoration | `P1` | `Simple` |
| CAN-L3-G-487 | Infrastructure Responsibility Change to Effective Responsibility Transfer | `P1` | `Cross-domain` |
| CAN-L3-G-490 | Administrative Infrastructure Change to Governed Update | `P1` | `Simple` |
| CAN-L3-G-493 | Compliance Status Change to Infrastructure Compliance Update | `P1` | `Critical` |
| CAN-L3-G-504 | Sustainability Requirement to Infrastructure Sustainability Record | `P1` | `Cross-domain` |
| CAN-L3-G-505 | Sustainability Attribute Change to Effective Infrastructure Update | `P1` | `Simple` |
| CAN-L3-G-507 | Infrastructure Security Classification Change to Controlled Update | `P1` | `Cross-domain` |
| CAN-L3-G-516 | Infrastructure Archive Eligibility to Controlled Archival | `P1` | `Critical` |
| CAN-L3-G-517 | Archived Infrastructure Historical Access to Controlled Reconstruction | `P1` | `Cross-domain` |
| CAN-L3-G-518 | Infrastructure Configuration Change to Future Policy Availability | `P1` | `Simple` |
| CAN-L3-G-520 | Infrastructure Configuration Effective Date to Policy Transition | `P1` | `Simple` |
| CAN-L3-G-530 | Infrastructure History to Historical Reconstruction Report | `P1` | `Cross-domain` |
| CAN-L3-G-533 | Infrastructure Availability to Resource Availability Report | `P1` | `Cross-domain` |
| CAN-L3-G-534 | Infrastructure Compliance to Compliance Governance Report | `P1` | `Cross-domain` |
| CAN-L3-G-536 | Infrastructure Reservation to Reservation Utilization Report | `P1` | `Cross-domain` |

---

## 3. Malformed row (1)

`CAN-L3-G-395` carried nine data cells. The stray Type value was removed.

```text
was:  ... | G09 | G09, G06 | Cross-domain | Critical | P0 |   (9 cells)
now:  ... | G09 | G09, G06 | Cross-domain | P0 |              (8 cells)
```
`Cross-domain` retained to match sibling referral rows G-391..G-397; authored `P0` preserved.

---

## 4. Placeholder Domains resolved (43 rows)

Evidence used:

- `L2-G00` §9 Upstream Domains enumerates exactly G01-G10
- `L2-G01` §9 Downstream Consumers: Academic Delivery, Hostel, Library, Laboratory, Security, Examination
- `L2-G01` §16 Service Request Sources: Student, Faculty/Employee, Hostel, Library, Laboratory, Security, Medical Centre
- `L2-G02`/`G03`/`G04`/`G05` §9 Downstream Consumers

| Candidate | Was | Now |
|---|---|---|
| CAN-L3-G-004 | `G00, relevant G domains, B, H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, B, H` |
| CAN-L3-G-006 | `G00, relevant G domains, H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-007 | `G00, relevant G domains, H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-008 | `G00, relevant G domains, H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-009 | `G00, responsible G domain(s), H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-010 | `G00, responsible G domain(s), H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-011 | `G00, affected G domain(s), H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-013 | `G00, relevant G domains, H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-014 | `G00, relevant G domains, H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-015 | `G00, responsible G domain(s)` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10` |
| CAN-L3-G-016 | `G00, relevant G domain(s), H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, H` |
| CAN-L3-G-017 | `G00, relevant G domain(s), A, H` | `G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, A, H` |
| CAN-L3-G-019 | `G00, H, relevant G domains` | `G00, H, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10` |
| CAN-L3-G-020 | `G00, A, relevant participating domains` | `G00, A, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10` |
| CAN-L3-G-021 | `G01, G10, relevant requesting domain, H` | `G01, G10, C, E, G02, G03, G05, G06, G07, H` |
| CAN-L3-G-022 | `G01, G10, relevant affected domain, H` | `G01, G10, D, G02, G03, G05, G07, I, H` |
| CAN-L3-G-023 | `G01, G10, relevant affected domain` | `G01, G10, D, G02, G03, G05, G07, I` |
| CAN-L3-G-024 | `G01, G10, relevant operational domains, E` | `G01, G10, D, G02, G03, G05, G07, I, E` |
| CAN-L3-G-025 | `G01, G10, E, relevant affected domain, H` | `G01, G10, E, D, G02, G03, G05, G07, I, H` |
| CAN-L3-G-026 | `G01, G10, relevant affected domain` | `G01, G10, D, G02, G03, G05, G07, I` |
| CAN-L3-G-032 | `G01, G10, E, relevant affected domain` | `G01, G10, E, D, G02, G03, G05, G07, I` |
| CAN-L3-G-033 | `G01, G10, relevant affected domain` | `G01, G10, D, G02, G03, G05, G07, I` |
| CAN-L3-G-034 | `G01, G10, relevant affected domain` | `G01, G10, D, G02, G03, G05, G07, I` |
| CAN-L3-G-035 | `G01, G10, E, G07, relevant affected domain, H` | `G01, G10, E, G07, D, G02, G03, G05, I, H` |
| CAN-L3-G-044 | `G01, G10, H, relevant affected domain` | `G01, G10, H, D, G02, G03, G05, G07, I` |
| CAN-L3-G-045 | `G01, G10, H, relevant affected domain` | `G01, G10, H, D, G02, G03, G05, G07, I` |
| CAN-L3-G-046 | `G01, G10, E, relevant affected domain, H` | `G01, G10, E, D, G02, G03, G05, G07, I, H` |
| CAN-L3-G-047 | `G01, G10, relevant affected domain` | `G01, G10, D, G02, G03, G05, G07, I` |
| CAN-L3-G-048 | `G01, E, H, relevant affected domain` | `G01, E, H, D, G02, G03, G05, G07, I` |
| CAN-L3-G-049 | `G01, E, H, relevant affected domain` | `G01, E, H, D, G02, G03, G05, G07, I` |
| CAN-L3-G-050 | `G01, E, H, relevant affected domain` | `G01, E, H, D, G02, G03, G05, G07, I` |
| CAN-L3-G-051 | `G01, G10, relevant affected domain` | `G01, G10, D, G02, G03, G05, G07, I` |
| CAN-L3-G-052 | `G01, G10, E, relevant affected domain` | `G01, G10, E, D, G02, G03, G05, G07, I` |
| CAN-L3-G-055 | `G01, G10, relevant affected domain, H` | `G01, G10, D, G02, G03, G05, G07, I, H` |
| CAN-L3-G-056 | `G01, relevant requesting domain` | `G01, C, E, G02, G03, G05, G06, G07` |
| CAN-L3-G-057 | `G01, G10, relevant requesting domain` | `G01, G10, C, E, G02, G03, G05, G06, G07` |
| CAN-L3-G-058 | `G01, G10, E, relevant affected domain` | `G01, G10, E, D, G02, G03, G05, G07, I` |
| CAN-L3-G-059 | `G01, A, participating domains` | `G01, A, D, G02, G03, G05, G07, I` |
| CAN-L3-G-076 | `G02, G10, G01, H, relevant affected domain` | `G02, G10, G01, H, D, I` |
| CAN-L3-G-090 | `G02, A, participating domains` | `G02, A, D, I, H` |
| CAN-L3-G-118 | `G03, A, participating domains` | `G03, A, C, F, G07, H` |
| CAN-L3-G-155 | `G04, A, participating domains` | `G04, A, C, E, F, G07, H` |
| CAN-L3-G-192 | `G05, A, participating domains` | `G05, A, C, E, F, H` |

---

## 5. Domain token normalisation (59 rows, 67 token instances)

`L3_Phase1_Discovery_Rules.md` §11 requires section letters or short names.
Six tokens were neither, and four of them concealed the Section H dependency.

| Token | Instances | Replaced with | Basis |
|---|---:|---|---|
| `Finance` | 25 | `F` | same section already written as `F` on 19 other rows |
| `Security` | 14 | `G07` | same domain already written as `G07` on 100 other rows |
| `Institutional Reporting` | 17 | `H` | reporting is owned by `L2-H08` |
| `Analytics` | 8 | `H` | Section H — Institutional Services & Intelligence |
| `Accreditation` | 2 | `H` | owned by `L2-H09` Accreditation & Compliance |
| `Audit` | 1 | `A` | audit infrastructure is Section A |

Duplicates arising from the mapping were removed, order preserved
(e.g. `G05, Finance, H` + existing `F` -> `G05, F, H`).
Verified: zero occurrences of all six tokens remain in the Domains column.

---

## 6. Candidates added (2)

### CAN-L3-G-540 — Academic Session Readiness Assessment to Session Commencement Decision

| Field | Value |
|---|---|
| Primary Owner | G00 |
| Domains | G00, G01, G02, G03, G04, G05, G06, G07, G08, G09, G10, D, B |
| Type | Critical |
| Priority | P0 |

### CAN-L3-G-541 — Medical Clearance Request to Institutional Clearance Decision

| Field | Value |
|---|---|
| Primary Owner | G06 |
| Domains | G06, C, E, H |
| Type | Critical |
| Priority | P1 |

**G-540 basis:** `L2-G00` §16 Readiness Types enumerates seven readiness assessments.
Six had a candidate (G-001..G-006). Academic Session Readiness had none.
Its academic-calendar trigger and D/B participation make it distinct from G-001.

**G-541 basis:** `L2-G06` §12 Administrative Operations lists four operations.
Three were covered (Medical Certificates -> G-214, Sick Leave Recommendation -> subsumed
by G-214 via §22, Occupational Health -> G-220). Medical Clearance had none.
Structurally parallel to library clearance (G-182) and hostel exit clearance (G-108).

> IDs are deliberately non-contiguous within their subsection blocks.
> Renumbering 519 rows to insert two was rejected as higher risk.
> Phase 1 IDs are temporary per §6 and are not final L3 scenario IDs.

---

## 7. Structure corrections (3)

| Was | Now |
|---|---|
| G09 block had no section heading | `# L3 Phase 1 Inventory — Section G09` added |
| `# G10 L3 Phase 1 Candidate Scenarios` | `# L3 Phase 1 Inventory — Section G10` + `## Infrastructure Management` |
| stray empty `#` heading after G09 notes | removed; `# G10 Coverage Notes` demoted to `##` |

---

## 8. Post-change verification

```text
Candidate rows          541
Column count            8 on every row
Duplicate IDs           none
Type values             Critical 235 | Cross-domain 252 | Simple 54
Priority values         P0 275 | P1 257 | P2 9
Placeholder domains     0
Non-canonical tokens    0
Heading structure       G00-G10 consistent
```

---

## 9. Deliberately NOT changed

| Item | Reason |
|---|---|
| ~122 fragmented candidates (Clusters A-I) | Discovery Rules §17 assigns merging to Phase 2 |
| 6 over-broad candidates | Phase 2 boundary decision |
| Priority distribution (275 P0 = 51%) | Awaiting the P0 definition |
| 235 `Critical` classifications | Awaiting the risk threshold |
| ~20 reporting/dashboard candidates vs `L2-H07` §1.1 | Blocked until Section H is discovered |
| Wing/Block candidate | Confirmed folded into G-458 |
| 8 empty Coverage Notes headings | Author scaffolding, not an architecture defect |
| Sections F and H | Out of the approved scope for this pass |
