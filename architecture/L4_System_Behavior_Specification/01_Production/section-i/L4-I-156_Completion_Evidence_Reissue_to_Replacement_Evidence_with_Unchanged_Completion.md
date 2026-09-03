# Completion Evidence Reissue to Replacement Evidence with Unchanged Completion

## 1. Specification Metadata

**Specification ID:** L4-I-156  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Examination  
**Realizes Scenario:** L3-I-156 — Completion Evidence Reissue to Replacement Evidence with Unchanged Completion  
**Primary Business Owner:** Academic Completion & Completion Evidence  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Varad  
**Last Updated:** 2026-09-03  
**Depends On:** L3-I-156; L2-I13 Academic Completion, CGPA & Completion Evidence; L2-C05 Qualification Completion & Student Exit  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall establish replacement I13-owned completion evidence when governed reissue is required and the underlying completion position remains unchanged, without reopening or redefining the institutional Qualification Completion decision.

The boundary begins when a governed evidence-reissue need is established and ends when replacement evidence is the current authoritative evidence position while prior evidence remains traceable.

## 3. Required Business Context

- Existing I13 completion evidence and its history.
- Authoritative student and completion scope.
- Source-supported reissue condition and authority.
- Authoritative academic evidence confirming that the underlying completion basis has not changed.
- L2-C05 retains institutional Qualification Completion ownership.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-I-156-BH-001 | When governed evidence reissue is required, the system shall identify the existing completion evidence, student, completion scope, and reissue basis. | L3-I-156 §§3, 7–9 |
| L4-I-156-BH-002 | The system shall verify from authoritative business facts that the underlying completion position has not changed before treating the case as evidence-only reissue. | L3-I-156 §§3–4, 7, 9–12 |
| L4-I-156-BH-003 | The system shall preserve the existing evidence history before establishing replacement evidence through the existing I13 evidence lifecycle. | L3-I-156 §§9, 12–13, 15–16 |
| L4-I-156-BH-004 | The system shall establish replacement completion evidence that represents the same unchanged completion position and shall not create a new qualification decision. | L3-I-156 §§3, 9, 12–13, 16 |
| L4-I-156-BH-005 | The system shall distinguish the replacement evidence as current while preserving prior evidence as historical rather than silently deleting it. | L3-I-156 §§3, 9, 12–13, 16 |
| L4-I-156-BH-006 | If an authoritative academic outcome changes, the system shall stop the evidence-only reissue path and require the applicable evidence re-evaluation against the new authoritative outcome. | L3-I-156 §§10–12, 17 |
| L4-I-156-BH-007 | If the governed reissue condition is not satisfied, the system shall retain the existing authoritative evidence position rather than inventing a replacement outcome. | L3-I-156 §§10–12, 16 |

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-I-156-BH-008 | Where the replacement current I13 evidence position is supplied to L2-C05, the system shall use the existing `ExaminationCompletionEvidencePrepared` event; no `CompletionEvidenceReissued` event is introduced. | L3-I-156 §§14, 16–17; L2-I13 Ownership Resolution |

## 6. Failure / Rejection / Continuation

- If the unchanged-completion premise cannot be established from authoritative facts, the system shall not complete evidence-only reissue.
- If the existing evidence is not eligible for governed replacement, the system shall preserve the current position and route the case through the existing evidence-resolution path.
- If student association is unresolved, replacement evidence shall not be finalized until authoritative student context is available.

## 7. Audit & Evidence Behavior

The system shall preserve the original evidence, reissue basis and authority, unchanged-completion evidence, replacement evidence, prior/current distinction, and any governed hand-off to L2-C05.

## 8. Observable Completion Conditions

Behavior is complete when the unchanged completion basis is confirmed, replacement I13 evidence is established as current through the existing evidence lifecycle, prior evidence remains traceable, and any applicable C05 hand-off uses `ExaminationCompletionEvidencePrepared`. No qualification decision is reopened by this behavior.

## 9. Traceability & Provenance

**Parent:** L3-I-156 — Completion Evidence Reissue to Replacement Evidence with Unchanged Completion  
**Primary verification source:** L2-I13 — Academic Completion, CGPA & Completion Evidence Workflows  
**Material anchors:** L3-I-156 §§3, 5, 9, 11–17; L2-I13 Ownership Resolution — Post-Conferment Evidence and Qualification Ordering

## 10. Open Issues / Deferred Design

No scenario-specific L4 blocker remains. The parent does not establish a dedicated evidence-reissue event or new reissue lifecycle state; neither is invented. Technical implementation remains deferred to L5.

## Canonical Validation Note

This Draft preserves I13 evidence ownership and C05 Qualification Completion ownership while keeping reissue distinct from changed academic outcomes.