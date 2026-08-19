# SutraERP L3 Scenario Type and Priority Rules

**Document:** L3 Scenario Type and Priority Rules
**Architecture Level:** L3 — Business Scenario Architecture
**Version:** 1.1
**Status:** Approved for Phase 1/2 Classification; Phase 5 Validation Only
**Applies To:** Phase 1 candidate scenario inventory; Phase 2 consolidation; Phase 5 validation of already frozen Type and Priority values
**Last Updated:** 2026-08-19

---

## 1. Purpose

This document defines the allowed scenario type and priority values for SutraERP Level 3 Business Scenario Architecture.

Its classification rules governed Phase 1 candidate classification and Phase 2 consolidation. For Phase 5, the values are already frozen in the Phase 4 Stage 3 canonical scenario semantics and shall not be reclassified by authors.

---

## 2. Scope

This document defines:

* allowed scenario type values;
* allowed priority values;
* when each type was selected during discovery/consolidation;
* when each priority was selected during discovery/consolidation;
* examples of correct classification;
* examples of incorrect classification;
* Phase 5 validation rules for frozen Type and Priority values.

This document does not define final scenario ownership or final L3 numbering.

---

## 3. Allowed Type Values

Only the following values are allowed in the `Type` field:

```text
Simple
Cross-domain
Critical
```

Do not use alternate labels such as:

```text
Basic
Normal
Complex
High Risk
Major
Minor
Medium
```

---

## 4. Type: Simple

Use `Simple` when the candidate scenario is mainly owned by one domain and has limited cross-domain hand-offs.

A Simple scenario should still represent a complete business outcome.

Correct examples:

```text
Guardian Relationship Update
Library Membership Activation
Transport Route Assignment
Leave Request Cancellation
```

These are valid only if each title represents a complete business outcome with a clear trigger and final outcome.

Do not use `Simple` for action fragments such as:

```text
Update Guardian
Cancel Leave
Assign Route
```

These may be workflow actions unless the trigger and final outcome make the complete scenario clear.

---

## 5. Type: Cross-domain

Use `Cross-domain` when the scenario requires collaboration between multiple domains to complete one business outcome.

Correct examples:

```text
Approved Candidate to Active Student
Academic Registration to Course Enrollment Confirmation
Employee Leave Approval to Availability Update
Hostel Allocation with Fee Charge
```

A Cross-domain scenario should have:

* one final business outcome;
* one likely primary owner;
* two or more participating domains;
* clear hand-off points.

Do not use `Cross-domain` only because a notification, audit record, or authorization check exists. Platform services may participate in many scenarios, but their presence alone does not make a scenario cross-domain unless the business outcome depends on more than one business domain.

---

## 6. Type: Critical

Use `Critical` when the scenario is high-risk because it affects one or more of the following:

* official academic outcome;
* student lifecycle status;
* financial settlement;
* payroll or statutory payment;
* examination result;
* qualification completion;
* legal/compliance evidence;
* institutional audit trail;
* resource custody or inventory accuracy;
* irreversible or historically sensitive state transition.

Correct examples:

```text
Regular Examination to Result Publication
Student Fee Payment and Receipt
Qualification Completion and Student Exit
Payroll Finalization
Financial Period Closing
Purchase Requisition to Goods Receipt and Inventory
Student Termination
Finalized Attendance Correction
```

A Critical scenario may be single-domain or cross-domain.

The `Critical` type is based on institutional risk, not the number of participating domains.

---

## 7. Type Selection Rule

During Phase 1/2 classification, choose the type using this order:

```text
1. If the scenario is high-risk, mark it Critical.
2. Else if multiple business domains are required for the final outcome, mark it Cross-domain.
3. Else mark it Simple.
```

Examples:

| Candidate | Correct Type | Reason |
|---|---|---|
| Student Fee Payment and Receipt | Critical | Financial settlement and receipt are audit-sensitive |
| Approved Candidate to Active Student | Cross-domain | Requires admission, identity, finance clearance, and student lifecycle |
| Regular Examination to Result Publication | Critical | Official academic result has high academic and lifecycle impact |
| Guardian Relationship Update | Simple | Mainly student/guardian relationship maintenance |
| Employee Leave Approval to Availability Update | Cross-domain | Leave approval affects workforce availability and academic scheduling/operations |
| Financial Period Closing | Critical | Financial closure is historically and audit-sensitive |

---

## 8. Allowed Priority Values

Only the following values are allowed in the `Priority` field:

```text
P0
P1
P2
```

Do not use alternate values such as:

```text
High
Medium
Low
Urgent
Important
Later
Must Have
Nice to Have
```

---

## 9. Priority: P0

Use `P0` for scenarios mandatory for the first L3 release.

A P0 scenario is required before implementation can safely proceed for the core institutional ERP baseline.

Correct P0 examples:

```text
Approved Candidate to Active Student
Student Fee Payment and Receipt
Timetable Publication
Attendance Capture to Attendance Finalization
Regular Examination to Result Publication
Academic Progression
Payroll Finalization
Purchase Requisition to Goods Receipt and Inventory
```

A scenario should be P0 if missing it would create a major gap in:

* student activation;
* academic delivery;
* attendance;
* examination;
* finance;
* payroll;
* procurement/resource lifecycle;
* institutional readiness.

Do not mark a scenario P0 only because the domain is important.

---

## 10. Priority: P1

Use `P1` for important operational scenarios that are required for a useful and complete institutional operating baseline, but are not mandatory before the first L3 release can begin.

Correct P1 examples:

```text
Guardian Relationship Update
Attendance Correction
Employee Transfer
Revaluation and Revised Result
Hostel Room Change
Refund Processing
Faculty Substitution
Library Clearance
```

A P1 scenario usually supports operational completeness, exception handling, or important secondary flows.

---

## 11. Priority: P2

Use `P2` for rare, future, optional, institution-specific, or low-frequency scenarios.

Correct P2 examples:

```text
Historical Archive Restoration
Rare Finalized Record Correction
Optional External Integration Exception
Special Event-Based Facility Allocation
Legacy Data Reconciliation Scenario
```

P2 does not mean invalid.

It means the scenario may be inventoried now but authored later.

---

## 12. Priority Selection Rule

During Phase 1/2 classification, choose priority using this order:

```text
1. If the first L3 release cannot safely proceed without the scenario, mark it P0.
2. Else if the scenario is important for operational baseline completeness, mark it P1.
3. Else mark it P2.
```

---

## 13. Type and Priority Are Different

Type and priority must not be confused.

| Type | Priority | Example |
|---|---|---|
| Critical | P0 | Regular Examination to Result Publication |
| Critical | P1 | Refund Processing |
| Critical | P2 | Rare Historical Financial Correction |
| Cross-domain | P0 | Approved Candidate to Active Student |
| Cross-domain | P1 | Hostel Allocation with Fee Charge |
| Simple | P1 | Guardian Relationship Update |
| Simple | P2 | Archive Restoration Request |

`Critical` describes risk.

`P0/P1/P2` describes release priority.

---

## 14. Classification Mistakes to Avoid

### Mistake 1 — Marking all cross-domain scenarios as Critical

Incorrect:

```text
Every Cross-domain scenario = Critical
```

Correct:

```text
A scenario is Critical only if the business outcome carries high institutional risk.
```

### Mistake 2 — Marking all Section I or Section F scenarios as Critical

Incorrect:

```text
All Finance scenarios = Critical
All Examination scenarios = Critical
```

Correct:

```text
Some Finance and Examination scenarios are Critical, but classification depends on the specific business outcome.
```

### Mistake 3 — Marking all first-release scenarios as Critical

Incorrect:

```text
P0 = Critical
```

Correct:

```text
P0 means mandatory for the first L3 release.
Critical means high institutional risk.
```

### Mistake 4 — Using vague priorities

Incorrect:

```text
High
Medium
Low
```

Correct:

```text
P0
P1
P2
```

---

## 15. Phase 1 Classification Guidance

During Phase 1, classification was provisional.

Team members made the best reasonable classification based on:

* final outcome;
* participating domains;
* institutional risk;
* first-release necessity.

The Lead Architect / review process could correct Type and Priority during Phase 2 consolidation.

---

## 16. Phase 1/2 Classification Lock Statement

The following values were the only allowed classification values:

```text
Allowed Type values:
Simple
Cross-domain
Critical

Allowed Priority values:
P0
P1
P2
```

Type and Priority were provisional during Phase 1 and confirmed through consolidation/canonical-set construction.

---

## 17. Phase 5 Frozen-Value Rule

Phase 5 is an authoring and verification phase, not a classification phase.

For every final L3 scenario:

- `Scenario Type` must be copied unchanged from the frozen Phase 4 Stage 3 canonical scenario semantics;
- `Priority` must be copied unchanged from the frozen Phase 4 Stage 3 canonical scenario semantics;
- authors must not rerun the Type Selection Rule or Priority Selection Rule;
- authors must not change Type or Priority based on personal judgment, document length, perceived complexity, author workload, or implementation preference;
- any suspected classification defect must be escalated through architecture change control rather than silently corrected inside an L3 scenario document.

The locked Phase 5 authorization decision `../05_Phase5_Parallel_Authoring_and_Verification/L3_Phase5_Parallel_Authoring_Authorization_2026-08-19.md` governs this use.

---

## 18. Phase 5 Lock Statement

```text
Phase 5 Type and Priority are frozen inputs.
Authors copy them from the frozen Stage 3 canonical semantics.
Phase 5 authors do not reclassify scenarios.
Any change requires explicit architecture change control.
```
