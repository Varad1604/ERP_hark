# ADR-L5-0005 — Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern

**ADR ID:** `ADR-L5-0005`  
**Title:** Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-B-02 — Database & Persistence Architecture  
**Assigned Author:** Architecture remediation for Dhruv Stage-1 corpus  
**Reviewers:** SAOS architecture review; promotion explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-11  
**Affected L5 Specifications:** `L5-B-02-001`; `L5-B-02-002`  
**Related ADRs:** `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0008`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

SutraERP requires one reusable persistence structure that preserves canonical owner boundaries and Tenant isolation without forcing a fixed physical database count. The Stage-1 B02 architecture already establishes relational-first authoritative persistence, owner-scoped persistence boundaries and a hybrid logical/physical topology. Because persistence-isolation strategy is reusable and materially affects multiple L5 specifications, ADR governance is mandatory.

## 2. Scope & Affected Architecture

This ADR governs authoritative relational persistence, owner-scoped logical persistence boundaries, owner-scoped runtime identities/namespaces and hybrid Tenant placement. It does not select a database product, define transaction/concurrency semantics, define schema migration mechanics, or establish recovery/deployment topology.

## 3. Authoritative Constraints

1. Every authoritative persisted business fact remains owned by exactly one canonical owner.
2. Direct cross-owner database reads/writes shall not become a collaboration mechanism.
3. Tenant/Institution scope from A04 must be preserved at protected persistence boundaries.
4. Physical database count shall not be inferred from business-domain count.
5. Logical ownership/isolation must remain valid when infrastructure is shared.
6. Dedicated physical separation may be used where justified by security, privacy, workload, resilience, recovery or deployment constraints.
7. Derived stores shall not become hidden authoritative sources.

## 4. Decision Criteria

Ownership integrity, Tenant isolation, deployment portability, operational manageability, scalability, recovery compatibility, testability and avoidance of unnecessary database proliferation.

## 5. Alternatives Considered

### A — One database per business domain/owner by default
Rejected as universal rule: preserves separation but hard-codes physical topology and creates avoidable operational burden.

### B — One unrestricted shared schema/database
Rejected: makes owner boundaries weak and encourages cross-owner access.

### C — Owner-scoped logical persistence with hybrid physical placement
Selected.

## 6. Evaluation

| Criterion | A | B | C |
|---|---|---|---|
| Ownership isolation | Strong | Weak | Strong |
| Deployment flexibility | Weak | Strong | Strong |
| Operational burden | Weak | Strong | Acceptable/Strong |
| Tenant-placement flexibility | Acceptable | Weak | Strong |
| Cross-owner bypass prevention | Strong | Weak | Strong |

## 7. Decision

SutraERP shall use **owner-scoped logical persistence boundaries with hybrid physical placement**.

1. Each canonical owner maps to one authoritative persistence boundary.
2. Owner-scoped namespaces and runtime persistence identities shall prevent routine cross-owner access.
3. Shared physical relational infrastructure is permitted where ownership, Tenant isolation, security and operational constraints remain enforceable.
4. Tenant placement may remain shared inside an owner boundary by default and may escalate to owner+Tenant namespace or dedicated database when justified by approved constraints.
5. No fixed physical database count is canonical.
6. Cross-owner collaboration uses governed contracts/events/workflows, not direct persistence access.
7. The primary relational database product remains a separate Repository ADR/technology-selection decision.

## 8. Consequences

Positive: strong logical ownership, flexible deployment, no artificial database-count rule, scalable Tenant placement.  
Cost: runtime identities/namespaces and policy enforcement must be maintained; shared infrastructure requires clear evidence of logical separation.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Shared DB encourages direct joins/writes | Owner-scoped identities and conformance prohibit cross-owner access. |
| Too many dedicated databases | Dedicated placement requires explicit justification, not preference. |
| Tenant isolation becomes physical-topology dependent | A04 logical scope remains authoritative. |
| Derived stores become authoritative | B02 conformance keeps canonical write authority with the owner boundary. |

## 10. Migration / Exit Position

No database vendor is selected. Physical placement may evolve without changing canonical ownership if owner/Tenant boundaries remain preserved. Technology replacement requires its own governed technology ADR where mandatory.

## 11. Traceability

Primary realization: `L5-B-02-001_Database_and_Persistence_Architecture_Baseline.md`; `L5-B-02-002_Domain_Persistence_Ownership_and_Isolation_Architecture.md`.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The Stage-1 review confirmed one shared B02 decision scope for persistence topology and isolation, with database-product selection intentionally separate. Controlled remediation was explicitly authorized on 2026-09-11. This approval does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| B02 technical decision owner | Recommended | 2026-09-10 | B02-001/B02-002 architecture |
| SAOS architecture reviewer | PASS | 2026-09-11 | Stage-1 production architecture review |
| Architecture promotion authorization | Approved | 2026-09-11 | Explicit remediation authorization |
