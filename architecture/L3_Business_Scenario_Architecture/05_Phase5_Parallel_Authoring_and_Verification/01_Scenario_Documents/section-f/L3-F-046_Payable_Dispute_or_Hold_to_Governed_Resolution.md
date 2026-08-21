# Payable Dispute or Hold to Governed Resolution

# 1. Scenario Metadata

**Scenario ID:** L3-F-046  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Accounts Payable  
**Participating Domains:** F, J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-F00_Finance_Domain_Model_and_Foundation.md`; `L2-F05_Accounting_and_General_Ledger_Management.md`; `L2-F08_Accounts_Payable_Management.md`; `L2-F10_Treasury_and_Banking.md`; `L2-F14_Procurement_Finance_Consumption_Workflows.md`; relevant locked Section J source architecture; frozen Phase 4 Stage 3 Section F canonical scenario register  
**Referenced By:** None yet  

---
# 2. Scenario Purpose

This scenario governs containment, investigation, and resolution of a dispute or institutional hold affecting an existing payable. It prevents payment-readiness, settlement, or closure progression while the issue remains active, preserves the liability and all source evidence, and restores or redirects the Payable only through an authoritative governed outcome.

---

# 3. Business Outcome

The payable dispute or hold has a complete and auditable governed resolution: the Payable is validly released to operational processing, remains suspended under a continuing authorized hold, or is directed to an authoritative source correction, adjustment, reversal, cancellation, settlement, or write-off path without losing historical liability evidence.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an active Payable is subject to a vendor dispute, legal dispute, compliance investigation, documentation deficiency, audit hold, management hold, institutional-policy hold, or another authoritatively governed condition that may prevent progression.

## 4.2 End Boundary

The scenario ends when the dispute or hold decision, evidence, authority, and Payable consequence are recorded and the Payable either resumes an allowed operational position, remains suspended with a governed review condition, or transitions to the separately owned follow-on scenario required by the resolution.

## 4.3 In Scope

- Identification and validation of the affected Payable and dispute/hold basis.
- Preservation of liability, vendor, invoice, procurement, contract, receipt, approval, readiness, settlement, Accounting, and audit evidence.
- Temporary suspension of Payable progression where the hold condition is valid.
- Blocking or changing Payment Readiness and preventing settlement or closure as applicable.
- Cross-domain investigation by the canonical owner of the disputed fact.
- Recording of resolution decision, reasons, authority, effective boundary, and supporting evidence.
- Release/resumption, continued suspension, or governed hand-off to correction, adjustment, reversal, cancellation, write-off, or other applicable scenario.
- Publication of supported Payable state/readiness facts.

## 4.4 Out of Scope

- Destructive deletion or rewriting of the Payable, obligation, invoice, procurement, settlement, or Accounting history.
- Accounts Payable modification of vendor, procurement, contract, goods receipt, invoice, legal, audit, or Accounting aggregates.
- Treasury execution while the Payable is suspended or not ready.
- Performing the financial adjustment, reversal, cancellation, settlement, write-off, or legal proceeding selected by the resolution.
- Vendor communication procedure, evidence-exchange mechanism, or legal case-management design not defined upstream.
- Lower-level enforcement, notification, transport, or records-storage design.

## 4.5 Related Scenarios - Optional

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | `L3-F-044` Vendor Invoice to Approved Payable Liability | Establishes the Payable and source evidence that may later be disputed or held. |
| Related | `L3-F-045` Approved Payable to Financial Settlement | Settlement must not proceed while a valid dispute or hold blocks readiness. |
| Followed by | `L3-F-047` Payable Change to Approved Adjustment or Reversal | A resolution may require an authoritative Payable change. |
| Related | `L3-F-048` Unsettled Payable to Suspension Reopening or Write-Off | Governs broader exceptional lifecycle outcomes for unsettled Payables. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Accounts Payable / Section F | Owns the Payable, suspension, Payment Readiness, outstanding liability, due schedule, lifecycle state, aggregate version, dispute/hold reference, and final Payable resolution consequence. |
| Originating obligation owner | Owns the underlying obligation and any authoritative correction, cancellation, acceptance, or rejection fact affecting it. |
| Procurement and Resource Management / Section J | Owns supplier, invoice, Purchase Order, contract, receipt, inspection, acceptance, and procurement-dispute facts. |
| Enterprise Governance | Owns applicable hold/release policies, approval rules, decision authority, escalation, and immutable governance outcomes. |
| Legal and Compliance | Owns legal-dispute, statutory, regulatory, investigation, or compliance conclusions within its canonical boundary. |
| Audit | Owns audit examination and findings without directly changing the Payable. |
| Treasury & Banking / Section F | Owns payment and settlement execution and must respect Payable readiness/suspension facts. |
| Accounting & General Ledger / Section F | Owns Journals, Ledger effects, and Accounting corrections arising from the resolution. |

No participant may directly modify the `Payable` except Accounts Payable through validated aggregate behaviour.

---

# 6. Business Actors, Roles and Responsibilities

`L2-F08` defines no canonical actors for raising, reviewing, approving, releasing, continuing, escalating, or resolving a dispute or hold. It lists reasons as configurable concepts but does not define role authority. This Draft therefore assigns responsibilities to canonical domains and records the missing actor and decision model in Section 17.

---

# 7. Preconditions and Entry Criteria

- A valid Accounts Payable-owned `Payable` exists with immutable identity and complete originating-obligation traceability.
- The Payable has a valid outstanding-liability, due-schedule, readiness, lifecycle, and version position.
- A dispute or hold assertion identifies the affected Payable, source fact, reason, evidence, requesting/deciding authority where defined, and effective business boundary.
- The asserted reason belongs to an authoritative owner or institutionally configured suspension category.
- Relevant invoice, procurement, contract, supplier, settlement, adjustment, Accounting, legal/compliance, and audit evidence is available to the extent applicable.
- No final resolution has already been incorporated for the same dispute/hold evidence.

---

# 8. Trigger

Accounts Payable receives an authoritative dispute or hold fact indicating that further Payable progression requires suspension or governed review.

Illustrative source-defined suspension reasons include legal dispute, vendor dispute, compliance investigation, documentation pending, audit hold, management hold, and institutional policy.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Payable, outstanding liability, source obligation, due schedule, current readiness/lifecycle, and the disputed or held business fact. | Accounts Payable | A bounded dispute/hold case is linked to the correct aggregate. |
| 2 | Validate the reason, evidence, source ownership, authority, effective boundary, and whether immediate suspension is required. | Accounts Payable with the canonical issue owner | The case is confirmed as actionable or returned for clarification. |
| 3 | Record the immutable dispute/hold reference and preserve all current Payable, source, settlement, approval, and Accounting evidence. | Accounts Payable | Historical and financial traceability is protected before state change. |
| 4 | Suspend the Payable where required, advance aggregate version, and change Payment Readiness to a non-executable position. | Accounts Payable | Further readiness, settlement, and closure progression is stopped without deleting liability history. |
| 5 | Publish `PayableSuspended` and `PaymentReadinessChanged` after aggregate and policy validation. | Accounts Payable | Treasury and other governed consumers receive the hold fact. |
| 6 | Investigate the disputed obligation, invoice, procurement, vendor, contract, documentation, legal/compliance, audit, payment, or Accounting facts under their canonical owners. | Affected source owners and Enterprise Governance | The institution develops an authoritative resolution basis without cross-domain mutation. |
| 7 | Obtain a governed resolution decision with reason, authority, effective boundary, evidence, and required follow-on action. | Enterprise Governance and canonical issue owner | A valid release, continuation, correction, adjustment, reversal, cancellation, write-off, or other outcome is established. |
| 8 | Validate the resolution against the Payable's current state, outstanding amount, source references, policies, and aggregate invariants. | Accounts Payable | The decision is eligible for incorporation exactly once. |
| 9 | Apply the Payable-owned consequence: release/resume operational processing, maintain suspension with a governed review condition, or retain suspension pending the separately owned follow-on outcome. | Accounts Payable | The Payable reflects the authoritative resolution without performing another domain's action. |
| 10 | Publish supported readiness and lifecycle events and retain complete case and decision evidence. | Accounts Payable | The governed resolution is visible, auditable, and safe for subsequent processing. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Dispute resolved with no financial change | Evidence confirms the original liability remains valid and all hold conditions are satisfied. | Record the decision, release the hold, re-evaluate readiness and due schedule, and resume the Payable's valid operational path. | The liability remains unchanged and may become Payment Ready. | Accounts Payable and decision owner |
| Hold continues | Investigation is incomplete or the governing condition remains active. | Retain `Suspended`, record the continuing reason and authority, and define the next governed review boundary where available. | Payment and closure remain blocked. | Accounts Payable and decision owner |
| Documentation supplied | A documentation-pending hold is cured by authoritative evidence. | Validate the new documentation under its owner, record the resolution, and re-evaluate validation/readiness. | The Payable resumes if no other block remains. | Source owner and Accounts Payable |
| Liability correction required | The dispute proves that amount, classification, schedule, obligation, or source references require change. | Keep the Payable suspended and route the authoritative correction through `L3-F-047`. | No direct or destructive update occurs in this scenario. | Source owner, Accounts Payable, and Financial Adjustment owner where applicable |
| Liability rejected or cancelled | The source owner determines that the underlying obligation is invalid or cancelled. | Preserve the decision and route the Payable to the source-defined cancellation, adjustment, reversal, write-off, or closure path. | Ordinary settlement remains blocked pending authoritative financial effect. | Originating obligation owner and Accounts Payable |
| Partial dispute | Only part of the Payable amount is disputed and institutional policy permits separation. | Preserve the disputed amount and evidence, and release only the undisputed amount if an authoritative partial-readiness rule and approval permit it. | Settlement eligibility is limited to the governed undisputed amount. | Accounts Payable and Enterprise Governance |
| Legal/compliance or audit outcome | A legal decision, compliance conclusion, or audit correction resolves the case. | Consume the authoritative outcome and apply only the permitted Payable consequence; route financial or Accounting changes to their owners. | The Payable resumes, remains suspended, or enters a governed follow-on path. | Legal/Compliance or Audit, Enterprise Governance, and Accounts Payable |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Dispute/hold lacks authoritative source, reason, evidence, or Payable reference. | Accounts Payable cannot safely change lifecycle or readiness. | Retain the last valid Payable position, seek clarification from the claimed owner, and do not publish a suspension outcome until validated. | Accounts Payable and claimed source owner |
| Treasury execution is attempted after suspension or blocked readiness. | Cash movement could occur against a disputed liability. | Reject or stop progression where still possible, preserve the attempted exception, and coordinate any already-started outcome with Treasury's governed failure/uncertainty process. | Accounts Payable and Treasury & Banking |
| Conflicting authorities issue hold and release outcomes. | The Payable has no deterministic valid resolution. | Maintain the protective suspended position and escalate the authority conflict to Enterprise Governance; do not choose locally. | Enterprise Governance and Accounts Payable |
| Resolution evidence conflicts with current outstanding liability or source obligation. | Applying the decision could compromise financial integrity. | Keep the Payable suspended and reconcile the source-owned facts before any release or financial change. | Accounts Payable and affected source owner |
| Hold expires or review boundary passes without decision. | The Payable may remain blocked without valid continuing authority. | Preserve suspension until the canonical policy owner supplies the required continuation or release decision; no automatic release rule is invented. | Enterprise Governance and Accounts Payable |
| Resolution requires a financial change but the authoritative adjustment/reversal is incomplete. | Releasing the Payable prematurely would expose an incorrect liability. | Maintain suspension until the separately owned financial fact is completed and consumed. | Financial Adjustment/source owner and Accounts Payable |
| Aggregate validation fails when applying resolution. | No valid Payable resolution state exists. | Preserve the prior suspended position, do not publish events, and resolve the inconsistent decision or evidence. | Accounts Payable and decision owner |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Suspension temporarily prevents further Payable progression, is reversible, preserves financial integrity, and never destroys business history. | Accounts Payable | A valid dispute/hold changes operational progression without deleting or extinguishing the liability. | `L2-F08`, Capability 8 |
| Illustrative suspension reasons include legal dispute, vendor dispute, compliance investigation, documentation pending, audit hold, management hold, and institutional policy. | Institutional policy owners and Accounts Payable | The reason must be governed and traceable; exact categories remain configurable. | `L2-F08`, Capability 8 |
| Suspension may prevent payment readiness, settlement progression, and closure. | Accounts Payable | Treasury execution and Payable completion cannot proceed while the applicable block remains. | `L2-F08`, Capability 8 |
| Suspended Payables cannot become Payment Ready. | Accounts Payable | Release/resumption must occur before readiness can return to an executable position. | `L2-F08`, Aggregate Invariants |
| Payment Readiness evaluates dispute and suspension status and never initiates or authorizes Treasury execution. | Accounts Payable and Treasury & Banking | Any active dispute/hold must be reflected in readiness and respected by Treasury. | `L2-F08`, Capability 5 |
| External domains influence Payable state only through published business events and never directly modify the aggregate. | Accounts Payable and source owners | Investigators and decision owners publish facts; Accounts Payable applies its own consequence. | `L2-F08`, Sections 5, 6 and 13 |
| Corrections occur through new compensating business events rather than destructive updates. | Canonical source owner | Resolution history supplements and never overwrites the original liability or dispute evidence. | `L2-F08`, Architectural Principles and Event Architecture |
| Aggregate consistency, financial validity, version advancement, due schedule, readiness, and policy validation precede state transition/event publication. | Accounts Payable | No incomplete or invalid hold/resolution becomes observable as a completed Payable fact. | `L2-F08`, Sections 9, 12 and 13 |
| Treasury consumes approved obligations and readiness but never determines payment approval or owns the Payable. | Treasury & Banking and Accounts Payable | Treasury must respect the current Payable hold/readiness fact. | `L2-F10`, Accounts Payable integration |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| `Payable` | `Registered`, `Validated`, `Payment Ready`, `Partially Settled`, or another active source-permitted position | A valid dispute/hold temporarily blocks progression while preserving identity, liability, references, and history. | `Suspended`; release/resumption target is unresolved between `Reopened`, `Validated`, or another policy-defined operational position | Accounts Payable | `L2-F08`, Capability 8, Aggregate State Model and Invariants |
| Payment Readiness | `Ready`, `Pending`, `Blocked`, or another configured position | Re-evaluates dispute, suspension, documentation, approval, and policy conditions. | `Suspended`, `Blocked`, `Awaiting Documentation`, or another configured non-executable outcome | Accounts Payable | `L2-F08`, Capability 5 |
| Due Schedule | One active valid schedule | Remains historically preserved; may require a separately governed replacement after resolution. | Unchanged or replaced under Due Schedule rules | Accounts Payable | `L2-F08`, Capability 4 |
| Source obligation/procurement/legal/audit fact | Source-defined current position | Supplies dispute/hold and resolution evidence but is not modified by Accounts Payable. | Source-owned outcome | Canonical source owner | `L2-F08`, upstream relationships; `L2-F14` where applicable |
| Treasury Payment/Settlement | Not started, scheduled, or otherwise affected by readiness | Must not progress to approved execution/settlement while the valid block applies. | Withheld, cancelled, failed, or separately resolved under Treasury governance where already initiated | Treasury & Banking | `L2-F10` |

The exact source state for suspension, release target, and meaning of `Reopened` are internally inconsistent or incomplete in `L2-F08` and remain recorded in Section 17.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| Authoritative dispute/hold fact | Procurement, obligation owner, Legal/Compliance, Audit, Enterprise Governance, or other canonical owner | Accounts Payable | Supplies the governed basis for suspension; exact event names are not defined. | `L2-F08`, Capability 8 and upstream relationships |
| `PayableSuspended` | Accounts Payable | Treasury, Accounting, Reporting, Audit, dashboards, Notification Services | Announces that further Payable progression has temporarily stopped. | `L2-F08`, Section 13 |
| `PaymentReadinessChanged` | Accounts Payable | Treasury and governed Finance consumers | Communicates the resulting non-executable or restored readiness position. | `L2-F08`, Section 13 |
| Authoritative dispute/hold resolution fact | Canonical issue owner or Enterprise Governance | Accounts Payable | Supplies the immutable release, continuation, correction, cancellation, or other resolution decision. | Required by `L2-F08` event-driven boundary; exact catalogue missing |
| `PayableReopened` | Accounts Payable | Treasury, Accounting, Reporting, Audit, dashboards | Potentially announces resumed operation after a valid policy decision, subject to the lifecycle inconsistency. | `L2-F08`, Capability 9 and Section 13 |
| `DueScheduleUpdated` | Accounts Payable | Treasury, Accounting, Reporting, Audit | Communicates a valid replacement schedule where resolution changes payment timing. | `L2-F08`, Section 13 |

No PayableDisputed, PayableHoldApplied, HoldReleased, DisputeResolved, HoldContinued, or ResolutionRejected events are defined in `L2-F08`.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Source obligation/Procurement/Legal/Compliance/Audit/Governance owner | Accounts Payable | Dispute or hold fact with Payable/source identity, reason, evidence, authority, effective boundary, and requested protective effect | Enables Accounts Payable to validate and apply suspension without transferring source ownership. | `L2-F08`, Capabilities 5 and 8; upstream event principles |
| Accounts Payable | Treasury & Banking | `PayableSuspended` and changed readiness with liability, due-schedule, and reference context | Prevents ineligible execution while Treasury retains payment/settlement ownership. | `L2-F08`, Sections 10, 13 and 14; `L2-F10`, Accounts Payable integration |
| Canonical issue owner / Enterprise Governance | Accounts Payable | Immutable resolution outcome and required follow-on action | Permits release, continued suspension, or governed redirection. | `L2-F08`, event-driven collaboration; exact contract missing |
| Accounts Payable | Accounting, Reporting, Audit, dashboards | Current Payable lifecycle/readiness and preserved dispute/hold resolution evidence | Supports financial visibility and audit without cross-domain modification. | `L2-F08`, downstream relationships |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Raise dispute or hold | No canonical actor defined | Must reference a valid Payable/source fact and provide an institutionally permitted reason and evidence. | Canonical issue owner / institutional policy owner |
| Apply Payable suspension | No canonical Accounts Payable actor defined | Requires validated authority, reason, evidence, policy, and aggregate consistency. | Accounts Payable |
| Investigate source facts | No canonical actor defined | Each domain investigates only its owned aggregate or business conclusion. | Relevant source owner |
| Approve release, continuation, escalation, or follow-on action | No canonical role, matrix, hierarchy, or threshold defined | Must produce an immutable, traceable governed decision. | Enterprise Governance and canonical issue owner |
| Resume Payable/readiness | No canonical Accounts Payable actor defined | Permitted only after all blocking conditions are authoritatively resolved and invariants succeed. | Accounts Payable |

## 15.2 Separation of Duties, Overrides and Exceptions

| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| Hold requester versus resolution authority | Requesting source and governed decision authority | A requester cannot silently release its own hold unless the upstream policy explicitly grants that authority. |
| Investigation versus Payable state | Source investigators and Accounts Payable | Investigators provide facts/decisions; Accounts Payable alone changes Payable lifecycle/readiness. |
| Payable release versus payment execution | Accounts Payable, Authorization & Policy, and Treasury | Release/readiness does not itself authorize or execute settlement. |
| Financial correction versus dispute resolution | Source/Adjustment owner and Accounts Payable | Resolution identifies the need; the canonical owner completes the financial change before Accounts Payable incorporates it. |
| Accounting correction versus Payable resolution | Accounting & General Ledger and Accounts Payable | Accounting correction cannot directly modify Payable state, and Payable resolution cannot rewrite Journals. |

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Prove case basis | Payable/source identities, reason, category, evidence, authority, effective boundary, and current financial position | Requesting owner and Accounts Payable |
| Prove protective action | Prior lifecycle/readiness, suspension validation, resulting state, aggregate version, and `PayableSuspended`/`PaymentReadinessChanged` | Accounts Payable |
| Prove investigation | Source documents, findings, communications/evidence where governed, legal/compliance/audit conclusions, and exception history | Each canonical issue owner |
| Prove resolution | Decision, reason, authority, approval, effective boundary, release/continuation/follow-on requirement, and immutable reference | Enterprise Governance and canonical issue owner |
| Prove Payable consequence | Resolved source references, resulting lifecycle/readiness, due-schedule effect, version, validation, and published events | Accounts Payable |
| Preserve financial history | Original liability, settlement attempts, holds, decisions, corrections, Accounting references, and later outcomes without overwrite | Each canonical owner |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The affected Payable, originating obligation, current liability, due schedule, readiness, lifecycle, and source evidence are identified.
- The dispute/hold reason, authority, effective boundary, and evidence are validated.
- Required suspension blocks Payment Readiness, settlement progression, and closure as applicable.
- Canonical issue owners complete their investigation without directly modifying the Payable.
- A governed resolution outcome identifies release, continuation, or the required follow-on path.
- Accounts Payable incorporates the outcome exactly once and preserves aggregate consistency, versioning, liability, references, and history.
- Supported suspension, readiness, reopening, or schedule events are published only after successful validation.

## 16.2 Postconditions

- Accounts Payable retains ownership of the Payable and its dispute/hold consequence.
- A released Payable resumes only from an allowed lifecycle/readiness position and remains subject to separate authorization before settlement.
- A continuing hold remains traceable and blocks prohibited progression.
- A financial, source, legal, audit, or Accounting correction is executed only by its canonical owner through the appropriate follow-on scenario.
- Treasury does not treat a suspended or non-ready Payable as eligible for settlement.
- All original liability, dispute, hold, investigation, decision, resolution, event, and audit evidence remains immutable and reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| `L2-F08` permits `Suspended -> Reopened` in the state model, but Capability 9 and `PayableReopened` describe reopening a previously closed Payable. | Sections 9, 13, and 14 cannot lock the canonical release/resumption state or event for a suspended Payable. | Accounts Payable owner and Lead Architect must normalize suspension release versus closed-Payable reopening. |
| No canonical actors, dispute owner, hold requester, investigation role, release authority, escalation authority, approval matrix, hierarchy, threshold, or separation-of-duty policy are defined. | Sections 6 and 15 cannot lock who may initiate, apply, continue, resolve, or release a hold. | Enterprise Governance, Accounts Payable, source domains, and Lead Architect must define business roles and authority. |
| Suspension reasons are illustrative/configurable, but no category definitions, required evidence, severity, effective-date, expiry, review, escalation, or precedence rules are defined. | Sections 7-12 cannot deterministically govern competing or time-bound holds. | Institutional policy and Accounts Payable owners must define the hold/dispute policy model. |
| No canonical dispute or hold aggregate, entity, reference model, status lifecycle, partial-dispute rule, or relationship to Outstanding Liability is defined in `L2-F08`. | The Draft cannot lock whether dispute/hold is Payable state, readiness state, supporting reference, or separately owned case. | Accounts Payable and governance owners must define the canonical dispute/hold business concept upstream. |
| No exact upstream events are defined for vendor dispute, legal dispute, compliance investigation, documentation hold, audit hold, management hold, release, continuation, or resolution. | Section 14 cannot establish a complete canonical event contract. | Source domains and Accounts Payable must define event names, publishers, meanings, and evidence requirements. |
| `L2-F08` defines `PayableSuspended` and `PayableReopened` but no HoldApplied, DisputeRaised, HoldReleased, DisputeResolved, HoldContinued, ResolutionRejected, or partial-release event. | Downstream visibility cannot distinguish reason and resolution outcomes using canonical Payable events. | Accounts Payable owner must define additional events or approve generic readiness/lifecycle publication. |
| Exact treatment of a partially disputed Payable is undefined, including whether undisputed value may become Ready or settle independently. | Section 10 cannot lock partial-release or amount-segmentation behaviour. | Accounts Payable, Treasury, Procurement, and governance owners must define partial-dispute policy. |
| Treasury behavior when suspension arrives after scheduling or execution begins is not fully defined across `L2-F08` and `L2-F10`. | Section 11 cannot determine cancellation, stop, uncertainty, or settlement treatment for in-flight payment activity. | Treasury and Accounts Payable owners must define the coordinated in-flight hold contract. |
| No exact Accounting or Financial Period consequences are defined for dispute provisioning, accrual changes, withheld payment, liability correction, or release. | The scenario cannot lock Accounting sequence or reporting effects. | Accounting, Financial Period, Accounts Payable, and policy owners must define the financial-book contract. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

**End of Scenario L3-F-046**
