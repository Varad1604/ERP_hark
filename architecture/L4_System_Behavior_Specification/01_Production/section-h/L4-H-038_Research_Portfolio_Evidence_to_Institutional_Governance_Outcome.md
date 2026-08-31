# Research Portfolio Evidence to Institutional Governance Outcome

## 1. Specification Metadata
**Specification ID:** L4-H-038  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Research  
**Realizes Scenario:** L3-H-038  
**Primary Business Owner:** Research  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-038; L2-H05 — Research Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall compile and govern a Research Portfolio projection from authoritative Research and referenced upstream evidence without becoming an alternative owner of its source facts.

## 3. Required Business Context
Defined portfolio/reporting scope; governed Research facts; referenced E03/Finance/upstream evidence; Research Director authority; provenance metadata.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-038-BH-001 | Research shall compile the Portfolio only from canonical Research records and authoritative referenced upstream facts, preserving source references and ownership. | L3-H-038 §§7–9,12 |
| L4-H-038-BH-002 | Portfolio lifecycle shall follow `Compiled → Under Review → Approved → Published`, with `Under Review → Revision Required → Compiled` and `Published → Superseded` branches. | L3-H-038 §§9–13 |
| L4-H-038-BH-003 | Missing/unverifiable source evidence shall prevent the affected claim from being treated as verified; correction shall occur at the source or through portfolio recompilation, not source rewriting by Research. | L3-H-038 §§10–12 |
| L4-H-038-BH-004 | A successor Published Portfolio may supersede the predecessor for current use but shall not overwrite prior Published evidence. | L3-H-038 §§9,10,12–13 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-038-BH-005 | Research shall publish `ResearchPortfolioReviewStarted`, `ResearchPortfolioApproved`, `ResearchPortfolioRevisionRequired`, `ResearchPortfolioPublished` and `ResearchPortfolioSuperseded`; source-domain events retain their original publishers. | L3-H-038 §14 |

## 6. Failure / Rejection / Continuation
Unavailable or unverifiable source evidence blocks Portfolio approval for the affected content while preserving source facts unchanged.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-038-BH-006 | The system shall preserve portfolio scope, source references/provenance, review findings, approval/revision decision, publication, supersession and immutable Portfolio history. | L3-H-038 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-038-BH-007 | Completion shall require an approved/published Portfolio whose required evidence is traceable to authoritative source facts, with no source ownership transfer or source record modification. | L3-H-038 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-038 — Research Portfolio Evidence to Institutional Governance Outcome  
**Primary verification source:** Locked H05 governance/lifecycle amendment.

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation.