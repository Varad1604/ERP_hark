# SutraERP Level 3 Development Path — Team Discussion Draft

**Document Type:** L3 Planning and Decision Summary  
**Project:** SutraERP  
**Status:** Discussion Draft — Not Yet Locked  
**Target Duration:** 20 Days  
**Prepared For:** SutraERP Architecture Team  
**Date:** 1 August 2026  

---

## 1. Purpose of This Document

This document summarizes the decisions made so far for creating **Level 3 Business Scenario Architecture** for SutraERP.

It is intended to help the team clearly understand:

- what Level 3 means;
- how the work will proceed;
- what has already been agreed;
- what is still open for discussion;
- how ChatGPT Go and ChatGPT Plus will be used;
- how documents will be created, reviewed, corrected, and finally locked.

This document is for team discussion. It becomes the working development charter only after the remaining open decisions are approved.

---

# 2. What Level 3 Means

Level 3 describes **real business scenarios** that happen inside an educational institution.

Each L3 file should describe one complete business outcome.

### Example

```text
Admission approved
    ↓
Identity confirmed
    ↓
Financial clearance confirmed
    ↓
Documents verified
    ↓
Student Context activated
```

This can become one scenario:

```text
Approved Candidate to Active Student
```

Level 3 does **not** define:

- API endpoints;
- database tables;
- software classes;
- technology choices;
- screen designs;
- button labels;
- deployment architecture.

Those belong to later architecture levels.

---

# 3. Main L3 Documentation Rule

Each L3 file should represent **one complete business outcome**, not one small action.

### Correct

```text
Student Online Payment and Receipt
```

### Incorrect as separate L3 files

```text
Validate Payment
Allocate Payment
Generate Receipt
Send Payment Notification
```

These smaller actions should normally remain steps inside the complete payment scenario.

---

# 4. Development Approach

The Level 3 development path will use the following phases.

## Phase 0 — Correct L2 Blockers

This phase is currently in progress.

The team must correct important L2 conflicts involving:

- domain ownership;
- lifecycle states;
- event names;
- dependencies;
- source references;
- Architecture Index and Matrix consistency.

Scenario discovery may begin while Phase 0 is running, but a scenario affected by an unresolved critical conflict must not be locked.

## Phase 1 — Discover Business Scenarios

Each team member studies the L2 documents assigned to their domains and creates a list of real business scenarios.

Only a short inventory is created during this phase.

### Phase 1 Inventory Format

| Field | Meaning |
|---|---|
| Candidate ID | Temporary scenario identifier |
| Scenario Title | Clear real-world business situation |
| Trigger | What starts the scenario |
| Final Outcome | What must be achieved |
| Primary Owner | Domain owning the main business result |
| Participating Domains | Other domains involved |
| Scenario Type | Simple, Cross-domain, or Critical |
| Priority | P0, P1, or P2 |

### Example

| Candidate ID | Scenario Title | Trigger | Final Outcome | Primary Owner | Domains | Type | Priority |
|---|---|---|---|---|---|---|---|
| CAN-STU-001 | Approved Candidate to Active Student | Admission approved | Active Student Context | Student Lifecycle | A, C, F | Cross-domain | P0 |

No full 15-section document is created during Phase 1.

## Phase 2 — Consolidate and Assign Scenarios

The complete scenario inventory is reviewed across all domains.

The team will:

- remove duplicate scenarios;
- merge scenarios that are too small;
- split scenarios that are too broad;
- identify related scenarios;
- decide the primary owner;
- assign one creator;
- assign mandatory reviewers;
- mark blocked scenarios;
- finalize scenario priority.

### Example of Merging

Instead of separate files for:

- validate online payment;
- allocate payment;
- generate receipt;
- update outstanding balance;
- send notification;

create one scenario:

```text
Student Online Payment and Receipt
```

### Example of Splitting

Do not create one file called:

```text
Complete Examination Management
```

Split it into meaningful outcomes such as:

- Examination Session Creation;
- Examination Planning and Scheduling;
- Examination Registration and Hall Ticket;
- Theory Examination Conduct;
- Marks Evaluation and Approval;
- Result Computation and Publication;
- Revaluation and Revised Result;
- Supplementary Examination;
- Examination Closure and Archival.

## Phase 3 — Create Pilot Documents

Three pilot scenarios are approved.

1. **Approved Candidate to Active Student**
2. **Regular Examination to Result Publication**
3. **Purchase Requisition to Goods Receipt and Inventory**

These pilots will test whether the L3 template properly handles:

- ownership;
- triggers;
- preconditions;
- cross-domain hand-offs;
- events;
- alternative flows;
- failure and recovery;
- authorization;
- audit;
- completion conditions;
- L1/L2 traceability.

Two optional pilots may be created only if the first three reveal missing template requirements:

4. Employee Leave to Payroll Impact  
5. Student Fee Payment and Receipt  

The default plan remains three pilots.

## Phase 4 — Finalize the Template

After the pilot documents are created and reviewed, the team will improve the template.

The team will check:

- whether any important information has no proper section;
- whether any section is unclear or unnecessary;
- whether cross-domain ownership is easy to understand;
- whether failure and recovery are sufficiently covered;
- whether documents are becoming too technical;
- whether an L4 author can understand the expected business behavior.

After correction, the template will be versioned and used for all remaining L3 documents.

## Phase 5 — Parallel Creation and Daily Verification

All five team members create assigned scenarios using ChatGPT Go.

The single ChatGPT Plus account verifies the submitted documents.

### Daily Cycle

```text
Scenario assigned
    ↓
Creator drafts using ChatGPT Go
    ↓
Creator performs self-check
    ↓
Document submitted for Plus review
    ↓
Plus identifies exact issues
    ↓
Creator corrects the document
    ↓
Corrected document is rechecked
```

Verification must happen continuously. The team should not wait until the last few days to review all documents.

## Phase 6 — Repository-Wide Validation

After the main scenario set is created, the full L3 repository is checked for consistency.

The validation must identify:

- duplicate ownership;
- overlapping scenarios;
- incompatible event names;
- incompatible state names;
- incorrect dependency direction;
- missing L2 coverage;
- missing lifecycle coverage;
- missing failure paths;
- inconsistent terminology;
- traceability gaps;
- unresolved critical open issues.

No document should be marked Locked only because it individually passed review. Final lock happens after repository-wide validation.

## Phase 7 — Final Index and L3 Lock

The team completes:

- L3 Scenario Index;
- L3 Coverage Matrix;
- L3 Scenario Dependency Map;
- Open Issue Register;
- Deferred Scenario Register;
- document status updates;
- final L3 validation report.

Only approved scenarios are locked.

P2 scenarios may remain listed for a later release.

---

# 5. Target Timeline

The target is **20 days**.

A practical working schedule is:

| Days | Work |
|---|---|
| 1–3 | Finish critical Phase 0 corrections |
| 4–5 | Create scenario inventory |
| 6 | Consolidate, prioritize, and assign scenarios |
| 7–8 | Create and verify the three pilots |
| 9–16 | Parallel creation and daily verification |
| 17–19 | Repository-wide validation and corrections |
| 20 | Index, coverage review, final approval, and lock decision |

The exact dates may move if Phase 0 takes longer, but the work order should remain the same.

---

# 6. Use of ChatGPT Accounts

## ChatGPT Go — Five Accounts

The five Go accounts will be used for:

- Phase 1 scenario discovery;
- first-draft creation;
- applying corrections;
- maintaining assigned scenario families;
- preparing files for Plus verification.

Each creator must work from:

- locked L1;
- relevant original L2 files;
- relevant canonical AI context;
- approved L3 template;
- approved scenario inventory;
- relevant dependency and integrity records.

The original L2 files remain authoritative. AI context summaries are supporting material, not replacements for the original source documents.

## ChatGPT Plus — One Verification Account

The Plus account will act as the quality gate.

Its job is:

```text
Review
→ Find exact problems
→ Recommend exact corrections
→ Recheck
→ Approve or reject
```

The Plus account should not rewrite every file from the beginning.

It should verify:

- L1 and L2 alignment;
- canonical ownership;
- scenario boundaries;
- missing business steps;
- event producer and consumer compatibility;
- state consistency;
- business rules;
- failure and recovery;
- authorization and audit;
- traceability;
- duplication with other scenarios.

---

# 7. Approved L3 Template Direction

The current proposed template has 15 sections.

## L3 Scenario Template

1. Metadata  
2. Purpose and Scope  
3. Trigger and Final Outcome  
4. Actors  
5. Participating Domains and Ownership  
6. Preconditions  
7. Main Flow  
8. Alternative Flows  
9. Failure and Recovery  
10. Business Rules  
11. Events and Hand-offs  
12. Authorization and Audit  
13. Completion Criteria  
14. L1/L2 Traceability  
15. Open Issues  

## Important Template Rules

### Purpose and Scope

This section must define:

- why the scenario exists;
- where it starts;
- where it ends;
- what is included;
- what is excluded;
- which related scenarios exist.

This prevents scenario overlap and uncontrolled document size.

### Failure and Recovery

Failure and Recovery should be mandatory for almost every L3 scenario.

It should explain:

- what can fail;
- where failure is detected;
- which owner is responsible;
- how state is affected;
- whether the scenario can resume;
- how duplicate actions are prevented;
- what happens when recovery is impossible.

Only a truly read-only or informational scenario may mark this section as “Not Applicable,” and it must provide a reason.

### Open Issues

Open issues must not be silently solved by the author or AI.

Each issue should state:

- the exact uncertainty;
- affected domain or source;
- status;
- required owner or decision;
- whether the scenario is blocked.

---

# 8. Scenario Classification

Every scenario will be classified in Metadata.

## Type A — Simple Scenario

Usually one primary domain and limited hand-offs.

Examples:

- Cancel Leave Request;
- Reissue Hall Ticket;
- Update Guardian Relationship;
- Transfer Stock Between Bins.

## Type B — Cross-Domain Scenario

Several domains participate in one clear business outcome.

Examples:

- Approved Candidate to Active Student;
- Student Fee Payment and Receipt;
- Hostel Allocation with Fee Charge;
- Employee Leave and Availability Update.

## Type C — Critical Scenario

Financially, legally, academically, operationally, or historically high-risk.

Examples:

- Result Publication;
- Qualification Completion;
- Payroll Finalization;
- Refund Approval;
- Procurement to Inventory and Payable;
- Financial Period Closing;
- Student Termination;
- Correction of Finalized Academic Records.

The scenario type controls the required depth of review, not the number of pages.

---

# 9. Scenario Priority

Every Phase 1 candidate will receive a priority.

## P0 — Mandatory for the First L3 Release

Required before implementation can safely proceed.

Examples:

- student activation;
- fee payment;
- timetable publication;
- attendance finalization;
- examination result publication;
- academic progression;
- payroll;
- purchase receipt;
- inventory entry.

## P1 — Important Operational Scenario

Required for a useful and complete operational baseline.

Examples:

- guardian update;
- attendance correction;
- employee transfer;
- revaluation;
- hostel room change;
- refund processing.

## P2 — Rare, Future, or Low-Frequency Scenario

May remain inventoried and be authored later.

Examples:

- unusual archival restoration;
- rare historical correction;
- optional external-integration exceptions.

There will be no fixed target for the total number of L3 files.

The first release is complete based on coverage and quality, not file count.

---

# 10. Progress Measurement

The team should not use “number of files per day” as the only target.

One critical scenario may require more work than several simple scenarios.

## Proposed Effort Points

| Scenario Work | Points |
|---|---:|
| Type A — Simple Scenario | 1 |
| Type B — Cross-Domain Scenario | 2 |
| Type C — Critical Scenario | 3 |
| Major Correction After Review | +1 |

A suggested daily creator target is **3–4 effort points**, subject to final team approval.

Examples:

```text
3 simple scenarios = 3 points
```

```text
1 cross-domain + 1 simple = 3 points
```

```text
1 critical scenario = 3 points
```

This method is not yet formally locked.

---

# 11. Verification Levels

The Plus account should apply different review depth based on scenario risk.

## Level 1 — Basic Review

For Type A scenarios:

- template completeness;
- clear owner;
- terminology;
- basic failure handling;
- traceability;
- obvious missing steps.

## Level 2 — Full Review

For Type B scenarios:

- all Level 1 checks;
- cross-domain ownership;
- events and hand-offs;
- alternative flows;
- failure and recovery;
- authorization;
- overlap with other scenarios.

## Level 3 — Strict Review

For Type C scenarios:

- all Level 1 and Level 2 checks;
- direct L1/L2 source validation;
- rule ownership;
- state compatibility;
- event compatibility;
- correction and recovery paths;
- segregation of duties;
- audit evidence;
- repository-wide conflict search.

The exact daily Plus-review capacity is still open.

---

# 12. Team Assignment Direction

The exact scenario allocation will be finalized after Phase 1 and Phase 2.

The working responsibility areas are:

## Dhruv

- L3 governance;
- template management;
- platform;
- identity;
- authorization;
- workflow;
- events;
- audit;
- Section H;
- cross-domain consolidation;
- final architecture decisions.

## Pallav

- institution;
- organisation;
- curriculum;
- academic calendar;
- operational readiness;
- academic delivery;
- timetable;
- teaching and attendance.

## Varad

- admissions;
- student lifecycle;
- registration;
- progression;
- qualification;
- workforce;
- HR;
- leave;
- payroll.

## Chetan

- fees;
- payments;
- refunds;
- accounting;
- budgeting;
- procurement;
- inventory;
- assets;
- payables and receivables.

## Devang

- campus services;
- infrastructure;
- hostel;
- transport;
- security;
- housekeeping;
- examination;
- examination exceptions.

For cross-domain scenarios, one person will be the creator and other affected domain owners will be mandatory reviewers.

### Example

| Scenario | Creator | Mandatory Reviewers |
|---|---|---|
| Approved Candidate to Active Student | Varad | Dhruv, Chetan |
| Academic Registration to First Teaching Session | Pallav | Varad, Devang |
| Examination to Result Publication | Devang | Pallav, Varad |
| Procurement to Inventory | Chetan | Devang, Pallav |
| Qualification to Alumni | Varad | Devang, Dhruv |

Final assignments are not yet locked.

---

# 13. No Fixed File Target

The team has agreed not to set a fixed number of L3 files.

This avoids:

- artificial splitting;
- rushing;
- duplicate scenarios;
- weak documents;
- optimizing for quantity instead of architecture quality.

The number of files will emerge naturally after:

- scenario discovery;
- consolidation;
- priority assignment;
- coverage validation.

---

# 14. Coverage-Based Completion

L3 should be considered complete when:

- all P0 scenarios are approved;
- P1 scenarios are approved or explicitly deferred;
- all important L2 workflows are mapped;
- governance-only workflows are marked clearly;
- major institutional lifecycles are covered end to end;
- all critical cross-domain hand-offs are covered;
- no critical ownership conflict remains;
- no incompatible event contract remains;
- no incompatible lifecycle state remains;
- Failure and Recovery is addressed;
- all approved documents pass Plus verification;
- L3 Index and Coverage Matrix are complete;
- remaining P2 scenarios are listed for the next release.

This completion model is approved in principle but the final lock checklist is still to be written.

---

# 15. Items Already Agreed

The following items are considered agreed for the current plan:

- Level 3 will describe complete real-world business scenarios.
- Each file will represent one clear business outcome.
- The target duration is 20 days.
- Phase 0 is currently in progress.
- Five ChatGPT Go accounts will create documents.
- One ChatGPT Plus account will verify documents.
- Verification will happen daily, not only at the end.
- Creators will correct their own reviewed documents.
- Three pilot scenarios are approved.
- Two additional pilots may be added only if needed.
- No fixed target will be set for number of files.
- The template will contain 15 consistent headings.
- “Purpose” is expanded to “Purpose and Scope.”
- Failure and Recovery will normally be mandatory.
- Scenarios will be classified as Simple, Cross-domain, or Critical.
- Scenarios will be prioritized as P0, P1, or P2.
- Documents will be locked only after repository-wide validation.
- Progress will be judged mainly by coverage and quality.

---

# 16. Items Not Yet Locked

The following decisions still require final approval:

- exact instructions and required fields inside every template section;
- final L3 scenario numbering convention;
- final L3 repository folder structure;
- final person-wise scenario assignments;
- final effort-point target;
- final Plus-account review capacity;
- exact verification checklist for each review level;
- final Phase 1 spreadsheet or Markdown format;
- final L3 completion and lock checklist;
- whether the optional fourth and fifth pilots are needed;
- exact daily submission deadline and correction deadline.

Numbering, repository structure, and assignments can be prepared by AI after the template and development rules are locked.

---

# 17. Recommended Next Discussion

Before locking the Level 3 development path, the team should finalize:

1. Exact rules for all 15 template sections.
2. Phase 1 inventory format.
3. Effort-point target and daily deadlines.
4. Plus-account review capacity.
5. Final completion and lock checklist.
6. Rules for blocked scenarios affected by unresolved Phase 0 issues.

After these decisions, AI can generate:

- L3 folder structure;
- numbering convention;
- team assignment matrix;
- Phase 1 inventory template;
- L3 scenario template;
- review checklist;
- daily tracking sheet;
- repository index templates.

---

# 18. Proposed Team Decision Summary

| Decision Area | Current Position | Status |
|---|---|---|
| L3 purpose | Complete business scenarios | Agreed |
| Duration | 20 days | Agreed |
| Creation accounts | Five ChatGPT Go accounts | Agreed |
| Verification account | One ChatGPT Plus account | Agreed |
| Verification model | Daily review and correction | Agreed |
| Pilot count | Three, with two optional | Agreed |
| File-count target | No fixed target | Agreed |
| Template headings | 15 sections | Agreed in principle |
| Failure and Recovery | Normally mandatory | Agreed |
| Scenario classes | Type A, B, C | Agreed in principle |
| Priorities | P0, P1, P2 | Agreed in principle |
| Effort points | Proposed | Not locked |
| Numbering | AI to propose | Not locked |
| Repository structure | AI to propose | Not locked |
| Team assignments | AI to finalize after inventory | Not locked |
| Completion checklist | Coverage-based | Not fully locked |

---

# 19. Final Working Recommendation

The project should not optimize for the maximum number of documents.

It should optimize for:

```text
Clear scenario boundary
+ One canonical owner chain
+ Complete main flow
+ Real failure and recovery
+ Compatible events and states
+ L1/L2 traceability
+ Daily verification
= Reliable Level 3 Architecture
```

The development path is close to being ready for lock.

The next major decision should be the detailed authoring rules for the 15-section template.
