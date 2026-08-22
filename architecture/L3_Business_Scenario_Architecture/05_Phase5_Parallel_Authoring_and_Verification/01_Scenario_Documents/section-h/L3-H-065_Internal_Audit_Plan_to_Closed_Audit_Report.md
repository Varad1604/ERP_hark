# Internal Audit Plan to Closed Audit Report

# 1. Scenario Metadata

**Scenario ID:** L3-H-065  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Audited Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Observation, finding, CAPA, compliance, readiness, and institutional quality scenarios

# 2. Scenario Purpose

Govern an H09 internal audit from an authorized risk-based plan through evidence review, testing, observations, findings, management response, reporting, and formal closure while the audited business domain retains ownership of its operational facts.

# 3. Business Outcome

Scope, evidence, testing, observations, findings, response, and approval produce a formally closed audit report

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A risk-based internal audit is authorized.

## 4.2 End Boundary

The frozen outcome requires formal audit closure after management response/approval. H09 defines the audit workflow, lifecycle, separation-of-duty rule, management acknowledgement prerequisite, and `Audit Closed` event, but it does not identify the canonical business role that provides the mandatory management acknowledgement for a variable audited domain. The final authority step is therefore unresolved.

## 4.3 In Scope

- Approved audit plan, scope, schedule, and audit-team assignment.
- Evidence review and audit testing at business level.
- Observation, finding, and non-conformity recording.
- Audit report preparation and closing meeting.
- Mandatory management acknowledgement/acceptance before closure.
- CAPA initiation where required by findings.
- Audit closure only when H09 rules permit.

## 4.4 Out of Scope

- Inventing a generic audited-domain approving role.
- Correcting audited-domain operational records inside H09.
- Closing Critical findings without required resolution.
- CAPA execution beyond the audit's creation of required CAPA items.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns Audit, Observation/Finding governance, audit report, audit lifecycle, and audit closure decision constraints. |
| VAR:Audited Domain | Owns the operational facts/processes under review and provides the business response/evidence required by the audit under its own authority. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Internal Auditor | Executes the source-defined internal audit and prepares audit conclusions. | Performs |
| IQAC | Participates in accreditation/readiness audit governance. | Reviews |
| Compliance Cell | Participates in compliance audit governance and follow-up. | Reviews / coordinates |
| Department Coordinator | Participates for departmental audits where applicable. | Provides evidence / response |
| Registrar | Participates in the H09 internal-audit workflow. | Reviews / coordinates |

H09 requires management acceptance/acknowledgement before closure but does not define a canonical generic management actor for `VAR:Audited Domain`.

# 7. Preconditions and Entry Criteria

- The audit plan is approved.
- Audit scope is defined.
- Audit team is assigned.
- Required evidence is available.
- Auditor independence satisfies BR-H09-016.

# 8. Trigger

A risk-based internal audit is authorized

# 9. Main Business Flow

1. H09 creates the Internal Audit under the approved audit plan.
2. The Internal Auditor defines the audit scope and participating organizational units consistent with the authorized plan.
3. H09 assigns the audit team and publishes the approved schedule.
4. The audit team reviews available evidence from H09 and the audited domain without taking ownership of source records.
5. The Internal Auditor conducts the audit and records observations, findings, and non-conformities supported by the review.
6. H09 prepares the Audit Report and conducts the source-defined closing meeting.
7. H09 requires management acceptance/acknowledgement before closure under BR-H09-017.
8. Where Critical findings exist, H09 prevents closure until those findings are resolved under BR-H09-018; mandatory CAPA is created where BR-H09-023 applies.
9. Stop before asserting the final management acknowledgement and `Closed` transition because the canonical role/authority that provides that mandatory acceptance for a variable audited domain is not defined upstream.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Critical finding remains unresolved | Audit reaches proposed closure with an unresolved Critical finding | Keep the audit open; route the finding through the governed remediation path | Closure blocked | Accreditation & Compliance |
| Finding requires CAPA | Critical finding is formally established | Create the required CAPA relationship and preserve audit/finding traceability | Remediation obligation established | Accreditation & Compliance / remediation owner in later scenario |
| Departmental audit | Audited scope is a department | Department Coordinator participates in evidence and response activities | Departmental response preserved | Audited domain / Accreditation & Compliance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Auditor has operational ownership of audited unit | Separation-of-duty rule is violated | Reassign the audit so the auditor does not audit an organizational unit they own operationally | Accreditation & Compliance |
| Management acknowledgement is absent | Audit closure is prohibited | Obtain the required acknowledgement from the proper upstream-defined authority | Accreditation & Compliance / audited domain |
| Critical finding remains open | Audit closure is prohibited | Resolve the Critical finding under governed remediation before later closure | Accreditation & Compliance / remediation owner |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Audit schedules require approval before publication | H09 | Schedule cannot be published without approval | L2-H09 BR-H09-015 |
| Auditors cannot audit organizational units for which they have operational ownership | H09 | Enforces business separation of duty | L2-H09 BR-H09-016 |
| Audit reports require management acknowledgement before closure | H09 | Final closure requires a management decision | L2-H09 BR-H09-017 |
| Audit closure is prohibited while unresolved Critical findings exist | H09 | Critical findings must be resolved before closure | L2-H09 BR-H09-018 |
| Critical Findings require CAPA creation | H09 | Remediation obligation is created when applicable | L2-H09 BR-H09-023 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Audit | Planned | Approve/schedule the audit | Scheduled | H09 | L2-H09 H09-A05; §7 |
| Audit | Scheduled | Conduct the audit | In Progress | H09 | L2-H09 H09-A05; §7 |
| Audit | In Progress | Prepare findings and report | Reporting | H09 | L2-H09 H09-A05; §7 |
| Audit | Reporting | Conduct management review/acknowledgement | `Management Review` in H09-A05 / `Review` in §7 | H09 | L2-H09 H09-A05; §7 |
| Audit | Review position | Frozen outcome requires formal closure | Closed, but authority step is unresolved | H09 | L2-H09 H09-A05; H09-06 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Audit Planned | Audit aggregate / H09 | Audit governance participants | Records audit planning | L2-H09 §11 |
| Audit Scheduled | Audit aggregate / H09 | Audited participants and governance consumers | Records approved schedule | L2-H09 §11 |
| Audit Started | Audit aggregate / H09 | Audit governance consumers | Records audit execution start | L2-H09 §11 |
| Audit Completed | Audit aggregate / H09 | Audit reporting consumers | Records completion of audit work | L2-H09 §11 |
| Audit Report Generated | Audit aggregate / H09 | Governance and audited participants | Records report availability | L2-H09 §11 |
| Audit Closed | Audit aggregate / H09 | Governance consumers | Records formal closure when valid | L2-H09 §11 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

The audited domain supplies source-owned evidence and its business response. H09 records audit observations/findings and the Audit Report but does not acquire ownership of the audited operational records.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The audit plan and schedule are governed by H09. BR-H09-017 requires management acknowledgement before closure, but H09 does not define the canonical actor/role that provides that acknowledgement across `VAR:Audited Domain` scopes. L3 therefore cannot assign the final closure authority locally.

## 15.2 Separation of Duties, Overrides and Exceptions

BR-H09-016 prohibits an auditor from auditing an organizational unit for which the auditor has operational ownership. No exception is invented. Critical findings cannot be bypassed for closure.

## 15.3 Business Audit and Evidence Obligations

Preserve authorized plan, audit scope, team assignment, schedule approval, evidence references, testing conclusions, observations, findings, non-conformities, management response/acknowledgement, Audit Report, CAPA links, and closure evidence.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The audit can be authored through report generation and closing review. The frozen formally closed outcome cannot be claimed until the canonical management acknowledgement authority is defined and the valid closure decision occurs.

## 16.2 Postconditions

Audit findings and report history remain traceable; no unsupported `Closed` transition or management authority is asserted.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- H09-W06 requires `Obtain Management Acceptance` before `Close Audit`.
- BR-H09-017 independently requires management acknowledgement before closure.
- H09-06's actor list contains Internal Auditor, IQAC, Compliance Cell, Department Coordinator, and Registrar, but it does not identify which role is the required management acknowledgement authority for a variable audited domain.
- H09-A05 calls the pre-closure state `Management Review`; §7 calls it `Review`.

The missing closure authority materially affects the frozen final outcome.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H09 must identify the canonical management acknowledgement authority or authority rule for internal-audit closure and normalize the pre-closure state name.
