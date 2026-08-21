# Approved Procurement Demand to Governed Tender Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-006  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-018 Insufficient Competition to Governed Sourcing Disposition; L3-J-019 Active Sourcing Event to Governed Cancellation; downstream J04 Purchase Order scenarios consuming an approved sourcing outcome  

---
# 2. Scenario Purpose

This scenario governs formal tender sourcing when approved procurement demand must be subjected to an open, limited, national, international, single-stage, two-stage, or other institutionally authorised tender procedure. It preserves transparency, competition, equal treatment, confidentiality, delegated authority, and a reconstructable award decision.

The scenario establishes an approved tender sourcing outcome for downstream commitment. It does not itself execute a contract, create a Purchase Order, recognise a liability, receive goods or services, or make payment.

---

# 3. Business Outcome

Approved procurement demand is subjected to the applicable governed tender procedure and results in an authorised, auditable tender sourcing outcome, or an explicitly governed alternative terminal disposition, without prematurely creating supplier commitment.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when J01 receives valid approved procurement demand for which policy, value, risk, category, funding condition, or institutional decision requires a formal tender method.

## 4.2 End Boundary

The scenario ends when the tender award recommendation is approved and the Sourcing Event reaches Completed with an approved tender sourcing outcome, or reaches a governed terminal outcome such as No Response, No Suitable Bid, Re-Sourcing Required, or Cancelled.

## 4.3 In Scope

- validating demand authority and tender-method applicability;
- selecting and approving the applicable tender form;
- creating the Tender Sourcing Event and controlled tender package;
- defining eligibility, qualification, responsiveness, technical, commercial, security, submission, evaluation, and award requirements before publication;
- publication, invitation, sale or controlled release of tender documents where applicable;
- pre-bid conference, site visit, clarification, corrigendum, addendum, and deadline extension;
- bid registration, secure receipt, withdrawal, substitution, modification, closure, and formal opening;
- bid security or earnest-money evidence where applicable, without performing payment accounting;
- preliminary, eligibility, technical, financial, and combined evaluation under the approved tender method;
- recording committee constitution, conflicts, quorum, opening, evaluation, clarifications, scores, deviations, and disqualifications;
- competition sufficiency, exceptional bid treatment, negotiation or best-and-final offer only where authorised;
- award recommendation, approval, rejection, return, cancellation, or re-tender decision;
- publishing the approved outcome for downstream commitment.

## 4.4 Out of Scope

- procurement-plan and demand creation or approval;
- supplier-master creation, qualification lifecycle, suspension, and blacklisting, owned by J02;
- quotation or RFQ sourcing, governed by L3-J-004;
- RFP sourcing, governed by L3-J-005;
- direct or simplified sourcing, governed by L3-J-007;
- Purchase Order, work order, letter of acceptance, contract execution, amendment, or supplier commitment;
- bid-security banking, refund accounting, forfeiture accounting, payable, receipt, inventory, asset, or payment processing;
- legal drafting of tender conditions or contracts;
- portal, e-procurement, cryptography, digital-signature, API, schema, or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-018 Insufficient Competition to Governed Sourcing Disposition | Governs tender competition shortfall and its authorised disposition. |
| Related | L3-J-019 Active Sourcing Event to Governed Cancellation | Governs authorised cancellation before an approved tender outcome. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns tender execution, Tender Sourcing Event, documents, publication or invitation, bid evidence, opening, evaluation governance, recommendation, approval, and outcome. |

J01 consumes approved demand from J03 and supplier facts from J02. Requesting, technical, legal, finance, vigilance, or other specialist functions may contribute authorised expertise, but J01 remains accountable for procurement-process integrity and the sourcing outcome. Each participating function retains its own authoritative semantics.

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

- J03 has produced identifiable approved procurement demand; draft, returned, rejected, withdrawn, or superseded demand is not valid tender authority.
- Applicable policy authorises or requires the selected tender method for the demand's category, anticipated value, risk, funding source, or complexity.
- Requirement, specifications or outcomes, quantity or scope, delivery or completion expectations, eligibility conditions, and acceptance basis are sufficiently defined.
- Estimated-value and budget-validation references required by policy are traceable without transferring monetary truth to J01.
- Tender conditions, submission structure, opening stages, evaluation method, qualification requirements, mandatory criteria, weights or price basis, tie handling, bid-security rules, and approval path are defined before publication.
- Publication period, channels, thresholds, committee requirements, conflict declarations, confidentiality, and delegated authorities are effective.
- Supplier eligibility facts can be resolved through J02 where prequalification or limited tendering applies.
- No conflicting active sourcing event governs the same demand unless authorised splitting, lots, consolidation, or parallel sourcing applies.

---

# 8. Trigger

Approved procurement demand is accepted by J01 and the applicable governance requires a formal tender sourcing procedure.

---

# 9. Main Business Flow

1. The Procurement Officer accepts the approved demand and records its authoritative J03 reference.
2. Demand status, scope, specifications or outcomes, quantities, delivery or completion dates, category, value reference, funding or policy conditions, and requesting unit are validated.
3. The officer determines the governed tender form and records the policy basis and any authorised method exception.
4. A Tender Sourcing Event is created with unique identity and links to the approved demand, procurement plan where applicable, method, policy, lots, timetable, and responsible actors.
5. The controlled tender package is prepared, including notice, instructions, scope or specifications, bidder conditions, qualification evidence, bid format, commercial schedule, security requirements, contractual references, milestones, evaluation basis, and award rules.
6. Mandatory eligibility and responsiveness conditions, technical and financial evaluation stages, criteria, weights or price basis, thresholds, tie rules, correction rules, and approval route are fixed before publication.
7. Tender-opening and evaluation responsibilities are constituted where required. Members record confidentiality and conflict-of-interest declarations before restricted participation.
8. The tender package and publication or invitation decision receive required approval.
9. The tender is published or issued through authorised channels for the required period. Notice, version, date, channels, recipients where limited, access conditions, deadline, and issuing actor are preserved.
10. Bidder registrations, document access, participation, and authorised prequalification facts are recorded without replacing J02 supplier ownership.
11. Pre-bid questions, conference or site-visit matters are governed. Material information is communicated consistently to all affected bidders.
12. Any corrigendum or addendum records changed content, reason, authority, publication, recipients, and revised deadline where applicable; earlier versions remain evidence.
13. Each bid received is associated with the correct bidder, Sourcing Event, lot, tender version, receipt timestamp, channel, and required submission components.
14. Bid security or exemption evidence is associated with the bid where required, while Finance or Treasury retains its own monetary processing authority.
15. Permitted withdrawal, substitution, or modification before the deadline is versioned and time-stamped. Earlier submissions remain historical evidence.
16. At the governed deadline, bid submission closes. Late, incomplete, duplicate, inaccessible, or exceptional submissions receive the configured disposition with reason and authority.
17. The Committee Chair / Convenor and authorized Committee Members open the applicable bid stage, records date, time, attendance or quorum, bids present, declared particulars, exceptions, and opening record.
18. Preliminary eligibility and responsiveness evaluation is completed. Every rejection identifies the rule, fact, evidence, evaluator, and authority.
19. Technical evaluation of responsive bids is completed against the published criteria. Clarifications, demonstrations, samples, site assessments, reference checks, deviations, scores, and reasons remain attributable.
20. Only technically qualified bids proceed to financial opening where two-envelope or staged evaluation applies.
21. Financial bids are opened under the governed procedure. Original quoted values and terms are preserved.
22. Commercial evaluation records permitted arithmetic correction, taxes or charges treatment, discounts, loading, normalisation, total evaluated cost, preference rules, validity, deviations, and ranking without changing original bid facts.
23. Competition sufficiency and reasonableness are assessed. A shortfall or exceptional commercial result follows explicit governed disposition.
24. Any permitted clarification, negotiation, auction, or best-and-final-offer stage uses pre-authorised rules, equal treatment, controlled invitations, preserved versions, and attributable outcomes.
25. The Procurement Recommender records the proposed award by supplier, lot, line, quantity or scope, evaluated value reference, basis, risks, conditions, deviations, and approved exceptions.
26. The Procurement Approver reviews demand authority, tender method, publication, competition, opening, evaluation, conflicts, exceptions, recommendation, and delegated authority.
27. On approval, the exact recommendation becomes the approved tender sourcing outcome and the event reaches Completed.
28. The approved outcome is made available for downstream commitment without J01 creating a Purchase Order, letter of acceptance, or contract.

---

# 10. Alternate Business Flows

## 10.1 Open, Limited, National, or International Tender

Publication or invitation coverage follows applicable policy. The chosen route, eligibility population, notice period, channels, language or currency conditions where applicable, and approval basis remain traceable.

## 10.2 Single-Stage or Two-Stage Tender

Where one stage is used, qualification, technical, and commercial components follow the approved opening and evaluation sequence. Where two stages are authorised, first-stage technical or solution development and second-stage final bidding remain separately versioned and governed.

## 10.3 Prequalification

Where prequalification is required, only candidates satisfying the approved prequalification outcome proceed. The qualification decision remains traceable and does not overwrite J02 supplier status.

## 10.4 Corrigendum, Addendum, or Extension

A material change records the exact amendment, reason, authority, publication or communication, affected bidders, and deadline impact. If a fair response is no longer possible, the event is restarted, cancelled, or re-tendered rather than silently continued.

## 10.5 Bid Withdrawal, Substitution, or Modification

Permitted bidder action before closure is recorded with actual time and version lineage. Post-deadline change is prohibited unless an explicit tender rule and authority permits the specific action.

## 10.6 Bid Clarification

Clarification may resolve ambiguity but must not silently create a materially new bid, cure a prohibited deficiency, change published criteria, or create unequal treatment. Request, response, authority, and evaluation impact are preserved.

## 10.7 Insufficient Competition or Single Valid Bid

The case follows L3-J-018 through extension, wider publication, re-tender, approved competition exception, alternate method, cancellation, or other authorised disposition. A single valid bid is not automatically accepted or rejected without applicable governance.

## 10.8 No Suitable Bid

If bids exist but none meet mandatory or evaluated requirements, the event reaches No Suitable Bid or Re-Sourcing Required with evidence and decision authority preserved.

## 10.9 Negotiation, Auction, or Best-and-Final Offer

These stages occur only when tender rules and policy explicitly allow them. Participant eligibility, common rules, invitations, bid versions, chronology, evaluation basis, and decision are retained.

## 10.10 Award Recommendation Returned or Rejected

The Procurement Approver may return the recommendation for correction or re-evaluation, reject it, cancel the event, or order re-tender. Prior evidence remains immutable and no commitment is authorised.

## 10.11 Multi-Lot or Split Award

Where tender conditions permit, the recommendation may allocate lots, lines, quantities, phases, or scope across bidders. Every award component remains traceable to demand, bid, evaluation, authority, and award rules.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid or Changed Demand

If demand authority is absent, withdrawn, superseded, or materially changed, the tender is withheld or suspended until J03 provides authoritative resolution. J01 does not silently amend the demand basis.

## 11.2 Defective Tender Conditions

Ambiguous, contradictory, discriminatory, incomplete, or unauthorised requirements, criteria, security conditions, or timelines require controlled correction, extension, restart, cancellation, or re-tender. The defect and decision remain recorded.

## 11.3 Publication or Communication Failure

Failed or unequal publication, document access, clarification delivery, or addendum communication is assessed for fairness impact. Authorised correction may include reissue, extension, restart, or cancellation without rewriting original facts.

## 11.4 Late, Inaccessible, or Uncertain Bid

Actual receipt time, channel, condition, and available evidence are preserved. Acceptance, rejection, controlled recovery, or escalation follows published rules and authority.

## 11.5 Confidentiality or Premature Opening Breach

Affected processing is suspended and protected. The breach is recorded and escalated for authorised continuation, re-opening, re-evaluation, exclusion, cancellation, or re-tender.

## 11.6 Committee Conflict, Quorum Failure, or Unavailability

Affected opening or evaluation is suspended. Recusal, replacement, reconstitution, or repeated activity follows authority while preserving previous evidence and reasons.

## 11.7 Evaluation Error or Inconsistency

Missing reasons, arithmetic errors, altered criteria, unsupported disqualification, inconsistent treatment, or unexplained scoring requires correction and re-review. Original evaluation evidence is retained.

## 11.8 Bid Validity or Security Issue

Expiry, invalid security, exemption ambiguity, or refusal to extend validity follows tender terms and the Procurement Approver. Procurement records the sourcing disposition; monetary processing remains with the owning domain.

## 11.9 Insufficient or No Response

The event follows the configured exception path. Competition requirements are not silently waived and continuation does not imply bid acceptability.

## 11.10 Approval Interruption

An interrupted approval leaves the recommendation unapproved. Processing resumes against the same controlled version unless a governed successor is required.

## 11.11 Post-Completion Error

A completed outcome is not destructively rewritten. Before downstream commitment, authorised correction, cancellation, or re-tender applies. After commitment, the downstream owner governs its consequences.

---

# 12. Business Rules and Decision Constraints

- Tender sourcing must originate from valid approved demand.
- The tender method must be justified by applicable policy, threshold, funding, category, risk, or complexity.
- Tender notice, conditions, specifications or outcomes, mandatory requirements, evaluation method, and award rules must be approved before publication.
- Published conditions, criteria, weights, thresholds, and award rules cannot be changed after bid visibility except through an authorised amendment and fair continuation, or a governed restart.
- Publication or invitation must satisfy the approved route and period.
- Material clarification and amendment information must be communicated consistently to affected bidders.
- Supplier eligibility facts remain owned by J02 and must be preserved at the applicable decision time.
- Tender documents, amendments, bids, opening records, clarifications, evaluations, and recommendations must be versioned; prior evidence cannot be overwritten.
- Actual receipt timestamps and channels remain immutable evidence.
- Bid opening must follow authorised timing, stage, quorum, confidentiality, and record requirements.
- Technical and financial bid access must remain segregated where the tender method requires it.
- Only qualified bids proceed to financial opening under a staged method.
- A bid cannot be rejected without a published rule or other valid authority, supporting fact, reason, and attributable decision.
- Original quoted information remains distinguishable from corrected, loaded, normalised, or evaluated values.
- Arithmetic correction, preference, tax, currency, discount, and ranking treatment must follow the published method.
- Lowest price alone does not override mandatory, technical, capability, quality, risk, or other approved requirements.
- A single valid bid or competition shortfall requires an explicit governed disposition.
- Negotiation, auction, or best-and-final offer is prohibited unless authorised by applicable tender rules.
- Preparation, opening, evaluation, recommendation, exception approval, and final approval follow configured separation rules.
- Bid security does not itself determine bid acceptability and its monetary processing remains outside J01.
- An approved tender outcome does not create a Purchase Order, letter of acceptance, contract, payable, accounting entry, receipt, stock, asset, or payment.

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
| `SourcingEventApprovedForPublication` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SourcingEventPublished` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SupplierSubmissionsClosed` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `BidOpeningFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `TechnicalEvaluationFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `CommercialEvaluationFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Domain / Capability | Collaboration | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Supplies approved demand, justification, scope, and budget-validation references. | J03 retains demand ownership. |
| J02 Vendor and Supplier Management | Supplies bidder identity, eligibility, qualification, suspension, and blacklisting facts. | J02 retains supplier ownership. |
| Requesting and specialist domains | Supply technical, legal, security, finance, works, service, or domain evaluation expertise. | Each specialist retains its authoritative semantics; J01 owns tender governance. |
| Finance / Treasury | Supplies budget or monetary references and governs bid-security monetary treatment where applicable. | J01 does not create financial truth or payment. |
| J04 Purchase Order Management or authorised contracting capability | Consumes approved tender outcome for downstream commitment. | Downstream owner alone creates and manages commitment. |
| Enterprise document capability | Preserves tender, bid, evaluation, decision, and award evidence. | J01 owns business relationships, not storage design. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- approved demand, procurement-plan reference where applicable, and governing policy;
- tender-method determination and any exception authority;
- event identity, tender notice, package, specifications or outcomes, conditions, and controlled versions;
- approved eligibility, responsiveness, evaluation, opening, security, and award rules before publication;
- publication or invitation evidence, channels, dates, recipients where applicable, and deadlines;
- pre-bid questions, conferences, site visits, clarifications, corrigenda, addenda, and extensions;
- bidder registrations, eligibility snapshots, bid versions, receipt timestamps, channels, and participation dispositions;
- bid-security or exemption references where required;
- committee nominations, conflicts, confidentiality, quorum, and opening records;
- eligibility, responsiveness, technical, financial, correction, scoring, deviation, disqualification, and ranking evidence;
- competition assessment, reasonableness review, and exception approvals;
- negotiation or final-offer evidence where authorised;
- recommendation, risks, conditions, reasons, reviewers, Procurement Approver, decision, and timestamps;
- approved or alternative terminal tender outcome.

## 15.3 Audit Obligations

The scenario must permit reconstruction from approved demand through method selection, tender preparation, publication, every bid and communication, opening, evaluation, recommendation, approval, and outcome. Significant facts, versions, actors, timestamps, conflicts, reasons, authority, and correlation references must remain attributable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- approved demand and tender-method authority are traceable;
- the exact tender notice, package, rules, evaluation basis, and issued versions are preserved;
- publication or invitation coverage and duration satisfy applicable governance;
- bidder participation, eligibility, bid versions, receipt times, channels, and exceptional dispositions are retained;
- amendments and material clarifications are equally and traceably communicated;
- opening bodies and evaluators have valid authority, conflicts, confidentiality, and quorum evidence where required;
- submission closure, opening, qualification, technical evaluation, and financial evaluation follow the approved sequence;
- original bid facts remain distinguishable from evaluated values;
- every rejection, correction, score, deviation, and ranking has a rule, evidence, reason, and actor;
- competition is sufficient or an authorised exception exists;
- any negotiation, auction, or final-offer stage is authorised and preserved;
- the exact award recommendation has valid approval;
- downstream commitment can consume the outcome without J01 creating commitment;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the event reaches Completed with an approved tender sourcing outcome;
- complete audit and documentary evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. `L2-J01` does not define separate lifecycle states for prequalification, opening stages, negotiation, auction, recommendation, or re-tender. Governance must decide whether supporting records require canonical lifecycles; this L3 scenario uses only the Sourcing Event lifecycle in Section 13.
2. `L2-J01` defines the generic Sourcing Event events used in Section 14 but no tender-stage event set for publication, bid receipt, opening, evaluation, or recommendation. Governance must decide whether the generic events are sufficient; this L3 scenario does not invent tender-stage events.
3. Institution-specific statutes, funding rules, procurement manuals, government e-marketplace or e-procurement mandates, notice periods, tender-security requirements, preference policies, and competent authorities cannot be universalised at L3.
4. The canonical governance for single valid bids, abnormally low bids, bid validity extension, arithmetic correction, tie resolution, reverse auction, negotiation, and final offers requires operating-profile configuration.
5. The boundary between correction, clarification, re-evaluation, corrigendum, tender restart, cancellation, and re-tender requires exact state and event governance before locking.

## 17.2 Assumptions

- Formal tendering is distinct from RFQ and RFP because its publication, opening, competition, committee, and award controls are materially stronger.
- Electronic and physical tender channels may coexist; L3 specifies business evidence without choosing technology.
- Bid-security references may influence eligibility, but monetary receipt, refund, or forfeiture remains externally owned.
- A completed tender outcome is necessary but not sufficient for supplier commitment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-006**
