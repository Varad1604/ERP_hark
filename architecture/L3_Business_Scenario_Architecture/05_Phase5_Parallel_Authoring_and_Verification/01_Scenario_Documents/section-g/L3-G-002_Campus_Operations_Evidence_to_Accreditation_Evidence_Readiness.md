# Campus Operations Evidence to Accreditation Evidence Readiness

# 1. Scenario Metadata

**Scenario ID:** L3-G-002  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance; applicable L2-G01 through L2-G10 operational domains; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows; L2-H07 — Enterprise Dashboard Workflows; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Campus governance, executive oversight, audit, reporting and authorized downstream consumers

---

# 2. Scenario Purpose

Assess and publish a source-owned campus-operations evidence package for H09 using the Accreditation Evidence Readiness type while preserving H09 ownership of institutional accreditation readiness.

---

# 3. Business Outcome

G00 assesses and publishes the campus-operations evidence package for H09 without making the institutional accreditation-readiness decision

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an L2-H09 accreditation cycle requests campus governance and operational-readiness evidence.

## 4.2 End Boundary

The scenario ends when the frozen business outcome has been recorded under the source-defined governance boundary, all required source references and decisions are preserved, applicable follow-up is linked, and the full activity is auditable.

## 4.3 In Scope

- Tenant, institution, campus, purpose and correlation resolution.
- Authorization of the governance activity and evidence access.
- H09 accreditation cycle and criterion context, g00 governance history, operational-readiness facts, unresolved gaps and source references.
- Use of the `Campus Readiness Assessment` business responsibility.
- Source-domain information consumed by reference without ownership transfer.
- Verified campus-operations evidence package, operational readiness conclusion and unresolved-gap references for h09.
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

An L2-H09 accreditation cycle requests campus governance and operational-readiness evidence

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the H09 request with framework, cycle, criterion, Tenant and correlation context. | Campus Operations Governance; applicable participant | Receive the H09 request with framework, cycle, criterion, Tenant and correlation context. |
| 2 | Authorize the accreditation purpose and define the permitted campus-operations evidence scope. | Campus Operations Governance; applicable participant | Authorize the accreditation purpose and define the permitted campus-operations evidence scope. |
| 3 | Create an Accreditation Evidence Readiness assessment under G00. | Campus Operations Governance; applicable participant | Create an Accreditation Evidence Readiness assessment under G00. |
| 4 | Collect governance history and current operational-readiness facts from their owning G domains. | Campus Operations Governance; applicable participant | Collect governance history and current operational-readiness facts from their owning G domains. |
| 5 | Validate provenance, version, period, completeness, integrity and permitted accreditation use. | Campus Operations Governance; applicable participant | Validate provenance, version, period, completeness, integrity and permitted accreditation use. |
| 6 | Calculate the campus-operations evidence-readiness result and identify unresolved evidence gaps. | Campus Operations Governance; applicable participant | Calculate the campus-operations evidence-readiness result and identify unresolved evidence gaps. |
| 7 | Review the result within G00 without calculating the institutional accreditation-readiness score. | Campus Operations Governance; applicable participant | Review the result within G00 without calculating the institutional accreditation-readiness score. |
| 8 | Assemble the source evidence references, approved snapshots where permitted, result and gaps into a governed package. | Campus Operations Governance; applicable participant | Assemble the source evidence references, approved snapshots where permitted, result and gaps into a governed package. |
| 9 | Publish the verified package to H09 through the Accreditation Readiness Evidence Contract. | Campus Operations Governance; applicable participant | Publish the verified package to H09 through the Accreditation Readiness Evidence Contract. |
| 10 | Close the G00 assessment and preserve delivery and evidence lineage for audit. | Campus Operations Governance; applicable participant | Close the G00 assessment and preserve delivery and evidence lineage for audit. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| 1 | Evidence is incomplete | publish only a qualified package with explicit unresolved gaps or retain the assessment for correction. | The existing lifecycle and source ownership remain preserved. |
| 2 | H09 requests correction | the owning G domain corrects its operational fact and G00 issues a new evidence version; H09 does not rewrite the source record. | The existing lifecycle and source ownership remain preserved. |

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
| `Campus Readiness Assessment` is the scenario’s governing business responsibility. | Campus Operations Governance | The scenario cannot transfer its outcome to a source or infrastructure domain. | L2-G00 §§8, 11–12, 16–19 |
| Applicable lifecycle is `Scheduled → Collecting → Calculated → Reviewed → Closed`. | Campus Operations Governance | Only source-supported states and order may be used. | L2-G00 §§11, 23 |
| Applicable services are Create Readiness Assessment; Collect Domain Readiness; Calculate Campus Readiness; Publish Readiness Result; Generate Readiness Summary. | Campus Operations Governance | Interactions use published business contracts. | L2-G00 §22 |
| Configuration, thresholds and extensions remain governed and effective-dated where specified. | Campus Operations Governance; Platform | Behaviour cannot be hardcoded or changed retrospectively. | L2-G00 §§6, 26 |
| Significant completed transitions publish immutable domain events. | Campus Operations Governance | Failed or incomplete outcomes cannot publish success facts. | L2-G00 §§20–21 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Campus Readiness Assessment | Trigger and authorized context available | Progresses only through the applicable source-defined governance journey. | Scheduled → Collecting → Calculated → Reviewed → Closed | Campus Operations Governance |
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
| `CampusReadinessAssessmentScheduled` | After the corresponding completed G00 transition or calculation. | Authorized governance, audit, notification and reporting consumers receive the completed fact. |
| `CampusReadinessCalculated` | After the corresponding completed G00 transition or calculation. | Authorized governance, audit, notification and reporting consumers receive the completed fact. |

Event Infrastructure distributes events but does not own the G00 business fact.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Source operational information | Publishing operational domain | Campus Operations Governance | Immutable identifiers, status, period, scope, metric/evidence and provenance. |
| Authorization Decision | Authorization & Policy | Campus Operations Governance | Allowed actor, action, Tenant, campus and governance scope. |
| G00 governance service | Campus Operations Governance | Authorized participant | Create Readiness Assessment; Collect Domain Readiness; Calculate Campus Readiness; Publish Readiness Result; Generate Readiness Summary. |
| Governed outcome | Campus Operations Governance | Executive, H-domain or other authorized consumer | verified campus-operations evidence package, operational readiness conclusion and unresolved-gap references for H09. |
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

- receive the H09 request with framework, cycle, criterion, Tenant and correlation context.
- authorize the accreditation purpose and define the permitted campus-operations evidence scope.
- create an Accreditation Evidence Readiness assessment under G00.
- collect governance history and current operational-readiness facts from their owning G domains.
- validate provenance, version, period, completeness, integrity and permitted accreditation use.
- calculate the campus-operations evidence-readiness result and identify unresolved evidence gaps.
- review the result within G00 without calculating the institutional accreditation-readiness score.
- assemble the source evidence references, approved snapshots where permitted, result and gaps into a governed package.
- publish the verified package to H09 through the Accreditation Readiness Evidence Contract.
- close the G00 assessment and preserve delivery and evidence lineage for audit.


Postconditions:

- Source-domain operational records and ownership remain unchanged by G00.
- The completed governance outcome and its lineage are available to authorized consumers.
- Governance history remains immutable and reproducible.
- No undefined state, command, event, publisher, actor or technical mechanism has been assumed.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Required Resolution |
|---|---|---|---|
| G-002-ISSUE-01 | Non-blocking | G00 defines the evidence-readiness type, reports and H09 boundary but no package aggregate, package lifecycle, package publication event or sole publisher. | Resolve in L2-G00 or the named owning domain before downstream specification relies on the missing contract. |
| G-002-ISSUE-02 | Non-blocking | The exact approval role for publishing a campus-operations evidence package is not named. | Resolve in L2-G00 or the named owning domain before downstream specification relies on the missing contract. |

**Source Assessment:** PASS WITH ISSUES

No unsupported operational ownership transfer, actor, approval authority, lifecycle state, event, publisher or implementation mechanism is assumed by this Draft.
