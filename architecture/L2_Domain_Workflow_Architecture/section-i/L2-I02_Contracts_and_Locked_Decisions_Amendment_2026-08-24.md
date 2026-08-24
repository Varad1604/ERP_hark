# L2-I02 — Examination Session Management — Locked Structural Amendment

| Attribute | Value |
|---|---|
| Target | L2-I02 — Examination Session Management Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-24 |
| Change Type | Structural completion / canonical consolidation |
| Authority | Explicit repository-change authorization recorded 2026-08-24 |

This amendment is part of the canonical L2-I02 architecture. It formalizes the **Contracts** and **Locked Decisions** sections that the original L2-I02 canonical crosswalk recorded as absent. It introduces no new ownership, lifecycle state, event publisher, authority, business rule, or implementation concept. Existing L2-I02 and L2-I16 decisions remain authoritative.

---

## 1. Canonical Business Contracts and Hand-offs

All contracts below are business-level exchanges of authoritative facts or governed business context. No API, message schema, transport, persistence, or technical integration mechanism is defined.

### 1.1 Contracts Consumed by L2-I02

| Provider / Canonical Owner | Business Contract / Hand-off | L2-I02 Use | Ownership Boundary |
|---|---|---|---|
| Institution Domain | Authoritative Institution context and institutional examination-governance facts | Establish institutional ownership and governance context for an Examination Session. | Institution facts remain owned by Institution. |
| Organisation Domain | Authoritative organisational structure and administrative-scope facts | Establish campus, faculty, school, department, and administrative applicability. | Organisation facts remain owned by Organisation. |
| Curriculum Framework Domain | Programme, course, curriculum-version, and academic-applicability facts | Establish academic applicability of the Examination Session. | Curriculum facts remain owned by Curriculum. |
| Academic Calendar Domain | Published academic period and examination-window context | Establish the academic time context in which the Examination Session may operate. | Academic Calendar facts remain owned by Academic Calendar. |
| L2-I01 Examination Policy & Configuration | Approved Examination Policy | Governs session approval, activation, and downstream examination behaviour without duplicating policy ownership. | Examination Policy remains owned by L2-I01. |
| Platform Foundation capabilities | Identity, authorization, event, notification, and audit capabilities | Support the governed business workflow without acquiring Examination Session ownership. | Platform capability ownership remains unchanged. |

### 1.2 Existing Consumed Business Events

The following existing L2-I02 consumed events remain canonical:

- `Academic Period Published` — Academic Calendar Domain.
- `Curriculum Updated` — Curriculum Framework Domain.
- `Programme Structure Changed` — Curriculum Framework Domain.
- `Examination Policy Approved` — L2-I01.
- `Institution Updated` — Institution Domain.
- `Organisation Structure Updated` — Organisation Domain.
- `Administrative Approval Granted` — applicable governance process.
- `Audit Request Raised` — Audit Infrastructure.

This amendment does not rename or republish these events.

### 1.3 Contracts Produced by L2-I02

| Provider | Consumer | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| L2-I02 Examination Session Management | L2-I03 Examination Planning & Scheduling | Approved/Active Examination Session context and applicable session governance facts | Permit planning only within a governed Examination Session. |
| L2-I02 Examination Session Management | L2-I04 through applicable downstream Examination workflows | Active Examination Session context and authoritative lifecycle milestones | Permit downstream examination operations only when the session is in the required L2-I02 state. |
| L2-I02 Examination Session Management | L2-I16 Session Closure & Archival | Examination Session released for closure after operational conclusion | Hand off lifecycle responsibility at the existing ownership boundary without transferring historical ownership of earlier L2-I02 decisions. |

### 1.4 Existing Published Business Events

The following existing L2-I02 published events remain canonical:

- `Examination Session Created`
- `Examination Session Updated`
- `Examination Session Submitted`
- `Examination Session Approved`
- `Examination Session Activated`
- `Examination Session Suspended`
- `Examination Session Reopened`
- `Examination Session Handed Over for Closure`

L2-I02 remains the canonical publisher for these Examination Session lifecycle milestones where the original document defines them. Closure and archival lifecycle events from `Closure Initiated` onward remain owned by L2-I16 under the existing repository ruling.

---

## 2. Locked Decisions

The following existing architectural decisions are consolidated and Locked for canonical traceability:

1. **Canonical lifecycle owner before closure:** L2-I02 owns Examination Session creation, configuration, classification, review/approval, scheduling, activation, suspension, reopening, version management, and governance through the pre-closure lifecycle.
2. **Closure and archival ownership:** L2-I16 exclusively owns Examination Session closure and archival from `Closure Initiated` onward, as already resolved under repository conflict `I-RC-01`.
3. **Lifecycle integrity:** Every Examination Session begins in `Draft`; only reviewed sessions may become `Approved`; approved sessions may become `Scheduled`; scheduled sessions become `Active` at the appropriate academic period; Active sessions may be `Suspended`; Suspended sessions may be `Reopened`.
4. **Operational-consumption rule:** Only `Active` Examination Sessions may be consumed by downstream operational examination workflows.
5. **Policy ownership:** Every Examination Session references an approved Examination Policy from L2-I01; L2-I02 consumes policy and does not redefine it.
6. **Academic-context ownership:** Institution, Organisation, Curriculum, and Academic Calendar facts remain owned by their originating domains and are consumed by L2-I02 without ownership transfer.
7. **Session classification is not event ownership:** Regular, Supplementary, Improvement, Makeup, Special, Entrance, Certification, Exit, and similar session types are classifications, not lifecycle events.
8. **Historical integrity:** governed versioning or amendments must preserve the Examination Session context under which completed examination activities occurred.
9. **Audit ownership boundary:** L2-I02 owns the business semantics of Examination Session decisions and lifecycle evidence; Platform Audit Infrastructure owns audit capability implementation.
10. **Implementation independence:** L2-I02 defines business workflow architecture only and shall not define APIs, databases, UI behaviour, technical scheduling algorithms, deployment, storage models, or implementation recovery mechanics.
11. **Cross-domain hand-offs are contracts, not ownership transfers:** downstream workflows consume approved/active session context and do not acquire ownership of the Examination Session lifecycle.
12. **L3 consumption rule:** Level 3 scenarios shall consume these L2 decisions without inventing new actors, states, event names, contract names, ownership, authority, or implementation mechanics.

---

## 3. Canonical Section Completion

For repository-governance purposes, this amendment closes the two gaps recorded by the original L2-I02 canonical crosswalk:

| Required L2 Section | Canonical Location After Amendment |
|---|---|
| Contracts | This amendment §1 |
| Locked Decisions | This amendment §2 |

The original L2-I02 document, its existing ownership-resolution ruling, and this structural amendment shall be read together as the canonical L2-I02 architecture.
