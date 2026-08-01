# L2-D07 - Internal Assessment Workflows

**Repository:** SutraERP  
**Section:** D - Academic Delivery  
**Domain:** D07 - Internal Assessment  
**Document ID:** L2-D07  
**Version:** 1.0  
**Status:** Architecture Locked  
**Classification:** Level 2 Domain Architecture  
**Owner:** Pallav  
**Dependencies:** Section B, L2-D01, L2-D04, L2-I01, Student Lifecycle reference data

---

# 1. Purpose

The Internal Assessment domain owns the institution-controlled assessment activities that occur during academic delivery before final examination computation.

It governs the canonical lifecycle of internal and continuous assessment components, including assessment blueprinting, assessment window control, submission capture, moderation, finalization, and archival of assessment evidence.

The domain exists so that internal assessment remains an explicit academic delivery capability instead of being folded into examination conduct or finance operations.

---

# 2. Scope

This domain includes:

- Internal assessment blueprint definition
- Continuous assessment component definition
- Assessment window opening and closure
- Submission capture
- Mark entry and correction
- Moderation and approval
- Finalization of internal assessment marks
- Archival of internal assessment evidence

This domain excludes:

- Course definition
- Course offering creation
- Timetable generation
- Teaching session execution
- Attendance recording
- Examination conduct
- Result computation
- Result publication

---

# 3. Domain Responsibilities

The domain is responsible for:

- Defining internal assessment structures for a course offering or academic term.
- Managing the lifecycle of internal assessment windows.
- Capturing student submissions and assessor marks.
- Supporting governed moderation and correction.
- Producing final internal assessment facts for downstream examination workflows.

The domain does not own examination marks, grades, or official academic results.

---

# 4. Bounded Context Position

```text
Section B (Curriculum / Calendar)
        |
        v
L2-D01 Course Offering
        |
        v
L2-D04 Teaching Session & Attendance
        |
        v
L2-D07 Internal Assessment
        |
        v
Section I Examination
```

Internal assessment consumes course-offering context and teaching evidence, then publishes final internal assessment facts for examination processing.

---

# 5. Lifecycle Overview

```text
Draft -> Published -> Open -> In Progress -> Moderated -> Finalized -> Archived
```

Lifecycle rules:

- A blueprint must be published before an assessment window can open.
- A submission must be recorded before moderation.
- Finalized records are immutable except through controlled correction workflow.
- Archived records remain queryable for audit and governance.

---

# 6. Aggregate Ownership

## 6.1 InternalAssessmentBlueprint

Owns the component structure, weightage, and policy binding for an assessment cycle.

## 6.2 AssessmentWindow

Owns release, deadline, extension, and closure control for internal assessment activities.

## 6.3 AssessmentSubmission

Owns the captured submission record and awarded internal marks for a student.

## 6.4 AssessmentModerationCase

Owns moderation, correction, and approval workflow for submitted internal assessment outcomes.

---

# 7. Business Events

The domain publishes immutable business facts:

- InternalAssessmentBlueprintPublished
- AssessmentWindowOpened
- AssessmentSubmissionRecorded
- AssessmentSubmissionUpdated
- AssessmentModerationRequested
- AssessmentModerationCompleted
- InternalAssessmentFinalized
- InternalAssessmentArchived

Consumers:

- Section I Examination
- Academic Monitoring
- Notification

---

# 8. Commands

The domain exposes canonical commands:

- CreateInternalAssessmentBlueprint
- PublishInternalAssessmentBlueprint
- OpenAssessmentWindow
- ExtendAssessmentWindow
- CloseAssessmentWindow
- RecordAssessmentSubmission
- UpdateAssessmentMarks
- RequestAssessmentModeration
- ApproveAssessmentModeration
- RejectAssessmentModeration
- FinalizeInternalAssessment
- ArchiveInternalAssessment

---

# 9. Multi-Domain Rules

- D01 provides the published course offering context.
- D04 provides teaching completion and attendance evidence.
- D07 does not alter teaching session records.
- D07 does not compute official examination grades.
- Section I consumes only finalized internal assessment facts.

---

# 10. API Surface

Representative canonical APIs:

- GET /internal-assessments/blueprints
- POST /internal-assessments/blueprints
- POST /internal-assessments/windows
- POST /internal-assessments/submissions
- POST /internal-assessments/moderation-cases
- POST /internal-assessments/{id}/finalize

---

# 11. Architectural Constraints

- Internal assessment must remain separate from examination result computation.
- Finalized internal assessment data is immutable.
- Moderation preserves the original submission audit trail.
- No direct database coupling to Section I is permitted.
- All state transitions must be event driven and auditable.

---

# 12. Repository Principle

Internal assessment is an academic delivery capability and belongs in Section D.

Examination consumes the finalized outcome, but does not own the internal assessment lifecycle.

---

## Workflows

Workflow behavior is not restated in a single consolidated section; it is expressed through the Lifecycle Overview (Section 5) state sequence and the Commands catalogue (Section 8): blueprint definition and publication, assessment window opening/extension/closure, submission capture and mark entry, moderation request/approval/rejection, and finalization/archival. See Sections 5 and 8 for the operational sequence and canonical command set.
