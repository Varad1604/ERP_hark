# ADR-L5-0037 — Valkey Derived Cache & Failure Behavior Standard

**ADR ID:** `ADR-L5-0037`  
**Title:** Valkey Derived Cache & Failure Behavior Standard  
**Scope:** Shared / Family  
**Decision Owner:** L5-E-02 — Performance & Capacity Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-E-02-002`; constrains cache-using L5 capabilities including `L5-B-06-002`  
**Related ADRs:** `ADR-L5-0005`; `ADR-L5-0014`; `ADR-L5-0020`; `ADR-L5-0036`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

E02 already defines cache validity, backpressure and degradation semantics but deliberately left the cache product unresolved. Backend preparation now requires a shared distributed cache product while preserving the rule that caches never become canonical business truth.

## 2. Decision

1. **Product:** **Valkey** is the canonical shared distributed cache product; production uses a repository-approved supported stable release.
2. **Authority:** cached values are derived/non-authoritative. Authoritative facts, lifecycle state, durable work and canonical events remain with their owners.
3. **Freshness:** freshness-critical operations revalidate against the canonical owner or explicitly bypass cache.
4. **Failure behavior:** cache unavailability shall never trigger direct cross-owner database access. Where safe, fall back through the canonical owner path; otherwise return a governed degraded/unavailable outcome.
5. **Scope isolation:** entries are bound to environment and canonical owner, plus Tenant/Institution scope where applicable.
6. **Invalidation:** owner-driven and/or event-driven invalidation may be used. TTL is a freshness mechanism, not ownership; no universal TTL is locked.
7. **Eviction:** eviction may remove cache state without loss of authoritative data. Eviction policy/topology stays profile-driven.
8. **No hidden authority:** Valkey is not automatically the canonical lock manager, scheduler, queue, session authority, event stream or distributed transaction coordinator.

## 3. Conformance Rules

- Cache miss/loss is a normal technical condition.
- Cache success does not prove canonical freshness unless the owner contract guarantees the applicable freshness class.
- Sensitive cached data remains subject to B05/D03/D02.
- Cache namespaces cannot be used to infer Tenant/Institution authority.
- Cross-domain reports should prefer governed B06 projections rather than turning Valkey into a system of record.

## 4. Consequences

SutraERP gains a standard distributed cache while keeping correctness independent of cache availability.

## 5. External Evidence

- Valkey documentation: https://valkey.io/docs/
- Valkey introduction: https://valkey.io/topics/introduction/

## 6. Approval Record

**Decision selected:** Valkey as shared distributed cache; derived/non-authoritative data only; canonical-owner fallback or explicit degradation; no cross-owner database bypass.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
