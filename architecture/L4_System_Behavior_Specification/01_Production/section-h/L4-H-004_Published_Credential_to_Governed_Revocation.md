# Published Credential to Governed Revocation

## 1. Specification Metadata
**Specification ID:** L4-H-004  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** APAAR & DigiLocker  
**Realizes Scenario:** L3-H-004  
**Primary Business Owner:** APAAR & DigiLocker  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** L3-H-004; L2-H01 — APAAR & DigiLocker Workflows; applicable Locked source-domain credential architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall apply an authoritative source revocation requirement to a previously published H01 government Publication without revoking, erasing or rewriting the source credential.

## 3. Required Business Context
Authoritative source revocation basis; corresponding `Published` H01 Publication; Institution Administrator authority for the H01 request; Connector revocation capability; original government reference.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-004-BH-001 | When an authoritative source establishes that a government-published credential must be revoked, the system shall admit H01 revocation only if the corresponding Publication is `Published`, Institution Administrator authority is valid, Connector revocation capability exists, and the original government reference is available. | L3-H-004 §§6–9 |
| L4-H-004-BH-002 | H01 shall consume the authoritative source revocation basis without changing the source credential or assuming the source-domain revocation decision. | L3-H-004 §§5,12 |
| L4-H-004-BH-003 | On successful supported government revocation, the H01 Publication shall transition `Published → Revoked`; unsupported capability or government rejection shall not create a false `Revoked` state and shall preserve the existing Publication state. | L3-H-004 §§9–13 |
| L4-H-004-BH-004 | Completed publication and revocation history shall remain preserved and traceable after revocation. | L3-H-004 §§12,16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-004-BH-005 | H01 shall publish `CredentialRevocationRequested` when the governed H01 request is recorded and `CredentialRevoked` only after successful supported government revocation. | L3-H-004 §14 |

## 6. Failure / Rejection / Continuation
Unclear source authority, non-`Published` H01 position, missing government reference, unsupported Connector revocation or government rejection shall prevent a successful `Revoked` outcome and preserve history for governed follow-up.

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-004-BH-006 | The system shall preserve source revocation basis, original Publication/government reference, Institution Administrator request authority, lifecycle outcome and government revocation evidence. | L3-H-004 §15 |

## 8. Observable Completion Conditions
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-004-BH-007 | Successful completion shall require an authoritative source basis, eligible `Published` H01 Publication, supported and authorized government revocation, state `Revoked`, event `CredentialRevoked`, and preserved history. | L3-H-004 §16 |

## 9. Traceability & Provenance
**Parent:** L3-H-004 — Published Credential to Governed Revocation  
**Primary verification source:** L2-H01 — APAAR & DigiLocker Workflows.

## 10. Open Issues / Deferred Design
The parent does not universally map the generic source revocation event/publisher across C/D/F/I; none is invented here.