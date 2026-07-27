# Security Review Change Log

Date: 2026-07-25

This file records the architectural changes made to Section A after the security, compliance, and legal architecture review.

## Section A Documents

### L2-A01 - Platform & Tenant Foundation

- Added an explicit shared security governance section.
- Defined platform ownership for machine identities and service accounts.
- Defined platform ownership for shared cryptographic material, including keys and certificates.
- Added explicit rules for service-to-service trust and revocation/rotation authority.

### L2-A02 - Person & Identity Workflows

- Added a privacy boundary section for identity propagation.
- Restricted downstream sharing to minimum, purpose-bound identity attributes.
- Clarified that government identity data, contact data, and privacy preferences are not globally visible by default.
- Added cryptographic governance clarification that shared keys and certificates are platform-owned, not identity-domain-owned.

### L2-A03 - Authentication Workflows

- Added explicit non-human identity governance.
- Clarified that service accounts and machine identities are tenant-aware.
- Restricted API credentials to system integrations.
- Required service identities to follow the same audit and revocation boundary as human identities.

### L2-A04 - Authorization and Policy Workflows

- Reviewed for delegate, temporary access, SoD, and break-glass controls.
- No architectural edits were required for the reported issues because the document already defined those boundaries explicitly.

### L2-A05 - Workflow Engine Workflows

- Reviewed for operational authority, retention, and legal-hold related architecture.
- No changes were required for the reported issues.

### L2-A06 - Event Infrastructure Workflows

- Reviewed for cross-domain trust, tenant isolation, and event governance.
- No changes were required for the reported issues.

### L2-A07 - Audit Infrastructure Workflows

- Added a narrower cross-tenant audit access governance rule.
- Required cross-tenant evidence access to use an explicit audit/governance approval path.
- Added legal preservation governance so active preservation obligations override ordinary retention completion.

### L2-A08 - Module Registry & Configuration Workflows

- Reviewed for administrative governance and approval control.
- No changes were required for the reported issues.

### L2-A09 - Master Data Workflows

- Reviewed for ownership, retention, auditability, and data governance.
- No changes were required for the reported issues.

## Reference Documents Reviewed

- L0 Business Vision
- L1 Business Domain Architecture
- Architecture Index
- Dependency Map
- L2 Domain Workflow Matrix
- Team Assignments

## Summary of Architectural Fixes

- Explicit machine identity and service-account governance was added.
- Shared cryptographic ownership was made explicit.
- Identity privacy propagation was narrowed to minimum necessary attributes.
- Cross-tenant audit access was tightened.
- Legal preservation governance was added for audit evidence.

