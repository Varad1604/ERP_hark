# SutraERP L3 Phase 5 — Scenario File Structure Instructions

**Architecture Level:** L3 — Business Scenario Architecture  
**Phase:** Phase 5 — Parallel Authoring and Continuous Verification  
**Status:** Approved for Controlled Pilot Validation — Permanent Placement Not Yet Locked  
**Owner:** Lead Architect  
**Date:** 2026-08-19  
**Canonical Scenario Population:** 1,084  
**Depends On:** Phase 4 frozen canonical scenario set; Phase 4 final ID register; locked L3 Naming and Numbering Standard; Phase 5 operating guide

---

## 1. Purpose

This document defines the **single proposed repository placement convention** to be used by the controlled Phase 5 pilot and, if the pilot passes, by broad L3 scenario authoring.

It does not redefine:

- canonical scenario semantics;
- final L3 IDs;
- scenario titles;
- canonical ownership;
- participating domains;
- Type or Priority;
- business rules, lifecycle states, events, contracts or dependencies;
- human author allocation.

The existing Phase 5 operating guide remains the authority for exact human assignment and exact filenames.

---

## 2. Authority Separation

The following authorities must remain separate:

| Concern | Authority |
|---|---|
| Canonical scenario meaning | Phase 4 Stage 3 frozen canonical register |
| Final scenario identity | Phase 4 Stage 4 Final Canonical Scenario ID Register |
| Exact final filename | Phase 5 operating guide — Section 8, `Exact Phase 5 Scenario File Table` |
| Human author assignment | Phase 5 operating guide |
| File naming rule | `../00_Governance/L3_Naming_Numbering_Standard.md` |
| Scenario document structure | `../00_Governance/L3_Business_Scenario_Template_v2.0.md` |
| Proposed scenario-document placement | **This document** |

This document must never become a second scenario register or a second filename register.

---

## 3. Pilot Gate

The Phase 5 operating baseline currently requires the controlled pilot to validate permanent scenario-document placement before broad authoring begins.

Therefore this structure is approved **for pilot validation first**.

The pilot shall create its selected scenario documents directly in the proposed target structure below. This avoids a temporary pilot-only layout and minimizes later file movement.

If the pilot confirms the structure, the Lead Architect may promote this placement convention to the permanent Phase 5 authoring structure without moving the pilot scenario files.

If the pilot finds a placement defect, correct this placement standard before broad authoring. Do not change final IDs, canonical titles or exact filenames merely to solve a folder-layout problem.

---

## 4. Proposed Phase 5 Repository Structure

```text
architecture/L3_Business_Scenario_Architecture/
└── 05_Phase5_Parallel_Authoring_and_Verification/
    ├── README.md
    ├── L3_Phase5_Authoring_Workload_and_Execution_Report_2026-08-18.md
    ├── L3_Phase5_Scenario_File_Structure_Instructions.md
    └── 01_Scenario_Documents/
        ├── section-a/
        ├── section-b/
        ├── section-c/
        ├── section-d/
        ├── section-e/
        ├── section-f/
        ├── section-g/
        ├── section-h/
        ├── section-i/
        └── section-j/
```

`01_Scenario_Documents/` and its section folders do not need to exist empty. They shall be materialized as pilot or approved authoring files are added.

---

## 5. Section Folder Table

| Canonical Section | Required Folder | Final ID Range | Expected Final Count |
|---|---|---|---:|
| A — Platform Foundation | `01_Scenario_Documents/section-a/` | `L3-A-001` … `L3-A-062` | 62 |
| B — Institution & Structure | `01_Scenario_Documents/section-b/` | `L3-B-001` … `L3-B-033` | 33 |
| C — Student Lifecycle | `01_Scenario_Documents/section-c/` | `L3-C-001` … `L3-C-075` | 75 |
| D — Academic Delivery | `01_Scenario_Documents/section-d/` | `L3-D-001` … `L3-D-057` | 57 |
| E — People & Workforce | `01_Scenario_Documents/section-e/` | `L3-E-001` … `L3-E-075` | 75 |
| F — Finance | `01_Scenario_Documents/section-f/` | `L3-F-001` … `L3-F-085` | 85 |
| G — Campus Operations | `01_Scenario_Documents/section-g/` | `L3-G-001` … `L3-G-320` | 320 |
| H — Institutional Services & Intelligence | `01_Scenario_Documents/section-h/` | `L3-H-001` … `L3-H-081` | 81 |
| I — Examination | `01_Scenario_Documents/section-i/` | `L3-I-001` … `L3-I-192` | 192 |
| J — Enterprise Resource Management | `01_Scenario_Documents/section-j/` | `L3-J-001` … `L3-J-104` | 104 |
| **Total** |  |  | **1,084** |

The folder is determined by the canonical section encoded in the final L3 ID, not by the assigned human author or by the most visible participating domain.

---

## 6. Exact Scenario File Path Rule

The exact file path is:

```text
architecture/L3_Business_Scenario_Architecture/
05_Phase5_Parallel_Authoring_and_Verification/
01_Scenario_Documents/
section-<lowercase-primary-owner-section>/
<EXACT_FILENAME_FROM_PHASE5_SECTION_8>
```

Example:

```text
architecture/L3_Business_Scenario_Architecture/
05_Phase5_Parallel_Authoring_and_Verification/
01_Scenario_Documents/
section-c/
L3-C-001_Approved_Candidate_to_Active_Student.md
```

For every scenario, authors must copy the filename **exactly** from Section 8 of:

`L3_Phase5_Authoring_Workload_and_Execution_Report_2026-08-18.md`

Authors must not regenerate, shorten, improve, normalize differently or manually rename an already-defined Phase 5 filename.

---

## 7. Placement Rules

The following rules are mandatory during the pilot and, if approved, broad Phase 5 authoring:

1. **One final L3 ID = one scenario file.**
2. The scenario file belongs under the folder matching the section in its final L3 ID.
3. Cross-domain participation does not change the folder.
4. Assigned author does not change the folder.
5. Human author names must not be used as scenario-storage folders.
6. L2 document IDs must not be used as scenario-storage subfolders.
7. Type (`Simple`, `Cross-domain`, `Critical`) must not determine folder placement.
8. Priority (`P0`, `P1`, `P2`) must not determine folder placement.
9. Draft/review/approval status must not determine folder placement.
10. Phase 1 candidate IDs and Phase 2 working IDs must not appear in scenario filenames or folder names.
11. A scenario must not be copied into multiple section folders for convenience.
12. A reviewer or second author edits/reviews the same canonical file; they do not create a reviewer-specific copy inside `01_Scenario_Documents/`.

---

## 8. Status Handling

Scenario status belongs inside the scenario document metadata.

Allowed progression remains:

```text
Draft
  ↓
Review
  ↓
Architecture Approved
  ↓
Locked
```

Do not create folders such as:

```text
Draft/
Review/
Approved/
Locked/
```

and do not move scenario files when their status changes.

A stable file path is required for repository traceability.

---

## 9. Cross-Domain Scenario Placement

A cross-domain scenario is stored only under its **primary-owner section**.

Example:

```text
L3-C-001
Primary owner section: C
Participating sections: A, B, C, F
```

Required location:

```text
01_Scenario_Documents/section-c/
L3-C-001_Approved_Candidate_to_Active_Student.md
```

It must not be duplicated under Sections A, B or F.

This preserves the repository rule that participation does not transfer canonical ownership.

---

## 10. Split, Merge and Variant Outcomes

Phase 4 has already resolved split, merge and variant transformations.

Phase 5 placement uses only the final L3 identity.

Therefore:

- each approved split child is stored as its own final L3 file under its final owner section;
- absorbed merge/variant candidates do not receive standalone files;
- historical Phase 1/Phase 2 lineage remains in the consolidation/freeze artifacts and does not create extra Phase 5 files.

No author may create extra files to preserve historical candidate rows.

---

## 11. What Belongs in `01_Scenario_Documents/`

Only canonical L3 scenario documents using the locked v2.0 template belong in this folder tree.

Do not place the following beside canonical scenario files:

- review reports;
- AI prompts;
- scratch notes;
- temporary exports;
- duplicate drafts;
- source extracts;
- personal checklists;
- generated comparison files;
- meeting notes.

Any future review-evidence folder must be separately governed and must not alter the canonical scenario path.

---

## 12. Authoring Procedure for File Creation

Before creating a scenario file:

1. locate the final ID in the Stage 4 final ID register;
2. locate the scenario in the frozen Stage 3 canonical register;
3. locate the exact author and exact filename in Phase 5 operating guide Section 8;
4. confirm the target section folder from the final L3 ID;
5. read the applicable authoritative L2 source documents;
6. create exactly one file at the required path;
7. use the locked v2.0 template;
8. set initial scenario status to `Draft`;
9. perform the Phase 5 self-check before moving the document to `Review`.

Do not create the file if the final ID, canonical title, exact filename and section folder do not reconcile.

---

## 13. Pilot Validation Checklist

The pilot must verify the placement convention mechanically and architecturally.

For every pilot file confirm:

- final ID exists in the Stage 4 register;
- exact filename matches Phase 5 Section 8;
- parent folder matches the final ID section;
- only one canonical copy exists;
- scenario status is metadata, not folder placement;
- assigned author has not affected canonical placement;
- cross-domain participation has not created duplicate copies;
- the file uses the locked v2.0 template;
- source references remain resolvable after placement;
- review can occur without moving or duplicating the scenario file.

The placement convention passes only if the pilot demonstrates stable, section-oriented storage without weakening canonical ownership or traceability.

---

## 14. Broad-Authoring Gate

This document does **not** independently authorize broad authoring.

Broad authoring remains gated by the controlled pilot defined in the Phase 5 operating guide.

After the pilot passes, the Lead Architect should record the placement approval and update this document status from:

```text
Approved for Controlled Pilot Validation — Permanent Placement Not Yet Locked
```

to the approved permanent status.

Only then should the 1,084-file section tree be treated as the permanent Phase 5 scenario-document placement convention.

---

## 15. Repository Integrity Rule

File organization is an operational repository concern. It must never be used to infer or change business architecture.

```text
Canonical Owner ≠ Assigned Author ≠ Folder Maintainer ≠ Reviewer
```

The final L3 ID and locked L1/L2 ownership remain authoritative regardless of who creates, reviews or maintains the physical file.
