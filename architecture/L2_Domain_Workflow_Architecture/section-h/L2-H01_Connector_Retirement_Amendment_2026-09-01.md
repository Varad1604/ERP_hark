# L2-H01 — Government Connector Retirement — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H01 — APAAR & DigiLocker Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Connector retirement

`Retired` is a canonical terminal H01 Connector state and is distinct from temporary or governed `Disabled` status.

A Connector shall be unavailable for normal use before retirement. Canonical retirement progression is:

`Activated/Operational or Maintenance → Disabled → Retired`

A Retired Connector cannot be reactivated. Reintroduction requires a new governed Connector identity/version as applicable under H01 governance.

## 2. Authority and event

**Platform Administrator** is the canonical retirement authority for the H01 Connector lifecycle.

H01 publishes `ConnectorRetired` when a Connector enters the terminal Retired state.

Existing activation, configuration, health-validation and disablement events remain unchanged.

## 3. Downstream effect

This amendment resolves the blocker recorded by L3-H-009.
