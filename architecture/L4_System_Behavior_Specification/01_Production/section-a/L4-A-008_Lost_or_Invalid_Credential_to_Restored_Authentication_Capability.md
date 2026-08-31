# Lost or Invalid Credential to Restored Authentication Capability

## 1. Specification Metadata

**Specification ID:** L4-A-008  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authentication  
**Realizes Scenario:** L3-A-008  
**Primary Business Owner:** Authentication  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** `L3-A-008_Lost_or_Invalid_Credential_to_Restored_Authentication_Capability.md`; `L2-A02_Person_and_Identity_Workflows.md`; `L2-A03_Authentication_Workflows.md`; `L2-A06_Event_Infrastructure_Workflows.md`; `L2-A07_Audit_Infrastructure_Workflows.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall restore authentication capability for a legitimate user through a governed recovery method, while preserving A02 Identity ownership, validating the replacement Credential under A03 policy, and preventing continued authentication with previous Credentials covered by the recovery decision. A008 ends before a new authenticated session is created.

## 3. Required Business Context

- One resolved Tenant.
- Canonical active Person and Identity references and an eligible User Account where login is required.
- Lost, unavailable, or invalid current Credential.
- Applicable Credential Policy, Authentication Policy and recovery policy.
- A governed Recovery Authentication method and required verification evidence.
- Source-defined Credential lifecycle and event catalogue.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-008-BH-001 | The system shall prevent Credential recovery unless the Tenant, canonical Identity and eligible User Account context are resolvable. | L3-A-008 §§7, 9, 11–12 |
| L4-A-008-BH-002 | The system shall select only a recovery method permitted by the applicable recovery and Authentication policies. | L3-A-008 §§9–12 |
| L4-A-008-BH-003 | The system shall prevent replacement-Credential availability when identity verification through the selected governed recovery method fails. | L3-A-008 §§9, 11–12 |
| L4-A-008-BH-004 | After successful recovery verification, the system shall create and policy-validate the replacement Credential under Authentication ownership without changing Person, Identity or User Account lifecycle state. | L3-A-008 §§4–5, 9, 12–13 |
| L4-A-008-BH-005 | The system shall not expose the replacement Credential as valid for authentication until it satisfies the source-defined condition for an `Active` Credential. | L3-A-008 §§9, 12–13 |
| L4-A-008-BH-006 | The system shall not declare recovery complete while any previous Credential covered by the recovery decision remains capable of authentication. | L3-A-008 §§3, 9, 11–13 |
| L4-A-008-BH-007 | The system shall apply an existing-session consequence only when the applicable Authentication Policy requires it. | L3-A-008 §§9–13 |

## 5. Cross-Domain & Event Behavior

The system may publish only canonical A03 Credential events whose source-defined transition conditions are actually satisfied. `CredentialCreated`, `CredentialActivated`, `CredentialRotated`, and `CredentialRevoked` shall not be substituted for an undefined canonical recovery-completion event. No new recovery event name or publisher is introduced.

## 6. Failure / Rejection / Continuation

The system shall reject or halt recovery when Tenant/Identity context cannot be resolved, the User Account is ineligible, no permitted recovery method exists, verification fails, or the replacement Credential fails policy validation. If the exact source-defined previous-Credential invalidation transition or recovery-completion event cannot be determined, the system shall not claim the final restored outcome.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-008-BH-008 | The system shall preserve attributable evidence of recovery request context, selected recovery method, verification outcome, replacement-Credential validation, previous-Credential invalidation status, applicable session consequence, and final recovery result without exposing Credential values. | L3-A-008 §§12, 15–16 |

## 8. Observable Completion Conditions

Completion requires successful governed identity recovery, a replacement Credential valid for authentication, previous Credentials covered by the decision rendered incapable of authentication, any policy-required session consequence applied, and sufficient audit evidence. A008 does not create an Authentication Session; later authentication remains A007.

## 9. Traceability & Provenance

**Parent:** L3-A-008 — Lost or Invalid Credential to Restored Authentication Capability  
**Primary verification source:** L2-A03 — Authentication Workflows  
**Additional verification:** L2-A02 — Person & Identity; L2-A06 — Event Infrastructure; L2-A07 — Audit Infrastructure

## 10. Open Issues / Deferred Design

The Locked parent preserves unresolved A03 semantics for the exact previous-Credential invalidation transition, reset-specific activation sequence, canonical recovery-completion event, and administrative-recovery authority. This Draft therefore defines observable gates and outcomes without choosing an unsupported state, event, or approver.
