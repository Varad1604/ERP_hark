# Active Procurement Plan to Governed Closure

# 1. Scenario Metadata

**Scenario ID:** L3-J-003  
**Architecture Level:** L3 - Business Scenario Architecture  
**Primary Owner:** Procurement Planning & Sourcing  
**Participating Domains:** J  
**Scenario Type:** Simple  
**Priority:** P2  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Chetan  
**Last Updated:** 2026-08-20  
**Depends On:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `L2-J00_Enterprise_Resource_Management_Architecture_and_Governance.md`; `L2-J01_Procurement_Planning_and_Sourcing_Workflows.md`; `Section_J_Canonical_Register.md`; `L3_Final_Canonical_Scenario_ID_Register.md`; `L3_Business_Scenario_Template_v2.0.md`  
**Referenced By:** L3-J-001 Procurement Planning Requirement to Approved Active Procurement Plan; L3-J-002 Approved Procurement Plan Change to Governed Successor Plan; later reporting, audit, and procurement-planning scenarios consuming closed-plan history  

---
# 2. Scenario Purpose

This scenario governs the formal closure of an active Procurement Plan when its planning period reaches the institutionally defined completion point. It establishes a final planning record while preserving every plan version, planned requirement, revision, disposition, and planned-versus-actual reference needed for institutional accountability.

Closure ends the plan's active planning lifecycle. It does not close, cancel, approve, or alter downstream requisitions, sourcing events, Purchase Orders, receipts, inventory, assets, financial commitments, payables, or payments.

---

# 3. Business Outcome

The Procurement Plan is formally closed for its defined scope and period, with planned requirements, revisions, dispositions, approval evidence, and planned-versus-actual traceability preserved as governed historical evidence.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an active Procurement Plan reaches its institutionally defined completion point or another authorised closure condition applies.

## 4.2 End Boundary

The scenario ends when closure readiness has been reviewed, unresolved planned requirements have explicit dispositions, authorised closure is recorded, the plan reaches Completed or Closed status, and its history becomes protected from ordinary planning change.

## 4.3 In Scope

- identifying the applicable completion point and closure authority;
- confirming the exact effective plan version and full version chain;
- reviewing every planned requirement and its final planning disposition;
- preserving planned-versus-actual references where authoritative downstream facts are available;
- documenting carried-forward, deferred, cancelled, uninitiated, completed, partially fulfilled, or otherwise unresolved planning positions;
- resolving or explicitly accepting closure exceptions under authority;
- approving and recording formal closure;
- preserving immutable closure and historical evidence.

## 4.4 Out of Scope

- creating the initial plan, governed by L3-J-001;
- revising the active plan through a successor, governed by L3-J-002;
- creating or changing Purchase Requisitions or authorised procurement demand, owned by J03;
- closing or cancelling sourcing events, owned by J01 sourcing scenarios;
- closing or amending Purchase Orders, owned by J04;
- goods receipt, inspection, acceptance, inventory, asset, disposal, or financial closure;
- calculating authoritative expenditure, commitments, payables, or payments;
- deleting plan history or rewriting planned requirements to match actual outcomes;
- technical archival, retention implementation, storage, or reporting design.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by | L3-J-001 Procurement Planning Requirement to Approved Active Procurement Plan | Establishes the plan that later reaches closure. |
| Related | L3-J-002 Approved Procurement Plan Change to Governed Successor Plan | Governs controlled revision before closure where planning content must still change. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Enterprise Resource Management - Procurement Planning and Sourcing | Owns closure review, planned-requirement disposition, plan lifecycle transition, preservation of version history, and final closed-plan outcome. |

Downstream domains retain ownership of requisition, sourcing, order, receipt, inventory, asset, and financial facts referenced for planned-versus-actual traceability. J01 records references and planning interpretations without changing those facts.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Procurement Planner | Prepares closure review, reconciles plan versions and requirement dispositions, and assembles supporting evidence. | Prepares and submits closure. |
| Requesting Unit Representative | Confirms unresolved, deferred, cancelled, or carried-forward requirements where institutional policy requires. | Contributes planning disposition evidence. |
| Procurement Manager | Reviews closure completeness, exceptions, traceability, and consistency with policy. | Reviews and may return closure preparation. |
| Procurement Approver | Exercises institutionally configured authority to approve formal plan closure or accepted exceptions. | Approves closure. |
| Auditor / Reviewer | Examines the closed plan, version chain, dispositions, planned-versus-actual references, and closure evidence. | Reviews protected evidence. |

Role combinations and separation requirements follow institutional policy.

---

# 7. Preconditions and Entry Criteria

- A unique Procurement Plan exists in Active status for a defined organisational scope and planning period.
- The institutionally defined completion point or another valid closure condition has been reached.
- The effective plan version and all prior approved versions are available.
- Planned requirements and successor relationships are traceable across the complete version chain.
- Applicable closure policy, authority, evidence, exception, and retention requirements are known.
- Relevant downstream references can be obtained without J01 assuming ownership of downstream records.
- Any plan revision still required to correct current planning content has either completed under L3-J-002 or is explicitly governed before closure.

---

# 8. Trigger

The Procurement Plan's defined planning period reaches its institutionally governed completion point, initiating formal closure review.

---

# 9. Main Business Flow

1. The Procurement Planner identifies the active plan, its scope, planning period, effective version, and applicable closure condition.
2. The planner confirms the complete predecessor-successor version chain and preserves all approved versions as closure evidence.
3. The planner assembles the active plan's planned requirements and reconciles them to their latest effective planning form.
4. Each planned requirement is assigned a final planning disposition supported by available evidence, such as completed, initiated, partially fulfilled, deferred, carried forward, cancelled, no longer required, not initiated, or otherwise unresolved.
5. Where authoritative downstream records exist, the planner records traceable references to the applicable requisition, sourcing, order, receipt, inventory, asset, or financial facts without copying or redefining their ownership.
6. Planned-versus-actual analysis is prepared using the plan's estimates and authoritative downstream references. Differences remain truthful and are not removed by rewriting planned values.
7. Requirements proposed for carry-forward are linked to the relevant successor planning context where one exists. Closure of the current plan does not itself create or approve the successor requirement.
8. Open or exceptional positions are identified with reason, owner, evidence, and proposed treatment.
9. The planner verifies that no pending plan revision is being bypassed through closure and that every planned requirement has a governed closure disposition.
10. The closure package is submitted as a controlled review baseline.
11. The Procurement Manager reviews lifecycle completeness, version integrity, requirement dispositions, downstream traceability, exception treatment, and evidence sufficiency.
12. Correctable deficiencies are returned without altering the active plan or implying closure.
13. When closure readiness is established, the Procurement Approver reviews the exact closure package under applicable authority.
14. The approver records the closure decision, accepted exceptions or conditions, authority, timestamp, and supporting evidence.
15. The plan transitions from Active to Completed or Closed according to the canonical L2 lifecycle terminology confirmed for implementation.
16. Ordinary planning change is prohibited after closure; later material correction must preserve the closed record and proceed through governed supplemental treatment if authorised upstream.
17. The closed plan, versions, requirement dispositions, planned-versus-actual references, approvals, and closure evidence remain available for audit and reporting.

---

# 10. Alternate Business Flows

## 10.1 Return for Closure Completion

1. The reviewer or approver identifies missing dispositions, incomplete version history, unsupported planned-versus-actual references, ungoverned exceptions, or insufficient evidence.
2. The closure package is returned with specific reasons.
3. The plan remains Active and the returned package remains traceable.
4. The planner corrects the package and resubmits it through review and approval.

## 10.2 Closure with Carried-Forward Requirements

Requirements not completed in the closing period may be marked for carry-forward where policy permits. The current plan records the closure disposition and reference; the future plan separately governs inclusion, approval, and activation.

## 10.3 Closure with Open Downstream Activity

The plan may close while a requisition, sourcing event, Purchase Order, receipt, payable, or other downstream process remains open, if institutional policy permits. The open downstream reference and its independent ownership must remain explicit.

## 10.4 Closure with Accepted Exceptions

An authorised approver may accept identified residual planning exceptions where policy allows. Each exception requires reason, authority, evidence, impact, and any follow-up obligation.

## 10.5 Early Closure

If an authorised institutional decision ends the planning period early, the same requirement-by-requirement disposition, review, approval, traceability, and evidence controls apply. Early closure must not conceal incomplete planning activity.

## 10.6 Simplified Closure

An institution using the configured simplified planning lifecycle may use a reduced review path, but closure identity, authority, final dispositions, version preservation, and audit evidence remain mandatory.

---

# 11. Failure, Interruption and Business Recovery

## 11.1 Incomplete Version Chain

If any approved predecessor, successor relationship, effective date, or revision evidence is missing, closure is withheld until the chain is restored or the gap is formally governed.

## 11.2 Undispositioned Planned Requirement

If a planned requirement lacks a final planning disposition, closure is withheld unless an authorised exception explicitly preserves the unresolved position and follow-up responsibility.

## 11.3 Conflicting Downstream Evidence

If downstream references conflict, J01 records the discrepancy and seeks clarification from the owning domain. J01 must not choose or alter another domain's authoritative fact merely to complete closure.

## 11.4 Invalid Closure Authority

If the deciding actor lacks applicable authority, the plan remains Active and the closure proposal is escalated for valid review and approval.

## 11.5 Review or Approval Interruption

An interrupted closure review preserves the active status and submitted closure baseline. No closure may be inferred until the final authorised decision is recorded.

## 11.6 Closure Attempt During Pending Revision

If a controlled successor is still under review, governance must determine whether to complete, withdraw, reject, or otherwise disposition that revision before closure. Closure must not silently abandon it.

## 11.7 Material Error Found After Closure

The closed plan and its evidence are not rewritten. Any permitted correction must use a governed supplemental record or separately authorised process that preserves the original closure history. The canonical supplemental correction scenario is not defined in the current final J register and is recorded as an open issue.

---

# 12. Business Rules and Decision Constraints

- Only an Active Procurement Plan may enter the normal closure path.
- Closure must apply to a uniquely identified effective plan version and preserve the complete version chain.
- Closure must not remove historical planned requirements.
- Every planned requirement must retain a final planning disposition or an explicitly authorised unresolved exception.
- Planned-versus-actual traceability must preserve original estimates and reference authoritative downstream facts.
- Planned values must not be rewritten to equal actual outcomes.
- Procurement Plan closure does not close or alter downstream requisitions, sourcing events, Purchase Orders, receipts, inventory, assets, financial commitments, payables, or payments.
- Open downstream activity does not automatically block plan closure unless institutional policy says it does.
- Carry-forward is a disposition and reference, not automatic creation or approval of a successor-plan requirement.
- Approved and historical plan versions must remain protected from destructive change.
- Closure authority and separation follow institution-configured policy.
- A returned or unapproved closure package must not change the plan's Active status.
- Closure exceptions must be explicit, attributable, evidenced, and auditable.
- Historical evidence remains available according to applicable retention and audit policy.

---

# 13. State and Lifecycle Impact

## 13.1 Procurement Plan Lifecycle

| From State | Business Action | To State | Required Result |
|---|---|---|---|
| Active | Initiate closure review | Active | Controlled closure package is prepared; plan remains operationally active. |
| Active | Return closure package | Active | Deficiencies are recorded and corrected without implying closure. |
| Active | Approve completed planning lifecycle | Completed / Closed | Plan is no longer active; all versions and planned requirements remain historical evidence. |

L2-J01 displays `Completed / Closed` without selecting one canonical state name or defining whether these are alternatives or distinct sequential states. This scenario preserves the upstream terminology and records the ambiguity in Section 17.

## 13.2 Planned Requirement Lifecycle Impact

Each requirement receives a closure disposition. The disposition describes its position within the closed planning context and does not change the lifecycle of any downstream record.

## 13.3 Evidence Impact

The effective version, predecessor versions, closure package, disposition evidence, approval, exceptions, and traceability references become protected historical evidence. Closure does not imply physical deletion or technical archival.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published Business Events

| Business Event | Publisher | Business Meaning |
|---|---|---|
| ProcurementPlanClosed | Procurement Planning and Sourcing | The plan has completed its governed planning lifecycle and its history is preserved. |

No closure event may imply that linked requisitions, sourcing events, Purchase Orders, receipts, inventory, assets, or financial records are themselves closed.

## 14.2 Consumed Facts and References

| Source | Consumed Fact or Reference | Boundary |
|---|---|---|
| J03 Purchase Requisition and Approval | Requisition or authorised-demand status reference | J03 retains ownership and determines its own lifecycle. |
| J01 sourcing records | Sourcing-event and outcome reference | Closure records planning traceability but does not close sourcing. |
| J04 Purchase Order Management | Purchase Order status reference | J04 retains commitment and order lifecycle ownership. |
| J05, J06, and J08 where applicable | Receipt, inventory, or asset reference | J01 records references only. |
| Finance where applicable | Authoritative commitment, expenditure, payable, or payment reference | Finance retains monetary truth. |
| Requesting unit or specialist domain | Requirement outcome or continuing need evidence | Specialist meaning remains externally owned. |
| Enterprise document and audit capabilities | Controlled documents and audit evidence | J01 owns the procurement relationship, not storage infrastructure. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Approval

- Closure preparation, review, approval, exception acceptance, and audit access must follow institution-configured authority.
- The approver must be authorised for the plan's institution, scope, period, category context, and any material residual exception.
- Preparation and approval must be separated where policy requires.
- The exact closure package reviewed and approved must be the package attached to the lifecycle transition.

## 15.2 Required Closure Evidence

- plan identity, scope, period, and final effective version;
- complete version and revision chain;
- final disposition of every planned requirement;
- planned-versus-actual references and identified variances;
- carried-forward, deferred, cancelled, open, or exceptional positions;
- reviewer and approver identities, authority, decisions, and timestamps;
- accepted exceptions, conditions, and follow-up obligations;
- supporting document and correlation references.

## 15.3 Audit Obligations

The closed record must support reconstruction of:

- what the institution originally planned and every governed revision;
- which version was effective during each relevant period;
- how each planned requirement concluded within the plan;
- which authoritative downstream records were referenced;
- why planned and actual positions differed;
- who prepared, reviewed, returned, approved, and closed the plan;
- which exceptions were accepted and by what authority;
- that no historical version or requirement was destructively rewritten.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The scenario is complete only when all of the following criteria are satisfied:

- the active plan and exact final effective version are identified;
- the complete approved version chain remains preserved;
- every planned requirement has a governed final disposition or authorised unresolved exception;
- planned-versus-actual references are traceable where available;
- original estimates and historical versions remain unchanged;
- carried-forward and continuing requirements have explicit references without automatic successor approval;
- material closure exceptions are resolved or accepted under valid authority;
- the exact closure package has completed required review and approval;
- the plan has reached the governed Completed or Closed lifecycle outcome;
- `ProcurementPlanClosed` represents the completed planning fact without implying downstream closure;
- ordinary planning change against the closed plan is prohibited.

## 16.2 Postconditions

Upon successful scenario completion, the following institutional postconditions hold:

- closure evidence is attributable, reconstructable, and retained;

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

1. L2-J01 uses `Completed / Closed` in the Procurement Plan lifecycle but does not define whether these are two distinct states, sequential states, or alternative institution-configured labels. A canonical state decision is required before L4 design and final locking.
2. L2-J01 does not define the complete controlled vocabulary for planned-requirement closure disposition. The draft uses descriptive business outcomes without declaring a locked enumeration.
3. The minimum planned-versus-actual data set and authoritative matching rules across J03, J04, J05, J06, J08, and Finance are not defined upstream.
4. Upstream architecture does not state whether every open downstream process must be resolved before plan closure or whether institutions may configure closure with open references.
5. No final canonical J scenario expressly governs a material error discovered after Procurement Plan closure. A governed supplemental-correction boundary requires architecture confirmation.
6. Retention duration and the distinction between business closure and technical archival remain governed outside this scenario.

## 17.2 Assumptions

- The planning period's institutionally defined completion point is available from approved configuration or governance.
- Closed-plan evidence remains readable for authorised audit and reporting.
- Downstream status references are snapshots or governed references to authoritative facts and do not transfer ownership to J01.
- Closure ends active planning use but does not automatically delete, archive, or dispose of records.

## 17.3 Deferred Questions

No scenario-specific deferred questions.

---

---

**End of Scenario L3-J-003**
