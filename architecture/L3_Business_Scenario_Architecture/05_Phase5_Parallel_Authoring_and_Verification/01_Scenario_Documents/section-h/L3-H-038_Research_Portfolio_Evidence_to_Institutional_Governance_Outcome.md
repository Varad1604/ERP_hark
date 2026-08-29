# Research Portfolio Evidence to Institutional Governance Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-038  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Provide institutional governance with a Research-owned portfolio outcome derived from authoritative Research and referenced upstream evidence without turning the Research Portfolio into a source of record.

# 3. Business Outcome

Verified project, funding, ethics, output, IP, and collaboration evidence supports the institutional review without changing source records

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A reporting, ranking, audit, or compliance cycle requires authoritative research evidence.

## 4.2 End Boundary

The scenario ends when a governed Research Portfolio has been compiled, reviewed, approved and published from authoritative Research records and referenced upstream facts, with provenance preserved and no source record altered.

## 4.3 In Scope

Research Portfolio compilation; project, Grant/funding metadata, Ethics, Publication/output, IP, Collaboration and Consultancy evidence; governance review; publication and supersession.

## 4.4 Out of Scope

Re-owning E03 workforce facts, Finance accounting/payment/reconciliation facts, or any other source-domain fact.

# 5. Ownership and Participating Domains

E03 retains authoritative workforce facts. Finance retains authoritative monetary/accounting facts. Research owns the derived Research Portfolio and its governance lifecycle. The Portfolio remains a projection/metadata aggregate, not an alternative owner of its inputs.

# 6. Business Actors, Roles and Responsibilities

Research Office compiles and administers the Portfolio. Research Director is the canonical business authority for strategic portfolio approval/revision decisions where institutional approval is required. Management consumes the published Portfolio for institutional governance.

# 7. Preconditions and Entry Criteria

- Applicable source facts exist in governed lifecycle positions.
- Required Research and referenced upstream evidence is available.
- Portfolio/reporting scope is defined.
- Source ownership is unchanged.

# 8. Trigger

A reporting, ranking, audit, or compliance cycle requires authoritative research evidence

# 9. Main Business Flow

1. Establish the governance evidence scope.
2. Compile the Research Portfolio from canonical Research records and referenced authoritative upstream facts.
3. Include project, Grant/funding, Ethics, Publication/output, IP, Collaboration and Consultancy evidence by reference without duplicating ownership.
4. Move the Portfolio to `Under Review` and validate provenance/completeness.
5. If revision is required, move `Under Review → Revision Required → Compiled` and publish `ResearchPortfolioRevisionRequired`.
6. If approved, move to `Approved` and publish `ResearchPortfolioApproved`.
7. Publish the approved Portfolio, move to `Published`, and publish `ResearchPortfolioPublished`.
8. When a successor approved Portfolio is published, mark the predecessor `Superseded` and publish `ResearchPortfolioSuperseded` while preserving immutable history.

# 10. Alternate Business Flows

- Missing authoritative evidence prevents the affected claim from being treated as verified.
- Revision Required returns the Portfolio to Compiled without changing source records.
- A successor Portfolio supersedes but does not overwrite the prior Published Portfolio.

# 11. Failure, Interruption and Business Recovery

Unavailable or unverifiable source evidence blocks approval of the affected Portfolio content. Correction occurs in the source-owning domain or through portfolio recompilation; Research does not rewrite source facts.

# 12. Business Rules and Decision Constraints

- Research Portfolio is a governed H05 projection/metadata aggregate.
- It cannot become an alternative owner of source facts.
- Canonical lifecycle: `Compiled → Under Review → Approved → Published`, with Revision Required and Superseded branches.
- Finance remains sole authority for monetary/accounting facts; E03 remains workforce owner.

# 13. State and Lifecycle Impact

Canonical portfolio lifecycle:

`Compiled → Under Review → Approved → Published`

Branches:

- `Under Review → Revision Required → Compiled`
- `Published → Superseded`

No source aggregate lifecycle changes.

# 14. Business Events and Cross-Domain Collaboration

Research publishes:

- `ResearchPortfolioReviewStarted`
- `ResearchPortfolioApproved`
- `ResearchPortfolioRevisionRequired`
- `ResearchPortfolioPublished`
- `ResearchPortfolioSuperseded`

Source-domain facts are consumed by contract/reference only and retain their original semantic publishers.

# 15. Business Authorization, Approval and Audit Obligations

Research Director owns the applicable portfolio governance decision. Evidence preserves portfolio scope, source references, provenance, review findings, approval/revision decision, publication, supersession and immutable history.

# 16. Completion Criteria and Postconditions

Completion requires an approved/published Research Portfolio whose required evidence is traceable to authoritative Research or external source facts, with no source ownership transfer and no source record modification.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing portfolio governance paths for ethics, collaboration and referenced funding evidence were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`, which defines the governed Research Portfolio as a projection over canonical Research records and referenced upstream facts; the scenario passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
