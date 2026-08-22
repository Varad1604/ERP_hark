# Approved Procurement Demand to Competitive Quotation or RFQ Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-004  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-018 Insufficient Competition to Governed Sourcing Disposition; L3-J-019 Active Sourcing Event to Governed Cancellation; downstream J04 Purchase Order scenarios consuming an approved sourcing outcome  

---
# 2. Scenario Purpose

This scenario governs competitive commercial sourcing through quotation collection or a Request for Quotation when approved procurement demand contains sufficiently defined requirements. It ensures that supplier invitation, response receipt, commercial comparison, evaluation, recommendation, approval, and exception handling produce a traceable institutional sourcing outcome.

The scenario ends before supplier commitment. It establishes the approved sourcing authority that J04 may consume when creating a Purchase Order, but does not itself create an order, financial commitment, payable, receipt, inventory movement, or payment.

---

# 3. Business Outcome

Eligible supplier responses are governed through quotation or RFQ processing and an approved sourcing outcome is established for downstream commitment, with the demand, sourcing method, invitations, submissions, comparison, recommendation, authority, and evidence fully traceable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when J01 receives valid approved procurement demand requiring competitive quotation or RFQ sourcing against sufficiently defined requirements.

## 4.2 End Boundary

The scenario ends when the supplier recommendation is approved and the Sourcing Event reaches Completed with an approved sourcing outcome, or reaches a governed alternative terminal disposition such as No Response, No Suitable Bid, Re-Sourcing Required, or Cancelled.

## 4.3 In Scope

- validating approved-demand eligibility and traceability;
- determining that quotation or RFQ is the applicable configured method;
- creating and approving a controlled Sourcing Event and RFQ or quotation request;
- selecting eligible suppliers for invitation;
- issuing controlled invitations and preserving the issued version;
- receiving, time-stamping, validating, and preserving supplier responses;
- closing submissions and governing late, withdrawn, revised, partial, or non-responsive participation;
- comparing valid commercial and applicable technical information;
- finalising a Comparative Statement where required;
- recording supplier recommendation and justification;
- approving the sourcing outcome or governing rejection, return, insufficient competition, no suitable bid, re-sourcing, or cancellation;
- publishing the completed sourcing fact for downstream commitment.

## 4.4 Out of Scope

- creation or approval of procurement demand, owned by J03;
- supplier master, qualification, suspension, or blacklisting, owned by J02;
- RFP sourcing, governed by L3-J-005;
- tender sourcing, governed by L3-J-006;
- direct or simplified sourcing, governed by L3-J-007;
- single-source, emergency, rate-contract, or external procurement paths;
- canonical Purchase Order creation, supplier commitment, amendment, or cancellation, owned by J04;
- goods receipt, inspection, inventory, asset, accounting, payable, or payment processing;
- technical portal, messaging channel, cryptography, API, schema, or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-018 Insufficient Competition to Governed Sourcing Disposition | Governs response shortfall where configured competition is not achieved. |
| Related | L3-J-019 Active Sourcing Event to Governed Cancellation | Governs authorised cancellation before a completed sourcing outcome. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns method execution, Sourcing Event, RFQ or quotation request, invitations, response evidence, evaluation, Comparative Statement, recommendation, approval, and final sourcing outcome. |

J01 consumes approved demand from J03 and supplier identity or eligibility from J02 without changing either aggregate. Finance-owned monetary truth and J04-owned supplier commitment remain outside the outcome.

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

- J03 has produced identifiable approved procurement demand; draft, returned, or rejected demand is not valid normal sourcing authority.
- Requirement description, lines, quantities, specification references, delivery requirements, and completion expectations are sufficiently defined for quotation or RFQ sourcing.
- Applicable procurement policy, method thresholds, competition rules, approval requirements, exception authorities, evaluation rules, and document requirements are effective.
- Quotation or RFQ is an allowed method for the demand, or any method override has explicit authority and justification.
- Supplier identities and eligibility facts can be resolved through J02 for the invitation decision.
- The Procurement Officer is authorised for the institution, organisational unit, category, method, and anticipated value context.
- Any procurement-plan reference is traceable where applicable, without making the plan a substitute for approved demand.
- No conflicting active sourcing event already governs the same demand unless consolidation, splitting, or parallel sourcing is explicitly authorised.

---

# 8. Trigger

Approved procurement demand is accepted by J01 and the configured procurement method requires competitive quotation collection or an RFQ.

---

# 9. Main Business Flow

1. The Procurement Officer accepts the approved demand and records its authoritative J03 reference.
2. The officer validates demand status, scope, lines, specifications, required dates, delivery expectations, estimated-value reference, category, requesting unit, and applicable procurement policy.
3. The officer confirms quotation or RFQ as the governed method and records any approved method override.
4. A Sourcing Event is created with a unique identity and links to the approved demand, requisition, procurement plan where applicable, method, policy, category, delivery requirements, and responsible actor.
5. The officer prepares the controlled quotation request or RFQ containing requirement lines, quantities, specifications, delivery location and date, commercial requirements, warranty requirements, required documents, terms references, clarification process, and submission deadline.
6. The draft sourcing package is reviewed and receives required authority for publication or invitation.
7. J02 supplier eligibility is evaluated for the relevant category and point in time. The approved invitation list and eligibility snapshot are preserved.
8. The RFQ or quotation request is issued through approved channels. Version, issue date, invited suppliers, deadline, issuing actor, and delivery channel are recorded.
9. Supplier participation is tracked as invited, accepted, declined, no response, response submitted, or withdrawn, while preserving chronology.
10. Each received response is associated with the correct supplier, Sourcing Event, and issued version and receives its actual submission timestamp and channel.
11. Submitted commercial terms, line prices, charges, discounts, delivery, warranty, validity, deviations, supporting documents, and applicable technical content are preserved without accounting reinterpretation.
12. Permitted revisions received before the deadline are versioned; earlier supplier submissions remain historical evidence.
13. At the governed deadline, submissions close. Late, withdrawn, incomplete, duplicate, or otherwise exceptional responses receive their configured disposition with authority and reason.
14. Valid responses proceed to opening and evaluation under applicable confidentiality and separation controls.
15. Technical compliance is assessed where required. Commercial evaluation compares original quoted information, normalised views where configured, delivery, warranty, charges, validity, and visible deviations.
16. A Comparative Statement is prepared and reviewed where required, preserving the exact evaluation inputs.
17. Competition sufficiency is assessed. A shortfall follows re-invitation, reissue, approved exception, alternate route, cancellation, or re-sourcing rather than silent waiver.
18. The Procurement Recommender records the recommended supplier or suppliers, lines, quantities, evaluated reference, basis, reason, supporting comparison, and any approved exception.
19. Non-lowest recommendation receives explicit justification where policy requires; lowest commercial price is not treated as automatic selection.
20. The Procurement Approver evaluates the recommendation, authority, policy compliance, evidence, deviations, competition position, and exception approvals.
21. On approval, the exact recommendation becomes the approved sourcing outcome and the Sourcing Event reaches Completed.
22. The completed outcome is made available to J04 for possible downstream Purchase Order creation without J01 creating supplier commitment.

---

# 10. Alternate Business Flows

## 10.1 Simplified Quotation Collection

Where configured policy permits a reduced quotation process, unnecessary stages may be omitted. Approved demand, supplier identity, issued request, actual responses, comparison basis, authority, decision, and evidence remain mandatory.

## 10.2 RFQ Amendment

1. A material clarification or correction requires change after issue.
2. The amendment records changed content, reason, approval where required, amendment date, and revised deadline.
3. Prior issued versions remain accessible.
4. All affected suppliers receive the governed amended version through approved channels.
5. Responses are evaluated against the applicable version without rewriting earlier submissions.

## 10.3 Revised Supplier Response

Where policy permits revision before the deadline, a later response version becomes the evaluation candidate while earlier versions and timestamps remain preserved.

## 10.4 Partial Quotation

Where partial bidding is allowed, a supplier may respond to selected lines or quantities. Comparison and recommendation remain line- and quantity-traceable. If partial bidding is prohibited, the response receives the configured invalid or non-responsive disposition.

## 10.5 Insufficient Competition

If valid responses are below the configured minimum, the case follows L3-J-018 through reissue, additional invitation, approved competition exception, alternate method, cancellation, or re-sourcing. The shortfall is never hidden.

## 10.6 No Suitable Response

If responses exist but none satisfy the governed requirements, the Sourcing Event reaches No Suitable Bid or Re-Sourcing Required, with evaluation evidence and decision authority preserved.

## 10.7 Recommendation Returned or Rejected

The approver may return the recommendation for correction or re-evaluation, or reject it. The prior recommendation and decision remain preserved; no downstream commitment is authorised.

## 10.8 Multi-Supplier or Split Outcome

Where policy and demand permit, the recommendation may allocate lines or quantities across suppliers. The approved result remains traceable to every demand line, evaluated response, and selection basis and relates to L3-J-010.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid Demand Authority

If the demand is not approved, has been withdrawn, superseded, or materially changed, sourcing is withheld or suspended until J03 provides authoritative resolution. J01 does not repair the demand.

## 11.2 Supplier Eligibility Change

If supplier status changes after invitation, the applicable policy determines continued participation. The eligibility snapshot at invitation, later status, decision, authority, and reason remain traceable.

## 11.3 Invitation Delivery Failure

Failed delivery does not imply supplier receipt. Re-delivery, alternate approved channel, replacement invitation, or supplier disposition is recorded without changing the original issue fact.

## 11.4 Late or Uncertain Submission

The actual receipt timestamp and channel are preserved. Acceptance, rejection, or escalation follows configured policy and authority; the submission timestamp is never rewritten.

## 11.5 Confidentiality or Premature Opening Breach

The affected response and sourcing process are protected from further ordinary evaluation. The breach is recorded and escalated for authorised continuation, restart, cancellation, or other governed disposition.

## 11.6 Evaluation Inconsistency

Conflicting scores, missing criteria, unexplained disqualification, or comparison errors require correction and re-review. Original evaluation evidence remains preserved.

## 11.7 Insufficient or No Response

The event follows the configured exception path and records the authorised terminal or continuation decision. Competition requirements are not silently waived.

## 11.8 Approval Interruption

An interrupted approval leaves the recommendation unapproved and prevents downstream commitment. Processing resumes against the same controlled recommendation unless governed change requires a new version.

## 11.9 Post-Completion Error

A completed sourcing outcome is not destructively rewritten. Before Purchase Order commitment, governed cancellation, re-sourcing, or supplemental correction applies as authorised. After J04 commitment, J04 and other owning domains govern their own consequences.

---

# 12. Business Rules and Decision Constraints

- Standard sourcing must originate from approved procurement demand.
- Every Sourcing Event must identify its method and applicable procurement-policy reference.
- Quotation or RFQ is used where requirements are sufficiently defined and commercial or delivery quotation is the primary sourcing need.
- Invitation eligibility must use J02-owned supplier status and preserve the status applicable at invitation.
- Suspended or blacklisted suppliers are normally prohibited from new participation unless authoritative policy provides a governed exception.
- Issued sourcing documents and supplier submissions must be versioned; prior versions cannot be overwritten.
- Actual submission timestamp and channel must remain immutable evidence.
- Late, revised, partial, withdrawn, or exceptional responses follow configured policy and explicit authority.
- Restricted responses remain confidential according to procurement stage and authorisation.
- A supplier must not be disqualified without recorded basis.
- Original quoted information must remain preserved when a normalised comparison is produced.
- Lowest price is not automatically the selected supplier.
- Non-lowest selection requires explicit justification where policy requires.
- Minimum competition shortfall requires a visible governed disposition.
- The exact evaluated inputs supporting the final Comparative Statement and recommendation must remain traceable.
- Preparation, evaluation, recommendation, exception approval, and final approval follow configured separation rules.
- An approved sourcing outcome does not itself create a Purchase Order, budget commitment, payable, accounting entry, receipt, stock, asset, or payment.

---

# 13. State and Lifecycle Impact

## 13.1 Sourcing Event Lifecycle

| From State | Business Action | To State | Required Result |
|---|---|---|---|
| None | Create from approved demand | Draft | Controlled Sourcing Event and demand reference exist. |
| Draft | Prepare sourcing package | Preparation | RFQ or quotation request and invitation basis are defined. |
| Preparation | Approve issue | Approved for Publication / Invitation | Exact issue version is authorised. |
| Approved for Publication / Invitation | Issue invitations | Open for Supplier Response | Invitation evidence and deadline are recorded. |
| Open for Supplier Response | Reach deadline | Submission Closed | Received responses and participation positions are preserved. |
| Submission Closed | Open valid responses | Opening | Governed opening evidence exists. |
| Opening | Evaluate | Evaluation | Technical and commercial results are attributable. |
| Evaluation | Finalise recommendation | Recommendation | Comparative basis and recommended outcome exist. |
| Recommendation | Submit for decision | Approval | Exact recommendation is under authorised decision. |
| Approval | Approve | Completed | Approved sourcing outcome exists for downstream use. |

Alternative terminal outcomes include Cancelled, No Response, No Suitable Bid, and Re-Sourcing Required.

## 13.2 Supplier Participation and Response Impact

Participation and response versions remain historically traceable. Withdrawal, rejection, lateness, or non-selection does not delete a supplier's submitted evidence.

## 13.3 Approved Demand Impact

J01 links sourcing progress and outcome to approved demand but does not change J03 demand lifecycle. J03 remains authoritative for demand status.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SourcingEventCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SourcingEventPublished` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SupplierSubmissionsClosed` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ComparativeStatementFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Domain / Capability | Collaboration | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Supplies approved demand, justification, scope, and budget-validation references. | J03 retains demand ownership. |
| J02 Vendor and Supplier Management | Supplies supplier identity, category eligibility, qualification, suspension, and blacklisting facts. | J02 retains supplier ownership. |
| Requesting domain | Supplies technical and operational requirement context. | Specialist semantics remain externally owned. |
| Finance | Supplies monetary or budget references where required. | Finance retains monetary truth; J01 performs procurement comparison only. |
| J04 Purchase Order Management | Consumes approved sourcing outcome for downstream commitment. | J04 alone creates and manages the Purchase Order. |
| Enterprise document capability | Preserves controlled requests, quotations, comparisons, decisions, and evidence. | J01 owns business relationships, not storage design. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- approved demand and applicable policy references;
- method determination and any override authority;
- Sourcing Event identity and controlled issue versions;
- invitation list, supplier eligibility snapshot, issue date, deadline, channel, and issuing actor;
- supplier response versions, timestamps, channels, original commercial terms, and documents;
- opening, evaluation, disqualification, deviation, and comparison evidence;
- competition assessment and exception disposition;
- Comparative Statement and recommendation basis;
- non-lowest or other exceptional selection justification;
- reviewer, recommender, approver, decisions, authority, and timestamps;
- approved or alternative terminal sourcing outcome.

## 15.3 Audit Obligations

The scenario must permit reconstruction from approved demand through invitation, every response, evaluation, recommendation, approval, and final outcome. Significant actions, changes, reasons, evidence, actors, timestamps, prior and successor versions, authority, and correlation references must remain attributable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- valid approved demand and method authority are traceable;
- the exact quotation request or RFQ version and invitation population are preserved;
- supplier eligibility at invitation is evidenced;
- all received response versions, actual timestamps, channels, and participation dispositions are retained;
- submission closure and opening are governed;
- valid responses are evaluated against approved criteria;
- original commercial information and any normalised comparison are distinguishable;
- competition requirements are met or a valid exception is approved;
- the Comparative Statement and recommendation basis are finalised where applicable;
- exceptional or non-lowest selection has required justification;
- the exact recommendation has valid approval;
- J04 can consume the outcome without J01 creating supplier commitment;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the Sourcing Event reaches Completed with an approved sourcing outcome;
- complete audit and documentary evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. L2-J01 describes representative sourcing stages but permits simplified procurement to omit unnecessary stages; the minimum mandatory RFQ state sequence requires configuration governance before L4 design.
2. `L2-J01` defines the generic Sourcing Event events used in Section 14 but no RFQ-specific event set for issue, response, evaluation, or recommendation. Governance must decide whether the generic events are sufficient; this L3 scenario does not invent RFQ-specific events.
3. L2-J01 allows configurable participation status terminology; the canonical cross-institution vocabulary must be resolved before implementation locking.
4. The legal or policy sufficiency of non-lowest selection justification, late-response acceptance, competition exception, and prospective-supplier participation remains institution-specific and cannot be universalised at L3.
5. The boundary between a returned recommendation, re-evaluation, re-sourcing, and sourcing cancellation requires exact state and event governance for each configured operating profile.

## 17.2 Assumptions

- Competitive quotation and RFQ are governed as one canonical scenario because they share the approved-demand-to-approved-sourcing-outcome lifecycle.
- Supplier self-service is not required; authorised institutional personnel may record submissions received through approved external channels.
- Commercial comparison is a procurement activity and does not create Finance-owned accounting interpretation.
- A completed sourcing outcome is necessary but not sufficient for supplier commitment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-004**
