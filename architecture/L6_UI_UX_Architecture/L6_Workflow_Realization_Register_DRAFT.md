# SutraERP L6 — Workflow Realization Register — Draft

**Version:** 0.1.0  
**Status:** Draft / Controlled Expansion Register  
**Owner:** Lead Architect  
**Architecture Level:** L6 — UI / UX Architecture Support  
**Last Updated:** 2026-09-19  
**Depends On:** canonical 1,084-scenario L3 corpus; existing L6 workspace inventory and traceability register; `00_Governance/L6_Full_Workflow_Mock_ERP_Expansion_Authorization_2026-09-19.md`.  
**Purpose:** Record exactly how every canonical L3 scenario is realized, observed or deliberately not directly interacted with in the SutraERP UI without duplicating business architecture.

> This register is a realization/coverage locator. It is not a new business-workflow authority, not an authorization model, not a backend contract, and not a requirement for one screen per scenario.

---

## 1. Coverage Rule

Every canonical L3 scenario must have exactly one **primary UI-realization disposition**.

Coverage target:

```text
Canonical L3 scenarios          1,084
Required realization rows       1,084
Unexplained scenarios           0
```

A scenario may contribute to multiple UI surfaces, but one primary disposition is required for accounting and review ownership.

---

## 2. Primary Dispositions

Allowed primary values:

- `INTERACTIVE-WORKFLOW`
- `REVIEW-APPROVAL-WORKFLOW`
- `TRACKING-WORKFLOW`
- `EXCEPTION-RECOVERY-WORKFLOW`
- `CROSS-WORKSPACE-HANDOFF`
- `SYSTEM-BACKGROUND-OBSERVABLE`
- `NO-DIRECT-END-USER-INTERACTION`

These classifications describe UI realization only. They do not change the upstream business scenario type or ownership.

---

## 3. Required Fields

Each realization row shall record:

| Field | Meaning |
|---|---|
| L3 scenario | Canonical final L3 ID |
| Scenario title | Canonical source title |
| Primary workspace | Existing L6 workspace family / shared surface |
| Presentation profiles | Supported UX presentation contexts |
| Primary disposition | One controlled realization category |
| User-facing label | Human-readable workflow/result name |
| Discovery entry | Search/workspace/task/handoff/history/etc. |
| Observable outcome | What a reviewer/end user can meaningfully observe |
| Mock interaction | Required / not required / prohibited |
| UX archetype(s) | Existing reusable archetype mapping |
| Owner/handoff | Canonical owner/destination where relevant |
| Upstream trace | Existing traceability-register/source reference |
| Realization maturity | Mapped / discoverable / mock-interactive / observable / reviewed |
| Open issue | Gap/escalation if any |

---

## 4. User-Facing Naming Rule

Ordinary UI shall not expose L3/L4/L5 identifiers.

Example:

```text
Reviewer trace: L3-F-0XX
Ordinary UI: Process Student Refund
Workspace: Refunds & Adjustments
```

Reviewer tooling preserves exact internal traceability.

---

## 5. Interaction Rule

Do not force a scenario into an interactive control.

If the scenario is automatic/system-driven, the correct realization may be:

- status;
- result;
- history;
- notification;
- evidence;
- exception;
- read-only projection;
- downstream handoff.

A fake “Start” button is an architecture defect.

---

## 6. Relationship to Existing Registers

- Workspace Inventory — answers **where sustained UI context lives**.
- Persona/Journey Map — answers **how users navigate toward goals**.
- Screen/Journey Traceability Register — answers **which upstream scenarios justify workspace/surface coverage**.
- Workflow Realization Register — answers **how each individual canonical scenario is actually exposed or observed in the UI**.
- Experience Slice Register — locates **selected deep implementation/review increments**.

No register replaces another.

---

## 7. Population Method

Population must be generated from canonical repository sources and independently validated.

Required process:

1. enumerate canonical 1,084 L3 IDs;
2. resolve each scenario to its existing L6 workspace-family coverage;
3. inspect scenario semantics and relevant L4/L5 evidence where interaction meaning requires it;
4. assign primary UI disposition;
5. assign human-readable label and discovery route;
6. determine mock interaction requirement;
7. detect duplicates/gaps/conflicts;
8. independently review;
9. record final counts.

No sampling is permitted for completion claims.

---

## 8. Initial Status

This file defines the controlled schema only.

The 1,084 realization rows are **not yet populated** by this authorization update.

Current state:

```text
Realization schema                 ESTABLISHED
Canonical target                   1,084
Rows populated                     0 / 1,084
Completion claim                   NOT AUTHORIZED
```

Population occurs in Stage 2 of the Full Workflow Realization & Mock-Operational ERP Execution Plan.
