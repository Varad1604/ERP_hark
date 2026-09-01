# L4-I-156 — Completion Evidence Reissue to Replacement Evidence with Unchanged Completion

## 1. Specification Metadata
- Specification ID: L4-I-156
- Architecture Level: L4 — System Behavior Specification
- Realizes Scenario: L3-I-156 — Completion Evidence Reissue to Replacement Evidence with Unchanged Completion
- Primary Owner: Academic Completion & Completion Evidence
- Participating Domains: C — Student Lifecycle; H — Institutional Services & Intelligence; I — Examination
- Scenario Type: Critical; Priority: P1
- Status: Draft; Version: 1.0

## 2. Behavioral Objective & Boundary
Govern reissue of completion evidence when the evidence representation must be replaced while the underlying authoritative completion decision remains unchanged.

Start: an authorized reissue condition affects existing completion evidence. End: replacement evidence is authoritative and the unchanged completion decision remains canonical.

Out of scope: changing qualification completion, recalculating examination results, or defining technical credential generation.

## 3. Required Business Context
- Existing completion evidence is identifiable.
- Underlying qualification/completion decision is authoritative.
- Reissue authority and reason are established.
- Source facts supporting unchanged completion remain valid.

## 4. Required System Behaviors
1. Identify the existing evidence and reissue reason.
2. Confirm that the underlying completion decision remains unchanged and authoritative.
3. Validate source evidence required for replacement issuance.
4. Establish replacement completion evidence without changing the qualification decision.
5. Preserve the superseded evidence and reason for replacement.
6. Associate the replacement evidence with the same authoritative completion outcome.
7. Make only the valid replacement evidence available for authorized use.

## 5. Cross-Domain Behavior
Academic Completion & Completion Evidence owns the evidence outcome. Student Lifecycle retains qualification ownership. Section H consumes the governed evidence within its own service scope. Examination facts remain source-owned.

## 6. Failure / Recovery
- Reissue authority missing: do not replace evidence.
- Underlying completion outcome changed: treat as re-evaluation, not simple reissue.
- Source discrepancy: resolve with canonical owner before replacement.

## 7. Audit & Evidence
Preserve original evidence, reissue authority, reason, validation basis, replacement evidence and association to unchanged completion.

## 8. Observable Completion
A replacement evidence position is authoritative, historically linked to the superseded evidence, while the underlying completion decision remains unchanged.

## 9. Traceability
Parent: L3-I-156 — Completion Evidence Reissue to Replacement Evidence with Unchanged Completion.

## 10. Deferred Design
Technical document rendering, storage, signing and delivery remain outside L4.