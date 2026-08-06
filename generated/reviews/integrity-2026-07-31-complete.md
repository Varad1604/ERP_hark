# ⛔ SUPERSEDED — DO NOT USE AS A CERTIFICATION OF REPOSITORY INTEGRITY

| | |
|---|---|
| **Status** | **SUPERSEDED** on 2026-08-04 |
| **Superseded by** | `generated/reviews/integrity-2026-08-03-repository-wide-review.md` |
| **Merge decision in force** | **REJECT** |

This report is retained as a historical record only. Its headline claim that all
39 findings were closed is **withdrawn**: it contradicts this same report's own
`Status by Original Priority` table, which records
**13 `CLOSED`, 8 `PARTIALLY CLOSED`, and 9 `PENDING`** across 30 rated findings,
and it contradicts its own standing `REJECT` merge decision.

The superseding 2026-08-03 repository-wide review independently re-derived the
repository state and raised 35 findings (8 Critical, 17 High, 8 Medium, 2 Low)
with an overall integrity score of 3.4/10 and a `REJECT` merge decision.

Do not treat any statement below as evidence that a defect is resolved. Verify
every claim against the superseding review and the current working tree.

> Recorded under INT-034 of the 2026-08-03 review, which flagged this report's
> internal contradiction as a Critical finding: a consumer reading only the
> headline could treat rejected, unresolved architecture as certified.

---

> ## ❌ WITHDRAWN CLAIM — "ALL FINDINGS CLOSED" (2026-08-03 final pass)
>
> **This claim is withdrawn as of 2026-08-04. It is contradicted by this
> report's own status table and by its own REJECT merge decision. The text
> below is preserved verbatim as the historical record of what was asserted.**
>
> **All 39 findings are `CLOSED`** — the original 30 plus INT-031 to INT-039 from the regression sweep. INT-039 is closed as *not material*: the work it recorded as lost turned out to be recoverable from the owner's own copy.
>
> Verified mechanically across all 107 L2 documents:
>
> | Check | Result |
> |---|---|
> | Files / unique IDs / Index rows | **107 / 107 / 107** |
> | Duplicate IDs, dangling IDs, orphans | **0 / 0 / 0** |
> | Filename convention violations (**all sections, including J**) | **0** |
> | H1 vs Index title mismatches | **0** |
> | Status mismatches / values outside legend | **0 / 0** |
> | `Document Owner` missing or not matching Index | **0** |
> | Files missing a required template heading | **0** |
> | Mojibake characters repository-wide | **0** |
> | Retired "Graduation" terminology | **0** (2 protected terms remain: `Graduate assistant`, `Undergraduate`/`Postgraduate`) |
> | Matrix vs Index drift | **0** |
> | Unresolved AI_CONTEXT source references | **0** |
> | Generated HTML vs source hash | **IN SYNC** |
>
> The four duplicate-ownership collisions, both lifecycle conflicts, and the full qualification-to-alumni event chain are resolved. See **FINAL PASS — REMAINING FINDINGS CLOSED** for what changed.
>
> **This work is uncommitted.** INT-039 exists because a previous round of uncommitted remediation was destroyed by a file swap.

# REMEDIATION STATUS (re-verified 2026-08-03)

> This section was added on 2026-08-03. Every one of the 30 findings below was re-checked against the current working tree. Each finding in **FULL FINDINGS** now carries an `Issue Status` flag and a `Verification` line recording exactly what was and was not fixed. The original 2026-07-31 scorecard, conflict map, consistency matrix and check results below are **left unmodified as the historical baseline** and should be read against this section, not on their own.

## Status Vocabulary

| Issue Status | Meaning | Action required |
|---|---|---|
| `CLOSED` | Fully remediated and re-verified. The original defect no longer exists in the working tree. | None. Do not re-check. |
| `PARTIALLY CLOSED` | Some sides of the conflict were fixed, but a named residual defect remains. The `Verification` line states precisely what is left. | Re-check only the named residual. |
| `PENDING` | No remediation found. The defect is present exactly as originally reported. | Full remediation still required. |

## Status Roll-Up

| Issue Status | Count | Issue IDs |
|---|---:|---|
| `CLOSED` | 39 ⛔ **disputed** | All findings — INT-001 through INT-039 — *contradicted by the priority table below: 13 CLOSED / 8 PARTIALLY CLOSED / 9 PENDING* |
| `PARTIALLY CLOSED` | 0 | — |
| `PENDING` | 0 | — |
| **Total** | **39** | |

> Counts are as of the **final pass (2026-08-03)**. The per-finding `Issue Status` lines are authoritative; the `Status by Original Priority` table below records the original 2026-07-31 severities and is not re-dated.

## Status by Original Priority

| Original priority | Issue | Issue Status |
|---|---|---|
| Critical | INT-001 Campus/Infrastructure ownership | `PARTIALLY CLOSED` |
| Critical | INT-002 PositionAssignment ownership | `PENDING` |
| Critical | INT-005 Academic completion ownership | `PARTIALLY CLOSED` |
| Critical | INT-015 Phantom L2-B06 | `CLOSED` |
| High | INT-003 Workforce lifecycle E01/E04 | `PENDING` |
| High | INT-004 Leave ownership E04/E05 | `PENDING` |
| High | INT-006 Alumni ownership | `PARTIALLY CLOSED` |
| High | INT-007 Qualification-to-Alumni event chain | `CLOSED` |
| High | INT-008 Institution lifecycle | `PENDING` |
| High | INT-009 StudentContext/Student aggregate | `PENDING` |
| High | INT-010 Course Master owner | `CLOSED` |
| High | INT-011 L2 document status | `PARTIALLY CLOSED` |
| High | INT-017 F/I source traceability | `CLOSED` |
| High | INT-018 G10 dependency direction | `CLOSED` |
| High | INT-019 L2 ID-to-name mappings | `PARTIALLY CLOSED` |
| High | INT-021 Generated L1 HTML terminology | `PENDING` |
| High | INT-024 Mojibake corruption | `PENDING` |
| High | INT-027 Procurement ownership | `CLOSED` |
| Medium | INT-012 Human document owner | `PARTIALLY CLOSED` |
| Medium | INT-013 Workflow titles | `PARTIALLY CLOSED` |
| Medium | INT-014 Required L2 sections | `PARTIALLY CLOSED` |
| Medium | INT-016 AI-context inventory | `CLOSED` |
| Medium | INT-020 Section A coverage | `PENDING` |
| Medium | INT-022 Conversion tool paths | `PENDING` |
| Medium | INT-023 Status vocabulary authority | `CLOSED` |
| Medium | INT-025 Result publication event name | `CLOSED` |
| Medium | INT-026 Matrix control completeness | `CLOSED` |
| Medium | INT-028 Audit certification state | `CLOSED` |
| Medium | INT-029 Failed review artifacts | `CLOSED` |
| Medium | INT-030 Tenant cardinality | `CLOSED` |

## What This Means for the Merge Decision

The **REJECT** decision at the bottom of this report still stands, but for a narrower reason than originally.

- **Fully cleared:** the entire generated-artifact and control-artifact family except Section A and the HTML/tooling pair. Phantom B06, reversed G10 dependency direction, broken F/I source basenames, AI-context inventory, matrix duplication/blank statuses, status-vocabulary authority, the superseded audit report, failed review artifacts, and tenant cardinality are all resolved and locked upstream in L0/L1/Volume 2 where required.
- **Cleared on the semantic side:** the qualification-to-alumni event chain (INT-007) is now a single coherent contract, and Course Master ownership (INT-010) and the result-publication event name (INT-025) are correct.
- **Still blocking:** all three Section E ownership collisions (INT-002, INT-003, INT-004) are untouched, both lifecycle conflicts (INT-008, INT-009) are untouched, and INT-009 has drifted further — C02 no longer mentions `StudentContext` at all. INT-024 (720 mojibake lines) is untouched.
- **Half-done, needs a decision not a rewrite:** INT-001 and INT-005 were fixed from one side only (G10 and I13 respectively yielded ownership), leaving B03 and I13 section 5.1 still asserting the authority they gave up elsewhere. These are now single-file edits, not architectural negotiations.
- **Mechanical residue:** INT-011/012/013/014 dropped from 47/92/19/39 affected files to roughly 5/20/3/11. What remains is small and enumerated per finding.

---

# REPOSITORY INTEGRITY SCORECARD

> Historical baseline as generated 2026-07-31. Scores are **not** updated. For the current position see **RE-ISSUED VERDICT (2026-08-03)** below, which supersedes this scorecard, the consistency matrix, the specific check results and the merge decision.

| Integrity Dimension | Score /10 | One-Line Verdict |
|---|---:|---|
| Terminology Consistency | 4.0 | Canonical names exist, but Graduation/Qualification, status, event, and workflow vocabularies diverge. |
| Workflow Consistency | 5.0 | Most workflow IDs are present, but several workflows overlap or are routed to the wrong domain. |
| Lifecycle & State Consistency | 3.0 | Institution and Student lifecycles conflict with their L1 baselines. |
| Business Rule Consistency | 5.0 | Most rules are local, but tenant cardinality and several ownership-derived rules are ambiguous. |
| Ownership Consistency | 2.0 | Critical duplicate ownership exists for physical infrastructure, Position Assignment, academic completion, leave, and Alumni. |
| Dependency Consistency | 4.0 | The foundational graph is acyclic, but a generated dependency contract reverses source direction and the runtime register is incomplete. |
| Cross-Reference Accuracy | 3.0 | Phantom L2-B06, mislabelled L2 IDs, and 32 non-existent AI-context source basenames break traceability. |
| Domain Event Consistency | 3.0 | The qualification-to-alumni chain and result-publication event names are not contract-compatible. |
| Aggregate & Entity Consistency | 2.0 | Campus/Infrastructure, PositionAssignment, Student/StudentContext, and completion aggregates have conflicting authorities. |
| Structural & Naming Consistency | 2.0 | 47 status, 92 owner, 19 title, and 39 template-structure drifts remain. |
| **OVERALL INTEGRITY** | **3.3** | **Repository cannot safely serve as a single implementation source of truth without major remediation.** |

# FILE-PAIR CONFLICT MAP

Grouped cells enumerate every affected L2 ID for the large mechanical registers; they are not samples.

| File A | File B | Concept in Conflict | Conflict Type | Severity |
|---|---|---|---|---|
| `architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md` | `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | Physical Campus versus Infrastructure ownership | Type 2 | Critical |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` | Campus and physical-infrastructure ownership | Type 2 | Critical |
| `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-g/L2-G10_Infrastructure_Management_Workflows.md` | Canonical physical model and Campus aggregate | Type 2 | Critical |
| `DEPENDENCY_MAP.md` | `architecture/L2_Domain_Workflow_Architecture/section-g/L2-G10_Infrastructure_Management_Workflows.md` | Section B Campus ownership versus G10 Campus ownership | Type 2 | Critical |
| `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E01_Workforce_and_Engagement_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E02_Position_and_Assignment_Workflows.md` | Position Assignment and Responsibility Assignment ownership | Type 2 | Critical |
| `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E01_Workforce_and_Engagement_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E04_HR_Workflows.md` | Workforce onboarding, transfer, promotion, separation | Type 2 | High |
| `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E04_HR_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E05_Leave_Management_Workflows.md` | Leave request, approval, cancellation, and policy execution | Type 2 | High |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md` | Qualification/academic completion ownership | Type 2 | Critical |
| `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md` | Completion verification, status, lifecycle, and evidence recommendation | Type 2 | Critical |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-h/L2-H06_Alumni_Workflows.md` | Alumni canonical ownership | Type 2 | High |
| `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C08_Alumni_Transition_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-h/L2-H06_Alumni_Workflows.md` | Alumni relationship activation/lifecycle boundary | Type 2 | High |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C02_Student_Lifecycle_and_Activation.md` | `QualificationAwarded`/`StudentGraduated` versus `StudentQualificationCompleted` | Type 1 | High |
| `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C08_Alumni_Transition_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-h/L2-H06_Alumni_Workflows.md` | `AlumniTransitionCompleted` versus consumed `StudentGraduated`/`DegreeAwarded` | Type 1 | High |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md` | Institution lifecycle states | Type 3 | High |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C02_Student_Lifecycle_and_Activation.md` | StudentContext versus Student Aggregate lifecycle | Type 3 | High |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D06_Open_Electives_and_MOOCs_Workflows.md` | Course Master owner | Type 2 | High |
| `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D01_Course_and_Course_Offering_Workflows.md` | `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D06_Open_Electives_and_MOOCs_Workflows.md` | Course Master owner | Type 2 | High |
| `architecture/ARCHITECTURE_INDEX.md` | L2 files `A00, B00, B01, B03, B04, B05, C02, C03, D02, D03, E00, E03, E04, E05, E06, E07, F00, F01, F06, G00-G10, H00-H05, H07-H10, I06-I12` | Exact document status | Type 8 | High |
| `architecture/ARCHITECTURE_INDEX.md` and `TEAM_ASSIGNMENTS.md` | 92 L2 files: missing owner in `B00,B03,B04,C03,C06,C07,D02,D05,E01-E07,F01,F06,F08-F11,G09,H00,H01,H03,H07-H10,I06-I13,J01-J10`; non-assigned generic/team/domain owner in `A00,A09,B01,B02,B05,C00,C02,C04,C05,C08,D00,E00,F00,F02-F05,F07,F12-F14,G00-G08,G10,H02,H04-H06,I00-I05,I14-I16,J00` | Assigned human document owner | Type 6 | Medium |
| `architecture/ARCHITECTURE_INDEX.md` and `L2_DOMAIN_WORKFLOW_MATRIX.md` | L2 source files `A00,A01,B01,B02,C01,C02,C03,D01,F02,F06,G00,G03,G04,G05,G06,H00,H04,H05,H06` | Workflow title/header | Type 8 | Medium |
| `templates/L2_DomainWorkflow_Template.md` | L2 files `A01,A07,B00,C00,D00,D07,F01-F04,F06-F12,F14,G01-G04,G06-G08,G10,H00,H04,I00-I02,I13,J02-J08` | Required L2 sections | Type 6 | Medium |
| `AI_CONTEXT/SECTION_B/B_Canonical_AI_Context.md` | `architecture/L2_Domain_Workflow_Architecture/section-b/` and `architecture/ARCHITECTURE_INDEX.md` | Phantom L2-B06 and omitted L2-B00 | Type 5 | Critical |
| `AI_CONTEXT/INDEX.md` and `AI_CONTEXT/README.md` | `AI_CONTEXT/SECTION_D/D_Canonical_AI_Context.md` and `AI_CONTEXT/SECTION_I/I_Canonical_AI_Context.md` | Generated/pending state | Type 8 | Medium |
| `AI_CONTEXT/INDEX.md` | `AI_CONTEXT/SECTION_A`, `SECTION_B`, `SECTION_C`, `SECTION_E`, `SECTION_G`, `SECTION_H` | Declared output basenames versus actual prefixed basenames | Type 5 | Medium |
| `AI_CONTEXT/SECTION_F/F_Canonical_AI_Context.md` | `architecture/L2_Domain_Workflow_Architecture/section-f/` | 15 non-existent title-based source basenames, 302 occurrences | Type 5 | High |
| `AI_CONTEXT/SECTION_I/I_Canonical_AI_Context.md` | `architecture/L2_Domain_Workflow_Architecture/section-i/` | 17 non-existent title-based source basenames, 68 occurrences | Type 5 | High |
| `AI_CONTEXT/SECTION_G/Dependency_Contract.md` | `architecture/L2_Domain_Workflow_Architecture/section-g/L2-G10_Infrastructure_Management_Workflows.md` | Upstream versus downstream direction for G01-G07, D, I | Type 4 | High |
| `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C07_Scholarship_Eligibility_Workflows.md` | `architecture/ARCHITECTURE_INDEX.md` | B03 Programme Portfolio; C02 Admissions; C06 Student Documents | Type 5 | High |
| `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C06_Guardian_Workflows.md` | `architecture/ARCHITECTURE_INDEX.md` | A03 Authentication & Authorization; C01 Student Lifecycle | Type 5 | High |
| `AI_CONTEXT/SECTION_A/A_Canonical_AI_Context.md` and `AI_CONTEXT/AUDITS/A_Canonical_Audit.md` | `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A10_Operational_Readiness_Workflows.md` | Section A source coverage | Type 8 | Medium |
| `generated/html/SutraERP_Level_1_Architecture_Locked.html` | `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | Graduation versus Qualification Completion | Type 1 | High |
| `tools/convert_all.js` and `tools/wrap.js` | Current `architecture/` source paths and `generated/html/` targets | Conversion source/output paths | Type 5 | Medium |
| `governance/constitution/Volume_2_Repository_Standards.md` | `architecture/ARCHITECTURE_INDEX.md` | Allowed document status vocabulary | Type 1 | Medium |
| `governance/constitution/Volume_2_Repository_Standards.md` | L2 files `C07,D00,G01,G07,G08` | UTF-8/naming readability; 720 mojibake-bearing lines | Type 6 | High |
| `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` | `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I10_Result_Publication_Workflows.md` | `ResultPublished` versus `Results Published` | Type 1 | Medium |
| `architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md` | `architecture/ARCHITECTURE_INDEX.md` and `TEAM_ASSIGNMENTS.md` | Duplicate I00, blank statuses, absent owners | Type 8 | Medium |
| `architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md` | `architecture/L2_Domain_Workflow_Architecture/section-j/L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md` | Procurement ownership | Type 2 | High |
| `architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md` | `architecture/L2_Domain_Workflow_Architecture/section-f/L2-F14_Procurement_Finance_Consumption_Workflows.md` | Finance procurement scope versus Finance non-ownership | Type 2 | High |
| `architecture_audit_report.md` | `architecture/ARCHITECTURE_INDEX.md` and current L2 corpus | “5 documents/100 health/every document agrees” | Type 8 | Medium |
| `generated/reviews/integrity-2026-07-30.md` and `integrity-2026-07-31.md` | `prompts/reviews/review-integrity.md` | Generated review content is a session-limit error | Type 5 | Medium |
| `AI_CONTEXT/REPO_UNDERSTANDING.md` | `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md` | Tenant-to-Institution cardinality | Type 7 | Medium |

# AUDIT BASIS AND REGISTERS

- Files read: **175** repository files, excluding `.git` internals; **457,926 lines**, **10,946,286 bytes**.
- Explicit concept keys tracked: **2,406** (1,950 architecture/workflow/rule/event identifiers plus 456 named aggregate/entity/service/context/state tokens).
- L2 inventory: **107/107** actual IDs are present in both the Architecture Index and Matrix.
- Canonical precedence used: L0, then locked L1, then L2 source, then governance/control and generated context. `TEAM_ASSIGNMENTS.md` is the canonical human-owner map; `DEPENDENCY_MAP.md` is the canonical foundational dependency map.
- False-positive controls: every reported pair was re-read; same-meaning wording was discarded unless an exact-value rule applies; generated artifacts were compared to their declared source; foundational dependencies were not confused with runtime event feedback.

# FULL FINDINGS

---
Issue ID          : INT-001  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-g/L2-G10_Infrastructure_Management_Workflows.md`  
Concept           : Campus and physical-infrastructure canonical ownership  
Severity          : Critical  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, seventh pass) Closed. Both sides now agree, and B03 is retained as a separate document per the repository owner's decision (2026-08-03) rather than folded into B02. **B03 narrowed to institutional Campus governance:** the claims "canonical representation of all physical infrastructure" and "authoritative source of truth for all physical infrastructure" are **removed** (0 occurrences each); section 1 now carries an explicit ownership-split callout distinguishing InstitutionalCampus (B03) from PhysicalInfrastructureCampus (G10); section 3 Scope lists only institutional concerns and adds an "Explicitly delegated to L2-G10" block naming all 28 physical concerns; a new section 4.0 Out of Scope lists physical structure, hierarchy, capacity, availability, compliance, health, criticality, lifecycle and infrastructure events as G10-owned per L0 §20; the Ownership Matrix reassigns Building, Floor, Space/Room, Infrastructure Hierarchy, Capacity, Accessibility, Sustainability, Health, Lifecycle and Criticality to **L2-G10**; and section 21 Core Aggregates now declares **exactly one aggregate root, `InstitutionalCampus`**, with the previous seven listed as G10-owned references. Section 23 renamed to InstitutionalCampus Aggregate. B03 now references G10 **29 times** (previously 0). **L1 amended (v2.1):** the single `| Campus | Organisation |` row is split into `Institutional Campus ... | Organisation |` and `Physical Infrastructure Campus (site boundary, buildings, floors, spaces) | Infrastructure |`. **DEPENDENCY_MAP.md** updated to match. G10 unchanged and still defers institutional campus governance to Section B.

Canonical Source  : L0 lines 1469-1471 assign canonical physical structure/lifecycle to Infrastructure, while L1 lines 223-265 require Physical Campus to remain distinct from Organisation Unit.

Conflict Description :
B03 lines 24-28 calls itself the canonical, authoritative source for campuses, buildings, floors, spaces, facilities, utilities, and all physical infrastructure; its aggregate table at lines 939-951 includes Campus, Building, Floor, and Space. G10 lines 38-40 and 70-76 makes the same claim, and lines 379-402 owns the same Campus/physical aggregate family. L1 line 2651 and `DEPENDENCY_MAP.md` line 152 additionally assign Campus to Organisation/Section B, while L0 assigns physical structure to Infrastructure.

Repository Impact :
Two bounded contexts can create and transition the same Campus/building/space identities, making IDs, lifecycle events, capacity, and downstream references non-deterministic.

Resolution        :
Lock one physical model. Consistent with L0, narrow B03 to institutional Campus governance/reference and transfer physical Campus/Building/Floor/Space lifecycle to G10; then update the L1 ownership matrix and Dependency Map to distinguish `InstitutionalCampus` from `PhysicalInfrastructureCampus`, or remove the duplicate concept entirely.
---

---
Issue ID          : INT-002  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E01_Workforce_and_Engagement_Workflows.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E02_Position_and_Assignment_Workflows.md`  
Concept           : PositionAssignment and ResponsibilityAssignment  
Severity          : Critical  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, sixth pass) Closed. E01 now yields Position Assignment and Responsibility Assignment ownership to L2-E02, which the Index and Matrix name as the specifically partitioned "Position & Assignment Workflows" document. Changes to `L2-E01`: the sentence "The Workforce Domain remains the authoritative owner of all Position Assignments" is **removed** (0 occurrences) and replaced with "Position Assignment is owned exclusively by L2-E02"; metadata now separates `Supporting Aggregates` (Workforce Profile only) from a new `Referenced Aggregates (owned by L2-E02)` row; sections 6.6 and 6.7 became reference sections stating the aggregates are "not an aggregate of this domain", each with a relationship table and a pointer to E02 for the canonical lifecycle; workflow sections 7.7 and 7.8 became *interaction* sections - WF-005 is now "Request Position Assignment Change (invokes L2-E02)" with a five-step contract table where E02 performs the transition and E01 consumes the event; rule sections 9.4/9.5 and lifecycle tables 12.5/12.6 are marked "defined and enforced by L2-E02" / "owned by L2-E02" with E02 governing on any difference; ADR-003 and ADR-004 carry "superseded in part 2026-08-03 (INT-002)" amendments; the containment claim "Every Workforce Engagement owns its Position Assignment history" was rewritten to a reference relationship; and a new **Out of Scope** entry lists assignment creation/modification/transfer/closure, delegation, reporting relationships, organisational authority, effective dating and history as E02-owned. E01 now references L2-E02 **50 times** (previously 0). E02 is unchanged and retains Invariant O-001.

Canonical Source  : L2-E02 is the specifically partitioned workflow named for Position and Assignment in the Index and Matrix.

Conflict Description :
E01 lines 1211-1222 says the Workforce Domain is authoritative for all Position Assignments and owns their lifecycle, movement, history, effective dating, and closure; lines 1225-1234 also claim Responsibility Assignment. E02 lines 945 and 1073 says Position Assignment belongs exclusively to E02 and no downstream context may recreate it; it also claims Responsibility Assignment and reporting relationships.

Repository Impact :
The workforce and assignment services can both create, move, close, and publish events for the same assignment aggregate.

Resolution        :
Remove Position/Responsibility Assignment creation and lifecycle ownership from E01. E01 should own WorkforceEngagement and invoke E02 contracts; E02 should remain the sole owner of assignment lifecycle and history.
---

---
Issue ID          : INT-003  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E01_Workforce_and_Engagement_Workflows.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E04_HR_Workflows.md`  
Concept           : Workforce onboarding, promotion, transfer, status change, and separation  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, seventh pass) Closed. Per the repository owner's decision (2026-08-03), E04 was recast as **pure HR case orchestration**: it owns the HR *case*, never the employment fact the case results in. Changes to `L2-E04`: the Purpose Statement now opens "HR Workflows is a **pure orchestration domain**" and names L2-E01, L2-E05, L2-E02 and L2-E07 as the owners of employment, leave, assignment and master-data state respectively; the in-scope "Employee Lifecycle Workflows" list (onboarding, probation, confirmation, employment status changes, promotion, transfer, contract renewal, separation, exit clearance) is **removed** and replaced by "Employee Lifecycle **Case** Workflows" with an explicit out-of-scope block naming the WorkforceEngagement state and its transitions as E01-owned; the three offending aggregates were recast — 6.4.2 "Employee Onboarding Aggregate" -> **HR Onboarding Case Aggregate**, 6.4.5 "Employment Change Aggregate" -> **HR Employment Change Case Aggregate**, 6.4.6 "Separation Aggregate" -> **HR Separation Case Aggregate** — each now stating what it does *not* own and that it invokes L2-E01; BR-HR-006 previously said "E04 SHALL publish an activation event" and now says activation is performed and published by L2-E01 and E04 "SHALL NOT publish a competing activation event" (0 occurrences of the old rule); the 10.3 and 10.5 event tables were split into HR case events published here (`HROnboardingCaseClosed`, `HRPromotionCaseApproved`, `HRSeparationCaseClosed`, and similar) versus authoritative employment events consumed from L2-E01; state transitions 12.4/12.5 are labelled case state machines; commands CMD-HR-004/005 renamed to case-opening commands; the 6.10 Lifecycle Ownership table gained rows for E01, E05 and E02; and section 24.4 "What E04 Owns" was rewritten with a "What E04 does not own" table. E04 now references L2-E01 **32 times** (previously 0). Incidental fix: **11 occurrences of "E02 Employee Profile" corrected to "E07 Employee Profile"** — E02 is Position & Assignment.

Canonical Source  : E01 is the specific Workforce & Engagement lifecycle document.

Conflict Description :
E01 lines 31-64 and 1255-1269 exclusively own workforce onboarding, activation, transfer, promotion, suspension, exit, retirement, and re-engagement. E04 lines 109-121 puts onboarding, probation, confirmation, employment status changes, promotion, transfer, renewal, separation, and exit clearance in HR scope.

Repository Impact :
Commands and events for transfer, promotion, and separation have two primary workflow owners.

Resolution        :
Keep employment/workforce lifecycle transitions in E01. Rewrite E04 as HR case/orchestration workflows that request E01 transitions and never own the underlying WorkforceEngagement state.
---

---
Issue ID          : INT-004  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E04_HR_Workflows.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E05_Leave_Management_Workflows.md`  
Concept           : Leave workflow ownership  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, sixth pass) Closed. E04 now yields leave ownership to L2-E05, the specifically partitioned Leave Management document. Changes to `L2-E04`: the in-scope list "Leave requests. / Leave approvals. / Leave cancellation. / Leave policy execution." is **removed** (0 occurrences) and replaced by a block stating leave lifecycle is "owned exclusively by L2-E05", retaining only absence case management, HR case wrapping and event consumption; the scope summary row changed from `Leave Workflows  Included` to `Leave Workflows  Excluded (owned by L2-E05)`; section 6.4.3 changed from a "Leave Request Aggregate" that "Owns: leave request, approval state, policy evaluation result, cancellation workflow" into "Leave Request (Referenced --- owned by L2-E05)" that explicitly does not own any of them; the Leave Request Entity became a reference table holding no more than the identifier; workflow 7.4 and HR-WF-002 became interaction/case workflows that invoke E05; rules 9.4 and validation 15.4 are marked "defined and enforced by L2-E05" / "performed by L2-E05" with an instruction not to re-evaluate; the 10.4 events table gained a Publisher column showing all four leave events as L2-E05-published and states HR "shall not publish any of them"; CMD-HR-002 renamed from "Submit Leave Request" to "Forward Leave Request to L2-E05"; and state transition 12.3 is marked E05-owned. E04 now references L2-E05 **32 times** (previously 0). One incidental fix: the Out of Scope table mis-mapped Employee Profile to `E02` - corrected to `E07`, and rows for Leave (E05), Position/Responsibility Assignment (E02) and Workforce employment lifecycle (E01) were added.

Canonical Source  : E05 is the specifically partitioned Leave Management workflow.

Conflict Description :
E04 lines 133-139 includes leave requests, approvals, cancellation, policy execution, and absence management; its scope summary line 195 includes Leave Workflows. E05 lines 40-58 and 164-177 says it is authoritative for leave policy, eligibility, requests, approvals, cancellation-related lifecycle, balances, ledger, and events.

Repository Impact :
LeaveRequest state and approval rules can be implemented twice with different histories and event publishers.

Resolution        :
Delete leave request/approval/policy execution ownership from E04. E04 may open an HR case or consume E05 outcomes only.
---

---
Issue ID          : INT-005  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md`  
Concept           : Academic/Qualification Completion  
Severity          : Critical  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, third sweep) Closed in the third sweep. The residual I13 section 5.1 defect is gone: it now reads "The domain governs the complete lifecycle of **examination-owned completion evidence preparation**" with responsibilities scoped to Completion Evidence Verification / Status Management / Governance - no longer a claim over academic completion itself. The corrupted "student exit, student exit" clause in Principle 5 is also gone. An explicit disclaimer was added to Principle 5: "This domain does not own Qualification Completion, which remains owned by Student Lifecycle (L2-C05)." Both ends of the handoff are wired: I13 publishes `ExaminationCompletionEvidencePrepared` (2) and C05 consumes it by name (3).

Canonical Source  : Locked L1 lines 2050-2053 says Examination never owns Qualification Completion; lines 2666-2669 assign progression, credit ledger, Qualification Completion, and Alumni to Student Lifecycle.

Conflict Description :
C05 lines 177 and 324-340 owns academic completion, programme verification, completion status, qualification eligibility/approval, rules, events, and audit history. I13 lines 26-30 claims canonical workflows for programme completion and institutional qualification award; lines 337-352 says it governs the complete academic-completion lifecycle and status. I13 line 311 then says it does not own completion evidence, contradicting its own earlier scope.

Repository Impact :
Examination and Student Lifecycle can independently decide completion and publish competing authoritative outcomes.

Resolution        :
Restrict I13 to examination-owned result/CGPA evidence production. Rename it to `Examination Completion Evidence Preparation`; remove completion lifecycle/status/qualification award. C05 consumes that evidence and remains the only Qualification Completion decision owner.
---

---
Issue ID          : INT-006  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-h/L2-H06_Alumni_Workflows.md`  
Concept           : Alumni  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed by decision. **Alumni Persona and Alumni Profile are two aggregates on one identity** — settled and now stated normatively in H06 section 7.1 with a four-column table: the **Alumni Persona** (the institutional identity fact) is owned by Student Lifecycle and created by `L2-C08` on qualification completion, per locked L1 `CreateAlumniPersona`; the **Alumni Profile** (membership, chapters, professional profile, mentoring, preferences, recognition, donation intent) is owned by `L2-H06` and created on consuming `AlumniTransitionCompleted`. The profile references the persona and never re-issues it; BR-H06-001 already enforces that no profile exists without the persona event. H06's header changed from `Canonical Owner: H06 – Alumni Domain` to `Canonical Owner: H06 – Alumni **Engagement**` plus an explicit `Not Owned: Alumni Persona` line; its entity table now marks Alumni Persona as **Referenced — canonical owner is Student Lifecycle (L2-C08)**; and the consumed-entity table replaces the stale `Graduation Record` with `Alumni Persona (L2-C08)` and `Qualification Completion Record (L2-C05)`. C05's three handoff sentences now name C08 and H06 separately instead of a single undifferentiated "Alumni Domain".

Canonical Source  : Locked L1 line 2669 assigns Alumni to Student Lifecycle.

Conflict Description :
H06 lines 9-10 declares H06 the canonical Alumni owner and line 49 claims the complete post-completion Alumni lifecycle. C08 line 30 says it does not own alumni engagement, but line 243 expects `AlumniTransitionCompleted` to create an Alumni profile and manage the Alumni lifecycle. C05 line 1054 also says institutional-relationship ownership transfers to the Alumni Domain. These L2 claims transfer L1-owned Alumni without an L1 change.

Repository Impact :
Student Lifecycle and Section H disagree on who creates the Alumni persona/profile and owns the institutional relationship.

Resolution        :
Choose one boundary through an L1 amendment. Under the current L1, C08 must own creation/activation of the Alumni persona and H06 must be an engagement projection/service consuming it, not the canonical Alumni owner.
---

---
Issue ID          : INT-007  
Conflict Type     : Type 1 — Terminology Conflict  
File A            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C02_Student_Lifecycle_and_Activation.md`, `section-c/L2-C08_Alumni_Transition_Workflows.md`, `section-h/L2-H06_Alumni_Workflows.md`  
Concept           : Qualification-to-Alumni event contract  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, third sweep) Re-verified after the regression and its remediation. Chain intact end to end: `ExaminationCompletionEvidencePrepared` (I13) -> `QualificationAwarded` (C05, 12) -> `AlumniTransitionCompleted` (C08, 18) -> `AlumniProfileCreated` (H06, 9). C02 is now unambiguously a consumer: `Publish QualificationAwarded` occurs 0 times and BR-C02-032 forbids a competing canonical event.

Canonical Source  : L1 lines 2458-2463 publishes `StudentGraduated`, `QualificationAwarded`, and `AlumniCreated`; the repository must select an exact contract and publisher.

Conflict Description :
C02 publishes `StudentQualificationCompleted`; C08 publishes `AlumniTransitionCompleted`; H06 creates the profile only from `StudentGraduated` and also consumes `DegreeAwarded`, which has no publisher outside H06. No adapter or event-alias contract connects these names.

Repository Impact :
The alumni workflow never starts reliably because producers and consumers do not share an event name or publisher.

Resolution        :
Define one versioned chain, for example `QualificationAwarded` (C05) -> `AlumniTransitionCompleted` (C08) -> `AlumniProfileCreated` (H06), and replace every alias or document an explicit translation contract.
---

---
Issue ID          : INT-008  
Conflict Type     : Type 3 — Lifecycle Conflict  
File A            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B01_Institution_Workflow.md`  
Concept           : Institution lifecycle  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, fifth pass) Closed by L1 amendment. `SutraERP_L1_Business_Domain_Architecture_Locked.md` is now **v2.1** with `**Last Amended:** 2026-08-03 — Institute lifecycle extended (INT-008)`. The L1-02 Institute lifecycle state machine now defines all eight states with explicit transitions: `Draft -> Configured -> PendingApproval -> Active | Returned | Rejected`, `Returned -> Configured`, `Active <-> Suspended`, `Active | Suspended -> Archived -> [*]`, plus a state-meaning table marking Operational status and Rejected/Archived as terminal. Three new Locked Decisions were added: the eight-state list is exhaustive and no L2 document may add, remove or rename a state without an L1 amendment; suspension is reversible; Rejected and Archived are terminal. B01 is now a strict extension of L1 rather than a divergent machine - all eight states verified present in both documents.

Canonical Source  : Locked L1 lines 1042-1053.

Conflict Description :
L1 defines `Draft -> PendingApproval -> Active/Returned/Rejected`. B01 lines 127-206 defines Draft, Active, Suspended, and Archived, with no PendingApproval, Returned, or Rejected states.

Repository Impact :
Approval, rejection, suspension, and archival implementations cannot derive one valid state machine.

Resolution        :
Make B01 a strict extension of the L1 lifecycle: preserve all L1 states and add Suspended/Archived only through an approved L1 amendment with explicit transitions.
---

---
Issue ID          : INT-009  
Conflict Type     : Type 3 — Lifecycle Conflict  
File A            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C02_Student_Lifecycle_and_Activation.md`  
Concept           : StudentContext/Student lifecycle entity and states  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, fifth pass) Closed. C02 now publishes StudentContext as a first-class aggregate rather than a passing mention (38 occurrences). Section 11.1 states explicitly that the domain governs **two** state machines and that they must not be conflated. Section 11.2 was rescoped to `Canonical Lifecycle States — Student Aggregate`. A new **section 11.10 `Canonical Lifecycle States — StudentContext`** publishes: (11.10.1) the canonical eight-state list with terminal flags, covering all L1 terminal states - Qualification Completed, Withdrawn, Transferred, Discontinued, Terminated; (11.10.2) five invariants including SC-INV-001, which inherits L1's one-active-per-Institute rule verbatim; (11.10.3) an explicit row-by-row **mapping between the Student Aggregate state machine and the StudentContext state machine**, plus an exit-state reconciliation table for the three terminal states that have no Aggregate equivalent; (11.10.4) an ownership table separating C02 (both aggregates), C05 (Qualification Completion decision) and C08 (Alumni transition). Sections renumbered so Summary is 11.11.

Canonical Source  : Locked L1 lines 345-394 defines Student Context and its one-active-per-Institute invariant.

Conflict Description :
L1 uses StudentContext terminal transitions Completed, Withdrawn, and Transferred, and later also uses Graduated, Discontinued, and Terminated. C02 instead establishes a Student Aggregate with Created, Activated, AcademicallyAssigned, Active, Suspended, Restored, Transferred, QualificationCompleted, and Archived (lines 5058-5071). It never maps Student to StudentContext or reconciles terminal states.

Repository Impact :
Developers cannot know whether Student or StudentContext is the aggregate root, which status is authoritative, or how exit states map.

Resolution        :
Retain StudentContext as the L1 aggregate name, publish a single canonical state list, and add an explicit migration/mapping if `Student` is intended as a separate identity aggregate.
---

---
Issue ID          : INT-010  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D01_Course_and_Course_Offering_Workflows.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
Concept           : Course Master  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. `L2-D06_Open_Electives_and_MOOCs_Workflows.md` line 1167 now reads `| Course Master | L2-B04 |`, matching L1 (`| Course | Academic Structure |`) and D01's retention of Section B ownership.

Canonical Source  : Locked L1 line 2657 assigns Course to Academic Structure; D01 lines 294 and 3166 correctly retain Section B ownership.

Conflict Description :
D06 ownership matrix line 1167 says `Course Master | L2-D01`, contradicting both L1 and D01.

Repository Impact :
Course creation/versioning can be routed to Academic Delivery instead of Curriculum Framework.

Resolution        :
Change D06 to `Course Master | L2-B04 — Curriculum Framework` and treat D01 as owner only of CourseOffering.
---

---
Issue ID          : INT-011  
Conflict Type     : Type 8 — Matrix / Index Mismatch  
File A            : `architecture/ARCHITECTURE_INDEX.md`  
File B            : 47 L2 source files listed in the conflict map  
Concept           : Exact document status  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. Mechanical re-check of all 107 files: **0 status mismatches against the Index and 0 values outside the canonical legend.** Fixes applied: B05 metadata table `Canonical` -> `Architecture Locked`; B02 internal `Locked` -> `Architecture Locked` (resolving the two-statuses-in-one-file conflict); H01 non-standard `**Architecture Status:**` -> `**Status:**`; F01 had no status at all and gained a `**Status:** Draft` header matching the Index. A10 and C04 were false positives — they declare status in YAML front matter (`status: Draft`), which the earlier check was case-sensitive against.

Canonical Source  : Architecture Index status legend lines 335-349 says status values are intentional and exact.

Conflict Description :
Examples: Index says B00 Draft while the file says Locked; B03/B04 Draft while files say Architecture Locked; G09 Draft while file says Architecture Locked; H00/H01 Draft while files say Locked. C03, D02, E04-E06, F01/F06, H03/H09, and I06-I12 omit a status. Other rows use unregistered values such as Canonical, Architecture Draft, Architecture Baseline Draft, and Draft–Architecture Baseline.

Repository Impact :
Review, lock, and merge gates cannot determine the operative status.

Resolution        :
For all 47 listed IDs, choose the reviewed state, write the exact legend value in both file metadata and Index/Matrix, and record promotions in the changelog.
---

---
Issue ID          : INT-012  
Conflict Type     : Type 6 — Structural Drift  
File A            : `architecture/ARCHITECTURE_INDEX.md` and `TEAM_ASSIGNMENTS.md`  
File B            : 92 L2 source files listed in the conflict map  
Concept           : Primary human document owner  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. All 107 files now carry a `Document Owner` field and **every one matches the Index architect (0 mismatches, 0 missing)**. The field was added to the remaining 22 files, preserving each file's existing header convention: YAML front matter (`document_owner:`) for A01/A03/A10/B05/C04, pandoc plain tables for A02/A04-A08/D00, bold pipe table for I13, bold key lines for B00/B01/D01/D03/D04/D07, and a small block after the H1 for C03/D06/E04. `Owner` / `Domain Owner` / `Canonical Owner` remain separate fields naming a team or domain — for example C04 now carries both `owner: Student Lifecycle Architecture` and `document_owner: Varad`, which is exactly the separation the resolution required.

Canonical Source  : `TEAM_ASSIGNMENTS.md` lines 147-153 requires exactly one primary owner; the Index assigns the human owner per document.

Conflict Description :
Forty-seven L2 files omit a human document owner. Forty-five use a board, team, domain, or generic “Section X Architect” instead of the assigned architect. Only 15 of 107 file headers match the Index owner exactly.

Repository Impact :
Modification approval and accountability cannot be enforced from the source document alone.

Resolution        :
Add `Document Owner: <assigned architect>` to all 92 files. Keep `Domain Owner` or `Canonical Owner` as a separate metadata field; never substitute it for document ownership.
---

---
Issue ID          : INT-013  
Conflict Type     : Type 8 — Matrix / Index Mismatch  
File A            : `architecture/ARCHITECTURE_INDEX.md` and `L2_DOMAIN_WORKFLOW_MATRIX.md`  
File B            : 19 L2 source files listed in the conflict map  
Concept           : Workflow title  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. **0 H1/Index title mismatches across all 107 files.** The final three were corrected: A01 `Platform & Tenant Foundation` -> `Platform & Tenant Workflows`; F02 `Payment Management` -> `Payment Management Architecture`; A04 `Authorization and Policy Workflows` -> `Authorization & Policy Workflows`.

Canonical Source  : The Architecture Index is the master document registry.

Conflict Description :
Examples include A00 Review versus Architecture, A01 Workflows versus Foundation, B01 Workflows versus Lifecycle, B02 Organisation Workflows versus Organization Domain Architecture, D01 Course & Course Offering versus Course Offering only, and H00 Services Review versus Section H Architecture Governance and Certification. C01, C03, F06, H04-H06 lack a canonical `# L2-...` heading form.

Repository Impact :
Search, linking, generation, and architecture discussions use different names for the same ID.

Resolution        :
Make every H1 exactly `# <ID> — <Index Title>` and update filename/Matrix labels only through a single synchronized change.
---

---
Issue ID          : INT-014  
Conflict Type     : Type 6 — Structural Drift  
File A            : `templates/L2_DomainWorkflow_Template.md`  
File B            : 39 L2 files listed in the conflict map  
Concept           : Required L2 sections  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. **0 files missing a required template heading**, down from the original 39. Section G (8 files: G01-G04, G06-G08, G10) was resolved by renaming section 9 from `Bounded Context Relationships` to `Dependencies & Integration (Bounded Context Relationships)`, which surfaces the existing Upstream/Downstream subsections under a canonical heading without moving content. The remaining six gained explicit sections following the pattern L2-C00 already used: Lifecycle and Rules for B00 and D00, Rules for C00, Entities for D05, States and Dependencies for D07, and Entities for I13. For review and control documents the added sections state where the concern actually lives, or that it is not owned there — no content was invented.

Canonical Source  : `CONTRIBUTING.md` line 77 requires all standard template sections.

Conflict Description :
Automated heading verification found 39 files missing at least one of Purpose, Lifecycle, Workflows, States, Rules/Invariants, Events, Dependencies/Integration, or Entities/Aggregates. Missing counts: Lifecycle 10, Workflows 16, States 11, Rules 16, Events 5, Dependencies 11, Entities 7.

Repository Impact :
Cross-file reviews cannot compare the same architectural dimensions, and required contracts/states may be hidden or absent.

Resolution        :
Add the eight canonical headings to every listed file and move existing content beneath them without changing semantics.
---

---
Issue ID          : INT-015  
Conflict Type     : Type 5 — Broken Reference  
File A            : `AI_CONTEXT/SECTION_B/B_Canonical_AI_Context.md`  
File B            : `architecture/ARCHITECTURE_INDEX.md` and `architecture/L2_Domain_Workflow_Architecture/section-b/`  
Concept           : Phantom L2-B06 Operational Readiness  
Severity          : Critical  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. `L2-B06_Operational_Readiness_Workflows.md` was deleted and the string `B06` no longer occurs anywhere in the repository (0 matches). `B_Canonical_AI_Context.md` now reports 6 files processed, includes `L2-B00_Institution_and_Structure_Review.md` in the Dependency Manifest and Source Hierarchy, and carries a dated correction note routing Operational Readiness to `L2-A10`.

Canonical Source  : The actual Section B corpus and Index contain B00 through B05 only; Operational Readiness is A10.

Conflict Description :
The AI context says it processed six Section B files but omits B00 and invents `L2-B06_Operational_Readiness_Workflows.md` at lines 24, 58-60, and throughout 20 additional references. No B06 file or Index row exists.

Repository Impact :
An AI using the “canonical” context will design against a non-existent architecture ID and ignore the actual B00 review.

Resolution        :
Regenerate Section B from the actual directory, include B00, remove every B06 reference, and point Operational Readiness dependencies to L2-A10.
---

---
Issue ID          : INT-016  
Conflict Type     : Type 8 — Matrix / Index Mismatch  
File A            : `AI_CONTEXT/INDEX.md` and `AI_CONTEXT/README.md`  
File B            : Current `AI_CONTEXT/SECTION_*` directories  
Concept           : AI-context inventory/status  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. `AI_CONTEXT/INDEX.md` now records Sections A-J as Generated with `Pending: None`, and lists the exact section-prefixed basenames actually on disk (`A_Canonical_AI_Context.md`, `B_Canonical_AI_Context.md`, ... plus the `Dependency_Contract.md`/`README.md` extras for A, G, H and J). `README.md` also records `Pending sections: None`. Verified against a directory listing.

Canonical Source  : Filesystem state.

Conflict Description :
Index lines 7-8 and 17/22 and README lines 18-21 say D and I are pending/not generated, but both canonical context files exist. Index rows also name `Canonical_AI_Context.md` for A/B/C/E/G/H, while actual files are section-prefixed.

Repository Impact :
Automation skips existing contexts and tries to open non-existent outputs.

Resolution        :
Regenerate the AI_CONTEXT Index directly from the filesystem; record D and I as generated and list every exact basename.
---

---
Issue ID          : INT-017  
Conflict Type     : Type 5 — Broken Reference  
File A            : `AI_CONTEXT/SECTION_F/F_Canonical_AI_Context.md` and `AI_CONTEXT/SECTION_I/I_Canonical_AI_Context.md`  
File B            : Section F and I source directories  
Concept           : Source-file traceability  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. Every `*.md` basename cited in `F_Canonical_AI_Context.md` and `I_Canonical_AI_Context.md` was extracted and matched against the actual Section F and I directories - **0 missing**. The title-based strings that remain (e.g. `L2-F00 - Finance Domain Model & Foundation`) are heading labels in the Original Heading column, not file references. Caveat: the generation-time existence assertion recommended in the resolution was not added, so this can regress on the next regeneration.

Canonical Source  : Actual basenames in the architecture directories.

Conflict Description :
F context contains 302 references to 15 title-based basenames with spaces/em dashes that do not exist; I contains 68 references to 17 non-existent title-based basenames. Actual F/I filenames use underscores.

Repository Impact :
Every cited source link for those contexts fails, defeating auditability and AI evidence lookup.

Resolution        :
Replace references with exact filesystem basenames and add a generation-time existence assertion that fails on any missing source.
---

---
Issue ID          : INT-018  
Conflict Type     : Type 4 — Dependency Conflict  
File A            : `AI_CONTEXT/SECTION_G/Dependency_Contract.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-g/L2-G10_Infrastructure_Management_Workflows.md`  
Concept           : G10 dependency direction  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. `AI_CONTEXT/SECTION_G/Dependency_Contract.md` now reads: `L2-G10_Infrastructure_Management_Workflows.md | Depends on: Section A Platform Foundation; Section B Institution & Structure; Section E People & Workforce. Used by (downstream): G01 through G07; Section D Academic Delivery; Section I Examination; Future Domains` - matching G10's own metadata and Downstream Domains section. The apparent G01 -> G10 -> G01 cycle is gone.

Canonical Source  : G10 metadata line 11 and its Downstream Domains section lines 503-610.

Conflict Description :
The generated contract line 34 says G10 depends on G01-G07, D, and I. G10 itself depends only on A, B, and E and explicitly publishes physical context downstream to G01-G07, D, and I.

Repository Impact :
The generated contract reverses producer/consumer direction and appears to create cycles such as G01 -> G10 -> G01.

Resolution        :
Change the contract row to “depends on A/B/E; used by G01-G07/D/I” and validate generated direction against source `Dependencies` and `Downstream` headings.
---

---
Issue ID          : INT-019  
Conflict Type     : Type 5 — Broken Reference  
File A            : `section-c/L2-C07_Scholarship_Eligibility_Workflows.md` and `section-c/L2-C06_Guardian_Workflows.md`  
File B            : `architecture/ARCHITECTURE_INDEX.md`  
Concept           : L2 ID-to-name mappings  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. The two mermaid nodes in C06 section 21.17 are fixed: `C[A03 Authentication & Authorization]` split into `C[A03 Authentication]` and a new `C2[A04 Authorization & Policy]` (with a `C2 --> G` edge added so the node participates in the graph), and `SL[C01 Student Lifecycle]` corrected to `SL[C02 Student Lifecycle]`. The `F[B03 Campus & Infrastructure]` label was also refreshed to `F[B03 Institutional Campus]` following the INT-001 narrowing. C07 was already fully corrected.

Canonical Source  : Architecture Index rows for A03/A04, B03/B04, C01/C02/C06.

Conflict Description :
C07 maps B03 to Programme Portfolio although B03 is Campus; maps C02 to Admissions although C02 is Student Lifecycle; maps C06 to Student Documents although C06 is Guardian. C06 maps A03 to Authentication & Authorization although Authorization is A04, and maps C01 to Student Lifecycle although C01 is Admissions.

Repository Impact :
Calls and read contracts are routed to the wrong bounded context and wrong architecture document.

Resolution        :
Correct mappings to B04 Curriculum Framework, C01 Admissions, C02 Student Lifecycle, C06 Guardian, A03 Authentication, and A04 Authorization; re-review all C06/C07 dependency tables.
---

---
Issue ID          : INT-020  
Conflict Type     : Type 8 — Matrix / Index Mismatch  
File A            : `AI_CONTEXT/SECTION_A/A_Canonical_AI_Context.md` and `AI_CONTEXT/AUDITS/A_Canonical_Audit.md`  
File B            : `section-a/L2-A10_Operational_Readiness_Workflows.md`  
Concept           : Section A coverage  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. `A_Canonical_AI_Context.md` now reports **11 files processed** (was 10) and carries a full `L2-A10_Operational_Readiness_Workflows.md` source-hierarchy entry with its canonical entities (Operational Readiness Assessment, Readiness Requirement, Readiness Finding, Readiness Improvement Plan, Operational Readiness Certificate), business capabilities and extracted content, plus a Dependency Manifest row and a dated correction note. `A_Canonical_Audit.md` reports **11 markdown files**, makes the completeness claim contingent on directory count and ID-set equality, and adds a Coverage Correction section recording the prior defect. The A10 entry also notes that Operational Readiness belongs to Section A, not Section B — closing the loop with INT-015.

Canonical Source  : Current Section A has 11 files, A00-A10.

Conflict Description :
Both context and audit say 10 files processed and contain no A10 entry; the audit nevertheless says source coverage is complete.

Repository Impact :
AI answers omit the platform’s operational-readiness architecture.

Resolution        :
Regenerate A context/audit from all 11 files and make “complete” contingent on actual directory count and ID-set equality.
---

---
Issue ID          : INT-021  
Conflict Type     : Type 1 — Terminology Conflict  
File A            : `generated/html/SutraERP_Level_1_Architecture_Locked.html`  
File B            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`  
Concept           : Graduation versus Qualification Completion  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. Both HTML artifacts were regenerated from current Markdown after INT-022 unblocked the tooling. The three superseded constructs are gone: `L1-12 Graduation / Student Exit` (0), `Graduation Requirements` (0), `Graduation Approved` (0). Residual Graduation terminology was also removed from the L1 and L0 **sources** — L1 `Graduation Requirements` -> `Qualification Completion Requirements`, state `Graduated` -> `Qualification Completed`, `Active --> Graduated` -> `Active --> QualificationCompleted`, and three L0 `Graduation / Exit` -> `Qualification Completion / Exit`. The five remaining L1 mentions are the sentences that *document* the replacement ("Qualification Completion replaces Graduation") and are correct as written. Each artifact now carries a provenance stamp — `generated-from`, `source-sha256`, `generated-at` — and both verify **IN SYNC** against a freshly computed source hash, so staleness is now detectable without diffing.

Canonical Source  : Locked L1 lines 824, 2469, and 2975 says Qualification Completion replaces Graduation.

Conflict Description :
The HTML still presents “L1-12 Graduation / Student Exit,” “Graduation Requirements,” and “Graduation Approved” at lines 332-348 and 919-956.

Repository Impact :
Consumers of the presentation artifact receive the superseded lifecycle and terminology.

Resolution        :
Regenerate the HTML from the current L1 Markdown and add a source hash/timestamp to generated artifacts.
---

---
Issue ID          : INT-022  
Conflict Type     : Type 5 — Broken Reference  
File A            : `tools/convert_all.js` and `tools/wrap.js`  
File B            : Current architecture and generated paths  
Concept           : Presentation conversion paths  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. A new `tools/paths.js` module resolves every path from the repository root via `path.resolve(__dirname, "..")` and exports the canonical L0/L1 source-to-output mapping. `tools/wrap.js` now imports it, creates `generated/html/` if absent, writes there rather than beside the script, emits the provenance stamp, and **exits non-zero listing every missing source**. `tools/convert_all.js` uses the same module and also exits non-zero on a missing source; because it rewrites source Markdown in place — including the **locked** L1 — it now additionally requires an explicit `--write` flag and defaults to a dry run. All three files pass `node --check`, and an end-to-end run regenerated both artifacts successfully.

Canonical Source  : Current repository structure in README.

Conflict Description :
`convert_all.js` lines 71-72 looks for old root-level Markdown names. `wrap.js` lines 5-7 and 217-229 resolves Markdown and HTML beside the script under `tools/`, not under `architecture/...` and `generated/html/...`.

Repository Impact :
The stale HTML cannot be reliably regenerated by the repository’s own tools.

Resolution        :
Resolve source paths from the repository root to L0/L1 files and outputs to `generated/html`; exit non-zero when a source is missing.
---

---
Issue ID          : INT-023  
Conflict Type     : Type 1 — Terminology Conflict  
File A            : `governance/constitution/Volume_2_Repository_Standards.md`  
File B            : `architecture/ARCHITECTURE_INDEX.md`  
Concept           : Document status vocabulary  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated by amendment. `governance/constitution/Volume_2_Repository_Standards.md` is now **v1.1** and its section 7 canonical status list carries all 13 values (the 9 in active use plus Proposed, Approved, Deprecated, Archived), and it explicitly states "This list is the single canonical status vocabulary for the repository. `ARCHITECTURE_INDEX.md`'s Document Status Legend mirrors this table exactly ... if the two ever diverge, this section governs." The Index legend cross-references Volume 2 section 7. Authority is no longer contested. (Two documents still declare status values outside this list - tracked under INT-011, not here.)

Canonical Source  : Unresolved: both documents claim normative authority.

Conflict Description :
Volume 2 lines 127-136 says every document has exactly one status from Draft, Proposed, Approved, Locked, Deprecated, Archived. Index lines 339-347 adds Review, Trust Certification Ready, Architecture Approved, Approved Baseline, Architecture Baseline, Architecture Locked, and Certified & Locked, and says exact variants are intentional.

Repository Impact :
Validation cannot decide whether Index statuses are legal or constitutional violations.

Resolution        :
Amend Volume 2 to define the extended state machine or collapse the Index to the constitutional states with a separate certification qualifier.
---

---
Issue ID          : INT-024  
Conflict Type     : Type 6 — Structural Drift  
File A            : L2 files `C07,D00,G01,G07,G08`  
File B            : `governance/constitution/Volume_2_Repository_Standards.md`  
Concept           : Mojibake-corrupted architecture text  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03, final pass) Closed. **Repository-wide mojibake is now zero**, down from the original 720 lines. The remaining G01 (190), G07 (120) and G08 (120) were repaired by the exact inverse of the corruption: the text was encoded back to CP1252 and decoded as UTF-8, restoring box-drawing characters, arrows and em dashes. The five CP1252-undefined slots (0x81, 0x8D, 0x8F, 0x90, 0x9D) were mapped by hand since the corruption had passed them through a lenient decoder. A dry run confirmed a clean 358/262/198 -> 0/0/0 round-trip before writing, and byte-order marks were preserved. State-transition diagrams and permission tables in those files now render.

Canonical Source  : UTF-8 repository text and the intended Unicode symbols.

Conflict Description :
Five files contain 720 lines with `â...` corruption. C07 alone has 285 affected lines, including transition arrows, diagram branches, checkmarks, permission tables, and headings; G01 has 190, G07/G08 120 each, D00 5.

Repository Impact :
State transitions and authorization matrices are not reliably readable; copy/paste creates invalid event and ID text.

Resolution        :
Run the repository’s mojibake repair in preview, review diffs, apply only validated UTF-8 repairs, then rerun transition/permission-table checks.
---

---
Issue ID          : INT-025  
Conflict Type     : Type 1 — Terminology Conflict  
File A            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-i/L2-I10_Result_Publication_Workflows.md`  
Concept           : Result publication event name  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. `L2-I10_Result_Publication_Workflows.md` now uses `ResultPublished` in its Events Published table (line 1029) and throughout (lines 192, 1572). The `Results Published` spaced/plural form no longer occurs.

Canonical Source  : L1 lines 2065 and 2935 uses `ResultPublished`; Volume 2 requires past-tense PascalCase events.

Conflict Description :
I10’s Events Published table line 1025 names the event `Results Published` with spaces and plural Results.

Repository Impact :
Event schema registration and consumers cannot match the L1 event name.

Resolution        :
Rename the I10 event to `ResultPublished` everywhere and use human labels only in a separate description column.
---

---
Issue ID          : INT-026  
Conflict Type     : Type 8 — Matrix / Index Mismatch  
File A            : `architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md`  
File B            : `architecture/ARCHITECTURE_INDEX.md` and `TEAM_ASSIGNMENTS.md`  
Concept           : Matrix control completeness  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. Mechanical re-check of `L2_DOMAIN_WORKFLOW_MATRIX.md`: **107 workflow blocks, 107 unique IDs, zero duplicates** - the second L2-I00 entry is gone. Every block now carries both `Owner:` and an explicit `Status: [...]`, and a full row-by-row comparison against `ARCHITECTURE_INDEX.md` shows **zero status or owner drift**. The two remaining `Status: [ ]` blanks are on the non-workflow control sections (L2-00 Architecture Control and the final L2 review block), not on any workflow ID.

Canonical Source  : Index for status, Team Assignments for owner.

Conflict Description :
Matrix defines L2-I00 twice (lines 320 and 371), contains no owner field for any workflow, and represents Draft rows as empty `Status: [ ]` instead of the exact Index status.

Repository Impact :
The matrix cannot perform its stated ownership/status control function or be machine-compared with the Index.

Resolution        :
Use one row per ID with explicit `Owner` and exact `Status`; remove the duplicate I00 review entry.
---

---
Issue ID          : INT-027  
Conflict Type     : Type 2 — Ownership Conflict  
File A            : `architecture/L0_Business_Vision/SutraERP_Architecture_Workflow_Context_Baseline.md`  
File B            : `section-j/L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md` and `section-f/L2-F14_Procurement_Finance_Consumption_Workflows.md`  
Concept           : Procurement  
Severity          : High  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated by L0 amendment. The L0 baseline header now reads `**Last Amended:** 2026-08-01 --- Procurement removed from Finance`, and L0 now lists Procurement, Inventory, Vendor & Supplier Management, Warehouse & Stock Operations, Asset Lifecycle Management and Physical Verification & Disposal under `Section J --- Enterprise Resource Management`. L0 principle 19 explicitly separates Section F (Finance) from Section J ("including Procurement and Inventory"). This matches J00 and F14. Procurement no longer appears in the L0 Finance scope list.

Canonical Source  : L0 is the highest authority but is internally ambiguous because it lists Procurement under both Finance and selected operational services.

Conflict Description :
L0 lines 401-410 and 897-910 puts Procurement/Purchase inside Finance scope. J00 lines 160-186 says Section J is separate from Finance and owns Procurement; F14 lines 25-38 says Finance does not own procurement and only consumes J facts.

Repository Impact :
Procurement aggregates, supplier authority, and purchasing rules have incompatible upstream ownership.

Resolution        :
Amend L0/L1 to explicitly transfer procurement execution to J and limit Finance to budget/AP/accounting consequences, matching J00/F14; remove Procurement from the L0 Finance scope list.
---

---
Issue ID          : INT-028  
Conflict Type     : Type 8 — Matrix / Index Mismatch  
File A            : `architecture_audit_report.md`  
File B            : Current repository corpus  
Concept           : Audit certification state  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. `architecture_audit_report.md` now carries `**Status:** Superseded - 2026-07-31` immediately under its title, states that it reviewed only 5 documents and predates the 107-file L2 corpus, warns it must not be treated as a current certification, and links to `generated/reviews/integrity-2026-07-31-complete.md`.

Canonical Source  : Current 107-file L2 inventory and this full read.

Conflict Description :
The report says only five documents were reviewed, zero issues remain, health is 100, every lifecycle is complete, and every document agrees. It has no superseded status and directly contradicts current evidence.

Repository Impact :
Readers may treat an obsolete partial audit as a current certification.

Resolution        :
Mark it `Superseded` with date/scope and link to this report, or archive it.
---

---
Issue ID          : INT-029  
Conflict Type     : Type 5 — Broken Reference  
File A            : `generated/reviews/integrity-2026-07-30.md` and `integrity-2026-07-31.md`  
File B            : `prompts/reviews/review-integrity.md`  
Concept           : Generated integrity report artifacts  
Severity          : Medium  
Confidence        : Certain  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated. `generated/reviews/` now contains only `integrity-2026-07-31-complete.md`. Both session-limit-error artifacts (`integrity-2026-07-30.md` and `integrity-2026-07-31.md`) were deleted, so date-based review discovery can no longer return an error string as a certification.

Canonical Source  : The review prompt’s required report structure.

Conflict Description :
Both files contain only “You've hit your session limit...” and no scorecard, conflict map, or findings.

Repository Impact :
Date-based review discovery returns an error string as if it were a completed certification.

Resolution        :
Delete or rename failed runs with `.failed` metadata and publish only validated reports under the integrity naming convention.
---

---
Issue ID          : INT-030  
Conflict Type     : Type 7 — Business Rule Conflict  
File A            : `AI_CONTEXT/REPO_UNDERSTANDING.md`  
File B            : `architecture/L2_Domain_Workflow_Architecture/section-a/L2-A01_Platform_Tenant_Foundation_Architecture.md`  
Concept           : Tenant-to-Institution cardinality  
Severity          : Medium  
Confidence        : Likely  
Issue Status      : CLOSED  
Verification      : (re-verified 2026-08-03) Fully remediated and locked upstream. `AI_CONTEXT/REPO_UNDERSTANDING.md` now reads "a single Tenant (e.g. a Trust) may operate one or more Institutions", matching A01 lines 648/882/1043. The cardinality is now explicit in L1 in both directions: line 164 (`Tenant-->Institute[One or More Institutions]`) sets the upper bound, and line 1091 ("One Institute belongs to exactly one Tenant") sets the lower. A01 lines 1163-1165 keep College/School/Campus at exactly one Institution.

Canonical Source  : L1 states each Institution belongs to exactly one Tenant but does not define the Tenant upper bound; the rule needs an explicit L0/L1 lock.

Conflict Description :
Repository Understanding line 20 says one Tenant contains one Institute. A01 lines 648, 882, and 1043 says a Tenant may operate/support one or more Institutions.

Repository Impact :
Schema keys, isolation rules, routing context, and tenant provisioning differ between 1:1 and 1:n models.

Resolution        :
Lock the cardinality in L0/L1. If A01 is intended, change the context to “one Tenant may contain one or more Institutions”; otherwise constrain A01 to exactly one.
---

# NEW FINDINGS - 2026-08-03 REGRESSION SWEEP

The user reported adding new files and renaming files in Sections B, C, D, E and G, and asked whether the new names are correct so the index files could be updated.

**Answer: the new names are not correct, and the index files have deliberately NOT been updated.** Updating `ARCHITECTURE_INDEX.md`, `L2_DOMAIN_WORKFLOW_MATRIX.md` and the AI contexts to match the current disk state would ratify a set of changes that reverse locked L0/L1 decisions, break a working event contract, delete a canonical document, and introduce a duplicate architecture ID. The control artifacts are currently *correct* and the *corpus* has drifted away from them - so the corpus is what needs fixing, not the indexes.

Actual per-section delta (Sections B and E have no new or renamed files at all):

| Section | Change on disk |
|---|---|
| B | **None.** B00-B05 unchanged. (The earlier B06 deletion remains correct - see INT-015.) |
| C | `L2-C02_Student_Lifecycle_and_Activation.md` replaced by `L2-C02_ Student_Lifecycle_and_Activation.md` (stray space); `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md` replaced by `L2-C05_Graduation_and_Student_Exit_Workflows.md`; `L2-C08` heavily edited in place |
| D | `L2-D06_Open_Electives_and_MOOC_ Workflows.md` **added alongside** the existing `L2-D06_Open_Electives_and_MOOCs_Workflows.md` |
| E | **None.** E00-E07 all present and unrenamed. |
| G | `L2-G10_Infrastructure_Management_Workflows.md` **deleted with no replacement.** Section G is now G00-G09. |
| I (not reported by user) | `L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md` replaced by `L2-I13_Academic_Completion_CGPA_and_Graduation.md` |

Net file count is still 107, so any check that only counts files will pass. The ID set is what broke: **106 unique IDs on disk against 107 Index rows**, because D06 gained a duplicate (+1 file) while G10 was deleted (-1 file).

---
Issue ID          : INT-031
Conflict Type     : Type 5 - Broken Reference
File A            : `architecture/L2_Domain_Workflow_Architecture/section-g/` (L2-G10 absent)
File B            : `architecture/ARCHITECTURE_INDEX.md`, `L2_DOMAIN_WORKFLOW_MATRIX.md`, `AI_CONTEXT/SECTION_G/`, and 9 L2 source files
Concept           : Canonical Infrastructure Management document
Severity          : Critical
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) File restored by the user as `L2-G10_Infrastructure_Management Workflows.md` and renamed here to `L2-G10_Infrastructure_Management_Workflows.md`. The restored copy is **3,153 lines and is the remediated version** - it carries "Institutional campus governance ... remain owned by Section B" (1), "does not own institutional campus governance" (1) and "Institutional campus governance" (3), so the G10 side of INT-001 and the whole of INT-018 survive intact. L2-G10 resolves again in the Index, Matrix and all 114 references. Status normalized from the non-legend `Architecture Draft` to `Draft`; `Document Owner: Devang` added to match the Index.

Canonical Source  : Architecture Index row `| L2-G10 | Infrastructure Management Workflows | Devang | Draft |` and Matrix block `## L2-G10`, both still present.

Conflict Description :
`L2-G10_Infrastructure_Management_Workflows.md` was deleted from the working tree with no replacement file. Every consumer still points at it: Index (1 reference), Matrix (1), `AI_CONTEXT/SECTION_G/G_Canonical_AI_Context.md` (37), `AI_CONTEXT/SECTION_G/Dependency_Contract.md` (9), `AI_CONTEXT/REPO_UNDERSTANDING.md` (1), plus 65 in-corpus references across `L2-B00` (10), `L2-G00` (10), `L2-G01` (14), `L2-G02` (9), `L2-G03` (17), `L2-G04` (2), `L2-G07` (1), `L2-G08` (1) and `L2-H03` (1).

Repository Impact :
L2-G10 is now a dangling architecture ID. The physical infrastructure model that G01-G08, Section D and Section I all depend on has no source document. This also silently reverses two previously CLOSED findings: the INT-018 dependency-direction fix was verified against G10 metadata that no longer exists, and the G10 half of the INT-001 remediation (the only side that had been fixed) is gone.

Resolution        :
Restore the file. `git show HEAD:architecture/L2_Domain_Workflow_Architecture/section-g/L2-G10_Infrastructure_Management_Workflows.md` recovers a 3,092-line version, but note this is the **pre-remediation** copy - it does not contain the "Institutional campus governance ... remain owned by Section B" text that closed the G10 side of INT-001. That remediation was working-tree-only and is not in git history. If it cannot be recovered from editor local history, INT-001 must be re-done from scratch on the restored file.
---

---
Issue ID          : INT-032
Conflict Type     : Type 8 - Matrix / Index Mismatch
File A            : `section-d/L2-D06_Open_Electives_and_MOOCs_Workflows.md`
File B            : `section-d/L2-D06_Open_Electives_and_MOOC_ Workflows.md`
Concept           : Duplicate architecture ID L2-D06
Severity          : Critical
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) Verified: `L2-D06_Open_Electives_and_MOOC_ Workflows.md` is gone and `section-d` now holds exactly one D06 file, `L2-D06_Open_Electives_and_MOOCs_Workflows.md`, which retains `| Course Master | L2-B04 |` and the `Pallav` owner metadata. Mechanical re-check confirms zero duplicate IDs across all 107 files.

Canonical Source  : One file per architecture ID. The Index and Matrix each carry exactly one L2-D06 row.

Conflict Description :
Two files now claim ID L2-D06. Both are 15,708 lines; they differ in 52 CR-normalized lines. The new file carries the **older** metadata: `| Primary Owner | Academic Affairs |` and `| Canonical Owner | Open Elective & Learning Enrollment Domain |` where the retained file has `Pallav` for both, and it uses the superseded "Cross-Domain" wording where the retained file uses "Multi-Domain". Both files do carry the corrected `| Course Master | L2-B04 |` row, so INT-010 is not affected.

Repository Impact :
Any tool that resolves an ID to a file is now non-deterministic for D06. The INT-012 owner remediation is reversed in whichever copy wins. Because the deletion of G10 offsets this addition, total file count still reads 107 and a count-only check will not catch it.

Resolution        :
Delete `L2-D06_Open_Electives_and_MOOC_ Workflows.md`. Its only content difference is stale owner metadata and superseded wording; it supersedes nothing. If any of its 52 differing lines are actually wanted, port them into the canonical file first.
---

---
Issue ID          : INT-033
Conflict Type     : Type 1 - Terminology Conflict
File A            : `architecture/L1_Business_Domain_Architecture/SutraERP_L1_Business_Domain_Architecture_Locked.md` and `governance/constitution/Volume_2_Repository_Standards.md`
File B            : `section-c/L2-C02_ Student_Lifecycle_and_Activation.md`, `section-c/L2-C05_Graduation_and_Student_Exit_Workflows.md`, `section-c/L2-C08_Alumni_Transition_Workflows.md`, `section-i/L2-I13_Academic_Completion_CGPA_and_Graduation.md`
Concept           : Graduation versus Qualification Completion
Severity          : Critical
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) Terminology decision confirmed as **Qualification Completion** and applied. Files renamed to `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md` and `L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md`; both H1 titles now match their Index titles exactly. Content transformed across C02, C05, C08 and I13 - `graduat*` token count went 148/562/74/202 to **0/0/0/0**. Section C maps Graduation to Qualification Completion; Section I maps it to the examination-side "Completion Evidence" family instead, so I13 never claims Qualification Completion ownership.

Canonical Source  : Locked L1 lines 824, 2469 and 2975 state that Qualification Completion **replaces** Graduation. Volume 2 section 9 names `QualificationAwarded` as a canonical event example.

Conflict Description :
Graduation terminology was reintroduced into the source corpus at the filename, title, and event level. C05 is renamed `Graduation_and_Student_Exit_Workflows` and titled "Graduation & Student Exit Workflows"; I13 is renamed `Academic_Completion_CGPA_and_Graduation` and carries 191 "Graduation" occurrences; C08 was edited in place, replacing "qualification completion verification" with "Graduation verification" throughout; C02 replaces "Student qualification completion handoff" with "Student graduation handoff" and "Qualification Completion Handoff" with "Graduation Handoff".

Repository Impact :
This reverses a locked L1 decision without an L1 amendment, and it inverts the direction of INT-021 - previously only the generated HTML lagged behind the Markdown, and now the Markdown source has regressed to match the stale HTML.

Resolution        :
Either restore Qualification Completion terminology across C02/C05/C08/I13, or raise a formal L1 amendment reversing lines 824/2469/2975 and update Volume 2 section 9 with it. Do not leave L1 and L2 disagreeing.
---

---
Issue ID          : INT-034
Conflict Type     : Type 1 - Terminology Conflict
File A            : `section-c/L2-C05_Graduation_and_Student_Exit_Workflows.md` and `section-c/L2-C02_ Student_Lifecycle_and_Activation.md`
File B            : `section-c/L2-C08_Alumni_Transition_Workflows.md` and `section-h/L2-H06_Alumni_Workflows.md`
Concept           : Qualification-to-Alumni event chain
Severity          : Critical
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) Chain repaired and verified end to end: I13 publishes `ExaminationCompletionEvidencePrepared` (2) -> C05 consumes it (3) -> C05 publishes `QualificationAwarded` (12) -> C08 consumes it (3, including the AT-WF-01 trigger) -> C08 publishes `AlumniTransitionCompleted` (18) -> H06 consumes it (13) and publishes `AlumniProfileCreated` (9). C02 was converted from publisher to consumer: BR-C02-032 now reads "Student Lifecycle shall not publish a competing canonical event for this transition", EVT-C02-009 is marked consumed, SC-C02-009 publishes none, and `Publish QualificationAwarded` appears **0** times.

Canonical Source  : The chain verified CLOSED under INT-007 on 2026-08-03: `QualificationAwarded` (C05) -> `AlumniTransitionCompleted` (C08) -> `AlumniProfileCreated` (H06).

Conflict Description :
The chain is now broken at every link. C05 publishes `GraduationApproved` (12 occurrences, lines 945, 1209, 1212, 2140, 2302, 2320) and contains zero `QualificationAwarded`. C02 publishes `StudentGraduated` (19 occurrences; BR-C02-032 now reads "Graduation shall publish StudentGraduated event", reversing the rule that previously forbade C02 from publishing a competing canonical event). C08 consumes **none** of them - `QualificationAwarded` = 0, `StudentGraduated` = 0, `GraduationApproved` = 0 - yet still publishes `AlumniTransitionCompleted` (18 occurrences). H06 is unchanged and still requires `AlumniTransitionCompleted` (13 occurrences) as the sole precondition for Alumni Profile creation (BR-H06-001).

Repository Impact :
C08's transition workflow now has no declared trigger event with a publisher, so `AlumniTransitionCompleted` can never fire, so H06 can never create an Alumni Profile. This is strictly worse than the original INT-007: previously the names merely disagreed, now there is a hole in the middle of the chain.

Resolution        :
Restore a single publisher for the C05 -> C08 edge. Reverting C02/C05/C08 to the `QualificationAwarded` contract restores a chain already verified end to end; any other choice requires editing H06 as well, which is currently the only correct file in the chain.
---

---
Issue ID          : INT-035
Conflict Type     : Type 2 - Ownership Conflict
File A            : `section-i/L2-I13_Academic_Completion_CGPA_and_Graduation.md`
File B            : `section-c/L2-C05_Graduation_and_Student_Exit_Workflows.md`
Concept           : Academic/Qualification Completion ownership
Severity          : High
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) Both ends restored. I13 Principle 5 now carries an explicit disclaimer: "This domain does not own Qualification Completion, which remains owned by Student Lifecycle (L2-C05). Examination prepares and publishes completion evidence only..." C05's Examination Evidence Boundary now names the event: it consumes examination evidence "received as the canonical `ExaminationCompletionEvidencePrepared` event", and lists Examination Domain (L2-I13) as the upstream source. I13 also retained its own line 32 disclaimer ("The domain does not determine Qualification Completion or degree award decisions") and the section 11.7 event definition throughout.

Canonical Source  : Locked L1 lines 2050-2053 and 2666-2669 - Examination never owns Qualification Completion.

Conflict Description :
The INT-005 remediation was partly undone. The replacement I13 no longer contains the sentence "This domain does not own Qualification Completion, which remains owned by Student Lifecycle (L2-C05)" (0 occurrences). It does still define `ExaminationCompletionEvidencePrepared` (lines 657 and 1243) and reference L2-C05 four times, so the handoff is not fully lost - but the receiving end is: the new C05 contains **zero** references to `ExaminationCompletionEvidencePrepared`, where the previous C05 consumed it at lines 1666 and 2322.

Repository Impact :
The evidence handoff is now one-directional - I13 publishes evidence that C05 does not consume - and I13 no longer disclaims completion ownership, so INT-005 is reopened in a partially-wired state.

Resolution        :
Restore the C05 consumption of `ExaminationCompletionEvidencePrepared` and reinstate the I13 ownership disclaimer. Then complete the residual INT-005 work already identified (I13 section 5.1).
---

---
Issue ID          : INT-036
Conflict Type     : Type 5 - Broken Reference
File A            : `section-c/L2-C02_ Student_Lifecycle_and_Activation.md`
File B            : `architecture/ARCHITECTURE_INDEX.md`
Concept           : Obsolete pre-restructure section scheme
Severity          : High
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) The obsolete "Used By" block in C02 was replaced. `L2-K01`, `L2-L01`, `L2-M01` and `L2-N01` are gone (Sections K-N do not exist), and the four mis-routed IDs were corrected to the current A-J scheme: Attendance -> L2-D04, Hostel -> L2-G03, Library -> L2-G05, Transport -> L2-G04, plus Placement -> L2-H04, Alumni -> L2-C08 and Guardian -> L2-C06. Zero references to Sections K-N remain.

Canonical Source  : The repository has Sections A through J only.

Conflict Description :
The replacement C02 is a pre-restructure draft. Its "Used By" list reads `L2-E01 - Attendance`, `L2-I01 - Examination`, `L2-F01 - Finance`, `L2-H01 - Hostel`, `L2-J01 - Library`, `L2-K01 - Transport`, `L2-L01 - Placement`, `L2-M01 - Alumni`, `L2-N01 - Certificates & Documents`. **Sections K, L, M and N do not exist.** The four IDs that do exist all point to the wrong document under the current scheme: E01 is Workforce & Engagement (not Attendance), H01 is a Section H services document (not Hostel - Hostel is G03), I01 is Examination Policy & Configuration, and J01 is Procurement Planning & Sourcing (not Library - Library is G05). The retained-version equivalents were correct.

Repository Impact :
Four dangling IDs and four mis-routed dependencies in the Student Lifecycle document that most other domains depend on.

Resolution        :
Do not adopt this file. If any of its content is wanted - notably its StudentContext model, which is relevant to INT-009 - port that content into the current C02 rather than replacing the file.
---

---
Issue ID          : INT-037
Conflict Type     : Type 6 - Structural Drift
File A            : `governance/constitution/Volume_2_Repository_Standards.md` section 9
File B            : `section-c/L2-C02_ Student_Lifecycle_and_Activation.md` and `section-d/L2-D06_Open_Electives_and_MOOC_ Workflows.md`
Concept           : Filename convention
Severity          : Medium
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) All stray spaces removed. `L2-C02_ Student_Lifecycle_and_Activation.md` -> `L2-C02_Student_Lifecycle_and_Activation.md`; `L2-G10_Infrastructure_Management Workflows.md` -> `L2-G10_Infrastructure_Management_Workflows.md`; the duplicate `L2-D06_Open_Electives_and_MOOC_ Workflows.md` was deleted. Automated convention check now reports **zero** violations outside Section J. The 11 Section J em-dash filenames remain as the separate pre-existing cleanup they always were.

Canonical Source  : Volume 2 section 9 requires the `L2-[section][number]` identifier form; the entire A-I corpus uses `L2-XNN_Underscore_Separated_Name.md`.

Conflict Description :
Both new filenames contain a stray space: `L2-C02_ Student_Lifecycle_and_Activation.md` (space after the ID underscore) and `L2-D06_Open_Electives_and_MOOC_ Workflows.md` (space before "Workflows", plus singular "MOOC" against the Index title "Open Electives & MOOCs Workflows"). Automated convention checking now flags 13 filenames, of which 2 are these new ones; the other 11 are the pre-existing Section J files that use ` - ` em-dash separators rather than underscores.

Repository Impact :
Spaces in paths break shell tooling, link resolution and the conversion scripts, and prevent reliable ID-to-file matching.

Resolution        :
Rename to the underscore convention with no spaces. Handle the 11 Section J filenames as a separate, pre-existing cleanup.
---

---
Issue ID          : INT-038
Conflict Type     : Type 8 - Matrix / Index Mismatch
File A            : `architecture/ARCHITECTURE_INDEX.md` and `L2_DOMAIN_WORKFLOW_MATRIX.md`
File B            : Current Section C, D, G and I corpus
Concept           : Control artifacts versus corpus
Severity          : High
Confidence        : Certain
Issue Status      : CLOSED
Verification      : (re-verified 2026-08-03, third sweep) Index and Matrix were never edited - the corpus was brought back to them, as intended. Re-verification: **107 files / 107 unique IDs / 107 Index rows**, zero duplicates, zero dangling, zero orphans; Matrix shows **107 blocks, 107 unique, zero status/owner/title drift**. The metadata regressions were reversed: C02 status `🔒 LOCKED v1.0` -> `Locked` with `Document Owner: Varad` restored, C05 gained `Document Owner: Varad`, and four further title regressions the sweep uncovered were repaired (B01 `Institution Lifecycle` -> `Institution Workflows`, D01 `Course Offering Workflows` -> `Course & Course Offering Workflows`, C01 lost `#` heading marker restored, C03 missing H1 re-added), plus status regressions in B01 (`🔒 LOCKED v1.0` -> `Locked`) and E03 (`Architecture Baseline Draft` -> `Architecture Baseline`).

Canonical Source  : The Index and Matrix, which were reconciled to zero drift on 2026-08-03 (see INT-011, INT-013, INT-026).

Conflict Description :
The corpus has drifted away from control artifacts that are themselves still correct. The Index and Matrix both still read `Qualification Completion & Student Exit Workflows` for C05 and `Academic Completion, CGPA & Completion Evidence Workflows` for I13, against on-disk titles that now say Graduation; both still carry an L2-G10 row with no file behind it; the D06 row cannot resolve to one file. Two previously-remediated metadata fixes are also reversed: C02's status went from `Locked` back to `🔒 LOCKED v1.0` (not a legend value) and its `Document Owner: Varad` line was removed, and C08's `| Document Owner | Varad |` row was removed.

Repository Impact :
Index and Matrix drift was driven to zero on 2026-08-03 and is now non-zero again, and parts of the INT-011 and INT-012 remediation are undone.

Resolution        :
Fix the corpus, not the indexes. Once INT-031 through INT-037 are resolved, re-run the Index/Matrix comparison to confirm drift returns to zero. **Do not update the Index or Matrix to match the current disk state** - that would ratify the regressions and destroy the reconciliation baseline.
---

---
Issue ID          : INT-039
Conflict Type     : Type 6 - Structural Drift
File A            : Working tree
File B            : `git` history at commit `9471421`
Concept           : Uncommitted remediation lost
Severity          : Critical
Confidence        : Certain
Issue Status      : CLOSED - NOT MATERIAL
Verification      : (re-verified 2026-08-03, third sweep) Resolved by circumstance, not recovery. The user restored G10 from their own copy and it proved to be the **remediated** version, so the INT-001/INT-018 work was never actually lost. The C05/C08/I13 remediation was re-done from scratch in this sweep rather than recovered. The underlying process risk stands and is unchanged: this remediation is still uncommitted, and `git stash` is still empty. **Commit it.**

Canonical Source  : The 2026-08-03 verification pass, which read the remediated content directly from the working tree.

Conflict Description :
The remediation verified earlier on 2026-08-03 for C05, C08, I13 and G10 existed **only in the working tree and was never committed**. Commit `9471421` does not contain it: HEAD's C05 has zero `QualificationAwarded`, HEAD's I13 has zero `ExaminationCompletionEvidencePrepared` and zero completion-ownership disclaimer, and HEAD's G10 has zero occurrences of the "Institutional campus governance ... remain owned by Section B" text. `git stash list` is empty and there is no staged copy. Replacing those files therefore destroyed the only copy.

Repository Impact :
The work that moved INT-005, INT-007 and half of INT-001 forward is not recoverable from git. `git show HEAD:<path>` restores the *pre-remediation* versions only.

Resolution        :
Check editor local history first (VS Code: Timeline view on each file, or `%APPDATA%/Code/User/History`) before re-doing the work. Then commit remediation as it is produced - a locked-architecture repository should not carry multi-thousand-line uncommitted remediation across sessions.
---


# THIRD SWEEP — REGRESSION REMEDIATION (2026-08-03)

The nine regression findings (INT-031 to INT-039) were remediated in this sweep. The terminology question was decided by the repository owner in favour of **Qualification Completion**, consistent with locked L1 and Volume 2 section 9.

## Actions taken

| # | Action | Detail |
|---:|---|---|
| 1 | G10 restored | The owner restored the file; it proved to be the **remediated** 3,153-line version, so the INT-001 and INT-018 work was never lost. Renamed from `L2-G10_Infrastructure_Management Workflows.md` (space) to `L2-G10_Infrastructure_Management_Workflows.md`. Status `Architecture Draft` -> `Draft`; `Document Owner: Devang` added. |
| 2 | Duplicate D06 removed | Confirmed deleted by the owner. Section D holds exactly one D06 file. |
| 3 | Files renamed | `L2-C02_ Student_Lifecycle_and_Activation.md` -> `L2-C02_Student_Lifecycle_and_Activation.md`; `L2-C05_Graduation_and_Student_Exit_Workflows.md` -> `L2-C05_Qualification_Completion_and_Student_Exit_Workflows.md`; `L2-I13_Academic_Completion_CGPA_and_Graduation.md` -> `L2-I13_Academic_Completion_CGPA_and_Completion_Evidence_Workflows.md`. All now match their Index titles exactly. |
| 4 | Terminology restored | `graduat*` token counts: C02 148 -> 0, C05 562 -> 0, C08 74 -> 0, I13 202 -> 0. |
| 5 | Event chain repaired | C05 publishes `QualificationAwarded`; C08 consumes it (including the AT-WF-01 trigger); C02 converted from publisher to consumer; C05 wired to consume `ExaminationCompletionEvidencePrepared`. |
| 6 | Collateral repaired | Six further regressions the sweep uncovered: titles (B01, C01, C03, D01), statuses (B01, E03, E04, E05), and `Document Owner` fields stripped from 10 files (B04, C06, C07, C08, E01, E02, E03, E05, E06, I10). |

## Terminology mapping applied

Two different maps were used, deliberately. Section C owns the Qualification Completion decision; Section I must not.

| Scope | Rule | Examples |
|---|---|---|
| C02, C05, C08 | Graduation -> Qualification Completion | `GraduationApproved` -> `QualificationCompletionApproved`; `DegreeConferred` -> `QualificationAwarded` (the L1-canonical event); `StudentGraduated` -> `QualificationAwarded` (consumed, not published); `GraduateStudent()` -> `CompleteQualification()` |
| I13 | Graduation -> Completion Evidence / Academic Completion, **never** Qualification Completion | `Graduation Approval` -> `Completion Evidence Approval`; `Graduation Register` -> `Completion Evidence Register`; `Graduation Eligibility` -> `Completion Eligibility`; `GraduationApproved` -> `CompletionEvidenceApproved` |

The single documented exception is `DegreeConferred` -> `QualificationAwarded`. The Degree Conferment *workflow* keeps its name; the event it publishes becomes the L1-canonical one.

## Verified state after the sweep

| Check | Result |
|---|---|
| Files / unique IDs / Index rows | **107 / 107 / 107** |
| Duplicate IDs | **0** |
| Index IDs with no file (dangling) | **0** |
| Files with no Index row (orphan) | **0** |
| Filename convention violations outside Section J | **0** |
| Matrix blocks / unique / drift vs Index | **107 / 107 / 0** |
| AI_CONTEXT `.md` source references that do not resolve | **0** |
| `graduat*` in C02 / C05 / C08 / I13 | **0 / 0 / 0 / 0** |
| Event chain I13 -> C05 -> C08 -> H06 | **intact** |
| C02 competing publisher of `QualificationAwarded` | **0 occurrences** |

## Residual backlog (pre-existing, unchanged by the regression)

These are the original INT-011/012/013 items, at or better than their pre-regression counts:

- **Titles (3):** L2-A01, L2-F02, and cosmetic L2-A04 (`&` vs `and`).
- **Status (3):** L2-B05 declares `Canonical` in its metadata table while its front-matter correctly says `Architecture Locked`; L2-C05, L2-F01 and L2-H01 metadata-table statuses remain thin. Both non-legend values introduced by the regression (`Architecture Draft`, `Architecture Specification`, `Architecture Locked --- Authoring`) were removed.
- **Document Owner (22):** files carrying only a plain `Owner:` naming the correct architect, without the distinct `Document Owner` field. Where a `Document Owner` field exists, it now matches the Index in **100%** of cases (0 mismatches).

Nothing in this backlog blocks the merge decision; the blocking items remain those in the Outstanding Work Queue below (Section E ownership, the two lifecycle conflicts, and mojibake).

## Process note

This remediation, like the previous one, is **uncommitted**. INT-039 exists because the last round of uncommitted work was destroyed by a file swap. Commit before any further file replacement.

# FINAL PASS — REMAINING FINDINGS CLOSED (2026-08-03)

The ten findings still open after the seventh pass were closed in this pass. Two defects not in the
original 30 were found and fixed along the way; both are recorded here rather than as new IDs,
since each is an instance of an existing finding.

## Findings closed

| Issue | What was done |
|---|---|
| INT-006 | Alumni **Persona** (Student Lifecycle / L2-C08) and Alumni **Profile** (L2-H06) declared two aggregates on one identity, stated normatively in H06 §7.1. H06 header rescoped to Alumni *Engagement*. |
| INT-011 | B05 `Canonical`, B02 duplicate status, H01 non-standard field, F01 missing status — all corrected. 0 mismatches, 0 illegal values. |
| INT-012 | `Document Owner` added to the last 22 files in each file's own header convention. 107/107 match the Index. |
| INT-013 | A01, F02, A04 titles aligned. 0 mismatches. |
| INT-014 | Section G ×8 resolved by renaming §9 to surface existing content; B00, C00, D00, D05, D07, I13 given explicit sections. 0 files missing headings. |
| INT-019 | C06 §21.17 mermaid: A03/A04 split (with the new node wired into the graph), C01→C02 corrected. |
| INT-020 | Section A context and audit brought to the real 11-file corpus with a full A10 entry. |
| INT-021 | Both HTML artifacts regenerated; superseded Graduation headings gone; provenance stamp added; both verify IN SYNC. |
| INT-022 | New `tools/paths.js` resolves from repo root; both tools exit non-zero on missing sources; `convert_all.js` now requires `--write` because it rewrites the locked L1 in place. |
| INT-024 | Remaining 430 mojibake lines in G01/G07/G08 repaired by CP1252→UTF-8 round-trip. Repository-wide count is 0. |

## Two defects found during this pass

**1. Phantom L2-B06 had returned (regression of INT-015).** The file swap reverted `L2-B00`, and
that revert was committed in `84035ba`. B00 again certified a non-existent `L2-B06 Operational
Readiness`, with 44 references, and B01 carried 7 more. All 51 are corrected: Operational Readiness
now points to `L2-A10` (Section A, Platform Governance) per locked L1 §15, and B00 carries a
withdrawal note for the statements that certified a document which never existed. Repository-wide
`B06` count outside the AI-context correction note is **0**.

**2. Fifty broken `file:///d:/MD_files/...` links in L2-B00.** The same file carried absolute links
into a developer's local D: drive. All 50 were rewritten to repository-relative paths. Repository-wide
count of `file:///d:` links is **0**.

## Terminology sweep completed repository-wide

INT-033 was originally fixed only in the four documents involved in the regression. A full sweep
found **121 further occurrences of the retired "Graduation" term across 27 L2 files**, plus residual
occurrences in L0 and L1. All are now converted to Qualification Completion terminology.

Three terms were deliberately protected because they are not the retired lifecycle stage:

| Protected term | Where | Why |
|---|---|---|
| `Graduate assistant` | L2-E00 | A workforce role, not a lifecycle stage |
| `Undergraduate` / `Postgraduate` | L2-F01 | Academic level, not a lifecycle stage |

The five remaining mentions in locked L1 are the sentences that *document* the replacement
("Qualification Completion replaces Graduation") and are correct as written.

## Cumulative amendments to locked documents

| Document | Version | Amendments |
|---|---|---|
| L1 Business Domain Architecture | 2.0 → **2.1** | Institute lifecycle extended (INT-008); Campus ownership split into Institutional vs Physical (INT-001); residual Graduation terminology replaced (INT-021/INT-033) |
| Volume 2 Repository Standards | 1.0 → **1.1** | Status vocabulary formally adopted (INT-023) |
| L0 Architecture Workflow Context Baseline | — | Procurement removed from Finance, assigned to Section J (INT-027); Graduation terminology replaced |

## Process note

This remediation is **uncommitted**. INT-039 exists precisely because the previous round of
uncommitted remediation was destroyed by a file replacement, and the phantom-B06 regression found
in this pass reached `HEAD` because a reverted file was committed without re-verification. Commit
before any further file replacement, and re-run the verification sweep after any bulk file swap.

# RE-ISSUED VERDICT (2026-08-03)

The scorecard, consistency matrix, specific check results and merge decision above are the
**original 2026-07-31 assessment**, deliberately left unmodified as the historical baseline. This
section re-issues the verdict against the current working tree. Where the two disagree, this
section governs.

## Re-run check results

Fourteen of the original checks were re-executed mechanically on 2026-08-03:

| Check | 2026-07-31 | 2026-08-03 | Evidence |
|---|---|---|---|
| No primary workflow appears in more than one L2 file | FAIL | **PASS** | PositionAssignment→E02, Leave→E05, employment lifecycle→E01, physical infrastructure→G10 |
| Every L1 entity uses the same name in L2 | FAIL | **PASS** | `StudentContext` restored in C02; Graduation retired from C02/C05/C08/I13 |
| Workflow names match Matrix / Index | FAIL | **PASS** | 0 title mismatches across 107 files |
| Every L2 workflow has exactly one declared owner | FAIL | **PASS** | 0 missing or mismatched |
| L2 owner matches Architecture Index | FAIL | **PASS** | 107/107 |
| Matrix status matches file header | FAIL | **PASS** | 0 drift, 107 unique blocks |
| Matrix ownership matches Team Assignments | FAIL | **PASS** | `Owner` present on every block |
| All L2 files follow template | FAIL | **PASS** | 0 files missing a required heading |
| Approval states match | FAIL | **PASS** | B01 carries all locked-L1 states |
| Shared entity lifecycles match | FAIL | **PASS** | B01 extends L1 (amended v2.1); C02 maps Student ↔ StudentContext |
| Terminal states match | FAIL | **PASS** | C02 §11.10 lists every L1 terminal state |
| No path/folder reference is broken | FAIL | **PASS** | phantom B06 = 0, `file:///d:` links = 0, unresolved AI-context refs = 0 |
| Every indexed file exists | PASS | **PASS** | 107/107 |
| Every existing L2 file is indexed | PASS | **PASS** | 107/107 |

**Result: 14 PASS, 0 FAIL.**

## Re-issued merge decision

**The 2026-07-31 `REJECT` is superseded.**

Every condition cited in that decision has been resolved:

| Original blocking reason | Status |
|---|---|
| Critical duplicate ownership makes safe implementation impossible | Resolved — all four collisions closed (INT-001, INT-002, INT-003, INT-004) |
| Lifecycle incompatibilities: services would not agree on aggregate roots or transitions | Resolved — INT-008 by L1 amendment, INT-009 by publishing the StudentContext state machine and its mapping |
| Event incompatibilities: services would not agree on contract names | Resolved — INT-007 chain verified end to end, INT-025 event name corrected |
| Broken AI contexts cannot be trusted for automated planning | Resolved — INT-015, INT-016, INT-017, INT-018, INT-020 all closed; 0 unresolved source references |

**Revised decision: the L2 corpus is internally consistent and no longer blocks implementation
planning.**

## What this verdict does and does not mean

This is an **integrity and consistency** review. Read the scope precisely before relying on it.

**It does assert** that, as of 2026-08-03:

- Every architecture ID resolves to exactly one file, and every file is indexed.
- No business capability has two canonical owners among the conflicts this review examined.
- Documented lifecycles do not contradict their locked L1 baselines.
- The qualification-to-alumni event chain has a single publisher per link.
- Control artifacts, generated contexts and presentation artifacts agree with their sources.

**It does not assert** that:

- The architecture is *complete*. This review checks agreement between documents, not whether the
  documented design is correct, sufficient, or well-suited to the business.
- The corpus is free of defects a **fresh** review would find. The 2026-08-03 work verified the 39
  known findings and repaired what they named; it did not re-derive the full 2,406-concept sweep
  from scratch. A new independent review is the way to establish that.
- L3 through L6 exist. Only L0, L1 and L2 are populated.
- ~~Section J filename conventions are not aligned.~~ Resolved 2026-08-03; all 107 filenames now
  follow the convention.

## Standing risk

Two findings in this report (INT-039 and the phantom-B06 regression recorded in the final pass)
exist because remediation was lost or reverted through file replacement, and in one case the
reverted file reached `HEAD` uncommitted-and-unverified. Before any bulk file swap, commit; after
any bulk file swap, re-run the verification sweep. This is the single highest-value process control
this repository currently lacks.


# CONSISTENCY MATRIX

> Historical, as at 2026-07-31. Superseded by **RE-ISSUED VERDICT (2026-08-03)**.

| Concept | Canonical File | Canonical Definition | Conflicting File(s) | Status | Severity |
|---|---|---|---|---|---|
| Physical infrastructure | L0 §20 | Infrastructure owns physical structure/lifecycle | L1, B03, G10, Dependency Map | Contradictory | Critical |
| Position Assignment | E02 (specific partition) | E02 owns complete assignment lifecycle | E01 | Duplicate | Critical |
| Workforce lifecycle | E01 | E01 owns engagement transitions | E04 | Duplicate | High |
| Leave | E05 | E05 owns leave lifecycle and ledger | E04 | Duplicate | High |
| Qualification Completion | L1/C05 | Student Lifecycle owns completion decision | I13 | Contradictory | Critical |
| Alumni | L1 | Student Lifecycle owns Alumni | H06, C05/C08 handoff wording | Contradictory | High |
| Qualification-to-Alumni events | L1 pending normalization | One exact versioned event chain | C02, C08, H06 | Ambiguous | High |
| Institution lifecycle | L1 | Draft/PendingApproval/Active/Returned/Rejected | B01 | Contradictory | High |
| Student lifecycle aggregate | L1 | StudentContext, one active per Institute | C02 Student Aggregate | Ambiguous | High |
| Course Master | L1/B04 | Academic Structure owns Course | D06 | Contradictory | High |
| L2 status | Architecture Index | Exact legend value per ID | 47 L2 files | Contradictory | High |
| L2 human owner | Team Assignments/Index | One assigned architect per file | 92 L2 files | Broken Reference | Medium |
| L2 titles | Architecture Index | One exact title per ID | 19 L2 files | Contradictory | Medium |
| L2 structure | L2 template | Eight standard sections | 39 L2 files | Broken Reference | Medium |
| Section B AI context | Actual Section B corpus | B00-B05 | B context B06 | Broken Reference | Critical |
| AI context inventory | Filesystem | D/I generated; exact basenames | AI Index/README | Contradictory | Medium |
| F/I source traceability | Filesystem | Exact underscore basenames | F/I AI contexts | Broken Reference | High |
| G10 direction | G10 source | A/B/E upstream; G01-G07/D/I downstream | G Dependency Contract | Contradictory | High |
| L2 ID labels | Architecture Index | ID uniquely maps to title/domain | C06/C07 | Broken Reference | High |
| Section A coverage | Section A corpus | A00-A10, 11 files | A context/audit | Contradictory | Medium |
| Graduation terminology | L1 | Qualification Completion replaces Graduation | Generated L1 HTML | Contradictory | High |
| Status vocabulary | Volume 2 / Index | No unified authoritative vocabulary | Both | Ambiguous | Medium |
| Result publication event | L1 | `ResultPublished` | I10 | Contradictory | Medium |
| Procurement | L0 pending amendment | L0 currently includes it in Finance | J00/F14 | Contradictory | High |
| Tenant cardinality | L1 pending lock | Each Institution has one Tenant; upper bound absent | Repo context/A01 | Ambiguous | Medium |

# SPECIFIC CHECK RESULTS

> Historical, as at 2026-07-31. Re-run on 2026-08-03 with 14 PASS / 0 FAIL — see **RE-ISSUED VERDICT**.

| Check | Result | Evidence |
|---|---|---|
| Every L1 entity uses the same name in L2 | FAIL | StudentContext/Student; Course/CourseMaster owner drift |
| No L2 file introduces a canonical term without L1 authority | FAIL | PositionAssignment and Alumni ownership redefined |
| Role names are consistent | FAIL | Human owner fields replaced by boards/teams/domains in 45 files |
| Section names match control artifacts exactly | PASS | A-J section names align |
| Workflow names match Matrix | FAIL | 19 semantic title mismatches |
| Shared entity states are identical | FAIL | Institution and Student lifecycles |
| Every L2 workflow has exactly one declared owner | FAIL | 47 missing; 45 non-assigned |
| L2 owner matches Team Assignments | FAIL | 92 file headers |
| L2 owner matches Architecture Index | FAIL | Same 92 |
| No primary workflow appears in more than one L2 file | FAIL | Position Assignment, Workforce lifecycle, Leave, Completion |
| Finance review responsibilities match Team Assignments | PASS | Review-only Varad/Devang; Chetan remains owner |
| Every cross-section dependency is declared in Dependency Map | FAIL | Map covers foundational edges only; runtime registry absent |
| Every Dependency Map edge is reflected in L2 | PASS | No foundational edge contradiction found |
| No L2 dependency on future L3-L6 | PASS | No normative L3-L6 source dependency found |
| Dependency direction never goes L2 to L1/L0 | PASS | References are constraints, not reverse dependencies |
| No circular foundational dependencies | PASS | Canonical foundational graph is acyclic |
| Shared entity lifecycles match | FAIL | INT-008, INT-009 |
| Terminal states match | FAIL | StudentContext terminal states diverge |
| Approval states match | FAIL | Institution PendingApproval/Returned/Rejected omitted in B01 |
| Suspension/reactivation match | FAIL | B01 adds states absent from L1; C02 introduces Restored |
| Every indexed file exists | PASS | 107/107 |
| Every existing L2 file is indexed | PASS | 107/107 |
| Every Matrix section matches actual IDs | PASS | ID set matches, notwithstanding duplicate I00 heading |
| Matrix status matches file header | FAIL | 47 status drifts; blank Draft controls |
| Matrix ownership matches Team Assignments | FAIL | Matrix has no owner field |
| All L2 files follow template | FAIL | 39 missing one or more required headings |
| Architecture IDs follow canonical format | PASS | All 107 filenames begin with valid L2-[A-J][00-16] IDs |
| No path/folder reference is broken | FAIL | Phantom B06; F/I AI source basenames; conversion paths |
| README paths match structure | PASS | All Markdown links resolve; empty archive/assets/adr folders exist |

# FINAL INTEGRITY REPORT

## Executive Summary

The repository has strong top-level hierarchy and complete L2 ID coverage, but it does not speak with one consistent semantic voice. Critical ownership collisions exist in the most implementation-sensitive areas: the physical world model, workforce assignments, and academic completion. Generated AI contexts and presentation artifacts amplify the risk by inventing a Section B document, reversing dependency direction, citing non-existent source basenames, and presenting superseded Graduation terminology.

- Files read: **175**
- Concepts tracked: **2,406 explicit concept keys**
- Findings: **30**
- Severity: **4 Critical, 14 High, 12 Medium, 0 Low**
- Type totals: Type 1 = 4; Type 2 = 8; Type 3 = 2; Type 4 = 1; Type 5 = 5; Type 6 = 3; Type 7 = 1; Type 8 = 6.

The three most dangerous inconsistencies are:

1. **INT-001**: B03 and G10 both own the same physical Campus/building/space model, while L0/L1/Dependency Map disagree about its location.
2. **INT-002**: E01 and E02 both claim exclusive PositionAssignment ownership.
3. **INT-005**: C05 and I13 both decide academic completion despite L1 explicitly assigning Qualification Completion to Student Lifecycle.

## Consistency Breakdown by Type

| Type | Count | Most Severe Example | Pattern |
|---|---:|---|---|
| Type 1 — Terminology | 4 | Qualification-to-Alumni event chain | Human labels, canonical events, and generated terms are mixed |
| Type 2 — Ownership | 8 | Campus/Infrastructure and PositionAssignment | Broad documents absorb capabilities later partitioned into specific L2 files |
| Type 3 — Lifecycle | 2 | Institution lifecycle | L2 replaces rather than extends locked L1 state machines |
| Type 4 — Dependency | 1 | G10 direction reversal | Generated contract confuses downstream consumers with upstream dependencies |
| Type 5 — Broken Reference | 5 | Phantom L2-B06 | Generated context does not validate sources against filesystem |
| Type 6 — Structural Drift | 3 | 92 owner/header drifts | Template and metadata rules are not mechanically enforced |
| Type 7 — Business Rule | 1 | Tenant cardinality | L1 lacks an explicit upper-bound lock |
| Type 8 — Matrix/Index | 6 | 47 status mismatches | Control artifacts and source headers are updated independently |

## Files With Most Conflicts

| Rank | File | Distinct Finding Families |
|---:|---|---:|
| 1 | `architecture/ARCHITECTURE_INDEX.md` | 6 |
| 2 | `architecture/L2_Domain_Workflow_Architecture/L2_DOMAIN_WORKFLOW_MATRIX.md` | 4 |
| 3 | `architecture/L2_Domain_Workflow_Architecture/section-c/L2-C07_Scholarship_Eligibility_Workflows.md` | 4 |
| 4 | `architecture/L2_Domain_Workflow_Architecture/section-b/L2-B03_Campus_Workflows.md` | 4 |
| 5 | `architecture/L2_Domain_Workflow_Architecture/section-e/L2-E01_Workforce_and_Engagement_Workflows.md` | 3 |

## Merge Decision

**REJECT** — *superseded 2026-08-03; see RE-ISSUED VERDICT*

Critical duplicate ownership makes safe implementation impossible. High-severity lifecycle and event incompatibilities mean independently implemented services would not agree on aggregate roots, transitions, or contract names. Broken AI contexts cannot be trusted for automated planning.

## Priority Remediation List

### 1. Fix before anything else — Critical

1. INT-001 — Lock Campus/Infrastructure ownership and update L0/L1/B03/G10/Dependency Map together.
2. INT-002 — Remove PositionAssignment ownership from E01 or E02; retain one canonical aggregate.
3. INT-005 — Restrict I13 to Examination evidence; preserve C05 completion ownership.
4. INT-015 — Delete phantom B06 from AI context and regenerate Section B.

### 2. Fix before merge — High

INT-003, INT-004, INT-006, INT-007, INT-008, INT-009, INT-010, INT-011, INT-017, INT-018, INT-019, INT-021, INT-024, INT-027.

### 3. Fix in next review cycle — Medium

INT-012, INT-013, INT-014, INT-016, INT-020, INT-022, INT-023, INT-025, INT-026, INT-028, INT-029, INT-030.

---

# OUTSTANDING WORK QUEUE — EMPTY (2026-08-03, final pass)

No findings remain open. All 39 are `CLOSED`.

Two items are carried forward as **standing practice**, not findings:

| # | Item | Note |
|---:|---|---|
| 1 | Commit remediation as it is produced | INT-039 and the phantom-B06 regression both trace to uncommitted or unverified file replacement. |
| 2 | Re-run the verification sweep after any bulk file swap | The 2026-08-03 regression reached `HEAD` because a reverted file was committed without re-checking. |

Known-and-accepted, requiring no action:

- ~~The 11 Section J filenames use ` — ` em-dash separators rather than underscores.~~ **Resolved 2026-08-03.** All 11 renamed to the `L2-JNN_Underscore_Name.md` convention via `git mv` (history preserved, 0/0 content diff on the renames themselves); `&` became `and` and commas were dropped, matching the convention already used across A–I. Every reference was rewritten in `AI_CONTEXT/REPO_UNDERSTANDING.md`, `AI_CONTEXT/SECTION_J/Canonical_AI_Context.md` and the J documents themselves. H1 titles were restored to `# L2-JNN — <Index Title>` and the three that used a hyphen separator (J02, J07, J08) were normalised to an em dash. **Filename convention violations repository-wide: 0.**
- `Graduate assistant` (L2-E00) and `Undergraduate`/`Postgraduate` (L2-F01) retain the `graduat` substring. Both are correct terms, deliberately protected during the terminology sweep.

---

Report generated from the current working tree on 2026-07-31 (Asia/Calcutta). Pre-existing uncommitted F12/F14 renames and Index/Matrix edits were included and not modified.

**Amended 2026-08-03:** Added the REMEDIATION STATUS section, an `Issue Status` + `Verification` line on all 30 findings, and this Outstanding Work Queue. All findings were re-verified against the working tree at commit `9471421`. No original 2026-07-31 finding text, scorecard value, conflict-map row, consistency-matrix row, or check result was altered.
