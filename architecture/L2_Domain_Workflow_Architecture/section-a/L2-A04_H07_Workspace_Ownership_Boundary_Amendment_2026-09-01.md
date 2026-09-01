# L2-A04 / L2-H07 — Workspace Ownership Boundary — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-A04 — Authorization & Policy Workflows; L2-H07 — Enterprise Dashboard Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-01 |
| Approved Through | ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 |

## 1. Canonical ownership

A04 is the sole canonical owner of **Workspace** identity, Workspace availability/resolution and the authorization decision determining which Workspaces are available to an authenticated institutional user.

H07 owns **Dashboard Workspace Composition**: the governed association of dashboard layouts, dashboard sets and widgets to an A04 Workspace context. It does not own or redefine the A04 Workspace aggregate or authorization decision.

Where older H07 text states that H07 owns generic `Workspace` or a generic Workspace lifecycle, that wording is superseded by this boundary. The H07 lifecycle applies only to Dashboard Workspace Composition/configuration.

## 2. Cross-domain contract

A04 exposes **Authorized Workspace Context** to H07 after evaluating authoritative Role/Role Assignment and applicable organizational/campus context.

B02/B03 retain ownership of Organization and InstitutionalCampus facts. H07 does not require synthetic `UserGroupChanged` or `OrganisationRestructured` events to own those facts. When authoritative context changes, A04 re-evaluates Workspace availability and H07 consumes the resulting Authorized Workspace Context.

If A04 withdraws a Workspace from the authorized context, H07 shall make its associated dashboard composition unavailable to that user/context without changing the underlying A04 Workspace.

## 3. H07 event vocabulary

H07 may publish `DashboardWorkspaceCompositionAssigned` when governed dashboard composition is associated with an A04 Workspace context. This event does not mean that H07 created or authorized the Workspace.

## 4. Downstream effect

This amendment resolves the ownership/lifecycle/contract blocker recorded by L3-H-050.
