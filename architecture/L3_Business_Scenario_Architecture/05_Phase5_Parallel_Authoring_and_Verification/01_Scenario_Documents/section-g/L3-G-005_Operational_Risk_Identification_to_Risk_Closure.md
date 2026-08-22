# Operational Risk Identification to Risk Closure

# 1. Scenario Metadata

**Scenario ID:** L3-G-005  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance; applicable G01–G10 operational risk sources  
**Referenced By:** Authorized campus-governance and institutional intelligence consumers

# 2. Scenario Purpose

Govern a multi-domain campus operational risk from identification through assessment, mitigation, monitoring, and closure while operational incidents and corrective execution remain owned by their source domains.

# 3. Business Outcome

Risk is assessed, mitigation is coordinated, monitoring is completed, and the governance risk is closed

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A multi-domain operational risk is identified.

## 4.2 End Boundary

The scenario ends when the Operational Risk has progressed through mitigation and monitoring to `Closed`, with responsible-domain references, mitigation evidence, and immutable governance history preserved.

## 4.3 In Scope

- Registration of one G00 Operational Risk.
- Risk classification and severity assessment.
- Responsible operational-domain reference.
- Mitigation planning and governance coordination.
- Monitoring, escalation where applicable, and closure.
- Source-defined risk lifecycle and events.

## 4.4 Out of Scope

- Ownership or execution of source-domain incidents and operational remediation.
- Enterprise risks outside the G00 campus-operations governance boundary.
- Dashboard or notification delivery.
- Inventing risk states, severity values, or closure rules beyond configured G00 governance.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-G-006 — Multi-Domain Action Plan to Verified Closure | A mitigation plan may coordinate corrective actions through a separate action-plan lifecycle. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Campus Operations Governance / G | Owns the Operational Risk aggregate, its classification, mitigation governance, monitoring, closure, and history. |
| Institutional Services & Intelligence / H | Consumes governed risk information within its own intelligence/notification responsibilities; it does not own the risk. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Source-Supported Authority |
|---|---|---|
| Risk Coordinator | Registers and coordinates the Operational Risk. | L2-G00 authorization matrix explicitly assigns `Register Operational Risk` to this role. |
| Campus Operations Manager | Reviews operational governance information relevant to risk treatment. | L2-G00-defined role. |
| Executive Reviewer | Participates when configured escalation or executive review is required. | L2-G00-defined typical role. |

# 7. Preconditions and Entry Criteria

- The identified condition affects campus operations and qualifies for G00 governance treatment.
- At least one responsible operational domain can be referenced as required by the G00 invariant.
- Evidence sufficient for initial risk assessment is available or its absence can be treated as a governance exception.
- Risk categories, severity rules, and mitigation governance can be resolved from configured G00 policy.

# 8. Trigger

A multi-domain operational risk is identified

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register the identified governance risk and reference the responsible operational domain. | Risk Coordinator / G00 | The risk enters `Identified`. |
| 2 | Classify and assess the risk using the configured category and severity rules. | Risk Coordinator / G00 | The risk reaches `Assessed`. |
| 3 | Establish the mitigation plan, responsible owner/domain, target, and applicable verification expectations. | G00 | The risk progresses to `Mitigation`. |
| 4 | Coordinate the mitigation while source domains execute their own operational work. | G00 / source domains | Governance tracks mitigation without owning execution. |
| 5 | Monitor progress, dependencies, blockers, and residual exposure. | G00 | The risk progresses to `Monitoring`. |
| 6 | Escalate the risk where configured thresholds require executive attention. | Risk Coordinator / G00 | The escalation is governed without changing risk ownership. |
| 7 | Close the risk when the configured governance closure conditions are satisfied. | G00 | The Operational Risk reaches `Closed`. |
| 8 | Preserve the complete risk, mitigation, monitoring, escalation, and closure history. | G00 | The frozen business outcome is auditable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Continued exposure | Residual risk remains above the configured acceptable condition | Retain the risk in monitoring or escalate it rather than closing. | The risk remains governed and open. | G00 |
| Reassessment | New evidence materially changes the assessed condition | Reassess under the configured G00 risk rules before continuing mitigation/monitoring. | Risk governance remains source-supported. | G00 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Responsible operational domain cannot be identified | The G00 invariant is not satisfied | Correct or escalate the ownership exception before valid risk progression. | G00 |
| Risk classification is invalid | Assessment cannot be relied upon | Reassess using the configured risk classification and severity rules. | G00 |
| Mitigation evidence is incomplete | Closure would be unsupported | Continue mitigation/monitoring and do not close the risk. | G00 / responsible source domain |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Operational Risk shall reference one responsible operational domain. | G00 | Risk registration requires an attributable operational responsibility. | L2-G00 §13 |
| G00 governs enterprise operational risks while operational incidents remain source-domain owned. | G00 / source domains | Risk governance cannot take ownership of the underlying incident. | L2-G00 §18 |
| The Operational Risk lifecycle is `Identified → Assessed → Mitigation → Monitoring → Closed`. | G00 | No local state may be invented. | L2-G00 §23 |
| Risk categories, severity, escalation, and mitigation are configurable. | G00 | Scenario uses the active governed configuration. | L2-G00 §18, §26 |
| Governance history is immutable and lifecycle transitions are audited. | G00 | Risk history remains reconstructable. | L2-G00 §13, §27 |

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Operational Risk | Identified | Assessed, mitigated, monitored, and closed | `Identified → Assessed → Mitigation → Monitoring → Closed` | G00 |
| Source-domain incident/operation | Existing canonical state | Referenced and acted upon by its own owner | Unchanged by G00 directly | Respective operational domain |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Scenario Purpose |
|---|---|---|
| `OperationalRiskRegistered` | Campus Operations Review (G00) | Represents risk registration. |
| `MitigationPlanCreated` | Campus Operations Review (G00) | Represents initiation of mitigation governance. |
| `OperationalRiskEscalated` | Campus Operations Review (G00) | Represents governed escalation where applicable. |
| `OperationalRiskClosed` | Campus Operations Review (G00) | Represents risk closure. |

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Hand-off | Purpose |
|---|---|---|---|
| Source operational domain | G00 | Incident/readiness/operational evidence and remediation progress | Supports risk governance without ownership transfer. |
| G00 | H | Governed risk status and closure information | Supports authorized downstream intelligence/notification use. |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

L2-G00 explicitly assigns `Register Operational Risk` to the `Risk Coordinator`. Subsequent mitigation and closure are governed by the G00 risk lifecycle and configured policy; no additional universal named closure approver is defined or invented.

## 15.2 Separation of Duties, Overrides and Exceptions

G00 coordinates mitigation but does not execute source-domain operational remediation. No additional scenario-specific separation-of-duty rule is defined upstream.

## 15.3 Business Audit and Evidence Obligations

Risk registration, classification, severity, responsible domain, mitigation, monitoring, escalation, closure, lifecycle transitions, and canonical events must remain auditable.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The Operational Risk references a responsible operational domain.
- Assessment and mitigation governance are complete.
- Monitoring confirms the configured closure conditions are satisfied.
- The risk reaches `Closed` through the canonical lifecycle.
- Governance history and applicable risk events are preserved.

## 16.2 Postconditions

- A closed and auditable G00 Operational Risk exists.
- Source-domain operational facts and incidents remain owned by their canonical domains.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

No open issues.

**Architecture assessment:** `PASS`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
