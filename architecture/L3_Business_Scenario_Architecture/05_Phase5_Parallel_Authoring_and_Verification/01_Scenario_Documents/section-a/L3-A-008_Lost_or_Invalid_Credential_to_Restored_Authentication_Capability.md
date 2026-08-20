# Lost or Invalid Credential to Restored Authentication Capability

# 1. Scenario Metadata

**Scenario ID:** L3-A-008  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authentication  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows, including the approved 2026-08-20 sequencing decision; L2-A03 — Authentication Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** L3-A-007 — Eligible Identity to Authenticated Session Availability

---

# 2. Scenario Purpose

Restore authentication capability for a legitimate user who can no longer authenticate with the current Credential, while preserving canonical Identity ownership, applying the governed recovery method and preventing continued authentication with the previous Credential.

---

# 3. Business Outcome

Identity is verified through the governed recovery method and valid authentication capability is restored, with previous credentials invalidated.

The authoritative L2 sources do not currently define the exact Credential transition and canonical event sequence required to complete this outcome. The scenario is therefore classified `BLOCKED`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a legitimate user can no longer authenticate using the current Credential.

## 4.2 End Boundary

The intended end boundary is reached when:

- the canonical Identity has been verified through the applicable governed recovery method;
- a replacement Credential is valid for authentication;
- every previous Credential covered by the recovery decision has been invalidated;
- the required canonical Credential and recovery events have been published; and
- the restored authentication capability is available for a later authentication request under L3-A-007.

Locked L2-A03 does not define the exact previous-Credential terminal state or the canonical recovery-completion event. Consequently, this boundary cannot yet be specified as an executable L3 outcome.

## 4.3 In Scope

- Tenant and canonical Identity-reference resolution.
- User Account eligibility validation through L2-A02 contracts.
- Evaluation of the applicable Credential and recovery policies.
- Selection of a governed recovery method.
- Identity verification through that recovery method.
- Creation of a replacement Credential.
- Validation of the replacement Credential against the applicable Credential Policy.
- Activation required to make the replacement Credential valid for authentication.
- Invalidation of previous Credentials.
- Authentication-session consequences where explicitly required by Authentication Policy.
- Canonical Credential and recovery event publication.
- Immutable audit evidence for the recovery operation.

## 4.4 Out of Scope

- Person or Identity creation, verification lifecycle or activation.
- User Account creation or lifecycle change.
- Normal authentication and Authentication Session creation after recovery.
- MFA Enrollment as a new active factor.
- Authentication Provider onboarding or recovery.
- Role, permission or authorization decisions.
- User Account unlocking unless independently required under the L2-A02 account lifecycle.
- Technical recovery codes, implementation procedures, APIs, database structures, UI design or infrastructure.
- Inventing an unapproved Credential state, transition, event or publisher to complete the recovery workflow.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Downstream | L3-A-007 — Eligible Identity to Authenticated Session Availability | Normal authentication resumes only after Credential replacement and invalidation have completed. A008 does not itself create an authenticated session. |
| Related | L3-A-009 — Multi-Factor Enrollment to Active Verified Authentication Factor | Governs enrollment and activation of an additional authentication factor. A008 may consume a governed factor as a recovery method but does not own new-factor enrollment. |
| Supporting | L3-A-055 — Institutional Authentication Provider Onboarding to Governed Provider Availability | Provider lifecycle and availability remain separate from recovery of an individual Credential. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Authentication / Section A | Primary owner. Owns Credential lifecycle, recovery eligibility, Credential Policy, Authentication Method, Credential events and restoration of authentication capability. |
| Platform & Tenant / Section A | Supplies the resolved Tenant context required for policy-scoped recovery. |
| Person & Identity / Section A | Supplies canonical Person, Identity and User Account references, eligible account status and verified recovery-contact references without owning the Credential lifecycle. |
| Event Infrastructure / Section A | Owns event-publication, routing and delivery infrastructure but does not own Authentication business facts. |
| Audit Infrastructure / Section A | Owns immutable audit evidence and its lifecycle for recovery activities and Credential changes. |

The frozen Participating Domains value remains exactly `A`.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Participation / Authority |
|---|---|---|
| Legitimate user | Is the subject of recovery and supplies the information required by the governed recovery method. | Supported by the frozen trigger. No approval or override authority is implied. |

L2-A03 explicitly records that its canonical Actors section is absent. It does not name the role that accepts a recovery request, verifies recovery evidence or authorizes Administrative Recovery.

L2-A02 names a User Account administrator and lists Administrative Recovery as a supported mechanism, but it does not assign that actor a canonical A03 Credential-recovery decision or approval authority. No such authority is invented here.

---

# 7. Preconditions and Entry Criteria

- The recovery request can be associated with exactly one resolved Tenant.
- A canonical Person and associated Identity exist and are both `Active`.
- A User Account exists where login capability is required.
- The User Account status permits Credential recovery independently of any separate account restriction.
- The current Credential is lost, unavailable or invalid for authentication.
- The current Credential belongs to the referenced Identity and Tenant.
- An applicable Credential Policy and recovery policy can be determined.
- A governed Recovery Authentication method is available.
- The evidence required by the selected recovery method is available for verification.
- Recovery has not yet created an authenticated session or granted authorization.

---

# 8. Trigger

A legitimate user can no longer authenticate using the current credential.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Accept the Credential-recovery request within a resolved Tenant context. | Authentication; Platform & Tenant | Recovery processing is associated with exactly one Tenant. |
| 2 | Resolve the canonical Identity and User Account references through L2-A02 contracts. | Person & Identity; Authentication | The recovery request is associated with the correct canonical Identity. |
| 3 | Confirm that the recovery concern relates to a lost or invalid Credential rather than Person, Identity or User Account establishment. | Authentication; Person & Identity | The request remains within the A008 boundary. |
| 4 | Evaluate the applicable Credential Policy and recovery requirements. | Authentication — Authentication Policy | The eligible recovery methods and replacement constraints are determined. |
| 5 | Select a governed Recovery Authentication method. | Authentication | A policy-permitted recovery path is selected. |
| 6 | Verify the Identity using the selected governed recovery method. | Authentication; Person & Identity reference provider | The identity-recovery requirement succeeds without changing the Identity lifecycle. |
| 7 | Create the replacement Credential and validate it against the applicable Credential Policy. | Authentication — Credential aggregate | A replacement Credential exists and satisfies the applicable policy. |
| 8 | Complete the transition required to make the replacement Credential valid for authentication. | Authentication — Credential aggregate | The required result is a replacement Credential in `Active`; however, the Password Reset workflow does not define this activation step or event. |
| 9 | Immediately invalidate the previous Credential or Credentials governed by the recovery decision. | Authentication — Credential aggregate | Continued authentication with previous Credentials is prevented; however, L2-A03 does not define whether reset causes rotation, revocation, expiration or another canonical transition. |
| 10 | Publish the canonical Credential-recovery events after the successful state changes. | Authentication — Credential aggregate | L2-A03 requires a “Credential Reset Event” but does not define its canonical event name or publisher contract. |
| 11 | Apply any existing-session consequence required by Authentication Policy. | Authentication — Authentication Session | L2-A03 permits existing sessions to be terminated according to policy but does not define the recovery-specific decision or event sequence. |
| 12 | Produce immutable audit evidence for recovery verification, Credential replacement, previous-Credential invalidation and the final outcome. | Authentication; Audit Infrastructure | The recovery activity is traceable. |
| 13 | Make the restored authentication capability available for a new authentication request. | Authentication | Normal authentication may subsequently begin under L3-A-007; no session is created by A008. |

Steps 8–10 cannot be finalized without upstream lifecycle and event decisions. They are retained to expose the exact architectural blocker rather than silently selecting unsupported states or events.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Verified mobile recovery | Tenant policy permits recovery through a verified mobile number. | Verify the Identity using the governed verified-mobile recovery method before Credential replacement. | Recovery proceeds only if verification succeeds. | Authentication; Person & Identity reference provider |
| Verified email recovery | Tenant policy permits recovery through a verified email address. | Verify the Identity using the governed verified-email recovery method before Credential replacement. | Recovery proceeds only if verification succeeds. | Authentication; Person & Identity reference provider |
| Administrative Recovery | Tenant policy permits an administrative recovery method. | Apply the governed administrative recovery method and its required authorization. | L2-A02 identifies the mechanism, but the deciding role and approval contract are not canonically assigned to A03. | Authentication; Person & Identity |
| Recovery Authentication method | A non-password recovery method is selected by policy. | Execute the method through the common Authentication Method model. | L2-A03 supports `Recovery Authentication`, but does not define a credential-generic replacement workflow. | Authentication |
| Existing sessions retained | Authentication Policy does not require termination of existing sessions. | Complete Credential recovery without changing session state. | Authentication capability is restored; existing session state remains unchanged. | Authentication |
| Existing sessions affected | Authentication Policy requires existing sessions to terminate. | Apply the policy-governed session consequence. | The exact recovery-specific termination decision and event sequence are not defined upstream. | Authentication |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant cannot be resolved. | Recovery cannot proceed within a valid Tenant boundary. | Reject the request without changing Credential state. | Platform & Tenant; Authentication |
| Canonical Identity cannot be resolved. | Recovery cannot establish that the request concerns a valid Identity. | Reject the request; Identity correction remains governed by L2-A02. | Person & Identity; Authentication |
| User Account status prevents recovery. | Credential recovery cannot restore usable authentication capability. | Stop recovery until the account concern is resolved under the authoritative User Account lifecycle. | Person & Identity |
| Recovery method is not permitted by policy. | Identity verification cannot proceed through that method. | Select another policy-permitted method where one exists; otherwise reject the recovery request. | Authentication |
| Recovery verification fails. | The requester is not established as the legitimate Identity owner through the governed method. | Reject the request, preserve existing Credential states and record the failed recovery outcome. | Authentication |
| Replacement Credential fails policy validation. | The replacement cannot become valid for authentication. | Do not make it available for authentication; reevaluation requires a later valid recovery attempt. | Authentication |
| Replacement is created but activation is incomplete. | Authentication capability remains unavailable. | Do not treat recovery as completed. The authoritative recovery-to-`Active` transition is missing from L2-A03. | Authentication |
| Previous Credential invalidation is incomplete. | More than one Credential may remain capable of authentication contrary to the frozen outcome. | Do not treat recovery as completed. The required canonical invalidation transition must be resolved upstream. | Authentication |
| Required recovery event cannot be identified. | The completion fact cannot be published through a canonical contract. | Do not invent or rename an event; obtain an upstream L2-A03 decision. | Authentication |

No technical retry, rollback or implementation recovery mechanism is defined by this scenario.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Credential lifecycle is owned exclusively by L2-A03. | Authentication | Person & Identity may supply references and recovery evidence but cannot change Credential state. | L2-A02 — INT-301; L2-A03 — Canonical Ownership |
| Credentials are associated with one canonical Identity and preserve Tenant isolation. | Authentication | Recovery must resolve the correct Identity and Tenant before replacement. | L2-A03 — Credential Aggregate and Multi-Tenant Authentication |
| Identity recovery must complete before Credential replacement. | Authentication | No replacement Credential may be created from an unsuccessful recovery verification. | L2-A03 — Password Reset Business Rules |
| Password Reset never modifies Identity information. | Person & Identity; Authentication | Recovery uses the Identity reference without changing the Identity lifecycle. | L2-A03 — Password Reset Business Rules |
| Recovery behaviour is governed by Credential and Authentication Policies. | Authentication | The method and replacement constraints must be policy-derived. | L2-A03 — Advanced Authentication Workflows; Credential Policies |
| Recovery Authentication is a supported Authentication Method. | Authentication | Recovery may use the common method model without becoming provider-specific. | L2-A03 — Supported Authentication Methods |
| Previous Credentials are immediately invalidated after successful replacement. | Authentication | The scenario is incomplete while a previous Credential remains usable. | Frozen Phase 2 outcome; L2-A03 — Password Reset Business Rules |
| A Credential in `Active` is valid for authentication. | Authentication | Restored authentication capability requires an eligible replacement Credential in `Active`. | L2-A03 — Credential State Definitions |
| Revoked Credentials cannot authenticate. | Authentication | `Revoked` is one possible invalid state, but the reset workflow does not select it as the canonical outcome. | L2-A03 — Credential Aggregate invariants |
| Expired Credentials cannot reactivate. | Authentication | Recovery cannot restore an expired Credential through reactivation. | L2-A03 — Credential Aggregate invariants |
| Credential rotation creates a later version and invalidates the prior version. | Authentication | Rotation may satisfy part of the frozen outcome, but Password Reset does not explicitly identify reset as rotation. | L2-A03 — Credential State Transition Matrix |
| Existing sessions may terminate according to Authentication Policy. | Authentication | Session consequences are conditional and cannot be assumed. | L2-A03 — Password Reset Business Rules |
| Every Password Reset and significant Credential activity must be audited. | Authentication; Audit Infrastructure | Recovery verification, replacement and invalidation require immutable evidence. | L2-A03 — Authentication Audit Architecture; L2-A07 |
| Credentials must not be exposed through events. | Authentication | Recovery events may identify the business fact but cannot expose the Credential value. | L2-A02 — Security Business Rules; L2-A03 event contracts |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Required Scenario Impact | Intended Result | Canonical Owner |
|---|---|---|---|---|
| Person | `Active` | Used only to confirm eligibility; no transition. | Remains `Active`. | Person & Identity |
| Identity | `Active` | Verified through the governed recovery method without changing lifecycle state. | Remains `Active`. | Person & Identity |
| User Account | Existing and eligible for recovery | Supplies the stable account and eligibility reference; no account transition is defined by A008. | Existing eligible state retained. | Person & Identity |
| Previous Credential | Lost, unavailable or otherwise invalid for the user’s authentication need | Must become incapable of authentication immediately after successful replacement. | Exact resulting canonical state is unresolved. | Authentication |
| Replacement Credential | Does not yet exist | Created, policy-validated and made valid for authentication. | Intended result is `Active`; reset-specific activation transition is absent from L2-A03. | Authentication |
| Authentication Session | May or may not exist | May terminate only when required by Authentication Policy. | Conditional outcome unresolved for this workflow. | Authentication |
| Authentication capability | Unavailable through the current Credential | Restored through a valid replacement Credential. | Available for a later A007 authentication request. | Authentication |

The following canonical Credential states remain unchanged:

- `Pending Activation`
- `Active`
- `Suspended`
- `Expired`
- `Revoked`
- `Archived`

No `Inactive` or `Deactivated` Credential state is introduced. The L2-A03 Password Reset instruction to “Deactivate Previous Credential” cannot be mapped to a canonical state without an upstream decision.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Source-Supported Credential Events

| Event | Canonical Publisher | Relevance to A008 |
|---|---|---|
| `CredentialCreated` | Credential aggregate | Supports the explicit creation of the replacement Credential. |
| `CredentialActivated` | Credential aggregate | Would represent the replacement Credential becoming available for authentication, but the Password Reset workflow omits the corresponding activation step. |
| `CredentialRotated` | Credential aggregate | Represents replacement through rotation, but L2-A03 does not state that Password Reset is canonically a rotation. |
| `CredentialRevoked` | Credential aggregate | Represents permanent invalidation, but L2-A03 does not require reset to transition the previous Credential to `Revoked`. |
| `PlatformCredentialExpired` | Credential aggregate | Represents validity ending through expiration, but reset is not defined as an expiration action. |
| `CredentialArchived` | Credential aggregate | Represents historical archival after `Expired` or `Revoked`; it is not identified as the immediate reset outcome. |

L2-A03’s Password Reset Workflow instructs publication of a “Credential Reset Event,” but no event with that exact canonical name appears in its Credential event catalogue or Aggregate-to-Event Matrix.

L2-A02 lists `PasswordResetRequested` and `PasswordResetCompleted` in later security prose. The INT-301 ownership amendment makes later A02 Credential event ownership non-authoritative where it conflicts with A03. This Draft therefore does not adopt those events or assign their publisher.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant Context | Platform & Tenant | Authentication | Resolved Tenant boundary for recovery. |
| Canonical Identity Reference | Person & Identity | Authentication | Identity associated with the User Account and Credential. |
| User Account eligibility | Person & Identity | Authentication | Account reference, login enablement and eligibility context. |
| Verified recovery-contact reference | Person & Identity | Authentication | Verified mobile or email reference when used by the governed recovery method. |
| Restored authentication capability | Authentication | Later A007 authentication workflow | Availability of a valid replacement Credential for a new authentication request. |
| Recovery audit input | Authentication | Audit Infrastructure | Recovery decision, Credential changes, actor attribution, correlation and outcome. |

These contracts are separate from published business events.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Credential recovery does not grant access to any business operation.
- A successfully recovered Credential must still be used through L3-A-007 to establish an authenticated session.
- Subsequent permission evaluation remains governed exclusively by L2-A04.
- No technical RBAC, IAM role or permission is defined here.

## 15.2 Approval

- Normal verified-mobile, verified-email and Recovery Authentication paths do not have an upstream approval role defined.
- L2-A02 identifies Administrative Recovery and states that administrative operations require authorization, but it does not name the canonical A03 recovery decision-maker or approval contract.
- This Draft does not invent an administrative approval authority.

## 15.3 Audit

The following must be auditable:

- Recovery request.
- Tenant, Identity and User Account references.
- Selected recovery method.
- Recovery-policy evaluation.
- Recovery-verification success or failure.
- Replacement Credential creation.
- Replacement Credential activation.
- Previous-Credential invalidation.
- Conditional session termination.
- Final recovery outcome.
- Actor, timestamp and correlation information.

Authentication owns the recovery and Credential business facts. Event Infrastructure owns their governed distribution. Audit Infrastructure owns immutable audit evidence, integrity, retention and evidence lifecycle.

---

# 16. Completion Criteria and Postconditions

The intended scenario completion criteria are:

- The request was processed within exactly one resolved Tenant.
- The canonical Identity and User Account were resolved.
- A policy-permitted recovery method was selected.
- Identity recovery succeeded before Credential replacement.
- The replacement Credential satisfied the applicable Credential Policy.
- The replacement Credential became valid for authentication in `Active`.
- Every previous Credential covered by the recovery decision was made incapable of authentication.
- Required session consequences were applied according to Authentication Policy.
- Canonical Credential-recovery events were published.
- Immutable audit evidence was produced.
- Restored authentication capability became available for a later A007 authentication request.
- No Authentication Session or authorization decision was created by A008 itself.

These criteria cannot presently be validated because locked L2-A03 does not define:

- the reset-specific transition that activates the replacement Credential;
- the canonical invalidation transition for the previous Credential;
- the exact recovery-completion event and publisher; or
- whether Password Reset is the authoritative workflow for all Credential types covered by the frozen A008 scenario.

Accordingly, this Draft cannot claim the canonical business outcome has been fully specified.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue / Deferred Question | Precise Scenario Impact | Required Resolution |
|---|---|---|---|---|
| A008-BLOCKER-01 | Blocking | The frozen A008 scenario concerns a lost or invalid `Credential`, but locked L2-A03 defines only a concrete `Password Reset Workflow`. | The Draft cannot determine whether password, passkey, federation, API, recovery-code and other Credential types use one common recovery lifecycle or distinct canonical variants. | L2-A03 must define the credential-generic recovery boundary or explicitly restrict A008 to a supported Credential class without changing the frozen title. |
| A008-BLOCKER-02 | Blocking | Password Reset creates a new Credential but does not define its transition from `Pending Activation` to `Active` or publish `CredentialActivated`. | The exact restoration transition required for valid authentication capability cannot be specified. | L2-A03 must define the reset-specific replacement-Credential activation sequence. |
| A008-BLOCKER-03 | Blocking | Password Reset instructs “Deactivate Previous Credential,” but `Deactivated` or `Inactive` is not a canonical Credential state. Available authoritative actions are rotation, suspension, expiration and revocation. | The previous-Credential state, transition and canonical event cannot be selected without invention. | L2-A03 must identify the exact invalidation action, resulting state and event. |
| A008-BLOCKER-04 | Blocking | Password Reset instructs publication of a “Credential Reset Event,” but this is not an exact canonical event name in L2-A03. | Section 14 cannot identify a canonical recovery-completion event and publisher. | L2-A03 must add or select the canonical event name and its sole publisher. |
| A008-ISSUE-01 | Non-blocking | L2-A02 lists `PasswordResetRequested` and `PasswordResetCompleted`, but INT-301 assigns Credential events and lifecycle to A03 and supersedes conflicting later A02 wording. | The A02 event names cannot safely fill A03’s missing recovery event contract. | Align A02 security-event references with the authoritative A03 event catalogue. |
| A008-ISSUE-02 | Non-blocking | L2-A03 has no canonical Actors section and does not name who evaluates recovery evidence or authorizes Administrative Recovery. | Business responsibility and approval authority cannot be assigned beyond the frozen “legitimate user” subject and authoritative domains. | Define the recovery actor and any administrative decision authority upstream. |
| A008-ISSUE-03 | Non-blocking | L2-A03 states that existing sessions may terminate according to Authentication Policy but does not define the recovery-specific decision, resulting session state or event sequence. | The Draft cannot state whether recovery preserves, terminates or revokes existing sessions in a given policy outcome. | Define the recovery-session policy outcomes and canonical events. |
| A008-ISSUE-04 | Non-blocking | Locked L1 §3.4 uses an Institute-scoped Person boundary while locked L2-A01, L2-A02 and L2-A03 use Tenant-scoped identity and Credential boundaries. | The Draft follows the L2 Tenant workflow, but the enterprise scope of the recovered Identity remains inconsistent across architecture levels. | Lead Architect alignment is required before Architecture Approval or locking. |

No unsupported actor, approval authority, Credential state, transition, event, publisher or session consequence is assumed by this Draft.