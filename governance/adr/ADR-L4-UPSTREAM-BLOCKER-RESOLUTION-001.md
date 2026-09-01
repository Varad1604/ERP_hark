# ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001 — L4 Upstream Blocker Resolution

**Status:** Approved — Authorized for incorporation  
**Date:** 2026-09-01  
**Authority:** Lead Architect explicit authorization recorded in SutraERP_Level_3 project  
**Architecture Scope:** L2 and L3 corrections required to remove 18 L4 parent-readiness blockers  

## 1. Decision

The 18 identified L4 upstream blockers shall be resolved at their canonical owning architecture level. L4 shall consume the resulting architecture and shall not invent business rules, states, events, publishers, authorities, ownership, or cross-domain contracts.

The approved resolutions are:

| Scenario | Locked resolution |
|---|---|
| L3-F-078 | Published Financial Reports remain immutable. Post-publication correction creates a successor version. Supersession and permitted withdrawal are explicit F13 dispositions governed by the Financial Report Approval Authority; archival remains separate. |
| L3-F-084 | Finance configuration rollback is a new governed version based on an eligible prior approved configuration; historical versions are never reactivated or rewritten. Finance configuration approval/rollback authority and events are defined in F00. |
| L3-G-007 | No SLA Deviation aggregate is introduced. SLA governance completion is evidenced through the canonical Operational Review and any required Multi-Domain Action Plan closure. |
| L3-G-010 | G00 owns a configurable Operational Maturity Assessment model with governed criteria, lifecycle, authority, evidence and events. |
| L3-G-012 | G00 owns an Executive Governance Resolution record. Executive resolution records a binding G00 governance decision but does not directly mutate an originating aggregate owned elsewhere. |
| L3-G-285 | `ZoneCreated` is the canonical Zone establishment event and a new Zone enters `Proposed`. |
| L3-G-286 | The scenario governs reclassification of an existing Space. `SpaceClassificationChanged` is canonical. Initial classification remains part of Space registration. |
| L3-G-303 | G10 owns Infrastructure Inspection; G01 owns Facilities operational inspection. Corrective maintenance is handed to G01 and material risk is referred to G00 without transferring physical Infrastructure ownership. |
| L3-G-304 | G10 owns the Infrastructure preventive-maintenance requirement; G01 exclusively owns Preventive Maintenance Plan, Maintenance Schedule and execution. The G10 requirement event is distinct from G01 scheduling. |
| L3-G-306 | G10 publishes the Infrastructure emergency-maintenance requirement; G01 owns operational emergency-maintenance response and retains its response event semantics. |
| L3-G-313 | No new Disaster Management domain is created. G00 owns cross-campus recovery governance; G10 retains physical Infrastructure state/recovery facts; operational execution remains with the applicable operational owner. |
| L3-G-316 | `Decommissioned → Archived` is the canonical archival path. “Otherwise eligible” does not create another entry state. Infrastructure Manager is the archival approval authority. |
| L3-H-009 | H01 Connector gains terminal `Retired`; retirement is distinct from `Disabled`; `ConnectorRetired` is published by H01; Platform Administrator is the retirement authority. |
| L3-H-012 | Notification Template Approval Authority is the canonical authority for Template review/approval. Existing Template lifecycle remains; no Template lifecycle event is added without a consumer requirement. |
| L3-H-028 | Scenario is narrowed to H04-supported Placement statistics/outcome reporting. No diversity framework, report publication lifecycle or generic reporting-period aggregate is introduced. |
| L3-H-045 | Existing H06 Volunteer Assignment, Guest Lecture and Industry/Advisory engagement variants remain canonical. No generic Alumni Contribution aggregate or C/D/E acceptance lifecycle is introduced. |
| L3-H-047 | H06 consumes Finance canonical `PaymentSettlementCompleted`. H06 records a Finance settlement reference/correlation only and never owns the financial outcome. |
| L3-H-050 | A04 is the canonical owner of Workspace availability/resolution. H07 owns dashboard composition associated with an A04 Workspace and consumes A04 authorized Workspace context. |

## 2. Ownership safeguards

- No L1 domain ownership is changed.
- No source-owned financial, academic, student, workforce, institutional-campus or physical-infrastructure fact transfers ownership.
- New events are published only by the owner of the business fact they represent.
- Section A capabilities remain authorization/configuration/audit/platform capabilities and do not become semantic publishers for business-domain facts.

## 3. Downstream incorporation

The decision shall be incorporated through targeted L2 amendments, synchronized L3 parent corrections, readiness-baseline reconciliation for these 18 identities, and corresponding L4 repairs. Unaffected L2/L3/L4 architecture remains unchanged.

## 4. Prohibition

This ADR does not authorize redesign outside the 18 listed scenarios, final L4 corpus lock, or modification of `main` outside normal repository integration governance.
