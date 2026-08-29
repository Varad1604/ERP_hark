# Multi-Domain SLA Deviation to Governance Resolution

# 1. Scenario Metadata

**Scenario ID:** L3-G-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance  
**Referenced By:** Authorized campus-governance consumers and participating domains defined by the frozen scenario

# 2. Scenario Purpose

Govern a significant campus-operations SLA deviation without inventing a dedicated SLA lifecycle or transferring the underlying service-performance fact from its operational owner.

# 3. Business Outcome

SLA deviation is reviewed, responsibility and corrective action are established, and governance resolution is completed

**Architecture status:** `BLOCKED` — the frozen outcome cannot be fully specified without resolving the material upstream gaps recorded in Section 17.

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A significant SLA deviation is identified across campus operations.

## 4.2 End Boundary

The intended end boundary is the frozen business outcome above. A canonical end-state interpretation is not asserted where Section 17 identifies a material upstream lifecycle, authority, event, or contract gap.

## 4.3 In Scope

- The source-owned SLA deviation and its scope, period, service context, and evidence.
- G00 review of SLA performance under its Multi-Domain SLA Governance responsibility.
- Recording responsibility and a governance finding where supported.
- Creation and verification of a Multi-Domain Action Plan where corrective action is required.
- Preservation of SLA governance history and source ownership.

## 4.4 Out of Scope

- Execution or correction of operational work owned by G01–G10.
- Transfer of ownership from source operational domains to G00.
- Dashboard composition, notification delivery, or other Section H responsibilities merely because H participates in or consumes the outcome.
- Any locally invented lifecycle, approval authority, event, publisher, or recovery mechanism.

## 4.5 Related Scenarios — Optional

- Related governance scenarios may consume or produce findings, risks, action plans, readiness decisions, or executive recommendations only through their own canonical boundaries.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Campus Operations Governance / G | Primary owner of the frozen governance outcome and any G00-owned governance information used in this scenario. |

The frozen participating-domain value is preserved exactly as `G`.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Source-Supported Authority |
|---|---|---|
| Campus Operations Manager | Conducts the G00 operational review of the deviation. | L2-G00 assigns Conduct Operational Review. |
| Governance Coordinator | Creates a corrective Action Plan when the review determines action is required. | L2-G00 assigns Create Action Plan. |
| Quality Assurance Officer | Verifies action completion when a corrective Action Plan is used. | L2-G00 assigns Verify Action Completion. |

No additional actor or decision authority is inferred beyond the L2-G00 role catalogue and authorization matrix.

# 7. Preconditions and Entry Criteria

- The frozen trigger has occurred within a valid governance scope.
- Required source information can be attributed to its canonical owner, or its absence is explicitly treated as a governance exception.
- Applicable configuration and governance expectations can be resolved where L2-G00 defines them.
- The participating governance activity is authorized under applicable institutional policy.
- Existing governance history remains available without modification.

# 8. Trigger

A significant SLA deviation is identified across campus operations

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate the source-owned SLA deviation and its governance scope. | Campus Operations Manager / G00 | A valid SLA-governance matter is established. |
| 2 | Review SLA performance, impact, and available evidence without changing the source fact. | Campus Operations Manager / G00 | The deviation is reviewed under G00 governance. |
| 3 | Record the governance finding and responsible domain or owner where supported. | G00 | Responsibility is recorded. |
| 4 | If corrective action is required, create and assign a Multi-Domain Action Plan linked to the finding. | Governance Coordinator / G00 | Corrective action is established under the Action Plan lifecycle. |
| 5 | Monitor source-domain completion evidence without executing the operational correction. | G00 | Governance progress remains traceable. |
| 6 | Verify corrective-action evidence where the Action Plan path applies. | Quality Assurance Officer / G00 | Action completion is verified under the source-defined responsibility. |
| 7 | Do not assert a distinct SLA governance-resolution state beyond the source-defined review/action lifecycles. | G00 | The Draft stops at the authoritative boundary pending the blocking gap. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Below governance materiality | The deviation does not require G00 treatment | Leave operational handling with its canonical owner and preserve the evidence. | No G00 resolution is invented. |
| Enterprise risk emerges | The reviewed deviation represents an operational risk | Use the canonical Operational Risk journey separately. | Risk ownership remains G00; the SLA fact remains source-owned. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required source information is missing or inconsistent | A trustworthy governance outcome cannot be established | Correct, defer, or escalate the evidence gap without inventing the missing fact | G00 and the owning source domain |
| Required governance authority is unresolved | A decision cannot be treated as institutionally authoritative | Preserve the current governance position and route the authority gap for upstream resolution | Campus Operations Governance |
| The applicable lifecycle does not support the proposed conclusion | Completion would create an unauthorized state or transition | Do not assert completion; retain the last authoritative lifecycle position | Campus Operations Governance |

Recovery is business-level only. No technical retry, queue, rollback, database, infrastructure, or deployment mechanism is specified.

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G00 governs oversight and never executes operational work. | G00 | Applies to this scenario | L2-G00 |
| G00 never modifies operational data owned by G01–G10. | G00 | Applies to this scenario | L2-G00 |
| Operational reviews may consider SLA Performance as reference-only input. | G00 | Applies to this scenario | L2-G00 |
| Multi-Domain SLA Review is an Executive Governance workflow family. | G00 | Applies to this scenario | L2-G00 |

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry Condition | Scenario Impact | Resulting Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| SLA deviation | Source fact exists | Referenced and governed; no G00 lifecycle is defined for the deviation itself | No dedicated SLA resolution state is defined | Publishing operational domain |
| Operational Review | Review is initiated where required | May progress through its canonical lifecycle | Planned → Active → Findings Recorded → Completed → Closed | G00 |
| Multi-Domain Action Plan | Corrective action is required | May progress through its canonical lifecycle | Created → Assigned → In Progress → Verified → Closed | G00 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Business Meaning in this Scenario |
|---|---|---|
| `OperationalFindingRecorded` | Campus Operations Review (G00) | A governance finding has been recorded. |
| `ActionPlanCreated` | Campus Operations Review (G00) | A corrective Action Plan has been created. |
| `ActionAssigned` | Campus Operations Review (G00) | A corrective action has been assigned. |
| `ActionVerified` | Campus Operations Review (G00) | Action completion has been verified. |

L2-G00 states that Campus Operations Review publishes its immutable governance domain events; therefore G00 is the publisher only for event names explicitly catalogued there.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Business Hand-off | Ownership Effect |
|---|---|---|---|
| Publishing operational domain | G00 | SLA status/deviation evidence | Source domain retains ownership. |
| G00 | Responsible operational owner | Governance finding and corrective-action requirement | Operational execution remains with the source owner. |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Decision Authority

L2-G00 identifies authorization for conducting Operational Reviews, creating Action Plans, and verifying Action completion, but it does not define a canonical authority that declares an SLA deviation “governance resolved.”

## 15.2 Separation of Duties

No additional separation-of-duty rule is created locally. Where G00 explicitly separates creation, review, risk registration, or action verification roles, those upstream responsibilities remain intact.

## 15.3 Audit Obligations

- Preserve the triggering governance context and source references.
- Preserve the applicable configuration or expectation used in the decision.
- Preserve every source-supported governance transition, finding, recommendation, decision, and reason.
- Preserve applicable canonical event publication and downstream hand-offs.
- Preserve unresolved gaps rather than replacing them with assumptions.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The frozen completion criteria are understood, but canonical completion cannot be asserted until the blocking issues in Section 17 are resolved. The Draft therefore preserves only the source-supported portion of the journey.

## 16.2 Postconditions

- No operational source record acquires G00 ownership.
- No unsupported lifecycle state, event, publisher, actor, or decision authority has been introduced.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-007-BLOCKER-01 | Blocking | L2-G00 defines Multi-Domain SLA Governance/Review but no SLA Deviation aggregate, lifecycle, or resolution state. | Define the canonical SLA-governance resolution semantics. |
| G-007-BLOCKER-02 | Blocking | No dedicated SLA-resolution event or business contract identifies when the frozen “governance resolution is completed” boundary has been reached. | Define the completion fact/contract and canonical publisher if event publication is required. |
| G-007-BLOCKER-03 | Blocking | The older Draft inferred a composition of Operational Review and Action Plan as the SLA resolution lifecycle; that composition is not established upstream. | Resolve upstream rather than composing lifecycles locally. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
