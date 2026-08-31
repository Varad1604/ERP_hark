# L4-C-024 — Backlog and Repeat Course Registration

**Architecture Level:** L4 — System Behavior Specification  
**L3 Parent:** `L3-C-024` — Backlog and Repeat Course Registration  
**Primary Owner:** Academic Registration  
**Participating Domains:** C, D, I  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Author:** Varad

## 1. Purpose
Specify system behavior for permitted repeat/backlog course selection and registration within the canonical Academic Registration lifecycle.

## 2. Preconditions and Trigger
An eligible Student is permitted under academic policy to register a backlog/repeat course during an active registration period. Student, academic context, offering and relevant examination evidence are authoritative inputs.

## 3. Main Behavior
1. Establish Student and registration context.
2. Confirm the registration window and eligibility conditions.
3. Identify the permitted repeat/backlog offering through Academic Offering Selection.
4. Validate programme/curriculum applicability, prerequisites/co-requisites, duplicate constraints and credit limits.
5. Use authoritative examination information where required; do not recreate examination-side determination.
6. Record the selected repeat/backlog offering within Academic Registration.
7. Revalidate the resulting registration.
8. Obtain configured approval where required.
9. Publish only canonical registration events supported by L2-C03, including `CourseRegistered`/`RegistrationSubmitted`/`StudentRegistrationApproved` where their conditions are met.
10. Audit the selection and resulting registration.

## 4. Alternate Behavior
- Examination-side eligibility unavailable: defer rather than infer.
- Academic rule fails: do not establish the invalid selection.
- Duplicate or credit violation: reject the violating selection.
- Approval rejected: do not establish approval-dependent completion.

## 5. Ownership
Academic Registration owns the registration outcome. Student Lifecycle, Academic Delivery and Examination retain ownership of their respective facts and consequences.

## 6. Lifecycle and Events
Repeat/backlog selection remains within the normal C03 registration lifecycle. No special lifecycle state is invented. Canonical event publisher remains Academic Registration.

## 7. Authorization and Audit
Configured registration authority applies. Evidence must show the relevant Student context, selected offering, validation results, approval where required, and resulting registration outcome.

## 8. Business Failure Handling
Closed window, ineligibility, prerequisite/co-requisite failure, duplicate registration, credit-limit violation, missing configuration or absent authority prevents successful completion until resolved through the owning process.

## 9. Traceability
L3-C-024; L2-C03 Academic Registration workflows; dependencies on authoritative Examination context and downstream Academic Delivery consumption.

## 10. Open Issues
L2-C03 supports repeat-course selection but does not provide a complete backlog-specific examination decision matrix. L4 does not invent one. Detailed cross-domain contract semantics remain for later implementation architecture.

**Classification: PASS WITH ISSUES — Draft**
