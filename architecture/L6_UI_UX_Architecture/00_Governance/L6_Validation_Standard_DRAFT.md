# SutraERP L6 UI / UX Architecture Validation Standard — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Governance  
**Last Updated:** 2026-09-16  
**Depends On:** L6 Governance and Architecture Standard (Draft); L6 UI / UX Architecture Template v0.1 (Draft); approved L5 architecture; Architecture Level Validation Standards.  

> This is a review draft. Hard-fail identifiers and gate wording become canonical only after governance approval.

---

## 1. Purpose

Define the mandatory review gates for L6 UI / UX Architecture before it may guide production frontend implementation.

L6 validation prioritizes repository integrity, upstream traceability and user-state correctness over visual polish.

## 2. Parent / Upstream Readiness Gate

An L6 artifact may proceed only when:

- applicable L5 sources exist and are sufficiently approved/stable for L6 derivation;
- the L6 artifact does not depend on unresolved L5 decisions it would otherwise have to invent;
- material user-visible L4/L3 semantics are traceable where necessary;
- no known upstream ownership/rule/state/authority conflict prevents safe UI derivation.

If L5 is materially insufficient, L6 must be blocked by upstream rather than silently deciding solution architecture.

## 3. Proposed Hard-Fail Categories

### HF-L6-01 — Business Logic Ownership
L6 invents, changes, duplicates or claims ownership of a business rule, business fact, lifecycle, event, publisher or canonical entity.

### HF-L6-02 — Authority Corruption
A screen, route, hidden control, enabled control or client-side permission is treated as authoritative authorization or approval authority.

### HF-L6-03 — L5 Contradiction
L6 contradicts approved L5 client/security/scope/contract architecture or silently replaces an L5 technical decision.

### HF-L6-04 — Client Authority Inversion
Browser/client state is treated as canonical business, authentication, authorization, Tenant/Institution or protected-scope truth.

### HF-L6-05 — State Corruption
L6 invents, renames, merges or visually misrepresents canonical business lifecycle states/transitions in a way that changes their meaning.

### HF-L6-06 — Outcome Ambiguity
UI architecture allows pending, technical failure or indeterminate mutation outcome to be represented as authoritative business success.

### HF-L6-07 — Cross-Scope Leakage
L6 permits protected information/actions from a previous Tenant/Institution or protected context to remain active after governed context change.

### HF-L6-08 — Unsupported Persona / Actor / Action
L6 invents a business persona, actor authority, action or decision path unsupported by upstream architecture.

### HF-L6-09 — Navigation-as-Ownership
User-facing navigation or workspace composition is treated as transferring canonical domain ownership.

### HF-L6-10 — Implementation Architecture Leakage
L6 prescribes backend API/database/service/infrastructure architecture or selects a material technical product outside its authority without applicable L5/ADR governance.

### HF-L6-11 — Inaccessible Critical Journey
A required critical interaction is architected without keyboard/focus/semantic/error-accessibility behavior sufficient for the approved accessibility baseline.

### HF-L6-12 — Missing Traceability
A material L6 interaction/workspace has no defensible approved L5 provenance or cannot explain its upstream behavioral basis.

## 4. Architecture Review Checklist

### 4.1 Level Purity

- interaction/navigation/presentation only;
- no business logic ownership;
- no backend implementation architecture;
- no unauthorized technology selection.

### 4.2 Upstream Integrity

- L5 sources identified;
- L4/L3 verification included where user-visible business semantics require it;
- exact upstream terminology preserved for business states/decisions;
- no silent upstream repair.

### 4.3 Client Trust Integrity

- client state remains non-authoritative;
- UI visibility does not replace authorization;
- context/scope changes invalidate prior protected presentation as required;
- technical uncertainty does not become business success.

### 4.4 Information Architecture

- navigation is user-goal oriented;
- internal architecture sections are not mechanically exposed;
- cross-domain workspace composition preserves ownership;
- current user context is understandable.

### 4.5 Interaction Completeness

- primary journey complete;
- applicable alternate/rejection/recovery paths covered;
- loading/empty/pending/rejection/technical failure/indeterminate states distinguished;
- destructive/high-consequence actions treated appropriately;
- bulk actions state selection/impact clearly.

### 4.6 Accessibility

- keyboard operation defined;
- focus behavior defined where material;
- semantic structure and accessible naming considered;
- errors/statuses not color-only;
- text/reflow/responsive behavior considered;
- critical journeys meet the approved accessibility target.

### 4.7 Responsive/Density

- desktop enterprise workflow is usable;
- narrower viewport behavior is defined;
- mobile task support/limitations are explicit;
- dense data presentation remains legible and operable.

### 4.8 Design-System Integrity

- approved patterns reused where applicable;
- one-off components justified;
- states/accessibility are complete;
- component semantics do not embed business authority.

### 4.9 Observable Acceptance

- frontend implementation can be evaluated objectively;
- user can identify context, state and safe next action;
- no essential outcome depends on visual inference alone.

## 5. Review Verdicts

Recommended decisions:

- **PASS / Architecture Approved** — no hard failures; no material open issue.
- **PASS WITH NON-BLOCKING FINDINGS** — no hard failure; minor corrections before lock.
- **RETURN FOR CORRECTION** — L6 defect exists and upstream is sufficient.
- **BLOCKED BY UPSTREAM** — L5/higher architecture must be resolved first.

Only authorized governance may move an L6 artifact to `Locked`.

## 6. Frontend Implementation Proceed Gate

A production frontend slice should proceed when:

1. its applicable L6 foundation is approved/authorized;
2. the specific L6 interaction/workspace is approved/authorized at the required granularity;
3. L5 technical dependencies used by the frontend are stable enough for implementation;
4. no hard-fail issue is open;
5. implementation traceability target is known.

A visual prototype may be used earlier for validation only when explicitly treated as non-production evidence.

## 7. Implementation Review Feedback

Findings from the running frontend are classified as:

- **Implementation defect** — code does not conform to approved L6;
- **L6 UX defect** — approved L6 interaction needs controlled amendment;
- **L5 feasibility/architecture issue** — technical architecture requires upstream review;
- **Business architecture issue** — L4/L3/L2/L1 governance required.

No category may be silently fixed at the wrong architecture level.

## 8. Foundation Approval Gate

The initial L6 foundation may be approved only when the following are mutually consistent:

- L6 Governance and Architecture Standard;
- L6 UI / UX Architecture Template;
- L6 Validation Standard;
- L6 Design System Foundation;
- L6 Application Shell and Information Architecture;
- L6 Interaction, State and Feedback Standard.

The foundation approval does not automatically approve all future workspaces/screens.

---

**Draft verdict:** ready for governance review; not yet canonical.
