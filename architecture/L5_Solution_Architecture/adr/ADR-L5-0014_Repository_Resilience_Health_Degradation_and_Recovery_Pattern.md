# ADR-L5-0014 — Repository Resilience, Health, Degradation & Recovery Pattern

**ADR ID:** `ADR-L5-0014`  
**Title:** Repository Resilience, Health, Degradation & Recovery Pattern  
**Scope:** Repository  
**Decision Owner:** L5-E-01 — Availability & Resilience Architecture  
**Assigned Author:** Architecture remediation for Varad Stage-3 corpus  
**Reviewers:** Controlled L5 architecture remediation review  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-15  
**Affected L5 Specifications:** `L5-E-01-001`; `L5-E-01-002`; constrains reliability-aware L5 consumers  
**Related ADRs:** `ADR-L5-0003`; `ADR-L5-0005`; `ADR-L5-0006`; `ADR-L5-0010`; `ADR-L5-0013`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

SutraERP requires one repository-level resilience model so that availability, health, degradation, failure containment, recovery and restoration are interpreted consistently across technical capabilities. Without a shared decision, each specification could define incompatible health states, retry/recovery semantics or failover assumptions.

The decision must remain technology-neutral, preserve on-premises-primary and cloud-capable deployment, avoid fabricated business availability promises, and preserve canonical ownership of business state and recovery decisions.

## 2. Scope & Affected Architecture

This ADR governs technical resilience semantics: health-state interpretation, failure containment, controlled degradation, recovery stages, restoration validation and return-to-service criteria.

It does not establish business continuity policy, contractual SLA/SLO values, RTO/RPO values, business workflow recovery policy, product-specific clustering, cloud-provider services, runtime packaging or database replication technology.

## 3. Authoritative Constraints

1. Health state is a technical operational representation and shall not redefine business lifecycle state.
2. Canonical repository health states are `Healthy`, `Degraded` and `Unavailable` at the relevant technical scope.
3. Degradation is permitted only where a predeclared capability profile or equivalent governed rule identifies what remains safe and authoritative.
4. A degraded capability shall never fabricate success, authoritative freshness or business completion for functions that are unavailable or stale.
5. Failure containment shall prefer the smallest safe technical blast radius consistent with data and business correctness.
6. Recovery shall proceed through explicit detection, isolation, repair/restart/failover as applicable, validation, and controlled return to service.
7. Restoration is complete only after integrity/health validation succeeds; absence of an active error alone is insufficient.
8. State-owning components retain ownership of authoritative state during failover/recovery; E01 does not gain cross-owner write authority.
9. E04 owns physical runtime packaging, placement and deployment topology. E01 defines resilience requirements that E04 must realize.
10. E02 owns capacity/performance mechanisms; resilience may consume capacity signals but shall not redefine capacity architecture.
11. A08 owns technical background execution/retry; E01 may constrain recovery policy but shall not duplicate job/retry ownership.
12. C02 retains durable event delivery/recovery semantics for messaging; E01 provides repository resilience constraints only.
13. The pattern must support on-premises-primary deployment and remain cloud/provider neutral.
14. No technology, vendor, numeric target or topology count is selected by this ADR.

## 4. Decision Criteria

The model must make failure states explicit, contain blast radius, preserve state ownership, support safe degraded operation, prevent premature restoration, integrate with E04/E02/A08/C02, and remain usable across both co-located and selectively isolated runtime units.

## 5. Alternatives Considered

### A — Binary up/down status only
Rejected because it cannot safely represent partial capability loss or controlled degradation.

### B — Each subsystem defines independent health/degradation semantics
Rejected because repository-wide operational interpretation becomes inconsistent and downstream automation cannot safely reason about state.

### C — Shared three-state health model with explicit degradation profiles and validated recovery
Selected.

### D — Automatic failover/restoration whenever a health probe changes
Rejected because probe state alone is insufficient to prove authoritative state integrity or business-safe restoration.

## 6. Decision

SutraERP shall use a shared technical resilience model with scoped health and recovery state.

### 6.1 Health Model

At each governed technical scope:

- **Healthy** — required capability is available and its authoritative technical prerequisites are valid.
- **Degraded** — capability remains intentionally available with a declared limitation; callers and operators can determine the limitation.
- **Unavailable** — the capability cannot safely provide its required operation.

Health state is scoped; one unhealthy unit shall not automatically imply repository-wide unavailability.

### 6.2 Degradation Model

Every allowed degraded mode shall declare:

- affected capability/scope;
- functions that remain permitted;
- functions that are blocked;
- authoritative/staleness limitations;
- caller-visible technical signal;
- exit/recovery condition.

Undeclared degradation is prohibited for business-critical behavior; fail closed instead.

### 6.3 Recovery Pipeline

The canonical technical sequence is:

```text
Detect
 -> classify affected scope
 -> contain/isolate
 -> establish safe recovery action
 -> repair/restart/failover/replay as owned by the responsible architecture
 -> validate health + state integrity
 -> ControlledReturn
 -> Healthy or declared Degraded
```

Recovery actions that affect owner state execute only through the owner’s contract. E01 coordinates technical resilience criteria; it does not directly mutate another owner’s business data.

### 6.4 Restoration Gate

Return to Healthy requires successful validation of all applicable prerequisites, including state consistency/integrity, required dependencies, authority/context validity and absence of unresolved recovery work that would make normal operation unsafe.

## 7. Integration Boundaries

### E01 → E04
E01 supplies logical resilience requirements: isolation scope, restart/failover eligibility, required redundancy behavior and restoration validation. E04 owns concrete packaging, hosts/processes/containers, orchestration and deployment realization.

### E01 → E02
E02 supplies capacity and backpressure signals. E01 may mark a capability Degraded/Unavailable when safe capacity is exhausted but shall not define queue/capacity algorithms.

### E01 → A08
A08 owns background job execution/retry. E01 may require pausing release, isolating a worker group or delaying restoration when execution safety is uncertain; it does not create retry attempts itself.

### E01 → C02
C02 owns event delivery/replay/recovery. E01 may consume messaging health and require isolation/restoration criteria; it does not redefine event delivery semantics.

### E01 → B02
B02 owns persistence/concurrency. Recovery/failover shall preserve B02 authority and consistency rules; E01 cannot bypass them.

## 8. Consequences

Positive: consistent operational health semantics, explicit safe degradation, smaller failure blast radius, governed restoration and clearer cross-family ownership.  
Cost: every degraded mode and recovery path requires explicit contracts, validation evidence and operational instrumentation.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Degraded mode serves unsafe/stale data | Require explicit degradation profile and authoritative/staleness limits. |
| Automatic recovery corrupts owner state | Owner-contract-only mutation plus integrity validation before return. |
| Health probes give false confidence | Health is evidence input, not sole restoration authority. |
| E01 duplicates E04 topology | Keep placement/runtime realization with E04. |
| Retry storms during recovery | A08/C02 retain bounded retry; E01 can constrain release during degraded state. |
| Provider-specific failover leaks into architecture | Keep products/topology outside ADR and E04/A02 governed. |

## 10. Migration / Exit Position

Any runtime may realize this model using different products or topology if the health, degradation, containment, ownership and restoration invariants are preserved. Incompatible resilience semantics require a superseding ADR.

## 11. Traceability

Primary realization: `L5-E-01-001` and `L5-E-01-002`.  
Mandatory Stage-3 dependencies: E01 consumes A01 and E04.  
Relevant restoration behavior includes `L4-A-057-BH-006` through `BH-008` where the controlled platform-change scenario applies.  
This ADR intentionally does not claim `L4-A-057-BH-001` as a restoration obligation.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The resilience/health/degradation/recovery pattern is repository-wide and therefore satisfies mandatory ADR criteria. The user explicitly authorized remediation of the production-audit findings on 2026-09-15. Approval does not select infrastructure technology and does not imply Locked status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| E01 technical decision owner | Recommended | 2026-09-15 | E01 Stage-3 remediation architecture |
| L5 architecture remediation reviewer | PASS | 2026-09-15 | Production audit corrective action |
| Architecture remediation authorization | Approved | 2026-09-15 | Explicit user request to fix highlighted issues |