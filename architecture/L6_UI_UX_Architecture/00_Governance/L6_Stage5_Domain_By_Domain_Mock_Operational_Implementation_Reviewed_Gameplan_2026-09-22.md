# SutraERP L6 — Stage 5 Domain-by-Domain Mock-Operational Implementation — Reviewed Gameplan — 2026-09-22

**Version:** 1.2.0  
**Status:** Architecture Approved — Planning Baseline  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Stage:** Stage 5 — Domain-by-Domain Mock-Operational Implementation  
**Execution State:** PLAN APPROVED; BATCHES 0–10 COMPLETE; BATCH 11 IN EXECUTION; REMAINING-TOPOLOGY CONSOLIDATION DECISION ACTIVE  
**Depends On:** `../L6_Full_Workflow_Realization_and_Mock_ERP_Execution_Plan_DRAFT.md`; `L6_Stage3_Final_Validation_and_Closure_Decision_2026-09-21.md`; `L6_Stage4_Batch4_Full_Discovery_Coverage_Exit_Gate_Evidence_2026-09-22.md`; Stage-2 Group-15 master reconciliation; current live L5 architecture; current frontend Stage-4 canonical main.

---

## 1. Executive Decision

Stage 5 shall convert the discovery-complete SutraERP frontend into a deterministic mock-operational ERP without introducing backend services, persistence, production workflow execution, production authorization, or new business architecture.

The controlling principle is:

```text
existing canonical workflow authority
        ↓
one deterministic frontend demo-state authority
        ↓
small reusable mock-operation foundation
        ↓
bounded domain/workspace rollout batches
        ↓
cross-domain reconciliation
        ↓
full 595-workflow reconciliation
        ↓
Stage-5 closure
```

Stage 5 shall extend the Stage-3 state authority and Stage-4 discovery layer. It shall not replace either.

Stage 5 has two inseparable success dimensions:

```text
architecture / workflow fidelity
        +
coherent ordinary ERP product experience
```

A batch is not considered strategically successful merely because its assigned workflows have proof coverage. The accumulated frontend must progressively converge toward one browseable, stateful SutraERP product in which ordinary users encounter business terminology, business context, legitimate actions, outcomes and handoffs rather than programme/batch/proof/reviewer terminology.

Reviewer tooling, architecture provenance, deterministic scenario controls, failure injection and validation evidence remain required, but they are support instrumentation and shall be separated from ordinary ERP UX.

The Stage-5 implementation model is intentionally split into small batches that can be executed, validated, browser-tested, self-reviewed, evidenced, and reported in one execution conversation. Large batches that cannot reliably complete end-to-end in one execution chat must be split before work starts.

---

## 2. Live Repository Baseline

Verified Architecture baseline at planning approval:

```text
Architecture main:
35456eaaa58bf08c7c3bf8f17f8c1863f6c856af
```

Verified Stage-4 canonical frontend integration:

```text
Frontend main:
cc723c2379bd2d7085db2b3823708352bbe5dc96

Stage-4 implementation head:
eec69310c2078a2af7cd796439c6adff3cf86e8d

Frontend PR #21:
MERGED

Architecture PR #56:
MERGED

Final Stage-4 CI:
Frontend PR Validation #156
Run ID 35716209191
SUCCESS
```

Architecture main contains later L5 backend-readiness decisions after Stage-4 planning began. Stage 5 shall consume the live current L5 repository state for each affected batch and shall not freeze itself to an older Stage-4-era L5 snapshot.

Open Architecture PRs at plan creation: 0.

---

## 3. Protected Canonical Counts

The following are protected Stage-5 baselines:

```text
595 normalized workflows
1,084 canonical scenarios
99 workspaces
12 UX archetypes
10 mixed workflows
```

Workflow classification baseline:

```text
REVIEW-APPROVAL-WORKFLOW               271
INTERACTIVE-WORKFLOW                   154
EXCEPTION-RECOVERY-WORKFLOW            103
SYSTEM-BACKGROUND-OBSERVABLE             32
CROSS-WORKSPACE-HANDOFF                  28
TRACKING-WORKFLOW                         6
NO-DIRECT-END-USER-INTERACTION            1
TOTAL                                   595
```

Mock-disposition baseline:

```text
REQUIRED                                551
NOT-REQUIRED                             11
PROHIBITED                               33
TOTAL                                   595
```

These counts shall not be silently changed.

---

## 4. Domain Distribution

The Stage-2 master workflow catalog currently resolves to:

| Domain | Workflows | Mock REQUIRED | Workspaces |
|---|---:|---:|---:|
| A — Platform / Administration | 35 | 29 | 10 |
| B — Institution & Structure | 15 | 15 | 5 |
| C — Student Lifecycle | 32 | 25 | 8 |
| D — Academic Delivery | 38 | 36 | 7 |
| E — People & Workforce | 55 | 46 | 7 |
| F — Finance | 59 | 47 | 15 |
| G — Campus Operations | 154 | 152 | 11 |
| H — Institutional Services & Intelligence | 54 | 52 | 10 |
| I — Examination | 99 | 99 | 16 |
| J — Enterprise Resource Management | 54 | 50 | 10 |
| **Total** | **595** | **551** | **99** |

---

## 5. Workspace Maturity Baseline

Current frontend workspace implementation maturity remains:

```text
99 total workspaces

81 SHELL-IMPLEMENTED
18 INTERACTIVE-FIXTURE-IMPLEMENTED
```

The 18 existing interactive workspaces shall be audited and reused where architecturally valid.

Stage 5 shall not rewrite already-good interactions merely to force uniform implementation.

The 81 shell workspaces shall be upgraded through reusable archetype behavior and shared mock-operation primitives rather than separate local state islands.

---

## 6. Stage-5 Purpose

Stage 5 shall allow reviewers to experience legitimate work as:

```text
discover / enter
↓
inspect authoritative context
↓
express mock user intent
↓
observe intermediate state where legitimate
↓
observe confirmed / rejected / failed /
indeterminate / handoff outcome
↓
observe related presentation update
↓
inspect demo history / evidence / activity
```

The exact path varies by workflow.

Stage 5 does not create a universal workflow engine.

### Product-convergence rule

From consolidated Batch 12 onward, new realization work shall prefer reusable product-facing workspace composition and shared interaction patterns over new batch-specific proof surfaces where the existing foundations can express the required behavior safely.

Existing Batch 0–11 proof/reviewer surfaces remain valid evidence and shall not be rewritten merely for cosmetic uniformity. They may be progressively moved behind Review / Experience Lab boundaries when product-facing equivalents exist.

Ordinary ERP UX shall not expose, as user-facing product concepts:

- Stage / Batch / Chunk identities;
- workflow-reviewer IDs;
- proof-case IDs;
- architecture-level IDs or repository paths;
- "reviewer replay", "bounded review", "proof" or equivalent validation-language where a normal business term is available.

This rule does not prohibit reviewer-only tooling. It requires separation between the product surface and the validation surface.

---

## 7. Non-Goals

Stage 5 shall not create:

- backend APIs;
- production databases;
- production persistence;
- event-bus infrastructure;
- production workflow engines;
- production authorization;
- real background jobs;
- real integrations;
- deployment architecture;
- new business rules;
- new business events;
- new lifecycle states;
- new ownership.

Stage 5 shall not modify L0–L5 as part of ordinary implementation.

---

## 8. Existing Reusable Foundations

Stage 5 shall extend rather than replace:

- `GovernedAction`;
- `OwnerHandoff`;
- `WorkQueue`;
- the 12 UX archetypes;
- Stage-3 canonical institutional seed;
- Stage-3 shared selectors/projections;
- Stage-3 active demo state;
- baseline and exception-rich presets;
- deterministic mock clock;
- tasks/notifications/exceptions/activity;
- Experience Lab;
- Stage-4 workflow discovery;
- workspace-local workflow discovery;
- operational context discovery;
- existing browser evidence and CI validators.

Current `GovernedAction` already distinguishes:

```text
review/unavailable
pending
confirmed
rejected
technical failure
indeterminate
```

It is a presentation/outcome primitive and shall not become business-rule authority.

---

## 9. State and Mutation Architecture

Stage 5 shall preserve one active deterministic demo graph.

The approved model is:

```text
immutable canonical seed
        ↓
selected deterministic preset
        ↓
one active demo state
        ↓
controlled Stage-5 mock operation
        ↓
new deterministic active demo state
        ↓
derived projections + demo history
        ↓
deterministic reset
```

No second global state authority is permitted.

No per-workspace hidden databases or independent mutation stores are permitted.

All Stage-5 mutations shall remain frontend-local and non-authoritative.

A Stage-5 mock operation may consume:

- workflow identity;
- source-supported context;
- source-supported user intent;
- deterministic review outcome;
- affected demo projections;
- demo-history representation.

A mock operation shall never own:

- canonical business rules;
- eligibility logic;
- approval authority;
- canonical lifecycle validity;
- canonical event definitions;
- production authorization;
- production persistence.

One mock operation should produce one coherent next demo graph from the caller's perspective so related projections do not become inconsistent.

---

## 10. Outcome Model

Where applicable, Stage-5 interactions shall distinguish:

```text
pending
confirmed
rejected
technical failure
indeterminate
handoff
```

`request accepted` shall not be presented as `business operation completed` unless upstream architecture establishes that equivalence.

Indeterminate outcomes shall not encourage unsafe duplicate execution.

---

## 11. History / Activity Model

Stage 5 may record deterministic demo-operation history.

Demo-operation history is not canonical audit evidence.

It should be review-safe and may include:

- workflow reviewer identity;
- subject reference;
- intent label;
- deterministic mock timestamp;
- mock outcome class;
- affected presentation references.

Canonical business events and audit evidence remain upstream-owned.

---

## 12. Handoff Model

Cross-workspace handoff shall preserve the canonical owner.

Context may travel.

Responsibility may not silently transfer.

A source workspace may expose:

- initiation;
- evidence;
- handoff context;
- destination;
- acknowledgement/status;

but it shall not execute another owner's canonical operation.

---

## 13. Workflow-Class Treatment

### INTERACTIVE-WORKFLOW

May use MOCK-INTERACTIVE treatment when mock disposition permits it.

Classification alone does not define the action.

### REVIEW-APPROVAL-WORKFLOW

Must preserve:

- review context;
- decision authority;
- pending semantics;
- approved/rejected semantics;
- consequence visibility.

Persona visibility shall not manufacture approval authority.

### EXCEPTION-RECOVERY-WORKFLOW

No universal Retry / Resolve / Override control is allowed.

Recovery intent must be explicitly supported by upstream architecture.

### CROSS-WORKSPACE-HANDOFF

Target mode: HANDOFF.

Owner preservation is mandatory.

### SYSTEM-BACKGROUND-OBSERVABLE

Target mode: OBSERVABLE-NONINTERACTIVE.

No fake human action is allowed.

### TRACKING-WORKFLOW

Target mode: TRACKING.

Prefer status, timeline, evidence and progress.

### NO-DIRECT-END-USER-INTERACTION

Target mode: NO-DIRECT-INTERACTION.

The workflow remains non-actionable unless upstream authority says otherwise.

---

## 14. Mock-Disposition Treatment

### REQUIRED — 551 workflows

Every REQUIRED workflow shall receive legitimate Stage-5 treatment.

REQUIRED does not imply mutation. A tracking, background or handoff realization may be correctly non-mutating.

### NOT-REQUIRED — 11 workflows

Every NOT-REQUIRED workflow shall receive an explicit non-interactive Stage-5 disposition and validation evidence.

### PROHIBITED — 33 workflows

PROHIBITED workflows must have:

```text
Stage-5 mutation count = 0
```

Any PROHIBITED workflow receiving a mutation binding is a hard validation failure.

---

## 15. Mixed Workflows

The 10 mixed workflows shall preserve scenario-level overrides.

A workflow-wide interaction flag may not flatten scenario semantics.

The Stage-5 model shall permit:

```text
workflow shell
        ↓
scenario override
        ├── interactive
        ├── observation-only
        ├── handoff
        └── prohibited
```

Mixed-workflow override integrity is a hard validation gate.

---

## 16. Stage-5 Census

Batch 0 shall produce a machine-checkable census for all 595 workflows.

Each workflow shall contain at least:

```text
workflowReviewerId
label
domain
workspace
classification
mockDisposition
presentationRelevance
archetypes

scenarioIds
scenarioCount
scenarioOverrides
mixed

workspaceMaturity
existingInteractionCoverage
reusableComponents

targetMode

mockUserIntentRequired
stateMutationRequired
outcomeRequired
exceptionTreatmentRequired
historyRequired
handoffRequired

canonicalOwner
dependencies
upstreamSources

executionBatch
validationRequirement
openAmbiguity
```

The census shall be generated from canonical sources and Stage-5 planning metadata.

It shall not become a parallel business-architecture authority.

---

## 17. Batch Sizing Rule

Stage-5 batches are intentionally smaller than the earlier research hypothesis.

A normal implementation batch should target approximately:

```text
15–30 workflows
1–4 closely related workspaces
one coherent business boundary
```

Preferred ceiling:

```text
~30 workflows
```

Exceptional ceiling:

```text
~35 workflows only where implementation reuse makes the work demonstrably bounded
```

Every batch must be capable of completing in one execution conversation:

```text
reconstruct live state
→ implement
→ validate
→ browser evidence
→ accessibility review
→ cleanup
→ self-review
→ architecture evidence
→ report
→ STOP
```

If that is not realistic, the master chat must split the batch before issuing the execution prompt.

---

## 18. Stage-5 Batch Topology

### Batch 0 — Authority, Census & Baseline Freeze

Scope:

- 595 workflows;
- 1,084 scenarios;
- 99 workspaces;
- 12 archetypes;
- 10 mixed workflows.

Purpose:

- freeze source identities;
- generate Stage-5 census;
- classify current interaction coverage;
- identify all existing action surfaces;
- identify any existing prohibited-interaction defects;
- establish `stage5:validate` baseline.

No business implementation.

---

### Batch 1 — Mock-Operation Core Foundation

Purpose:

- controlled mock-operation contract;
- deterministic operation execution;
- result/outcome representation;
- atomic active-demo-state replacement;
- demo-history contract;
- reset compatibility;
- preset compatibility;
- accessibility foundation.

Use only a small representative proof set.

Do not create a generic workflow engine.

---

### Batch 2 — Golden-Slice Interaction Proof

Purpose:

Prove reusable mechanics across representative categories:

- ordinary interactive;
- review/approval;
- exception/recovery;
- handoff;
- tracking;
- background observable;
- guided flow;
- bulk/roster;
- financial consequence;
- high-consequence governed lifecycle action.

Golden slices prove mechanics only and do not automatically count as full domain completion.

---

### Batch 3 — Platform / Administration Part 1

Target: approximately 17 workflows.

Use a coherent workspace/dependency split determined from the live census at execution time.

---

### Batch 4 — Platform / Administration Part 2

Target: approximately 18 workflows.

Completion of Domain A.

---

### Batch 5 — Institution & Structure

```text
15 workflows
5 workspaces
```

---

### Batch 6 — Student Lifecycle Part 1

Target: approximately 19 workflows.

Reuse the established Student Workspace where appropriate.

---

### Batch 7 — Student Lifecycle Part 2

Target: approximately 13 workflows.

Completion of Domain C.

---

### Batch 8 — Academic Delivery Part 1

Target: approximately 19 workflows.

---

### Batch 9 — Academic Delivery Part 2

Target: approximately 19 workflows.

Completion of Domain D.

---

### Batch 10 — People & Workforce Part 1

Target: approximately 26 workflows.

---

### Batch 11 — People & Workforce Part 2

Target: approximately 29 workflows.

Completion of Domain E.

---

### Batches 12–20 — Consolidated Remaining Topology

The original Batch 12–30 topology below is superseded for future execution by:

`L6_Stage5_Remaining_Batch_Consolidation_Decision_2026-09-26.md`

The controlling remaining topology is:

| Batch | Scope |
|---|---|
| 12 | Finance — complete Domain F |
| 13 | Campus Operations — complete Domain G through mandatory internal chunks/workstreams |
| 14 | Institutional Services & Intelligence — complete Domain H |
| 15 | Examination — complete Domain I through mandatory high-consequence chunks |
| 16 | Enterprise Resource Management — complete Domain J |
| 17 | Product-Surface Convergence |
| 18 | Enterprise Cross-Domain Journey & Exception Convergence |
| 19 | Full Corpus Reconciliation |
| 20 | Final Stage-5 Validation, Cleanup & Stage-6 Readiness |

Batches 0–11 retain their historical/canonical identities.

The consolidation changes execution boundaries only. It does not change protected corpus counts, workflow/scenario dispositions, canonical ownership, workspaces, archetypes, business rules, lifecycle states, events or validation obligations.

Large consolidated domain batches shall be implemented through bounded chunks/workstreams and integrated before one domain closure decision. Conversation-size limits are not architecture boundaries.

---

## 19. Batch Entry Gate

Every implementation batch must begin only after:

```text
previous batch merged
Architecture evidence canonical
Frontend main synchronized
no blocking PR
current Architecture main reconstructed
current Frontend main reconstructed
Stage-5 census unchanged or governed difference explained
protected counts unchanged
affected live L5 sources rechecked
```

Prompts shall never assume historical SHAs remain current.

---

## 20. Batch Exit Gate

Every implementation batch shall prove, where applicable:

- assigned workflows accounted for;
- assigned workspaces accounted for;
- mock dispositions preserved;
- mixed overrides preserved;
- one state authority preserved;
- deterministic reset passes;
- preset switching passes;
- owner preservation passes;
- result/history semantics pass;
- Stage-3 validation passes;
- Stage-4 validation passes;
- Stage-5 batch validation passes;
- typecheck passes;
- build passes;
- dedicated browser evidence passes;
- accessibility review passes;
- Architecture evidence is synchronized;
- ordinary product UX remains business-facing rather than validation-facing;
- new batch-specific UI scaffolding is justified where reusable product composition cannot safely satisfy the workflow;
- representative user paths introduced or affected by the batch remain coherent outside Review / Experience Lab mode.

---

## 21. Stage-5 Validator

Stage 5 should add:

```text
npm run stage5:validate
```

The validator should eventually hard-fail on:

- workflow missing Stage-5 treatment;
- REQUIRED workflow without legitimate realization;
- PROHIBITED workflow with mutation;
- NOT-REQUIRED workflow with unexplained interactive behavior;
- mixed scenario override flattened;
- unknown workflow ID;
- unknown workspace;
- unknown archetype;
- duplicate operation binding;
- duplicate state authority;
- operation without outcome model where an outcome is required;
- operation without reset compatibility;
- owner-handoff mutating foreign owner;
- background workflow receiving fake human action;
- workflow-specific route added;
- second workspace registry;
- new archetype introduced without governance;
- reviewer metadata leaked into ordinary UX;
- nondeterministic timestamps in canonical demo behavior;
- preset mutation leakage;
- history referencing nonexistent operation;
- false INTEGRATED or PRODUCTION-VALIDATED maturity;
- Stage / Batch / Chunk / proof-case / reviewer terminology leaked into ordinary ERP UX;
- reviewer-only interaction required to perform a product-facing workflow;
- new batch-specific product shell introduced without a demonstrated reuse gap;
- product-facing workflow implemented only as a proof card when a normal workspace interaction is required by the realization disposition.

---

## 22. Browser Evidence

Every implementation batch shall include browser evidence for the actual behavior introduced.

Depending on scope:

- desktop;
- narrow/mobile;
- keyboard;
- relevant presentation profiles;
- baseline preset;
- exception-rich preset where material;
- confirmed path;
- pending path where legitimate;
- rejected path where legitimate;
- failure path where legitimate;
- indeterminate path where legitimate;
- handoff;
- observation-only behavior;
- empty state;
- state change;
- related projection update;
- history/activity update;
- reset.

Screenshots are visual evidence only.

They are not accessibility proof.

---

## 23. Accessibility Gate

Each batch shall explicitly review:

- semantic structure;
- form labels;
- descriptions;
- error association;
- keyboard flow;
- focus management;
- dialog focus;
- confirmation focus;
- status announcements;
- `aria-live` usage;
- disabled-state semantics;
- touch targets;
- responsive layout;
- non-color-only status meaning;
- high-density readability.

Accessibility is a hard gate.

---

## 24. Cross-Domain Consistency

The active Stage-3 graph remains the shared institutional source.

Important invariants include:

```text
Person ↔ Student Context
Student Context ↔ Attendance
Student Context ↔ Examination
Student Context ↔ Finance
Person ↔ Workforce ↔ Faculty ↔ Academic Assignment
```

Owning-domain mock fact mutation shall update dependent projections through the shared graph rather than duplicate copied facts.

---

## 25. Reset and Preset Integrity

Every Stage-5 mutation shall support:

```text
baseline
→ operation
→ changed demo presentation
→ reset
→ exact baseline restored
```

and where applicable:

```text
exception-rich
→ operation
→ changed demo presentation
→ reset
→ exact exception-rich preset restored
```

Switching presets shall not leak mutations between presets.

---

## 26. Maturity Boundaries

Maintain distinct maturity dimensions:

```text
SCENARIO-REALIZATION-MAPPED
WORKFLOW-DISCOVERABLE
MOCK-INTERACTIVE / OBSERVABLE-NONINTERACTIVE
UX-REVIEWED
INTEGRATION-PENDING / INTEGRATED
PRODUCTION-VALIDATED
```

Stage 5 may increase mock-interaction and UX-review maturity.

Stage 5 shall not falsely increase integration or production-validation maturity.

---

## 27. Architecture Evidence

Every implementation batch should create one bounded L6 evidence record containing:

- entry SHAs;
- exact scope;
- exact workflow/workspace population;
- source authority;
- operation/mutation model;
- outcome model;
- PROHIBITED proof;
- mixed-workflow proof where relevant;
- cross-domain proof;
- reset proof;
- browser evidence;
- accessibility review;
- CI;
- findings;
- final verdict.

No ordinary Stage-5 evidence record may modify L0–L5.

---

## 28. Merge Discipline

The permanent Stage-5 master-control process is:

```text
worker implements
↓
worker reports
↓
master independently reopens repository / PR / diff
↓
master independently checks CI and browser evidence
↓
master challenges implementation
↓
targeted fixes if required
↓
master revalidates
↓
merge recommendation only when evidence is complete
↓
explicit user merge instruction
```

Worker-reported PASS is not merge authority.

---

## 29. Risk Register

| Risk | Severity | Control |
|---|---|---|
| second demo-state authority | Critical | all mutations use existing active demo graph |
| frontend business-rule ownership | Critical | operations consume upstream semantics only |
| PROHIBITED fake interaction | Critical | validator hard fail |
| mixed-workflow override loss | Critical | scenario-level validation |
| Finance false completion | Critical | explicit pending/confirmed/indeterminate semantics |
| Examination result corruption | Critical | owner/state gates |
| cross-domain duplicate mutation | High | owner-only fact mutation + shared projections |
| preset state leakage | High | reconstructive reset/preset tests |
| workflow-engine creep | High | Foundation scope gate |
| oversized G/I execution | High | one governance batch per domain with mandatory bounded chunks/workstreams and integrated domain closure |
| shell-local fixture silos | High | reusable archetype operation primitives |
| reviewer metadata leakage | High | ordinary-UX validator |
| accessibility debt | High | batch browser/keyboard/accessibility gate |
| existing clickable demo over-counted | Medium | Stage-5-sufficient classification requires governed consequence |
| demo history mistaken for audit | Medium | separate demo-history semantics |

---

## 30. Stage-6 Readiness Criteria

Stage 5 may close only when:

```text
595 / 595 workflows Stage-5-dispositioned
551 / 551 REQUIRED legitimately realized
11 / 11 NOT-REQUIRED explicitly accounted for
33 / 33 PROHIBITED mutation-free
10 / 10 mixed workflows override-safe
1,084 / 1,084 scenario mappings intact
99 / 99 workspaces intact
12 / 12 archetypes intact

all consolidated domain batches closed
product-surface convergence closed
enterprise journey/exception convergence closed

one deterministic state authority preserved
exact preset reset proven
no cross-preset leakage
no broken owner handoff
no fake background action
no fake authoritative history
no reviewer metadata in ordinary UX

Stage-3 validation PASS
Stage-4 validation PASS
Stage-5 validation PASS
typecheck PASS
build PASS

browser evidence complete
accessibility findings dispositioned

INTEGRATED not falsely claimed
PRODUCTION-VALIDATED not falsely claimed
```

Only then may Stage 6 begin full ERP human review.

---

## 31. Planning Verdict

```text
STAGE 5 MASTER PLAN — READY WITH NON-BLOCKING FINDINGS
```

Stage-5 implementation has not started.

The batch topology above supersedes the earlier coarse Stage-5 hypothesis in the programme execution plan. In particular, large domains are deliberately decomposed so every batch can be completed and independently validated in one execution conversation.


---

## 28. Product-Convergence Amendment — 2026-09-26

This amendment refines execution method only. It does not change L0–L5 authority, the 1,084-scenario corpus, the 595-workflow register, the 99-workspace baseline, the 12 archetypes, mock dispositions, canonical ownership, lifecycle states or business events. Its original preservation of the 31-batch topology is superseded by the 2026-09-26 Remaining Batch Consolidation Decision.

### 28.1 Realization depth is risk-based

Complete workflow coverage remains mandatory, but bespoke UI/proof machinery is not required uniformly.

Deeper interactive realization is expected where workflows involve one or more of:

- lifecycle transition;
- approval/authority consequence;
- cross-domain handoff;
- financial consequence;
- examination/result consequence;
- correction/reversal/rollback;
- bulk or roster operation;
- exception/recovery;
- indeterminate outcome;
- material downstream projection change.

Straightforward reference, configuration, read-only, tracking or simple bounded workflows may reuse established workspace/archetype patterns when that preserves the registered realization semantics and validation evidence.

This does not downgrade REQUIRED workflows. It prevents implementation ceremony from becoming a parallel product architecture.

### 28.2 Representative institutional journeys

Stage-5 closure shall include cross-domain journey evidence. Exact journey composition must be derived from canonical workflow authority at execution time; this amendment does not invent new business workflows.

At minimum, the final corpus shall demonstrate representative journeys spanning:

1. institution/academic setup into downstream operational readiness;
2. applicant/admission into active Student context;
3. Student registration into academic delivery and assessment/examination consequences;
4. workforce engagement/assignment into faculty or payroll-relevant downstream context where upstream authority supports it;
5. procurement/resource acquisition into receiving/inventory/financial consequences where upstream authority supports it;
6. Student completion/exit into qualification/alumni consequences where upstream authority supports it.

A journey is validation composition across existing canonical workflows. It is not a new canonical workflow or ownership transfer.

### 28.3 Frontend structural convergence

Batch-specific implementation modules may remain as evidence/internal organization during Stage 5, but Stage-5 closure shall not leave the ordinary product runtime architecturally dependent on an ever-growing chain of batch-specific product shells.

The preferred end state is:

```text
workspace registry
→ governed workspace composition
→ reusable action / outcome / handoff patterns
→ deterministic mock-operation/state layer
→ reviewer instrumentation behind explicit review boundaries
```

Any refactor toward this structure must preserve current behavior and traceability and must not rewrite upstream architecture.

### 28.4 Final L6 product target

Stage 5 and Stage 6 together shall leave a frontend that functions as an **executable ERP product blueprint**:

- browseable as SutraERP by ordinary users;
- stateful and deterministic without a backend;
- capable of representative legitimate workflow execution;
- capable of showing rejection, failure, indeterminate and handoff outcomes where supported;
- cross-domain coherent;
- resettable and replayable;
- architecture-traceable in reviewer tooling;
- suitable for discovering workflow and UX defects before backend integration;
- explicitly not production-integrated or production-validated merely because mock behavior is complete.

This product target is a refinement of the existing programme purpose, not a new architecture level or upstream business decision.
