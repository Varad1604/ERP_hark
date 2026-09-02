# Attempt Policy Change to Authoritative Candidate Eligibility

## 1. Specification Metadata

**Specification ID:** L4-I-143  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-143 — Attempt Policy Change to Authoritative Candidate Eligibility  
**Primary Business Owner:** Supplementary, Backlog & Improvement Examination  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-I-143; applicable locked I architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Govern re-evaluation of candidate examination eligibility when an authoritative attempt policy change affects the permitted attempt position. Out of scope: defining examination policy, changing Student Lifecycle ownership, altering historical attempts, or prescribing technical implementation.

## 3. Required Business Context

- Candidate identity and examination context are authoritative.
- Existing attempt history is available.
- The policy change is authorised and applicable.
- Student Lifecycle remains owner of student-status consequences.
- Examination owns examination eligibility within its scope.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-143-BH-001 | The system shall identify candidates and examination contexts affected by the policy change. | L3-I-143 §§3, 9, 12 |
| L4-I-143-BH-002 | The system shall establish the authoritative policy version and effective applicability. | L3-I-143 §§3, 9, 12 |
| L4-I-143-BH-003 | The system shall re-evaluate existing attempt history against the changed policy. | L3-I-143 §§3, 9, 12 |
| L4-I-143-BH-004 | The system shall determine the resulting candidate examination-eligibility position. | L3-I-143 §§3, 9, 12 |
| L4-I-143-BH-005 | The system shall preserve the prior eligibility basis and the policy-change rationale. | L3-I-143 §§3, 9, 12 |
| L4-I-143-BH-006 | The system shall prevent historical attempts from being rewritten merely because eligibility changes. | L3-I-143 §§3, 9, 12 |
| L4-I-143-BH-007 | The system shall hand any resulting student-status consequence to Student Lifecycle through its canonical ownership. | L3-I-143 §§3, 9, 12 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-143-BH-008 | On successful completion of the governed business outcome, the system shall publish the canonical business event or make the authoritative outcome available under Supplementary, Backlog & Improvement Examination ownership. | L3-I-143 §14 |

## 6. Failure / Rejection / Continuation

- If required business eligibility or prerequisite conditions are not satisfied, prevent progression and maintain the prior valid state.
- If conflicting or unverified source facts are received, retain the business position as unresolved without inferring missing values.
- Technical retry, database rollback, queue recovery, and infrastructure mechanisms are excluded.

## 7. Audit & Evidence Behavior

The system shall preserve attributable audit evidence of the initiation context, eligibility evaluation, governing policy basis, authorized decisions, resulting lifecycle state transitions, and completed business outcomes.

## 8. Observable Completion Conditions

Completion requires governing conditions validated, the authoritative business position recorded under Supplementary, Backlog & Improvement Examination ownership, attributable audit evidence preserved, and canonical business events published.

## 9. Traceability & Provenance

**Parent:** L3-I-143 — Attempt Policy Change to Authoritative Candidate Eligibility  
**Primary verification source:** Authoritative Examination L2 workflow architecture.

## 10. Open Issues / Deferred Design

Institution-specific approval roles and configurable policies remain upstream/configuration governed. Technical APIs, database persistence schemas, UI screens, network transport, and retry mechanics are deferred to L5 Solution Architecture.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
