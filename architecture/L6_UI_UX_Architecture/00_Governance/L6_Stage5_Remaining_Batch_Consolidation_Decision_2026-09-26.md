# SutraERP L6 — Stage 5 Remaining Batch Consolidation Decision — 2026-09-26

**Version:** 1.1.0  
**Status:** Architecture Decision — Proposed Replacement Topology  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Applies After:** Batch 11 completion  
**Supersedes:** Original Stage-5 Batch 12–30 execution topology only  
**Does Not Supersede:** protected corpus counts, workflow/scenario dispositions, workspace/archetype authority, Stage-5 validation gates, or Stage-6 Full ERP Human Review purpose.

---

## 1. Decision

The original 31-batch Stage-5 topology is no longer the preferred execution shape after Batch 11.

Batches 0–11 remain historical/canonical execution identities.

After Batch 11, the former Batches 12–30 are consolidated into **nine remaining execution waves**, numbered Batch 12–20.

The programme therefore becomes:

```text
Batches 0–11   historical / completed or current
Batches 12–17  remaining domain realization
Batch 18       enterprise product + cross-domain convergence
Batch 19       full corpus reconciliation
Batch 20       final Stage-5 closure / Stage-6 readiness
```

Total Stage-5 topology after this decision: **21 batches (0–20)** rather than 31 batches (0–30).

This is a reduction in execution ceremony, not a reduction in architectural or workflow coverage.

---

## 2. Why Consolidation Is Correct Now

The original topology intentionally constrained a normal batch to approximately 15–30 workflows so one execution conversation could implement, validate, evidence and close it safely.

That constraint was useful while the mock-operation foundation, deterministic state model, browser evidence pattern and reusable UX primitives were still being proven.

By the end of Batch 11 those foundations have been exercised across Platform / Administration, Institution & Structure, Student Lifecycle, Academic Delivery and People & Workforce.

Continuing to split every remaining domain into 20–30-workflow batches would increasingly optimize for execution-chat size and proof ceremony rather than product architecture.

The new controlling rule is:

```text
complete coverage
+ risk-bounded implementation
+ reusable product composition
+ independent validation
≠
one PR / one proof surface per small workflow group
```

Large consolidated batches may execute internally in chunks or parallel workstreams. A batch identity is a governance boundary, not a single-response size limit.

---

## 3. Stage-5 / Stage-6 Boundary

Stage 5 owns:

- complete workflow realization/disposition;
- deterministic mock-operational behavior;
- correct outcome semantics;
- owner-safe handoffs;
- shared-state/projection coherence;
- reset/preset integrity;
- reviewer/product separation;
- automated/static/browser evidence sufficient to establish that the executable ERP blueprint is ready for human review.

Stage 6 owns **Full ERP Human Review**:

- discoverability;
- terminology;
- navigation;
- task continuity;
- workflow grouping;
- information density;
- forms/data-entry burden;
- dead ends;
- duplicate workflows;
- handoff clarity;
- exception/recovery clarity;
- history/evidence clarity;
- responsive behavior;
- keyboard/focus/accessibility experience;
- role/presentation confusion;
- cross-domain subject/context continuity;
- remediation and disposition of human-review findings.

Therefore Stage 5 shall not multiply batches merely to perform repeated human-product critique that Stage 6 is explicitly designed to perform.

Stage 6 shall not accept missing Stage-5 workflow realization, broken ownership, missing outcome semantics, nondeterministic state or unexplained coverage gaps.

---

## 4. Consolidated Remaining Topology

### Batch 12 — Finance — Complete Domain F

**Replaces former:** Batches 12–13  
**Population:** 59 workflows / 15 workspaces  
**Execution model:** chunked internally by coherent Finance capability and risk.

Required protections:

- no fake settlement;
- accepted request is not equivalent to completed financial operation;
- indeterminate results suppress unsafe duplicate execution;
- financial consequences remain owner-correct;
- reversal/correction behavior is represented only where upstream-supported.

Finance remains one governance batch because the domain's semantics and shared financial consequences benefit from one integrated validation boundary. It may use multiple implementation chunks and independent subreviews.

---

### Batch 13 — Campus Operations — Complete Domain G

**Replaces former:** Batches 14–19  
**Population:** 154 workflows / 11 workspaces  
**Execution model:** mandatory chunked/parallel implementation by G workspace groups; one domain-level integration and closure gate.

The former six batches were primarily a size-management mechanism. They become internal workstreams rather than six independent governance closures.

Required protections:

- each G workspace retains exact workflow coverage;
- no workspace-local state authority;
- shared institutional subjects remain coherent;
- background/observable workflows remain non-fake-interactive;
- cross-workspace consequences are integrated before domain closure.

Batch 13 must not be executed as one monolithic response or one undifferentiated code change.

---

### Batch 14 — Institutional Services & Intelligence — Complete Domain H

**Replaces former:** Batches 20–21  
**Population:** approximately 54 workflows / 10 workspaces  
**Execution model:** chunked by coherent H workspace/capability groups.

Special emphasis:

- reporting/intelligence must not become canonical business ownership;
- read/observe experiences remain distinct from owner-domain mutation;
- institutional service workflows retain legitimate interaction where registered.

---

### Batch 15 — Examination — Complete Domain I

**Replaces former:** Batches 22–25  
**Population:** 99 workflows / 16 workspaces  
**Execution model:** mandatory multi-chunk implementation with explicit high-consequence subgates.

Examination is consolidated at the governance level but **not flattened operationally**.

Mandatory deep validation applies to:

- examination lifecycle transitions;
- assessment/result consequences;
- approval/authority-sensitive outcomes;
- correction/reversal where supported;
- publication/release consequences;
- Student/Academic cross-domain projections.

The former four batches become internal implementation groups so the whole Examination domain receives one coherent final integration review.

---

### Batch 16 — Enterprise Resource Management — Complete Domain J

**Replaces former:** Batches 26–27  
**Population:** approximately 54 workflows / 10 workspaces  
**Execution model:** chunked by coherent resource/procurement/asset capability.

Special emphasis:

- procurement/resource ownership;
- receiving/inventory consequences;
- Finance handoffs;
- supplier/resource context continuity;
- no foreign-owner mutation.

---

### Batch 17 — Product-Surface Convergence

**New consolidated purpose.**

This batch performs the structural convergence required before enterprise journey testing.

It shall:

- move ordinary product usage away from batch/proof/reviewer framing;
- ensure product-facing workspaces use business terminology;
- converge reusable workspace/action/outcome/handoff composition;
- remove or isolate unnecessary batch-specific product shells;
- retain reviewer/provenance tooling behind explicit Review / Experience Lab boundaries;
- preserve all deterministic state and traceability;
- run product-language leakage and duplicate-composition audits.

This is primarily a frontend-architecture convergence batch. It shall not invent business behavior.

---

### Batch 18 — Enterprise Cross-Domain Journey & Exception Convergence

**Replaces former:** Batch 28 plus the journey portion previously deferred toward final closure.

Primary validation populations remain:

```text
28 CROSS-WORKSPACE-HANDOFF
103 EXCEPTION-RECOVERY-WORKFLOW
10 mixed workflows
```

This batch validates already-realized behavior rather than reimplementing those populations.

It shall also prove representative canonical institutional journeys, derived from existing workflow authority, including where supported:

- institution/academic setup → operational readiness;
- applicant/admission → active Student context;
- Student registration → academic delivery → assessment/examination consequence;
- workforce engagement/assignment → faculty/payroll-relevant consequence;
- procurement/resource acquisition → receiving/inventory/Finance consequence;
- Student completion/exit → qualification/alumni consequence.

Hard fail if cross-domain state, ownership, context continuity, recovery or handoff semantics break.

This is automated/executable convergence evidence. Stage 6 remains responsible for the broader human usability judgment.

---

### Batch 19 — Full Corpus Reconciliation

**Replaces former:** Batch 29

Required proof:

```text
595 / 595 workflows accounted for
551 / 551 REQUIRED legitimately treated
11 / 11 NOT-REQUIRED explicitly treated
33 / 33 PROHIBITED mutation-free
10 / 10 mixed workflows override-safe
1,084 / 1,084 scenario mappings intact
99 / 99 workspaces intact
12 / 12 archetypes intact
one deterministic state authority
ordinary/reviewer surface separation intact
representative institutional journeys executable
```

No new domain implementation should normally be introduced except correction of reconciliation defects.

---

### Batch 20 — Final Stage-5 Validation, Cleanup & Stage-6 Readiness

**Replaces former:** Batch 30

Responsibilities:

- final CI and validator suite;
- complete browser-evidence reconciliation;
- deterministic reset/replay proof;
- preset leakage check;
- dead-code and obsolete batch-shell cleanup;
- reviewer metadata/product-language leakage scan;
- accessibility gate sufficient for Stage-6 entry;
- maturity-claim audit;
- architecture evidence reconciliation;
- explicit Stage-6 readiness decision.

No broad human UX redesign belongs here. Human institution-like review is Stage 6.

---

## 5. Response-Safe Internal Chunking Standard

Consolidation does not authorize oversized unreviewable execution.

A governance batch is **not** a ChatGPT-response boundary. No large consolidated batch is expected to complete in one response.

For Batches 12–16, and for any later convergence/reconciliation batch whose work cannot safely complete in one response, execution shall be decomposed into bounded response-safe chunks before implementation begins.

The chunk plan shall be derived from the live census, dependency seams, shared-state touch points, workspace boundaries and risk. Chunk count is therefore evidence-driven rather than pre-fixed.

For Batches 12–16:

```text
one governance batch
→ pre-declared bounded chunk plan
→ one response performs one bounded chunk unless two small adjacent chunks clearly fit
→ chunk-local implementation + targeted validation
→ commit/checkpoint on the same batch branch
→ next response continues the same batch
→ integrated domain validation
→ independent domain audit
→ one domain closure decision
```

A normal chunk should perform only the work necessary to finish its assigned slice:

1. reconstruct the relevant current state;
2. implement the bounded scope;
3. run targeted validators/tests;
4. correct defects discovered in that slice;
5. commit/checkpoint the completed slice;
6. report compact execution status;
7. stop.

A chunk shall not automatically perform full-domain audit, final Architecture closure, merge, or programme-level reconciliation unless it is explicitly the final/integration chunk.

Chunks may run in parallel only when they do not compete for the same state authority, shared component seam or unresolved ownership decision.

### 5.1 Mandatory end-of-response status

Every chunked execution response shall end with a compact status containing:

```text
Batch X — Chunk Y/N — COMPLETE | BLOCKED
Batch workflows completed: A/B
Current chunk workflows: C/C
Chunks remaining: N
Blocking findings: count / summary
Next chunk: concise scope
```

The status is conversational execution metadata, not canonical architecture authority.

### 5.2 Branch / PR discipline

The default model for a consolidated batch is:

```text
one batch branch
→ multiple chunk commits/checkpoints
→ one frontend PR for the batch
→ integrated validation
→ independent audit
→ one canonical Architecture closure record
```

Do not open a separate PR merely because a new ChatGPT response begins.

A separate PR/workstream is justified only when repository isolation, independent parallel ownership, conflict avoidance or risk containment materially requires it.

### 5.3 Architecture-document discipline

A chunk is not an architecture lifecycle object.

Do not create one Architecture evidence document per chunk merely to record progress.

Chunk evidence may remain in commits, PR descriptions, CI/test evidence or concise execution logs. Architecture governance records material decisions, governed scope, validation outcomes and batch closure.

A chunk is not a new canonical batch and does not require a separate architecture closure document unless a material architectural decision occurs.

---

## 6. Evidence Simplification

The previous rule encouraging one full Architecture evidence record for every small implementation batch is refined.

For consolidated domain batches:

- one canonical domain-batch closure record is required;
- chunk/workstream evidence may live in CI artifacts, PR descriptions, test evidence or bounded supporting records;
- do not create architecture documents merely to record conversational progress;
- canonical Architecture evidence records decisions, coverage, validation and closure—not every implementation step.

This reduces repository noise without weakening traceability.

---

## 7. What Is Not Being Reduced

This decision does **not** reduce:

- 595-workflow accountability;
- 551 REQUIRED realization obligations;
- 11 NOT-REQUIRED explicit dispositions;
- 33 PROHIBITED mutation-free obligations;
- 10 mixed-workflow override requirements;
- 1,084 scenario mappings;
- 99 workspace coverage;
- 12 archetypes;
- owner preservation;
- lifecycle/event/rule authority;
- deterministic reset;
- accessibility requirements;
- browser evidence;
- independent review;
- final reconciliation.

---

## 8. Stage-6 Entry Contract

Stage 6 may begin only when Batch 20 proves that a reviewer can enter a coherent mock ERP with:

- complete workflow coverage;
- deterministic and resettable state;
- legitimate business actions/outcomes;
- cross-domain state coherence;
- owner-safe handoffs;
- representative institutional journeys;
- no dependency on architecture/batch/proof terminology for normal use;
- reviewer tooling available separately;
- no unexplained coverage gap.

Stage 6 then answers the different question:

> Is this executable ERP understandable, efficient, coherent and usable when reviewed as a real institution would experience it?

Stage 6 findings may cause targeted frontend remediation and, where evidence reveals an upstream contradiction, governed escalation. Stage 6 does not silently redefine L0–L5.

---

## 9. Final Decision

The original 31-batch topology was safe but is now unnecessarily granular for the mature Stage-5 foundation.

The approved target topology after Batch 11 is:

```text
12 Finance
13 Campus Operations
14 Institutional Services & Intelligence
15 Examination
16 Enterprise Resource Management
17 Product-Surface Convergence
18 Cross-Domain Journey / Exception Convergence
19 Full Corpus Reconciliation
20 Final Validation / Stage-6 Readiness
```

**Remaining governance batches after Batch 11: 9.**

This topology is the minimum recommended consolidation that preserves risk isolation for Finance and Examination, allows Campus Operations to execute through internal parallel workstreams, creates an explicit product-convergence boundary, and hands Stage 6 a complete executable ERP rather than a collection of proof surfaces.
