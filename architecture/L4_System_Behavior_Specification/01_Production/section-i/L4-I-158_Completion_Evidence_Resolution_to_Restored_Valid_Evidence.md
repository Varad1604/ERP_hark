# L4-I-158 — Completion Evidence Resolution to Restored Valid Evidence

## 1. Specification Metadata
- Specification ID: L4-I-158
- Architecture Level: L4 — System Behavior Specification
- Realizes Scenario: L3-I-158 — Completion Evidence Resolution to Restored Valid Evidence
- Primary Business Owner: Academic Completion & Completion Evidence
- Participating Domains: C — Student Lifecycle; H — Institutional Services & Intelligence; I — Examination
- Scenario Type: Critical; Priority: P1
- Status: Draft; Version: 1.0

## 2. Behavioral Objective & Boundary
Restore a valid completion-evidence position after the authoritative cause of a prior invalid or blocked evidence position has been resolved.

Start: a previously invalid, blocked or unresolved evidence position has an authoritative resolution basis. End: a restored valid evidence position is established within the existing evidence lifecycle.

Out of scope: changing source-owned examination results, qualification ownership, or technical restoration mechanisms.

## 3. Required Business Context
- Prior completion-evidence position and its history are identifiable.
- Authoritative resolution of the blocking/invalidating condition exists.
- Supporting academic/examination evidence is current.
- Applicable restoration authority is established.

## 4. Required System Behaviors
1. Identify the prior completion-evidence position and reason it became invalid, blocked or unresolved.
2. Verify the authoritative resolution of that reason.
3. Re-evaluate completion evidence against current authoritative inputs.
4. Establish restored valid evidence where all applicable conditions are satisfied.
5. Preserve prior invalid/blocked evidence history and the resolution basis.
6. Prevent restoration where the underlying issue remains unresolved.
7. Make restored evidence available for authorized downstream use.

## 5. Cross-Domain Behavior
Examination retains ownership of examination facts; Student Lifecycle retains student/qualification lifecycle ownership; completion evidence owns the evidence restoration decision within its scope. Downstream services consume the resulting authoritative evidence.

## 6. Failure / Recovery
- Resolution is non-authoritative: keep prior governed position.
- Current evidence remains insufficient: do not restore validity.
- Conflicting source facts: resolve with canonical source owner before restoration.

## 7. Audit & Evidence
Preserve prior evidence position, blocking/invalidating cause, authoritative resolution, re-evaluation, restoration decision and resulting evidence history.

## 8. Observable Completion
A valid completion-evidence position is restored and traceably linked to the prior blocked/invalid position and its authoritative resolution.

## 9. Traceability
Parent: L3-I-158 — Completion Evidence Resolution to Restored Valid Evidence.

## 10. Deferred Design
Technical restoration, persistence, notifications and integration remain outside L4.