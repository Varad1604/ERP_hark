# Approved Procurement Demand to Direct or Simplified Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-007  
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
**Referenced By:** downstream J04 Purchase Order scenarios consuming an approved sourcing outcome; L3-J-019 Active Sourcing Event to Governed Cancellation where an active event is cancelled  

---
# 2. Scenario Purpose

This scenario governs direct or simplified sourcing when approved procurement demand qualifies for a reduced competitive process or a direct supplier approach under applicable policy. It prevents procedural convenience, urgency, low value, compatibility, proprietary conditions, continuity, or limited market availability from becoming an undocumented bypass of procurement control.

The scenario establishes a justified and approved sourcing outcome supported by supplier eligibility, scope, commercial evidence, price reasonableness, conflicts, authority, and audit history. It ends before Purchase Order, work order, contract, liability, receipt, or payment creation.

---

# 3. Business Outcome

Approved procurement demand is resolved through an authorised direct or simplified sourcing method and results in a traceable approved supplier and commercial sourcing outcome, or an explicitly governed rejection, cancellation, or referral to another sourcing method.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when J01 receives valid approved procurement demand that is proposed for direct or simplified sourcing under an applicable policy ground, threshold, category, market condition, or authorised exception.

## 4.2 End Boundary

The scenario ends when the exact supplier, scope, evaluated commercial reference, method justification, and conditions are approved as the sourcing outcome, or when the request is rejected, cancelled, returned, or redirected to competitive sourcing.

## 4.3 In Scope

- validating approved-demand authority and completeness;
- identifying the proposed direct or simplified method and governing policy ground;
- verifying threshold, aggregation, urgency, exclusivity, compatibility, continuity, standardisation, market availability, or other applicable justification;
- detecting artificial demand splitting or repeated use intended to avoid competition;
- validating proposed supplier identity, eligibility, conflicts, and related-party disclosures;
- obtaining and preserving quotation, proposal, catalogue, rate, estimate, or other commercial evidence appropriate to the method;
- assessing specification compliance, delivery, warranty, service, terms, risk, and price reasonableness;
- recording market research, historical price, benchmark, published rate, prior procurement, negotiation, or independent estimate evidence where applicable;
- reviewing method justification, exceptions, deviations, and approval authority;
- approving the direct or simplified outcome or redirecting the demand to RFQ, RFP, tender, re-sourcing, or cancellation;
- publishing the approved outcome for downstream commitment.

## 4.4 Out of Scope

- procurement-plan and demand creation or approval;
- supplier-master creation, qualification lifecycle, suspension, and blacklisting, owned by J02;
- competitive quotation or RFQ sourcing, governed by L3-J-004;
- RFP sourcing, governed by L3-J-005;
- formal tender sourcing, governed by L3-J-006;
- emergency procurement, single-source procurement, rate-contract call-off, or external marketplace procurement where separate canonical scenarios govern them;
- Purchase Order, work order, letter of acceptance, contract execution, amendment, or supplier commitment;
- budget commitment, accounting, payable, receipt, inventory, asset, or payment processing;
- legal opinion, technical portal, API, schema, calculation engine, or user-interface design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-004 Approved Procurement Demand to Competitive Quotation or RFQ Sourcing Outcome | Receives demand when simplified or direct authority is not valid but competitive quotation is appropriate. |
| Related | L3-J-019 Active Sourcing Event to Governed Cancellation | Governs authorised cancellation after an active sourcing event exists. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns method qualification, Sourcing Event, supplier approach, commercial evidence, reasonableness assessment, exception governance, recommendation, approval, and final sourcing outcome. |

J01 consumes approved demand from J03 and supplier facts from J02 without changing either aggregate. Requesting or specialist functions may confirm compatibility, exclusivity, standardisation, urgency, technical compliance, continuity, or market conditions. J01 remains accountable for the procurement-method decision and outcome.

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

- J03 has produced identifiable approved procurement demand; draft, rejected, returned, withdrawn, or superseded demand is not valid normal sourcing authority.
- Requirement, scope, quantity, specifications or outcomes, delivery expectations, and acceptance context are sufficiently defined.
- A direct or simplified method is potentially allowed by applicable policy, threshold, approved category, standardisation, compatibility, proprietary condition, continuity need, market limitation, or other governed basis.
- The demand has not been artificially divided, sequenced, or repeatedly raised to avoid a competitive threshold or approval authority.
- Estimated-value and budget-validation references required by policy are traceable.
- Proposed supplier identity and eligibility can be resolved through J02.
- Conflict-of-interest, related-party, and requester-supplier relationship disclosures can be obtained where required.
- Applicable approval, exception, quotation, market-check, benchmarking, negotiation, and documentary requirements are effective.
- No conflicting active sourcing event governs the same demand unless explicitly authorised.

---

# 8. Trigger

Approved procurement demand is accepted by J01 and is proposed for direct or simplified sourcing under a stated policy ground.

---

# 9. Main Business Flow

1. The Procurement Officer accepts the approved demand and records its authoritative J03 reference.
2. The officer validates demand status, scope, quantity, specifications or outcomes, delivery needs, category, value reference, requesting unit, and governing policy.
3. The proposed method is classified as simplified competition, direct supplier approach, low-value purchase, catalogue or published-rate purchase, compatibility or continuity purchase, proprietary purchase, or another permitted configured basis.
4. The officer records the exact policy ground, threshold or exception provision, supporting facts, anticipated value, and required authority.
5. Related demand, recent similar purchases, cumulative category or supplier spend, and split indicators are reviewed where policy requires aggregation.
6. If the method basis is not valid, the demand is returned or redirected to RFQ, RFP, tender, or another governed route.
7. A controlled Sourcing Event is created and linked to approved demand, procurement plan where applicable, method, justification, supplier candidate, policy, and responsible actors.
8. J02 supplier identity, category eligibility, qualification, suspension, blacklisting, and other applicable status facts are checked and preserved for the decision time.
9. Required conflict-of-interest, related-party, exclusivity, compatibility, standardisation, urgency, continuity, or market-availability declarations and supporting evidence are obtained.
10. The proposed supplier receives a controlled request or the institution obtains an authorised published catalogue, rate, estimate, proposal, or other commercial reference.
11. Received commercial evidence is time-stamped, versioned, and linked to the supplier, demand, event, scope, quantity, validity period, and applicable terms.
12. Technical or operational compliance is confirmed against the approved demand without silently changing the demand.
13. Price and commercial reasonableness are assessed using evidence appropriate to the method, such as market checks, comparable quotations, recent purchases, published prices, framework rates, independent estimate, cost information, total-cost considerations, or documented negotiation.
14. Original supplier terms remain preserved when evaluated, normalised, or negotiated views are recorded.
15. Delivery, warranty, service, support, taxes and charges, payment terms, validity, deviations, risk, and dependency conditions are reviewed.
16. Where negotiation is permitted, the institution records the pre-negotiation offer, negotiated subjects, participants, authority, supplier response, final offer, and reasons.
17. The Procurement Recommender records the proposed supplier, lines, quantities or scope, evaluated commercial reference, method justification, price-reasonableness basis, risks, conditions, and deviations.
18. The Procurement Manager reviews the evidence, including aggregation, repeated use, supplier status, conflicts, benchmark sufficiency, and policy compliance.
19. Any required method-exception authority evaluates and approves or rejects the use of the direct or simplified route.
20. The final Procurement Approver reviews the exact recommendation and delegated authority and may approve, reject, return, cancel, or redirect it.
21. On approval, the exact recommendation becomes the approved sourcing outcome and the Sourcing Event reaches Completed.
22. The outcome is made available for downstream commitment without J01 creating a Purchase Order, contract, liability, receipt, or payment.

---

# 10. Alternate Business Flows

## 10.1 Simplified Competitive Check

Where policy requires a small number of informal or documented market checks rather than a full RFQ, each supplier or source, request, response, timestamp, scope, price, terms, non-response, and selection reason is preserved. The process does not masquerade as a full competitive RFQ.

## 10.2 Low-Value Purchase

Where anticipated and aggregated value is within a permitted low-value route, the required quotation, catalogue, cash-memo, market-check, certification, or approval evidence is obtained. Value alone does not waive supplier, conflict, budget, or audit controls.

## 10.3 Proprietary, Sole-Available, or Compatibility Basis

The requester or specialist provides evidence of exclusivity, compatibility, intellectual-property constraint, standardisation, warranty dependency, interoperability, or absence of reasonable substitutes. Procurement independently reviews the evidence and applicable alternatives before recommendation.

## 10.4 Continuity or Additional-Scope Basis

Where continuity with an existing supplier is proposed, the institutional benefit, switching risk, existing commitment reference, scope relationship, rate basis, cumulative value, duration, and prohibition on improper scope expansion are reviewed.

## 10.5 Catalogue, Published Rate, or Framework Reference

Where a valid published catalogue, regulated rate, institutional framework, or other approved price reference is used, version, validity, scope fit, applicable discount, delivery, and supplier eligibility remain traceable.

## 10.6 Negotiated Direct Outcome

Negotiation occurs only with authority and a recorded objective. Original and final offers, participants, matters negotiated, concessions, unresolved deviations, and reasonableness conclusion remain evidence.

## 10.7 Supplier Rejected or Unavailable

If the proposed supplier is ineligible, conflicted, unwilling, unavailable, non-responsive, or unable to meet the requirement, the outcome is returned for another eligible supplier, competitive sourcing, cancellation, or authoritative demand correction.

## 10.8 Method Returned or Redirected

The approver may determine that direct or simplified sourcing is not justified. The prior request and reasons remain preserved and the demand is referred to the appropriate governed sourcing method without creating commitment.

## 10.9 Multi-Line or Partial Outcome

Where policy allows only some demand lines to use the simplified or direct basis, the approved outcome identifies each covered line and redirects remaining lines to the applicable method. Threshold and aggregation tests consider the whole relevant demand.

## 10.10 Recommendation Returned or Rejected

The recommendation may be returned for missing evidence, revised reasonableness analysis, corrected scope, or re-evaluation, or rejected. Prior versions remain preserved.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid or Changed Demand

If demand approval is absent, withdrawn, superseded, or materially changed, sourcing is withheld or suspended until J03 provides authoritative resolution.

## 11.2 Invalid Method Basis

If threshold, policy ground, exclusivity, urgency, compatibility, standardisation, continuity, or market-limitation evidence is insufficient, the direct or simplified method is rejected or redirected. Convenience alone is not a valid exception.

## 11.3 Suspected Demand Splitting or Repeated Avoidance

Processing is suspended and escalated when related purchases appear divided or repeated to remain below competition or authority thresholds. Evidence, cumulative view, decision, and corrective route are recorded.

## 11.4 Supplier Eligibility or Conflict Issue

If supplier status, ownership, related-party position, conflict, suspension, or blacklisting prevents ordinary participation, the recommendation is withheld pending J02 or competent-authority resolution.

## 11.5 Missing or Unreliable Commercial Evidence

If quotation, catalogue, estimate, benchmark, or market evidence is incomplete, expired, inconsistent, or unverifiable, the assessment is returned for correction, further evidence, competitive sourcing, or cancellation.

## 11.6 Price Reasonableness Not Established

If value cannot be reasonably supported, approval is withheld. Further market research, independent estimate, negotiation, competitive sourcing, scope correction, or cancellation may follow.

## 11.7 Supplier Changes Offer or Withdraws

The earlier offer remains preserved. A revised offer is evaluated as a successor version if permitted; otherwise the case is returned, redirected, or cancelled.

## 11.8 Approval Interruption

An interrupted approval leaves the outcome unapproved and prevents downstream commitment. Processing resumes against the same version unless governed change requires a successor.

## 11.9 Post-Completion Error

A completed sourcing outcome is not destructively rewritten. Before downstream commitment, authorised cancellation, correction, or re-sourcing applies. After commitment, the owning downstream process governs consequences.

---

# 12. Business Rules and Decision Constraints

- Direct or simplified sourcing must originate from valid approved demand.
- The method must be permitted by an effective policy ground, threshold, category, standardisation, compatibility, proprietary condition, continuity need, limited market, or approved exception.
- Convenience, preference for a familiar supplier, lack of planning, or avoidable delay is not by itself sufficient justification.
- Related demand and cumulative value must be considered where policy requires aggregation.
- Artificial splitting, sequencing, or repeated use to avoid competition or approval thresholds is prohibited.
- A method exception and final supplier selection must use the required authority and separation.
- Supplier identity and eligibility remain J02-owned facts and must be preserved at the decision time.
- Conflict-of-interest and related-party disclosures are mandatory where applicable.
- Proprietary, compatibility, continuity, or sole-availability claims require attributable specialist evidence and procurement review.
- Original quotations, catalogues, proposals, benchmarks, estimates, and negotiated offers must be versioned and cannot be overwritten.
- Price reasonableness must be supported by evidence proportionate to value, risk, and available market information.
- Absence of competition does not remove the obligation to assess technical suitability, delivery, warranty, service, risk, and commercial terms.
- A lowest observed price is not automatically reasonable if scope, quality, timing, risk, or terms differ.
- Normalised or evaluated values must remain distinguishable from supplier-originated facts.
- Deviations and negotiation outcomes require reasons, authority, and retained versions.
- An ineligible, suspended, or blacklisted supplier cannot receive normal approval unless an authoritative governed exception exists.
- If the method basis fails, the demand must be redirected rather than silently processed as direct sourcing.
- An approved sourcing outcome does not itself create a Purchase Order, contract, budget commitment, payable, receipt, accounting entry, asset, stock, or payment.

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
| `ProcurementMethodDetermined` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SupplierResponseRecorded` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `CommercialEvaluationFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `AwardRecommendationApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Domain / Capability | Collaboration | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Supplies approved demand, justification, scope, urgency facts, and budget-validation references. | J03 retains demand ownership. |
| J02 Vendor and Supplier Management | Supplies supplier identity, eligibility, qualification, suspension, blacklisting, and related facts. | J02 retains supplier ownership. |
| Requesting or specialist domain | Supplies compatibility, proprietary, technical, continuity, urgency, standardisation, or market evidence. | Specialist retains its authoritative semantics; J01 decides sourcing method. |
| Finance | Supplies budget, historical price, monetary, tax, or benchmark references where required. | Finance retains monetary truth; J01 performs procurement evaluation only. |
| J04 Purchase Order Management or authorised contracting capability | Consumes approved sourcing outcome for downstream commitment. | Downstream owner alone creates commitment. |
| Enterprise document capability | Preserves requests, quotations, evidence, decisions, and approvals. | J01 owns business relationships, not storage design. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- approved demand and applicable policy references;
- method classification, threshold, exception ground, and authority;
- aggregation, cumulative spend, related-demand, and split checks where required;
- event identity and controlled supplier request or commercial source;
- supplier identity and eligibility snapshot;
- conflict, related-party, proprietary, compatibility, urgency, continuity, standardisation, or market declarations where applicable;
- quotation, proposal, catalogue, rate, estimate, benchmark, historical purchase, market check, and independent evidence used;
- technical compliance, scope fit, delivery, warranty, service, validity, and deviation assessment;
- original and evaluated commercial positions;
- price-reasonableness reasoning and negotiation evidence;
- reviewers, recommender, exception approver, final approver, authority, decisions, reasons, and timestamps;
- approved or alternative terminal sourcing outcome.

## 15.3 Audit Obligations

The scenario must permit reconstruction from approved demand through method selection, aggregation checks, supplier choice, commercial evidence, reasonableness assessment, recommendation, authority, and final outcome. Significant actions, changes, actors, timestamps, reasons, versions, conflicts, and correlations remain attributable.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- valid approved demand and applicable direct or simplified policy authority are traceable;
- threshold, aggregated value, related demand, repeated use, and split indicators have been addressed as required;
- supplier identity, eligibility, conflicts, and related-party position are evidenced;
- the method justification and specialist evidence are complete and attributable;
- supplier-originated commercial evidence and all versions are preserved;
- scope, quantity, technical fit, delivery, warranty, service, validity, and deviations have been reviewed;
- price reasonableness is supported by proportionate evidence;
- original and evaluated or negotiated values remain distinguishable;
- required method exception and final recommendation approvals are valid;
- downstream commitment can consume the outcome without J01 creating commitment;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the event reaches Completed with the exact approved sourcing outcome;
- complete audit and documentary evidence is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. `L2-J01` does not define method-specific lifecycle states for low-value, simplified, direct, proprietary, compatibility, continuity, or limited-market routes. Governance must decide whether supporting records require canonical lifecycles; this L3 scenario uses only the Sourcing Event lifecycle in Section 13.
2. `L2-J01` defines the generic Sourcing Event events used in Section 14 but no direct/simplified-method event set for method approval, supplier approach, offer receipt, or recommendation. Governance must decide whether the generic events are sufficient; this L3 scenario does not invent method-specific events.
3. Institution-specific procurement manuals, funding conditions, thresholds, aggregation windows, quotation counts, certificates, and competent authorities cannot be universalised at L3.
4. The precise boundary between this scenario and separate emergency, single-source, rate-contract, marketplace, or repeat-order scenarios requires final canonical cross-reference validation.
5. Price-reasonableness evidence sufficiency, repeated-use monitoring, and related-party approval require operating-profile configuration before implementation locking.

## 17.2 Assumptions

- Direct and simplified methods share one canonical scenario because both reduce ordinary competition but require explicit policy basis, evidence, and approval.
- Supplier self-service is not required; authorised personnel may record commercial evidence received through approved channels.
- Procurement comparison does not create Finance-owned accounting interpretation.
- A completed sourcing outcome is necessary but not sufficient for supplier commitment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-007**
