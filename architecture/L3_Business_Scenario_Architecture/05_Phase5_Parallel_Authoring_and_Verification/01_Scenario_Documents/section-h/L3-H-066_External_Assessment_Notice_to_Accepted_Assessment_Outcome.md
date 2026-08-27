# External Assessment Notice to Accepted Assessment Outcome

## 1. Scenario Metadata

**Scenario ID:** L3-H-066  
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

Coordinate external accreditation/compliance assessment evidence and govern the institution's acceptance or challenge of the resulting external assessment outcome while preserving source-evidence ownership.

## 3. Business Outcome

Evidence exchange, assessor interaction, observations, and institutional response produce an accepted assessment outcome or governed challenge

## 4. Scenario Boundary and Scope

The scenario begins when an external assessment is scheduled and ends when the H09 response to the authoritative external outcome reaches Accepted or completes the governed challenge/resolution path.

## 5. Ownership and Participating Domains

- **H09:** owns institutional response governance, observations/findings and H09 external-assessment response lifecycle.
- **VAR:Evidence-Owning Domain:** retains ownership of authoritative operational evidence/business facts.
- The external assessment outcome itself is authoritative external evidence referenced by H09.

## 6. Business Actors, Roles and Responsibilities

IQAC / Accreditation Cell coordination remains as defined upstream. The **Accreditation and Compliance Governance Authority** is the canonical institutional business authority for accepting or challenging the external assessment outcome.

## 7. Preconditions and Entry Criteria

- External assessment schedule/context exists.
- Applicable Accreditation Cycle/evidence context exists.
- Source evidence can be supplied by reference without ownership transfer.

## 8. Trigger

An accreditation or compliance body schedules an external assessment

## 9. Main Business Flow

1. Receive the external assessment schedule and coordinate readiness.
2. Prepare governed evidence references from source-owning domains.
3. Coordinate assessor interaction and record H09 observations/findings.
4. Record the authoritative external assessment outcome in `Received`.
5. Move to `Under Review` for institutional response governance.
6. If accepted, move to `Accepted` and publish `ExternalAssessmentOutcomeAccepted`.
7. Preserve all external evidence, institutional review and decision history.

## 10. Alternate Business Flows

- `Under Review → Challenged`, publishing `ExternalAssessmentOutcomeChallenged`.
- After external/governed resolution, move `Challenged → Resolved` and publish `ExternalAssessmentChallengeResolved`.
- From Resolved, move to `Accepted` or `Replaced` according to the authoritative later outcome.
- If Replaced, the prior external outcome remains historical and a later authoritative outcome becomes current for use.

## 11. Failure, Interruption and Business Recovery

Missing evidence or unresolved challenge prevents final acceptance. H09 does not alter the external assessment evidence or source-domain facts to resolve a discrepancy.

## 12. Business Rules and Decision Constraints

- External assessment evidence remains authoritative by reference.
- H09 governs the institutional response, not the external authority's record.
- Canonical response lifecycle is `Received → Under Review → Accepted`, with `Under Review → Challenged → Resolved → Accepted / Replaced`.
- Source evidence remains source-domain owned.

## 13. State and Lifecycle Impact

Only the H09 institutional response lifecycle changes. Source operational lifecycles and external authority records remain unchanged.

## 14. Business Events and Cross-Domain Collaboration

H09 publishes `ExternalAssessmentOutcomeAccepted`, `ExternalAssessmentOutcomeChallenged`, `ExternalAssessmentChallengeResolved`, and `ExternalAssessmentOutcomeReplaced`. Source evidence is consumed by reference only.

## 15. Business Authorization, Approval and Audit Obligations

Accreditation and Compliance Governance Authority owns acceptance/challenge decisions. Evidence preserves assessment schedule, source references, observations/findings, external outcome, institutional decision, challenge/resolution and immutable history.

## 16. Completion Criteria and Postconditions

Completion requires an authoritative Accepted outcome or a completed governed challenge/resolution disposition, with source evidence ownership and external evidence history preserved.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Resolved Historical Blocker

The former missing external-assessment acceptance/challenge lifecycle, authority and events were resolved by locked `L2-H09_Accreditation_Compliance_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

### 17.2 Assumptions

No assumptions. `VAR:Evidence-Owning Domain` remains preserved exactly as frozen.

### 17.3 Deferred Questions

No additional scenario-specific questions.
