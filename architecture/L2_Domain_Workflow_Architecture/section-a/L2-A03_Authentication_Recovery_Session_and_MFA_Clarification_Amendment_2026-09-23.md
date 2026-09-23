# L2-A03 — Authentication, Recovery, Session and MFA Clarification — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-A03 — Authentication Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-09-23 |
| Approved Through | Explicit Lead Architect decision in L5 Post-Validation Upstream Blocker Resolution programme |
| Scope | Authentication actor/event ownership; session creation/activation; credential-generic recovery; recovery authority; MFA lifecycle/activation |

This amendment is authoritative only for the listed subjects. All unaffected A03 ownership, policy, provider, credential, token, trusted-device and session architecture remains unchanged.

## 1. Authentication Initiation and Generic Execution Events

An **Authentication Requestor** is the logical actor/subject initiating authentication. It may be a human user or an eligible technical caller as governed by the selected Authentication Method/Provider and policy.

Generic authentication execution facts are owned by the **Authentication Attempt** aggregate:

- `AuthenticationStarted`;
- `AuthenticationSucceeded`;
- `PlatformAuthenticationFailed`;
- `AuthenticationChallenged`;
- `AuthenticationCompleted`.

Authentication Session events remain owned by Authentication Session.

## 2. Authentication Session Lifecycle Clarification

Session creation and activation are distinct observable lifecycle steps:

```text
Created → Active → Idle/Suspended/Renewed → Expired/Closed/Revoked
```

The exact valid transition matrix already defined by A03 remains authoritative.

For the login path:

1. successful Authentication Attempt permits creation of an Authentication Session;
2. `AuthSessionCreated` records creation;
3. activation makes the session usable;
4. `SessionActivated` records activation.

Legacy wording that sessions “begin in Active” is superseded by this explicit created-then-active interpretation.

## 3. Credential-Generic Recovery Boundary

A008 is credential-generic recovery.

Recovery applies to any A03 Credential type whose Authentication Method and Authentication Policy declare that governed recovery/replacement is supported.

Password Reset is one concrete recovery variant, not the universal definition of all Credential recovery.

Unsupported Credential types fail closed and require their own governed recovery path.

## 4. Replacement Credential Lifecycle

For governed recovery:

1. resolve Tenant/Identity/User Account and applicable recovery policy;
2. verify the source-permitted recovery method;
3. create replacement Credential in `Pending Activation`;
4. validate required policy conditions;
5. transition replacement Credential `Pending Activation → Active`;
6. publish `CredentialActivated`;
7. permanently invalidate every previous Credential covered by the recovery decision using `RevokeCredential`;
8. publish `CredentialRevoked` for each revoked Credential.

Recovery completion does not require a new generic “Credential Reset Event”.

The authoritative recovery completion is evidenced by the successful replacement-Credential activation plus required prior-Credential revocation and any policy-required session consequences.

## 5. Authentication Recovery Authority

**Authentication Recovery Authority** is the canonical abstract business authority for exceptional administrative recovery when self-service recovery cannot establish sufficient evidence.

This authority:

- may approve or reject administrative recovery;
- does not bypass identity verification or Authentication Policy;
- cannot directly change A02 Person/Identity/User Account lifecycle state;
- may require stronger evidence or manual governance.

Institutional/platform policy may map the authority to an eligible local security/administrative role.

## 6. Recovery Session Consequences

Authentication Policy governs whether existing Authentication Sessions must be terminated following recovery.

When termination is required:

- A03 terminates/revokes the affected sessions through existing canonical session operations/events;
- recovery is not represented as complete until required session consequences are confirmed.

When policy permits session continuity, unchanged sessions remain governed by their existing lifecycle.

## 7. MFA Enrollment Lifecycle Clarification

The canonical MFA Enrollment lifecycle for new enrollment is:

```text
Pending Verification → Verified → Active
```

The word `Enrolled` is not a separate durable state for new enrollment.

`EnrollMFAFactor` creates the MFA Enrollment directly in `Pending Verification` and publishes `MFAEnrollmentStarted`.

`VerifyMFAFactor` transitions:

`Pending Verification → Verified`

and publishes `MFAEnrollmentVerified`.

A new canonical business operation `ActivateMFAFactor` transitions:

`Verified → Active`

and publishes existing canonical event `MFAActivated`.

Legacy prose “Publish MFA Enrolled Event” is superseded; no `MFAEnrolled` event is canonical.

## 8. Locked Decisions

1. Authentication Attempt owns the generic authentication execution events.
2. Authentication Session creation and activation are distinct: `Created → Active`.
3. `AuthSessionCreated` and `SessionActivated` remain Authentication Session events.
4. Recovery is Credential-generic where the credential type/method/policy supports it.
5. Replacement Credential is created `Pending Activation` then activated.
6. Covered previous Credentials are revoked using `RevokeCredential` / `CredentialRevoked`.
7. No generic Credential Reset event is required.
8. Authentication Recovery Authority governs exceptional administrative recovery.
9. Policy-required session termination is part of recovery completion.
10. MFA new-enrollment path is `Pending Verification → Verified → Active`.
11. `ActivateMFAFactor` is the canonical activation operation; `MFAActivated` remains the activation event.
12. `Enrolled` and `MFAEnrolled` are not canonical durable state/event names.
