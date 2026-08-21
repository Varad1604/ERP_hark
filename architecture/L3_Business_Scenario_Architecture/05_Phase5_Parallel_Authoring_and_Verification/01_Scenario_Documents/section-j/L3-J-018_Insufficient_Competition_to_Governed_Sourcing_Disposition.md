# Insufficient Competition to Governed Sourcing Disposition

# 1. Scenario Metadata

**Scenario ID:** L3-J-018  
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
**Referenced By:** sourcing-event evaluation, re-sourcing, negotiation, award, cancellation, commitment, supplier-governance, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs the decision required when a competitive sourcing event does not produce the minimum quantity or quality of responsive competition expected by law, policy, method, or the approved sourcing strategy.

It prevents automatic award to the sole or limited respondent, silent relaxation of competition requirements, repeated re-sourcing without analysis, and conversion of a failed competitive event into an unauthorised direct award. It preserves bidder fairness while enabling a reasoned and authorised disposition.

---

# 3. Business Outcome

Insufficient competition reaches a documented and approved disposition: proceed with evaluation or award where expressly permitted, seek clarification, conduct a controlled negotiation, extend or reissue the event, revise and re-source the demand, move to another authorised sourcing method, cancel the affected scope, or reject all responses.

The outcome records the competition deficiency, cause assessment, market and price reasonableness, bidder treatment, authority, demand impact, conditions, validity, and evidence lineage without creating a Purchase Order, contract, receipt, payable, accounting entry, or payment.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an active competitive sourcing event reaches a response, opening, responsiveness, evaluation, or award checkpoint and the valid competitive response population is below the required or expected level.

## 4.2 End Boundary

The scenario ends when an authorised disposition is recorded and effective, bidders are treated consistently, affected demand and event scope are placed on a clear next path, and the original event and evidence remain immutable and auditable.

## 4.3 In Scope

- confirming the event, method, lots, publication, invitation population, response deadline, responses received, responsive responses, and minimum-competition rule;
- distinguishing no response, single response, limited response, non-responsive bids, bidder withdrawal, and collusion or market-integrity concerns;
- testing whether the event was sufficiently open, accessible, correctly specified, fairly timed, and free of avoidable restrictions;
- assessing market conditions, supplier capacity, price and commercial reasonableness, urgency, demand validity, and re-sourcing consequences;
- selecting an authorised disposition by event, lot, line, category, location, or demand portion;
- governing any clarification, negotiation, extension, reissue, method change, or cancellation condition;
- preserving equal treatment, confidentiality, approvals, notifications, demand traceability, and audit evidence.

## 4.4 Out of Scope

- correcting unapproved demand or changing authoritative specifications without demand governance;
- normal evaluation where adequate competition exists;
- supplier registration, qualification, suspension, or blacklisting owned by J02;
- cancellation of an active sourcing event governed by L3-J-019, though cancellation may be the selected hand-off;
- creating a Purchase Order, contract, receipt, payable, accounting entry, or payment;
- technical workflow, portal, notification, or evaluation-engine design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-004 to L3-J-006 | Competitive RFQ, RFP, or tender event produced the response population. |
| Related | L3-J-015 to L3-J-017 | Alternative exceptional or externally recognised routes require their own grounds. |
| Followed by | L3-J-019 | Governs full event cancellation where selected. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns competition sufficiency assessment, cause analysis, fair disposition recommendation, approval, bidder treatment, demand hand-off, and evidence. |

Requesting units retain demand and specification authority. J02 retains supplier truth, J04 retains commitment truth, and Finance retains budget and monetary truth.

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

- The sourcing event, approved demand, method, lots, criteria, publication or invitation, dates, and responsible roles are identifiable.
- The response deadline or relevant evaluation checkpoint has been reached unless an authorised pre-deadline correction is required.
- All timely responses, withdrawals, disqualifications, and responsiveness determinations are preserved.
- The applicable minimum-competition, single-bid, extension, reissue, negotiation, cancellation, and approval rules are effective.
- No award or commitment has been created for the affected scope, or any apparent downstream action is held for investigation.
- Market, supplier, benchmark, urgency, funding, and demand facts can be obtained to the extent required.

---

# 8. Trigger

The event receives no valid response, one valid response, fewer valid responses than required, or an inadequate competitive population after responsiveness review, requiring an explicit disposition before evaluation or award can proceed.

---

# 9. Main Business Flow

1. Procurement records the sourcing-event identity, method, lots, demand references, publication and response dates, invited or accessible market, responses received, withdrawals, and current state.
2. Submission custody, deadline enforcement, opening record, response count, bidder identities, and response integrity are verified before any disposition analysis.
3. Each response is classified as timely or late, responsive or non-responsive, withdrawn or active, and eligible or restricted using the approved criteria without changing those criteria after visibility of bids.
4. The applicable competition expectation is identified by method, value, category, funding, jurisdiction, risk, and institutional rule.
5. Procurement determines whether the issue is no response, single response, limited response, widespread non-responsiveness, bidder withdrawal, supplier concentration, suspected coordination, or another integrity concern.
6. Event design and conduct are reviewed for adequate notice, accessible channels, reasonable response time, neutral specification, proportionate qualification, correct lotting, clear terms, market reach, and consistent clarifications.
7. The requesting and technical owners confirm whether the demand, specification, quantity, delivery need, budget context, and urgency remain valid.
8. Market and category review assesses supplier availability, barriers to entry, capacity, timing, location, aggregation, previous sourcing history, and the likelihood that extension or reissue would improve competition.
9. Where at least one responsive offer exists, its technical acceptability and price or commercial reasonableness are assessed independently of response count using estimates, benchmarks, catalogues, prior prices, market indices, cost analysis, or another defensible basis.
10. Conflicts, bidder relationships, common ownership, bid similarities, communication patterns, and other collusion or integrity indicators are reviewed and escalated where necessary.
11. Procurement develops permitted options and evaluates fairness, legality, urgency, price, operational impact, bidder expectations, re-sourcing cost, demand validity, and institutional risk for each.
12. The recommendation identifies the exact event or lot, deficiency, cause, evidence, chosen disposition, alternatives rejected, conditions, bidder treatment, demand effect, responsible next owner, and validity.
13. Required evaluation, legal, compliance, finance, technical, and oversight reviews are completed without allowing the existing respondent to influence the method decision improperly.
14. The authorised approver approves, conditionally approves, returns, or rejects the recommendation with reason and effective time.
15. If continuation, clarification, or negotiation is authorised, it follows identical recorded rules and cannot introduce material scope or criterion changes that would have attracted other bidders.
16. If extension, reissue, revision, method change, rejection, or cancellation is authorised, the affected event and demand are transitioned without duplicate consumption or loss of original history.
17. Bidders and stakeholders receive authorised communications, and all response, analysis, approval, disposition, transition, actor, and timestamp evidence is retained.

---

# 10. Alternate Business Flows

## 10.1 No Response

Procurement analyses market reach, specification, timing, supplier capacity, and method. The event may be extended where still open and fair, reissued, revised through demand governance, moved to another authorised method, or cancelled.

## 10.2 Single Responsive Bid

The single bid may proceed only where rules expressly permit and the institution establishes event integrity, continuing need, supplier eligibility, technical acceptability, commercial reasonableness, absence of collusion concerns, and correct exceptional approval.

## 10.3 Multiple Responses but Only One Responsive Bid

Non-responsiveness decisions are independently checked against pre-disclosed criteria. Criteria cannot be relaxed selectively. The remaining response follows the single-responsive-bid controls.

## 10.4 Lot-Level Insufficiency

Adequately competed lots continue independently. Only deficient lots receive a separate disposition, with demand and bidder communications kept traceable.

## 10.5 Controlled Clarification

Clarification may resolve ambiguity but cannot cure a material omission, change price or scope improperly, create a new bid, or advantage one respondent contrary to the approved method.

## 10.6 Permitted Negotiation

Negotiation occurs only under an expressly authorised basis, defined objectives, designated roles, recorded exchanges, and price or term reasonableness controls. It does not disguise an unauthorised post-bid auction or direct award.

## 10.7 Suspected Collusion or Market Manipulation

Ordinary disposition is held, evidence is protected, and the matter is escalated to appropriate compliance, legal, competition, supplier-governance, or investigative authority.

## 10.8 Urgent Need Remains

Urgency is reassessed. Emergency or single-source treatment requires the separate grounds and governance of L3-J-016 or L3-J-015 and is not automatically created by a failed competition.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Response Count or Custody Dispute

Disposition is suspended while submission logs, timestamps, opening records, and custody evidence are independently reconciled.

## 11.2 Evaluation Criteria Applied Inconsistently

Evaluation is held, affected determinations are reviewed, and a fair corrective action is authorised without silent score or criterion changes.

## 11.3 Price Cannot Be Demonstrated Reasonable

Continuation or award is blocked pending lawful negotiation, further cost evidence, reissue, scope revision, alternate method, or cancellation.

## 11.4 Market Analysis Inconclusive

The case is returned for proportionate additional evidence or proceeds only under a valid higher-risk authority with explicit limitations.

## 11.5 Bidder Challenge or Complaint

The affected action is held where required, confidentiality is protected, and the complaint follows the applicable review process without overwriting event history.

## 11.6 Disposition Approval Expires or Is Interrupted

The event remains in a non-award state. On resumption, demand, supplier, price, funding, validity, and legal facts are refreshed.

## 11.7 Unauthorised Award or Commitment Attempt

The attempt is blocked and escalated. Any created record is handled by its authoritative owner and does not validate the underlying disposition.

---

# 12. Business Rules and Decision Constraints

- Insufficient competition requires an explicit disposition; it does not create automatic authority to award.
- Minimum competition and single-bid rules are determined from the applicable method, value, category, funding, jurisdiction, and policy.
- Response and responsiveness facts must be established before the disposition decision.
- Evaluation criteria cannot be changed after bid visibility to manufacture responsiveness or competition.
- A fair and adequately publicised event may support permitted single-bid continuation only with independent technical, commercial, supplier, integrity, and approval controls.
- Price reasonableness must be demonstrated separately from response count.
- Extension or reissue must preserve equal access and cannot disclose protected competitor information.
- Negotiation requires express authority, defined scope, documented roles, and immutable exchanges.
- Failed competition does not itself prove emergency, exclusivity, or proprietary status.
- Suspected collusion, common control, fraud, or bid manipulation requires specialist escalation.
- Lot-level dispositions must preserve aggregate demand and prevent duplicate sourcing or threshold avoidance.
- Bid security, proposal validity, confidentiality, and return or release obligations remain governed.
- No bidder may be promised an award before authorised disposition and award approval.
- This scenario does not create a Purchase Order, contract, receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Sourcing Event | Existing canonical sourcing position | Applies the approved disposition without creating a separate disposition lifecycle. | Continues at an existing lifecycle position or reaches `Cancelled`, `No Response`, `No Suitable Bid` or `Re-Sourcing Required` | Procurement Planning & Sourcing (J01) | `L2-J01` Section 12.3 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `NoResponseOutcomeDeclared` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `NoSuitableBidOutcomeDeclared` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ReSourcingRequested` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ReSourcingEventCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Requesting Unit / J03 | Confirms continuing demand, specification, urgency, and revision needs. | Retains demand authority. |
| J02 | Supplies supplier identity, eligibility, ownership, restriction, and integrity facts. | Retains supplier lifecycle. |
| J04 | Receives only a valid award outcome after separate approval. | Retains commitment lifecycle. |
| Finance / Funding Owner | Supplies funding, benchmark, value, bid-security, and monetary-control facts. | Retains financial truth. |
| Legal / Compliance / Oversight | Reviews fairness, method, complaints, negotiation, collusion, and authority. | Retains specialist authority. |
| L3-J-019 | Receives an approved cancellation hand-off. | Retains cancellation governance. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- approved demand, sourcing strategy, event, method, lots, criteria, publication or invitation, clarification, deadline, and opening record;
- all responses, withdrawals, timestamps, eligibility and responsiveness determinations, and custody evidence;
- applicable competition rule, deficiency classification, event-design review, market and supplier analysis, prior sourcing history, and urgency;
- benchmarks, independent estimate, price and commercial analysis, conflicts, ownership, collusion indicators, and specialist reviews;
- options considered, recommendation, conditions, approval, bidder communications, demand transition, actors, versions, reasons, and timestamps.

## 15.3 Audit Obligations

The institution must reconstruct why competition was insufficient, whether the event was fair and accessible, how responses were classified, how price and market conditions were tested, why the selected disposition was lawful and proportionate, who approved it, and how bidders and demand were treated afterward.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- an insufficient-competition condition has been identified and documented with a verifiable factual basis for the conclusion;
- the disposition authority has reviewed the condition, alternatives, and institutional risk before approving a governed path;
- the chosen disposition—re-tender, modified scope, negotiated award, sole-source authorisation, demand deferral, or cancellation—is approved with explicit authority, conditions, and lineage;
- the original sourcing event, bid evidence, competition analysis, and disposition decision are preserved and not altered;
- re-tender or successor events reference the original competition history and corrective scope changes;
- no supplier is unfairly advantaged or disadvantaged by the disposition decision.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the sourcing event has an approved insufficient-competition disposition that is traceable, auditable, and immutable;
- where re-tender or modified sourcing is authorised, a new sourcing event has a distinct identity and preserved predecessor lineage;
- where negotiated award or sole-source authorisation is approved, the justification satisfies institutional policy and the authority level appropriate to the demand value and risk;
- where demand is deferred or cancelled, the demand lifecycle reflects the approved disposition;
- no original bidder evidence is destroyed, and all competition and disposition facts remain reconstructable.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Institution-specific minimum-response, single-bid, extension, negotiation, reissue, publication, and approval rules require confirmation.
2. The exact boundary between clarification and material bid alteration requires method-specific governance.
3. Mandatory independent review and price-analysis standards for single responsive bids require operating-profile definition.

## 17.2 Assumptions

- The source event is competitive and has not yet produced an approved commitment.
- Insufficient competition is a decision condition, not an automatic sourcing outcome.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-018**
