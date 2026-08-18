# SutraERP L3 Naming and Numbering Standard

**Document:** L3 Naming and Numbering Standard  
**Architecture Level:** L3 — Business Scenario Architecture Governance  
**Version:** 1.1  
**Status:** Locked  
**Owner:** Lead Architect  
**Last Updated:** 2026-08-18  
**Depends On:** Repository Constitution; locked L1/L2 architecture; Phase 1 frozen baseline; Phase 2 final consolidation register  
**Referenced By:** Canonical L3 scenario template and future canonical L3 scenario documents

---

## 1. Purpose

This document defines naming and numbering rules for SutraERP Level 3 Business Scenario Architecture.

The rules keep L3 identifiers stable, ownership-aligned, independent of L2 numbering, and traceable through the repository without duplicating consolidation history inside every final scenario document.

---

## 2. Core Rule

L3 numbering is independent of L2 numbering.

An L3 scenario may derive from one or many L2 workflow documents. Therefore L3 scenario numbers must not copy L2 workflow numbers.

```text
Correct:   L3-C-001_Approved_Candidate_to_Active_Student.md
Incorrect: L3-C01-001_Approved_Candidate_to_Active_Student.md
```

`C01` is an L2 document identifier and must not be embedded in a final L3 scenario ID.

---

## 3. Identifier Types

| Identifier Type | Format | Purpose |
|---|---|---|
| Phase 1 candidate ID | `CAN-L3-<AREA>-<NNN>` | Frozen discovery candidate identifier |
| Phase 2 working ID | `P2W-<SECTION>-<NNNN>` | Consolidation-register traceability only; never a final L3 ID |
| Final L3 scenario ID | `L3-<PRIMARY_OWNER_SECTION>-<NNN>` | Canonical L3 scenario identifier |
| L3 issue ID | `L3-ISS-<NNN>` | L3 architecture issue identifier where used |
| L3 architecture decision ID | `L3-ADR-<NNN>` | L3 architecture decision identifier where the ADR structure is used |

---

## 4. Phase 1 Candidate ID Format

Phase 1 candidates use:

```text
CAN-L3-<AREA>-<NNN>
```

Example:

```text
CAN-L3-C-001
```

Candidate IDs are frozen historical discovery identifiers. They do not represent final scenario ownership or final L3 numbering and do not guarantee a one-to-one final scenario relationship.

---

## 5. Discovery Area Codes

| Code | Discovery Area |
|---|---|
| A | Platform Foundation |
| B | Institution & Structure |
| C | Student Lifecycle |
| D | Academic Delivery |
| E | People & Workforce |
| F | Finance |
| G | Campus Operations |
| H | Institutional Services & Intelligence |
| I | Examination |
| J | Enterprise Resource Management |
| X | Cross-domain or uncertain primary area during discovery only |

No final L3 scenario may remain under `X`.

`X` is a discovery identifier-area code, not a wildcard for participating domains.

---

## 6. Candidate Numbering Rules

Candidate numbering starts at `001` in each discovery area and has a minimum width of three digits with no maximum width.

The one-time repository-wide candidate normalization performed on 2026-08-17 is recorded in `L3_Phase1_Candidate_ID_Migration_2026-08-17.md`.

After the final Section C addition `CAN-L3-C-075`, the complete Phase 1 candidate baseline is frozen.

Phase 2 or later work must never:

- delete a frozen Phase 1 candidate because of a merge, split, duplicate, variant, rejection, deferral, or other disposition;
- renumber another candidate because of a later disposition;
- reuse a frozen candidate ID;
- repeat the 2026-08-17 normalization.

---

## 7. Final L3 Scenario ID Format

Final canonical scenarios use:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>
```

Example:

```text
L3-C-001
```

The owner section is the section that owns the final business outcome under locked L1/L2 architecture.

The discoverer, first participant, most visible participant, or domain with the most flow steps does not automatically become the final owner.

---

## 8. Final L3 File Name Format

Final canonical scenario files use:

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md
```

Examples:

```text
L3-C-001_Approved_Candidate_to_Active_Student.md
L3-F-001_Student_Fee_Payment_and_Receipt.md
L3-I-001_Regular_Examination_to_Result_Publication.md
L3-J-001_Purchase_Requisition_to_Goods_Receipt_and_Inventory.md
```

---

## 9. Scenario Slug Rules

A scenario slug must:

- use clear business terminology;
- use title-style words separated by underscores;
- describe the business outcome;
- avoid implementation terminology;
- avoid abbreviations unless already canonical.

Incorrect examples include implementation- or step-oriented names such as `Student_API`, `Payment_DB_Update`, `Generate_Receipt`, or `Update_Status`.

---

## 10. Scenario Title Rules

Scenario titles must be outcome-based and represent complete institutional business outcomes.

Preferred patterns include:

```text
<Starting Business State> to <Final Business Outcome>
```

or a clear business-outcome noun phrase.

Small workflow actions such as `Validate Payment`, `Check Eligibility`, `Send Notification`, or `Update Student Status` are not sufficient L3 scenario titles unless the complete scenario boundary and outcome justify them as a full business scenario.

---

## 11. Final Owner Rule

Final owner selection must preserve canonical L1/L2 ownership.

Examples of the governing logic:

- Active Student Context outcome → Student Lifecycle;
- financial settlement and receipt outcome → Finance;
- official examination result publication → Examination;
- procurement/inventory/resource lifecycle outcome → Enterprise Resource Management.

Participation does not transfer ownership.

---

## 12. Phase 2 Working IDs

`P2W-<SECTION>-<NNNN>` identifiers belong only to the Phase 2 consolidation register.

They must not:

- become final L3 IDs;
- appear as the numbering basis of final scenario files;
- be renumbered to imitate final L3 numbering.

---

## 13. Candidate-to-Final Traceability

The Phase 2 canonical scenario register is the authoritative source for candidate-to-final consolidation lineage, including Phase 1 candidate IDs, Phase 2 working IDs, dispositions, merge/variant/duplicate relationships, and resolved split decisions.

Final canonical L3 scenario documents are **not required to repeat Phase 1 candidate IDs or Phase 2 working IDs in their metadata**.

This rule changes only the placement of consolidation traceability. It does not remove repository traceability or alter any frozen Phase 1/Phase 2 history.

Final scenario documents must still trace to their authoritative higher-level business architecture through the metadata and source references required by the locked canonical L3 scenario template.

---

## 14. Merge and Split Traceability

Merge and split history remains in the Phase 2 consolidation register.

Conceptual examples:

```text
CAN-L3-F-001 + CAN-L3-F-002 -> one canonical L3 scenario
CAN-L3-X-001 -> two canonical L3 scenarios
```

A split does not create new Phase 1 candidate IDs. Final child scenarios inherit the existing consolidation lineage recorded in Phase 2.

---

## 15. Final Number Assignment

Final L3 IDs must be assigned only during authorized canonical scenario-set construction after Phase 2 closure and under the locked canonical L3 template.

Assignment must occur against the constructed canonical scenario set, not mechanically row-for-row from the 1,124-row Phase 2 traceability register.

The Phase 2 register contains traceability rows, not a guaranteed one-row-to-one-final-scenario numbering list.

---

## 16. Versioning

L3 governance documents use semantic versioning.

- major: breaking structural or governance change;
- minor: additive or non-breaking governance change;
- patch: clarification, correction, or formatting improvement.

The move from pilot scenario template v1.0 to canonical template v2.0 is a major structural change.

---

## 17. Prohibited Patterns

Do not use:

```text
L3-C01-001
L3-F02-001
L3-I10-001
CAN-L3-C-001_Approved_Candidate_to_Active_Student.md
L3-C-001_Student_API.md
L3-F-001_Payment_DB_Update.md
```

Do not assign a final L3 ID solely by copying a Phase 1 candidate number or Phase 2 working number.

---

## 18. Lock Statement

The following rules are locked:

```text
Phase 1 candidate IDs remain CAN-L3-<AREA>-<NNN> and are frozen.
Phase 2 P2W identifiers are traceability-only and never final L3 IDs.
Final L3 scenario IDs use L3-<PRIMARY_OWNER_SECTION>-<NNN>.
Final L3 files use L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md.
L3 numbering is independent of L2 numbering.
Final owner follows the canonical owner of the final business outcome.
Phase 1/Phase 2 consolidation lineage remains authoritative in the Phase 2 register and is not required to be duplicated in every final L3 scenario document.
Final IDs are assigned only from the constructed canonical scenario set under the locked L3 template.
```
