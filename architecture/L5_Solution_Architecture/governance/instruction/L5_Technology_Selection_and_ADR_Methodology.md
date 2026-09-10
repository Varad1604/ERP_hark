# SutraERP L5 Technology Selection & ADR Methodology

**Version:** 1.0.1  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-09-09  
**Depends On:** L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0; L5 Validation Standard; approved L5 Candidate Document Corpus Register  
**Referenced By:** L5 specifications, Architecture Decision Records, technology evaluations and architecture reviews  

---

## 1. Purpose

This methodology governs material technical decisions and technology selection within SutraERP L5.

It prevents:

- preference-driven technology selection;
- conflicting architecture decisions;
- unnecessary technology proliferation;
- premature vendor lock-in;
- hidden material decisions;
- ADR inflation;
- ADRs being used to override upstream architecture.

The decision sequence is:

```text
Approved behavior / technical need
        ↓
Authoritative constraints
        ↓
Quality attributes
        ↓
Decision scope
        ↓
Viable alternatives
        ↓
Evidence-based evaluation
        ↓
ADR where required
        ↓
Approved architecture decision
        ↓
L5 specification / technical standard
        ↓
Implementation
```

An ADR records **why a material architecture decision was made**.

The applicable L5 specification or technical standard defines **the resulting architecture that must be followed**.

An ADR does not create business authority.

### 1.1 Architecture Specificity While Technology Selection Is Pending

Technology selection may remain pending while ordinary solution architecture is defined.

```text
Technology-neutral
        ≠
Architecture-neutral
```

Before a product, framework or platform is selected, L5 may and shall where necessary define product-independent architectural structures and mechanisms such as:

- logical components, modules, services and workers;
- adapters and technical control points;
- request/execution/job/message contexts;
- API/contract responsibilities;
- repositories and persistence boundaries;
- transaction/consistency/concurrency mechanisms at architecture level;
- event publishers/consumers and durable-delivery roles;
- Policy Decision Points and Policy Enforcement Points;
- trust boundaries and execution sandboxes;
- cache, index and projection boundaries;
- runtime and deployment units.

A pending programming-language, framework, database, broker, cloud or orchestration decision shall not be used to defer technical architecture that can be defined independently of that selection.

Conversely, where the architectural pattern itself establishes a reusable cross-specification standard, material trust boundary, persistence strategy, interaction strategy or another mandatory ADR trigger, that pattern shall still follow ADR governance even if no named technology has yet been selected.

---

## 2. Local `AD-*` Decision vs ADR

Not every architecture decision requires a repository ADR.

### Local L5 Decision

A decision may remain only as a local `AD-*` when all of the following are true:

- it remains contained within one L5 specification;
- it does not establish a reusable standard;
- it does not materially affect another L5 owner;
- it does not materially affect security or trust boundaries;
- it does not materially constrain deployment or portability;
- it introduces no significant technology or vendor lock-in;
- reversal would remain reasonably contained;
- no mandatory ADR trigger applies.

Example:

```text
AD-003 — Use an internal adapter between two components
owned by the same L5 specification.
```

### ADR

A decision requires an ADR when its consequences are materially architectural even if it affects only one specification.

Therefore:

```text
One specification
≠
Automatically local decision
```

A Finance Computation Engine security-isolation decision, for example, may still require an ADR because of its trust and security consequences.

---

## 3. Mandatory ADR Triggers

An ADR is mandatory for a material decision that does one or more of the following.

### Technology & Platform

- selects a primary programming language or runtime;
- selects a major framework;
- selects database/persistence technology;
- selects messaging/broker technology;
- selects search/index technology;
- selects object-storage technology;
- selects workflow/orchestration technology;
- selects major security/cryptographic technology;
- selects a cloud-specific managed service;
- selects container/orchestration technology;
- establishes a shared client platform/framework;
- selects a material commercial/vendor platform.

### Architecture Structure

- establishes a reusable cross-specification pattern;
- creates a major shared runtime/platform dependency;
- materially changes module/service boundaries;
- establishes persistence-isolation or consistency strategy;
- establishes synchronous/asynchronous interaction strategy;
- establishes a major runtime/deployment pattern.

### Security & Trust

- creates or changes a material trust boundary;
- changes secure-engine isolation;
- materially changes identity/authentication/authorization architecture;
- creates privileged cross-boundary access;
- materially changes cryptographic/key/secrets architecture.

### Operations & Portability

- materially affects on-premises support;
- materially affects cloud portability;
- creates significant operational burden;
- establishes repository-wide resilience/recovery strategy;
- introduces material vendor or infrastructure lock-in.

### Repository Impact

- materially affects multiple L5 specifications;
- establishes a precedent expected to be reused;
- supersedes an approved architecture decision;
- materially changes an approved technical standard.

Where material impact exists and classification remains uncertain, escalate for architecture review rather than silently treating the decision as local.

Do not create ADRs for trivial implementation details.

---

## 4. ADR Scope & Decision Precedence

Every ADR shall declare its scope.

### Scope A — Specification ADR

Primarily applies to one L5 specification.

### Scope B — Shared / Family ADR

Applies across multiple L5 specifications within a family or defined shared capability.

### Scope C — Repository ADR

Establishes a SutraERP-wide or cross-family technical decision.

Examples may include:

- primary runtime/language;
- primary persistence technology;
- canonical deployment packaging approach;
- repository-wide client technical platform.

### Technical Decision Precedence

Within L5 technical architecture:

```text
Locked Repository / Higher-Level Authority
        ↓
Locked L5 Governance
        ↓
Repository ADR
        ↓
Shared / Family ADR
        ↓
Specification ADR
        ↓
Local AD-* Decision
        ↓
Implementation Detail
```

A lower-scope decision shall not contradict a higher-scope decision.

Where a legitimate specialization requires deviation, use the governed exception mechanism.

Technical decision precedence never overrides the repository's higher architecture authority hierarchy.

---

## 5. ADR Identity, Register & Lifecycle

### Canonical ADR Identifier

```text
ADR-L5-NNNN
```

Example:

```text
ADR-L5-0001
```

ADR IDs are repository-wide.

They shall not encode:

- author;
- family;
- technology;
- vendor;
- selected outcome.

### Central ADR Register

ADR identifiers shall be allocated through one canonical L5 ADR Register.

Authors shall not independently invent the next ADR number.

The register shall track at minimum:

- ADR ID;
- title;
- scope;
- decision owner;
- status;
- affected L5 specifications;
- supersession state.

Once allocated, an ADR identifier shall never be reused.

Rejected, retired and superseded ADR identifiers remain historical.

### Filename

Recommended:

```text
ADR-L5-NNNN_<Decision_Slug>.md
```

### Required ADR Metadata

**ADR ID:**  
**Title:**  
**Scope:** Specification / Shared / Repository  
**Decision Owner:**  
**Assigned Author:**  
**Reviewers:**  
**Status:**  
**Decision Date:**  
**Affected L5 Specifications:**  
**Related ADRs:**  
**Supersedes:**  
**Superseded By:**  

### Lifecycle

```text
Proposed
   ↓
Under Evaluation
   ↓
Architecture Approved
   ↓
Locked
```

Additional terminal states:

```text
Rejected
Superseded
Retired
```

An author shall not approve or lock their own ADR unilaterally.

Historical ADRs shall not be deleted merely because the decision changed.

---

## 6. Technology Selection Method

### Step 1 — Define the Decision

State:

- what must be decided;
- why a decision is required;
- canonical L5 owner;
- affected specifications;
- consequence of deferring the decision.

### Step 2 — Establish Constraints

Identify authoritative constraints before comparing technology.

Examples:

- approved L4 behavior;
- L5 ownership;
- Tenant/Institution architecture;
- security/privacy requirements;
- high-integrity boundaries;
- on-premises requirements;
- cloud/provider portability;
- interoperability;
- existing ADRs.

A preference is not a constraint.

### Step 3 — Establish Evaluation Criteria

Criteria shall be established before final candidate evaluation.

Possible criteria include:

- architectural fit;
- upstream compatibility;
- security/privacy;
- on-premises operability;
- portability;
- provider neutrality;
- resilience;
- performance/capacity;
- scalability;
- maintainability;
- operational complexity;
- observability;
- testability;
- interoperability;
- ecosystem maturity;
- supportability;
- licensing;
- cost implications;
- skills/delivery burden;
- migration complexity;
- replacement difficulty;
- vendor lock-in.

Existing team familiarity may influence delivery-risk evaluation.

It shall not by itself override a materially superior architecture or mandatory constraint.

### Step 4 — Identify Viable Alternatives

Consider genuine alternatives where they exist.

Include:

```text
No new technology / current approved approach
```

where legitimate.

Do not fabricate weak alternatives merely to make the preferred choice appear superior.

### Step 5 — Apply Mandatory Gates

Any candidate violating a mandatory constraint is non-viable.

Example:

```text
Cloud-only service
        +
Mandatory on-premises support
        ↓
Non-viable candidate
```

Comparative scoring cannot override a failed mandatory gate.

### Step 6 — Evaluate Evidence

Use evidence proportional to decision importance.

Evidence may include:

- architecture analysis;
- official technical documentation;
- security analysis;
- licensing analysis;
- operational analysis;
- interoperability assessment;
- benchmark;
- controlled proof-of-concept.

Distinguish:

- established fact;
- measured evidence;
- inference;
- assumption.

### Step 7 — Decide

Record:

- selected option;
- rationale;
- meaningful disadvantages;
- rejected viable alternatives;
- consequences;
- residual risks;
- migration/exit implications.

### Step 8 — Propagate

After approval:

- update affected L5 specifications;
- update `AD-*` references;
- update technical standards where necessary;
- update the ADR register;
- conduct dependency/change impact assessment.

---

## 7. Evaluation & Evidence Discipline

Technology selection uses two layers.

### Mandatory Gates

Use PASS / FAIL against applicable mandatory constraints.

A failed mandatory constraint eliminates the candidate unless the governing constraint itself is formally changed.

### Comparative Evaluation

For viable alternatives, use:

```text
Strong
Acceptable
Weak
```

or a justified weighted model where useful.

If numeric weighting is used:

- criteria and weights shall be established before final scoring;
- weights must be justified;
- false precision shall be avoided;
- architecture judgment remains authoritative.

The numerically highest candidate does not automatically win.

### Benchmarks & Proof-of-Concepts

A benchmark or PoC should be used only where material uncertainty cannot be resolved sufficiently through architecture analysis and authoritative evidence.

Before executing it, define:

- question being tested;
- success criteria;
- relevant workload/conditions;
- comparison method.

Results shall not be selectively interpreted after the fact to favor a preferred technology.

PoC code is evidence, not canonical architecture authority.

---

## 8. ADR Minimum Content

Every ADR shall contain:

1. **Decision Context**
2. **Scope & Affected Architecture**
3. **Authoritative Constraints**
4. **Decision Criteria**
5. **Alternatives Considered**
6. **Evaluation**
7. **Decision**
8. **Consequences**
9. **Risks & Mitigations**
10. **Migration / Exit Position**, where material
11. **Traceability**
12. **Approval Record**

An ADR should remain concise.

Detailed research or PoC evidence may be retained separately and referenced.

Do not convert an ADR into a full technical specification.

---

## 9. Decision Authority & Exceptions

### Specification ADR

Requires review by:

- applicable technical architecture owner;
- independent architecture reviewer/approver.

### Shared / Family ADR

Requires review by:

- materially affected canonical L5 owners;
- architecture reviewer/approver.

A technical family cannot silently impose a decision on another canonical owner.

### Repository ADR

Requires Lead Architect approval.

Repository-wide decisions shall not be established merely by repeated local adoption.

### Material Exceptions

A material exception to an approved ADR shall not be embedded silently inside an L5 specification.

The exception shall:

1. reference the governing ADR;
2. identify exact exception scope;
3. explain why the governing decision is insufficient;
4. analyze security, portability, dependency and operational consequences;
5. receive architecture approval;
6. remain traceable to affected L5 specifications.

A material exception to a Locked Shared or Repository ADR should normally be governed through a new scope-limited ADR.

Repeated exceptions shall trigger review of the governing ADR itself.

---

## 10. ADR vs Technical Standard

An ADR answers:

```text
Why did SutraERP choose this architecture?
```

A technical standard or L5 specification answers:

```text
What architecture must implementations conform to?
```

Example:

```text
ADR:
Why PostgreSQL was selected.

L5 technical standard/specification:
Approved persistence patterns, boundaries,
usage constraints and conformance requirements.
```

An ADR shall not become the only source describing a major reusable technical architecture.

Where a Repository or Shared ADR establishes a reusable standard, the resulting rule shall be propagated into its canonical L5 owner or technical-standard artifact.

---

## 11. Supersession & Re-Evaluation

Locked ADRs may change only through explicit governance.

Use:

```text
Current ADR
   ↓
Re-evaluation / impact analysis
   ↓
New ADR
   ↓
Current ADR → Superseded
```

Never rewrite historical decision evidence to make a replacement appear to have always been the architecture.

### Re-Evaluation Triggers

Impact review is required when materially relevant:

- upstream architecture changes;
- affected L5 architecture changes;
- security posture materially changes;
- product reaches deprecation/end-of-support;
- licensing materially changes;
- portability assumptions change;
- operational evidence contradicts original assumptions;
- major scale/performance assumptions change;
- repeated approved exceptions accumulate.

Re-evaluation does not automatically mean replacement.

It means the decision's continuing validity must be assessed.

Materially affected approved L5 specifications shall return to `Review` where required by L5 governance.

---

## 12. Historical Technology & Governance Effect

Historical SutraERP technology-stack, persistence baselines, prototypes and code may be used as:

- candidate input;
- feasibility evidence;
- historical context.

They have no automatic authority under this methodology.

Any historical technology proposed for canonical adoption shall pass the same decision process as any new alternative.

Locking this methodology:

- authorizes governed ADR creation;
- authorizes governed technology evaluation;
- establishes ADR identity/lifecycle rules;
- does **not** select any technology;
- does **not** approve the historical stack;
- does **not** authorize implementation;
- does **not** authorize broad L5 authoring;
- does **not** lock the final L5 corpus.

Technology authority exists only through approved ADRs, canonical L5 specifications, approved technical standards, or other explicit repository decisions.

---

**Status:** Locked  
**Authority:** Lead Architect — L5 Technology Selection & ADR Governance Baseline  
**Technology Stack Selected:** No  
**Broad L5 Authoring:** Not Authorized  
**Final L5 Corpus Lock:** Not Authorized
