# L2-A01 — Controlled Tenant Change Authority — Locked Amendment

| Attribute | Value |
|---|---|
| Amendment Version | 1.2.0 |
| Status | Locked |
| Architecture Level | L2 — Business Domain Architecture |
| Canonical Owner | Platform & Tenant |
| Approved Through | Explicit Lead Architect decision, 2026-09-23 |
| Authority | Post-R8 Option A upstream business-architecture remediation |

---

## 1. Purpose

This amendment closes the authority gap recorded by L3-A-057 for controlled Tenant maintenance and upgrade while preserving existing L2-A01 Tenant lifecycle ownership.

It introduces no new Tenant state and no new downstream-domain authority.

---

## 2. Canonical Authority

### 2.1 Tenant Change Authority

`Tenant Change Authority` is the canonical abstract business authority for governed entry into and completion of controlled Tenant Maintenance or Upgrade.

Institutional policy may map this authority to an eligible platform administration/governance office or actor.

It authorizes:

- controlled `Active → Maintenance`;
- controlled `Active → Upgrade`;
- governed completion and return `Maintenance → Active`;
- governed completion and return `Upgrade → Active`.

### 2.2 Authorized Platform Actor

An `Authorized Platform Actor` may request and execute the approved controlled change within the scope granted by institutional policy.

Request/execution participation does not transfer Tenant lifecycle ownership.

### 2.3 Platform & Tenant

Platform & Tenant / L2-A01 remains sole owner of:

- Tenant identity;
- Tenant lifecycle;
- lifecycle eligibility validation;
- Maintenance/Upgrade restrictions;
- Tenant context;
- transition truth.

No downstream business domain may initiate or complete a Tenant lifecycle transition as owner.

---

## 3. Controlled Change Operations

| Operation | Preconditions | Outcome |
|---|---|---|
| Request Controlled Tenant Change | Tenant is `Active`; change need and scope are identified | Change is presented to Tenant Change Authority |
| Authorize Maintenance | Tenant is eligible; authority approves | `Active → Maintenance` may occur |
| Authorize Upgrade | Tenant is eligible; authority approves | `Active → Upgrade` may occur |
| Validate Controlled Change Completion | Intended change has completed and Tenant integrity/availability checks pass | Return-to-Active may be considered |
| Restore Tenant Availability | Tenant Change Authority accepts completion/validation outcome | `Maintenance/Upgrade → Active` |

These are business operations, not APIs or deployment procedures.

---

## 4. Validation Boundary

Return to `Active` requires a governed validation outcome showing that:

- Tenant identity remains intact;
- Tenant configuration remains valid;
- required Tenant data/history remain preserved;
- applicable Platform integrity checks pass;
- no known unresolved condition requires the Tenant to remain restricted.

Exact technical checks, deployment mechanics, rollback mechanisms and infrastructure tooling remain L4/L5 concerns.

---

## 5. Event Decision

No dedicated Maintenance/Upgrade lifecycle event names are made canonical by this amendment.

The authoritative business truth for this path is the Platform-owned Tenant lifecycle state and governed Tenant context.

If a later architecture introduces dedicated lifecycle events, it must do so through an explicit approved amendment and preserve L2-A01 as semantic publisher.

Existing canonical Tenant events defined by earlier amendments remain unchanged.

---

## 6. Locked Decisions

1. `Tenant Change Authority` is the canonical abstract authority for controlled Maintenance/Upgrade entry and governed return to Active.
2. Platform & Tenant / L2-A01 remains sole owner of Tenant lifecycle and transition truth.
3. An Authorized Platform Actor may request/execute an approved controlled change but does not own lifecycle authority.
4. Existing lifecycle transitions `Active ↔ Maintenance` and `Active ↔ Upgrade` are preserved exactly.
5. No new Tenant lifecycle state is introduced.
6. Return to Active requires successful governed validation and Tenant Change Authority acceptance.
7. No dedicated Maintenance/Upgrade event names are made canonical by this amendment.
8. Downstream domains consume Tenant availability state but cannot modify it.
9. No universal institutional job title or universal separation-of-duties rule is introduced.
