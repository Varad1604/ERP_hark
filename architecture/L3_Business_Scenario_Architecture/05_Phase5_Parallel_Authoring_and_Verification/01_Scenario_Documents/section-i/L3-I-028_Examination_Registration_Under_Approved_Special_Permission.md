# Examination Registration Under Approved Special Permission

**Scenario ID:** L3-I-028  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Examination Registration  
**Participating Domains:** A, C, I  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Varad  
**Last Updated:** 2026-08-21  
**Depends On:** Locked L1 Business Domain Architecture; L2-I01 Examination Policy & Configuration; L2-I04 Examination Registration Workflows; applicable institutional authorization, audit, Student and Examination Session/Schedule architecture  
**Referenced By:** Downstream examination workflows consuming approved/confirmed Examination Registrations

## 1. Scenario Metadata

| Attribute | Canonical Value |
|---|---|
| Final L3 ID | L3-I-028 |
| Canonical Title | Examination Registration Under Approved Special Permission |
| Primary Owner | Examination Registration |
| Participating Domains | A, C, I |
| Scenario Type | Critical |
| Priority | P1 |
| Status | Locked |

This metadata is reproduced from the existing scenario header only; no frozen Phase 5 identity or classification value is changed.

## 2. Scenario Purpose

This scenario governs examination registration performed under an approved special permission when the candidate's registration circumstance requires an institutionally authorised exception to the normal registration path.

It preserves the distinction between the special permission decision and the Examination Registration decision: the institutional authority permits an exceptional registration path, while Examination Registration remains the canonical owner of the resulting examination participation.

## 3. Business Outcome

An eligible candidate is registered for the authorised examination scope under an approved special permission, with the permission basis, authorisation, registration outcome, and audit history preserved and with all remaining applicable registration requirements continuing to apply.

## 4. Scenario Boundary and Scope

**Start Boundary:** A candidate requires examination registration under a special circumstance for which institutional policy provides an approved special-permission path.

**In Scope:**
- Identification of the special-permission circumstance.
- Verification that an approved permission exists.
- Establishment of candidate and examination context.
- Evaluation of remaining registration eligibility and scope.
- Controlled registration under the approved permission.
- Required review, approval, confirmation, and audit.
- Publication of normal registration lifecycle events.

**Out of Scope:**
- Creation or modification of the examination policy that defines the special permission.
- Creation of institutional authorization mechanisms.
- Changing Student academic ownership.
- Financial processing.
- Hall ticket generation.
- Technical access control, API, storage, or implementation mechanics.

**End Boundary:** The special-permission registration is either confirmed through the normal governed registration lifecycle or rejected because the approved permission or remaining registration conditions do not support it.

## 5. Ownership and Participating Domains

**Examination Registration (I) — Primary Owner:** Owns the examination participation and resulting registration.

**Platform Foundation (A):** Provides authoritative authorization and audit capabilities. It does not own the special-permission business decision or registration.

**Student Lifecycle (C):** Provides candidate academic context and retains ownership of student information.

The permission authorisation remains governed by the applicable institutional policy and authority. No participating domain acquires ownership of the Examination Registration.

## 6. Business Actors, Roles and Responsibilities

- **Candidate (Student):** Is the affected candidate and may participate in the special-permission request process.
- **Examination Administrator:** Verifies the approved permission and manages the registration.
- **Controller of Examination:** Governs examination registration policies and approvals.
- **Authorised Institutional Officer:** Provides the institutional authority represented by the approved special permission where applicable.
- **Academic Administration:** May provide supporting academic context where the permission depends on academic circumstances.

The exact authority and permission process remain governed by upstream policy.

## 7. Preconditions and Entry Criteria

- An Active Examination Session exists.
- An approved Examination Schedule has been published.
- The candidate is identifiable and has the required academic context.
- A special-permission basis is recognised by applicable Examination Policy.
- The required institutional permission has already been approved by the appropriate authority.
- The permission applies to the candidate, examination context, and registration circumstance being considered.
- Remaining registration eligibility requirements can be evaluated.
- Any applicable registration window or approved exception is satisfied.

## 8. Trigger

An approved special permission is presented for an examination registration that would otherwise require exceptional institutional treatment.

## 9. Main Business Flow

1. The candidate and affected examination context are established.
2. The approved special permission and its business applicability are verified.
3. Examination Registration evaluates the candidate and requested registration scope against the remaining applicable eligibility and registration requirements.
4. The approved permission is applied only to the exceptional condition for which it was granted.
5. Required Examination Registration review and approval are completed where applicable.
6. The authorised registration scope is established.
7. The registration is confirmed when all required registration conditions are satisfied.
8. The permission basis and registration decision are preserved in the registration audit history.
9. The normal registration events, including **Registration Approved** and **Registration Confirmed**, are published at their applicable lifecycle milestones.
10. The confirmed registration becomes the authoritative business input for downstream examination workflows.

## 10. Alternate Business Flows

### 10.1 Special Permission Covers Registration Timing

Where the permission authorises registration outside the normal registration window, the registration may proceed through the controlled exception while remaining subject to all other applicable registration requirements.

### 10.2 Special Permission Covers a Defined Examination Opportunity

Where the permission applies to a special examination category or opportunity supported by policy, the registration is established within that approved category.

### 10.3 Special Permission Requires Administrative Registration

Where the candidate is not directly performing the registration and an authorised administrator acts on the candidate's behalf, the registration remains candidate-centric and follows the same validation and audit requirements.

### 10.4 Special Permission Does Not Satisfy All Requirements

If the permission addresses only one exceptional condition but another required registration condition remains unsatisfied, the registration does not bypass that remaining requirement.

## 11. Failure, Interruption and Business Recovery

### 11.1 Permission Not Approved

If no approved special permission exists, the registration follows the normal registration policy and cannot rely on an unapproved exception.

### 11.2 Permission Does Not Apply

If the permission is approved but does not cover the candidate, examination session, registration scope, or circumstance presented, the special-permission path is not applied.

### 11.3 Remaining Eligibility Failure

An approved special permission does not automatically make the candidate eligible for every examination condition. If another mandatory eligibility requirement fails, the registration remains pending or is rejected according to applicable policy.

### 11.4 Permission Withdrawn or Superseded Before Registration

If the permission is no longer authoritative under the applicable institutional governance, the registration cannot rely on it until a valid permission is established.

### 11.5 Registration Approval Not Granted

Where Examination Registration approval remains required, the special permission does not replace that approval. The registration remains unconfirmed until the required registration decision is completed.

## 12. Business Rules and Decision Constraints

- Special registration opportunities are governed by Examination Policy and configuration.
- Institutions may configure registration windows, late registration, extended registration, special registration, and approval requirements without changing the canonical workflow.
- Exceptional administrative overrides require appropriate authority, business justification, and complete audit records.
- A special permission does not transfer ownership of examination policy to Examination Registration.
- A special permission does not automatically waive unrelated eligibility, subject applicability, financial eligibility, or registration approval requirements.
- Examination Registration remains the authoritative owner of the resulting examination participation.
- Platform authorization and audit capabilities remain owned by Section A.
- Candidate academic information remains owned by Student Lifecycle.

## 13. State and Lifecycle Impact

The scenario does not introduce a special-permission lifecycle state.

The approved permission changes the business conditions under which the existing Examination Registration lifecycle may proceed. The registration still progresses through the applicable initiation, validation, review, approval, confirmation, and locking stages.

If the special permission enables an otherwise late or exceptional registration, the resulting registration remains governed by the same canonical lifecycle.

## 14. Business Events and Cross-Domain Collaboration

Examination Registration remains the canonical publisher of registration lifecycle events.

Relevant events include:

- **Registration Initiated**
- **Eligibility Validated**
- **Registration Submitted**
- **Registration Approved**
- **Registration Confirmed**
- **Registration Locked**

No dedicated **Special Permission Approved** event is invented because the applicable policy/authorization architecture does not define one in L2-I04.

The special permission is an upstream business condition consumed by Examination Registration. Platform Foundation provides authorization/audit capabilities without becoming the owner of the examination registration.

## 15. Business Authorization, Approval and Audit Obligations

- The special permission must have been approved by the authority defined by applicable institutional policy.
- The permission's business basis and applicability must be traceable.
- Examination Registration must still complete any registration-specific approval required by policy.
- The special-permission registration and significant administrative actions must be auditable.
- Business justification must be preserved for exceptional administrative overrides.
- No technical IAM/RBAC mechanism is defined at L3.

## 16. Completion Criteria and Postconditions

**Completion Criteria:**
- The special permission is verified as approved and applicable.
- Candidate and examination context are established.
- Remaining eligibility and registration requirements are satisfied.
- Required registration approval is completed.
- The registration is confirmed.
- The permission basis and registration history are auditable.
- Applicable registration lifecycle events are published.

**Postconditions:**
- The confirmed Examination Registration is authoritative for the approved examination participation.
- Downstream examination workflows may consume the confirmed registration subject to their own business prerequisites.
- The special permission does not alter upstream policy ownership.
- Student, authorization, audit, and other domain ownership boundaries remain unchanged.

## 17. Open Issues, Assumptions and Deferred Questions

1. L2-I04 explicitly supports **Special** registration, configurable special registration windows, and exceptional administrative overrides, while L2-I01 defines examination policy and governance of special/exception approval. No new permission type or lifecycle state is introduced here.
2. The exact special-permission authority, eligibility exception, duration, and scope are institution-specific and must come from the applicable approved Examination Policy; this scenario does not invent them.
3. L2-I04 does not define a dedicated special-permission event, so normal registration lifecycle events remain the canonical event model.
