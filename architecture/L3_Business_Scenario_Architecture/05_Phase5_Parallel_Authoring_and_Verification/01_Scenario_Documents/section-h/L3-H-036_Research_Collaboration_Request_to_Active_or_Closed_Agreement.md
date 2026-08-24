# Research Collaboration Request to Active or Closed Agreement

# 1. Scenario Metadata

**Scenario ID:** L3-H-036  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; L2-F00 — Finance Domain Model & Foundation

# 2. Scenario Purpose

Govern an institutional Research Collaboration using the H05-defined registration, approval, activity, and completion lifecycle while preserving external agreement, workforce, and Finance ownership boundaries.

# 3. Business Outcome

Due diligence and approval establish, reject, amend, terminate, or close a governed collaboration

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An internal or external party proposes research collaboration

## 4.2 End Boundary

H05 supports `Proposed → Active → Completed`. The frozen outcome additionally requires rejection, amendment, termination, and a closed-agreement outcome that H05 does not define canonically.

## 4.3 In Scope

Partner/agreement validation, Collaboration registration, configured approval, active collaboration activities, completion, evidence preservation, and source ownership.

## 4.4 Out of Scope

Invented rejection, amendment/versioning, termination, or closed states/events; legal agreement ownership; invented Finance hand-offs.

# 5. Ownership and Participating Domains

E03 owns workforce facts. Finance owns monetary/accounting consequences where collaboration has them. Research owns Collaboration metadata, H05 lifecycle, activities, and Research events. Legal agreement content remains outside Research.

# 6. Business Actors, Roles and Responsibilities

Research Office registers/completes. Research Director / Authorized Institutional Approver performs configured approval. Principal Investigator / Research Office records active collaboration activities.

# 7. Preconditions and Entry Criteria

Partner organizations are identified and verified; agreement metadata/evidence is available; participants are valid; applicable approval workflow is configured.

# 8. Trigger

An internal or external party proposes research collaboration

# 9. Main Business Flow

1. Validate participating organization and agreement context.
2. Register the Collaboration; enter `Proposed` and publish `CollaborationCreated`.
3. Complete configured institutional approval.
4. Enter `Active` and publish `CollaborationApproved`.
5. Record governed collaboration activities and `CollaborationActivityRecorded`.
6. When required obligations are satisfied, complete the Collaboration and publish `CollaborationCompleted`.
7. Stop before unsupported reject, amend, terminate, or close branches.

# 10. Alternate Business Flows

Multi-institution collaborations are permitted. H05 mentions administrative termination only in prose; no canonical termination state/transition/event exists.

# 11. Failure, Interruption and Business Recovery

Unverified organizations, missing agreements, or incomplete approval prevent activation. Unsupported frozen branches remain architecture gaps.

# 12. Business Rules and Decision Constraints

At least one external organization is required; agreement and participant validation are mandatory; approval routing is configurable; legal agreements remain outside Research; completed history is immutable; Finance retains monetary ownership.

# 13. State and Lifecycle Impact

Collaboration: `Proposed → Active → Completed`; activities retain `Active`. No `Rejected`, `Amended`, `Terminated`, or `Closed` lifecycle positions are defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research publishes `CollaborationCreated`, `CollaborationApproved`, `CollaborationActivityRecorded`, and `CollaborationCompleted`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

E03 source facts remain authoritative. Finance remains authoritative for financial consequences, but H05 defines no collaboration-specific Finance contract required to satisfy a separate financial outcome.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Only H05-defined Research Office, Research Director / Authorized Institutional Approver, and PI responsibilities are used.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 role may invent reject/amend/terminate/close authority.

## 15.3 Business Audit and Evidence Obligations

Preserve partner/agreement references, approval authority/history, collaboration activities, completion evidence, state history, and Research Collaboration events.

# 16. Completion Criteria and Postconditions

The supported H05 path completes at `Completed`. The frozen scenario cannot claim all required outcomes while rejection, amendment, termination, and closed-agreement semantics remain undefined.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- No Collaboration rejection state/command/authority/event.
- No Collaboration/agreement amendment lifecycle or version event.
- No canonical termination state/transition/event; administrative termination is prose only.
- H05 terminates at `Completed`, not the frozen “closed agreement” outcome.
- Finance is frozen as participating, but H05 defines no collaboration-specific Finance hand-off.

The missing frozen lifecycle branches are material.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
