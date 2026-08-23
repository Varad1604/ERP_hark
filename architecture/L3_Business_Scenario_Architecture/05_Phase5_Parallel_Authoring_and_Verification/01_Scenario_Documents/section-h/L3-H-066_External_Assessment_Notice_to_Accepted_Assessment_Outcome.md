# External Assessment Notice to Accepted Assessment Outcome

## 1. Scenario Metadata

**Scenario ID:** L3-H-066  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Coordinate an external accreditation/compliance assessment while preserving institutional evidence ownership and governing the resulting assessment record.

## 3. Business Outcome

Evidence exchange, assessor interaction, observations, and institutional response produce an accepted assessment outcome or governed challenge

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An accreditation or compliance body schedules an external assessment.

### 4.2 End Boundary

Evidence exchange, assessor interaction, observations, and institutional response produce an accepted assessment outcome or governed challenge.

### 4.3 In Scope

- H09-owned governance behaviour materially required to reach the frozen business outcome.
- Applicable source-domain references, evidence and governed hand-offs without ownership transfer.
- Only upstream-defined rules, lifecycle positions, events, approvals and evidence obligations relevant to this scenario.

### 4.4 Out of Scope

- Operational business-state ownership belonging to participating source domains.
- Adjacent downstream institutional outcomes after this scenario's End Boundary.
- Later architecture-level implementation concerns.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance (H) | Owns the H09 governance outcome and applicable H09 lifecycles. |
| VAR:Evidence-Owning Domain | Retains ownership of authoritative source evidence/business facts while supplying governed references or readiness evidence. |

Participation and evidence contribution do not transfer canonical ownership.

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Coordinates institutional external-assessment readiness and response. | Performs / reviews |
| Accreditation Cell | Coordinates assessment preparation and evidence. | Performs |
| Registrar | Participates in institutional coordination. | Participates |
| Institutional Leadership | Participates in external assessment governance. | Participates |
| External Assessment Team | Performs the external assessment interaction. | External participant |

## 7. Preconditions and Entry Criteria

- An external assessment schedule has been received.
- The applicable Accreditation Cycle and evidence context exist.

## 8. Trigger

An accreditation or compliance body schedules an external assessment

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the assessment schedule and confirm the external assessment context. | Accreditation & Compliance | External assessment coordination begins. |
| 2 | Verify readiness and validate the governed evidence repository, including source-domain references. | IQAC / VAR:Evidence-Owning Domain | Assessment evidence is prepared without transferring source ownership. |
| 3 | Coordinate institutional timeline, department interactions, campus visit and assessor activity. | Accreditation Cell | Assessment interaction is supported. |
| 4 | Receive preliminary observations and record assessment findings. | Accreditation & Compliance | Observations/findings are governed in H09. |
| 5 | Publish the Final Assessment Record. | Accreditation & Compliance | The H09-07 workflow reaches its defined final assessment record. |

> The authoritative flow cannot be extended to the frozen End Boundary without resolving the material issue in Section 17.1.

## 10. Alternate Business Flows

No scenario-specific alternate business flow is defined by the authoritative L2 architecture.

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Frozen outcome requires an accepted assessment outcome or governed challenge | H09-07 ends at Final Assessment Record and defines neither an acceptance decision nor challenge path/state/event. | Do not infer acceptance from publication and do not invent a challenge lifecycle; upstream resolution is required. | Accreditation & Compliance |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| BR-H09-019 — every Observation belongs to an Audit, External Assessment, Compliance Review, Management Review, Quality Review or equivalent governed review. | Accreditation & Compliance | Maintains observation provenance. | L2-H09_Accreditation_and_Compliance_Workflows.md §8 |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Accreditation Cycle | External Assessment | External assessment is coordinated and assessment findings are recorded. | Result Published / Completed position is defined for the cycle, but no accepted/challenged assessment state exists. | Accreditation & Compliance | L2-H09_Accreditation_and_Compliance_Workflows.md §6 H09-A02 / §7 |

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Observation Recorded | Observation aggregate | Finding; CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |
| Finding Created | Finding aggregate | CAPA | Records the governed business fact at the scenario boundary. | L2-H09_Accreditation_and_Compliance_Workflows.md §11 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| VAR:Evidence-Owning Domain | Accreditation & Compliance | Source Evidence Reference Contract | Provide authoritative evidence references for external assessment. | L2-H09_Accreditation_and_Compliance_Workflows.md §23.1 |

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

The exact authority required to complete the frozen outcome is not established upstream; see Section 17.1.

### 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty rule is defined for external assessment outcome acceptance/challenge.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| External assessment coordination | Assessment schedule, governed evidence references, observations, findings and Final Assessment Record | Accreditation & Compliance |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The frozen outcome requires an accepted assessment outcome or a governed challenge after assessor interaction and institutional response.
- Locked H09 does not define the acceptance/challenge semantics needed to prove completion.

### 16.2 Postconditions

- Recorded observations and findings remain H09-owned governance records.
- Source operational facts remain owned by `VAR:Evidence-Owning Domain`.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H09-07 terminates with `Publish Final Assessment Record`; no accepted-assessment lifecycle, acceptance event/decision, governed challenge path, or challenge event is defined. | The frozen final outcome cannot be reached without inventing outcome semantics. | Define external-assessment acceptance/challenge governance upstream in H09. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
