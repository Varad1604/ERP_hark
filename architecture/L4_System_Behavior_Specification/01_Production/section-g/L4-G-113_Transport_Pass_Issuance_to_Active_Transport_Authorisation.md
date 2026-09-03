# Transport Pass Issuance to Active Transport Authorisation

## 1. Specification Metadata

**Specification ID:** L4-G-113  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations  
**Realizes Scenario:** L3-G-113 — Transport Pass Issuance to Active Transport Authorisation  
**Primary Business Owner:** Transport Management  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-G-113; applicable locked G architecture sources  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall govern an eligible transport pass from issuance through Active transport authorisation without transferring holder identity or financial ownership.

## 3. Required Business Context

- An eligible/approved transport allocation exists.
- Authoritative holder identity/status is available from its canonical owner.
- Transport Management owns the Transport Pass lifecycle.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-G-113-BH-001 | Identify the eligible transport allocation, holder and applicable transport context. | L3-G-113 §§3, 7, 9 |
| L4-G-113-BH-002 | Evaluate pass-issuance and required approval conditions against authoritative policy. | L3-G-113 §§9, 12, 15 |
| L4-G-113-BH-003 | Issue the Transport Pass when all authoritative conditions are satisfied. | L3-G-113 §§9, 16 |
| L4-G-113-BH-004 | Establish the pass as Active for its applicable authorised validity period. | L3-G-113 §§9, 13, 16 |
| L4-G-113-BH-005 | Preserve issuance, authority, validity and lifecycle history. | L3-G-113 §§15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-G-113-BH-006 | The exact canonical event identifier for Transport Pass issuance/activation is unresolved upstream; this L4 specification shall not invent one. The active G04-owned transport authorisation shall remain available to approved consumers while C/E/F/H retain their canonical ownership. | L3-G-113 §§14, 17 |

## 6. Failure / Rejection / Continuation

- Ineligible or incomplete requests shall not become Active.
- Missing authoritative holder/allocation context shall keep issuance unresolved until the source-owned condition is available.
- A conflicting active pass shall follow the applicable upstream pass change/replacement governance.

## 7. Audit & Evidence Behavior

Preserve attributable evidence of holder/allocation context, eligibility, approval, issuance, validity and activation.

## 8. Observable Completion Conditions

The pass is issued, reaches Active authorisation for the applicable validity period, and its issuance history is traceable.

## 9. Traceability & Provenance

**Parent:** L3-G-113 — Transport Pass Issuance to Active Transport Authorisation  
**Primary verification source:** L3-G-113 §§3, 9–17; authoritative L2-G04 Transport Management workflow architecture.

## 10. Open Issues / Deferred Design

Exact Transport Pass issuance/activation event vocabulary remains unresolved upstream and shall not be invented at L4. Technical implementation remains deferred to L5/L6.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
