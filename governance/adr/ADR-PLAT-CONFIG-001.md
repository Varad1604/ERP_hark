# ADR-PLAT-CONFIG-001 — Canonical Configuration Governance Completion

| Field | Value |
|---|---|
| ADR ID | ADR-PLAT-CONFIG-001 |
| Change Type | L2 platform configuration governance completion |
| Target | L2-A01 — Platform & Tenant Foundation, §6 Platform Configuration Framework |
| Directly Resolves | L3/L4-A-056; L3/L4-G-014 |
| Status | Approved — Incorporated |
| Date | 2026-09-20 |
| Authority | User-authorized controlled upstream remediation in SutraERP_Level_3 project |

## 1. Problem

L2-A01 already owns the common Platform Configuration Framework and defines configuration ownership, hierarchy, validation, lifecycle and resolution. It does not define the business authority and lifecycle operations needed to progress a configuration version through `Draft → Validation → Approved → Active → Superseded → Archived`, nor exact framework lifecycle events and publishers.

This leaves L3/L4-A-056 blocked and prevents G00 configuration scenario L3/L4-G-014 from reaching its frozen effective outcome without inventing an approver or event.

## 2. Decision

### 2.1 Configuration Approval Authority

**Configuration Approval Authority** is the canonical business authority for deciding whether a validated configuration version may enter `Approved`.

The authority is scoped to the canonical owner of the configuration layer:

- Platform Defaults and Tenant Configuration — operates on behalf of Platform Foundation / A01.
- Institution Configuration — operates on behalf of Institution Foundation / B01.
- Organisation Configuration — operates on behalf of Organisation Foundation / B02.
- Business Domain Configuration — operates on behalf of the respective canonical L2 business-domain owner.
- Operational Runtime Configuration — operates only within Platform Services-owned runtime configuration.

This authority is institution-independent and is not a universal job title or technical role. Institutional policy maps it to an authorized business actor for the owning configuration layer.

Approval does not transfer configuration semantic ownership to A01.

### 2.2 Authoring and validation

An authorized configuration administrator or owner-domain actor may propose a distinct successor configuration version within its canonical ownership scope.

Validation is performed under the A01 framework for:
- ownership and scope;
- hierarchy/inheritance integrity;
- allowed override boundaries;
- dependency consistency;
- version compatibility;
- security and institution/Tenant isolation;
- completeness; and
- owner-domain semantic validation where the configuration is not A01-owned.

Authoring or validation does not imply approval authority.

### 2.3 Lifecycle progression

The existing A01 lifecycle remains unchanged:

`Draft → Validation → Approved → Active → Superseded → Archived`

Canonical transition rules:

1. A proposal enters `Draft` as a distinct successor version; prior versions are never overwritten in place.
2. `Draft → Validation` occurs when the candidate is submitted for governed validation.
3. `Validation → Approved` requires successful validation and an affirmative Configuration Approval Authority decision.
4. A rejected or incomplete candidate remains outside `Approved`; no new Rejected state is introduced.
5. `Approved → Active` occurs only when the approved effective boundary is reached and required parent/dependency/validation conditions remain valid.
6. Activation does not require a second human approval unless the owning domain explicitly defines one.
7. When a successor becomes `Active` for the same owner/scope/key, the prior `Active` version becomes `Superseded`.
8. `Superseded → Archived` follows governed retention/disposition rules.
9. Only `Active` configuration participates in business execution.

### 2.4 Canonical business operations

A01 defines the following implementation-independent configuration governance operations:

- Propose Configuration Version
- Submit Configuration for Validation
- Validate Configuration Version
- Record Configuration Approval Decision
- Activate Approved Configuration Version
- Supersede Prior Configuration Version
- Archive Superseded Configuration Version
- Resolve Configuration Context

These are business/service contracts, not APIs or technical commands.

### 2.5 Canonical events and publishers

The **Platform Configuration Framework / A01** is the sole canonical publisher of framework lifecycle facts:

- `ConfigurationVersionProposed`
- `ConfigurationVersionValidated`
- `ConfigurationVersionApproved`
- `ConfigurationVersionActivated`
- `ConfigurationVersionSuperseded`
- `ConfigurationVersionArchived`
- `ResolvedConfigurationPublished`

These events report the framework lifecycle of a configuration version. They do not transfer ownership of domain configuration values or policies to A01.

An owning business domain may additionally publish its own domain-semantic event only where that event is separately defined by authoritative domain architecture.

### 2.6 Activation and effective dating

An approved future-effective version remains `Approved` until its effective boundary. At that boundary, it may become `Active` only if validation, hierarchy, dependency and scope conditions remain satisfied.

If activation conditions fail, the current `Active` version remains authoritative and no activation event is published.

### 2.7 Audit

Configuration governance evidence shall preserve:
- canonical owner/layer;
- scope;
- predecessor/successor version;
- proposal/validation evidence;
- approving authority and decision;
- intended/effective boundary;
- activation/supersession/archive history; and
- resolved-context version lineage.

## 3. Ownership Preservation

- A01 owns the common configuration framework, lifecycle mechanics, hierarchical resolution and generic lifecycle-event publication.
- Each configuration layer retains ownership of its values and business semantics.
- Business-domain configuration approval is a decision made on behalf of the owning business domain, not by Platform Foundation interpreting that domain's business meaning.
- A01 does not become owner of Institution, Organisation or business-domain configuration semantics.

## 4. Downstream Effect

- L3/L4-A-056 can now complete its Platform/Tenant configuration outcome.
- L3/L4-G-014 can use the owner-scoped Configuration Approval Authority and A01 lifecycle events without inventing a G00-specific lifecycle.
- Existing domain-specific configuration authorities/events remain authoritative where explicitly defined; this ADR does not rename or override them.
- Other scenarios with unresolved references to the generic A01 lifecycle may consume this decision in later controlled revalidation.

## 5. Rejected Alternatives

1. Give Platform Foundation approval authority over all domain configuration semantics — rejected because it violates canonical business ownership.
2. Require every domain to invent its own copy of the configuration lifecycle — rejected because A01 already owns the common lifecycle.
3. Make activation a second mandatory human decision — rejected because A01 does not require a separate business decision after approval and effective-boundary validation.
4. Use one domain-specific event such as a G00 configuration event as the generic lifecycle fact — rejected because A01 must own common lifecycle-event publication.
5. Introduce APIs, storage or technical workflow implementation — rejected as out of scope for L2.

## 6. Decision Status

**Approved and incorporated on 2026-09-20.**
