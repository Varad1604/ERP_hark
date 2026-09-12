# ADR-L5-0009 — Non-Human Principal Authorization Binding Pattern

**Status:** Architecture Approved  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-02 — Authorization Architecture  
**Allocated On:** 2026-09-12  
**Decision Date:** 2026-09-12  
**Affected Specifications:** `L5-D-02-001`; `L5-D-02-002`; `L5-D-02-003`; `L5-D-02-004`; `L5-D-02-006`; consumes `L5-D-01-002`  
**Depends On:** `L5-D-01-002_Identity_Account_and_Service_Identity_Architecture.md`; `L5-A-04-001`; `L5-A-04-002`; `L5-D-03-001`; `L5-D-03-004`; `L5-D-02-001` through `L5-D-02-004`; L5 Technology Selection & ADR Methodology; L5 ADR Register

---

## 1. Context

SutraERP L5 already establishes a canonical technical identity model for non-human service, workload, job and integration principals in `L5-D-01-002`.

That architecture establishes that a non-human principal:

- is explicitly modeled as a technical principal rather than fabricated as a Person or human User Account;
- is bound to exactly one Tenant for execution;
- carries a governed technical owner/association reference;
- has identity separated from credential/secret material;
- can preserve an originating human actor separately from the technical executor when both participate;
- supplies identity context only and does not itself grant Roles, Permissions or authorization.

The D02 authorization family requires those principals to perform protected service-to-service, background-job, integration and privileged machine operations. The initial D02 Stage-2 drafts intentionally did not fabricate a Person-based Role Assignment for machine identities, but consequently left the authority-binding model unresolved.

The missing decision is therefore technical and cross-specification:

```text
How does a canonical D01 non-human principal receive bounded D02 authority
without becoming a Person, User Account or business Role Assignment subject?
```

This decision is required to make the D02 architecture implementable for non-human execution while preserving upstream business semantics.

---

## 2. Authoritative Constraints

The decision shall preserve all of the following:

1. `L5-D-01-002` owns non-human technical-principal identity.
2. A non-human principal shall not be represented by a fabricated Person or reused human User Account.
3. Every principal is Tenant-bound before protected execution.
4. `ADR-PLAT-AUTH-001` remains authoritative for human Role Assignment: canonical human Role Assignment subject is Person.
5. D02 owns authorization architecture, not identity or credential lifecycle.
6. A Permission/capability is not a final Authorization Decision.
7. Scope, resource context, policy, SoD, explicit Deny, trust and freshness controls remain mandatory.
8. Business resources/actions and business policy remain source-domain owned.
9. Technical-principal authority shall not become an implicit global service-account superuser model.
10. No technology product, token format, policy engine or workload-identity platform is selected by this ADR.

---

## 3. Decision

### 3.1 Establish a D02 Technical Principal Authority Binding

SutraERP shall use a distinct D02-owned technical construct named **Technical Principal Authority Binding** for non-human authorization.

It is a technical authorization binding, not a new L1-L4 business entity and not a human Role Assignment.

Conceptually:

```text
TechnicalPrincipalAuthorityBinding
{
  binding_reference

  tenant_reference
  institution_reference where applicable
  lower_scope_references[] where governed

  technical_principal_reference      // canonical D01-002 non-human principal
  technical_owner_association_reference

  permission_or_capability_references[]
  authorization_policy_reference(s) where applicable

  effective_from where applicable
  expires_at where applicable
  authority_status / validity reference governed at L5

  provenance / approval-or-governance-reference
  version / freshness reference
  correlation / evidence reference
}
```

This is a conceptual architecture contract. It does not prescribe an API payload, database schema or product.

### 3.2 Binding Ownership

Canonical ownership is split as follows:

```text
D01-002
  owns non-human principal identity
        ↓
D02
  owns Technical Principal Authority Binding
        ↓
D02-002
  resolves Permissions / capabilities
        ↓
D02-003
  constrains Tenant / Institution / resource scope
        ↓
D02-004
  evaluates policy and produces Allow / Deny
```

D01 never stores the authorization grant.

D02 never creates or mutates the principal's identity/credential truth.

### 3.3 Human Role Assignment Remains Unchanged

This ADR does not generalize human Role Assignment to machines.

For humans:

```text
Person
  ↓
Role Assignment
  ↓
Role / Permissions
```

For non-human principals:

```text
D01 Non-Human Principal
  ↓
Technical Principal Authority Binding
  ↓
Permissions / Capabilities
```

The two models converge only at the effective-authority and policy-evaluation boundary.

### 3.4 Permission Binding

A Technical Principal Authority Binding may reference only governed D02 Permissions/capabilities that represent real protected actions/resources.

It shall not grant authority using:

- UI pages;
- routes;
- database roles;
- operating-system privilege alone;
- network location;
- service-account classification;
- credential possession alone.

Possessing a valid machine credential authenticates the technical principal; it does not independently authorize protected execution.

### 3.5 Tenant and Scope Binding

Every binding is rooted in the same Tenant as the D01 non-human principal.

Rules:

1. `principal.tenant == binding.tenant` is mandatory.
2. Institution/lower scope may only narrow valid Tenant-rooted authority.
3. A binding never permits cross-Tenant execution.
4. Missing or ambiguous scope fails closed.
5. Lower-scope relationships remain source-owned and are evaluated through D02-003.
6. Scope is not encoded into the Permission identity merely to avoid D02-003 evaluation.

### 3.6 Policy and Decision Participation

A Technical Principal Authority Binding contributes candidate effective authority only.

It does not itself create `Allow`.

D02-004 shall still evaluate:

- principal validity;
- Tenant/Institution/scope validity;
- requested Permission/capability;
- resource state/ownership context where material;
- applicable authorization policy/version;
- explicit Deny;
- SoD or machine-specific conflict rules where source-governed;
- privileged controls where applicable;
- trust/security prerequisites;
- freshness/revocation state.

### 3.7 Privileged Machine Authority

Privileged non-human execution uses the same binding model with additional D02-006 safeguards.

A binding for a privileged capability shall be:

- explicit;
- least-privilege;
- Tenant/scope bound;
- attributable to the non-human principal and technical owner association;
- policy evaluated;
- freshness/revocation aware;
- evidenced through D04;
- subject to D03 credential/trust controls.

Infrastructure privilege or possession of a powerful credential never substitutes for D02 privileged authorization.

### 3.8 Lifecycle and Revocation

The binding shall have sufficient L5 technical validity semantics to support safe execution without inventing a business lifecycle.

At minimum:

- a binding must be positively valid/effective before it contributes authority;
- expired, revoked, disabled, unresolved or stale bindings contribute no authority;
- any change that narrows/removes machine authority invalidates dependent effective-authority projections/caches;
- reauthorization is required after material binding/scope/policy change;
- exact persistence status names and storage mechanics remain implementation design unless separately governed.

No canonical business event names are created by this ADR.

### 3.9 Attribution

Authorization evidence shall distinguish:

- the executing non-human principal;
- its Technical Principal Authority Binding;
- its technical owner/association reference;
- an originating human actor when D01 execution context supplies one;
- evaluated Permission/capability;
- scope/resource context;
- policy version;
- final Authorization Decision.

The technical principal shall never be presented as the originating Person merely for audit convenience.

---

## 4. Alternatives Considered

### Alternative A — Reuse Human Person Role Assignment for Machines

**Rejected.**

This would fabricate Person semantics, violate D01's explicit non-human principal model and corrupt `ADR-PLAT-AUTH-001` human authorization ownership.

### Alternative B — Attach Permissions Directly to Machine Credentials

**Rejected.**

Credential identity and authorization authority must remain separate. Credential rotation/replacement must not redefine authority.

### Alternative C — Treat Internal/Trusted Services as Automatically Authorized

**Rejected.**

Trust-boundary admission and authorization are separate. Internal placement, network trust or service identity does not imply permission.

### Alternative D — Technical Principal Authority Binding

**Selected.**

It preserves D01 identity ownership, D02 authorization ownership, Tenant isolation and common policy evaluation while giving implementation a concrete machine-authority model.

---

## 5. Consequences

### Positive

- closes the D02 machine-principal implementation gap;
- preserves Person and human Role Assignment semantics;
- provides one reusable model across service, workload, job and integration principals;
- supports privileged machine execution without a universal superuser;
- keeps credential technology independent from authority;
- supports Tenant/scope isolation and auditable machine execution;
- gives A08/C03/service-to-service architecture a canonical authorization dependency.

### Costs / Constraints

- requires D02-owned persistence/projection for technical authority bindings;
- requires authority freshness/revocation coordination with B02/C02 where used;
- requires D02-003 scope support for machine authority;
- requires D02-004 evaluation support for machine bindings;
- requires D02-006 privileged controls when machine capabilities are high-risk;
- requires D04 evidence to retain technical-principal attribution.

---

## 6. Conformance Requirements

1. No non-human principal shall receive protected execution authority through a fabricated Person/User Account Role Assignment.
2. Every machine authority binding shall reference exactly one canonical D01 non-human principal and one Tenant.
3. A binding shall not cross or widen Tenant boundaries.
4. Permission/capability references shall use D02-002 governed authorization semantics.
5. Scope shall be evaluated through D02-003 and may only narrow the validated Tenant-rooted context.
6. A binding alone shall never equal final `Allow`.
7. D02-004 shall evaluate machine authority through the normal policy/deny/enforcement pipeline.
8. Machine credential possession shall never independently establish authorization.
9. Privileged machine authority shall consume D02-006 safeguards.
10. Expired/revoked/stale/unresolved bindings shall contribute no authority.
11. Audit/evidence shall preserve the executing technical principal and binding provenance.
12. No implementation shall infer a mandatory IAM/workload-identity product, token format, policy engine, cache, database or deployment topology from this ADR.

---

## 7. Approval Record

The decision has been independently reviewed in the Devang Stage-2 remediation and found to:

- remain entirely within L5 technical authorization authority;
- consume, rather than redefine, D01 non-human identity ownership;
- preserve `ADR-PLAT-AUTH-001` human Person/Role Assignment semantics;
- preserve A04 Tenant/Institution isolation;
- preserve D03 trust/credential ownership;
- preserve D02 Permission/scope/policy/privilege boundaries;
- introduce no vendor/product or technology lock-in;
- close the material HF-11 machine-principal authorization gap identified in Stage-2 validation.

**Decision:** Architecture Approved — 2026-09-12.  
**Lock:** Not authorized by this ADR approval.
