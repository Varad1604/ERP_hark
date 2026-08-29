# Approved Procurement Plan Change to Governed Successor Plan

# 1. Scenario Metadata

**Scenario ID:** L3-J-002  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-001 Procurement Planning Requirement to Approved Active Procurement Plan; L3-J-003 Active Procurement Plan to Governed Closure; downstream procurement-demand and sourcing scenarios that consume the effective planned-requirement version  

---
# 2. Scenario Purpose

This scenario governs a material change to an approved or active Procurement Plan through a controlled successor version. It allows planning information to remain current while protecting the exact content, authority, and evidence of the prior approved version.

The scenario prevents silent overwrite of procurement-planning history and ensures that changed requirements, change reason, actor, approval, and effective date remain traceable.

---

# 3. Business Outcome

An approved successor Procurement Plan is effective for the defined scope and period, while the prior approved version, changed requirements, reason, authority, approval evidence, and effective date remain permanently traceable.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when valid business evidence establishes that an approved or active Procurement Plan requires a controlled revision.

## 4.2 End Boundary

The scenario ends when the successor plan version is approved and becomes effective, with its relationship to the prior version and the disposition of every changed planned requirement preserved. A returned revision remains incomplete and does not replace the effective plan.

## 4.3 In Scope

- identifying and justifying a change to an approved or active plan;
- creating a successor version linked to the prior approved version;
- adding, changing, rescheduling, consolidating, separating, or removing planned requirements where authorised;
- assessing effects on scope, period, estimates, priority, specifications, proposed method, or downstream references;
- reviewing and approving the exact successor version;
- assigning the effective date of the successor;
- preserving prior version, changed requirements, reason, actor, authority, and approval history;
- making the approved successor the effective planning reference.

## 4.4 Out of Scope

- creation of the initial Procurement Plan, governed by L3-J-001;
- closure of the active plan, governed by L3-J-003;
- correction of a draft plan before its first approval, which remains within the initial-plan scenario;
- Purchase Requisition or procurement-demand amendment, owned by J03;
- sourcing-event amendment, cancellation, re-sourcing, or supplier evaluation;
- budget revision, commitment, expenditure, accounting, or payment, owned by Finance;
- Purchase Order amendment, owned by J04;
- destructive replacement of prior plan history;
- technical versioning, data storage, interface, or concurrency design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-001 Procurement Planning Requirement to Approved Active Procurement Plan | Establishes the approved or active plan that becomes the revision baseline. |
| Followed by | L3-J-003 Active Procurement Plan to Governed Closure | Governs later completion and closure of the effective plan. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns the revision request, successor plan version, version relationship, planned-requirement changes, review, approval, effective date, and final successor-plan outcome. |

External organisational, item, specification, budget, and operational facts may support the revision but remain owned by their authoritative domains. Their contribution does not transfer successor-plan ownership.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Planner | Records the revision basis, prepares the successor version, identifies every material change, and submits it for review. | Prepares and submits. |
| Requesting Unit Representative | Confirms changed requirement, timing, quantity, priority, purpose, or specialist specification context where relevant. | Contributes evidence and planning context. |
| Procurement Manager | Reviews necessity, completeness, consolidation, traceability, policy alignment, and downstream impact of the successor. | Reviews and may return for revision. |
| Procurement Approver | Decides whether the successor version may replace the prior version as the effective plan. | Approves the exact successor version. |
| Auditor / Reviewer | Reviews the predecessor-successor chain, change evidence, approval, and effective history. | Reviews evidence without modifying plan history. |

Role combinations and separation requirements follow institution-configured policy.

---

# 7. Preconditions and Entry Criteria

- A unique approved or active Procurement Plan and its effective version exist.
- The prior version, approval evidence, scope, period, and planned requirements are available as an immutable revision baseline.
- A material change requirement is supported by an attributable business reason or evidence.
- The intended change falls within the Procurement Plan boundary rather than a downstream requisition, sourcing event, Purchase Order, or financial record.
- The initiator is authorised for the applicable institution and organisational scope.
- Applicable revision, review, approval, and effective-date rules are known for the plan context.
- Relevant downstream references to the prior plan or planned requirements can be identified for impact assessment.

---

# 8. Trigger

An authorised actor identifies that an approved or active Procurement Plan must change because planning assumptions, requirements, timing, quantities, priorities, scope, supporting references, or institutional circumstances have materially changed.

---

# 9. Main Business Flow

1. The Procurement Planner receives the change requirement and identifies the currently effective plan and exact approved version.
2. The planner records the revision reason, initiating actor, evidence, requested effective timing, and affected planned requirements.
3. The planner confirms that the requested change belongs to procurement planning and does not attempt to rewrite a downstream authoritative record.
4. A successor plan version is created from the approved baseline with an explicit predecessor-successor relationship. The prior version remains unchanged.
5. The planner records each addition, modification, rescheduling, consolidation, separation, or removal of a planned requirement.
6. For every material change, the successor preserves the prior value, successor value or disposition, reason, source evidence, and affected requirement identity.
7. The planner reassesses internal plan consistency, planning period, organisational scope, estimated quantity, expected date, priority, estimated-cost reference, specification reference, proposed procurement method, and budget reference where applicable.
8. Any consolidation or deconsolidation preserves traceability to originating units and requirements.
9. The planner assesses whether existing downstream requisition or sourcing references are merely linked to the prior planning version or require action by their owning domains. J01 does not alter those records.
10. The successor version is submitted as a controlled review baseline.
11. The Procurement Manager reviews the change justification, complete change set, policy compliance, traceability, estimates, affected references, and proposed effective date.
12. The Procurement Approver reviews the exact successor version under the applicable delegated authority.
13. When approved, the approval decision, authority, timestamp, conditions, evidence, and exact successor version are recorded.
14. The approved successor becomes effective on its governed effective date.
15. The prior version remains historical and traceable; it is not deleted, rewritten, or represented as having contained the successor changes.
16. The effective successor and changed planned requirements become available as the current planning references for subsequent activity.
17. Completed revision and effectiveness facts are recorded with an auditable predecessor-successor chain.

---

# 10. Alternate Business Flows

## 10.1 Return for Revision

1. The reviewer or approver identifies an incomplete change rationale, unsupported estimate, missing impact, inconsistent requirement, invalid authority, or evidence deficiency.
2. The successor submission is returned with specific reasons.
3. The submitted successor version and return disposition remain preserved.
4. The planner prepares a corrected successor version or controlled iteration without changing the prior approved plan.
5. The corrected version is resubmitted through the applicable review and approval path.

## 10.2 Addition of a New Planned Requirement

A newly anticipated requirement may be added to the successor plan with its origin, planning details, justification, and effective date. Addition to the plan does not create authorised procurement demand.

## 10.3 Reduction, Deferral, or Removal

A requirement may be reduced, deferred, or removed when supported by authority and reason. The prior requirement remains visible in historical versions, and any downstream record already created from it remains under its owning domain.

## 10.4 Consolidation or Separation

Similar requirements may be consolidated, or a consolidated requirement may be separated, when planning governance permits. Source-to-successor traceability must remain complete.

## 10.5 Future-Dated Successor

The successor may be approved before its effective date. Until that date, the current approved version remains the effective planning reference unless policy defines another governed transition.

## 10.6 Non-Material Correction

If policy distinguishes non-material administrative correction from controlled plan revision, the applicable treatment must still preserve audit history. L2-J01 does not define this distinction; it is recorded as an open issue rather than invented here.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid or Stale Baseline

If the proposed successor was prepared from a plan version that is no longer effective, approval is withheld. The change must be reconciled against the current version and resubmitted as a new controlled successor.

## 11.2 Concurrent Revision Proposals

Where multiple successor proposals exist for the same effective plan, they must be consolidated, sequenced, or otherwise governed before one becomes effective. Competing successors must not silently overwrite each other.

## 11.3 Incomplete Change Traceability

If the changed requirements, prior values, successor values, removal dispositions, reasons, or evidence cannot be reconstructed, the successor cannot be approved.

## 11.4 Invalid Approval Authority

If the approving actor lacks authority for the plan scope, category, anticipated value, or revision type, the successor remains ineffective and is escalated for valid approval.

## 11.5 Review or Approval Interruption

An interruption preserves the submitted successor and its predecessor relationship. No approval or effectiveness may be inferred from partial review activity.

## 11.6 Downstream Reference Conflict

If a changed planned requirement already relates to a requisition, sourcing event, or other downstream record, J01 preserves that reference and communicates the planning change. The downstream owner determines any consequence; J01 does not rewrite the external record.

## 11.7 Erroneous Successor Activation

If an effective successor is later found materially wrong, it is not rewritten. A further governed successor must be created, approved, and made effective, preserving both earlier versions.

---

# 12. Business Rules and Decision Constraints

- An approved or active Procurement Plan must never be silently overwritten.
- Every successor must identify its direct predecessor and preserve a continuous version chain.
- Revision evidence must preserve the previous version, new version, change reason, changed requirements, actor, approval where required, and effective date.
- The exact successor version reviewed and approved must be the version made effective.
- The prior version remains historically truthful after the successor becomes effective.
- Planned estimates remain planning information and must not become Finance-owned balances, commitments, expenditure, or payment obligations.
- A changed planned requirement must not rewrite an existing Purchase Requisition, authorised demand, sourcing event, supplier outcome, Purchase Order, or financial record.
- Addition or continuation of a requirement in an approved plan does not itself authorise procurement demand or sourcing.
- Removal from a successor plan must not erase the fact that the requirement existed in a prior approved version.
- Consolidation and separation must preserve traceability to originating requirements and units.
- Approval follows institution-configured delegated authority and separation rules.
- A returned or unapproved successor must not replace the currently effective plan.
- A future-dated successor does not become current before its governed effective date.
- Historical documents, approval evidence, and version relationships must remain attributable and reconstructable.

---

# 13. State and Lifecycle Impact

## 13.1 Successor Revision Lifecycle

| Revision Position | Business Action | Resulting Position | Required Result |
|---|---|---|---|
| Approved or Active plan version | Initiate controlled revision | Draft successor | New version is linked to the effective predecessor. |
| Draft successor | Submit | Submitted successor | Exact successor version becomes the review baseline. |
| Submitted successor | Begin review | Successor under review | Change rationale and complete change set are examined. |
| Successor under review | Return | Returned successor | Return reasons are recorded; predecessor remains effective. |
| Returned successor | Correct and resubmit | Submitted successor | Corrected controlled version re-enters review. |
| Successor under review | Approve | Approved successor | Approval applies to the exact successor version. |
| Approved successor | Reach governed effective date | Effective successor | Successor becomes current; predecessor becomes historical. |

L2-J01 states that an active plan is revised through a controlled version but does not define separate canonical states for successor preparation. The positions above describe business progression without adding states to the authoritative Procurement Plan lifecycle.

## 13.2 Predecessor Impact

The predecessor is superseded for current planning use when the successor becomes effective. It remains immutable, historically valid for its former effective period, and available for audit and downstream traceability.

## 13.3 Planned Requirement Impact

Each affected planned requirement receives a traceable successor treatment: added, continued unchanged, modified, rescheduled, consolidated, separated, deferred, reduced, or removed. These treatments do not create downstream procurement or financial authority.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published Business Events

| Business Event | Publisher | Business Meaning |
|---|---|---|
| ProcurementPlanRevisionCreated | Procurement Planning and Sourcing | A controlled successor version has been created from an approved baseline. |
| ProcurementPlanSubmitted | Procurement Planning and Sourcing | The successor version has entered governed review. |
| ProcurementPlanReturnedForRevision | Procurement Planning and Sourcing | The successor requires correction and has not replaced the effective plan. |
| ProcurementPlanRevisionApproved | Procurement Planning and Sourcing | The exact successor version has received valid approval. |
| ProcurementPlanActivated | Procurement Planning and Sourcing | The approved successor has become effective as the current plan. |

The event catalogue does not provide a separately named successor-effective or predecessor-superseded event. This scenario uses the locked representative event names without inventing additional canonical events.

## 14.2 Cross-Domain Collaboration

| Source or Consumer | Collaboration | Boundary |
|---|---|---|
| Requesting unit or specialist domain | Supplies changed operational requirement or specification evidence. | Retains ownership of specialist meaning. |
| Institution and Organisation architecture | Supplies current organisational identities and scope. | J01 does not change organisational structure. |
| J03 Purchase Requisition and Approval | May consume updated planning references or assess an already-created demand. | J03 alone governs requisition and demand consequences. |
| J06 where applicable | Supplies item or category reference. | J01 does not alter Item Master. |
| Finance where applicable | Supplies budget or cost reference and receives planning-change context where relevant. | Finance retains monetary truth and determines financial consequences. |
| Enterprise document capability | Preserves controlled plan and evidence documents. | J01 owns their procurement relationship, not storage architecture. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Approval

- The revision initiator must be authorised for the applicable plan scope.
- The successor must follow the approval authority effective for its scope, category, anticipated value context, materiality, and effective date.
- Preparation and approval must be separated where institutional policy requires.
- Approval must identify the exact predecessor and successor versions.
- Conditions attached to approval must be explicit and must not be treated as satisfied without evidence.

## 15.2 Required Approval Evidence

- plan identity and predecessor version;
- successor version and complete change set;
- change reason and initiating evidence;
- changed requirement identities and dispositions;
- approving actor, authority, decision, and timestamp;
- proposed and confirmed effective date;
- material downstream references and communication evidence where applicable;
- supporting document versions and integrity references.

## 15.3 Audit Obligations

The scenario must permit reconstruction of:

- who initiated, prepared, reviewed, returned, approved, and activated the successor;
- what changed between the predecessor and successor;
- why each material change occurred;
- when each version was approved and effective;
- which version governed a given point in time;
- how originating requirements and consolidated entries relate across versions;
- which downstream records referenced either version;
- every returned, abandoned, or superseded successor proposal without erasing it.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the prior approved or active plan and its exact effective version are preserved;
- a uniquely identified successor version is linked to that predecessor;
- every material planned-requirement change and disposition is traceable;
- the revision reason, actor, evidence, and affected scope are recorded;
- estimates and references remain correctly bounded as planning information;
- downstream impacts have been identified without J01 rewriting external records;
- applicable review and valid approval are complete against the exact successor version;
- the successor's effective date is explicit;
- the approved successor is effective as the current planning reference;
- the predecessor remains immutable and historically available;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- published facts do not imply requisition approval, sourcing authority, supplier award, Purchase Order commitment, or financial posting;
- the complete predecessor-successor, approval, and evidence trail is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. L2-J01 requires controlled revision but does not define whether successor preparation uses the standard Draft, Submitted, Under Review, Returned for Revision, and Approved states within the same aggregate or a separate revision sub-lifecycle. This must be resolved before L4 state design.
2. L2-J01 does not define whether an approved successor becomes effective immediately by default or may require a separately authorised future effective date in all institutions.
3. No canonical event is listed specifically for `ProcurementPlanSuccessorEffective` or `ProcurementPlanSuperseded`; event sufficiency must be confirmed before locking cross-domain consumption.
4. Material versus non-material plan change is not defined upstream. This scenario treats material change as requiring a controlled successor and does not invent a shortcut for administrative correction.
5. Upstream architecture does not define the resolution rule for concurrent successor proposals. Governance must establish whether proposals are merged, sequenced, withdrawn, or otherwise dispositioned.

## 17.2 Assumptions

- The currently effective approved version remains current until a valid successor reaches its governed effective date.
- A plan-version change does not automatically change downstream requisitions, sourcing events, Purchase Orders, inventory, assets, or financial records.
- The successor is a complete governed plan version rather than a destructive patch over historical content.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-002**
