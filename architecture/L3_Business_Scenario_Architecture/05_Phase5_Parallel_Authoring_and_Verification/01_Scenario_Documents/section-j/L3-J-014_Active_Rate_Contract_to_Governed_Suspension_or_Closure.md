# Active Rate Contract to Governed Suspension or Closure

# 1. Scenario Metadata

**Scenario ID:** L3-J-014  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Critical  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `L2-J02_Vendor_and_Supplier_Management_Workflows.md`; `L2-J03_Purchase_Requisition_and_Approval_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** downstream sourcing and J04 commitment controls that must respect suspended, expired, or closed availability  

---
# 2. Scenario Purpose

This scenario governs temporary suspension or permanent closure of an Active rate contract when expiry, supplier restriction, performance failure, legal or policy change, commercial defect, ceiling exhaustion, institutional decision, or completion of purpose removes or limits its authority for new demand sourcing.

It protects existing valid commitments and historical evidence while immediately preventing unauthorised new call-offs.

---

# 3. Business Outcome

An Active rate contract, supplier position, catalogue scope, or line is placed into an authorised Suspended or Closed state with effective timing, reason, affected scope, treatment of open demand and commitments, notification, recovery or successor path, and immutable history.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed trigger indicates that an Active rate contract or part of it may no longer remain available for new sourcing.

## 4.2 End Boundary

The scenario ends when suspension or closure is approved and effective, affected new call-offs are blocked, existing positions are dispositioned, and any remediation, reopening, successor, re-sourcing, or archival path is recorded.

## 4.3 In Scope

- validating trigger, contract identity, version, affected suppliers, catalogue lines, locations, and effective timing;
- distinguishing temporary suspension from permanent closure or expiry;
- assessing supplier eligibility, performance, compliance, dispute, security, ceiling, legal, commercial, and institutional facts;
- identifying active sourcing outcomes, pending call-offs, Purchase Orders, deliveries, disputes, securities, warranties, and residual demand;
- approving full or partial suspension or closure;
- blocking new use within affected scope while preserving existing commitment authority;
- notifying affected units and publishing authoritative availability facts;
- governing remediation, reinstatement, successor, re-sourcing, expiry, and archival hand-offs.

## 4.4 Out of Scope

- original establishment or normal call-off;
- supplier suspension or blacklisting lifecycle owned by J02, though its facts may trigger this scenario;
- Purchase Order cancellation, amendment, receipt, payable, accounting, or payment;
- legal dispute resolution or technical enforcement design;
- deletion of contract, supplier, rate, or call-off history.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-011 | Established the Active arrangement. |
| Related | L3-J-012 | Must stop or redirect affected demand use. |
| Related | L3-J-013 | May provide a successor before closure. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns contract availability, suspension or closure assessment, decision, effective scope, downstream sourcing restriction, remediation or successor hand-off, and historical record. |

J02 retains supplier status, J04 retains Purchase Orders, and Finance retains monetary truth. J01 does not use rate-contract closure to cancel or rewrite those externally owned records.

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

- The Rate Contract and affected version, supplier, line, location, or scope are identifiable.
- The affected position is Active, Approved awaiting activation, Suspended, or in an expiry or closure transition.
- A trigger and supporting evidence exist.
- Current call-offs, pending sourcing recommendations, Purchase Orders, deliveries, disputes, securities, warranties, ceilings, and residual demand can be identified.
- Applicable notice, representation, urgent-action, suspension, closure, reinstatement, and authority rules are effective.
- Downstream owners can receive availability restrictions without J01 rewriting their records.

---

# 8. Trigger

Expiry, ceiling exhaustion, supplier restriction, material performance or compliance finding, legal or policy prohibition, unresolved commercial defect, institutional decision, replacement by successor, or completion of purpose requires review of continued availability.

---

# 9. Main Business Flow

1. Procurement records the trigger, evidence source, discovery time, affected contract identity, version, suppliers, lines, locations, and proposed effective time.
2. Current lifecycle, validity, ceiling, supplier status, performance, incidents, disputes, and contractual conditions are verified.
3. Pending and completed call-off sourcing outcomes, active Purchase Orders, deliveries, warranties, claims, securities, and residual demand are identified.
4. The impact on safety, legality, compliance, continuity, price, quality, service, funding, and institutional operations is assessed.
5. Procurement determines whether no action, partial suspension, full suspension, expiry closure, early closure, supplier-specific suspension, line closure, or immediate urgent restriction is appropriate.
6. Urgent provisional restriction may block new use pending final decision where policy permits and risk requires it.
7. The recommendation defines scope, reason, evidence, effective date and time, duration where temporary, notice, affected new sourcing, open commitments, residual demand, and recovery or successor path.
8. Existing Purchase Orders are classified for continuation, review, hold, amendment, cancellation, or other J04-owned action; J01 does not decide their lifecycle here.
9. Supplier notice and opportunity to respond or remediate are provided where required, unless urgent rules lawfully permit immediate action.
10. Specialist facts and supplier representations are reviewed without erasing the original trigger.
11. The approver decides the exact action, scope, timing, duration, conditions, and authority.
12. On suspension, affected availability reaches Suspended and new call-off sourcing is blocked until authorised reinstatement or closure.
13. On closure or expiry, affected availability reaches Closed and cannot authorise new call-offs.
14. Pending uncommitted sourcing outcomes are cancelled, returned, or redirected under their own governance.
15. Residual demand is made available for successor, alternate sourcing, or authoritative demand change.
16. Affected units, J02, J04, Finance where relevant, suppliers, and oversight roles receive governed facts.
17. Historical contract terms, earlier availability, call-offs, commitments, actions, reasons, and versions remain immutable and auditable.

---

# 10. Alternate Business Flows

## 10.1 Partial Suspension

Only specified suppliers, lines, categories, locations, or call-off types are suspended. Unaffected scope remains Active with explicit boundaries.

## 10.2 Immediate Protective Suspension

Where safety, fraud, legal prohibition, security, blacklisting, or serious risk requires immediate action, provisional suspension is effective under emergency authority and proceeds to timely review.

## 10.3 Time-Bound Suspension and Remediation

The supplier or institution may satisfy defined remediation, evidence, security, performance, or compliance conditions. Reinstatement requires a separate attributable decision; elapsed time alone does not reactivate the contract.

## 10.4 Closure at Expiry

The contract closes when validity ends and no approved extension or overlap applies. Existing valid commitments remain governed by J04.

## 10.5 Early Closure by Institutional Decision

Early closure records authority, contractual basis, notice, operational impact, residual demand, and downstream review needs.

## 10.6 Closure After Successor Activation

The predecessor closes at the authorised cut-off after successor activation. Controlled overlap and remaining commitments remain traceable.

## 10.7 Ceiling Exhaustion

New sourcing stops when the applicable ceiling is exhausted. Additional authority requires a governed amendment, successor, or new sourcing arrangement, not silent ceiling increase.

## 10.8 Suspension Lifted

Where remediation succeeds, an approved reinstatement creates a new availability fact and effective time while preserving the suspension interval.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Unverified Trigger

Ordinary permanent action is withheld pending evidence, but urgent provisional protection may apply under valid authority.

## 11.2 Scope Ambiguity

If affected suppliers, lines, locations, or periods cannot be determined, new use is conservatively withheld within the authorised risk boundary until clarified.

## 11.3 Downstream Commitment Conflict

Suspension or closure does not silently cancel Purchase Orders. J04 receives the fact and governs each commitment.

## 11.4 Notification Failure

Business effectiveness follows the approved rule, not mere message success. Failed delivery is recorded and reissued through approved channels.

## 11.5 Unauthorised Continued Use

Any post-effective call-off attempt is blocked and escalated. If a commitment was created, it is investigated under J04 and governance controls.

## 11.6 Reinstatement Error

An expired or closed contract cannot be reopened merely by removing a suspension flag. Valid successor, amendment, or reinstatement authority is required.

## 11.7 Post-Closure Discovery

New evidence is appended through a governed supplemental record; historical closure is not erased.

---

# 12. Business Rules and Decision Constraints

- Suspension and closure require identifiable scope, reason, evidence, authority, and effective time.
- Urgent provisional suspension must have a valid basis and subsequent review.
- Partial action must clearly distinguish affected and unaffected scope.
- Suspended, expired, or closed positions cannot authorise new call-offs.
- Existing valid Purchase Orders do not automatically terminate.
- Ceiling exhaustion cannot be bypassed through backdating, duplicate outcomes, or silent increase.
- Supplier status remains J02-owned, but J01 must consume restrictions for sourcing availability.
- Reinstatement must be explicit, approved, and effective-dated.
- Closure is permanent for the controlled version; future need requires a successor or new arrangement.
- Historical rates, versions, call-offs, decisions, and suspension intervals cannot be overwritten.
- Residual demand remains visible and is not deemed fulfilled.
- This scenario does not create accounting entries, payables, receipts, or payments.

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Rate Contract | `Active` or `Suspended` | Applies only the governed lifecycle decision supported by J01. | `Active`, `Suspended` or `Expired / Closed` | Procurement Planning & Sourcing (J01) | `L2-J01` Section 27.3 |

No provisional, partial, review or successor-required state is introduced by this scenario.
---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Material Consumer(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `RateContractSuspended` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractExpired` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
| `RateContractClosed` | Procurement Planning & Sourcing (J01) | J01 participants; J04 for approved sourcing outcomes; Audit and Reporting | Records the exact completed business fact material to this scenario. | `L2-J01 Section 35` |
## 14.2 Cross-Domain Collaboration

| Participant | Collaboration | Boundary |
|---|---|---|
| J02 | Publishes supplier eligibility, suspension, blacklisting, and remediation facts. | Retains supplier lifecycle. |
| J03 / Requesting Units | Supply residual demand and operational impact. | Retain demand authority. |
| J04 | Receives restriction and reviews open commitments. | Retains Purchase Order lifecycle. |
| Finance | Receives facts affecting monetary, security, payable, or budget review. | Retains financial truth. |
| J-013 | Supplies successor and cut-off facts. | Retains successor governance. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

The canonical J01 roles are separated as institutional policy requires: Procurement Officer; Procurement Manager; Requesting Unit Representative; Technical Evaluator; Commercial Evaluator; Committee Member and Committee Chair / Convenor where used; Procurement Recommender; Procurement Approver; External Procurement Coordinator where applicable; and Auditor / Reviewer. A requester, evaluator, recommender, or conflicted participant cannot replace the Procurement Approver decision.

## 15.2 Required Evidence

- contract identity, version, status, validity, suppliers, catalogue, locations, rates, ceilings, and call-off rules;
- trigger, evidence, supplier status, performance, incidents, legal or policy facts;
- pending outcomes, Purchase Orders, deliveries, disputes, warranties, securities, and residual demand;
- scope, effective time, urgency, notice, representation, remediation, successor, and downstream disposition;
- recommendation, approval, reinstatement or closure decision, actors, reasons, timestamps, and versions.

## 15.3 Audit Obligations

The institution must reconstruct availability at any point in time and explain every suspension, restriction, reinstatement, expiry, and closure and its effect on new sourcing and existing commitments.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- an active Rate Contract has been transitioned to Suspended or Closed through an authorised governance decision with a documented reason;
- the suspension or closure authority, evidence, scope, and effective time are recorded and attributable;
- demand routing against the affected contract is blocked from the effective time of the suspension or closure decision;
- outstanding demand, open purchase orders, and delivery obligations are dispositioned or explicitly preserved under their authoritative owners;
- the Rate Contract's history, terms, and audit lineage are preserved and not altered;
- a governed successor path, reactivation condition, or permanent closure determination is explicit where applicable.


## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- the Rate Contract is in a Suspended or Closed state and is no longer available for new demand routing;
- all active demand, open orders, and pending obligations referencing the contract have a governed disposition under their authoritative domain owners;
- the suspension or closure decision, authority, reason, and effective time are auditable and immutable;
- a reactivation path or permanent closure fact is established where institutionally required;
- no new procurement commitment may be created against a Closed Rate Contract.


---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. Notice, representation, urgent-action, remediation, and appeal requirements are institution-specific.
2. Boundary between rate-contract suspension and J02 supplier restriction requires final event and authority mapping.
3. Treatment of approved but uncommitted call-off outcomes at suspension requires operating-profile governance.

## 17.2 Assumptions

- Suspension blocks new use but does not erase the contract.
- Closure ends new-use authority while preserving existing valid commitments and history.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-014**
