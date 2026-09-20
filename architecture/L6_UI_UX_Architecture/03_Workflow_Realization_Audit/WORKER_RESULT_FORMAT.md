# SutraERP L6 Stage 2 — Worker Result Format

Every worker creates exactly one Markdown result artifact. The artifact must be mechanically parseable and human-reviewable.

## Header contract

Use these fields exactly:

```text
workerId:
assignedRange:
assignedCount:
processedCount:
missingCount:
duplicateCount:
baseMainSha:
workingBranch:
resultStatus:
```

`resultStatus` is `COMPLETE-FOR-MASTER-REVIEW` only when assignedCount = processedCount, missingCount = 0, duplicateCount = 0, and every assigned scenario has exactly one row.

## Row contract

Use exactly this column order and these field names:

| l3ScenarioId | canonicalTitle | primaryWorkspace | presentationProfiles | primaryDisposition | userFacingLabel | discoveryEntry | observableOutcome | mockInteraction | archetypes | ownerOrHandoff | upstreamTrace | realizationMaturity | openIssue |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|

Rules:

- one row per assigned canonical scenario, in ascending canonical ID order;
- `canonicalTitle` must exactly match the canonical L3 title;
- `primaryWorkspace` must be exactly one existing L6 workspace/shared-surface key such as `W-C02` or `S-02`; do not invent a workspace;
- `presentationProfiles` contains UX presentation contexts only, not authorization roles; use existing L6 terminology and separate multiple values with `; `;
- `primaryDisposition` must be exactly one of `INTERACTIVE-WORKFLOW`, `REVIEW-APPROVAL-WORKFLOW`, `TRACKING-WORKFLOW`, `EXCEPTION-RECOVERY-WORKFLOW`, `CROSS-WORKSPACE-HANDOFF`, `SYSTEM-BACKGROUND-OBSERVABLE`, `NO-DIRECT-END-USER-INTERACTION`;
- `mockInteraction` must be exactly `REQUIRED`, `NOT-REQUIRED`, or `PROHIBITED`;
- `archetypes` may contain only `entity-360`, `search-worklist`, `master-detail-queue`, `guided-flow`, `governed-lifecycle-action`, `roster-bulk-action`, `schedule-calendar`, `financial-ledger`, `request-document-tracking`, `configuration-master-data`, `analytics-monitoring`, `persona-home-dashboard`; separate multiple values with `; `;
- `upstreamTrace` must include the canonical L3 parent and material L4/L5 evidence actually used; L2/L1 may be included where exact semantics required verification;
- default `realizationMaturity` is exactly `SCENARIO-REALIZATION-MAPPED`; use a stronger maturity only when current independent repository evidence explicitly justifies it and cite that evidence;
- `openIssue` is empty when no issue exists. Otherwise prefix the issue with exactly one finding class: `Implementation defect`, `L6 UX architecture defect`, `L5 feasibility/integration issue`, `L4/L3/L2/L1 architecture issue`, `Review-process issue`, `Implementation-local improvement`;
- escape literal Markdown pipes inside cell content as `\|`;
- do not insert line breaks inside table cells;
- do not create workflow IDs.

## Required footer

After the rows, include:

```text
Assigned count:
Processed count:
Missing count:
Duplicate count:
Open issues:
Potential duplicate workflow labels:
Potential cross-worker normalization concerns:
```

List only real concerns. Similar labels are not automatically duplicates; compare semantics, workspace, actors/presentation, outcomes, interaction pattern, owner and business boundary.

## Acceptance

The master controller rejects a worker artifact if row count, range, canonical title, workspace key, disposition, archetype, mockInteraction, required field, or source trace is invalid, or if the artifact silently invents business semantics.
