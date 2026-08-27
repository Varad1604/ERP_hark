# Accreditation Cycle Initiation to Approved Assessment Plan

## 1. Scenario Metadata

**Scenario ID:** L3-H-062  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Evidence-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md; `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

## 2. Scenario Purpose

Establish and approve the institutional Accreditation Assessment Plan for an opened accreditation cycle, including responsibilities, milestones, evidence sources, review points and submission obligations.

## 3. Business Outcome

Responsibilities, milestones, criteria, evidence sources, reviews, and submission obligations are established in an approved plan

## 4. Scenario Boundary and Scope

The scenario begins when an accreditation/compliance cycle is opened and ends when the Assessment Plan reaches `Approved` and is ready for governed activation/execution. Evidence-owning domains retain source ownership.

## 5. Ownership and Participating Domains

- **H09:** owns the Accreditation Assessment Plan and its governance lifecycle.
- **VAR:Evidence-Owning Domain:** retains ownership of authoritative source evidence/business facts.

## 6. Business Actors, Roles and Responsibilities

IQAC / Accreditation Cell coordination remains as defined in H09. The **Accreditation and Compliance Governance Authority** is the canonical institutional business authority for Assessment Plan approval/rejection and related governed decisions.

## 7. Preconditions and Entry Criteria

- The applicable Accreditation Framework/cycle context exists.
- Institutional scope is established.
- Responsibilities, criteria, evidence sources and review obligations can be defined.

## 8. Trigger

An accreditation or compliance cycle is opened for an institutional scope

## 9. Main Business Flow

1. Create the Assessment Plan in `Draft`.
2. Define responsibilities, milestones, criteria, evidence sources, review points and submission obligations.
3. Move the plan to `Under Review`.
4. Accreditation and Compliance Governance Authority reviews the plan.
5. If approved, move to `Approved` and publish `AccreditationAssessmentPlanApproved`.
6. Preserve plan, review, decision and source-evidence references without transferring source ownership.

## 10. Alternate Business Flows

- `Under Review → Revision Required → Draft`, publishing `AccreditationAssessmentPlanRevisionRequired`.
- `Under Review → Rejected`, publishing `AccreditationAssessmentPlanRejected`.
- After approval, later cycle execution may activate the plan and eventually complete/close it under H09; those later outcomes are outside this scenario's frozen end boundary.

## 11. Failure, Interruption and Business Recovery

Incomplete plan/evidence-source governance prevents approval. Revision Required returns the plan to Draft without changing source-domain evidence.

## 12. Business Rules and Decision Constraints

- H09 owns Assessment Plan governance.
- Canonical lifecycle: `Draft → Under Review → Approved → Active → Completed → Closed`.
- Revision Required and Rejected are governed pre-approval branches.
- Source evidence remains owned by `VAR:Evidence-Owning Domain`.

## 13. State and Lifecycle Impact

This scenario moves the Assessment Plan through `Draft → Under Review → Approved`. Later Active/Completed/Closed positions remain governed by the same H09 lifecycle.

## 14. Business Events and Cross-Domain Collaboration

H09 publishes `AccreditationAssessmentPlanApproved`, `AccreditationAssessmentPlanRevisionRequired`, and `AccreditationAssessmentPlanRejected` as applicable. Source evidence is consumed through governed references/contracts only.

## 15. Business Authorization, Approval and Audit Obligations

Accreditation and Compliance Governance Authority owns the plan approval/rejection decision. Evidence preserves responsibilities, milestones, source references, review, decision authority and version/history.

## 16. Completion Criteria and Postconditions

Completion requires an authoritative Approved Assessment Plan containing the frozen required planning content, with evidence-source ownership unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Resolved Historical Blocker

The former missing Assessment Plan approval lifecycle, authority and event were resolved by locked `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

### 17.2 Assumptions

No assumptions. `VAR:Evidence-Owning Domain` remains preserved exactly as frozen.

### 17.3 Deferred Questions

No additional scenario-specific questions.
