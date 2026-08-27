# Intellectual Property Disclosure to Protection or Closure Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-035  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Research  
**Participating Domains:** E, F, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Architecture Approved  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-27  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H05 — Research Workflows; L2-E03 — Faculty Workforce Workflows; locked Finance architecture; `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md`  
**Referenced By:** L3 targeted revalidation and formal Architecture Approval review

# 2. Scenario Purpose

Govern Research IP disclosure and institutional protection-workflow metadata through review, protection, abandonment, commercialization or closure while preserving external legal authority and Finance ownership of monetary facts.

# 3. Business Outcome

Ownership review, protection decision, filing, commercialization, abandonment, or closure is completed under policy

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A potentially protectable research result is disclosed.

## 4.2 End Boundary

The scenario ends when the H05 Research IP record reaches a governed protection, abandonment, commercialization or closure outcome with immutable evidence preserved.

## 4.3 In Scope

IP disclosure, institutional review, protection approval/progress, protected status evidence, abandonment, commercialization metadata and closure.

## 4.4 Out of Scope

Creation of external legal rights, external statutory authority, or Finance-owned monetary/accounting facts.

# 5. Ownership and Participating Domains

E03 retains inventor/faculty facts. Finance owns accounting/payment/revenue and other monetary consequences. Research owns Research IP disclosure and institutional protection-workflow metadata. External legal/statutory status remains evidenced by the applicable authority.

# 6. Business Actors, Roles and Responsibilities

Principal Investigator / Inventor initiates disclosure. Research Director is the canonical business authority for strategic protection, rejection, abandonment, commercialization and closure decisions where institutional approval is required. Research Administrator records governed decisions and evidence without replacing decision authority.

# 7. Preconditions and Entry Criteria

- A potentially protectable Research result exists.
- Inventors and project references are valid where applicable.
- Required disclosure and ownership-policy evidence is available.
- External statutory evidence is consumed only as authoritative evidence when applicable.

# 8. Trigger

A potentially protectable research result is disclosed

# 9. Main Business Flow

1. Record the IP disclosure and establish the H05 Research IP record in `Disclosed`.
2. Move to `Under Review` and evaluate the applicable institutional ownership/protection policy.
3. If protection is approved, move to `Protection Approved` and publish `ResearchIPProtectionApproved`.
4. Progress through `Protection In Progress` while preserving filing/protection evidence.
5. When authoritative protection evidence exists, move to `Protected` and publish `ResearchIPProtectionRecorded`.
6. If the governed decision is not to proceed, move the eligible record to `Abandoned` and publish `ResearchIPAbandoned`.
7. Where commercialization applies, move `Protected → Commercialization Active`, publish `ResearchIPCommercializationActivated`, and later move to `Commercialization Closed`.
8. Where no commercialization lifecycle is applicable, move `Protected → Closed`.
9. Publish `ResearchIPClosed` for the governed closure outcome and preserve immutable history.

# 10. Alternate Business Flows

- `Under Review → Rejected` where the protection request is not approved.
- `Under Review / Protection Approved / Protection In Progress → Abandoned` where the governed process is discontinued.
- `Protected → Closed` where commercialization is not applicable.

# 11. Failure, Interruption and Business Recovery

Invalid inventor/ownership context blocks progression. External legal status is never invented by H05. Abandonment/closure preserves prior evidence and does not imply alteration of Finance facts or external statutory records.

# 12. Business Rules and Decision Constraints

- Research owns disclosure/protection-workflow metadata only.
- H05 does not create external legal rights by itself.
- `Protected` means authoritative evidence of the applicable protection outcome has been recorded.
- Finance remains sole owner of monetary/accounting consequences.
- Prior IP history remains immutable through abandonment, commercialization and closure.

# 13. State and Lifecycle Impact

Canonical Research IP lifecycle:

`Disclosed → Under Review → Protection Approved → Protection In Progress → Protected`

Alternative outcomes:

- `Under Review → Rejected`
- `Under Review / Protection Approved / Protection In Progress → Abandoned`
- `Protected → Commercialization Active → Commercialization Closed`
- `Protected → Closed`

# 14. Business Events and Cross-Domain Collaboration

Research publishes:

- `ResearchIPDisclosed`
- `ResearchIPProtectionApproved`
- `ResearchIPProtectionRecorded`
- `ResearchIPAbandoned`
- `ResearchIPCommercializationActivated`
- `ResearchIPClosed`

Finance and external legal/statutory facts remain externally owned.

# 15. Business Authorization, Approval and Audit Obligations

Research Director owns applicable strategic Research IP decisions. Evidence preserves disclosure, inventors, ownership-policy evaluation, protection decision, external protection evidence, abandonment/commercialization/closure decisions, decision authority and immutable history.

# 16. Completion Criteria and Postconditions

Completion occurs when the Research IP record reaches its governed protection, abandonment, commercialization or closure outcome with all required evidence preserved. No external legal or Finance-owned fact is re-owned by Research.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Resolved Historical Blocker

The former missing disclosure, protection-decision, abandonment, commercialization and closure semantics were resolved by locked `L2-H05_Research_Governance_and_Lifecycle_Amendment_2026-08-26.md` and passed targeted revalidation on 2026-08-26.

**Current architecture assessment:** `READY FOR ARCHITECTURE APPROVAL REVIEW`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific questions.
