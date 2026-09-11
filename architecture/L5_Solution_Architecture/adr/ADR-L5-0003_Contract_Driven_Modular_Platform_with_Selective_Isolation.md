# ADR-L5-0003 — Contract-Driven Modular Platform with Selective Isolation

**ADR ID:** `ADR-L5-0003`  
**Title:** Contract-Driven Modular Platform with Selective Isolation  
**Scope:** Repository  
**Decision Owner:** L5-A-01 — Application & Platform Architecture  
**Assigned Author:** Architecture remediation for Dhruv Stage-1 corpus  
**Reviewers:** SAOS architecture review; promotion explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-11  
**Affected L5 Specifications:** `L5-A-01-001`; materially constrains `L5-A-03-001`; `L5-A-03-002`; `L5-A-04-*`; `L5-A-07-*`; `L5-E-04-*`  
**Related ADRs:** `ADR-L5-0004`; `ADR-L5-0005`; `ADR-L5-0006`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

SutraERP requires one reusable application-composition model that preserves canonical business ownership while avoiding two invalid extremes: treating every business-domain boundary as a separately deployed service, or collapsing all architecture into an unrestricted shared runtime.

`L5-A-01-001` already establishes a product-neutral direction in which strong technical-module boundaries are the default and separately executable boundaries are introduced only when a material architecture reason requires them. Because this structure governs multiple L5 families and materially affects service/runtime boundaries, the locked ADR methodology requires Repository ADR governance.

---

## 2. Scope & Affected Architecture

This ADR governs the top-level application-composition pattern only:

- Modular Application Core as the default logical composition model;
- Selectively Isolated Execution Units where explicit canonical L5 architecture requires stronger execution isolation;
- governed technical contracts/events for core↔isolated interaction;
- separation of business-domain ownership, technical-module boundary, execution boundary and deployment boundary.

It does not own detailed module decomposition/enforcement (`L5-A-03-*`), Tenant isolation (`L5-A-04-*`), persistence (`L5-B-02-*`), contracts/events (`L5-C-*`), security controls (`L5-D-*`) or physical runtime/deployment topology (`L5-E-04-*`).

---

## 3. Authoritative Constraints

1. Canonical business ownership shall not change because of technical placement.
2. Direct cross-owner persistence access remains prohibited.
3. Tenant/Institution isolation shall remain valid across all application boundaries.
4. Module activation is not deployment/runtime creation.
5. Business-domain identity alone shall not mandate a network/process/service/deployment boundary.
6. A required stronger isolation boundary shall not be collapsed for implementation convenience.
7. The decision must remain provider-, framework- and orchestration-neutral until separate technology selection is approved.

---

## 4. Decision Criteria

The selected model must provide strong ownership preservation, manageable distributed-system complexity, support for high-integrity isolated capabilities, deployment portability, explicit cross-boundary collaboration, testability and future evolvability without forcing repository-wide microservices or a single physical runtime.

---

## 5. Alternatives Considered

### Alternative A — Service-per-business-domain

Every canonical business domain becomes an independently executed service.

**Rejected.** Business ownership does not automatically justify distributed execution; this would introduce unnecessary runtime, operational and consistency complexity.

### Alternative B — One unrestricted monolithic runtime

All capabilities share one implementation/runtime boundary without explicit technical isolation semantics.

**Rejected.** This would weaken module, trust and high-integrity isolation boundaries and make selective isolation difficult to govern.

### Alternative C — Contract-Driven Modular Platform with Selective Isolation

Strong technical modules compose by default inside the Modular Application Core. A capability becomes separately executable only when its canonical L5 owner and applicable governance justify isolation; crossings use governed C01/C02 boundaries.

**Selected.**

---

## 6. Evaluation

| Criterion | A — Service per Domain | B — Unrestricted Monolith | C — Modular + Selective Isolation |
|---|---|---|---|
| Ownership preservation | Acceptable | Weak | Strong |
| Distributed-system complexity | Weak | Strong | Strong |
| High-integrity isolation | Strong | Weak | Strong |
| Deployment neutrality | Weak | Acceptable | Strong |
| Evolvability | Acceptable | Weak | Strong |
| Cross-spec reuse | Weak | Weak | Strong |

Alternative C satisfies the full architecture constraints without converting business-domain boundaries into infrastructure topology.

---

## 7. Decision

SutraERP shall use a **Contract-Driven Modular Platform with Selective Isolation**.

1. The **Modular Application Core** is the default logical composition model.
2. Business-domain identity alone shall not create a service, process, network or deployment boundary.
3. A **Selectively Isolated Execution Unit** is introduced only when explicit canonical L5 architecture or an approved ADR establishes a material security, workload, resilience, integrity or operational reason.
4. Core↔isolated collaboration shall use governed technical boundaries owned by C01/C02 and preserve A04 scope, B02 persistence ownership and applicable D-family controls.
5. Physical packaging/deployment remains E04-owned and may strengthen but shall not weaken required logical boundaries.

---

## 8. Consequences

### Positive

- avoids unnecessary service proliferation;
- preserves strong technical boundaries in a modular application;
- supports secure/specialized isolated engines;
- keeps deployment technology and topology open;
- permits later extraction or consolidation without changing business ownership.

### Negative / Cost

- requires disciplined A03 boundary enforcement;
- isolated units require explicit contract/event boundaries and operational handling;
- implementation teams cannot infer deployment architecture solely from business-domain structure.

---

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Modular core becomes a shared-state monolith | A03 and B02 conformance prohibit cross-owner coupling and persistence bypass. |
| Selective isolation turns into ad-hoc microservices | Isolation requires canonical owner justification and applicable ADR/governance. |
| Physical co-location weakens security | A04/D03 logical controls remain mandatory regardless of placement. |
| ADR is mistaken for deployment topology | E04 remains canonical owner of physical/runtime realization. |

---

## 10. Migration / Exit Position

No product or vendor migration is implied. A future replacement must preserve canonical ownership, governed cross-boundary collaboration, Tenant/Institution isolation and peer-L5 responsibility boundaries. Replacement requires explicit supersession through a new ADR.

---

## 11. Traceability

Primary realization: `L5-A-01-001_Application_and_Platform_Architecture.md`.

Material collaborating architecture: `L5-A-03-*`, `L5-A-04-*`, `L5-B-02-*`, `L5-C-01-*`, `L5-C-02-*`, `L5-D-03-*`, `L5-E-04-*`, `L5-A-07-*`.

---

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The Stage-1 architecture review confirmed mandatory ADR applicability, repository scope, technology neutrality, preservation of upstream ownership, explicit alternatives and compatibility with the locked dependency model. The controlled remediation instruction on 2026-09-11 explicitly authorized resolution and canonicalization of the identified Stage-1 blockers. This approval does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| A01 technical decision owner | Recommended | 2026-09-10 | `L5-A-01-001` AD-001 |
| SAOS architecture reviewer | PASS — no blocking ADR defect found | 2026-09-11 | Stage-1 production architecture review |
| Architecture promotion authorization | Approved | 2026-09-11 | Explicit controlled-session remediation authorization |
