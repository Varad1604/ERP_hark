# Template, Preference & Communication Policy Execution Architecture

## 1. Specification Metadata

**Specification ID:** `L5-C-04-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-C — Contracts, Events & Integration Architecture  
**Major Area:** L5-C-04 — Notification & Communication Architecture  
**Canonical Technical Responsibility:** Technical template/preference resolution and approved communication-policy execution.  
**Technical Architecture Owner:** L5-C-04 — Notification & Communication Architecture  
**Assigned Author:** Chetan  
**Version:** `1.1.0`  
**Status:** `Locked`  
**Last Updated:** `2026-09-23`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Stage 3 Final Closure & Stage 4 Proceed Decision 2026-09-15; `L2-H02_Notification_Workflows.md` v1.0 Locked; `L2-H02_Notification_Template_Approval_Amendment_2026-09-01.md` v1.1.0 Locked; `L5-C-04-001` v1.2.0 Architecture Approved; `L5-A-06-001`; `L5-A-06-002`; `L5-A-04-002`; `L5-B-02-003`; `L5-B-05-002`; `L5-D-01-001`; `L5-D-02-004`; `L5-D-04-001`; `L5-C-01-001`.  
**Referenced By:** `L5-C-04-001`; `L5-C-04-003`; Notification-consuming capabilities requiring a resolved Template, recipient and communication-policy context.  
**Applicable ADRs:** `ADR-L5-0004`; `ADR-L5-0006`. No C04-002-specific technology/product ADR is selected by this specification.  

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the technical architecture that resolves an H02 Notification into a delivery-ready communication context by:

- resolving an approved immutable Notification Template version;
- resolving and validating eligible communication recipients without acquiring Person/Identity ownership;
- rendering provider-independent Notification content from governed variables;
- resolving recipient preferences and subscriptions;
- executing approved communication-policy precedence, quiet-hour and Emergency/Critical override rules;
- producing a deterministic communication-policy result for `L5-C-04-001` and `L5-C-04-003`.

It also defines the technical realization of the upstream Notification Template lifecycle and Template Approval Authority without inventing a new Template event, Preference/Subscription lifecycle or business approval role.

### In Scope

- Notification Template version creation, validation, review/approval/publication enforcement and retirement realization;
- exact use of the upstream Template lifecycle;
- immutable Published Template versions and version selection;
- localization and variable-definition validation;
- provider-independent rendering;
- communication-recipient eligibility filtering over the candidate recipient set supplied by C04-001;
- recipient Preference/Subscription change admission and immutable change history;
- deterministic effective Preference/Subscription resolution;
- communication policy/configuration resolution;
- mandatory-versus-optional communication enforcement;
- quiet-hour evaluation;
- expressly permitted Emergency/Critical preference/subscription/quiet-hour override execution;
- permitted-channel policy resolution;
- sensitive-value masking/redaction handoff to `L5-B-05-002`;
- evidence handoff to D04;
- resolved-context handoff to C04-001/C04-003.

### Out of Scope / Non-Goals

This specification does not own:

- originating business facts, rules, transactions, events or publishers;
- canonical Person/Identity/User Account truth — upstream / `L5-D-01`;
- authorization policy, roles, permissions or institutional authority — upstream / `L5-D-02`;
- generic configuration registration/inheritance/effective-dating mechanics — `L5-A-06-001/002`;
- Notification identity, canonical Notification lifecycle, scheduling/background execution or lifecycle-event publication — `L5-C-04-001`;
- provider adapters, provider selection, provider health, provider failover, Delivery Attempts, receipts or final delivery evidence — `L5-C-04-003`;
- event broker/durable message transport — `L5-C-02`;
- business-document generation/template architecture — `L5-B-04-004`;
- data classification/masking policy ownership — `L5-B-05`;
- physical persistence/database technology — `L5-B-02`;
- screens, forms, preference-management UI, Template editor UI or other L6 interaction design;
- template language, rendering engine, localization library, database, cache, broker, provider or framework selection.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-C-04-002 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-C-04-002` row | Technical template/preference resolution and approved communication-policy execution. | Establishes the canonical responsibility and ownership ceiling. |
| L5 Governance | `L5_Stage3_Final_Closure_and_Stage4_Proceed_Decision_2026-09-15.md` | v1.0.0 / Locked | §6 Stage-4 Authorization | C04 is authorized to proceed with stabilized C01/C02 dependency consumption. | Authorizes Stage-4 Draft authoring. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§8.2–8.3, 10, 13–17, 20–22, 35–37 | H02 owns recipient resolution, Template management/versioning/rendering/localization, Preferences, Quiet Hours, Subscriptions and policy-driven channel context; business/identity/authorization/provider ownership remains separate. | Primary H02 capability and boundary authority. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§14–16 | Published Templates are immutable; revisions create new versions; variables validate before rendering; rendering executes no business logic; undefined variables are not rendered; sensitive values are masked where policy requires. | Defines Template/rendering technical invariants. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §§20–22 | Enterprise policy takes precedence over user preference; preference inheritance is deterministic; mandatory Notifications cannot be unsubscribed; Emergency policy may bypass preferences/subscriptions/quiet hours only where governed. | Defines Preference/Subscription/policy execution boundaries. |
| L2 | `L2-H02_Notification_Workflows.md` | v1.0 / Locked | §36 | Notification configuration is version-controlled, auditable, Tenant-isolated, supports Institution overrides and must be valid before production use. | Requires C04 policy execution to consume governed configuration rather than hard-coded local policy. |
| L2 Amendment | `L2-H02_Notification_Template_Approval_Amendment_2026-09-01.md` | v1.1.0 / Locked | §§1–3 | **Notification Template Approval Authority** approves/returns review, authorizes publication, operates in authorized scope; Template lifecycle is fixed; no new Template event is introduced. | Canonical approval-authority and event boundary. |
| L4 | `L4-H-010` | v1.0.0 / Draft | `BH-001`, `BH-002`, `BH-006`, `BH-007` | A Notification requires authorization, approved Template, eligible recipients and permitted channel/policy context; resolution must preserve Template version and recipient/channel evidence. | Drives delivery-ready resolution output. Relation state: **Specified** only. |
| L4 | `L4-H-012` | v1.1.0 / Draft | `BH-001`–`BH-006`; §§5–8 | Exact Template lifecycle, validation, approval/publication authority, immutable Published versions, retirement, no new Template event and preserved evidence. | Drives Template technical realization. Relation state: **Specified** only. |
| L4 | `L4-H-013` | v1.0.0 / Draft | `BH-001`–`BH-007` | Resolve recipient/scope, recipient-control eligibility, enterprise-policy precedence, deterministic preference inheritance, future-effective permitted changes, mandatory obligations, no invented Preference/Subscription event and preserved history. | Drives Preference/Subscription and policy-resolution architecture. Relation state: **Specified** only. |
| L4 | `L4-H-014` | v1.0.0 / Draft | `BH-001`–`BH-003`, `BH-006`, `BH-007` | H02 consumes authoritative Emergency/Critical fact, applies only expressly permitted overrides, resolves eligible recipients/channels and preserves override evidence. | Drives Emergency/Critical communication-policy execution without source-emergency ownership. Relation state: **Specified** only. |
| L5 Peer | `L5-C-04-001` | v1.2.0 / Architecture Approved | §5.5 Resolution Boundary | Baseline delegates Template/version, recipient, preference/subscription, quiet-hour and communication-policy resolution to C04-002 and consumes `ResolvedNotificationContext`. | Defines the C04 family handoff. |
| L5 Peer | `L5-A-06-001` | v1.0.0 / Architecture Approved | §§2, 4–5 | Generic governed configuration registration/versioning/effective-use control while upstream owners retain configuration meaning. | C04 policy/config artifacts consume generic configuration control-plane mechanics without transferring H02 policy ownership. |
| L5 Peer | `L5-A-06-002` | v1.0.0 / Architecture Approved | §§2–5 | Deterministic configuration scope, inheritance, override, effective-dating and immutable execution-resolution context. | Resolves governed communication configuration where H02 policy is represented through shared configuration. |
| L5 Peer | `L5-A-04-002` | v1.1.0 / Architecture Approved | §§2–4 | Tenant/Institution context propagation and fail-closed scope integrity. | Constrains Template, Preference and recipient resolution scope. |
| L5 Peer | `L5-D-01-001` | v1.3.0 / Architecture Approved | §§2–4 | IAM consumes authoritative Person/Identity/User Account truth without owning it. | C04 consumes authoritative recipient identity/account references; it does not create Person/Identity truth. |
| L5 Peer | `L5-D-02-004` | v1.0.0 / Architecture Approved | §§2–4 | Technical policy decision/enforcement while upstream owns authority/policy meaning. | Enforces protected Template/Preference operations and Template Approval Authority decisions. |
| L5 Peer | `L5-B-02-003` | v1.1.0 / Architecture Approved | §§5–8 | Owner-local transaction/concurrency and explicit conflict detection. | Governs Template/Preference mutation consistency; no distributed transaction is introduced. |
| L5 Peer | `L5-B-05-002` | v1.0.1 / Locked | §§2–8 | Reusable masking/redaction and controlled sensitive-data projection from authoritative handling directives. | Applied at rendering/output preparation where sensitive values require governed masking/redaction. |
| L5 Peer | `L5-D-04-001` | v1.1.1 / Locked | §§2–4 | Shared attributable audit/evidence architecture without transferring business meaning. | Receives Template approval/publication and Preference/policy-resolution evidence. |
| L5 Peer | `L5-C-01-001` | v1.3.0 / Architecture Approved | §§4–6 | Ownership-preserving synchronous technical contracts. | Realizes protected Template/Preference management/request boundaries where synchronous access is required. |

### 3.2 L4 Applicability

The current L4-H sources remain `Draft`. Their relations in this specification are therefore **Specified**, not `Validated`.

| L4 BH | C04-002 Technical Contribution | L5 Evidence Target | Relation State |
|---|---|---|---|
| `L4-H-010-BH-001/002` | Resolves approved Template, eligible communication recipients and governed channel/policy context. | §§5.3–5.9; `CR-001`, `CR-006`, `CR-009` | Specified |
| `L4-H-010-BH-006/007` | Returns Template/recipient/policy resolution evidence needed by the governed Notification. | §§5.9–5.10; `CR-014` | Specified |
| `L4-H-012-BH-001`–`BH-006` | Implements exact Template version lifecycle, validation, authority gate, immutable publication and retirement. | §§5.2–5.4; `CR-002`–`CR-005` | Specified |
| `L4-H-013-BH-001`–`BH-004` | Resolves recipient/scope, preference/subscription eligibility, higher-authority policy precedence and future-effective permitted changes. | §§5.5–5.8; `CR-007`–`CR-012` | Specified |
| `L4-H-013-BH-005` | Explicitly prevents invented Preference/Subscription events. | §§4, 5.5, 6; `CR-013` | Specified |
| `L4-H-013-BH-006/007` | Preserves requested change, decision, prior/resulting effective position and resolution evidence. | §§5.5, 5.10; `CR-014` | Specified |
| `L4-H-014-BH-001/002/003` | Resolves Emergency/Critical communication classification, only permitted overrides and eligible channels; provider execution remains C04-003. | §§5.7–5.9; `CR-010`–`CR-012` | Specified |
| `L4-H-014-BH-006/007` | Preserves applied override and resolved policy/recipient/channel evidence. | §5.10; `CR-014` | Specified |

### 3.3 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| `C-001` | Published Notification Template versions are immutable; content change creates a new version. | L2-H02 §14.4; `L4-H-012-BH-005` | Renderer binds to an immutable Published Template version; no in-place production edit. |
| `C-002` | Template lifecycle and approval authority are upstream-defined. | L2-H02 §15; Locked Template Approval Amendment | Technical transitions must use exact states and consume D02 enforcement of the named authority; no local approver is invented. |
| `C-003` | Templates contain presentation definitions, not executable business logic. | L2-H02 §14.4 | Rendering/variables cannot become a business-rules engine. |
| `C-004` | Recipient resolution is Tenant-scoped, deterministic, authorization-aware and completed before delivery. | L2-H02 §13 | Recipient resolution cannot create cross-Tenant targets or defer identity ownership to provider execution. |
| `C-005` | Enterprise/higher-authority policy constrains user preference; mandatory Notifications are not suppressible by optional preferences/subscriptions. | L2-H02 §§20–22; `L4-H-013-BH-001/003` | Policy resolver applies authoritative precedence before producing a delivery policy context. |
| `C-006` | Emergency preference/subscription/quiet-hour bypass is permitted only where H02 policy expressly allows it. | `L4-H-014-BH-002`; L2-H02 §§20–22 | No unrestricted emergency bypass exists. |
| `C-007` | Preference/Subscription history and prior Notification outcomes are preserved. | `L4-H-013-BH-002/004/006` | Preference changes are version/evidence preserving and do not mutate historical Notifications. |
| `C-008` | H02 defines no canonical Preference/Subscription state machine or change event. | `L4-H-013-BH-005`, §10 | C04-002 shall not invent one. |
| `C-009` | Sensitive values are masked/redacted according to authoritative handling policy. | L2-H02 §16.3; `L5-B-05-002` | Rendering consumes B05-controlled projection rather than inventing masking rules. |
| `C-010` | Generic configuration hierarchy/inheritance/effective dating is A06-owned. | `L5-A-06-001/002` | C04 owns communication-policy meaning/execution but consumes A06 technical resolution mechanics. |

### 3.4 Assumptions

No material assumption is promoted to fact. Where a particular institution has not supplied a complete communication-policy precedence or Template-selection rule, the affected runtime resolution fails rather than silently choosing a default; this is an operational configuration condition, not a new architecture assumption.

---

## 4. Technical Ownership, Boundaries & Dependencies

### Owns

`L5-C-04-002` owns:

- Notification Template technical lifecycle realization using the exact H02 states;
- Template version validation and immutable Published-version enforcement;
- technical enforcement of Notification Template Approval Authority decisions;
- Published Template selection and localization resolution;
- Template variable-definition validation and provider-independent rendering;
- communication-recipient policy eligibility resolution over C04-001-supplied candidate recipients, while identity truth and generic recipient expansion remain outside this specification;
- Preference/Subscription change technical boundary and historical change preservation;
- effective Preference/Subscription resolution;
- H02-specific communication-policy execution;
- mandatory/optional communication constraints;
- quiet-hour evaluation;
- permitted Emergency/Critical override evaluation;
- permitted-channel policy result;
- immutable resolution/evidence context returned to C04-001/C04-003.

### Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-C-04-001` | Notification baseline/lifecycle | Supplies Notification identity, validated scope, source context and the candidate recipient set; consumes the policy-eligible resolved context returned by C04-002. | Prevents C04-002 from owning Notification lifecycle/scheduling/events. |
| `L5-C-04-003` | Provider/channel delivery | Consumes eligible channel/policy/rendered-content context; owns provider selection/failover/Delivery Attempt/receipt execution. | Separates communication policy from delivery-provider mechanics. |
| `L5-A-06-001` | Governed configuration control plane | Owns generic configuration definition/version/effective-use mechanics. | Stores/admits reusable communication configuration without transferring H02 policy meaning. |
| `L5-A-06-002` | Configuration scope/inheritance/effective dating | Resolves governed Platform/Tenant/Institution/etc. configuration according to approved resolution policy. | Avoids a second generic configuration hierarchy inside C04. |
| `L5-A-04-002` | Tenant/Institution context | Supplies validated protected execution scope. | Prevents cross-scope Template/Preference/recipient resolution. |
| `L5-D-01-001` | Identity/account technical baseline | Supplies technical references to upstream-owned identity/account context. | Recipient resolution must not recreate Person/Identity truth. |
| `L5-D-02-004` | Authorization decision/enforcement | Enforces Template/Preference management and Template Approval Authority decisions. | C04 cannot invent roles/permissions/authority. |
| `L5-B-02-003` | Transaction/concurrency | Supplies owner-local conflict detection and transaction boundary. | Prevents lost Template/Preference updates. |
| `L5-B-05-002` | Sensitive-data masking/redaction | Supplies governed projection at rendering/egress preparation. | Prevents renderer from inventing data-handling policy. |
| `L5-D-04-001` | Audit/evidence | Receives attributable technical evidence. | C04 does not own audit persistence/integrity. |
| `L5-C-01-001` | Synchronous contract boundary | Realizes protected management/query calls where needed. | C04 does not choose API transport/style. |

### Must Not Own

C04-002 shall not:

- create or change source-domain business data/rules/events;
- create a new human approval role or reinterpret Notification Template Approval Authority;
- invent Template lifecycle states/transitions beyond the locked H02 model;
- invent Preference/Subscription lifecycle states or Preference/Subscription events;
- own Person/Identity/User Account business truth;
- own generic Platform/Tenant/Institution configuration hierarchy/inheritance;
- encode business logic inside Templates or variables;
- select a provider, execute provider failover or own Delivery Attempt/receipt semantics;
- bypass B05 masking/privacy controls;
- write another owner's persistence directly;
- prescribe UI interaction or Template-editor design.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| Template Version Boundary | Component / Store Boundary | Create distinct Template versions and preserve immutable Published versions. | Template definition/change request, H02 scope, predecessor version. | H02 Template version + exact upstream lifecycle state; physical storage B02-owned. | Version submitted to validation/review/publication flow. | B02, A04 | Published content cannot be edited in place; concurrent update conflict is explicit. | L2-H02 §§14–15; L4-H-012. |
| Template Validation Gate | Control Point | Validate metadata, variable definitions and content constraints before `Under Review`. | Draft Template version + governed constraints. | Validation evidence/reference. | Eligible transition request or rejection. | A06 where constraints are configured; B05 for protected-data directives where applicable. | Invalid/ambiguous metadata/variables/content cannot advance. | `L4-H-012-BH-002`. |
| Template Authority Enforcement Point | Control Point | Enforce only the upstream Notification Template Approval Authority for return/approval/publication authorization. | Current Template state, authority context, D02 Authorization Decision. | Approval/review/publication evidence; no locally invented role. | Exact permitted H02 transition request. | D02, D04 | Missing/denied authority fails closed; no publication. | Locked H02 Template Approval Amendment; `L4-H-012-BH-003/004`. |
| Published Template Resolver | Component | Resolve one eligible Published immutable Template version for Notification rendering. | Template identity/category, scope, channel/language context, effective-use configuration where defined. | Resolution provenance + exact Template version reference. | Exact Published Template version. | A06, A04 | No silent "latest" fallback when multiple versions are ambiguously eligible; unresolved selection fails. | L2-H02 §§14–16; H010. |
| Recipient Policy Eligibility Resolver | Component | Apply H02 communication eligibility, Preference/Subscription and policy constraints to the candidate recipient set supplied by C04-001 without owning identity truth or generic recipient expansion. | Notification scope, candidate recipient set, current Preference/Subscription positions, authoritative communication policy context. | Policy eligibility decisions and provenance. | Policy-eligible recipient set. | C04-001, D01 reference context, D02, A04 | Policy-ineligible recipients are excluded as governed; missing/ambiguous required context fails closed. | L2-H02 §§13, 20–22; H010/H013. |
| Preference/Subscription Change Boundary | Control Point | Admit permitted recipient preference/subscription changes and preserve prior/effective position history. | Recipient/scope, requested change, current effective position, higher-authority policy context, authorization where required. | Versioned/evidenced effective position + immutable change history; no business lifecycle state machine. | Permitted resulting position or unchanged prior position. | A04, D01, D02, B02, D04 | Policy-conflicting, mandatory-category or unresolvable changes are not applied. | L4-H-013. |
| Communication Configuration Resolver | Adapter / Control Point | Consume A06-governed configuration for communication policy, scope, effective dating and institution-specific overrides. | Validated scope + evaluation point + relevant communication configuration definition. | A06 Resolution Snapshot/Execution Configuration Context. | Resolved communication configuration. | A06-001/002 | Missing mandatory or ambiguous configuration fails A06 resolution; C04 shall not invent fallback precedence. | L2-H02 §36; A06. |
| Communication Policy Resolver | Component | Apply higher-authority policy, mandatory/optional classification, effective Preference/Subscription positions, quiet-hour rules and permitted Emergency/Critical overrides. | Notification classification/context, recipient set, A06-resolved configuration, current Preference/Subscription positions, evaluation time/context. | Immutable policy-decision context/evidence for this resolution. | Eligible recipients, permitted channels/constraints, defer/block/allow policy outcome, override evidence. | A06, A04, D02 | Ambiguous/missing required policy fails closed; optional preference cannot suppress mandatory communication; unpermitted override is rejected. | L2-H02 §§20–22; H013/H014. |
| Localization & Variable Resolver | Component | Resolve language/regional presentation and governed variable values for the selected Template. | Published Template version, recipient/context, source-provided governed values/references, resolved policy/localization context. | Resolved values/provenance; no peer business-state ownership. | Validated rendering inputs. | C01/source-owner contracts, B05 | Undefined variables are not rendered; unresolved required value prevents rendering. | L2-H02 §16. |
| Sensitive Data Projection Gate | Adapter | Apply B05-owned masking/redaction/projection before protected values enter rendered content. | Variable value + authoritative handling directive + authorization decision/reference where required. | No new business privacy state. | Governed projected value or blocked disclosure. | B05-002, D02 | Missing handling/authorization never degrades into permissive cleartext disclosure. | L2-H02 §16.3; B05-002. |
| Provider-Independent Renderer | Component | Produce deterministic provider-independent rendered Notification content from exact Template version and governed variables. | Selected Template, validated variables, localization context, governed projected values. | Rendered content reference; immutable for the delivery execution. | `ResolvedNotificationContext` to C04-001/C04-003. | Template/Variable/Policy elements | Rendering executes no business logic; provider-specific transformation remains C04-003. | L2-H02 §§14–17. |
| Evidence Handoff | Adapter | Supply Template/Preference/policy decision evidence to D04. | Version, authority, change, resolution, override and outcome references. | D04-owned evidence. | Evidence reference. | D04 | Raw secrets/credentials and unnecessary sensitive content are excluded. | H012/H013/H014 evidence obligations. |

### 5.2 Template Lifecycle Realization

C04-002 shall technically realize only the canonical H02 Template lifecycle:

```text
Draft
  ↓
Under Review
  ├────────→ Draft
  ↓
Approved
  ├────────→ Under Review
  ↓
Published
  ↓
Deprecated
  ↓
Retired
```

The Locked amendment additionally requires:

- **Notification Template Approval Authority** may return `Under Review → Draft`;
- the authority approves `Under Review → Approved`;
- the authority authorizes `Approved → Published`;
- the authority acts within institutional communication policy and authorized scope.

Technical enforcement:

```text
Template change
    ↓
new distinct Template version in Draft
    ↓
Template Validation Gate
    ↓
Under Review
    ↓
D02-enforced Notification Template Approval Authority decision
    ↓
Approved
    ↓
D02-enforced publication authorization
    ↓
Published immutable version
```

Rules:

1. Published Template content is immutable.
2. A content change creates a new Template version.
3. Published historical versions remain reconstructable.
4. `Published → Deprecated → Retired` uses the existing H02 lifecycle.
5. No Template lifecycle event is invented or published by this specification.
6. D04 receives attributable validation/review/approval/publication/retirement evidence.
7. C04-002 shall not create an alternative technical approval role.

### 5.3 Template Selection and Effective Use

A Notification shall render only from one exact eligible Published Template version.

Selection inputs may include only source/governance-supported context such as:

```text
TemplateResolutionContext
{
  Template identity/category reference
  validated Tenant / Institution scope
  Notification category/context
  permitted channel context
  language/localization context
  evaluation point where effective-use configuration applies
}
```

Rules:

- C04-002 shall not interpret `highest version`, `latest created`, storage order or another implicit convention as canonical selection unless a governed selection rule explicitly establishes it.
- Where A06-managed effective-use/scope configuration determines Template applicability, C04 consumes A06-001/002 resolution.
- If more than one version is ambiguously eligible, resolution fails.
- A selected Template version reference becomes part of the resolved Notification context returned to C04-001.
- A later Template revision shall not mutate a previously resolved historical Notification.

### 5.4 Variable Resolution, Localization and Rendering

Rendering operates on the exact selected Published Template version.

Conceptual input:

```text
TemplateRenderContext
{
  exact Template version reference
  recipient reference/context
  language/localization context
  validated source-owned variable values or references
  authoritative sensitive-data handling references where applicable
  correlation/evidence context
}
```

Architecture rules:

1. Variable definitions are validated before use.
2. Undefined variables shall not be rendered.
3. Null-value behavior follows governed rendering policy; no local default is invented.
4. Variable evaluation is deterministic for the supplied resolution context.
5. Template rendering executes presentation substitution/localization only and shall not execute business rules.
6. Business/source values remain owned by their source domains; C04 consumes values/references through governed boundaries rather than directly querying peer persistence.
7. Sensitive values use `L5-B-05-002` masking/redaction/projection rules.
8. Localization changes presentation, not Template identity.
9. The rendered content used for one delivery execution remains immutable through that execution.
10. Provider-specific payload transformation is deferred to C04-003.

### 5.5 Recipient Communication Eligibility

Generic recipient reference/group expansion, deduplication and protected-scope validation are owned by the C04-001 baseline. C04-002 receives that **candidate recipient set** and determines communication eligibility under H02 Preference/Subscription and policy rules.

```text
C04-001 candidate recipient set
        ↓
validated Tenant / Institution scope
        ↓
current Preference / Subscription positions
        +
higher-authority communication policy
        +
mandatory / optional communication classification
        ↓
recipient communication-eligibility evaluation
        ↓
policy-eligible recipient references
```

Rules:

- C04-002 shall not create or modify Person, Identity, User Account, organization membership or Role Assignment truth;
- candidate recipients remain bound to the scope validated by C04-001/A04;
- Preference/Subscription policy may filter communication eligibility only within upstream-defined authority;
- mandatory Notifications cannot be suppressed by optional Preference/Subscription choices;
- invalid or ambiguous policy context does not create a permissive recipient outcome;
- D01 remains the technical identity/account reference boundary and D02 remains the authorization owner;
- C04-002 shall not repeat generic recipient-group expansion already completed by C04-001.

### 5.6 Preference and Subscription Change Model

H02 defines Preference/Subscription behavior but no canonical lifecycle state machine. C04-002 therefore uses a **versioned effective-position model**, not an invented business lifecycle.

A change operation conceptually contains:

```text
PreferenceSubscriptionChangeContext
{
  recipient reference
  validated scope
  category / channel scope
  requested change
  recipient-control eligibility context
  current effective position reference
  higher-authority communication policy context
}
```

The technical flow is:

```text
change request
    ↓
resolve recipient + scope
    ↓
resolve higher-authority communication policy
    ↓
determine recipient-control eligibility
    ↓
evaluate mandatory / optional constraint
    ↓
policy permits? ── no ─→ preserve current effective position
    │
   yes
    ↓
commit new effective position + immutable change evidence
```

Rules:

1. A permitted change applies to future eligible Notifications as defined upstream.
2. Historical Notification outcomes are never rewritten.
3. Mandatory Notifications cannot be suppressed by an optional unsubscribe/preference.
4. A policy-conflicting change preserves the current effective position.
5. Preference/Subscription changes use owner-local conflict detection so concurrent updates do not silently overwrite each other.
6. The system preserves recipient/scope, requested change, policy decision and prior/resulting effective position references.
7. No Preference/Subscription event is created because upstream defines none.
8. No local `Active`, `Disabled`, `Subscribed`, `Unsubscribed` or similar canonical lifecycle state set is introduced unless upstream later defines one.

### 5.7 Communication Configuration and Deterministic Policy Resolution

C04-002 owns H02 communication-policy **execution**, not the generic configuration platform.

Where communication policy is configurable:

```text
H02-owned communication policy meaning
        ↓
A06-001 governed configuration definition/version
        ↓
A06-002 scope/inheritance/effective-date resolution
        ↓
immutable resolved communication configuration context
        ↓
C04-002 Communication Policy Resolver
```

The policy resolver may consume source-governed information including:

- mandatory/optional communication classification;
- recipient-control eligibility;
- preference/subscription applicability;
- channel eligibility/constraints;
- quiet-hour configuration;
- localization/time-zone context;
- Emergency/Critical override permissions;
- governed communication priority/context;
- institution-specific communication constraints.

The resolver shall not hard-code a universal precedence order across every H02 Preference scope merely because L2 lists Platform, Tenant, Institution, Organization, Department, Role, User, Category and Channel scopes.

Instead:

- enterprise/higher-authority policy always constrains lower recipient preference as required upstream;
- any additional precedence/inheritance rule required for a policy must be represented in the governed communication policy/configuration;
- A06 supplies deterministic generic configuration resolution where applicable;
- C04 applies the H02 communication meaning of the resolved policy;
- ambiguous or incomplete required policy resolution fails rather than selecting by storage order or developer preference.

### 5.8 Quiet Hours and Emergency/Critical Overrides

For an otherwise eligible recipient/channel context, C04-002 evaluates quiet-hour and override policy immediately before C04-001/C04-003 delivery eligibility.

Conceptual outcome:

```text
CommunicationPolicyOutcome
{
  eligible recipient references
  permitted channel set / channel-policy constraints
  delivery eligibility: allow / defer / block
  policy decision references
  preference/subscription decision references
  quiet-hour decision reference where applicable
  Emergency/Critical override reference where applicable
}
```

`allow / defer / block` are technical decision outcomes, not canonical H02 business lifecycle states.

Rules:

1. non-critical communication respects governed quiet hours;
2. defer decisions preserve Notification identity and are returned to C04-001 scheduling;
3. an expired Notification shall not be made executable by a quiet-hour rule;
4. Emergency/Critical override applies only where an authoritative source condition exists and H02 policy expressly permits the specific bypass;
5. C04-002 never declares or resolves the source emergency;
6. provider availability is not evaluated here; C04-003 owns provider execution/failover;
7. channel-policy output constrains C04-003 but does not select a concrete provider.

### 5.9 Resolved Notification Context

Successful resolution returns one provider-independent context to the C04 baseline/delivery boundary:

```text
ResolvedNotificationContext
{
  Notification identity
  validated Tenant / Institution scope
  exact Published Template version reference
  policy-eligible recipient reference set derived from C04-001 candidate recipients
  rendered Notification content reference
  localization context
  permitted channel / communication-policy context
  Preference / Subscription decision references where applicable
  quiet-hour decision / defer context where applicable
  Emergency/Critical override reference where applicable
  resolution provenance / evidence references
}
```

This is a conceptual architecture contract, not a wire schema.

The context shall be internally coherent: Template, recipient, scope, policy and rendered-content references shall come from one governed resolution cycle. It shall not silently combine incompatible policy/configuration versions.

### 5.10 Persistence, Concurrency and Evidence

C04-002 owns its H02-specific logical state while consuming B02 physical persistence and concurrency architecture.

#### Template versions

- each Template version has stable identity/version lineage;
- transitions are conditional on current upstream-defined lifecycle state;
- Published content is immutable;
- concurrent review/edit/publication conflicts are detected rather than overwritten.

#### Preference/Subscription positions

- the current effective position is reconstructable from preserved change history;
- a change records prior/resulting effective position references and decision provenance;
- concurrent modifications use explicit conflict detection;
- historical positions used for prior Notification evidence are not rewritten.

#### Evidence

D04 receives attributable references for:

- Template validation/review/approval/publication/retirement;
- Template version selected for a Notification;
- recipient/scope resolution;
- Preference/Subscription requested change and resulting position;
- communication-policy decision;
- quiet-hour decision;
- Emergency/Critical override;
- masking/redaction outcome references where material.

Raw secrets, credentials and unnecessary sensitive Notification content shall not be copied into evidence.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-001` | Template lifecycle realization | Use exactly H02 `Draft → Under Review → Approved → Published → Deprecated → Retired` plus defined return/reconsideration transitions; no local states. | Locked L2-H02 + Template Approval Amendment; L4-H-012 | Preserves upstream lifecycle and approval authority. | None |
| `AD-002` | Published Template mutability | Published versions are immutable; content revision creates a new version. | L2-H02 §14.4; `L4-H-012-BH-005` | Preserves reproducibility/auditability of historical Notifications. | None |
| `AD-003` | Approval enforcement | C04-002 consumes D02 authorization to enforce the upstream **Notification Template Approval Authority**; no new role/permission is defined here. | Locked amendment + D02 ownership | Preserves institutional authority while making it technically enforceable. | None |
| `AD-004` | Template selection | Resolve one exact eligible Published version through governed explicit selection/effective-use rules; ambiguous multiple eligibility fails. | H010/H012 + A06 architecture | Avoids invented "latest version" semantics. | None |
| `AD-005` | Preference/Subscription representation | Use versioned effective positions + immutable change history; do not create a canonical lifecycle state machine/event. | `L4-H-013-BH-002/005/006` | Makes changes implementable without corrupting upstream semantics. | None |
| `AD-006` | Communication-policy configuration | H02 owns policy meaning; generic configuration definition/version/scope/inheritance/effective-dating is consumed from A06-001/002. | L2-H02 §§20–22,36; A06 ownership | Prevents duplicate configuration architecture. | `ADR-L5-0004`, `ADR-L5-0006` constrain consumed scope/concurrency |
| `AD-007` | Policy precedence | Higher-authority enterprise policy constrains recipient preference; additional precedence/inheritance must come from governed policy/configuration and shall not be hard-coded by C04. | H013 + L2-H02 §20.4 | Provides deterministic execution without inventing business policy. | None |
| `AD-008` | Emergency override | Only explicitly permitted H02 override rules may bypass preference/subscription/quiet hours; source emergency ownership remains unchanged. | `L4-H-014-BH-001/002` | Prevents unrestricted bypass or emergency ownership transfer. | None |
| `AD-009` | Sensitive rendering | Rendering consumes B05-owned masking/redaction decisions; Template logic does not invent privacy controls. | L2-H02 §16.3; B05-002 | Preserves canonical privacy/data-handling ownership. | None |
| `AD-010` | Provider boundary | C04-002 returns eligible channel/policy constraints; C04-003 owns provider selection/failover/delivery evidence. | Frozen C04 corpus | Separates communication policy from provider execution. | None |
| `AD-011` | Technology position | No template engine, policy engine, rules language, localization library, database, cache, API style or provider is selected here. | L5 governance / ADR methodology | Avoids ungoverned product lock. | Required only for a later canonical material selection |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Tenant / Institution isolation | Template, recipient, Preference/Subscription and policy resolution remain bound to validated protected scope and fail closed on cross-scope ambiguity. | L2-H02; `ADR-L5-0004` | `L5-A-04-002` |
| Authorization / authority | Template approval/publication and protected Preference operations consume D02 decisions; C04 defines no role/permission. | Locked Template Approval Amendment; H013 | `L5-D-02-004` |
| Identity ownership | Recipient resolution consumes authoritative identity/account references without creating Person/Identity truth. | H013; D01 boundary | `L5-D-01-001` |
| Sensitive content | Rendering applies B05-owned masking/redaction/controlled disclosure before protected values enter rendered output. | L2-H02 §16.3 | `L5-B-05-002` |
| Configuration consistency | One policy resolution consumes a coherent A06 resolved configuration context; ambiguous/incomplete configuration does not silently resolve. | L2-H02 §36; A06 | `L5-A-06-001/002` |
| Concurrency | Template and Preference mutations use explicit owner-local conflict detection. | `ADR-L5-0006` | `L5-B-02-003` |
| Auditability | Version, authority, change, policy and override decisions are evidencable without raw protected content. | H012/H013/H014 | `L5-D-04-001` |
| Determinism | Same governed Template/policy/input resolution context yields the same technical selection/rendering result; ambiguous selection fails. | L2-H02 §§14–22 | C04-002 |
| Portability | Provider/rendering/policy products remain unselected; architecture is provider/runtime neutral. | L5 governance | C04-002 / C04-003 / E04 |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | C04-002 shall return one coherent resolved Template/recipient/communication-policy context before delivery begins; unresolved required context shall not be silently bypassed. | §§5.1, 5.9 | `L4-H-010-BH-001/002` |
| `CR-002` | Notification Template transitions shall use only the exact upstream H02 Template lifecycle and defined return/reconsideration transitions. | §5.2; `AD-001` | L2-H02 §15; L4-H-012 |
| `CR-003` | `Under Review → Approved` and authorization of `Approved → Published` shall require the upstream Notification Template Approval Authority enforced through D02; C04 shall not invent a substitute authority. | §5.2; `AD-003` | Locked Template Approval Amendment |
| `CR-004` | Published Template content shall be immutable; a content change shall create a new Template version. | §§5.2–5.3; `AD-002` | L2-H02 §14.4; `L4-H-012-BH-005` |
| `CR-005` | C04-002 shall not publish a new Template lifecycle event because upstream defines none. | §5.2 | Locked Template Approval Amendment; L4-H-012 §5 |
| `CR-006` | Template selection shall resolve exactly one eligible Published version and shall reject ambiguous multi-version eligibility rather than applying an implicit "latest" rule. | §5.3; `AD-004` | H010/H012; A06 |
| `CR-007` | Template rendering shall execute no business logic, shall reject undefined required variables and shall use governed null/localization rules. | §5.4 | L2-H02 §§14.4, 16.3 |
| `CR-008` | Sensitive values in rendering shall consume B05-owned masking/redaction/controlled-disclosure decisions; C04 shall not invent field masking patterns or disclosure authority. | §5.4; `AD-009` | L2-H02 §16.3; B05-002 |
| `CR-009` | C04-002 shall evaluate communication eligibility only over the C04-001-supplied candidate recipient set, preserve validated scope, and shall not duplicate generic recipient expansion or create/modify Person/Identity/Role Assignment truth. | §5.5 | L2-H02 §§13, 20–22; H013 |
| `CR-010` | Preference/Subscription changes conflicting with higher-authority policy or mandatory communication obligations shall not become effective; the current authoritative position shall remain unchanged. | §5.6 | `L4-H-013-BH-003/004` |
| `CR-011` | Permitted Preference/Subscription changes shall preserve prior/resulting effective-position history and shall not rewrite historical Notification outcomes. | §5.6; `AD-005` | `L4-H-013-BH-002/006` |
| `CR-012` | C04-002 shall not introduce a canonical Preference/Subscription lifecycle state machine or preference/subscription change event. | §5.6; `AD-005` | `L4-H-013-BH-005` |
| `CR-013` | Communication-policy execution shall consume A06-governed configuration resolution for generic scope/inheritance/effective-dating mechanics and shall not duplicate those mechanics locally. | §5.7; `AD-006` | L2-H02 §36; A06 |
| `CR-014` | Emergency/Critical preference/subscription/quiet-hour bypass shall occur only when an authoritative source condition exists and the applicable H02 policy expressly permits the bypass. | §5.8; `AD-008` | `L4-H-014-BH-001/002` |
| `CR-015` | C04-002 shall return permitted channel/policy constraints only; provider selection, health evaluation, failover, Delivery Attempt and receipt handling shall remain C04-003-owned. | §§5.8–5.9; `AD-010` | Frozen C04-003 boundary |
| `CR-016` | Template/Preference mutations shall use explicit owner-local conflict detection so stale writers do not silently overwrite a newer version/position. | §5.10 | `ADR-L5-0006`; B02-003 |
| `CR-017` | D04 evidence shall preserve attributable version/authority/policy/change/override references without copying secrets, credentials or unnecessary sensitive rendered content. | §5.10 | H012/H013/H014; D04/B05 |
| `CR-018` | No Template engine, policy engine, localization library, API style, database, cache, provider, framework or numeric operating threshold shall become canonical through this specification without applicable governance. | `AD-011` | L5 Governance / ADR methodology |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Material C04-002 L4 sources `L4-H-010`, `L4-H-012`, `L4-H-013` and `L4-H-014` remain `Draft`; relations cannot be `Validated`. | Upstream / L4 | Current L4 lifecycle metadata | L4 governance / authorized architecture review | **Yes for Architecture Approved; No for this authorized Draft** | Approve applicable L4 behavior or issue explicit approval-level use authorization, then revalidate relations. |
| `OD-002` | C04-001, C04-002 and C04-003 are all present on canonical `main`; the family repository-integration concern is resolved. | Resolved repository item / C04 family | Canonical repository state after family integration | Repository governance / architecture review | No | No further integration action is required; maintain synchronized ownership and handoff references when any C04 specification changes materially. |
| `OD-003` | H02 lists multiple Preference scopes but does not establish one universal hard-coded precedence order across every Preference dimension. | Upstream policy/configuration | L2-H02 §20.2 + H013 require determinism but policy values/order are institution/governance dependent | H02 communication-policy owner using A06-governed configuration | No for architecture; required before a concrete policy is activated | Each active communication policy/configuration must supply deterministic applicability/precedence; ambiguous resolution fails. |
| `OD-004` | Exact Template syntax, rendering/localization engine, policy-engine implementation and cache strategy are unselected. | Technology / Implementation | No approved technology decision | Applicable technical owner / Lead Architect if ADR is material | No for technology-neutral Draft | Select only through governed technology/ADR process if a canonical repository choice is needed. |

---

## 10. Traceability Summary

### Primary L4 Applicability
- `L4-H-010-BH-001/002/006/007` — approved Template, eligible recipient and governed communication-policy resolution.
- `L4-H-012-BH-001`–`BH-006` — exact Template lifecycle, approval/publication gate, immutable versioning and retirement.
- `L4-H-013-BH-001`–`BH-007` — Preference/Subscription resolution, mandatory-policy precedence, future-effective change and evidence; no invented event/state machine.
- `L4-H-014-BH-001`–`BH-003`, `BH-006`, `BH-007` — Emergency/Critical policy, permitted override and eligible channel/recipient resolution.

All current relations are **Specified**, not `Validated`, while the cited L4 sources remain Draft.

### Material Higher-Level / ADR Constraints
- `L2-H02_Notification_Workflows.md` §§13–17, 20–22, 35–37 — Notification Template, recipient, Preference/Subscription, quiet-hour, rendering and configuration authority.
- `L2-H02_Notification_Template_Approval_Amendment_2026-09-01.md` — Notification Template Approval Authority and exact lifecycle authority.
- `ADR-L5-0004` — protected Tenant/Institution scope.
- `ADR-L5-0006` — owner-local mutation/concurrency.

### Material L5 Dependencies
- `L5-C-04-001` — Notification baseline/lifecycle handoff.
- `L5-C-04-003` — provider/channel delivery boundary.
- `L5-A-06-001/002` — generic governed configuration control and deterministic resolution.
- `L5-A-04-002` — Tenant/Institution context.
- `L5-D-01-001` — identity/account technical reference boundary.
- `L5-D-02-004` — authorization enforcement.
- `L5-B-02-003` — transaction/concurrency.
- `L5-B-05-002` — sensitive-data projection.
- `L5-D-04-001` — audit/evidence.
- `L5-C-01-001` — synchronous contract boundary.

### Architecture Decisions
- `AD-001`–`AD-011`

### Conformance Requirements
- `CR-001`–`CR-018`

### Open / Blocking Items
- `OD-001` — Draft L4 sources block Architecture Approved status.
- `OD-002` — C04 family canonical repository integration is resolved; no repository blocker remains.
- `OD-003` — concrete active policy must supply deterministic preference precedence where needed.
- `OD-004` — technology choices intentionally deferred.


## 2026-09-23 R6 Targeted Revalidation Synchronization

R6 revalidated this specification against HF-01…HF-15 after canonical R1/R2 upstream authority consolidation and R5 affected L4→L5 traceability validation. The former Draft/Review-L4 promotion blocker is resolved. Earlier embedded source-status labels describing Draft/Review L4 state are retained only as authoring provenance and are superseded for current authority by the Architecture Approved R1/R2 parent decisions. No business ownership, event ownership, lifecycle authority, or technical responsibility is transferred by this promotion. HF-01…HF-15: PASS. Lifecycle: Architecture Approved. Final corpus lock remains separate governance.
