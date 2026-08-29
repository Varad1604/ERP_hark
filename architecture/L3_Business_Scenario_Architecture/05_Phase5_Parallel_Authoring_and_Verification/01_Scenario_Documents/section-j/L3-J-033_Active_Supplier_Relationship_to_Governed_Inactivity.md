# Active Supplier Relationship to Governed Inactivity

# 1. Scenario Metadata

**Scenario ID:** L3-J-033  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Vendor & Supplier Management  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** supplier qualification, sourcing, invitation, reactivation, relationship closure, archival, duplicate reconciliation, contract, finance, monitoring, and audit scenarios  

---
# 2. Scenario Purpose

This scenario governs transition of an Active supplier relationship into a non-punitive Inactive state when defined business-activity conditions are met. It reduces uncontrolled future use of dormant or no-longer-current supplier relationships while preserving identity, qualifications, history, open obligations, contractual rights, financial records, and the ability to reactivate through governed checks.

Inactivity is not restriction, suspension, blacklisting, termination, deletion, or archival. It must not be used to conceal adverse findings or bypass due process.

---

# 3. Business Outcome

An Active supplier relationship reaches Governed Inactivity with defined reason, effective time, affected future use, open-position treatment, supplier communication where applicable, reactivation requirements, owner, and immutable history, while unrelated legal, contractual, and financial obligations remain intact.

Outcome labels in this section are business dispositions or supporting-record results, not additional canonical lifecycle states. Section 13 states the only canonical lifecycle impact.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised inactivity review is triggered by prolonged non-use, supplier request, category or relationship rationalisation, duplicate or replacement relationship, cessation of ordinary business need, or another approved non-punitive reason.

## 4.2 End Boundary

The scenario ends when inactivity is approved, rejected, returned, or deferred; new ordinary reliance is controlled; open sourcing, commitment, contract, delivery, warranty, claim, payable, and payment positions are identified for their owners; and reactivation or closure paths are recorded.

## 4.3 In Scope

- validating canonical supplier identity, current Active state, last meaningful activity, inactivity reason, affected relationship scope, and request authority;
- distinguishing no recent use from adverse governance, qualification expiry, legal closure, supplier cessation, and duplicate identity;
- checking open sourcing, recommendations, awards, commitments, contracts, deliveries, warranties, claims, disputes, payables, payments, and statutory retention needs;
- defining future invitation, qualification reliance, sourcing, award, or new-commitment controls for inactive suppliers;
- governing supplier notice, reactivation prerequisites, periodic review, and referral to closure and archival under L3-J-034;
- preserving historical identity, activity, qualification, performance, and financial evidence.

## 4.4 Out of Scope

- imposing restriction, suspension, blacklisting, debarment, or punitive consequence;
- resolving duplicate supplier identities, governed by L3-J-035;
- closing and archiving the supplier relationship, governed by L3-J-034;
- automatically terminating Purchase Orders or contracts;
- reversing receipts, payables, accounting entries, payments, claims, recoveries, or bank transactions;
- technical dormancy detection, notification, enforcement, or archival implementation.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-021 | Supplier relationship was approved and Active. |
| Related | L3-J-022 to L3-J-032 | Qualification and governance states remain distinct. |
| Followed by | L3-J-034 | Long-term completed relationships may close and archive. |
| Related | L3-J-035 | Duplicate records require identity reconciliation, not mere inactivity. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Vendor & Supplier Management | Owns inactivity review, open-position coordination, approval, status publication, reactivation conditions, and history. |

Contract, finance, legal, compliance, category, qualification, and operational owners retain their authoritative records and open obligations.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Supplier Representative | Receives notice and supplies permitted response or remediation evidence; cannot remove institutional restrictions. | Responds. |
| Institutional Verifier | Independently verifies evidence material to the governance decision. | Verifies evidence. |
| Supplier Governance Authority | Proposes or approves restriction, suspension, blacklisting, reactivation, inactivation, closure or merge treatment. | Decides governed supplier treatment. |
---

# 7. Preconditions and Entry Criteria

- A canonical supplier relationship exists in Active state.
- Approved inactivity reasons, activity measures, review periods, authority, notice, future-use controls, reactivation, closure, and retention rules are effective.
- Current supplier approval, qualification, restriction, suspension, blacklisting, sourcing, commitment, contract, and finance positions are available.
- Open obligations can be identified without rewriting their authoritative records.
- Adverse governance facts can be routed to L3-J-028 to L3-J-031 rather than concealed as inactivity.
- Duplicate identity can be referred to L3-J-035.

---

# 8. Trigger

An approved review detects prolonged absence of qualifying activity, receives a supplier or relationship-owner inactivity request, or identifies another non-punitive reason to stop ordinary future use.

---

# 9. Main Business Flow

1. The relationship owner or governed review records the canonical supplier, requested reason, affected scope, last known activity, source, and proposed effective time.
2. The Supplier Governance Authority validates identity, Active state, approval history, qualifications, profile currency, representatives, related entities, and duplicate indicators.
3. Last meaningful sourcing, award, commitment, delivery, service, warranty, claim, dispute, invoice, payment, communication, and review activity is determined using approved definitions.
4. The proposed reason is classified as non-use, supplier request, relationship rationalisation, replacement, cessation of business need, or another approved non-punitive ground.
5. Any fraud, misconduct, safety, security, legal, sanctions, performance, qualification, or compliance concern is routed to its governance process and not relabelled as ordinary inactivity.
6. Open sourcing events, recommendations, awards, proposed commitments, Purchase Orders, contracts, deliveries, warranties, claims, disputes, credit balances, payables, payments, and statutory evidence are identified.
7. Each open position is referred to its authoritative owner; inactivity does not cancel, settle, waive, terminate, or reverse it.
8. Category and operational owners confirm whether foreseeable demand, critical dependency, sole-source need, emergency capability, or current market strategy requires continued Active status.
9. The supplier is notified or consulted where policy, relationship, or reactivation planning requires it.
10. Conflicts and unsupported requests are identified, including attempts to avoid governance, payment, contract, claim, or evidence obligations.
11. The recommendation defines reason, affected relationship scope, effective time, permitted historical and open-obligation activity, blocked future use, reactivation requirements, review date, and closure referral conditions.
12. The Supplier Governance Authority reviews reason, evidence, current governance states, open positions, supplier communication, operational need, and reactivation feasibility.
13. The approver rejects, returns, defers, or approves the transition.
14. The Supplier transitions prospectively from `Active` to the canonical `Inactive` state; supplier identity and all prior approval, qualification, activity, performance, contract, and finance history remain preserved.
15. New ordinary invitations, participation, award, and commitment reliance are blocked or require reactivation according to rule; existing-authority actions continue only within their scope.
16. Authorised sourcing, qualification, contract, finance, legal, compliance, supplier, and audit consumers receive the current relationship state and applicable reactivation conditions.
17. Reason, activity evidence, open-position referrals, approvals, notices, reviews, reactivation, closure referral, actors, timestamps, and versions remain auditable.

---

# 10. Alternate Business Flows

## 10.1 Supplier Requests Inactivity

The request is verified against identity and authority. Open obligations, legal holds, claims, warranties, tax, payment, and retention requirements remain enforceable.

## 10.2 Automatic Dormancy Candidate

A time-based rule creates a review candidate only. It does not change status until governed validation and approval occur.

## 10.3 Active Contract or Warranty

Relationship may remain Active or become Inactive with controlled existing-obligation access according to policy. The contract or warranty remains authoritative.

## 10.4 Expected Future Demand

The transition is rejected or deferred where justified. Continued Active status and review date are recorded.

## 10.5 Adverse Finding Discovered

The case is routed for assessment, restriction, suspension, or blacklisting. Inactivity cannot substitute for punitive governance.

## 10.6 Reactivation Requested

Identity, profile, representatives, qualifications, restrictions, suspension, blacklisting, legal, tax, banking-reference, conflict, and other required checks are refreshed before Active status is restored.

## 10.7 Relationship Ready for Closure

When no open obligations or foreseeable need remains and retention requirements are known, the relationship is referred to L3-J-034.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Open Position Not Detected

The status and affected position are reconciled. The authoritative contract or financial record remains valid.

## 11.2 Incorrect Inactivity Classification

The state is corrected prospectively and affected sourcing or operational actions are reviewed. Original error remains visible.

## 11.3 Punitive Reason Hidden as Inactivity

The attempt is rejected and referred to the correct governance authority.

## 11.4 Unauthorised Reactivation or Use

The action is blocked or escalated; any resulting transaction remains preserved for its owner's decision.

## 11.5 Notification Failure

Notice is reissued and delivery evidence retained. Business-effective state and communication failure remain separate.

## 11.6 System Interruption

The approved business-effective state remains controlling and is reconciled on recovery without backdating.

---

# 12. Business Rules and Decision Constraints

- Inactivity is a non-punitive relationship state based on approved reasons and evidence.
- A time threshold or lack of transactions creates a review candidate, not automatic inactivity unless expressly governed.
- Inactivity is distinct from qualification expiry, restriction, suspension, blacklisting, closure, and archival.
- Adverse findings cannot be hidden or disposed of through inactivity.
- Open Purchase Orders, contracts, warranties, claims, disputes, payables, payments, tax records, and legal holds remain with their authoritative owners.
- Inactivity does not delete supplier identity, qualifications, performance, transaction, communication, or audit history.
- Future ordinary use requires reactivation or an expressly permitted existing-obligation action.
- Reactivation requires current checks and cannot bypass restriction, suspension, blacklisting, or mandatory prohibition.
- A supplier may be eligible but inactive, or active but restricted in a specific scope; lifecycle states must remain distinguishable.
- Closure and archival require L3-J-034 and cannot occur merely because inactivity was approved.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Supplier | Existing canonical J02 state | Applies only an exact J02 supplier-governance transition. | `Restricted`, `Suspended`, `Blacklisted`, `Active`, `Inactive` or `Closed / Archived` as material to the scenario | Vendor & Supplier Management (J02) | `L2-J02` Section 6 |
| SupplierGovernanceCase | No canonical lifecycle defined | Preserves evidence and the attributable governance decision without creating local case states. | Governed case record linked to the exact Supplier state | Vendor & Supplier Management (J02) | `L2-J02` Section 5 |
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `SupplierInactivated` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
| `SupplierReactivated` | Vendor & Supplier Management (J02) | J01, J03, J04, J05, Finance and Reporting where material | Records the exact completed business fact material to this scenario. | `L2-J02 Section 13` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| Qualification / Sourcing | Supplies current eligibility, use, and future demand. | Retains qualification and sourcing decisions. |
| J04 / Contract / Operations | Supplies open commitments, warranties, claims, and continuity. | Inactivity does not change them. |
| Finance | Supplies payable, payment, tax, balance, and retention positions. | Retains monetary truth. |
| L3-J-028 to L3-J-032 | Supplies adverse governance and restoration states. | Inactivity cannot override them. |
| L3-J-034 / L3-J-035 | Receives closure or duplicate-identity referrals. | Retains closure and identity authority. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J02 roles are separated as policy requires: Supplier Representative; Institutional Verifier; and Supplier Governance Authority. The Supplier Representative cannot verify, approve, or remove an institutional restriction on itself, and a conflicted verifier cannot decide the governed outcome.

## 15.2 Required Evidence

- canonical supplier, current states, inactivity request and reason, last meaningful activity, activity definitions, profile and qualification currency, related identities, and duplicate indicators;
- sourcing, award, commitment, contract, delivery, warranty, claim, dispute, payable, payment, tax, legal-hold, and retention positions;
- supplier communication, operational need, conflicts, recommendation, approval, effective time, permitted activity, reactivation requirements, reviews, actors, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct why inactivity was considered, how activity and open positions were checked, why the reason was non-punitive, who approved the status, which future use was controlled, what obligations continued, and how reactivation or closure was later governed.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- an active supplier relationship has been transitioned to an Inactive status through an authorised governance decision based on verifiable inactivity criteria;
- the inactivity determination, evidence basis, effective date, scope, and authority are documented and attributable;
- existing obligations, open purchase orders, and in-progress transactions referencing the relationship have been assessed and dispositioned under their authoritative owners;
- the inactive status prevents new sourcing selection, award, or commitment creation under the relationship;
- the supplier's identity, historical transactions, and governance evidence are preserved and not destroyed;
- the inactivity decision, authority, actors, and audit trail are preserved and immutable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the supplier relationship is in an Inactive state from the effective date with an explicit inactivity determination and authority record;
- the inactive status prevents new institutional activity under the relationship without a separate activation or reactivation decision;
- existing obligations are governed by their authoritative domain owners under the disposition established at inactivity time;
- the complete relationship history is permanently preserved and auditable;
- reactivation requires a separate governed authority decision and may require re-qualification depending on the elapsed period.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Qualifying activity definitions, dormancy periods, automatic-candidate rules, approval authorities, review frequency, and reactivation checks require confirmation.
2. Treatment of active warranties, claims, critical suppliers, framework agreements, and credit or payable balances requires upstream cross-domain business contracts.
3. Supplier-requested inactivity and legal-entity cessation require legal and contract mapping.

## 17.2 Assumptions

- Inactivity is non-punitive and reversible.
- Existing contractual and financial obligations survive the relationship-state change.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-033**
