# L2-I09 — Result Processing & Computation Workflows

**Section:** I — Examination  
**Status:** Draft  
**Document Owner:** Devang

---

## Canonical Section Crosswalk

> Added 2026-08-04 under integrity finding INT-018. `governance/constitution/Volume_3_Architecture_Framework.md` section 18 requires every L2 document to define eleven sections. This document uses its own numbered-section convention; the table below maps the required sections to where they actually live. No content was moved.

| Required section (Volume 3 §18) | Location in this document |
|---|---|
| Purpose | §1 Purpose |
| Scope | §2 Scope |
| Actors | §6 Business Actors; Primary Business Actors; Supporting Business Actors |
| Entities | — not present — |
| Business Rules | §9 Business Rules |
| Workflow | §8 Result Processing Lifecycle Workflows; §17 Workflow Completion Criteria; L2-I09 — Result Processing & Computation Workflows |
| State Machine | §20 Result Processing & Computation State Model |
| Contracts | — not present — |
| Events | §11 Events Published; §12 Events Consumed; §4.9 Event-Driven Result Processing |
| Dependencies | §19 Result Processing & Computation Dependency Matrix |
| Locked Decisions | — not present — |

> **Gap:** 3 required section(s) have no counterpart in this document: Entities, Contracts, Locked Decisions. Recorded, not authored.

---

# 1. Purpose

This document defines the canonical business workflows governing Result Processing & Computation within the SutraERP Examination domain.

It establishes the enterprise framework for aggregating approved marks, applying institutional academic regulations, computing subject results, calculating grades and grade points, determining pass or fail status, computing Semester Grade Point Average (SGPA), Cumulative Grade Point Average (CGPA), credit points, programme outcomes, academic standing, and producing institutionally approved results for publication.

The document governs academic result processing beginning with Final Approved Marks received from Marks Collection & Evaluation and concluding with institutionally approved results ready for publication.

This document standardises academic result computation while allowing institutions to configure grading schemes, credit systems, academic regulations, grace rules, classification policies, and progression rules without modifying the canonical repository architecture.

---

# 2. Scope

The Result Processing & Computation domain governs the complete academic result computation lifecycle.

The scope includes:

- Final Approved Marks Consumption
- Marks Aggregation
- Component Weightage Calculation
- Subject Result Computation
- Pass / Fail Determination
- Grace Marks Processing
- Benefit Rule Processing
- Credit Point Calculation
- Grade Calculation
- Grade Point Calculation
- SGPA Calculation
- CGPA Calculation
- Semester Result Processing
- Academic Year Result Processing
- Programme Result Processing
- Classification & Division Determination
- Academic Standing Computation
- Result Validation
- Result Approval
- Result Finalisation
- Result Processing Lifecycle Governance

This document does not govern:

- Examination Policy Management
- Examination Session Management
- Examination Planning & Scheduling
- Examination Registration
- Hall Ticket Management
- Practical & Viva Examination Conduct
- Theory Examination Conduct
- Marks Collection & Evaluation
- Result Publication
- Revaluation & Rechecking
- Supplementary / Backlog / Improvement Examination Execution
- Academic Completion & Completion Evidence

Ownership of these business capabilities remains with their respective workflow documents.

---

# 3. Repository Context

The Result Processing & Computation domain is positioned immediately after Marks Collection & Evaluation within the Examination repository.

It consumes Final Approved Marks generated through academic evaluation and transforms them into institutionally approved academic results according to configured academic regulations.

This document consumes business information from:

- L2-I01 — Examination Policy & Configuration Workflows
- L2-I02 — Examination Session Management Workflows
- L2-I03 — Examination Planning & Scheduling Workflows
- L2-I04 — Examination Registration Workflows
- L2-I08 — Marks Collection & Evaluation Workflows

It collaborates with:

- Academic Calendar Domain
- Organisation Domain
- Person & Identity Domain
- Platform Foundation Services

It produces authoritative academic results consumed by:

- L2-I10 — Result Publication Workflows

This document owns academic computation only.

It does not own academic evaluation, examination conduct, or publication of results.

---

# 4. Architectural Principles

The following architectural principles govern Result Processing & Computation across the SutraERP repository.

---

## 4.1 Final Approved Marks Before Computation

Academic result computation shall begin only after Final Approved Marks have been established by Marks Collection & Evaluation.

Result Processing shall never compute results using provisional or unapproved marks.

---

## 4.2 Computation Before Publication

Academic results shall be fully computed, validated, and institutionally approved before publication.

Result Publication consumes approved results without recalculating academic outcomes.

---

## 4.3 Separation of Evaluation and Computation

Academic evaluation and academic result computation represent separate business responsibilities.

Evaluation determines marks.

Result Processing determines academic outcomes based on approved marks and institutional regulations.

---

## 4.4 Configurable Academic Regulations

Institutions may configure academic regulations while preserving repository-standard workflows.

Configurable regulations may include:

- Grading Schemes
- Credit Systems
- Weightage Models
- Grace Policies
- Pass Criteria
- Academic Standing Rules
- Classification Policies

Academic regulations remain institution configurable.

---

## 4.5 Rule-Based Academic Computation

Every academic result shall be computed according to approved institutional regulations.

Computation shall apply configured business rules consistently across all candidates.

Manual intervention shall occur only through authorised institutional procedures.

---

## 4.6 Immutable Evaluation Outcomes

Final Approved Marks received from L2-I08 shall be treated as immutable inputs.

Academic computation shall not alter evaluation history or awarded marks.

Any subsequent change to approved marks shall occur only through authorised downstream workflows.

---

## 4.7 Approved Results as the Authoritative Outcome

The authoritative output of this domain is the Institutionally Approved Result.

Institutionally Approved Results become the sole input for Result Publication.

---

## 4.8 Multi-Domain Collaboration

Result Processing & Computation collaborates with Examination Policy, Marks Collection & Evaluation, Academic Calendar, Organisation, Person & Identity, and Platform Foundation domains while preserving ownership boundaries.

---

## 4.9 Event-Driven Result Processing

Significant business transitions during result computation shall publish business events.

Examples include:

- Result Processing Started
- Subject Result Computed
- Semester Result Computed
- CGPA Calculated
- Result Approved
- Result Ready for Publication

---

## 4.10 Complete Auditability

Every significant academic computation activity shall generate auditable business records supporting institutional governance, regulatory compliance, academic transparency, and historical traceability.

---

# 5. Business Capabilities

The Result Processing & Computation domain provides the canonical business capabilities required to transform Final Approved Marks into institutionally approved academic results.

Each capability represents a governed business responsibility independent of implementation technology.

---

## 5.1 Final Approved Marks Consumption

The Result Processing & Computation workflow begins by consuming Final Approved Marks produced by Marks Collection & Evaluation.

The capability includes:

- Approved Marks Intake
- Marks Availability Verification
- Subject Marks Identification
- Assessment Component Recognition
- Computation Readiness Verification

Only Final Approved Marks are accepted for computation.

---

## 5.2 Marks Aggregation

Marks from multiple assessment components shall be aggregated according to institutional academic regulations.

Aggregation may include:

- Theory Components
- Practical Components
- Viva Components
- Project Components
- Internal Assessment
- Continuous Assessment
- Component Weightages

Aggregation establishes the total marks available for academic computation.

---

## 5.3 Academic Rule Processing

Academic regulations shall be applied before determining academic outcomes.

Rule processing may include:

- Pass Rules
- Fail Rules
- Eligibility Rules
- Mandatory Component Rules
- Minimum Marks Rules
- Attendance-linked Rules
- Programme-specific Regulations

Academic rules remain institution configurable.

---

## 5.4 Grace & Academic Benefit Processing

Where permitted by institutional regulations, academic benefit rules may be applied.

Supported capabilities include:

- Grace Marks
- Condonation
- Benefit Rules
- Borderline Adjustments
- Special Academic Regulations

Every applied academic benefit shall remain fully auditable.

---

## 5.5 Subject Result Computation

The domain computes the academic outcome for each registered subject.

Subject computation includes:

- Total Marks
- Pass / Fail Status
- Subject Grade
- Grade Point
- Earned Credits
- Subject Completion Status

Subject results become inputs for higher-level computation.

---

## 5.6 Credit Point Calculation

Credit-based academic systems require calculation of earned academic credits.

The capability includes:

- Credit Recognition
- Credit Achievement
- Credit Point Calculation
- Credit Accumulation
- Programme Credit Progress

Credit ownership remains within academic regulations.

---

## 5.7 Grade Calculation

Grades shall be calculated according to approved institutional grading schemes.

Supported grading approaches may include:

- Letter Grades
- Numeric Grades
- Percentage Grades
- Performance Bands
- Programme-specific Grades

Grading methodologies remain institution configurable.

---

## 5.8 Grade Point Calculation

Where grade point systems are adopted, grade points shall be calculated from approved grades.

The capability supports:

- Grade Point Assignment
- Weighted Grade Points
- Credit-weighted Grade Points
- Grade Point Validation

Grade Point computation supports downstream GPA calculations.

---

## 5.9 SGPA & CGPA Calculation

The domain computes cumulative academic performance indicators.

Supported computations include:

- Semester Grade Point Average (SGPA)
- Cumulative Grade Point Average (CGPA)
- Programme Grade Point Average
- Running Academic Performance

Calculation methodologies remain institution configurable.

---

## 5.10 Programme Result Computation

Academic performance shall be computed across the applicable academic structure.

Programme computation may include:

- Semester Result
- Academic Year Result
- Programme Result
- Stage Result
- Overall Academic Progress

Programme computation establishes institutional academic standing.

---

## 5.11 Classification & Academic Standing

The domain determines institutional academic classifications.

Supported outcomes may include:

- Distinction
- First Class
- Second Class
- Pass Class
- Honours
- Academic Probation
- Promotion Eligibility
- Academic Standing

Classification rules remain configurable according to institutional regulations.

---

## 5.12 Result Validation

Computed results shall undergo institutional validation before approval.

Validation confirms:

- Computational Accuracy
- Academic Policy Compliance
- Credit Consistency
- Grade Consistency
- Programme Consistency
- Regulation Compliance

Only validated results proceed for approval.

---

## 5.13 Result Approval

Institutional authorities shall approve validated academic results.

Approval may include:

- Department Approval
- Faculty Approval
- Controller Approval
- Institutional Approval

Approved results become the authoritative academic outcome.

---

## 5.14 Result Finalisation

Approved results shall be finalised before publication.

Finalisation confirms:

- Academic Computation Complete
- Institutional Approval Complete
- Result Integrity Verified
- Publication Readiness Achieved

Finalised results become available for Result Publication.

---

## 5.15 Result Processing Lifecycle

The domain governs the complete lifecycle of academic result computation.

The lifecycle includes:

1. Final Approved Marks Consumption
2. Marks Aggregation
3. Academic Rule Processing
4. Grace & Benefit Processing
5. Subject Result Computation
6. Credit Point Calculation
7. Grade Calculation
8. SGPA / CGPA Calculation
9. Programme Result Computation
10. Classification & Academic Standing
11. Result Validation
12. Result Approval
13. Result Finalisation
14. Ready for Publication

The lifecycle concludes with Institutionally Approved Results ready for publication.

---

# 6. Business Actors

The following business actors participate in Result Processing & Computation workflows.

---

## Primary Business Actors

- Candidate
- Controller of Examination
- Examination Administrator
- Result Processing Officer
- Academic Registrar
- Department Head
- Faculty Dean
- Academic Committee

---

## Supporting Business Actors

- Organisation Domain
- Person & Identity Domain
- Academic Calendar Domain
- Platform Foundation
- Audit Infrastructure

Business actors collaborate while preserving ownership boundaries.

---

# 7. Business Framework

The Result Processing & Computation framework transforms approved academic evaluation outcomes into institutionally approved academic results.

The framework ensures that every academic outcome is computed consistently according to institutional regulations before publication.

---

## 7.1 Business Objectives

The primary objectives of this domain are:

- Consume Final Approved Marks
- Apply Academic Regulations
- Compute Academic Results
- Calculate Grades and Grade Points
- Compute SGPA and CGPA
- Determine Academic Standing
- Validate Results
- Approve Results
- Produce Results Ready for Publication

---

## 7.2 Result Processing Framework

The canonical computation framework is:

```text
Final Approved Marks
        │
        ▼
Marks Aggregation
        │
        ▼
Academic Rule Processing
        │
        ▼
Grace & Benefit Processing
        │
        ▼
Subject Result Computation
        │
        ▼
Grade Calculation
        │
        ▼
Credit Point Calculation
        │
        ▼
SGPA / CGPA
        │
        ▼
Programme Result
        │
        ▼
Academic Standing
        │
        ▼
Validation
        │
        ▼
Approval
        │
        ▼
Institutionally Approved Result
```

The framework preserves the separation between academic evaluation and academic result computation.

---

## 7.3 Academic Regulation Framework

Result computation operates according to configured institutional regulations.

The framework governs:

- Pass Criteria
- Fail Criteria
- Credit Rules
- Grading Schemes
- Grace Policies
- Classification Rules
- Academic Progression Rules

Institutional regulations influence computation without changing the canonical workflow.

---

## 7.4 Academic Performance Framework

Academic performance is computed at multiple levels.

Supported levels include:

- Subject Performance
- Semester Performance
- Academic Year Performance
- Programme Performance
- Cumulative Academic Performance

Each level contributes to the final institutional academic result.

---

## 7.5 Result Governance Framework

Academic results progress through controlled governance states.

```text
Computed Result
        │
        ▼
Validated Result
        │
        ▼
Approved Result
        │
        ▼
Finalised Result
        │
        ▼
Ready for Publication
```

Only **Finalised Results** are transferred to **L2-I10 — Result Publication Workflows**.

# 8. Result Processing Lifecycle Workflows

The Result Processing & Computation domain governs the complete lifecycle through which Final Approved Marks are transformed into institutionally approved academic results.

Each lifecycle stage represents a controlled academic business transition.

---

## 8.1 Lifecycle Overview

The canonical Result Processing lifecycle consists of the following stages:

1. Final Approved Marks Consumption
2. Marks Aggregation
3. Academic Rule Processing
4. Grace & Benefit Processing
5. Subject Result Computation
6. Credit Point Calculation
7. Grade Calculation
8. SGPA & CGPA Calculation
9. Programme Result Computation
10. Academic Standing Determination
11. Result Validation
12. Result Approval
13. Result Finalisation
14. Ready for Publication
15. Archival

Each stage shall be completed before progression to the next business state.

---

## 8.2 Final Approved Marks Consumption

Result Processing begins after Final Approved Marks have been received from Marks Collection & Evaluation.

The process includes:

- Marks Availability Verification
- Subject-wise Marks Identification
- Assessment Component Recognition
- Candidate Result Eligibility Verification
- Computation Readiness Verification

Only institutionally approved marks may enter Result Processing.

---

## 8.3 Marks Aggregation

Approved assessment components shall be aggregated according to institutional regulations.

Aggregation may include:

- Theory Marks
- Practical Marks
- Viva Marks
- Project Marks
- Internal Assessment
- Continuous Assessment
- Weightage Components

Aggregation establishes the academic basis for result computation.

---

## 8.4 Academic Rule Processing

Institutional academic regulations shall be applied before determining academic outcomes.

Rule processing may include:

- Minimum Passing Marks
- Mandatory Component Rules
- Credit Completion Rules
- Programme-specific Regulations
- Eligibility Rules
- Attendance-linked Rules
- Institutional Academic Policies

Only compliant academic records proceed to computation.

---

## 8.5 Grace & Benefit Processing

Where permitted, institutional academic benefits shall be applied.

Supported processing includes:

- Grace Marks
- Condonation
- Benefit Rules
- Borderline Adjustments
- Special Academic Provisions

Every applied benefit shall remain completely auditable.

---

## 8.6 Subject Result Computation

Academic outcomes shall be computed for each registered subject.

Subject computation includes:

- Total Marks
- Pass Status
- Fail Status
- Subject Grade
- Grade Point
- Credit Earned
- Subject Completion Status

Each computed subject result contributes to programme-level computation.

---

## 8.7 Credit Point Calculation

Academic credit calculations shall follow approved institutional regulations.

Processing includes:

- Credit Achievement
- Earned Credits
- Credit Points
- Credit Accumulation
- Programme Credit Progress

Credit computation supports academic progression.

---

## 8.8 Grade Calculation

Grades shall be calculated using approved institutional grading schemes.

Supported grading models include:

- Letter Grade
- Percentage Grade
- Numeric Grade
- Performance Band
- Programme-specific Grade

Grade calculation follows institutional grading policies.

---

## 8.9 SGPA & CGPA Calculation

Institutional academic performance indicators shall be computed after subject completion.

Supported calculations include:

- Semester Grade Point Average (SGPA)
- Cumulative Grade Point Average (CGPA)
- Programme Grade Point Average
- Running Academic Performance

Calculation methodologies remain institution configurable.

---

## 8.10 Programme Result Computation

Programme-level academic performance shall be determined.

Computation may include:

- Semester Result
- Academic Year Result
- Programme Result
- Stage Result
- Overall Academic Performance

Programme computation establishes institutional academic outcomes.

---

## 8.11 Academic Standing Determination

Academic standing shall be determined after programme computation.

Standing may include:

- Promotion Eligible
- Programme Completed
- Academic Probation
- Backlog
- ATKT / Carry Forward
- Withheld
- Detained
- Failed

Academic standing follows institutional academic regulations.

---

## 8.12 Result Validation

Computed academic results shall undergo institutional validation.

Validation verifies:

- Computational Accuracy
- Academic Regulation Compliance
- Credit Consistency
- Grade Consistency
- Classification Accuracy
- Programme Consistency

Only validated results proceed for approval.

---

## 8.13 Result Approval

Validated academic results shall be approved by authorised academic authorities.

Approval may include:

- Department Approval
- Faculty Approval
- Controller Approval
- Institutional Approval


Approval establishes institutional acceptance of computed results.

---

## 8.14 Result Finalisation

Approved results become institutional Final Results.

Finalisation confirms:

- Academic Computation Complete
- Validation Complete
- Institutional Approval Complete
- Publication Readiness Achieved

Finalised results become authoritative academic outcomes.

---

## 8.15 Ready for Publication

Institutionally approved results become available for Result Publication.

Operational responsibility transitions to:

**L2-I10 — Result Publication Workflows**

This document concludes responsibility once Finalised Results have been produced.

---

## 8.16 Archival

Completed Result Processing records enter archival status after institutional closure.

Archived records remain available for:

- Academic Audit
- Regulatory Compliance
- Historical Reporting
- Institutional Analytics
- Academic Governance

Archival preserves institutional academic history.

---

# 9. Business Rules

The following business rules govern Result Processing & Computation.

---

## 9.1 Approved Marks Rule

Result Processing shall consume only Final Approved Marks received from L2-I08.

Provisional or unapproved marks shall not be processed.

---

## 9.2 Regulation Compliance Rule

Academic computation shall follow approved institutional regulations.

Manual computation outside authorised academic regulations shall not be permitted.

---

## 9.3 Component Aggregation Rule

Only authorised assessment components shall contribute to academic computation.

Aggregation shall follow configured institutional weightage policies.

---

## 9.4 Grace & Benefit Rule

Grace Marks, condonation and academic benefit rules shall be applied only where permitted by institutional regulations.

Every applied benefit shall remain fully auditable.

---

## 9.5 Grade Calculation Rule

Grades shall be calculated using approved institutional grading schemes.

Alternative grading methodologies shall be institution configurable.

---

## 9.6 Academic Performance Rule

SGPA, CGPA, Credit Points and Programme Results shall be calculated using approved academic regulations.

Computation methodologies shall remain consistent across candidates within the same academic regulation.

---

## 9.7 Result Validation Rule

Every computed academic result shall undergo validation before institutional approval.

Validation failures shall prevent approval.

---

## 9.8 Result Approval Rule

Only authorised academic authorities may approve computed results.

Unapproved results shall remain provisional.

---

## 9.9 Result Finalisation Rule

Only approved academic results shall become Finalised Results.

Finalised Results become the authoritative input for Result Publication.

---

## 9.10 Audit Rule

Every significant Result Processing activity shall generate audit records.

Examples include:

- Marks Aggregation
- Academic Rule Application
- Grace Processing
- Subject Result Computation
- Grade Calculation
- SGPA Calculation
- CGPA Calculation
- Academic Standing Determination
- Result Validation
- Result Approval
- Result Finalisation
- Administrative Override

All business transitions shall remain historically traceable.

---

# 10. Exception Scenarios

The Result Processing & Computation domain supports controlled handling of exceptional academic situations.

---

## 10.1 Missing Final Approved Marks

If Final Approved Marks are unavailable:

- Result Processing shall not begin.
- Responsible authorities shall be notified.
- Processing resumes only after approved marks become available.

---

## 10.2 Regulation Conflict

If conflicting academic regulations are detected:

- Computation shall be suspended.
- Institutional review shall be initiated.
- Processing resumes only after regulatory resolution.

---

## 10.3 Validation Failure

If validation detects computational inconsistencies:

- Results shall return for recomputation.
- Approval shall not proceed until validation succeeds.

---

## 10.4 Approval Rejected

If institutional approval is denied:

- Results return for academic review.
- Corrective actions follow institutional procedures.

---

## 10.5 Academic Benefit Review Required

Where grace, condonation or benefit application requires additional review:

- Result finalisation shall remain pending.
- Authorised academic committees shall determine the outcome.

---

## 10.6 Withheld Result

Where institutional regulations require withholding results:

- Result computation may complete.
- Publication shall remain blocked.
- Appropriate academic authorities shall determine release.

---

## 10.7 Administrative Override

Authorised officers may perform controlled administrative overrides under exceptional circumstances.

Every override shall:

- Require institutional approval.
- Record business justification.
- Preserve complete audit history.
- Publish appropriate business events.

# 11. Events Published

The Result Processing & Computation domain publishes business events representing significant academic computation lifecycle transitions.

These events communicate authoritative academic outcomes to downstream workflows while preserving ownership boundaries.

| Event | Published When | Primary Consumers |
|-------|----------------|-------------------|
| Result Processing Started | Academic computation begins | Audit |
| Marks Aggregated | Assessment aggregation completed | Audit |
| Academic Rules Applied | Academic regulation processing completed | Audit |
| Grace Processing Completed | Grace and benefit rules applied | Audit |
| Subject Result Computed | Subject computation completed | Programme Result Processing |
| Grade Calculated | Grade determination completed | Academic Performance |
| Credit Points Calculated | Credit computation completed | SGPA / CGPA Calculation |
| SGPA Calculated | Semester GPA computed | Programme Result Processing |
| CGPA Calculated | Cumulative GPA computed | Programme Result Processing |
| Programme Result Computed | Programme-level computation completed | Validation |
| Academic Standing Determined | Academic status established | Validation |
| Result Validated | Validation completed successfully | Approval |
| Result Approved | Institutional approval completed | Result Publication |
| Result Finalised | Final result established | L2-I10 |
| Result Ready for Publication | Result processing completed | L2-I10 |
| Result Processing Closed | Workflow completed | Audit |
| Result Archived | Result archived | Reporting, Audit |

Business events represent authoritative academic computation transitions.

---

# 12. Events Consumed

The Result Processing & Computation domain consumes business events published by collaborating domains.

Ownership of business information remains with the originating domain.

| Event | Published By |
|-------|---------------|
| Examination Policy Updated | L2-I01 |
| Examination Session Activated | L2-I02 |
| Examination Schedule Published | L2-I03 |
| Examination Registration Confirmed | L2-I04 |
| Final Approved Marks Available | L2-I08 |
| Academic Calendar Updated | Academic Calendar Domain |
| Candidate Information Updated | Person & Identity Domain |
| Programme Structure Updated | Curriculum Framework Domain |
| Credit Structure Updated | Curriculum Framework Domain |
| Audit Request Raised | Audit Infrastructure |

Consumed events influence Result Processing without transferring ownership.

---

# 13. Multi-Domain Interactions

The Result Processing & Computation domain collaborates with multiple business domains while preserving clear ownership boundaries.

Business ownership always remains with the originating domain.

---

## 13.1 Examination Policy Domain

L2-I01 governs institutional examination regulations.

Business interactions include:

- Grading Policies
- Credit Policies
- Pass Criteria
- Grace Policies
- Classification Rules
- Academic Standing Policies

Result Processing consumes academic regulations without modifying them.

---

## 13.2 Examination Session Management Domain

L2-I02 governs Examination Sessions.

Business interactions include:

- Active Examination Session
- Academic Session Validity
- Session Governance

Result Processing occurs only within authorised examination sessions.

---

## 13.3 Examination Planning & Scheduling Domain

L2-I03 provides approved examination planning information.

Business interactions include:

- Examination Schedule
- Academic Timeline
- Result Processing Window
- Academic Calendar

Scheduling ownership remains external.

---

## 13.4 Examination Registration Domain

L2-I04 provides authorised examination participation.

Business interactions include:

- Registered Candidates
- Registered Subjects
- Programme Registration
- Examination Eligibility

Registration ownership remains outside Result Processing.

---

## 13.5 Marks Collection & Evaluation Domain

L2-I08 provides Final Approved Marks.

Business interactions include:

- Final Approved Marks
- Subject Marks
- Assessment Components
- Approved Evaluation Outcomes

Result Processing consumes approved marks without modifying evaluation history.

---

## 13.6 Curriculum Framework Domain

The Curriculum Framework domain provides academic programme structures.

Business interactions include:

- Programme Structure
- Curriculum Structure
- Subject Credit Values
- Credit Requirements
- Semester Structure
- Programme Regulations

Curriculum ownership remains external.

---

## 13.7 Academic Calendar Domain

The Academic Calendar domain provides academic period definitions.

Business interactions include:

- Academic Year
- Semester
- Term
- Academic Timeline

Calendar ownership remains external.

---

## 13.8 Person & Identity Domain

The Person & Identity domain provides identity information.

Business interactions include:

- Candidate Identity
- Academic Staff Identity
- Result Authority Identity

Identity ownership remains outside the Examination domain.

---

## 13.9 Organisation Domain

The Organisation domain provides institutional organisational structures.

Business interactions include:

- Institution
- Faculty
- Department
- Programme
- Academic Authority

Organisation ownership remains external.

---

## 13.10 Platform Foundation

Platform Foundation provides enterprise infrastructure supporting Result Processing.

Supporting services include:

- Authentication
- Authorization
- Event Infrastructure
- Notification Workflows
- Audit Infrastructure

Platform services support business workflows without owning academic computation.

---

# 14. Ownership Boundaries

The Result Processing & Computation domain owns academic computation from Final Approved Marks until Institutionally Approved Results are ready for publication.

---

## Owned by L2-I09

The following business capabilities are owned by this domain:

- Final Approved Marks Consumption
- Marks Aggregation
- Academic Rule Processing
- Grace & Benefit Processing
- Subject Result Computation
- Credit Point Calculation
- Grade Calculation
- Grade Point Calculation
- SGPA Calculation
- CGPA Calculation
- Programme Result Computation
- Academic Standing Determination
- Result Validation
- Result Approval
- Result Finalisation
- Result Processing Lifecycle Governance

---

## Not Owned by L2-I09

The following business capabilities belong to other workflow documents.

| Business Capability | Owning Document |
|---------------------|-----------------|
| Examination Policy Management | L2-I01 |
| Examination Session Management | L2-I02 |
| Examination Planning & Scheduling | L2-I03 |
| Examination Registration | L2-I04 |
| Hall Ticket Management | L2-I05 |
| Practical & Viva Examination Conduct | L2-I06 |
| Theory Examination Conduct | L2-I07 |
| Marks Collection & Evaluation | L2-I08 |
| Result Publication | L2-I10 |
| Revaluation & Rechecking | L2-I11 |
| Supplementary / Backlog / Improvement Examination Execution | L2-I12 |
| Academic Completion & Completion Evidence | L2-I13 |
| Malpractice & Examination Committee | L2-I14 |
| Examination Exception & Contingency Management | L2-I15 |
| Examination Session Closure & Archival | L2-I16 |

This separation maintains clear ownership across the Examination repository.

---

# 15. Configurable Matrix

Institutions may configure Result Processing & Computation without modifying the canonical repository architecture.

| Configuration Area | Examples |
|--------------------|----------|
| Grading Scheme | Letter Grade, Numeric Grade, Percentage Grade |
| Credit System | CBCS, Credit-Based, Institution-Specific |
| Component Weightage | Theory, Practical, Viva, Internal |
| Grace Policy | Enabled, Disabled, Rule-Based |
| Benefit Rules | Enabled, Disabled |
| Pass Criteria | Marks-Based, Grade-Based, Credit-Based |
| SGPA / CGPA Formula | Institution Configurable |
| Classification Policy | Distinction, First Class, Second Class, Pass |
| Academic Standing Rules | Promotion, Probation, ATKT, Detention |
| Approval Hierarchy | Department, Faculty, Controller, Institution |

Configuration affects operational behaviour while preserving canonical workflow architecture.

---

# 16. Audit Requirements

Every Result Processing activity shall generate auditable business records.

Auditability supports academic integrity, institutional governance, regulatory compliance, dispute resolution, historical traceability, and quality assurance.

---

## Auditable Activities

The following business activities shall be auditable:

- Marks Aggregation
- Academic Rule Processing
- Grace Processing
- Subject Result Computation
- Credit Point Calculation
- Grade Calculation
- Grade Point Calculation
- SGPA Calculation
- CGPA Calculation
- Programme Result Computation
- Academic Standing Determination
- Result Validation
- Result Approval
- Result Finalisation
- Administrative Override
- Result Closure
- Result Archival

Every significant business transition shall be recorded.

---

## Audit Information

Audit records may include:

- Examination Session
- Academic Programme
- Candidate Identifier
- Subject Identifier
- Academic Regulation Version
- Applied Rules
- Grade Awarded
- Grade Point
- SGPA
- CGPA
- Academic Standing
- Approval Authority
- Previous Business State
- New Business State
- Business Justification
- Date and Time
- Approval Reference

Audit implementation is provided by the Platform Audit Infrastructure.

# 17. Workflow Completion Criteria

A Result Processing & Computation workflow is considered complete when:

- Final Approved Marks have been successfully consumed.
- Assessment components have been aggregated.
- Academic regulations have been applied.
- Grace and academic benefit processing has been completed where applicable.
- Subject results have been computed.
- Credit Points have been calculated.
- Grades and Grade Points have been determined.
- SGPA and CGPA have been calculated where applicable.
- Programme results have been computed.
- Academic Standing has been determined.
- Results have been validated.
- Institutional approval has been obtained.
- Final Results have been established.
- Business events have been published.
- Audit records have been generated.
- Results are available for Result Publication.

Completion establishes the authoritative academic result for publication.

---

# 18. Result Processing & Computation Catalogue

The Result Processing & Computation Catalogue defines the canonical business capabilities owned by this domain.

These capabilities are consumed by downstream workflows without redefining academic computation.

| Business Capability | Business Purpose | Primary Consumers |
|---------------------|------------------|-------------------|
| Final Approved Marks Consumption | Accept approved evaluation outcomes | I09 |
| Marks Aggregation | Aggregate assessment components | I09 |
| Academic Rule Processing | Apply institutional regulations | I09 |
| Grace & Benefit Processing | Apply academic benefits | I09 |
| Subject Result Computation | Compute subject outcomes | I09 |
| Credit Point Calculation | Calculate earned credits | I09 |
| Grade Calculation | Determine grades | I09 |
| Grade Point Calculation | Determine grade points | I09 |
| SGPA & CGPA Calculation | Compute cumulative performance | I09 |
| Programme Result Computation | Compute programme outcomes | I09 |
| Academic Standing Determination | Determine academic status | I09 |
| Result Validation | Verify computed results | I09 |
| Result Approval | Obtain institutional approval | I09 |
| Result Finalisation | Produce final academic results | I10 |
| Result Processing Lifecycle Governance | Govern complete computation lifecycle | Audit |

Additional institutional capabilities may be introduced through configuration without modifying the canonical repository architecture.

---

# 19. Result Processing & Computation Dependency Matrix

The Result Processing & Computation domain depends upon business information owned by collaborating domains.

Ownership remains with the originating business domains.

| Business Capability | Depends Upon |
|---------------------|--------------|
| Final Approved Marks Consumption | L2-I08 |
| Marks Aggregation | Final Approved Marks |
| Academic Rule Processing | L2-I01 |
| Grace & Benefit Processing | Academic Regulations |
| Subject Result Computation | Marks Aggregation |
| Credit Point Calculation | Curriculum Framework |
| Grade Calculation | Institutional Grading Scheme |
| Grade Point Calculation | Grade Calculation |
| SGPA & CGPA Calculation | Grade Points & Credits |
| Programme Result Computation | Subject Results |
| Academic Standing Determination | Programme Result |
| Result Validation | Computed Results |
| Result Approval | Validated Results |
| Result Finalisation | Approved Results |
| Result Publication Readiness | Finalised Results |

The dependency matrix establishes business sequencing while preserving ownership boundaries.

---

# 20. Result Processing & Computation State Model

Every academic result progresses through a controlled business lifecycle.

| State | Description |
|--------|-------------|
| Pending | Awaiting Final Approved Marks |
| Aggregated | Marks successfully aggregated |
| Rule Processing | Academic regulations being applied |
| Computed | Academic computation completed |
| Validated | Result successfully validated |
| Approved | Institutionally approved |
| Finalised | Final academic result established |
| Ready for Publication | Available for publication workflows |
| Closed | Result Processing completed |
| Archived | Historical result retained |

Business rules governing state transitions:

- Every Result Processing workflow begins in **Pending**.
- Approved marks produce the **Aggregated** state.
- Regulation application progresses to **Rule Processing**.
- Successful computation produces the **Computed** state.
- Validated results become **Validated**.
- Approved results become **Approved**.
- Institutionally accepted results become **Finalised**.
- Finalised results become **Ready for Publication**.
- Published processing workflows become **Closed**.
- Closed workflows enter **Archived** status.

---

# 21. Configurable Parameters Catalogue

Institutions may configure Result Processing & Computation while preserving repository-standard workflows.

| Configuration Category | Examples |
|------------------------|----------|
| Grading Scheme | Letter, Percentage, Numeric |
| Grade Boundaries | Institution Specific |
| Credit Calculation | CBCS, Credit-Based |
| Component Weightage | Theory, Practical, Internal |
| Grace Marks Policy | Enabled, Disabled |
| Condonation Policy | Institution Specific |
| Benefit Rules | Programme Specific |
| Pass Criteria | Marks, Grade, Credit |
| SGPA Formula | Institution Configurable |
| CGPA Formula | Institution Configurable |
| Classification Rules | Distinction, First Class, Second Class |
| Academic Standing Rules | Promotion, ATKT, Probation, Detention |
| Result Approval Workflow | Single-Level, Multi-Level |

Institution-specific values may vary without changing the canonical workflow architecture.

---

# 22. Domain Responsibility Summary

The Result Processing & Computation domain governs academic computation from Final Approved Marks until Institutionally Approved Results are ready for publication.

| Business Capability | Owned by L2-I09 | Owned by Other Workflow |
|---------------------|-----------------|-------------------------|
| Final Approved Marks Consumption | ✓ | Marks produced by L2-I08 |
| Marks Aggregation | ✓ | — |
| Academic Rule Processing | ✓ | Policies provided by L2-I01 |
| Grace & Benefit Processing | ✓ | Regulations provided by L2-I01 |
| Subject Result Computation | ✓ | — |
| Credit Point Calculation | ✓ | Curriculum provides credit definitions |
| Grade Calculation | ✓ | — |
| Grade Point Calculation | ✓ | — |
| SGPA & CGPA Calculation | ✓ | — |
| Programme Result Computation | ✓ | — |
| Academic Standing Determination | ✓ | — |
| Result Validation | ✓ | — |
| Result Approval | ✓ | — |
| Result Finalisation | ✓ | L2-I10 consumes finalised results |
| Result Publication | — | L2-I10 |
| Revaluation & Rechecking | — | L2-I11 |
| Supplementary Examination Execution | — | L2-I12 |
| Academic Completion & Completion Evidence | — | L2-I13 |

The Result Processing & Computation domain governs academic computation only and does not own publication, revaluation, supplementary examinations, or qualification completion.

---

# 23. Repository Integration Summary

L2-I09 establishes the canonical business workflows governing Result Processing & Computation within the SutraERP Examination domain.

Every academic result shall be computed through the workflows defined in this document.

The relationship between Examination workflow documents is as follows:

| Document | Responsibility |
|----------|----------------|
| L2-I01 | Examination Policy & Configuration |
| L2-I02 | Examination Session Management |
| L2-I03 | Examination Planning & Scheduling |
| L2-I04 | Examination Registration |
| L2-I05 | Hall Ticket Management |
| L2-I06 | Practical & Viva Examination Conduct |
| L2-I07 | Theory Examination Conduct |
| L2-I08 | Marks Collection & Evaluation |
| **L2-I09** | **Result Processing & Computation** |
| L2-I10 | Result Publication |
| L2-I11 | Revaluation & Rechecking |
| L2-I12 | Supplementary / Backlog / Improvement Examination Execution |
| L2-I13 | Academic Completion, CGPA & Completion Evidence Workflows |
| L2-I14 | Malpractice & Examination Committee |
| L2-I15 | Examination Exception & Contingency Management |
| L2-I16 | Examination Session Closure & Archival |

L2-I09 consumes Final Approved Marks from L2-I08 and produces Institutionally Approved Results consumed by L2-I10.

The domain governs academic computation only and does not own academic evaluation, result publication, or post-publication academic changes.

---

# Appendix A — Result Processing & Computation Business Reference Model

This appendix provides the canonical business reference model for Result Processing & Computation.

The appendix defines the conceptual structure governing academic computation while preserving repository ownership boundaries.

---

# A.1 Result Processing Business Model

The Result Processing & Computation domain transforms approved academic evaluation outcomes into institutionally approved academic results.

```text
Final Approved Marks
        │
        ▼
Marks Aggregation
        │
        ▼
Academic Rule Processing
        │
        ▼
Grace & Benefit Processing
        │
        ▼
Subject Result Computation
        │
        ▼
Grade & Credit Calculation
        │
        ▼
SGPA / CGPA Computation
        │
        ▼
Programme Result
        │
        ▼
Academic Standing
        │
        ▼
Validation
        │
        ▼
Approval
        │
        ▼
Finalised Result
        │
        ▼
Ready for Publication
```

---

# A.2 Academic Computation Framework

Academic computation progresses through successive levels.

```text
Assessment Components
        │
        ▼
Subject Result
        │
        ▼
Semester Result
        │
        ▼
Academic Year Result
        │
        ▼
Programme Result
        │
        ▼
Institutional Academic Outcome
```

Each level depends upon successful completion of the preceding computation stage.

---

# A.3 Academic Performance Framework

Institutional academic performance indicators may include:

- Subject Grade
- Grade Point
- Credit Earned
- Credit Points
- SGPA
- CGPA
- Overall Programme Performance
- Academic Standing
- Classification

These indicators represent academic outcomes and are produced only after successful computation.

---

# A.4 Academic Rules Framework

Result Processing applies institutional academic regulations before establishing final results.

Supported rule categories include:

- Pass Rules
- Fail Rules
- Grace Policies
- Benefit Rules
- Credit Rules
- Grade Rules
- Classification Rules
- Promotion Rules
- Academic Standing Rules

Institutions may configure these rules without changing the canonical business workflow.

---

# A.5 Academic Standing Framework

Academic standing is determined after successful computation.

Possible outcomes include:

- Passed
- Failed
- Promoted
- ATKT / Carry Forward
- Probation
- Detained
- Programme Completed
- Withheld

Academic standing governs downstream academic processes.

---

# A.6 Ownership Summary

The Result Processing & Computation domain owns:

- Marks Aggregation
- Academic Rule Processing
- Grace Processing
- Subject Result Computation
- Grade & Grade Point Calculation
- Credit Point Calculation
- SGPA & CGPA Calculation
- Programme Result Computation
- Academic Standing Determination
- Result Validation
- Result Approval
- Result Finalisation

The domain does **not** own:

- Examination Conduct
- Marks Evaluation
- Result Publication
- Revaluation
- Supplementary Examination Execution
- Qualification Completion
- Degree Award

---

# A.7 Governing Principles

The Result Processing & Computation domain is governed by the following principles:

- Only Final Approved Marks may enter Result Processing.
- Academic computation follows approved institutional regulations.
- Evaluation and computation remain separate business responsibilities.
- Grades and academic performance indicators are derived through governed computation.
- Only validated results may be approved.
- Only approved results may be finalised.
- Only Finalised Results are transferred to Result Publication.
- Every computation activity shall remain fully auditable.

---

**End of Document**





