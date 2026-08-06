# SutraERP L3 Scenario Type and Priority Rules

**Document:** L3 Scenario Type and Priority Rules
**Architecture Level:** L3 — Business Scenario Architecture
**Version:** 1.0
**Status:** Approved for Phase 1 Use
**Applies To:** Phase 1 candidate scenario inventory and Phase 2 consolidation

---

## 1. Purpose

This document defines the allowed scenario type and priority values for SutraERP Level 3 Business Scenario Architecture.

The purpose is to ensure that all Phase 1 candidate scenarios are classified consistently before review and consolidation.

---

## 2. Scope

This document defines:

* allowed scenario type values;
* allowed priority values;
* when to use each type;
* when to use each priority;
* examples of correct classification;
* examples of incorrect classification.

This document does not define final scenario ownership or final L3 numbering.

---

## 3. Allowed Type Values

Only the following values are allowed in the `Type` field:

```text id="b4c68g"
Simple
Cross-domain
Critical
```

Do not use alternate labels such as:

```text id="xjy9m8"
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

```text id="qjxj3v"
Guardian Relationship Update
Library Membership Activation
Transport Route Assignment
Leave Request Cancellation
```

These are valid only if each title represents a complete business outcome with a clear trigger and final outcome.

Do not use `Simple` for action fragments such as:

```text id="v2joda"
Update Guardian
Cancel Leave
Assign Route
```

These may be workflow actions unless the trigger and final outcome make the complete scenario clear.

---

## 5. Type: Cross-domain

Use `Cross-domain` when the scenario requires collaboration between multiple domains to complete one business outcome.

Correct examples:

```text id="l53et9"
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

```text id="s1hhbx"
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

Choose the type using this order:

```text id="7evdck"
1. If the scenario is high-risk, mark it Critical.
2. Else if multiple business domains are required for the final outcome, mark it Cross-domain.
3. Else mark it Simple.
```

Examples:

| Candidate                                      | Correct Type | Reason                                                                           |
| ---------------------------------------------- | ------------ | -------------------------------------------------------------------------------- |
| Student Fee Payment and Receipt                | Critical     | Financial settlement and receipt are audit-sensitive                             |
| Approved Candidate to Active Student           | Cross-domain | Requires admission, identity, finance clearance, and student lifecycle           |
| Regular Examination to Result Publication      | Critical     | Official academic result has high academic and lifecycle impact                  |
| Guardian Relationship Update                   | Simple       | Mainly student/guardian relationship maintenance                                 |
| Employee Leave Approval to Availability Update | Cross-domain | Leave approval affects workforce availability and academic scheduling/operations |
| Financial Period Closing                       | Critical     | Financial closure is historically and audit-sensitive                            |

---

## 8. Allowed Priority Values

Only the following values are allowed in the `Priority` field:

```text id="dvv36h"
P0
P1
P2
```

Do not use alternate values such as:

```text id="iu6m6j"
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

```text id="ezq5ws"
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

```text id="o835lh"
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

```text id="nmgjz7"
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

Choose priority using this order:

```text id="ldslra"
1. If the first L3 release cannot safely proceed without the scenario, mark it P0.
2. Else if the scenario is important for operational baseline completeness, mark it P1.
3. Else mark it P2.
```

---

## 13. Type and Priority Are Different

Type and priority must not be confused.

A scenario can be:

| Type         | Priority | Example                                   |
| ------------ | -------- | ----------------------------------------- |
| Critical     | P0       | Regular Examination to Result Publication |
| Critical     | P1       | Refund Processing                         |
| Critical     | P2       | Rare Historical Financial Correction      |
| Cross-domain | P0       | Approved Candidate to Active Student      |
| Cross-domain | P1       | Hostel Allocation with Fee Charge         |
| Simple       | P1       | Guardian Relationship Update              |
| Simple       | P2       | Archive Restoration Request               |

`Critical` describes risk.

`P0/P1/P2` describes release priority.

---

## 14. Classification Mistakes to Avoid

### Mistake 1 — Marking all cross-domain scenarios as Critical

Incorrect:

```text id="oi5va1"
Every Cross-domain scenario = Critical
```

Correct:

```text id="rplnu8"
A scenario is Critical only if the business outcome carries high institutional risk.
```

### Mistake 2 — Marking all Section I or Section F scenarios as Critical

Incorrect:

```text id="sosc7m"
All Finance scenarios = Critical
All Examination scenarios = Critical
```

Correct:

```text id="q2j52s"
Some Finance and Examination scenarios are Critical, but classification depends on the specific business outcome.
```

### Mistake 3 — Marking all first-release scenarios as Critical

Incorrect:

```text id="ccce03"
P0 = Critical
```

Correct:

```text id="e8y1mp"
P0 means mandatory for the first L3 release.
Critical means high institutional risk.
```

### Mistake 4 — Using vague priorities

Incorrect:

```text id="y5sl3b"
High
Medium
Low
```

Correct:

```text id="2uzfk5"
P0
P1
P2
```

---

## 15. Phase 1 Classification Guidance

During Phase 1, classification is provisional.

Team members should make the best reasonable classification based on:

* final outcome;
* participating domains;
* institutional risk;
* first-release necessity.

The Lead Architect / Plus review may correct Type and Priority during Phase 2 consolidation.

---

## 16. Lock Statement

The following values are locked for Phase 1:

```text id="edmt01"
Allowed Type values:
Simple
Cross-domain
Critical

Allowed Priority values:
P0
P1
P2

Type and Priority are provisional during Phase 1 and are confirmed during Phase 2 consolidation.
```
