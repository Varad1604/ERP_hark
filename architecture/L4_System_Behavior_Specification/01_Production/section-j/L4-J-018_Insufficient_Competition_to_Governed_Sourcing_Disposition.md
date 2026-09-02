# Insufficient Competition to Governed Sourcing Disposition

## 1. Specification Metadata

**Specification ID:** L4-J-018  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Procurement Planning & Sourcing  
**Realizes Scenario:** L3-J-018  
**Primary Business Owner:** Procurement Planning & Sourcing  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Chetan  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-J-018_Insufficient_Competition_to_Governed_Sourcing_Disposition.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`  
**Referenced By:** Sourcing evaluation, re-sourcing, negotiation, award, cancellation, commitment, supplier-governance, and audit specifications where applicable.

---

## 2. Behavioral Objective & Boundary

SutraERP shall require an explicit, attributable, policy-governed disposition when a competitive sourcing event produces fewer valid or responsive responses than the required or expected level.

The behavior begins when an active competitive sourcing event reaches a response, opening, responsiveness, evaluation, or award checkpoint and the valid competitive response population is insufficient. It ends when an authorised disposition is effective, bidder treatment remains consistent, the affected event/demand scope has a clear next path, and original sourcing evidence remains immutable.

This specification does not create authority for automatic award, alter J03-owned demand/specification, change J02 Supplier lifecycle, perform the separate L3-J-019 cancellation scenario, or create Purchase Order, contract, receipt, payable, accounting, or payment outcomes.

---

## 3. Required Business Context

- Identifiable sourcing event, method, approved demand, lots/lines, criteria, publication/invitation, response deadline, and current J01 lifecycle position.
- Preserved timely responses, withdrawals, opening/custody evidence, responsiveness decisions, and bidder identities.
- Applicable minimum-competition, single-bid, extension, reissue, clarification, negotiation, cancellation, and approval rules.
- Current demand/specification/urgency facts from J03/requesting owners.
- Market, supplier, benchmark, funding, price, and integrity evidence as required.
- J02-owned supplier identity, eligibility, restriction, ownership, and integrity facts where material.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-J-018-BH-001 | When insufficient competition is detected, the system shall preserve the sourcing-event identity, method, affected lots/scope, demand references, publication/invitation dates, response deadline, accessible/invited market, responses received, withdrawals, and current lifecycle position. | L3-J-018 §§4.1, 8, 9.1 |
| L4-J-018-BH-002 | Before a disposition is considered, the system shall require reconciliation of submission custody, deadline enforcement, opening evidence, response count, bidder identities, and response integrity. | L3-J-018 §§7, 9.2, 11.1 |
| L4-J-018-BH-003 | The system shall preserve each response's source-supported classification as timely/late, responsive/non-responsive, withdrawn/active, and eligible/restricted using the approved criteria without post-visibility criterion changes. | L3-J-018 §§9.3, 10.3, 12 |
| L4-J-018-BH-004 | The system shall determine the applicable competition expectation only from institutionally governed method, value, category, funding, jurisdiction, risk, and policy facts. | L3-J-018 §§9.4, 12 |
| L4-J-018-BH-005 | The system shall distinguish no response, single response, limited response, widespread non-responsiveness, bidder withdrawal, supplier concentration, and integrity/collusion concerns so that the disposition is based on the actual deficiency. | L3-J-018 §§4.3, 9.5 |
| L4-J-018-BH-006 | Before recommending continuation or re-sourcing, the system shall preserve the review of event notice, accessibility, response time, specification neutrality, qualification proportionality, lotting, terms, market reach, and clarification consistency. | L3-J-018 §§9.6, 10.1 |
| L4-J-018-BH-007 | The system shall require confirmation from the authoritative demand/requesting owners that need, specification, quantity, delivery requirement, funding context, and urgency remain valid; J01 shall not silently revise J03-owned demand. | L3-J-018 §§4.4, 9.7, 12 |
| L4-J-018-BH-008 | The system shall preserve market/category evidence relevant to supplier availability, barriers, capacity, timing, location, aggregation, prior sourcing, and whether extension or reissue is reasonably expected to improve competition. | L3-J-018 §§9.8, 11.4 |
| L4-J-018-BH-009 | Where at least one responsive offer exists, the system shall require technical acceptability and commercial/price reasonableness to be assessed independently of response count before any permitted continuation or award disposition. | L3-J-018 §§9.9, 10.2–10.3, 11.3, 12 |
| L4-J-018-BH-010 | The system shall preserve conflict, bidder relationship, common ownership, bid-similarity, communication-pattern, and other integrity indicators and shall hold ordinary disposition where specialist escalation is required. | L3-J-018 §§9.10, 10.7, 12 |
| L4-J-018-BH-011 | The system shall present only institutionally permitted disposition options and preserve the assessment of fairness, legality, urgency, price, operational impact, bidder expectations, re-sourcing consequences, demand validity, and institutional risk. | L3-J-018 §§3, 9.11 |
| L4-J-018-BH-012 | The recommendation shall identify the exact event/lot/scope, competition deficiency, cause, evidence, chosen disposition, rejected alternatives, conditions, bidder treatment, demand effect, next owner, and validity. | L3-J-018 §9.12 |
| L4-J-018-BH-013 | When the Procurement Approver authorises the recommendation, the system shall apply only the approved continuation, clarification, negotiation, extension, reissue, revision/re-sourcing, method-change, rejection, or cancellation hand-off supported by the parent. | L3-J-018 §§3, 9.14–9.16 |
| L4-J-018-BH-014 | If continuation, clarification, or negotiation is authorised, the system shall preserve identical governed rules and shall not permit material scope/criterion changes that could have affected bidder participation. | L3-J-018 §§9.15, 10.5–10.6, 12 |
| L4-J-018-BH-015 | If extension, reissue, re-sourcing, rejection, method change, or cancellation hand-off is authorised, the system shall preserve the original event history and prevent duplicate consumption of affected demand. | L3-J-018 §§9.16, 10.1, 10.4, 12 |
| L4-J-018-BH-016 | The system shall not infer emergency, single-source, proprietary, or direct-award authority merely because competitive sourcing failed; those routes require their separate source-supported grounds and governance. | L3-J-018 §§2, 10.8, 12 |
| L4-J-018-BH-017 | Adequately competed lots shall remain distinguishable from deficient lots so that a lot-level disposition does not unnecessarily alter unaffected scope. | L3-J-018 §§4.3, 10.4, 12 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-J-018-BH-018 | When the governed result is no valid response, the system shall publish `NoResponseOutcomeDeclared` with Procurement Planning & Sourcing (J01) as canonical publisher. | L3-J-018 §14.1; L2-J01 §35.13 |
| L4-J-018-BH-019 | When the governed result is no suitable bid, the system shall publish `NoSuitableBidOutcomeDeclared` with J01 as canonical publisher. | L3-J-018 §14.1; L2-J01 §35.13 |
| L4-J-018-BH-020 | When re-sourcing is authorised, the system shall publish `ReSourcingRequested` with J01 as canonical publisher. | L3-J-018 §14.1; L2-J01 §35.13 |
| L4-J-018-BH-021 | When the resulting re-sourcing event is created under J01 governance, the system shall publish `ReSourcingEventCreated` with J01 as canonical publisher. | L3-J-018 §14.1; L2-J01 §35.13 |

J03/requesting units retain demand/specification authority. J02 retains Supplier lifecycle. J04 receives only separately valid award/commitment outcomes. Finance/funding authorities retain monetary facts. Legal/compliance/oversight retain complaint, collusion, fairness, and specialist authority. L3-J-019 remains the separate cancellation-governance scenario.

---

## 6. Failure / Rejection / Continuation

- A response-count or custody dispute shall suspend disposition until authoritative submission/opening evidence is reconciled.
- Inconsistent application of evaluation criteria shall hold the affected decision until a fair source-supported corrective action is authorised.
- If price reasonableness cannot be demonstrated, continuation/award shall remain blocked pending an allowed negotiation, further evidence, reissue, scope revision, alternate method, or cancellation path.
- Inconclusive market analysis shall result in additional evidence or only the parent-supported higher-risk authority with explicit limitations.
- A bidder complaint/challenge shall preserve confidentiality and follow the authoritative review path without rewriting event history.
- If disposition approval expires or is interrupted, the sourcing event shall remain non-awarded and relevant demand/supplier/price/funding/validity/legal facts shall be refreshed before continuation.
- An unauthorised award/commitment attempt shall not validate the sourcing disposition and remains governed by its canonical owner.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- approved demand, sourcing strategy, event, method, lots, criteria, publication/invitation, clarifications, deadline, and opening/custody record;
- all responses, withdrawals, timestamps, eligibility/responsiveness determinations, and bidder identities;
- applicable competition requirement and deficiency classification;
- event-design, market, supplier, urgency, and prior-sourcing analysis;
- benchmarks, independent estimates, commercial analysis, conflicts, ownership, and collusion indicators;
- options considered, recommendation, conditions, approval/return/rejection decision, effective time, and responsible next owner;
- bidder/stakeholder communications and immutable original event history;
- exact J01 no-response/no-suitable-bid/re-sourcing events where applicable.

---

## 8. Observable Completion Conditions

The scenario is observably complete when:

1. the deficient competitive population and applicable competition expectation are established from preserved evidence;
2. event integrity, responsiveness, market conditions, continuing demand, supplier facts, and price reasonableness have been assessed to the extent required by the chosen disposition;
3. the authorised disposition and its affected event/lot/demand scope are explicit;
4. bidder treatment is consistent and original criteria/history have not been silently altered;
5. affected demand has one clear next path without duplicate consumption;
6. cancellation, if selected, is handed to L3-J-019 rather than locally redesigning cancellation governance;
7. no Purchase Order or other downstream financial/commitment record is created by this scenario itself.

---

## 9. Traceability & Provenance

**Parent:** L3-J-018 — Insufficient Competition to Governed Sourcing Disposition  
**Primary L2 source:** L2-J01 — Procurement Planning & Sourcing Workflows  
**Supporting L2 sources:** L2-J00; L2-J02; L2-J03.  

The exact J01 closure/re-sourcing events verified for this scenario are `NoResponseOutcomeDeclared`, `NoSuitableBidOutcomeDeclared`, `ReSourcingRequested`, and `ReSourcingEventCreated`. The parent explicitly prevents failed competition from silently becoming emergency, proprietary, or direct-award authority.

---

## 10. Open Issues / Deferred Design

- Institution-specific minimum-competition thresholds, single-bid rules, extension/reissue limits, negotiation authority, complaint treatment, and higher-risk approvals remain upstream-governed.
- The parent does not define a separate "insufficient competition disposition" lifecycle; this L4 therefore preserves the existing J01 Sourcing Event lifecycle and exact terminal positions rather than inventing local states.
- Full sourcing-event cancellation remains L3-J-019; this L4 only preserves the authorised cancellation hand-off.
- `Section_J_Canonical_Register.md` could not be located by exact filename in the current repository. No substitute semantics are inferred because the material behavior is supported by Locked L3/J00/J01/J02/J03 evidence.
- Technical evaluation engines, portal behavior, notification delivery, APIs, persistence, services, event transport, retry mechanics, infrastructure, and UI/UX remain deferred to L5/L6.

**Review Verdict:** PASS WITH NON-BLOCKING FINDINGS
