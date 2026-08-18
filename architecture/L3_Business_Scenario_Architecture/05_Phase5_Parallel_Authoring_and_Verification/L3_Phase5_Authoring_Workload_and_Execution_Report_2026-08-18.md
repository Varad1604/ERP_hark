# SutraERP L3 Phase 5 — Parallel Authoring and Verification Report

**Architecture Level:** L3 — Business Scenario Architecture  
**Phase:** Phase 5 — Parallel Authoring and Continuous Verification  
**Status:** Approved Planning Baseline — Detailed Scenario Allocation Pending Final Capability-Boundary Normalization  
**Owner:** Lead Architect  
**Date:** 2026-08-18  
**Canonical Scenario Population:** 1,084  

---

## 1. Purpose

This report establishes the Phase 5 operating baseline for parallel authoring of the 1,084 final canonical L3 scenarios after completion of Phase 4 canonical-set construction and final-ID assignment.

Phase 5 is an **authoring and verification phase**. It does not change canonical L1/L2 ownership, business rules, lifecycle states, event ownership, dependencies, or final L3 scenario identity.

The assignment of a human author is a workload decision only.

---

## 2. Authoring Authority

All Phase 5 authoring shall use:

1. the frozen Phase 3/Phase 4 canonical scenario semantics;
2. the Stage 4 final L3 IDs;
3. `../00_Governance/L3_Business_Scenario_Template_v2.0.md`;
4. `../00_Governance/L3_Naming_Numbering_Standard.md`;
5. authoritative L1 and L2 architecture for business content.

No author may reinterpret a workload assignment as transfer of canonical domain ownership.

---

## 3. Approved Team Authoring Quotas

The Phase 5 workload distribution is approved as follows:

| Team Member | Assigned Authoring Quota | Relative Workload |
|---|---:|---|
| Dhruv | **250** | Medium — dedicated ERP plus Lead Architect responsibilities |
| Pallav | **337** | High — dedicated ERP |
| Varad | **337** | High — dedicated ERP |
| Chetan | **80** | Reduced — concurrent responsibility for other projects |
| Devang | **80** | Reduced — concurrent responsibility for other projects |
| **Total** | **1,084** | **100%** |

The workload hierarchy is therefore:

```text
Pallav = Varad > Dhruv >> Chetan = Devang
```

Dhruv carries fewer authoring scenarios than Pallav and Varad because Phase 5 also requires repository-wide architectural governance, cross-domain conflict resolution, final review, and consistency control by the Lead Architect.

---

## 4. Assignment Principles

Scenario allocation shall follow these rules:

- preserve complete capability blocks wherever practical;
- prefer author-domain affinity over arbitrary numeric slicing;
- do not change canonical Primary Owner because of the assigned human author;
- keep specialist Finance/Procurement scenarios concentrated with Chetan where his expertise adds the most value;
- keep specialist Examination/Campus Operations scenarios concentrated with Devang where his expertise adds the most value;
- redistribute transferable F/J/G/I authoring primarily to Dhruv, Pallav, and Varad;
- avoid splitting one L2 capability between multiple authors unless a documented exception is necessary;
- balance complexity, not only scenario count, during final allocation review.

---

## 5. Canonical Ownership vs Human Assignment

Phase 5 distinguishes four responsibilities:

```text
Canonical Owner
      ≠
Assigned Author
      ≠
Domain Reviewer
      ≠
Final Architecture Approver
```

A scenario authored by a person outside its canonical section remains owned by the canonical business domain defined by L1/L2.

Example:

```text
Pallav authors an Examination scenario
        ↓
Canonical Primary Owner remains Examination
        ↓
Required domain expertise is consulted where needed
        ↓
Lead Architect performs final architecture consistency review
```

---

## 6. Current Allocation Design

The working allocation is based on coherent capability blocks rather than random ID ranges.

### Dhruv

Primary concentration:

- Section A — Platform Foundation;
- Section H — Institutional Services & Intelligence;
- selected governance, infrastructure, reporting, publication, and cross-domain scenarios from F/G/I.

Target: **250 scenarios**.

### Pallav

Primary concentration:

- Section B — Institution & Structure;
- Section D — Academic Delivery;
- academic/resource-adjacent scenarios from F/G/I/J, including facilities, laboratory, examination planning, practical/viva, evaluation, and resource-operation scenarios where business affinity is strong.

Target: **337 scenarios**.

### Varad

Primary concentration:

- Section C — Student Lifecycle;
- Section E — People & Workforce;
- student/person-centric scenarios from F/G/I, including student finance, hostel, transport, library, health, examination registration, hall tickets, subsequent examination attempts, and academic completion.

Target: **337 scenarios**.

### Chetan

Primary concentration:

- specialist Finance areas;
- procurement planning and sourcing;
- vendor/supplier management;
- accounting, payables/receivables, treasury, and physical verification where specialist finance/resource knowledge is most valuable.

Target: **80 scenarios**.

### Devang

Primary concentration:

- specialist Campus Operations and Examination areas;
- Security Operations;
- theory examination conduct;
- malpractice;
- examination exception/contingency;
- examination closure/archival.

Target: **80 scenarios**.

---

## 7. Open Allocation Normalization

The current working G allocation contains one structural weakness:

**Housekeeping Operations is split between Dhruv and Pallav.**

Phase 5 shall not lock the detailed scenario-ID assignment until this is normalized so that the complete Housekeeping capability is preferably assigned to one author.

Recommended direction:

- assign the complete Housekeeping Operations capability to one author;
- compensate the other author's quota using a complete transferable capability block elsewhere;
- preserve the approved total quotas of 250 / 337 / 337 / 80 / 80.

This is a workload-normalization issue only. It does not affect canonical G ownership or scenario semantics.

---

## 8. Controlled Pilot Gate

Broad parallel authoring shall begin only after the controlled pilot validates the v2.0 authoring conventions for:

- ownership and participating-domain expression;
- business actors and responsibilities;
- scenario boundary and outcome completeness;
- business recovery;
- lifecycle impact;
- event and cross-domain collaboration;
- authorization, approval, audit, and evidence obligations;
- implementation independence;
- compact authoring quality.

The pilot shall include representative split-derived, merge-survivor, variant-survivor, cross-domain, lifecycle-correction, infrastructure-support, Examination/Student/Finance, and resource-finance scenarios.

---

## 9. Phase 5 Authoring Cycle

Each assigned scenario follows this cycle:

```text
Scenario allocated
    ↓
Author verifies authoritative L1/L2 sources
    ↓
Author drafts using L3 v2.0 template
    ↓
Author performs self-check
    ↓
Architecture/domain review as required
    ↓
Exact findings returned to author
    ↓
Author corrects scenario
    ↓
Scenario revalidated
    ↓
Approved scenario enters completed L3 set
```

Verification shall occur continuously during Phase 5 rather than being deferred until all 1,084 documents are complete.

---

## 10. Phase 5 Safety Rules

Phase 5 authoring must not:

- change final L3 IDs;
- create new canonical scenarios without architecture change control;
- delete or merge approved canonical scenarios during drafting;
- change L1/L2 ownership;
- invent unsupported business rules, lifecycle states, events, contracts, or dependencies;
- introduce APIs, database design, software classes, UI design, deployment details, or technical implementation mechanics;
- treat author assignment as canonical ownership.

If authoritative upstream architecture is conflicting or incomplete, the author shall record the issue in the scenario's open-issues section and escalate it rather than silently resolving it.

---

## 11. Completion Criteria

Phase 5 is complete when:

1. all 1,084 canonical scenarios have authored L3 documents;
2. every document uses its assigned final L3 ID and canonical title/ownership baseline;
3. every document passes required review and validation;
4. no unresolved architecture-level ownership, lifecycle, event, rule, dependency, or boundary conflict remains hidden;
5. repository-wide L3 validation can begin.

---

## 12. Next Governance Action

Before the Phase 5 detailed workload register is declared locked:

1. normalize the Housekeeping author assignment;
2. produce the exact 1,084-row authoring register;
3. mechanically verify author totals equal 250 / 337 / 337 / 80 / 80;
4. verify every final L3 ID appears exactly once;
5. verify no canonical owner or scenario semantic field is changed by the allocation.

After this validation, the detailed Phase 5 workload register may be locked for parallel authoring.