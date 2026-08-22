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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows  
**Referenced By:** Related Research governance, audit, reporting, and downstream institutional use

# 2. Scenario Purpose

H05 supports publication registration, verification, publication, and archival while external scholarly sources remain authoritative for citation metadata.

# 3. Business Outcome

The output is reviewed, published or rejected, corrected where permitted, and retained under repository policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A verified research output is submitted for institutional publication.

## 4.2 End Boundary

The frozen end outcome is reached only when all required branches in Section 3 are canonically supported. Locked H05 supports only the partial path stated in Section 2; material gaps are recorded in Section 17.

## 4.3 In Scope

- The H05-supported lifecycle path materially needed for this scenario.
- Source-domain references from E and, where applicable, Finance without ownership transfer.
- Exact H05 events, rules, authority, and evidence obligations that are defined upstream.

## 4.4 Out of Scope

- Any missing lifecycle, decision, actor authority, event, or Finance contract identified in Section 17.
- Changes to Faculty Workforce or Finance-owned records by Research.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| People & Workforce / Section E | Retains canonical Faculty Workforce identity, capability, availability, capacity, and eligibility used by Research. |
| Research / Section H | Owns the Research aggregate/lifecycle and exact H05 business events for the supported path. |

# 6. Business Actors, Roles and Responsibilities

Source-supported actors: Faculty Member / Principal Investigator / Research Administrator; Research Office.

# 7. Preconditions and Entry Criteria

- The applicable Research aggregate and authoritative source references exist as required by H05.
- Required institutional policy/configuration and evidence are available.
- Source-domain ownership remains unchanged.

# 8. Trigger

A verified research output is submitted for institutional publication

# 9. Main Business Flow

Register publication metadata; verify authenticity/affiliation/ownership and duplicate/DOI rules; publish the verified output; archive when policy permits; preserve history and stop before unsupported reject/correct branches.

# 10. Alternate Business Flows

Manual verification is permitted when external verification is unavailable; duplicate registration is refused; eligible published records may be archived.

# 11. Failure, Interruption and Business Recovery

Failed verification prevents publication; missing metadata blocks progression; frozen rejection/correction requires upstream resolution.

# 12. Business Rules and Decision Constraints

At least one institutional author; DOI uniqueness where applicable; duplicates prohibited; documents remain outside Research; verification/publication history remains traceable; Faculty identity remains E-owned.

# 13. State and Lifecycle Impact

`Draft` → `Verified` → `Published` → `Archived`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H05 defines: `PublicationRegistered`, `PublicationVerified`, `ResearchOutputPublished`, `PublicationArchived`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research consumes authoritative workforce facts from L2-E03. No non-frozen participating domain is introduced by this L3 scenario.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Only the source-supported actors named in Section 6 may perform the H05-defined decisions/actions. Missing frozen authority is an upstream gap.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 override may create a missing state, decision, event, actor authority, or Finance outcome.

## 15.3 Business Audit and Evidence Obligations

Preserve the applicable Research identity, prior/current lifecycle position, decision/action authority, reasons/conditions where required, evidence references, source-domain references, and exact H05 events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported H05 path may complete only to its defined lifecycle position. The full frozen institutional outcome cannot be claimed while any material Section 17 gap remains unresolved.

## 16.2 Postconditions

Research history remains traceable; E ownership remains unchanged; unsupported frozen semantics are not approximated.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen **published or rejected** branch has no H05 `Rejected` Publication state/authority/event.
- Frozen **corrected where permitted** has no correction/amendment/version-successor lifecycle.
- Frozen “reviewed” is broader than H05 `VerifyPublication`; L3 cannot silently equate the two.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions beyond the open issues above.
