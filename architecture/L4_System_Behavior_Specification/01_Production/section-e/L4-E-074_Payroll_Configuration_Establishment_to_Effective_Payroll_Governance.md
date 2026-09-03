# Payroll Configuration Establishment to Effective Payroll Governance

## 1. Specification Metadata

**Specification ID:** L4-E-074  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-074 — Payroll Configuration Establishment to Effective Payroll Governance  
**Primary Business Owner:** Payroll  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-074; L2-E06 — Payroll Workflows  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Establish and activate a governed Payroll Configuration through its canonical lifecycle so applicable payroll runs can resolve against an approved Active configuration. Compensation and compliance source policies remain owned by their originating domains.

## 3. Required Business Context

- Proposed Payroll Configuration or successor configuration version.
- Applicable source-owned compensation/compliance rules.
- Effective period and version context.
- Governed review and approval conditions.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-074-BH-001 | The system shall establish a proposed Payroll Configuration in `Draft`. | L3-E-074 §§3, 9, 13 |
| L4-E-074-BH-002 | The system shall move the configuration through `Review` only after validating its intended payroll scope, applicable source policies and effective period. | L3-E-074 §§7, 9, 11–13 |
| L4-E-074-BH-003 | The configuration shall enter `Approved` only when the governed approval condition is satisfied. | L3-E-074 §§6, 9, 13, 15 |
| L4-E-074-BH-004 | An approved configuration shall become `Active` only when eligible to become effective for its governed period. | L3-E-074 §§3, 9, 13, 16 |
| L4-E-074-BH-005 | When a successor configuration becomes Active, the prior effective configuration shall progress through the canonical `Superseded` and, when eligible, `Archived` lifecycle positions. | L3-E-074 §§9–10, 13, 16 |
| L4-E-074-BH-006 | The system shall preserve configuration version, effective-period and approval history so payroll usage remains traceable to a specific configuration version. | L3-E-074 §§12, 15–16 |
| L4-E-074-BH-007 | Payroll Run validation shall be able to require and resolve an applicable `Active` Payroll Configuration without this scenario creating a Payroll Run. | L3-E-074 §§3–4, 12–13, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-074-BH-008 | Payroll shall consume authoritative compensation or compliance policy/version facts from their owning domains without acquiring ownership of those source policies. | L3-E-074 §§4.4, 5, 12, 14.2 |
| L4-E-074-BH-009 | The system shall not publish an invented Payroll Configuration lifecycle event because the Locked parent explicitly states that L2-E06 exposes no configuration-specific canonical event name for this scenario. | L3-E-074 §§14.1, 17 |

## 6. Failure / Rejection / Continuation

- A configuration that fails review shall not become Active.
- Missing required approval shall prevent activation.
- Invalid/conflicting effective periods shall be resolved before activation.
- Missing source-owned policy shall prevent dependent configuration activation.
- The existing valid Active configuration remains effective until a valid successor becomes Active.

## 7. Audit & Evidence Behavior

Configuration basis, source-policy references, review/approval evidence, effective period, version lineage and resulting lifecycle position shall remain traceable.

## 8. Observable Completion Conditions

The Payroll Configuration has progressed through the required governed lifecycle to `Active`, its effective version is identifiable, prior versions remain traceable, and payroll processing can resolve the Active configuration without ownership transfer from source policy domains.

## 9. Traceability & Provenance

**Parent:** L3-E-074 — Payroll Configuration Establishment to Effective Payroll Governance  
**Primary verification source:** L3-E-074 §§3, 5, 9, 11–17 and authoritative L2-E06 architecture.

## 10. Open Issues / Deferred Design

The parent explicitly records no canonical Payroll Configuration event name and less-specific actor mapping than Payroll Run approval. L4 preserves those upstream limits and introduces neither a new event nor a new role. Technical configuration storage/deployment remains L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
