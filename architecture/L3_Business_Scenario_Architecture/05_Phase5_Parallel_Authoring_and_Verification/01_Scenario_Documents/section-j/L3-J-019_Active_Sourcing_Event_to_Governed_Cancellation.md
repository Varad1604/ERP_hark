# Active Sourcing Event to Governed Cancellation

# 1. Scenario Metadata

**Scenario ID:** L3-J-019  
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
**Referenced By:** demand release, re-sourcing, bidder communication, security release, supplier-governance, commitment prevention, reporting, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs cancellation of an active sourcing event before a valid downstream commitment is created. It ensures cancellation is based on an authorised, documented, non-discriminatory reason and that bidders, confidential submissions, securities, internal demand, approvals, evaluation work, and public or regulatory obligations receive a controlled disposition.

It prevents cancellation being used to favour a supplier, avoid an unfavourable evaluation, conceal process failure, evade an award recommendation, manipulate prices, or erase the historical record.

---

# 3. Business Outcome

An active sourcing event, or an independently separable part of it, reaches Governed Cancellation with an approved reason, effective time, affected scope, bidder and security treatment, demand release or successor path, downstream prevention, notification, reporting, and immutable evidence.

Where cancellation is not approved, the event remains on an explicitly authorised continuation, correction, suspension, or review path.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised role proposes cancellation of an active sourcing event or affected lot because of demand withdrawal, material error, funding loss, legal or policy change, market failure, integrity concern, force majeure, changed institutional need, insufficient competition disposition, or another permitted ground.

## 4.2 End Boundary

The scenario ends when cancellation is approved or rejected; the event state and downstream controls are effective; bidders and stakeholders are addressed; demand, securities, submissions, and successor needs are dispositioned; and all records remain auditable.

## 4.3 In Scope

- validating event identity, lifecycle stage, lots, demand, authority, responses, evaluation status, recommendation, award status, and downstream commitments;
- establishing the cancellation ground, chronology, materiality, alternatives, urgency, bidder impact, and legal or commercial consequences;
- distinguishing full cancellation, partial cancellation, lot cancellation, temporary suspension, correction, extension, reissue, and award rejection;
- approving or rejecting cancellation before commitment;
- stopping new event actions and preventing award or commitment for cancelled scope;
- governing bidder notice, confidentiality, submission retention, bid or proposal security, fee, document, and complaint treatment;
- releasing or redirecting demand and preserving re-sourcing or successor lineage;
- publishing required cancellation facts while protecting confidential information.

## 4.4 Out of Scope

- cancelling or terminating an executed Purchase Order, contract, receipt, payable, accounting entry, or payment;
- supplier suspension, blacklisting, or disciplinary determination owned by J02;
- changing approved demand, funding, or specification without its authoritative governance;
- deleting bids, scores, recommendations, approvals, communications, or event history;
- technical workflow, portal, record-retention, or notification implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-004 to L3-J-018 | The active event or approved disposition may create the cancellation need. |
| Related | L3-J-008 and L3-J-009 | Consolidated or split demand may require partial lineage-preserving cancellation. |
| Followed by | Planning, demand, or sourcing scenarios | Released demand may be revised, deferred, closed, or re-sourced. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns cancellation assessment, recommendation, approval coordination, event state, bidder treatment, demand hand-off, notification, and audit evidence. |

Requesting units retain demand authority. J02 retains supplier truth, J04 retains any commitment truth, and Finance retains funding, security, refund, and monetary truth.

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

- The sourcing event and affected event, lot, line, category, location, or demand scope are identifiable.
- The event is planned, published, open, closed for response, under evaluation, recommended, or otherwise active, but no valid commitment governs the affected scope.
- The proposed cancellation ground and supporting facts are recorded.
- Applicable cancellation, suspension, correction, notice, publication, complaint, security, fee, retention, and approval rules are effective.
- Responses, evaluation records, bidder communications, securities, fees, demand consumption, and downstream attempts can be identified.
- An appropriate authority exists to decide cancellation at the current event stage.

---

# 8. Trigger

A material condition makes continued sourcing unnecessary, unlawful, unfair, unviable, unsafe, unaffordable, procedurally defective, or contrary to institutional interest, and an authorised role submits a cancellation proposal.

---

# 9. Main Business Flow

1. The requester records the cancellation proposal, event identity, affected scope, ground, discovery time, urgency, evidence, and initiating authority.
2. Procurement verifies the event lifecycle, publication and response dates, lots, bidders, submissions, clarifications, opening, evaluation, recommendation, award notices, approvals, and any downstream commitment attempt.
3. A protective hold is placed on new event actions for the proposed scope where continuing activity could prejudice bidders, create cost, or produce an unauthorised award.
4. The cancellation ground is classified, such as demand withdrawal, specification defect, funding loss, legal or policy change, material process error, insufficient competition, integrity concern, market change, force majeure, institutional restructuring, or superseding arrangement.
5. The factual cause, chronology, responsibility, materiality, affected parties, and whether the cause was foreseeable or avoidable are assessed.
6. Procurement evaluates alternatives including correction, clarification, extension, partial cancellation, lot separation, temporary suspension, reissue, revised demand, alternate method, or continued evaluation.
7. The requesting and technical owners confirm whether the need has ended, changed, been deferred, remains urgent, or requires a governed successor.
8. Bidder impact is assessed, including preparation cost where relevant, equal treatment, reliance, confidentiality, challenge rights, securities, deposits, fees, samples, intellectual property, and submitted documents.
9. Evaluation and recommendation evidence is preserved exactly as it stood; cancellation cannot be used to erase an inconvenient result or reveal protected competitive information.
10. Supplier-integrity, conflict, fraud, collusion, complaint, protest, litigation, regulatory, and investigative implications are assessed and referred to the authoritative process where required.
11. Finance and custodial roles identify bid-security, guarantee, deposit, fee, encashment, release, refund, budget, and monetary consequences without procurement rewriting financial records.
12. Procurement prepares a recommendation defining exact scope, reason, authority, effective time, bidder treatment, demand disposition, successor or re-sourcing path, publications, controls, and corrective actions.
13. Required demand, evaluation, legal, compliance, finance, funding, and oversight reviews are obtained in accordance with event stage and risk.
14. The authorised approver approves, conditionally approves, limits, returns, or rejects cancellation with an attributable reason and effective time.
15. Approved cancellation changes only the affected event scope to Cancelled, blocks award and commitment, closes or freezes event actions, and preserves all prior states and versions.
16. Bidders and stakeholders receive authorised notice; securities, fees, submissions, samples, confidential material, and complaints follow their governed disposition; demand is released, revised, deferred, closed, or linked to a successor as approved.
17. Cancellation, reason, approvals, communications, publications, demand and security hand-offs, corrective actions, actors, timestamps, and any later review remain immutable and auditable.

---

# 10. Alternate Business Flows

## 10.1 Cancellation Before Publication

The planned event is withdrawn under the appropriate authority. No bidder notice is required unless external communication occurred, but demand, approval, and planning history remain preserved.

## 10.2 Cancellation While Event Is Open

New submissions are stopped at the effective time, all registered or invited parties receive equal notice, received submissions remain protected, and deadline or portal behaviour does not determine legal effectiveness by itself.

## 10.3 Cancellation After Bid Opening or During Evaluation

Heightened justification and independent review address bidder reliance, exposed prices, evaluation progress, neutrality, confidentiality, and whether correction or reissue would be fair.

## 10.4 Cancellation After Recommendation but Before Commitment

The recommendation, scores, approvals, and reason for not proceeding remain preserved. Cancellation requires authority appropriate to the advanced stage and cannot be based on undisclosed supplier preference.

## 10.5 Partial or Lot Cancellation

Only independently separable scope is cancelled. Unaffected lots continue without altered evaluation or competitive advantage, and shared demand, ceilings, securities, and bidder communications remain traceable.

## 10.6 Temporary Suspension Instead of Cancellation

Where the cause may be resolved without prejudicing fairness, the event may be suspended under defined authority, duration, bidder notice, preservation, review date, and restart or cancellation condition.

## 10.7 Cancellation for Integrity or Legal Hold

Evidence is protected, ordinary disclosure is limited lawfully, and supplier, investigative, complaint, legal, audit, or regulatory processes receive the necessary facts.

## 10.8 Re-sourcing Required

A new event receives a new identity and references the cancelled predecessor, released demand, corrected scope, and applicable lessons. Bids and prices from the cancelled event are not reused improperly.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Cancellation Ground Not Sustained

The proposal is rejected or returned and the event resumes only through an attributable continuation decision that addresses any protective hold and elapsed time.

## 11.2 Commitment May Already Exist

Cancellation is held while J04 or the authoritative contract owner determines whether a valid commitment exists. This scenario cannot terminate an executed commitment.

## 11.3 Bidder Notification Failure

The failed delivery is recorded and reissued through approved channels. Effectiveness follows applicable rules, and no bidder is selectively advantaged.

## 11.4 Security or Fee Disposition Failure

Cancellation remains recorded, but the unresolved monetary or custody item stays open with its authoritative owner and is escalated until evidenced closure.

## 11.5 Portal or Record Interruption

The authorised decision time and signed evidence control. The event is protected from further action and systems are reconciled when available without backdating.

## 11.6 Unauthorised Post-Cancellation Action

Submission, evaluation, award, or commitment activity after the effective cancellation is blocked, preserved, and investigated.

## 11.7 Complaint, Stay, or Review

The relevant records and action are held as required. Any reversal, remand, or replacement decision creates a new attributable fact and does not erase cancellation history.

## 11.8 Post-Cancellation Discovery

New evidence is appended and may trigger corrective, supplier, disciplinary, legal, financial, or audit action without reopening the event silently.

---

# 12. Business Rules and Decision Constraints

- Cancellation requires a permitted, documented, material, and non-discriminatory reason.
- The authority must be appropriate to the event stage, value, method, funding, category, and risk.
- Cancellation cannot be used to favour a bidder, avoid an unfavourable recommendation, conceal error, or obtain disclosed prices for a preferred re-sourcing exercise.
- Event, bid, evaluation, recommendation, and approval history cannot be deleted or overwritten.
- Protective suspension and permanent cancellation are distinct states and decisions.
- Partial cancellation is permitted only where the affected scope is objectively separable and remaining competition stays fair.
- Cancellation does not itself change approved demand; demand must be released or transferred to its authoritative lifecycle.
- A new sourcing event requires a new identity and explicit predecessor lineage.
- Bidder confidentiality, intellectual property, security, fee, sample, record-retention, complaint, and notice obligations survive cancellation as applicable.
- Suspected fraud, collusion, conflict, or manipulation must be escalated and cannot be resolved merely by cancelling the event.
- No award or commitment may be created for cancelled scope after the effective time.
- If a valid commitment already exists, its cancellation or termination follows J04 or the authoritative contract process.
- This scenario does not create, amend, cancel, or reverse a receipt, payable, accounting entry, or payment.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Sourcing Event | `Open for Supplier Response`, `Submission Closed`, `Opening`, `Evaluation`, `Recommendation` or `Approval` | Applies the governed cancellation decision without creating review, hold, partial-cancellation or remand states. | `Cancelled` when approved; otherwise the existing canonical position remains controlling | Procurement Planning & Sourcing (J01) | `L2-J01` Section 12.3 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SourcingEventCancelled` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SourcingEventClosed` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Requesting Unit / J03 | Confirms demand withdrawal, change, deferral, continuing need, or successor requirement. | Retains demand lifecycle. |
| J02 | Receives relevant bidder integrity or restriction facts and supplies supplier status. | Retains supplier lifecycle. |
| J04 | Confirms whether commitment exists and blocks new commitment for cancelled scope. | Retains PO and contract lifecycle. |
| Finance / Funding / Security Custodian | Handles budget, fees, deposits, securities, guarantees, encashment, release, and refunds. | Retains financial truth and custody. |
| Legal / Compliance / Oversight | Reviews fairness, liability, complaints, investigations, publication, and authority. | Retains specialist authority. |
| Planning / Sourcing | Receives released demand and predecessor facts for revision or re-sourcing. | Creates a distinct successor process. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- event identity, method, lots, demand, approvals, publication, invitations, clarifications, dates, submissions, opening, evaluation, recommendation, and current state;
- cancellation ground, discovery chronology, cause, responsibility, materiality, urgency, affected scope, and evidence;
- alternatives assessed, demand-owner position, bidder impact, legal and integrity review, conflicts, complaints, and downstream commitment check;
- securities, guarantees, fees, deposits, samples, documents, confidentiality, retention, release, refund, and custodial instructions;
- recommendation, approval or rejection, conditions, effective time, bidder communications, publication, demand and successor hand-off, corrective actions, actors, versions, reasons, and timestamps.

## 15.3 Audit Obligations

The institution must reconstruct why cancellation was proposed, what event stage and bidder information were known, which alternatives were considered, whether bidders were treated fairly, who approved the action, how demand and monetary custodial items were handled, and whether any later re-sourcing or commitment improperly benefited from the cancelled event.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the cancellation ground is documented, classified, and approved by an authority appropriate to the event stage, value, method, and institutional risk;
- the affected event scope is in a Cancelled, Suspended, or Partially Cancelled state with an explicit effective time;
- no award or commitment has been created or can be created for the cancelled scope after the effective time;
- bidders have received authorised notice appropriate to the event stage and applicable obligations;
- bid securities, fees, and submitted materials have a governed disposition attributable to an authoritative owner;
- demand has been released, deferred, or linked to a governed successor with explicit lineage;
- all evaluation, recommendation, and approval records remain intact, unaltered, and auditable;
- the complete audit history of the cancellation decision is reconstructable and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the sourcing event record reflects the final cancelled, suspended, or partially cancelled state with reason, authority, actor, and effective timestamp;
- no downstream award, commitment, or payment obligation exists for the cancelled scope;
- bidder confidentiality, intellectual property, security, and notice obligations remain in force as applicable;
- demand is available for a separately governed and distinctly identified successor process;
- the complete event, bidder, evaluation, and cancellation history is permanently auditable and immutable;
- any re-sourcing activity uses a new event identity that references the cancelled predecessor.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Institution-specific cancellation grounds, stage-based authorities, publication, bidder-remedy, and approval rules require confirmation.
2. Treatment of bid preparation costs, fees, samples, and securities by method and cause requires operating-profile mapping.
3. Exact boundary between cancellation, suspension, correction, award rejection, and contract termination requires final state and event governance.

## 17.2 Assumptions

- No valid downstream commitment exists for the affected scope when cancellation is approved.
- Cancellation preserves the complete event and bidder history and does not erase approved demand automatically.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-019**
