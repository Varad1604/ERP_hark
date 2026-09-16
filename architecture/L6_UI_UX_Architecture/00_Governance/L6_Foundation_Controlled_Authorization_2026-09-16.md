# SutraERP L6 — Foundation Controlled Authorization — 2026-09-16

**Architecture Level:** L6 — UI / UX Architecture  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Decision Date:** 2026-09-16  
**Authority:** Lead Architect approval following review of the running SutraERP frontend visual direction  
**Applies To:** `architecture/L6_UI_UX_Architecture/` and downstream SutraERP frontend implementation  
**Does Not Lock:** permanent L6 numbering, final folder taxonomy, exact token values, component-library product selection, icon-library ownership, dark mode, or the complete L6 screen corpus  

---

## 1. Decision

The initial SutraERP L6 foundation is **explicitly authorized for controlled production authoring and implementation slices**.

This authorization is the repository-governed foundation gate permitted by the existing L6 draft governance model. It does not silently convert every foundation draft into a Locked artifact, and it does not create a permanent L6 numbering scheme.

The six existing foundation drafts remain the working architecture package:

- `00_Governance/L6_Governance_and_Architecture_Standard_DRAFT.md`;
- `00_Governance/L6_UI_UX_Architecture_Template_v0.1_DRAFT.md`;
- `00_Governance/L6_Validation_Standard_DRAFT.md`;
- `01_Foundation/L6_Design_System_Foundation_DRAFT.md`;
- `01_Foundation/L6_Application_Shell_and_Information_Architecture_DRAFT.md`;
- `01_Foundation/L6_Interaction_State_and_Feedback_Standard_DRAFT.md`.

Where this decision explicitly resolves a matter that those drafts still describe as open or candidate, this decision has higher authority for the controlled L6 work it authorizes.

## 2. Architectural Boundary Preserved

L6 continues to own interaction, navigation, screen/workspace composition, responsive presentation, accessibility, information hierarchy and user journeys.

L6 does **not** own or redefine:

- business rules;
- canonical business states or lifecycle transitions;
- canonical domain/entity ownership;
- business-event ownership;
- approval authority;
- authorization policy;
- authentication/session authority;
- Tenant/Institution validation authority;
- API/database/backend service architecture;
- infrastructure/deployment architecture already governed upstream.

A route, visible control, hidden control, disabled control or persona presentation remains non-authoritative for security and business authority.

## 3. Approved Product Experience Direction

The approved SutraERP experience direction is:

- calm, modern and premium without becoming decorative;
- appropriate to educational institutions;
- youthful enough for students without becoming playful or juvenile;
- operationally efficient for faculty and institutional administrators;
- information-dense where work requires density, but not visually noisy;
- task-oriented rather than a collection of CRUD pages;
- one shared design system with persona-sensitive composition;
- light-theme first for the current production direction;
- responsive across desktop, tablet and mobile-supported tasks;
- progressively disclosed so users see the right information before exhaustive detail.

The current approved light visual language uses quiet neutral surfaces, a restrained indigo/blue interactive accent and semantic success/attention/review/error treatment. Exact color values remain implementation-tunable until a later design-system lock.

Dark mode is explicitly deferred. Its absence does not block the current L6 foundation or production authoring.

## 4. Approved Persona Home Architecture

The authenticated home experience shall not force all personas into the same work-queue structure.

### 4.1 Administrator

The Administrator home is intervention- and institution-oriented.

Preferred hierarchy:

```text
Context / greeting
    ↓
Institution Pulse
    ↓
Critical attention / exception context
    ↓
Priority Work / true work queue
    ↓
Recent work / re-entry
    ↓
Day / operational schedule and lower-priority institutional updates
```

A true enterprise work queue is appropriate for administrator actions such as supported approvals, reviews, exceptions and follow-ups.

### 4.2 Faculty

The Faculty home is teaching-day oriented and shall not be presented as an administrator ticket queue.

Preferred hierarchy:

```text
Teaching context / greeting
    ↓
Teaching Pulse
    ↓
Next class
    ↓
Today's Teaching / Needs Attention
    ↓
Recent teaching work
    ↓
Day schedule / lower-priority institutional updates
```

Teaching actions may represent upstream-supported attendance, review, student follow-up or assessment work, but their presentation shall remain pedagogical/task-oriented rather than an enterprise queue metaphor.

### 4.3 Student

The Student home is schedule-, deadline- and progress-oriented and shall not use an enterprise work-queue metaphor.

Preferred hierarchy:

```text
Academic context / greeting
    ↓
My Semester
    ↓
Up Next
    ↓
Due Soon
    ↓
Academic progress / recent destinations
    ↓
Day schedule / lower-priority campus updates
```

Student language shall favor understandable academic concepts over administrative workflow terminology.

## 5. Shared Pulse Pattern

A compact shared summary pattern is approved as a cross-persona design-system concept:

- Administrator: **Institution Pulse**;
- Faculty: **Teaching Pulse**;
- Student: **My Semester**.

The pattern may compose source-owned projections but does not own their data or calculations. Metrics must come from approved upstream capabilities/contracts when connected to real data.

## 6. Application Shell Direction

The controlled shell direction is approved with the following principles:

- persistent/collapsible primary navigation where desktop width permits;
- responsive drawer/bottom-navigation adaptation for supported mobile tasks;
- visible but visually quiet Institution and Academic Period context;
- global search/discovery entry point;
- notification entry point where supported;
- clear current location/workspace orientation;
- persona-relevant information architecture without treating persona switching as authorization;
- architecture/domain/module boundaries shall not mechanically become navigation labels.

The current prototype persona switcher remains review tooling only and is not an authentication, authorization or production persona-selection mechanism.

## 7. Visual Implementation Evidence

The SutraERP frontend repository is downstream implementation evidence for this decision.

At the time of this authorization, the reviewed implementation baseline is:

```text
Repository: DhruvNaik26/SutraERP_frontend
Branch: main
Commit: 2b198191ec46e27f5acf91195f9fda69136df2f9
```

The code demonstrates the approved visual direction, responsive shell and persona-specific home composition.

The frontend repository does not become the architecture authority. If code and approved L6 architecture later conflict, the architecture repository governs unless a controlled architecture amendment is approved.

## 8. Technology Boundary

This decision inherits, but does not redefine, approved L5 client architecture including React, TypeScript, React Router Framework Mode, SPA-first rendering, static deployment and a single modular web application.

Current implementation use of any icon library, CSS organization or other implementation helper is not elevated to a permanent L6 architecture decision by this authorization.

## 9. Accessibility and Responsive Direction

Controlled production L6 work shall preserve the draft foundation objective of WCAG 2.2 AA and shall define keyboard, focus, semantic, status/error, contrast, reduced-motion and reflow behavior for material interactions.

Desktop remains the primary complex institutional-work surface, while tablet and mobile shall support appropriate tasks without losing context or state meaning.

## 10. Authorized Next Work

This authorization permits the following controlled L6 activities:

1. complete screen/workspace and user-journey inventory derived from approved L3/L4/L5 architecture;
2. persona/task navigation mapping;
3. production-ready design-system pattern definition where needed by approved slices;
4. L6 workspace/journey specifications using the draft template as the controlled working template;
5. frontend implementation of an L6 slice only after its upstream L6 boundary is sufficiently approved/authorized;
6. usability and implementation feedback routed back through controlled L6 amendment when architecture changes are required.

Implementation shall not race ahead of an undefined L6 interaction boundary.

## 11. Explicitly Deferred Decisions

The following remain open and shall not be inferred from the current prototype:

- permanent L6 specification numbering;
- final L6 production folder taxonomy;
- exact design-token values;
- final production logo treatment;
- dark mode;
- shared component-library product selection;
- permanent icon-library governance;
- charting library;
- state/cache library beyond L5-governed client boundaries;
- exact responsive breakpoint values;
- complete persona-specific navigation map;
- complete screen/workspace inventory.

## 12. Proceed Gate

**Decision:** PASS — CONTROLLED L6 FOUNDATION AUTHORIZED.

The repository may proceed to L6 screen/journey inventory and controlled production UI/UX authoring. This decision does not constitute Final L6 Corpus Lock.
