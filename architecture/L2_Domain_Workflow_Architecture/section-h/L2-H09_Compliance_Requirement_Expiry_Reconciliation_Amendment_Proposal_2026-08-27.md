# L2-H09 — Accreditation & Compliance Workflows — Compliance Requirement Expiry Reconciliation Amendment Proposal

| Attribute | Value |
|---|---|
| Target | L2-H09 — Accreditation & Compliance Workflows v1.0 plus locked 2026-08-26 H09 lifecycle amendment |
| Proposed Amendment Version | 1.2.0 |
| Status | Review |
| Date | 2026-08-27 |
| Owner | Lead Architect |
| Architectural Level | L2 — Domain Workflow Architecture |
| Domain | H09 — Accreditation & Compliance |
| Triggering Gap | `L3-H-063` frozen `expired` / `retired` outcome conflict |
| Canonical Effect | **None until independently reviewed and approved/locked** |

---

## 1. Purpose

This proposal reconciles two currently conflicting locked H09 semantics:

- the base H09 architecture contains an `Expired` compliance-obligation concept and requires expired obligations to enter review; and
- the 2026-08-26 amendment defines canonical Compliance Requirement lifecycle states including `Retired` for a requirement version that is withdrawn or superseded, but does not explicitly reconcile the base `Expired` meaning.

`L3-H-063` cannot locally equate `Expired` with `Retired` without an upstream decision.

This proposal is **Review-status architecture only** and has no canonical effect until formally approved.

---

## 2. Ownership Preservation

- H09 continues to own the Compliance Requirement governance record and lifecycle.
- `VAR:Obligation-Owning Domain` retains ownership of the underlying operational/source obligation facts where applicable.
- H09 expiry review does not modify the external legal/regulatory obligation itself.

---

## 3. Proposed Reconciled Lifecycle Semantics

The existing amendment lifecycle remains authoritative for normal progression:

`Identified → Applicable → Active → Satisfied → Closed`

Existing branches remain:

- `Applicable / Active → Waiver Requested → Waived`
- `Active → Noncompliant → Active`
- `Applicable / Active / Satisfied → Retired` when formally withdrawn or superseded

### Proposed explicit expiry branch

`Applicable / Active / Satisfied → Expired → Under Review`

After review, one of the following governed outcomes applies:

- `Under Review → Retired` when the requirement version is no longer current because its validity period ended and no successor/current obligation applies;
- `Under Review → Applicable` when the obligation remains applicable under a renewed/extended governing basis;
- `Under Review → Active` when the renewed/current obligation is immediately active and monitoring resumes;
- a successor Compliance Requirement version may be created when the governing obligation is materially revised.

`Expired` and `Retired` are therefore distinct:

- `Expired` = the governed validity period ended and review is mandatory;
- `Retired` = the requirement version is no longer current after formal withdrawal, supersession, or post-expiry review.

---

## 4. Proposed Business Authority

The existing H09 governance chain remains authoritative.

- The accountable Business Owner remains responsible for the requirement context/evidence.
- The **Accreditation and Compliance Governance Authority** owns governed retirement decisions and any institutional exception decision after expiry review.
- Routine expiry detection does not itself constitute retirement.

---

## 5. Proposed Business Rules

1. Expiry shall not be silently mapped to retirement.
2. Every Expired Compliance Requirement shall enter governed review.
3. Expiry does not imply satisfaction or waiver.
4. Retirement following expiry requires an explicit governed conclusion that the requirement version is no longer current.
5. A renewed/extended obligation may return the governance record to Applicable/Active or may require a successor version when materially changed.
6. Historical evidence and prior review decisions remain immutable.

---

## 6. Proposed Canonical H09 Events

If approved, H09 would be the sole semantic publisher of:

- `ComplianceRequirementExpired`
- `ComplianceRequirementExpiryReviewStarted`

Existing events remain unchanged:

- `ComplianceWaiverRequested`
- `ComplianceRequirementWaived`
- `ComplianceRequirementRetired`
- `ComplianceRequirementSatisfied`

No duplicate retirement event is introduced.

---

## 7. Downstream Effect

If formally approved and locked:

- `L3-H-063` may be synchronized to treat `Expired` and `Retired` as distinct source-supported outcomes;
- the frozen `revised`, `fulfilled`, `waived`, `expired`, and `retired` wording can be mapped without L3 invention;
- H063 may return to independent Architecture Approval review.

---

## 8. Review Questions

Independent architecture review shall confirm:

1. whether `Expired → Under Review` should be the only allowed expiry transition;
2. whether `Under Review → Applicable/Active/Retired` is sufficient for renewal/continuation cases;
3. whether material renewal must always create a successor requirement version; and
4. whether event vocabulary conflicts with any existing base-H09 event terminology.

---

## 9. Decision Status

**PROPOSAL STATUS:** REVIEW  
**LOCKED/CANONICAL EFFECT:** NONE  
**L3-H-063:** REMAINS UPSTREAM-BLOCKED UNTIL THIS OR AN ALTERNATIVE RESOLUTION IS APPROVED
