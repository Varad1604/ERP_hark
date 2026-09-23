# SutraERP L5 R3C — Non-Human Principal Lifecycle Resolution — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect / L5-D-01  
**Programme:** Post-Validation Upstream Blocker Resolution & Final-Lock Readiness  
**Scope:** `L5-D-01-002`; `L5-D-01-004`; related D01/D02/D03 technical consumers  
**Decision ADR:** `ADR-L5-0048`  

## 1. Decision

R3C resolves the remaining D01 non-human principal lifecycle gap as a genuine L5-owned technical architecture decision.

Canonical lifecycle:

```text
Provisioned → Active ↔ Suspended → Retired
```

`Retired` is terminal.

The lifecycle applies to explicit service, workload, job and integration technical principals.

## 2. Ownership

- D01-002 owns technical-principal identity, category, Tenant binding, technical owner, lifecycle state and lifecycle events.
- D01-004 owns provisioning/lifecycle coordination, idempotency, reconciliation and completion barriers.
- D01-003/D03 own authentication credential/session and secret/key mechanics.
- D02 owns authority binding, permission/scope/policy evaluation and Authorization Decision.
- No Person or User Account is created for a machine principal.

## 3. Lifecycle Events

The D01 technical principal registry owns:

- `TechnicalPrincipalProvisioned`
- `TechnicalPrincipalActivated`
- `TechnicalPrincipalSuspended`
- `TechnicalPrincipalResumed`
- `TechnicalPrincipalRetired`

These are technical lifecycle events, not business Person/User Account events.

## 4. Impact Review

### D01-002

Material update required and applied:

- lifecycle state model established;
- OD-003 closed;
- version advanced to 1.3.0;
- ADR-L5-0048 added.

### D01-004

Material update required and applied:

- non-human lifecycle coordination now consumes ADR-L5-0048;
- OD-007 closed;
- version advanced to 1.1.0;
- ADR-L5-0048 added.

### D01-003

No ownership change.

Its authentication/session/credential responsibility remains intact. It consumes Active/Suspended/Retired principal eligibility as context and realizes authentication-side consequences through its existing responsibility.

### D02

No ownership change.

`ADR-L5-0009` remains authoritative for Technical Principal Authority Binding. Principal lifecycle state does not grant permission.

### D03

No ownership change.

Machine credential, key, certificate and secret custody remain D03-governed.

## 5. ADR Allocation

R3C allocates:

`ADR-L5-0048 — Non-Human Technical Principal Lifecycle Profile`

After merge, the next canonical allocation is:

`ADR-L5-0049`

No other ADR is required for R3C.

## 6. Validation

The R3C architecture satisfies the required boundaries:

- no fabricated human Person/User Account;
- no duplicate authorization ownership;
- no implicit internal-network trust;
- Tenant binding mandatory;
- accountable technical owner mandatory;
- lifecycle and credential ownership separated;
- lifecycle and authorization separated;
- Retired terminal;
- stale activation cannot override later suspension/retirement;
- history/evidence preserved.

## 7. Programme Effect

R3C closes the non-human principal lifecycle semantic blocker recorded in D01.

It does **not** by itself clear the full D01 Batch-9 blocker population because:

- the R3A/R3B upstream branches must become canonical;
- exact affected L4→L5 relations must be validated;
- D01-001…004 require targeted HF revalidation;
- the 105-row production validation register remains unchanged until that revalidation is complete.

## 8. Final Decision

```text
ADR-L5-0048                         LOCKED
D01-002 OD-003                      RESOLVED
D01-004 OD-007                      RESOLVED
D01-003 / D02 / D03 ownership       UNCHANGED
L5 D01 blocker dispositions         NOT YET CHANGED
Final L5 Corpus Lock                NOT AUTHORIZED
```

**Final Decision:** `R3C PASS — NON-HUMAN TECHNICAL PRINCIPAL LIFECYCLE GOVERNED; PROCEED TO CANONICAL MERGE / D01 TRACEABILITY AND TARGETED REVALIDATION`.
