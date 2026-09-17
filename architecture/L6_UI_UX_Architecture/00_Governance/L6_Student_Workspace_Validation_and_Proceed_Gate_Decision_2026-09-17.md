# SutraERP L6 — Student Workspace Validation & Proceed-Gate Decision

**Decision Date:** 2026-09-17  
**Architecture Level:** L6 — UI / UX Architecture Governance  
**Scope:** `W-C02 — Student Workspace`  
**Reviewed Artifact:** `L6_Student_Workspace_DRAFT.md` v0.1.0  
**Decision Type:** Production L6 Validation / Frontend Proceed Gate Re-run  
**Status:** Architecture Approved — Controlled Frontend Implementation Authorized  
**Owner:** Lead Architect  

---

## 1. Decision

The W-C02 Student Workspace L6 slice is **Architecture Approved for controlled frontend implementation** within the boundary defined by this decision.

The prior proceed-gate result recorded in this same governance artifact was `BLOCKED BY UPSTREAM TRACEABILITY`. That blocker has now been remediated through the canonical L4→L5 traceability process and the proceed gate has been re-run against current `main`.

This approval does not:

- lock the underlying L6 draft;
- allocate a permanent L6 identifier;
- constitute Final L6 Corpus Lock;
- authorize backend/API/database/service/infrastructure redesign;
- transfer business ownership, lifecycle ownership, event ownership or approval authority into L6 or the frontend.

The underlying `L6_Student_Workspace_DRAFT.md` remains a controlled draft-form architecture artifact, following the same governance pattern used for the approved authenticated-shell slice.

---

## 2. Review Baseline

The proceed-gate re-run was performed against current repository `main` after completion of the W-C02 upstream remediation.

Material review authorities include:

- `L6_Foundation_Controlled_Authorization_2026-09-16.md` — Architecture Approved L6 foundation for controlled production slices;
- `L6_Validation_Standard_DRAFT.md` — current authorized L6 validation model under the foundation decision;
- `L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md` — controlled W-C02 product/UX decisions;
- `L4_W-C02_Student_Workspace_Parent_Set_Architecture_Approval_Decision_2026-09-17.md` — Architecture Approved W-C02 parent set;
- canonical L4→L5 Section-C relation store;
- `L5_W-C02_Traceability_Validation_Decision_2026-09-17.md` — validated W-C02 BH-level L5 coverage;
- applicable approved/locked L5 client, persistence, contracts, events, authorization, audit/evidence and document-generation architecture.

---

## 3. Reviewed Scope

The exact Student Workspace scenario population remains:

```text
L3/L4-C-001
L3/L4-C-008 … L3/L4-C-019
```

The approved slice preserves:

- Student Lifecycle canonical ownership;
- Person & Identity ownership boundaries for C-016/C-017;
- Admissions boundary integrity for C-001;
- Institution & Structure / Programme context ownership boundaries;
- Document Services ownership boundaries for C-018;
- read-only cross-domain projections with ownership-preserving handoff;
- one canonical Student context with responsibility-sensitive staff and Student presentations;
- non-authoritative client state;
- server-authoritative authentication, authorization, scope and business outcomes;
- distinct pending, rejection, technical-failure and indeterminate mutation states;
- responsive and accessibility requirements;
- no L6-owned business rule, lifecycle transition or event publisher.

---

## 4. Traceability Remediation Result

The former HF-L6-12 blocker is closed by the canonical L5 validation decision.

Current upstream result:

```text
Architecture Approved L4 parents    = 13 / 13
Approved L4 BHs in scope            = 89 / 89
Canonical BH→L5 relation rows       = 130
Validated relation rows             = 130 / 130
BHs with VALIDATED COVERAGE         = 89 / 89
BHs with PARTIAL COVERAGE           = 0
BHs BLOCKED                         = 0
BHs IMPACTED                        = 0
Duplicate relation keys             = 0
```

The W-C02 L4→L5 coverage gate is therefore satisfied.

The validated relation set uses only existing canonical technical owners whose responsibilities materially contribute to the mapped behaviors. It does not force every Student Lifecycle BH onto generic workflow/platform architecture.

Controlled boundary findings remain preserved:

- C-013 receives no invented completion event, approving actor or business hand-off;
- C-018 uses document-generation architecture only for reusable technical generation/rendering/evidence responsibility and does not transfer Student request lifecycle or business issuance authority;
- event infrastructure never becomes canonical publisher of Student Lifecycle facts;
- authorization architecture enforces authority but does not create institutional authority.

---

## 5. Product / UX Decision Disposition

The controlled selections recorded in `L6_Student_Workspace_Product_UX_Decision_Record_2026-09-17.md` remain valid and incorporated into the reviewed slice.

No material product/UX decision is open for approval of W-C02.

The approved direction includes:

- one canonical Student context with responsibility-sensitive staff and Student self-service presentation;
- Student Lifecycle-owned core record plus governed read-only projections;
- balanced staff overview;
- local information architecture of `Overview / Student Record / Lifecycle / Related / Documents & History`;
- simplified Student self-service composition;
- risk-sensitive lifecycle action presentation;
- human-readable lifecycle history separate from canonical audit evidence;
- integrated supported document/certificate request presentation;
- source-owned attention and ownership-preserving cross-workspace handoff;
- concise default presentation with progressive detail;
- authoritative pending/rejection/failure/indeterminate feedback;
- task-sensitive responsive behavior and first-class Student mobile support for applicable tasks.

---

## 6. Hard-Fail Re-review

| L6 hard fail | Result | Review conclusion |
|---|---|---|
| HF-L6-01 — Business Logic Ownership | PASS | L6 owns presentation/interaction only; no Student business rule or lifecycle ownership is created. |
| HF-L6-02 — Authority Corruption | PASS | Visible/enabled controls remain non-authoritative; server-side authority is preserved. |
| HF-L6-03 — L5 Contradiction | PASS | The slice conforms to the validated W-C02 L5 contribution set and mandatory Web Client Baseline. |
| HF-L6-04 — Client Authority Inversion | PASS | Browser state remains non-authoritative for Student state, authorization, scope and business outcomes. |
| HF-L6-05 — State Corruption | PASS | Canonical Student Lifecycle meaning is preserved and state is not modeled as a generic editable field. |
| HF-L6-06 — Outcome Ambiguity | PASS | Pending, authoritative success, rejection, technical failure and indeterminate outcomes remain distinct. |
| HF-L6-07 — Cross-Scope Leakage | PASS | Invalidated Institution/protected context requires prior protected Student presentation to be quarantined/re-resolved. |
| HF-L6-08 — Unsupported Persona / Actor / Action | PASS | UX profiles do not create roles; actions remain conditional on upstream capability and authorization. |
| HF-L6-09 — Navigation-as-Ownership | PASS | Workspace composition and cross-workspace navigation do not transfer ownership. |
| HF-L6-10 — Implementation Architecture Leakage | PASS | No backend/API/database/service/deployment design is created by the L6 slice. |
| HF-L6-11 — Inaccessible Critical Journey | PASS | Keyboard, focus, semantic, error/status, reflow and responsive requirements are explicit; implementation conformance remains testable. |
| HF-L6-12 — Missing Traceability | **PASS** | 89/89 W-C02 BHs have validated canonical L5 coverage through 130 reviewed relations. |

**Hard failures open:** `0`.

---

## 7. Architecture Review Verdict

```text
W-C02 Product / UX Decisions        = COMPLETE
W-C02 L6 Draft Authoring            = COMPLETE
L3/L4 Scenario Boundary             = RESOLVED
W-C02 L4 Parent Readiness           = 13 / 13 ARCHITECTURE APPROVED
Mandatory Web Client Baseline       = RESOLVED
Domain BH→L5 Technical Coverage     = 89 / 89 VALIDATED COVERAGE
L6 hard fails 01–12                 = PASS
Architecture Approval               = AUTHORIZED
Frontend Production Implementation  = AUTHORIZED WITHIN CONTROLLED BOUNDARY
```

**Verdict:** `PASS / ARCHITECTURE APPROVED — CONTROLLED FRONTEND IMPLEMENTATION AUTHORIZED`.

---

## 8. Frontend Implementation Authorization Boundary

### 8.1 Authorized now

Controlled frontend implementation may proceed for the W-C02 Student Workspace architecture, including:

1. Student Workspace shell/context inside the already-approved authenticated application shell;
2. staff composition using `Overview / Student Record / Lifecycle / Related / Documents & History`;
3. Student self-service composition using simpler goal-oriented presentation of the same canonical Student context;
4. current Student identity/context and Student Lifecycle state presentation;
5. current/historical Student enrollment and academic-assignment presentation where supplied through authoritative boundaries;
6. supported Student Lifecycle action entry for C-008 through C-019 only where released backend capability and authorization exist;
7. deliberate high-consequence presentation for suspension, restoration, reassignment/transfer, identity correction, duplicate resolution and archival/closure;
8. read-only cross-domain projections with source/ownership-preserving continuation to owning workspaces;
9. C-018 Student document/certificate request, status and resulting-document presentation through approved business/technical boundaries;
10. lifecycle history/timeline presentation that is explicitly distinct from the canonical audit record;
11. pending/success/rejection/technical-failure/indeterminate/stale/authorization-denial states;
12. responsive desktop/tablet/mobile behavior defined by the slice;
13. keyboard/focus/semantic/status/error/reflow accessibility behavior;
14. safe re-resolution of Student data after mutation or protected-context changes.

### 8.2 Conditional implementation rule

A UI action may be implemented only when the corresponding backend capability/contract used by the frontend is released/stable enough for implementation and the current authorization context permits invocation.

Where a supported L6 action has no released implementation contract yet, the frontend may implement the approved presentation/state structure using non-authoritative fixtures or disabled integration seams, but it shall not invent request semantics, business rules, approval behavior or success conditions.

### 8.3 Not authorized by this decision

This decision does not authorize:

- backend/API/database/service changes merely to fit the UI;
- new Student Lifecycle rules, states, transitions, events or publishers;
- new Person/Identity merge or correction semantics;
- a new C-013 completion event or transfer authority;
- Document Services business issuance authority inside Student Workspace;
- cross-domain editing of Finance, Examination, Academic Registration, Progression, Attendance, Scholarship or other projected facts;
- client-side authorization as a substitute for D02 enforcement;
- permanent L6 numbering/final screen taxonomy;
- Final L6 Corpus Lock;
- implementation-local technology choices that contradict approved L5 architecture.

---

## 9. Implementation Conformance Gate

Before W-C02 frontend code is accepted as conforming to this approval, review shall verify at minimum that:

- current Student and confirmed Institution/context are unambiguous;
- staff can distinguish Student-owned information from source-owned projections;
- Student self-service does not expose staff controls or create a parallel Student model;
- Student lifecycle state is not represented as a freely editable field;
- hidden/disabled controls are not treated as authorization;
- protected deep links re-resolve current context and authority;
- high-consequence actions do not bypass source-required evidence/approval;
- identity correction does not imply Person replacement;
- duplicate resolution is not implemented as a client-side merge;
- C-018 generated artifacts are not represented as issued merely because rendering succeeded;
- cross-domain projections remain read-only and route to canonical owners for detailed work;
- stale protected data cannot remain presented as current after a scope/context invalidation;
- pending/rejection/failure/indeterminate outcomes remain visually and semantically distinct;
- technical failure never becomes business success;
- lifecycle history is not represented as canonical audit evidence;
- supported critical journeys are keyboard operable;
- status/error meaning is not color-only;
- mobile Student self-service remains coherent for supported tasks;
- no implementation introduces business, security or technical authority not present in upstream architecture.

---

## 10. Repository Synchronization Rule

The earlier blocked wording in `L6_Student_Workspace_DRAFT.md`, `L6_Screen_Journey_Traceability_Register_DRAFT.md` and `L6_UI_UX_Architecture/README.md` predates completion of the W-C02 L4→L5 remediation.

This Architecture Approved decision is authoritative over that stale wording for W-C02 lifecycle/proceed-gate interpretation.

The next mechanical edit of those Draft planning/specification artifacts shall synchronize them to the following current position without reopening product/UX decisions or changing approved upstream semantics:

```text
W-C02 Traceability                 = SATISFIED
W-C02 Trace Register State         = Slice Ready
W-C02 Architecture                 = Architecture Approved
W-C02 Frontend Implementation      = Controlled implementation authorized
```

Stale blocked wording shall not be used to deny or expand this approval boundary.

---

## 11. Change-Control Boundary

A material change to any of the following requires W-C02 impact review before continued reliance on this approval:

- bound L4-C behavior;
- canonical W-C02 BH→L5 relation state;
- material L5 owner responsibility or applicable ADR;
- Student Lifecycle / Person & Identity / Document Services ownership boundary;
- Web Client Baseline security/scope behavior;
- approved L6 interaction/state/accessibility semantics.

A materially impacted dependency shall not be silently absorbed in frontend code.

---

## 12. Final Governance Effect

This decision:

- closes the former HF-L6-12 upstream-traceability blocker;
- approves the W-C02 Student Workspace slice for controlled production frontend implementation;
- preserves all upstream canonical ownership and business semantics;
- preserves the approved authenticated-shell slice;
- does not reopen L5 Stage 6;
- does not lock the Student Workspace Draft;
- does not establish permanent L6 numbering;
- does not constitute Final L6 Corpus Lock.

**Final W-C02 Status:** `ARCHITECTURE APPROVED / SLICE READY / CONTROLLED FRONTEND IMPLEMENTATION AUTHORIZED`.
