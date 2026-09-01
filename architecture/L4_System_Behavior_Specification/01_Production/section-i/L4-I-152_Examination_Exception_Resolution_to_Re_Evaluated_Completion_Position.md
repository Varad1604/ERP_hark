# L4-I-152 — Examination Exception Resolution to Re-Evaluated Completion Position

## 1. Specification Metadata
**Specification ID:** L4-I-152  
**Architecture Level:** L4 — System Behavior Specification  
**Realizes Scenario:** L3-I-152 — Examination Exception Resolution to Re-Evaluated Completion Position  
**Primary Business Owner:** Academic Completion & Completion Evidence  
**Participating Domains:** C — Student Lifecycle; I — Examination  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0

## 2. Behavioral Objective & Boundary
Govern re-evaluation of the authoritative completion position when an examination exception is resolved and changes the examination evidence relevant to completion.

**Start:** An authoritative examination exception resolution affects completion-related evidence.  
**End:** A revised completion position is established from the resolved authoritative evidence, or a governed unresolved outcome remains.

Out of scope: owning the examination exception lifecycle, modifying student status directly, or defining technical exception processing.

## 3. Required Business Context
- Student/programme context is authoritative.
- Examination exception resolution is authoritative within the examination governance process.
- Existing completion-evidence evaluation is identifiable.
- Applicable completion-evidence requirements are available.
- Student Lifecycle retains downstream qualification-completion ownership.

## 4. Required System Behaviors
1. Identify the completion position affected by the resolved examination exception.
2. Establish the authoritative exception resolution and its effective scope.
3. Determine which examination evidence is changed by that resolution.
4. Re-evaluate completion evidence using the resulting authoritative examination facts.
5. Establish the revised completion position within I13 scope.
6. Preserve the prior position, exception basis, resolution authority, and re-evaluation history.
7. Prevent obsolete or superseded evidence from remaining current.
8. Make the resulting completion evidence available to the canonical downstream qualification-completion process.

## 5. Cross-Domain Behavior
Examination retains ownership of examination exception and examination facts. Student Lifecycle retains ownership of student context and final qualification completion. Academic Completion & Completion Evidence owns the re-evaluated completion position within its defined scope.

## 6. Failure / Recovery
Incomplete exception resolution, unresolved evidence impact, or conflicting authoritative examination facts shall prevent a conclusive re-evaluation. The issue returns to its canonical examination owner rather than being resolved locally.

## 7. Audit & Evidence
Preserve exception resolution, affected examination evidence, prior completion position, re-evaluation basis, resulting position, authority, and historical traceability.

## 8. Observable Completion
The completion position is re-evaluated from the authoritative resolved examination evidence and is current and traceable, or the unresolved condition remains explicit.

## 9. Traceability
Parent: L3-I-152 — Examination Exception Resolution to Re-Evaluated Completion Position.

## 10. Deferred Design
Technical exception integration, recalculation, storage, interfaces, and notifications remain outside L4.