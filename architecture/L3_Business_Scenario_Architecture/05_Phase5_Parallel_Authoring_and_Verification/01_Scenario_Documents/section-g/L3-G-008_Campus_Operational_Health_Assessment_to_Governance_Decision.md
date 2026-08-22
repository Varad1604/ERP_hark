# Campus Operational Health Assessment to Governance Decision

# 1. Scenario Metadata

**Scenario ID:** L3-G-008  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G, H  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance  
**Referenced By:** Authorized campus-governance consumers and participating domains defined by the frozen scenario

# 2. Scenario Purpose

Evaluate the G00-owned Campus Operational Health view from source-domain information and produce a governed decision or recommendation without converting the health view into an invented lifecycle aggregate.

# 3. Business Outcome

Campus operational health is evaluated and an appropriate governance decision or recommendation is produced

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

Consolidated operational information requires an institutional health assessment.

## 4.2 End Boundary

The scenario ends only when campus operational health is evaluated and an appropriate governance decision or recommendation is produced.

## 4.3 In Scope

- G00-owned Campus Operational Health.
- Readiness, risk, SLA, action, and governance KPI information used as source-owned inputs.
- Calculation of governance health using configured definitions.
- Governance review of the health result and recording of a decision or recommendation.
- Authorized consumption by Section H without ownership transfer.

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
| Institutional Services & Intelligence / H | Participates or consumes the governed outcome within its own locked responsibilities; no G00 ownership transfers to H. |

The frozen participating-domain value is preserved exactly as `G, H`.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Source-Supported Authority |
|---|---|---|
| Campus Operations Manager | Conducts the operational governance review of the health result. | L2-G00 assigns Conduct Operational Review. |
| Executive Reviewer | Participates in executive review or recommendation where required. | L2-G00 lists Executive Reviewer as a typical governance role; final authority is not universally assigned. |

No additional actor or decision authority is inferred beyond the L2-G00 role catalogue and authorization matrix.

# 7. Preconditions and Entry Criteria

- The frozen trigger has occurred within a valid governance scope.
- Required source information can be attributed to its canonical owner, or its absence is explicitly treated as a governance exception.
- Applicable configuration and governance expectations can be resolved where L2-G00 defines them.
- The participating governance activity is authorized under applicable institutional policy.
- Existing governance history remains available without modification.

# 8. Trigger

Consolidated operational information requires an institutional health assessment

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Establish the authorized campus, period, and purpose of the health assessment. | G00 | The assessment scope is established. |
| 2 | Collect the required source-owned readiness, SLA, risk, action, and KPI information. | G00 | Authoritative inputs are available by reference. |
| 3 | Validate lineage, scope, currency, and comparability of the inputs. | G00 | Only valid source information participates. |
| 4 | Calculate Campus Operational Health using the applicable configured governance definition. | G00 | Campus Operational Health is recalculated. |
| 5 | Review the health result and material drivers. | Campus Operations Manager / G00 | The result is governance-reviewed. |
| 6 | Record the appropriate governance decision or recommendation within the available authority. | G00 / Executive Reviewer where applicable | The frozen outcome is recorded. |
| 7 | Preserve the result, inputs, recommendation/decision, and audit history. | G00 | The outcome remains traceable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Insufficient source information | Health cannot be evaluated reliably | Record the evidence gap and defer or escalate the assessment. | No health outcome is invented. |
| Material adverse health | The assessment warrants follow-up | Create or link a source-supported finding, risk, action plan, or escalation. | Follow-up uses its own canonical boundary. |

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
| Campus Operational Health is owned by G00. | G00 | Applies to this scenario | L2-G00 |
| Governance KPIs reference source-domain metrics. | G00 | Applies to this scenario | L2-G00 |
| G00 never modifies source operational data. | G00 | Applies to this scenario | L2-G00 |
| Governance history is immutable and audited. | G00 | Applies to this scenario | L2-G00 |

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry Condition | Scenario Impact | Resulting Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Campus Operational Health | Consolidated operational information is available | Recalculated and reviewed | No formal lifecycle is defined by L2-G00 | G00 |
| Operational Review | Governance review is used | Progresses only if a review is instantiated | Planned → Active → Findings Recorded → Completed → Closed | G00 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Business Meaning in this Scenario |
|---|---|---|
| `CampusOperationalHealthUpdated` | Campus Operations Review (G00) | Campus Operational Health has been recalculated. |
| `GovernanceRecommendationPublished` | Campus Operations Review (G00) | A governance recommendation has been issued where applicable. |
| `MultiDomainReviewCompleted` | Campus Operations Review (G00) | A multi-domain governance review has completed where applicable. |

L2-G00 states that Campus Operations Review publishes its immutable governance domain events; therefore G00 is the publisher only for event names explicitly catalogued there.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Business Hand-off | Ownership Effect |
|---|---|---|---|
| G01–G10 source domains | G00 | Authoritative operational information used for health evaluation | Source domains retain operational ownership. |
| G00 | Section H | Governed Campus Operational Health and authorized decision/recommendation | H consumes under its own responsibilities; G00 ownership remains unchanged. |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Decision Authority

L2-G00 supports review through Campus Operations Manager and identifies Executive Reviewer as a typical role, but it does not assign one universal final decision authority for every Campus Operational Health assessment.

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

- Campus operational health is evaluated and an appropriate governance decision or recommendation is produced.
- Source ownership remains unchanged.
- Applicable G00 governance history is preserved and auditable.

## 16.2 Postconditions

- No operational source record acquires G00 ownership.
- No unsupported lifecycle state, event, publisher, actor, or decision authority has been introduced.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-008-ISSUE-01 | Non-blocking | Campus Operational Health is explicitly owned and has a calculation service/event, but L2-G00 defines no dedicated aggregate or lifecycle. | Keep L4 from inventing health states unless upstream defines them. |
| G-008-ISSUE-02 | Non-blocking | The exact authority for choosing between a governance decision and recommendation is not universally assigned. | Resolve the institutional decision-authority mapping before downstream authority enforcement. |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
