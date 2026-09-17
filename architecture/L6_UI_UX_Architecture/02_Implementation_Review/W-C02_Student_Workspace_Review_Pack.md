# W-C02 — Student Workspace UI Review Pack

**Workspace:** `W-C02 — Student Workspace`  
**Architecture Source:** `../L6_Student_Workspace_DRAFT.md`  
**Proceed Gate:** `../00_Governance/L6_Student_Workspace_Validation_and_Proceed_Gate_Decision_2026-09-17.md`  
**Implementation Status:** Controlled frontend implementation authorized  
**Purpose:** Make W-C02 reviewable through realistic interactive UI states before backend integration is complete.  

---

## 1. Review Profiles

Review at minimum:

- Student Operations / authorized staff presentation;
- Institutional Administrator presentation where applicable;
- Student self-service presentation.

UI profiles are presentation compositions only and do not create authorization.

---

## 2. Required Review Records

Maintain clearly non-authoritative fixtures for at least:

### Record A — Normal active Student

Used for default orientation and read-only projection review.

### Record B — Student with lifecycle attention

Used to expose high-consequence lifecycle entry and consequence messaging.

### Record C — Student with document request in progress

Used for pending/status/result presentation.

### Record D — Student with cross-domain exception/projection

Used to test Finance/Registration/Examination/Attendance projection clarity and handoff.

### Record E — Historical / closure-oriented Student context

Used to test lifecycle history and archival/closure presentation without implying physical deletion.

Fixture fields must remain supported by approved upstream architecture and shall not invent new states or rules.

---

## 3. Staff Workspace Sections to Review

```text
Overview
Student Record
Lifecycle
Related
Documents & History
```

Reviewer should be able to move between all sections without losing the active Student/context.

---

## 4. Student Self-Service Sections to Review

```text
My Details
Academic Context
Requests & Documents
Related Statuses
```

The Student composition must remain simpler than the staff workspace and shall not expose institutional lifecycle-management controls.

---

## 5. Primary Review Journeys

### J1 — Find and understand a Student

Reviewer enters W-C02 and can identify:

- Student identity/context;
- current Institution/academic context;
- current Student Lifecycle state;
- current placement/enrollment/academic-assignment orientation;
- source-owned related statuses.

### J2 — Inspect Student record and history

Reviewer moves from Overview to Student Record and Documents & History while retaining orientation and distinguishing human-readable history from canonical audit evidence.

### J3 — Review a high-consequence lifecycle action

Reviewer opens a supported lifecycle action and can understand:

- action name;
- current context;
- potential consequence;
- evidence/input dependency where applicable;
- that the browser has not yet completed the business operation;
- that authoritative outcome comes from the owning backend boundary.

### J4 — Follow a cross-domain projection

Reviewer sees a Finance/Registration/Examination/Attendance projection and understands:

- it is read-only in W-C02;
- which domain owns it;
- where detailed work continues.

### J5 — Student document request status

Reviewer can see request state/result presentation and does not confuse document rendering with authoritative issuance.

### J6 — Student self-service

Student persona can understand their details, academic context, request/document status and related statuses without seeing staff lifecycle controls.

---

## 6. State Scenarios Required in Reviewer Mode

The W-C02 review selector should eventually expose at least these scenarios:

| Review scenario | Required presentation |
|---|---|
| Normal active Student | Default populated workspace |
| Loading Student context | Skeleton/progress with no false prior record |
| Student not found / legitimate empty | Clear no-result state and recovery path |
| Document request pending | Pending without success implication |
| Document request successful | Authoritative-success fixture presentation |
| Business rejection | Rejection distinct from technical failure |
| Technical failure | Safe retry/recovery wording without business inference |
| Indeterminate mutation | Outcome unknown; prevent unsafe repeat and require reconciliation |
| Institution context stale | Old Student data quarantined; re-resolution required |
| Authorization denied | Clear refusal without implying hidden controls are security |
| Cross-domain source unavailable | Other W-C02 content remains usable while unavailable projection is clearly source-specific |
| Historical/closed context | History preserved; archival not shown as physical deletion |

---

## 7. High-Consequence Review Targets

Review presentation for the approved W-C02 set where supported by architecture, including:

- suspension;
- restoration after suspension;
- reassignment / transfer;
- identity correction;
- duplicate Student resolution;
- permanent record closure / archival.

These review states shall not invent eligibility, evidence requirements, approving actors, merge semantics, completion events or backend success conditions.

---

## 8. Cross-Domain Projection Review

Check at minimum:

- Finance;
- Academic Registration;
- Examination;
- Attendance.

For each projection ask:

1. Is the source/owner understandable?
2. Is it clearly read-only here?
3. Is the summary useful without duplicating the owning workspace?
4. Is the handoff understandable?
5. If the source is unavailable, does W-C02 degrade gracefully rather than looking globally broken?

---

## 9. Responsive Checkpoints

Review:

- wide desktop;
- normal laptop;
- tablet / narrow desktop;
- Student self-service mobile;
- staff mobile only for tasks explicitly supported by the approved L6 boundary.

Check context visibility, local navigation, high-consequence action clarity, dialog reflow, status readability and no horizontal trapping of critical content.

---

## 10. Accessibility Checkpoints

At minimum:

- keyboard-only section navigation;
- visible focus;
- logical heading structure;
- accessible dialog title/focus return;
- status not communicated by color alone;
- readable error/rejection/indeterminate text;
- touch target adequacy where mobile is supported;
- reduced-motion compatibility;
- reflow without loss of critical information.

---

## 11. Current Implementation Review Findings to Track

The first W-C02 frontend implementation establishes the workspace composition and representative content, but the review process should now expand it beyond a single happy-path fixture.

Priority implementation-review gaps:

1. add explicit reviewer scenario/state switching;
2. separate fixture/scenario data from production workspace component code;
3. provide loading/empty/rejection/failure/indeterminate/stale/denied variants;
4. make cross-domain handoffs visibly testable;
5. review Student vs staff composition side by side;
6. validate high-consequence dialogs with multiple fixture contexts;
7. test responsive and keyboard journeys rather than only static layout.

These are implementation-review gaps, not upstream business-architecture changes.

---

## 12. Finding Log Template

| ID | Scenario | Observation | Classification | Severity | Proposed disposition | Status |
|---|---|---|---|---|---|---|
| W-C02-UX-001 |  |  | Implementation defect / L6 defect / L5 issue / upstream issue / preference |  |  | Open |

Do not change upstream architecture merely to resolve a local visual preference. Classify first, then act.
