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
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance; applicable L2-G01 through L2-G10 operational domains; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L2-H07 — Enterprise Dashboard Workflows  
**Referenced By:** Campus governance, executive oversight, audit, reporting and authorized downstream consumers

---

# 2. Scenario Purpose

Govern a multi-domain campus operational risk from identification through assessment, mitigation, monitoring and evidence-based closure while source domains retain their incidents and operational work.

---

# 3. Business Outcome

Risk is assessed, mitigation is coordinated, monitoring is completed, and the governance risk is closed

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a multi-domain operational risk is identified.

## 4.2 End Boundary

The scenario ends when the frozen business outcome has been recorded under the source-defined governance boundary, all required source references and decisions are preserved, applicable follow-up is linked, and the full activity is auditable.

## 4.3 In Scope

- Tenant, institution, campus, purpose and correlation resolution.
- Authorization of the governance activity and evidence access.
- Risk evidence, responsible operational domain, configured category and severity matrix, affected scope and mitigation dependencies.
- Use of the `Operational Risk` business responsibility.
- Source-domain information consumed by reference without ownership transfer.
- Classified risk, mitigation plan, monitoring evidence, escalation where required and closed risk history.
- Applicable lifecycle transitions, services and domain-event publication explicitly defined by L2-G00.
- Immutable audit evidence and historical reconstruction.

## 4.4 Out of Scope

- Executing operational work owned by G01–G10.
- Modifying operational transactions, source-domain master data or published readiness facts.
- Replacing H07 dashboard composition, Section H notification delivery or H09 accreditation governance.
- Replacing the Platform Workflow, Authorization, Event or Audit infrastructure.
- Inventing lifecycle states, commands, events, publishers, approval authorities or technical mechanisms absent upstream.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-G-001 — Campus Readiness Assessment to Governance Readiness Decision | Provides or consumes readiness governance where applicable. |
| Related | L3-G-003 — Campus Operational Review to Governance Closure | Owns the canonical Operational Review journey. |
| Related | L3-G-005 — Operational Risk Identification to Risk Closure | Owns enterprise operational-risk governance. |
| Related | L3-G-006 — Multi-Domain Action Plan to Verified Closure | Owns corrective-action coordination and verification. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Campus Operations Governance / Section G | Primary owner. Owns the governance aggregate, conclusions, findings, recommendations, risks, action coordination and immutable governance history applicable to this scenario. |
| Operational domains / Section G and other named domains | Own their operational facts, indicators, incidents, actions and corrections; provide published references without transferring ownership. |
| Platform Foundation / Section A | Supplies Tenant, configuration, authorization, workflow, event and audit capabilities without owning the G00 business outcome. |
| Institutional Services & Intelligence / Section H, where frozen | Consumes governance outputs for dashboards, notification, executive intelligence or accreditation under its own ownership boundaries. |

The frozen participating-domain value remains exactly `G, H`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Governance Coordinator | Coordinates the assessment, review, finding, action or evidence request within authorized scope. | May initiate and coordinate; cannot assume executive or approval authority. |
| Campus Operations Manager | Reviews campus operational evidence and governance consequences. | Acts only within configured campus and governance authorization. |
| Executive Reviewer | Provides executive review, recommendation or decision where required. | Authority is policy-derived and not implied for every scenario step. |
| Risk Coordinator / Quality Assurance Officer | Governs risk treatment or verifies corrective-action evidence where applicable. | Does not execute source-domain operational work. |
| Read Only Auditor / Compliance Consumer | Receives authorized historical evidence. | Cannot change source or governance records. |

L2-G00 calls these typical roles and allows institutions to configure additional roles. It does not establish a universal named approver for every scenario.

---

# 7. Preconditions and Entry Criteria

- The Tenant, institution and campus are valid and resolvable.
- The trigger and authorized governance purpose are established.
- The affected G00 aggregate or required governance context can be identified.
- Required information is available from its canonical owner or its absence is explicitly handled as an exception.
- Applicable configuration, thresholds, templates, categories and policies can be resolved.
- Source information is reference-only and remains owned by its publishing domain.
- Authorization, actor and correlation context are available.
- Existing governance history is immutable and accessible under policy.

---

# 8. Trigger

A multi-domain operational risk is identified

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register the identified multi-domain risk with Tenant, campus, source and correlation references. | Campus Operations Governance; applicable participant | Register the identified multi-domain risk with Tenant, campus, source and correlation references. |
| 2 | Confirm that G00 owns the governance risk while source domains retain incidents and operations. | Campus Operations Governance; applicable participant | Confirm that G00 owns the governance risk while source domains retain incidents and operations. |
| 3 | Classify the risk category and responsible operational domain. | Campus Operations Governance; applicable participant | Classify the risk category and responsible operational domain. |
| 4 | Assess severity using the configured matrix and evidence. | Campus Operations Governance; applicable participant | Assess severity using the configured matrix and evidence. |
| 5 | Define the mitigation plan, responsible owner, target date and verification expectations. | Campus Operations Governance; applicable participant | Define the mitigation plan, responsible owner, target date and verification expectations. |
| 6 | Publish the mitigation initiation fact and coordinate required domain actions. | Campus Operations Governance; applicable participant | Publish the mitigation initiation fact and coordinate required domain actions. |
| 7 | Monitor progress, dependencies, blockers and residual exposure. | Campus Operations Governance; applicable participant | Monitor progress, dependencies, blockers and residual exposure. |
| 8 | Escalate the risk when configured thresholds or overdue conditions are met. | Campus Operations Governance; applicable participant | Escalate the risk when configured thresholds or overdue conditions are met. |
| 9 | Verify mitigation evidence and close the risk only when the governed closure criteria are satisfied. | Campus Operations Governance; applicable participant | Verify mitigation evidence and close the risk only when the governed closure criteria are satisfied. |
| 10 | Publish risk events and preserve immutable risk history. | Campus Operations Governance; applicable participant | Publish risk events and preserve immutable risk history. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| 1 | The risk remains above the permitted residual threshold | retain Monitoring or escalate rather than closing. | The existing lifecycle and source ownership remain preserved. |
| 2 | The identified condition belongs to one domain only | reference or return it to the source domain unless enterprise governance treatment is still required. | The existing lifecycle and source ownership remain preserved. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant, campus or authorization cannot be resolved. | The governance activity cannot proceed lawfully. | Reject or pause without changing any source or governance state. | Platform & Tenant; Campus Operations Governance |
| Required source information is missing, stale or inconsistent. | A trustworthy result cannot be produced. | Use the G00 exception workflow; correct, defer or escalate without inventing an outcome. | Source domain; Campus Operations Governance |
| A business invariant fails. | The requested transition would violate ownership or lifecycle integrity. | Reject the transition and preserve failure evidence. | Campus Operations Governance |
| Required review, verification or approval is incomplete. | The outcome cannot be treated as complete. | Retain the current source-defined lifecycle position and route follow-up. | Campus Operations Governance |
| Event publication condition is not satisfied. | Consumers must not receive a false completed fact. | Publish only a source-defined event after its corresponding transition. | Campus Operations Governance; Event Infrastructure |
| Processing is interrupted. | The governance activity remains incomplete. | Preserve history and resume only from a valid existing lifecycle position. | Campus Operations Governance |

No technical retry, rollback, database, UI, queue or infrastructure recovery design is introduced.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G00 governs oversight and never executes operational work. | Campus Operations Governance | Corrective execution remains with the source domain. | L2-G00 §§3–6, §14 |
| G00 never modifies operational data owned by G01–G10. | Campus Operations Governance; source domains | Inputs remain reference-only. | L2-G00 §§9, 13–14 |
| Governance history remains immutable and every lifecycle transition is audited. | Campus Operations Governance; Audit Infrastructure | Prior evidence and decisions cannot be rewritten. | L2-G00 §§13, 27 |
| `Operational Risk` is the scenario’s governing business responsibility. | Campus Operations Governance | The scenario cannot transfer its outcome to a source or infrastructure domain. | L2-G00 §§8, 11–12, 16–19 |
| Applicable lifecycle is `Identified → Assessed → Mitigation → Monitoring → Closed`. | Campus Operations Governance | Only source-supported states and order may be used. | L2-G00 §§11, 23 |
| Applicable services are Register Risk; Assess Risk; Record Mitigation; Close Risk. | Campus Operations Governance | Interactions use published business contracts. | L2-G00 §22 |
| Configuration, thresholds and extensions remain governed and effective-dated where specified. | Campus Operations Governance; Platform | Behaviour cannot be hardcoded or changed retrospectively. | L2-G00 §§6, 26 |
| Significant completed transitions publish immutable domain events. | Campus Operations Governance | Failed or incomplete outcomes cannot publish success facts. | L2-G00 §§20–21 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Operational Risk | Trigger and authorized context available | Progresses only through the applicable source-defined governance journey. | Identified → Assessed → Mitigation → Monitoring → Closed | Campus Operations Governance |
| Source-domain operational record | Existing canonical state | Referenced as evidence or receives a governed request; no direct mutation. | Unchanged by G00. | Publishing operational domain |
| Governance history | Existing immutable history | Receives new actor, transition, decision and correlation evidence. | Preserved and extended immutably. | Campus Operations Governance; Audit Infrastructure |
| Downstream reporting/evidence view | Absent or requires refresh | Receives the completed governed outcome where authorized. | Available without becoming the source of operational truth. | Applicable reporting or evidence owner |

Where L2-G00 does not define a scenario-specific aggregate or lifecycle, this Draft records that limitation and does not create state names by inference.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

L2-G00 states that Campus Operations Governance publishes immutable events and provides the following applicable catalogue entries. It does not provide an Aggregate-to-Event Matrix or a sole-publisher table.

| Event / Required Fact | Publication Condition | Business Effect / Consumer |
|---|---|---|
| `OperationalRiskRegistered` | After the corresponding completed G00 transition or calculation. | Authorized governance, audit, notification and reporting consumers receive the completed fact. |
| `OperationalRiskEscalated` | After the corresponding completed G00 transition or calculation. | Authorized governance, audit, notification and reporting consumers receive the completed fact. |
| `MitigationPlanCreated` | After the corresponding completed G00 transition or calculation. | Authorized governance, audit, notification and reporting consumers receive the completed fact. |
| `OperationalRiskClosed` | After the corresponding completed G00 transition or calculation. | Authorized governance, audit, notification and reporting consumers receive the completed fact. |

Event Infrastructure distributes events but does not own the G00 business fact.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Source operational information | Publishing operational domain | Campus Operations Governance | Immutable identifiers, status, period, scope, metric/evidence and provenance. |
| Authorization Decision | Authorization & Policy | Campus Operations Governance | Allowed actor, action, Tenant, campus and governance scope. |
| G00 governance service | Campus Operations Governance | Authorized participant | Register Risk; Assess Risk; Record Mitigation; Close Risk. |
| Governed outcome | Campus Operations Governance | Executive, H-domain or other authorized consumer | classified risk, mitigation plan, monitoring evidence, escalation where required and closed risk history. |
| Audit input | Campus Operations Governance | Audit Infrastructure | Actor, prior/new state, references, reason, time, correlation and outcome. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Every activity requires authorization for its Tenant, campus, governance scope and information classification.
- Authentication and Platform roles do not transfer business ownership to the Platform.
- Delegated or temporary authority must be effective-dated and auditable.
- Read access to governance history does not permit mutation.

## 15.2 Approval and Separation of Duty

- Review, executive decision, verification and configuration approval occur only where required by the applicable source lifecycle and policy.
- L2-G00 provides a configurable authorization matrix and typical roles rather than a universal approval actor.
- Verification responsibility must remain distinguishable from source-domain execution where policy requires separation.
- No unsupported approver, override or SoD exception is introduced.

## 15.3 Audit Obligations

The following must be auditable:

- Tenant, institution, campus, purpose and scope.
- Initiating actor and applicable authorization decision.
- Source identifiers, versions, periods and owning domains.
- Configuration, template, threshold or rule version used.
- Every lifecycle transition, finding, recommendation, decision and reason.
- Assigned owners, dates, dependencies and verification where applicable.
- Published canonical events and downstream hand-offs.
- Exceptions, recovery, correlation and final outcome.

---

# 16. Completion Criteria and Postconditions

The scenario is complete when:

- register the identified multi-domain risk with Tenant, campus, source and correlation references.
- confirm that G00 owns the governance risk while source domains retain incidents and operations.
- classify the risk category and responsible operational domain.
- assess severity using the configured matrix and evidence.
- define the mitigation plan, responsible owner, target date and verification expectations.
- publish the mitigation initiation fact and coordinate required domain actions.
- monitor progress, dependencies, blockers and residual exposure.
- escalate the risk when configured thresholds or overdue conditions are met.
- verify mitigation evidence and close the risk only when the governed closure criteria are satisfied.
- publish risk events and preserve immutable risk history.


Postconditions:

- Source-domain operational records and ownership remain unchanged by G00.
- The completed governance outcome and its lineage are available to authorized consumers.
- Governance history remains immutable and reproducible.
- No undefined state, command, event, publisher, actor or technical mechanism has been assumed.

---

# 17. Open Issues, Assumptions and Deferred Questions

No open issue prevents this scenario from proceeding to Architecture Approval. L2-G00 supplies the owning aggregate, lifecycle, services and canonical event catalogue required for the frozen outcome.

**Source Assessment:** PASS

No unsupported operational ownership transfer, actor, approval authority, lifecycle state, event, publisher or implementation mechanism is assumed by this Draft.
