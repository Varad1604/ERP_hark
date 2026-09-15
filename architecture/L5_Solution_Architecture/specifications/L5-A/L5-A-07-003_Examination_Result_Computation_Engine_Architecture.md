# L5-A-07-003 — Examination Result Computation Engine Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-A-07-003 |
| Specification Title | Examination Result Computation Engine Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-A — Core Platform & Application Architecture |
| Major Area | L5-A-07 — Secure Computational Engine Architecture |
| Canonical Technical Responsibility | Executes approved result formulas/regulations; cannot approve or publish official results autonomously. |
| Technical Architecture Owner | Examination Result Computation Technical Architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-A-07-001; L5-A-07-005; L5-B-02-001; L5-D-03-001; L5-E-04-001; L4-I-083; L4-I-084; L4-I-085; L4-I-086; L4-I-087 |
| Referenced By | Academic Examination Operations; Grade Moderation; Transcript Publication workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the solution architecture for the Examination Result Computation Engine within SutraERP. The engine executes approved academic regulations, grading formulas, moderation adjustments, grace mark distributions, and SGPA/CGPA calculations, while strictly prohibiting the engine from autonomously approving marks, lifting institutional exam withholdings, or publishing official student transcripts.

### In Scope

- Execution of approved, versioned grading schemes, credit weightings, and mark-to-grade mapping formulas.
- GPA/CGPA computation preserving credit weightings and institutional rounding rules.
- Automated evaluation of mandatory pass/fail criteria and prerequisite completion invariants.
- Grace mark and moderation distribution computation under authorized policy ceilings.
- Multi-cycle recalculation lineage tracking (re-evaluation, exam re-conduct, malpractice penalty adjustments).
- Emission of cryptographic result computation evidence records.

### Out of Scope / Non-Goals

- Granting institutional approval to examination results (owned by Examination Board / Controller of Examinations).
- Publishing official student grade cards or transcripts to portals (owned by L4-I-083/L4-I-100).
- Authorizing moderation policies or changing passing thresholds (owned by Academic Council).
- Modifying raw student exam marks directly in academic records.
- UI/UX grade entry interfaces or faculty grading screens.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L4 | `L4-I-083` | Approved / Locked | `L4-I-083-BH-001` | Admission of final approved marks into result computation | Examination evaluation admission boundary |
| L4 | `L4-I-083` | Approved / Locked | `L4-I-083-BH-002` | Execution of grading schemes, credit weightings, and GPA algorithms | Core academic result computation pipeline |
| L4 | `L4-I-083` | Approved / Locked | `L4-I-083-BH-009` | Attributable evidence generation binding inputs and regulations | Computation evidence record emission |
| L4 | `L4-I-084` | Approved / Locked | `L4-I-084-BH-003` | Replacement academic outcome calculation following exam re-conduct | Recomputation and supersession execution |
| L4 | `L4-I-085` | Approved / Locked | `L4-I-085-BH-001` | Evaluation of mandatory result conditions and withholding triggers | Invariant verification and withholding flagging |
| L4 | `L4-I-085` | Approved / Locked | `L4-I-085-BH-002` | Halting academic progression when result conditions are unsatisfied | Progression barrier enforcement |
| L4 | `L4-I-086` | Approved / Locked | `L4-I-086-BH-003` | Resumed calculation from preserved evaluation checkpoint | Checkpointed state continuation |
| L4 | `L4-I-087` | Approved / Locked | `L4-I-087-BH-002` | Pre-approval recalculation cycle after marks correction | Re-evaluation attempt lineage tracking |
| Decision | Corpus Register | Locked | `L5-A-07-003` | Executes result formulas; cannot publish official results | Canonical technical responsibility |

### 3.2 Authoritative Constraints

| Constraint ID | Constraint | Exact Source | L5 Effect |
|---|---|---|---|
| C-001 | Examination results produced by the engine are technically calculated, not institutionally published. | L5 Candidate Register §3.1 | Engine outputs carry status `CALCULATED_UNAPPROVED` and cannot be viewed by students. |
| C-002 | Grading regulations must be cryptographically versioned and effective-dated. | Academic Governance Baseline | Historical evaluations must be reproducibly re-evaluated under their original regulations. |
| C-003 | Grace marks and moderation shall not exceed authorized institutional maximums. | Examination Regulations | Hard policy bounds enforced during calculation; excess adjustments are rejected. |

### 3.3 Assumptions

| ID | Assumption | Why Needed | Impact if False | Validation / Resolution |
|---|---|---|---|---|
| AS-001 | Calling academic workflows provide validated, finalized component marks. | Engine does not fetch raw exam papers or verify evaluator signatures. | Unfinalized marks cause immediate admission rejection. | Calling workflow must certify mark completion before invoking engine. |
| AS-002 | Grading rules and formula expressions are defined declaratively via L5-A-07-005. | Ensures consistent formula parsing across subjects. | Rule syntax errors trapped prior to execution. | Regulations compiled into immutable AST packages. |

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

- Pure academic result computation (subject grade mapping, SGPA, CGPA, total credits earned).
- Verification of mandatory passing criteria (minimum internal marks, minimum external marks, aggregate threshold).
- Grace mark and moderation distribution computation within authorized parameters.
- Re-evaluation differential computation and historical attempt lineage hashing.
- Detection of withheld result conditions (unmatched prerequisites, malpractice flags).

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability / Collaboration | Boundary | Why Needed |
|---|---|---|---|
| `L5-A-07-001` | Secure Computational Engine Baseline | Execution framework | Inherits admission gate, sandbox execution, and lineage hashing. |
| `L5-A-07-005` | General Rules & Formula Engine | Regulation evaluation | Evaluates declarative subject grading rules and conditional criteria. |
| `L5-B-02-001` | Database & Persistence Architecture Baseline | Persistence handoff | Stores calculation evidence records immutably (ADR-L5-0005). |
| `L5-D-03-001` | Canonical Trust Boundary & Segmentation | Security boundary | Protects grading calculations against unauthorized runtime manipulation. |
| `L5-E-04-001` | Runtime Packaging & Deployment Baseline | Worker execution | Packages examination computation workers as isolated, stateless runtimes. |

### 4.3 Must Not Own

- Institutional approval or certification of examination results.
- Publishing results to student web portals or generating physical transcripts.
- Authorizing moderation policies or granting individual student waivers.
- Modifying raw student marks or examination attendance records.

---

## 5. Solution Architecture

### 5.1 Technical Realization Model

| Element | Type | Responsibility | Input / Context | State / Data | Output / Handoff | Depends On | Failure Boundary | Basis |
|---|---|---|---|---|---|---|---|---|
| `ExamAdmissionGate` | Control Point | Validates exam calculation request, student enrollment, mark completeness, and regulation ID. | `ExamEvaluationRequest` | Stateless | Admitted exam envelope | `L5-A-07-001` | Rejects unfinalized marks or invalid regulation binding. | L4-I-083-BH-001 |
| `AcademicRegulationBinder` | Component | Resolves approved grading scheme, grade point mapping table, and credit weights for student batch. | Regulation ID & Version | Cached regulation AST | Bound academic calculation plan | `L5-A-07-005` | Fails on missing or unapproved regulation package. | C-002 |
| `GradingEvaluationPipeline` | Component | Computes component total, applies grace marks (if eligible), maps mark to letter grade and grade points. | Component marks & bound rules | Ephemeral registers | `SubjectResultItem` | `L5-A-07-001` | Flags passing criteria failure or incomplete components. | L4-I-083-BH-002 |
| `GPACalculator` | Component | Computes semester SGPA and cumulative CGPA using credit-weighted grade points and institutional rounding. | List of `SubjectResultItem` | Ephemeral registers | `ComputedGPA` | `L5-A-07-001` | Traps divide-by-zero on zero total credits. | L4-I-083-BH-002 |
| `ConditionInvariantValidator` | Control Point | Evaluates mandatory result invariants (attendance threshold, malpractice flags, backlog limits). | Subject results & student flags | Stateless | Validated exam outcome | Academic rules | Flags `WITHHELD` or `FAIL_INCOMPLETE` condition. | L4-I-085-BH-001 |

### 5.2 Primary Technical Flow

```text
Examination Processing Workflow (L4-I-083)
         ↓
ExamAdmissionGate (verify Tenant, Course, Mark Completeness)
         ↓
AcademicRegulationBinder (bind approved Grading Scheme & Credit Weights)
         ↓
GradingEvaluationPipeline (Internal + External Marks -> Grace Marks -> Subject Grade)
         ↓
ConditionInvariantValidator (verify Mandatory Passing Invariants & Withholding Flags)
         ↓
GPACalculator (Credit-Weighted Sum(GradePoints * Credits) / TotalCredits -> SGPA / CGPA)
         ↓
LineageEvidenceEmitter (emit SHA-256 evidence record via L5-A-07-001)
         ↓
Return ComputedExamOutcome to Examination Board (Status: CALCULATED_UNAPPROVED)
```

### 5.3 Material Failure Model

| Failure Class | Detection Boundary | Required Technical Outcome | Forbidden Outcome | Recovery / Delegated Owner | Basis |
|---|---|---|---|---|---|
| Incomplete Component Marks | `ExamAdmissionGate` | Calculation rejected (`ERR_MARKS_INCOMPLETE`). | Calculating SGPA with missing internal/external marks. | Academic department must submit missing marks. | L4-I-083-BH-001 |
| Missing Grading Regulation | `AcademicRegulationBinder` | Immediate abort (`ERR_REGULATION_UNRESOLVED`). | Falling back to generic or unapproved grading scale. | Academic Controller must register approved regulation. | C-002 |
| Zero Credit Divisor | `GPACalculator` | Calculation fault (`ERR_ZERO_CREDIT_ENROLLMENT`); emit alert. | Emitting NaN or infinity as student SGPA/CGPA. | Curriculum administrator must correct course credits. | L4-I-083-BH-002 |
| Grace Mark Limit Violation | `GradingEvaluationPipeline` | Reject grace mark application (`ERR_GRACE_EXCEEDS_POLICY`). | Granting marks exceeding statutory policy ceiling. | Examination Committee must review borderline case. | C-003 |
| Unresolved Malpractice Flag | `ConditionInvariantValidator` | Mark result `STATUS_WITHHELD` with reason code. | Emitting a passing grade card while malpractice is pending. | Disciplinary committee must adjudicate case. | L4-I-085-BH-001 |

### 5.4 Conceptual Contracts

#### `ExamEvaluationRequest`
```text
ExamEvaluationRequest {
  TenantId: UUID
  InstitutionId: UUID
  EvaluationCorrelationId: UUID
  StudentId: UUID
  ProgramId: String
  SemesterId: String
  RegulationId: String
  RegulationVersion: String
  CourseMarksList: List<CourseMarkEntry>
  StudentDisciplinaryFlags: List<String>
  PriorCumulativeCredits: Integer
  PriorCumulativeGradePoints: Decimal(10, 4)
}
```

#### `ComputedAcademicOutcome`
```text
ComputedAcademicOutcome {
  EvaluationCorrelationId: UUID
  EvidenceRecordId: UUID
  StudentId: UUID
  SemesterId: String
  SubjectGrades: List<SubjectGradeRecord>
  SemesterCreditsRegistered: Integer
  SemesterCreditsEarned: Integer
  SemesterSGPA: Decimal(5, 2)
  CumulativeCGPA: Decimal(5, 2)
  ResultStatus: Enum [PASSED, FAILED, WITHHELD]
  WithholdingReasonCode: Optional<String>
  CalculationLineageHash: String (SHA-256)
  EvaluationStatus: Enum [SUCCESS, FAILED_PREREQUISITE, FAILED_INCOMPLETE]
}
```

### 5.5 State, Persistence & Consistency Boundaries

The engine is **stateless**. It does not write to the student's official academic history or publish to student portals. Calculation results are emitted as unapproved evaluation candidates. Historical recalculation cycles (e.g., following re-evaluation or malpractice resolution) append new attempt evidence records referencing the prior attempt hash via `L5-A-07-001`.

### 5.6 Asynchronous & Job Realization

Semester result processing (evaluating results for an entire graduating cohort) is executed as background batch jobs partitioned by academic department and section, coordinated via `L5-A-08-001` under `ADR-L5-0013`.

### 5.7 Security, Trust & Isolation

- Calculation sandboxes enforce zero network and disk I/O.
- Grade mapping tables and regulation formulas are cryptographically signed to prevent tampering with passing criteria.
- Result status `WITHHELD` is strictly enforced whenever a student possesses an active disciplinary or fee hold flag.

### 5.8 Runtime & Deployment Implications

Deploys as part of the isolated computational worker cluster (`L5-E-04-001`). Workers are dynamically scaled during exam processing seasons without impacting campus operational APIs.

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis / Derivation | Rationale | ADR |
|---|---|---|---|---|---|
| `AD-A07-030` | Pure Function Grade Computation | Result computation is modeled as a pure mathematical function: `(Marks, RegulationAST) -> Result`. | Architecture isolation baseline | Guarantees identical result reproduction during independent auditing or court inquiries. | None (Local L5 Architecture Decision) |
| `AD-A07-031` | Checkpointed Recomputation Continuation | Recomputation can resume from verified component marks without re-evaluating unchanged courses. | L4-I-086-BH-003 | Optimizes compute during re-evaluation while preserving strict evaluation checkpoint hashes. | None (Local L5 Architecture Decision) |
| `AD-A07-032` | Automatic Withholding Flag Trapping | Any unresolved administrative or malpractice flag automatically forces `ResultStatus = WITHHELD`. | L4-I-085-BH-001; C-001 | Prevents accidental publication of disputed examination results. | None (Local L5 Architecture Decision) |
| `AD-A07-033` | Separation of Calculation from Publication | Engine produces `CALCULATED_UNAPPROVED` results; publishing requires explicit Controller of Examinations workflow approval. | Corpus Register L5-A-07-003 | Preserves institutional authority and regulatory compliance. | Governed by ADR-L5-0005 |

---

## 7. Security, Privacy, Quality & Operational Impact

| Attribute / Concern | Architectural Impact | Source / Rationale | Owning / Collaborating L5 Spec |
|---|---|---|---|
| Grading Integrity | Complete determinism and non-repudiation; grade calculation logic is immutable once approved. | Higher education accreditation | `L5-A-07-003` |
| Student Privacy | Calculation workers process anonymized candidate IDs during bulk grading pipelines. | Data privacy compliance | `L5-B-05-002` |
| Result Lineage | Every grade change through re-evaluation or moderation creates a cryptographic audit trail. | Academic integrity standard | `L5-D-04-003` |

---

## 8. Architecture Conformance Requirements

| Requirement ID | Architecture Conformance Requirement | Derived From | Upstream / Decision Basis |
|---|---|---|---|
| `CR-001` | The implementation shall admit examination calculation only when all mandatory component marks for the enrolled course are present. | §5.1 | `L4-I-083-BH-001` |
| `CR-002` | The engine shall evaluate grade points and letter grades strictly according to the approved academic regulation version bound at admission. | §5.1, `AD-A07-030` | `L4-I-083-BH-002`, C-002 |
| `CR-003` | SGPA and CGPA calculations shall apply credit weightings exactly and round to two decimal places using institutional rounding rules. | §5.1, §5.4 | `L4-I-083-BH-002` |
| `CR-004` | When mandatory passing conditions for a course are unsatisfied, the result status shall be marked `FAIL` regardless of aggregate marks. | §5.1, §5.3 | `L4-I-085-BH-002` |
| `CR-005` | When an active malpractice or administrative blocking flag is present, the result status shall be marked `WITHHELD`. | §5.1, `AD-A07-032` | `L4-I-085-BH-001` |
| `CR-006` | Grace mark distribution shall not exceed the maximum policy ceiling established in the approved regulation. | §5.1, §5.3 | C-003 |
| `CR-007` | In recomputation scenarios, the engine shall compute replacement outcomes without overwriting prior calculation evidence records. | §5.5, `AD-A07-031` | `L4-I-084-BH-003`, `L4-I-087-BH-009` |
| `CR-008` | The engine shall emit a SHA-256 lineage evidence record binding marks, regulation version, and computed GPA via `L5-A-07-001`. | §5.1, §5.4 | `L4-I-083-BH-009` |
| `CR-009` | The engine shall carry result status `CALCULATED_UNAPPROVED` and shall not publish results or write official transcript tables. | §4.3, `AD-A07-033` | C-001 |
| `CR-010` | The implementation shall trap zero-credit division faults and return structured error codes rather than NaN. | §5.3 | `L4-I-083-BH-002` |
| `CR-011` | Checkpointed continuation shall verify that input mark hashes match the preserved checkpoint prior to re-execution. | §5.1, `AD-A07-031` | `L4-I-086-BH-003` |
| `CR-012` | Batch semester evaluations shall integrate with the background job execution framework defined in `L5-A-08-001`. | §5.6 | ADR-L5-0013 |

---

## 9. Open Decisions & Deferred Design

| Item ID | Item | Classification | Missing / Conflicting Source | Decision Authority | Blocking? | Required Resolution |
|---|---|---|---|---|---|---|
| `OD-001` | Normalization Curve Algorithm Selection | Technology / Method | Statistical moderation curve methods (Linear vs Gaussian vs Z-score) | Academic Board / Controller of Examinations | No | Specific moderation formulas are defined declaratively per academic session in L5-A-07-005. |
| `OD-002` | Anonymized Blind Grading Identifiers | Security Architecture | Candidate code masking integration | Examination Controller / L5-B-05 | No | Integration with candidate masking is validated during exam scheduling. |
| `OD-003` | Automatic Degree Class Distinction Rules | Academic Policy | First Class with Distinction criteria across autonomous colleges | University Regulation Committee | No | Configured via institutional policy tables in L5-A-06. |

---

## 10. Traceability Summary

- `L4-I-083-BH-001` — Realized by `ExamAdmissionGate` verifying final component mark completeness (§5.1, `CR-001`).
- `L4-I-083-BH-002` — Realized by `GradingEvaluationPipeline` and `GPACalculator` computing grades and GPA (§5.1, `CR-002`, `CR-003`).
- `L4-I-083-BH-009` — Realized by `LineageEvidenceEmitter` generating cryptographic result provenance (§5.1, `CR-008`).
- `L4-I-084-BH-003` — Realized by recomputation execution from re-conducted examination marks (§5.1, `CR-007`).
- `L4-I-085-BH-001` — Realized by `ConditionInvariantValidator` evaluating mandatory passing rules and withholdings (§5.1, `CR-005`).
- `L4-I-085-BH-002` — Realized by progression barrier enforcement upon unsatisfied passing criteria (§5.1, `CR-004`).
- `L4-I-086-BH-003` — Realized by checkpointed state continuation from preserved evaluation checkpoints (§5.1, `CR-011`).
- `L4-I-087-BH-002` — Realized by pre-approval recalculation cycle after marks correction (§5.1, `CR-007`).
