# L2-A01 — Platform Configuration Governance Completion — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-A01 — Platform & Tenant Foundation, §6 Platform Configuration Framework |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-20 |
| Approved Through | ADR-PLAT-CONFIG-001 |

This amendment is part of canonical L2-A01. Where it conflicts with earlier A01 text concerning configuration governance workflow, approval authority, lifecycle operations or lifecycle events, this amendment is authoritative. Existing ownership, hierarchy, validation, lifecycle states, resolution rules and unaffected A01 content remain unchanged.

## 1. Configuration Approval Authority

**Configuration Approval Authority** is the canonical business authority that decides whether a validated configuration version may enter `Approved`.

The authority acts on behalf of the canonical owner of the applicable configuration layer:

| Configuration layer | Authority acts on behalf of |
|---|---|
| Platform Defaults | Platform Foundation / A01 |
| Tenant Configuration | Platform Foundation / A01 |
| Institution Configuration | Institution Foundation / B01 |
| Organisation Configuration | Organisation Foundation / B02 |
| Business Domain Configuration | Respective canonical L2 business-domain owner |
| Operational Runtime Configuration | Platform Services |

The authority is a business authority, not a universal job title or technical role. Institutional policy maps it to an authorized business actor within the owning layer.

Configuration authoring/validation does not imply configuration approval.

## 2. Canonical configuration-governance flow

```text
Requirement
   ↓
Propose distinct Draft version
   ↓
Submit for Validation
   ↓
Validate ownership / scope / hierarchy / dependencies / compatibility / completeness
   ↓
Configuration Approval Authority decision
   ↓ approved
Approved
   ↓ effective boundary + continuing validation
Active
   ↓ successor activation
Superseded
   ↓ retention disposition
Archived
```

Rules:

1. The existing lifecycle remains `Draft → Validation → Approved → Active → Superseded → Archived`.
2. Prior versions shall never be destructively rewritten.
3. Validation failure keeps the candidate outside `Approved`.
4. Approval rejection keeps the candidate outside `Approved`; no new lifecycle state is introduced.
5. `Approved → Active` occurs only when the approved effective boundary is reached and required validation/dependency conditions remain true.
6. No separate human activation approval is required unless the canonical owner defines one in its own business architecture.
7. Activating a successor for the same owner/scope/key supersedes the prior Active version.
8. Only Active configuration participates in business execution.
9. Historical and resolved configuration contexts retain exact version lineage.

## 3. Canonical business operations

The common framework provides the following implementation-independent business/service contracts:

- Propose Configuration Version
- Submit Configuration for Validation
- Validate Configuration Version
- Record Configuration Approval Decision
- Activate Approved Configuration Version
- Supersede Prior Configuration Version
- Archive Superseded Configuration Version
- Resolve Configuration Context

These names describe business operations only. They do not prescribe API endpoints, software classes, queues or persistence.

## 4. Canonical framework lifecycle events

Platform Configuration Framework / A01 is the sole canonical publisher of:

| Event | Meaning |
|---|---|
| `ConfigurationVersionProposed` | A distinct candidate version has been established under a canonical owner/scope. |
| `ConfigurationVersionValidated` | Framework and owner-domain validation has completed successfully. |
| `ConfigurationVersionApproved` | The applicable Configuration Approval Authority has approved the validated version. |
| `ConfigurationVersionActivated` | The approved version has become Active for its effective scope. |
| `ConfigurationVersionSuperseded` | A prior Active version has been replaced by the successor for the same owner/scope/key. |
| `ConfigurationVersionArchived` | A Superseded version has entered governed archival. |
| `ResolvedConfigurationPublished` | A deterministic resolved configuration context has been produced for governed consumption. |

These are framework lifecycle facts. They do not make A01 owner of Institution, Organisation or business-domain configuration semantics.

Domain-specific events remain separate and may be published only when explicitly defined by the canonical domain owner.

## 5. Validation and activation guards

A configuration version shall not become Active when:
- the canonical owner/layer is ambiguous;
- Tenant/Institution/Organisation scope is invalid;
- hierarchy/inheritance is cyclic or references an ineligible parent;
- required configuration is incomplete;
- required dependencies are unavailable;
- validation has failed;
- approval evidence is absent or rejected; or
- the effective boundary has not yet been reached.

The current Active version remains authoritative when successor activation cannot lawfully occur.

## 6. Audit obligations

Configuration governance history shall preserve:
- owner/layer;
- scope;
- predecessor/successor version;
- validation evidence;
- approving authority and decision;
- intended and actual effective boundary;
- activation, supersession and archival history; and
- resolved-context version lineage.

## 7. Ownership boundary

A01 owns:
- the common configuration framework;
- the generic lifecycle;
- hierarchical resolution;
- lifecycle transition validation; and
- generic framework lifecycle-event publication.

The applicable configuration layer owns:
- configuration values;
- business meaning;
- permitted business choices; and
- owner-domain semantic review.

## 8. Downstream resolution

This amendment resolves the generic A01 configuration-governance gaps recorded by `L3-A-056` / `L4-A-056` and supplies the common approval/event contract required by `L3-G-014` / `L4-G-014`.

## 9. Locked Decisions

Sections 1–8 are **Locked L2 decisions** under ADR-PLAT-CONFIG-001.
