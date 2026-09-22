# ADR-L5-0034 — Synchronous API Wire, Versioning, Error & Idempotency Profile

**ADR ID:** `ADR-L5-0034`  
**Title:** Synchronous API Wire, Versioning, Error & Idempotency Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-C-01 — API & Contract Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-C-01-001`; `L5-C-01-002`; `L5-C-01-003`  
**Related ADRs:** `ADR-L5-0003`; `ADR-L5-0004`; `ADR-L5-0006`; `ADR-L5-0019`; `ADR-L5-0021`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

ADR-L5-0021 locked HTTPS + REST/JSON + OpenAPI 3.2 as the default synchronous API style, but the repository still requires one canonical wire/profile convention for JSON member naming, contract source-of-truth, major-version routing, technical error representation and retry-safe idempotency.

## 2. Scope & Ownership

This ADR is owned by C01. It defines technical synchronous-contract representation only. It does not create business commands, outcomes, rules, lifecycle states, error semantics or authorization policy; those remain with their canonical upstream owners.

## 3. Decision

SutraERP synchronous service contracts shall use:

1. **Contract-first authority:** released OpenAPI 3.2 contracts are the implementation-facing source of truth for synchronous APIs; Rust/Axum code conforms to the contract rather than becoming architecture authority.
2. **JSON naming:** SutraERP-defined JSON object member names use **camelCase**. Internal Rust identifiers may remain idiomatic `snake_case`.
3. **Version routing:** breaking major API generations use explicit URL-major routing such as `/api/v1/...`. Backward-compatible additive evolution stays within the current major version.
4. **Technical errors:** HTTP/API technical failures use **RFC 9457 Problem Details** (`application/problem+json`) with governed SutraERP extensions such as `errorCode`, `correlationId` and structured `validationErrors` where applicable.
5. **Business outcomes:** Problem Details shall not replace a source-domain business rejection/outcome when that outcome is part of the canonical business contract.
6. **Idempotency:** retry-sensitive mutating commands use the canonical `Idempotency-Key` request header and owner-scoped idempotency semantics. The same key with materially different request content is rejected.
7. **Internal synchronous communication:** REST/OpenAPI remains the default even between selectively isolated runtime units. gRPC remains exception-only under ADR-L5-0021 when measured technical requirements justify it.

## 4. Conformance Rules

- Generated or handwritten server/client code may not silently diverge from the released OpenAPI contract.
- Framework-specific Axum/Rust types shall not be exposed as canonical contract types.
- A new `/v2` route is created only for a materially breaking contract generation, not for ordinary additive change.
- Idempotency storage and retention mechanics remain implementation/profile concerns under C01/B02/E02; no universal duration is invented here.
- Tenant/Institution, authenticated-principal and authorization context remain governed by A04/D01/D02 and are not redefined by wire format.
- Correlation identifiers are technical context and do not become business identifiers.

## 5. Consequences

The backend and future clients share one predictable synchronous wire profile. Contract generation, linting and compatibility tests can be automated. The cost is a deliberate public contract discipline that implementation teams cannot bypass for convenience.

## 6. Migration / Exit Position

A replacement of JSON/OpenAPI/version/error/idempotency conventions requires a superseding C01 ADR and compatibility/migration plan. Canonical business semantics remain unchanged.

## 7. External Standards Evidence

- OpenAPI 3.2: https://spec.openapis.org/oas/v3.2.0.html
- RFC 9457 Problem Details: https://www.rfc-editor.org/rfc/rfc9457.html

## 8. Approval Record

**Decision selected:** camelCase JSON; contract-first OpenAPI 3.2; URL major versioning; RFC 9457 Problem Details + SutraERP extensions; selective `Idempotency-Key`; REST/OpenAPI default and gRPC by exception.  
**Approval source:** Explicit Lead Architect approval in the SutraERP architecture workflow on 2026-09-22.  
**Formal Status:** `Locked`.
