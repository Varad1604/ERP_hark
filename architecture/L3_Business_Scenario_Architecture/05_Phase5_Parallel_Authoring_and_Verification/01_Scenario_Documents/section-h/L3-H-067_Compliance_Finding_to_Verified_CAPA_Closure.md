# Compliance Finding to Verified CAPA Closure

## 1. Scenario Metadata

**Scenario ID:** L3-H-067  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Accreditation & Compliance  
**Participating Domains:** H, VAR:Remediation-Owning Domain  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-H09_Accreditation_and_Compliance_Workflows.md; `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

## 2. Scenario Purpose

Govern a compliance finding and CAPA through root-cause analysis, source-domain remediation evidence, verification, closure and governed reopening where effectiveness does not hold.

## 3. Business Outcome

Root cause, corrective and preventive actions, evidence, effectiveness review, reopening where needed, and final closure are governed end to end

## 4. Scenario Boundary and Scope

The scenario begins when a governed nonconformity/observation supports a Finding and ends when the CAPA is finally Closed after verification, including any governed reopen-and-rework cycle required by later evidence.

## 5. Ownership and Participating Domains

- **H09:** owns Finding/CAPA governance and lifecycle.
- **VAR:Remediation-Owning Domain:** owns correction of its operational business state and returns authoritative execution evidence.

## 6. Business Actors, Roles and Responsibilities

Compliance Cell/IQAC and Action Owner responsibilities remain as defined upstream. The **Accreditation and Compliance Governance Authority** is the canonical institutional business authority for CAPA closure/reopening decisions where H09 requires institutional authority.

## 7. Preconditions and Entry Criteria

- A governed observation/nonconformity exists.
- CAPA is required/appropriate under H09.
- Remediation ownership and evidence obligations can be assigned.

## 8. Trigger

An audit, assessment, review, or monitoring activity records a nonconformity or observation

## 9. Main Business Flow

1. Govern the Finding and establish the CAPA.
2. Move CAPA through `Opened → Action Planned → In Progress` while the accountable source domain executes operational remediation.
3. Receive authoritative remediation evidence and move to `Verification Pending`.
4. Verify effectiveness; when successful move to `Verified` and publish `CAPAVerified`.
5. Authorize closure, move to `Closed`, and publish `CAPAClosed`.
6. Preserve complete root-cause, action, evidence, verification and closure history.

## 10. Alternate Business Flows

- If post-closure evidence demonstrates the corrective outcome did not hold or the finding remains unresolved, the Accreditation and Compliance Governance Authority may authorize `Closed → Reopened`, publishing `CAPAReopened`.
- Move `Reopened → In Progress`, execute additional remediation in the source-owning domain, and repeat verification/closure.

## 11. Failure, Interruption and Business Recovery

CAPA cannot close before effectiveness verification. Reopening preserves the original closure evidence and records the new reason/decision; no history is erased.

## 12. Business Rules and Decision Constraints

- Canonical CAPA lifecycle: `Opened → Action Planned → In Progress → Verification Pending → Verified → Closed`.
- Reopening branch: `Closed → Reopened → In Progress`.
- Operational remediation remains owned by `VAR:Remediation-Owning Domain`.
- H09 owns governance/evidence/verification decisions, not the source operational state.

## 13. State and Lifecycle Impact

The CAPA may cycle through Reopened/In Progress after a prior closure when authoritative post-closure evidence requires it. Final closure requires renewed effectiveness verification.

## 14. Business Events and Cross-Domain Collaboration

H09 publishes `CAPAVerified`, `CAPAClosed`, and `CAPAReopened` along with the existing Finding/CAPA events. Source-domain remediation evidence returns through the governed CAPA execution hand-off.

## 15. Business Authorization, Approval and Audit Obligations

Accreditation and Compliance Governance Authority owns governed reopening/closure decisions. Evidence preserves root cause, action plan, source execution evidence, verification, original closure, reopening reason/authority, later remediation and final closure.

## 16. Completion Criteria and Postconditions

Completion requires verified effectiveness and final Closed CAPA; if reopening was needed, the reopened cycle must complete without erasing prior closure history. Source operational ownership remains unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Resolved Historical Blocker

The former missing CAPA reopening state/transition/event was resolved by locked `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

### 17.2 Assumptions

No assumptions. `VAR:Remediation-Owning Domain` remains preserved exactly as frozen.

### 17.3 Deferred Questions

No additional scenario-specific questions.
