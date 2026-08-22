# Procurement Planning Requirement to Approved Active Procurement Plan

# 1. Scenario Metadata

**Scenario ID:** L3-J-001  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Chetan  
**Last Updated:** 2026-08-21  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-002 Approved Procurement Plan Change to Governed Successor Plan; L3-J-003 Active Procurement Plan to Governed Closure; downstream purchase-requisition and sourcing scenarios where a planned-requirement reference is used  

---
# 2. Scenario Purpose

This scenario governs the establishment of an institutional Procurement Plan for a defined planning period and organisational scope. It converts an identified procurement-planning requirement into an approved, active plan containing traceable anticipated requirements that may inform later procurement demand.

The scenario provides planning discipline without treating estimated quantities, dates, methods, or costs as authorised procurement demand, financial commitment, expenditure, or supplier commitment.

---

# 3. Business Outcome

An approved Procurement Plan is active for the authorised institutional scope and planning period, with its planned requirements, approval evidence, effective version, and planning references available for traceable downstream use.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised institutional planning requirement exists for a defined period and organisational scope and procurement planning is applicable under the institution's configured operating model.

## 4.2 End Boundary

The scenario ends successfully only when the reviewed plan version has received valid institutional approval and is Active for the defined scope and planning period. A return for revision is an alternate business path that keeps the scenario incomplete until the corrected plan is resubmitted, approved, and activated. Rejection is not defined as a canonical Procurement Plan lifecycle state in L2-J01 and is therefore not invented here.

## 4.3 In Scope

- establishing the plan period and organisational scope;
- creating the initial controlled Procurement Plan version;
- recording anticipated resource or service requirements;
- preserving estimated quantity, timing, priority, cost reference, specification reference, and proposed procurement-method information where applicable;
- reviewing completeness, scope, traceability, and policy alignment;
- returning the plan for revision with reasons where necessary;
- approving and activating the accepted plan version;
- preserving approval and lifecycle evidence.

## 4.4 Out of Scope

- controlled revision of an approved or active plan, governed by L3-J-002;
- closure of an active plan, governed by L3-J-003;
- creation or approval of a Purchase Requisition or authorised procurement demand, owned by J03;
- sourcing-method execution, supplier participation, evaluation, award recommendation, or sourcing approval;
- supplier identity or qualification, owned by J02;
- Purchase Order creation or supplier commitment, owned by J04;
- budget ownership, financial commitment, expenditure, accounting, payable recognition, or payment, owned by Finance;
- technical implementation, user-interface, data-schema, or access-control design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Followed by | L3-J-002 Approved Procurement Plan Change to Governed Successor Plan | Governs controlled change after approval or activation. |
| Followed by | L3-J-003 Active Procurement Plan to Governed Closure | Governs completion and closure of the active plan. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns the Procurement Plan, planned requirements, lifecycle, version, approval state, activation, revision history, and final active-plan outcome. |

Organisational identities, specialist requirement semantics, item references, and budget references remain owned by their authoritative domains. Their use as planning references does not transfer ownership or make those domains participants in this simple J-owned scenario.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Planner | Establishes the plan, planning period, scope, planned requirements, and supporting references. | Prepares and submits. |
| Requesting Unit Representative | Supplies or confirms anticipated requirement and specification context for the represented unit where required. | Contributes planning information. |
| Procurement Manager | Reviews the plan for institutional completeness, consolidation, policy alignment, and readiness. | Reviews and may return for revision. |
| Procurement Approver | Exercises the institutionally configured approval authority over the plan. | Approves the plan version for activation. |
| Auditor / Reviewer | Examines plan, approval, change, and traceability evidence where authorised. | Reviews evidence without changing the plan. |

An institution may combine roles where its policy permits. Any configured separation between preparation, review, and approval remains applicable.

---

# 7. Preconditions and Entry Criteria

- Procurement planning is enabled or otherwise required for the applicable institution and planning context.
- The intended planning period is identifiable and valid under institutional configuration; it may be a financial year, academic year, semester, quarter, or institution-defined period.
- The organisational scope is identifiable through authoritative institution, campus, department, school, faculty, or other organisational-unit references.
- The Procurement Planner is authorised for the applicable institution and organisational scope.
- Applicable planning policies, approval requirements, and evidence requirements are effective for the intended plan period.
- Anticipated requirements can be supported by sufficient planning information to permit meaningful review.
- Where item, specification, category, budget, or organisational references are used, they resolve to authoritative references without transferring their ownership to J01.

---

# 8. Trigger

An authorised institutional requirement to establish procurement planning for a new planning period and defined organisational scope is received by Procurement Planning and Sourcing.

---

# 9. Main Business Flow

1. The Procurement Planner accepts the planning requirement and confirms that procurement planning applies to the identified institutional context.
2. The planner establishes the planning period, organisational scope, and initial plan version.
3. The planner records anticipated procurement requirements for the scope. Each requirement receives a traceable identity within the plan.
4. For each planned requirement, the planner records the available planning information, which may include description, category, estimated quantity, unit of measure, expected date, requesting unit, purpose, priority, technical-specification reference, estimated-cost reference, proposed procurement method, budget reference, or existing-item reference.
5. The planner verifies that estimates are explicitly treated as planning information and do not represent authorised demand, financial commitment, expenditure, or supplier commitment.
6. Similar anticipated requirements may be consolidated where institutionally appropriate. Traceability to each originating unit and requirement is preserved.
7. The draft plan is checked for period and scope consistency, requirement completeness, duplicate or conflicting planning entries, material omissions, supporting references, and applicable policy requirements.
8. The Procurement Planner submits the plan for governed review. The submitted version becomes the review baseline and is not silently rewritten during review.
9. The Procurement Manager reviews the plan against applicable planning policy, scope, timing, prioritisation, consolidation, documentation, and downstream traceability expectations.
10. Where the plan is acceptable, the Procurement Approver reviews the submitted version and supporting evidence under the applicable delegated authority.
11. The approver records the approval decision, authority, date, and evidence reference against the exact reviewed version.
12. The approved version is activated for its defined scope and period.
13. The active plan and its planned requirements become available as traceable planning references for subsequent procurement-demand activity, without bypassing J03 approval semantics.
14. The scenario records the completed approval and activation facts with reconstructable audit evidence.

---

# 10. Alternate Business Flows

## 10.1 Simplified Institutional Planning Lifecycle

Where institutional configuration does not require a distinct formal approval sequence, the plan may follow the approved simplified lifecycle. The applicable authority, activation fact, plan version, and evidence must still be attributable and preserved.

## 10.2 Return for Revision

1. The reviewer or approver identifies a correctable omission, inconsistency, unsupported estimate, scope problem, or evidence deficiency.
2. The submitted plan is returned with specific reasons and required corrections.
3. The returned version and return decision remain preserved.
4. The Procurement Planner prepares a corrected controlled version or revision according to the applicable pre-approval rules.
5. The corrected plan is resubmitted through the required review and approval path.
6. This path does not complete the scenario until the corrected plan satisfies the successful End Boundary in §4.2.

## 10.3 Multi-Unit Consolidated Plan

Anticipated requirements from several campuses, departments, schools, faculties, or other units may be consolidated into one planning outcome. Each source requirement and originating unit remains traceable through consolidation.

## 10.4 Partial Readiness

Where some planned requirements are sufficiently defined and others require further information, the plan may proceed only if institutional policy permits and the unresolved entries are explicitly identified and governed. Material omissions must not be concealed to obtain approval.

## 10.5 External or Specialist Reference Contribution

The plan may reference requirements, specifications, estimates, or evidence maintained by other institutional domains. J01 preserves the procurement-planning relationship and reference but does not redefine the authoritative external content.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Invalid Planning Period or Scope

If the period or organisational scope is invalid, inactive, overlapping contrary to policy, or unresolved, submission or activation is withheld. The planner corrects the authoritative reference or records the matter for governed resolution before continuing.

## 11.2 Incomplete or Unsupported Planned Requirement

If a material requirement lacks sufficient description, quantity, timing, origin, purpose, priority, or supporting reference, it is corrected, explicitly deferred where policy permits, or removed from the submitted scope with the reason preserved.

## 11.3 Duplicate or Conflicting Requirements

Potential duplicates or conflicting anticipated requirements are reviewed before approval. Consolidation, separation, correction, or retention as distinct requirements must preserve the relationship and decision basis.

## 11.4 Missing Approval Authority

If no valid approver can be resolved for the scope, period, category, or value context, the plan remains under review and cannot become active. The authority gap is escalated through institutional governance.

## 11.5 Review or Approval Interruption

An interrupted review does not erase the submitted baseline or imply approval. Processing resumes against the same controlled version unless a governed change requires a new version.

## 11.6 Activation Failure at the Business Level

If approval exists but a business condition prevents activation, the plan remains Approved and inactive until the condition is resolved. Reapproval is required only when the approved content, effective scope, period, or governing authority materially changes.

## 11.7 Erroneous Approval or Activation

An approved or active plan is not silently overwritten. Correction proceeds through the controlled successor-plan scenario L3-J-002, preserving the original approval and activation history.

---

# 12. Business Rules and Decision Constraints

- Procurement planning is configurable and may be disabled for institutions that do not require it.
- The planning period must be explicit; academic and financial planning periods must not be assumed identical.
- The plan scope must resolve to authoritative organisational references.
- Estimated values are planning information and must not become authoritative budget balances, commitments, expenditure, accounting facts, or payment obligations.
- A Procurement Plan represents anticipated demand and must not replace a J03 Purchase Requisition or authorised procurement demand.
- The scenario must not silently bypass J03 approval semantics.
- Planned requirements must remain traceable to their originating scope and unit, including after consolidation.
- An approved plan version must not be silently overwritten.
- The version reviewed and approved must be the version activated.
- Approval must follow institution-configured authority and separation rules.
- Return for revision must preserve the submitted version, reasons, authority, and history.
- Approval or activation must not create a Purchase Order, supplier award, budget commitment, or sourcing outcome.
- Planning references owned outside J01 remain external references; J01 must not create competing organisational, item, supplier, specification, or financial masters.
- Significant planning actions must be attributable and reconstructable.

---

# 13. State and Lifecycle Impact

## 13.1 Procurement Plan Lifecycle

| From State | Business Action | To State | Required Result |
|---|---|---|---|
| None | Create initial plan | Draft | Plan identity, period, scope, and initial version exist. |
| Draft | Submit for review | Submitted | Submitted version is preserved as the review baseline. |
| Submitted | Begin governed review | Under Review | Applicable review is in progress. |
| Under Review | Return with reasons | Returned for Revision | Return decision and required corrections are recorded. |
| Returned for Revision | Correct and resubmit | Submitted | Corrected controlled version enters review. |
| Under Review | Approve | Approved | Approval applies to the exact reviewed version. |
| Approved | Activate | Active | Approved plan becomes effective for the defined scope and period. |

Completed or Closed belongs to L3-J-003. Controlled revision of an approved or active plan belongs to L3-J-002.

## 13.2 Planned Requirement Impact

Planned requirements become active planning references with the plan. They do not become approved procurement demand, sourcing authority, financial commitment, or supplier commitment merely because the plan is active.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published Business Events

| Business Event | Publisher | Business Meaning |
|---|---|---|
| ProcurementPlanCreated | Procurement Planning and Sourcing | A controlled Procurement Plan identity and draft version have been established. |
| PlannedRequirementAdded | Procurement Planning and Sourcing | An anticipated requirement has been added to the plan. |
| ProcurementPlanSubmitted | Procurement Planning and Sourcing | A controlled plan version has entered governed review. |
| ProcurementPlanReturnedForRevision | Procurement Planning and Sourcing | The reviewed plan requires correction before approval. |
| ProcurementPlanApproved | Procurement Planning and Sourcing | The exact reviewed plan version has received valid institutional approval. |
| ProcurementPlanActivated | Procurement Planning and Sourcing | The approved plan is active for the defined scope and period. |

Events represent completed business facts and must not imply downstream demand approval, financial commitment, sourcing initiation, supplier selection, or Purchase Order creation.

## 14.2 Consumed Facts and References

| Source | Consumed Fact or Reference | Boundary |
|---|---|---|
| Institution and Organisation architecture | Institution, campus, department, school, faculty, or organisational-unit identity and status | J01 references but does not alter organisational identity. |
| Requesting domain | Anticipated requirement and specialist specification context | The requesting domain retains specialist semantic ownership. |
| J06 where applicable | Existing item or category reference | J01 does not create a competing Item Master. |
| Finance where available | Budget or estimated-cost reference | Finance retains monetary truth; the plan stores planning references only. |
| Enterprise document capability | Controlled document and evidence references | J01 owns the business relationship, not document-storage architecture. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Separation

- Preparation, review, approval, and audit access must follow institution-configured authority for the relevant institution and organisational scope.
- Authorization may consider campus, organisational unit, procurement category, anticipated value band, delegated authority, and plan stage.
- The preparer must not approve the plan where institutional policy requires separation.
- Any permitted role combination must be supported by the applicable institutional policy rather than assumed by this scenario.

## 15.2 Approval Evidence

Approval evidence must identify:

- the exact plan identity and version;
- planning period and organisational scope;
- approving actor and institutional authority;
- approval decision and timestamp;
- material conditions or reservations, if permitted;
- supporting document or evidence references;
- correlation to the submitted review baseline.

## 15.3 Business Audit and Evidence Obligations

The scenario must preserve reconstructable evidence of:

- creation, preparation, submission, review, return, approval, and activation actions;
- actors, timestamps, reasons, authority, and state transitions;
- original and corrected controlled versions;
- planned-requirement origin and consolidation relationships;
- supporting specifications, estimates, documents, and external references;
- approval and activation against the same controlled version.

Historical approval and lifecycle evidence must not be destructively rewritten.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- a unique Procurement Plan exists for the intended planning context;
- its planning period and organisational scope are explicit and valid;
- the effective plan version is identifiable;
- planned requirements are recorded with sufficient traceability and planning context;
- material review issues have been resolved or governed according to policy;
- the exact submitted version has received valid institutional approval;
- approval authority and evidence are preserved;
- the approved version is Active for the defined scope and period;
- planned requirements are available for traceable downstream reference;

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- activation and related business events reflect completed facts;
- no Purchase Requisition, authorised procurement demand, sourcing authority, budget commitment, supplier award, Purchase Order, accounting fact, or payment obligation has been implied merely by plan activation;
- the complete lifecycle and evidence trail is reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. L2-J01 states that institutions not requiring approval may configure a simplified lifecycle but does not define its exact canonical state sequence or the minimum authority required for activation. This remains a non-main-path issue and does not change the successful scenario outcome defined in §3 and §4.2.
2. L2-J01 shows `Returned for Revision` as a lifecycle outcome but does not state whether it is a durable lifecycle state or a review disposition attached to a submitted version. This ambiguity affects the internal representation of the alternate path but does not constitute successful scenario completion.
3. The authoritative relationship between a planning period and an externally owned academic year, financial year, semester, quarter, or institution-defined period requires stable upstream contracts; exact contract identities are deferred.
4. The conditions under which policy may permit sourcing directly from an approved consolidated procurement authority are intentionally excluded from this scenario and require explicit configuration and traceability in the applicable downstream sourcing scenario.

## 17.2 Assumptions

- The standard lifecycle requiring review and approval is used as the main path; the simplified lifecycle remains governed by Open Issue 1.
- References to organisational units, items, specifications, budgets, documents, and actors are authoritative external references where their ownership lies outside J01.
- Activation makes the plan effective as planning context only.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-001**