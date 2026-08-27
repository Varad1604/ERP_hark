# Compliance Obligation to Effective Requirement Lifecycle

## 1. Scenario Metadata

**Scenario ID:** L3-H-063  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Obligation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Review  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md; `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

## 2. Scenario Purpose

Govern an institutional Compliance Requirement through identification, applicability, active monitoring, satisfaction, waiver, remediation and retirement while preserving source-obligation ownership and complete history.

## 3. Business Outcome

The requirement is classified, assigned, monitored, revised, fulfilled, waived, expired, or retired with full history

Frozen business wording is expressed through the canonical H09 lifecycle without introducing duplicate state names: fulfilled = `Satisfied`; waived = `Waived`; retired = `Retired`. Revision/supersession is governed through successor requirement/version treatment with the predecessor Retired when no longer current. Expiry is an end-of-current-obligation business condition and does not create an additional `Expired` state where H09 uses `Retired` for a no-longer-current requirement version.

## 4. Scenario Boundary and Scope

The scenario begins when a legal, regulatory, accreditation or institutional obligation is identified/changed and ends when the H09 Compliance Requirement reaches its applicable governed position with complete history.

## 5. Ownership and Participating Domains

- **H09:** owns the Compliance Requirement governance record and lifecycle.
- **VAR:Obligation-Owning Domain:** retains ownership of underlying source obligation/operational facts where applicable.

## 6. Business Actors, Roles and Responsibilities

Compliance Cell/IQAC responsibilities remain as defined upstream. Each Requirement has one accountable Business Owner. The **Accreditation and Compliance Governance Authority** owns governed waiver/retirement decisions where H09 requires institutional authority.

## 7. Preconditions and Entry Criteria

- The obligation is identified for an applicable institutional scope.
- Source obligation context/evidence is available.
- An accountable Business Owner can be assigned.

## 8. Trigger

A legal, regulatory, accreditation, or institutional obligation is identified or changed

## 9. Main Business Flow

1. Record the Compliance Requirement in `Identified` and classify its applicability.
2. Move to `Applicable`, assign the accountable Business Owner, review cadence and evidence obligations.
3. Move to `Active` for governed monitoring.
4. Evaluate compliance using authoritative evidence.
5. If fulfilled, move `Active → Satisfied` and publish `ComplianceRequirementSatisfied`.
6. When the satisfied/current obligation is formally completed, move to `Closed` where applicable.
7. If a waiver is requested, move `Applicable / Active → Waiver Requested`; after authorized decision move to `Waived` and publish `ComplianceRequirementWaived`.
8. If noncompliance is identified, move `Active → Noncompliant`; after governed remediation restores compliance, return to `Active`.
9. If the requirement version is withdrawn, superseded or otherwise no longer current, move the applicable current state to `Retired` and publish `ComplianceRequirementRetired`.
10. Preserve full requirement/evidence/decision history.

## 10. Alternate Business Flows

- Waiver denied: the requirement remains governed by its applicable non-waived lifecycle.
- Material revision creates a successor governed requirement/version; predecessor history is retained and may be Retired when superseded.
- Retirement does not erase evidence or imply source-domain facts were changed.

## 11. Failure, Interruption and Business Recovery

Missing evidence prevents satisfaction/waiver decisions. Noncompliance returns to Active only after governed remediation. No source obligation fact is modified by H09 governance.

## 12. Business Rules and Decision Constraints

- Canonical lifecycle begins `Identified → Applicable → Active → Satisfied → Closed`.
- Waiver branch: `Applicable / Active → Waiver Requested → Waived`.
- Remediation branch: `Active → Noncompliant → Active`.
- Current Requirement versions may become `Retired` when formally withdrawn or superseded.
- Waiver requires explicit authority and recorded rationale/evidence.
- `Waived` does not mean an external obligation ceased to exist.

## 13. State and Lifecycle Impact

Only the exact H09 states above are canonical. Frozen `revised`, `fulfilled`, `expired` and `retired` terminology remains business-outcome language and does not create parallel lifecycle states.

## 14. Business Events and Cross-Domain Collaboration

H09 publishes `ComplianceWaiverRequested`, `ComplianceRequirementWaived`, `ComplianceRequirementRetired`, and `ComplianceRequirementSatisfied` as applicable. Source obligation/evidence facts remain with their canonical owners.

## 15. Business Authorization, Approval and Audit Obligations

Accreditation and Compliance Governance Authority owns governed waiver/retirement decisions. The accountable Business Owner remains singular. Evidence preserves classification, applicability, source references, monitoring, waiver/remediation/satisfaction/retirement decisions and complete history.

## 16. Completion Criteria and Postconditions

Completion requires the Compliance Requirement to reach its applicable governed H09 outcome with full history preserved and no ownership transfer to H09 of underlying source facts.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Resolved Historical Blocker

The former missing waiver and retirement semantics were resolved by locked `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26. Frozen revision/expiry terminology is retained as business meaning without inventing additional lifecycle states.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

### 17.2 Assumptions

No assumptions. `VAR:Obligation-Owning Domain` remains preserved exactly as frozen.

### 17.3 Deferred Questions

No additional scenario-specific questions.
