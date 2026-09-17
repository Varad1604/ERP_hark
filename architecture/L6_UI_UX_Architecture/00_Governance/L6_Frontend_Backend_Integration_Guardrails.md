# SutraERP L6 — Frontend / Backend Integration Guardrails

**Architecture Level:** L6 — UI / UX Architecture Governance  
**Status:** Active Derived Guidance — Non-Canonical for Backend Design  
**Owner:** Lead Architect  
**Purpose:** Govern AI-assisted frontend implementation without allowing the client to invent or absorb backend/business authority.  
**Authority:** Derived from approved L5 client architecture and approved L6 governance. If this file conflicts with locked/approved upstream architecture, the upstream source wins.  

---

## 1. Why This File Exists

SutraERP frontend development is expected to be heavily AI-assisted. AI implementation agents therefore need one concise, repository-visible rule set explaining:

- when the UI must obtain data from an authoritative backend boundary;
- what may safely remain local presentation state;
- what the frontend must never decide, calculate or persist as authority;
- how to behave when a released backend contract does not yet exist;
- how to present pending, rejected, failed, indeterminate and stale outcomes without inventing business success.

This document does **not** define API endpoints, payloads, services, database schemas, backend components, protocol choices or business rules.

---

## 2. Governing Invariants

1. **Client state is never business authority.** Canonical business state remains with its owning domain and released server contract.
2. **The frontend never becomes authorization authority.** Visible, hidden, enabled or disabled controls are presentation only; protected operations remain server-enforced.
3. **Institution/Tenant scope is server-validated.** A client-selected context is non-authoritative until resolved through the approved protected execution boundary.
4. **Cross-domain projection does not transfer ownership.** A workspace may display another domain's facts, but mutation remains with the canonical owner unless an approved owning-domain contract explicitly permits the action.
5. **UI completion is not business completion.** Success may be shown only after an authoritative success outcome is known.
6. **An ambiguous mutation outcome is not success.** If the client cannot know whether a mutation completed, show an indeterminate state and reconcile before retrying or claiming completion.
7. **Protected old-context data must not survive as current data after a confirmed or pending protected context change.** Quarantine, invalidate and re-resolve it.
8. **No direct database access from the frontend.** The client consumes released application/backend contracts only.
9. **No offline authoritative business mutation.** Local queues may not create or claim canonical business outcomes unless architecture is explicitly amended upstream.
10. **No invented contract semantics.** The frontend shall not invent endpoint fallbacks, retry safety, idempotency identities, technical error meaning, approval behavior or business success conditions.

---

## 3. Data That Requires an Authoritative Backend Source

The frontend shall obtain authoritative values/outcomes through released upstream contracts when presenting or acting on:

- canonical entity/business records;
- lifecycle state and lifecycle transitions;
- enrollment, progression, examination, attendance, finance, payroll or other institutional facts;
- eligibility, approval, rejection, hold, clearance or policy outcomes;
- authorization/capability decisions;
- authenticated/session and protected scope context;
- canonical identity/person reconciliation outcomes;
- official document/certificate issuance status;
- financial calculations, balances, fees, charges, refunds or settlements;
- examination marks/results/eligibility or academic decisions;
- any workflow mutation whose result changes institutional truth;
- authoritative audit/evidence/history where required by the owning architecture.

A fixture may simulate presentation during controlled review, but shall never be represented as authoritative data.

---

## 4. State That May Remain Frontend-Local

Frontend-local state is permitted when it is non-authoritative presentation/interaction state, for example:

- current tab or panel;
- open/closed dialog, drawer or popover;
- temporary search/filter/sort selection;
- pagination/view preference;
- local focus/selection state;
- non-sensitive temporary form input before submission;
- safe display preference;
- current review-fixture/scenario selection in explicit review mode;
- pending client execution state while awaiting authoritative resolution.

Local state shall not be used to manufacture business truth.

---

## 5. What the Frontend Must Never Own

The frontend shall never independently own or decide:

- authorization or responsibility assignment;
- lifecycle transition validity;
- eligibility rules;
- approval rules or approving authority;
- fee, tax, payroll, grade, GPA, attendance or progression calculations owned upstream;
- canonical Person identity or duplicate-merge semantics;
- Tenant/Institution validation;
- business-event publication authority;
- official issuance of institutional documents;
- retry/idempotency safety;
- canonical audit/evidence truth;
- backend contract versioning or compatibility rules.

If a UI design appears to require one of these decisions locally, stop implementation and resolve the owning architecture/contract instead of coding an assumption.

---

## 6. Contract Availability Rule

For every UI action, determine one of three states:

### A. Released integration available

Use the approved backend/application contract. Render client state around the authoritative result.

### B. Approved UX exists but integration contract is not yet released

The UI may implement:

- layout and interaction structure;
- non-authoritative fixtures;
- disabled or clearly non-executing integration seams;
- pending/rejection/failure/indeterminate/stale presentation patterns.

The UI shall **not** invent request payloads, endpoint paths, workflow rules, approvals or success semantics.

### C. No approved UX/business boundary exists

Do not implement a business operation. Escalate to L6/upstream architecture rather than creating a speculative flow.

---

## 7. Mutation Outcome Model

Every authoritative mutation-capable UI shall distinguish, where applicable:

1. **Not started** — no operation in progress.
2. **Pending** — request accepted by the client but authoritative outcome not yet known.
3. **Authoritative success** — owning source confirms completion.
4. **Authoritative rejection / denial** — source confirms the operation was not permitted or did not satisfy governed conditions.
5. **Technical failure** — no business success may be inferred.
6. **Indeterminate** — the client cannot know whether the business mutation completed; reconcile before retry or success presentation.
7. **Stale / invalidated** — previously displayed protected projection is no longer trusted for the active context.

These states must not be visually collapsed into one generic success/error message.

---

## 8. Protected Context and Cache Rules

- Protected business projections are memory-scoped by default unless upstream architecture explicitly permits otherwise.
- Durable browser storage shall not contain authentication/session secrets, bearer-equivalent credentials, refresh/continuation secrets or unclassified protected business payloads.
- Client-selected Institution/Tenant references are preferences/inputs only until server validated.
- On protected context change, old protected projections shall be quarantined/invalidated and re-resolved.
- Late asynchronous results from an old context shall not populate the current protected state.
- Cached authorization is never a substitute for current server enforcement.

---

## 9. Cross-Domain Workspace Rule

A workspace may compose source-owned projections for user continuity while preserving canonical ownership.

For every projected fact the UI should make the ownership boundary discoverable where material and follow this rule:

```text
display in current workspace        = allowed when approved
edit/mutate in current workspace    = only through released owning-domain contract
ownership                           = never transferred by presentation
```

If detailed work belongs elsewhere, continue the user into the owning workspace rather than recreating the owner locally.

---

## 10. AI Implementation Checklist

Before an AI coding agent adds or changes a frontend flow, it shall answer from repository evidence:

- What L6 workspace/surface is being implemented?
- Is the slice authorized for frontend implementation?
- Which domain owns each displayed business fact?
- Which data is authoritative server state versus local presentation state?
- Which actions require a released contract?
- Are any cross-domain facts read-only here?
- What states must be shown for pending, rejection, technical failure, indeterminate and stale outcomes?
- What happens after Institution/Tenant/authenticated context changes?
- Is any local logic accidentally deciding authorization, lifecycle, eligibility or business success?
- Are fixtures clearly marked non-authoritative?

If any answer is unknown, the agent shall not invent it.

---

## 11. Workspace Integration Matrix Template

Each implementation-authorized workspace may maintain a derived integration matrix using this shape:

| UI need | Canonical owner | Frontend behavior | Backend/application requirement | Fixture allowed before integration? |
|---|---|---|---|---|
| Example authoritative state | Owning domain | Display | Read released authoritative projection | Yes, clearly non-authoritative |
| Example governed action | Owning domain | Collect intent + present outcome | Released mutation contract + server authorization | Presentation only |
| Example cross-domain status | Other domain | Read-only projection + handoff | Released read/projection contract | Yes |

The matrix records **consumption expectations only**. It shall not define endpoints or backend internals.

---

## 12. Precedence

When guidance conflicts, use repository authority order. At minimum:

```text
Locked repository decisions
→ approved/locked L0–L5 architecture
→ approved L6 slice decisions
→ this derived integration guardrail
→ frontend implementation
→ fixtures / local assumptions
```

This file exists to protect architecture while accelerating AI-assisted frontend delivery; it never replaces the canonical owners above.
