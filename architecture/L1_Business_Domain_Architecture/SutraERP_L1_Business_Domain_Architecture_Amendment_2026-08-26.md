# SutraERP L1 Business Domain Architecture — Locked Amendment

| Attribute | Value |
|---|---|
| Target | SutraERP L1 Business Domain Architecture — Locked |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | ADR-PLAT-AUTH-001 |
| Authority | Lead Architect approval recorded 2026-08-26 |

This amendment is part of the canonical L1 Business Domain Architecture. Where it conflicts with the earlier locked L1 wording for canonical Person scope, this amendment is authoritative. All unaffected L1 content remains unchanged.

## 1. Canonical Person Scope Clarification

The canonical **Person** entity is **Tenant-scoped**, not Institute-scoped.

Within one Tenant:

- every real human has exactly one canonical Person record;
- Institute, campus, student, staff, guardian and other institutional contexts reference that Person;
- participation in multiple Institutes within the same Tenant shall not create duplicate canonical Person records;
- Institute-scoped business entities and memberships remain Institute-scoped where their owning domains define them as such.

Cross-Tenant Person sharing is not established by this amendment. Tenant isolation remains mandatory.

## 2. Superseded L1 Wording

Any earlier L1 wording stating or implying that one canonical Person record exists **inside an Institute** is superseded only for canonical identity scope.

The intended meaning is now:

> A Tenant owns one canonical Person identity per real human participating in that Tenant. Institutes and other institutional contexts reference the canonical Person through their own domain-owned relationships and lifecycles.

## 3. Ownership Preservation

This amendment does not transfer ownership between domains.

- Platform Foundation / Person & Identity retains canonical Person and Identity ownership.
- Institute and other business domains retain ownership of their own Institute-scoped relationships, memberships, roles, records and business facts.
- No domain may duplicate Person identity merely to preserve an Institute boundary.

## 4. Locked Decision

The Tenant-scoped canonical Person rule is Locked under ADR-PLAT-AUTH-001 and shall be used by downstream L2/L3 architecture wherever Person scope is relevant.