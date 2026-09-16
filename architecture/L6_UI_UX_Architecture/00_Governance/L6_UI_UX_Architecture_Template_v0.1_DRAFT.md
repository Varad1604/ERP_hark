# SutraERP L6 UI / UX Architecture Template v0.1 — Draft

**Version:** 0.1.1  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Depends On:** L6 Governance and Architecture Standard; `L6_Product_UX_Decision_Field_Map_DRAFT.md`; applicable approved L5 specifications.  

> Review draft only. Do not treat this file as the locked canonical template until governance approval.

---

# <L6 Interaction / Workspace Title>

## 1. Specification Metadata

**L6 Identifier:** <provisional until naming standard approved>  
**Architecture Level:** L6 — UI / UX Architecture  
**Primary User Outcome:** <outcome>  
**Primary Business Context:** <domain / scenario / workspace context>  
**Version:** 0.1.0  
**Status:** Draft  
**Owner:** <document owner>  
**Last Updated:** <YYYY-MM-DD>  
**Depends On:** <approved L5 sources>  
**Material Upstream Verification:** <L4/L3 references where required>  
**Applicable UX Decision Fields:** <relevant UXD-* identifiers; distinguish inherited/resolved/open where material>  
**Frontend Implementation Reference:** <future path/feature reference; no source-code ownership implied>  

Before opening any product-direction decision, apply `L6_Product_UX_Decision_Field_Map_DRAFT.md`: derive upstream-fixed matters, inherit existing L6 decisions/defaults, and escalate only genuinely unresolved product/UX choices that materially affect this slice.

## 2. Purpose and User Outcome

State the user problem and the observable outcome the interaction/workspace must enable.

Do not redefine the underlying business outcome.

## 3. Scope and Boundary

### In Scope

- <interaction/navigation/presentation responsibilities>

### Out of Scope

- business-rule ownership;
- authorization authority;
- authentication/session lifecycle;
- API/database/service implementation;
- adjacent business outcomes not owned by this interaction.

## 4. Personas, Actors and Operating Context

| Persona / Actor | Context | User responsibility | Upstream authority |
|---|---|---|---|
| <actor> | <scope> | <responsibility> | <source> |

Use only actors/personas supported by authoritative architecture.

## 5. Information Architecture and Navigation Placement

Describe:

- where the interaction/workspace is discoverable;
- parent/child navigation relationship;
- contextual entry points;
- deep-link behavior conceptually;
- scope/context requirements;
- breadcrumbs or orientation requirements where useful.

Do not equate route existence with authorization.

## 6. Workspace / Screen Responsibilities

| Surface | Responsibility | Authoritative facts presented | Actions exposed |
|---|---|---|---|
| <surface> | <purpose> | <source-owned facts> | <user actions> |

A surface may compose facts across domains without transferring ownership.

## 7. Primary User Journey

Describe the minimum coherent user journey from entry to the intended UI outcome.

Keep business semantics sourced upstream; describe how the user experiences them.

## 8. Alternate and Exception Journeys

Cover only materially supported alternatives, including where applicable:

- no applicable data;
- source-defined rejection;
- correction/resubmission;
- unavailable dependency;
- permission/capability presentation;
- scope/context change;
- stale projection;
- indeterminate mutation outcome.

## 9. User Actions and Interaction Contract

| Action | Preconditions visible to user | Interaction behavior | Expected authoritative outcome source |
|---|---|---|---|
| <action> | <context> | <interaction> | <owner> |

UI controls do not create business authority.

## 10. Forms, Input and Validation Experience

For each material input flow define:

- grouping and sequence;
- required/optional presentation;
- client-side input assistance;
- authoritative validation feedback presentation;
- field/global error behavior;
- destructive or irreversible confirmation where justified;
- save/submit/pending behavior.

Client validation improves usability but never replaces authoritative server validation.

## 11. Data Presentation

Define relevant presentation patterns:

- tables/lists;
- filters/search;
- sorting;
- pagination/continuation;
- detail panels;
- summaries/KPIs;
- timeline/history;
- documents/evidence;
- status semantics.

Where data comes from multiple canonical owners, preserve source meaning.

## 12. Interaction States and Feedback

Specify applicable states:

- initial loading;
- progressive loading;
- legitimate empty state;
- authoritative success;
- authoritative rejection;
- pending mutation;
- indeterminate mutation outcome;
- technical/unavailable state;
- stale/invalidated state;
- partial composition where permitted.

Never represent technical uncertainty as confirmed business success.

## 13. Authorization and Capability Presentation

Describe only presentation behavior supported by L5 authorization/capability projections.

Rules:

- hidden/disabled controls are usability behavior, not security enforcement;
- server-side authorization remains authoritative;
- denied protected operations shall surface the authoritative denial without fabricating success;
- scope changes invalidate prior protected presentation as required by L5.

## 14. Accessibility Requirements

Define applicable requirements for:

- keyboard interaction;
- focus order and focus restoration;
- accessible names/descriptions;
- headings/landmarks;
- table semantics;
- status/error announcement;
- contrast/non-color-only meaning;
- text scaling/reflow;
- reduced motion.

Target WCAG 2.2 AA once approved by L6 governance.

## 15. Responsive and Density Behavior

Define behavior for:

- desktop institutional operations;
- smaller desktop/tablet widths;
- mobile-supported tasks;
- dense data presentation;
- overflow and secondary content;
- actions that intentionally require larger viewport where justified.

## 16. Design-System Dependencies

List reusable tokens, primitives, components and interaction patterns required.

Do not introduce a one-off pattern where an approved design-system pattern already exists.

## 17. Audit, Evidence and History Presentation

Where upstream architecture requires audit/evidence/history visibility, define how users locate and understand it without redefining the audit record itself.

## 18. Observable UX Completion Conditions

State objective conditions for an acceptable UI/UX realization, for example:

- user can identify current context and state;
- supported primary action can be completed without ambiguity;
- rejection/pending/indeterminate outcomes are distinguishable;
- keyboard-only operation is complete for the supported journey;
- no cross-scope protected projection remains visible after governed context change;
- the interaction does not imply ownership or authority inconsistent with upstream architecture.

## 19. Traceability

**Primary L5 sources:** <...>  
**Material L4/L3 verification:** <...>  
**L6 foundation dependencies:** <...>  
**UX decision field inheritance:** <applicable UXD-* fields and controlling decision/default>  

Explain any material derived constraint.

## 20. Open Issues / Deferred Decisions

Record only unresolved matters affecting L6 correctness or downstream implementation.

For material unresolved product/UX matters, reference the applicable `UXD-*` field rather than creating untracked free-form preference questions.

Implementation-specific choices remain deferred unless explicitly governed at L6/L5.

---

## Review Checklist

A reviewer shall confirm:

- correct L6 level and no business-rule ownership;
- explicit L5 traceability;
- no authorization/authentication authority leakage;
- no server/client authority inversion;
- coherent user outcome and navigation placement;
- applicable `UXD-*` fields identified and existing decisions/defaults inherited before opening new product choices;
- complete primary and material exception journeys;
- state feedback distinguishes success/rejection/pending/indeterminate/failure;
- accessibility and responsive behavior are addressed;
- design-system reuse is explicit;
- frontend implementation can be evaluated against observable acceptance conditions.
