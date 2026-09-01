# L2-H02 — Notification Template Approval Authority — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-H02 — Notification Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Template authority

**Notification Template Approval Authority** is the canonical business authority for the H02 Template lifecycle review decision.

The authority:

- approves `Under Review → Approved`;
- may return `Under Review → Draft` for revision;
- authorizes publication of an Approved Template version under the existing `Approved → Published` transition;
- acts within institutional communication policy and authorized scope.

Institutional policy may map this authority to an appropriate communication-governance role without changing H02 ownership.

## 2. Lifecycle and events

The existing lifecycle remains canonical:

`Draft → Under Review → Approved → Published → Deprecated → Retired`

with existing return/reconsideration transitions.

No new Template lifecycle event is introduced by this amendment because the current cross-domain architecture does not require one for completion. Template lifecycle and authority decisions remain auditable.

## 3. Downstream effect

This amendment resolves the approval-authority blocker recorded by L3-H-012.
