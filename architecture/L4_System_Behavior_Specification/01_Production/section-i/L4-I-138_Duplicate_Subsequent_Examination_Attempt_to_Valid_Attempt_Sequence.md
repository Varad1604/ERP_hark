# Duplicate Subsequent Examination Attempt to Valid Attempt Sequence

## 1. Specification Metadata

**Specification ID:** L4-I-138  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-138 — Duplicate Subsequent Examination Attempt to Valid Attempt Sequence  
**Primary Business Owner:** Supplementary, Backlog & Improvement Examination  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-138; L2-I12; L2-I04; authoritative Student Lifecycle context  
**Referenced By:** Downstream examination workflows consuming attempt authority

## 2. Behavioral Objective & Boundary

Resolve potentially duplicate subsequent-examination activity so that one valid authoritative attempt sequence remains, while legitimate distinct attempts remain distinct and Examination Registration stays I04-owned.

## 3. Required Business Context

- Authoritative candidate, examination scope and attempt history.
- Applicable attempt-policy position.
- Authoritative I04 registration context where relevant.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-138-BH-001 | Identify potentially duplicate activity for the same candidate and examination scope. | L3-I-138 §§3,8–9 |
| L4-I-138-BH-002 | Compare candidate, scope, session, authorization, chronology and attempt history. | L3-I-138 §§9,12 |
| L4-I-138-BH-003 | Distinguish a duplicate from a legitimately distinct policy-permitted attempt. | L3-I-138 §§3,9–12 |
| L4-I-138-BH-004 | Prevent duplicate activity from becoming a competing authoritative attempt. | L3-I-138 §§3,9,12 |
| L4-I-138-BH-005 | Route any registration correction through I04 rather than creating a parallel registration disposition. | L3-I-138 §§5,9,12–13 |
| L4-I-138-BH-006 | Preserve all affected attempt activity and the reconciliation rationale. | L3-I-138 §§3,9,15–16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-138-BH-007 | The authoritative attempt sequence shall be made available under I12 ownership. Existing I12 or I04 events shall be published only when their actual governed lifecycle milestones occur; no `DuplicateAttemptResolved` or `AttemptMerged` event is introduced. | L3-I-138 §14 |

## 6. Failure / Rejection / Continuation

- Ambiguous candidate or examination context shall keep reconciliation unresolved.
- Attempt-limit exhaustion shall not be bypassed.
- Conflicting registrations return to I04.
- Downstream consequences remain with their owning workflows.

## 7. Audit & Evidence Behavior

Preserve candidate/scope matching, prior attempts, duplicate evidence, registration context, decision authority and resulting sequence.

## 8. Observable Completion Conditions

Completion occurs when the activity is authoritatively classified as duplicate or legitimate, one valid attempt sequence is established, registration ownership remains intact, and the resolution is auditable.

## 9. Traceability & Provenance

**Parent:** L3-I-138  
**Primary verification sources:** L2-I12 and L2-I04.

## 10. Open Issues / Deferred Design

Exact criteria distinguishing duplicate from legitimate subsequent attempt remain policy-owned. No duplicate-specific lifecycle state or event is defined.

## Canonical Validation Note

This specification preserves I12 attempt ownership and I04 registration ownership.
