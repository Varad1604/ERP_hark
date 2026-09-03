# Position Assignment Administrative Correction to Corrected Authoritative Occupancy

## 1. Specification Metadata

**Specification ID:** L4-E-071  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** People & Workforce  
**Realizes Scenario:** L3-E-071 — Position Assignment Administrative Correction to Corrected Authoritative Occupancy  
**Primary Business Owner:** Position & Assignment  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-02  
**Depends On:** L3-E-071; L2-E02 — Position & Assignment; L2-E01 — Workforce & Engagement; Organisation Position Definition context  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

Correct an administrative error in an existing Position Assignment so the authoritative occupancy is accurate while preserving effective dating, version lineage and historical traceability. Position Definition and Workforce Engagement remain separately owned and are not modified by this behavior.

## 3. Required Business Context

- Existing canonical Position Assignment.
- Valid Workforce Engagement reference.
- Valid Position Definition and organisational placement context.
- Identified administrative discrepancy and correction evidence.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-E-071-BH-001 | The system shall identify the affected canonical Position Assignment and establish the correction basis from authoritative business evidence. | L3-E-071 §§3, 7–9 |
| L4-E-071-BH-002 | The system shall validate the proposed correction against Position Definition, Workforce Engagement, organisational placement, effective dating and Position Assignment invariants. | L3-E-071 §§7, 9, 11–12 |
| L4-E-071-BH-003 | The system shall reject a correction that would require Position & Assignment to redefine Position Definition or Workforce Engagement ownership. | L3-E-071 §§4.4, 5, 10–12 |
| L4-E-071-BH-004 | When valid, the system shall establish the corrected Position Assignment through the governed effective-dated/versioned assignment evolution model. | L3-E-071 §§3, 9, 12–13 |
| L4-E-071-BH-005 | The system shall preserve the prior assignment state and effective history as immutable reconstructable lineage. | L3-E-071 §§9, 12–13, 15–16 |
| L4-E-071-BH-006 | The corrected assignment shall become the authoritative current occupancy only after all Position Assignment invariants are satisfied. | L3-E-071 §§3, 9, 12, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-E-071-BH-007 | Organisation shall remain the authoritative source of Position Definition context and Workforce & Engagement shall remain the authoritative source of Workforce Engagement context consumed by the correction. | L3-E-071 §§5, 10, 13–14 |
| L4-E-071-BH-008 | Position & Assignment shall make the corrected authoritative occupancy fact available to dependent consumers without transferring Position Assignment ownership. | L3-E-071 §§3, 14.2, 16 |
| L4-E-071-BH-009 | The system shall not invent a correction event name because the Locked parent requires the exact frozen L2-E02/Stage-3 correction vocabulary to be confirmed upstream. | L3-E-071 §§14.1, 17 |

## 6. Failure / Rejection / Continuation

- Invalid assignment corrections shall not be applied.
- Missing or invalid Position Definition context shall leave the current valid assignment authoritative.
- Incompatible Workforce Engagement context shall prevent invalid occupancy.
- Insufficient correction evidence shall leave the authoritative assignment unchanged.

## 7. Audit & Evidence Behavior

The correction basis, evidence, prior assignment state, corrected state, effective dating and assignment lineage shall remain traceable.

## 8. Observable Completion Conditions

The corrected Position Assignment is authoritative, all assignment invariants are satisfied, prior assignment history remains reconstructable, and Position Definition and Workforce Engagement ownership remain unchanged.

## 9. Traceability & Provenance

**Parent:** L3-E-071 — Position Assignment Administrative Correction to Corrected Authoritative Occupancy  
**Primary verification source:** L3-E-071 §§3, 5, 9, 11–17 and authoritative L2-E02 architecture.

## 10. Open Issues / Deferred Design

The exact canonical administrative-correction event and transition terminology remain upstream confirmation items in L3-E-071. L4 preserves that gap and does not invent vocabulary. Technical data-repair mechanisms remain deferred to L5.

## Canonical Validation Note

This specification is a canonical L4 artifact adhering to L4 Template v1.0 and is not self-locked.
