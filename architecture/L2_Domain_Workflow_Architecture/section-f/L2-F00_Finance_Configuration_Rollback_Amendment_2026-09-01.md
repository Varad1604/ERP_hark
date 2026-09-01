# L2-F00 — Finance Configuration Rollback — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-F00 — Finance Domain Model & Foundation; L2-A01 configuration-framework participation |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Rollback model

Finance configuration rollback is not a reverse lifecycle transition and does not reactivate a historical `Superseded` version.

When an active Finance configuration must be rolled back:

1. an eligible prior approved configuration is selected as the rollback basis;
2. a **new Finance configuration version** is created from that approved business meaning;
3. the new version follows the canonical `Draft → Validation → Approval → Published → Active` path;
4. when the rollback version becomes Active, the previously Active version becomes `Superseded`;
5. all historical transactions retain the configuration version that governed their execution.

## 2. Authority

**Finance Configuration Approval Authority** is the canonical business authority for normal configuration approval and rollback authorization. Institutional policy maps this authority to the applicable level of the existing F00 Finance approval hierarchy.

## 3. Events

Finance Foundation publishes:

- `FinanceConfigurationVersionActivated` — a governed Finance configuration version becomes Active.
- `FinanceConfigurationVersionSuperseded` — an Active Finance configuration version is replaced by another Active version.
- `FinanceConfigurationRollbackAuthorized` — a rollback decision has authorized creation/progression of a new version based on an eligible prior approved configuration.

The Platform Configuration Framework may transport or resolve configuration but does not publish these Finance semantic events.

## 4. Audit obligations

Rollback evidence shall preserve rollback reason, selected prior approved basis, new version identity, decision authority, predecessor version, validation/approval evidence and activation outcome.

## 5. Downstream effect

This amendment resolves the upstream blocker recorded by L3-F-084.
