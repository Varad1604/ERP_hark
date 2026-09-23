# SutraERP L6 — Stage 5 Batch 5 — Institution & Structure — Final Evidence & Pre-Merge Closure

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Evidence Type:** Batch Closure Evidence  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Decision Date:** 2026-09-23  
**Batch Result:** PASS  
**Merge Gate:** PASS  
**Repository State:** PASS / COMPLETE / FRONTEND MERGED / ARCHITECTURE CLOSURE PROPOSED  
**Domain B:** COMPLETE AT STAGE-5 BATCH-5 SCOPE  
**Next Batch:** Batch 6 ENTRY-READY ONLY AFTER THIS CLOSURE IS CANONICAL; NOT STARTED  

---

## 1. Live Repository Reconstruction

Batch 5 was reconstructed from live canonical repository state before implementation.

### Frontend

Initial canonical `main`:

```text
52b76e4cffe58aff4ce4641196c5454bbe3d0639
```

Batch-5 validated implementation head:

```text
4c014a52c5115b9e515a5104b25271cbac6e298f
```

Frontend PR #30:

```text
Stage 5 Batch 5 — Institution & Structure mock-operational realization
MERGED
merge commit:
39f3754961fb24ec183bb1ebb741a9bcf9d14e33
```

Final frontend validation:

```text
Frontend PR Validation #251
run ID: 35861169197
SUCCESS
```

Post-merge frontend `main` is `39f3754961fb24ec183bb1ebb741a9bcf9d14e33`.

### Architecture

Initial Batch-5 reconstruction observed Architecture `main` at:

```text
3c0a577d91291e44a85cb810d32714d172f68507
```

Architecture advanced during Batch-5 execution through independent L5 synchronization work.

Current canonical Architecture `main` at closure reconstruction:

```text
4741d7503458256d5426933f8837fe33f56026ec
```

The live Architecture state therefore supersedes the orientation SHA in the execution prompt.

At closure time, one unrelated Architecture PR remains open. It is not canonical and does not alter Batch-5 authority.

---

## 2. Batch-4 Entry Gate

Batch 4 remains canonically:

```text
PASS / COMPLETE / MERGED
Domain A: 35 / 35 workflows
Domain A: 62 / 62 scenarios
Batch 5: ENTRY-READY
```

Canonical Batch-4 frontend merge remains PR #29 at:

```text
52b76e4cffe58aff4ce4641196c5454bbe3d0639
```

No live-main change invalidated the Batch-4 closure or Batch-5 entry gate.

---

## 3. Protected Stage-5 Census and Provenance

No census regeneration was warranted.

All protected source blobs remain byte-identical to the Stage-5 baseline:

| Protected source | Live blob SHA | Result |
|---|---|---|
| Stage-2 Group-15 master reconciliation | `22dd6e0f38067936c7171c7e4386ebb58564f3f8` | MATCH |
| L6 Workflow Realization Register | `53448e586a066c08745ec6be27f9e58016f34483` | MATCH |
| Stage-4 Workflow Presentation Relevance Map | `b47748cb2cbf9b692d41fbfea2f147f63159bb7a` | MATCH |
| `DEPENDENCY_MAP.md` | `6d0dd83dbe4e0b015858ec54b336f4803c329c36` | MATCH |
| Stage-5 reviewed gameplan | `bc9dd50ff148607b7bd6d45def8bb27961f84b8a` | MATCH |
| Frontend generated census | `5514e9fc6400e72ce24f10e497fdb605673fa581` | MATCH |

Protected corpus remains:

```text
595 workflows
1,084 canonical L3 scenarios
99 workspaces
12 UX archetypes
10 mixed workflows
```

---

## 4. Exact Batch-5 Census

Batch 5 is exactly:

```text
Section B — Institution & Structure
15 workflows
33 unique L3 scenarios
5 workspaces
15 REQUIRED
0 NOT-REQUIRED
0 PROHIBITED
0 mixed workflows
```

Workspaces:

- W-B01 — Institution Administration
- W-B02 — Organisation Structure & Positions
- W-B03 — Institutional Campus Registry
- W-B04 — Curriculum Framework
- W-B05 — Academic Calendar

### Canonical workflow matrix

| Workflow | Workspace | Workflow family | Scenarios | Classification | Mock | L4 treatment |
|---|---|---|---|---|---|---|
| L6-RWF-0036 | W-B01 | Institution lifecycle | B-001;002;003 | REVIEW-APPROVAL-WORKFLOW | REQUIRED | 3 / 3 Architecture Approved |
| L6-RWF-0037 | W-B01 | Institution profile revision | B-004 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0038 | W-B01 | Institutional policy revision | B-005 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0039 | W-B02 | Organisation unit lifecycle | B-006;007;008;012;016 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0040 | W-B02 | Organisation structure transformation | B-009;010 | REVIEW-APPROVAL-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0041 | W-B02 | Organisation governance configuration | B-011 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0042 | W-B02 | Position definition lifecycle | B-013;014;015 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0043 | W-B03 | Institutional campus lifecycle | B-017…022 | REVIEW-APPROVAL-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0044 | W-B04 | Curriculum lifecycle | B-023;024;025 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0045 | W-B04 | School academic model lifecycle | B-026 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0046 | W-B05 | Academic calendar lifecycle | B-027;029;033 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0047 | W-B05 | Calendar assignment | B-028 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0048 | W-B05 | Calendar exception | B-030 | EXCEPTION-RECOVERY-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0049 | W-B05 | Academic year closure | B-031 | INTERACTIVE-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |
| L6-RWF-0050 | W-B05 | Calendar change simulation | B-032 | REVIEW-APPROVAL-WORKFLOW | REQUIRED | Review — conservative Locked-L3 replay |

Every row remains execution Batch 5.

---

## 5. L2 / L3 / L4 / L5 Authority Reconstruction

### L2

Canonical owners remain:

- L2-B01 — Institution
- L2-B02 — Organisation
- L2-B03 — Institutional Campus
- L2-B04 — Curriculum Framework
- L2-B05 — Academic Calendar

Institution Activation Authority remains an upstream abstract business authority. No universal Principal, Registrar, Administrator, Trustee, Board or other job title was fabricated as the canonical Institution approver.

Institutional Campus identity/status remains separate from the physical site/building/space model owned by Campus Operations / Infrastructure.

Position Definition remains Organisation-owned and distinct from technical system role/permission and from People/HR assignment semantics.

### L3

All 33 Section-B parents are current canonical Locked scenarios.

A repository-quality nuance was found in `L3-B-030`: its current header is `Locked`, while older historical body wording still says the architecture lock was not complete because an earlier authority issue remained open.

The later explicit 1,084-scenario corpus-lock decision governs current status and explicitly states that historical blocker wording does not silently override the later Locked corpus decision unless a governed change reopens the parent. Batch 5 therefore treats the residual wording as historical evidence and does not silently edit a Locked L3 file.

### L4

Current Section-B lifecycle distribution relevant to this batch is:

```text
L4-B-001 … L4-B-003     Architecture Approved
L4-B-004 … L4-B-033     Review
```

Consequently:

- L6-RWF-0036 may consume the approved L4 behavior baseline for its three parents;
- the remaining 14 workflows must not treat Review L4 documents as approved behavior authority;
- those 14 use Locked L3 business authority and conservative reviewer-local replay only;
- no event name, lifecycle transition, approver, formula, precedence rule, recovery rule or system responsibility was invented from Review material.

### L5

Architecture advanced during Batch-5 execution through the independent final L5 synchronization audit.

Current canonical L5 state is:

```text
0 Draft
0 Review
72 Architecture Approved
33 Locked
105 total
0 current hard blockers
Final L5 Corpus Lock = ready for distinct explicit authorization; not executed
```

The canonical L4→L5 traceability store still has no Section-B partition on current `main`.

That absence was treated as a constraint, not as permission to invent relations.

Batch 5 therefore:

- does not fabricate direct `L4-B-*` → `L5-*` relations;
- consumes only already-established reusable frontend/client/state/security constraints where applicable;
- creates no missing Section-B technical semantics;
- remains reviewer-local and non-production.

---

## 6. Authority Boundaries Preserved

### W-B01 — Institution Administration

Preserved:

- Institution identity and profile authority;
- Institution lifecycle authority;
- institutional policy ownership;
- abstract Institution Activation Authority;
- distinction between reviewer convenience and institutional decision authority.

No frontend-local Institution lifecycle truth exists.

### W-B02 — Organisation Structure & Positions

Preserved:

- Organisation hierarchy ownership;
- parent/child relationship authority;
- restructuring/type-migration authority;
- Organisation governance configuration;
- Position Definition lifecycle;
- People/HR as contextual consumer rather than Organisation owner.

The UI renders structure context; it is not an Organisation hierarchy engine.

### W-B03 — Institutional Campus Registry

Preserved:

- Institution-versus-Campus distinction;
- Institutional Campus identity/status ownership;
- campus lifecycle/history;
- modification versus decommissioning versus archival;
- physical/infrastructure state remains outside Section B.

No physical site condition/readiness was inferred.

### W-B04 — Curriculum Framework

Preserved separation between:

- Curriculum governance;
- School academic model;
- Academic Delivery;
- Student Lifecycle;
- scheduling / Course Offering execution.

No downstream academic-delivery or Student programme-state behavior was pulled into Batch 5.

### W-B05 — Academic Calendar

Preserved:

- Academic Year / Academic Period / Calendar governance;
- assignment, exception, closure and simulation boundaries;
- Student / Faculty access as contextual projection;
- Calendar-change simulation as reviewer/demo presentation only.

No scheduling engine, conflict solver, automatic closure engine or impact-simulation service was created.

---

## 7. Mock-Operation Contract

All 15 REQUIRED workflows use the established Stage-5 foundation.

No second operation framework was introduced.

Contract:

```text
review
→ pending
→ confirmed reviewer-local replay
→ one reviewer-local history record
```

Properties:

- exact workflow binding validated;
- exact proof-case binding validated;
- exact subject binding validated;
- exact intent binding validated;
- synchronous duplicate submission guarded;
- repeated completion rejected when reviewer-local history already exists;
- supplied canonical/mock business state remains unchanged;
- existing DemoState authority appends reviewer-local activity;
- reset reconstructs the active preset;
- preset replacement removes stale pending/local state;
- Back-to-review restores review focus;
- reset/preset replacement restore review focus;
- completion focuses live confirmed status;
- reviewer-local history is explicitly not canonical business event/audit evidence.

---

## 8. Frontend Implementation Boundary

Batch-5 additions are bounded to:

- `app/mock/operations/batch5-institution-structure.ts`;
- existing operation-catalog registration/export;
- `app/review/institution-structure-batch5-review-context.tsx`;
- `app/features/institution-structure-batch5/`;
- scoped responsive CSS;
- Batch-5 hard-gate validator;
- Batch-5 Playwright evidence;
- minimal route/root/CI wiring;
- extension of the pre-existing H02 catalog-order regression guard to preserve Batch-5 append order.

The feature layer does not import the DemoState authority directly. State execution remains behind the review-layer boundary.

No backend service, API client, persistence, browser storage, event bus, workflow engine, authorization engine, hierarchy engine, campus registry backend, Curriculum engine, scheduling engine, conflict solver, closure engine, simulation service or deployment infrastructure was introduced.

---

## 9. Determinism and Fixture Integrity

**PASS.**

No Batch-5 operation/UI source introduces:

- `Date.now()`;
- uncontrolled `new Date()`;
- `Math.random()`;
- `crypto.randomUUID()`;
- localStorage/sessionStorage/IndexedDB;
- fetch/XMLHttpRequest/WebSocket;
- new EventTarget-based infrastructure.

Reviewer subjects use explicit `review-case-*` references.

Existing coherent Stage-3/Stage-5 fixture context is reused for Institution / Organisation / academic-period presentation.

No parallel canonical fixture store was introduced.

---

## 10. Persona and Authorization Result

**PASS.**

Administrator presentation is reviewer convenience only.

It does not imply:

- Institution Activation Authority;
- Organisation governance authority;
- Campus lifecycle authority;
- Curriculum decision authority;
- Calendar exception/closure/simulation authority.

Student and Faculty contextual projections are read-only.

Browser evidence verifies those projections expose no Batch-5 governance controls.

---

## 11. Browser / Accessibility Evidence

Final browser validation passed:

| Suite | Result |
|---|---:|
| Stage-3 fixture convergence | 2 / 2 PASS |
| Stage-4 global workflow discovery | 8 / 8 PASS |
| Stage-4 workspace-local discovery | 7 / 7 PASS |
| Stage-4 full discovery coverage | 8 / 8 PASS |
| Stage-5 Batch-0 baseline regression | 1 / 1 PASS |
| Stage-5 Batch-1 H02 | 6 / 6 PASS |
| Stage-5 Batch-2 corrected golden slices | 12 / 12 PASS |
| Stage-5 Batch-3 Platform/Admin | 8 / 8 PASS |
| Stage-5 Batch-4 Platform/Admin | 10 / 10 PASS |
| Stage-5 Batch-5 Institution & Structure | 11 / 11 PASS |
| **Total** | **73 / 73 PASS** |

Batch-5 browser coverage includes:

- Stage-4 discovery into the correct Section-B workspace;
- all five workspaces;
- review → pending → confirmed;
- duplicate-submit protection;
- reset reconstruction;
- preset replacement/stale-state removal;
- Back-to-review focus;
- confirmed/live-status focus;
- Student/Faculty projection restrictions;
- visible metadata-leakage checks;
- dense Organisation hierarchy context;
- dense Academic Calendar context;
- desktop;
- narrow/mobile;
- horizontal-overflow prevention;
- keyboard operation;
- minimum 44px mobile touch target for Batch-5 proof controls.

### Evidence artifact

```text
Artifact ID: 10750084227
Name: stage5-batch5-browser-evidence
Size: 17,736,522 bytes
SHA-256: 65fd1a6f7e87c6b8e6d5b223693f07b870ebf9ea4d919355a1f1158d6483004d
Expires: 2026-12-22T12:32:58Z
```

---

## 12. Independent Devil's-Advocate Findings

The quality-first process found and corrected four implementation/test defects before PASS:

1. **Legacy H02 catalog regression guard was closed-ended at Batch 4.**  
   The guard was extended to require the exact Batch-5 append while preserving H02-first and the exact prior Batch-2/3/4 order. The earlier invariant was not weakened.

2. **Initial Batch-5 L4-authority validator counted a TypeScript union declaration as an approved proof instance.**  
   The gate was tightened to count concrete proof-object assignments only.

3. **One connector edit left a literal `\n` sequence in the operation-catalog import line.**  
   Structural gates passed, but TypeScript correctly rejected the source. The character-level defect was corrected; catalog ordering/semantics were unchanged.

4. **One Calendar browser assertion used “no authoritative Calendar mutation” while the rendered required boundary correctly said “without authoritative Calendar mutation.”**  
   Ten of eleven Batch-5 browser tests already passed. The assertion was aligned to the implementation wording; business behavior was not changed.

No unresolved material Batch-5 implementation defect remains.

---

## 13. Regression Result

**PASS.**

Final run #251 passed:

- review validation;
- Stage-4 discovery validation;
- Stage-4 provenance;
- mock seed / projection / state / fixture convergence;
- Stage-5 provenance;
- Stage-5 baseline;
- Batch-1 H02;
- Batch-2 corrected validation;
- Batch-3 Platform/Admin;
- Batch-4 Platform/Admin;
- Batch-5 hard gate;
- typecheck;
- production build;
- complete browser evidence.

Earlier batches remain green.

---

## 14. Repository Integrity Review

| Gate | Result |
|---|---|
| Correct L6 level boundary | PASS |
| Batch-4 entry gate | PASS |
| Protected census/provenance | PASS |
| Exact 15 workflow cohort | PASS |
| Exact 33 scenario cohort | PASS |
| Exact 5 workspace cohort | PASS |
| All 15 REQUIRED | PASS |
| Mixed workflows | PASS — 0 |
| L2 ownership | PASS |
| 33 Locked L3 parents | PASS |
| L4 lifecycle respected | PASS |
| Review L4 not promoted to authority | PASS |
| Current L5 reconstructed post-sync | PASS |
| Missing Section-B direct L4→L5 mapping not fabricated | PASS |
| Single state authority | PASS |
| Shared reviewer/reset/preset reuse | PASS |
| No feature-layer direct state authority import | PASS |
| Determinism | PASS |
| Duplicate completion protection | PASS |
| Reviewer-local history semantics | PASS |
| Persona / authority boundaries | PASS |
| Student / Faculty read projections | PASS |
| Metadata leakage | PASS |
| Responsive / overflow / touch targets | PASS |
| Typecheck / build | PASS |
| Browser evidence | PASS — 73/73 combined |
| Backend/infrastructure simulation | PASS — none introduced |
| Frontend merge | PASS — PR #30 merged |

---

## 15. Remaining Non-Blocking Repository Facts

These are not Batch-5 implementation failures:

1. `L4-B-004…033` remain `Review`; Batch 5 deliberately does not elevate them.
2. The L5 traceability store has no canonical Section-B partition; Batch 5 deliberately does not fabricate one.
3. `L3-B-030` contains historical residual “not locked” wording despite the later corpus-level Lock; the later corpus-lock authority governs. Any text cleanup requires controlled Locked-L3 change governance.
4. Final L5 Corpus Lock is ready for separate explicit authorization but remains unexecuted.

---

## 16. Decision

```text
STAGE 5 BATCH 5 — PASS
FRONTEND — COMPLETE / MERGED
ARCHITECTURE CLOSURE — MERGE GATE PASS
SECTION B BATCH-5 COHORT — 15 / 15 WORKFLOWS COMPLETE
SECTION B BATCH-5 SCENARIOS — 33 / 33 ACCOUNTED FOR
BROWSER EVIDENCE — 73 / 73 PASS
BATCH 6 — NOT STARTED
```

After this evidence is merged to canonical Architecture `main`, Batch 5 becomes repository-canonical PASS / COMPLETE / MERGED and Batch 6 may be marked ENTRY-READY under its own explicit execution instruction.
