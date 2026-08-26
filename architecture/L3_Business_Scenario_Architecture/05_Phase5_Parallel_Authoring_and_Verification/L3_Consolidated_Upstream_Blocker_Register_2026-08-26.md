# SutraERP L3 Phase 5 — Consolidated Upstream Blocker Register

**Date:** 2026-08-26  
**Scope:** Dhruv frozen L3 Phase 5 allocation — 250 scenarios  
**Purpose:** Consolidate upstream architecture blockers discovered during L3 authoring/review, map each blocker to its canonical L1/L2 owner, record the controlled resolution, and define the targeted revalidation set before Architecture Approval/Lock.  
**L3 Corpus Policy:** Preserve the frozen scenario corpus. Do not redesign, renumber, merge, split, or broadly rewrite scenarios to compensate for upstream architecture gaps.  
**Governance Status:** Architecture remediation register; scenario approval/lock remains a separate governance action.

---

## 1. Authority and Interpretation

This register is governed by the SutraERP architecture authority hierarchy and must be read with:

1. Locked repository decisions and governance standards.
2. Locked L1 architecture.
3. `DEPENDENCY_MAP.md`.
4. SutraERP Architecture Workflow Context Baseline.
5. Locked L2 source documents and their controlled amendments.
6. L3 Phase 5 authoring standards and the frozen scenario register.

A `Resolved` entry means the upstream architecture gap has been closed by a Locked controlled amendment. It does **not** mean the affected L3 scenario is Architecture Approved or Locked.

Historical `Source Assessment: BLOCKED` text in a Draft L3 file records the authoring-time upstream state. Current approval readiness is determined only after targeted revalidation against the amendment listed here.

---

## 2. Frozen Dhruv Corpus

| Section | Frozen Allocation | Count |
|---|---:|---:|
| A | L3-A-001–062 | 62 |
| H | L3-H-001–081 | 81 |
| F | L3-F-073–079, 084–085 | 9 |
| G | L3-G-001–015, 278–320 | 58 |
| I | L3-I-083–117 | 35 |
| J | L3-J-038–042 | 5 |
| **Total** |  | **250** |

No scenario ID, title, primary-owner section, or frozen outcome is changed by this register.

---

## 3. Consolidated Upstream Blockers and Controlled Resolutions

| Blocker | Canonical Owner | Gap Type | Affected L3 Scenarios | Controlled Resolution | Status |
|---|---|---|---|---|---|
| UB-H01-01 | H01 Government Learner Account / Credential Publication | Lifecycle, recovery authority, lineage, events | H005, H008 | `L2-H01_Publication_Recovery_and_Republishing_Amendment_2026-08-26.md` | Resolved |
| UB-H03-01 | H03 Enterprise Search | Approval authority, projection lifecycle, configuration versioning/rollback, events | H020, H021 | `L2-H03_Search_Governance_Amendment_2026-08-26.md` | Resolved |
| UB-H04-01 | H04 Placement | Employer activation/retirement, offer expiry, internship verified completion, disruption/reopening events | H022, H025, H026, H027 | `L2-H04_Placement_Lifecycle_Completion_Amendment_2026-08-26.md` | Resolved |
| UB-H05-01 | H05 Research | Proposal withdrawal boundary, grant amendment/closure, ethics, publication, IP, collaboration, consultancy, portfolio lifecycle/events | H030, H032–H038 | `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` | Resolved |
| UB-C08-H06-01 | C08 Student Lifecycle / H06 Alumni | Cross-domain canonical ownership boundary | H039 | `L2-C08_Alumni_Relationship_Boundary_Clarification_2026-08-26.md` + `L2-H06_Alumni_Engagement_Lifecycle_Amendment_2026-08-26.md` | Resolved |
| UB-H06-02 | H06 Alumni | Membership, chapter, networking, recognition lifecycle/events | H040, H042, H043, H046 | `L2-H06_Alumni_Engagement_Lifecycle_Amendment_2026-08-26.md` | Resolved |
| UB-H07-01 | H07 Enterprise Dashboard | Decision authority, rejection, retirement/restoration, widget lifecycle/events | H048, H053, H054 | `L2-H07_Dashboard_Governance_Lifecycle_Amendment_2026-08-26.md` | Resolved |
| UB-H08-01 | H08 Regulatory Reporting | Framework/plan/package approval authority and lifecycle/versioning | H055, H056, H058 | `L2-H08_Regulatory_Reporting_Authority_and_Lifecycle_Amendment_2026-08-26.md` | Resolved |
| UB-H09-01 | H09 Accreditation & Compliance | Assessment-plan approval, waiver/retirement, external assessment outcome, CAPA reopening, quality-improvement closure | H062, H063, H066, H067, H068 | `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md` | Resolved |
| UB-A03-H10-01 | A03 Authentication / H10 External Education Integration | Authentication-provider vs provider-trust ownership conflict | H074 | `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md` | Resolved — A03 retains authentication ownership |
| UB-H10-02 | H10 External Education Integration | Connector rollback, trust revocation/expiry, enrollment exchange, credential-verification result, exchange rejection/quarantine events | H073, H076, H079, H081 | `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md` | Resolved |
| UB-B03-G10-01 | B03 Institutional Campus / G10 Infrastructure | Cross-domain campus reference direction, contract and actor boundary | G279 | `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` | Resolved — G10 persists the reference; B03 reverse-resolves by read contract |
| UB-G10-02 | G10 Infrastructure | Structural registration event vocabulary / initial lifecycle ambiguity | G282, G283 and neighboring structural-registration scenarios relying on the same vocabulary | `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` | Resolved |
| UB-G10-03 | G10 Infrastructure | Operational Capability workflow event and consumer contract | G287 | `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` | Resolved |
| UB-G10-04 | G10 Infrastructure | Capacity event-name conflict | G288 | `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` | Resolved — `CapacityUpdated` canonical |
| UB-G10-05 | G10 Infrastructure | Availability establishment/change event vocabulary | G289, G290 | `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` | Resolved |
| UB-G10-06 | G10 Infrastructure | Reservation cancellation/closure lifecycle meaning | G293 | `L2-G10_Infrastructure_Governance_Completion_Amendment_2026-08-26.md` | Resolved — `Cancelled → Archived`; “closed” is business completion meaning |

---

## 4. Previously Resolved High-Risk Items Retained for Traceability

These issues were remediated before creation of this register and are not reopened:

| Item | Resolution | Current Register Treatment |
|---|---|---|
| A001 Tenant provisioning authority/event traceability | `L2-A01_Tenant_Provisioning_Authority_and_Event_Amendment_2026-08-26.md` and targeted L3-A-001 update | Resolved; no further corpus change required unless review finds a new defect |
| H001 government learner account link success/failure boundary | Targeted H001 remediation | Resolved |
| F073 financial reporting correction vs successful publication | Targeted F073 remediation | Resolved |
| G001 readiness alternate-flow vs failure/recovery classification | Targeted G001 remediation | Resolved |
| G278 Physical Campus registration initial state/event terminology | `L2-G10_Infrastructure_Management_Workflows_Amendment_2026-08-26.md` | Resolved upstream; targeted revalidation only if approval review requires it |

---

## 5. Explicitly Excluded from the Affected Set

`L3-H-071_Approved_Accreditation_Evidence_to_Self_Study_Report_Approval_and_Handoff.md` was checked separately because it falls in the H09 neighborhood. Its current Draft records **no open issues** and its H09→H08 ownership boundary is already explicit. It is therefore not treated as an upstream-blocked scenario by this remediation register.

---

## 6. Targeted Revalidation Set

The mandatory targeted revalidation set created by the controlled amendments is:

### H scenarios — 37

H005, H008, H020, H021, H022, H025, H026, H027, H030, H032, H033, H034, H035, H036, H037, H038, H039, H040, H042, H043, H046, H048, H053, H054, H055, H056, H058, H062, H063, H066, H067, H068, H073, H074, H076, H079, H081.

### G scenarios — minimum direct blocker set

G279, G282, G283, G287, G288, G289, G290, G293.

Neighboring G10 scenarios shall be revalidated only where their current §17 issue explicitly relies on the same normalized event/lifecycle vocabulary. They shall not be rewritten merely because they share G10 as owner.

---

## 7. Revalidation Rules

Each affected scenario must be checked against its new authoritative amendment for:

1. frozen scenario ID/title/outcome preserved;
2. primary owner unchanged unless the frozen final outcome itself was previously misclassified;
3. no duplicate entity, lifecycle or event ownership introduced;
4. exact lifecycle state/event names now supported upstream;
5. decision authority is business-level and upstream-supported;
6. cross-domain hand-offs preserve source ownership;
7. §10 alternate flows and §11 failure/recovery remain correctly classified;
8. no L4/L5 implementation leakage introduced;
9. historical blocker is traceable to the amendment that resolved it;
10. scenario remains Draft/Review until independent Architecture Approval.

---

## 8. Approval / Lock Gate

Upstream remediation is complete for every blocker listed in §3. The affected L3 scenarios must still pass targeted scenario-level revalidation before Architecture Approval.

**Architecture Approval may proceed only when:**

- no affected scenario retains a material contradiction with its controlling amendment;
- any historical `BLOCKED` assessment is clearly marked as resolved or is superseded by an approved revalidation record;
- no new ownership/event/lifecycle/authority conflict is discovered; and
- the governance reviewer, not the author, performs the approval action.

**Lock may proceed only after Architecture Approval and repository-wide consistency checks required by the L3 governance standards.**

---

## 9. Current Register Status

**Upstream blocker remediation:** COMPLETE for the blockers recorded in this register.  
**L3 corpus redesign:** NOT PERFORMED.  
**Targeted L3 revalidation:** REQUIRED / tracked separately.  
**Architecture Approval:** NOT YET GRANTED.  
**Lock:** NOT YET GRANTED.
