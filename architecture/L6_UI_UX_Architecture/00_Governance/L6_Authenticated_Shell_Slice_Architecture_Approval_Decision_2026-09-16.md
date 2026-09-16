# SutraERP L6 — Authenticated Shell Slice Architecture Approval Decision — 2026-09-16

**Architecture Level:** L6 — UI / UX Architecture Governance  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Decision Date:** 2026-09-16  
**Decision Type:** Controlled production-slice validation and approval  
**Scope:** `L6_Authenticated_Application_Shell_Context_Search_and_Notifications_DRAFT.md`

---

## 1. Decision

The authenticated application shell, context, global search/discovery and notification-presentation slice is **Architecture Approved for controlled frontend implementation**, subject to the implementation boundary in §6.

This decision does not convert the underlying L6 specification from `Draft` to `Locked`, does not allocate a permanent L6 identifier and does not constitute a Final L6 Corpus Lock.

The validation result is:

```text
PASS WITH NON-BLOCKING / UPSTREAM-MATURITY FINDING
```

No L6 hard-fail defect was found in the authored slice.

---

## 2. Review Basis

The review applied the current L6 validation standard, foundation authorization, application-shell foundation, interaction/state standard and screen/journey traceability baseline.

Material upstream sources reviewed include:

- `L4-A-007` — authenticated session availability without authorization transfer;
- `L4-A-050` — Workspace context remains representational and cannot grant Permission;
- `L4-B-001`, `L4-B-027`, `L4-B-028` — Institution / Academic Calendar scope provenance;
- `L4-H-010`, `L4-H-013`, `L4-H-014` — Notification, recipient preference and emergency/critical communication behavior;
- `L4-H-017`, `L4-H-018`, `L4-H-019` — authorized enterprise discovery, search projection and degraded/recovery behavior;
- `L5-G-01-003` — non-authoritative client state, protected state partitioning, context invalidation, stale-response containment and safe contract consumption;
- `L5-A-04-001/002` — Tenant/Institution isolation and context propagation;
- `L5-D-02-004` — server-side authorization decision/enforcement;
- `L5-B-01-001/002` — enterprise search/indexing and authorization-aware filtering;
- `L5-C-04-001/002/003` — notification/communication technical architecture.

---

## 3. Hard-Fail Review

| L6 hard-fail category | Result | Review conclusion |
|---|---|---|
| HF-L6-01 — Business Logic Ownership | PASS | Slice owns presentation only and explicitly preserves upstream ownership. |
| HF-L6-02 — Authority Corruption | PASS | Navigation, search visibility, context selectors and controls are non-authoritative. |
| HF-L6-03 — L5 Contradiction | PASS | Slice follows G01-003, A04, D02 and B01 boundaries. |
| HF-L6-04 — Client Authority Inversion | PASS | Client scope/session/search state remains projection/input only. |
| HF-L6-05 — State Corruption | PASS | No business lifecycle is renamed or redefined. |
| HF-L6-06 — Outcome Ambiguity | PASS | Pending/rejection/failure/stale/indeterminate states remain distinct. |
| HF-L6-07 — Cross-Scope Leakage | PASS | Context switching requires quarantine/invalidation before new protected presentation. |
| HF-L6-08 — Unsupported Persona / Actor / Action | PASS | UX profiles are explicitly non-authoritative presentation cohorts. |
| HF-L6-09 — Navigation-as-Ownership | PASS | Navigation composition does not change canonical ownership. |
| HF-L6-10 — Implementation Architecture Leakage | PASS | No backend/API/database/deployment architecture or unauthorized product choice is introduced. |
| HF-L6-11 — Inaccessible Critical Journey | PASS | Keyboard, focus, semantics, reflow and status accessibility requirements are explicit. |
| HF-L6-12 — Missing Traceability | PASS | Material L3/L4/L5 provenance is explicitly recorded. |

---

## 4. Architecture Strengths Confirmed

The approved slice correctly establishes:

- one shared authenticated shell without forcing all users into one persona/work-queue structure;
- persistent but visually quiet institutional/academic context;
- responsibility-aware navigation that cannot create or alter permissions;
- search as authorization-filtered discovery over non-canonical projections;
- canonical-owner resolution after search-result selection;
- notifications as communication outcomes distinct from workflow tasks;
- emergency/critical visual prominence only when source classification is supplied;
- local failure containment for independently unavailable search/notification projections;
- stale protected-state invalidation on context/session changes;
- observable accessibility and responsive acceptance conditions.

---

## 5. Findings

### L6-SHELL-F-001 — C04 Lifecycle Metadata Remains Review

**Classification:** Upstream maturity / non-blocking for L6 architecture; constrains live integration.

`L5-C-04-001`, `L5-C-04-002` and `L5-C-04-003` currently carry `Review` metadata. Their ownership and technical boundaries are sufficiently explicit for this L6 presentation architecture, but this L6 decision shall not silently promote those L5 specifications or their Draft/Specified L4→L5 relations.

**Required treatment:**

- notification shell/presentation may be implemented;
- mock/demo notification data remains non-authoritative evidence only;
- live notification integration shall consume only released/approved contracts;
- frontend code shall not embed provider, retry, channel, template-approval or delivery-evidence assumptions beyond released architecture/contracts;
- any material C04 change triggers dependency-impact review on this slice.

### L6-SHELL-F-002 — A04 Documents Carry Review Metadata

**Classification:** Non-blocking.

`L5-A-04-001/002` remain `Review`, but `L5-G-01-003` explicitly consumes the A04 boundary as stable and is Architecture Approved. The L6 slice therefore uses only the already stabilized fail-closed scope/isolation semantics and does not promote A04 metadata.

---

## 6. Implementation Authorization Boundary

### Authorized now

Controlled frontend implementation may proceed for:

1. shared authenticated shell framing/orientation;
2. persona/responsibility-sensitive navigation composition using approved L6 planning artifacts;
3. current Institution / academic-context presentation;
4. context-switch pending/revalidation/stale-state UX;
5. global search/discovery interaction and result presentation against approved/released search contracts;
6. search empty/degraded/stale/unavailable presentation;
7. notification entry/list/detail presentation framework;
8. notification-vs-task distinction;
9. accessible/responsive shell behavior;
10. account/session projection affordance without credential/session-secret ownership.

### Conditionally authorized

Live notification data integration is permitted only through released contracts whose semantics are stable enough for implementation. Unreleased or unresolved C04/provider mechanics shall not be inferred in frontend code.

### Not authorized by this decision

- notification administration, provider management, retry/failover controls or receipt evidence tooling;
- task/work-queue lifecycle implementation beyond a separately approved `S-06` slice;
- business workspace content not separately specified/approved at L6;
- final production persona/role selector;
- permanent L6 numbering or final folder taxonomy;
- backend/API/database changes.

---

## 7. Frontend Proceed Conditions

Before code is accepted as conforming to this approval, review shall verify that:

- the prototype persona switcher is not treated as production authorization;
- old-context protected content cannot survive a confirmed or pending cross-scope switch as current data;
- search result selection resolves into the owning workspace rather than editing search projection state;
- hidden/disabled navigation/actions are not treated as server-side authorization;
- notification entries remain informational unless a separate upstream task/action relation exists;
- technical notification/search failure remains distinct from business rejection/success;
- keyboard/focus/reflow requirements are implemented;
- no secret/authentication credential is stored in script-readable durable browser storage as a result of shell implementation.

---

## 8. Repository Integrity Verdict

**L6 level purity:** PASS  
**Canonical ownership preservation:** PASS  
**Cross-domain integrity:** PASS  
**Client trust integrity:** PASS  
**Traceability:** PASS  
**Accessibility architecture:** PASS  
**Responsive architecture:** PASS  
**Upstream notification maturity:** CONTROLLED FINDING / NON-BLOCKING FOR SHELL ARCHITECTURE  

**Final Decision:** `ARCHITECTURE APPROVED — CONTROLLED FRONTEND IMPLEMENTATION AUTHORIZED WITH C04 LIVE-INTEGRATION CONSTRAINT`.
