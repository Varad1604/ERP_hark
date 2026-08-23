# Research Output to Governed Publication and Archival

# 1. Scenario Metadata

**Scenario ID:** L3-H-034  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows

# 2. Scenario Purpose

Govern institutional Research Publication metadata from verified output registration through H05-defined verification, publication, and archival while preserving workforce ownership and external scholarly authority for external metadata.

# 3. Business Outcome

The output is reviewed, published or rejected, corrected where permitted, and retained under repository policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A verified research output is submitted for institutional publication

## 4.2 End Boundary

H05 supports `Draft → Verified → Published → Archived`. The frozen outcome also requires reviewed/rejected and correction paths, which are not canonically defined.

## 4.3 In Scope

Register publication metadata, verify authenticity/affiliation/ownership, publish verified output, archive according to policy, and preserve history.

## 4.4 Out of Scope

Invented Publication rejection, correction, amendment, successor-version, or rejection-authority semantics; ownership of publication documents or E03 workforce facts.

# 5. Ownership and Participating Domains

E03 owns workforce identity and capability facts. Research owns Publication metadata, its H05 lifecycle, verification history, publication state, and archival state. Publication documents remain outside Research as defined by H05.

# 6. Business Actors, Roles and Responsibilities

Faculty Member / Principal Investigator / Research Administrator may register. Research Office verifies, publishes, and archives according to H05.

# 7. Preconditions and Entry Criteria

A verified research output is available; institutional author references are valid; publication type is configured; duplicate/DOI checks can be completed; required metadata exists.

# 8. Trigger

A verified research output is submitted for institutional publication

# 9. Main Business Flow

1. Register publication metadata and enter `Draft`.
2. Validate authors, DOI where applicable, publication type, duplicate rules, and referenced project.
3. Verify authenticity and institutional ownership; enter `Verified`.
4. Publish the verified output; enter `Published`.
5. Preserve publication authority/date and historical metadata.
6. Archive an eligible published record under repository/publication policy; enter `Archived`.
7. Stop before unsupported rejection or correction branches.

# 10. Alternate Business Flows

Manual verification may be used when external verification is unavailable. Duplicate registration is rejected before lifecycle creation/progression. Eligible published records may be archived.

# 11. Failure, Interruption and Business Recovery

Failed verification prevents publication. Missing required metadata blocks progression. Frozen rejection/correction branches require upstream architecture resolution rather than local states.

# 12. Business Rules and Decision Constraints

At least one institutional author is required; DOI is unique where applicable; duplicates are prohibited; published date/history is traceable; documents remain outside Research; archived publications are read-only and available for reporting.

# 13. State and Lifecycle Impact

Publication: `Draft → Verified → Published → Archived`. H05 defines no `Rejected` Publication state and no correction/version-successor lifecycle.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research publishes `PublicationRegistered`, `PublicationVerified`, `ResearchOutputPublished`, and `PublicationArchived`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

E03 workforce identity remains source-owned. External scholarly sources may remain authoritative for externally owned citation/verification metadata as H05 specifies.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Only the H05 roles above are assigned. No rejection/correction authority is created.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 override may bypass verification before publication or invent unsupported lifecycle branches.

## 15.3 Business Audit and Evidence Obligations

Preserve registration metadata, author references, verification evidence, publishing authority/date, archival reason/history, and Publication events.

# 16. Completion Criteria and Postconditions

The H05-supported path completes through `Published` and, when policy requires, `Archived`. The full frozen outcome cannot complete because rejection and correction are missing.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- No canonical `Rejected` Publication state, transition, decision authority, or event.
- No correction/amendment/version-successor lifecycle for a published Research output.
- Frozen “reviewed” is broader than H05 `VerifyPublication`; L3 cannot silently equate them.

These gaps materially affect the frozen outcome.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
