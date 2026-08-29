# Expiring Rate Contract to Approved Successor Arrangement

# 1. Scenario Metadata

**Scenario ID:** L3-J-013  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-012 demand sourcing during transition; L3-J-014 closure of the predecessor arrangement  

---
# 2. Scenario Purpose

This scenario governs continuity when an Active rate contract approaches expiry and continuing institutional need requires a successor. It prevents silent extension, backdating, uncontrolled overlap, price carry-forward, or loss of historical lineage while allowing a new competitively sourced arrangement or a strictly authorised interim or extension decision.

---

# 3. Business Outcome

An expiring rate contract is linked to an approved successor arrangement with controlled scope, suppliers, rates, validity, transition, overlap, ceilings, and effective dates, while the predecessor remains historically immutable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an Active rate contract enters its governed renewal or expiry-review window and continuing need is identified.

## 4.2 End Boundary

The scenario ends when a successor reaches Approved or Active with an authorised transition from the predecessor, or when the institution records no successor, temporary gap, permitted extension, alternate sourcing, or discontinuation.

## 4.3 In Scope

- expiry monitoring and continuing-need assessment;
- review of utilisation, supplier performance, rates, market conditions, demand forecast, scope, risk, and outstanding call-offs;
- successor strategy, method, timetable, overlap, migration, and contingency planning;
- re-sourcing and evaluation under the applicable method;
- controlled extension or interim arrangement only where policy permits;
- mapping predecessor catalogue, supplier, rates, ceilings, open commitments, and demand to successor positions;
- preventing new predecessor call-offs beyond permitted cut-off;
- approving successor version and effective date;
- preserving predecessor history and publishing transition facts.

## 4.4 Out of Scope

- original establishment, governed by J-011;
- ordinary demand call-off, governed by J-012;
- suspension or final closure, governed by J-014;
- supplier remediation or qualification lifecycle;
- Purchase Order, contract migration mechanics, receipt, payable, accounting, or payment;
- technical expiry scheduler, migration script, API, schema, or UI design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-011 | Established the predecessor. |
| Related | L3-J-012 | Uses the valid arrangement during governed transition. |
| Followed by | L3-J-014 | Closes the predecessor after transition. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns expiry review, successor strategy, sourcing, transition mapping, extension exception, approval, and successor outcome. |

J02 retains supplier truth, Finance retains monetary truth, and J04 retains existing commitments. A successor does not rewrite predecessor call-offs or obligations.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Officer | Administers the rate-contract sourcing or lifecycle activity and preserves its evidence. | Performs procurement administration. |
| Procurement Manager | Supervises the governed procurement activity. | Reviews and supervises. |
| Requesting Unit Representative | Supplies requirement and forecast context without acquiring procurement authority. | Provides business input. |
| Procurement Recommender | Records the attributable recommendation. | Recommends. |
| Procurement Approver | Approves, rejects or returns the governed outcome within established authority. | Decides. |
| Auditor / Reviewer | Reviews contract, decision and lifecycle evidence without modifying the record. | Reviews evidence. |
---

# 7. Preconditions and Entry Criteria

- The predecessor Rate Contract is identifiable, Active or in a governed expiry-review state, and has a known end date.
- Continuing need, forecast, category, scope, locations, and transition risks can be assessed.
- Utilisation, supplier performance, open call-offs, ceiling position, disputes, and commercial history are available.
- Applicable renewal, extension, re-sourcing, competition, notice, and approval rules are effective.
- Sufficient lead time exists for normal re-sourcing, or the cause and authority for an exceptional interim measure can be evidenced.
- No successor already governs the same future scope unless controlled overlap is intended.

---

# 8. Trigger

The predecessor enters its expiry-review window or Procurement receives an authorised continuing-need requirement before expiry.

---

# 9. Main Business Flow

1. Procurement identifies the predecessor, validity, suppliers, catalogue, rates, ceilings, locations, call-off rules, and open positions.
2. Continuing need and revised forecast are confirmed with requesting and specialist units.
3. Utilisation, performance, incidents, delivery, quality, disputes, capacity, supplier eligibility, rate competitiveness, market change, and scope change are reviewed.
4. Procurement decides whether to discontinue, re-source substantially unchanged scope, redesign scope, use another sourcing model, or seek a permitted extension.
5. The successor timetable, method, aggregate value, competition, approval, transition, contingency, and predecessor cut-off are defined.
6. Normal competitive sourcing begins early enough to avoid avoidable expiry gaps.
7. The successor package preserves relevant predecessor lessons without automatically carrying forward suppliers, ranks, rates, or terms.
8. Supplier responses are evaluated under the new issued rules and current eligibility facts.
9. Any proposed extension or interim arrangement records legal or policy basis, duration, scope, rates, ceiling, supplier, reason, value, and authority.
10. The recommendation identifies successor suppliers, catalogue, rates, validity, ceilings, call-off rules, effective date, predecessor cut-off, overlap, migration, and residual predecessor obligations.
11. Open predecessor Purchase Orders and obligations are distinguished from authority for new call-offs.
12. The Procurement Manager reviews competition, timing, avoidable delay, performance, rate reasonableness, exception, overlap, and continuity risk.
13. Required exception and final approvals are obtained.
14. The successor receives a unique identity and version; it never replaces the predecessor identity.
15. Where prerequisites are satisfied, the successor becomes Approved and then Active on its authorised effective date.
16. The predecessor stops accepting new call-off sourcing at its expiry or earlier cut-off, except for expressly authorised transition conditions.
17. Transition facts are published for J-012 and J-014, preserving all historical predecessor use.

---

# 10. Alternate Business Flows

## 10.1 Competitive Successor

A new sourcing event produces a successor with changed or retained suppliers based solely on current evaluation and authority.

## 10.2 Permitted Extension

An extension is a controlled successor version or explicitly governed amendment, never a silent date edit. Duration, value, rates, scope, justification, and authority are bounded.

## 10.3 Interim Arrangement

Where normal sourcing cannot complete and continuity is critical, a temporary arrangement may be approved under the applicable exceptional method. Avoidable planning failure is disclosed, not concealed.

## 10.4 Controlled Overlap

Predecessor and successor may overlap only for migration, location, catalogue, supplier, or service-continuity reasons. Scope, precedence, ceilings, call-off eligibility, and end date are explicit.

## 10.5 Gap Between Arrangements

If the predecessor expires before successor activation, new call-offs are blocked or redirected to another governed route. Backdating is prohibited.

## 10.6 No Successor

Discontinued need, obsolete category, policy change, or unsatisfactory sourcing may result in no successor. Residual demand is redirected and the predecessor proceeds to closure.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Late Review or Re-Sourcing Delay

Delay is recorded with cause and impact. Extension or interim authority is not automatic; demand may require alternate sourcing or service-continuity escalation.

## 11.2 Invalid Extension

An extension lacking policy basis, competition justification, authority, or bounded duration is rejected. The predecessor expires as originally approved.

## 11.3 Supplier Performance or Eligibility Failure

Poor performance, suspension, blacklisting, expired qualification, or risk finding affects successor evaluation and may block extension or continued participation.

## 11.4 Successor Sourcing Failure

No response, no suitable offer, cancellation, or approval failure produces re-sourcing, interim disposition, alternate method, demand redesign, or gap management.

## 11.5 Overlap or Ceiling Conflict

Unclear applicability or combined ceiling risk blocks activation until predecessor and successor authority are separated.

## 11.6 Post-Approval Change

Material successor changes require a controlled version or new sourcing decision. Predecessor history remains immutable.

---

# 12. Business Rules and Decision Constraints

- Successor planning must begin within the governed review window.
- Expiry does not automatically authorise extension, renewal, incumbent preference, or rate carry-forward.
- Current aggregate value, market, scope, risk, and policy determine successor method and authority.
- Incumbent performance informs but does not predetermine selection.
- Predecessor and successor identities, versions, rates, suppliers, ceilings, and periods remain distinct.
- Backdated activation and silent validity extension are prohibited.
- New predecessor call-offs after cut-off or expiry are prohibited.
- Existing valid commitments may continue under their own authority after contract expiry.
- Controlled overlap requires explicit scope, precedence, ceilings, and end date.
- Extension and interim measures must be minimal, bounded, justified, and separately approved.
- Successor activation does not itself create demand or Purchase Orders.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Predecessor Rate Contract | `Active` | Remains authoritative until its governed end; this scenario does not invent an interim lifecycle state. | `Expired / Closed` when its authority ends | Procurement Planning & Sourcing (J01) | `L2-J01` Section 27.3 |
| Successor Rate Contract | `Draft` | Proceeds through `Review` and `Approved` under the canonical lifecycle. | `Active` when the approved successor becomes effective | Procurement Planning & Sourcing (J01) | `L2-J01` Section 27.3 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `RateContractRenewalCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractCreated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractActivated` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractExpired` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractClosed` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| J02 | Supplies current and historical supplier eligibility and performance facts. | Retains supplier truth. |
| J03 / Requesting Units | Supply continuing demand, forecast, scope, and timing. | Retain demand authority. |
| Finance | Supplies budget, historic spend, index, tax, and monetary references. | Retains monetary truth. |
| J04 | Supplies open-commitment facts and consumes successor outcomes. | Retains commitment authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- predecessor identity, version, validity, suppliers, catalogue, rates, ceilings, utilisation, performance, and open commitments;
- continuing need, forecast, revised scope, market assessment, and successor strategy;
- sourcing event, responses, evaluation, and current supplier eligibility;
- extension or interim basis, duration, value, rates, scope, and authority;
- transition, cut-off, overlap, gap, migration, predecessor residual, and successor effective date;
- recommendation, approvals, actors, reasons, timestamps, and lineage.

## 15.3 Audit Obligations

The institution must reconstruct why and when successor action began, whether delay was avoidable, how suppliers and rates were selected, and how predecessor and successor authority were separated.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the expiring Rate Contract has been identified before its effective end date with a governance-accepted reason for succession;
- a successor arrangement has been approved with a distinct identity, valid scope, effective dates, and clear predecessor lineage;
- the transition boundary between the expiring and successor contracts is explicit and prevents a coverage gap for governed procurement categories;
- no demand continues to be routed against the expired contract after its effective end;
- approval authority, evidence, and the succession decision are preserved and attributable;
- the expiring contract's history, outstanding obligations, and any transitional provisions are preserved and not silently overwritten.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the successor Rate Contract is active and eligible to receive demand for its defined scope and effective period;
- the expiring Rate Contract is in a terminal or expired state and no new demand may be committed against it after the effective end;
- predecessor lineage between the expiring and successor contracts is recorded and auditable;
- suppliers, institutional users, and governance records reflect the approved successor arrangement;
- no coverage gap exists for demand within the governed scope during the transition period.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Institution-specific extension duration and value limits cannot be universalised.
2. Treatment of call-offs issued before expiry but delivered afterward requires operating-profile rules.
3. Boundary between extension, amendment, successor, interim arrangement, and emergency procurement requires final validation.

## 17.2 Assumptions

- A successor has a distinct identity and does not overwrite the predecessor.
- Existing valid commitments survive according to their own terms.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-013**
