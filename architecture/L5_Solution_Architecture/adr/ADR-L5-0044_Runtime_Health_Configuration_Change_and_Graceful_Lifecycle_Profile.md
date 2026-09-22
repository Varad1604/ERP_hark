# ADR-L5-0044 — Runtime Health, Configuration Change & Graceful Lifecycle Profile

**ADR ID:** `ADR-L5-0044`  
**Title:** Runtime Health, Configuration Change & Graceful Lifecycle Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-E-04-001`; `L5-E-04-002`; `L5-E-04-005`; collaborates with E01/E05/F01/E03/D02  
**Related ADRs:** `ADR-L5-0014`; `ADR-L5-0017`; `ADR-L5-0030`; `ADR-L5-0043`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

E04 already requires lifecycle/readiness hooks and F01 already requires immutable artifact promotion. Backend preparation requires one runtime lifecycle profile covering health semantics, graceful termination and governed configuration/deployment change.

## 2. Decision

1. **Immutable release consumption:** E04 deploys the exact immutable artifact digest promoted by F01; environments do not rebuild release artifacts. Environment differences are configuration/profile differences, not artifact-byte differences.
2. **Governed configuration change:** durable production runtime/configuration changes flow through version-controlled, attributable configuration/deployment pipelines. Manual mutation inside running containers/pods is not normal configuration management.
3. **Emergency operations:** emergency runtime/configuration changes are exceptional, explicitly authorized, attributable, auditable and reconciled back into canonical configuration/source immediately after stabilization.
4. **Health semantics:** managed runtime units expose separate **startup**, **liveness** and **readiness** semantics where the runtime/platform supports them.
5. **Startup:** startup health prevents premature restart/liveness action while legitimate initialization is in progress, subject to profile limits.
6. **Liveness:** liveness indicates whether the process/runtime is internally able to continue execution; it shall not be used merely because a remote optional dependency is unavailable.
7. **Readiness:** readiness indicates whether the runtime can safely accept the applicable work. Mandatory dependencies/configuration/security prerequisites for that work may make the runtime not-ready.
8. **Graceful shutdown:** before normal termination, runtimes stop admitting new work, mark themselves non-ready, drain or safely hand off accepted in-flight requests/jobs within the applicable profile, settle/release leases/claims safely, flush required non-secret operational telemetry where practical, and terminate.
9. **No universal timing:** startup, termination grace, drain, timeout and probe intervals are deployment/capacity-profile values; no universal numeric values are invented.
10. **Stateful shutdown/recovery:** stateful supporting services use their own product/E03-consistent shutdown/recovery procedures rather than application-process assumptions.
11. **Operational administration:** runtime operations consume D02 privileged-access architecture: named attributable identities, least privilege, explicit scope, fresh authorization, stronger authentication posture where required, temporary/elevated access where the authoritative policy calls for it, and D04 evidence. Shared permanent human admin credentials are prohibited.
12. **Backup ownership:** E03 centrally governs backup/restore/DR requirements and validation across PostgreSQL, object storage, platform configuration and other stateful services. Individual application modules do not invent independent backup authority.

## 3. Conformance Rules

- A successful liveness probe is not a business-health assertion.
- A failed optional dependency does not automatically make a runtime not-ready unless the applicable function cannot be served safely.
- Termination must not acknowledge/drop accepted durable work incorrectly.
- Emergency mutations cannot become undocumented permanent drift.
- Readiness logic shall not bypass D02/D03 or fabricate dependency success.
- Backup completion alone is not proof of recoverability; E03 restore/validation evidence remains authoritative.

## 4. Consequences

Operations can reason consistently about safe admission, restart, deployment, change and termination without hard-coding environment-specific timings into architecture.

## 5. Approval Record

**Decision selected:** same immutable digest promoted through environments; governed configuration pipeline; exceptional reconciled emergency change; separate startup/liveness/readiness; graceful draining/shutdown; E03 centralized recovery; D02-governed named privileged access.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
