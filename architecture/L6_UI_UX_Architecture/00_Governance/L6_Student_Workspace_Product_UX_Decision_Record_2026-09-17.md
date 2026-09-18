# SutraERP L6 — Student Workspace Product / UX Decision Record

**Decision Date:** 2026-09-17  
**Architecture Level:** L6 — UI / UX Architecture Governance  
**Workspace:** `W-C02 — Student Workspace`  
**Status:** Historical Selection Record — subsequently resolved by Architecture Approved proceed-gate decision  
**Owner:** Lead Architect  
**Authority Boundary:** Product/UX direction only; no upstream business, lifecycle, authorization, API, database or technical-ownership change is authorized by this record.  

---

## 1. Purpose

This record preserves the explicit product/UX selections made for the `W-C02 — Student Workspace` production slice before formal L6 architecture approval.

**Current disposition:** The selections in this record were subsequently incorporated into and approved by `L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md`. This file preserves the dated decision history; it is not the current proceed-gate authority.

It exists to prevent implementation agents or later L6 authors from silently re-opening or replacing already-selected product direction.

The selections remain subordinate to locked/approved L0–L5 architecture and the L6 foundation authorization. Where any selection conflicts with authoritative upstream architecture, upstream architecture governs and the conflict must be reviewed explicitly.

---

## 2. Upstream Scope

The Student Workspace is anchored to final Student Lifecycle scenarios:

```text
L3-C-001
L3-C-008 … L3-C-019
```

with mechanically corresponding L4 specifications:

```text
L4-C-001
L4-C-008 … L4-C-019
```

The canonical scenario titles are:

- `L3-C-001` — Approved Candidate to Active Student;
- `L3-C-008` — Student Enrollment Update;
- `L3-C-009` — Student Academic Assignment Change;
- `L3-C-010` — Student Suspension;
- `L3-C-011` — Student Restoration After Suspension;
- `L3-C-012` — Intra-Institute Student Academic Reassignment;
- `L3-C-013` — Inter-Institute Student Transfer with Student Context Re-establishment;
- `L3-C-014` — Student Profile Update;
- `L3-C-015` — Student Communication Preference Update;
- `L3-C-016` — Student Identity Correction;
- `L3-C-017` — Duplicate Student Lifecycle Resolution;
- `L3-C-018` — Student Document or Certificate Request;
- `L3-C-019` — Student Context Archival and Permanent Record Closure.

`L3-C-001` legitimately contributes to both Admissions and Student Workspace presentation because it crosses the admission-to-active-student boundary. This record does not transfer Admissions or Student Lifecycle ownership.

---

## 3. Canonical Context and Persona Presentation

### Decision SW-UX-01 — One canonical Student context

**Selection:** one canonical Student context with two responsibility-sensitive presentations:

- authorized staff / Student Operations presentation; and
- Student self-service presentation where upstream responsibility and authorization permit.

This does **not** create separate canonical Student records or separate business lifecycles.

**UXD:** `UXD-3.02`.

### Decision SW-UX-02 — Core record plus governed projections

**Selection:** the Student Workspace contains the Student Lifecycle-owned core Student context plus governed read-only cross-domain projections.

Potential projection families include, where upstream contracts/responsibility support them:

- Academic Registration;
- Academic Progression;
- Timetable / academic context;
- Attendance;
- Internal Assessment;
- Student Fees / Finance;
- Examination;
- Scholarship;
- other source-owned status relevant to the current Student.

Detailed mutation remains in the canonical owning workspace. A projection never transfers ownership.

**UXD:** `UXD-3.06`, `UXD-4.01`, `UXD-4.05`.

---

## 4. Staff Workspace Composition

### Decision SW-UX-03 — Balanced default landing

**Selection:** the staff Student Workspace opens with a balanced overview:

1. Student identity/context header;
2. prominent current Student state and academic context;
3. compact Student Lifecycle operational attention;
4. high-value cross-domain summaries;
5. recent meaningful lifecycle activity/history.

The overview shall not become a dense enterprise record dump or a generic cross-domain work queue.

**UXD:** `UXD-4.01`, `UXD-4.03`, `UXD-4.04`.

### Decision SW-UX-04 — Staff local navigation

**Selection:** conceptual local navigation is:

```text
Overview
Student Record
Lifecycle
Related
Documents & History
```

This is information architecture, not a permanent route naming decision.

**UXD:** `UXD-4.02`.

### Decision SW-UX-05 — Hybrid projection placement

**Selection:** a small number of high-value source-owned summaries may appear on `Overview`; fuller read-only cross-domain projections belong under `Related`.

Where detailed work exists, the user is handed off to the canonical owning workspace.

**UXD:** `UXD-4.05`.

---

## 5. Student Self-Service Presentation

### Decision SW-UX-06 — Simplified student-oriented composition

**Selection:** Student self-service uses the same canonical Student context but a materially simplified presentation oriented around plain-language user goals.

Conceptual student-facing sections are:

```text
My Details
Academic Context
Requests & Documents
Related Statuses
```

Operational lifecycle controls remain staff-oriented unless upstream responsibility explicitly supports student initiation.

**UXD:** `UXD-3.02`, `UXD-3.07`, `UXD-4.02`, `UXD-4.03`.

---

## 6. Actions and Workflow Interaction

### Decision SW-UX-07 — Risk-sensitive action model

**Selection:** lifecycle actions are presented according to business consequence and upstream authority.

- routine, lower-risk Student Lifecycle actions may appear contextually where upstream behavior supports them;
- consequential lifecycle actions use explicit governed action flows rather than generic inline state editing;
- lifecycle state shall never be exposed as a free-form editable field merely for UI convenience.

Consequential examples include suspension/restoration, transfer/reassignment, identity correction, duplicate resolution and archival/closure. The examples identify presentation risk only; they do not invent eligibility or authority.

**UXD:** `UXD-5.01`, `UXD-5.02`, `UXD-5.05`, `UXD-5.08`.

### Decision SW-UX-08 — Contextual action placement

**Selection:** actions appear near the Student information they affect, with a controlled primary-action area for materially important lifecycle operations. A single unstructured mega-menu of unrelated actions is not the default.

**UXD:** `UXD-5.02`, `UXD-4.08`.

### Decision SW-UX-09 — Consequence-sensitive confirmation and outcome states

**Selection:** routine reversible work may use lightweight confirmation/feedback; high-impact operations require explicit consequence-oriented confirmation where applicable.

All mutations preserve authoritative pending/success/rejection/failure/indeterminate distinctions. Submission by the browser never equals confirmed business success.

**UXD:** `UXD-5.05`, `UXD-7.03`, `UXD-7.04`, `UXD-7.05`, `UXD-7.06`.

---

## 7. History, Evidence and Documents

### Decision SW-UX-10 — Lifecycle timeline separate from governed audit evidence

**Selection:** users may receive a human-readable lifecycle/history presentation where source data supports it. Technical/governed audit evidence remains conceptually separate and is surfaced only where upstream authorization and evidence architecture permit.

A lifecycle timeline must not be represented as the canonical audit record.

**UXD:** `UXD-4.06`.

### Decision SW-UX-11 — Integrated document/request area

**Selection:** `Documents & History` may present Student Lifecycle-owned document/certificate request journeys, their source-owned status/outcomes and available resulting documents where upstream behavior supports them.

The Student Workspace must not absorb document issuance/acceptance responsibilities owned elsewhere. Reusable technical document-platform ownership remains L5-B-04.

**UXD:** `UXD-4.07` and conditional file/evidence fields.

---

## 8. Attention, Related Information and Cross-Workspace Handoffs

### Decision SW-UX-12 — Student Lifecycle attention only as primary workspace attention

**Selection:** prominent attention items in W-C02 are limited to actionable Student Lifecycle concerns supported by upstream behavior.

Cross-domain issues may appear as contextual indicators but resolve in their canonical owning workspace.

Source-owned severity/classification must be preserved; L6 shall not manufacture criticality.

**UXD:** `UXD-7.08`, `UXD-7.07`.

### Decision SW-UX-13 — Ownership-preserving handoff

**Selection:** read-only cross-domain projections provide an understandable `View details` / `Go to owning workspace` continuation where appropriate.

W-C02 does not directly mutate Academic Registration, Attendance, Finance, Examination, Progression, Scholarship or other source-owned records.

**UXD:** `UXD-4.05`, `UXD-2.08`.

---

## 9. Search, Density and Progressive Disclosure

### Decision SW-UX-14 — Concise default, progressive detail

**Selection:** the default Student view is concise and visually quiet. Additional detail is disclosed progressively. Staff-oriented local find/search/filter may be provided where the amount of Student information justifies it and the governing contract supports it.

This does not authorize enterprise-search indexing or a new business ranking rule.

**UXD:** `UXD-6.02`, `UXD-6.05`, `UXD-6.08`, `UXD-4.03`.

---

## 10. Responsive and Mobile Direction

### Decision SW-UX-15 — Task-sensitive responsive capability

**Selection:** Student self-service is strongly mobile-capable. The staff Student Workspace remains responsive across supported viewports, but complex/high-consequence institutional operations may use larger-viewport-primary interaction patterns where justified.

Capability shall not be removed solely because the user is on a mobile viewport. If a particular operation is intentionally constrained, that constraint requires an explicit usability/architecture justification and must not masquerade as authorization.

Dense data shall be intentionally reflowed/adapted rather than blindly compressed.

**UXD:** `UXD-8.01`, `UXD-8.02`, `UXD-8.03`, `UXD-8.05`, `UXD-8.08`.

---

## 11. Information Exposure and Authority Boundary

### Decision SW-UX-16 — Responsibility-sensitive information presentation

**Selection:** staff and Student self-service may present different information depth from the same canonical Student context according to upstream responsibility and authorization.

Client-side hiding, disabling or persona composition is usability behavior only. L5-D-02 remains authoritative for protected operations and information access.

**UXD:** `UXD-3.02`, `UXD-3.03`.

---

## 12. Decision Status

All decisions in this record are:

```text
Historical selection — resolved by subsequent approved proceed gate
```

They are authorized inputs to the Student Workspace L6 draft. They are **not** a substitute for:

- L6 production-slice validation;
- L3/L4/L5 traceability;
- architecture approval of `L6_Student_Workspace_DRAFT.md`;
- a frontend implementation proceed gate;
- final L6 naming/numbering governance;
- Final L6 Corpus Lock.

No additional product/UX question is presently required for the W-C02 draft unless live upstream evidence exposes a material conflict that cannot be derived safely.
