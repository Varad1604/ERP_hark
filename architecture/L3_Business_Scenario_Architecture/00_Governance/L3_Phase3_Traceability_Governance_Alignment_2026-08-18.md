# SutraERP L3 Phase 3 — Traceability Governance Alignment

**Architecture Level:** L3 — Business Scenario Architecture Governance  
**Date:** 2026-08-18  
**Status:** Approved Phase 3 Alignment  
**Scope:** Candidate-to-final-scenario traceability placement

---

## 1. Conflict Identified

`L3_Naming_Numbering_Standard.md` §13 states that a final L3 document should include the original Phase 1 candidate ID in its metadata.

The approved Phase 3 template design decision `L3_Phase3_Canonical_Template_Design_Decision_2026-08-18.md` D-06 establishes a different repository-maintainability rule: detailed Phase 1 and Phase 2 consolidation lineage is maintained canonically in the Phase 2 scenario register and is not duplicated as mandatory metadata or traceability tables in every final L3 scenario document.

Leaving both instructions active would create contradictory authoring requirements.

---

## 2. Resolution

For canonical L3 scenario documents authored under `L3_Business_Scenario_Template_v2.0.md`:

- the Phase 2 canonical scenario register is the authoritative source for candidate-to-final-scenario consolidation lineage;
- Phase 1 candidate IDs are **not mandatory fields** in final L3 scenario metadata;
- Phase 2 working IDs and disposition history are **not mandatory fields** in final L3 scenario documents;
- merge, variant, duplicate, and split lineage remains preserved in the Phase 2 register;
- no Phase 1 or Phase 2 history is deleted or weakened by this placement rule.

This alignment **supersedes the final sentence of `L3_Naming_Numbering_Standard.md` §13** for documents using template v2.0:

> “The final L3 document should include the original candidate ID in its metadata.”

All other naming, numbering, file-name, candidate-freeze, merge/split traceability, and final-ID rules in `L3_Naming_Numbering_Standard.md` remain unchanged.

---

## 3. Architectural Rationale

The repository already has a dedicated canonical consolidation register containing Phase 1 candidate IDs, Phase 2 working IDs, dispositions, and consolidation relationships.

Repeating this history in every final scenario would create duplicate traceability ownership and increase the risk of divergence between scenario documents and the canonical consolidation register.

The final L3 scenario document therefore focuses on the canonical business scenario, while the Phase 2 register remains the repository traceability authority for how that scenario was derived.

---

## 4. Integrity Statement

This alignment does not:

- renumber or modify any Phase 1 candidate;
- modify any Phase 2 disposition;
- create final L3 IDs;
- alter canonical business ownership;
- remove repository traceability.

It changes only where consolidation lineage is canonically maintained for Phase 3 and later authoring.