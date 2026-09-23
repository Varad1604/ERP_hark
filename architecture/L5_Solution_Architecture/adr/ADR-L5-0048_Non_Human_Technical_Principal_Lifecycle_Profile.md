# ADR-L5-0048 — Non-Human Technical Principal Lifecycle Profile

**ADR ID:** `ADR-L5-0048`  
**Title:** Non-Human Technical Principal Lifecycle Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-01 — IAM Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-23  
**Affected L5 Specifications:** `L5-D-01-002`; `L5-D-01-004`; constrains `L5-D-01-003`, `L5-D-02-*`, `L5-D-03-*`, `L5-A-08-*`, `L5-C-03-*` and protected technical consumers  
**Related ADRs:** `ADR-L5-0009`; `ADR-L5-0024`; `ADR-L5-0039`; `ADR-L5-0040`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

The existing D01 architecture already distinguishes human account principals from non-human service/workload/job/integration principals.

`ADR-L5-0039` governs how non-human principals authenticate and bind into D02 authorization, but it intentionally does not define the lifecycle state model of the principal identity itself.

Production implementation cannot safely invent lifecycle semantics independently. A canonical D01-owned technical lifecycle is therefore required.

This is an L5 technical-identity decision. It does not create a Person, Identity or User Account business lifecycle and does not move business authority into L5.

## 2. Decision

### 2.1 Canonical Principal Categories

The lifecycle applies to D01 non-human technical principals classified as:

- service principal;
- workload principal;
- job principal;
- integration principal.

Every principal remains:

- explicitly modeled;
- Tenant-bound;
- associated with an accountable technical owner;
- independently revocable;
- separate from any human Person/User Account;
- subject to D02 authorization after authentication.

### 2.2 Canonical Lifecycle States

The canonical technical lifecycle is:

```text
Provisioned
    ↓
Active
    ↓
Suspended
    ↓
Active
```

and terminal retirement paths:

```text
Provisioned → Retired
Active      → Retired
Suspended   → Retired
```

`Retired` is terminal.

No other non-human principal lifecycle state is canonical unless introduced by a later ADR.

### 2.3 State Semantics

| State | Meaning |
|---|---|
| `Provisioned` | The technical principal record exists, is Tenant-bound, has an accountable technical owner, and may have credential/bootstrap preparation in progress, but is not eligible for normal authenticated execution. |
| `Active` | The principal is eligible to authenticate through the approved D01/D03 mechanism and may participate in protected execution subject to D02 authorization and all applicable scope/policy checks. |
| `Suspended` | Authentication/use is temporarily blocked while the principal record and history remain preserved. |
| `Retired` | The principal is permanently withdrawn from normal authenticated execution; historical identity and evidence remain preserved. |

### 2.4 Canonical Technical Operations

The D01 lifecycle operations are:

- `ProvisionTechnicalPrincipal`;
- `ActivateTechnicalPrincipal`;
- `SuspendTechnicalPrincipal`;
- `ResumeTechnicalPrincipal`;
- `RetireTechnicalPrincipal`.

These are L5 technical architecture operations, not business-domain commands.

### 2.5 Canonical Technical Lifecycle Events

The D01 technical principal registry is the canonical publisher of:

- `TechnicalPrincipalProvisioned`;
- `TechnicalPrincipalActivated`;
- `TechnicalPrincipalSuspended`;
- `TechnicalPrincipalResumed`;
- `TechnicalPrincipalRetired`.

These are technical lifecycle events and shall not be presented as Person/User Account business events.

### 2.6 Activation Gate

A non-human principal may enter `Active` only when all applicable conditions are satisfied:

1. exactly one Tenant context is resolved;
2. an accountable technical owner is recorded;
3. the principal category is one of the governed categories;
4. required D01 authentication mechanism/binding exists;
5. required credential/secret material is governed by D03;
6. any required D02 Technical Principal Authority Binding is valid for the intended protected execution;
7. no contradictory suspension/retirement condition exists.

Authentication eligibility and authorization remain separate. Reaching `Active` does not itself grant any Permission or capability.

### 2.7 Suspension and Resume

Suspension:

- blocks new authenticated execution for the principal;
- requires D01/D03 credential or session consequences appropriate to the selected authentication mechanism;
- leaves D02 authority bindings non-effective for new protected execution while the principal cannot authenticate;
- preserves identity, technical-owner, Tenant and evidence history.

Resume requires:

- resolution of the suspension cause;
- current valid Tenant/owner context;
- revalidation of authentication mechanism/credential eligibility;
- revalidation of applicable D02 authority binding where required.

Resume returns `Suspended → Active`.

### 2.8 Retirement

Retirement:

- is permanent for that technical principal identity;
- blocks future authenticated execution;
- requires revocation/termination of applicable credential/session capability through D01-003/D03;
- makes D02 authority bindings ineffective for future execution;
- preserves historical identity and audit/evidence references;
- shall not be implemented as deletion of the technical principal record.

A retired identity shall not return to `Active`. A materially new principal requires a new principal identity.

## 3. Ownership Boundaries

### D01-002 Owns

- non-human technical principal identity;
- category;
- Tenant binding;
- accountable technical-owner association;
- canonical technical lifecycle state;
- lifecycle transition validation;
- lifecycle event publication.

### D01-004 Owns

- provisioning/lifecycle coordination;
- idempotent transition orchestration;
- reconciliation after partial/interrupted operations;
- coordination of D01-003/D03 credential/session consequences;
- lifecycle completion barriers and evidence handoff.

### D01-003 / D03 Own

- authentication mechanism;
- credential/session realization;
- secret/key/certificate protection and revocation mechanics.

### D02 Owns

- Permission/capability authority;
- Technical Principal Authority Binding;
- scope/policy evaluation;
- final Authorization Decision.

No lifecycle state in this ADR grants authorization.

## 4. Conformance Rules

1. Non-human principals shall never be represented by fabricated Person or User Account records.
2. Every principal shall resolve to exactly one Tenant before `Active`.
3. Every principal shall have an accountable technical owner before `Active`.
4. `Provisioned` principals shall not perform normal protected execution.
5. Only `Active` principals may proceed to authentication/authorization evaluation.
6. `Suspended` and `Retired` principals shall fail closed for new authenticated execution.
7. `Retired` is terminal.
8. Credential/session revocation is coordinated with lifecycle transitions but remains D01-003/D03 owned.
9. Authorization remains D02 owned and cannot be inferred from lifecycle state or Keycloak/service-account existence.
10. Lifecycle operations shall be Tenant-safe, idempotent and attributable.
11. Replayed/stale activation shall not override a later Suspension or Retirement.
12. Historical principal references and lifecycle evidence shall remain preserved.
13. Provider/integration credential lifecycle shall not silently substitute for technical-principal lifecycle.
14. Human and non-human credentials shall never be interchangeable.

## 5. Interaction with Existing ADRs

### ADR-L5-0039

Remains authoritative for:

- non-human authentication profile;
- Keycloak service-account / approved workload mechanisms;
- no internal-network trust;
- human/non-human credential separation.

This ADR adds lifecycle semantics only.

### ADR-L5-0009

Remains authoritative for D02 Technical Principal Authority Binding.

This ADR does not alter D02 permission/scope/policy ownership.

### ADR-L5-0040

Remains authoritative for machine credential and secret/key custody.

This ADR does not select cryptographic mechanisms.

## 6. Failure and Recovery

| Condition | Required Outcome |
|---|---|
| Tenant unresolved/mismatched | Block provisioning/activation/use. |
| Technical owner absent | Do not activate. |
| Credential/authentication preparation incomplete | Remain `Provisioned` or `Suspended`; do not claim `Active`. |
| D02 authority binding absent where protected execution requires it | Principal may remain identity-valid but cannot obtain protected authority. |
| Suspension consequence incomplete | Do not report completed suspension where authentication remains usable. |
| Retirement consequence incomplete | Do not report completed retirement while normal authentication remains possible. |
| Stale activation after suspension/retirement | Suppress stale activation and preserve later authoritative lifecycle state. |
| Duplicate transition request | Apply idempotently; do not duplicate principal identity or irreversible consequence. |

## 7. Consequences

This decision closes the previously unresolved non-human lifecycle semantics in D01 while preserving the established separation between:

- technical principal identity;
- authentication mechanism/credential;
- authorization authority;
- business Person/User Account identity.

No new product selection is introduced.

## 8. Approval Record

**Decision selected:** D01-owned non-human technical principal lifecycle `Provisioned → Active ↔ Suspended → Retired`, with `Retired` terminal; explicit Tenant/technical-owner binding; lifecycle events owned by the D01 principal registry; D01-003/D03 retain credential/session mechanics; D02 retains authorization.  
**Approval source:** Explicit Lead Architect authorization through the R3C post-validation blocker-resolution continuation on 2026-09-23.  
**Formal Status:** `Locked`.
