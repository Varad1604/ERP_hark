# L4-I-033 — Conflicting Registration Scope to Reconciled Registration Position

## 1. Specification Metadata
**Specification ID:** L4-I-033  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination Registration  
**Realizes Scenario:** L3-I-033 — Conflicting Registration Scope to Reconciled Registration Position  
**Primary Business Owner:** Examination Registration  
**Version:** 1.0  
**Status:** Draft

## 2. Behavioral Objective & Boundary
Govern reconciliation where authoritative information produces conflicting examination-registration scope positions.

**Start:** Conflicting registration-scope positions are identified.  
**End:** One governed authoritative registration position is established, or the conflict remains unresolved under a controlled business state.

## 3. Required Business Context
- Candidate and examination context are identifiable.
- Conflicting scope information is identifiable.
- Canonical ownership of contributing information is known.
- Applicable examination policy is available.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-033-BH-001 | Identify the conflicting registration-scope positions and their authoritative sources. | L3-I-033 |
| L4-I-033-BH-002 | Evaluate each position against applicable academic and examination authority. | L3-I-033 |
| L4-I-033-BH-003 | Resolve the conflict using the applicable governing rule and authorised decision path. | L3-I-033 |
| L4-I-033-BH-004 | Establish the reconciled registration position as authoritative when resolution is valid. | L3-I-033 |
| L4-I-033-BH-005 | Preserve conflicting inputs and reconciliation evidence. | L3-I-033 |

## 5. Cross-Domain & Event Behavior
Contributing domains retain ownership of their source information. Examination Registration owns the resulting examination participation position. No unsupported conflict-resolution event is invented.

## 6. Failure / Rejection / Continuation
- Unresolved authority conflicts shall not be silently resolved.
- Non-authoritative information shall not override authoritative information.
- Where a decision is required, the registration remains in its governed position until the decision is established.

## 7. Audit & Evidence Behavior
Preserve source positions, authority evaluation, decision, resulting reconciliation, and prior registration history.

## 8. Observable Completion Conditions
Completion occurs when a reconciled authoritative registration position is established with traceable evidence, or an unresolved governed position is retained.

## 9. Traceability & Provenance
**Parent:** L3-I-033 — Conflicting Registration Scope to Reconciled Registration Position

## 10. Open Issues / Deferred Design
Exact conflict precedence is determined by applicable institutional policy and remains outside technical implementation design.