# L2-F00 — Financial Rule Governance — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F00 — Finance Domain Model & Foundation |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-002 |

## 1. Canonical authority

The **Finance Financial Rule Governance Authority** is the canonical Finance business authority for approval of new or revised Financial Rule versions. Institutional policy maps this authority to the applicable level of the existing Finance approval hierarchy without transferring rule ownership outside Finance Foundation.

## 2. Canonical lifecycle

The existing Financial Rule lifecycle remains:

`Draft → Validation → Testing → Approval → Published → Active → Deprecated → Archived`

Only `Active` Financial Rule versions govern new eligible Finance decisions. Historical decisions remain bound to their original governing version.

## 3. Canonical events and publisher

Finance Foundation is the canonical publisher of Financial Rule lifecycle facts. The minimum canonical event set required by the governed rule-change path is:

- `FinancialRuleApproved`
- `FinancialRuleActivated`
- `FinancialRuleDeprecated`
- `FinancialRuleArchived`

Validation/testing evidence remains part of rule-governance history; no additional event is required merely to satisfy an implementation transport concern.

## 4. Audit obligations

Approval evidence shall preserve rule identity/version, owner/category/scope, validation/testing evidence, approving authority, approval reference, effective point, predecessor/successor lineage and historical evaluation references.

## 5. Downstream effect

This amendment resolves the authority and event-publisher gaps recorded by L3-F-085. It does not change the Finance rule language, evaluation engine, persistence or implementation architecture.
