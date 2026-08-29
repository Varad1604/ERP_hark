# Multi-Factor Enrollment to Active Verified Authentication Factor

# 1. Scenario Metadata

**Scenario ID:** L3-A-009  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authentication  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows, including the approved 2026-08-20 sequencing decision; L2-A03 — Authentication Workflows; L2-A04 — Authorization & Policy Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** L3-A-007 — Eligible Identity to Authenticated Session Availability; authentication workflows requiring multi-factor verification

---

# 2. Scenario Purpose

Enroll an additional authentication factor for an authenticated Identity, validate it under the applicable MFA Enrollment Policy, verify factor ownership and activate the factor so it can satisfy subsequent multi-factor authentication challenges.

---

# 3. Business Outcome

The factor is registered, validated against enrollment policy, verified and activated, becoming available to satisfy multi-factor challenges, with immutable enrollment history preserved.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authenticated Identity enrols an additional authentication factor, either by user choice or to satisfy an applicable Authentication Policy.

## 4.2 End Boundary

The scenario ends when the MFA Enrollment has progressed through governed registration and verification to `Active`, `MFAEnrollmentStarted`, `MFAEnrollmentVerified` and `MFAActivated` have been published by the MFA Enrollment aggregate, and immutable enrollment history is available for audit.

## 4.3 In Scope

- Validation of an existing authenticated session.
- Tenant and canonical Identity-reference resolution.
- Evaluation of the applicable MFA Enrollment Policy.
- Selection of a policy-permitted authentication factor and method.
- Creation of the MFA Enrollment record.
- Registration of the authentication factor.
- Placement of the enrollment in `Pending Verification`.
- Verification of factor ownership and applicable method requirements.
- Transition to `Verified`.
- Validation of activation eligibility.
- Transition to `Active`.
- Publication of canonical MFA enrollment, verification and activation events.
- Preservation of immutable enrollment history.
- Availability of the active verified factor for subsequent MFA challenges.
- Audit evidence for enrollment, verification and activation.

## 4.4 Out of Scope

- Person, Identity or User Account creation or activation.
- Initial authentication and Authentication Session creation.
- Credential recovery or replacement.
- Recovery of a lost or inaccessible authentication factor.
- Replacement, suspension, reactivation, disablement or archival of an existing MFA Enrollment.
- Execution of an MFA challenge during login.
- Authentication Session renewal or termination.
- Authentication Provider onboarding.
- Role, permission or authorization decisions.
- Factor secrets, codes, implementation details or storage mechanisms.
- APIs, database structures, UI design, technical RBAC, infrastructure and deployment.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Upstream | L3-A-007 — Eligible Identity to Authenticated Session Availability | A009 requires an authenticated session before enrollment can begin. |
| Related recovery | L3-A-008 — Lost or Invalid Credential to Restored Authentication Capability | Owns Credential recovery. A009 does not replace or restore the primary Credential. |
| Downstream consumption | L3-A-007 — Eligible Identity to Authenticated Session Availability | A later login may consume the active factor when the applicable Authentication Policy requires MFA. |
| Related provider governance | L3-A-055 — Institutional Authentication Provider Onboarding to Governed Provider Availability | Provider lifecycle remains separate from enrollment of an individual factor. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authentication / Section A | Primary owner. Owns MFA Enrollment, factor registration, verification state, activation eligibility, replacement history, Authentication Policy and all canonical MFA events. |
| Platform & Tenant / Section A | Supplies the resolved Tenant context and Tenant-scoped configuration required for enrollment-policy evaluation. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references without owning authentication factors or their lifecycle. |
| Authorization & Policy / Section A | May consume the resulting authentication-assurance information but does not create, verify, activate or modify the MFA Enrollment. |
| Event Infrastructure / Section A | Owns governed event publication, routing and delivery infrastructure without owning MFA business facts. |
| Audit Infrastructure / Section A | Owns immutable audit evidence, integrity, retention and evidence lifecycle for enrollment activity. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Authenticated User | Initiates enrollment of an additional authentication factor and completes the factor-ownership verification required by the selected method. | Explicitly identified by the L2-A03 MFA Enrollment Workflow. Enrollment remains subject to policy and does not grant approval authority. |
| Authenticated Identity | Canonical Identity associated with the active session and MFA Enrollment. | Business subject of the enrollment; does not own the factor lifecycle. |

L2-A03’s document crosswalk records its general Actors section as absent, but its MFA Enrollment Workflow explicitly names `Authenticated User`. No additional administrator, approver or security role is invented.

---

# 7. Preconditions and Entry Criteria

- The enrollment request belongs to exactly one resolved Tenant.
- The Tenant is eligible for authentication execution.
- The Person and associated Identity are both `Active`.
- A User Account exists where login capability is required.
- The Identity has successfully completed authentication under L3-A-007.
- An Authentication Session associated with exactly one Tenant and the Identity is `Active`.
- An applicable MFA Enrollment Policy can be resolved.
- The proposed authentication factor and method are supported and permitted by that policy.
- The information required to register and verify factor ownership is available.
- Enrollment is not being used to bypass an existing MFA challenge or recovery requirement.

---

# 8. Trigger

An authenticated identity enrols an additional authentication factor, by user choice or to satisfy an applicable authentication policy.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the additional-factor enrollment request from the Authenticated User. | Authenticated User; Authentication | A governed MFA Enrollment request begins. |
| 2 | Validate that the request is associated with an `Active` Authentication Session within exactly one Tenant. | Authentication — Authentication Session; Platform & Tenant | The requester has the authenticated context required for enrollment. |
| 3 | Resolve the canonical Identity associated with the session without changing Person, Identity or User Account state. | Person & Identity; Authentication | The MFA Enrollment is associated with one canonical Identity. |
| 4 | Determine whether enrollment is user-chosen or required by an applicable Authentication Policy. | Authentication — Authentication Policy | The policy basis for enrollment is established. |
| 5 | Evaluate the applicable MFA Enrollment Policy, including permitted factor, method, verification and activation requirements. | Authentication — Authentication Policy | Enrollment requirements are determined before factor registration. |
| 6 | Select a policy-permitted authentication factor and Authentication Method. | Authentication | An eligible factor-registration path is selected. |
| 7 | Create the MFA Enrollment for exactly one Tenant and one Identity, register the factor and publish `MFAEnrollmentStarted`. | Authentication — MFA Enrollment aggregate | The enrollment exists and has entered the governed verification lifecycle. |
| 8 | Place the enrollment in `Pending Verification`. | Authentication — MFA Enrollment aggregate | The factor remains unavailable for authentication while ownership verification is pending. |
| 9 | Verify factor ownership and validate the applicable method and policy requirements. | Authenticated User; Authentication — MFA Enrollment aggregate | The factor satisfies the required verification conditions. |
| 10 | Complete the `Pending Verification → Verified` transition and publish `MFAEnrollmentVerified`. | Authentication — MFA Enrollment aggregate | The authentication factor is `Verified` but not yet available for use. |
| 11 | Confirm that the canonical Identity remains valid and that activation satisfies the applicable Authentication Policy. | Authentication; Person & Identity reference provider | Activation eligibility is established. |
| 12 | Complete the `Verified → Active` transition and publish `MFAActivated`. | Authentication — MFA Enrollment aggregate | The active verified factor becomes available for authentication. |
| 13 | Preserve immutable enrollment and verification history. | Authentication; Audit Infrastructure | Enrollment, verification and activation remain traceable. |
| 14 | Make the active factor and resulting assurance information available for later policy-driven MFA challenges and authorized assurance consumption. | Authentication | The frozen business outcome is complete without executing a challenge or granting authorization. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| User-choice enrollment | The applicable policy permits an additional optional factor. | Evaluate the same enrollment, verification and activation requirements. | The factor becomes `Active` only after verification and policy validation. | Authentication |
| Policy-required enrollment | Authentication Policy requires an additional factor. | Apply the policy-mandated factor and verification requirements. | The same active verified factor outcome is required. | Authentication |
| Multiple factors already exist | The Identity already has one or more active factors. | Create a distinct MFA Enrollment where policy permits multiple simultaneous factors. | Existing factors remain unchanged and the additional factor follows its own lifecycle. | Authentication |
| Verification remains pending | Registration succeeds but factor-ownership verification has not completed. | Retain the enrollment outside `Active`; do not publish verification or activation success events. | The scenario remains incomplete. | Authentication |
| Different permitted method | More than one authentication-factor method is permitted. | Select a method according to the applicable MFA Enrollment Policy. | The same canonical enrollment lifecycle and final outcome apply. | Authentication |

Factor replacement and factor recovery do not use these alternate paths; they remain outside the A009 boundary.

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant cannot be resolved. | MFA Enrollment cannot proceed within a valid Tenant boundary. | Reject the request without creating an active factor. | Platform & Tenant; Authentication |
| Authentication Session is absent or not `Active`. | The precondition for enrollment is not satisfied. | Stop enrollment. A new valid authenticated session must be established under L3-A-007 before a later enrollment request. | Authentication |
| Canonical Identity cannot be resolved or is not eligible. | The MFA Enrollment cannot be associated with a valid Identity. | Reject the request without changing Identity state. Identity correction remains governed by L2-A02. | Person & Identity; Authentication |
| Authentication Policy does not permit the proposed factor or method. | Registration cannot validly proceed. | Reject that factor or select another policy-permitted method where available. | Authentication |
| Factor registration fails. | No governed enrollment can progress to verification. | Do not publish verification or activation events and do not make the factor available. | Authentication |
| Factor-ownership verification fails. | The factor cannot become `Verified` or `Active`. | Stop progression beyond `Pending Verification`; retain the auditable unsuccessful outcome. | Authentication |
| Activation eligibility fails after verification. | The verified factor cannot become available for authentication. | Do not complete the `Verified → Active` transition or publish `MFAActivated`. | Authentication |
| Enrollment is interrupted while verification is pending. | The factor remains unavailable for authentication. | Preserve enrollment history. L2-A03 does not define a scenario-specific expiry or resumption outcome for pending enrollment. | Authentication |
| Event publication condition is not satisfied. | A success event cannot represent an incomplete state transition. | Do not publish the corresponding success event. | Authentication; Event Infrastructure |

No technical retry, transaction rollback or implementation recovery mechanism is defined.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| MFA Enrollment is owned exclusively by L2-A03. | Authentication | No other domain may register, verify, activate, replace or disable a factor. | L2-A02 — INT-301; L2-A03 — Canonical Ownership |
| Enrollment requires an authenticated session. | Authentication | An Identity without an active authenticated context cannot begin A009. | L2-A03 — MFA Enrollment Workflow Business Rules |
| Every MFA Enrollment belongs to one Tenant. | Authentication | Enrollment and policy evaluation cannot cross Tenant boundaries. | L2-A03 — MFA Enrollment Aggregate invariants |
| Every MFA Enrollment belongs to one Identity. | Authentication | The factor cannot be shared between canonical Identities. | L2-A03 — MFA Enrollment Aggregate invariants |
| Enrollment must reference a known Identity. | Authentication; Person & Identity | Registration cannot create or modify the Identity. | L2-A03 — MFA Enrollment validation rules |
| Enrollment must be validated under the applicable Authentication Policy. | Authentication | Factor, method, verification and activation requirements are policy-derived. | Frozen outcome; L2-A03 — MFA Workflow and MFA Policies |
| Authentication factors remain inactive until verified. | Authentication | Registration alone cannot make a factor available for authentication. | L2-A03 — MFA Enrollment Business Rules |
| Verification must satisfy policy and method requirements. | Authentication | Only a compliant factor may reach `Verified`. | L2-A03 — MFA Enrollment validation rules |
| `Verified → Active` requires a valid Identity and applicable policy. | Authentication | Activation is a separate governed lifecycle transition after verification. | L2-A03 — MFA Enrollment State Transition Matrix |
| An `Active` MFA Enrollment is available for authentication. | Authentication | Only the final active factor may satisfy later MFA challenges. | L2-A03 — MFA States |
| The enrolled factor is immutable for a given verified record. | Authentication | Changing a verified factor requires the separate replacement lifecycle. | L2-A03 — MFA Enrollment Aggregate invariants |
| Multiple authentication factors may exist simultaneously. | Authentication | Enrollment of an additional factor does not automatically replace existing factors. | L2-A03 — MFA Enrollment Business Rules |
| A disabled enrollment cannot contribute to authentication. | Authentication | Disabled factors cannot satisfy later challenges. | L2-A03 — MFA Enrollment Aggregate invariants |
| Enrollment state cannot bypass policy-based challenges. | Authentication | Activation makes the factor eligible; it does not itself satisfy a later challenge. | L2-A03 — MFA Enrollment business constraints |
| Enrollment history remains immutable. | Authentication; Audit Infrastructure | Registration, verification and activation evidence must be preserved. | Frozen outcome; L2-A03 — MFA Enrollment Business Rules |
| Events are published only after successful business transitions. | Authentication; Event Infrastructure | Failed verification or activation cannot publish success events. | L2-A03 — Event Principles and Publishing Rules |
| MFA Enrollment and MFA Verification must be audited. | Authentication; Audit Infrastructure | The entire enrollment outcome remains traceable. | L2-A03 — Authentication Audit Architecture; L2-A07 |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Person | `Active` | Referenced for eligibility; no transition. | Remains `Active`. | Person & Identity |
| Identity | `Active` and authenticated | Referenced by the MFA Enrollment; no Identity transition. | Remains `Active`. | Person & Identity |
| User Account | Authentication eligible | Supplies the stable account reference; no account transition. | Remains in its existing eligible state. | Person & Identity |
| Authentication Session | `Active` | Validated as the authenticated enrollment context; no session transition. | Remains `Active`, subject to its existing Session Policy. | Authentication |
| MFA Enrollment | No enrollment exists for the proposed additional factor | Registration begins the lifecycle, followed by verification and activation. | `Pending Verification → Verified → Active`. | Authentication |
| Authentication factor | Proposed and not yet available | Registered, ownership-verified and activated through the MFA Enrollment. | Available for later authentication through the `Active` enrollment. | Authentication |
| Authentication Policy | Applicable active policy | Evaluated without policy-state change. | Unchanged. | Authentication |
| Authorization | No enrollment-specific decision | No role, permission or authorization outcome is created. | Unchanged; assurance information may be consumed later. | Authorization & Policy |

L2-A03 additionally uses `Enrolled` as a current state in its transition matrix and lifecycle-ownership prose but omits it from the MFA state-definition table. This Draft therefore treats creation and registration as the action that places the enrollment in the explicitly defined `Pending Verification` state.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Canonical Publisher | Publication Condition | Business Effect / Consumer |
|---|---|---|---|
| `MFAEnrollmentStarted` | MFA Enrollment aggregate | The factor enrollment is initiated and registered for the Identity. | Audit, monitoring and notification consumers become aware of the enrollment. |
| `MFAEnrollmentVerified` | MFA Enrollment aggregate | Factor ownership and applicable verification requirements succeed and the enrollment reaches `Verified`. | Authentication, audit and monitoring consumers receive the verified-enrollment fact. |
| `MFAActivated` | MFA Enrollment aggregate | The `Verified → Active` transition succeeds. | The factor becomes available for Authentication Attempts; Authorization may consume the resulting assurance information. |

The following events are not part of A009 completion:

- `MFAChallengeStarted`
- `MFAChallengeSucceeded`
- `MFAChallengeFailed`
- `MFADisabled`

Challenge events belong to later authentication use. `MFADisabled` belongs to the separate disablement lifecycle.

L2-A03’s workflow wording “Publish MFA Enrolled Event” is not an exact canonical event name. This Draft uses the exact canonical events defined by the MFA Enrollment aggregate and MFA event catalogue.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant Context | Platform & Tenant | Authentication | Resolved Tenant boundary and Tenant-scoped configuration. |
| Identity and User Account references | Person & Identity | Authentication | Canonical Identity and authentication-eligibility context. |
| Active Authentication Session | Authentication Session | MFA Enrollment | Trusted authenticated context required to initiate enrollment. |
| Applicable MFA Enrollment Policy | Authentication Policy | MFA Enrollment | Permitted factor, method, verification and activation requirements. |
| Active verified factor availability | MFA Enrollment | Authentication Attempt | Factor availability for later policy-driven MFA challenges. |
| Authentication-assurance information | Authentication | Authorization & Policy | Assurance information that may be consumed without transferring MFA ownership. |
| Enrollment audit input | Authentication | Audit Infrastructure | Enrollment, verification, activation, actor and correlation facts. |

Synchronous contracts and hand-offs are separate from published events.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- An `Active` Authentication Session is required before enrollment begins.
- The session proves authentication but does not itself guarantee that every proposed factor is eligible.
- Authentication Policy determines factor-enrollment eligibility.
- Activation of a factor does not grant a role, permission or business access.
- Authorization remains exclusively governed by L2-A04.
- No technical RBAC or IAM design is introduced.

## 15.2 Approval and Separation of Duty

- L2-A03 does not define a separate approval requirement for normal user-choice or policy-required MFA Enrollment.
- No enrollment approver or separation-of-duty requirement is named upstream.
- This Draft therefore introduces no approval authority or SoD rule.
- Factor verification is proof of factor ownership and policy compliance; it is not a business approval.

## 15.3 Audit Obligations

The following must be auditable:

- Tenant and Identity references.
- Authenticated Session reference.
- Enrollment trigger: user choice or policy requirement.
- Authentication Policy used.
- Factor and Authentication Method classification without exposing the factor secret.
- Enrollment initiation.
- Factor-ownership verification outcome.
- Transition to `Verified`.
- Transition to `Active`.
- Published MFA events.
- Actor, timestamp, correlation and final outcome.

Authentication owns the MFA business activity and events. Event Infrastructure owns their governed distribution. Audit Infrastructure owns immutable audit evidence and its lifecycle.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when all of the following are true:

- Enrollment was initiated by an Authenticated User.
- The request was processed within exactly one Tenant.
- The active Authentication Session and canonical Identity were validated.
- The applicable MFA Enrollment Policy was evaluated.
- The selected factor and method were permitted by policy.
- An MFA Enrollment was created for exactly one Tenant and one Identity.
- `MFAEnrollmentStarted` was published by the MFA Enrollment aggregate.
- The enrollment entered `Pending Verification`.
- Factor ownership and applicable method requirements were successfully verified.
- The enrollment transitioned to `Verified`.
- `MFAEnrollmentVerified` was published by the MFA Enrollment aggregate.
- Activation eligibility was confirmed using the valid Identity and applicable policy.
- The enrollment transitioned from `Verified` to `Active`.
- `MFAActivated` was published by the MFA Enrollment aggregate.
- Immutable enrollment history and audit information were preserved.
- The factor became available for subsequent policy-driven MFA challenges.

Postconditions:

- The Person, Identity and User Account remain unchanged.
- The initiating Authentication Session remains governed by its existing Session Policy.
- Existing MFA Enrollments remain unchanged.
- No MFA challenge has yet been executed using the new factor.
- No Credential recovery, role assignment, permission or authorization decision has occurred.
- The active factor may subsequently contribute to Authentication Strength and authorization-assurance evaluation without transferring ownership from Authentication.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Type | Issue / Deferred Question | Precise Scenario Impact | Disposition |
|---|---|---|---|---|
| A009-ISSUE-01 | Upstream lifecycle inconsistency | L2-A03’s MFA Enrollment transition matrix begins with `Enrolled → Pending Verification`, and lifecycle-ownership prose includes `enrolled`, but the MFA state-definition table omits `Enrolled`. | The Draft cannot treat `Enrolled` as an independently defined canonical state. Registration is therefore described as placing the enrollment into the explicitly defined `Pending Verification` state. | Align the transition matrix and state-definition table before Architecture Approval or locking. |
| A009-ISSUE-02 | Upstream event wording inconsistency | The MFA Enrollment Workflow says “Publish MFA Enrolled Event,” but no event with that exact name exists in the canonical MFA event catalogue. | The Draft uses the exact aggregate events `MFAEnrollmentStarted`, `MFAEnrollmentVerified` and `MFAActivated` and does not invent `MFAEnrolled`. | Correct the workflow wording to the selected canonical event sequence. |
| A009-ISSUE-03 | Upstream command gap | L2-A03 defines `Verified → Active` and `MFAActivated`, but the MFA Enrollment aggregate’s Owned Commands table does not include an activation command. | The business transition and event are clear enough for L3, but L4 cannot identify the canonical activation command contract without clarification. | Add or identify the authoritative activation command before L4 specification. |
| A009-ISSUE-04 | Upstream actor documentation inconsistency | L2-A03’s crosswalk records Actors as absent, while the MFA Enrollment Workflow explicitly names `Authenticated User`. | The Draft uses only the scenario-specific actor explicitly present in the workflow and does not infer any administrator or approver. | Align the crosswalk with the workflow actor before Architecture Approval or locking. |
| A009-ISSUE-05 | Upstream boundary conflict | Locked L1 §3.4 states that one Person exists inside an Institute, while locked L2-A01, L2-A02 and L2-A03 use Tenant-scoped Identity, session and MFA Enrollment boundaries. | The Draft follows the authoritative L2 Tenant workflow, but the scope of the referenced canonical Person remains inconsistent across levels. | Lead Architect alignment is required before Architecture Approval or locking. |

No unsupported factor type, actor, approval authority, lifecycle state, transition, event, publisher, authorization rule or implementation mechanism is assumed by this Draft.