# Compliance Obligation to Effective Requirement Lifecycle

# 1. Scenario Metadata

**Scenario ID:** L3-H-063  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Obligation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** Compliance review, evidence, audit, CAPA, readiness, and institutional governance scenarios

# 2. Scenario Purpose

Govern H09's institutional Compliance Requirement record for a legal, regulatory, accreditation, or institutional obligation while the operational domain accountable for satisfying the underlying obligation retains its own business ownership.

# 3. Business Outcome

The requirement is classified, assigned, monitored, revised, fulfilled, waived, expired, or retired with full history

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A legal, regulatory, accreditation, or institutional obligation is identified or changed.

## 4.2 End Boundary

The frozen outcome requires governed branches for revision, fulfilment, waiver, expiry, and retirement. H09 explicitly supports creation, classification, assignment, activation, monitoring, review, compliance, expiry, renewal, and archival, but it does not define waiver or retirement states/actions/events and does not define a revision lifecycle branch. The full frozen outcome therefore cannot be completed authoritatively.

## 4.3 In Scope

- Registration and classification of a Compliance Requirement.
- Assignment of exactly one accountable Business Owner.
- Applicability, review frequency, renewal rules, evidence expectations, and due dates.
- Activation, monitoring, review, compliant outcome, expiry, renewal, and archival where supported.
- Complete compliance history.

## 4.4 Out of Scope

- Inventing `Waived`, `Retired`, or `Revised` states.
- Mapping `Archived` to `Retired` or `Compliant` to every possible meaning of `fulfilled` without upstream authority.
- Performing the underlying operational obligation on behalf of its owning domain.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accreditation & Compliance / H | Owns Compliance Requirement, Compliance Register, requirement classification, review schedule, compliance status, and requirement history. |
| VAR:Obligation-Owning Domain | Remains accountable for the underlying institutional obligation or operational business activity whose compliance status H09 governs. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Compliance Cell | Owns the institutional compliance register, review cadence, deviation tracking, and action follow-up within H09. | Coordinates / reviews |
| IQAC | Participates in institutional compliance governance. | Reviews |
| Registrar | Participates in compliance requirement governance. | Reviews / coordinates |
| Department Compliance Officers | Participate where the accountable obligation sits within a department. | Performs / reports |
| Business Owner | Holds the source-defined accountability assigned to one Compliance Requirement. | Accountable for the obligation |

# 7. Preconditions and Entry Criteria

- The obligation has a sufficiently defined legal, regulatory, accreditation, or institutional basis to be recorded.
- Applicability and accountable ownership can be established without transferring the underlying business capability into H09.

# 8. Trigger

A legal, regulatory, accreditation, or institutional obligation is identified or changed

# 9. Main Business Flow

1. H09 registers the Compliance Requirement in the institutional compliance register.
2. H09 classifies the requirement and defines its applicability.
3. H09 assigns exactly one accountable Business Owner.
4. H09 establishes review frequency, renewal rules, mandatory evidence expectations, and due dates.
5. H09 activates the requirement and begins source-defined compliance monitoring.
6. At each governed review point, H09 reviews current compliance status and supporting evidence.
7. Where the obligation is satisfied, H09 may progress the requirement to the source-defined `Compliant` lifecycle position.
8. Where the requirement reaches expiry, H09 follows the source-defined expiry/review/renewal or archival path.
9. Stop before asserting frozen branches for waiver, retirement, or revision because H09 defines no authoritative lifecycle/state/event for those outcomes.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Requirement renewed | Continuing obligation reaches a renewal point | Preserve history and record the source-defined renewal outcome | Requirement continues under renewed governance | Accreditation & Compliance |
| Compliance breach identified | Review determines that the obligation is not being met | Record the source-defined breach and route governed remediation/CAPA where applicable | Non-compliance remains traceable | Accreditation & Compliance / obligation owner |
| Compliance restored | Evidence confirms that a previously breached obligation is restored | Record the source-defined restored outcome | Current compliance status updated | Accreditation & Compliance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Business Owner cannot be assigned | BR-H09-005 cannot be satisfied | Resolve accountable ownership before activation | Accreditation & Compliance / obligation-owning domain |
| Required review frequency is absent | Requirement governance is incomplete | Define the mandatory review cadence before relying on the requirement lifecycle | Accreditation & Compliance |
| Frozen waiver/retirement/revision branch is required | H09 lacks an authoritative lifecycle outcome | Escalate the upstream lifecycle gap; do not invent a local state | Accreditation & Compliance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every Compliance Requirement has exactly one accountable Business Owner | H09 | Activation requires accountable ownership | L2-H09 BR-H09-005 |
| Compliance Requirements define mandatory review frequencies | H09 | Review cadence is part of the governed requirement | L2-H09 BR-H09-006 |
| Expired obligations enter review status | H09 | Expiry does not silently terminate the record | L2-H09 BR-H09-007 |
| Compliance records preserve complete audit history | H09 | Every governed change/outcome remains traceable | L2-H09 BR-H09-008 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Compliance Requirement | Created | Classify and assign accountable owner | Assigned | H09 | L2-H09 H09-A03 |
| Compliance Requirement | Assigned | Make the governed requirement effective | Active | H09 | L2-H09 H09-A03; H09-03 |
| Compliance Requirement | Active | Perform scheduled compliance review | Under Review | H09 | L2-H09 H09-A03 |
| Compliance Requirement | Under Review | Confirm source-supported compliance | Compliant | H09 | L2-H09 H09-A03 |
| Compliance Requirement | Compliant / governed expiry point | Apply expiry governance | Expired → Archived where applicable, or renewed under H09 workflow | H09 | L2-H09 H09-A03; H09-03 |

No canonical `Waived`, `Retired`, or `Revised` state is defined by H09.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Compliance Requirement Created | Compliance Requirement aggregate / H09 | Compliance governance consumers | Records requirement creation | L2-H09 §11 |
| Compliance Requirement Assigned | Compliance Requirement aggregate / H09 | Obligation owner and governance consumers | Records accountable assignment | L2-H09 §11 |
| Compliance Review Due | Compliance Requirement aggregate / H09 | Compliance governance participants | Records due review | L2-H09 §11 |
| Compliance Status Updated | Compliance Requirement aggregate / H09 | Compliance governance consumers | Records current compliance status | L2-H09 §11 |
| Compliance Breach Identified | Compliance Requirement aggregate / H09 | CAPA/compliance governance | Records breach | L2-H09 §11 |
| Compliance Restored | Compliance Requirement aggregate / H09 | Compliance governance consumers | Records restored compliance | L2-H09 §11 |
| Compliance Requirement Renewed | Compliance Requirement aggregate / H09 | Compliance governance consumers | Records renewal | L2-H09 §11 |

No waiver, retirement, or revision event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The obligation-owning domain supplies current status and authoritative evidence under its own business governance. H09 records compliance governance outcomes without taking ownership of the operational obligation itself.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The Compliance Cell owns the compliance register and review governance. Each requirement must have one source-defined Business Owner. Compliance exception approval sits with the compliance governance chain before institutional escalation, but H09 defines no waiver lifecycle/action to apply such an exception to the frozen `waived` branch.

## 15.2 Separation of Duties, Overrides and Exceptions

An obligation-owning domain remains responsible for the underlying business activity. H09 cannot use a compliance status change to rewrite the source-domain fact.

## 15.3 Business Audit and Evidence Obligations

Preserve requirement basis, classification, applicability, Business Owner, review schedule, renewal rules, evidence requirements, status changes, breaches, restoration, renewal, expiry, and archival history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The supported H09 lifecycle branches can be recorded through assignment, activation, review, compliance, expiry, renewal, and archival. The full frozen outcome cannot be completed until waiver, retirement, and revision semantics are defined upstream.

## 16.2 Postconditions

The Compliance Requirement history remains governed and traceable; no unsupported waiver, retirement, or revision result is asserted.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

- Frozen semantics require the requirement to support `revised`, `fulfilled`, `waived`, `expired`, or `retired` outcomes.
- H09-A03 defines `Created → Assigned → Active → Under Review → Compliant → Expired → Archived`.
- H09-03 additionally supports renewal, monitoring, and review but defines no waiver or retirement path.
- H09 §11 defines creation, assignment, review-due, status-update, breach, restoration, and renewal events; no waiver, retirement, or explicit revision event exists.
- `Compliant` may support a fulfilled compliance position, but L3 cannot use it to manufacture the missing waiver/retirement/revision semantics.

These gaps materially affect frozen lifecycle outcomes.

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

Upstream H09 architecture must define canonical revision, waiver, and retirement semantics for Compliance Requirement and clarify how `fulfilled` maps to the owned lifecycle.
