# Reporting Cycle Initiation to Governed Collection Plan

# 1. Scenario Metadata

**Scenario ID:** L3-H-056  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** NEP, NAAC & AISHE Reporting  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H08 — NEP, NAAC & AISHE Reporting Workflows  
**Referenced By:** Regulatory data collection, validation, snapshot, approval, submission, correction, and closure scenarios

# 2. Scenario Purpose

Establish a governed H08 Reporting Cycle and collection plan for a statutory or accreditation reporting period, including schedule, responsibilities, source-reporting context, validation/evidence obligations, and downstream approval expectations while source domains retain ownership of all business evidence.

# 3. Business Outcome

Scope, responsibilities, deadlines, source contracts, validations, and approvals are established in an active reporting cycle

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A statutory or accreditation reporting period opens.

## 4.2 End Boundary

The scenario ends when H08 has established the Reporting Cycle in its source-defined `Planned` position with its reporting schedule, template, responsible-officer assignments, and governed reporting requirements established for subsequent Data Collection. The frozen word `active` is not asserted as a canonical lifecycle state because H08 defines no `Active` Reporting Cycle state.

## 4.3 In Scope

- Creation of the H08 Reporting Cycle for one published Reporting Framework.
- Assignment of a published Report Template.
- Establishment of timeline and deadlines through the Reporting Schedule owned by the Reporting Cycle.
- Assignment of source-defined reporting responsibilities.
- Identification of the approved reporting information/evidence expected from evidence-owning domains.
- Establishment of validation, evidence, approval, and submission-governance expectations already defined by the governing framework/template.

## 4.4 Out of Scope

- Starting or completing source-domain operational work.
- Copying canonical source records into H08 ownership.
- Inventing an `Active` Reporting Cycle state.
- Inventing source-domain evidence actors or new cross-domain reporting contracts.
- Performing data collection, validation, snapshot generation, approval, or submission within this scenario.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| NEP, NAAC & AISHE Reporting / H | Owns Reporting Cycle, Reporting Schedule, reporting timeline, cycle status, reporting requirements, and H08 reporting governance. |
| VAR:Evidence-Owning Domain | Owns the authoritative institutional facts/evidence later referenced or consumed by H08; participation never transfers source ownership. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Reporting Coordinator | Manages the H08 Reporting Cycle and its planning context where authorized. | Initiates / coordinates |
| Responsible Officer | H08-W02 records responsible-officer assignment within the Reporting Cycle plan. | Performs the assigned reporting responsibility as governed by the plan |

L2-H08 records a formal Actors-section gap. `Reporting Coordinator` is listed as a typical reporting role and H08-W02 explicitly assigns Responsible Officers, but H08 does not provide a complete canonical actor taxonomy for every evidence-owning domain.

# 7. Preconditions and Entry Criteria

- A source-supported published Reporting Framework exists for the reporting period.
- A published Report Template exists and is applicable to the Reporting Cycle.
- The reporting period, deadline, and reporting scope can be determined from the governing H08 configuration.
- Evidence-owning domains retain authoritative ownership of the facts expected during later collection.

# 8. Trigger

A statutory or accreditation reporting period opens

# 9. Main Business Flow

1. H08 identifies the published Reporting Framework governing the newly opened reporting period.
2. H08 creates the Reporting Cycle for the applicable institutional scope and period.
3. H08 assigns the applicable published Report Template to the cycle.
4. H08 establishes the Reporting Schedule, timeline, deadlines, and milestones for the cycle.
5. H08 assigns the Responsible Officers required by the source-defined planning workflow.
6. H08 records the governed source-reporting requirements and evidence categories that later collection must obtain from their authoritative owners.
7. H08 carries forward the framework/template validation, evidence, approval, and submission requirements that govern downstream processing.
8. H08 publishes the reporting schedule and leaves the Reporting Cycle in the source-defined `Planned` lifecycle position, ready for a later transition into Data Collection.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Administrative initiation | The reporting period is opened by an authorized institutional initiation rather than schedule | Create and govern the Reporting Cycle through the same H08 planning requirements | Planned cycle established | NEP, NAAC & AISHE Reporting |
| Regulatory deadline initiates planning | A regulatory deadline establishes the need for the cycle | Establish timeline and reporting schedule against the governing published framework/template | Planned cycle established | NEP, NAAC & AISHE Reporting |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Applicable Reporting Framework is not published | Cycle cannot validly depend on the framework | Govern the framework upstream before later cycle creation | NEP, NAAC & AISHE Reporting |
| Applicable Report Template is not published | Reporting Cycle cannot meet BR-002 | Govern the template upstream before later cycle creation | NEP, NAAC & AISHE Reporting |
| Required planning information is incomplete | Collection plan is not ready | Complete the H08-owned schedule/responsibility planning before later Data Collection | NEP, NAAC & AISHE Reporting |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Reporting Cycle references one published Reporting Framework | NEP, NAAC & AISHE Reporting | The cycle cannot be planned against an unpublished framework | L2-H08 BR-001 |
| Every Reporting Cycle references one published Report Template | NEP, NAAC & AISHE Reporting | The applicable template must be governed before the cycle is established | L2-H08 BR-002 |
| Reporting data originates from canonical operational domains | Evidence-owning domains | The collection plan references source information without transferring ownership | L2-H08 BR-003; O-002 |
| H08 never modifies operational business data | Evidence-owning domains / H08 boundary | Planning cannot change source facts | L2-H08 BR-004 |
| Reporting Schedule is owned by Reporting Cycle as a value within that aggregate | NEP, NAAC & AISHE Reporting | Schedule governance remains within the cycle rather than creating another owner | L2-H08 §§6.3, 6.6 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Reporting Cycle | New cycle | Create, assign template, configure timeline, assign responsibility, publish schedule | Planned | H08 | L2-H08 §6.6; H08-W02; §18.1 |
| Reporting Schedule | Owned within Reporting Cycle | Establish reporting timeline/deadlines | Governed schedule for the Planned cycle | H08 | L2-H08 §§6.3, 6.6 |

H08's canonical Reporting Cycle lifecycle contains `Planned` and `DataCollection`, not `Active`.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ReportingCycleCreated | H08 | Reporting governance participants | Records cycle creation | L2-H08 §10.3 |
| ReportingCycleScheduled | H08 | Reporting governance participants | Records publication of the Reporting Schedule | L2-H08 §10.3 |
| ReportingCycleStarted | H08 | Reporting participants | H08 defines this event for reporting start; this Draft does not use it to invent an `Active` state | L2-H08 §10.3 |

## 14.2 Business Contracts and Cross-Domain Hand-offs

The cycle plan identifies approved reporting information required from evidence-owning domains. H08 may consume source information through approved Reporting Contracts defined upstream, but the cycle-planning workflow does not itself create or rename those source contracts. Evidence-owning domains remain authoritative for the underlying facts.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Reporting Coordinator is source-supported for Reporting Cycle management. H08-W02 also records assignment of Responsible Officers. The complete actor/authority model for each variable evidence-owning domain is not defined in H08 and is not invented here.

## 15.2 Separation of Duties, Overrides and Exceptions

The later institutional approval hierarchy is configurable under H08. This planning scenario records the requirement for downstream approval but does not perform or invent an approval decision.

## 15.3 Business Audit and Evidence Obligations

Preserve Reporting Cycle creation, selected framework/template version, reporting scope, reporting period, schedule/deadlines, responsible-officer assignments, and the governing validation/evidence/approval requirements.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A Reporting Cycle exists against one published framework and one published template.
- Its Reporting Schedule, timeline, responsibilities, and downstream reporting-governance requirements are established.
- Required evidence/source information remains assigned to its canonical owner.
- The source-defined lifecycle position is `Planned` and ready for later Data Collection.

## 16.2 Postconditions

The governed collection plan is available for subsequent reporting execution without altering source-domain ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen outcome uses the phrase **active reporting cycle**, while H08's canonical Reporting Cycle lifecycle has no `Active` state; H08-W02's explicit postcondition is `Planned`. This Draft preserves `Planned` and treats the frozen wording as describing an established current cycle rather than inventing a state.
- H08 defines approved Reporting Contracts as a consumption mechanism, but H08-W02 does not define a dedicated cycle-level workflow for selecting or version-binding each source contract named in the frozen outcome.
- H08's canonical crosswalk records that the required Actors section is absent. The source still identifies Reporting Coordinator and W02 Responsible Officers, so planning can be drafted, but actor completeness remains an upstream documentation issue.

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions beyond the upstream actor and source-contract-detail gaps recorded above.
