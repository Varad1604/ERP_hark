# ADR-D-MON-PURITY-001 — Academic Monitoring Business-Architecture Purity

| Attribute | Value |
|---|---|
| ADR ID | ADR-D-MON-PURITY-001 |
| Date | 2026-08-24 |
| Status | Approved |
| Architecture Level | L2 — Domain Workflow Architecture |
| Domain | D05 — Academic Monitoring |
| Branch | dhruv |
| Approval Source | Explicit repository-change authorization recorded in the architecture working session: “Go ahead I authorize you.” |

## 1. Context

Locked `L2-D05_Academic_Monitoring_Workflows.md` includes solution-oriented execution terminology such as durable workflow state, idempotent processing, compensation workflows and controlled parallel execution. SutraERP L2 is business workflow architecture and shall remain implementation independent.

The underlying institutional requirements are valid, but their expression must remain at business-architecture level so that L3 does not inherit implementation mechanics.

## 2. Decision

The implementation-oriented terms are interpreted canonically as business requirements:

1. Interrupted monitoring preserves the last valid authoritative business position and evidence required for governed continuation.
2. Repeated receipt or reconsideration of the same authoritative business fact shall not create duplicate institutional monitoring outcomes or governance consequences.
3. Corrections, withdrawals and reversals occur only through explicitly governed business paths and preserve prior history.
4. Concurrent monitoring activities are permitted only where their governed scopes, authorities and decisions do not conflict or create duplicate authoritative outcomes.
5. Technical mechanisms for persistence, retries, deduplication, locking, transactions, orchestration, queues, scheduling or infrastructure belong to later architecture levels.
6. Automated/system execution may support calculation, evaluation or publication where already permitted, but it does not become an institutional business actor or governance authority at L3.

## 3. Preserved Architecture

This decision does not change D05 ownership, aggregates, lifecycle states, business outcomes, business rules, event names, publishers, governance authority or cross-domain ownership.

## 4. Repository Effect

`L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md` is authoritative for this clarification. Downstream L3 scenarios shall use business-level continuity, duplicate-prevention and actor semantics and shall avoid implementation-level recovery or execution mechanisms.

## 5. Traceability

- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D05_Academic_Monitoring_Workflows.md`
- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D05_Business_Architecture_Purity_Amendment_2026-08-24.md`
- `governance/constitution/Volume_5_Change_Management.md`
