# Approved Procurement Demand to RFP Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-005  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-018 Insufficient Competition to Governed Sourcing Disposition; L3-J-019 Active Sourcing Event to Governed Cancellation; downstream J04 Purchase Order scenarios consuming an approved sourcing outcome  

---
# 2. Scenario Purpose

This scenario governs sourcing through a Request for Proposal when approved procurement demand requires suppliers to propose a solution, approach, methodology, service model, implementation plan, or other technically and commercially evaluated response rather than provide price against a fully predetermined specification alone.

It ensures that the RFP package, supplier participation, proposal receipt, technical and commercial evaluation, clarification, recommendation, approval, and exception paths produce a fair, traceable, and institutionally authorised sourcing outcome. It ends before Purchase Order or contract creation.

---

# 3. Business Outcome

Eligible proposals are evaluated against the pre-approved RFP criteria and an approved sourcing outcome is established for downstream commitment, with the demand, issued RFP, proposal versions, technical and commercial assessments, recommendation, authority, and evidence fully traceable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when J01 receives valid approved procurement demand for which the governed sourcing method is an RFP because the institution requires solution, capability, methodology, quality, implementation, service, and commercial proposals to be evaluated together.

## 4.2 End Boundary

The scenario ends when the selected proposal recommendation is approved and the Sourcing Event reaches Completed with an approved RFP sourcing outcome, or reaches a governed alternative terminal disposition such as No Response, No Suitable Bid, Re-Sourcing Required, or Cancelled.

## 4.3 In Scope

- validating approved-demand authority, scope, and RFP-method eligibility;
- creating and approving the RFP Sourcing Event;
- defining the statement of requirements, proposal instructions, qualification conditions, evaluation criteria, weights, stages, and approval path before issue;
- identifying eligible suppliers and issuing a controlled RFP package;
- managing pre-proposal clarification, conference, presentation, site visit, and RFP amendment where applicable;
- receiving, time-stamping, versioning, closing, and securely governing proposals;
- eligibility, responsiveness, technical, functional, implementation, service, and commercial evaluation;
- recording evaluator declarations, scores, reasons, consensus, deviations, and disqualifications;
- governing clarifications, demonstrations, presentations, negotiations, or best-and-final offers only where authorised;
- preparing and approving the proposal evaluation and supplier recommendation;
- governing insufficient competition, no suitable proposal, return, rejection, cancellation, or re-sourcing;
- publishing the approved sourcing outcome for downstream commitment.

## 4.4 Out of Scope

- procurement-plan or demand creation and approval, owned by J01 planning and J03 requisition governance;
- supplier master, qualification, suspension, or blacklisting, owned by J02;
- quotation or RFQ sourcing, governed by L3-J-004;
- tender sourcing, governed by L3-J-006;
- direct or simplified sourcing, governed by L3-J-007;
- single-source, emergency, rate-contract, or external procurement routes;
- Purchase Order or contract creation, amendment, supplier commitment, or cancellation, owned downstream;
- goods or service receipt, inspection, inventory, asset, accounting, payable, or payment processing;
- technical portal, cryptography, API, schema, scoring-engine, or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-018 Insufficient Competition to Governed Sourcing Disposition | Governs a proposal shortfall against configured competition requirements. |
| Related | L3-J-019 Active Sourcing Event to Governed Cancellation | Governs authorised cancellation before a completed RFP outcome. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns RFP-method execution, Sourcing Event, RFP package, invitations, proposal evidence, evaluation governance, recommendation, approval, and final sourcing outcome. |

J01 consumes approved demand from J03 and supplier identity or eligibility from J02 without changing either aggregate. Requesting and specialist units contribute evaluation expertise, but J01 retains accountability for the sourcing outcome. Finance-owned monetary truth and downstream supplier commitment remain outside this scenario.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Officer | Manages the governed sourcing activity and preserves its evidence. | Performs sourcing. |
| Procurement Manager | Supervises procurement within established authority. | Reviews and supervises. |
| Requesting Unit Representative | Provides requirement and specification context. | Contributes demand context. |
| Technical Evaluator | Evaluates technical compliance within assigned criteria. | Evaluates. |
| Commercial Evaluator | Performs commercial evaluation within assigned criteria. | Evaluates. |
| Committee Member | Participates in a configured committee decision where required. | Participates within mandate. |
| Committee Chair / Convenor | Coordinates the committee decision where required. | Coordinates within mandate. |
| Procurement Recommender | Records the supplier recommendation. | Recommends. |
| Procurement Approver | Approves, rejects or returns the sourcing outcome within established authority. | Decides. |
| Auditor / Reviewer | Reviews procurement evidence and decision traceability. | Reviews evidence. |
# 7. Preconditions and Entry Criteria

- J03 has produced identifiable approved procurement demand; draft, returned, rejected, withdrawn, or superseded demand is not valid normal sourcing authority.
- The requirement is sufficiently defined to state required outcomes and constraints, while allowing suppliers to propose differentiated solutions or approaches.
- Applicable procurement policy authorises the RFP method for the category, value, risk, or complexity, or a method override has explicit authority and justification.
- The RFP scope, deliverables, proposal instructions, mandatory conditions, evaluation stages, criteria, weights, minimum qualifying scores, tie treatment, and approval path are defined before issue.
- Criteria are relevant to the approved demand, objectively assessable to the extent practicable, and collectively capable of identifying the most advantageous governed outcome.
- Supplier identities and eligibility facts can be resolved through J02.
- Evaluation roles, committee composition where required, conflict declarations, confidentiality obligations, and separation controls are established.
- Any budget, estimated-value, procurement-plan, or specialist reference required by policy is traceable.
- No conflicting active sourcing event governs the same demand unless consolidation, splitting, or parallel sourcing is explicitly authorised.

---

# 8. Trigger

Approved procurement demand is accepted by J01 and the configured procurement method requires a Request for Proposal and governed multi-criteria evaluation.

---

# 9. Main Business Flow

1. The Procurement Officer accepts the approved demand and records its authoritative J03 reference.
2. The officer validates demand status, intended outcome, scope, deliverables, constraints, timing, estimated-value reference, category, requesting unit, and applicable policy.
3. The officer confirms RFP as the governed method and records any approved method override.
4. A Sourcing Event is created with a unique identity and links to the demand, requisition, procurement plan where applicable, method, policy, category, and responsible actors.
5. The institution prepares the controlled RFP package, including background, required outcomes, scope, deliverables, constraints, proposal format, mandatory submissions, commercial instructions, contractual references, timetable, clarification rules, and validity requirements.
6. Evaluation stages, criteria, weights, mandatory pass conditions, minimum technical threshold, commercial treatment, presentation or demonstration rules, tie treatment, and recommendation authority are fixed and approved before issue.
7. Evaluators or committee members are nominated where required and record confidentiality and conflict-of-interest declarations before accessing restricted proposal content.
8. J02 supplier eligibility is assessed for the relevant category and time. The approved invitation or publication population and eligibility snapshot are preserved.
9. The RFP package receives required approval and is issued through authorised channels. Issued version, date, recipients or publication basis, deadline, issuing actor, and channel are recorded.
10. Supplier participation is tracked as invited or aware, participating, declined, no response, submitted, revised, or withdrawn while preserving chronology.
11. Questions are handled through the governed clarification process. Material information affecting proposal preparation is communicated consistently to all affected participants.
12. Any RFP amendment preserves the earlier version, approval, reason, issue evidence, affected participants, and revised deadline where applicable.
13. Each proposal received is associated with the correct supplier, Sourcing Event, and issued RFP version and receives its actual timestamp and channel.
14. Permitted proposal revisions received before the deadline are versioned; earlier submissions remain historical evidence.
15. At the governed deadline, submissions close. Late, incomplete, duplicate, withdrawn, corrupted, or otherwise exceptional proposals receive the configured disposition with authority and reason.
16. Proposal opening occurs under applicable confidentiality, chronology, quorum, and separation controls.
17. Eligibility and mandatory responsiveness are evaluated first where required. Every exclusion identifies the unmet requirement, evidence, evaluator, and authority.
18. Technical or quality evaluation is completed against the approved criteria. Individual scores, reasons, consensus adjustments, demonstrations, presentations, reference checks, and clarification outcomes remain attributable.
19. Only proposals meeting the governed technical or mandatory threshold proceed to commercial evaluation where staged segregation is required.
20. Commercial evaluation preserves original proposed prices and terms while recording any permitted evaluated or normalised view, assumptions, exclusions, total-cost considerations, validity, and deviations.
21. The combined evaluation is calculated or concluded using the pre-approved method. Criteria or weights are not changed after proposal access.
22. Competition sufficiency is assessed. A shortfall follows a governed exception, extension, reissue, alternate method, cancellation, or re-sourcing path rather than silent waiver.
23. The Procurement Recommender records the proposed supplier or suppliers, solution and scope, evaluated value reference, scoring basis, material risks, conditions, deviations, and justification.
24. Where negotiation or best-and-final offers are authorised, the invitation basis, participants, permitted scope, proposal versions, evaluation treatment, and outcome remain traceable and equal-treatment obligations are preserved.
25. The Procurement Approver evaluates the recommendation, authority, policy compliance, evaluation evidence, conflicts, deviations, competition position, and exception approvals.
26. On approval, the exact recommendation becomes the approved RFP sourcing outcome and the Sourcing Event reaches Completed.
27. The outcome is made available for downstream Purchase Order or contract processing without J01 creating supplier commitment.

---

# 10. Alternate Business Flows

## 10.1 Open or Limited RFP

The RFP may be publicly issued or restricted to an approved supplier population according to policy. Publication or invitation basis, eligibility treatment, equal access to material information, and submission evidence remain traceable.

## 10.2 Pre-Proposal Conference, Site Visit, or Presentation

Where the requirement benefits from common briefing or site understanding, attendance, questions, material answers, access conditions, and any effect on eligibility are recorded. Material information is made available consistently to affected participants.

## 10.3 RFP Amendment or Deadline Extension

1. A material clarification or correction requires change after issue.
2. The amendment records changed content, reason, authority, issue date, recipients, and revised deadline.
3. Prior versions remain accessible.
4. Proposals are evaluated against the applicable controlled version.

## 10.4 Proposal Clarification

Evaluators may request clarification only within the approved rules. Clarification must not silently permit a material new proposal, change criteria, cure a prohibited deficiency, or create unequal treatment. Request, response, timing, evaluator use, and impact are preserved.

## 10.5 Demonstration, Presentation, or Proof of Capability

Where pre-authorised, shortlisted suppliers may demonstrate or present against common rules. Attendees, questions, evidence, scores, and reasons are recorded, and no undisclosed criterion is introduced.

## 10.6 Negotiation or Best-and-Final Offer

Where policy authorises a negotiation or best-and-final-offer stage, participation conditions, permitted subjects, invitation, supplier submissions, decision rules, and final evaluated versions are controlled. Earlier proposals remain preserved.

## 10.7 Consortium, Joint, or Subcontracted Proposal

Where permitted, the proposal identifies the accountable supplier, members or subcontractors, allocation of responsibility, eligibility, conflicts, dependencies, and approval conditions. J02 retains supplier-master authority.

## 10.8 Insufficient Competition

If valid proposals are below the configured minimum, the case follows L3-J-018 through extension, additional invitation, reissue, approved exception, alternate route, cancellation, or re-sourcing. The shortfall is never hidden.

## 10.9 No Suitable Bid

If proposals exist but none meet the governed requirements or threshold, the event reaches No Suitable Bid or Re-Sourcing Required with evaluation evidence and decision authority preserved.

## 10.10 Recommendation Returned or Rejected

The approver may return the recommendation for correction or re-evaluation, or reject it. Prior evaluation and recommendation evidence remain preserved; no downstream commitment is authorised.

## 10.11 Multi-Supplier or Phased Outcome

Where the RFP and policy permit, the recommendation may allocate defined lots, phases, or responsibilities across suppliers. The approved outcome remains traceable to each demand component, proposal, evaluation basis, dependency, and condition.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid or Changed Demand Authority

If approved demand is withdrawn, superseded, or materially changed, RFP processing is withheld or suspended until J03 provides authoritative resolution. J01 does not repair the demand or conceal the changed basis.

## 11.2 Defective RFP or Evaluation Design

If material requirements, criteria, weights, stages, or mandatory conditions are ambiguous, inconsistent, discriminatory, or incomplete, issue or evaluation is suspended. Authorised amendment, deadline extension, restart, cancellation, or re-sourcing preserves the defect and decision evidence.

## 11.3 Supplier Eligibility Change

If supplier status changes after issue, applicable policy determines continued participation. The earlier eligibility snapshot, later status, decision, authority, and reason remain traceable.

## 11.4 Issue or Communication Failure

Failed delivery does not imply supplier receipt. Re-delivery, alternate approved channel, deadline adjustment, replacement invitation, or participation disposition is recorded without rewriting the original issue fact.

## 11.5 Late, Inaccessible, or Uncertain Proposal

The actual receipt timestamp, channel, and condition are preserved. Acceptance, rejection, controlled recovery, or escalation follows policy and authority without altering the original evidence.

## 11.6 Confidentiality or Premature Disclosure Breach

The affected proposal and process are protected from further ordinary handling. The breach is recorded and escalated for authorised continuation, exclusion, restart, cancellation, or other governed disposition.

## 11.7 Evaluator Conflict or Unavailability

An undeclared conflict or loss of required evaluator capacity suspends affected evaluation. Replacement, recusal, rescoring, or committee reconstitution follows authority while preserving prior activity and reasons.

## 11.8 Scoring or Evaluation Inconsistency

Arithmetic errors, missing reasons, unsupported disqualification, undisclosed criteria, unexplained score divergence, or altered weights require correction and re-review. Original evaluation evidence remains preserved.

## 11.9 Insufficient or No Response

The event follows the configured exception path and records the authorised continuation or terminal decision. Competition requirements are not silently waived.

## 11.10 Approval Interruption

An interrupted approval leaves the recommendation unapproved and prevents downstream commitment. Processing resumes against the same controlled recommendation unless governed change requires a successor version.

## 11.11 Post-Completion Error

A completed RFP outcome is not destructively rewritten. Before downstream commitment, authorised correction, cancellation, or re-sourcing applies. After commitment, the owning downstream scenario governs its consequences.

---

# 12. Business Rules and Decision Constraints

- RFP sourcing must originate from valid approved procurement demand.
- RFP is appropriate when the institution must evaluate differentiated solutions, methods, quality, capability, implementation, or service together with commercial value.
- Every Sourcing Event must identify its method and applicable procurement-policy reference.
- The RFP package, mandatory conditions, evaluation stages, criteria, weights, thresholds, and decision rules must be approved before issue.
- Criteria must relate to the approved requirement and cannot be changed after proposal visibility except through an authorised process restart or other explicit governed disposition.
- Invitation eligibility must use J02-owned supplier facts and preserve the applicable point-in-time status.
- Issued RFPs, amendments, proposals, clarifications, presentations, negotiated submissions, and recommendations must be versioned; prior evidence cannot be overwritten.
- Actual receipt timestamps and channels remain immutable business evidence.
- Material clarifications affecting proposal preparation must be communicated consistently to affected participants.
- Late, incomplete, revised, withdrawn, joint, or exceptional proposals follow configured policy and explicit authority.
- Proposal content remains confidential according to the sourcing stage and authorised access.
- Evaluators must declare conflicts and evaluate only assigned approved criteria.
- Individual evaluation evidence must not be erased by a committee consensus result.
- A supplier must not be excluded without a recorded rule, fact, reason, and responsible decision.
- Technical and financial proposal access must remain segregated where policy requires staged evaluation.
- Only technically qualified proposals proceed to commercial evaluation where a technical threshold applies.
- Original commercial information must remain distinguishable from evaluated, normalised, or total-cost views.
- Lowest price is not automatically the most advantageous RFP outcome.
- Negotiation or best-and-final offers require explicit policy authority and equal, recorded treatment.
- Minimum competition shortfall requires a visible governed disposition.
- Preparation, evaluation, recommendation, exception approval, and final approval follow configured separation rules.
- An approved RFP outcome does not itself create a Purchase Order, contract, budget commitment, payable, accounting entry, receipt, stock, asset, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Sourcing Event | `Draft` or the applicable existing J01 lifecycle position | Applies only the configured canonical sourcing stages required by this scenario. | `Completed` or the exact applicable terminal state: `Cancelled`, `No Response`, `No Suitable Bid` or `Re-Sourcing Required` | Procurement Planning & Sourcing (J01) | `L2-J01` Section 12.3 |
| Approved Procurement Demand | Source-defined approved position | Correlated to sourcing activity without changing demand state. | Unchanged source-owned position | Purchase Requisition & Approval (J03) | `L2-J03` |

Any scenario-specific consolidation, split, allocation, proposal or case record has no separately defined canonical lifecycle in J01. This scenario therefore does not create one.
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SourcingEventCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SourcingEventPublished` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SupplierResponseSubmitted` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SupplierSubmissionsClosed` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `TechnicalEvaluationFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `CommercialEvaluationFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Domain / Capability | Collaboration | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Supplies approved demand, justification, intended outcome, scope, and budget-validation references. | J03 retains demand ownership. |
| J02 Vendor and Supplier Management | Supplies supplier identity, category eligibility, qualification, suspension, and blacklisting facts. | J02 retains supplier ownership. |
| Requesting or specialist domain | Supplies business, technical, implementation, service, legal, security, or domain evaluation expertise. | Each specialist retains its authoritative semantics; J01 owns sourcing governance. |
| Finance | Supplies monetary or budget references where required. | Finance retains monetary truth; J01 evaluates proposals for procurement selection only. |
| J04 Purchase Order Management or authorised contracting capability | Consumes the approved sourcing outcome for downstream commitment. | The downstream owner alone creates and manages commitment. |
| Enterprise document capability | Preserves controlled RFPs, proposals, evaluations, decisions, and evidence. | J01 owns the business relationships, not storage design. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- approved demand, plan where applicable, and policy references;
- RFP-method determination and any override authority;
- Sourcing Event identity and controlled RFP or amendment versions;
- requirements, mandatory conditions, evaluation stages, criteria, weights, thresholds, and approval before issue;
- invitation or publication basis, supplier eligibility snapshot, issue date, deadline, channel, and actor;
- clarification, conference, site-visit, presentation, and amendment evidence;
- proposal versions, timestamps, channels, withdrawals, and exceptional dispositions;
- evaluator appointments, confidentiality and conflict declarations;
- eligibility, responsiveness, technical, commercial, scoring, consensus, disqualification, and deviation evidence;
- original and evaluated commercial positions and calculation basis;
- competition assessment and any exception approval;
- negotiation or best-and-final-offer authority and version history where applicable;
- recommendation, risks, conditions, reasons, reviewers, approver, authority, decision, and timestamps;
- approved or alternative terminal sourcing outcome.

## 15.3 Audit Obligations

The scenario must permit reconstruction from approved demand through RFP design, issue, every proposal and communication, evaluation, recommendation, approval, and final outcome. Criteria, scores, changes, reasons, actors, timestamps, conflicts, authority, prior and successor versions, and correlation references must remain attributable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- valid approved demand and RFP-method authority are traceable;
- the exact approved RFP package, criteria, weights, thresholds, timetable, and issued versions are preserved;
- supplier eligibility and invitation or publication basis are evidenced;
- all proposal versions, actual timestamps, channels, communications, and participation dispositions are retained;
- submission closure and proposal opening are governed;
- evaluators have valid appointments, confidentiality obligations, and conflict declarations;
- eligibility, technical, commercial, and combined evaluation follow the pre-approved basis;
- every material score, exclusion, clarification, deviation, and consensus adjustment has evidence and reason;
- original commercial content and evaluated views remain distinguishable;
- competition requirements are met or a valid exception is approved;
- any negotiation or best-and-final-offer stage is expressly authorised and traceable;
- the exact supplier and proposal recommendation has valid approval;
- the downstream commitment owner can consume the outcome without J01 creating commitment;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the Sourcing Event reaches Completed with an approved RFP sourcing outcome;
- complete audit and documentary evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. The exact mandatory RFP lifecycle states and whether technical, commercial, negotiation, and best-and-final-offer stages require separate canonical states must be confirmed against the locked J01 lifecycle catalogue.
2. `L2-J01` defines the generic Sourcing Event events used in Section 14 but no RFP-specific event set for issue, proposal receipt, evaluation, or recommendation. Governance must decide whether the generic events are sufficient; this L3 scenario does not invent RFP-specific events.
3. Institution-specific procurement rules determine when open, limited, two-stage, quality-and-cost, presentation, negotiation, or best-and-final-offer methods are legally or procedurally permitted; L3 cannot universalise them.
4. The canonical treatment of evaluator consensus, score moderation, tie resolution, abnormally low or high commercial positions, and post-evaluation due diligence requires configuration governance before implementation locking.
5. The boundary between a returned recommendation, clarification, re-evaluation, amended RFP, re-sourcing, and cancellation requires exact state and event governance for each configured operating profile.

## 17.2 Assumptions

- RFP sourcing is distinct from RFQ because differentiated solutions or methods and multi-criteria evaluation materially determine the outcome.
- Supplier self-service is not required; authorised institutional personnel may record proposals received through approved external channels.
- Procurement scoring and comparison do not create Finance-owned accounting interpretation.
- A completed RFP sourcing outcome is necessary but not sufficient for supplier commitment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-005**
