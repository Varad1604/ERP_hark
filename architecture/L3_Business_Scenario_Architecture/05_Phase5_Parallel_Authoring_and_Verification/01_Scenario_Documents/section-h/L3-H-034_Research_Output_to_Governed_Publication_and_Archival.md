# Research Output to Governed Publication and Archival

# 1. Scenario Metadata

**Scenario ID:** L3-H-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern Research Publication metadata from submission through review, approval/rejection, publication, successor correction and archival while preserving workforce ownership and external scholarly authority for externally owned metadata.

# 3. Business Outcome

The output is reviewed, published or rejected, corrected where permitted, and retained under repository policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A verified research output is submitted for institutional publication.

## 4.2 End Boundary

The scenario ends when the Research Publication reaches the applicable governed outcome: Published, Rejected, Archived, or a successor version supersedes a Published predecessor after material correction.

## 4.3 In Scope

Publication submission/review, revision-required branch, approval/rejection, publication, successor correction, supersession and archival.

## 4.4 Out of Scope

Ownership of publication documents, E03 workforce facts, or external legal/scholarly authority.

# 5. Ownership and Participating Domains

E03 owns workforce identity/capability facts. Research owns Publication metadata, its H05 lifecycle/events and institutional publication history. Authoritative content/document ownership remains with its applicable source/document owner.

# 6. Business Actors, Roles and Responsibilities

Faculty Member / Principal Investigator / Research Administrator may register or submit publication metadata according to H05. Existing H05 review actors retain their responsibilities. Research Director is the canonical business authority for strategic approval/rejection/archival decisions where institutional approval is required.

# 7. Preconditions and Entry Criteria

- A valid research output exists.
- Institutional author references are valid.
- Required metadata and project references are available.
- Duplicate/DOI and source-verification checks can be completed.

# 8. Trigger

A verified research output is submitted for institutional publication

# 9. Main Business Flow

1. Register publication metadata in `Draft`.
2. Submit it and move to `Submitted`.
3. Move to `Under Review` and evaluate authenticity, ownership, institutional affiliation and publication requirements.
4. If revision is required, move `Under Review → Revision Required → Submitted` and publish `ResearchPublicationRevisionRequired`.
5. If rejected, move `Under Review → Rejected` and publish `ResearchPublicationRejected`.
6. If approved, move `Under Review → Approved`.
7. Publish the approved record, move to `Published`, and publish `ResearchPublicationPublished`.
8. If a material correction is required after publication, create a successor Publication version; the Published predecessor remains immutable and may become `Superseded` for current-use purposes, publishing `ResearchPublicationSuperseded`.
9. Archive an eligible Publication and publish `ResearchPublicationArchived` while preserving history.

# 10. Alternate Business Flows

- Rejected Publication may be revised only through a successor Draft/submission path where governance permits.
- Material post-publication correction creates a successor version; the predecessor is not overwritten.
- Archival preserves historical availability according to repository policy.

# 11. Failure, Interruption and Business Recovery

Failed review prevents approval/publication. Missing required metadata or duplicate conflicts stop progression. Correction never mutates a Published predecessor; a successor version preserves lineage.

# 12. Business Rules and Decision Constraints

- Canonical lifecycle: `Draft → Submitted → Under Review → Approved → Published → Archived`.
- `Under Review → Revision Required → Submitted` and `Under Review → Rejected` are governed branches.
- Published predecessors remain immutable after material correction.
- Research Publication metadata does not transfer ownership of source documents or E03 workforce facts.

# 13. State and Lifecycle Impact

The current Publication version follows the canonical lifecycle. A material correction after publication establishes a successor version; the predecessor may become `Superseded` for current-use purposes while historical evidence remains immutable.

# 14. Business Events and Cross-Domain Collaboration

Research publishes:

- `ResearchPublicationRevisionRequired`
- `ResearchPublicationRejected`
- `ResearchPublicationPublished`
- `ResearchPublicationSuperseded`
- `ResearchPublicationArchived`

E03 and external source facts are referenced only; their ownership is unchanged.

# 15. Business Authorization, Approval and Audit Obligations

Use only H05-defined review and approval responsibilities. Audit evidence preserves submission/review, revision/rejection/approval decisions, publication, successor lineage, supersession and archival history.

# 16. Completion Criteria and Postconditions

Completion occurs when the Publication reaches its governed final/current outcome and all review/publication/correction/archival evidence is preserved. Source documents and workforce facts remain externally owned.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing rejection, revision/correction, successor-version and publication-governance semantics were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
