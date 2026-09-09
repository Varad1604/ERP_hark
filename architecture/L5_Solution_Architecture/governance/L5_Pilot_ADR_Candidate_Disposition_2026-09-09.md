# SutraERP L5 — Pilot ADR Candidate Disposition

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Decision Date:** 2026-09-09  
**Depends On:** L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; completed representative L5 pilot  

---

## 1. Purpose

This decision records the material ADR candidates discovered during the representative L5 pilot and consolidates them before canonical ADR allocation.

The objective is to avoid both:

- hidden material technical decisions; and
- ADR inflation through duplicate or premature ADR creation.

No ADR number is allocated by this record.

---

## 2. Consolidated ADR Candidates

| Candidate | Decision Area | Proposed Scope | Canonical Decision Owner | Pilot Basis |
|---|---|---|---|---|
| ADR-C1 | Canonical Tenant/Institution isolation-enforcement pattern | Shared / Family | L5-A-04 | L5-A-04-001 / L5-A-04-002 |
| ADR-C2 | Authorization PDP/PEP realization pattern | Shared / Family | L5-D-02 | L5-D-02-004 |
| ADR-C3 | Persistence transaction, concurrency and consistency strategy | Shared / Family | L5-B-02 | L5-B-02-003 |
| ADR-C4 | State-to-event durable publication and delivery strategy | Repository or Shared — resolve during ADR scoping | L5-B-02 / L5-C-02 governed boundary | L5-B-02-003 / L5-C-02-002 |
| ADR-C5 | Runtime packaging and deployment portability pattern | Repository | L5-E-04 | L5-E-04-001 |
| ADR-C6 | Secure computational-engine isolation/runtime pattern | Shared / Family | L5-A-07 with L5-D/L5-E dependencies | L5-A-07-001 |

---

## 3. Disposition

All six candidates are retained as **material ADR candidates requiring formal ADR-necessity and scope review** before affected architecture is finally approved.

They are not yet canonical ADRs.

```text
Pilot ADR Candidate
        ≠
Allocated ADR
        ≠
Approved Decision
```

Before allocation, governance shall:

1. re-check whether another approved ADR already governs the issue;
2. confirm the candidate still requires an ADR under the locked methodology;
3. determine exact scope — Specification / Shared / Family / Repository as permitted by governance vocabulary;
4. confirm one canonical decision owner;
5. consolidate overlapping candidates if the decision is genuinely one architecture decision;
6. allocate the next canonical ID from `L5_ADR_Register.md` on `main`.

---

## 4. Broad-Authoring Effect

These candidates do not automatically block controlled Draft authoring of the 105-spec population.

Affected specifications may proceed in Draft with the relevant material decision explicitly unresolved.

However:

- an author shall not silently select the architecture governed by a candidate;
- an author shall not invent an ADR number;
- an affected specification shall not advance to Architecture Approved while a required blocking ADR remains unresolved.

---

## 5. Specific Boundary Notes

### ADR-C1 — Isolation

Must preserve separation between:

- A-04-001 isolation semantics/control points; and
- A-04-002 context propagation/data isolation mechanics.

### ADR-C2 — Authorization

Must preserve upstream ownership of Authorization Policy and canonical Authorization Decision semantics. L5 owns technical evaluation/enforcement realization.

### ADR-C3 — Persistence

Must preserve canonical technical ownership and prohibit cross-owner direct persistence writes.

### ADR-C4 — State/Event Boundary

Must not duplicate B-02-003 and C-02-002 ownership. B-02-003 owns state/publication-handoff consistency; C-02-002 owns durable delivery after the handoff exists.

### ADR-C5 — Runtime/Deployment

Must preserve on-premises-primary, cloud-capable, provider-neutral architecture and must not assume Kubernetes or a cloud provider.

### ADR-C6 — Secure Computation

Must preserve specialist engine separation, approved-rule execution, no arbitrary code execution, and no autonomous business approval/posting/publication authority.

---

## 6. Canonical ADR Register State

At the time of this decision:

- canonically allocated L5 ADR IDs: **0**;
- next possible canonical allocation: `ADR-L5-0001`;
- no candidate ID in this document reserves an ADR number.

---

## 7. Governance Effect

**Pilot ADR Candidates Identified:** 6  
**Canonical ADR IDs Allocated by This Record:** 0  
**Technology Selected:** No  
**Workload Allocation:** Not decided by this record  
**Broad L5 Authoring:** Not authorized by this record
