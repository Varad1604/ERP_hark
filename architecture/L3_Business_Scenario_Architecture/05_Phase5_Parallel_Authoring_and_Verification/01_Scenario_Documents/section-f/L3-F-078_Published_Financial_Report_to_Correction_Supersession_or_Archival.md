# Published Financial Report to Correction, Supersession, or Archival

# 1. Scenario Metadata

**Scenario ID:** L3-F-078  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Financial Reporting & Analytics  
**Participating Domains:** F  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-F13 — Financial Reporting & Analytics Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Financial reporting governance, audit, historical reconstruction and downstream financial-report consumers

---

# 2. Scenario Purpose

Govern a published Financial Report when an error, superseding publication or retention condition requires correction, supersession, permitted withdrawal or archival, while preserving the immutable original, its approval context and complete version lineage.

---

# 3. Business Outcome

A corrected successor, supersession, withdrawal where permitted, or archival preserves full version and approval lineage

Locked L2-F13 defines publication immutability, successor-version correction and archival, but it does not define the correction, supersession or withdrawal workflows, states, commands, events, publishers or approval authorities needed to implement the complete frozen outcome. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a report is erroneous, superseded, or reaches retention maturity.

## 4.2 End Boundary

The intended end boundary is reached when one governed disposition has completed:

- a corrected successor report version has been validated and published while the original remains immutable;
- a successor publication has formally superseded the prior published version;
- a withdrawal permitted by authoritative policy has been recorded without erasing the original; or
- the report has transitioned from `Published` to `Archived` and remains available for audit;
- full version, approval, correction and disposition lineage has been preserved; and
- applicable canonical events have been published by their sole owners.

Only the archival outcome and the general successor-version principle are fully defined upstream.

## 4.3 In Scope

- Authorization and classification of the disposition requirement.
- Preservation of the immutable published report.
- Evaluation of correction, supersession, permitted withdrawal or archival.
- Creation of a new report version where correction requires changed content.
- Validation and publication of a successor version using the established report lifecycle.
- `Published → Archived` where archival is selected.
- `FinancialReportArchived` for successful archival.
- Preservation of report, definition, template, execution, approval and disposition lineage.
- Audit evidence for the complete governance action.

## 4.4 Out of Scope

- Editing a Published report in place.
- Changing canonical operational Finance facts through reporting.
- Reopening or correcting a closed Financial Period; that remains owned by Financial Period and source-domain workflows.
- Inventing `Corrected`, `Superseded` or `Withdrawn` Financial Report states.
- Inventing correction, supersession or withdrawal commands or events.
- Inventing approval authorities or withdrawal permissions.
- Technical storage deletion, file replacement, database mutation or retention-job design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Upstream | L3-F-073 — Closed Financial Information to Approved Financial Statements | Produces governed statements that may later require disposition. |
| Upstream | L3-F-075 — Management Reporting Request to Governed Financial Report | Produces governed management reports. |
| Related | L3-F-079 — Financial Evidence to Audit and Historical Reconstruction | Uses preserved original, successor and archival lineage. |
| Upstream control | L2-F09 — Financial Period and Closing | Owns period reopening or source correction; F078 cannot alter closed facts itself. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Financial Reporting & Analytics / Section F | Primary owner. Owns Financial Report publication, immutability, successor versions, archival and reporting lineage. |
| Applicable Finance source domains / Section F | Retain ownership of canonical financial facts; correct facts only through their own governed workflows. |
| Authorization & Policy / Section A | Supplies centralized authorization and policy decisions for protected disposition actions. |
| Event Infrastructure / Section A | Distributes canonical events without owning report disposition facts. |
| Audit Infrastructure / Section A | Preserves immutable evidence and retention history. |

The frozen participating-domain value remains exactly `F`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authorized Report Governance Requester | Raises a documented error, supersession, withdrawal or retention-maturity condition. | May initiate review; disposition authority is not implied. |
| Financial Reporting & Analytics | Assesses the condition, preserves the original and executes only an upstream-defined disposition. | Owns Financial Report lifecycle outcomes. |
| Audit and Records Governance | Requires preservation and retrieval of report lineage. | Does not alter financial facts or approve a disposition unless separately authorized. |

L2-F13 defines no canonical human actors or approval authorities for correction, supersession, withdrawal or archival. The labels above identify responsibilities only.

---

# 7. Preconditions and Entry Criteria

- A governed Financial Report exists and is `Published`.
- The published report, definition version, template version, execution context, source references and validation evidence are preserved.
- A documented error, successor publication, permitted-withdrawal consideration or retention-maturity condition exists.
- The Tenant, report identity, version, reporting period and audience can be resolved.
- Applicable authorization, retention, disclosure and approval policies can be evaluated.
- The original published report will not be edited or deleted in place.
- Where correction of source facts is required, those facts have been or will be corrected through their owning Finance workflows.
- Audit and correlation context are available.

---

# 8. Trigger

A report is erroneous, superseded, or reaches retention maturity

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive and classify the report-governance condition as error, supersession, possible withdrawal or retention maturity. | Financial Reporting & Analytics | The required disposition category is identified. |
| 2 | Resolve the Tenant, report identity, published version, period, audience and correlation context. | Financial Reporting & Analytics | The exact immutable report subject is established. |
| 3 | Authorize access to the report and the proposed governance action. | Authorization & Policy; Financial Reporting & Analytics | Unauthorized disposition is prevented. |
| 4 | Preserve the existing Published report and all definition, template, execution, validation and publication evidence without modification. | Financial Reporting & Analytics; Audit Infrastructure | The authoritative historical version remains intact. |
| 5 | Determine whether the condition originates in report generation, a changed canonical source fact, a successor publication, a withdrawal permission or retention maturity. | Financial Reporting & Analytics; applicable Finance source domain | The responsible correction or disposition boundary is identified. |
| 6 | If corrected content is required, obtain corrected canonical facts through the owning Finance domain and create a new Financial Report version. | Finance source domain; Financial Reporting & Analytics | A distinct successor candidate is produced without editing the original. |
| 7 | Process a successor candidate through the defined `Generating → Generated → Validated → Published` lifecycle. | Financial Reporting & Analytics | A corrected or successor report may become Published under the existing lifecycle. |
| 8 | Establish explicit lineage between the prior published report and the successor candidate. | Financial Reporting & Analytics | Original and successor versions are historically related. |
| 9 | Record the intended correction, supersession or permitted-withdrawal disposition. | Financial Reporting & Analytics | **Blocked:** canonical disposition states, commands and events are not defined by L2-F13. |
| 10 | If retention maturity requires archival, validate archival eligibility and issue the upstream-defined `Archive Report` command. | Financial Reporting & Analytics | The supported archival transition begins. |
| 11 | Complete `Published → Archived` and publish `FinancialReportArchived`. | Financial Reporting & Analytics — Financial Report; Event Infrastructure | The report is Archived and remains accessible for audit. |
| 12 | Preserve the report-version, approval, correction, disposition and archival lineage. | Financial Reporting & Analytics; Audit Infrastructure | Historical reconstruction remains possible. |
| 13 | Notify authorized consumers of the governed disposition using only canonical contracts. | Financial Reporting & Analytics | **Blocked for correction, supersession and withdrawal:** no canonical events or publishers are defined. |

Steps 9 and 13 prevent complete canonical specification of three frozen disposition paths.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Report-generation correction | The error arose from report definition, template, execution or calculation rather than source facts. | Correct the governed reporting input and generate a distinct successor report version. | The original remains immutable; successor semantics remain incomplete upstream. | Financial Reporting & Analytics |
| Source-fact correction | The report reflects a canonical source fact later corrected through its owning workflow. | Consume the corrected fact and create a new report version under the report lifecycle. | Both report versions remain traceable to their source states. | Finance source domain; Financial Reporting & Analytics |
| Supersession | A newer report publication replaces the prior version for future use. | Link the prior and successor versions and preserve both. | **Blocked:** no canonical supersession state, command, event or publisher exists. | Financial Reporting & Analytics |
| Withdrawal where permitted | Authoritative policy permits a published report to be withdrawn from current use. | Preserve the immutable report and record its governed availability change. | **Blocked:** permission rules and withdrawal lifecycle are undefined. | Financial Reporting & Analytics |
| Archival | The Published report reaches retention maturity and qualifies for archival. | Execute `Archive Report`, transition `Published → Archived`, publish `FinancialReportArchived`. | A canonical archived report remains accessible for audit. | Financial Reporting & Analytics |
| Disposition not approved | The proposed action is not authorized or permitted. | Preserve the Published report and record the denied outcome. | No report lifecycle change occurs. | Authorization & Policy; Financial Reporting & Analytics |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Report identity or version cannot be resolved. | The correct historical subject cannot be governed safely. | Reject the action without changing any report. | Financial Reporting & Analytics |
| Authorization fails. | The requester cannot perform or view the disposition. | Deny the action and preserve audit evidence. | Authorization & Policy |
| Original report or lineage evidence is incomplete. | Correction or disposition cannot preserve required traceability. | Stop progression and restore evidence availability before retrying. | Financial Reporting & Analytics; Audit Infrastructure |
| Corrected source facts are unavailable. | A valid successor cannot be generated. | Keep the original immutable; defer successor generation to the source-owner correction outcome. | Finance source domain |
| Successor generation or validation fails. | No corrected Published successor exists. | Do not publish the candidate; preserve the failed execution evidence. | Financial Reporting & Analytics |
| Correction, supersession or withdrawal contract is required. | The frozen outcome cannot be represented canonically. | Do not invent a state, command, event or publisher; obtain an L2-F13 decision. | Financial Reporting & Analytics |
| Archival eligibility fails. | `Published → Archived` cannot proceed. | Keep the report Published and record the failed review. | Financial Reporting & Analytics |
| Archival event condition is not satisfied. | Consumers must not receive a false archival fact. | Publish `FinancialReportArchived` only after successful archival. | Financial Reporting & Analytics; Event Infrastructure |

No technical rollback, deletion, retry or storage-reclamation mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| A Published Financial Report is immutable. | Financial Reporting & Analytics | The report cannot be edited in place. | L2-F13 — Lifecycle Constraints |
| Corrections require a new report version. | Financial Reporting & Analytics | A changed report must be a distinct successor candidate. | L2-F13 — Lifecycle Constraints |
| The canonical report lifecycle is `Defined → Scheduled → Generating → Generated → Validated → Published → Archived`. | Financial Reporting & Analytics | Only defined states and transitions may be used. | L2-F13 — Report Lifecycle |
| Lifecycle states cannot be skipped. | Financial Reporting & Analytics | A successor must pass generation and validation before publication. | L2-F13 — Lifecycle Constraints |
| Validation is mandatory before publication. | Financial Reporting & Analytics | An unvalidated successor cannot become Published. | L2-F13 — Lifecycle Constraints |
| Archived reports remain accessible for audit. | Financial Reporting & Analytics | Archival cannot erase historical evidence. | L2-F13 — Lifecycle Constraints |
| `Archive Report` is an upstream-defined command. | Financial Reporting & Analytics | Archival has an authoritative business operation. | L2-F13 — Commands |
| `FinancialReportArchived` is published by Financial Report after successful archival. | Financial Reporting & Analytics | The event represents only the completed archival fact. | L2-F13 — Event Catalogue and Aggregate-to-Event Matrix |
| Reports do not become sources of financial truth. | Financial Reporting & Analytics | Corrections to source facts remain owned by canonical Finance domains. | L2-F13 — Foundational Principles |
| Historical results preserve configuration context. | Financial Reporting & Analytics | Original and successor versions retain definition and template lineage. | L2-F13 — Configuration-Driven Behaviour |
| Correction, supersession and withdrawal semantics are not defined. | Financial Reporting & Analytics | Their states, transitions, commands, events and approval rules cannot be inferred. | L2-F13 source gap |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Original Financial Report | `Published` | Remains immutable and historically accessible. | Still preserved in its canonical published history. | Financial Reporting & Analytics |
| Corrected or successor Financial Report | Does not yet exist | Created as a new version and processed through the defined lifecycle. | Intended `Published` successor; supersession semantics undefined. | Financial Reporting & Analytics |
| Archived Financial Report | `Published` and archive-eligible | Executes the defined archival transition. | `Archived`. | Financial Reporting & Analytics |
| Withdrawn report availability | Published report considered for withdrawal | Must change current availability without erasing history. | **Undefined:** no canonical state or transition. | Financial Reporting & Analytics |
| Canonical Finance facts | Existing source-owned state | Corrected only through owning workflows; consumed read-only by reporting. | Unchanged by F078. | Applicable Finance source domain |
| Audit evidence | Existing report evidence | Receives disposition and lineage evidence. | Immutable historical evidence preserved. | Audit Infrastructure |

This Draft does not introduce `Corrected`, `Superseded` or `Withdrawn` states because they are absent from the locked L2-F13 lifecycle.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Required Business Fact | Canonical Resolution | Scenario Treatment |
|---|---|---|
| Successor report generated | `FinancialReportGenerated`, publisher Financial Report | May be used when a distinct successor completes generation. |
| Successor report validated | `FinancialReportValidated`, publisher Financial Report | May be used when the successor passes mandatory validation. |
| Successor report published | `FinancialReportPublished`, publisher Financial Report | May be used when the successor reaches Published. |
| Original report corrected | No correction-specific canonical event or publisher. | No event is invented. |
| Original report superseded | No supersession-specific canonical event or publisher. | No event is invented. |
| Published report withdrawn | No withdrawal-specific canonical event or publisher. | No event is invented. |
| Report archived | `FinancialReportArchived`, publisher Financial Report | Published only after `Published → Archived` succeeds. |

General successor publication events do not by themselves establish correction, supersession or withdrawal meaning.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Published report and lineage | Financial Reporting & Analytics | Disposition review | Immutable report, definition, template, execution, validation and publication context. |
| Corrected canonical facts | Applicable Finance source domain | Financial Reporting & Analytics | Governed source corrections for successor generation. |
| Authorization decision | Authorization & Policy | Financial Reporting & Analytics | Permission and scope outcome for the proposed action. |
| Successor-version link | Financial Reporting & Analytics | Audit and authorized consumers | Relationship between original and successor reports; canonical contract details are absent upstream. |
| Archival outcome | Financial Reporting & Analytics | Audit and authorized consumers | Archived report and `FinancialReportArchived` fact. |
| Audit input | Financial Reporting & Analytics | Audit Infrastructure | Original, successor, approval, disposition, event and correlation evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Viewing or governing a report requires authorization for its Tenant, reporting scope and confidentiality classification.
- A correction request does not authorize mutation of canonical source facts.
- Withdrawal may occur only where an authoritative policy permits it; no such permission rule is specified by L2-F13.
- Archived availability remains protected.

## 15.2 Approval and Separation of Duty

- L2-F13 supports configurable approval policy and treats validation as mandatory before publication.
- It does not name correction, supersession, withdrawal or archival approvers.
- It does not define a separate successor-approval transition or event.
- No actor, approval authority or SoD exception is invented.

## 15.3 Audit Obligations

The following must be auditable:

- Tenant, report identity, version and period.
- Triggering condition and evidence.
- Original definition, template, execution, validation and publication context.
- Authorization and any applicable approval outcome.
- Source-fact correction references where applicable.
- Successor generation, validation and publication.
- Original-to-successor lineage.
- Correction, supersession, withdrawal or archival disposition.
- Canonical event publications.
- Actor, timestamp, correlation and final outcome.

---

# 16. Completion Criteria and Postconditions

The intended completion criteria are:

- the exact Published report and triggering condition were resolved;
- the action was authorized;
- the original report remained immutable;
- applicable source corrections were owned outside reporting;
- any changed report content was produced as a distinct successor version;
- the successor passed the defined generation, validation and publication lifecycle;
- original-to-successor and approval lineage was preserved;
- a correction, supersession or permitted-withdrawal disposition was recorded canonically; or
- archival completed through `Published → Archived` and `FinancialReportArchived` was published;
- immutable audit evidence was preserved.

The archival path can meet these criteria. The complete frozen outcome cannot be validated because correction, supersession and withdrawal lack canonical lifecycle and event contracts.

Postconditions:

- No Published report has been edited or deleted in place.
- Canonical Finance facts remain owned by their source domains.
- Archived reports remain available for authorized audit access.
- Undefined disposition semantics are not represented by invented states or events.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Precise Scenario Impact | Required Resolution |
|---|---|---|---|---|
| F078-BLOCKER-01 | Blocking | L2-F13 states that corrections require a new report version but defines no correction workflow, command, state, transition or correction-specific event. | A successor can be generated and published, but its canonical correction meaning and relationship to the original cannot be completed exactly. | Define the correction lifecycle, commands, lineage contract, events and sole publishers. |
| F078-BLOCKER-02 | Blocking | L2-F13 defines no supersession lifecycle, state, command or event. | The frozen supersession outcome cannot distinguish an ordinary new publication from a governed replacement. | Define supersession eligibility, transition, lineage and canonical event. |
| F078-BLOCKER-03 | Blocking | L2-F13 defines no withdrawal permission rules, lifecycle, command, state or event. | “Withdrawal where permitted” cannot be executed or audited canonically. | Define authoritative withdrawal policy, actors, states, commands and events. |
| F078-BLOCKER-04 | Blocking | L2-F13 does not name disposition requestors, approvers or approval authorities. | Approval lineage required by the frozen outcome cannot be assigned to authoritative roles. | Define canonical actors and approval contracts for each disposition. |
| F078-ISSUE-01 | Non-blocking | Archival is defined by `Archive Report`, `Published → Archived` and `FinancialReportArchived`, but archive eligibility and approval detail are not specified. | The supported archive path can be represented, but its policy decision remains high level. | Define archive eligibility and approval policy before L4. |

**Source Assessment:** BLOCKED

No unsupported actor, approval authority, disposition state, transition, event, publisher, withdrawal permission or technical mechanism is assumed by this Draft.
