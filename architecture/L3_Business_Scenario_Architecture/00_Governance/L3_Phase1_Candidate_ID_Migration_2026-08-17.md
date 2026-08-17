# L3 Phase 1 Candidate ID Migration

**Architecture Level:** L3 — Business Scenario Architecture
**Document Type:** Migration crosswalk and authorization record
**Date:** 2026-08-17
**Status:** Applied

---

## 1. Migration Purpose

Every active L3 Phase 1 discovery section (A–J) carried a candidate numbering range that started at an arbitrary offset rather than at `001` — for example `CAN-L3-G-542`…`CAN-L3-G-863`, `CAN-L3-I-315`…`CAN-L3-I-511`, and `CAN-L3-B-0997`…`CAN-L3-B-1028`. This migration renumbers every active candidate in every section to a clean, continuous range starting at `001`, while preserving complete old-ID → new-ID traceability so no historical reference becomes unresolvable.

## 2. Authorization

Authorized as a controlled, one-time, repository-wide identifier normalization. This is an identifier/governance/traceability remediation only. No business scenario was added, removed, merged, split, or reclassified. No L0/L1/L2 business architecture, canonical ownership, event, lifecycle state, aggregate, or dependency was changed.

## 3. The Old Numbering Problem

Section G and Section I had each already undergone a prior, narrower renumbering earlier the same day (2026-08-17), which deliberately started each section's continuous range one past its own historical maximum (`G-542`, `I-315`) specifically to avoid reusing retired IDs. That approach is documented in `L3_Section_G_Phase2_Consolidation_Watchlist.md` §14 and `L3_Section_I_Phase2_Consolidation_Watchlist.md` ("ID Renumbering — 2026-08-17"). The remaining sections (A, B, C, D, E, F, H, J) had never been renumbered at all and simply carried whatever range resulted from incremental Phase 1 discovery and consolidation activity — the C and F ranges happened to start near `001`; the others did not.

The result was a repository where every section used a different, non-`001` starting point, and where several governance documents contained cross-references that had been written against an even earlier numbering generation and were never updated when their target candidate was later renumbered (for example `CAN-L3-E-103`, cited by `RC-01`, had itself been renumbered to `CAN-L3-E-119` by an earlier Section E split and the citation was never corrected).

This migration supersedes the G/I disjoint-start approach: both sections are renumbered again, this time to `001`-start, consistent with every other section.

## 4. New Numbering Rule

```text
CAN-L3-<AREA>-<NUMBER>

<NUMBER> starts at 001 within each discovery area (A-J, X).
<NUMBER> is continuous within each area's active inventory -- no gaps.
<NUMBER> uses a minimum of three digits and is unbounded upward
  (001 ... 999 ... 1000 ... 1001 ...).
```

See `L3_Naming_Numbering_Standard.md` §4 and §7 for the authoritative rule text, and `check_l3_inventory.py` for its automated enforcement.

## 5. Pre-Migration Counts (verified directly from the repository, 2026-08-17)

| Section | Count | Pre-Migration Range |
|---|---:|---|
| A | 62 | `CAN-L3-A-070` … `CAN-L3-A-131` |
| B | 32 | `CAN-L3-B-997` … `CAN-L3-B-1028` |
| C | 74 | `CAN-L3-C-001` … `CAN-L3-C-074` |
| D | 57 | `CAN-L3-D-058` … `CAN-L3-D-114` |
| E | 75 | `CAN-L3-E-105` … `CAN-L3-E-179` |
| F | 85 | `CAN-L3-F-001` … `CAN-L3-F-085` |
| G | 322 | `CAN-L3-G-542` … `CAN-L3-G-863` |
| H | 81 | `CAN-L3-H-086` … `CAN-L3-H-166` |
| I | 197 | `CAN-L3-I-315` … `CAN-L3-I-511` |
| J | 110 | `CAN-L3-J-682` … `CAN-L3-J-791` |
| **TOTAL** | **1095** | |

These counts were independently re-derived from the live inventory files rather than assumed, and match the expected baseline (A 62, B 32, C 74, D 57, E 75, F 85, G 322, H 81, I 197, J 110 = 1,095).

## 6. Post-Migration Counts

| Section | Count | Post-Migration Range |
|---|---:|---|
| A | 62 | `CAN-L3-A-001` … `CAN-L3-A-062` |
| B | 32 | `CAN-L3-B-001` … `CAN-L3-B-032` |
| C | 74 | `CAN-L3-C-001` … `CAN-L3-C-074` |
| D | 57 | `CAN-L3-D-001` … `CAN-L3-D-057` |
| E | 75 | `CAN-L3-E-001` … `CAN-L3-E-075` |
| F | 85 | `CAN-L3-F-001` … `CAN-L3-F-085` |
| G | 322 | `CAN-L3-G-001` … `CAN-L3-G-322` |
| H | 81 | `CAN-L3-H-001` … `CAN-L3-H-081` |
| I | 197 | `CAN-L3-I-001` … `CAN-L3-I-197` |
| J | 110 | `CAN-L3-J-001` … `CAN-L3-J-110` |
| **TOTAL** | **1095** | |

Counts are unchanged by this migration — only the numbering changed.

## 7. Business Architecture Statement

**No business architecture changed.** This migration touched only the `Candidate ID` field of each row and literal `CAN-L3-` ID tokens in prose cross-references. Scenario Title, Trigger, Final Outcome, Primary Owner, Domains, Type, and Priority were verified byte-for-byte identical for all 1,095 active candidates before and after migration (see the companion final report, section "Validation Results"). No L1/L2 ownership, lifecycle state, aggregate, business rule, or event was touched.

## 8. Old → New Candidate ID Mapping

Complete for all 1,095 active Phase 1 candidates, grouped by section, in original row order (migration order = repository row order, per the authorized migration scope; no candidate was reordered).

### Section A — Platform Foundation (62 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Dhruv.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-A-070` | `CAN-L3-A-001` | Tenant Provisioning to Active Tenant Availability | ACTIVE |
| `CAN-L3-A-071` | `CAN-L3-A-002` | Active Tenant Governance Concern to Controlled Restriction and Restored Availability | ACTIVE |
| `CAN-L3-A-072` | `CAN-L3-A-003` | Active Tenant to Governed Archival | ACTIVE |
| `CAN-L3-A-073` | `CAN-L3-A-004` | Person Establishment to Active Canonical Identity | ACTIVE |
| `CAN-L3-A-074` | `CAN-L3-A-005` | Duplicate Person Identification to Canonical Identity Reconciliation | ACTIVE |
| `CAN-L3-A-075` | `CAN-L3-A-006` | Erroneous Person Consolidation to Restored Identity Separation | ACTIVE |
| `CAN-L3-A-076` | `CAN-L3-A-007` | Eligible Identity to Authenticated Session Availability | ACTIVE |
| `CAN-L3-A-077` | `CAN-L3-A-008` | Lost or Invalid Credential to Restored Authentication Capability | ACTIVE |
| `CAN-L3-A-078` | `CAN-L3-A-009` | Multi-Factor Enrollment to Active Verified Authentication Factor | ACTIVE |
| `CAN-L3-A-079` | `CAN-L3-A-010` | Institutional Responsibility Assignment to Effective Authorized Access | ACTIVE |
| `CAN-L3-A-080` | `CAN-L3-A-011` | Responsibility Delegation to Automatic Authority Reversion | ACTIVE |
| `CAN-L3-A-081` | `CAN-L3-A-012` | Temporary Access Request to Governed Access Expiry | ACTIVE |
| `CAN-L3-A-082` | `CAN-L3-A-013` | Emergency Access Request to Audited Emergency Access Closure | ACTIVE |
| `CAN-L3-A-083` | `CAN-L3-A-014` | Workflow Definition to Activated Orchestration Availability | ACTIVE |
| `CAN-L3-A-084` | `CAN-L3-A-015` | Workflow Initiation to Completed Orchestrated Execution | ACTIVE |
| `CAN-L3-A-085` | `CAN-L3-A-016` | Workflow Execution Failure to Controlled Recovery Outcome | ACTIVE |
| `CAN-L3-A-086` | `CAN-L3-A-017` | Workflow Change to Governed New Version Availability | ACTIVE |
| `CAN-L3-A-087` | `CAN-L3-A-018` | Event Definition Proposal to Active Canonical Event Contract | ACTIVE |
| `CAN-L3-A-088` | `CAN-L3-A-019` | Committed Business Fact to Reliable Consumer Delivery | ACTIVE |
| `CAN-L3-A-089` | `CAN-L3-A-020` | Failed Event Delivery to Controlled Recovery Outcome | ACTIVE |
| `CAN-L3-A-090` | `CAN-L3-A-021` | Historical Event Replay Request to Controlled Redelivery Completion | ACTIVE |
| `CAN-L3-A-091` | `CAN-L3-A-022` | Canonical Business Event to Immutable Audit Evidence Availability | ACTIVE |
| `CAN-L3-A-092` | `CAN-L3-A-023` | Investigation Request to Authorized Historical Evidence Outcome | ACTIVE |
| `CAN-L3-A-093` | `CAN-L3-A-024` | Audit Evidence Verification Request to Integrity Verification Outcome | ACTIVE |
| `CAN-L3-A-094` | `CAN-L3-A-025` | Audit Evidence Retention Review to Governed Retention Decision | ACTIVE |
| `CAN-L3-A-095` | `CAN-L3-A-026` | Retention Decision to Governed Audit Evidence Archival | ACTIVE |
| `CAN-L3-A-096` | `CAN-L3-A-027` | Archived Audit Evidence Request to Authorized Restored Access | ACTIVE |
| `CAN-L3-A-097` | `CAN-L3-A-028` | Module Proposal to Active Governed Enterprise Capability | ACTIVE |
| `CAN-L3-A-098` | `CAN-L3-A-029` | Module Architectural Change to Governed Active Successor Version | ACTIVE |
| `CAN-L3-A-099` | `CAN-L3-A-030` | Active Module Governance Concern to Controlled Suspension and Restored Availability | ACTIVE |
| `CAN-L3-A-100` | `CAN-L3-A-031` | Active Module to Governed Retirement | ACTIVE |
| `CAN-L3-A-101` | `CAN-L3-A-032` | Module Dependency Change to Validated Architectural Dependency Outcome | ACTIVE |
| `CAN-L3-A-102` | `CAN-L3-A-033` | Business Dataset Onboarding to Governed Dataset Registration | ACTIVE |
| `CAN-L3-A-103` | `CAN-L3-A-034` | Governed Dataset Creation to Published Enterprise Availability | ACTIVE |
| `CAN-L3-A-104` | `CAN-L3-A-035` | Published Dataset Change, Quality Deficiency, or Historical Restoration to Governed Successor Version Availability | ACTIVE |
| `CAN-L3-A-105` | `CAN-L3-A-036` | Authorized Bulk Data Source to Governed Imported Dataset Outcome | ACTIVE |
| `CAN-L3-A-106` | `CAN-L3-A-037` | Governed Dataset Export Request to Authorized Data Delivery | ACTIVE |
| `CAN-L3-A-107` | `CAN-L3-A-038` | Published Dataset Change to Synchronized Consumer Availability | ACTIVE |
| `CAN-L3-A-108` | `CAN-L3-A-039` | Dataset Localization Requirement to Published Localized Availability | ACTIVE |
| `CAN-L3-A-109` | `CAN-L3-A-040` | Published Dataset to Governed Retirement and Historical Preservation | ACTIVE |
| `CAN-L3-A-110` | `CAN-L3-A-041` | Retired Dataset to Governed Archival | ACTIVE |
| `CAN-L3-A-111` | `CAN-L3-A-042` | Operational Scope Governance Assessment to Certification Eligibility Decision | ACTIVE |
| `CAN-L3-A-112` | `CAN-L3-A-043` | Readiness Evidence Deficiency to Completed Evidence Verification Outcome | ACTIVE |
| `CAN-L3-A-113` | `CAN-L3-A-044` | Readiness Requirement Failure to Governed Improvement Plan | ACTIVE |
| `CAN-L3-A-114` | `CAN-L3-A-045` | Remediation Completion to Verified Readiness Finding Closure | ACTIVE |
| `CAN-L3-A-115` | `CAN-L3-A-046` | Approved Certification Eligibility to Operational Readiness Certificate Availability | ACTIVE |
| `CAN-L3-A-116` | `CAN-L3-A-047` | Active Readiness Certificate Suspension Condition to Controlled Suspension and Restoration | ACTIVE |
| `CAN-L3-A-117` | `CAN-L3-A-048` | Governance-Significant Change to Readiness Impact Decision | ACTIVE |
| `CAN-L3-A-118` | `CAN-L3-A-049` | Completed Readiness Governance Cycle to Final Governance Closure | ACTIVE |
| `CAN-L3-A-119` | `CAN-L3-A-050` | Workspace Definition to Governed Resolvable Experience Context | ACTIVE |
| `CAN-L3-A-120` | `CAN-L3-A-051` | Conflicting Responsibility Risk to Enforced Separation of Duties | ACTIVE |
| `CAN-L3-A-121` | `CAN-L3-A-052` | Authorization Policy Change to Governed Active Policy Version | ACTIVE |
| `CAN-L3-A-122` | `CAN-L3-A-053` | Person Institutional Exit to Governed Identity Closure | ACTIVE |
| `CAN-L3-A-123` | `CAN-L3-A-054` | Existing Institutional Population to Governed Canonical Identity Onboarding | ACTIVE |
| `CAN-L3-A-124` | `CAN-L3-A-055` | Institutional Authentication Provider Onboarding to Governed Provider Availability | ACTIVE |
| `CAN-L3-A-125` | `CAN-L3-A-056` | Platform and Tenant Configuration Requirement to Resolved Governed Configuration | ACTIVE |
| `CAN-L3-A-126` | `CAN-L3-A-057` | Controlled Platform Change to Restored Tenant Operational Availability | ACTIVE |
| `CAN-L3-A-127` | `CAN-L3-A-058` | Human Decision Requirement to Completed or Escalated Task Resolution | ACTIVE |
| `CAN-L3-A-128` | `CAN-L3-A-059` | Consumer Subscription Request to Authorized Active Event Subscription | ACTIVE |
| `CAN-L3-A-129` | `CAN-L3-A-060` | Active Event Contract Change to Governed Compatible Version Transition | ACTIVE |
| `CAN-L3-A-130` | `CAN-L3-A-061` | Active Readiness Certificate to Governed Supersession by Successor Certification | ACTIVE |
| `CAN-L3-A-131` | `CAN-L3-A-062` | Active Readiness Certificate Revocation Decision to Terminal Historical Closure | ACTIVE |

### Section B — Institution & Structure (32 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Pallav.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-B-0997` | `CAN-L3-B-001` | Institution Establishment to Final Activation Decision | ACTIVE |
| `CAN-L3-B-0998` | `CAN-L3-B-002` | Institution Suspension and Operational Restoration | ACTIVE |
| `CAN-L3-B-0999` | `CAN-L3-B-003` | Institution Archival | ACTIVE |
| `CAN-L3-B-1000` | `CAN-L3-B-004` | Institution Profile Revision | ACTIVE |
| `CAN-L3-B-1001` | `CAN-L3-B-005` | Institutional Policy Revision | ACTIVE |
| `CAN-L3-B-1002` | `CAN-L3-B-006` | Organisation Unit Registration | ACTIVE |
| `CAN-L3-B-1003` | `CAN-L3-B-007` | Organisation Unit Activation to Operational Availability | ACTIVE |
| `CAN-L3-B-1004` | `CAN-L3-B-008` | Organisation Unit Governed Modification | ACTIVE |
| `CAN-L3-B-1005` | `CAN-L3-B-009` | Approved Organisation Structure Transformation | ACTIVE |
| `CAN-L3-B-1006` | `CAN-L3-B-010` | Organisation Type Migration to Effective Blueprint | ACTIVE |
| `CAN-L3-B-1007` | `CAN-L3-B-011` | Organisation Governance Configuration Lifecycle | ACTIVE |
| `CAN-L3-B-1008` | `CAN-L3-B-012` | Organisation Unit Decommission and Historical Archive | ACTIVE |
| `CAN-L3-B-1009` | `CAN-L3-B-013` | Position Definition Establishment and Activation | ACTIVE |
| `CAN-L3-B-1010` | `CAN-L3-B-014` | Position Definition Effective-Dated Change | ACTIVE |
| `CAN-L3-B-1011` | `CAN-L3-B-015` | Position Definition Retirement and Archive | ACTIVE |
| `CAN-L3-B-1012` | `CAN-L3-B-016` | Organisation Suspension and Operational Restoration | ACTIVE |
| `CAN-L3-B-1013` | `CAN-L3-B-017` | Institutional Campus Registration | ACTIVE |
| `CAN-L3-B-1014` | `CAN-L3-B-018` | Institutional Campus Activation Decision | ACTIVE |
| `CAN-L3-B-1015` | `CAN-L3-B-019` | Institutional Campus Governed Modification | ACTIVE |
| `CAN-L3-B-1016` | `CAN-L3-B-020` | Institutional Campus Suspension and Operational Restoration | ACTIVE |
| `CAN-L3-B-1017` | `CAN-L3-B-021` | Institutional Campus Decommission and Archive | ACTIVE |
| `CAN-L3-B-1018` | `CAN-L3-B-022` | Academic Structure and Curriculum Establishment to Published Availability | ACTIVE |
| `CAN-L3-B-1019` | `CAN-L3-B-023` | Curriculum Revision to Immutable Successor Programme Version | ACTIVE |
| `CAN-L3-B-1020` | `CAN-L3-B-024` | Academic Structure or Curriculum Retirement | ACTIVE |
| `CAN-L3-B-1021` | `CAN-L3-B-025` | School Academic Model Establishment to Published Availability | ACTIVE |
| `CAN-L3-B-1022` | `CAN-L3-B-026` | Academic Calendar Establishment to Open Academic Context | ACTIVE |
| `CAN-L3-B-1023` | `CAN-L3-B-027` | Calendar Assignment to Effective Governing Scope | ACTIVE |
| `CAN-L3-B-1024` | `CAN-L3-B-028` | Academic Calendar Revision to Successor Version | ACTIVE |
| `CAN-L3-B-1025` | `CAN-L3-B-029` | Calendar Exception to Completed Temporary Deviation | ACTIVE |
| `CAN-L3-B-1026` | `CAN-L3-B-030` | Academic Year Closure to Governed Next-Cycle Foundation | ACTIVE |
| `CAN-L3-B-1027` | `CAN-L3-B-031` | Calendar Change Simulation to Governed Impact Decision | ACTIVE |
| `CAN-L3-B-1028` | `CAN-L3-B-032` | Academic Calendar Retirement and Historical Preservation | ACTIVE |

### Section C — Student Lifecycle (74 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Chetan.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-C-001` | `CAN-L3-C-001` | Approved Candidate to Active Student | ACTIVE |
| `CAN-L3-C-002` | `CAN-L3-C-002` | Admission Cancellation | ACTIVE |
| `CAN-L3-C-003` | `CAN-L3-C-003` | Admission Withdrawal | ACTIVE |
| `CAN-L3-C-004` | `CAN-L3-C-004` | Admission Revocation | ACTIVE |
| `CAN-L3-C-005` | `CAN-L3-C-005` | Student Readmission | ACTIVE |
| `CAN-L3-C-006` | `CAN-L3-C-006` | Admission Offer Expiry | ACTIVE |
| `CAN-L3-C-072` | `CAN-L3-C-007` | Admission Document Verification and Approval Decision | ACTIVE |
| `CAN-L3-C-007` | `CAN-L3-C-008` | Student Enrollment Update | ACTIVE |
| `CAN-L3-C-008` | `CAN-L3-C-009` | Student Academic Assignment Change | ACTIVE |
| `CAN-L3-C-009` | `CAN-L3-C-010` | Student Suspension | ACTIVE |
| `CAN-L3-C-010` | `CAN-L3-C-011` | Student Restoration After Suspension | ACTIVE |
| `CAN-L3-C-011` | `CAN-L3-C-012` | Intra-Institute Student Academic Reassignment | ACTIVE |
| `CAN-L3-C-074` | `CAN-L3-C-013` | Inter-Institute Student Transfer with Student Context Re-establishment | ACTIVE |
| `CAN-L3-C-012` | `CAN-L3-C-014` | Student Profile Update | ACTIVE |
| `CAN-L3-C-013` | `CAN-L3-C-015` | Student Communication Preference Update | ACTIVE |
| `CAN-L3-C-014` | `CAN-L3-C-016` | Student Identity Correction | ACTIVE |
| `CAN-L3-C-015` | `CAN-L3-C-017` | Duplicate Student Lifecycle Resolution | ACTIVE |
| `CAN-L3-C-016` | `CAN-L3-C-018` | Student Document or Certificate Request | ACTIVE |
| `CAN-L3-C-017` | `CAN-L3-C-019` | Student Context Archival and Permanent Record Closure | ACTIVE |
| `CAN-L3-C-018` | `CAN-L3-C-020` | Academic Registration Window Opening and Closure | ACTIVE |
| `CAN-L3-C-019` | `CAN-L3-C-021` | Student Academic Registration | ACTIVE |
| `CAN-L3-C-020` | `CAN-L3-C-022` | Academic Registration Modification and Add/Drop | ACTIVE |
| `CAN-L3-C-021` | `CAN-L3-C-023` | Academic Registration Cancellation | ACTIVE |
| `CAN-L3-C-022` | `CAN-L3-C-024` | Backlog and Repeat Course Registration | ACTIVE |
| `CAN-L3-C-023` | `CAN-L3-C-025` | Academic Registration Finalization and Publication | ACTIVE |
| `CAN-L3-C-073` | `CAN-L3-C-026` | Registration Hold Placement and Release | ACTIVE |
| `CAN-L3-C-024` | `CAN-L3-C-027` | Student Academic Progression Decision | ACTIVE |
| `CAN-L3-C-025` | `CAN-L3-C-028` | Academic Credit Ledger Update | ACTIVE |
| `CAN-L3-C-026` | `CAN-L3-C-029` | Conditional Promotion Condition Fulfilment | ACTIVE |
| `CAN-L3-C-027` | `CAN-L3-C-030` | Academic Probation Management and Resolution | ACTIVE |
| `CAN-L3-C-028` | `CAN-L3-C-031` | Academic Detention and Repeat Determination | ACTIVE |
| `CAN-L3-C-029` | `CAN-L3-C-032` | Student Credit Transfer and Recognition | ACTIVE |
| `CAN-L3-C-030` | `CAN-L3-C-033` | Student Programme Migration | ACTIVE |
| `CAN-L3-C-031` | `CAN-L3-C-034` | Student Academic Break | ACTIVE |
| `CAN-L3-C-032` | `CAN-L3-C-035` | Student Rejoining After Academic Break | ACTIVE |
| `CAN-L3-C-033` | `CAN-L3-C-036` | Qualification Eligibility Determination | ACTIVE |
| `CAN-L3-C-034` | `CAN-L3-C-037` | Academic Credit Ledger to ABC Synchronisation | ACTIVE |
| `CAN-L3-C-035` | `CAN-L3-C-038` | Progression Decision Revision After Re-evaluation or Appeal | ACTIVE |
| `CAN-L3-C-036` | `CAN-L3-C-039` | Qualification Completion and Degree Conferment | ACTIVE |
| `CAN-L3-C-037` | `CAN-L3-C-040` | Academic Record Finalization and Student Exit Completion | ACTIVE |
| `CAN-L3-C-038` | `CAN-L3-C-041` | NEP/MEME Intermediate Qualification Exit | ACTIVE |
| `CAN-L3-C-039` | `CAN-L3-C-042` | Student Withdrawal Exit | ACTIVE |
| `CAN-L3-C-040` | `CAN-L3-C-043` | Student Transfer Exit | ACTIVE |
| `CAN-L3-C-041` | `CAN-L3-C-044` | Student Discontinuation Exit | ACTIVE |
| `CAN-L3-C-042` | `CAN-L3-C-045` | Student Termination Exit | ACTIVE |
| `CAN-L3-C-043` | `CAN-L3-C-046` | Institutional Exit Clearance Completion | ACTIVE |
| `CAN-L3-C-044` | `CAN-L3-C-047` | Administrative Exit Closure | ACTIVE |
| `CAN-L3-C-045` | `CAN-L3-C-048` | Deceased Student Academic Record Closure | ACTIVE |
| `CAN-L3-C-046` | `CAN-L3-C-049` | Qualification Completion Deferral and Remediation | ACTIVE |
| `CAN-L3-C-047` | `CAN-L3-C-050` | Conferred Qualification Correction or Revocation | ACTIVE |
| `CAN-L3-C-048` | `CAN-L3-C-051` | Guardian Relationship Establishment and Verification | ACTIVE |
| `CAN-L3-C-049` | `CAN-L3-C-052` | Guardian Relationship Modification | ACTIVE |
| `CAN-L3-C-050` | `CAN-L3-C-053` | Guardian Legal Custody Designation and Change | ACTIVE |
| `CAN-L3-C-051` | `CAN-L3-C-054` | Guardian Consent Authorization and Withdrawal | ACTIVE |
| `CAN-L3-C-052` | `CAN-L3-C-055` | Guardian Communication Authorization Governance | ACTIVE |
| `CAN-L3-C-053` | `CAN-L3-C-056` | Student Emergency Contact Assignment | ACTIVE |
| `CAN-L3-C-054` | `CAN-L3-C-057` | Guardian Financial Responsibility Assignment | ACTIVE |
| `CAN-L3-C-055` | `CAN-L3-C-058` | Student Pickup Authorization Management | ACTIVE |
| `CAN-L3-C-056` | `CAN-L3-C-059` | Guardian Relationship Suspension | ACTIVE |
| `CAN-L3-C-057` | `CAN-L3-C-060` | Guardian Relationship Reinstatement | ACTIVE |
| `CAN-L3-C-058` | `CAN-L3-C-061` | Guardian Relationship Termination | ACTIVE |
| `CAN-L3-C-059` | `CAN-L3-C-062` | Scholarship Definition Publication and Availability | ACTIVE |
| `CAN-L3-C-060` | `CAN-L3-C-063` | Scholarship Eligibility Policy Lifecycle Governance | ACTIVE |
| `CAN-L3-C-061` | `CAN-L3-C-064` | Student Scholarship Eligibility Determination and Recommendation | ACTIVE |
| `CAN-L3-C-062` | `CAN-L3-C-065` | Conditional Scholarship Eligibility Resolution or Expiry | ACTIVE |
| `CAN-L3-C-063` | `CAN-L3-C-066` | Scholarship Eligibility Reassessment | ACTIVE |
| `CAN-L3-C-064` | `CAN-L3-C-067` | Scholarship Renewal Evaluation | ACTIVE |
| `CAN-L3-C-065` | `CAN-L3-C-068` | Scholarship Eligibility Expiration and Archival | ACTIVE |
| `CAN-L3-C-066` | `CAN-L3-C-069` | External Scholarship Ecosystem Synchronization | ACTIVE |
| `CAN-L3-C-067` | `CAN-L3-C-070` | Bulk Scholarship Eligibility Recalculation | ACTIVE |
| `CAN-L3-C-068` | `CAN-L3-C-071` | Scholarship Eligibility to Financial Aid Processing | ACTIVE |
| `CAN-L3-C-069` | `CAN-L3-C-072` | Completed or Eligible Exit Student to Alumni Transition | ACTIVE |
| `CAN-L3-C-070` | `CAN-L3-C-073` | Alumni Transition Correction | ACTIVE |
| `CAN-L3-C-071` | `CAN-L3-C-074` | Alumni Transition Archival | ACTIVE |

### Section D — Academic Delivery (57 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Pallav.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-D-058` | `CAN-L3-D-001` | Course Offering Establishment to Activation | ACTIVE |
| `CAN-L3-D-059` | `CAN-L3-D-002` | Course Offering Suspension | ACTIVE |
| `CAN-L3-D-060` | `CAN-L3-D-003` | Course Offering Reactivation | ACTIVE |
| `CAN-L3-D-061` | `CAN-L3-D-004` | Administrative Course Offering Cancellation | ACTIVE |
| `CAN-L3-D-062` | `CAN-L3-D-005` | Course Offering Capacity Governance | ACTIVE |
| `CAN-L3-D-063` | `CAN-L3-D-006` | Course Offering Enrollment Eligibility and Restriction Governance | ACTIVE |
| `CAN-L3-D-064` | `CAN-L3-D-007` | Course Offering Waitlist to Seat Resolution | ACTIVE |
| `CAN-L3-D-065` | `CAN-L3-D-008` | Cross-Campus and Cross-Institution Course Offering Enrollment Governance | ACTIVE |
| `CAN-L3-D-066` | `CAN-L3-D-009` | Course Offering Revision, Replacement and Rollback | ACTIVE |
| `CAN-L3-D-067` | `CAN-L3-D-010` | Course Offering Archive | ACTIVE |
| `CAN-L3-D-068` | `CAN-L3-D-011` | Course Offering Merge and Split Reorganization | ACTIVE |
| `CAN-L3-D-069` | `CAN-L3-D-012` | Governed Template-Based Course Offering Generation | ACTIVE |
| `CAN-L3-D-070` | `CAN-L3-D-013` | Course Offering Freeze to Governed Unfreeze | ACTIVE |
| `CAN-L3-D-071` | `CAN-L3-D-014` | Course Offering Enrollment Window Governance | ACTIVE |
| `CAN-L3-D-072` | `CAN-L3-D-015` | Course Offering Quota Governance | ACTIVE |
| `CAN-L3-D-073` | `CAN-L3-D-016` | Course Offering Change Simulation to Governed Impact Decision | ACTIVE |
| `CAN-L3-D-074` | `CAN-L3-D-017` | Faculty Academic Assignment Establishment | ACTIVE |
| `CAN-L3-D-075` | `CAN-L3-D-018` | Faculty Academic Assignment Operational Change | ACTIVE |
| `CAN-L3-D-076` | `CAN-L3-D-019` | Faculty Academic Assignment Governance Configuration Change | ACTIVE |
| `CAN-L3-D-077` | `CAN-L3-D-020` | Academic Scheduling Requirement to Governed Operational Timetable | ACTIVE |
| `CAN-L3-D-078` | `CAN-L3-D-021` | Operational Timetable to Governed Revised Timetable | ACTIVE |
| `CAN-L3-D-079` | `CAN-L3-D-022` | Operational Timetable to Retired Historical Timetable | ACTIVE |
| `CAN-L3-D-080` | `CAN-L3-D-023` | Scheduled Academic Session to Governed Teaching Session Outcome | ACTIVE |
| `CAN-L3-D-081` | `CAN-L3-D-024` | Teaching Session Attendance Capture to Attendance Finalization | ACTIVE |
| `CAN-L3-D-082` | `CAN-L3-D-025` | Recorded Attendance to Governed Corrected Attendance | ACTIVE |
| `CAN-L3-D-083` | `CAN-L3-D-026` | Teaching Progress to Published Syllabus Coverage Record | ACTIVE |
| `CAN-L3-D-084` | `CAN-L3-D-027` | Academic Monitoring Requirement to Activated Monitoring Case | ACTIVE |
| `CAN-L3-D-085` | `CAN-L3-D-028` | Activated Monitoring Cycle to Published Monitoring Outcome | ACTIVE |
| `CAN-L3-D-086` | `CAN-L3-D-029` | Monitoring Evidence or Outcome to Governed Academic Risk Assessment | ACTIVE |
| `CAN-L3-D-087` | `CAN-L3-D-030` | Academic Monitoring Governance Exception to Governed Exception Resolution | ACTIVE |
| `CAN-L3-D-088` | `CAN-L3-D-031` | Governed Monitoring Decision to Triggered Monitoring Intervention | ACTIVE |
| `CAN-L3-D-089` | `CAN-L3-D-032` | Monitoring Governance Review to Authorized Governance Decision | ACTIVE |
| `CAN-L3-D-090` | `CAN-L3-D-033` | Monitoring Completion Certification to Governed Case Closure | ACTIVE |
| `CAN-L3-D-091` | `CAN-L3-D-034` | Closed Monitoring Case Reopening to Reinitialized Monitoring Cycle | ACTIVE |
| `CAN-L3-D-092` | `CAN-L3-D-035` | Flexible Learning Opportunity to Governed Flexible Learning Enrollment | ACTIVE |
| `CAN-L3-D-093` | `CAN-L3-D-036` | Eligible Flexible Learning Demand to Governed Seat Allocation | ACTIVE |
| `CAN-L3-D-094` | `CAN-L3-D-037` | Unallocated Eligible Demand to Governed D06 Waitlist Outcome | ACTIVE |
| `CAN-L3-D-095` | `CAN-L3-D-038` | Approved Participation to Governed External Learning Registration | ACTIVE |
| `CAN-L3-D-096` | `CAN-L3-D-039` | Governed Flexible or External Learning Participation to Recorded Learning Completion | ACTIVE |
| `CAN-L3-D-097` | `CAN-L3-D-040` | Verified Flexible Learning Achievement to Governed Credit Recognition and Posting Request | ACTIVE |
| `CAN-L3-D-098` | `CAN-L3-D-041` | Learning Opportunity Establishment to Governed Active Availability | ACTIVE |
| `CAN-L3-D-099` | `CAN-L3-D-042` | Learning Opportunity Suspension to Archived Historical Opportunity | ACTIVE |
| `CAN-L3-D-100` | `CAN-L3-D-043` | External Learning Certificate Verification to Accepted, Rejected or Flagged Certificate Outcome | ACTIVE |
| `CAN-L3-D-101` | `CAN-L3-D-044` | Learning Provider Onboarding to Governed Active Provider | ACTIVE |
| `CAN-L3-D-102` | `CAN-L3-D-045` | Flexible Learning Policy and Approval Configuration | ACTIVE |
| `CAN-L3-D-103` | `CAN-L3-D-046` | Credit Mapping Rule Configuration | ACTIVE |
| `CAN-L3-D-104` | `CAN-L3-D-047` | Flexible Learning Academic Session Archival | ACTIVE |
| `CAN-L3-D-105` | `CAN-L3-D-048` | Seat Allocation Conflict Resolution | ACTIVE |
| `CAN-L3-D-106` | `CAN-L3-D-049` | External Learning Record Reconciliation | ACTIVE |
| `CAN-L3-D-107` | `CAN-L3-D-050` | Flexible Learning Exception Case to Governed Closure | ACTIVE |
| `CAN-L3-D-108` | `CAN-L3-D-051` | Internal Assessment Requirement to Published Assessment Blueprint | ACTIVE |
| `CAN-L3-D-109` | `CAN-L3-D-052` | Published Assessment Blueprint to Governed Assessment Window | ACTIVE |
| `CAN-L3-D-110` | `CAN-L3-D-053` | Open Assessment Opportunity to Governed Student Assessment Submission | ACTIVE |
| `CAN-L3-D-111` | `CAN-L3-D-054` | Submitted Internal Assessment Outcome to Governed Moderated Assessment Outcome | ACTIVE |
| `CAN-L3-D-112` | `CAN-L3-D-055` | Completed Internal Assessment Outcomes to Governed Finalized Internal Assessment | ACTIVE |
| `CAN-L3-D-113` | `CAN-L3-D-056` | Finalized Internal Assessment to Archived Historical Internal Assessment | ACTIVE |
| `CAN-L3-D-114` | `CAN-L3-D-057` | Finalized Internal Assessment Correction to Governed Corrected Assessment | ACTIVE |

### Section E — People & Workforce (75 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Pallav.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-E-105` | `CAN-L3-E-001` | Workforce Engagement Establishment and Activation | ACTIVE |
| `CAN-L3-E-106` | `CAN-L3-E-002` | Active Workforce Engagement Modification | ACTIVE |
| `CAN-L3-E-107` | `CAN-L3-E-003` | Workforce Transfer Between Organisational Contexts | ACTIVE |
| `CAN-L3-E-108` | `CAN-L3-E-004` | Workforce Engagement Suspension and Operational Restoration | ACTIVE |
| `CAN-L3-E-109` | `CAN-L3-E-005` | Workforce Deputation Lifecycle | ACTIVE |
| `CAN-L3-E-110` | `CAN-L3-E-006` | Workforce Long-Leave Lifecycle and Operational Availability | ACTIVE |
| `CAN-L3-E-111` | `CAN-L3-E-007` | Workforce Engagement Closure and Institutional Exit | ACTIVE |
| `CAN-L3-E-112` | `CAN-L3-E-008` | Former Workforce Member Rejoining | ACTIVE |
| `CAN-L3-E-113` | `CAN-L3-E-009` | Duplicate Workforce Profiles to Canonical Workforce Resolution | ACTIVE |
| `CAN-L3-E-114` | `CAN-L3-E-010` | Bulk Workforce Organisational Transfer | ACTIVE |
| `CAN-L3-E-115` | `CAN-L3-E-011` | Workforce Member Assignment to Active Institutional Position | ACTIVE |
| `CAN-L3-E-116` | `CAN-L3-E-012` | Position Assignment Change or Reassignment | ACTIVE |
| `CAN-L3-E-117` | `CAN-L3-E-013` | Position Assignment Completion or Termination | ACTIVE |
| `CAN-L3-E-118` | `CAN-L3-E-014` | Supplementary Position Occupancy Assignment | ACTIVE |
| `CAN-L3-E-119` | `CAN-L3-E-015` | Delegated Authority and Additional Charge Assignment | ACTIVE |
| `CAN-L3-E-120` | `CAN-L3-E-016` | Committee Membership Assignment | ACTIVE |
| `CAN-L3-E-121` | `CAN-L3-E-017` | Responsibility Assignment Change or Withdrawal | ACTIVE |
| `CAN-L3-E-122` | `CAN-L3-E-018` | Reporting Relationship Establishment and Governance | ACTIVE |
| `CAN-L3-E-123` | `CAN-L3-E-019` | Reporting Relationship Change | ACTIVE |
| `CAN-L3-E-124` | `CAN-L3-E-020` | Service Jurisdiction Assignment to Workforce Authority | ACTIVE |
| `CAN-L3-E-125` | `CAN-L3-E-021` | Service Jurisdiction Change or Withdrawal | ACTIVE |
| `CAN-L3-E-126` | `CAN-L3-E-022` | Organisational Authority Resolution | ACTIVE |
| `CAN-L3-E-127` | `CAN-L3-E-023` | Position Occupancy Conflict Resolution | ACTIVE |
| `CAN-L3-E-128` | `CAN-L3-E-024` | Position Assignment Suspension or Freeze to Operational Restoration | ACTIVE |
| `CAN-L3-E-129` | `CAN-L3-E-025` | Position Assignment Expiry to Historical Archive | ACTIVE |
| `CAN-L3-E-130` | `CAN-L3-E-026` | Workforce Member Establishment as Active Faculty | ACTIVE |
| `CAN-L3-E-131` | `CAN-L3-E-027` | Faculty Capability Lifecycle | ACTIVE |
| `CAN-L3-E-132` | `CAN-L3-E-028` | Faculty Qualification Association to Verified Academic-Use Outcome | ACTIVE |
| `CAN-L3-E-133` | `CAN-L3-E-029` | Faculty Teaching Eligibility Lifecycle | ACTIVE |
| `CAN-L3-E-134` | `CAN-L3-E-030` | Faculty Availability and Workload Plan to Governed Capacity | ACTIVE |
| `CAN-L3-E-135` | `CAN-L3-E-031` | Faculty Workforce Suspension and Reactivation | ACTIVE |
| `CAN-L3-E-136` | `CAN-L3-E-032` | Faculty Workforce Closure and Archive | ACTIVE |
| `CAN-L3-E-137` | `CAN-L3-E-033` | Employee Onboarding Case to Accepted Workforce Handoffs | ACTIVE |
| `CAN-L3-E-138` | `CAN-L3-E-034` | Employee Leave Case Coordination to Leave Decision | ACTIVE |
| `CAN-L3-E-139` | `CAN-L3-E-035` | Employee HR Request to Service Resolution | ACTIVE |
| `CAN-L3-E-140` | `CAN-L3-E-036` | Employee Grievance to Investigated Resolution | ACTIVE |
| `CAN-L3-E-141` | `CAN-L3-E-037` | Employee Promotion Case to Accepted Employment-Change Handoffs | ACTIVE |
| `CAN-L3-E-142` | `CAN-L3-E-038` | Employee Transfer Case to Accepted Organisational-Change Handoffs | ACTIVE |
| `CAN-L3-E-143` | `CAN-L3-E-039` | Employee Certificate Request to Authorized Certificate Outcome | ACTIVE |
| `CAN-L3-E-144` | `CAN-L3-E-040` | Employment Contract Renewal Case to Accepted Renewal Handoff | ACTIVE |
| `CAN-L3-E-145` | `CAN-L3-E-041` | Employee Separation Case to Accepted Exit Handoffs | ACTIVE |
| `CAN-L3-E-146` | `CAN-L3-E-042` | HR Compliance Case to Compliance Resolution | ACTIVE |
| `CAN-L3-E-147` | `CAN-L3-E-043` | Employee Probation Review to Confirmation Decision | ACTIVE |
| `CAN-L3-E-148` | `CAN-L3-E-044` | Leave Policy Establishment and Activation | ACTIVE |
| `CAN-L3-E-149` | `CAN-L3-E-045` | Employee Leave Request to Final Decision | ACTIVE |
| `CAN-L3-E-150` | `CAN-L3-E-046` | Periodic Leave Entitlement Accrual | ACTIVE |
| `CAN-L3-E-151` | `CAN-L3-E-047` | Approved Leave to Balance Consumption | ACTIVE |
| `CAN-L3-E-152` | `CAN-L3-E-048` | Approved or Reserved Leave Cancellation | ACTIVE |
| `CAN-L3-E-153` | `CAN-L3-E-049` | Governed Leave Balance Adjustment | ACTIVE |
| `CAN-L3-E-154` | `CAN-L3-E-050` | Leave Year-End Carry Forward | ACTIVE |
| `CAN-L3-E-155` | `CAN-L3-E-051` | Leave Encashment Eligibility Decision | ACTIVE |
| `CAN-L3-E-156` | `CAN-L3-E-052` | Leave Exception Review and Resolution | ACTIVE |
| `CAN-L3-E-157` | `CAN-L3-E-053` | Eligible Payroll Period to Finalized and Published Payroll Outcome | ACTIVE |
| `CAN-L3-E-158` | `CAN-L3-E-054` | Payroll Adjustment to Corrected Payroll Outcome | ACTIVE |
| `CAN-L3-E-159` | `CAN-L3-E-055` | Retrospective Change to Retroactive Pay Result | ACTIVE |
| `CAN-L3-E-160` | `CAN-L3-E-056` | Employee Separation to Final Payroll Settlement Result | ACTIVE |
| `CAN-L3-E-161` | `CAN-L3-E-057` | Exceptional Requirement to Off-Cycle Payroll Completion | ACTIVE |
| `CAN-L3-E-162` | `CAN-L3-E-058` | Workforce Creation to Active Verified Employee Profile | ACTIVE |
| `CAN-L3-E-163` | `CAN-L3-E-059` | Employee Core Profile Maintenance | ACTIVE |
| `CAN-L3-E-164` | `CAN-L3-E-060` | Employee Qualification Registration and Verification | ACTIVE |
| `CAN-L3-E-165` | `CAN-L3-E-061` | Employee Certification Registration and Validity Management | ACTIVE |
| `CAN-L3-E-166` | `CAN-L3-E-062` | Employee Skill Profile Maintenance | ACTIVE |
| `CAN-L3-E-167` | `CAN-L3-E-063` | Employee Language Profile Maintenance | ACTIVE |
| `CAN-L3-E-168` | `CAN-L3-E-064` | Employee Profile Document Registration and Verification | ACTIVE |
| `CAN-L3-E-169` | `CAN-L3-E-065` | Employee Profile Correction with Historical Traceability | ACTIVE |
| `CAN-L3-E-170` | `CAN-L3-E-066` | Workforce Change to Employee Profile Synchronisation | ACTIVE |
| `CAN-L3-E-171` | `CAN-L3-E-067` | HR Case Outcome to Employee Profile Update | ACTIVE |
| `CAN-L3-E-172` | `CAN-L3-E-068` | Employee Separation to Governed Profile Archive | ACTIVE |
| `CAN-L3-E-173` | `CAN-L3-E-069` | Employee Profile Suspension and Restoration | ACTIVE |
| `CAN-L3-E-174` | `CAN-L3-E-070` | Duplicate Employee Profiles to Canonical Profile Merge | ACTIVE |
| `CAN-L3-E-175` | `CAN-L3-E-071` | Position Assignment Administrative Correction to Corrected Authoritative Occupancy | ACTIVE |
| `CAN-L3-E-176` | `CAN-L3-E-072` | Position Assignment Rollback to Restored Valid Organisational State | ACTIVE |
| `CAN-L3-E-177` | `CAN-L3-E-073` | Bulk Position Assignment Operation to Governed Batch Outcome | ACTIVE |
| `CAN-L3-E-178` | `CAN-L3-E-074` | Payroll Configuration Establishment to Effective Payroll Governance | ACTIVE |
| `CAN-L3-E-179` | `CAN-L3-E-075` | Archived Employee Profile Restoration to Active Use | ACTIVE |

### Section F — Finance (85 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Varad.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-F-001` | `CAN-L3-F-001` | Business Obligation to Published Student Charge | ACTIVE |
| `CAN-L3-F-002` | `CAN-L3-F-002` | Published Student Charge to Governed Revision or Adjustment | ACTIVE |
| `CAN-L3-F-003` | `CAN-L3-F-003` | Student Charge to Cancelled, Suspended, or Resumed Obligation | ACTIVE |
| `CAN-L3-F-004` | `CAN-L3-F-004` | Student Charge Dispute to Governed Resolution | ACTIVE |
| `CAN-L3-F-005` | `CAN-L3-F-005` | Student Charge to Installment and Due Schedule | ACTIVE |
| `CAN-L3-F-006` | `CAN-L3-F-006` | Charge Settlement to Financial Closure and Archival | ACTIVE |
| `CAN-L3-F-007` | `CAN-L3-F-007` | Student or Sponsor Payment to Receipt and Accounting Completion | ACTIVE |
| `CAN-L3-F-008` | `CAN-L3-F-008` | Partial or Multi-Obligation Payment to Governed Allocation | ACTIVE |
| `CAN-L3-F-009` | `CAN-L3-F-009` | Unidentified or Unallocated Receipt to Reconciled Payment | ACTIVE |
| `CAN-L3-F-010` | `CAN-L3-F-010` | Payment Failure or Timeout to Safe Recovery | ACTIVE |
| `CAN-L3-F-011` | `CAN-L3-F-011` | Completed Payment to Reversal or Chargeback Outcome | ACTIVE |
| `CAN-L3-F-012` | `CAN-L3-F-012` | Payment Cancellation to Controlled Terminal Outcome | ACTIVE |
| `CAN-L3-F-013` | `CAN-L3-F-013` | Payment Population to Governed Bulk Collection Outcome | ACTIVE |
| `CAN-L3-F-014` | `CAN-L3-F-014` | Refund Request to Approval or Rejection Decision | ACTIVE |
| `CAN-L3-F-015` | `CAN-L3-F-015` | Approved Refund to Financial Completion | ACTIVE |
| `CAN-L3-F-016` | `CAN-L3-F-016` | Approved Refund to Cancellation or Withdrawal | ACTIVE |
| `CAN-L3-F-017` | `CAN-L3-F-017` | Refund Failure to Retry or Manual Resolution | ACTIVE |
| `CAN-L3-F-018` | `CAN-L3-F-018` | Completed Refund to Reversal or Recovery | ACTIVE |
| `CAN-L3-F-019` | `CAN-L3-F-019` | Refund Closure to Historical Reconstruction | ACTIVE |
| `CAN-L3-F-020` | `CAN-L3-F-020` | Approved Credit Note to Reduced Financial Liability | ACTIVE |
| `CAN-L3-F-021` | `CAN-L3-F-021` | Approved Debit Note to Increased Financial Liability | ACTIVE |
| `CAN-L3-F-022` | `CAN-L3-F-022` | Approved Write-Off Adjustment to Governed Financial Outcome | ACTIVE |
| `CAN-L3-F-023` | `CAN-L3-F-023` | Posted Adjustment to Reversal | ACTIVE |
| `CAN-L3-F-024` | `CAN-L3-F-024` | Adjustment Error to Corrected Successor Record | ACTIVE |
| `CAN-L3-F-025` | `CAN-L3-F-025` | Financial Transaction to Balanced Ledger Posting | ACTIVE |
| `CAN-L3-F-026` | `CAN-L3-F-026` | Manual Journal Request to Approved Posting or Rejection | ACTIVE |
| `CAN-L3-F-027` | `CAN-L3-F-027` | Accounting Batch to Governed Posting Outcome | ACTIVE |
| `CAN-L3-F-028` | `CAN-L3-F-028` | Posted Journal to Accounting Reversal | ACTIVE |
| `CAN-L3-F-029` | `CAN-L3-F-029` | Accounting Source and Ledger to Reconciled Position | ACTIVE |
| `CAN-L3-F-030` | `CAN-L3-F-030` | Accounting Period Activity to Trial Balance | ACTIVE |
| `CAN-L3-F-031` | `CAN-L3-F-031` | Ledger Period to Controlled Accounting Close | ACTIVE |
| `CAN-L3-F-032` | `CAN-L3-F-032` | Closed Ledger Evidence to Auditable Financial History | ACTIVE |
| `CAN-L3-F-033` | `CAN-L3-F-033` | Budget Proposal to Approved Active Budget | ACTIVE |
| `CAN-L3-F-034` | `CAN-L3-F-034` | Active Budget to Authorized Revision or Virement | ACTIVE |
| `CAN-L3-F-035` | `CAN-L3-F-035` | Approved Budget to Controlled Allocation | ACTIVE |
| `CAN-L3-F-036` | `CAN-L3-F-036` | Financial Demand to Budget Control Decision | ACTIVE |
| `CAN-L3-F-037` | `CAN-L3-F-037` | Cost Center Request to Active or Closed Cost Center | ACTIVE |
| `CAN-L3-F-038` | `CAN-L3-F-038` | Budget Period Completion to Governed Closure | ACTIVE |
| `CAN-L3-F-039` | `CAN-L3-F-039` | Published Financial Obligation to Accounts Receivable Recognition | ACTIVE |
| `CAN-L3-F-040` | `CAN-L3-F-040` | Overdue Receivable to Governed Dunning Outcome | ACTIVE |
| `CAN-L3-F-041` | `CAN-L3-F-041` | Receipt or Adjustment to Receivable Settlement | ACTIVE |
| `CAN-L3-F-042` | `CAN-L3-F-042` | Uncollectible Receivable to Write-Off Decision | ACTIVE |
| `CAN-L3-F-043` | `CAN-L3-F-043` | New Evidence to Reopened Receivable | ACTIVE |
| `CAN-L3-F-044` | `CAN-L3-F-044` | Vendor Invoice to Approved Payable Liability | ACTIVE |
| `CAN-L3-F-045` | `CAN-L3-F-045` | Approved Payable to Financial Settlement | ACTIVE |
| `CAN-L3-F-046` | `CAN-L3-F-046` | Payable Dispute or Hold to Governed Resolution | ACTIVE |
| `CAN-L3-F-047` | `CAN-L3-F-047` | Payable Change to Approved Adjustment or Reversal | ACTIVE |
| `CAN-L3-F-048` | `CAN-L3-F-048` | Unsettled Payable to Suspension, Reopening, or Write-Off | ACTIVE |
| `CAN-L3-F-049` | `CAN-L3-F-049` | Payable Population to Reconciled Period Outcome | ACTIVE |
| `CAN-L3-F-050` | `CAN-L3-F-050` | Completed Payable to Historical Closure | ACTIVE |
| `CAN-L3-F-051` | `CAN-L3-F-051` | Financial Year and Period Establishment to Open Processing Window | ACTIVE |
| `CAN-L3-F-052` | `CAN-L3-F-052` | Open Financial Period to Locked Close | ACTIVE |
| `CAN-L3-F-053` | `CAN-L3-F-053` | Financial Year End to Immutable Year Closure | ACTIVE |
| `CAN-L3-F-054` | `CAN-L3-F-054` | Closed Period to Controlled Reopening and Reclosure | ACTIVE |
| `CAN-L3-F-055` | `CAN-L3-F-055` | Closed Financial Period to Governed Archival | ACTIVE |
| `CAN-L3-F-056` | `CAN-L3-F-056` | Bank Account Request to Active or Closed Treasury Account | ACTIVE |
| `CAN-L3-F-057` | `CAN-L3-F-057` | Approved Payment Population to Bank Execution Outcome | ACTIVE |
| `CAN-L3-F-058` | `CAN-L3-F-058` | Bank Statement to Reconciled Cash Position | ACTIVE |
| `CAN-L3-F-059` | `CAN-L3-F-059` | Treasury Demand to Current Cash Position and Liquidity Decision | ACTIVE |
| `CAN-L3-F-060` | `CAN-L3-F-060` | Authorized Fund Transfer to Settled Inter-Account Movement | ACTIVE |
| `CAN-L3-F-061` | `CAN-L3-F-061` | Failed, Returned, or Uncertain Bank Transaction to Resolution | ACTIVE |
| `CAN-L3-F-062` | `CAN-L3-F-062` | Approved Aid Decision to Financial Allocation | ACTIVE |
| `CAN-L3-F-063` | `CAN-L3-F-063` | Financial Aid Allocation to Disbursement or Charge Adjustment | ACTIVE |
| `CAN-L3-F-064` | `CAN-L3-F-064` | Continuing Aid Eligibility to Renewal Decision | ACTIVE |
| `CAN-L3-F-065` | `CAN-L3-F-065` | Aid Breach or Reversal to Revocation and Recovery | ACTIVE |
| `CAN-L3-F-066` | `CAN-L3-F-066` | Aid Funding and Utilization to Reconciled Programme Outcome | ACTIVE |
| `CAN-L3-F-067` | `CAN-L3-F-067` | Operational Asset Recognition to Capitalized Financial Asset | ACTIVE |
| `CAN-L3-F-068` | `CAN-L3-F-068` | Active Financial Asset to Periodic Depreciation Posting | ACTIVE |
| `CAN-L3-F-069` | `CAN-L3-F-069` | Financial Asset Revaluation to Updated Carrying Value | ACTIVE |
| `CAN-L3-F-070` | `CAN-L3-F-070` | Impairment Evidence to Recognized Financial Loss | ACTIVE |
| `CAN-L3-F-071` | `CAN-L3-F-071` | Operational Asset Disposal to Completed Disposal Accounting | ACTIVE |
| `CAN-L3-F-072` | `CAN-L3-F-072` | Approved Operational Write-Off to Completed Financial Write-Off | ACTIVE |
| `CAN-L3-F-073` | `CAN-L3-F-073` | Closed Financial Information to Approved Financial Statements | ACTIVE |
| `CAN-L3-F-074` | `CAN-L3-F-074` | Statutory Reporting Obligation to Accepted Financial Report | ACTIVE |
| `CAN-L3-F-075` | `CAN-L3-F-075` | Management Reporting Request to Governed Financial Report | ACTIVE |
| `CAN-L3-F-076` | `CAN-L3-F-076` | Financial Definition Change to Approved KPI or Analytical Model | ACTIVE |
| `CAN-L3-F-077` | `CAN-L3-F-077` | Financial Facts to Published KPI and Dashboard Projection | ACTIVE |
| `CAN-L3-F-078` | `CAN-L3-F-078` | Published Financial Report to Correction, Supersession, or Archival | ACTIVE |
| `CAN-L3-F-079` | `CAN-L3-F-079` | Financial Evidence to Audit and Historical Reconstruction | ACTIVE |
| `CAN-L3-F-080` | `CAN-L3-F-080` | Approved Procurement Demand to Budget Commitment Outcome | ACTIVE |
| `CAN-L3-F-081` | `CAN-L3-F-081` | Purchase Order, Receipt, and Invoice Evidence to Match Decision | ACTIVE |
| `CAN-L3-F-082` | `CAN-L3-F-082` | Accepted Supplier Invoice to Vendor Liability Recognition | ACTIVE |
| `CAN-L3-F-083` | `CAN-L3-F-083` | Procurement Completion to Financial Accounting Closure | ACTIVE |
| `CAN-L3-F-084` | `CAN-L3-F-084` | Finance Configuration Change to Governed Active Configuration | ACTIVE |
| `CAN-L3-F-085` | `CAN-L3-F-085` | Financial Rule Change to Governed Active Policy | ACTIVE |

### Section G — Campus Operations (322 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Varad.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-G-542` | `CAN-L3-G-001` | Campus Readiness Assessment to Governance Readiness Decision | ACTIVE |
| `CAN-L3-G-543` | `CAN-L3-G-002` | Examination Readiness Assessment | ACTIVE |
| `CAN-L3-G-544` | `CAN-L3-G-003` | Campus Operations Evidence to Accreditation Evidence Readiness | ACTIVE |
| `CAN-L3-G-545` | `CAN-L3-G-004` | Campus Operational Review to Governance Closure | ACTIVE |
| `CAN-L3-G-546` | `CAN-L3-G-005` | Executive Governance Review to Executive Recommendation | ACTIVE |
| `CAN-L3-G-547` | `CAN-L3-G-006` | Operational Risk Identification to Risk Closure | ACTIVE |
| `CAN-L3-G-548` | `CAN-L3-G-007` | Multi-Domain Action Plan to Verified Closure | ACTIVE |
| `CAN-L3-G-549` | `CAN-L3-G-008` | Multi-Domain SLA Deviation to Governance Resolution | ACTIVE |
| `CAN-L3-G-550` | `CAN-L3-G-009` | Campus Operational Health Assessment to Governance Decision | ACTIVE |
| `CAN-L3-G-551` | `CAN-L3-G-010` | Governance KPI Review to Governance Outcome | ACTIVE |
| `CAN-L3-G-552` | `CAN-L3-G-011` | Operational Maturity Assessment | ACTIVE |
| `CAN-L3-G-553` | `CAN-L3-G-012` | Governance Finding to Corrective Action Decision | ACTIVE |
| `CAN-L3-G-554` | `CAN-L3-G-013` | Governance Escalation to Executive Resolution | ACTIVE |
| `CAN-L3-G-555` | `CAN-L3-G-014` | Governance Evidence or Dependency Exception to Resolution | ACTIVE |
| `CAN-L3-G-556` | `CAN-L3-G-015` | Governance Configuration Change to Effective Governance State | ACTIVE |
| `CAN-L3-G-557` | `CAN-L3-G-016` | Governance Review to Accreditation Evidence | ACTIVE |
| `CAN-L3-G-558` | `CAN-L3-G-017` | Facility Service Request to Operational Resolution | ACTIVE |
| `CAN-L3-G-559` | `CAN-L3-G-018` | Facility Complaint to Verified Closure | ACTIVE |
| `CAN-L3-G-560` | `CAN-L3-G-019` | Duplicate Facility Complaints to Consolidated Resolution | ACTIVE |
| `CAN-L3-G-561` | `CAN-L3-G-020` | Facility Complaint to Multi-Work-Order Resolution | ACTIVE |
| `CAN-L3-G-562` | `CAN-L3-G-021` | Facility Work Order to Verified Closure | ACTIVE |
| `CAN-L3-G-563` | `CAN-L3-G-022` | Facility Work Order Reopening to Verified Resolution | ACTIVE |
| `CAN-L3-G-564` | `CAN-L3-G-023` | Technician Assignment Exception to Reassignment | ACTIVE |
| `CAN-L3-G-565` | `CAN-L3-G-024` | Preventive Maintenance Plan to Completed Maintenance Cycle | ACTIVE |
| `CAN-L3-G-566` | `CAN-L3-G-025` | Deferred Preventive Maintenance to Controlled Rescheduling | ACTIVE |
| `CAN-L3-G-567` | `CAN-L3-G-026` | Missed Preventive Maintenance to Escalated Recovery | ACTIVE |
| `CAN-L3-G-568` | `CAN-L3-G-027` | Corrective Maintenance to Restored Operation | ACTIVE |
| `CAN-L3-G-569` | `CAN-L3-G-028` | Significant Maintenance Failure to Root Cause Resolution | ACTIVE |
| `CAN-L3-G-570` | `CAN-L3-G-029` | Repeat Facility Failure to Preventive Improvement | ACTIVE |
| `CAN-L3-G-571` | `CAN-L3-G-030` | Emergency Facility Incident to Operational Restoration | ACTIVE |
| `CAN-L3-G-572` | `CAN-L3-G-031` | Emergency Maintenance to Post-Incident Review | ACTIVE |
| `CAN-L3-G-573` | `CAN-L3-G-032` | Vendor Assignment to Verified Service Completion | ACTIVE |
| `CAN-L3-G-574` | `CAN-L3-G-033` | Vendor Service Failure to Corrected Operation and Supplier Evidence | ACTIVE |
| `CAN-L3-G-575` | `CAN-L3-G-034` | Facilities Vendor Performance to Operational Evaluation Evidence | ACTIVE |
| `CAN-L3-G-576` | `CAN-L3-G-035` | AMC Service Cycle to Verified Completion | ACTIVE |
| `CAN-L3-G-577` | `CAN-L3-G-036` | AMC Service Failure to Corrective Action | ACTIVE |
| `CAN-L3-G-578` | `CAN-L3-G-037` | AMC Expiry to Renewal Action | ACTIVE |
| `CAN-L3-G-579` | `CAN-L3-G-038` | Utility Reading to Consumption and Operational Alert | ACTIVE |
| `CAN-L3-G-580` | `CAN-L3-G-039` | Utility Outage to Service Restoration | ACTIVE |
| `CAN-L3-G-581` | `CAN-L3-G-040` | Utility Anomaly to Operational Intervention | ACTIVE |
| `CAN-L3-G-582` | `CAN-L3-G-041` | Facility Inspection to Compliance Closure | ACTIVE |
| `CAN-L3-G-583` | `CAN-L3-G-042` | Facility SLA Monitoring to Satisfied or Breached Resolution | ACTIVE |
| `CAN-L3-G-584` | `CAN-L3-G-043` | Facility Operational Escalation to Resolution | ACTIVE |
| `CAN-L3-G-585` | `CAN-L3-G-044` | Maintenance Consumable Usage to Recorded Consumption History | ACTIVE |
| `CAN-L3-G-586` | `CAN-L3-G-045` | Facility Operational Cost to Recorded Maintenance History | ACTIVE |
| `CAN-L3-G-587` | `CAN-L3-G-046` | Facility Incident to Immediate Maintenance Intervention | ACTIVE |
| `CAN-L3-G-588` | `CAN-L3-G-047` | Facility Maintenance Request to Administrative Closure | ACTIVE |
| `CAN-L3-G-589` | `CAN-L3-G-048` | Invalid Facility Infrastructure Reference to Corrected Resolution | ACTIVE |
| `CAN-L3-G-590` | `CAN-L3-G-049` | Facility Work Conflict to Replanned Execution | ACTIVE |
| `CAN-L3-G-591` | `CAN-L3-G-050` | Facility Configuration Change to Effective Operational Policy | ACTIVE |
| `CAN-L3-G-592` | `CAN-L3-G-051` | Laboratory Readiness Assessment to Operational Readiness | ACTIVE |
| `CAN-L3-G-593` | `CAN-L3-G-052` | Laboratory Readiness Warning to Administrative Decision | ACTIVE |
| `CAN-L3-G-594` | `CAN-L3-G-053` | Laboratory Unavailability to Operational Recovery | ACTIVE |
| `CAN-L3-G-595` | `CAN-L3-G-054` | Practical Session Preparation to Completion | ACTIVE |
| `CAN-L3-G-596` | `CAN-L3-G-055` | Practical Session Postponement to Operational Rescheduling | ACTIVE |
| `CAN-L3-G-597` | `CAN-L3-G-056` | Practical Session Cancellation to Operational Closure | ACTIVE |
| `CAN-L3-G-598` | `CAN-L3-G-057` | Laboratory Scheduling Conflict to Operational Resolution | ACTIVE |
| `CAN-L3-G-599` | `CAN-L3-G-058` | Batch Allocation to Capacity-Valid Laboratory Assignment | ACTIVE |
| `CAN-L3-G-600` | `CAN-L3-G-059` | Batch Capacity Conflict to Reallocation | ACTIVE |
| `CAN-L3-G-601` | `CAN-L3-G-060` | Shared Laboratory Allocation to Multi-Department Session Execution | ACTIVE |
| `CAN-L3-G-602` | `CAN-L3-G-061` | Equipment Operational Status to Laboratory Availability Decision | ACTIVE |
| `CAN-L3-G-603` | `CAN-L3-G-062` | Equipment Restriction to Operational Recovery | ACTIVE |
| `CAN-L3-G-604` | `CAN-L3-G-063` | Laboratory Equipment Usage to Operational Usage History | ACTIVE |
| `CAN-L3-G-605` | `CAN-L3-G-064` | Laboratory Safety Verification to Safe Session Approval | ACTIVE |
| `CAN-L3-G-606` | `CAN-L3-G-065` | Laboratory Safety Warning to Controlled Operational Response | ACTIVE |
| `CAN-L3-G-607` | `CAN-L3-G-066` | Laboratory Incident to Operational Resolution | ACTIVE |
| `CAN-L3-G-608` | `CAN-L3-G-067` | Laboratory Incident to Maintenance Recommendation | ACTIVE |
| `CAN-L3-G-609` | `CAN-L3-G-068` | Laboratory Inspection to Operational Closure | ACTIVE |
| `CAN-L3-G-610` | `CAN-L3-G-069` | Laboratory Inspection Finding to Corrective Recommendation | ACTIVE |
| `CAN-L3-G-611` | `CAN-L3-G-070` | Laboratory Compliance Assessment to Compliance Decision | ACTIVE |
| `CAN-L3-G-612` | `CAN-L3-G-071` | Laboratory Non-Compliance to Resolution | ACTIVE |
| `CAN-L3-G-613` | `CAN-L3-G-072` | Laboratory Consumable Usage to Operational Consumption History | ACTIVE |
| `CAN-L3-G-614` | `CAN-L3-G-073` | Laboratory Operational Restriction to Recovery | ACTIVE |
| `CAN-L3-G-615` | `CAN-L3-G-074` | Laboratory Utility Failure to Operational Recovery | ACTIVE |
| `CAN-L3-G-616` | `CAN-L3-G-075` | Laboratory Batch Reallocation to Session Continuation | ACTIVE |
| `CAN-L3-G-617` | `CAN-L3-G-076` | Laboratory Emergency Procedure Activation to Operational Stabilisation | ACTIVE |
| `CAN-L3-G-618` | `CAN-L3-G-077` | Laboratory Operational Recommendation to Controlled Follow-Up | ACTIVE |
| `CAN-L3-G-619` | `CAN-L3-G-078` | Laboratory Configuration Change to Effective Operational Policy | ACTIVE |
| `CAN-L3-G-620` | `CAN-L3-G-079` | Hostel Admission Application to Admission Decision | ACTIVE |
| `CAN-L3-G-621` | `CAN-L3-G-080` | Approved Hostel Admission to Active Residency | ACTIVE |
| `CAN-L3-G-622` | `CAN-L3-G-081` | Hostel Capacity Constraint to Waiting List Placement | ACTIVE |
| `CAN-L3-G-623` | `CAN-L3-G-082` | Hostel Residency End to Cleared and Closed Occupancy | ACTIVE |
| `CAN-L3-G-624` | `CAN-L3-G-083` | Resident Leave Request to Verified Return | ACTIVE |
| `CAN-L3-G-625` | `CAN-L3-G-084` | Curfew Exception to Authorised Late Entry | ACTIVE |
| `CAN-L3-G-626` | `CAN-L3-G-085` | Visitor Request to Closed Hostel Visit | ACTIVE |
| `CAN-L3-G-627` | `CAN-L3-G-086` | Hostel Discipline Incident to Disciplinary Resolution | ACTIVE |
| `CAN-L3-G-628` | `CAN-L3-G-087` | Curfew Violation to Disciplinary Action | ACTIVE |
| `CAN-L3-G-629` | `CAN-L3-G-088` | Room Transfer Request to New Room Allocation | ACTIVE |
| `CAN-L3-G-630` | `CAN-L3-G-089` | Hostel Complaint to Governed Resolution | ACTIVE |
| `CAN-L3-G-631` | `CAN-L3-G-090` | Hostel Room Inspection to Damage Resolution | ACTIVE |
| `CAN-L3-G-632` | `CAN-L3-G-091` | Mess Allocation to Active Dining Assignment | ACTIVE |
| `CAN-L3-G-633` | `CAN-L3-G-092` | Mess Allocation Change to Effective Reallocation | ACTIVE |
| `CAN-L3-G-634` | `CAN-L3-G-093` | Hostel Configuration Change to Effective Residency Policy | ACTIVE |
| `CAN-L3-G-635` | `CAN-L3-G-094` | Transport Route Creation to Operational Activation | ACTIVE |
| `CAN-L3-G-636` | `CAN-L3-G-095` | Transport Route Modification to Effective Route Version | ACTIVE |
| `CAN-L3-G-637` | `CAN-L3-G-096` | Route Suspension to Operational Transport Restriction | ACTIVE |
| `CAN-L3-G-638` | `CAN-L3-G-097` | Transport Stop Creation to Active Pickup/Drop Location | ACTIVE |
| `CAN-L3-G-639` | `CAN-L3-G-098` | Transport Stop Suspension to Route Operational Adjustment | ACTIVE |
| `CAN-L3-G-640` | `CAN-L3-G-099` | Route Sequencing Change to Operational Route Update | ACTIVE |
| `CAN-L3-G-641` | `CAN-L3-G-100` | Transport Request to Active Allocation | ACTIVE |
| `CAN-L3-G-642` | `CAN-L3-G-101` | Transport Allocation Change to Effective Route/Stop Reassignment | ACTIVE |
| `CAN-L3-G-643` | `CAN-L3-G-102` | Transport Allocation Withdrawal to Service Closure | ACTIVE |
| `CAN-L3-G-644` | `CAN-L3-G-103` | Driver Assignment to Trip Readiness | ACTIVE |
| `CAN-L3-G-645` | `CAN-L3-G-104` | Driver Replacement to Maintained Trip Readiness | ACTIVE |
| `CAN-L3-G-646` | `CAN-L3-G-105` | Trip Planning to Ready Transport Trip | ACTIVE |
| `CAN-L3-G-647` | `CAN-L3-G-106` | Scheduled Trip Cancellation to Operational Closure | ACTIVE |
| `CAN-L3-G-648` | `CAN-L3-G-107` | Trip Ready to Trip Execution and Completion | ACTIVE |
| `CAN-L3-G-649` | `CAN-L3-G-108` | Trip Execution with Route Deviation to Operational Completion | ACTIVE |
| `CAN-L3-G-650` | `CAN-L3-G-109` | Vehicle Availability to Operational Vehicle Assignment | ACTIVE |
| `CAN-L3-G-651` | `CAN-L3-G-110` | Vehicle Breakdown to Replacement Vehicle Assignment | ACTIVE |
| `CAN-L3-G-652` | `CAN-L3-G-111` | Vehicle Operational Issue to Maintenance Recommendation | ACTIVE |
| `CAN-L3-G-653` | `CAN-L3-G-112` | Pre-Trip Operational Checklist to Trip Readiness | ACTIVE |
| `CAN-L3-G-654` | `CAN-L3-G-113` | Post-Trip Inspection to Operational Follow-Up | ACTIVE |
| `CAN-L3-G-655` | `CAN-L3-G-114` | Transport Pass Issuance to Active Transport Authorisation | ACTIVE |
| `CAN-L3-G-656` | `CAN-L3-G-115` | Transport Pass Renewal to Continued Transport Authorisation | ACTIVE |
| `CAN-L3-G-657` | `CAN-L3-G-116` | Lost or Damaged Transport Pass to Replacement Pass | ACTIVE |
| `CAN-L3-G-658` | `CAN-L3-G-117` | Transport Pass Suspension to Restricted Transport Access | ACTIVE |
| `CAN-L3-G-659` | `CAN-L3-G-118` | Route Diversion Request to Emergency Operational Route | ACTIVE |
| `CAN-L3-G-660` | `CAN-L3-G-119` | Temporary Stop Change to Affected Passenger Reassignment | ACTIVE |
| `CAN-L3-G-661` | `CAN-L3-G-120` | Permanent Route Revision to New Operational Route | ACTIVE |
| `CAN-L3-G-662` | `CAN-L3-G-121` | Transport Complaint to Operational Resolution | ACTIVE |
| `CAN-L3-G-663` | `CAN-L3-G-122` | Transport Complaint to Maintenance Recommendation | ACTIVE |
| `CAN-L3-G-664` | `CAN-L3-G-123` | Transport Complaint to Driver or Service Corrective Action | ACTIVE |
| `CAN-L3-G-665` | `CAN-L3-G-124` | Transport Configuration Change to Effective Operational Policy | ACTIVE |
| `CAN-L3-G-666` | `CAN-L3-G-125` | Library Membership Request to Active Membership | ACTIVE |
| `CAN-L3-G-667` | `CAN-L3-G-126` | Library Membership Suspension to Restricted Library Access | ACTIVE |
| `CAN-L3-G-668` | `CAN-L3-G-127` | Library Membership Closure to Library Access Termination | ACTIVE |
| `CAN-L3-G-669` | `CAN-L3-G-128` | Catalogue Resource Identification to Published Catalogue Record | ACTIVE |
| `CAN-L3-G-670` | `CAN-L3-G-129` | Catalogue Update to Effective Resource Information | ACTIVE |
| `CAN-L3-G-671` | `CAN-L3-G-130` | Library Copy Registration to Circulation Availability | ACTIVE |
| `CAN-L3-G-672` | `CAN-L3-G-131` | Library Copy Condition Change to Operational Restriction | ACTIVE |
| `CAN-L3-G-673` | `CAN-L3-G-132` | Library Resource Issue to Active Borrowing | ACTIVE |
| `CAN-L3-G-674` | `CAN-L3-G-133` | Library Resource Return to Closed Circulation | ACTIVE |
| `CAN-L3-G-675` | `CAN-L3-G-134` | Library Borrowing Renewal to Extended Due Date | ACTIVE |
| `CAN-L3-G-676` | `CAN-L3-G-135` | Library Resource Reservation to Waiting List Position | ACTIVE |
| `CAN-L3-G-677` | `CAN-L3-G-136` | Reserved Resource Availability to Collection | ACTIVE |
| `CAN-L3-G-678` | `CAN-L3-G-137` | Reservation Expiry to Reservation Closure | ACTIVE |
| `CAN-L3-G-679` | `CAN-L3-G-138` | Overdue Borrowing to Fine Calculation | ACTIVE |
| `CAN-L3-G-680` | `CAN-L3-G-139` | Fine Waiver Request to Administrative Decision | ACTIVE |
| `CAN-L3-G-681` | `CAN-L3-G-140` | Lost Library Resource to Replacement Obligation | ACTIVE |
| `CAN-L3-G-682` | `CAN-L3-G-141` | Damaged Library Resource to Replacement or Recovery Action | ACTIVE |
| `CAN-L3-G-683` | `CAN-L3-G-142` | Acquisition Request to Approved Library Resource Procurement Reference | ACTIVE |
| `CAN-L3-G-684` | `CAN-L3-G-143` | Library Resource Receipt to Catalogued Library Copy | ACTIVE |
| `CAN-L3-G-685` | `CAN-L3-G-144` | Reading Room Opening to Operational Availability | ACTIVE |
| `CAN-L3-G-686` | `CAN-L3-G-145` | Reading Room Closure to Restricted Library Usage | ACTIVE |
| `CAN-L3-G-687` | `CAN-L3-G-146` | Reading Room Operational Usage to Session Closure | ACTIVE |
| `CAN-L3-G-688` | `CAN-L3-G-147` | Stock Verification Initiation to Verification Completion | ACTIVE |
| `CAN-L3-G-689` | `CAN-L3-G-148` | Missing Library Resource to Investigation Outcome | ACTIVE |
| `CAN-L3-G-690` | `CAN-L3-G-149` | Damaged Library Resource During Stock Verification to Administrative Review | ACTIVE |
| `CAN-L3-G-691` | `CAN-L3-G-150` | Library Clearance Request to Clearance Approval | ACTIVE |
| `CAN-L3-G-692` | `CAN-L3-G-151` | Library Clearance Approval to Institutional Exit Consumption | ACTIVE |
| `CAN-L3-G-693` | `CAN-L3-G-152` | Library Resource Availability Conflict to Alternative Resolution | ACTIVE |
| `CAN-L3-G-694` | `CAN-L3-G-153` | Library Borrowing Restriction to Administrative Resolution | ACTIVE |
| `CAN-L3-G-695` | `CAN-L3-G-154` | Library Configuration Change to Effective Operational Policy | ACTIVE |
| `CAN-L3-G-696` | `CAN-L3-G-155` | Library Operational Reporting to Institutional Governance Outcome | ACTIVE |
| `CAN-L3-G-697` | `CAN-L3-G-156` | Health Centre Configuration to Operational Activation | ACTIVE |
| `CAN-L3-G-698` | `CAN-L3-G-157` | Health Centre Temporary Closure to Restricted Healthcare Availability | ACTIVE |
| `CAN-L3-G-699` | `CAN-L3-G-158` | Patient Identification to Healthcare Visit Registration | ACTIVE |
| `CAN-L3-G-700` | `CAN-L3-G-159` | Patient Registration Exception to Alternative Identification Resolution | ACTIVE |
| `CAN-L3-G-701` | `CAN-L3-G-160` | Medical History Review to Consultation Readiness | ACTIVE |
| `CAN-L3-G-702` | `CAN-L3-G-161` | Walk-in Consultation to Completed Medical Encounter | ACTIVE |
| `CAN-L3-G-703` | `CAN-L3-G-162` | Follow-up Recommendation to Follow-up Healthcare Activity | ACTIVE |
| `CAN-L3-G-704` | `CAN-L3-G-163` | Emergency Incident to Immediate Institutional Medical Response | ACTIVE |
| `CAN-L3-G-705` | `CAN-L3-G-164` | First Aid Incident to Stabilised Patient Outcome | ACTIVE |
| `CAN-L3-G-706` | `CAN-L3-G-165` | Emergency Case to External Referral | ACTIVE |
| `CAN-L3-G-707` | `CAN-L3-G-166` | Emergency Case to Ambulance Coordination | ACTIVE |
| `CAN-L3-G-708` | `CAN-L3-G-167` | Consultation to Prescription Creation | ACTIVE |
| `CAN-L3-G-709` | `CAN-L3-G-168` | Prescription to Medicine Dispensing | ACTIVE |
| `CAN-L3-G-710` | `CAN-L3-G-169` | Medicine Availability Exception to Alternative Medical Action | ACTIVE |
| `CAN-L3-G-711` | `CAN-L3-G-170` | Medicine Inventory Threshold or Expiry to Medical Operations Alert | ACTIVE |
| `CAN-L3-G-712` | `CAN-L3-G-171` | Observation Admission to Observation Completion | ACTIVE |
| `CAN-L3-G-713` | `CAN-L3-G-172` | Observation Capacity Exception to Alternative Care Decision | ACTIVE |
| `CAN-L3-G-714` | `CAN-L3-G-173` | Medical Referral Recommendation to Referral Completion | ACTIVE |
| `CAN-L3-G-715` | `CAN-L3-G-174` | Referral Follow-up to Updated Healthcare History | ACTIVE |
| `CAN-L3-G-716` | `CAN-L3-G-175` | Medical Assessment to Medical Certificate Issuance | ACTIVE |
| `CAN-L3-G-717` | `CAN-L3-G-176` | Medical Certificate Review to Certificate Revocation | ACTIVE |
| `CAN-L3-G-718` | `CAN-L3-G-177` | Health Programme Planning to Programme Completion | ACTIVE |
| `CAN-L3-G-719` | `CAN-L3-G-178` | Vaccination Programme to Recorded Vaccination Outcome | ACTIVE |
| `CAN-L3-G-720` | `CAN-L3-G-179` | Annual Health Checkup to Recorded Assessment Outcome | ACTIVE |
| `CAN-L3-G-721` | `CAN-L3-G-180` | Health Screening or Camp Finding to Follow-up Recommendation | ACTIVE |
| `CAN-L3-G-722` | `CAN-L3-G-181` | Occupational Health Assessment to Medical Outcome | ACTIVE |
| `CAN-L3-G-723` | `CAN-L3-G-182` | Medical Event to Student Portal Health Information | ACTIVE |
| `CAN-L3-G-724` | `CAN-L3-G-183` | Medical Event to Employee Portal Health Information | ACTIVE |
| `CAN-L3-G-725` | `CAN-L3-G-184` | Medical Professional Unavailability to Alternative Operational Assignment | ACTIVE |
| `CAN-L3-G-726` | `CAN-L3-G-185` | Medical Configuration Change to Effective Healthcare Policy | ACTIVE |
| `CAN-L3-G-727` | `CAN-L3-G-186` | Healthcare Operational Reporting to Institutional Governance Outcome | ACTIVE |
| `CAN-L3-G-728` | `CAN-L3-G-187` | Medical Clearance Request to Institutional Clearance Decision | ACTIVE |
| `CAN-L3-G-729` | `CAN-L3-G-188` | Security Office Configuration to Operational Activation | ACTIVE |
| `CAN-L3-G-730` | `CAN-L3-G-189` | Security Office Temporary Closure to Restricted Security Availability | ACTIVE |
| `CAN-L3-G-731` | `CAN-L3-G-190` | Gate Configuration to Operational Activation | ACTIVE |
| `CAN-L3-G-732` | `CAN-L3-G-191` | Gate Opening to Operational Availability | ACTIVE |
| `CAN-L3-G-733` | `CAN-L3-G-192` | Gate Closure to Restricted Access | ACTIVE |
| `CAN-L3-G-734` | `CAN-L3-G-193` | Person Gate Access to Recorded Entry, Denial, or Authorised Override | ACTIVE |
| `CAN-L3-G-735` | `CAN-L3-G-194` | Visitor Registration to Authorised Visit and Governed Pass | ACTIVE |
| `CAN-L3-G-736` | `CAN-L3-G-195` | Visitor Check-In to Visit Closure or Overstay Resolution | ACTIVE |
| `CAN-L3-G-737` | `CAN-L3-G-196` | Vehicle Arrival to Closed Campus Movement | ACTIVE |
| `CAN-L3-G-738` | `CAN-L3-G-197` | Material Movement Request to Completed or Rejected Gate Pass | ACTIVE |
| `CAN-L3-G-739` | `CAN-L3-G-198` | Courier Arrival to Verified Delivery Completion and Exit | ACTIVE |
| `CAN-L3-G-740` | `CAN-L3-G-199` | Contractor Registration to Completed Work Access and Exit | ACTIVE |
| `CAN-L3-G-741` | `CAN-L3-G-200` | Patrol Requirement to Completed Patrol or Escalated Security Exception | ACTIVE |
| `CAN-L3-G-742` | `CAN-L3-G-201` | Security Shift Handover to Operational Continuity | ACTIVE |
| `CAN-L3-G-743` | `CAN-L3-G-202` | Security Incident Report to Closed Resolution | ACTIVE |
| `CAN-L3-G-744` | `CAN-L3-G-203` | Security Emergency to Coordinated Emergency Response | ACTIVE |
| `CAN-L3-G-745` | `CAN-L3-G-204` | Medical Security Incident to G06 Medical Coordination | ACTIVE |
| `CAN-L3-G-746` | `CAN-L3-G-205` | Missing Person Report to Security Response | ACTIVE |
| `CAN-L3-G-747` | `CAN-L3-G-206` | Fire or Safety Incident to Coordinated Emergency Response | ACTIVE |
| `CAN-L3-G-748` | `CAN-L3-G-207` | Lost Property Report to Lost Item Registration | ACTIVE |
| `CAN-L3-G-749` | `CAN-L3-G-208` | Recovered Property to Found Item Registration | ACTIVE |
| `CAN-L3-G-750` | `CAN-L3-G-209` | Lost or Found Ownership Verification to Property Claim | ACTIVE |
| `CAN-L3-G-751` | `CAN-L3-G-210` | Verified Property Claim to Handover | ACTIVE |
| `CAN-L3-G-752` | `CAN-L3-G-211` | Unclaimed Property to Administrative Disposition | ACTIVE |
| `CAN-L3-G-753` | `CAN-L3-G-212` | Escort Request to Escort Assignment | ACTIVE |
| `CAN-L3-G-754` | `CAN-L3-G-213` | Escort Assignment to Completed Escort | ACTIVE |
| `CAN-L3-G-755` | `CAN-L3-G-214` | Escort Request Exception to Alternative Security Response | ACTIVE |
| `CAN-L3-G-756` | `CAN-L3-G-215` | Security Policy Configuration to Effective Operational Governance | ACTIVE |
| `CAN-L3-G-757` | `CAN-L3-G-216` | Security Delegation to Temporary Operational Authority | ACTIVE |
| `CAN-L3-G-758` | `CAN-L3-G-217` | Security Operations to Governed Report and Dashboard Projection | ACTIVE |
| `CAN-L3-G-759` | `CAN-L3-G-218` | Housekeeping Zone Configuration to Operational Activation | ACTIVE |
| `CAN-L3-G-760` | `CAN-L3-G-219` | Housekeeping Zone Configuration Change to Effective Operational Policy | ACTIVE |
| `CAN-L3-G-761` | `CAN-L3-G-220` | Housekeeping Zone Deactivation to Controlled Service Restriction | ACTIVE |
| `CAN-L3-G-762` | `CAN-L3-G-221` | Cleaning Schedule Planning to Active Cleaning Programme | ACTIVE |
| `CAN-L3-G-763` | `CAN-L3-G-222` | Active Cleaning Schedule to Verified Task Completion | ACTIVE |
| `CAN-L3-G-764` | `CAN-L3-G-223` | Cleaning Assignment Exception to Alternative Housekeeping Assignment | ACTIVE |
| `CAN-L3-G-765` | `CAN-L3-G-224` | Housekeeping Quality Failure to Verified Re-clean Resolution | ACTIVE |
| `CAN-L3-G-766` | `CAN-L3-G-225` | Deep Cleaning Plan to Completed Deep Cleaning Cycle | ACTIVE |
| `CAN-L3-G-767` | `CAN-L3-G-226` | Event Cleaning Plan to Event-Ready Campus Area | ACTIVE |
| `CAN-L3-G-768` | `CAN-L3-G-227` | Emergency or Critical Cleaning Need to Restored Area | ACTIVE |
| `CAN-L3-G-769` | `CAN-L3-G-228` | Special Cleaning Requirement to Completed Special Cleaning | ACTIVE |
| `CAN-L3-G-770` | `CAN-L3-G-229` | Housekeeping Complaint Registration to Verified Resolution | ACTIVE |
| `CAN-L3-G-771` | `CAN-L3-G-230` | Repeat Cleaning Complaint to Corrective Resolution | ACTIVE |
| `CAN-L3-G-772` | `CAN-L3-G-231` | SLA Warning to Proactive Cleaning Escalation | ACTIVE |
| `CAN-L3-G-773` | `CAN-L3-G-232` | Cleaning Request SLA Breach to Escalated Resolution | ACTIVE |
| `CAN-L3-G-774` | `CAN-L3-G-233` | Hygiene Inspection to Compliance or Corrective Outcome | ACTIVE |
| `CAN-L3-G-775` | `CAN-L3-G-234` | Washroom Consumable Shortage to Restocking or Operational Escalation | ACTIVE |
| `CAN-L3-G-776` | `CAN-L3-G-235` | Cleaning Quality Inspection to Approved Service Outcome | ACTIVE |
| `CAN-L3-G-777` | `CAN-L3-G-236` | Housekeeping Incident to Operational Resolution | ACTIVE |
| `CAN-L3-G-778` | `CAN-L3-G-237` | Facility Issue to Governed Maintenance Handoff | ACTIVE |
| `CAN-L3-G-779` | `CAN-L3-G-238` | Housekeeping Incident Escalation to Controlled Resolution | ACTIVE |
| `CAN-L3-G-780` | `CAN-L3-G-239` | Waste Collection Schedule to Completed Collection | ACTIVE |
| `CAN-L3-G-781` | `CAN-L3-G-240` | Waste Collection Exception to Recovery | ACTIVE |
| `CAN-L3-G-782` | `CAN-L3-G-241` | Waste Segregation to Verified Collection | ACTIVE |
| `CAN-L3-G-783` | `CAN-L3-G-242` | Specialised Waste Reference to External Handling Coordination | ACTIVE |
| `CAN-L3-G-784` | `CAN-L3-G-243` | Housekeeping Consumable Availability Monitoring to Operational Alert | ACTIVE |
| `CAN-L3-G-785` | `CAN-L3-G-244` | Consumable Shortage to Operational Continuity Decision | ACTIVE |
| `CAN-L3-G-786` | `CAN-L3-G-245` | Vendor Operational Assignment to Service Delivery | ACTIVE |
| `CAN-L3-G-787` | `CAN-L3-G-246` | Vendor Operational Performance to Service Quality Outcome | ACTIVE |
| `CAN-L3-G-788` | `CAN-L3-G-247` | SLA Performance Monitoring to Service Governance Outcome | ACTIVE |
| `CAN-L3-G-789` | `CAN-L3-G-248` | Housekeeping Vendor SLA Breach to Corrective Service and Supplier Evidence | ACTIVE |
| `CAN-L3-G-790` | `CAN-L3-G-249` | Housekeeping Service Quality Trend to Governance Review | ACTIVE |
| `CAN-L3-G-791` | `CAN-L3-G-250` | Zone Cleanliness Monitoring to Operational Improvement | ACTIVE |
| `CAN-L3-G-792` | `CAN-L3-G-251` | Housekeeping Policy Configuration to Effective Service Governance | ACTIVE |
| `CAN-L3-G-793` | `CAN-L3-G-252` | Housekeeping Delegation to Temporary Operational Authority | ACTIVE |
| `CAN-L3-G-794` | `CAN-L3-G-253` | Housekeeping Operations to Governed Report and Dashboard Projection | ACTIVE |
| `CAN-L3-G-795` | `CAN-L3-G-254` | Teacher Guardian Assignment Publication to Mentorship Relationship Activation | ACTIVE |
| `CAN-L3-G-796` | `CAN-L3-G-255` | Teacher Guardian Assignment Change to New Mentorship Relationship | ACTIVE |
| `CAN-L3-G-797` | `CAN-L3-G-256` | Student Status Change to Mentorship Relationship Review | ACTIVE |
| `CAN-L3-G-798` | `CAN-L3-G-257` | Teacher Guardian Unavailability to Mentorship Relationship Review | ACTIVE |
| `CAN-L3-G-799` | `CAN-L3-G-258` | Mentorship Relationship Activation to Mentoring Workspace Availability | ACTIVE |
| `CAN-L3-G-800` | `CAN-L3-G-259` | Scheduled Mentorship Period to Mentoring Session Initiation | ACTIVE |
| `CAN-L3-G-801` | `CAN-L3-G-260` | Unscheduled Mentoring Interaction to Recorded Mentoring Session | ACTIVE |
| `CAN-L3-G-802` | `CAN-L3-G-261` | Authorised Student Context to Privacy-Controlled Mentoring Readiness | ACTIVE |
| `CAN-L3-G-803` | `CAN-L3-G-262` | External Context Unavailability to Mentoring Continuation | ACTIVE |
| `CAN-L3-G-804` | `CAN-L3-G-263` | Mentoring Session to Completed Mentoring Record | ACTIVE |
| `CAN-L3-G-805` | `CAN-L3-G-264` | Mentoring Discussion to Parent Interaction Requirement | ACTIVE |
| `CAN-L3-G-806` | `CAN-L3-G-265` | Parent Interaction Initiation to Recorded Parent Discussion | ACTIVE |
| `CAN-L3-G-807` | `CAN-L3-G-266` | Parent Interaction to Follow-up Requirement | ACTIVE |
| `CAN-L3-G-808` | `CAN-L3-G-267` | Mentoring Issue to Governed Institutional Referral | ACTIVE |
| `CAN-L3-G-809` | `CAN-L3-G-268` | Mentoring Follow-up Cycle to Completed Progress Outcome | ACTIVE |
| `CAN-L3-G-810` | `CAN-L3-G-269` | Academic Session Completion to Mentorship Case Closure Review | ACTIVE |
| `CAN-L3-G-811` | `CAN-L3-G-270` | New Teacher Guardian Assignment to Previous Mentorship Case Closure | ACTIVE |
| `CAN-L3-G-812` | `CAN-L3-G-271` | Student Exit to Mentorship Case Closure | ACTIVE |
| `CAN-L3-G-813` | `CAN-L3-G-272` | Administrative Mentorship Closure to Archived Mentoring History | ACTIVE |
| `CAN-L3-G-814` | `CAN-L3-G-273` | Pending Follow-up During Closure to Controlled Closure Resolution | ACTIVE |
| `CAN-L3-G-815` | `CAN-L3-G-274` | Mentorship Record Correction to Audited Successor Record | ACTIVE |
| `CAN-L3-G-816` | `CAN-L3-G-275` | Mentoring Information Request to Policy-Controlled Disclosure Outcome | ACTIVE |
| `CAN-L3-G-817` | `CAN-L3-G-276` | Mentorship Event to Complete Mentoring Timeline | ACTIVE |
| `CAN-L3-G-818` | `CAN-L3-G-277` | Mentorship History to Student Mentoring Report | ACTIVE |
| `CAN-L3-G-819` | `CAN-L3-G-278` | Mentorship Operations to Governed Management Report and Projection | ACTIVE |
| `CAN-L3-G-820` | `CAN-L3-G-279` | Mentorship Extension Configuration to New Mentoring Capability | ACTIVE |
| `CAN-L3-G-821` | `CAN-L3-G-280` | Institutional Infrastructure Registration to Physical Campus Establishment | ACTIVE |
| `CAN-L3-G-822` | `CAN-L3-G-281` | Institutional Campus Reference to Physical Campus Association | ACTIVE |
| `CAN-L3-G-823` | `CAN-L3-G-282` | Physical Infrastructure Attribute Change to Effective-Dated Governed Update | ACTIVE |
| `CAN-L3-G-824` | `CAN-L3-G-283` | Infrastructure Hierarchy Creation to Canonical Physical Structure | ACTIVE |
| `CAN-L3-G-825` | `CAN-L3-G-284` | Building Registration to Physical Building Establishment | ACTIVE |
| `CAN-L3-G-826` | `CAN-L3-G-285` | Floor Registration to Building Hierarchy Extension | ACTIVE |
| `CAN-L3-G-827` | `CAN-L3-G-286` | Space Registration to Physical Space Establishment | ACTIVE |
| `CAN-L3-G-828` | `CAN-L3-G-287` | Physical Zone Establishment to Infrastructure Grouping | ACTIVE |
| `CAN-L3-G-829` | `CAN-L3-G-288` | Space Classification to Controlled Physical Classification | ACTIVE |
| `CAN-L3-G-830` | `CAN-L3-G-289` | Space Capability Definition to Operational Capability Availability | ACTIVE |
| `CAN-L3-G-831` | `CAN-L3-G-290` | Infrastructure Capacity Definition to Capacity Profile Activation | ACTIVE |
| `CAN-L3-G-832` | `CAN-L3-G-291` | Infrastructure Availability Definition to Availability Profile Activation | ACTIVE |
| `CAN-L3-G-833` | `CAN-L3-G-292` | Infrastructure Availability Change to Suspension or Restoration Outcome | ACTIVE |
| `CAN-L3-G-834` | `CAN-L3-G-293` | Infrastructure Reservation Request to Controlled Reservation | ACTIVE |
| `CAN-L3-G-835` | `CAN-L3-G-294` | Infrastructure Reservation Change to Controlled Rescheduling | ACTIVE |
| `CAN-L3-G-836` | `CAN-L3-G-295` | Infrastructure Reservation Cancellation to Resource Availability Restoration | ACTIVE |
| `CAN-L3-G-837` | `CAN-L3-G-296` | Operational Allocation to Infrastructure Allocation Establishment | ACTIVE |
| `CAN-L3-G-838` | `CAN-L3-G-297` | Operational Allocation Change to Controlled Reallocation | ACTIVE |
| `CAN-L3-G-839` | `CAN-L3-G-298` | Infrastructure Operational Responsibility Assignment to Governance Record | ACTIVE |
| `CAN-L3-G-840` | `CAN-L3-G-299` | Infrastructure Responsibility Change to Effective Responsibility Transfer | ACTIVE |
| `CAN-L3-G-841` | `CAN-L3-G-300` | Infrastructure Validation to Physical Consistency Confirmation | ACTIVE |
| `CAN-L3-G-842` | `CAN-L3-G-301` | Administrative Infrastructure Change to Governed Update | ACTIVE |
| `CAN-L3-G-843` | `CAN-L3-G-302` | Infrastructure Compliance Requirement to Compliance Record Establishment | ACTIVE |
| `CAN-L3-G-844` | `CAN-L3-G-303` | Compliance Status Change to Infrastructure Compliance Update | ACTIVE |
| `CAN-L3-G-845` | `CAN-L3-G-304` | Compliance Failure to Controlled Infrastructure Restriction | ACTIVE |
| `CAN-L3-G-846` | `CAN-L3-G-305` | Infrastructure Inspection Requirement to Closed Risk-Reviewed Outcome | ACTIVE |
| `CAN-L3-G-847` | `CAN-L3-G-306` | Preventive Maintenance Governance to Infrastructure Maintenance Requirement | ACTIVE |
| `CAN-L3-G-848` | `CAN-L3-G-307` | Corrective Maintenance Governance to Infrastructure Remediation Requirement | ACTIVE |
| `CAN-L3-G-849` | `CAN-L3-G-308` | Emergency Infrastructure Condition to Emergency Maintenance Governance | ACTIVE |
| `CAN-L3-G-850` | `CAN-L3-G-309` | Maintenance Completion to Infrastructure Condition Reconciliation | ACTIVE |
| `CAN-L3-G-851` | `CAN-L3-G-310` | Capital Renovation Proposal to Infrastructure Change Governance | ACTIVE |
| `CAN-L3-G-852` | `CAN-L3-G-311` | Capital Renovation Completion to Updated Physical Infrastructure | ACTIVE |
| `CAN-L3-G-853` | `CAN-L3-G-312` | Sustainability Requirement to Infrastructure Sustainability Record | ACTIVE |
| `CAN-L3-G-854` | `CAN-L3-G-313` | Infrastructure Security Classification to Controlled Classification | ACTIVE |
| `CAN-L3-G-855` | `CAN-L3-G-314` | Disaster Recovery Infrastructure Requirement to Recovery Governance Record | ACTIVE |
| `CAN-L3-G-856` | `CAN-L3-G-315` | Infrastructure Recovery Condition to Controlled Recovery State | ACTIVE |
| `CAN-L3-G-857` | `CAN-L3-G-316` | Infrastructure Lifecycle Initiation to Active Infrastructure State | ACTIVE |
| `CAN-L3-G-858` | `CAN-L3-G-317` | Infrastructure Decommissioning Initiation to Historical Closure | ACTIVE |
| `CAN-L3-G-859` | `CAN-L3-G-318` | Infrastructure Archive Eligibility to Controlled Archival | ACTIVE |
| `CAN-L3-G-860` | `CAN-L3-G-319` | Infrastructure Configuration Change to Future Policy Availability | ACTIVE |
| `CAN-L3-G-861` | `CAN-L3-G-320` | Physical Campus Boundary Change to Institutional Context Update | ACTIVE |
| `CAN-L3-G-862` | `CAN-L3-G-321` | Infrastructure Correction to Audited Historical Amendment | ACTIVE |
| `CAN-L3-G-863` | `CAN-L3-G-322` | Infrastructure Portfolio to Governed Report and Dashboard Projection | ACTIVE |

### Section H — Institutional Services & Intelligence (81 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Varad.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-H-086` | `CAN-L3-H-001` | Verified Student to Government Learner Account Link | ACTIVE |
| `CAN-L3-H-087` | `CAN-L3-H-002` | Approved Credential to Government Publication Outcome | ACTIVE |
| `CAN-L3-H-088` | `CAN-L3-H-003` | Government Publication Consent Lifecycle | ACTIVE |
| `CAN-L3-H-089` | `CAN-L3-H-004` | Published Credential to Governed Revocation | ACTIVE |
| `CAN-L3-H-090` | `CAN-L3-H-005` | Corrected Credential to Controlled Republication | ACTIVE |
| `CAN-L3-H-091` | `CAN-L3-H-006` | Approved Credential Population to Bulk Publication Outcome | ACTIVE |
| `CAN-L3-H-092` | `CAN-L3-H-007` | Institutional and Government Credential Reconciliation | ACTIVE |
| `CAN-L3-H-093` | `CAN-L3-H-008` | Failed or Pending Government Publication to Controlled Recovery | ACTIVE |
| `CAN-L3-H-094` | `CAN-L3-H-009` | Government Connector to Governed Active or Retired State | ACTIVE |
| `CAN-L3-H-095` | `CAN-L3-H-010` | Authoritative Business Outcome to Governed Communication Outcome | ACTIVE |
| `CAN-L3-H-096` | `CAN-L3-H-011` | Notification Schedule to Delivered, Cancelled, or Expired Outcome | ACTIVE |
| `CAN-L3-H-097` | `CAN-L3-H-012` | Notification Template Request to Governed Published Template | ACTIVE |
| `CAN-L3-H-098` | `CAN-L3-H-013` | Recipient Preference or Subscription Change to Effective Delivery Policy | ACTIVE |
| `CAN-L3-H-099` | `CAN-L3-H-014` | Emergency Communication Trigger to Priority Delivery Outcome | ACTIVE |
| `CAN-L3-H-100` | `CAN-L3-H-015` | Failed Notification Delivery to Recovery or Terminal Failure | ACTIVE |
| `CAN-L3-H-101` | `CAN-L3-H-016` | Provider Receipt to Final Notification Evidence | ACTIVE |
| `CAN-L3-H-102` | `CAN-L3-H-017` | Entity Context to Related Enterprise Discovery | ACTIVE |
| `CAN-L3-H-103` | `CAN-L3-H-018` | Authoritative Source Change to Current Search Projection | ACTIVE |
| `CAN-L3-H-104` | `CAN-L3-H-019` | Enterprise Search Index Failure to Verified Recovery | ACTIVE |
| `CAN-L3-H-105` | `CAN-L3-H-020` | New Domain Projection to Governed Search Availability | ACTIVE |
| `CAN-L3-H-106` | `CAN-L3-H-021` | Enterprise Search Configuration Change to Effective Governed Behavior | ACTIVE |
| `CAN-L3-H-107` | `CAN-L3-H-022` | Employer Registration to Governed Placement Participation | ACTIVE |
| `CAN-L3-H-108` | `CAN-L3-H-023` | Placement Requirement to Published Opportunity and Closed Drive | ACTIVE |
| `CAN-L3-H-109` | `CAN-L3-H-024` | Eligible Student Application to Selection Decision | ACTIVE |
| `CAN-L3-H-110` | `CAN-L3-H-025` | Selected Candidate to Offer and Joining Outcome | ACTIVE |
| `CAN-L3-H-111` | `CAN-L3-H-026` | Internship Opportunity to Verified Completion | ACTIVE |
| `CAN-L3-H-112` | `CAN-L3-H-027` | Placement Disruption to Governed Cancellation, Withdrawal, or Reopening | ACTIVE |
| `CAN-L3-H-113` | `CAN-L3-H-028` | Completed Placement Cycle to Institutional Outcome Report | ACTIVE |
| `CAN-L3-H-114` | `CAN-L3-H-029` | Research Proposal Submission to Governance Decision | ACTIVE |
| `CAN-L3-H-115` | `CAN-L3-H-030` | Submitted Research Proposal to Governed Withdrawal | ACTIVE |
| `CAN-L3-H-116` | `CAN-L3-H-031` | Approved Research Proposal to Project Closure | ACTIVE |
| `CAN-L3-H-117` | `CAN-L3-H-032` | Grant Opportunity to Award and Funding Closure | ACTIVE |
| `CAN-L3-H-118` | `CAN-L3-H-033` | Research Activity to Ethics Decision and Continuing Compliance | ACTIVE |
| `CAN-L3-H-119` | `CAN-L3-H-034` | Research Output to Governed Publication and Archival | ACTIVE |
| `CAN-L3-H-120` | `CAN-L3-H-035` | Intellectual Property Disclosure to Protection or Closure Outcome | ACTIVE |
| `CAN-L3-H-121` | `CAN-L3-H-036` | Research Collaboration Request to Active or Closed Agreement | ACTIVE |
| `CAN-L3-H-122` | `CAN-L3-H-037` | Consultancy Request to Delivered and Financially Reconciled Outcome | ACTIVE |
| `CAN-L3-H-123` | `CAN-L3-H-038` | Research Portfolio Evidence to Institutional Governance Outcome | ACTIVE |
| `CAN-L3-H-124` | `CAN-L3-H-039` | Eligible Student Completion to Alumni Relationship | ACTIVE |
| `CAN-L3-H-125` | `CAN-L3-H-040` | Alumni Membership Request to Active or Closed Membership | ACTIVE |
| `CAN-L3-H-126` | `CAN-L3-H-041` | Alumni Profile or Preference Change to Current Governed Record | ACTIVE |
| `CAN-L3-H-127` | `CAN-L3-H-042` | Alumni Chapter Proposal to Active or Retired Chapter | ACTIVE |
| `CAN-L3-H-128` | `CAN-L3-H-043` | Alumni Connection Request to Verified Networking Relationship | ACTIVE |
| `CAN-L3-H-129` | `CAN-L3-H-044` | Mentorship Request to Completed Alumni Mentoring Engagement | ACTIVE |
| `CAN-L3-H-130` | `CAN-L3-H-045` | Alumni Contribution Offer to Completed Institutional Engagement | ACTIVE |
| `CAN-L3-H-131` | `CAN-L3-H-046` | Alumni Contribution Evidence to Recognition Outcome | ACTIVE |
| `CAN-L3-H-132` | `CAN-L3-H-047` | Alumni Donation Intent to Finance-Confirmed Contribution | ACTIVE |
| `CAN-L3-H-133` | `CAN-L3-H-048` | Dashboard Requirement to Governed Published Dashboard | ACTIVE |
| `CAN-L3-H-134` | `CAN-L3-H-049` | Dashboard Template Request to Effective Governed Template | ACTIVE |
| `CAN-L3-H-135` | `CAN-L3-H-050` | Role or Organizational Assignment to Available Dashboard Workspace | ACTIVE |
| `CAN-L3-H-136` | `CAN-L3-H-051` | Dashboard Sharing Request to Controlled Access Outcome | ACTIVE |
| `CAN-L3-H-137` | `CAN-L3-H-052` | Dashboard Change to Versioned Publication or Rollback | ACTIVE |
| `CAN-L3-H-138` | `CAN-L3-H-053` | Dashboard Retirement Decision to Archive or Restoration | ACTIVE |
| `CAN-L3-H-139` | `CAN-L3-H-054` | Widget Definition to Governed Availability or Retirement | ACTIVE |
| `CAN-L3-H-140` | `CAN-L3-H-055` | Regulatory Framework Change to Active Reporting Definition | ACTIVE |
| `CAN-L3-H-141` | `CAN-L3-H-056` | Reporting Cycle Initiation to Governed Collection Plan | ACTIVE |
| `CAN-L3-H-142` | `CAN-L3-H-057` | Source Data Collection to Validated Regulatory Snapshot | ACTIVE |
| `CAN-L3-H-143` | `CAN-L3-H-058` | Validated Regulatory Snapshot to Approved Submission Package | ACTIVE |
| `CAN-L3-H-144` | `CAN-L3-H-059` | Approved Regulatory Package to Submission Acknowledgement | ACTIVE |
| `CAN-L3-H-145` | `CAN-L3-H-060` | Regulatory Correction or Cycle Completion to Governed Closure | ACTIVE |
| `CAN-L3-H-146` | `CAN-L3-H-061` | Accreditation Framework Change to Active Governed Framework | ACTIVE |
| `CAN-L3-H-147` | `CAN-L3-H-062` | Accreditation Cycle Initiation to Approved Assessment Plan | ACTIVE |
| `CAN-L3-H-148` | `CAN-L3-H-063` | Compliance Obligation to Effective Requirement Lifecycle | ACTIVE |
| `CAN-L3-H-149` | `CAN-L3-H-064` | Accreditation Evidence Request to Approved Evidence Record | ACTIVE |
| `CAN-L3-H-150` | `CAN-L3-H-065` | Internal Audit Plan to Closed Audit Report | ACTIVE |
| `CAN-L3-H-151` | `CAN-L3-H-066` | External Assessment Notice to Accepted Assessment Outcome | ACTIVE |
| `CAN-L3-H-152` | `CAN-L3-H-067` | Compliance Finding to Verified CAPA Closure | ACTIVE |
| `CAN-L3-H-153` | `CAN-L3-H-068` | Quality Improvement Proposal to Verified Institutional Outcome | ACTIVE |
| `CAN-L3-H-154` | `CAN-L3-H-069` | Source Evidence to Institutional Accreditation Readiness Decision | ACTIVE |
| `CAN-L3-H-155` | `CAN-L3-H-070` | Periodic Compliance Review to Governed Compliance Status | ACTIVE |
| `CAN-L3-H-156` | `CAN-L3-H-071` | Approved Accreditation Evidence to Self-Study Report Approval and Handoff | ACTIVE |
| `CAN-L3-H-157` | `CAN-L3-H-072` | External Education Provider to Governed Active or Retired Relationship | ACTIVE |
| `CAN-L3-H-158` | `CAN-L3-H-073` | External Connector Configuration to Governed Activation or Retirement | ACTIVE |
| `CAN-L3-H-159` | `CAN-L3-H-074` | External Trust Requirement to Active or Revoked Authentication Relationship | ACTIVE |
| `CAN-L3-H-160` | `CAN-L3-H-075` | Synchronization Request to Verified Completion or Controlled Recovery | ACTIVE |
| `CAN-L3-H-161` | `CAN-L3-H-076` | External Enrollment Request to Confirmed or Rejected Enrollment Exchange | ACTIVE |
| `CAN-L3-H-162` | `CAN-L3-H-077` | External Learning Activity to Synchronized Progress Outcome | ACTIVE |
| `CAN-L3-H-163` | `CAN-L3-H-078` | Verified External Credit Request to Synchronized Academic Decision | ACTIVE |
| `CAN-L3-H-164` | `CAN-L3-H-079` | External Credential Verification Request to Verified or Rejected Outcome | ACTIVE |
| `CAN-L3-H-165` | `CAN-L3-H-080` | Integration Discrepancy to Reconciled External and Internal References | ACTIVE |
| `CAN-L3-H-166` | `CAN-L3-H-081` | Governed External Education Data Exchange to Accepted or Rejected Transfer | ACTIVE |

### Section I — Examination (197 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Devang.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-I-315` | `CAN-L3-I-001` | Examination Policy Definition to Institutional Activation | ACTIVE |
| `CAN-L3-I-316` | `CAN-L3-I-002` | Examination Policy Revision to Authoritative Superseding Policy | ACTIVE |
| `CAN-L3-I-317` | `CAN-L3-I-003` | Retire an Examination Policy with Historical Preservation | ACTIVE |
| `CAN-L3-I-318` | `CAN-L3-I-004` | Academic Examination Cycle to Governed Examination Session | ACTIVE |
| `CAN-L3-I-319` | `CAN-L3-I-005` | Examination Session Governance Revision to Preserved Downstream Continuity | ACTIVE |
| `CAN-L3-I-320` | `CAN-L3-I-006` | Duplicate Examination Session to Single Authoritative Session | ACTIVE |
| `CAN-L3-I-321` | `CAN-L3-I-007` | Plan and Approve an Examination Schedule | ACTIVE |
| `CAN-L3-I-322` | `CAN-L3-I-008` | Examination Plan Revision to Authoritative Updated Schedule | ACTIVE |
| `CAN-L3-I-323` | `CAN-L3-I-009` | Scheduled Examination to Allocated Examination Venues | ACTIVE |
| `CAN-L3-I-324` | `CAN-L3-I-010` | Examination Schedule to Assigned Invigilation | ACTIVE |
| `CAN-L3-I-325` | `CAN-L3-I-011` | Invigilator Shortage to Maintained Examination Schedule | ACTIVE |
| `CAN-L3-I-326` | `CAN-L3-I-012` | Practical or Viva Examination to Allocated Examiners | ACTIVE |
| `CAN-L3-I-327` | `CAN-L3-I-013` | Approved Examination Schedule to Published Official Timetable | ACTIVE |
| `CAN-L3-I-328` | `CAN-L3-I-014` | Detect and Resolve Examination Scheduling Conflicts | ACTIVE |
| `CAN-L3-I-329` | `CAN-L3-I-015` | Examination Venue Allocation to Authoritative Seating Arrangement | ACTIVE |
| `CAN-L3-I-330` | `CAN-L3-I-016` | Timetable Withdrawal to Notified Stakeholders | ACTIVE |
| `CAN-L3-I-331` | `CAN-L3-I-017` | Candidate Examination Registration to Confirmed Registration | ACTIVE |
| `CAN-L3-I-332` | `CAN-L3-I-018` | Institution-Assisted or Bulk Examination Registration | ACTIVE |
| `CAN-L3-I-333` | `CAN-L3-I-019` | Late Examination Registration | ACTIVE |
| `CAN-L3-I-334` | `CAN-L3-I-020` | Examination Registration Deadline Override | ACTIVE |
| `CAN-L3-I-335` | `CAN-L3-I-021` | Registration Amendment to Authoritative Corrected Registration | ACTIVE |
| `CAN-L3-I-336` | `CAN-L3-I-022` | Erroneous Examination Registration to Traceable Corrected Registration | ACTIVE |
| `CAN-L3-I-337` | `CAN-L3-I-023` | Registration Withdrawal to Withdrawn Participation Authority | ACTIVE |
| `CAN-L3-I-338` | `CAN-L3-I-024` | Withdrawn Registration Reinstatement to Authoritative Valid Registration | ACTIVE |
| `CAN-L3-I-339` | `CAN-L3-I-025` | Registration Reopening to Available Examination Registration | ACTIVE |
| `CAN-L3-I-340` | `CAN-L3-I-026` | Examination Registration Rejection or Ineligibility Outcome | ACTIVE |
| `CAN-L3-I-341` | `CAN-L3-I-027` | Examination Registration Scope Change Following Academic Registration Change | ACTIVE |
| `CAN-L3-I-342` | `CAN-L3-I-028` | Examination Registration Under Approved Special Permission | ACTIVE |
| `CAN-L3-I-343` | `CAN-L3-I-029` | Examination Registration for a Candidate with Approved Special Examination Arrangements | ACTIVE |
| `CAN-L3-I-344` | `CAN-L3-I-030` | Examination Registration for a Supplementary, Backlog or Improvement Opportunity | ACTIVE |
| `CAN-L3-I-345` | `CAN-L3-I-031` | Examination Registration Withdrawal Due to Examination Opportunity Change | ACTIVE |
| `CAN-L3-I-346` | `CAN-L3-I-032` | Duplicate Examination Registration to Single Valid Registration | ACTIVE |
| `CAN-L3-I-347` | `CAN-L3-I-033` | Conflicting Registration Scope to Reconciled Registration Position | ACTIVE |
| `CAN-L3-I-348` | `CAN-L3-I-034` | Administrative Examination Registration on Behalf of a Candidate | ACTIVE |
| `CAN-L3-I-349` | `CAN-L3-I-035` | Examination Registration Transfer Following Authorized Examination Context Change | ACTIVE |
| `CAN-L3-I-350` | `CAN-L3-I-036` | Finalize Examination Registration Population for Downstream Examination Processing | ACTIVE |
| `CAN-L3-I-351` | `CAN-L3-I-037` | Hall Ticket Issuance to Eligible Registered Candidates | ACTIVE |
| `CAN-L3-I-352` | `CAN-L3-I-038` | Unsatisfied Hall Ticket Conditions to Withheld Entry Authorization | ACTIVE |
| `CAN-L3-I-353` | `CAN-L3-I-039` | Invalidate a Hall Ticket Following Examination Registration Withdrawal | ACTIVE |
| `CAN-L3-I-354` | `CAN-L3-I-040` | Approved Special Arrangement to Valid Hall Ticket | ACTIVE |
| `CAN-L3-I-355` | `CAN-L3-I-041` | Duplicate or Conflicting Hall Ticket to Single Valid Hall Ticket | ACTIVE |
| `CAN-L3-I-356` | `CAN-L3-I-042` | Finalize Hall Ticket Population for Examination Conduct | ACTIVE |
| `CAN-L3-I-357` | `CAN-L3-I-043` | Scheduled Non-Theory Assessment to Completed Assessment Record | ACTIVE |
| `CAN-L3-I-358` | `CAN-L3-I-044` | Approved Special Arrangement to Completed Non-Theory Assessment | ACTIVE |
| `CAN-L3-I-359` | `CAN-L3-I-045` | Failed Non-Theory Entry Condition to Governed Non-Participation Record | ACTIVE |
| `CAN-L3-I-360` | `CAN-L3-I-046` | Non-Theory Entry Identity Discrepancy to Permit-or-Deny Participation Outcome | ACTIVE |
| `CAN-L3-I-361` | `CAN-L3-I-047` | Authorized Non-Theory Re-Conduct Decision to Completed Re-Examination | ACTIVE |
| `CAN-L3-I-362` | `CAN-L3-I-048` | Practical Resource Unavailability to Valid Examination Arrangement | ACTIVE |
| `CAN-L3-I-363` | `CAN-L3-I-049` | Practical Batch Allocation Conflict to Valid Allocation | ACTIVE |
| `CAN-L3-I-364` | `CAN-L3-I-050` | Panel Viva Examination to Completed Panel Conduct Outcome | ACTIVE |
| `CAN-L3-I-365` | `CAN-L3-I-051` | Mid-Conduct Examiner Replacement to Governed Examination Continuity | ACTIVE |
| `CAN-L3-I-366` | `CAN-L3-I-052` | Candidate Medical Interruption During Non-Theory Conduct to Authoritative Conduct Outcome | ACTIVE |
| `CAN-L3-I-367` | `CAN-L3-I-053` | Candidate Withdrawal During Non-Theory Conduct to Recorded Participation Outcome | ACTIVE |
| `CAN-L3-I-368` | `CAN-L3-I-054` | Non-Theory Conduct Record Error to Traceable Corrected Record | ACTIVE |
| `CAN-L3-I-369` | `CAN-L3-I-055` | Scheduled Theory Examination to Completed Conduct Record | ACTIVE |
| `CAN-L3-I-370` | `CAN-L3-I-056` | Multi-Centre Theory Examination to Consistent Governed Conduct Outcome | ACTIVE |
| `CAN-L3-I-371` | `CAN-L3-I-057` | Failed Theory Entry Condition to Governed Non-Participation Record | ACTIVE |
| `CAN-L3-I-372` | `CAN-L3-I-058` | Theory Entry Identity Discrepancy to Permit-or-Deny Participation Outcome | ACTIVE |
| `CAN-L3-I-373` | `CAN-L3-I-059` | Theory Seating Conflict to Valid Seating Outcome | ACTIVE |
| `CAN-L3-I-374` | `CAN-L3-I-060` | Mid-Conduct Invigilator Replacement to Governed Examination Continuity | ACTIVE |
| `CAN-L3-I-375` | `CAN-L3-I-061` | Authorized Theory Re-Conduct Decision to Completed Re-Examination | ACTIVE |
| `CAN-L3-I-376` | `CAN-L3-I-062` | Approved Special Arrangement to Completed Theory Examination | ACTIVE |
| `CAN-L3-I-377` | `CAN-L3-I-063` | Question Paper Unavailability to Valid Examination Start Arrangement | ACTIVE |
| `CAN-L3-I-378` | `CAN-L3-I-064` | Compromised Question Paper to Authoritative Replacement or Exception Outcome | ACTIVE |
| `CAN-L3-I-379` | `CAN-L3-I-065` | Question Paper Issue During Conduct to Governed Examination Disposition | ACTIVE |
| `CAN-L3-I-380` | `CAN-L3-I-066` | Missing Answer Script at Collection to Conclusive Custody Resolution | ACTIVE |
| `CAN-L3-I-381` | `CAN-L3-I-067` | Duplicate or Misidentified Answer Script to Authoritative Script Record | ACTIVE |
| `CAN-L3-I-382` | `CAN-L3-I-068` | Candidate Medical Interruption During Theory Conduct to Authoritative Conduct Outcome | ACTIVE |
| `CAN-L3-I-383` | `CAN-L3-I-069` | Candidate Withdrawal During Theory Conduct to Recorded Participation Outcome | ACTIVE |
| `CAN-L3-I-384` | `CAN-L3-I-070` | Theory Conduct Record Error to Traceable Corrected Record | ACTIVE |
| `CAN-L3-I-385` | `CAN-L3-I-071` | Marks Collection for Completed Examination Activities | ACTIVE |
| `CAN-L3-I-386` | `CAN-L3-I-072` | Examination Evaluation to Final Approved Marks | ACTIVE |
| `CAN-L3-I-387` | `CAN-L3-I-073` | Panel Evaluation to Authoritative Evaluation Outcome | ACTIVE |
| `CAN-L3-I-388` | `CAN-L3-I-074` | Marks Submission Exception to Authoritative Marks Outcome | ACTIVE |
| `CAN-L3-I-389` | `CAN-L3-I-075` | Moderate Examination Marks Under Approved Policy | ACTIVE |
| `CAN-L3-I-390` | `CAN-L3-I-076` | Evaluation Discrepancy Before Approval to Valid Evaluated Marks | ACTIVE |
| `CAN-L3-I-391` | `CAN-L3-I-077` | Incomplete Evaluation to Withheld Marks Approval | ACTIVE |
| `CAN-L3-I-392` | `CAN-L3-I-078` | Supplementary or Improvement Examination to Authoritative Evaluated Marks | ACTIVE |
| `CAN-L3-I-393` | `CAN-L3-I-079` | Re-Conducted Examination to Completed Evaluation | ACTIVE |
| `CAN-L3-I-394` | `CAN-L3-I-080` | Approved Special Arrangement to Authoritative Evaluated Marks | ACTIVE |
| `CAN-L3-I-395` | `CAN-L3-I-081` | Unavailable Theory Answer Script to Resolved Evaluation Outcome | ACTIVE |
| `CAN-L3-I-396` | `CAN-L3-I-082` | Damaged Evaluation Evidence to Authoritative Evaluation Resolution | ACTIVE |
| `CAN-L3-I-397` | `CAN-L3-I-083` | Candidate-to-Script Mapping Error to Correct Evaluation Record | ACTIVE |
| `CAN-L3-I-398` | `CAN-L3-I-084` | Policy-Required Second Evaluation to Incorporated Evaluation Outcome | ACTIVE |
| `CAN-L3-I-399` | `CAN-L3-I-085` | First and Second Evaluation Difference to Authoritative Evaluated Marks | ACTIVE |
| `CAN-L3-I-400` | `CAN-L3-I-086` | Policy-Required Adjudicating Evaluation to Final Evaluated Marks | ACTIVE |
| `CAN-L3-I-401` | `CAN-L3-I-087` | Evaluation Process Interruption to Resumed Custody-Preserving Evaluation | ACTIVE |
| `CAN-L3-I-402` | `CAN-L3-I-088` | Final Approved Marks to Institutionally Approved Result | ACTIVE |
| `CAN-L3-I-403` | `CAN-L3-I-089` | Re-Conducted Examination to Authoritative Computed Result | ACTIVE |
| `CAN-L3-I-404` | `CAN-L3-I-090` | Unsatisfied Result Conditions to Withheld Result Computation | ACTIVE |
| `CAN-L3-I-405` | `CAN-L3-I-091` | Blocking Condition Resolution to Resumed Result Computation | ACTIVE |
| `CAN-L3-I-406` | `CAN-L3-I-092` | Recompute Result Before Institutional Approval | ACTIVE |
| `CAN-L3-I-407` | `CAN-L3-I-093` | Approved Marks Correction to Recomputed Authoritative Result | ACTIVE |
| `CAN-L3-I-408` | `CAN-L3-I-094` | Approved Moderation Outcome to Moderated Computed Result | ACTIVE |
| `CAN-L3-I-409` | `CAN-L3-I-095` | Approved Grace or Conditional Policy to Policy-Governed Computed Result | ACTIVE |
| `CAN-L3-I-410` | `CAN-L3-I-096` | Approved Special Arrangement to Authoritative Computed Result | ACTIVE |
| `CAN-L3-I-411` | `CAN-L3-I-097` | Result Processing Identity Mismatch to Correct Candidate Association | ACTIVE |
| `CAN-L3-I-412` | `CAN-L3-I-098` | Duplicate Result Computation to Single Authoritative Result | ACTIVE |
| `CAN-L3-I-413` | `CAN-L3-I-099` | Conflicting Result Outcomes to Single Valid Computed Result | ACTIVE |
| `CAN-L3-I-414` | `CAN-L3-I-100` | Malpractice Decision to Consequence-Applied Result Processing | ACTIVE |
| `CAN-L3-I-415` | `CAN-L3-I-101` | Examination Exception Resolution to Post-Resolution Computed Result | ACTIVE |
| `CAN-L3-I-416` | `CAN-L3-I-102` | Approval Blocking Condition to Withheld Individual Result | ACTIVE |
| `CAN-L3-I-417` | `CAN-L3-I-103` | Withheld Result Release to Approval-Eligible Result | ACTIVE |
| `CAN-L3-I-418` | `CAN-L3-I-104` | Result Processing Reopening to Corrected Approved Result | ACTIVE |
| `CAN-L3-I-419` | `CAN-L3-I-105` | Pre-Publication Result Replacement to Traceable Corrected Approved Result | ACTIVE |
| `CAN-L3-I-420` | `CAN-L3-I-106` | Publish Official Examination Results | ACTIVE |
| `CAN-L3-I-421` | `CAN-L3-I-107` | Publication Blocking Condition to Withheld Approved Result | ACTIVE |
| `CAN-L3-I-422` | `CAN-L3-I-108` | Withheld Result Release to Published Official Result | ACTIVE |
| `CAN-L3-I-423` | `CAN-L3-I-109` | Delay Result Publication for an Examination Scope | ACTIVE |
| `CAN-L3-I-424` | `CAN-L3-I-110` | Publication Delay Resolution to Published Official Result | ACTIVE |
| `CAN-L3-I-425` | `CAN-L3-I-111` | Pending Publication Cancellation to Conclusively Cancelled Publication | ACTIVE |
| `CAN-L3-I-426` | `CAN-L3-I-112` | Authorized Post-Publication Change to Replaced Official Result | ACTIVE |
| `CAN-L3-I-427` | `CAN-L3-I-113` | Republish Corrected Examination Result | ACTIVE |
| `CAN-L3-I-428` | `CAN-L3-I-114` | Authorized Withdrawal Decision to Withdrawn Published Result | ACTIVE |
| `CAN-L3-I-429` | `CAN-L3-I-115` | Withdrawn Result Restoration to Active Published Result | ACTIVE |
| `CAN-L3-I-430` | `CAN-L3-I-116` | Revaluation or Rechecking Outcome to Published Revised Result | ACTIVE |
| `CAN-L3-I-431` | `CAN-L3-I-117` | Malpractice Decision to Published Consequence-Bearing Result | ACTIVE |
| `CAN-L3-I-432` | `CAN-L3-I-118` | Examination Exception Resolution to Published Official Result | ACTIVE |
| `CAN-L3-I-433` | `CAN-L3-I-119` | Governed Absence Consequence to Published Official Result | ACTIVE |
| `CAN-L3-I-434` | `CAN-L3-I-120` | Incomplete Examination Outcome to Policy-Governed Published Result | ACTIVE |
| `CAN-L3-I-435` | `CAN-L3-I-121` | Result Publication Failure to Safe Publication Recovery | ACTIVE |
| `CAN-L3-I-436` | `CAN-L3-I-122` | Published Result Discrepancy to Corrected Official Publication | ACTIVE |
| `CAN-L3-I-437` | `CAN-L3-I-123` | Revaluation Request to Revised Marks Outcome | ACTIVE |
| `CAN-L3-I-438` | `CAN-L3-I-124` | Rechecking Request to Verified Marks Outcome | ACTIVE |
| `CAN-L3-I-439` | `CAN-L3-I-125` | Revaluation Request Withdrawal to Closed Revaluation Request | ACTIVE |
| `CAN-L3-I-440` | `CAN-L3-I-126` | Rechecking Request Withdrawal to Closed Rechecking Request | ACTIVE |
| `CAN-L3-I-441` | `CAN-L3-I-127` | Policy-Permitted Practical Revaluation to Authoritative Revaluation Outcome | ACTIVE |
| `CAN-L3-I-442` | `CAN-L3-I-128` | Policy-Permitted Practical Rechecking to Authoritative Rechecking Outcome | ACTIVE |
| `CAN-L3-I-443` | `CAN-L3-I-129` | Revaluation Evaluator Conflict of Interest to Governed Replacement Evaluation | ACTIVE |
| `CAN-L3-I-444` | `CAN-L3-I-130` | Policy-Required Additional Revaluation to Incorporated Marks Outcome | ACTIVE |
| `CAN-L3-I-445` | `CAN-L3-I-131` | Persistent Revaluation Difference to Adjudicated Revised Marks | ACTIVE |
| `CAN-L3-I-446` | `CAN-L3-I-132` | Revaluation Process Interruption to Conclusive Revaluation Outcome | ACTIVE |
| `CAN-L3-I-447` | `CAN-L3-I-133` | Duplicate Revaluation Request to Single Authoritative Request | ACTIVE |
| `CAN-L3-I-448` | `CAN-L3-I-134` | Duplicate Rechecking Request to Single Authoritative Request | ACTIVE |
| `CAN-L3-I-449` | `CAN-L3-I-135` | Conflicting Revaluation and Rechecking Requests to Valid Review Path | ACTIVE |
| `CAN-L3-I-450` | `CAN-L3-I-136` | Revaluation Reopening to Valid Authoritative Outcome | ACTIVE |
| `CAN-L3-I-451` | `CAN-L3-I-137` | Finalized Revaluation Correction to Traceable Authoritative Outcome | ACTIVE |
| `CAN-L3-I-452` | `CAN-L3-I-138` | Subsequent Examination Application, Eligibility and Attempt Authorization | ACTIVE |
| `CAN-L3-I-453` | `CAN-L3-I-139` | Subsequent Examination Opportunity to Authoritative Examination Schedule | ACTIVE |
| `CAN-L3-I-454` | `CAN-L3-I-140` | Subsequent Examination Schedule Revision to Authoritative Revised Schedule | ACTIVE |
| `CAN-L3-I-455` | `CAN-L3-I-141` | Reschedule Supplementary, Backlog, or Improvement Examination Following Authorized Disruption | ACTIVE |
| `CAN-L3-I-456` | `CAN-L3-I-142` | Re-Conduct Supplementary, Backlog, or Improvement Examination Following Authorized Decision | ACTIVE |
| `CAN-L3-I-457` | `CAN-L3-I-143` | Duplicate Subsequent Examination Attempt to Valid Attempt Sequence | ACTIVE |
| `CAN-L3-I-458` | `CAN-L3-I-144` | Conflicting Subsequent Examination Results to Single Valid Result Position | ACTIVE |
| `CAN-L3-I-459` | `CAN-L3-I-145` | Pre-Conduct Opportunity Cancellation to Routed Affected Candidates | ACTIVE |
| `CAN-L3-I-460` | `CAN-L3-I-146` | Cancelled Opportunity Restoration to Governed Examination Opportunity | ACTIVE |
| `CAN-L3-I-461` | `CAN-L3-I-147` | Exhausted Examination Attempts to Conclusive Eligibility Determination | ACTIVE |
| `CAN-L3-I-462` | `CAN-L3-I-148` | Attempt Policy Change to Authoritative Candidate Eligibility | ACTIVE |
| `CAN-L3-I-463` | `CAN-L3-I-149` | Subsequent Examination Execution Coordination | ACTIVE |
| `CAN-L3-I-464` | `CAN-L3-I-150` | Completion Evaluation to Authoritative Completion Eligibility | ACTIVE |
| `CAN-L3-I-465` | `CAN-L3-I-151` | Incomplete Academic Record to Resolved or Blocked Completion Position | ACTIVE |
| `CAN-L3-I-466` | `CAN-L3-I-152` | Conflicting Completion Inputs to Valid Completion Basis | ACTIVE |
| `CAN-L3-I-467` | `CAN-L3-I-153` | Completion Identity Mismatch to Correct Student Context Association | ACTIVE |
| `CAN-L3-I-468` | `CAN-L3-I-154` | Duplicate Completion Evaluation to Single Authoritative Evaluation | ACTIVE |
| `CAN-L3-I-469` | `CAN-L3-I-155` | Revised Result to Re-Evaluated Completion Eligibility | ACTIVE |
| `CAN-L3-I-470` | `CAN-L3-I-156` | Malpractice Outcome to Re-Evaluated Completion Position | ACTIVE |
| `CAN-L3-I-471` | `CAN-L3-I-157` | Examination Exception Resolution to Re-Evaluated Completion Position | ACTIVE |
| `CAN-L3-I-472` | `CAN-L3-I-158` | Academic Completion to Governed Completion Evidence | ACTIVE |
| `CAN-L3-I-473` | `CAN-L3-I-159` | Authorized Provisional Completion to Provisional Completion Evidence | ACTIVE |
| `CAN-L3-I-474` | `CAN-L3-I-160` | Pre-Release Evidence Correction to Authoritative Completion Evidence | ACTIVE |
| `CAN-L3-I-475` | `CAN-L3-I-161` | Completion Evidence Reissue to Replacement Evidence with Unchanged Completion | ACTIVE |
| `CAN-L3-I-476` | `CAN-L3-I-162` | Authorized Result Change to Withdrawn Invalid Completion Evidence | ACTIVE |
| `CAN-L3-I-477` | `CAN-L3-I-163` | Completion Evidence Resolution to Restored Valid Evidence | ACTIVE |
| `CAN-L3-I-478` | `CAN-L3-I-164` | Duplicate Completion Evidence to Single Authoritative Evidence Position | ACTIVE |
| `CAN-L3-I-479` | `CAN-L3-I-165` | Completion Evidence Identity Mismatch to Correct Candidate Association | ACTIVE |
| `CAN-L3-I-480` | `CAN-L3-I-166` | Upstream Examination Dependency to Blocked Academic Completion | ACTIVE |
| `CAN-L3-I-481` | `CAN-L3-I-167` | Authorized Malpractice Case Review to Formally Reopened Case | ACTIVE |
| `CAN-L3-I-482` | `CAN-L3-I-168` | Review Malpractice Decision on Authorized Appeal | ACTIVE |
| `CAN-L3-I-483` | `CAN-L3-I-169` | Uphold Malpractice Decision After Review | ACTIVE |
| `CAN-L3-I-484` | `CAN-L3-I-170` | Modify Malpractice Consequence After Review | ACTIVE |
| `CAN-L3-I-485` | `CAN-L3-I-171` | Overturn Malpractice Finding After Review | ACTIVE |
| `CAN-L3-I-486` | `CAN-L3-I-172` | Overturned Malpractice Decision to Restored Examination Outcome | ACTIVE |
| `CAN-L3-I-487` | `CAN-L3-I-173` | Duplicate Malpractice Case to Single Authoritative Case | ACTIVE |
| `CAN-L3-I-488` | `CAN-L3-I-174` | Conflicting Malpractice Decisions to Single Authoritative Decision | ACTIVE |
| `CAN-L3-I-489` | `CAN-L3-I-175` | Suspected Examination Malpractice to Adjudicated Penalty Determination | ACTIVE |
| `CAN-L3-I-490` | `CAN-L3-I-176` | Malpractice Case Closure and Archival | ACTIVE |
| `CAN-L3-I-491` | `CAN-L3-I-177` | Examination Exception to Governed Exception Case | ACTIVE |
| `CAN-L3-I-492` | `CAN-L3-I-178` | Classify an Examination Exception and Assess Operational Impact | ACTIVE |
| `CAN-L3-I-493` | `CAN-L3-I-179` | Authorized Contingency to Suspended Examination Conduct | ACTIVE |
| `CAN-L3-I-494` | `CAN-L3-I-180` | Temporary Interruption Resolution to Resumed Examination Conduct | ACTIVE |
| `CAN-L3-I-495` | `CAN-L3-I-181` | Terminate an In-Progress Examination Following Major Contingency | ACTIVE |
| `CAN-L3-I-496` | `CAN-L3-I-182` | Authorize Re-Conduct Following Invalidated Examination | ACTIVE |
| `CAN-L3-I-497` | `CAN-L3-I-183` | Contingency Re-Examination to Valid Examination Occurrence | ACTIVE |
| `CAN-L3-I-498` | `CAN-L3-I-184` | Communicate Examination Contingency Decision to Affected Stakeholders | ACTIVE |
| `CAN-L3-I-499` | `CAN-L3-I-185` | Unresolved Examination Exception to Higher Institutional Decision Authority | ACTIVE |
| `CAN-L3-I-500` | `CAN-L3-I-186` | Conflicting Exception Decisions to Single Authoritative Exception Outcome | ACTIVE |
| `CAN-L3-I-501` | `CAN-L3-I-187` | Pre-Resolution Exception Reopening to Updated Authoritative Facts | ACTIVE |
| `CAN-L3-I-502` | `CAN-L3-I-188` | Authorized Review of Finalized Exception to New Authoritative Outcome | ACTIVE |
| `CAN-L3-I-503` | `CAN-L3-I-189` | Examination Exception Case Resolution to Closure | ACTIVE |
| `CAN-L3-I-504` | `CAN-L3-I-190` | Postpone and Reschedule an Examination Under Authorized Contingency | ACTIVE |
| `CAN-L3-I-505` | `CAN-L3-I-191` | Prepare and Authorize an Examination Contingency Plan | ACTIVE |
| `CAN-L3-I-506` | `CAN-L3-I-192` | Examination Session Closure and Finalization | ACTIVE |
| `CAN-L3-I-507` | `CAN-L3-I-193` | Archived Record Retrieval to Authorized Record Availability | ACTIVE |
| `CAN-L3-I-508` | `CAN-L3-I-194` | Examination Record Retention Governance to Applied Retention Classification | ACTIVE |
| `CAN-L3-I-509` | `CAN-L3-I-195` | Authorized Disposal of Archived Examination Records After Retention Expiry | ACTIVE |
| `CAN-L3-I-510` | `CAN-L3-I-196` | Legal Hold Application to Disposal-Excluded Examination Records | ACTIVE |
| `CAN-L3-I-511` | `CAN-L3-I-197` | Examination Record Archival | ACTIVE |

### Section J — Enterprise Resource Management (110 candidates)

Source: `01_Phase1_Discovery/L3_Phase1_Inventory_Dhruv.md`

| Old Candidate ID | New Candidate ID | Scenario Title | Status |
|---|---|---|---|
| `CAN-L3-J-682` | `CAN-L3-J-001` | Procurement Planning Requirement to Approved Active Procurement Plan | ACTIVE |
| `CAN-L3-J-683` | `CAN-L3-J-002` | Approved Procurement Plan Change to Governed Successor Plan | ACTIVE |
| `CAN-L3-J-684` | `CAN-L3-J-003` | Active Procurement Plan to Governed Closure | ACTIVE |
| `CAN-L3-J-685` | `CAN-L3-J-004` | Approved Procurement Demand to Competitive Quotation or RFQ Sourcing Outcome | ACTIVE |
| `CAN-L3-J-686` | `CAN-L3-J-005` | Approved Procurement Demand to RFP Sourcing Outcome | ACTIVE |
| `CAN-L3-J-687` | `CAN-L3-J-006` | Approved Procurement Demand to Governed Tender Sourcing Outcome | ACTIVE |
| `CAN-L3-J-688` | `CAN-L3-J-007` | Approved Procurement Demand to Direct or Simplified Sourcing Outcome | ACTIVE |
| `CAN-L3-J-689` | `CAN-L3-J-008` | Consolidated Approved Demand to Traceable Sourcing Outcome | ACTIVE |
| `CAN-L3-J-690` | `CAN-L3-J-009` | Split Approved Demand to Traceable Sourcing Outcomes | ACTIVE |
| `CAN-L3-J-691` | `CAN-L3-J-010` | Evaluated Procurement Demand to Approved Multi-Supplier Award | ACTIVE |
| `CAN-L3-J-692` | `CAN-L3-J-011` | Rate Contract Requirement to Approved Active Rate Contract | ACTIVE |
| `CAN-L3-J-693` | `CAN-L3-J-012` | Approved Demand to Rate-Contract Sourcing Outcome | ACTIVE |
| `CAN-L3-J-694` | `CAN-L3-J-013` | Expiring Rate Contract to Approved Successor Arrangement | ACTIVE |
| `CAN-L3-J-695` | `CAN-L3-J-014` | Active Rate Contract to Governed Suspension or Closure | ACTIVE |
| `CAN-L3-J-696` | `CAN-L3-J-015` | Approved Demand to Governed Single-Source or Proprietary Sourcing Outcome | ACTIVE |
| `CAN-L3-J-697` | `CAN-L3-J-016` | Urgent Approved Requirement to Governed Emergency Procurement Outcome | ACTIVE |
| `CAN-L3-J-698` | `CAN-L3-J-017` | External Procurement Requirement to Institutionally Recognised Sourcing Outcome | ACTIVE |
| `CAN-L3-J-699` | `CAN-L3-J-018` | Insufficient Competition to Governed Sourcing Disposition | ACTIVE |
| `CAN-L3-J-700` | `CAN-L3-J-019` | Active Sourcing Event to Governed Cancellation | ACTIVE |
| `CAN-L3-J-701` | `CAN-L3-J-020` | Prospective Supplier to Canonical Registered Supplier | ACTIVE |
| `CAN-L3-J-702` | `CAN-L3-J-021` | Registered Supplier to Approved Active Supplier | ACTIVE |
| `CAN-L3-J-703` | `CAN-L3-J-022` | Approved Supplier to Qualified Institutional Eligibility | ACTIVE |
| `CAN-L3-J-704` | `CAN-L3-J-023` | Expiring Supplier Evidence or Qualification to Renewed Eligibility | ACTIVE |
| `CAN-L3-J-705` | `CAN-L3-J-024` | Expired Supplier Qualification to Governed Eligibility Restriction | ACTIVE |
| `CAN-L3-J-706` | `CAN-L3-J-025` | Supplier Classification Requirement to Governed Category Eligibility | ACTIVE |
| `CAN-L3-J-707` | `CAN-L3-J-026` | Supplier Profile Change to Governed Current Profile | ACTIVE |
| `CAN-L3-J-708` | `CAN-L3-J-027` | Sensitive Supplier Profile Change to Approved Authoritative Profile | ACTIVE |
| `CAN-L3-J-709` | `CAN-L3-J-028` | Supplier Performance or Incident Evidence to Governed Supplier Assessment Record | ACTIVE |
| `CAN-L3-J-710` | `CAN-L3-J-029` | Supplier Risk Finding to Governed Restriction | ACTIVE |
| `CAN-L3-J-711` | `CAN-L3-J-030` | Supplier Governance Finding to Controlled Suspension | ACTIVE |
| `CAN-L3-J-712` | `CAN-L3-J-031` | Severe Supplier Governance Finding to Blacklisted Status | ACTIVE |
| `CAN-L3-J-713` | `CAN-L3-J-032` | Supplier Remediation to Restored Eligibility | ACTIVE |
| `CAN-L3-J-714` | `CAN-L3-J-033` | Active Supplier Relationship to Governed Inactivity | ACTIVE |
| `CAN-L3-J-715` | `CAN-L3-J-034` | Supplier Relationship Completion to Governed Closure and Archival | ACTIVE |
| `CAN-L3-J-716` | `CAN-L3-J-035` | Duplicate Supplier Records to Canonical Identity Reconciliation | ACTIVE |
| `CAN-L3-J-717` | `CAN-L3-J-036` | External Supplier Reference to Canonical Supplier Mapping | ACTIVE |
| `CAN-L3-J-718` | `CAN-L3-J-037` | Supplier Representative Requirement to Governed Representation Lifecycle | ACTIVE |
| `CAN-L3-J-719` | `CAN-L3-J-038` | Institutional Demand to Approved Purchase Requisition Release | ACTIVE |
| `CAN-L3-J-720` | `CAN-L3-J-039` | Submitted Purchase Requisition to Governed Rejection | ACTIVE |
| `CAN-L3-J-721` | `CAN-L3-J-040` | Active Purchase Requisition to Governed Amended Demand | ACTIVE |
| `CAN-L3-J-722` | `CAN-L3-J-041` | Purchase Requisition Withdrawal or Cancellation to Governed Termination | ACTIVE |
| `CAN-L3-J-723` | `CAN-L3-J-042` | Emergency Institutional Requirement to Governed Purchase Requisition Release | ACTIVE |
| `CAN-L3-J-724` | `CAN-L3-J-043` | Released Procurement Authority to Issued Purchase Order | ACTIVE |
| `CAN-L3-J-725` | `CAN-L3-J-044` | Issued Purchase Order to Recorded Supplier Acknowledgement Outcome | ACTIVE |
| `CAN-L3-J-726` | `CAN-L3-J-045` | Issued Purchase Order Change to Governed Amended Commitment | ACTIVE |
| `CAN-L3-J-727` | `CAN-L3-J-046` | Issued or Partially Fulfilled Purchase Order to Governed Cancellation | ACTIVE |
| `CAN-L3-J-728` | `CAN-L3-J-047` | Fulfilled Purchase Order to Governed Closure | ACTIVE |
| `CAN-L3-J-729` | `CAN-L3-J-048` | Partially or Unfulfilled Purchase Order to Exceptional Closure | ACTIVE |
| `CAN-L3-J-730` | `CAN-L3-J-049` | Supplier Delivery to Governed Receipt, Inspection and Disposition | ACTIVE |
| `CAN-L3-J-731` | `CAN-L3-J-050` | Finalised Receipt Error to Governed Corrective Outcome | ACTIVE |
| `CAN-L3-J-732` | `CAN-L3-J-051` | Item Requirement to Governed Enterprise Item Master Record | ACTIVE |
| `CAN-L3-J-733` | `CAN-L3-J-052` | Inactive Item Requirement to Governed Item Archival | ACTIVE |
| `CAN-L3-J-734` | `CAN-L3-J-053` | Storage Requirement to Governed Warehouse and Location Structure | ACTIVE |
| `CAN-L3-J-735` | `CAN-L3-J-054` | Accepted Goods to Canonical Traceable Inventory Recognition | ACTIVE |
| `CAN-L3-J-736` | `CAN-L3-J-055` | Expiry-Controlled Inventory to Governed Availability Outcome | ACTIVE |
| `CAN-L3-J-737` | `CAN-L3-J-056` | Controlled Resource Stock to Policy-Governed Inventory Availability | ACTIVE |
| `CAN-L3-J-738` | `CAN-L3-J-057` | Authorised Inventory Activity to Current Stock Availability | ACTIVE |
| `CAN-L3-J-739` | `CAN-L3-J-058` | Completed Stock Movement to Canonical Ledger Consequence | ACTIVE |
| `CAN-L3-J-740` | `CAN-L3-J-059` | Approved Inventory Correction to Canonical Stock Adjustment | ACTIVE |
| `CAN-L3-J-741` | `CAN-L3-J-060` | Verified Inventory Discrepancy to Canonical Adjustment Outcome | ACTIVE |
| `CAN-L3-J-742` | `CAN-L3-J-061` | Reorder Policy or Stock Condition to Governed Replenishment Need | ACTIVE |
| `CAN-L3-J-743` | `CAN-L3-J-062` | Inventory Activity to Reconciled Valuation Reference | ACTIVE |
| `CAN-L3-J-744` | `CAN-L3-J-063` | Completed Disposal or Write-Off to Canonical Inventory Consequence | ACTIVE |
| `CAN-L3-J-745` | `CAN-L3-J-064` | Approved Stock Requirement to Completed Stock Issue | ACTIVE |
| `CAN-L3-J-746` | `CAN-L3-J-065` | Approved Stock Requirement to Governed Reservation Outcome | ACTIVE |
| `CAN-L3-J-747` | `CAN-L3-J-066` | Returned Stock to Accepted Institutional Custody | ACTIVE |
| `CAN-L3-J-748` | `CAN-L3-J-067` | Authorised Stock Transfer to Completed Destination Receipt | ACTIVE |
| `CAN-L3-J-749` | `CAN-L3-J-068` | Approved Distribution Requirement to Completed Recipient Distribution | ACTIVE |
| `CAN-L3-J-750` | `CAN-L3-J-069` | Issued Stock Use to Governed Consumption Outcome | ACTIVE |
| `CAN-L3-J-751` | `CAN-L3-J-070` | Custody Handover to Confirmed Responsibility Transfer | ACTIVE |
| `CAN-L3-J-752` | `CAN-L3-J-071` | Stock Movement Exception to Governed Resolution or Referral | ACTIVE |
| `CAN-L3-J-753` | `CAN-L3-J-072` | Asset-Linked Stock Movement to Reconciled Custody Handoff | ACTIVE |
| `CAN-L3-J-754` | `CAN-L3-J-073` | Authorised Stock Movement Cancellation or Reversal | ACTIVE |
| `CAN-L3-J-755` | `CAN-L3-J-074` | Asset Candidate to Registered Operational Asset | ACTIVE |
| `CAN-L3-J-756` | `CAN-L3-J-075` | Available Asset to Governed Assignment and Custody | ACTIVE |
| `CAN-L3-J-757` | `CAN-L3-J-076` | Assigned Asset to Governed Return Outcome | ACTIVE |
| `CAN-L3-J-758` | `CAN-L3-J-077` | Asset Location Change to Governed Operational Location | ACTIVE |
| `CAN-L3-J-759` | `CAN-L3-J-078` | Authorised Asset Transfer to Reconciled Custody and Location Outcome | ACTIVE |
| `CAN-L3-J-760` | `CAN-L3-J-079` | Asset Condition Change to Governed Lifecycle Condition | ACTIVE |
| `CAN-L3-J-761` | `CAN-L3-J-080` | Missing Asset to Recovery or Governed Disposition | ACTIVE |
| `CAN-L3-J-762` | `CAN-L3-J-081` | Repair or Maintenance Activity to Updated Asset Lifecycle Reference | ACTIVE |
| `CAN-L3-J-763` | `CAN-L3-J-082` | Asset Verification Finding to Reconciled Asset Lifecycle Outcome | ACTIVE |
| `CAN-L3-J-764` | `CAN-L3-J-083` | Asset Retirement Requirement to Governed Retirement Outcome | ACTIVE |
| `CAN-L3-J-765` | `CAN-L3-J-084` | Retired Asset to Governed Disposal Coordination and Outcome | ACTIVE |
| `CAN-L3-J-766` | `CAN-L3-J-085` | Completed Asset Lifecycle to Governed Archival | ACTIVE |
| `CAN-L3-J-767` | `CAN-L3-J-086` | Protected Asset History Error to Governed Supplemental Correction | ACTIVE |
| `CAN-L3-J-768` | `CAN-L3-J-087` | Authorized Inventory Verification Requirement to Reconciled Audit Closure | ACTIVE |
| `CAN-L3-J-769` | `CAN-L3-J-088` | Cancelled or Incomplete Audit to Preserved Governed Outcome | ACTIVE |
| `CAN-L3-J-770` | `CAN-L3-J-089` | Material Inventory Variance to Investigated and Reconciled Closure | ACTIVE |
| `CAN-L3-J-771` | `CAN-L3-J-090` | Approved Audit Determination to Canonical Inventory Adjustment Outcome | ACTIVE |
| `CAN-L3-J-772` | `CAN-L3-J-091` | Confirmed Audit Loss or Damage to Reconciled J10 Referral Outcome | ACTIVE |
| `CAN-L3-J-773` | `CAN-L3-J-092` | Asset-Linked Physical Verification to Reconciled Asset Finding Outcome | ACTIVE |
| `CAN-L3-J-774` | `CAN-L3-J-093` | Closed Audit Evidence to Governed Supplemental Determination | ACTIVE |
| `CAN-L3-J-775` | `CAN-L3-J-094` | Disposal or Write-Off Proposal to Executable Institutional Authorisation | ACTIVE |
| `CAN-L3-J-776` | `CAN-L3-J-095` | Proposed Disposal or Write-Off to Governed Rejection or Return Outcome | ACTIVE |
| `CAN-L3-J-777` | `CAN-L3-J-096` | Pre-Execution Disposal Case to Governed Cancellation | ACTIVE |
| `CAN-L3-J-778` | `CAN-L3-J-097` | Authorised Standard Disposal Case to Reconciled Physical Disposal Closure | ACTIVE |
| `CAN-L3-J-779` | `CAN-L3-J-098` | Authorised Disposal Case to Reconciled Partial Disposal Outcome | ACTIVE |
| `CAN-L3-J-780` | `CAN-L3-J-099` | Authorised Inventory to Reconciled Destruction Outcome | ACTIVE |
| `CAN-L3-J-781` | `CAN-L3-J-100` | Authorised Scrap Inventory to Reconciled Scrap Handover | ACTIVE |
| `CAN-L3-J-782` | `CAN-L3-J-101` | Authorised Inventory to Reconciled Sale or Auction Disposal | ACTIVE |
| `CAN-L3-J-783` | `CAN-L3-J-102` | Authorised Inventory to Reconciled Donation Outcome | ACTIVE |
| `CAN-L3-J-784` | `CAN-L3-J-103` | Authorised Inventory to Reconciled External Transfer-Out Outcome | ACTIVE |
| `CAN-L3-J-785` | `CAN-L3-J-104` | Authorised Inventory to Reconciled Recycling or External Return Outcome | ACTIVE |
| `CAN-L3-J-786` | `CAN-L3-J-105` | Confirmed Inventory Loss to Authorised and Reconciled Write-Off Outcome | ACTIVE |
| `CAN-L3-J-787` | `CAN-L3-J-106` | Audit Finding to Authorised and Reconciled Disposal or Write-Off Outcome | ACTIVE |
| `CAN-L3-J-788` | `CAN-L3-J-107` | Retired Asset to Reconciled Asset-Linked Disposal Outcome | ACTIVE |
| `CAN-L3-J-789` | `CAN-L3-J-108` | Urgent Safety or Regulatory Requirement to Reconciled Emergency Disposal Outcome | ACTIVE |
| `CAN-L3-J-790` | `CAN-L3-J-109` | Disposal Execution Exception to Governed Resolution and Closure | ACTIVE |
| `CAN-L3-J-791` | `CAN-L3-J-110` | Irreversible Disposal Error to Governed Supplemental Correction | ACTIVE |

## 9. Absorbed / Retired Historical IDs

This migration renumbers only the 1,095 candidates currently ACTIVE in the five live Phase 1 inventories. It does not resurrect, renumber, or reassign any absorbed, merged, duplicate, retired, or rejected candidate ID from an earlier consolidation generation. Their historical identity remains exactly where it was already authoritatively recorded, in the section watchlists that predate this migration:

| Section | Historical crosswalk location | Historical scheme preserved |
|---|---|---|
| A | `L3_Section_A_Phase2_Consolidation_Watchlist.md` | 18 relationship clusters (A039/A046, A047/A054, A052/A067/A068, etc.) |
| C | `L3_Section_C_Phase2_Consolidation_Watchlist.md` §6.1 | pre-2026-08-14 application record |
| F | `L3_Section_F_Phase2_Consolidation_Watchlist.md` §9, §11–12 | former `CAN-L3-F-0001`…`CAN-L3-F-1858` raw discovery IDs; retired `F-086`/`F-087` |
| G | `L3_Section_G_Phase2_Consolidation_Watchlist.md` §1–§13, "Absorbed Candidate Crosswalk" | former sparse `CAN-L3-G-001`…`CAN-L3-G-541` scheme; the 2026-08-17 disjoint-start `G-542`…`G-863` scheme (now itself superseded by this migration, see §3 above and §10 below) |
| H | `L3_Section_H_Phase2_Consolidation_Watchlist.md` §3 | former subsection-fragment `CAN-L3-H01-001`…`CAN-L3-H10-883` scheme (893 rows) |
| I | `L3_Section_I_Phase2_Consolidation_Watchlist.md` §1–§16 | former sparse `CAN-L3-I-001`…`CAN-L3-I-314` scheme; the 2026-08-17 disjoint-start `I-315`…`I-511` scheme (now itself superseded, see §3 above and §10 below) |

No new absorption mapping was invented by this migration. Where a cross-reference in an active document pointed at an ID that had itself been silently renumbered by an earlier consolidation (not merged or retired — simply renumbered without updating its citations), and the correct current target could be verified with certainty by an exact Scenario Title match against the live inventory, the citation was corrected as part of "repair live cross-references" (task scope item 4), and the correction is recorded in §10 below. No other pre-existing dangling reference was resolved by guesswork.

## 10. Verified Stale Cross-Reference Corrections Applied

These five citations pointed at an ID from an earlier numbering generation that had already been superseded before this migration began. Each was verified by an exact Scenario Title match against the live inventory, then carried through this migration's mapping to its final new ID.

| Stale reference found | Verified current (pre-migration) target | Final new ID | Verification |
|---|---|---|---|
| `CAN-L3-E-103` | `CAN-L3-E-119` | `CAN-L3-E-015` | Title match: "Delegated Authority and Additional Charge Assignment" |
| `CAN-L3-H-010` | `CAN-L3-H-095` | `CAN-L3-H-010` | Title match: "Authoritative Business Outcome to Governed Communication Outcome" |
| `CAN-L3-H-050` | `CAN-L3-H-133` | `CAN-L3-H-048` | Title match: "Dashboard Requirement to Governed Published Dashboard" |
| `CAN-L3-J-028` | `CAN-L3-J-709` | `CAN-L3-J-028` | Title match: "Supplier Performance or Incident Evidence to Governed Supplier Assessment Record" |
| `CAN-L3-J-029` | `CAN-L3-J-710` | `CAN-L3-J-029` | Title match: "Supplier Risk Finding to Governed Restriction" |

Sites corrected: `L3_Phase1_Inventory_Dhruv.md` (RC-01 note), `L3_Section_A_Phase2_Consolidation_Watchlist.md` (RC-01 record), `L3_Section_F_Phase2_Consolidation_Watchlist.md` §12 (retirement note), `L3_Section_G_Phase2_Consolidation_Watchlist.md` (G-FR-08, G-FR-09, G-XI-01, G-XI-02).

## 11. Superseded 2026-08-17 Disjoint-Start Renumbering (G, I)

Sections G and I each carry an internal "ID Renumbering — 2026-08-17" record documenting an earlier, narrower renumbering (to `G-542`…`G-863` and `I-315`…`I-511` respectively) that this migration now supersedes with the repository-wide `001`-start rule. That earlier record's own text is left exactly as it was written — it accurately documents what happened at that point in the day — and each watchlist now carries an additional note pointing forward to this document as the current authority. See `L3_Section_G_Phase2_Consolidation_Watchlist.md` and `L3_Section_I_Phase2_Consolidation_Watchlist.md`.

## 12. Affected Live Repository Files

- 5 Phase 1 inventory files (`01_Phase1_Discovery/`)
- 8 Phase 2 Consolidation Watchlists (`00_Governance/L3_Section_*_Phase2_Consolidation_Watchlist.md`)
- `02_Phase2_Consolidation/L3_Phase2_Proposed_Canonical_Scenario_Register.md` (Phase 1 ID column only; Phase 2 Working IDs, dispositions, blockers and evidence untouched)
- `README.md`
- `L3_Naming_Numbering_Standard.md`, `L3_Phase1_Discovery_Rules.md`, `L3_Phase1_Team_Instructions.md`
- `check_l3_inventory.py`

## 13. Historical-Report Handling Policy

Point-in-time reports under `generated/reviews/` are not rewritten. Each report that cites a pre-migration `CAN-L3-` ID received a single non-destructive note pointing to this document; no historical narrative, decision, or ID inside those reports was altered. The same treatment was applied to `L3_Section_A_Phase1_Audit_Application_Report_2026-08-08.md` (explicitly a change record, not governance) and to `CHANGELOG.md` (append-only; a new dated entry records this migration rather than editing prior entries).

## 14. Validation Result

See `generated/reviews/L3_Phase1_Continuous_Numbering_Migration_2026-08-17.md` for the full validation run: per-section count/sequence verification, content-preservation diff (0 unintended changes across all 1,095 rows), `check_l3_inventory.py` result, and Phase 2 register integrity check.

