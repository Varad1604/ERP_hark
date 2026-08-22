# External Assessment Notice to Accepted Assessment Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-066  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Observation, finding, CAPA, readiness, institutional quality, and accreditation-cycle closure scenarios

# 2. Scenario Purpose

Coordinate an H09 external accreditation or compliance assessment from formal notice through readiness preparation, governed evidence exchange, assessor interaction, observations, findings, and institutional response while preserving source-domain evidence ownership.

# 3. Business Outcome

Evidence exchange, assessor interaction, observations, and institutional response produce an accepted assessment outcome or governed challenge

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An accreditation or compliance body schedules an external assessment.

## 4.2 End Boundary

The frozen outcome requires either an accepted assessment outcome or a governed challenge. H09-07 ends with publication of a Final Assessment Record after observations/findings, but H09 defines no acceptance lifecycle, challenge workflow, challenge authority, challenge state, or challenge event for that external assessment outcome. The frozen end boundary therefore cannot be completed authoritatively.

## 4.3 In Scope

- Receipt and coordination of the external assessment schedule.
- Institutional readiness verification.
- Validation of the H09 evidence repository for assessment use.
- Department interactions and campus/organizational assessment coordination.
- External Assessment Team interaction.
- Recording preliminary observations and assessment findings.
- Publication of the source-defined Final Assessment Record.
- Preservation of evidence ownership and assessment traceability.

## 4.4 Out of Scope

- Inventing an `Accepted` external assessment state.
- Inventing a challenge/appeal workflow, challenge decision authority, or challenge event.
- Treating internal Readiness Assessment approval as acceptance of the external assessor's outcome.
- Changing source-domain evidence facts during the assessment.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns external assessment coordination, H09 evidence/readiness preparation, assessment observations/findings, and the institutional Final Assessment Record. |
| VAR:Evidence-Owning Domain | Owns the operational facts/evidence referenced during the assessment and participates in evidence provision under its own governance. |

The external assessment body remains an external participant; it does not acquire H09 ownership.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| IQAC | Coordinates institutional accreditation readiness and assessment governance. | Coordinates / reviews |
| Accreditation Cell | Coordinates external assessment preparation and interaction. | Coordinates |
| Registrar | Participates in institutional assessment coordination. | Coordinates / responds |
| Institutional Leadership | Participates in material institutional assessment governance. | Reviews / responds |
| External Assessment Team | Conducts the external assessment and provides observations/findings. | External assessor |

# 7. Preconditions and Entry Criteria

- An applicable Accreditation Cycle exists and is ready to enter External Assessment.
- Required evidence has been governed through H09 and source-domain references remain traceable.
- The external assessment schedule and institutional scope are known.

# 8. Trigger

An accreditation or compliance body schedules an external assessment

# 9. Main Business Flow

1. H09 receives the external assessment schedule and confirms the assessment context.
2. H09 publishes the institutional assessment timeline and coordinates participating units.
3. IQAC verifies institutional readiness for the scheduled assessment.
4. H09 validates that the governed evidence repository is ready for assessment and that source references remain traceable.
5. The Accreditation Cell coordinates required institutional presentations, department interactions, and assessment activities.
6. The External Assessment Team conducts the scheduled assessment and interacts with the institution within the defined scope.
7. H09 records preliminary observations and formal assessment findings without altering the evidence owner's source facts.
8. H09 records the institution's source-supported response material where applicable.
9. H09 publishes the source-defined Final Assessment Record.
10. Stop before asserting that the outcome becomes `accepted` or enters a governed `challenge` because H09 defines neither branch.

# 10. Alternate Business Flows

The frozen challenge branch is not authorable from H09 because no challenge/appeal workflow, authority, state, rule, or event is defined. Normal variation in assessment scheduling or evidence preparation remains within H09-07 but does not resolve the missing final outcome semantics.

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required evidence is not ready | Institutional assessment readiness is incomplete | Resolve evidence gaps under H09 evidence governance before relying on the evidence | Accreditation & Compliance / evidence owner |
| Assessment finding identifies a compliance gap | The issue requires governed remediation | Record the observation/finding and route it to the source-defined finding/CAPA paths | Accreditation & Compliance |
| Institution disputes the external outcome | Frozen challenge path is required but undefined | Escalate upstream; do not invent challenge authority or lifecycle | Accreditation & Compliance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Operational source domains remain systems of record | Evidence-owning domains | Assessment coordination cannot rewrite source facts | L2-H09 Principle 2 |
| Evidence remains traceable to originating source records/documents | H09 / source boundary | Assessment evidence must retain lineage | L2-H09 Principle 3; BR-H09-009 |
| Observations belong to a governed review event including External Assessment | H09 | External assessment observations remain formally governed | L2-H09 BR-H09-019 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Accreditation Cycle | Internal Review | Enter external assessor engagement | External Assessment | H09 | L2-H09 H09-A02; §7 |
| Accreditation Cycle | External Assessment | H09-07 publishes the final assessment record | H09-A02 names next position `Result Published`; §7 instead names it `Completed` | H09 | L2-H09 H09-A02; §7; H09-07 |
| Observation / Finding | New assessment issue | Record governed assessment issue | Source-defined observation/finding lifecycle | H09 | L2-H09 H09-A06/A07 |

No owned external-assessment `Accepted` or `Challenged` lifecycle state is defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H09 defines Accreditation Cycle events including `Accreditation Milestone Completed`, `Accreditation Cycle Completed`, and `Accreditation Cycle Closed`, and observation/finding events for assessment issues. It defines no canonical event for external assessment acceptance, institutional challenge, challenge submission, challenge decision, or challenge closure.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Evidence-owning domains provide authoritative evidence references and institutional response inputs under their own ownership. The External Assessment Team supplies external observations/findings. H09 owns the institutional assessment record and issue governance, not the external body's systems or the source-domain business facts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

IQAC, Accreditation Cell, Registrar, Institutional Leadership, and the External Assessment Team are source-defined participants in H09-07. H09 does not define who has institutional authority to initiate or approve a challenge to an external assessment outcome, nor how external acceptance becomes authoritative within H09.

## 15.2 Separation of Duties, Overrides and Exceptions

No internal readiness approval may be treated as a substitute for external assessment acceptance. No challenge authority is invented.

## 15.3 Business Audit and Evidence Obligations

Preserve assessment schedule, scope, participating units, readiness evidence, source evidence references, assessor interactions, preliminary observations, formal findings, institutional response material, and Final Assessment Record.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

H09 can complete the coordination path through publication of the Final Assessment Record. The frozen accepted-outcome/challenge result cannot be claimed until upstream architecture defines those lifecycle and authority semantics.

## 16.2 Postconditions

The external assessment record, observations/findings, and evidence lineage remain governed and traceable; no unsupported accepted/challenged state is asserted.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen outcome requires an `accepted assessment outcome or governed challenge`.
- H09-07 ends at `Publish Final Assessment Record` and defines no acceptance/challenge workflow.
- H09 defines no challenge actor/authority, challenge state, challenge business rule, or challenge event.
- H09-A02 uses `Result Published` after `External Assessment`, while §7 uses `Completed`; the lifecycle terminology is internally inconsistent at the same point.
- H09's canonical domain model describes an `Assessment` governance outcome, but the aggregate catalogue does not define a separate external Assessment aggregate/lifecycle corresponding to the frozen result.

These issues materially affect the frozen final outcome and authority boundary.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H09 must define external assessment acceptance/challenge governance, including the owning lifecycle, authority, decision outcomes, and canonical events, and reconcile `Result Published` versus `Completed`.
