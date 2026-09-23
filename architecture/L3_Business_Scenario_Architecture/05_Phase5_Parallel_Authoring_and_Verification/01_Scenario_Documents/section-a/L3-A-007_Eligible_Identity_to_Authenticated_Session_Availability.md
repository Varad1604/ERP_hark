# Eligible Identity to Authenticated Session Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authentication  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows, including the approved 2026-08-20 Person–Identity establishment and activation sequencing decision; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  ; `L2-A02_User_Account_and_Person_Reconciliation_Authority_Amendment_2026-09-23.md`; `L2-A03_Authentication_Recovery_Session_and_MFA_Clarification_Amendment_2026-09-23.md`
**Referenced By:** L2-A04 — Authorization & Policy Workflows; business domains requiring authenticated execution

---

# 2. Scenario Purpose

Authenticate an eligible canonical Identity under the applicable Tenant Authentication Policies and establish an active Authentication Session that can be consumed by subsequent authorization.

---

# 3. Business Outcome

Identity verification and applicable authentication requirements succeed and a valid authenticated session becomes available for subsequent authorization.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a valid Identity requires authenticated access to SutraERP and an authentication request is initiated within a resolved Tenant context.

## 4.2 End Boundary

The scenario ends when authentication has succeeded, an Authentication Session associated with exactly one Tenant and one authenticated Identity is available in `Active`, the applicable authentication and session events have been published, and the authenticated context is available for subsequent authorization by L2-A04.

## 4.3 In Scope

- Tenant resolution for the authentication request.
- Canonical Identity-reference resolution.
- Person, Identity and User Account eligibility validation through L2-A02 contracts.
- Applicable Authentication Policy evaluation.
- Authentication Method and Authentication Provider selection.
- Credential verification.
- Multi-factor challenge and verification when required by policy.
- Authentication decision and Authentication Attempt completion.
- Authentication Session creation and activation.
- Refresh Token generation as governed by the login workflow.
- Publication of canonical authentication, attempt, MFA, session and refresh-token events.
- Production of the required authentication audit record.
- Availability of the authenticated context for subsequent authorization.

## 4.4 Out of Scope

- Person or Identity establishment, verification or activation.
- User Account creation.
- Credential creation, recovery, replacement or restoration.
- MFA-factor enrollment, replacement or activation.
- Authentication Provider onboarding or lifecycle governance.
- Role, Role Assignment, Delegation or authorization-policy lifecycle.
- Permission evaluation or authorization decisions.
- Workspace authorization or selection decisions.
- Session renewal, suspension, revocation, expiration, logout or termination after the initial authenticated session becomes available.
- Business-domain operations performed after authorization.
- APIs, database structures, user-interface behaviour, infrastructure, deployment and other implementation mechanics.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Upstream | L3-A-004 — Person Establishment to Active Canonical Identity | Establishes the active Person and Identity required before login capability can be created and used. |
| Related recovery | L3-A-008 — Lost or Invalid Credential to Restored Authentication Capability | Owns restoration when the current credential cannot support authentication. |
| Related prerequisite | L3-A-009 — Multi-Factor Enrollment to Active Verified Authentication Factor | Establishes an active verified factor; this scenario only consumes it when policy requires MFA. |
| Downstream | L3-A-010 — Institutional Responsibility Assignment to Effective Authorized Access | Governs responsibility-based authorized access; successful authentication alone does not provide authorization. |
| Supporting | L3-A-055 — Institutional Authentication Provider Onboarding to Governed Provider Availability | Establishes governed provider availability; this scenario only selects and consumes an eligible provider. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authentication / Section A | Primary owner. Owns Authentication Attempts, Credentials, Authentication Methods, Authentication Providers, MFA Enrollment, Authentication Policies, Authentication Sessions, Refresh Tokens, authentication decisions and authentication events. |
| Platform & Tenant / Section A | Supplies the resolved Tenant context and validates the Tenant boundary required for authentication. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references and their eligibility statuses without transferring ownership to Authentication. |
| Authorization & Policy / Section A | Consumes the resulting active Authentication Session and authenticated context for later authorization decisions without changing Authentication Session state. |
| Event Infrastructure / Section A | Owns the governed publication, routing and delivery infrastructure for Authentication-owned business events. |
| Audit Infrastructure / Section A | Consumes authentication events and owns immutable audit evidence and its lifecycle. |

All participating capabilities remain within frozen Participating Domain `A`. Authentication remains the Primary Owner even where another Section A domain supplies or consumes a contract.

---

# 6. Business Actors, Roles and Responsibilities

L2-A03 explicitly records that its canonical Actors section is absent. It does not name the business actor or role that initiates the authentication request.

Accordingly, this scenario does not invent an initiator, administrator, approver or security role. The eligible Identity is the business subject of authentication, and responsibilities in the scenario flow are assigned to authoritative business domains and aggregates.

No business approval authority or separation-of-duty role is introduced by this scenario.

---

# 7. Preconditions and Entry Criteria

- The authentication request can be associated with exactly one resolved Tenant.
- The Tenant is eligible for downstream execution under L2-A01.
- A canonical Person and associated Identity have completed the L2-A02 establishment sequence and are both `Active`.
- Where login capability is required, the User Account was created only after successful Person and Identity activation.
- The User Account status permits authentication under the applicable Person & Identity and Authentication contracts.
- A valid canonical Identity reference can be resolved.
- An applicable Authentication Policy can be determined within the Tenant boundary.
- An eligible Authentication Method and Authentication Provider are available under that policy.
- A Credential in `Active` is available for the selected authentication method.
- Where policy requires MFA, an eligible active verified authentication factor is available for challenge.

---

# 8. Trigger

A valid identity requires authenticated access to SutraERP.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Accept the authentication request, establish an Authentication Attempt and publish `AuthenticationAttemptReceived` and `AuthenticationStarted`. | Authentication — Authentication Attempt and authentication execution | A traceable authentication attempt has begun. |
| 2 | Resolve the Tenant context and confirm that the request belongs to exactly one valid Tenant. | Platform & Tenant; Authentication | The authentication boundary is established. |
| 3 | Resolve the canonical Identity reference and validate the Person, Identity and User Account eligibility information supplied by L2-A02. | Person & Identity; Authentication | The request is associated with an eligible canonical identity and account context. |
| 4 | Begin processing the Authentication Attempt and publish `AuthenticationAttemptProcessing`. | Authentication — Authentication Attempt | The attempt is under governed authentication processing. |
| 5 | Evaluate the applicable Authentication Policy before credential verification. | Authentication — Authentication Policy | The required method, provider, credential conditions, MFA requirements and session constraints are determined. |
| 6 | Select an eligible Authentication Method and Authentication Provider in accordance with the evaluated policy. | Authentication | The governed verification path is selected. |
| 7 | Verify the Credential through the selected method and provider. | Authentication | The primary authentication requirement succeeds. |
| 8 | Where MFA is required, publish `AuthenticationChallenged` and `MFAChallengeStarted`, verify the applicable active factor, and publish `MFAChallengeSucceeded`. | Authentication — authentication execution and MFA Enrollment | The additional authentication requirement succeeds. |
| 9 | Complete the successful authentication decision and publish `AuthenticationSucceeded`. | Authentication execution | The Identity is successfully authenticated; authorization has not yet occurred. |
| 10 | Create the Authentication Session, associate it with exactly one Tenant and one authenticated Identity, apply the applicable Session Policy, and publish `AuthSessionCreated`. | Authentication — Authentication Session | The authenticated execution context is established. |
| 11 | Activate the Authentication Session and publish `SessionActivated`. | Authentication — Authentication Session | The Authentication Session is available in `Active`. |
| 12 | Generate the Refresh Token for the authenticated session as governed by the Login Workflow and publish `RefreshTokenGenerated`. | Authentication — Refresh Token | Governed session continuity is available without changing the authentication outcome. |
| 13 | Complete the Authentication Attempt, publish `AuthenticationAttemptCompleted` and `AuthenticationCompleted`, and produce the required immutable authentication audit record. | Authentication — Authentication Attempt and authentication execution; Audit Infrastructure | The successful terminal authentication outcome is traceable. |
| 14 | Make the active Authentication Session and immutable authenticated context available to L2-A04. | Authentication; Authorization & Policy | Subsequent authorization may begin without transferring session ownership to Authorization. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| MFA not required | The evaluated Authentication Policy does not require an additional factor. | Complete credential verification and proceed directly to the successful authentication decision. | The same active Authentication Session outcome is reached without an MFA challenge. | Authentication |
| MFA required | The evaluated policy requires an additional factor. | Challenge and verify an eligible active factor before authentication can succeed. | The scenario continues only after `MFAChallengeSucceeded`. | Authentication |
| Provider selected by policy | More than one Authentication Provider or method is eligible. | Apply the applicable policy and select the governed provider and method. | Authentication follows the same canonical workflow regardless of provider. | Authentication |
| Trusted-device policy applies | A governed trusted-device status affects the required authentication challenge. | Apply the applicable Trusted Device and Authentication Policies without bypassing policy evaluation. | Authentication requirements are determined by policy and the same session outcome remains required. | Authentication |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant cannot be resolved or is not eligible for execution. | Authentication cannot validly proceed outside a resolved Tenant boundary. | Reject the attempt; do not create an Authentication Session. Tenant restoration remains governed by L2-A01. | Platform & Tenant; Authentication |
| Identity reference cannot be resolved or the Person or Identity is not eligible. | Authentication cannot establish trust for a valid canonical Identity. | Reject the attempt; do not change Person or Identity state. Identity correction or activation remains governed by L2-A02. | Person & Identity; Authentication |
| User Account status restricts authentication. | Authentication is terminated and no session becomes available. | Preserve the failed outcome and resolve the account restriction through the applicable L2-A02 or governed recovery workflow. | Person & Identity; Authentication |
| Credential is invalid, `Suspended` or `Expired`. | Authentication is rejected. | Publish the applicable failure and terminal events; restoration of authentication capability belongs to L3-A-008. | Authentication |
| Required MFA challenge fails. | Authentication is rejected and no session is created. | Publish `MFAChallengeFailed`, the applicable authentication-failure event and the terminal completion events. Factor enrollment or replacement remains outside this scenario. | Authentication |
| Authentication Policy is violated. | Authentication is terminated. | Record the failed terminal outcome; authentication may proceed only through a later request that satisfies the applicable policy. | Authentication |
| No eligible method or provider is available. | Credential verification cannot complete. | Reject the attempt and retain the auditable failure. Provider governance remains outside this scenario. | Authentication |
| Authentication succeeds but session creation or activation does not complete. | The frozen business outcome is not achieved because no valid active session is available. | Do not expose an authenticated context for authorization. L2-A03 does not define a separate business recovery outcome for this interruption. | Authentication |

For every failed terminal outcome, authentication execution publishes `PlatformAuthenticationFailed` and `AuthenticationCompleted`; the Authentication Attempt publishes `AuthenticationAttemptCompleted`. Failed operations do not publish success events.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every authentication request belongs to exactly one Tenant. | Authentication / Platform & Tenant | Authentication stops if a single Tenant boundary cannot be resolved. | L2-A03 — Authentication Request Workflow |
| Authentication begins only after a valid Identity reference has been resolved. | Person & Identity; Authentication | Authentication cannot create or substitute an Identity. | L2-A03 — AW1 Identity Before Authentication |
| User Account creation occurs only after both Person and associated Identity are `Active`. | Person & Identity | Account eligibility is consumed as a post-activation precondition. | L2-A02 — approved 2026-08-20 sequencing decision |
| Every authentication request references one canonical Identity. | Authentication | The Authentication Attempt and resulting session remain bound to the resolved Identity. | L2-A03 — Authentication Request Business Rules |
| Authentication Policy evaluation occurs before credential verification. | Authentication | No provider or credential path may bypass policy evaluation. | L2-A03 — AW2 and Authentication Request Business Rules |
| Authentication workflows remain provider-independent. | Authentication | Provider selection does not alter the canonical business flow. | L2-A03 — AW3 |
| Authentication succeeds only after the applicable identity and authentication requirements succeed. | Authentication | A session cannot be created from an unsuccessful authentication decision. | Frozen Phase 2 outcome; L2-A03 — Login Business Rules |
| A Credential must be available for authentication in `Active`. | Authentication | `Suspended`, `Expired` or otherwise invalid credentials cannot authenticate. | L2-A03 — Credential States; Login Outcomes |
| Required MFA must succeed before authentication succeeds. | Authentication | Failure of a required challenge prevents session creation. | L2-A03 — Login Workflow and MFA architecture |
| Authentication Sessions are created only after successful authentication. | Authentication | Session creation cannot precede the successful authentication decision. | L2-A03 — Login Business Rules |
| An Authentication Session belongs to exactly one Tenant and one authenticated Identity. | Authentication | The resulting session preserves Tenant isolation and canonical Identity association. | L2-A03 — Session Rules |
| Session Policy determines expiration and applicable session constraints. | Authentication | Policy is applied before the session becomes available. | L2-A03 — Session Creation Workflow; Session Policies |
| Refresh Tokens are generated only for authenticated sessions. | Authentication | Session continuity cannot be created for an unsuccessful attempt. | L2-A03 — Login Business Rules |
| Successful authentication never implies authorization. | Authorization & Policy | The scenario ends before any permission or access decision. | L2-A03 — Out of Scope and Login Business Rules; L2-A04 — AP-02 |
| Events are published only after their owning business transition succeeds. | Authentication / Event Infrastructure | Failure paths cannot publish success events. | L2-A03 — Event Principles and Publishing Rules |
| Every authentication workflow and significant authentication activity is auditable. | Authentication / Audit Infrastructure | Requests, policy evaluation, success, failure, MFA and session creation remain traceable. | L2-A03 — AW5 and Authentication Audit Architecture; L2-A07 |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State or Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Person | `Active` | Eligibility is validated; no state change occurs. | Remains `Active`. | Person & Identity |
| Identity | `Active` and valid for authentication | Canonical reference and eligibility are validated; no Identity lifecycle transition occurs. | Remains `Active`. | Person & Identity |
| User Account | Status permits authentication | Account status is validated; this scenario does not change the account lifecycle. | Remains in its existing eligible status. | Person & Identity |
| Credential | `Active` | Credential is verified; successful use does not change its lifecycle state. | Remains `Active`. | Authentication |
| MFA Enrollment | Active verified factor, when required | The factor satisfies a challenge; enrollment lifecycle is not changed by successful verification. | Remains available under its existing state. | Authentication |
| Authentication Attempt | Authentication request received | The attempt is received, processed and completed with a terminal outcome. | Processing completed; canonical events record the outcome. | Authentication |
| Authentication Session | No session exists for this attempt | The session is created and activated after successful authentication. | `Active`. | Authentication |
| Refresh Token | None for this authentication outcome | A token is generated only for the authenticated session under the Login Workflow. | Governed by the Refresh Token lifecycle. | Authentication |
| Authentication Context | Constructed during successful authentication | The immutable verification result is associated with or made available through the session. | Available for subsequent authorization; it owns no business lifecycle. | Authentication |
| Authorization Context / Decision | Not yet evaluated | No authorization state or decision is created by this scenario. | Subsequent evaluation remains with L2-A04. | Authorization & Policy |

The Draft does not assert whether `Created` is a durable intermediate Authentication Session state. It preserves the authoritative creation and activation events and requires the resulting session to be `Active`.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Publication Condition | Relevant Consumer / Effect |
|---|---|---|---|
| `AuthenticationAttemptReceived` | Authentication Attempt aggregate | The authentication request is accepted as an attempt. | Audit and authentication processing. |
| `AuthenticationStarted` | Authentication execution, as stated by L2-A03 | Authentication is initiated. | Audit, monitoring and notification consumers. |
| `AuthenticationAttemptProcessing` | Authentication Attempt aggregate | Authentication processing begins. | Audit and monitoring. |
| `AuthenticationChallenged` | Authentication execution, as stated by L2-A03 | Additional verification is required. | MFA and audit consumers. |
| `MFAChallengeStarted` | MFA Enrollment aggregate | An applicable MFA challenge begins. | Authentication Attempt and audit consumers. |
| `MFAChallengeSucceeded` | MFA Enrollment aggregate | The required MFA challenge succeeds. | Authentication Attempt and audit consumers. |
| `MFAChallengeFailed` | MFA Enrollment aggregate | The required MFA challenge fails. | Authentication Attempt and audit consumers. |
| `AuthenticationSucceeded` | Authentication execution, as stated by L2-A03 | Identity authentication succeeds. | Authorization, audit, monitoring and business-domain consumers. |
| `PlatformAuthenticationFailed` | Authentication execution, as stated by L2-A03 | Authentication reaches an unsuccessful terminal outcome. | Audit, monitoring and notification consumers. |
| `AuthSessionCreated` | Authentication Session aggregate | The Authentication Session is established. | Authorization, audit, monitoring and business-domain consumers. |
| `SessionActivated` | Authentication Session aggregate | The session becomes available for use. | Authorization and downstream session-awareness consumers. |
| `RefreshTokenGenerated` | Refresh Token aggregate | A Refresh Token is generated for the authenticated session. | Authentication Session, audit and monitoring consumers. |
| `AuthenticationAttemptCompleted` | Authentication Attempt aggregate | Attempt processing reaches a terminal outcome. | Audit and monitoring. |
| `AuthenticationCompleted` | Authentication execution, as stated by L2-A03 | Authentication reaches any terminal outcome. | Authorization, audit, monitoring and business domains; consumers must inspect the outcome rather than assume success. |

Event Infrastructure owns event publication and delivery infrastructure. It does not become the publisher or owner of Authentication business facts.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information or Outcome |
|---|---|---|---|
| Tenant Context | Platform & Tenant | Authentication | Resolved Tenant boundary for the request. |
| Person and Identity eligibility | Person & Identity | Authentication | Canonical Person and Identity references and current eligibility statuses. |
| User Account eligibility | Person & Identity | Authentication | User Account reference and the status used to determine authentication eligibility. |
| Authentication Session | Authentication | Authorization & Policy | Active trusted execution context associated with the Tenant and authenticated Identity. |
| Authentication Context | Authentication | Authorization & Policy | Immutable identity-verification result used for subsequent authorization evaluation. |
| Authentication audit input | Authentication | Audit Infrastructure | Authentication, Credential, Session, MFA and related events required for immutable audit evidence. |

These contracts and hand-offs are distinct from event publication. Authorization consumes the authenticated context but does not modify Authentication Session state.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Authentication must complete before authorization begins.
- Successful authentication does not grant permission to perform any business operation.
- Authorization decisions, roles, permissions, policies, Delegations and workspaces remain exclusively governed by L2-A04.
- No technical RBAC or IAM design is defined by this scenario.

## 15.2 Approval and Separation of Duty

- No normal authentication approval authority is defined by the frozen scenario semantics or L2-A03.
- No scenario-specific separation-of-duty requirement is defined upstream.
- This Draft therefore introduces neither an approval step nor a separation-of-duty rule.

## 15.3 Audit Obligations

The following activities must be auditable:

- Authentication request and attempt initiation.
- Tenant and Identity references used for the attempt.
- User Account eligibility result.
- Authentication Policy evaluation.
- Authentication Method and Authentication Provider used.
- Credential-verification outcome.
- MFA challenge and outcome where applicable.
- Authentication success or failure.
- Authentication Session creation and activation.
- Refresh Token generation.
- Correlation and terminal execution outcome.

Authentication produces the authentication audit information and canonical business events. Event Infrastructure distributes the events. Audit Infrastructure owns immutable audit evidence, actor attribution, retention, integrity and audit-evidence lifecycle.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when all of the following are true:

- The authentication request was processed within exactly one resolved Tenant.
- One canonical Identity reference was resolved and validated for authentication.
- The applicable User Account status permitted authentication.
- Authentication Policy evaluation completed before credential verification.
- The selected Authentication Method and Authentication Provider satisfied the applicable policy.
- Credential verification succeeded.
- Every required MFA challenge succeeded.
- `AuthenticationSucceeded` was published.
- An Authentication Session associated with the Tenant and authenticated Identity was created.
- The resulting Authentication Session is `Active`.
- `AuthSessionCreated` and `SessionActivated` were published by the Authentication Session.
- The Authentication Attempt reached its terminal completed outcome.
- `AuthenticationCompleted` was published.
- Required authentication audit information was produced.
- The active Authentication Session and authenticated context are available to L2-A04 for subsequent authorization.

Postconditions:

- Person, Identity and User Account ownership and lifecycle remain with L2-A02.
- Credential, MFA Enrollment, Authentication Session and Refresh Token ownership remain with L2-A03.
- No role, permission or authorization decision has been granted by this scenario.
- Failed or incomplete authentication does not expose an authenticated context for authorization.
- Business operations remain unavailable until L2-A04 produces the required authorization decision.

---

## Canonical R3B Remediation

This scenario consumes the Locked A02 and A03 R3B amendments. Earlier authentication blockers are resolved as follows:

- authentication-eligible User Account state is `Enabled`; legacy “Active User Account” wording means `Enabled`;
- Authentication Requestor is the logical initiating actor;
- generic execution events are published by Authentication Attempt;
- Authentication Session creation and activation are distinct;
- `AuthSessionCreated` records `Created`; `SessionActivated` records `Created → Active`;
- successful authentication does not grant authorization.

The canonical Tenant-scoped Person/Identity boundary remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No blocking issues remain after the Locked R3B upstream amendment(s).

No assumptions.
