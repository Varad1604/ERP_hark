# SutraERP L3 — Final Canonical Scenario ID Register

**Architecture Level:** L3 — Business Scenario Architecture Governance  
**Phase:** Phase 4 — Stage 4 Final ID Assignment  
**Status:** Approved  
**Owner:** Lead Architect  
**Last Updated:** 2026-08-18  
**Canonical Scenario Count:** 1,084  
**Final L3 IDs Assigned:** Yes  
**Depends On:** Stage 3 frozen canonical register; Stage 3 validation; locked L3 Naming and Numbering Standard  

---

## 1. Purpose

This register assigns the stable final L3 identity to every scenario in the approved 1,084-scenario canonical set.

The final identifier format is:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>
```

The authoritative scenario semantics remain those of the frozen Stage 3 canonical register. This Stage 4 register establishes identity and mapping only; it does not redefine scenario title, owner, domains, Type, Priority, business rules, lifecycle, or event ownership.

---

## 2. Final ID Ranges

| Section | Final ID Range | Count |
|---|---|---:|
| A — Platform Foundation | `L3-A-001` … `L3-A-062` | 62 |
| B — Institution & Structure | `L3-B-001` … `L3-B-033` | 33 |
| C — Student Lifecycle | `L3-C-001` … `L3-C-075` | 75 |
| D — Academic Delivery | `L3-D-001` … `L3-D-057` | 57 |
| E — People & Workforce | `L3-E-001` … `L3-E-075` | 75 |
| F — Finance | `L3-F-001` … `L3-F-085` | 85 |
| G — Campus Operations | `L3-G-001` … `L3-G-320` | 320 |
| H — Institutional Services & Intelligence | `L3-H-001` … `L3-H-081` | 81 |
| I — Examination | `L3-I-001` … `L3-I-192` | 192 |
| J — Enterprise Resource Management | `L3-J-001` … `L3-J-104` | 104 |
| **Total** |  | **1,084** |

No gaps or duplicates are permitted inside these ranges.

---

## 3. Authoritative Mapping Rule

Final IDs are assigned sequentially according to the frozen Stage 3 canonical scenario order inside each primary-owner section.

For sections whose Stage 3 cardinality remained one-to-one with the frozen Phase 1 candidate order, the numeric suffix remains positionally aligned. This positional alignment is a consequence of canonical order, not reuse of the candidate ID as the numbering authority.

For sections where merge, variant, or split decisions changed canonical cardinality, the explicit mappings and range offsets below are authoritative.

---

## 4. Sections With Direct Positional Alignment

The following sections retained one canonical output for every frozen candidate in the same approved order:

| Section | Authoritative Mapping |
|---|---|
| A | `CAN-L3-A-001 … CAN-L3-A-062` → `L3-A-001 … L3-A-062` |
| C | `CAN-L3-C-001 … CAN-L3-C-075` → `L3-C-001 … L3-C-075` |
| E | `CAN-L3-E-001 … CAN-L3-E-075` → `L3-E-001 … L3-E-075` |
| F | `CAN-L3-F-001 … CAN-L3-F-085` → `L3-F-001 … L3-F-085` |
| H | `CAN-L3-H-001 … CAN-L3-H-081` → `L3-H-001 … L3-H-081` |

The final ID is still governed by Stage 4; the Phase 1 identifier remains historical lineage only.

---

## 5. Section B — Final Mapping

`CAN-L3-B-021` was split into two approved canonical outcomes. Therefore the final numbering is:

| Frozen Stage 3 Source / Canonical Output | Final L3 ID |
|---|---|
| `CAN-L3-B-001 … CAN-L3-B-020` | `L3-B-001 … L3-B-020` |
| **Institutional Campus Decommissioning to Decommissioned Campus** — source `CAN-L3-B-021` | **`L3-B-021`** |
| **Decommissioned Institutional Campus to Archived Historical Campus** — source `CAN-L3-B-021` | **`L3-B-022`** |
| `CAN-L3-B-022 … CAN-L3-B-032` | `L3-B-023 … L3-B-033` respectively |

The two split children inherit the same Phase 1/Phase 2 parent lineage but have distinct final canonical identities.

---

## 6. Section D — Final Mapping

`CAN-L3-D-027` was absorbed into `CAN-L3-D-028`; `CAN-L3-D-042` was split into two canonical outputs.

| Frozen Stage 3 Source / Canonical Output | Final L3 ID |
|---|---|
| `CAN-L3-D-001 … CAN-L3-D-026` | `L3-D-001 … L3-D-026` |
| **Academic Monitoring Requirement to Published Monitoring Outcome** — sources `CAN-L3-D-027; CAN-L3-D-028` | **`L3-D-027`** |
| `CAN-L3-D-029 … CAN-L3-D-041` | `L3-D-028 … L3-D-040` respectively |
| **Active Learning Opportunity to Governed Suspension** — source `CAN-L3-D-042` | **`L3-D-041`** |
| **Archival-Eligible Learning Opportunity to Archived Historical Opportunity** — source `CAN-L3-D-042` | **`L3-D-042`** |
| `CAN-L3-D-043 … CAN-L3-D-057` | `L3-D-043 … L3-D-057` respectively |

---

## 7. Section G — Final Mapping

`CAN-L3-G-002` is absorbed into G-001 and `CAN-L3-G-164` is absorbed into G-163.

| Frozen Stage 3 Source / Canonical Output | Final L3 ID |
|---|---|
| **Campus Readiness Assessment to Governance Readiness Decision** — sources `CAN-L3-G-001; CAN-L3-G-002` | **`L3-G-001`** |
| `CAN-L3-G-003 … CAN-L3-G-163` | `L3-G-002 … L3-G-162` respectively |
| **Emergency or First-Aid Incident to Immediate Institutional Medical Response** — sources `CAN-L3-G-163; CAN-L3-G-164` | **`L3-G-162`** |
| `CAN-L3-G-165 … CAN-L3-G-322` | `L3-G-163 … L3-G-320` respectively |

### G mapping clarification

The range statement `CAN-L3-G-003 … CAN-L3-G-163 → L3-G-002 … L3-G-162` includes the G-163 survivor. Its canonical row carries both `CAN-L3-G-163` and absorbed `CAN-L3-G-164` lineage. There is only one `L3-G-162` scenario.

Historical absorbed Section G rows retained in the Phase 2 register do not receive final IDs.

---

## 8. Section I — Final Mapping

Five candidates are absorbed into two surviving canonical scenarios:

- `CAN-L3-I-076` and `CAN-L3-I-087` → survivor `CAN-L3-I-072`;
- `CAN-L3-I-081`, `CAN-L3-I-082`, and `CAN-L3-I-083` → survivor `CAN-L3-I-074`.

The final mapping is:

| Frozen Stage 3 Source / Canonical Output | Final L3 ID |
|---|---|
| `CAN-L3-I-001 … CAN-L3-I-071` | `L3-I-001 … L3-I-071` |
| **Examination Evaluation to Final Approved Marks** — sources `CAN-L3-I-072; CAN-L3-I-076; CAN-L3-I-087` | **`L3-I-072`** |
| `CAN-L3-I-073` | `L3-I-073` |
| **Marks Submission Exception to Authoritative Marks Outcome** — sources `CAN-L3-I-074; CAN-L3-I-081; CAN-L3-I-082; CAN-L3-I-083` | **`L3-I-074`** |
| `CAN-L3-I-075` | `L3-I-075` |
| `CAN-L3-I-077 … CAN-L3-I-080` | `L3-I-076 … L3-I-079` respectively |
| `CAN-L3-I-084 … CAN-L3-I-086` | `L3-I-080 … L3-I-082` respectively |
| `CAN-L3-I-088 … CAN-L3-I-197` | `L3-I-083 … L3-I-192` respectively |

Absorbed candidates `I-076`, `I-081`, `I-082`, `I-083`, and `I-087` have no standalone final L3 ID.

---

## 9. Section J — Final Mapping

Six disposal-method variants are absorbed into the J-097 survivor:

`CAN-L3-J-099 … CAN-L3-J-104` → `CAN-L3-J-097` canonical survivor.

| Frozen Stage 3 Source / Canonical Output | Final L3 ID |
|---|---|
| `CAN-L3-J-001 … CAN-L3-J-096` | `L3-J-001 … L3-J-096` |
| **Authorised Disposal Method Execution to Reconciled Physical Disposal Closure** — sources `CAN-L3-J-097; CAN-L3-J-099 … CAN-L3-J-104` | **`L3-J-097`** |
| `CAN-L3-J-098` | `L3-J-098` |
| `CAN-L3-J-105 … CAN-L3-J-110` | `L3-J-099 … L3-J-104` respectively |

Absorbed candidates `J-099` through `J-104` have no standalone final L3 ID.

---

## 10. Completeness and Uniqueness Rules

This register is complete only if all of the following hold:

1. exactly 1,084 canonical outputs resolve to exactly 1,084 final IDs;
2. every final ID belongs to exactly one canonical output;
3. every canonical output has exactly one final ID;
4. section numbering begins at `001` and is continuous through the section maximum;
5. no absorbed MERGE or VARIANT row receives a standalone final ID;
6. each approved SPLIT child receives its own final ID;
7. no Phase 1 candidate ID or Phase 2 P2W ID is redefined or deleted;
8. final IDs do not encode L2 module numbers.

---

## 11. Final Identity Baseline

The final stable L3 identity ranges are now:

```text
L3-A-001 … L3-A-062
L3-B-001 … L3-B-033
L3-C-001 … L3-C-075
L3-D-001 … L3-D-057
L3-E-001 … L3-E-075
L3-F-001 … L3-F-085
L3-G-001 … L3-G-320
L3-H-001 … L3-H-081
L3-I-001 … L3-I-192
L3-J-001 … L3-J-104
```

These identifiers are the canonical identities to be used by subsequent full L3 scenario authoring.

---

## 12. Next Governance Gate

Before broad scenario authoring begins, Stage 4 shall receive a mechanical validation confirming:

- 1,084 final IDs;
- continuous section ranges;
- no duplicate final ID;
- no missing canonical output;
- correct treatment of all approved split/merge/variant transformations;
- no semantic changes introduced by numbering.

After Stage 4 validation passes, the repository may proceed to the controlled pilot authoring batch under the locked L3 v2.0 template.
