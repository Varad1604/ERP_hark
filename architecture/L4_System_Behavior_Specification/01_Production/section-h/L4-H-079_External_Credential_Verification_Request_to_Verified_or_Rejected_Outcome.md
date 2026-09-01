# External Credential Verification Request to Verified or Rejected Outcome

## 1. Specification Metadata
**Specification ID:** L4-H-079  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** External Education Integration  
**Realizes Scenario:** L3-H-079  
**Primary Business Owner:** External Education Integration  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L3-H-079; L2-H10 — External Education Integration Workflows; `L2-H10_Provider_Trust_and_Exchange_Governance_Amendment_2026-08-26.md`; `L2-H10_Enrollment_Cancellation_and_Verification_Result_Amendment_2026-08-27.md`; applicable Locked credential/source-domain architecture  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary
SutraERP shall govern an H10 External Credential Verification request through the complete authoritative four-outcome result lifecycle—Verified, Rejected, Expired or Inconclusive—while preserving immutable verification evidence and source-credential ownership.

## 3. Required Business Context
An authorized verification request, source credential reference, trusted external verification context/provider and governed verification window where applicable exist; H10 owns the verification request/result evidence only and does not own the source credential.

## 4. System Behavioral Obligations
| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-H-079-BH-001 | The system shall create the External Credential Verification request in `Requested` only when request authority, credential reference and trusted verification context are valid. | L3-H-079 §§7–9,12–13 |
| L4-H-079-BH-002 | The system shall transition `Requested → Verification Pending` and obtain/evaluate governed provider evidence without mutating the source credential. | L3-H-079 §§5,9,11–13 |
| L4-H-079-BH-003 | When authenticity is established from governed external evidence, the system shall transition `Verification Pending → Verified` and publish `ExternalCredentialVerificationVerified`. | L3-H-079 §§9,12–14 |
| L4-H-079-BH-004 | When the governed verification decision is rejection, the system shall transition `Verification Pending → Rejected` and publish `ExternalCredentialVerificationRejected`. | L3-H-079 §§9,12–14 |
| L4-H-079-BH-005 | When the governed verification request/result window ends before a conclusive result can be established, the system shall transition `Verification Pending → Expired` and publish `ExternalCredentialVerificationExpired`; this shall remain distinct from source-credential expiry. | L3-H-079 §§3,9–14 |
| L4-H-079-BH-006 | When governed evidence exists but is insufficient or contradictory such that neither Verified nor Rejected can be asserted, the system shall transition `Verification Pending → Inconclusive` and publish `ExternalCredentialVerificationInconclusive`. | L3-H-079 §§3,9–14 |
| L4-H-079-BH-007 | Provider unavailability or transport failure alone shall not create `Inconclusive`; the request shall remain unresolved until governed evidence exists or the verification window reaches `Expired`. | L3-H-079 §§10–12 |
| L4-H-079-BH-008 | After any terminal verification result, a later verification shall create a successor request/result and shall not overwrite the immutable prior verification evidence or result. | L3-H-079 §§9–12,15–16 |

## 5. Cross-Domain & Event Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-079-BH-009 | H10 shall publish `ExternalCredentialVerificationVerified`, `ExternalCredentialVerificationRejected`, `ExternalCredentialVerificationExpired`, and `ExternalCredentialVerificationInconclusive` only for H10 verification-result facts. | L3-H-079 §14 |
| L4-H-079-BH-010 | Source credentials/results shall retain their canonical owners and publishers; H10 shall not change source-credential lifecycle as a consequence of external verification. | L3-H-079 §§5,12–14,16 |

## 6. Failure / Rejection / Continuation
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-079-BH-011 | Missing or unresolved provider evidence shall not produce `Verified` or `Rejected`; it shall remain `Verification Pending` until governed evidence or the expiry condition produces an authoritative outcome. | L3-H-079 §§10–11 |

## 7. Audit & Evidence Behavior
| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-H-079-BH-012 | The system shall preserve request authority, source credential reference, provider evidence, verification window where applicable, authoritative result, decision provenance and immutable successor lineage. | L3-H-079 §15 |

## 8. Observable Completion Conditions
The behavior is complete when the External Credential Verification reaches one authoritative terminal result—`Verified`, `Rejected`, `Expired`, or `Inconclusive`—with immutable evidence/lineage preserved and the source credential unchanged by H10.

## 9. Traceability & Provenance
**Parent:** L3-H-079 — External Credential Verification Request to Verified or Rejected Outcome  
**Primary verification source:** L2-H10 — External Education Integration Workflows  
**Four-outcome lifecycle verification:** `L2-H10_Enrollment_Cancellation_and_Verification_Result_Amendment_2026-08-27.md`

## 10. Open Issues / Deferred Design
No material upstream issue prevents derivation. Verification protocol, API and UI implementation remain deferred.
