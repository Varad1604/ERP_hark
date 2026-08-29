# Approved Demand to Governed Single-Source or Proprietary Sourcing Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-J-015  
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
**Referenced By:** downstream award, commitment, supplier-governance, receipt, payable, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs exceptional sourcing from one identified supplier where competition is unavailable or inappropriate because of exclusive rights, proprietary technology, intellectual property, compatibility, standardisation, continuity, warranty protection, unique capability, or another authorised single-source basis.

It prevents convenience, urgency, supplier preference, fragmented demand, or weak market research from being presented as exclusivity and produces an independently reviewed, commercially reasonable, attributable sourcing outcome.

---

# 3. Business Outcome

Approved demand reaches an Approved Single-Source or Proprietary Sourcing Outcome, a Returned outcome, or a Rejected outcome. An approved outcome identifies the supplier, scope, quantity, value basis, exceptional justification, market and alternatives assessment, commercial reasonableness, conflicts, conditions, approval authority, validity, and evidence lineage without itself creating a Purchase Order or contract.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when approved, available procurement demand is proposed for single-source or proprietary sourcing with an identifiable supplier and documented exceptional basis.

## 4.2 End Boundary

The scenario ends when the proposal is approved, returned, or rejected; the demand remains traceable; and the governed result is available to the appropriate downstream commitment or alternate sourcing path.

## 4.3 In Scope

- validating demand, budget/funding context, specification, quantity, timing, aggregation, and sourcing eligibility;
- validating exclusive, proprietary, compatibility, continuity, standardisation, warranty, IP, or unique-capability evidence;
- conducting an alternatives and proportionate market check;
- verifying supplier eligibility and conflicts or related-party interests;
- establishing price and commercial reasonableness without relying on competitive bids;
- applying value, category, cumulative-spend, tenure, risk, and exception authorities;
- approving, returning, rejecting, time-limiting, or conditioning the exceptional sourcing outcome;
- preserving the full demand-to-supplier-to-decision evidence chain.

## 4.4 Out of Scope

- approving unapproved demand or changing demand authority;
- supplier onboarding, suspension, or blacklisting owned by J02;
- creating or signing a Purchase Order, contract, receipt, payable, accounting entry, or payment;
- treating emergency procurement as proprietary sourcing;
- technical implementation, workflow configuration, or notification design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-J-004 to L3-J-007 | Provide competitive or simplified sourcing alternatives. |
| Related | L3-J-008 to L3-J-010 | Govern consolidated, split, and multi-supplier demand. |
| Followed by | J04 commitment scenarios | Convert an approved sourcing result into a separately authorised commitment. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns exceptional-method eligibility, independent review, alternatives assessment, supplier and commercial evaluation, approval, outcome validity, and sourcing evidence. |

Requesting units retain need and specification accountability. J02 retains supplier truth, J04 retains commitment truth, and Finance retains budget and monetary truth.

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

- Demand is approved, identifiable, current, uncommitted, and available for sourcing.
- The requested scope, quantity, location, timing, specification, estimated value, and funding context are sufficiently defined.
- The proposed supplier and claimed exceptional basis are identifiable.
- Applicable competition thresholds, proprietary and single-source grounds, delegation, disclosure, aggregation, tenure, and review rules are effective.
- Supplier eligibility and beneficial ownership or related-party facts can be obtained.
- No prior sourcing outcome already consumes the same demand unless an authorised replacement or residual quantity is identified.

---

# 8. Trigger

An authorised requester or procurement role submits approved demand for exceptional single-source or proprietary sourcing because only one source is asserted to be lawfully or practically capable of satisfying the requirement.

---

# 9. Main Business Flow

1. Procurement records the demand, proposed supplier, claimed ground, initiating actor, submission time, and all supporting evidence.
2. Demand approval, remaining quantity, sourcing status, specification, value, funding context, delivery need, and aggregation with related demand are verified.
3. The asserted ground is classified, such as exclusive right, proprietary technology, IP/licence, compatibility, standardisation, continuity, warranty protection, or unique capability.
4. The requester certifies the factual need and explains why preference, convenience, schedule pressure, earlier investment, or poor planning is not the true basis.
5. An independent reviewer tests specifications for avoidable supplier bias and assesses feasible functional substitutes, interoperability, migration, alternative standards, and lifecycle options.
6. Procurement performs a proportionate market check and records identified suppliers, searches, prior competitions, authoritative rights evidence, and reasons alternatives cannot meet the need.
7. Cumulative and related demand, prior awards, renewals, extensions, and supplier concentration are reviewed to detect splitting, recurring exceptions, or avoidable lock-in.
8. Supplier identity, eligibility, capacity, ownership, restrictions, performance, and required declarations are verified from authoritative sources.
9. Requester, evaluators, reviewers, approvers, and supplier-related parties disclose conflicts; any conflict is mitigated or escalated before decision.
10. Price and commercial reasonableness are assessed using benchmarks, catalogues, historic prices, independent estimates, cost analysis, published rates, negotiation, total lifecycle cost, or other defensible evidence.
11. Commercial, legal, data, security, warranty, support, licensing, continuity, exit, and dependency risks are assessed proportionately.
12. Procurement prepares a recommendation identifying scope, supplier, ground, evidence, value, conditions, validity, limitations, demand consumption, and downstream path.
13. Required technical, legal, compliance, finance, and independent reviews are completed without replacing the accountable approval.
14. The authorised approver approves, conditionally approves, returns, or rejects with an attributable reason and effective time.
15. Approval creates a controlled sourcing outcome and reserves or consumes only the authorised demand quantity for the stated validity period.
16. Return or rejection preserves demand for correction or a permitted competitive, simplified, consolidated, or split sourcing route.
17. The complete evidence, versions, declarations, reviews, negotiation record, decision, actors, and timestamps are retained for downstream use and audit.

---

# 10. Alternate Business Flows

## 10.1 Proprietary Component Within Broader Demand

Only the genuinely proprietary component follows this method. Separable competitive scope is routed independently and all relationships remain traceable.

## 10.2 Compatibility or Standardisation Basis

The case quantifies interoperability, retraining, conversion, support, safety, data, lifecycle, and exit impacts and demonstrates why compatibility is proportionate rather than merely convenient.

## 10.3 Exclusive Rights

Rights ownership, territorial scope, validity dates, authorised resellers, licensing constraints, and any equivalent source are independently verified.

## 10.4 Continuity or Warranty Protection

Existing commitments, warranty terms, asset life, transition cost, operational risk, and realistic competitive transition date are recorded. Approval is time-bounded where continued dependency is temporary.

## 10.5 Negotiated Commercial Improvement

Where the technical basis is valid but price is unsupported, procurement negotiates or obtains further evidence before approval; technical exclusivity does not establish price reasonableness.

## 10.6 Exception Not Sustained

The proposal is rejected or returned and the demand proceeds through an eligible competitive or other governed method.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Unsupported Exclusivity

The case is returned or rejected; no exceptional outcome is issued.

## 11.2 Supplier Ineligible or Restricted

Approval is blocked and the demand is redirected or held pending authoritative resolution.

## 11.3 Conflict or Related-Party Concern

Affected actors are recused, independent review is increased, or the case is escalated or rejected under policy.

## 11.4 Duplicate or Fragmented Demand

The case is suspended while related demand and prior outcomes are reconciled. No duplicate consumption is permitted.

## 11.5 Price Not Reasonable

Negotiation, cost analysis, scope adjustment, alternate solution review, or rejection follows; exclusivity cannot waive commercial stewardship.

## 11.6 Approval or Evidence Interruption

The case remains in a non-approved state with preserved evidence and resumes from the last valid checkpoint. Expired facts are refreshed.

## 11.7 Post-Approval Material Change

Supplier, scope, price, quantity, ground, rights, specification, or risk change is reassessed and may require amendment, cancellation, or a new sourcing outcome.

---

# 12. Business Rules and Decision Constraints

- Single-source and proprietary sourcing are exceptional and require an expressly permitted ground.
- An identified preferred supplier is not evidence that only one capable source exists.
- Requester certification must be independently tested by procurement or another competent reviewer.
- Specifications cannot be written to manufacture exclusivity.
- Related and cumulative demand must be considered; demand cannot be split to avoid competition or authority thresholds.
- Technical exclusivity does not establish fair price, supplier eligibility, funding, or commitment authority.
- Conflicts, beneficial ownership, related parties, and prior relationships must be disclosed and governed.
- The approval must be limited to the verified supplier, demand, scope, quantity, value basis, period, and conditions.
- Recurring or long-term exceptions require heightened review of lock-in, transition, and competition-restoration options.
- A lapsed or materially changed basis cannot support a new commitment.
- Rejected demand remains intact and available for another authorised sourcing route.
- This scenario does not create a Purchase Order, contract, receipt, payable, accounting entry, or payment.

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
| `SingleSourceProcurementDeclared` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `SingleSourceProcurementApproved` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `ApprovedSourcingOutcomeFinalised` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Requesting Unit / J03 | Supplies approved demand and factual need. | Retains demand ownership. |
| J02 | Supplies supplier identity, eligibility, ownership, restriction, and performance facts. | Retains supplier lifecycle. |
| J04 | Consumes valid approved outcome for separate commitment control. | Retains PO and contract lifecycle. |
| Finance | Supplies funding, budget, tax, and monetary-control facts where required. | Retains financial truth. |
| Legal / Compliance | Reviews rights, conflicts, regulation, licensing, and exception legality. | Retains specialist authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- demand approval, specification, quantity, value, dates, funding context, aggregation, and remaining availability;
- exceptional ground, requester certificate, rights or proprietary evidence, compatibility or continuity analysis;
- market search, alternatives and substitute assessment, prior sourcing history, cumulative spend, and lock-in analysis;
- supplier identity, eligibility, ownership, restrictions, capacity, performance, and declarations;
- price benchmarks, independent estimate, cost analysis, negotiations, lifecycle cost, and commercial terms;
- conflict declarations, specialist reviews, recommendation, approval, conditions, actors, reasons, versions, and timestamps.

## 15.3 Audit Obligations

The institution must be able to reconstruct why competition was not used, why the supplier was uniquely suitable, how alternatives and price were tested, who was independent, what conflicts existed, what demand was consumed, and which authority approved the outcome.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the approved demand is supported by a documented, authorised, and non-discriminatory single-source or proprietary justification that satisfies institutional policy;
- the justification has been reviewed and approved by an authority appropriate to the demand value, category, and institutional risk;
- the sole supplier or rights holder is identified and the sourcing outcome is traceable to an institutional purpose that prohibits competition;
- the sourcing outcome and approved justification are recorded against the demand with a unique identity;
- the approval decision, authority, conditions, and audit evidence are preserved and immutable;
- no competitive sourcing event was required and no alternative supplier was unfairly excluded as a consequence of a false justification.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the demand is associated with an approved single-source or proprietary sourcing outcome with complete justification lineage;
- the sourcing outcome is available as a governed fact for downstream commitment and contractual scenarios;
- the approved justification and authority evidence are permanently preserved and auditable;
- institutional governance may independently review the justification without disrupting the approved sourcing outcome;
- no financial commitment, Purchase Order, or payment obligation has been created by this scenario alone.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Institution-specific permitted grounds, thresholds, publication, oversight, and review periods require confirmation.
2. Exact independence requirements for technical certification and commercial review require operating-profile governance.
3. Maximum duration and recurrence limits for compatibility or continuity cases require policy mapping.

## 17.2 Assumptions

- Approved demand exists before exceptional sourcing begins.
- Competition remains the default unless the exceptional ground is substantiated.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-015**
