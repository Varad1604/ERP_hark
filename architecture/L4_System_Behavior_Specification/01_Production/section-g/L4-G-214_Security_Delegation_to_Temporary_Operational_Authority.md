# Security Delegation to Temporary Operational Authority

## 1. Specification Metadata


**Specification ID:** ** `L4-G-214`  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Campus Operations — Security Operations (G07)  
**Realizes Scenario:** ** `L3-G-214` — Security Delegation to Temporary Operational Authority  
**Primary Business Owner:** Security Operations (G07)  
**Version:** 1.0.1  
**Status:** Draft  
**Owner:** Devang  
**Last Updated:** 2026-09-01  
**Depends On:** ** `L3-G-214_Security_Delegation_to_Temporary_Operational_Authority.md`; `L2-G07_Security_Operations_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`  
**Referenced By:** ** None yet


---

## 2. Behavioral Objective & Boundary

SutraERP shall support governed temporary delegation of Security Operations authority from a valid grantor to an acting security officer with explicit time bounds, scope limits, and restoration of original authority at expiry.

The behavior begins when a CSO/OIC submits a valid delegation notice. It ends when the temporary delegation expires and original authority resumes.

The activation behavior covered here results in the Security Delegation Dossier reaching `DELEGATION_ACTIVE_AND_BOUNDED`. Technical access-right provisioning is outside scope.

---

## 3. Required Business Context

- Valid security authority grantor.
- Acting Security Officer (grantee).
- Registrar / Appointing Authority participation.
- Delegation start/end date-time.
- Explicit delegated authority scope.
- Parent-defined non-sub-delegation and single-point-of-command rule.
- Parent-defined dual-authenticated appointment record.
- Security Operations ownership of the Security Delegation Dossier Aggregate.

---

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| `L4-G-214-BH-001` | When a valid delegation notice is submitted, the system shall establish the Security Delegation Dossier Aggregate in `DELEGATION_PROPOSED`. | L3-G-214 §§4, 9, 13 |
| `L4-G-214-BH-002` | The system shall require an explicit delegation start time, end time, and bounded authority scope before activation. | L3-G-214 §§2–3 |
| `L4-G-214-BH-003` | The system shall preserve the identity and authority of the grantor and grantee and the parent-required appointment evidence. | L3-G-214 §§2–3, 6 |
| `L4-G-214-BH-004` | The system shall enforce the parent-defined non-sub-delegation and single-point-of-command constraint. | L3-G-214 §12 |
| `L4-G-214-BH-005` | The system shall prevent activation if required appointing-authority verification or appointment evidence is missing. | L3-G-214 §§6, 11, 15 |
| `L4-G-214-BH-006` | The system shall prevent the grantee from being treated as holding authority beyond the approved scope or time bounds. | L3-G-214 §§2–4, 12 |
| `L4-G-214-BH-007` | On valid activation, the system shall transition the Security Delegation Dossier Aggregate from `DELEGATION_PROPOSED` to `DELEGATION_ACTIVE_AND_BOUNDED`. | L3-G-214 §§9, 13, 16 |
| `L4-G-214-BH-008` | At the end of the approved delegation period, the system shall recognize restoration of the original authority as required by the parent scenario without defining technical IAM mechanics. | L3-G-214 §4.2 |

---

## 5. Cross-Domain & Event Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| `L4-G-214-BH-009` | The system shall recognize appointing-authority and identity facts from their owning domains without transferring ownership of those facts to Security Operations. | L3-G-214 §§5–6 |
| `L4-G-214-BH-010` | When the delegation reaches `DELEGATION_ACTIVE_AND_BOUNDED`, the system shall produce the canonical `Security_Authority_Delegated` business event on behalf of Security Operations. | L3-G-214 §§14, 16 |

No RBAC/IAM design, credential provisioning, token management, or event transport is defined.

---

## 6. Failure / Rejection / Continuation

- If grantor/grantee identity or appointment authority cannot be validated, the delegation shall not activate.
- If scope or time bounds are missing, the delegation shall remain proposed.
- If sub-delegation is attempted, the system shall prevent it under BR-G07-033.
- If the approved period expires, the acting authority shall no longer be treated as valid for the delegated scope.

---

## 7. Audit & Evidence Behavior

The system shall preserve or make attributable:

- grantor;
- grantee;
- appointing-authority verification;
- delegation scope;
- start/end bounds;
- dual appointment evidence;
- non-sub-delegation compliance;
- activation and expiry/restoration basis;
- resulting `DELEGATION_ACTIVE_AND_BOUNDED` position;
- publication of `Security_Authority_Delegated`.

---

## 8. Observable Completion Conditions

L4 activation behavior is complete when:

- the Security Delegation Dossier Aggregate is `DELEGATION_ACTIVE_AND_BOUNDED`;
- grantor, grantee, scope, and time bounds are attributable;
- non-sub-delegation is enforceable at the business-behavior level;
- `Security_Authority_Delegated` has been produced.

The scenario lifecycle later ends when the approved period expires and original authority resumes.

---

## 9. Traceability & Provenance

**Parent:** `L3-G-214` — Security Delegation to Temporary Operational Authority  
**Primary verification source:** `L2-G07_Security_Operations_Workflows.md` §8.2.1 (Configuration and Activation), §11 (Failure and Exceptions), §13 (Lifecycle Invariants), §14 (Event Contracts)  
**Higher-level verification:** `SutraERP_L1_Business_Domain_Architecture_Locked.md`; `DEPENDENCY_MAP.md`

All BHs derive from the corrected Locked L3 parent.

---

## 10. Open Issues / Deferred Design

No unresolved upstream authority issue remains. The Chief Security Officer (Grantor) owns the business delegation decision; Registrar / Appointing Authority participation remains verification and appointment evidence. Technical authorization administration does not acquire business delegation authority.

IAM/RBAC implementation, credential provisioning, persistence, event transport, audit implementation, and UI/UX remain deferred.
