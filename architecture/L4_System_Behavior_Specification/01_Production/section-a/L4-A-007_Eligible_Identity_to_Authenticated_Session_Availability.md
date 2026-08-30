# Eligible Identity to Authenticated Session Availability

## 1. Specification Metadata

**Specification ID:** L4-A-007  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authentication  
**Realizes Scenario:** L3-A-007  
**Primary Business Owner:** Authentication  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-30  
**Depends On:** `L3-A-007_Eligible_Identity_to_Authenticated_Session_Availability.md`; `L2-A03_Authentication_Workflows.md`; `L2-A02_Person_and_Identity_Workflows.md`; `L2-A04_Authorization_and_Policy_Workflows.md`; `SutraERP_L1_Business_Domain_Architecture_Amendment_2026-08-26.md`  
**Referenced By:** None yet

## 2. Behavioral Objective & Boundary

SutraERP shall authenticate an eligible canonical Identity within exactly one Tenant, enforce the applicable authentication and MFA conditions, and expose an `Active` Authentication Session and authenticated context without granting business authorization.

## 3. Required Business Context

- One resolved Tenant and canonical Person/Identity references.
- User Account status that permits authentication; no unsupported `Active`/`Enabled` mapping is inferred.
- Applicable Authentication Policy, Authentication Method and Authentication Provider.
- Eligible Credential and, where required, active verified MFA factor.

## 4. System Behavioral Obligations

| Behavior ID | Required System Behavior | Provenance |
|---|---|---|
| L4-A-007-BH-001 | The system shall bind each authentication attempt to exactly one resolved Tenant and one canonical Identity before authentication proceeds. | L3-A-007 §§7, 9, 12, 16 |
| L4-A-007-BH-002 | The system shall prevent authentication when the applicable User Account status does not permit authentication. | L3-A-007 §§7, 9, 11–12 |
| L4-A-007-BH-003 | The system shall evaluate the applicable Authentication Policy before accepting credential verification as sufficient. | L3-A-007 §§9, 12, 16 |
| L4-A-007-BH-004 | The system shall accept only an Authentication Method, Authentication Provider and Credential that satisfy the applicable policy and source-defined eligibility conditions. | L3-A-007 §§9, 11–12 |
| L4-A-007-BH-005 | When policy requires MFA, the system shall prevent authentication success until every required challenge succeeds using an eligible active verified factor. | L3-A-007 §§9–12, 16 |
| L4-A-007-BH-006 | When all authentication conditions succeed, the system shall recognize the source-defined successful authentication outcome and publish `AuthenticationSucceeded` using the parent’s source-stated publisher semantics. | L3-A-007 §§9, 14, 16 |
| L4-A-007-BH-007 | Following successful authentication, the system shall establish the Authentication Session, publish `AuthSessionCreated`, and make the session `Active` with `SessionActivated`. | L3-A-007 §§9, 13–16 |
| L4-A-007-BH-008 | The system shall make the active authenticated context available to Authorization & Policy without treating authentication success as permission to perform a protected business operation. | L3-A-007 §§2–5, 14–16 |

## 5. Cross-Domain & Event Behavior

The system shall preserve the exact parent event semantics for authentication attempt/challenge/session lifecycle facts. `AuthenticationSucceeded` and the other generic authentication execution events retain the publisher wording established by L2-A03; L4 does not assign them to a different aggregate. `AuthSessionCreated` and `SessionActivated` remain Authentication Session facts. Authorization consumes authenticated context but owns later permission decisions.

## 6. Failure / Rejection / Continuation

If Tenant/Identity/account eligibility, policy, credential verification or required MFA fails, the system shall not expose an authenticated context or active session as a successful outcome. Where the parent’s terminal failure conditions are reached, the exact source-defined failure events shall be recognized without inventing recovery or retry mechanics.

## 7. Audit & Evidence Behavior

| Behavior ID | Required Behavior | Provenance |
|---|---|---|
| L4-A-007-BH-009 | The system shall preserve attributable evidence of the authentication attempt, Tenant/Identity context, account eligibility, policy and method/provider basis, credential/MFA outcomes, session establishment and terminal authentication outcome. | L3-A-007 §15.3 |

## 8. Observable Completion Conditions

Completion requires successful policy-governed authentication, an `Active` Authentication Session associated with the Tenant and authenticated Identity, source-defined success/session events, and authenticated context available to A04. No Role, Permission or Authorization Decision is created.

## 9. Traceability & Provenance

**Parent:** L3-A-007 — Eligible Identity to Authenticated Session Availability  
**Primary verification source:** L2-A03 — Authentication Workflows

## 10. Open Issues / Deferred Design

The parent records unresolved account-status terminology, generic authentication-event publisher wording and session-creation wording. This draft preserves the parent’s safe abstractions rather than selecting unsupported mappings. Technical credential handling, session persistence, protocols and UI remain deferred.