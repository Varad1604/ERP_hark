# L2-H09 — Accreditation & Compliance Workflows — Compliance Requirement Expiry Reconciliation Amendment

| Attribute | Value |
|---|---|
| Target | L2-H09 — Accreditation & Compliance Workflows v1.0 plus locked 2026-08-26 H09 lifecycle amendment |
| Amendment Version | 1.2.0 |
| Status | Locked |
| Effective Date | 2026-08-27 |
| Approved Through | ADR-H-COMPLIANCE-EXPIRY-001 |
| Authority | Explicit Lead Architect / repository-authority approval recorded 2026-08-27 in `H05_H09_H10_UPSTREAM_REMEDIATION_APPROVAL_RECORD_2026-08-27.md` |

This amendment is canonical L2-H09 architecture for reconciliation of Compliance Requirement `Expired` and `Retired` semantics. It supplements the 2026-08-26 H09 amendment. Unaffected H09 content remains unchanged.

## 1. Ownership Preservation

- H09 owns the Compliance Requirement governance record and lifecycle.
- `VAR:Obligation-Owning Domain` retains ownership of underlying operational/source obligation facts where applicable.
- H09 expiry review does not modify the external legal/regulatory obligation itself.

## 2. Reconciled Compliance Requirement Lifecycle

Normal progression remains:

`Identified → Applicable → Active → Satisfied → Closed`

Existing branches remain:

- `Applicable / Active → Waiver Requested → Waived`
- `Active → Noncompliant → Active`
- `Applicable / Active / Satisfied → Retired` when formally withdrawn or superseded

Canonical expiry branch:

`Applicable / Active / Satisfied → Expired → Under Review`

After expiry review:

- `Under Review → Retired` when the requirement version is no longer current because its validity period ended and no successor/current obligation applies;
- `Under Review → Applicable` when the obligation remains applicable under a renewed/extended governing basis;
- `Under Review → Active` when the renewed/current obligation is immediately active and monitoring resumes;
- a successor Compliance Requirement version may be created when the governing obligation is materially revised.

## 3. Canonical Meanings

- `Expired` = the governed validity period ended and review is mandatory.
- `Retired` = the requirement version is no longer current after formal withdrawal, supersession, or post-expiry review.
- Expiry does not imply satisfaction, waiver, or retirement.

## 4. Business Authority

- The accountable Business Owner remains responsible for requirement context/evidence.
- The **Accreditation and Compliance Governance Authority** owns governed retirement decisions and institutional exception decisions after expiry review where H09 authority is required.
- Routine expiry detection does not itself constitute retirement.

## 5. Business Rules

1. Expiry shall not be silently mapped to retirement.
2. Every Expired Compliance Requirement shall enter governed review.
3. Expiry does not imply satisfaction or waiver.
4. Retirement following expiry requires an explicit governed conclusion that the requirement version is no longer current.
5. A renewed/extended obligation may return the governance record to Applicable/Active or may require a successor version when materially changed.
6. Historical evidence and prior review decisions remain immutable.

## 6. Canonical H09 Events

H09 is the sole semantic publisher of:

- `ComplianceRequirementExpired`
- `ComplianceRequirementExpiryReviewStarted`

Existing event vocabulary remains unchanged, including:

- `ComplianceWaiverRequested`
- `ComplianceRequirementWaived`
- `ComplianceRequirementRetired`
- `ComplianceRequirementSatisfied`

## 7. Downstream Effect

This amendment resolves the upstream conflict identified by `L3-H-063` and authorizes scenario-level synchronization preserving `Expired` and `Retired` as distinct business/lifecycle outcomes.

## 8. Locked Decisions

The ownership, lifecycle, meanings, authority, rules and events in §§1–6 are Locked L2-H09 decisions under ADR-H-COMPLIANCE-EXPIRY-001.
