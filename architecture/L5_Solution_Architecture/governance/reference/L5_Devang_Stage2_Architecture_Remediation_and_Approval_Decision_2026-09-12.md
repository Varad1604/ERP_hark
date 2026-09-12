# SutraERP L5 — Devang Stage-2 Architecture Remediation & Approval Decision

**Decision Date:** 2026-09-12  
**Architecture Level:** L5 — Solution Architecture Governance  
**Decision Type:** Stage-2 Remediation, Cross-Specification Reconciliation and Architecture Approval  
**Status:** Architecture Approved  
**Review Baseline:** `main` after `ADR-L5-0009` approval  
**Scope:** Devang Stage-2 `L5-D-02-001` through `L5-D-02-007`

---

## 1. Decision

Devang's seven Stage-2 Authorization Architecture specifications are approved to advance as one reconciled L5-D-02 family:

- `L5-D-02-001` — Authorization Architecture Baseline;
- `L5-D-02-002` — Permission & Capability Model Architecture;
- `L5-D-02-003` — Hierarchical Scope & Resource Context Architecture;
- `L5-D-02-004` — Policy Decision & Enforcement Architecture;
- `L5-D-02-005` — Delegation & Temporary Authority Architecture;
- `L5-D-02-006` — Privileged & Administrative Access Architecture;
- `L5-D-02-007` — Separation-of-Duties, Maker-Checker & Emergency Override Architecture.

The Stage-2 corpus is now **architecture-valid, technically sufficient for its reusable L5 responsibility, dependency-valid, and ready for downstream Stage-3 consumption**.

This decision does not lock the specifications.

---

## 2. Governing Basis

The decision applies the locked L5 governance, validation standard, frozen 105-specification register, dependency-gated execution timeline, ADR methodology/register, applicable upstream ownership authorities, and the controlled validation-program rule that canonical L4 System Behavior Specifications are accepted as behavioral authority for the review even where local repository metadata remains `Draft` or `Review`.

This decision does not change those L4 files' own metadata.

The decision also consumes:

- `ADR-PLAT-AUTH-001` — Person / Role Assignment ownership and authentication/authorization separation;
- `ADR-L5-0001` — trust-boundary/default-deny constraints;
- `ADR-L5-0004` — Tenant / Institution isolation;
- `ADR-L5-0009` — approved Non-Human Principal Authorization Binding Pattern.

---

## 3. Primary Remediation — Non-Human Principal Authorization

The prior review identified one systemic HF-11 gap: D01 established non-human service/workload/job/integration principals, but D02 had not yet defined how those principals receive bounded authorization authority without fabricating human Person/Role Assignment semantics.

That blocker is resolved by:

`ADR-L5-0009 — Non-Human Principal Authorization Binding Pattern`.

The canonical pattern is now:

```text
D01-002 Non-Human Principal
        ↓
D02 Technical Principal Authority Binding
        ↓
D02-002 Permission / Capability
        ↓
D02-003 Tenant / Institution / Resource Scope
        ↓
D02-004 Policy Evaluation
        ↓
Allow | Deny
```

For privileged machine execution, D02-006 adds the applicable heightened controls.

This decision explicitly preserves:

- no fabricated Person for machines;
- no reuse of human User Account as machine identity;
- no direct attachment of authority to credential secret material;
- Tenant-rooted scope;
- normal Permission/policy/Deny evaluation;
- no implicit internal-service or infrastructure superuser;
- D03 credential/trust ownership;
- D04 attributable evidence.

Accordingly, machine-principal ODs in the Stage-2 drafts that stated production machine authorization was architecturally unresolved are **superseded by ADR-L5-0009 for current approval interpretation**.

---

## 4. Upstream Authority Gaps — Correct Disposition

The Stage-2 drafts correctly preserve several upstream gaps, including examples such as:

- exact Authorization Policy lifecycle/governance actors;
- Delegation approver and command authority;
- Temporary Access approver and command authority;
- SoD policy governance/override authorities;
- maker-checker qualification semantics for individual protected business processes;
- canonical event names/publishers where upstream has not defined them.

These gaps shall **not** be repaired inside L5.

They are reclassified as follows:

```text
Reusable D02 architecture
        = not blocked

Specific feature / workflow that requires an undefined upstream authority
        = blocked until that authority is defined
```

This is the correct ownership-preserving interpretation because D02 already defines the technical mechanism without claiming the missing business authority.

Examples:

- D02-005 may define time-bounded Delegation execution but shall not invent who approves a Delegation.
- D02-007 may enforce maker/checker distinction but shall not invent which business process requires it or who qualifies as checker.
- D02-004 may execute an authoritative policy version but shall not invent missing business policy approval roles.

These unresolved upstream semantics therefore remain visible and traceable, but they do not constitute HF-11 for the reusable D02 family itself.

---

## 5. D02 Family Reconciliation

The seven specifications were reviewed as one committed family rather than as isolated authoring drafts.

The family ownership decomposition is accepted as canonical:

| Specification | Canonical Detailed Responsibility |
|---|---|
| `L5-D-02-001` | Common authorization invariants and family control-plane baseline |
| `L5-D-02-002` | Permission / capability representation and resolution |
| `L5-D-02-003` | Hierarchical scope and resource authorization context |
| `L5-D-02-004` | Policy decision and mandatory enforcement |
| `L5-D-02-005` | Delegation and Temporary Authority technical realization |
| `L5-D-02-006` | Privileged and administrative access safeguards |
| `L5-D-02-007` | SoD, maker-checker and emergency override enforcement |

No material duplicate technical ownership remains among the seven files.

Peer-dependency ODs that described prior D02 files as merely "working authoring artifacts" are now stale authoring-time statements. The files coexist on canonical `main` and have been reconciled under this decision. Those peer-stabilization ODs are therefore **closed unless a later material change reopens the dependency**.

---

## 6. L4 Status Wording

Several D02 files contain authoring-time wording stating that cited L4 specifications being `Draft` prevents Architecture Approval.

For this controlled Stage-2 validation program, canonical L4 specifications are accepted as behavioral authority by explicit review governance.

Therefore, local D02 statements that treat L4 status metadata alone as an approval blocker are superseded for this Stage-2 approval decision.

This does not alter the L4 documents' status metadata and does not establish a repository-wide rule outside the governed validation program.

---

## 7. Hard-Fail Result After Remediation

| Hard Fail | Result | Reason |
|---|---|---|
| HF-01 Invalid Behavioral Authority | PASS | Canonical L4 behavior accepted under controlled review authorization. |
| HF-02 Orphan Architecture | PASS | D02 responsibility exists in frozen 105-spec corpus. |
| HF-03 Business-Semantic Corruption | PASS | No business rule/authority/lifecycle ownership is transferred. |
| HF-04 Canonical Ownership Transfer | PASS | D01, A04, D03, business-domain and upstream owners remain intact. |
| HF-05 Silent Upstream Repair | PASS | Missing business approvers/policy authorities remain explicit upstream gates. |
| HF-06 Duplicate Technical Ownership | PASS | Seven-file D02 decomposition reconciled and non-overlapping. |
| HF-07 Cross-Boundary Bypass | PASS | No unauthorized direct peer-state ownership/write authority. |
| HF-08 Claimed L4 Realization Failure | PASS | Authorization, scope, Deny, temporary authority, SoD and policy-version obligations are technically realizable. |
| HF-09 Ungoverned Material Decision | PASS | Machine-principal pattern governed by approved ADR-L5-0009. |
| HF-10 Hidden Material Assumption | PASS | Material upstream gaps remain explicit. |
| HF-11 Material Architecture Unimplementability | PASS | Machine-principal authority binding resolved; other upstream gaps are scoped feature gates rather than missing D02 mechanism. |
| HF-12 Security / Privacy / Trust Violation | PASS | Tenant isolation, attribution, fail-closed and privilege boundaries preserved. |
| HF-13 L6 Boundary Violation | PASS | UI/navigation remains non-authoritative. |
| HF-14 Blocking Unresolved Architecture | PASS | No unresolved D02-owned blocker remains. |
| HF-15 Canonical Identity / Metadata | PASS | Canonical IDs, family, titles and placement are correct. |

---

## 8. Approval Result by Specification

| Specification | Result | Remaining D02-Owned Blocking Findings |
|---|---|---:|
| `L5-D-02-001` | Architecture Approved | 0 |
| `L5-D-02-002` | Architecture Approved | 0 |
| `L5-D-02-003` | Architecture Approved | 0 |
| `L5-D-02-004` | Architecture Approved | 0 |
| `L5-D-02-005` | Architecture Approved | 0 |
| `L5-D-02-006` | Architecture Approved | 0 |
| `L5-D-02-007` | Architecture Approved | 0 |

Overall Devang Stage-2 result:

```text
7 / 7 Architecture Approved
0 unresolved D02-owned blocking findings
READY FOR STAGE 3 DEPENDENCY CONSUMPTION
LOCK-READY, BUT NOT LOCKED
```

---

## 9. Downstream Effect

The dependency timeline defines:

```text
Devang
D01
  ↓
D02
  ↓
A05 Workflow & Orchestration
```

With this remediation and approval, D02 is now sufficiently stable for `L5-A-05` Stage-3 authoring to consume:

- authenticated human/non-human principals from D01;
- bounded human and technical-principal authority;
- Permission/capability semantics;
- Tenant/Institution/resource scope;
- centralized policy decision/enforcement;
- Delegation/Temporary Access;
- privileged/admin controls;
- SoD/maker-checker/emergency enforcement.

A05 shall still not invent any upstream business approval authority that D02 deliberately leaves source-owned.

---

## 10. Metadata Synchronization Rule

The canonical architecture decision established here is authoritative over stale authoring-time `Draft` blocker wording in the seven D02 files.

The following mechanical synchronization is authorized and required when those specification files are next edited or normalized:

1. set `Status: Architecture Approved`;
2. add `ADR-L5-0009` to applicable ADR metadata for D02-001/002/003/004/006;
3. mark machine-principal blocking ODs resolved by `ADR-L5-0009`;
4. close stale peer-working-artifact ODs after this family reconciliation;
5. remove L4-status-only approval blockers under the controlled Stage-2 validation rule;
6. retain genuine upstream authority/event gaps as non-D02 feature-realization gates;
7. correct the D02-005 `reponsibility` typo to `responsibility`.

Until mechanical synchronization occurs, this decision governs current architectural status and interpretation.

---

## 11. Lock Governance Boundary

Architecture Approved does not equal Locked.

The seven D02 specifications may be treated as **lock-ready**, but final `Locked` state requires a distinct explicit final-governance authorization under the L5 governance standard.

Any material upstream, ADR or peer-architecture change before lock requires impact assessment.

---

## 12. Final Position

```text
Devang Stage 2
    = 7 / 7 specifications authored
    = machine-principal blocker resolved through ADR-L5-0009
    = D02 family cross-file reconciliation complete
    = upstream business-authority gaps preserved, not invented
    = 7 / 7 Architecture Approved
    = 0 D02-owned blocking findings
    = APPROVED TO PROCEED TO STAGE 3
    = LOCK-READY
    ≠ Locked until separate final governance authorization
```
