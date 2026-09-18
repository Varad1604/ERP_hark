# SutraERP L6 — Incremental UI Implementation Review

**Architecture Level:** L6 — UI / UX Architecture Support  
**Status:** Active Review Framework  
**Purpose:** Make implementation-authorized frontend work reviewable as realistic, reproducible user outcomes before backend integration is complete.  
**Controlling Guidance:** `../00_Governance/L6_Frontend_Backend_Integration_Guardrails.md`; applicable approved L6 package/proceed gate.

---

## 1. Delivery Unit

The architecture coverage baseline remains 99 business workspace families + 6 shared surfaces.

Frontend delivery uses **experience slices** inside approved L6 packages:

> An experience slice is a bounded, traceable increment in which a specified user, in a specified context, can achieve one meaningful observable outcome—or reach a clearly defined handoff—with the material alternate, failure, accessibility and responsive behavior reviewable.

Experience slices are delivery/review increments, not new architecture levels. Flow descriptions, state sets, fixture sets and acceptance evidence support a slice; they do not become separate central architecture registers.

---

## 2. Delivery Loop

Do not use:

```text
make static screen
→ approve appearance
→ later add workflow
```

Use:

```text
verify controlling L6 package
→ select bounded experience slice
→ define flow + acceptance boundary
→ prepare provenance-labelled fixtures
→ implement using production-intended components
→ exercise normal + applicable alternate/failure paths
→ review keyboard/mobile behavior
→ classify findings
→ fix locally or obtain controlled amendment
→ verify connected journey
→ continue
```

A visually complete screen is not a completed journey.

---

## 3. Required Review Case Shape

Every material case should be reproducible and record:

- case ID and experience-slice reference;
- starting presentation profile;
- starting Institution / academic / subject context;
- normal entry path;
- user goal;
- actions performed in sequence;
- expected observable behavior;
- alternate or failure outcomes applicable to the case;
- fixture reference and provenance/fidelity classification;
- accessibility checkpoints;
- responsive checkpoints;
- trace reference to the controlling L6 package/upstream scenario span;
- completion boundary;
- exclusions/blockers;
- actual review result and finding references.

A review case should test interaction sequence where sequence matters, not only a screenshot of a final state.

---

## 4. State Applicability

The shared review vocabulary is:

| State | Meaning |
|---|---|
| Normal populated | Current resolved data can be presented |
| Loading | Current data has not yet resolved |
| Legitimate empty | Source returned a valid no-data condition |
| Client submission pending | Client is awaiting the immediate authoritative response/acceptance position |
| Business request in progress | Source confirms a business request exists but is not complete |
| Authoritative success | Owning source confirms the relevant completion boundary |
| Business rejection / denial | Source confirms the requested business operation did not proceed |
| Technical failure | Technical execution failed; business success is not inferred |
| Indeterminate | Client cannot determine whether the mutation completed; unsafe repeat is constrained pending reconciliation |
| Stale / invalidated | Prior protected projection cannot be treated as current |
| Authorization denied | Server-authoritative refusal is presented clearly |
| Degraded source | One source is unavailable while unaffected content remains usable |

Not every state applies to every slice. Review packs must record **applicable states and justified exclusions** rather than mechanically manufacturing every combination.

---

## 5. Fixture Provenance and Fidelity

Every review fixture must state one of:

- **Presentation fixture** — architecture-supported representative content used only to exercise approved UI behavior.
- **Contract-backed integration fixture** — shaped by an actually released/stable contract and suitable for integration-oriented UI verification.
- **Integrated environment data** — returned through the real connected application boundary in a controlled environment.

Presentation fixtures shall not invent:

- lifecycle states;
- business events;
- approval authorities;
- eligibility/business rules;
- backend endpoints/payloads;
- calculations;
- authorization outcomes not supported by approved architecture;
- canonical ownership.

Fixture acceptance proves presentation behavior only. It does not prove released-contract integration or production correctness.

---

## 6. Non-Production UI Review Mode

The frontend should provide a visibly non-production **UI Review Mode** that uses the actual production-intended workspace components and handlers.

It may expose only curated supported review cases, for example:

- presentation profile;
- representative record/context;
- selected approved journey/slice;
- normal/loading/empty;
- pending/success/rejection/failure/indeterminate;
- stale/authorization-denied;
- independently degraded related source.

Requirements:

1. deterministic reset;
2. stable reproducible case IDs;
3. visible review-only indicator;
4. architecture revision, frontend revision and fixture/case revision metadata;
5. no real authorization grant;
6. no real institutional mutation from fixture controls;
7. no unrestricted combinatorial scenario builder;
8. no separate review-only reimplementation of the user journey;
9. no simple production URL parameter that enables privileged review controls.

Production enablement must be controlled at build/configuration time or an equivalently non-user-toggleable boundary.

---

## 7. Connected-Journey Acceptance

A delivery slice is not accepted merely because its local component renders correctly.

Where applicable, review:

- meaningful entry into the slice;
- preserved subject/context across section/action transitions;
- owner-preserving handoff;
- meaningful return path;
- context re-resolution after mutation;
- old-context late-response containment;
- keyboard entry, dialog interaction and focus return;
- supported mobile task completion.

If the owner destination or real integration does not exist, record a bounded preview/unavailable continuation and do not claim end-to-end acceptance.

---

## 8. Separate Readiness Dimensions

Track separately:

1. **Architecture authorization**
2. **Frontend implementation**
3. **Fixture-based UX acceptance**
4. **Released-contract integration**
5. **Integrated production validation**

`Workspace represented` does not mean `workspace complete`.  
`Fixture journey passed` does not mean `production ERP ready`.

---

## 9. Finding Classification and Disposition

| Classification | Meaning | Disposition |
|---|---|---|
| Implementation defect | Approved behavior missing/broken in code | Fix frontend |
| L6 UX architecture defect | Approved presentation/journey materially inadequate or contradictory | Controlled L6 amendment |
| L5 feasibility / integration issue | UI cannot be realized safely within technical/contract boundary | Record and escalate to L5 |
| L4/L3/L2 business architecture issue | Ownership/state/rule/authority ambiguity | Escalate upstream; do not patch in UI |
| Review-process issue | Case/fixture/evidence does not make behavior reproducible | Improve review tooling/evidence |
| Implementation-local improvement | Non-material code/presentation refinement within approved semantics | Fix locally |

Do not silently solve upstream uncertainty in frontend code.

---

## 10. Frontend Organization Principle

Frontend code should separate, as useful:

```text
feature workspace composition
staff/self-service sections
action flows
integration adapters
local presentation state
review fixtures
scripted review cases
shared UI/client infrastructure
```

Do not create one route/folder/component per experience-slice row automatically. Do not embed all fixtures, action metadata and every workspace section in one monolithic component when separation improves reviewability.

---

## 11. Review Evidence

For each reviewed case record:

- architecture commit/revision;
- frontend commit/revision;
- review-pack/case revision;
- fixture fidelity;
- automated checks run and result;
- browser interaction review performed or not performed;
- accessibility/responsive checks performed or not performed;
- findings and disposition.

Screenshots may supplement evidence but are never sufficient proof of journey correctness.
