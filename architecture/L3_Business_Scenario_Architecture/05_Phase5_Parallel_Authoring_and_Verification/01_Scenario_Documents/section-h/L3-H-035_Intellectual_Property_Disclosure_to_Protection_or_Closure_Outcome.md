# Intellectual Property Disclosure to Protection or Closure Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; L2-F00 — Finance Domain Model & Foundation

# 2. Scenario Purpose

Apply H05 Patent/IP governance to a protectable Research result without inventing missing disclosure, ownership-decision, commercialization, or abandonment semantics and without transferring financial ownership from Finance.

# 3. Business Outcome

Ownership review, protection decision, filing, commercialization, abandonment, or closure is completed under policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A potentially protectable research result is disclosed

## 4.2 End Boundary

H05 supports Patent registration, filing, verified statutory-status updates, and lifecycle closure. The frozen outcome starts earlier at disclosure and includes several decisions/actions not defined by H05.

## 4.3 In Scope

Patent/IP metadata registration after valid context exists; ownership-policy validation; filing; verified external statutory status; eligible lifecycle closure; evidence preservation.

## 4.4 Out of Scope

Invented IP Disclosure aggregate/event, ownership-review/protection decision, commercialization workflow, abandonment state/event, or financial-commercialization hand-off.

# 5. Ownership and Participating Domains

E03 retains inventor/faculty workforce facts. Finance owns monetary/accounting consequences. Research owns Patent/IP metadata and the H05 Patent lifecycle; external Patent Office remains statutory authority for external status.

# 6. Business Actors, Roles and Responsibilities

Principal Investigator / Inventor and Research Office participate in H05 Patent registration. Research Office performs filing, status update, and closure actions.

# 7. Preconditions and Entry Criteria

A patentable output context exists; inventors are valid; project reference exists where applicable; configured ownership policy is satisfied; filing evidence is available before filing.

# 8. Trigger

A potentially protectable research result is disclosed

# 9. Main Business Flow

1. Receive the disclosed result but stop short of inventing a disclosure lifecycle.
2. Where the upstream-required ownership/protection decision is already valid, register H05 Patent metadata and enter `Draft`.
3. Validate inventors, project, ownership policy, duplicate constraints, and filing readiness.
4. Submit filing; enter `Filed` and publish `PatentFiled`.
5. Record verified Patent Office status through the H05 status workflow.
6. Close a source-supported eligible lifecycle and publish `PatentLifecycleClosed`.
7. Do not claim commercialization or abandonment branches without upstream definitions.

# 10. Alternate Business Flows

External status may become `Under Examination`, `Granted`, or `Rejected` through the defined status update. H05 also depicts `Expired` for closure, but provides no explicit transition that creates `Expired`.

# 11. Failure, Interruption and Business Recovery

Invalid inventor/ownership context prevents registration. Incomplete filing prevents filing. Unsupported protection, commercialization, abandonment, or expiry semantics cannot be replaced by generic closure.

# 12. Business Rules and Decision Constraints

Configured IP ownership policy applies; at least one inventor is required; external Patent Office status is authoritative; identifiers and filing/status history are immutable; Finance alone owns monetary consequences.

# 13. State and Lifecycle Impact

H05: `Draft → Filed → Filed / Under Examination / Granted / Rejected`; `Granted / Rejected / Expired → Closed`. No disclosure/protection/commercialization/abandonment states exist, and no explicit transition into `Expired` is defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research publishes `PatentRegistered`, `PatentFiled`, `PatentStatusUpdated`, and `PatentLifecycleClosed`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

E03 owns workforce facts. Finance owns any monetary/accounting consequences. H05 defines no commercialization-specific Research→Finance business contract.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H05 roles are used only for Patent registration, filing, status, and closure. No disclosure reviewer, protection decision-maker, commercialization authority, or abandonment authority is invented.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 override can create missing IP lifecycle decisions.

## 15.3 Business Audit and Evidence Obligations

Preserve inventor declarations, ownership-policy reference, Patent metadata, filing references, statutory-status history, closure reason, and Patent events.

# 16. Completion Criteria and Postconditions

The source-supported Patent lifecycle can reach filing/status/closure outcomes. The full frozen scenario cannot complete because disclosure, ownership review, protection decision, commercialization, and abandonment are not fully defined.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen start boundary is IP **disclosure**, but H05 defines no disclosure aggregate/state/event.
- Ownership review and protection decision are not canonical H05 decisions/states/events.
- Commercialization has no H05 workflow or Research→Finance business hand-off.
- “Abandoned” appears only as a closure trigger; no `Abandoned` state/transition/event exists.
- `Expired` appears in the lifecycle/closure table but no canonical transition establishes it.

These gaps materially affect mandatory outcome branches.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
