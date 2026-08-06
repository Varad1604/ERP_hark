# SutraERP Repository-Wide Integrity & Semantic Consistency Review

**Review date:** 2026-08-03  
**Review scope:** Entire repository  
**Merge decision:** REJECT  
**Repository files read:** 176  
**Embedded ZIP members inspected:** 15  
**L2 source documents:** 107

## REPOSITORY INTEGRITY SCORECARD

| Integrity Dimension | Score /10 | One-Line Verdict |
|---|---:|---|
| Terminology Consistency | 4.0 | Shared event, status, team, and section terminology is not reliably canonical. |
| Workflow Consistency | 3.0 | Several workflows execute responsibilities explicitly assigned to other domains. |
| Lifecycle & State Consistency | 3.5 | Tenant, StudentContext, Institution, and document lifecycles diverge. |
| Business Rule Consistency | 7.0 | No direct numeric-rule contradiction survived verification, but ownership rules are frequently violated. |
| Ownership Consistency | 2.0 | Physical campus, completion, alumni, HR, leave, and stock ownership conflicts remain. |
| Dependency Consistency | 3.0 | The L1 dependency order conflicts with the canonical map, and runtime dependencies are incompletely registered. |
| Cross-Reference Accuracy | 4.5 | All Markdown links resolve, but semantic references, generated paths, and one filename are wrong. |
| Domain Event Consistency | 2.0 | Globally unique canonical-event rules are violated by numerous semantic name collisions. |
| Aggregate & Entity Consistency | 2.5 | Multiple aggregates and entity lifecycles have competing owners or classifications. |
| Structural & Naming Consistency | 2.5 | L2 templates, headings, statuses, archive placement, and section labels have drifted. |
| **OVERALL INTEGRITY** | **3.4** | **Repository cannot presently serve as an unambiguous implementation source of truth.** |

## FILE-PAIR CONFLICT MAP

Path aliases below are literal:

- L2/ = architecture/L2_Domain_Workflow_Architecture/
- AIC/ = AI_CONTEXT/
- GV/ = governance/constitution/

Brace lists expand to one file pair per listed ID/file. The L2 corpus denotes all 107 canonical L2 Markdown documents enumerated by the matrix.

| File A | File B | Concept in Conflict | Conflict Type | Severity |
|---|---|---|---|---|
| architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md | GV/Volume_1_Governance.md | Architecture-level hierarchy | Type 6 | Critical |
| Same L0 file | GV/Volume_1_Governance.md | L0 technology/workflow boundary | Type 6 | High |
| architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md | L2/TEAM_ASSIGNMENTS.md | Team 1/Team 2 ownership | Type 2 | Critical |
| generated/html/code.html | L2/TEAM_ASSIGNMENTS.md | Tenant-provisioning owner | Type 2 | High |
| L1 locked document | DEPENDENCY_MAP.md | L1 dependency direction/order | Type 4 | Critical |
| L1 locked document | Same file | Version 2.1 versus 2.0 | Type 6 | Medium |
| L1 locked document | L2/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md | Tenant lifecycle | Type 3 | High |
| L1 locked document | generated/html/code.html | Tenant lifecycle | Type 3 | High |
| L2/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md | generated/html/code.html | Tenant lifecycle | Type 3 | High |
| L1 locked document | L2/section-c/L2-C02_Student_Lifecycle_and_Activation.md | StudentContext states | Type 3 | High |
| L2/section-b/L2-B03_Campus_Workflows.md | L2/section-g/L2-G10_Infrastructure_Management_Workflows.md | Physical-campus ownership | Type 2 | Critical |
| L2/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md | L2/section-i/L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md | Academic-completion ownership | Type 2 | Critical |
| L2/section-c/L2-C08_Alumni_Transition_Workflows.md | L2/section-h/L2-H06_Alumni_Workflows.md | Alumni Profile and Membership | Type 2 | Critical |
| L2/section-c/L2-C08_Alumni_Transition_Workflows.md | L2/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md | Closing Student relationship | Type 2 | High |
| L2/section-e/L2-E04_HR_Workflows.md | L2/section-e/L2-E01_Workforce_and_Engagement_Workflows.md | Employment-state workflows | Type 2 | High |
| L2/section-e/L2-E04_HR_Workflows.md | L2/section-e/L2-E05_Leave_Management_Workflows.md | Leave Request ownership | Type 2 | High |
| L2/section-j/L2-J06_Inventory_and_Warehouse_Management.md | L2/section-j/L2-J07_Stock_Operations_and_Distribution.md | Stock issue/return/transfer | Type 2 | High |
| L2/section-j/L2-J06_Inventory_and_Warehouse_Management.md | L2/section-j/L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md | Stock-event publisher | Type 2 | High |
| L2/section-j/L2-J03_Purchase_Requisition_and_Approval_Workflows.md | L2/section-j/L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md | Dependency direction | Type 4 | High |
| L2/section-e/L2-E05_Leave_Management_Workflows.md | L2/section-g/L2-G03_Hostel_Management_Workflows.md | LeaveRequested, LeaveApproved | Type 1 | High |
| L2/section-g/L2-G05_Library_Management_Workflows.md | L2/section-g/L2-G10_Infrastructure_Management_Workflows.md | ReservationCreated, ReservationExpired | Type 1 | High |
| L2/section-i/L2-I14_Malpractice_and_Examination_Committee_Workflows.md | L2/section-j/L2-J01_Procurement_Planning_and_Sourcing_Workflows.md | CommitteeRecommendationRecorded | Type 1 | High |
| L2/section-i/L2-I14_Malpractice_and_Examination_Committee_Workflows.md | L2/section-g/L2-G07_Security_Operations_Workflows.md | InvestigationStarted | Type 1 | High |
| L2/section-d/L2-D03_Timetable_Workflows.md | L2/section-b/L2-B04_Curriculum_Framework_Workflows.md | ConflictDetected | Type 1 | High |
| L2/section-a/L2-A02_Person_and_Identity_Workflows.md | L2/section-b/L2-B04_Curriculum_Framework_Workflows.md | MigrationStarted, MigrationCompleted | Type 1 | High |
| L2/section-c/L2-C03_Student_Academic_Registration_Workflows.md | L2/section-b/L2-B04_Curriculum_Framework_Workflows.md | RegistrationApproved, RegistrationCancelled | Type 1 | High |
| L2/section-i/L2-I15_Examination_Exception_and_Contingency_Workflows.md | L2/section-b/L2-B04_Curriculum_Framework_Workflows.md | ExceptionResolved | Type 1 | High |
| L2/section-g/L2-G01_Facilities_Management_Workflows.md | L2/section-g/L2-G02_Laboratory_Management_Workflows.md | Inspection events | Type 1 | High |
| L2/section-g/L2-G01_Facilities_Management_Workflows.md | L2/section-g/L2-G10_Infrastructure_Management_Workflows.md | Inspection events | Type 1 | High |
| L2/section-g/L2-G02_Laboratory_Management_Workflows.md | L2/section-g/L2-G10_Infrastructure_Management_Workflows.md | Inspection events | Type 1 | High |
| L2/section-g/L2-G02_Laboratory_Management_Workflows.md | L2/section-g/L2-G07_Security_Operations_Workflows.md | IncidentReported, IncidentResolved | Type 1 | High |
| L2/section-g/L2-G01_Facilities_Management_Workflows.md | L2/section-g/L2-G04_Transport_Management_Workflows.md | ComplaintRegistered | Type 1 | High |
| L2/section-g/L2-G03_Hostel_Management_Workflows.md | L2/section-g/L2-G07_Security_Operations_Workflows.md | Visitor lifecycle events | Type 1 | High |
| L2/section-g/L2-G04_Transport_Management_Workflows.md | L2/section-g/L2-G10_Infrastructure_Management_Workflows.md | AllocationUpdated | Type 1 | High |
| L2/section-a/L2-A03_Authentication_Workflows.md | L2/section-h/L2-H10_External_Education_Integration_Workflows.md | Authentication/credential events | Type 1 | High |
| L2/section-a/L2-A02_Person_and_Identity_Workflows.md | L2/section-h/L2-H10_External_Education_Integration_Workflows.md | Synchronization events | Type 1 | High |
| L2/section-h/L2-H01_APAAR_and_DigiLocker_Workflows.md | L2/section-h/L2-H10_External_Education_Integration_Workflows.md | Synchronization events | Type 1 | High |
| GV/Volume_2_Repository_Standards.md | architecture/ARCHITECTURE_INDEX.md | Canonical status vocabulary | Type 1 | Medium |
| GV/Volume_2_Repository_Standards.md | README.md | Canonical status vocabulary | Type 1 | Medium |
| templates/L2_DomainWorkflow_Template.md | GV/Volume_3_Architecture_Framework.md | Required L2 sections | Type 6 | High |
| templates/L2_DomainWorkflow_Template.md | GV/Volume_6_Repository_Reference_Manual.md | Required architecture sections | Type 6 | High |
| templates/L2_DomainWorkflow_Template.md | All 107 L2/section-{a…j}/L2-*.md files | Template-heading conformance | Type 6 | High |
| architecture/ARCHITECTURE_INDEX.md | Same file | Five versus six core documents | Type 8 | Low |
| GV/Volume_2_Repository_Standards.md | L2/{C01,C02,C03,C08,D01,D04,E01,E06,F08,G00,G01,G02,G03,G04,G05,G06,G07,G08,G10,I14,I15,I16} | Missing, duplicate, or noncanonical status | Type 8 | High |
| architecture/ARCHITECTURE_INDEX.md | L2/{B03,B05,D05,D07,E04,E05,E06,H09} | H1 naming format | Type 6 | Medium |
| architecture/ARCHITECTURE_INDEX.md | L2/{D01,E06,E07,F00,F01} | Section-name drift | Type 6 | Medium |
| L2/section-d/L2-D01_Course_and_Course_Offering_Workflows.md | NONE | Missing L2-D01_Course_Offering_Workflows.md | Type 5 | Low |
| L2/section-f.zip members | Corresponding 15 live L2/section-f/L2-F00…F14 files | Undeclared stale duplicate architecture | Type 6 | High |
| prompts/Prompt_0_AI_Context_Repository_Bootstrap.md | AIC/SECTION_{A…I}/{A…I}_Canonical_AI_Context.md | Generated basename | Type 5 | Medium |
| prompts/Prompt_1_Canonical_AI_Context_Generator.md | Same nine context files | Generated basename | Type 5 | Medium |
| prompts/Prompt_2_Dependency_Contract_Generator.md | Same nine context files | Input basename | Type 5 | Medium |
| L2/section-j/README.md | architecture/ARCHITECTURE_INDEX.md | Section A trust-source names | Type 5 | Medium |
| AIC/REPO_UNDERSTANDING.md | architecture/ARCHITECTURE_INDEX.md | Claimed matrix/index divergence | Type 8 | Medium |
| AIC/REPO_UNDERSTANDING.md | AIC/INDEX.md | D/I contexts claimed pending | Type 8 | Medium |
| AIC/REPO_UNDERSTANDING.md | L1 locked document | L1 version | Type 8 | Medium |
| AIC/SECTION_A/A_Canonical_AI_Context.md | L2/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md | Institution ownership | Type 2 | High |
| AIC/SECTION_A/A_Canonical_AI_Context.md | L2/section-a/L2-A10_Operational_Readiness_Workflows.md | A10 source coverage | Type 8 | Medium |
| AIC/SECTION_A/Dependency_Contract.md | L2/section-a/L2-A10_Operational_Readiness_Workflows.md | A10 dependency coverage | Type 8 | Medium |
| AIC/SECTION_B/B_Canonical_AI_Context.md | L2/section-g/L2-G10_Infrastructure_Management_Workflows.md | Physical-infrastructure ownership | Type 2 | Critical |
| AIC/SECTION_B/B_Canonical_AI_Context.md | L2/section-b/L2-B03_Campus_Workflows.md | Physical-infrastructure ownership | Type 2 | Critical |
| AIC/SECTION_B/B_Canonical_AI_Context.md | L2/section-b/L2-B01_Institution_Workflows.md | Institution lifecycle | Type 3 | High |
| AIC/SECTION_C/C_Canonical_AI_Context.md | L2/section-c/L2-C08_Alumni_Transition_Workflows.md | Alumni entities | Type 2 | High |
| AIC/SECTION_C/C_Canonical_AI_Context.md | L2/section-h/L2-H06_Alumni_Workflows.md | Alumni entities | Type 2 | High |
| AIC/SECTION_E/E_Canonical_AI_Context.md | L2/section-e/L2-E01_Workforce_and_Engagement_Workflows.md | Employment entities | Type 2 | High |
| AIC/SECTION_E/E_Canonical_AI_Context.md | L2/section-e/L2-E04_HR_Workflows.md | HR-case entities | Type 2 | High |
| AIC/SECTION_E/E_Canonical_AI_Context.md | L2/section-e/L2-E05_Leave_Management_Workflows.md | Leave Request owner | Type 2 | High |
| AIC/SECTION_F/F_Canonical_AI_Context.md | L2/section-f/L2-F00_Finance_Domain_Model_and_Foundation.md | Institution/Campus ownership | Type 2 | High |
| generated/reviews/integrity-2026-07-31-complete.md | Same file | Closed/Pending/REJECT conclusions | Type 8 | Critical |
| DEPENDENCY_MAP.md | L2/section-f/L2-F14_Procurement_Finance_Consumption_Workflows.md | Finance-to-Section-J dependency | Type 4 | Medium |

## FULL FINDINGS

---

**Issue ID:** INT-001  
**Conflict Type:** Type 6 — Structural Drift  
**File A:** architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md  
**File B:** governance/constitution/Volume_1_Governance.md  
**Concept:** Architecture hierarchy  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** L0 §0, corroborated by Volume 1: L0 Business Vision → L1 Business Domain → L2 Domain Workflow → L3 Business Scenario → L4 System Behavior → L5 Solution Architecture → L6 UI/UX.

**Conflict Description:** L0 lines 14–42 declares that hierarchy final. Its later §23 instead labels L1 “Complete Institute Lifecycle,” L4 “Individual Module Workflows,” L5 role/position workflows, and L6 screen/API/contract/event/state workflows. Section 25 then calls “Complete Institute Lifecycle” the next L1 stage even though the repository already contains a locked L1 Business Domain document.

**Repository Impact:** Authors cannot determine which content belongs at each level; generated L3–L6 work can be placed under incompatible meanings.

**Resolution:** Replace §§23 and 25 with the §0 hierarchy and identify the existing locked L1 document as the parent for L2.

---

**Issue ID:** INT-002  
**Conflict Type:** Type 6 — Structural Drift  
**File A:** Same L0 document  
**File B:** governance/constitution/Volume_1_Governance.md  
**Concept:** Permitted L0 content  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** Volume 1 lines 152–168: L0 never defines workflows; the level is business-facing and technology-independent.

**Conflict Description:** L0 lines 260–320 prescribes Rust, Axum 0.7, Tokio, PostgreSQL 16, SQLx, PgBouncer, Redis, React 19, Vite, TypeScript, Zustand, and Axios. It also contains detailed event and workflow behavior, including a Tokio event bus.

**Repository Impact:** Technology decisions and system behavior become falsely elevated to immutable business vision.

**Resolution:** Move implementation stacks to L5 and behavioral workflows to L2–L4. Retain only technology-independent business objectives in L0.

---

**Issue ID:** INT-003  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** L1 locked document; generated/html/code.html  
**File B:** architecture/L2_Domain_Workflow_Architecture/TEAM_ASSIGNMENTS.md  
**Concept:** Repository ownership model  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** TEAM_ASSIGNMENTS.md, explicitly named canonical by L0 lines 1119–1129.

**Conflict Description:** L0 says Team 1/Team 2 ownership is obsolete. L1 lines 2739–2776 nevertheless allocates domains between “Team 1” and “Team 2.” code.html assigns Tenant Provisioning to Team 2. The assignment file instead allocates Sections A–J to named architects.

**Repository Impact:** Review, change authority, and accountability can be routed to nonexistent teams.

**Resolution:** Remove Team 1/Team 2 allocations from L1 and generated material. Replace them with section ownership and named owners from TEAM_ASSIGNMENTS.md, then regenerate HTML.

---

**Issue ID:** INT-004  
**Conflict Type:** Type 4 — Dependency Conflict  
**File A:** L1 locked document  
**File B:** DEPENDENCY_MAP.md  
**Concept:** L1 domain dependency graph  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** DEPENDENCY_MAP.md lines 101–117, the repository’s declared foundational dependency register.

**Conflict Description:** L1 lines 3055–3084 describes a serial order ending with Communication and then Workflow Engine. The map makes Section A, including the Workflow Engine, foundationally upstream. It also gives Finance dependencies A/B/C, not the Exam-before-Finance order implied by L1.

**Repository Impact:** Teams can construct initialization and integration sequencing in the wrong direction, including treating a foundational service as terminal.

**Resolution:** Replace the L1 serial chain with the exact directed matrix from DEPENDENCY_MAP.md; distinguish foundational, runtime-event, and read-model relationships.

---

**Issue ID:** INT-005  
**Conflict Type:** Type 6 — Structural Drift  
**File A:** L1 locked document  
**File B:** Same file  
**Concept:** Document version  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** The document header, which declares Version 2.1.

**Conflict Description:** Line 3 states Version 2.1; the closing metadata at line 3091 states Version 2.0.

**Repository Impact:** Citations, generated copies, and change reviews cannot identify the applicable baseline.

**Resolution:** Change the closing version to 2.1 and regenerate derived HTML.

---

**Issue ID:** INT-006  
**Conflict Type:** Type 3 — Lifecycle Conflict  
**File A:** L1 locked document  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md and generated/html/code.html  
**Concept:** Tenant lifecycle  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** L1 lines 937–951, pending a deliberate L1 amendment.

**Conflict Description:** L1 defines Requested → Provisioning → Active, with Failed returning to Provisioning. A01 adds Maintenance, Suspended, Upgrade, and Archived; its diagram includes Upgrade while its state table omits it. code.html returns to the short Requested/Provisioning/Active model with PROVISIONING_FAILED.

**Repository Impact:** Persistence enums, transition validation, archival behavior, and generated code will disagree.

**Resolution:** Decide whether the expanded lifecycle is required; amend L1 first if so. Then make A01’s diagram and table identical and regenerate code.html.

---

**Issue ID:** INT-007  
**Conflict Type:** Type 3 — Lifecycle Conflict  
**File A:** L1 locked document  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-c/L2-C02_Student_Lifecycle_and_Activation.md  
**Concept:** StudentContext lifecycle  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** L1, because it is the higher architecture level.

**Conflict Description:** L1 §3.5 requires a prior context to be Completed, Qualification Completed, Withdrawn, Transferred, Discontinued, or Terminated. Its own state machine instead uses PendingActivation, Active, QualificationCompleted, Withdrawn, Transferred, Discontinued, and Terminated—without Completed. C02 calls its list canonical, adds Suspended, and likewise omits Completed.

**Repository Impact:** “Can a new context be created?” has different answers depending on which passage is implemented.

**Resolution:** Establish one exact StudentContext state enumeration in L1, including whether Completed and Suspended are valid, then copy it verbatim into C02.

---

**Issue ID:** INT-008  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-g/L2-G10_Infrastructure_Management_Workflows.md  
**Concept:** Physical campus, building, floor, room, and space ownership  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** G10, supported by B03’s amended opening boundary.

**Conflict Description:** B03 lines 25–33 says it owns the institutional campus concept, not physical structures, and points to G10. Later, B03 says Campus & Infrastructure owns every physical location and its complete lifecycle. G10 expressly owns buildings, floors, rooms, spaces, and their physical lifecycle and says Section B does not.

**Repository Impact:** Two aggregates may be created for the same physical place, with conflicting identifiers, lifecycle authority, and events.

**Resolution:** Delete or rewrite B03’s later physical-ownership passages. Restrict B03 to institutional campus identity and references to G10 physical-space IDs.

---

**Issue ID:** INT-009  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-i/L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md  
**Concept:** Academic-completion eligibility and approval  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** C05, the Student Lifecycle owner named at the higher-level domain boundary.

**Conflict Description:** C05 owns validation of academic completion, qualification authorization, degree conferment, and all academic-completion/exit capabilities. I13 disclaims final qualification determination but then establishes “canonical academic completion,” determines completion eligibility, governs approval, owns Completion Verification and Completion Eligibility, and maintains an academic-completion register.

**Repository Impact:** Separate services can issue competing completion decisions and evidence for the same student.

**Resolution:** Restrict I13 to examination-derived CGPA and evidence publication. C05 alone should decide StudentContext completion, qualification authorization, and exit.

---

**Issue ID:** INT-010  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-c/L2-C08_Alumni_Transition_Workflows.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-h/L2-H06_Alumni_Workflows.md  
**Concept:** Alumni Profile and Membership  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** H06 for the post-transition alumni relationship; C08 itself acknowledges this boundary.

**Conflict Description:** C08 says it is transition-only and does not create or manage Alumni Profile, Membership, or engagement. Its sequence later performs “Create Alumni Profile” and “Activate Alumni Membership,” and its ownership diagram links transition to Membership, Profile, Benefits, and Engagement. H06 says those are its canonical lifecycle responsibilities after consuming AlumniTransitionCompleted.

**Repository Impact:** Alumni records can be created twice or activated before the H06 aggregate consumes the transition event.

**Resolution:** Replace C08’s create/activate operations with publication of AlumniTransitionCompleted; H06 must create Profile and Membership.

---

**Issue ID:** INT-011  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-c/L2-C08_Alumni_Transition_Workflows.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md  
**Concept:** Closing the Student relationship  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** C05, which owns qualification completion and Student exit.

**Conflict Description:** C08’s sequence executes “Close Student Relationship,” although its stated scope is only the transition after exit. C05 owns StudentContext closure and exit authorization.

**Repository Impact:** The StudentContext could be terminated by two workflows or before exit prerequisites are satisfied.

**Resolution:** C08 must consume C05’s completed-exit fact and must not mutate or close StudentContext.

---

**Issue ID:** INT-012  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-e/L2-E04_HR_Workflows.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-e/L2-E01_Workforce_and_Engagement_Workflows.md  
**Concept:** Employment-state workflows  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** E01 for WorkforceEngagement lifecycle; E04 for case orchestration only.

**Conflict Description:** E04’s opening boundary says it never owns employment state and invokes E01. Its later catalogue assigns onboarding, probation, confirmation, promotion, transfer, and separation to Employee Onboarding, Employment Change, and Separation aggregates and states that onboarding manages employee activation.

**Repository Impact:** Case orchestration and employment state can both emit authoritative outcomes.

**Resolution:** Remove employment aggregates and state-changing events from E04. E04 may coordinate cases and request E01 transitions.

---

**Issue ID:** INT-013  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-e/L2-E04_HR_Workflows.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-e/L2-E05_Leave_Management_Workflows.md  
**Concept:** Leave Request lifecycle  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** E05, also acknowledged by E04 lines 1025–1045.

**Conflict Description:** E04 says Leave Request is exclusively E05-owned. Later it lists a Leave Workflow with a Leave Request Aggregate, an approval progression from Submitted to Approved/Rejected, and LeaveApproved as an E04 aggregate event.

**Repository Impact:** Leave approval and balance consequences can be executed through two bounded contexts.

**Resolution:** Delete the leave aggregate, approval states, and outcome event from E04. Retain only an HR-case reference to an E05 LeaveRequest ID.

---

**Issue ID:** INT-014  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-j/L2-J06_Inventory_and_Warehouse_Management.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-j/L2-J07_Stock_Operations_and_Distribution.md; architecture/L2_Domain_Workflow_Architecture/section-j/L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md  
**Concept:** Stock issue, return, and transfer  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** J07, corroborated by J00’s canonical publisher matrix.

**Conflict Description:** J06 says it governs reservation, issue, return, and transfer and publishes StockIssued and StockReturned. J07 claims the same operational movement lifecycle. J00 assigns those events to J07.

**Repository Impact:** Stock balances may be mutated twice, and consumers cannot choose the authoritative publisher.

**Resolution:** Limit J06 to Item/Warehouse/Stock Ledger ownership. Move operational issue, return, transfer, and their events exclusively to J07.

---

**Issue ID:** INT-015  
**Conflict Type:** Type 4 — Dependency Conflict  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-j/L2-J03_Purchase_Requisition_and_Approval_Workflows.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-j/L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md  
**Concept:** J03 dependency direction  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** J00’s rule that “depends on” means requiring an upstream capability.

**Conflict Description:** J03 states that it depends on J01 “for downstream sourcing” and J04 “for order creation after release.” J00 instead models J03 as producing approved demand consumed downstream by J01/J04.

**Repository Impact:** Dependency tooling detects false cycles and may initialize consumers before their source.

**Resolution:** Change J03’s J01/J04 declarations to Referenced By or Downstream Consumers; retain only genuine upstream requirements under Depends On.

---

**Issue ID:** INT-016  
**Conflict Type:** Type 1 — Terminology Conflict  
**File A:** Multiple L2 publishers listed in the conflict map  
**File B:** Their paired publishers  
**Concept:** Global canonical-event namespace  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** A06 lines 780–804, 832, and 1102–1104: a canonical event name has one semantic definition and is globally unique.

**Conflict Description:** Exact event names are reused for different aggregates and meanings:

- Employee leave versus hostel temporary leave: LeaveRequested, LeaveApproved.
- Library reservation versus physical-space reservation: ReservationCreated, ReservationExpired.
- Examination committee versus procurement committee: CommitteeRecommendationRecorded.
- Malpractice versus security investigation: InvestigationStarted.
- Timetable versus curriculum conflict: ConflictDetected.
- Person-data versus curriculum migration: MigrationStarted, MigrationCompleted.
- Student-academic versus generic curriculum registration: RegistrationApproved, RegistrationCancelled.
- Examination contingency versus curriculum exception: ExceptionResolved.
- Facilities/laboratory/infrastructure inspections: InspectionScheduled, InspectionStarted, InspectionCompleted.
- Laboratory versus security incident: IncidentReported, IncidentResolved.
- Facilities versus transport complaint: ComplaintRegistered.
- Hostel versus security visitor lifecycle: VisitorRegistered, VisitorApproved, VisitorCheckedIn, VisitorCheckedOut.
- Transport versus infrastructure allocation: AllocationUpdated.
- Platform authentication versus external-provider authentication: AuthenticationFailed, CredentialExpired.
- Identity, APAAR, and external-integration synchronization: SynchronizationStarted, SynchronizationCompleted.

**Repository Impact:** Event routing, schemas, subscriptions, observability, and replay cannot infer which fact a name represents.

**Resolution:** Establish a registry keyed by canonical name and rename collisions with aggregate/domain qualifiers, such as EmployeeLeaveApproved, HostelLeaveApproved, and LibraryReservationCreated.

---

**Issue ID:** INT-017  
**Conflict Type:** Type 1 — Terminology Conflict  
**File A:** governance/constitution/Volume_2_Repository_Standards.md  
**File B:** architecture/ARCHITECTURE_INDEX.md; README.md  
**Concept:** Document-status vocabulary  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** Volume 2 lines 127–147, which requires exactly one of 13 status values.

**Conflict Description:** The index legend lists nine statuses and says only Proposed, Deprecated, and Archived were omitted, although Approved is also absent. README reduces the lifecycle to Draft, Review, TCR, and Locked.

**Repository Impact:** Status validators, reviewers, and contributors use different legal value sets.

**Resolution:** Publish the 13-value list once in governance and link to it. Correct the index omission count and label README’s four values as a simplified progression, not the full vocabulary.

---

**Issue ID:** INT-018  
**Conflict Type:** Type 6 — Structural Drift  
**File A:** templates/L2_DomainWorkflow_Template.md  
**File B:** Volumes 3 and 6; all 107 L2 documents  
**Concept:** Required L2 structure  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** Volume 3 lines 282–297, supported by Volume 6.

**Conflict Description:** Governance requires Purpose, Scope, Actors, Entities, Business Rules, Workflow, State Machine, Contracts, Events, Dependencies, and Locked Decisions. The template instead has eight differently named headings and omits Scope, Actors, Contracts, and Locked Decisions. An exact-heading scan found no L2 file matching all eight template headings: Purpose 106/107; Domain Lifecycle 1; Domain Workflows 12; Domain States 0; Domain Rules 0; Domain Events 52; Domain Dependencies 1; Domain Entities 2.

**Repository Impact:** Template conformance cannot be tested, and structurally equivalent content is labeled inconsistently.

**Resolution:** Replace the template headings with the Volume 3 canonical set, then normalize all 107 L2 files or formally version the structure.

---

**Issue ID:** INT-019  
**Conflict Type:** Type 8 — Matrix/Index Mismatch  
**File A:** architecture/ARCHITECTURE_INDEX.md  
**File B:** Same file  
**Concept:** Core-document count  
**Severity:** Low  
**Confidence:** Certain

**Canonical Source:** The index’s actual core-document table.

**Conflict Description:** The table contains six core documents, while statistics say five and calculate 112 total as 107 + 5.

**Repository Impact:** Repository completeness metrics are wrong.

**Resolution:** Change the core count to 6 and total to 113 if the table’s six entries are intended to count.

---

**Issue ID:** INT-020  
**Conflict Type:** Type 8 — Matrix/Index Mismatch  
**File A:** 22 L2 files identified below  
**File B:** governance/constitution/Volume_2_Repository_Standards.md  
**Concept:** Exactly one canonical document status  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** Volume 2’s status rule.

**Conflict Description:** C01 says Draft at the top and LOCKED at the end; C02 says Locked, Approved Baseline, and “Baseline Approved”; C03 has no document status; C08 says Draft, Architecture Complete, and Baseline; D01 and D04 combine locked statuses with “Repository Architecture Complete” or COMPLETE; E01 is Draft and Architecture Complete; E06 lacks a canonical header status and says Architecture Defined; F08 is Draft and “Ready for Architecture Lock.” G00–G08/G10 and I14–I16 combine Draft with noncanonical “Architecture Draft” variants.

**Repository Impact:** The matrix may agree with one header while the document communicates a different effective state elsewhere.

**Resolution:** Give each affected file one metadata status from the canonical vocabulary and remove status-like prose footers. Affected IDs: C01, C02, C03, C08, D01, D04, E01, E06, F08, G00–G08 excluding G09, G10, and I14–I16.

---

**Issue ID:** INT-021  
**Conflict Type:** Type 6 — Structural Drift  
**File A:** Twelve L2 documents in the conflict map  
**File B:** architecture/ARCHITECTURE_INDEX.md  
**Concept:** H1 and section naming  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** Architecture Index and README section names.

**Conflict Description:** B03 has no canonical separator; B05 and H09 use en dashes; D05 appends .md; D07 uses a hyphen; E04–E06 use triple hyphens. D01 calls Section D “Academic Operations”; E06/E07 alter Section E; F00/F01 call Section F “Finance & Accounting” rather than “Finance.”

**Repository Impact:** Heading-based parsers and human cross-references produce different names for the same section.

**Resolution:** Normalize H1 as # L2-Xnn — Canonical Name and copy section names verbatim from the index.

---

**Issue ID:** INT-022  
**Conflict Type:** Type 5 — Broken Reference  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-d/L2-D01_Course_and_Course_Offering_Workflows.md  
**File B:** NONE  
**Concept:** D01 filename  
**Severity:** Low  
**Confidence:** Certain

**Canonical Source:** The actual indexed D01 file.

**Conflict Description:** D01 lines 559 and 3229 cite L2-D01_Course_Offering_Workflows.md, which does not exist. The actual file is L2-D01_Course_and_Course_Offering_Workflows.md.

**Repository Impact:** Filename-based tooling and copied references fail.

**Resolution:** Replace both stale basenames with the actual filename.

---

**Issue ID:** INT-023  
**Conflict Type:** Type 6 — Structural Drift  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-f.zip  
**File B:** The 15 live Section F files  
**Concept:** Undeclared archived duplicate source  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** Live section-f/ files; README reserves archived material for archive/.

**Conflict Description:** The ZIP sits in the active architecture folder with no deprecation marker. All 15 embedded Markdown files differ byte-for-byte from their live counterparts; for example, F00’s ZIP SHA-256 starts 853C5D8FA396, while live F00 starts 0D03647724AA.

**Repository Impact:** Search, AI ingestion, and manual extraction can select a stale competing architecture.

**Resolution:** Move the ZIP into a dated archive/ location with provenance or remove it after confirming recoverability. Mark it explicitly noncanonical.

---

**Issue ID:** INT-024  
**Conflict Type:** Type 5 — Broken Reference  
**File A:** Prompt 0, Prompt 1, and Prompt 2  
**File B:** Actual Section A–I AI context files  
**Concept:** AI-context basename convention  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** Existing indexed files: A_Canonical_AI_Context.md through I_Canonical_AI_Context.md.

**Conflict Description:** Prompt 0 line 215 and Prompt 1 line 37 generate SECTION_{{PREFIX}}/Canonical_AI_Context.md; Prompt 2 reads SECTION_*/Canonical_AI_Context.md. Only Section J has that unprefixed alias. Sections A–I use prefixed basenames.

**Repository Impact:** Generators create parallel files and the dependency prompt misses nine canonical contexts.

**Resolution:** Change all three prompts to SECTION_{{PREFIX}}/{{PREFIX}}_Canonical_AI_Context.md, or rename all contexts through one controlled migration.

---

**Issue ID:** INT-025  
**Conflict Type:** Type 5 — Broken Reference  
**File A:** architecture/L2_Domain_Workflow_Architecture/section-j/README.md  
**File B:** architecture/ARCHITECTURE_INDEX.md  
**Concept:** Section A trust-source names  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** Architecture Index and actual filenames.

**Conflict Description:** Section J README maps A03 to “Identity and Access Management,” A04 to “Authorization and Entitlements,” A05 to “Workflow and Business Process Foundation,” A06 to “Notification and Eventing Architecture,” and A07 to “Audit, Compliance and Evidence Architecture.” Actual names are Authentication, Authorization & Policy, Workflow Engine, Event Infrastructure, and Audit Infrastructure. A06 particularly conflates Section A events with Section H notification ownership.

**Repository Impact:** Readers following trust-source labels reach the right IDs but infer the wrong capability boundaries.

**Resolution:** Replace all five labels with the exact index names.

---

**Issue ID:** INT-026  
**Conflict Type:** Type 8 — Matrix/Index Mismatch  
**File A:** AI_CONTEXT/REPO_UNDERSTANDING.md  
**File B:** Architecture Index, AI Context Index, and L1  
**Concept:** Repository-state summary  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** Current files and inventory.

**Conflict Description:** The context claims matrix/index divergence although the current 107-ID sets, owners, and top statuses match mechanically. It says D/I contexts are pending while AI_CONTEXT/INDEX.md says all A–J are generated and those files exist. It reports L1 2.0 although the L1 header is 2.1, and admits the section contexts were not read.

**Repository Impact:** The mandatory AI entry point teaches stale repository facts before an AI reads authoritative files.

**Resolution:** Regenerate it from the current inventory, record its evidence date, and distinguish verified facts from historical observations.

---

**Issue ID:** INT-027  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** AI_CONTEXT/SECTION_A/A_Canonical_AI_Context.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md  
**Concept:** Institution ownership  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** L1/Section B ownership; A01 preserves only the platform-to-institution relationship.

**Conflict Description:** The Section A context lists Institution among A-owned entities. A01 states B01 owns the Institution lifecycle and A01 owns only platform tenant foundation and relationship concerns.

**Repository Impact:** AI-generated code can place the Institution aggregate in the platform domain.

**Resolution:** Replace “Institution” under A-owned entities with an external InstitutionId/relationship reference and name B01 as owner.

---

**Issue ID:** INT-028  
**Conflict Type:** Type 8 — Matrix/Index Mismatch  
**File A:** AI_CONTEXT/SECTION_A/A_Canonical_AI_Context.md; AI_CONTEXT/SECTION_A/Dependency_Contract.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-a/L2-A10_Operational_Readiness_Workflows.md  
**Concept:** A10 context coverage  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** The Section A index/hierarchy, which contains A00–A10.

**Conflict Description:** The context says it manually corrected and represents all 11 files and lists A10 in its hierarchy, but its source mapping ends at A09. The dependency contract contains no A10 reference.

**Repository Impact:** Operational readiness is visible in navigation but absent from the asserted source and dependency model.

**Resolution:** Add A10 source mapping, ownership, events, dependencies, and contract implications—or narrow the context’s completeness claim.

---

**Issue ID:** INT-029  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** AI_CONTEXT/SECTION_B/B_Canonical_AI_Context.md  
**File B:** B03 and G10  
**Concept:** Physical infrastructure  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** G10 and B03’s amended opening boundary.

**Conflict Description:** The Section B context says Campus models physical infrastructure and lists Campus Site, Building, Wing, Floor, Room, and Lab as B-owned. G10 owns those concepts. B03’s opening agrees with G10, although its stale later sections do not.

**Repository Impact:** The canonical AI context instructs agents to regenerate the exact ownership defect the architecture attempted to correct.

**Resolution:** Rebuild the Section B context from B03’s amended boundary and G10. Section B should own institutional campus identity, not physical-space aggregates.

---

**Issue ID:** INT-030  
**Conflict Type:** Type 3 — Lifecycle Conflict  
**File A:** AI_CONTEXT/SECTION_B/B_Canonical_AI_Context.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflows.md  
**Concept:** Institution lifecycle  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** B01, aligned with the locked L1 lifecycle.

**Conflict Description:** The AI context lists Draft, Configured, Active, Suspended, and Archived. B01 includes Pending Approval, Returned, and Rejected approval states that the context omits.

**Repository Impact:** Generated approval logic may activate an institution without the canonical review path.

**Resolution:** Copy the complete B01 enumeration and transition rules into the Section B context verbatim.

---

**Issue ID:** INT-031  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** AI_CONTEXT/SECTION_C/C_Canonical_AI_Context.md  
**File B:** C08 and H06  
**Concept:** Alumni entities  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** H06 for Alumni Profile/Membership; C08 for the transition only.

**Conflict Description:** The context attributes Alumni Profile, Alumni Status, Alumni Membership, and Qualification Record to C08. C08 expressly disclaims Profile and Membership ownership and delegates the post-transition relationship to H06.

**Repository Impact:** AI consumers will place H06 aggregates in Section C.

**Resolution:** Assign C08 only AlumniTransition state/evidence. Assign Profile, Membership, benefits, and engagement to H06.

---

**Issue ID:** INT-032  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** AI_CONTEXT/SECTION_E/E_Canonical_AI_Context.md  
**File B:** E01, E04, and E05  
**Concept:** Employment, position, and leave entities  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** E01 for WorkforceEngagement/position assignment, E04 for HR cases, E05 for Leave Request.

**Conflict Description:** The context assigns Position/Responsibility Assignment to both E01 and E02; Leave Request to both E04 and E05; and onboarding, employment change, and separation aggregates to E04 despite its case-only boundary.

**Repository Impact:** The AI context reproduces duplicate aggregates across three workforce modules.

**Resolution:** Replace its owned-entity table with a single-owner mapping derived from E01–E05 and delete duplicate entries.

---

**Issue ID:** INT-033  
**Conflict Type:** Type 2 — Ownership Conflict  
**File A:** AI_CONTEXT/SECTION_F/F_Canonical_AI_Context.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-f/L2-F00_Finance_Domain_Model_and_Foundation.md  
**Concept:** Institution and Campus entities  
**Severity:** High  
**Confidence:** Certain

**Canonical Source:** F00’s ownership matrix and Sections B/G.

**Conflict Description:** The Finance context lists Institution, Campus, and Approval Workflow as owned entities. F00 says Institution and Campus are upstream canonical entities that Finance must not create, update, or delete.

**Repository Impact:** Finance implementations can create shadow organizational or campus records.

**Resolution:** Reclassify Institution and Campus as upstream references. Treat approvals as Section A workflow capabilities unless a specifically named Finance aggregate is defined.

---

**Issue ID:** INT-034  
**Conflict Type:** Type 8 — Matrix/Index Mismatch  
**File A:** generated/reviews/integrity-2026-07-31-complete.md  
**File B:** Same file  
**Concept:** Prior certification outcome  
**Severity:** Critical  
**Confidence:** Certain

**Canonical Source:** Current repository evidence and the report’s own detailed status table.

**Conflict Description:** The report header says all 39 findings are closed, none are pending, and repository integrity is effectively complete. Its “Status by Original Priority” still contains partially closed and pending work, while its merge decision remains REJECT with blocking findings.

**Repository Impact:** A consumer reading only the headline can treat rejected, unresolved architecture as certified.

**Resolution:** Withdraw the “complete” label or reissue the report with one internally consistent outcome. Never mark a finding closed unless its evidence and merge decision agree.

---

**Issue ID:** INT-035  
**Conflict Type:** Type 4 — Dependency Conflict  
**File A:** DEPENDENCY_MAP.md  
**File B:** architecture/L2_Domain_Workflow_Architecture/section-f/L2-F14_Procurement_Finance_Consumption_Workflows.md  
**Concept:** Runtime cross-section dependency coverage  
**Severity:** Medium  
**Confidence:** Certain

**Canonical Source:** Both files, once dependency classes are separated; this is an unregistered runtime dependency rather than an ownership reversal.

**Conflict Description:** The map records only foundational dependencies and lists Finance as depending on A/B/C. F14 metadata explicitly says Depends On … Section J and defines Finance consumption of Section J procurement/resource facts. The map contains no separate runtime-contract register for this F→J dependency.

**Repository Impact:** A reader treating DEPENDENCY_MAP.md as the repository dependency register cannot discover a required integration.

**Resolution:** Add distinct foundational, runtime-contract, event-consumption, and read-model dependency sections. Register F14 → Section J as runtime/event consumption without changing foundational ownership direction.

---

## CONSISTENCY MATRIX

| Concept | Canonical File | Canonical Definition | Conflicting File(s) | Status | Severity |
|---|---|---|---|---|---|
| Architecture hierarchy | L0 §0 | L0 Vision → L1 Domain → L2 Workflow → L3 Scenario → L4 Behavior → L5 Solution → L6 UI/UX | L0 §§23,25 | Contradictory | Critical |
| L0 content boundary | Volume 1 | Business vision; no workflow or implementation stack | L0 technology/workflow sections | Contradictory | High |
| Assignment model | TEAM_ASSIGNMENTS.md | Named architect per Section A–J | L1, code.html | Contradictory | Critical |
| Foundational dependency graph | DEPENDENCY_MAP.md | Directed section matrix | L1 serial order | Contradictory | Critical |
| L1 version | L1 header | 2.1 | L1 footer | Contradictory | Medium |
| Tenant lifecycle | L1 | Requested/Provisioning/Active/Failed baseline | A01, code.html | Ambiguous | High |
| StudentContext lifecycle | L1 | One terminal-state gate and state machine | C02 and L1 §3.5 | Contradictory | High |
| Physical campus | G10 | G10 owns physical spaces; B03 owns institutional campus identity | B03, AI Section B | Contradictory | Critical |
| Academic completion | C05 | C05 decides completion and Student exit | I13 | Duplicate | Critical |
| Alumni Profile/Membership | H06 | Created after transition event | C08, AI Section C | Duplicate | Critical |
| Student closure | C05 | C05 closes StudentContext/relationship | C08 | Duplicate | High |
| WorkforceEngagement | E01 | E01 owns employment lifecycle | E04, AI Section E | Duplicate | High |
| Leave Request | E05 | E05 owns request and approval lifecycle | E04, AI Section E | Duplicate | High |
| Stock operations | J07/J00 | J07 owns issue/return/transfer and events | J06 | Duplicate | High |
| J03 downstream consumers | J00 | J01/J04 consume J03 output | J03 dependencies | Contradictory | High |
| Canonical event names | A06 | Globally unique semantic names | 18 publisher pairs | Duplicate | High |
| Document statuses | Volume 2 | Exactly one of 13 values | README, Index, 22 L2 files | Contradictory | High |
| L2 structure | Volume 3 | Eleven required sections | Template and L2 corpus | Contradictory | High |
| Architecture ID inventory | Matrix/Index/files | 107 unique L2 IDs | None | Consistent | — |
| L2 owner metadata | Team Assignments/Index | Top-level owners match assignments | None in 107-file comparison | Consistent | — |
| Matrix/index top statuses | Matrix/Index | Same per-ID top status | None | Consistent | — |
| Core-document count | Index table | Six listed core documents | Index statistics | Contradictory | Low |
| D01 filename | Actual indexed file | L2-D01_Course_and_Course_Offering_Workflows.md | D01 self-reference | Broken Reference | Low |
| Section F canonical source | Live section-f/ | Live Markdown set | section-f.zip | Duplicate | High |
| AI-context basename | AI Context Index/files | Prefix-qualified basename for A–I | Prompts 0–2 | Broken Reference | Medium |
| Section A capability names | Architecture Index | Exact A03–A07 names | Section J README | Broken Reference | Medium |
| Repository state summary | Current inventory | All contexts present; matrix/index sets agree | REPO_UNDERSTANDING.md | Contradictory | Medium |
| Institution aggregate | B01/Section B | Section B owns Institution | AI Section A | Duplicate | High |
| Institution lifecycle | B01 | Full approval lifecycle | AI Section B | Contradictory | High |
| Finance upstream entities | F00 | Institution/Campus are external references | AI Section F | Contradictory | High |
| Markdown links | All Markdown files | 87/87 targets resolve | None | Consistent | — |
| Generated L0/L1 HTML provenance | Generated HTML manifests | Source hashes match current L0/L1 | None | Consistent | — |
| Runtime F→J integration | F14 | Finance consumes Section J facts | Dependency map lacks runtime register | Ambiguous | Medium |
| Prior integrity certification | Current evidence | Unresolved critical/high findings remain | 2026-07-31 report | Contradictory | Critical |

## CHECKLIST RESULTS

### Terminology checks

| Check | Result | Evidence |
|---|---|---|
| Every L1 entity uses the same name in L2 | **FAIL** | StudentContext, Tenant, and Institution lifecycle vocabularies diverge. |
| No L2 introduces an unauthorized canonical term | **FAIL** | Competing Academic Completion, physical-space, and alumni ownership claims exist. |
| Role names are consistent | **FAIL** | Obsolete Team 1/Team 2 model conflicts with named section owners. |
| Section names match all indexes and headers | **FAIL** | D, E, and F section-name drift. |
| Workflow names match the L2 matrix | **FAIL** | Multiple H1/title formatting and naming deviations. |
| Shared state names are identical | **FAIL** | Tenant, StudentContext, Institution, and document states differ. |

### Ownership checks

| Check | Result | Evidence |
|---|---|---|
| Every L2 workflow has exactly one owner | **FAIL** | Completion, alumni, employment, leave, and stock workflows overlap. |
| L2 header owner matches Team Assignments | **PASS** | All 107 top-level owner fields matched mechanically. |
| L2 header owner matches Architecture Index | **PASS** | All indexed owner mappings matched. |
| No primary workflow appears in multiple L2 files | **FAIL** | Multiple verified overlaps in INT-009 through INT-014. |
| Finance review responsibilities match assignments | **PASS** | No specific reviewer-assignment contradiction survived verification. |

### Dependency checks

| Check | Result | Evidence |
|---|---|---|
| Every cross-section dependency is in Dependency Map | **FAIL** | F14 → Section J runtime dependency is absent. |
| Every map dependency is reflected consistently | **FAIL** | L1’s serialized graph contradicts the map. |
| No L2 depends on nonexistent L3–L6 artifacts | **PASS** | No such dependency was found. |
| Dependency does not reverse authority hierarchy | **PASS** | No valid L2→L1 or L1→L0 implementation dependency was found. |
| No circular dependencies exist | **FAIL** | J03 mislabels downstream J01/J04 consumers as dependencies. |

### Lifecycle and state checks

| Check | Result | Evidence |
|---|---|---|
| Shared entity lifecycle states are identical | **FAIL** | Tenant, StudentContext, and Institution conflict. |
| Terminal states are consistent | **FAIL** | StudentContext Completed versus QualificationCompleted discrepancy. |
| Approval states are consistent | **FAIL** | AI Institution context omits B01 approval states. |
| Suspension/reactivation states are consistent | **FAIL** | Tenant and StudentContext documents disagree about Suspended. |

### Matrix and index checks

| Check | Result | Evidence |
|---|---|---|
| Every indexed architecture source file exists | **PASS** | All index targets exist. |
| Every canonical L2 source file is indexed | **PASS** | All 107 L2 IDs are represented. |
| Matrix sections match actual L2 files | **PASS** | ID sets match. |
| Matrix status matches each unambiguous file status | **FAIL** | Twenty-two documents contain missing, duplicate, or noncanonical statuses. |
| Matrix ownership matches Team Assignments | **PASS** | Mechanical comparison matched. |

### Structural checks

| Check | Result | Evidence |
|---|---|---|
| All L2 files follow the L2 template | **FAIL** | No L2 file matches all exact template headings. |
| Architecture IDs follow L2-[section][number] | **PASS** | All 107 canonical L2 IDs are well formed and unique. |
| No nonexistent path or filename is referenced | **FAIL** | D01 filename and AI-context prompt paths are broken. |
| README paths match repository structure | **PASS** | No broken README path survived verification. |

## FINAL INTEGRITY REPORT

### Executive Summary

The repository is structurally substantial but semantically unsafe as an implementation source of truth. The principal problem is not missing architecture; it is the coexistence of amended boundaries, stale later sections, derived AI contexts, prompts, generated artifacts, and prior certification material that continue to assert superseded meanings.

- Repository files read: **176**
- Embedded ZIP members additionally inspected: **15**
- L2 source documents: **107**
- Markdown links checked: **87; all resolve**
- Normalized identifier-form business/architecture concepts tracked: **6,204**
- Findings: **35**
- Critical: **8**
- High: **17**
- Medium: **8**
- Low: **2**
- Observation: **0**

The three most dangerous inconsistencies are:

1. **Physical campus ownership (INT-008/INT-029):** Section B and Section G can both be implemented as owners of the same physical structures.
2. **Academic completion ownership (INT-009):** C05 and I13 both determine completion eligibility and approval, risking competing graduation outcomes.
3. **Repository hierarchy and ownership authority (INT-001/INT-003/INT-004):** contradictory level definitions, obsolete teams, and reversed dependency sequencing corrupt the rules used to interpret every lower-level document.

### Consistency Breakdown by Type

| Type | Count | Most Severe Example | Pattern |
|---|---:|---|---|
| Type 1 — Terminology | 2 | Global event-name collisions | Generic event/status names are treated as canonical without one registry. |
| Type 2 — Ownership | 13 | Physical campus and academic completion | Opening disclaimers are contradicted by later workflow catalogues, diagrams, or AI summaries. |
| Type 3 — Lifecycle | 3 | Tenant lifecycle | Higher-level lifecycles were expanded in L2 without synchronized amendment. |
| Type 4 — Dependency | 3 | L1 dependency graph | Foundational and runtime dependencies are mixed or mislabeled. |
| Type 5 — Broken Reference | 3 | AI-context generator paths | IDs often resolve, but names and basenames have drifted. |
| Type 6 — Structural Drift | 6 | Conflicting architecture hierarchy | Governance, templates, headings, and archives do not express one enforceable structure. |
| Type 7 — Business Rule | 0 | None survived verification | No directly contradictory numeric or equivalent business rule met the reporting threshold. |
| Type 8 — Matrix/Index | 5 | False prior complete certification | Summaries and status artifacts are stale or internally contradictory. |

### Files With Most Conflicts

Counts below use discrete verified conflict incidences and exclude the corpus-wide template defect from every file’s total.

| Rank | File | Conflict Incidences |
|---:|---|---:|
| 1 | architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md | 5 |
| 2 | L2/section-b/L2-B04_Curriculum_Framework_Workflows.md | 4 |
| 3 | L2/section-c/L2-C08_Alumni_Transition_Workflows.md | 4 |
| 4 | L2/section-e/L2-E04_HR_Workflows.md | 4 |
| 5 | L2/section-b/L2-B03_Campus_Workflows.md | 3 |

### Merge Decision

**REJECT**

Eight critical and seventeen high-severity inconsistencies remain. The repository currently permits competing implementations of physical infrastructure, academic completion, alumni activation, employment state, leave approval, and stock movement. Its hierarchy and dependency rules are also contradictory. Those defects directly prevent architects, developers, and AI systems from reaching the same interpretation.

### Priority Remediation List

#### 1. Fix before anything else — Critical

1. INT-001 — Establish one architecture hierarchy.
2. INT-003 — Remove obsolete Team 1/Team 2 ownership.
3. INT-004 — Replace the L1 dependency chain with the canonical directed graph.
4. INT-008 and INT-029 — Make G10 the sole physical-infrastructure owner and repair Section B AI context.
5. INT-009 — Make C05 the sole completion/exit decision owner; constrain I13.
6. INT-010 — Remove Profile/Membership creation from C08.
7. INT-034 — Withdraw or correct the internally contradictory prior certification.

#### 2. Fix before merge — High

- INT-002, INT-006, INT-007
- INT-011 through INT-016
- INT-018, INT-020, INT-023
- INT-027, INT-030 through INT-033

#### 3. Fix in the next controlled review cycle — Medium/Low

- INT-005, INT-017, INT-019
- INT-021, INT-022
- INT-024 through INT-026
- INT-028, INT-035

After remediation, the integrity review should be rerun from a clean inventory, and AI contexts plus generated artifacts should be regenerated only after the canonical source documents pass.
