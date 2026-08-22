# Urgent Approved Requirement to Governed Emergency Procurement Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-016  
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
**Referenced By:** downstream award, commitment, supplier-governance, receipt, payable, continuity, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs procurement when an actual emergency requires action faster than ordinary sourcing can safely or lawfully provide. It preserves necessity, proportionality, supplier due diligence, price stewardship, chronology, approval, and retrospective accountability while allowing a bounded accelerated response.

It prevents poor planning, avoidable delay, convenience, budget expiry, or an anticipated recurring need from being misclassified as an emergency.

---

# 3. Business Outcome

An urgent approved requirement reaches an Approved Emergency Procurement Outcome, Conditional Outcome, Returned outcome, or Rejected outcome. Approval defines the emergency, immediate scope, supplier-selection basis, quantity, value, time window, conditions, authority, required ratification, transition path, and evidence without itself creating a Purchase Order, receipt, payable, accounting entry, or payment.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised role identifies an urgent approved requirement arising from an immediate threat to life, safety, assets, essential operations, legal compliance, disaster response, or another permitted emergency ground.

## 4.2 End Boundary

The scenario ends when the emergency sourcing decision is approved, conditionally approved, returned, or rejected; any immediate action and retrospective review are attributable; and residual demand is transferred to normal procurement when the emergency window ends.

## 4.3 In Scope

- validating the emergency event, discovery time, causal facts, urgency, and permitted ground;
- confirming requirement approval or lawful provisional authority where prior approval is impossible;
- limiting scope, quantity, value, supplier coverage, geography, and duration to immediate necessity;
- performing accelerated but proportionate supplier eligibility, capability, conflict, and price checks;
- recording contemporaneous decisions and actual chronology without backdating;
- applying emergency delegation, duty separation, conditions, ratification, reporting, and oversight;
- transitioning residual or continuing demand to a normal sourcing route;
- preserving evidence for independent post-event review.

## 4.4 Out of Scope

- using emergency rules to remedy ordinary planning failure or administrative delay;
- indefinite or recurring purchases after the emergency has stabilised;
- supplier master lifecycle, Purchase Order lifecycle, receipt, payable, accounting, or payment;
- rewriting dates or creating retroactive evidence;
- technical workflow or integration design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-004 to L3-J-007 | Govern normal sourcing once time permits. |
| Related | L3-J-015 | Distinguishes emergency urgency from single-source exclusivity. |
| Followed by | J04 commitment scenarios | Separately govern the resulting commitment. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns emergency-method validation, accelerated sourcing, bounded recommendation, approval, ratification coordination, transition, and audit evidence. |

The operational incident owner retains emergency facts and need accountability. J02 retains supplier truth, J04 commitment truth, and Finance monetary truth.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Officer | Administers the governed exception, external-procurement or closure activity. | Performs procurement administration. |
| Requesting Unit Representative | Supplies requirement, urgency or operational context without approving the procurement outcome. | Provides business input. |
| Procurement Recommender | Records the attributable recommendation and supporting basis. | Recommends. |
| Procurement Approver | Approves, rejects or returns the outcome within established authority. | Decides. |
| External Procurement Coordinator | Records externally executed procurement where applicable without changing the external record. | Coordinates external-procurement recognition only. |
| Auditor / Reviewer | Reviews authority, evidence and decision history without modifying the outcome. | Reviews evidence. |
---

# 7. Preconditions and Entry Criteria

- An urgent requirement and emergency event or condition are identifiable.
- The immediate consequence of delay is documented.
- The requirement is approved, or a valid emergency rule permits provisional action followed by explicit ratification.
- Minimum necessary scope, quantity, location, delivery window, estimated value, and operational owner can be identified.
- Emergency grounds, delegations, financial limits, reporting, ratification, conflict, and recordkeeping rules are effective.
- Supplier identity and minimum capability or restriction facts can be checked to the extent circumstances permit.

---

# 8. Trigger

An authorised incident owner or requester submits an urgent need where ordinary sourcing time would create unacceptable immediate harm and requests use of the emergency procurement method.

---

# 9. Main Business Flow

1. The incident owner records the event, actual discovery time, affected operations or persons, immediate risk, expected consequence of delay, and responsible authority.
2. Procurement validates that the asserted ground is permitted and distinguishes true emergency from poor planning, avoidable delay, convenience, expiring funds, or routine recurrence.
3. Requirement approval or lawful provisional emergency authority is verified and linked to the action.
4. The minimum immediate goods, works, or services, location, quantity, service level, delivery time, estimated value, and end condition are defined.
5. Scope is divided between immediate stabilisation and residual or longer-term need; only immediate necessity remains on the emergency route.
6. Procurement identifies capable suppliers using available market, framework, catalogue, prior performance, geographic, technical, safety, and continuity information.
7. Supplier identity, eligibility, restrictions, conflicts, capacity, delivery ability, and safety or compliance qualifications are checked proportionately to available time and risk.
8. Where time permits, rapid quotations or comparable commercial information are obtained; where it does not, the reason and available price evidence are recorded.
9. Price and total commercial reasonableness are assessed using historic prices, catalogues, benchmarks, independent estimates, open-book cost, negotiation, or post-action verification.
10. Procurement records the actual decision chronology, contacts, offers, evidence available at each point, limitations, and any control not practicable before action.
11. The recommendation specifies supplier, minimum scope, quantity, value limit, duration, delivery need, risks, conditions, conflicts, funding context, and ratification or reporting obligations.
12. Available specialist and independent review is obtained without causing disproportionate harm through delay.
13. The emergency approver approves, conditionally approves, limits, returns, rejects, or stops the case with reason and effective time.
14. An approved outcome authorises only the stated emergency sourcing scope and does not by itself create a commitment or confirm delivery.
15. If provisional action preceded full approval under an explicit rule, the complete record proceeds promptly to the designated ratifying authority; ratification does not erase chronology or control exceptions.
16. Procurement monitors the emergency end condition. When stabilisation occurs, new emergency use stops and remaining demand moves to normal planning and sourcing.
17. Evidence, actual results, deviations, value, supplier performance, ratification, lessons, corrective actions, actors, and timestamps are retained and reported as required.

---

# 10. Alternate Business Flows

## 10.1 Immediate Life or Safety Action

Minimum action may proceed under valid provisional authority before the full record is complete. The actor records what was known, why delay was unacceptable, and completes mandatory review without backdating.

## 10.2 Rapid Competition Is Feasible

Procurement obtains and documents rapid quotations from available capable suppliers while preserving the emergency time constraint and evaluation basis.

## 10.3 Only One Supplier Is Immediately Capable

The emergency basis and immediate-capability basis are both recorded. Lack of time is not automatically treated as proprietary exclusivity.

## 10.4 Funding Not Yet Fully Confirmed

Action proceeds only where an explicit emergency financial rule permits it, within the defined limit and with required subsequent regularisation. Procurement approval does not create funding truth.

## 10.5 Emergency Extends

Any extension requires refreshed facts, remaining harm, consumption, market options, value, duration, and authority. Extensions cannot become an indefinite substitute for normal procurement.

## 10.6 Emergency Ends Before Award

The emergency case is stopped or cancelled and the remaining requirement is routed through an appropriate normal method.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Emergency Ground Not Sustained

The case is rejected or returned to normal procurement; any unauthorised action is separately investigated and governed.

## 11.2 Supplier Facts Unavailable

Only the minimum authorised action may proceed where risk of delay outweighs residual supplier risk. Missing checks are explicitly recorded and completed as soon as practicable.

## 11.3 Price Cannot Be Established Before Action

A value ceiling, negotiation basis, cost transparency, and prompt post-action review are applied where permitted. Uncertainty is not hidden.

## 11.4 Approver Unavailable

Only a documented alternate emergency delegation may be used. Absence of an approver does not create implied authority.

## 11.5 Ratification Refused

The refusal, reasons, incurred facts, remediation, accountability, and downstream legal or financial handling are recorded without falsifying the original action.

## 11.6 Duplicate Emergency Purchase

Overlapping requirements and commitments are reconciled; unused demand is released and duplicate commitment risk is escalated to J04.

## 11.7 Post-Event Evidence or Allegation

New facts are appended, independently reviewed, and may trigger supplier, disciplinary, legal, financial, or audit action without overwriting the original record.

---

# 12. Business Rules and Decision Constraints

- Emergency use requires a permitted ground and a demonstrable unacceptable consequence of ordinary sourcing delay.
- Poor planning, administrative delay, convenience, or expiring budget is not by itself an emergency.
- Scope, quantity, value, duration, geography, and supplier coverage must be limited to immediate necessity.
- Actual chronology must be preserved; records, approvals, offers, or deliveries cannot be backdated.
- Retrospective ratification is allowed only where an explicit rule authorised provisional action and does not convert unauthorised action into compliant action automatically.
- Separation of requester, selector, approver, receiver, and payer applies as far as practicable; any compression is declared and reviewed.
- Supplier and price checks are proportionate to time and risk, not omitted without explanation.
- Emergency authority ends when the stated condition ends; residual demand follows normal procurement.
- Extensions require new evidence and authority.
- Emergency approval cannot confirm receipt, supplier performance, funding, payable, accounting, or payment.
- All exceptions and compensating controls remain visible to oversight.

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
| `EmergencyProcurementDeclared` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `EmergencyProcurementApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `EmergencyProcurementReviewCompleted` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Incident / Requesting Unit | Supplies emergency facts, minimum need, and stabilisation status. | Retains operational authority. |
| J02 | Supplies supplier identity, eligibility, restriction, and performance facts. | Retains supplier lifecycle. |
| J04 | Converts approved outcome into separately governed commitment and receives duplicate or cancellation facts. | Retains commitment lifecycle. |
| Finance | Supplies funding, limits, financial regularisation, and incurred monetary facts. | Retains financial truth. |
| Legal / Compliance / Safety | Advises on emergency basis, waivers, safety, reporting, and consequences. | Retains specialist authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- event, actual discovery time, threat, consequence of delay, emergency ground, and incident owner;
- approved or provisionally authorised requirement, minimum scope, quantity, location, time, value, and end condition;
- suppliers considered, capability, eligibility, restrictions, conflicts, offers, market information, and selection reason;
- price evidence, negotiation, ceiling, funding context, delivery expectation, risks, and compensating controls;
- actual chronology, approvals, deviations, ratification, stabilisation, residual-demand transition, results, actors, and timestamps.

## 15.3 Audit Obligations

The institution must reconstruct what happened and when, what was known at each decision, why ordinary procurement was unsafe or impracticable, how scope and value were limited, how the supplier and price were tested, which controls were compressed, and how emergency use ended.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the emergency condition is documented with a verifiable urgency basis, onset time, risk to institutional operations, and authority confirmation;
- an emergency procurement method has been authorised by a role with delegated emergency authority appropriate to the demand value and category;
- the sourcing outcome is traceable to the emergency demand with a unique identity, approved scope, and time-bound authority;
- the emergency authority, justification, approval, conditions, and limitations are preserved as immutable evidence;
- post-emergency reporting and ratification obligations required by institutional policy are explicitly identified;
- demand covered under emergency authority is not duplicated by a normal competitive sourcing event.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the emergency procurement outcome is approved, recorded, and available for downstream commitment and delivery scenarios;
- the emergency authority, justification, and time-bound scope are permanently preserved and auditable;
- post-procurement ratification, review, and exception reporting obligations are captured and attributed to their institutional owners;
- the emergency sourcing outcome cannot be retrospectively altered to disguise the emergency basis;
- no normal competitive procurement obligation has been waived for reasons other than the documented emergency condition.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Institution-specific emergency grounds, value limits, alternate delegations, reporting periods, and ratification deadlines require confirmation.
2. Minimum pre-action checks by emergency category and risk require operating-profile governance.
3. Treatment of incurred obligations where ratification is refused requires legal and financial mapping.

## 17.2 Assumptions

- Emergency authority is exceptional, temporary, and no broader than immediate necessity.
- An approved sourcing outcome remains distinct from a Purchase Order, receipt, payable, or payment.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-016**
