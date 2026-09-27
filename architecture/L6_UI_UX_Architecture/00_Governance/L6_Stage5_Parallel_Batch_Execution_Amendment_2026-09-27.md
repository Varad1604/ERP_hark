# SutraERP L6 — Stage 5 Parallel-Batch Execution Amendment — 2026-09-27

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support Governance  
**Programme:** SutraERP Full Workflow Realization & Mock ERP Programme  
**Decision Date:** 2026-09-27  
**Approval Source:** Explicit Lead Architect authorization to begin Batch 13 in parallel with active Batch 12 after repository reconstruction confirmed dependency safety  
**Amends:** `L6_Stage5_Domain_By_Domain_Mock_Operational_Implementation_Reviewed_Gameplan_2026-09-22.md` §19  
**Does Not Amend:** L0–L5 business/solution authority; canonical ownership; workflow/scenario dispositions; protected counts; workspace identities; lifecycle states; business rules; canonical events/publishers.

---

## 1. Decision

Stage 5 adopts a narrow **dependency-safe parallel-batch implementation exception**.

The default execution order remains serialized. However, a later consolidated domain batch may begin before the immediately preceding domain batch is merged when its foundational dependencies are already canonical and parallel execution can be isolated without inventing or transferring business authority.

The execution model is:

```text
dependency-safe parallel implementation
+
isolated batch branches
+
controlled shared-seam integration
+
serialized/revalidated canonical closure where cross-batch runtime seams exist
```

This decision changes execution sequencing only. It does not change architectural dependency direction.

---

## 2. Mandatory Entry Conditions

Parallel batch implementation is authorized only when all of the following hold:

1. every foundational architectural dependency of the later batch is already canonical;
2. the later batch does not use unfinished implementation from another active batch as architecture authority;
3. each batch has an isolated implementation branch;
4. shared/global seams are identified before implementation and are isolated, deferred, or centrally reconciled;
5. no unresolved ownership, lifecycle, event, rule, or authority conflict exists between the active batches;
6. any architectural conflict stops the affected scope rather than being resolved by frontend invention;
7. protected Stage-5 workflow/scenario/workspace/disposition counts remain unchanged unless separately governed.

Failure of any mandatory condition returns the affected batch to the normal serialized entry rule.

---

## 3. Closure Rule

Parallel implementation does not imply independent unverified closure.

Where a later batch consumes runtime/read-model behavior from an earlier concurrently active batch, the later batch shall not receive final canonical closure until:

1. the earlier batch's relevant implementation is merged or otherwise canonicalized;
2. shared/global changes are reconciled;
3. affected runtime/read-model workflows are rerun;
4. canonical ownership remains preserved;
5. no duplicate state authority has been introduced;
6. browser/validation evidence for the affected seams passes.

Unaffected local implementation may complete before this convergence gate.

---

## 4. Batch 12 / Batch 13 Authorization

Current repository evidence establishes:

```text
Batch 12 — Finance — Domain F
Batch 13 — Campus Operations — Domain G
```

Section G foundationally depends on Sections A, B, and E. It does not have a foundational dependency on Section F.

Therefore:

**Batch 13 Campus Operations is authorized to begin implementation while Batch 12 Finance remains active.**

Batch 13 shall:

- use a dedicated branch separate from Batch 12;
- consume only canonical Finance architecture for Finance-facing semantics;
- never manufacture payment settlement, refund settlement, ledger posting, vendor payment, or accounting completion;
- isolate/defer shared/global files where concurrent Finance edits create collision risk;
- record Finance-facing Campus workflows for final convergence;
- revalidate Finance-facing Campus behavior against the final merged Batch-12 implementation before Batch-13 closure.

---

## 5. Section J Boundary

Parallel authorization does not permit Campus Operations to absorb Enterprise Resource Management ownership.

Section J remains canonical owner of enterprise procurement, supplier/vendor operations, inventory/warehouse/stock operations and asset lifecycle where established by upstream architecture.

Campus may consume those facts/contracts without creating a second institutional resource truth.

---

## 6. Repository Safety Classification

Concurrent changes shall be classified as:

```text
LOCAL
batch/domain-owned implementation
→ proceed on isolated branch

SHARED
global registry, shell, shared state/projection, common validation/configuration
→ isolate, defer, or reconcile centrally

ARCHITECTURAL
ownership, lifecycle, rule, event/publisher, authority or cross-domain semantics
→ stop affected scope and escalate through architecture governance
```

No shared seam shall be independently redesigned by two concurrent batches.

---

## 7. Scope Preservation

This amendment does not alter:

- the consolidated Stage-5 Batch 12–20 topology;
- Batch-13 population of 154 workflows / 11 workspaces;
- any L6 workflow realization ID;
- canonical L3 scenario identities;
- REQUIRED / NOT-REQUIRED / PROHIBITED disposition authority;
- mixed-workflow override semantics;
- L0–L5 ownership or business semantics.

---

## 8. Lock Statement

**DEPENDENCY-SAFE PARALLEL STAGE-5 IMPLEMENTATION:** **AUTHORIZED**  
**DEFAULT SERIALIZED ENTRY RULE:** **RETAINED**  
**BATCH 12 / BATCH 13 PARALLEL IMPLEMENTATION:** **AUTHORIZED**  
**ISOLATED BATCH BRANCHES:** **MANDATORY**  
**SHARED-SEAM INDEPENDENT REDESIGN:** **PROHIBITED**  
**CROSS-BATCH RUNTIME REVALIDATION BEFORE LATER CLOSURE:** **MANDATORY**  
**L0–L5 SEMANTIC CHANGE:** **NONE**  
**STATUS:** **LOCKED — ACTIVE**
