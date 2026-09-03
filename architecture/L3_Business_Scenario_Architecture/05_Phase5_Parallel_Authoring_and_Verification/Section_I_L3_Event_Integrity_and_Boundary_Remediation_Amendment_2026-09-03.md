# Section I L3 Event Integrity and Boundary Remediation Amendment — Proposed

**Architecture Level:** L3 — Business Scenario Architecture  
**Change Type:** Controlled amendment to Locked L3 corpus  
**Status:** Review  
**Date:** 2026-09-03  
**Effective State:** Not effective until independent architecture approval and governed successor-lock authorization  
**Authority Basis:** `L3_Final_Corpus_Lock_Decision_1084_2026-08-29.md` §5 Change-Control Rule After Lock  

## 1. Finding

A subset of Locked Section I scenarios contains event-table rows that mix canonical business events with workflow, capability, configuration, session-type, policy, source-document, or activity labels. Downstream L4 authoring identified the issue but referenced an upstream amendment that did not yet exist.

## 2. Scope

This proposed amendment is limited to the following **48** Locked parent scenarios:

L3-I-005, L3-I-006, L3-I-008, L3-I-009, L3-I-010, L3-I-011, L3-I-012, L3-I-013, L3-I-014, L3-I-015, L3-I-016, L3-I-043, L3-I-044, L3-I-045, L3-I-046, L3-I-047, L3-I-048, L3-I-049, L3-I-050, L3-I-051, L3-I-052, L3-I-053, L3-I-054, L3-I-072, L3-I-073, L3-I-074, L3-I-075, L3-I-076, L3-I-077, L3-I-078, L3-I-079, L3-I-080, L3-I-081, L3-I-082, L3-I-119, L3-I-120, L3-I-121, L3-I-122, L3-I-123, L3-I-124, L3-I-125, L3-I-126, L3-I-127, L3-I-128, L3-I-129, L3-I-130, L3-I-131, L3-I-132

Unaffected Section I scenarios remain frozen and are not reopened. The amendment does not change scenario IDs, canonical owners, business outcomes, aggregate ownership, business rules, or lifecycle definitions except where a separately governed disposition is explicitly required.

## 3. Impact Assessment

- **L3 impact:** 48 Locked Section I parents contain one or more §14 rows whose labels are not canonical business events in their cited Locked L2 source. Scenario identity, owner, outcome, rules and lifecycle remain unchanged by this event-integrity correction.
- **L4 impact:** the 48 directly dependent Pallav L4-I specifications remain `Review` / **BLOCKED BY UPSTREAM** until this amendment is approved and the corresponding L3 successor interpretation is established.
- **Repository impact:** no corpus renumbering, owner transfer, new event, new publisher, lifecycle invention, or change to the 1,084-scenario corpus size is authorized.
- **Special impact — L3-I-131:** L2-I11 does not support the reopening premise; this scenario requires a separate governed disposition and cannot be cured by event-table cleanup alone.

## 4. Governing Event-Integrity Correction

For the scoped parents, §14 event rows are corrected by exact-name validation against the `Events Published` and `Events Consumed` sections of the cited Locked L2 source. A current row whose event label exactly matches a canonical L2 event is retained. A row that does not exactly match is removed from event classification; its underlying phrase may remain only as non-event business context where already supported elsewhere by the parent or L2.

This amendment does **not** infer a replacement event merely because an unsupported row appears semantically similar to a valid L2 event. Any additional event needed by a scenario must already be demonstrably inside the existing scenario boundary and must use the exact canonical L2 name and publisher.

Canonical publisher ownership remains exactly as defined by Locked L2. No event is renamed, created, re-owned, or republished.

## 5. Exact Per-Scenario Event Correction Register

| L3 Parent | L2 Source | Direction | Current §14 Row | Disposition |
|---|---|---|---|---|
| L3-I-005 | L2-I02 | Published | `Examination Session Submitted` | **KEEP** |
| L3-I-005 | L2-I02 | Published | `L2-I02` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-005 | L2-I02 | Published | `Examination Session Created` | **KEEP** |
| L3-I-005 | L2-I02 | Published | `Examination Session Updated` | **KEEP** |
| L3-I-005 | L2-I02 | Consumed | `L2-I02` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-005 | L2-I02 | Consumed | `Regular Examination Session` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-005 | L2-I02 | Consumed | `Supplementary Examination Session` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-005 | L2-I02 | Consumed | `Improvement Examination Session` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-006 | L2-I02 | Published | `Examination Session Created` | **KEEP** |
| L3-I-006 | L2-I02 | Published | `Examination Session Updated` | **KEEP** |
| L3-I-006 | L2-I02 | Published | `Examination Session Submitted` | **KEEP** |
| L3-I-006 | L2-I02 | Published | `Examination Session Approved` | **KEEP** |
| L3-I-006 | L2-I02 | Consumed | `Regular Examination Session` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-006 | L2-I02 | Consumed | `Supplementary Examination Session` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-006 | L2-I02 | Consumed | `Improvement Examination Session` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-006 | L2-I02 | Consumed | `Makeup Examination Session` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-008 | L2-I03 | Published | `Examination Schedule Created` | **KEEP** |
| L3-I-008 | L2-I03 | Published | `Schedule Updated` | **KEEP** |
| L3-I-008 | L2-I03 | Published | `Schedule Revised` | **KEEP** |
| L3-I-008 | L2-I03 | Published | `Schedule Cancelled` | **KEEP** |
| L3-I-008 | L2-I03 | Consumed | `Examination Policy Updated` | **KEEP** |
| L3-I-008 | L2-I03 | Consumed | `Schedule Revision Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-008 | L2-I03 | Consumed | `Schedule Cancellation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-008 | L2-I03 | Consumed | `Schedule Publication` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-009 | L2-I03 | Published | `Venue Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-009 | L2-I03 | Published | `Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-009 | L2-I03 | Published | `L2-I04` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-009 | L2-I03 | Published | `Examination Planning Initiated` | **KEEP** |
| L3-I-009 | L2-I03 | Consumed | `Venue Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-009 | L2-I03 | Consumed | `Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-009 | L2-I03 | Consumed | `L2-I04` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-009 | L2-I03 | Consumed | `Examination Session Activated` | **KEEP** |
| L3-I-010 | L2-I03 | Published | `Examination Schedule Created` | **KEEP** |
| L3-I-010 | L2-I03 | Published | `Schedule Cancelled` | **KEEP** |
| L3-I-010 | L2-I03 | Published | `Schedule Cancellation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-010 | L2-I03 | Published | `Schedule Publication` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-010 | L2-I03 | Consumed | `Schedule Cancellation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-010 | L2-I03 | Consumed | `Schedule Publication` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-010 | L2-I03 | Consumed | `Active` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-010 | L2-I03 | Consumed | `Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-011 | L2-I03 | Published | `Examination Schedule Created` | **KEEP** |
| L3-I-011 | L2-I03 | Published | `Schedule Cancelled` | **KEEP** |
| L3-I-011 | L2-I03 | Published | `Schedule Cancellation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-011 | L2-I03 | Published | `Schedule Publication` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-011 | L2-I03 | Consumed | `Schedule Cancellation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-011 | L2-I03 | Consumed | `Schedule Publication` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-011 | L2-I03 | Consumed | `Active` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-011 | L2-I03 | Consumed | `Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-012 | L2-I03 | Published | `Practical & Viva Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-012 | L2-I03 | Published | `Practical & Viva Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-012 | L2-I03 | Published | `L2-I06` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-012 | L2-I03 | Published | `Examination Planning Initiated` | **KEEP** |
| L3-I-012 | L2-I03 | Consumed | `Practical & Viva Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-012 | L2-I03 | Consumed | `Practical & Viva Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-012 | L2-I03 | Consumed | `L2-I06` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-012 | L2-I03 | Consumed | `Examination Session Activated` | **KEEP** |
| L3-I-013 | L2-I03 | Published | `Schedule Published` | **KEEP** |
| L3-I-013 | L2-I03 | Published | `Published` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-013 | L2-I03 | Published | `Examination Timetable` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-013 | L2-I03 | Published | `Schedule Revision Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-013 | L2-I03 | Consumed | `Published` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-013 | L2-I03 | Consumed | `Examination Timetable` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-013 | L2-I03 | Consumed | `Schedule Revision Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-013 | L2-I03 | Consumed | `Schedule Publication` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-014 | L2-I03 | Published | `L2-I03` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-014 | L2-I03 | Published | `Examination Planning Initiated` | **KEEP** |
| L3-I-014 | L2-I03 | Published | `Examination Schedule Created` | **KEEP** |
| L3-I-014 | L2-I03 | Published | `Schedule Cancelled` | **KEEP** |
| L3-I-014 | L2-I03 | Consumed | `L2-I03` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-014 | L2-I03 | Consumed | `Examination Session Activated` | **KEEP** |
| L3-I-014 | L2-I03 | Consumed | `Examination Policy Updated` | **KEEP** |
| L3-I-014 | L2-I03 | Consumed | `Examination Policy Governance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Published | `Venue Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Published | `Venue Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Published | `Room Allocation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Published | `Venue Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Consumed | `Venue Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Consumed | `Venue Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Consumed | `Room Allocation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-015 | L2-I03 | Consumed | `Venue Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-016 | L2-I03 | Published | `Schedule Published` | **KEEP** |
| L3-I-016 | L2-I03 | Published | `Timetable Structure` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-016 | L2-I03 | Published | `Examination Timetable` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-016 | L2-I03 | Published | `Schedule Revision Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-016 | L2-I03 | Consumed | `Timetable Structure` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-016 | L2-I03 | Consumed | `Examination Timetable` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-016 | L2-I03 | Consumed | `Schedule Revision Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-016 | L2-I03 | Consumed | `Schedule Cancellation Plan` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-043 | L2-I06 | Published | `L2-I08 Marks Collection & Evaluation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-043 | L2-I06 | Published | `Assessment Prepared` | **KEEP** |
| L3-I-043 | L2-I06 | Published | `Assessment Completed` | **KEEP** |
| L3-I-043 | L2-I06 | Published | `Assessment Attendance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-043 | L2-I06 | Consumed | `L2-I08 Marks Collection & Evaluation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-043 | L2-I06 | Consumed | `Assessment Attendance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-043 | L2-I06 | Consumed | `Assessment Completion` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-043 | L2-I06 | Consumed | `Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-044 | L2-I06 | Published | `Assessment Prepared` | **KEEP** |
| L3-I-044 | L2-I06 | Published | `Assessment Completed` | **KEEP** |
| L3-I-044 | L2-I06 | Published | `Assessment Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-044 | L2-I06 | Published | `Assessment Completion` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-044 | L2-I06 | Consumed | `Assessment Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-044 | L2-I06 | Consumed | `Assessment Completion` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-044 | L2-I06 | Consumed | `Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-044 | L2-I06 | Consumed | `L2-I08 Marks Collection & Evaluation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-045 | L2-I06 | Published | `Assessment Attendance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-045 | L2-I06 | Published | `Attendance Recording` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-045 | L2-I06 | Published | `Attendance Recorded` | **KEEP** |
| L3-I-045 | L2-I06 | Published | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-045 | L2-I06 | Consumed | `Assessment Attendance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-045 | L2-I06 | Consumed | `Attendance Recording` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-045 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-045 | L2-I06 | Consumed | `Candidate Check-in` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-046 | L2-I06 | Published | `Candidate Information Updated` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-046 | L2-I06 | Published | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-046 | L2-I06 | Published | `Identity Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-046 | L2-I06 | Published | `Candidate Check-in` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-046 | L2-I06 | Consumed | `Candidate Information Updated` | **KEEP** |
| L3-I-046 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-046 | L2-I06 | Consumed | `Identity Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-046 | L2-I06 | Consumed | `Candidate Check-in` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-047 | L2-I06 | Published | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-047 | L2-I06 | Published | `L2-I07` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-047 | L2-I06 | Published | `L2-I07 Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-047 | L2-I06 | Published | `Assessment Prepared` | **KEEP** |
| L3-I-047 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-047 | L2-I06 | Consumed | `L2-I07` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-047 | L2-I06 | Consumed | `L2-I07 Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-047 | L2-I06 | Consumed | `Assessment Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-048 | L2-I06 | Published | `Hall Ticket Validated` | **KEEP** |
| L3-I-048 | L2-I06 | Published | `Hall Ticket Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-048 | L2-I06 | Published | `**L2-I06**` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-048 | L2-I06 | Published | `Assessment Prepared` | **KEEP** |
| L3-I-048 | L2-I06 | Consumed | `Hall Ticket Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-048 | L2-I06 | Consumed | `**L2-I06**` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-048 | L2-I06 | Consumed | `Examination Policy Updated` | **KEEP** |
| L3-I-048 | L2-I06 | Consumed | `Examination Session Activated` | **KEEP** |
| L3-I-049 | L2-I06 | Published | `Batch Configuration` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-049 | L2-I06 | Published | `Examiner Assigned` | **KEEP** |
| L3-I-049 | L2-I06 | Published | `Resource Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-049 | L2-I06 | Published | `Examiner Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-049 | L2-I06 | Consumed | `Batch Configuration` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-049 | L2-I06 | Consumed | `Resource Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-049 | L2-I06 | Consumed | `Examiner Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-049 | L2-I06 | Consumed | `Venue Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-050 | L2-I06 | Published | `Assessment Prepared` | **KEEP** |
| L3-I-050 | L2-I06 | Published | `Hall Ticket Validated` | **KEEP** |
| L3-I-050 | L2-I06 | Published | `Conduct Verified` | **KEEP** |
| L3-I-050 | L2-I06 | Published | `Conduct Closed` | **KEEP** |
| L3-I-050 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-050 | L2-I06 | Consumed | `Assessment Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-050 | L2-I06 | Consumed | `Examiner Allocation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-050 | L2-I06 | Consumed | `**L2-I06**` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-051 | L2-I06 | Published | `Hall Ticket Validated` | **KEEP** |
| L3-I-051 | L2-I06 | Published | `Conduct Verified` | **KEEP** |
| L3-I-051 | L2-I06 | Published | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-051 | L2-I06 | Published | `Assessment Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-051 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-051 | L2-I06 | Consumed | `Assessment Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-051 | L2-I06 | Consumed | `**L2-I06**` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-051 | L2-I06 | Consumed | `L2-I07` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-052 | L2-I06 | Published | `Hall Ticket Validated` | **KEEP** |
| L3-I-052 | L2-I06 | Published | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-052 | L2-I06 | Published | `Hall Ticket Validation During Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-052 | L2-I06 | Published | `L2-I07` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-052 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-052 | L2-I06 | Consumed | `Hall Ticket Validation During Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-052 | L2-I06 | Consumed | `L2-I07` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-052 | L2-I06 | Consumed | `L2-I07 Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-053 | L2-I06 | Published | `Hall Ticket Validated` | **KEEP** |
| L3-I-053 | L2-I06 | Published | `Attendance Recorded` | **KEEP** |
| L3-I-053 | L2-I06 | Published | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-053 | L2-I06 | Published | `Candidate Check-in` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-053 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-053 | L2-I06 | Consumed | `Candidate Check-in` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-053 | L2-I06 | Consumed | `Assessment Attendance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-053 | L2-I06 | Consumed | `Attendance Recording` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-054 | L2-I06 | Published | `Archived` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-054 | L2-I06 | Published | `Attendance Recorded` | **KEEP** |
| L3-I-054 | L2-I06 | Published | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-054 | L2-I06 | Published | `Assessment Attendance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-054 | L2-I06 | Consumed | `Archived` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-054 | L2-I06 | Consumed | `Theory Examination Conduct` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-054 | L2-I06 | Consumed | `Assessment Attendance` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-054 | L2-I06 | Consumed | `Attendance Recording` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-072 | L2-I08 | Published | `Final Approved Marks Available` | **KEEP** |
| L3-I-072 | L2-I08 | Published | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-072 | L2-I08 | Published | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-072 | L2-I08 | Published | `Finalised` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-072 | L2-I08 | Consumed | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-072 | L2-I08 | Consumed | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-072 | L2-I08 | Consumed | `Finalised` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-072 | L2-I08 | Consumed | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-073 | L2-I08 | Published | `Assessment Evidence Received` | **KEEP** |
| L3-I-073 | L2-I08 | Published | `Evaluation Started` | **KEEP** |
| L3-I-073 | L2-I08 | Published | `Evaluation Completed` | **KEEP** |
| L3-I-073 | L2-I08 | Published | `Evaluation Closed` | **KEEP** |
| L3-I-073 | L2-I08 | Consumed | `Revaluation & Rechecking` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-073 | L2-I08 | Consumed | `Evaluation Model` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-073 | L2-I08 | Consumed | `Evaluation Method` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-073 | L2-I08 | Consumed | `Anonymous Evaluation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-074 | L2-I08 | Published | `Marks Submitted` | **KEEP** |
| L3-I-074 | L2-I08 | Published | `Marks Validated` | **KEEP** |
| L3-I-074 | L2-I08 | Published | `Marks Approved` | **KEEP** |
| L3-I-074 | L2-I08 | Published | `Final Approved Marks Available` | **KEEP** |
| L3-I-074 | L2-I08 | Consumed | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-074 | L2-I08 | Consumed | `Marks Collection` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-074 | L2-I08 | Consumed | `Marks Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-074 | L2-I08 | Consumed | `Marks Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-075 | L2-I08 | Published | `Marks Approved` | **KEEP** |
| L3-I-075 | L2-I08 | Published | `Final Approved Marks Available` | **KEEP** |
| L3-I-075 | L2-I08 | Published | `Examination Policy Updated` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-075 | L2-I08 | Published | `Examination Policy Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-075 | L2-I08 | Consumed | `Examination Policy Updated` | **KEEP** |
| L3-I-075 | L2-I08 | Consumed | `Examination Policy Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-075 | L2-I08 | Consumed | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-075 | L2-I08 | Consumed | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-076 | L2-I08 | Published | `Marks Validated` | **KEEP** |
| L3-I-076 | L2-I08 | Published | `Marks Approved` | **KEEP** |
| L3-I-076 | L2-I08 | Published | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-076 | L2-I08 | Published | `Marks Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-076 | L2-I08 | Consumed | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-076 | L2-I08 | Consumed | `Marks Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-076 | L2-I08 | Consumed | `Marks Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-076 | L2-I08 | Consumed | `Marks Collection` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-077 | L2-I08 | Published | `Supplementary / Backlog / Improvement Examinations` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-077 | L2-I08 | Published | `L2-I12` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-077 | L2-I08 | Published | `Marks Submitted` | **KEEP** |
| L3-I-077 | L2-I08 | Published | `Marks Validated` | **KEEP** |
| L3-I-077 | L2-I08 | Consumed | `Supplementary / Backlog / Improvement Examinations` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-077 | L2-I08 | Consumed | `L2-I12` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-077 | L2-I08 | Consumed | `Examination Policy Updated` | **KEEP** |
| L3-I-077 | L2-I08 | Consumed | `Examination Session Activated` | **KEEP** |
| L3-I-078 | L2-I08 | Published | `Evaluation Completed` | **KEEP** |
| L3-I-078 | L2-I08 | Published | `Evaluation Closed` | **KEEP** |
| L3-I-078 | L2-I08 | Published | `Practical Examination Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-078 | L2-I08 | Published | `Theory Examination Completed` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-078 | L2-I08 | Consumed | `Practical Examination Completed` | **KEEP** |
| L3-I-078 | L2-I08 | Consumed | `Theory Examination Completed` | **KEEP** |
| L3-I-078 | L2-I08 | Consumed | `Answer Script Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-078 | L2-I08 | Consumed | `Evaluation Planning` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-079 | L2-I08 | Published | `Marks Approved` | **KEEP** |
| L3-I-079 | L2-I08 | Published | `Final Approved Marks Available` | **KEEP** |
| L3-I-079 | L2-I08 | Published | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-079 | L2-I08 | Published | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-079 | L2-I08 | Consumed | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-079 | L2-I08 | Consumed | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-079 | L2-I08 | Consumed | `Finalised` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-079 | L2-I08 | Consumed | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-080 | L2-I08 | Published | `Assessment Evidence Received` | **KEEP** |
| L3-I-080 | L2-I08 | Published | `Evaluation Started` | **KEEP** |
| L3-I-080 | L2-I08 | Published | `Evaluation Completed` | **KEEP** |
| L3-I-080 | L2-I08 | Published | `Evaluation Closed` | **KEEP** |
| L3-I-080 | L2-I08 | Consumed | `Revaluation & Rechecking` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-080 | L2-I08 | Consumed | `Evaluation Model` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-080 | L2-I08 | Consumed | `Evaluation Method` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-080 | L2-I08 | Consumed | `Anonymous Evaluation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Published | `Marks Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Published | `Marks Collection` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Published | `Evaluated` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Published | `**L2-I08**` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Consumed | `Marks Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Consumed | `Marks Collection` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Consumed | `Evaluated` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-081 | L2-I08 | Consumed | `**L2-I08**` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-082 | L2-I08 | Published | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-082 | L2-I08 | Published | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-082 | L2-I08 | Published | `Final Approved Marks Available` | **KEEP** |
| L3-I-082 | L2-I08 | Published | `Marks Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-082 | L2-I08 | Consumed | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-082 | L2-I08 | Consumed | `Marks Finalisation Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-082 | L2-I08 | Consumed | `Marks Validation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-082 | L2-I08 | Consumed | `Marks Finalisation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Published | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Published | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Published | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Consumed | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Consumed | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-119 | L2-I11 | Consumed | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Published | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Published | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Consumed | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-120 | L2-I11 | Consumed | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Published | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Published | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Published | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Consumed | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Consumed | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-121 | L2-I11 | Consumed | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Published | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Published | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Consumed | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-122 | L2-I11 | Consumed | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Published | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Published | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Published | `Rechecking Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Published | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Consumed | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Consumed | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Consumed | `Rechecking Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-123 | L2-I11 | Consumed | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Published | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Published | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Consumed | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-124 | L2-I11 | Consumed | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Published | `Fee Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Published | `Notification Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Consumed | `Fee Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Consumed | `Notification Policy` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-125 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Published | `Revised Marks Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Published | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Consumed | `Revised Marks Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-126 | L2-I11 | Consumed | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Published | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Published | `L2-I09 — Result Processing & Computation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Consumed | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-127 | L2-I11 | Consumed | `L2-I09 — Result Processing & Computation` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Published | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Published | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Published | `Review Request Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Consumed | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Consumed | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-128 | L2-I11 | Consumed | `Review Request Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Published | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Published | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Published | `Review Request Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Consumed | `Rechecking Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Consumed | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-129 | L2-I11 | Consumed | `Review Request Approval` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Published | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Published | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Published | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Consumed | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Consumed | `Review Types` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Consumed | `Rechecking Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-130 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Published | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Published | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Consumed | `Review Eligibility Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-131 | L2-I11 | Consumed | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Published | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Published | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Published | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Published | `Review Outcome Verification` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Consumed | `Revaluation Eligibility` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Consumed | `Revaluation Request Management` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Consumed | `Revaluation Workflow` | **REMOVE FROM EVENT CLASSIFICATION** |
| L3-I-132 | L2-I11 | Consumed | `Review Outcome Verification` | **REMOVE FROM EVENT CLASSIFICATION** |

### 5.1 Correction semantics

- `KEEP` means the existing row label is an exact canonical event name in the cited Locked L2 event vocabulary. Publisher and direction must still remain consistent with L2.
- `REMOVE FROM EVENT CLASSIFICATION` means the row is not a canonical event name in that L2 source. It shall not be published or consumed as an event by L3/L4.
- This register is exhaustive for the current §14 Published/Consumed rows of the 48 scoped parent files at this amendment baseline.

## 6. Affected-Scenario Revalidation

Before this amendment can become effective, every scoped parent must pass revalidation confirming: (1) all remaining event rows are exact source-supported events; (2) publisher ownership matches L2; (3) no unsupported row remains classified as an event; (4) scenario start/end boundaries and final outcome are unchanged; and (5) no rule, state, authority, contract or ownership has drifted.

The revalidation result must be recorded as part of the governed amendment approval/successor decision. A generic package approval without the 48-scenario validation result is insufficient.

## 7. Successor-State and Versioning Decision

If approved, each of the 47 event-integrity-only parents other than `L3-I-131` advances to a **corrected Locked successor interpretation** under this amendment while retaining its existing canonical ID, title, owner, outcome and corpus position. The amendment is the authoritative overlay for corrected §14 event classification.

The physical Locked parent files are preserved as historical evidence and their local version metadata is not silently rewritten by this proposal. The governed approval record must identify this amendment version as the successor correction baseline. If repository governance requires physical parent version increments, that is a separate mechanical synchronization after approval without semantic expansion.

`L3-I-131` does **not** receive a valid successor state from this event-integrity amendment. It remains unresolved pending §9.1.

## 8. Locked-Parent Preservation

The original Locked L3 scenario files are not directly edited by this proposal. Upon governed approval, this amendment becomes the controlled corrective overlay for the explicitly enumerated event rows only.

## 9. Downstream L4 Effect and Special Upstream Dispositions

Until this amendment is approved, materially dependent L4 specifications remain in `Review` and are **BLOCKED BY UPSTREAM** for final architecture approval. After approval, each affected L4 specification must be revalidated against its Locked parent, this amendment, and the cited Locked L2 source before any L4 approval decision.

### 9.1 L3-I-131 — Revaluation Reopening

Current Locked `L2-I11` does not establish a review-reopening workflow, reopening lifecycle transition, reopening authority, or `Review Reopened` event sufficient to support the `L3-I-131` reopening premise. This amendment therefore does not manufacture behavior or declare that parent resolved.

A separate governed disposition is required: either (1) retire/merge `L3-I-131` through explicit corpus/index/parent-baseline change control, or (2) approve a separately governed upstream L2 change establishing the required reopening lifecycle/authority/event semantics and then revalidate L3-I-131. Until then, `L4-I-131` remains `Review` / **BLOCKED BY UPSTREAM**.

## 10. Approval / Lock Gate

This document remains a **proposed controlled remediation**. It does not assert Architecture Approval. Approval requires an independent architecture review confirming the exact correction register, impact assessment, 48-parent revalidation requirements, and successor-state treatment against the Locked L1/L2/L3 governance baseline.

Only after that governed approval may the resolved parents be treated as corrected Locked successors for downstream L4 revalidation. Final L4 approval and final L4 corpus lock remain separate decisions.

