# SutraERP L4 R1C — Workspace Governance — Architecture Approval Decision — 2026-09-23

**Version:** 1.0.0  
**Status:** Architecture Approved  
**Owner:** Lead Architect  
**Architecture Level:** Cross-level L2/L3/L4 governance  
**Decision Type:** Post-L5-validation upstream blocker remediation / controlled architecture decision incorporation  
**Scope:** L2-A04 Workspace governance amendment; L3-A-050 v1.1; L4-A-050 v1.0.1  
**Upstream Decision:** Option A — Workspace is the single governed aggregate; Workspace Mapping is governed Workspace configuration  

---

## 1. Decision

The previously blocked Workspace architecture has been resolved at its canonical upstream owner and propagated downward without L5 back-fill.

The approved architecture is:

- Authorization & Policy / L2-A04 owns the Workspace aggregate, lifecycle, mapping configuration, resolution semantics and Workspace lifecycle events.
- Workspace Mapping is configuration of a Workspace version, not an independent aggregate/lifecycle.
- Canonical Workspace lifecycle:
  - `Draft → Under Review → Active → Retired`;
  - governed return `Under Review → Draft`.
- Material change to an Active Workspace creates a governed successor version.
- `Workspace Governance Authority` is the abstract business authority for return, activation and retirement.
- Authorized administrative actors may configure Draft versions/mappings where institutional policy permits.
- Only Active Workspace versions are eligible for governed resolution.
- Canonical lifecycle events are:
  - `WorkspaceActivated`;
  - `WorkspaceRetired`.
- Authorization & Policy / L2-A04 is the sole semantic publisher.
- Workspace never grants Roles/Permissions and never changes Authorization Decisions.

---

## 2. Canonical Repository Changes

### L2

Added:

`architecture/L2_Domain_Workflow_Architecture/section-a/L2-A04_Workspace_Governance_Authority_and_Lifecycle_Amendment_2026-09-23.md`

Status: `Locked`.

This amendment supplies the missing canonical Workspace workflow/lifecycle, authority, mapping/version rules and lifecycle events.

### L3

Updated:

`L3-A-050_Workspace_Definition_to_Governed_Resolvable_Experience_Context.md`

- version `1.0 → 1.1`;
- status remains `Locked`;
- prior three blocking findings are resolved;
- exact Workspace actor/authority, lifecycle, successor-version path and event semantics now derive from the L2-A04 amendment;
- no UI implementation semantics are introduced.

### L4

Updated:

`L4-A-050_Workspace_Definition_to_Governed_Resolvable_Experience_Context.md`

- version `1.0.0 → 1.0.1`;
- status `Draft → Architecture Approved`;
- BHs now realize exact L3-A-050 v1.1 lifecycle, authority, resolution and event behavior;
- no implementation design is introduced.

---

## 3. Hard-Fail Validation Result

The remediated L4-A-050 passes the scoped L4 hard-fail gate.

Verified:

- no orphan behavior;
- no invented business rule;
- no lifecycle invention beyond the newly Locked L2-A04 amendment;
- no event invention beyond `WorkspaceActivated` / `WorkspaceRetired` authorized upstream;
- event publisher remains Authorization & Policy;
- Workspace Mapping does not become a duplicate aggregate or owner;
- Role/Permission ownership remains unchanged;
- Workspace does not alter Authorization Decisions;
- no unsupported universal local job title is invented;
- no L5/L6 API, database, service, infrastructure, navigation, dashboard or UI implementation leaks into L4;
- material Active changes use successor versioning and preserve history;
- Retired is terminal for that Workspace version.

**Verdict:** `PASS / Architecture Approved`.

---

## 4. Resolution of Previous A050 Blockers

| Previous blocker | Resolution |
|---|---|
| Workspace workflow/lifecycle/commands undefined | Resolved by Locked L2-A04 Workspace Governance amendment. |
| Workspace lifecycle events/publisher undefined | Resolved by canonical `WorkspaceActivated` and `WorkspaceRetired`, sole publisher Authorization & Policy / L2-A04. |
| Workspace request/configuration/approval authority undefined | Resolved through Authorized Administrative Actor plus abstract Workspace Governance Authority. |

No blocker is resolved by L4 or L5 invention.

---

## 5. Downstream Effect

This decision removes the upstream authority defect associated with L4-A-050.

It does **not** automatically:

- validate any L4-A-050 → L5 relation;
- change any L5 specification status;
- clear any Batch-9 L5 blocker disposition;
- create a new L5 ADR;
- authorize Final L5 Corpus Lock.

Affected L5 specifications must still pass the Locked L4→L5 traceability methodology and targeted HF revalidation.

---

## 6. R1 Programme Effect

With R1C complete, the original Shared Platform L4 Authority review set is fully resolved:

```text
R1A approved parents      6
R1B approved parents      7
R1C approved parent       1
---------------------------
R1 approved parents      14 / 14

Preserved R1 upstream blockers after R1C   0
```

This statement is limited to the R1 parent set. It does not claim that all 30 L5 blockers are resolved.

---

## 7. Final Decision

```text
L2-A04 Workspace amendment       LOCKED
L3-A-050                         LOCKED / v1.1 / blocker resolved
L4-A-050                         ARCHITECTURE APPROVED / v1.0.1
L4→L5 validation                 NOT YET PERFORMED
L5 disposition changes           0
New L5 ADRs                      0
Final L5 Corpus Lock             NOT AUTHORIZED
```

**Final Decision:** `R1C PASS — WORKSPACE UPSTREAM ARCHITECTURE RESOLVED; PROCEED TO CONTROLLED DOWNSTREAM TRACEABILITY / REMAINING REMEDIATION WORKSTREAMS`.
