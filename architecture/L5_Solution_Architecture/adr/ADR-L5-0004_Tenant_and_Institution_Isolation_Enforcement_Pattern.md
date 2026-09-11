# ADR-L5-0004 — Tenant and Institution Isolation Enforcement Pattern

**ADR ID:** `ADR-L5-0004`  
**Title:** Tenant and Institution Isolation Enforcement Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-A-04 — Tenant & Institution Isolation Architecture  
**Assigned Author:** Architecture remediation for Dhruv Stage-1 corpus  
**Reviewers:** SAOS architecture review; promotion explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-11  
**Affected L5 Specifications:** `L5-A-04-001`; `L5-A-04-002`  
**Related ADRs:** `ADR-L5-0001`; `ADR-L5-0003`; `ADR-L5-0005`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

SutraERP requires one reusable technical isolation pattern that makes Tenant the mandatory root protected scope while allowing Institution and lower scopes to remain subordinate and context-dependent. The pattern must work across synchronous, asynchronous, workflow, background, persistence and isolated-runtime boundaries without turning physical infrastructure separation into the definition of isolation.

Pilot candidate `ADR-C1` identified this as a material Shared / Family decision. `L5-A-04-001` owns isolation semantics/control points and `L5-A-04-002` owns propagation/data-isolation mechanics.

## 2. Scope & Affected Architecture

This ADR governs validated Tenant-rooted execution scope, subordinate Institution scope, fail-closed scope enforcement, propagation across technical boundaries and logical data-isolation requirements. It does not own Tenant/Institution business lifecycle, authentication, authorization policy, database technology, trust-boundary internals or deployment topology.

## 3. Authoritative Constraints

1. Tenant is the mandatory root protected technical scope.
2. Institution is subordinate to Tenant and cannot widen or replace Tenant scope.
3. Missing, ambiguous, mismatched or illegally widened scope shall fail closed.
4. No default/global Tenant fallback is permitted for protected mutable operations.
5. Scope semantics must survive synchronous, asynchronous, workflow, background/deferred and persistence handoffs.
6. Logical isolation remains mandatory even when physical infrastructure is shared.
7. Physical separation may strengthen isolation but is not the canonical semantic definition.

## 4. Decision Criteria

Security, cross-Tenant prevention, deterministic propagation, deployment neutrality, persistence compatibility, testability, explicit failure behavior and preservation of peer-L5 ownership.

## 5. Alternatives Considered

### A — Physical database/runtime per Tenant as the universal rule
Rejected: over-constrains deployment and persistence while confusing physical placement with isolation semantics.

### B — Optional Tenant context inferred from user/session/environment
Rejected: permits ambiguity and unsafe fallback, especially for background, integration and machine execution.

### C — Validated Tenant-rooted execution scope with fail-closed propagation
Selected.

## 6. Evaluation

| Criterion | A | B | C |
|---|---|---|---|
| Cross-Tenant safety | Strong | Weak | Strong |
| Deployment neutrality | Weak | Strong | Strong |
| Background/integration safety | Acceptable | Weak | Strong |
| Persistence compatibility | Weak | Acceptable | Strong |
| Testability | Acceptable | Weak | Strong |

## 7. Decision

SutraERP shall use a **Validated Execution Scope Context** rooted in Tenant.

- Tenant scope is mandatory for protected Tenant-scoped execution.
- Institution and lower scopes are carried only when required and must be valid descendants of the Tenant scope.
- Every protected handoff shall preserve and revalidate scope as appropriate to the receiving boundary.
- Missing, ambiguous, switched, widened or incompatible scope shall fail closed.
- Shared physical runtime/database infrastructure is permitted only when logical isolation remains enforceable and attributable.
- A04 defines isolation semantics and propagation requirements; B02, C01/C02, D01/D02/D03, A05/A08 and E04 realize their own peer-owned mechanics consistently with them.

## 8. Consequences

Positive: one consistent isolation model, safe background/integration execution, deployment portability, explicit negative testing.  
Cost: scope context must be propagated and validated across more technical boundaries; shared infrastructure requires strong enforcement evidence.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Hidden global/default Tenant | Explicitly prohibited; fail closed. |
| Institution treated as independent root | Require Tenant parent validation. |
| Peer specs duplicate A04 ownership | A04 owns semantics; peers own their realization only. |
| Shared infrastructure weakens separation | B02/D03/E04 must preserve the A04 invariant independent of placement. |

## 10. Migration / Exit Position

No technology is selected. Any replacement must preserve Tenant-rooted scope, subordinate Institution validation, fail-closed behavior and cross-boundary propagation. Replacement requires explicit supersession.

## 11. Traceability

Primary realization: `L5-A-04-001_Tenant_and_Institutional_Isolation_Architecture.md`; `L5-A-04-002_Tenant_and_Institution_Context_Propagation_and_Data_Isolation_Architecture.md`.

Pilot provenance: `ADR-C1` in `L5_Pilot_ADR_Candidate_Disposition_2026-09-09.md`.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** Mandatory ADR applicability and Shared / Family scope were confirmed; the decision preserves the A04-001/A04-002 split, upstream ownership, fail-closed isolation and technology neutrality. Controlled remediation was explicitly authorized on 2026-09-11. This approval does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| A04 technical decision owner | Recommended | 2026-09-10 | A04 Stage-1 specifications |
| SAOS architecture reviewer | PASS | 2026-09-11 | Stage-1 production architecture review |
| Architecture promotion authorization | Approved | 2026-09-11 | Explicit remediation authorization |
