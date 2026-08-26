# L2-H03 — Enterprise Search — Governance Amendment

| Attribute | Value |
|---|---|
| Target | L2-H03 — Enterprise Search Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-H-SEARCH-GOV-001 |
| Authority | Explicit Lead Architect remediation authorization recorded 2026-08-26 |

This amendment is canonical L2-H03 architecture. Unaffected H03 content remains unchanged.

## 1. Ownership

H03 retains exclusive ownership of Search Projection Definitions, Search Configuration and their H03 lifecycles. Source domains retain ownership of source business facts. Section A provides authorization and audit capability but does not own search configuration decisions.

## 2. Projection Approval Authority

The **Search Governance Authority** is the canonical institutional business authority for approving a new or materially changed Search Projection Definition before it becomes available for governed enterprise search.

The authority evaluates source-domain ownership, permitted projection scope, discoverability restrictions and institutional search policy. Technical administration does not substitute for this business approval.

## 3. Search Projection Lifecycle

Canonical Search Projection Definition lifecycle:

`Draft → Under Review → Approved → Active → Retired`

Alternative governed branches:

- `Under Review → Rejected`
- `Active → Suspended → Active`
- `Suspended → Retired`

A Rejected definition may be revised only by creating a successor Draft version; the rejected record remains historical.

## 4. Search Configuration Version Lifecycle

Canonical Search Configuration Version lifecycle:

`Draft → Under Review → Approved → Effective → Superseded`

A previously effective configuration remains immutable after supersession.

The Search Governance Authority may authorize rollback when a current effective version creates unacceptable business behavior. Rollback does not rewrite history; it makes an eligible prior approved version effective again as a new governed effective decision while retaining the superseded lineage.

## 5. Canonical H03 Events

H03 is sole semantic publisher of:

- `SearchProjectionApproved`
- `SearchProjectionActivated`
- `SearchProjectionRejected`
- `SearchProjectionRetired`
- `SearchConfigurationApproved`
- `SearchConfigurationEffective`
- `SearchConfigurationSuperseded`
- `SearchConfigurationRollbackAuthorized`
- `SearchConfigurationRolledBack`

These events do not publish or redefine source-domain business facts.

## 6. Downstream Effect

This amendment closes the upstream authority, lifecycle, versioning and rollback gaps identified by L3-H-020 and L3-H-021.

## 7. Locked Decisions

Ownership, authority, lifecycles, immutable versioning and events in §§1–5 are Locked L2-H03 decisions under ADR-H-SEARCH-GOV-001.
