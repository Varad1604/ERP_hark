# L2-H09 — Accreditation & Compliance Workflows — Compliance Requirement Expiry Reconciliation Amendment Proposal

| Attribute | Value |
|---|---|
| Target | L2-H09 — Accreditation & Compliance Workflows v1.0 plus locked 2026-08-26 H09 lifecycle amendment |
| Proposed Amendment Version | 1.2.0 |
| Status | Archived |
| Date | 2026-08-27 |
| Owner | Lead Architect |
| Architectural Level | L2 — Domain Workflow Architecture |
| Domain | H09 — Accreditation & Compliance |
| Triggering Gap | `L3-H-063` frozen `expired` / `retired` outcome conflict |
| Superseded By | `L2-H09_Compliance_Requirement_Expiry_Reconciliation_Amendment_2026-08-27.md` |

---

## 1. Historical Purpose

This proposal reconciled conflicting H09 semantics for `Expired` and `Retired` Compliance Requirement outcomes. It was approved by repository authority on 2026-08-27 and incorporated into the Locked supplemental amendment referenced above.

This file is retained only as historical proposal evidence.

---

## 2. Approved Proposal Content

Normal lifecycle remains:

`Identified → Applicable → Active → Satisfied → Closed`

Approved expiry branch:

`Applicable / Active / Satisfied → Expired → Under Review`

Post-expiry review may result in `Retired`, `Applicable`, `Active`, or a successor requirement version.

Canonical meanings:

- `Expired` = governed validity period ended and review is mandatory;
- `Retired` = requirement version is formally no longer current after withdrawal, supersession, or post-expiry review.

H09 publishes `ComplianceRequirementExpired` and `ComplianceRequirementExpiryReviewStarted`.

---

## 3. Current Decision Status

**PROPOSAL STATUS:** ARCHIVED  
**CANONICAL SOURCE:** `L2-H09_Compliance_Requirement_Expiry_Reconciliation_Amendment_2026-08-27.md`  
**APPROVAL RECORD:** `governance/adr/H05_H09_H10_UPSTREAM_REMEDIATION_APPROVAL_RECORD_2026-08-27.md`
