# Grant Opportunity to Award and Funding Closure

# 1. Scenario Metadata

**Scenario ID:** L3-H-032  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 Faculty Workforce; applicable locked Finance architecture  
**Referenced By:** Research Project execution, research funding governance, institutional grant reporting, Finance reconciliation, and research audit

# 2. Scenario Purpose

Govern a Research Grant from eligible funding opportunity/application through award, allocation, utilization, reporting, permitted change, and closure while preserving Finance as the authoritative owner of accounting and financial settlement.

# 3. Business Outcome

The award, funding use, reporting, amendment, and closure outcomes are governed and reconciled with Finance

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an eligible funding opportunity or approved grant application initiates grant governance.

## 4.2 End Boundary

The frozen end boundary requires award, funding use, reporting, amendment, and closure outcomes to be governed and reconciled with Finance. H05 defines award/allocation/utilization/closure but leaves material amendment and reconciliation semantics incomplete; the scenario is therefore blocked as recorded in Section 17.

## 4.3 In Scope

- Applying for a Research Grant from an eligible project/proposal context.
- Recording a verified grant award.
- Allocating Research funding within the awarded amount.
- Recording Research-side funding utilization metadata against valid allocations.
- Preserving grant reporting and compliance evidence required for closure.
- Closing the Research Grant after governed closure conditions are satisfied.
- Preserving Finance ownership of accounting, posting, payments, budgets, taxation, and financial compliance.

## 4.4 Out of Scope

- Creating Finance ledger entries, payments, journal entries, or tax calculations.
- Inventing a Grant amendment workflow, state, command, or event not defined in H05.
- Inventing a Finance reconciliation event/contract not defined upstream.
- Research Project lifecycle beyond grant eligibility and completion references.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| People & Workforce / Section E | Owns Principal Investigator eligibility, faculty capability, availability, capacity, and workforce identity referenced in grant eligibility. |
| Finance / Section F | Owns budgets, accounting, ledgers, payments, financial posting, taxation, cost centers, and authoritative financial facts used for grant financial reconciliation. |
| Research / Section H | Owns Research Grant application, award registration, allocation metadata, utilization metadata, grant reporting metadata, grant state, and grant closure. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Principal Investigator | Initiates grant application and may provide Research-side funding utilization updates. | Applies / records Research-side utilization evidence. |
| Research Office | Records grant award, participates in funding allocation, governs utilization metadata, and closes the grant. | Administers Research Grant lifecycle. |
| Finance Officer | Participates in funding allocation as defined by H05 while Finance retains financial ownership. | Provides Finance-side allocation participation. |

# 7. Preconditions and Entry Criteria

- An eligible Research Project exists, or an approved Research Proposal is eligible for grant application according to institutional policy.
- Principal Investigator eligibility is derived from L2-E03 Faculty Workforce.
- Funding scheme/source is active and institution-configured.
- Budget/request information required for application is complete.
- Finance-owned financial facts remain available through governed cross-domain collaboration.

# 8. Trigger

An eligible funding opportunity or approved grant application initiates grant governance

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate grant eligibility, funding scheme, budget context, and investigator eligibility. | Principal Investigator / Research with Sections E and F source facts | Eligible grant governance begins. |
| 2 | Create and submit the Research Grant application. | Principal Investigator / Research | Grant enters `Proposed`; `ResearchGrantApplied` is published. |
| 3 | On verified funding-agency award, record the award and immutable grant identifier. | Research Office / Research | Grant enters `Awarded`; `GrantAwarded` is published. |
| 4 | Allocate approved Research funding within the awarded amount, preserving Finance ownership of financial accounting. | Research Office / Finance Officer / Research and Finance | Grant enters `Active`; `FundingAllocated` is published. |
| 5 | Record Research-side funding utilization metadata against approved allocations and preserve utilization history. | Principal Investigator / Research Office / Research | `FundingUtilizationRecorded` preserves Research utilization evidence. |
| 6 | Complete required grant reporting/compliance obligations and prepare for closure. | Research | Grant becomes eligible for closure when H05 closure validation is satisfied. |
| 7 | Close the Research Grant and preserve closure/reporting evidence. | Research Office / Research | Grant reaches `Closed`; `ResearchGrantClosed` is published. |
| 8 | Stop before claiming complete frozen amendment and Finance reconciliation outcomes where upstream definitions are missing. | Research / Finance | No amendment lifecycle or reconciliation event is invented. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Partial award | Funding agency awards less than requested. | Record the verified award amount and govern allocations within the awarded limit. | Grant proceeds using the authoritative award. | Research |
| Multi-phase allocation | Institution permits phased allocation. | Record governed allocations without exceeding awarded funding. | Grant remains `Active` with preserved allocation history. | Research / Finance |
| Grant obligations incomplete | Reporting or funding obligations remain unresolved. | Reject premature closure and retain the current grant state. | Grant does not become `Closed`. | Research |
| Award amendment required | Award terms/amount require a governed amendment. | H05 mentions an amendment workflow but defines no executable lifecycle. | Path remains unresolved upstream. | Research / Architecture Governance |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Award information cannot be verified | Grant cannot safely enter `Awarded`. | Preserve the existing grant state until valid award evidence exists. | Research |
| Allocation exceeds awarded funding | Funding governance would violate H05. | Reject the allocation and preserve the prior valid state. | Research / Finance |
| Utilization does not reference a valid allocation | Research utilization evidence is invalid. | Reject the utilization record until a valid allocation reference exists. | Research |
| Finance reconciliation is required but no canonical hand-off is defined | Frozen closure outcome cannot be fully proven. | Escalate for upstream contract/event definition; do not invent reconciliation semantics. | Research / Finance / Architecture Governance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Funding source must belong to an institution-configured category. | Research | Only governed funding sources enter the Grant lifecycle. | L2-H05 GR-01 |
| Allocation shall never exceed awarded funding. | Research with Finance financial ownership | Over-allocation is prohibited. | L2-H05 GR-03 |
| Utilization shall reference approved allocations. | Research | Funding use must be traceable to valid allocation. | L2-H05 GR-04 |
| Financial accounting remains owned by Finance. | Finance | Research cannot create accounting/settlement consequences. | L2-H05 GR-05; §§7.5, 27 |
| Grant closure requires completion of institutional reporting obligations. | Research | Reporting/compliance evidence is mandatory before `Closed`. | L2-H05 GR-07 |
| Award amounts remain immutable after award registration unless amended through a governed amendment workflow. | Research | Amendment requires upstream-defined governance that is currently incomplete. | L2-H05 §27.11 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Research Grant | None | Eligible application is submitted. | `Proposed` | Research | L2-H05 §27.6 |
| Research Grant | `Proposed` | Verified award is recorded. | `Awarded` | Research | L2-H05 §27.6 |
| Research Grant | `Awarded` | Valid allocation is recorded. | `Active` | Research | L2-H05 §27.6 |
| Research Grant | `Active` | Utilization metadata is recorded while lifecycle remains active. | `Active` | Research | L2-H05 §27.6 |
| Research Grant | `Active` | Closure validation and reporting obligations complete. | `Closed` | Research | L2-H05 §27.6 |
| Research Grant | Awarded/Active | Frozen scenario requires amendment where applicable, but no canonical amendment transition exists. | Unresolved upstream. | Research | L2-H05 §27.11 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Research publishes exact Grant Aggregate events `ResearchGrantApplied`, `GrantAwarded`, `FundingAllocated`, `FundingUtilizationRecorded`, and `ResearchGrantClosed`.

H05 identifies Finance-domain inputs including `FundingSchemeUpdated` and `BudgetApproved` and retains Finance as owner of financial accounting. No exact canonical Grant-amendment event or Finance-reconciliation completion event is defined.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Research maintains grant and utilization metadata and communicates with Finance using governed cross-domain collaboration while Finance retains authoritative financial accounting. H05 does not define the exact business contract/event needed to certify the frozen “reconciled with Finance” closure outcome.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Principal Investigator applies. Research Office records award and closes the grant. Research Office and Finance Officer participate in allocation as defined by H05.

## 15.2 Separation of Duties, Overrides and Exceptions

No L3 override may permit over-allocation, unsupported utilization, post-closure modification, or a locally invented grant amendment.

## 15.3 Business Audit and Evidence Obligations

Preserve application evidence, funding scheme, award reference/document reference, awarded amount, grant identifier, allocation history and approving authority, utilization references/history, reporting/compliance evidence, closure evidence, and applicable Research Grant events.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The unambiguous H05 path can establish that:

- an eligible Grant application reached `Proposed`;
- a verified award reached `Awarded`;
- valid funding allocation reached `Active`;
- utilization references approved allocation;
- reporting/compliance closure criteria are satisfied;
- the Grant reaches `Closed` with preserved evidence.

The full frozen outcome remains incomplete until amendment and Finance reconciliation semantics are resolved upstream.

## 16.2 Postconditions

- Closed Research Grants are read-only and historically queryable.
- Research retains Grant lifecycle ownership.
- Finance retains accounting and financial settlement ownership.
- The frozen full outcome remains pending architecture resolution for amendment/reconciliation.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome explicitly requires governed **amendment** outcomes. H05 §27.11 states award amounts may be amended through a governed amendment workflow, but the Workflow Catalogue, commands, state matrix, and event catalogue define no Grant amendment workflow/transition/event. | L3 cannot author the required amendment path without inventing lifecycle and event semantics. | Define the canonical Research Grant amendment workflow, allowed states/transitions, authority, rules, and event(s). |
| The frozen outcome requires closure outcomes to be **reconciled with Finance**. H05 preserves Finance ownership and supports financial synchronization, but defines no exact reconciliation completion contract/event or acceptance criterion for Grant closure. | L3 cannot prove the frozen Finance-reconciled end boundary without inventing a mandatory cross-domain hand-off. | Define the canonical Finance reconciliation contract/event and closure criterion for Research Grants. |

**Architecture assessment:** `BLOCKED`

The normal grant application, award, allocation, utilization, reporting, and closure lifecycle is explicit, but the frozen amendment and Finance-reconciliation requirements are materially undefined upstream.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions beyond the open issues above.
