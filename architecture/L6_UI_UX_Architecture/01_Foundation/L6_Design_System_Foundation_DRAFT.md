# SutraERP L6 — Design System Foundation — Draft

**Version:** 0.1.0  
**Status:** Draft  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture  
**Last Updated:** 2026-09-16  
**Depends On:** L6 Governance and Architecture Standard (Draft); `L5-G-01-001`; `L5-G-01-002`; `L5-G-01-003`.  

> This document defines experience semantics, not a component-library product selection. Framework/library choices remain governed downstream or by applicable L5/ADR decisions.

---

## 1. Purpose

Define the shared visual, interaction and accessibility language for SutraERP so that hundreds of institutional workflows behave as one coherent product rather than independent module UIs.

## 2. Product Experience Character

The target experience is:

- calm;
- information-dense without being visually noisy;
- predictable;
- fast for repeated institutional tasks;
- accessible;
- state-aware;
- role/context-aware without becoming persona-fragmented;
- trustworthy in how it communicates authoritative outcomes.

The design system shall avoid decorative complexity that reduces operational clarity.

## 3. Core Principles

1. **Clarity before decoration.** Institutional state, action and consequence must be understandable before visual flourish.
2. **Consistency before local preference.** Shared patterns are preferred over domain-specific reinvention.
3. **Workspace before CRUD.** Components support task completion and institutional context, not only entity forms.
4. **Dense but readable.** ERP users need high information throughput without sacrificing hierarchy or accessibility.
5. **State is explicit.** Success, rejection, pending, indeterminate, stale and unavailable conditions must look meaningfully different.
6. **Color is never the only carrier of meaning.**
7. **Client presentation never implies authority.** UI visibility or enabled controls do not replace authoritative enforcement.
8. **Cross-domain composition preserves source meaning.**

## 4. Token Architecture

The design system shall use semantic tokens rather than ad hoc values.

Token classes shall include:

- typography;
- spacing;
- sizing;
- radius;
- border;
- elevation;
- motion;
- focus;
- foreground/background surfaces;
- semantic status;
- interactive states;
- data visualization where later approved.

### 4.1 Semantic Color Roles

Color roles should express meaning rather than module branding.

Minimum semantic roles:

- neutral/default;
- accent/interactive;
- informational;
- positive/success;
- caution/warning;
- destructive/error;
- pending/in-progress;
- disabled/unavailable;
- selected/focused;
- critical attention.

Exact color values are intentionally deferred until visual design review.

### 4.2 Typography Roles

At minimum:

- display/major workspace title;
- page/section heading;
- subsection heading;
- body;
- compact body/data;
- label;
- helper/meta;
- code/identifier where required.

Typography shall support large institutional tables and long-form records without forcing oversized layout.

### 4.3 Density

The system shall support a comfortable default enterprise density and may support a compact density for data-heavy workspaces.

Density changes shall preserve:

- minimum interaction target accessibility;
- text legibility;
- focus visibility;
- semantic grouping;
- error/status comprehension.

## 5. Foundational Primitives

Expected primitives include:

- text;
- icon;
- divider;
- surface/container;
- stack/cluster/layout primitives;
- scroll region;
- focus boundary;
- visually hidden text;
- portal/overlay boundary.

Primitives carry no business semantics.

## 6. Core Components

The initial component architecture shall cover:

- Button / Icon Button;
- Link;
- Input / Textarea;
- Select / Combobox;
- Checkbox / Radio / Switch;
- Date/Time input presentation;
- Field / Field Group;
- Validation Message;
- Badge / Status Indicator;
- Alert / Inline Notice;
- Toast / transient feedback where appropriate;
- Tooltip;
- Popover;
- Menu;
- Dialog;
- Drawer / side panel;
- Tabs;
- Breadcrumbs;
- Pagination / continuation controls;
- Table / Data Grid foundation;
- Empty State;
- Skeleton / loading indicator;
- Progress indicator;
- Timeline / activity item;
- Key-value detail presentation;
- Search / Filter controls.

A specialized business component must be composed from approved primitives/components where possible.

## 7. Enterprise Data Table Standard — Foundation

Tables are a first-class ERP interaction surface.

The table foundation shall support, where applicable:

- accessible column headers;
- stable row identity;
- sorting;
- filtering;
- search;
- pagination/continuation;
- selectable rows;
- bulk actions;
- row-level actions;
- column visibility;
- compact density;
- empty/loading/error states;
- keyboard navigation appropriate to the chosen interaction model;
- sticky context where useful;
- responsive degradation without hiding essential meaning.

Tables shall not embed business-rule logic in presentation components.

## 8. Form Standard — Foundation

Forms shall:

- group inputs by user task and meaning;
- distinguish required and optional inputs consistently;
- provide clear labels and helper text;
- display client-side assistance separately from authoritative validation;
- keep entered values when recoverable after rejection;
- identify field-level and form-level errors;
- prevent duplicate user action while an unsafe duplicate mutation could occur;
- distinguish pending from authoritative completion;
- use confirmation only for materially destructive/irreversible or high-consequence actions.

## 9. Status and Lifecycle Presentation

Business lifecycle/status values shall use exact upstream terminology where presented.

The design system may define visual status patterns but must not:

- rename canonical business states;
- invent intermediary states;
- merge distinct business states because they look similar;
- imply a transition that has not occurred.

Technical states such as loading, stale or indeterminate must remain visually distinct from business lifecycle states.

## 10. Workflow and Approval Presentation

Reusable patterns should support:

- current business state;
- current decision point;
- decision authority context where upstream-defined;
- prior decisions/history;
- required evidence/context;
- approve/reject/return/correct actions only where upstream-supported;
- pending or indeterminate technical outcome without fake success.

The component layer does not own approval authority.

## 11. Empty, Loading, Error and Unavailable Patterns

### Legitimate Empty

Communicate that there is currently no applicable data and, where supported, the next valid user action.

### Loading

Preserve layout/context where useful and avoid implying completion.

### Authoritative Rejection

Explain the rejection in source-supported terms and present supported correction/continuation options.

### Technical Failure / Unavailable

Do not translate technical failure into business rejection.

### Indeterminate Mutation

Clearly communicate that completion is not yet confirmed; prevent unsafe duplicate action until reconciliation behavior permits it.

## 12. Accessibility Foundation

Proposed target: WCAG 2.2 AA.

All components must support:

- keyboard access where interactive;
- visible focus;
- accessible names;
- semantic HTML/ARIA appropriate to the pattern;
- contrast requirements;
- non-color-only meaning;
- zoom/text scaling;
- error/status announcement where needed;
- reduced-motion preferences.

Custom widgets require an accessibility justification and keyboard interaction definition.

## 13. Responsive Foundation

The design system shall support:

- wide desktop institutional operations;
- standard laptop widths;
- tablet layouts where workflows remain practical;
- mobile-supported task subsets;
- horizontal overflow only where a deliberate data-table strategy requires it.

Complex desktop workflows may have a reduced mobile capability if explicitly specified by L6; they shall not silently become unusable.

## 14. Design-System Governance

A new component/pattern may enter the shared system only when:

- an existing component cannot satisfy the need without distortion;
- it is reusable beyond a single one-off page or is a foundational pattern;
- accessibility behavior is defined;
- states are complete;
- it does not own business rules;
- duplication with existing components has been checked.

## 15. Implementation Handoff

The frontend repository may implement this foundation in React/TypeScript using the approved L5 web architecture.

Implementation may choose technical component primitives/libraries only through the appropriate technical governance. This document does not select Tailwind, MUI, shadcn, Radix, React Aria, Headless UI or any other product.

## 16. Open Decisions

- exact color palette;
- exact typography family;
- exact token scale values;
- exact component primitive/library strategy;
- icon library;
- charting library;
- compact-density thresholds;
- motion timings;
- responsive breakpoint values;
- dark-mode requirement.

These remain unresolved until reviewed.

---

**Draft verdict:** foundation suitable for visual-system review; not yet canonical.
