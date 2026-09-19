# SutraERP L6 — Days 6–8 Full ERP UX Breadth

**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Architecture Base:** `061e8149aeeffef36d9a6d0d8f12a1e18c9d637b`  
**Frontend Base:** `acefde736eb8e3d2c26f0ee1b83d159cf48182fa`  
**Purpose:** Represent all 99 canonical business workspace families at meaningful shell level using the locked visual system and the established 12-archetype UX foundation.

## 1. Scope

Days 6–8 implement breadth, not deep business workflows.

The complete canonical workspace-family set remains exactly **99** business workspaces. No new business workspace family is created and no existing workspace is split, merged or renamed.

The locked visual system merged to frontend `main` is the presentation baseline. Days 6–8 shall not reopen the global visual direction except to correct a systemic defect.

## 2. Authority and Traceability

Workspace identity, scenario span and supported UX-profile language remain derived from:

- `L6_Screen_Journey_Traceability_Register_DRAFT.md`;
- the 99-workspace frontend registry;
- approved L2/L3/L4/L5 upstream sources;
- Days 1–5 L6 foundation/reference records.

The frontend may carry compact trace metadata for reviewer tooling and evidence, but that copy is not a new canonical relation store.

**UX boundary clarification (Days 1–8 cleanup):** internal architecture, traceability and review metadata is reviewer-only and shall not appear in normal end-user presentation. This includes workspace keys, L2/L3/L4/L6 identifiers, scenario spans, UX-profile labels, archetype names, implementation maturity, review maturity, fixture fidelity and revision metadata. Such fields may remain in registries, validation code and the Global Experience Lab. Ordinary ERP presentation must use task-oriented, human-readable business language and may communicate ownership or unavailable capability only where it materially helps the user's work. This clarification applies the Day-1 controlling rule and does not change workspace ownership, scenario ownership, archetype mapping, maturity classification or breadth coverage.

## 3. Breadth Completion Meaning

A workspace is `SHELL-IMPLEMENTED` when the production-intended frontend can:

1. discover/enter the canonical workspace through search or task/domain navigation;
2. identify the workspace and relevant institutional ownership/context in human-readable business language;
3. present a meaningful architecture-grounded primary user goal;
4. retain approved L3/L4 scenario span and L6 UX-profile description in reviewer evidence / Global Experience Lab, not ordinary end-user presentation;
5. compose the workspace from approved archetype primitives;
6. expose meaningful information areas and major action categories without inventing business rules;
7. show applicable loading, empty and degraded-source boundaries;
8. identify owner-preserving continuation/handoff expectations;
9. state integration and preview limitations honestly.

`SHELL-IMPLEMENTED` does **not** mean:

- a deep workflow exists;
- the workspace is integrated;
- backend contracts are known;
- fixture intent is authoritative;
- all business states are implemented;
- UX review or production validation is complete.

## 4. Primary-Goal Quality Rule

The Day-1 generic “Work with X…” placeholder form is invalid for all 99 workspace families after Days 6–8.

A breadth-level primary goal shall be derived from:

- canonical workspace name;
- approved archetype set;
- source ownership boundary;
- supported scenario/profile context.

Goals may remain pattern-based where L5 slice detail is unresolved, but must state what the user is trying to understand or accomplish and must not manufacture eligibility, approval, policy, lifecycle, calculation or completion semantics.

## 5. Information Architecture Convention

Breadth-level workspaces use the first approved archetype as the primary shell composition and may expose secondary archetype intent as supporting areas.

Reusable breadth sections are:

- **Current context** — workspace identity, current institutional/business context and applicable user presentation in human-readable language;
- **Primary work** — archetype-appropriate orientation/worklist/configuration/schedule/ledger/tracking surface;
- **What users can do here** — bounded presentation/action categories only;
- **Ownership & continuation** — source ownership and handoff boundary;
- **Integration boundary** — task-relevant availability and unavailable/deep-workflow boundary without exposing internal maturity labels.

This convention is configuration, not a thirteenth archetype.

## 6. Search and Navigation

The shell shall not expand into an unreadable list of all 99 workspaces.

All canonical workspaces must be discoverable through Global Search. Existing task/domain navigation remains curated.

Internal workspace IDs are permitted in review/trace surfaces but shall not appear in ordinary discovery or end-user presentation.

Visibility in search/navigation is not authorization.

## 7. State Coverage

All breadth workspaces shall share reusable presentation for:

- loading;
- legitimate empty state;
- degraded source/unavailable owner;
- integration pending / preview-only boundary.

Pending, rejection, technical failure, indeterminate and stale-context states are shown only where a deeper approved journey actually requires them. Breadth implementation shall not mechanically fabricate transactional outcomes.

## 8. Handoff Convention

Cross-domain continuation shall preserve canonical ownership.

A breadth shell may identify the likely owning destination from explicit registry/architecture support, but shall not invent a destination where none is established.

Where the owning deep destination is not implemented, the UI shall say so and shall not simulate end-to-end completion.

## 9. Fixture Strategy

Broad shell representation does not require bespoke fixtures for all 99 workspaces.

Days 6–8 use deterministic **breadth presentation context** derived from canonical registry/trace metadata.

Bespoke fixture data is added only when needed to demonstrate a meaningful reference/deep journey. Existing Days 3–5 fixture and Experience Lab rules remain unchanged.

## 10. Maturity

At Days 6–8 exit:

- the seven Days 3–5 references may remain `INTERACTIVE-FIXTURE-IMPLEMENTED`;
- all other workspace families may advance from `UX-MAPPED` to `SHELL-IMPLEMENTED` only if the shared breadth shell renders them and static validation confirms discoverability/metadata completeness;
- static inspection may support `STATIC-REVIEWED`, but not `UX-REVIEWED`;
- integration remains `presentation-only` unless released-contract evidence proves otherwise;
- production validation remains `not-validated`.

## 11. Locked Visual System

The frontend visual baseline at Days 6–8 start includes the merged locked visual system and subsequent sidebar alignment correction.

Breadth composition shall reuse that baseline. New workspace shells must look native to the current SutraERP design system rather than introduce a parallel visual language.

## 12. Validation Gate

Days 6–8 may pass only if:

- registry remains exactly 99 unique workspaces;
- all 99 have non-placeholder primary user goals;
- all 99 are discoverable through production-intended search;
- all 99 have architecture scenario/profile trace metadata;
- all 99 reference only approved archetypes;
- exactly 12 archetypes remain unless separately governed;
- all non-reference breadth workspaces use the typed/reusable breadth-shell composition rather than 92 copied implementations;
- no `Coming Soon` placeholder is used as completion;
- no workspace claims unsupported integration/review/production maturity;
- the seven Days 3–5 references and W-C02 review corpus remain intact;
- locked visual-system safeguards remain intact;
- `npm ci`, `npm run review:validate`, `npm run typecheck` and `npm run build` are green.

## 13. Exit Condition

Days 6–8 exit when the ERP is broadly navigable, all 99 workspace families are meaningfully represented at shell level, architecture coverage remains inspectable through reviewer tooling/evidence, and no architecture hard fail remains.

This does not authorize Days 9–11 deep-workflow semantics, backend integration, Final L6 Corpus Lock or production readiness.
