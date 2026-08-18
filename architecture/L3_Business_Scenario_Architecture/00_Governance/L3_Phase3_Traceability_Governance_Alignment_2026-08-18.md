# SutraERP L3 Phase 3 — Traceability Governance Alignment

**Architecture Level:** L3 — Business Scenario Architecture Governance  
**Version:** 1.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Date:** 2026-08-18  
**Scope:** Candidate-to-final-scenario traceability placement  
**Referenced By:** `L3_Naming_Numbering_Standard.md`; `L3_Business_Scenario_Template_v2.0.md`

---

## 1. Conflict Identified

The pilot-era naming guidance expected the original Phase 1 candidate ID to be repeated in each final L3 scenario document.

Phase 3 established a repository-maintainability rule: detailed Phase 1 and Phase 2 consolidation lineage is already owned by the Phase 2 canonical scenario register and must not be duplicated as mandatory metadata or traceability tables in every final L3 scenario.

Leaving both instructions active would create contradictory authoring requirements and duplicate traceability ownership.

---

## 2. Locked Resolution

For canonical L3 scenario documents authored under `L3_Business_Scenario_Template_v2.0.md`:

- the Phase 2 canonical scenario register is authoritative for candidate-to-final consolidation lineage;
- Phase 1 candidate IDs are not mandatory scenario metadata;
- Phase 2 working IDs and dispositions are not mandatory scenario metadata;
- merge, variant, duplicate, and split lineage remains preserved in the Phase 2 register;
- no Phase 1 or Phase 2 history is deleted or weakened;
- final scenarios retain higher-level architectural traceability through their `Depends On` metadata and selective authoritative source references.

`L3_Naming_Numbering_Standard.md` v1.1 incorporates this rule directly and is now the governing naming/numbering standard.

---

## 3. Split Scenario Rule

Where one Phase 1 candidate results in multiple final canonical scenarios:

- no new Phase 1 candidate IDs are manufactured;
- each final child scenario receives its own final L3 ID during canonical scenario-set construction;
- the Phase 2 register remains the authoritative lineage source connecting those children to the resolved split decision.

---

## 4. Integrity Statement

This alignment does not:

- renumber or modify any frozen Phase 1 candidate;
- change any Phase 2 disposition;
- create final L3 IDs;
- alter canonical ownership, lifecycles, rules, events, or dependencies;
- remove repository traceability.

It defines only the canonical placement of consolidation lineage for Phase 3 and later authoring.