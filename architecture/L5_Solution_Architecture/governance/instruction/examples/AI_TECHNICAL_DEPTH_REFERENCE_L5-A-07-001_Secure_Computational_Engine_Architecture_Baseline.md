# SutraERP L5 — AI Technical-Depth Reference Example

## Secure Computational Engine Architecture Baseline

> **Reference status:** Training / authoring-depth benchmark only.  
> **Modelled on canonical specification:** `L5-A-07-001` — Secure Computational Engine Architecture Baseline.  
> **Part of frozen 105-specification production corpus:** No.  
> **Purpose:** Show human authors and AI authoring agents the minimum expected level of technical specificity, evidence discipline and upstream traceability for SutraERP L5 mass authoring.  
> **Important:** The real production `L5-A-07-001` must be authored from the current authoritative repository sources under `architecture/L5_Solution_Architecture/specifications/L5-A/`.

---

# AI Authoring Directive — Mandatory Technical Depth, Zero Guessing and Exact Upstream Referencing

This section is a mandatory authoring guardrail for AI-generated SutraERP L5 specifications.

The AI is not acting as a brainstorming assistant when mass-authoring L5. It is acting as a **controlled architecture author operating inside a governed repository**.

The primary rule is:

```text
SUPPORTED ARCHITECTURE
        ↓
AUTHOR IT TECHNICALLY
        ↓
REFERENCE THE EXACT AUTHORITATIVE SOURCE

UNSUPPORTED / AMBIGUOUS / CONFLICTING ARCHITECTURE
        ↓
DO NOT GUESS
DO NOT COMPLETE BY PREFERENCE
DO NOT INVENT A FRAMEWORK OR NUMBER
        ↓
MARK ASSUMPTION / OPEN DECISION / ADR / UPSTREAM BLOCK
```

A document that looks complete because the AI silently filled missing architecture is **worse than an explicitly incomplete Draft**.

A document that contains technically plausible architecture but cannot show where its material obligations came from is also incomplete.

The AI must produce architecture at approximately the technical depth of this reference or deeper where the owning responsibility requires it, while remaining strictly within the canonical responsibility of the specification.

---

# A. Mandatory Source-Control Guardrail

Before authoring any production L5 specification, the AI shall establish the current authoritative source set from the repository.

At minimum it shall use, as applicable:

1. locked repository decisions and repository governance;
2. locked L5 Governance and Architecture Standard;
3. locked L5 Solution Architecture Specification Template;
4. locked L5 Validation Standard;
5. approved/frozen 105-specification corpus register;
6. workload/allocation decision;
7. dependency-gated authoring execution timeline;
8. naming and granularity specialization;
9. architecture family register;
10. `L5_L4_to_L5_Traceability_Methodology.md`;
11. technology-selection and ADR methodology;
12. ADR register and approved ADRs relevant to the specification;
13. applicable approved L4 System Behavior Specifications and exact BH obligations;
14. L1-L3 only for authoritative verification of ownership, terminology, rules, lifecycle semantics, event ownership and authority;
15. collaborating L5 specifications only when they are sufficiently stable to be consumed under the dependency gate.

The AI shall not treat the following as authority when they conflict with the canonical repository:

- model memory;
- a previous AI answer;
- an earlier chat assumption;
- historical code;
- historical L5 extraction documents;
- an author's personal technology preference;
- generic software-industry practice;
- a framework convention;
- an inferred architecture pattern that has not been justified by the repository.

If a required source is unavailable, unapproved, contradictory or too incomplete to support the technical decision, the AI shall **not fabricate the missing basis**.

---

# A.1 Mandatory Upstream Reference Protocol

Every production L5 specification shall show exactly which upstream material justifies its architecture.

The primary behavioral traceability unit is the approved **L4 BH**.

Canonical mapping concept:

```text
L4 Specification + exact BH
        ↓
Material technical need
        ↓
Canonical L5 technical owner
        ↓
L5 Architecture Section / AD-* / CR-*
```

The AI shall not write vague source statements such as:

```text
Based on L4 requirements.
Based on upstream architecture.
Representative calculation behavior applies.
According to existing business rules.
```

unless those statements are immediately resolved to exact canonical references.

For every materially applicable upstream source, record enough information to identify it unambiguously.

Minimum production reference format:

```markdown
| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-X-NNN` | `<version / status>` | `L4-X-NNN-BH-00Y` | `<approved observable behavior>` | `<specific technical contribution required from this L5 spec>` |
| L3 | `<canonical L3 ID>` | `<version / status>` | `<scenario section / exact item>` | `<ownership/outcome verification only>` | `<constraint verification; not new L5 behavior>` |
| L2 | `<canonical L2 source>` | `<version / status>` | `<workflow/state/event/rule reference>` | `<canonical workflow/state/event/rule>` | `<preserved upstream meaning>` |
| L1 | `<canonical L1 source>` | `<version / status>` | `<domain/ownership reference>` | `<canonical owner/entity responsibility>` | `<ownership verification>` |
| ADR | `<canonical ADR ID>` | `<status>` | `<decision section>` | `<approved technical choice>` | `<L5 architecture constraint>` |
| L5 Peer | `<L5-X-XX-XXX>` | `<version / status>` | `<section / AD / CR if applicable>` | `<peer-owned technical capability>` | `<capability consumed; not redefined>` |
```

## Upstream-reference rules

1. **L4 is the primary behavioral authority for L5.** Cite exact applicable L4 specification IDs and BH IDs wherever material behavior drives technical architecture.
2. **Do not cite only a document title when an exact BH exists.** Use the atomic BH reference.
3. **L1-L3 are verification sources, not a shortcut around missing L4 behavior.** Use them to verify canonical ownership, terminology, lifecycle semantics, business rules, events, publishers, authority and outcomes.
4. **If L4 is missing required behavior, do not manufacture it from L1-L3.** Record an upstream blocker.
5. **If an L4 source is Draft/unapproved, identify that status explicitly.** It may support controlled planning but must not be represented as approved behavioral authority.
6. **If two upstream sources conflict, cite both exact sources and mark the affected architecture blocked or impacted according to governance.** Do not silently choose one unless repository authority precedence resolves the conflict unambiguously.
7. **Every material `AD-*` shall have a traceable basis.** The basis may be an upstream BH, repository-wide technical constraint, approved ADR, canonical L5 ownership need or defensible technical derivation.
8. **Every material `CR-*` shall derive from established L5 architecture and should identify the architecture section / AD / upstream relation it enforces.**
9. **Peer L5 references must use exact canonical specification IDs where available.** Avoid wildcard-only references in final production documents when a specific dependency is known.
10. **Do not fabricate versions, statuses, paths, section numbers or BH IDs.** Verify them from the live repository.
11. **Do not copy large upstream passages into L5.** Reference and summarize the exact obligation; L5 should define technical realization, not duplicate upstream content.
12. **Traceability does not transfer ownership.** A mapped L5 specification contributes technical realization to an L4 BH; it does not become owner of that BH.

The canonical relation identity follows the locked traceability methodology:

```text
(L4 BH ID, L5 Specification ID)
```

A production L5 document should contain selective traceability sufficient to explain its applicability. Complete corpus coverage remains governed by the canonical repository traceability source.

---

# B. Evidence Classification — No Hidden Inference

Every material architecture statement generated by AI must be classified before it is written.

```text
1. ESTABLISHED REPOSITORY FACT
   Explicitly supported by authoritative source.

2. DEFENSIBLE L5 TECHNICAL DERIVATION
   Not copied verbatim from upstream, but technically necessary
   to realize approved behavior inside this specification's ownership.

3. ASSUMPTION
   Plausible but not yet authoritative.

4. OPEN DECISION
   Architecture cannot be completed safely without a decision.

5. ADR-REQUIRED DECISION
   Material choice needs governed architecture/technology selection.

6. UPSTREAM BLOCK / CONFLICT
   Source behavior or ownership is missing, ambiguous or contradictory.
```

Rules:

- Category 1 may be stated as fact and must be traceable to the authoritative source.
- Category 2 may become L5 architecture only when the derivation is necessary, within canonical L5 ownership, traceable and non-contradictory.
- Category 3 must be recorded explicitly as an `AS-*` assumption with impact and resolution path where material.
- Category 4 must be recorded as an `OD-*` item.
- Category 5 must remain unresolved until governed ADR handling occurs.
- Category 6 must block the affected architecture rather than being silently repaired.

**The AI shall never convert Category 3, 4, 5 or 6 into Category 1 merely to finish a document.**

---

# C. Absolute No-Guessing Rules

The AI shall not invent, assume or silently choose any of the following unless explicitly supported by authoritative architecture or justified through the approved L5/ADR process.

## C.1 No invented business architecture

Do not invent or change:

- business domains;
- canonical owners;
- business rules;
- business entities or facts;
- lifecycle states or transitions;
- canonical business events;
- canonical event publishers;
- institutional authority;
- approval authority;
- L3 outcomes;
- L4 Behavioral Obligations.

If L4 behavior is materially missing or ambiguous:

```text
STOP AFFECTED DESIGN
        ↓
Record exact missing/ambiguous upstream reference
        ↓
Record upstream issue / blocker
        ↓
Do not derive new required behavior directly from L1-L3
```

## C.2 No invented L5 ownership

Do not invent:

- new L5 areas;
- new production specification IDs;
- renamed IDs;
- merged or split specifications;
- duplicate technical owners;
- new cross-L5 responsibilities merely because a design pattern seems convenient.

Use only the canonical 105-specification register and approved governance decisions.

## C.3 No guessed frameworks, products or vendors

Unless an approved ADR or technical standard explicitly selects them, the AI shall not choose or imply a choice of:

- programming language;
- application framework;
- ORM;
- database product;
- message broker;
- cache product;
- search product;
- workflow product;
- identity provider;
- secrets platform;
- cloud provider;
- container platform;
- Kubernetes;
- service mesh;
- observability vendor;
- CI/CD product;
- testing framework;
- commercial SaaS;
- operating-system-specific mechanism.

Forbidden unsupported completion includes statements such as:

```text
Use PostgreSQL for this store.
Use Kafka for event delivery.
Use Redis for caching.
Deploy each engine as a Kubernetes pod.
Implement the API in Spring Boot / .NET / Node.js.
Use AWS KMS for key management.
```

If architecture establishes only a need, describe the product-independent mechanism and record any material product choice as unresolved/ADR-required.

## C.4 No guessed numeric targets

Never invent:

- latency targets;
- throughput targets;
- concurrency numbers;
- retry counts;
- timeout durations;
- cache TTLs;
- retention periods;
- RPO/RTO values;
- availability percentages;
- batch sizes;
- queue depths;
- CPU/memory limits;
- password/credential durations;
- cryptographic key rotation periods;
- rate limits;
- scaling thresholds.

If a number is architecturally required but not authoritative, use an `OD-*`, `AS-*` or governed decision rather than fabricating a value.

## C.5 No guessed technical lifecycle

AI may define technical execution states only when a state model is necessary to realize the owned technical responsibility.

Any introduced technical state must:

- be clearly technical, not business-semantic;
- belong to the current L5 owner;
- be justified by execution/recovery/concurrency needs;
- not contradict upstream lifecycle states;
- identify the upstream lifecycle references checked for non-conflict where lifecycle proximity is material.

If state names or transitions are not justified, do not invent them.

## C.6 No guessed contract fields

Conceptual L5 contracts may be defined only to the extent required to establish architecture boundaries.

Every material field or information category must be supported by approved upstream behavior, canonical L5 responsibility, an approved cross-L5 dependency or a necessary technical derivation.

Do not invent arbitrary fields because they are common in real systems.

Do not define programming-language types, framework annotations, serialization syntax, exact REST/GraphQL/gRPC wire definitions, physical database columns or full JSON schemas unless governed L5 architecture explicitly requires that representation.

When exact contract shape belongs to another L5 owner, state the required semantic information and reference that owner by exact canonical specification ID.

## C.7 No guessed failure policy

The AI must describe material failure behavior, but shall not invent exact operational policy.

It may establish:

```text
failure must not create an authoritative outcome
retry must not duplicate protected business effects
resource exhaustion must remain contained
```

but shall not invent exact retry counts, delays, backoff factors or dead-letter thresholds unless supported by authoritative technical decisions.

## C.8 No guessed security mechanism

The AI shall define required trust properties and enforcement points where owned, but shall not invent the exact security product or cryptographic design.

Do not choose algorithms, providers, key sizes, HSM products, certificate topology or identity technologies without authoritative support or ADR governance.

## C.9 No guessed deployment topology

Do not assume:

- microservices;
- monolith;
- separate physical servers;
- one container per component;
- Kubernetes;
- cloud-only operation;
- provider-specific managed services;
- multi-region deployment;
- active-active topology.

Define logical runtime/deployment boundaries only where justified by exact repository constraints and the owning L5 responsibility.

## C.10 No guessed event semantics

Do not create, rename or republish business events at L5.

Where messaging is needed, distinguish:

```text
Business event meaning / publisher
→ exact upstream canonical source

Technical publication, delivery, ordering, retry, schema compatibility
→ applicable L5 technical owner
```

A technical message must not silently become a new canonical business event.

---

# D. Conflict and Missing-Information Protocol

If authoritative sources disagree, the AI shall not average them, merge them, select the more convenient version or invent a compromise.

```text
Identify exact conflicting source A
        +
Identify exact conflicting source B
        ↓
State the conflicting propositions
        ↓
Identify affected L5 section / AD / CR
        ↓
Apply repository authority precedence only when unambiguous
        OR
mark architecture BLOCKED
        ↓
Do not silently continue through the conflict
```

If a required dependency is not sufficiently stable:

```text
DEPENDENCY NOT READY
        ↓
Reference exact dependency specification / decision
        ↓
Do not design that dependency internally
        ↓
Define only the consuming requirement already supported
        ↓
Record OD / dependency blocker
```

A Draft may remain incomplete. Unsupported architecture may not be disguised as complete.

---

# E. Technical-Depth Guardrail — Concrete Architecture, Not Generic Prose

Technology neutrality shall never be used as an excuse for architectural vagueness.

Where applicable and supported, the AI may identify concrete architecture-level elements such as logical components, services, workers, adapters, repositories, execution contexts, policy decision/enforcement points, trust boundaries, transaction boundaries, durable publication handoffs, logical messaging mechanisms, schedulers, caches, projections, indexes, document/object stores, runtime units, deployment boundaries, observability signals and recovery boundaries.

Every proposed element must pass this test:

```text
Is this element necessary to make this specification's
canonical technical responsibility implementable?

AND

Can its necessity be traced to approved behavior,
a repository constraint, canonical L5 ownership,
an approved ADR or a defensible technical derivation?

YES → define it precisely.
NO / UNSUPPORTED → do not add it.
```

The AI must not generate architecture by mechanically inserting enterprise patterns into every file.

---

# F. Responsibility-Specific Minimum Technical Views

The following are depth guardrails, not permission to invent unsupported detail.

## API / Contract architecture

Where applicable, address technical entry boundary, request/response responsibility, scope/context propagation, validation boundary, error/failure semantics, idempotency responsibility, contract versioning/compatibility ownership, authentication/authorization handoff and correlation.

Do not invent exact endpoint URLs, HTTP codes, field schemas or framework contracts unless governed.

## Persistence architecture

Where applicable, address canonical technical persistence owner, repository/store boundary, transaction boundary, consistency model, concurrency-control responsibility, cross-owner write prohibition, schema-evolution responsibility, migration safety and state-to-event handoff.

Do not invent tables, columns or product-specific SQL unless governed.

## Event / Messaging architecture

Where applicable, address canonical event ownership preservation, publication handoff, durable-delivery responsibility, ordering scope, duplicate/idempotency boundary, consumer-failure containment, schema/version compatibility and observability/evidence.

Do not invent broker products, retry counts or new business events.

## IAM / Authorization architecture

Where applicable, address identity/session/credential boundary, authorization context, decision point, enforcement point, scope/resource context, privileged/delegated authority boundary, fail-closed behavior and audit/evidence handoff.

Do not redefine Person, Role, Permission or business authority.

## Secure computation architecture

Where applicable, address rule-package acceptance/validation, version binding, normalized inputs, execution admission, isolated runtime boundary, deterministic evaluation constraints, resource containment, result/evidence production, failure containment and separation from authoritative business outcome.

Do not let the engine own Finance, Examination, Payroll or other business rules/outcomes.

## Runtime / Deployment architecture

Where applicable, address logical runtime units, packaging boundary, configuration/secrets boundary, isolation expectations, on-premises support, provider-neutral cloud capability, upgrade/rollback responsibility and operational-readiness implications.

Do not assume Kubernetes, a cloud vendor or physical topology without governance.

## Verification architecture

Where applicable, address architecture-conformance verification responsibility, component/integration/contract/E2E boundaries, failure/resilience/security verification categories and evidence required to demonstrate conformance.

Do not invent product-specific test frameworks without approval.

---

# G. Mandatory Pre-Authoring AI Workframe

Before generating prose for a production L5 file, the AI shall complete this workframe:

```text
STEP 1 — IDENTIFY
Exact canonical L5 ID, title, family, area and assigned author.

STEP 2 — VERIFY OWNERSHIP
What does this specification own?
What must it not own?
Which peers own adjacent responsibilities?

STEP 3 — BUILD UPSTREAM REFERENCE MATRIX
List exact applicable L4 specifications and BH IDs.
Record their version/status.
List exact L1/L2/L3 references used only for ownership/rule/state/event/authority verification.
List applicable repository decisions and ADRs.
Do not proceed from vague references.

STEP 4 — VERIFY AUTHORITY
Which approved L4 BHs create each technical need?
Which higher-level facts are only being verified?
Are any material sources Draft/unapproved?

STEP 5 — VERIFY DEPENDENCIES
Which exact L5 specifications are required?
Are they stable enough to consume?
What cannot yet be assumed?

STEP 6 — EXTRACT CONSTRAINTS
List only repository-supported constraints.
Attach each material constraint to its source.
Separate assumptions.

STEP 7 — DESIGN OWNED TECHNICAL REALIZATION
Define necessary components/control points/contracts/flows/state/failure boundaries.
Do not design peer internals.

STEP 8 — DECISION GATE
For each material choice:
Established and sourced?
Derived and defensible?
Assumption?
OD?
ADR-required?
Upstream blocked?

STEP 9 — CONFORMANCE
Convert established architecture into objective CR-* requirements.
Trace CR-* to architecture / AD-* and relevant upstream basis.

STEP 10 — TRACEABILITY REVIEW
For every material architecture section / AD-*:
Can the source or defensible derivation be identified?
For every claimed L4 relation:
Does this L5 section actually provide the claimed technical contribution?

STEP 11 — SELF-REVIEW
Run hard-fail checks against L5 Validation Standard.

STEP 12 — STATUS
Keep Draft if any blocking dependency, ADR, conflict or upstream issue remains.
```

This workframe is mandatory for AI mass authoring.

---

# H. Mandatory AI Stop Conditions

The AI shall stop the affected portion of authoring and explicitly mark it unresolved when:

- canonical L5 ID or ownership cannot be verified;
- applicable L4 behavior or exact BH reference cannot be identified;
- a cited L4 source is materially unapproved and no governance authorization permits approval-level use;
- L1/L2/L3 imply a technical need that is absent from L4 and would require inventing behavior;
- two upstream sources conflict materially;
- two L5 specifications appear to own the same technical responsibility;
- a required peer architecture is not stable enough to consume;
- a material technology choice requires ADR resolution;
- a numeric requirement is necessary but unavailable;
- a proposed state risks redefining a business lifecycle;
- a proposed message risks becoming a new canonical business event;
- a contract field or data requirement has no defensible basis;
- implementation feasibility appears to require changing upstream architecture;
- a reference path, ID, version or status cannot be verified from the repository.

The AI shall not use “typically,” “usually,” “best practice,” “industry standard,” “commonly” or “recommended” as a substitute for repository authority when making a normative L5 decision.

---

# I. Mandatory Post-Generation AI Hard-Fail Review

Before presenting a generated production specification as complete, the AI must ask:

```text
1. Did I invent any business rule, state, event, owner or authority?
2. Did I invent any L5 responsibility or duplicate a peer owner?
3. Did I select a language/framework/database/broker/cloud/runtime without approval?
4. Did I invent a numeric target?
5. Did I invent contract fields merely because they are common?
6. Did I invent retry/timeout/backoff behavior?
7. Did I assume microservices, Kubernetes, cloud-only, or physical separation?
8. Did I introduce a technical state machine without architectural necessity?
9. Did I create or rename a business event?
10. Did I hide an assumption as a fact?
11. Did I hide an unresolved ADR/OD to make the document look complete?
12. Did I redesign another L5 owner's internals?
13. Does every material L4-driven obligation cite the exact L4 spec and BH?
14. Are L1-L3 references used only for proper verification rather than bypassing L4?
15. Did I fabricate any source ID, path, version, status, BH or section reference?
16. Can every material AD-* be traced to a source or defensible L5 derivation?
17. Can every material CR-* be traced to established architecture?
18. Is the primary technical flow concrete enough to implement?
19. Are material failure boundaries explicit?
20. Could implementation proceed without inventing a major mechanism owned by this specification?
21. Could any major section be moved unchanged into L4? If yes, is L5 depth insufficient?
```

Any “yes” to questions 1–12 or 15 is a correction/blocking signal unless explicitly supported by authority.

A “no” to questions 13–14 and 16–20 is a traceability or technical-depth failure where applicable.

The completed Draft must then be validated against the locked `L5_Validation_Standard.md`. A hard failure cannot be compensated for by strong prose or a high numerical score.

---

# J. Canonical Output Rule for AI

A production L5 file must distinguish clearly between:

```text
ESTABLISHED UPSTREAM FACT
        → cite exact source / BH / decision

DEFENSIBLE L5 TECHNICAL DERIVATION
        → explain technical necessity and ownership basis

ASSUMPTION
        → AS-* with validation/resolution path

LOCAL ARCHITECTURE DECISION
        → AD-* with rationale and traceability

IMPLEMENTATION-CONFORMANCE CONDITION
        → CR-* derived from established architecture

UNRESOLVED MATERIAL ISSUE
        → OD-* / dependency block

MATERIAL CROSS-REPOSITORY DECISION
        → ADR candidate / approved ADR reference
```

The AI shall never make an unresolved item disappear by converting it into polished prose.

---

# K. Technical Depth Benchmark

Before considering an AI-authored L5 Draft complete, the AI must be able to answer:

```text
Which exact upstream documents and BHs justify this architecture?
What technical elements exist?
What does each element own?
How do they interact?
What enters and leaves the boundary?
What state/evidence/context is carried?
Where is validation performed?
What is the primary execution path?
What happens under each material failure class?
Where are trust and isolation enforced?
How are persistence/consistency/async concerns handled or delegated?
Which responsibilities belong to exact peer L5 specifications?
Which architecture decisions are established and what supports them?
Which decisions still require ADR/governance resolution?
What CR-* requirements can later be verified objectively?
```

**Depth rule:** implementation should not need to invent a major architectural mechanism that properly belongs to this L5 responsibility.

**Evidence rule:** material architecture shall not exist without a defensible repository basis or explicitly recorded unresolved status.

**Boundary rule:** additional technical detail is valuable only when it remains inside L5 ownership. Technical verbosity that steals another specification's responsibility or defines L6/implementation detail is incorrect.

---

## 1. Metadata

**ID:** `L5-A-07-001`  
**Status:** Draft  
**Technology:** Neutral

**Primary Basis — Training Example**

For this non-canonical training example, the basis is summarized only to illustrate technical depth:

- approved L5 engine decomposition;
- representative calculation-related L4 behavior;
- higher-level ownership of Finance, Examination and Payroll outcomes.

> **Production rule:** the real `L5-A-07-001` must replace this summary with exact repository references: applicable L4 specification IDs + BH IDs + versions/statuses, exact higher-level sources used for ownership/rule/outcome verification, applicable locked decisions/ADRs and exact peer L5 dependencies. “Representative behavior” is not sufficient production traceability.

**Child Engines**

- Finance;
- Examination Result;
- Payroll;
- General Rules & Formula;
- Scheduling & Optimization.

**Peers — Training shorthand only**

- `L5-D-03-*`;
- `L5-D-04-*`;
- `L5-A-08-*`;
- `L5-E-04-*`;
- `L5-F-02-*`.

> **Production rule:** replace wildcard shorthand with exact canonical peer specification IDs wherever the actual dependency can be identified.

## 2. Owns

Common engine architecture:

- rule-package loading;
- version binding;
- deterministic execution;
- execution isolation;
- controlled inputs/outputs;
- resource limits;
- computation evidence;
- failure containment.

Does not own business rules, approval, ledger posting, result publication or payroll finalization.

## 3. Architecture

```text
Approved Rule Package
        ↓
Rule Package Validator
        ↓
Execution Request
        ↓
Isolated Computation Runtime
        ↓
Deterministic Evaluation
        ↓
Computation Result
        +
Execution Evidence
        ↓
Owning Business Domain
        ↓
Validation / Approval / Publication
```

The architecture intentionally separates computation from the business authority that decides whether a result becomes an authoritative institutional outcome.

The common baseline governs reusable computation behavior. Specialist engine documents refine only their engine-specific technical obligations and shall not redefine the shared controls established here.

## 4. Contracts

Conceptual computation request:

```text
ComputationRequest
{
  EngineType
  RulePackageVersion
  InputSet
  ScopeContext
  CorrelationReference
}
```

Conceptual computation result:

```text
ComputationResult
{
  RulePackageVersion
  OutputSet
  ExecutionReference
  EvidenceReference
}
```

These are architecture-level conceptual contracts, not programming-language classes, API wire formats or database schemas.

The engine output is **not automatically an authoritative business outcome**.

The owning business capability remains responsible for downstream validation, approval, posting, publication or finalization according to its canonical upstream architecture.

> **Production traceability rule:** every material semantic input/output category shall be checked against the actual upstream L4 obligations and canonical peer architecture. Do not keep an illustrative field in the production document merely because it appears in this example.

## 5. Trust Boundary

Engine execution shall prevent unauthorized:

- rule modification;
- external network access;
- file/system access;
- credential access;
- cross-engine state access;
- business-state mutation.

```text
Untrusted / External Inputs
          ↓
Validation Boundary
          ↓
Approved Execution Inputs
          ↓
┌───────────────────────────────┐
│ Isolated Computation Boundary │
│                               │
│ Rule Package                  │
│ Deterministic Evaluation      │
│ Resource Enforcement          │
│ Evidence Generation           │
└───────────────────────────────┘
          ↓
Controlled Computation Result
          ↓
Owning Business Capability
```

Exact sandbox/runtime mechanism remains a governed technical decision.

The baseline defines required security properties without prematurely selecting a particular container, process-isolation, virtual-machine, WebAssembly, operating-system sandbox or other runtime mechanism.

## 6. Determinism

Equivalent:

```text
Rule Version
+
Normalized Inputs
+
Approved Engine Semantics
```

shall produce reproducible computation within defined deterministic boundaries.

Time, randomness or external mutable data may not silently influence deterministic calculations.

Where time, randomness, reference data or other variable context is legitimately required, that input must become explicit, versioned or otherwise reproducible according to supported architecture.

## 7. Failure Model

```text
Invalid Rule Package
Invalid Input
Resource Limit
Execution Failure
Determinism Violation
        ↓
COMPUTATION FAILURE
        ↓
No approval
No posting
No publication
```

Failure containment requirements:

- an invalid or unacceptable rule package shall not execute;
- invalid required inputs shall fail before authoritative computation output is produced;
- resource exhaustion shall terminate or reject computation according to governed execution policy rather than escaping the engine boundary;
- a failed computation shall not be represented as a successful authoritative result;
- computation failure shall not independently mutate owning-domain canonical business state;
- evidence sufficient to identify failed execution shall be retained or handed off according to canonical audit/evidence architecture;
- retry behavior, when permitted, must not silently create duplicate authoritative business effects.

## 8. Decisions

| ID | Decision | Production Traceability Expectation |
|---|---|---|
| `AD-001` | Engines execute rules; domains own rules and business outcomes. | Cite exact upstream ownership/rule/outcome sources plus L5 engine responsibility. |
| `AD-002` | Every execution binds to an exact rule-package version. | Cite technical need from applicable behavior/engine governance; justify as L5 derivation if not explicit upstream. |
| `AD-003` | High-integrity computation runs inside an explicit trust boundary. | Cite exact security/high-integrity constraints and applicable D03 dependency. |
| `AD-004` | Computation output cannot directly approve, post or publish a protected business outcome. | Cite exact upstream owner/approval/publication authority. |
| `AD-005` | Execution produces reproducibility/audit evidence. | Cite applicable L4 evidence obligations and/or canonical audit architecture. |
| `AD-006` | Specialist engines remain technically separable under the common baseline. | Cite locked engine decomposition and relevant security/deployment rationale. |

Engine isolation/runtime technology requires ADR review.

These `AD-*` identifiers are local architecture decisions within this specification. They are not canonical repository ADR numbers.

## 9. Conformance

- `CR-001` — engine shall execute only an approved/accepted rule package.
- `CR-002` — execution shall retain the exact rule version used.
- `CR-003` — engine shall not become canonical owner of business rules.
- `CR-004` — engine shall not directly approve, post or publish protected business outcomes.
- `CR-005` — engine shall prevent uncontrolled external side effects.
- `CR-006` — computation failure shall not produce an authoritative outcome.
- `CR-007` — execution shall produce sufficient evidence for reproducibility.
- `CR-008` — specialist engine isolation shall preserve common security controls.

> **Production traceability rule:** each CR must point back to the established architecture section / AD-* and, where materially driven by L4, the exact upstream BH relation. A CR is not permitted to introduce a new business requirement.

## 10. Open Decisions

- rule-package format;
- sandbox/runtime technology;
- deterministic numeric/time semantics;
- resource-limit model;
- rule signing/integrity mechanism;
- engine deployment separation;
- ADR requirements.

Open decisions remain explicit until resolved by the proper L5 owner and governance mechanism. An AI author must never fill these gaps merely to make the document appear complete.

---

# AI Quality Gate for Mass Authoring

For every generated L5 specification, compare the Draft against this reference and the locked traceability methodology before submission.

A Draft shall fail the depth/guardrail check if it:

- repeats L4 behavior without technical realization;
- uses vague upstream references instead of exact source IDs/BHs;
- fabricates source IDs, statuses, versions, sections or BHs;
- derives missing L4 behavior directly from L1-L3;
- lists generic principles instead of concrete architecture;
- names technologies without governed selection;
- contains no concrete technical flow where runtime behavior exists;
- contains no failure model where failure is material;
- omits ownership boundaries;
- duplicates a peer L5 responsibility;
- provides `AD-*` decisions without corresponding architecture or traceable basis;
- provides `CR-*` requirements that are vague, unsupported or impossible to verify;
- hides unresolved architecture inside assumptions;
- invents states, events, contract fields, numeric targets, retry policies or deployment topology without support;
- uses framework conventions as architecture authority;
- leaves implementation to invent major mechanisms that properly belong to L5.

A Draft passes the technical-depth and evidence intent of this example only when it gives implementation a concrete architectural model **and** provides defensible upstream references for the material obligations and decisions it realizes.

---

**Reference Verdict:** TECHNICAL-DEPTH + ZERO-GUESSING + EXACT-UPSTREAM-TRACEABILITY BENCHMARK FOR L5 MASS AUTHORING  
**Canonical Production Status:** NON-CANONICAL TRAINING REFERENCE
