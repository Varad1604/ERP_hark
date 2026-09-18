# SutraERP L6 — Product / UX Decision Field Map — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Governance  
**Last Updated:** 2026-09-16  
**Depends On:** L6 Governance and Architecture Standard; L6 UI / UX Architecture Template; L6 Foundation Controlled Authorization; L6 Persona Journey and Navigation Map; approved/locked L0–L5 architecture.  
**Downstream:** L6 production-slice specifications, L6 review decisions, frontend implementation acceptance.

> This document defines the controlled field taxonomy for product and UX decisions used during L6 authoring. It does not replace upstream business architecture and does not require every field to be re-decided for every workspace.

---

## 1. Purpose

Establish one repository-wide map of the product/UX decision fields that may require explicit direction during L6 authoring.

The map prevents two failure modes:

1. asking product-direction questions that authoritative L0–L5 architecture already answers; and
2. allowing implementation agents to invent unresolved product/UX choices silently.

The objective is controlled reuse: resolve a cross-cutting field once at the correct scope, inherit it everywhere it applies, and ask for additional direction only when a later slice introduces a materially new choice.

---

## 2. Decision Source Classes

Every L6 design decision shall be classified before it is treated as an open product question.

| Class | Meaning | Required treatment |
|---|---|---|
| **U — Upstream-fixed** | L0–L5 already defines the business/technical fact | Derive it. Do not ask product stakeholders to override it at L6. |
| **P — Product-direction** | Genuine product/experience choice not determined upstream | Obtain an explicit product decision when materially unresolved. |
| **A — Architecture-derived** | L6 can derive a safe answer from approved principles/patterns | Architect may resolve it and record rationale; escalate only when the choice materially changes product behavior. |
| **S — Slice-specific** | Relevant only to a particular workspace/journey | Resolve only when that slice is authored. Do not globalize it without evidence. |
| **I — Implementation-local** | Technical or code-level choice with no material L6 semantic effect | Defer to the applicable L5/implementation boundary. |

### 2.1 Matters That Are Never Product Questions at L6

The following remain upstream-owned and shall not be turned into UX preference questions:

- canonical domain/entity/lifecycle ownership;
- business rules, calculations, thresholds and eligibility;
- lifecycle states and transitions;
- business events and publishers;
- authorization policy, permission grants or authoritative access decisions;
- authentication/session authority;
- Tenant/Institution validation authority;
- canonical actor responsibilities already established upstream;
- API/database/backend/infrastructure design;
- L5-owned technology selections;
- canonical L3/L4 outcome semantics.

L6 decides how users experience those facts, not whether those facts are true.

---

## 3. Field Resolution States

These are field-resolution states, not repository document-status values.

| Resolution state | Meaning |
|---|---|
| `Open` | A material product/UX choice remains unresolved. |
| `Selected — Pending Formalization` | Direction has been explicitly selected but has not yet been incorporated into an approved slice/decision artifact. |
| `Resolved by Approved L6 Decision` | An Architecture Approved L6 decision controls the field for its stated scope. |
| `Derived from Upstream` | Authoritative L0–L5 architecture determines the answer. |
| `Inherited L6 Default` | An existing L6 pattern controls unless a justified slice exception is approved. |
| `Conditional` | The field is considered only when the relevant interaction exists. |
| `Deferred to Implementation` | The choice has no material architecture effect and remains below L6. |

A later narrower decision may specialize a broader one only when the scope and rationale are explicit. It shall not silently override an approved cross-cutting decision.

---

# 4. Core Product / UX Decision Field Catalogue

The catalogue contains **64 core fields in eight decision families**. These are fields, not pre-written questions. Only unresolved fields material to the current scope require explicit decision input.

## Family UXD-1 — Product Experience Foundation

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-1.01` | Overall visual character | P | Resolved by Approved L6 Decision: calm, modern, premium, education-appropriate |
| `UXD-1.02` | Institutional professionalism vs youthful warmth balance | P | Partially resolved by Approved L6 Decision; persona-sensitive expression permitted |
| `UXD-1.03` | Default information density philosophy | P/A | Resolved direction: dense where needed but visually quiet |
| `UXD-1.04` | Light / dark appearance strategy | P | Light-first resolved; dark mode deferred |
| `UXD-1.05` | Brand prominence within operational work | P/A | Open at detailed expression level |
| `UXD-1.06` | Typography character and hierarchy feel | P/A | Open at detailed expression level |
| `UXD-1.07` | Shape, elevation and surface character | P/A | Open at detailed expression level |
| `UXD-1.08` | Motion / animation expressiveness | P/A | Open; constrained by accessibility and reduced-motion requirements |

## Family UXD-2 — Global Shell and Information Architecture

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-2.01` | Primary-navigation model | P/A | Resolved direction: user-goal/task-oriented, persistent/collapsible desktop navigation |
| `UXD-2.02` | Global search / command discovery prominence | P/A | Resolved as shared shell capability; detailed presentation open |
| `UXD-2.03` | Notification entry prominence and placement | P/A | Resolved as shared shell capability; detailed presentation open |
| `UXD-2.04` | Institution / academic-context prominence | P/A | Resolved: visually quiet but unmistakable |
| `UXD-2.05` | Account / profile / support placement | P/A | Partially resolved by authenticated-shell slice |
| `UXD-2.06` | Recent work / quick access / favorites strategy | P | Partially resolved; detailed global behavior open |
| `UXD-2.07` | Command/shortcut interaction strategy | P/A | Optional; exact shortcut scheme open |
| `UXD-2.08` | Cross-workspace orientation and return-path model | A | Governed by shell/journey architecture; specialize per slice where needed |

## Family UXD-3 — Persona, Responsibility and Home Experience

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-3.01` | Persona/home information priority | P | Admin, Faculty and Student home priorities resolved by Approved L6 Decision |
| `UXD-3.02` | Self-service vs operational/staff presentation of the same canonical context | P/S | Student Workspace: **Resolved by Approved L6 Decision** — one canonical context with distinct Student/staff presentations; controlling decision `L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md` |
| `UXD-3.03` | Multi-responsibility navigation composition | P/A | Governed direction: compose responsibilities without creating UX-defined Roles |
| `UXD-3.04` | Default landing experience after authenticated entry | P/S | Partially resolved; Home/deep-link rules governed, exact profile defaults slice-dependent |
| `UXD-3.05` | Work-queue philosophy by persona | P | Resolved: true queue for Admin where appropriate; not default for Student/Faculty |
| `UXD-3.06` | Cross-domain projection depth within persona/workspace experiences | P/S | Student Workspace: **Resolved by Approved L6 Decision** — core Student record + governed read-only projections with ownership-preserving handoff; controlling decision `L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md` |
| `UXD-3.07` | User-facing terminology/tone by persona | P/A | Partially resolved; exact copy remains slice-level |
| `UXD-3.08` | User-controlled personalization depth | P | Open; shall not alter authority or canonical business semantics |

## Family UXD-4 — Workspace Composition and Local Information Architecture

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-4.01` | Workspace overview breadth | P/S | Resolve per workspace; Student Workspace selected as core record + governed projections |
| `UXD-4.02` | Local navigation / section model | P/A/S | Resolve per workspace using shared navigation principles |
| `UXD-4.03` | Summary vs detail balance | P/A/S | Resolve per workspace/persona |
| `UXD-4.04` | Status / KPI / pulse prominence | P/S | Source-owned facts only; prominence resolved per workspace |
| `UXD-4.05` | Cross-domain projection placement | P/A/S | Read/projection composition allowed; canonical editing remains with owner |
| `UXD-4.06` | History / timeline / audit presentation prominence | P/S | Conditional on upstream evidence/history need |
| `UXD-4.07` | Documents / evidence placement | P/S | Conditional on supported document/evidence journeys |
| `UXD-4.08` | Secondary/rare action disclosure strategy | P/A/S | Progressive disclosure is the default; exact placement is slice-specific |

## Family UXD-5 — Actions, Forms and Workflow Interaction

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-5.01` | Direct edit vs governed request/action pattern | P/S | Must follow upstream authority; presentation choice resolved per slice |
| `UXD-5.02` | Primary-action hierarchy | P/A/S | Resolve per outcome/journey |
| `UXD-5.03` | Save / submit interaction model | P/A/S | Resolve per mutation semantics; cannot override authoritative workflow |
| `UXD-5.04` | Draft / autosave presentation model | P/S | Conditional; only if upstream/L5 supports meaningful draft semantics |
| `UXD-5.05` | Destructive/irreversible confirmation treatment | P/A/S | Conditional; must match material business consequence |
| `UXD-5.06` | Bulk-action interaction model | P/S | Conditional; only where upstream supports bulk behavior |
| `UXD-5.07` | Long-form sequencing / progressive disclosure | P/A/S | Resolve for complex forms without changing business requirements |
| `UXD-5.08` | Correction / resubmission experience | P/A/S | Derived from upstream correction/retry semantics; presentation specialized per slice |

## Family UXD-6 — Data Presentation, Exploration and Insight

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-6.01` | Table / list / card presentation balance | P/A/S | Resolve per task/data shape |
| `UXD-6.02` | Default data density | P/A/S | Comfortable default with compact enterprise variants where justified |
| `UXD-6.03` | Filter model and filter persistence | P/A/S | Resolve per workspace; persistence cannot cross protected scope unsafely |
| `UXD-6.04` | Sorting / grouping / saved-view strategy | P/S | Conditional; do not imply business ranking unless upstream owns it |
| `UXD-6.05` | Workspace-local search / find behavior | P/A/S | Resolve where datasets justify it; distinct from enterprise H03 search |
| `UXD-6.06` | Chart / visualization usage and prominence | P/S | Conditional; metrics/calculations remain upstream-owned |
| `UXD-6.07` | Export / print / download experience | P/S | Conditional on upstream/L5-supported evidence/export capability |
| `UXD-6.08` | Detail-panel / comparison / drill-down model | P/A/S | Resolve per workspace and viewport |

## Family UXD-7 — State, Feedback, Attention and Communication

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-7.01` | Loading / progressive-loading treatment | A | Governed by L6 state standard; specialize per slice |
| `UXD-7.02` | Empty-state tone and recovery action | P/A/S | Resolve per legitimate-empty semantics |
| `UXD-7.03` | Authoritative-success feedback treatment | A/S | Governed by source outcome; presentation specialized per slice |
| `UXD-7.04` | Rejection / validation / denial feedback treatment | A/S | Must preserve authoritative meaning; presentation specialized per slice |
| `UXD-7.05` | Pending / indeterminate outcome treatment | A | Governed: never collapse uncertainty into success |
| `UXD-7.06` | Technical-failure / retry / degraded-mode treatment | A/S | Governed by state standard and applicable L5 behavior |
| `UXD-7.07` | Notification vs task/work distinction in attention surfaces | P/A | Resolved: keep conceptually distinct unless upstream explicitly relates them |
| `UXD-7.08` | Alert / escalation / critical-attention prominence | P/S | Only source-owned severity/classification may drive prominence |

## Family UXD-8 — Responsive, Accessibility and Input Model

| ID | Decision field | Typical class | Current control |
|---|---|---|---|
| `UXD-8.01` | Mobile-supported task boundary | P/S | Student self-service prioritized; complex institutional operations may remain desktop/tablet-primary |
| `UXD-8.02` | Tablet / narrow-desktop adaptation | P/A | Responsive adaptation required; exact pattern specialized per slice |
| `UXD-8.03` | Dense-table mobile behavior | P/A/S | Reflow/adapt intentionally; do not blindly compress desktop tables |
| `UXD-8.04` | Keyboard interaction model | A | WCAG 2.2 AA direction; keyboard-complete supported journeys |
| `UXD-8.05` | Dialog / drawer / sheet usage model | P/A/S | Resolve by task scope, interruption cost and viewport |
| `UXD-8.06` | Focus, announcement and assistive-feedback behavior | A | Governed accessibility requirement |
| `UXD-8.07` | Reduced-motion behavior | A | Mandatory where motion is used |
| `UXD-8.08` | Touch-target / compact-density balance | P/A/S | Resolve per device/task without sacrificing accessibility |

---

# 5. Conditional Decision Fields

These fields are not part of the 64-field core because they are only material when the corresponding capability exists. They shall be evaluated rather than assumed.

| Conditional field | Trigger |
|---|---|
| Onboarding / first-use guidance | New or complex workflows require discoverability support |
| Contextual help / policy explanation | Users need governed explanatory content during a task |
| Localization / multilingual presentation | Product scope requires more than one locale/language |
| Date/time/time-zone presentation | Cross-zone or legally material time interpretation exists |
| File upload / preview / evidence handling | Workspace accepts or presents files/evidence |
| Print-intensive official output | Official workflows rely on paper/PDF presentation |
| Session-expiry / reauthentication presentation | L5/session behavior exposes a user-facing recovery need |
| Degraded-connectivity behavior | Supported tasks must remain understandable under network degradation |
| Live/collaborative updates | Multiple actors may observe changing work concurrently |
| High-risk decision confirmation | Decision has exceptional legal, financial, academic or safety consequence |
| External-system handoff | Journey crosses into DigiLocker/APAAR/payment/provider/other external systems |
| Data visualization / analytics exploration | User outcome materially depends on charts or analytical exploration |
| High-volume/bulk operations | Source behavior supports bulk processing |
| Import/export reconciliation | Workflow includes source-governed import/export/reconciliation |
| Exceptional privacy/sensitivity treatment | Upstream architecture identifies information requiring special presentation constraints |

Conditional fields do not authorize the capability that triggers them.

---

# 6. Decision Ownership and Asking Rules

## 6.1 Ask Only When a Real Product Choice Exists

An L6 author shall not ask for a preference when:

- upstream architecture already fixes the answer;
- an Architecture Approved L6 decision already covers the scope;
- a controlled L6 default can be inherited without material downside;
- the choice is implementation-local and does not alter architecture.

## 6.2 Reuse Before Re-asking

Before opening a product decision field, check in order:

1. Locked/approved upstream architecture;
2. L6 approval decisions;
3. this field map's recorded resolution state;
4. approved production-slice specifications;
5. L6 foundation defaults;
6. only then treat the field as open.

A field resolved globally shall not be re-asked for every workspace.

## 6.3 Slice-Specific Exceptions

A workspace may require a different treatment from a global default. The slice shall record:

- the field ID being specialized;
- the scope of the exception;
- the reason the inherited default is insufficient;
- affected personas/journeys;
- whether the change is presentation-only or requires governance review.

## 6.4 No Preference-Based Business Drift

No answer to a product/UX decision field may:

- create a new business rule;
- rename canonical concepts without governed terminology mapping;
- grant authority;
- redefine lifecycle state;
- change event ownership;
- move canonical ownership between domains;
- bypass L4/L5 behavior/contract constraints.

---

# 7. Mapping to L6 Production Artifact Sections

| L6 template section | Primary decision families |
|---|---|
| Purpose and user outcome | UXD-3, UXD-4 |
| Scope and boundary | UXD-3, UXD-4, UXD-5 |
| Personas / actors | UXD-3 |
| Information architecture / navigation | UXD-2, UXD-3, UXD-4 |
| Workspace / screen responsibilities | UXD-4, UXD-6 |
| Primary / alternate journeys | UXD-4, UXD-5, UXD-7 |
| Actions / interaction contract | UXD-5, UXD-7 |
| Forms / validation | UXD-5, UXD-7 |
| Data presentation | UXD-4, UXD-6 |
| Interaction states / feedback | UXD-7 |
| Authorization presentation | Upstream-fixed + UXD-3/5 presentation only |
| Accessibility | UXD-8 |
| Responsive / density | UXD-1, UXD-6, UXD-8 |
| Design-system dependencies | UXD-1, UXD-8 |
| Audit / evidence / history | UXD-4, UXD-6 + conditional fields |
| Completion conditions | All applicable families |
| Open issues / deferred decisions | All unresolved applicable fields |

---

# 8. Decision Evidence Record

When a material product/UX field is resolved, the owning L6 artifact or approval decision should record enough evidence to reconstruct the decision without relying on informal context.

Minimum record:

| Field | Required value |
|---|---|
| Decision field ID | One or more `UXD-*` identifiers |
| Scope | Global / persona / workspace / journey / surface |
| Resolution | Selected direction |
| Source class | P / A / S, or inherited U/I treatment |
| Authority/evidence | Relevant approved decision, upstream source or explicit selection record |
| Rationale | Why this direction satisfies the user outcome and repository constraints |
| Affected artifacts | L6 specifications/patterns that inherit the decision |
| Exceptions | Explicitly named narrower scopes, if any |
| Resolution state | Open / Selected — Pending Formalization / Resolved by Approved L6 Decision / etc. |

This map is not intended to become a second business-decision repository. The actual resolved direction belongs in the owning L6 specification or approval decision; this map supplies the canonical field taxonomy and inheritance rules.

---

# 9. Current Product-Direction Coverage

The following cross-cutting areas already have meaningful controlled direction and should not be restarted from zero during slice authoring:

- calm, modern, premium, education-appropriate visual direction;
- one shared design system with persona-sensitive composition;
- light-theme-first product direction;
- responsive desktop/tablet/mobile-supported experience;
- task/user-goal navigation rather than architecture/module navigation;
- Admin home centered on Institution Pulse, exceptions and true priority work;
- Faculty home centered on Teaching Pulse, Next Class and teaching attention rather than a generic enterprise queue;
- Student home centered on My Semester, Up Next and Due Soon rather than a generic enterprise queue;
- global context, search and notifications as non-authoritative shared shell concerns;
- Student Workspace working direction: one canonical Student context with different Student self-view and authorized staff presentation;
- Student Workspace working direction: core Student-owned record plus governed read-only cross-domain projections, with detailed work returning to canonical owning workspaces.

The two Student Workspace directions above were originally selected pending formalization. They are now **Resolved by Approved L6 Decision** through `L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md`. The historical selection record remains preserved; the proceed-gate decision controls current status.

---

# 10. Production Authoring Gate

Before a production L6 slice is submitted for approval, the author shall demonstrate:

1. applicable core and conditional decision fields were identified;
2. upstream-fixed matters were derived rather than re-decided;
3. existing L6 decisions/defaults were inherited where applicable;
4. any genuinely open product fields were resolved or explicitly recorded as blockers/deferred decisions;
5. any slice-specific exception identifies the global field it specializes;
6. the slice contains no silent UX decision that materially changes product behavior;
7. the implementation acceptance conditions are sufficient for an implementation agent to proceed without inventing business or product semantics.

---

## Review Verdict

**Draft verdict:** suitable as the controlled L6 product/UX decision-field taxonomy. It does not lock final visual details, permanent L6 numbering, the final screen corpus or unresolved slice-specific choices.
