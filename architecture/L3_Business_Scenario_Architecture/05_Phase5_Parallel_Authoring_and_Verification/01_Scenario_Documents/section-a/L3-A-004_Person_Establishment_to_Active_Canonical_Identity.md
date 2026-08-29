# Person Establishment to Active Canonical Identity

# 1. Scenario Metadata

**Scenario ID:** L3-A-004  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Person & Identity  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows, including the approved 2026-08-20 Person–Identity establishment and activation sequencing decision  
**Referenced By:** None yet  

---

# 2. Scenario Purpose

Establish one real individual as a canonical Person and associated active Identity within the applicable Tenant boundary so institutional domains can reference the individual without creating or owning duplicate identity records.

---

# 3. Business Outcome

One canonical Person record and its associated Identity are created, verified and activated so downstream domains may reference them, without transferring business-role ownership from those domains.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a real individual must be represented within a valid Tenant for institutional participation.

## 4.2 End Boundary

The scenario ends when the canonical Person and its associated Identity are both `Active`, their canonical activation events have been published, and their immutable references are available to downstream domains.

## 4.3 In Scope

- Tenant validation for identity establishment.
- Person validation and duplicate detection.
- Canonical Person creation.
- Associated Identity creation.
- Person and Identity verification under the applicable Tenant policy.
- Person and Identity activation.
- Publication of the canonical creation, verification, and activation events.
- Availability of canonical Person and Identity references.

## 4.4 Out of Scope

- User Account creation or enablement after activation.
- Credentials, authentication factors or MFA enrollment, Authentication Providers, and Authentication Sessions owned by L2-A03.
- Roles, Role Assignments, Delegations, authorization-policy lifecycle, and authorization-context assignment owned by L2-A04.
- Student, employee, applicant, parent, faculty, or other business-role lifecycles owned by their respective business domains.
- Duplicate Person reconciliation after a duplicate is confirmed.

## 4.5 Related Scenarios

| Relationship | Related Scenario | Reason |
|---|---|---|
| Related | L3-A-005 — Duplicate Person Identification to Canonical Identity Reconciliation | Governs the separate outcome when multiple Person records are confirmed to represent the same real individual. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Person & Identity / Section A | Owns the Person and Identity aggregates, their verification and lifecycle transitions, their canonical events, duplicate detection, and the final active canonical identity outcome. |

Participation by downstream domains after completion does not transfer ownership of the Person, Identity, their lifecycle, or their canonical references.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Person | The real individual represented by the canonical Person and associated Identity. | Subject of identity establishment; no approval authority is assigned by this scenario. |
| Authorized users | Review or override duplicate warnings where Tenant policy permits. | Performs a governed override; every override is audited. |

L2-A02 does not name the canonical role that initiates Person establishment or the role that grants verification approval when approval is required. The gap is recorded in Section 17.

---

# 7. Preconditions and Entry Criteria

- The Tenant has been resolved and is in the authoritative `Active` state.
- The identity-establishment request is within exactly one Tenant boundary.
- The information required by the applicable Tenant validation and verification policies is available for evaluation.

---

# 8. Trigger

A real individual must be represented within a Tenant for institutional participation.

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Validate that the identity establishment is occurring within a resolved `Active` Tenant. | Platform & Tenant; Person & Identity | A valid Tenant boundary is confirmed. |
| 2 | Validate the required Person information and perform duplicate detection under the applicable Tenant policy. | Person & Identity | The request is eligible to proceed without silently creating a duplicate Person. |
| 3 | Create the canonical Person, complete the authoritative `Draft → Created` transition, and publish `PersonCreated`. | Person & Identity — Person aggregate | A canonical Person with immutable identifiers exists in `Created`. |
| 4 | Create the Identity associated with that Person in `Pending` and publish `IdentityCreated`. | Person & Identity — Identity aggregate | One associated Identity exists in `Pending`. |
| 5 | Determine the applicable Tenant verification policy, validate the required evidence, and obtain approval where that policy requires it. | Person & Identity | The Person and Identity are eligible for verification. |
| 6 | Transition the Person `Created → Verified` and publish `PersonVerified`. | Person & Identity — Person aggregate | The Person is `Verified`. |
| 7 | Transition the Identity `Pending → Verified` and publish `IdentityVerified`. | Person & Identity — Identity aggregate | The associated Identity is `Verified`. |
| 8 | Transition the Person `Verified → Active` and publish `PersonActivated`. | Person & Identity — Person aggregate | The Person is `Active`. |
| 9 | Transition the Identity `Verified → Active` and publish `IdentityActivated`. | Person & Identity — Identity aggregate | The associated Identity is `Active`; the establishment outcome is complete. |
| 10 | Make the immutable Person and Identity references available for governed downstream use. | Person & Identity | Downstream domains may reference the canonical identity without acquiring ownership. |

The flow ends before User Account creation, credential establishment, authentication enablement, or authorization assignment.

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Configured verification path | Tenant policy requires a different supported verification level or administrative review. | Apply the configured evidence and approval path before completing the same Person and Identity verification and activation transitions. | The same active canonical identity outcome is reached. | Person & Identity |
| Advisory duplicate warning | Duplicate detection produces a warning but Tenant policy does not mandate blocking. | An authorized user reviews the warning and, where permitted, records an audited override before creation continues. | The scenario may continue toward the same outcome if the request remains eligible. | Person & Identity |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant cannot be resolved or is not `Active`. | Identity establishment cannot validly proceed. | Stop progression until the Tenant is restored to a valid `Active` state under L2-A01. | Platform & Tenant |
| Duplicate confidence or Tenant policy blocks creation. | A new canonical Person cannot be created through this scenario. | Reject the creation or direct a confirmed duplicate to the governed reconciliation outcome in L3-A-005. | Person & Identity |
| Required verification does not succeed or required approval is not granted. | Neither the Person nor the Identity may proceed to `Active`. | Progression remains stopped; L2-A02 does not define a scenario-specific correction or resubmission path for this establishment outcome. | Person & Identity |

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Every operation executes within one Tenant boundary, and downstream execution requires an `Active` Tenant. | Platform & Tenant | Identity establishment stops when the Tenant boundary is invalid or inactive. | L2-A01 — Tenant Foundation and Tenant Lifecycle |
| One canonical Person represents a real individual within a Tenant. | Person & Identity | Duplicate Person creation is prohibited as the normal outcome. | L2-A02 — Domain Design Principles; Business Rules |
| PersonID and IdentityID are immutable. | Person & Identity | The created canonical references remain stable throughout later lifecycles. | L2-A02 — Person Identifier; Identity Identifier; Business Rules |
| An Identity belongs to exactly one Person and cannot exist independently. | Person & Identity | Identity creation follows canonical Person creation and preserves the association. | L2-A02 — Identity Aggregate |
| Duplicate detection occurs before Person creation where applicable; automatic blocking depends on Tenant policy. | Person & Identity | Duplicate findings may warn, require review, block creation, or permit an audited override. | L2-A02 — Duplicate Detection; Identity Governance Business Rules |
| Verification requirements are configurable per Tenant. | Person & Identity | The applicable evidence and conditional approval path are determined before verification. | L2-A02 — Verification Workflow; Identity Verification |
| Person and associated Identity must both be verified before activation and both must become active for this outcome to complete. | Person & Identity | Partial activation does not satisfy scenario completion. | L2-A02 — approved 2026-08-20 sequencing decision |
| User Account creation occurs only after successful Person and Identity activation. | Person & Identity | Account creation is excluded from this scenario's end boundary. | L2-A02 — approved 2026-08-20 sequencing decision; User Account Service Contracts |
| Business domains reference canonical Person identifiers and do not own or directly modify Person or Identity. | Person & Identity | Downstream use does not transfer ownership. | L2-A02 — Canonical Ownership; Aggregate Ownership Rules |
| Every lifecycle transition and duplicate-warning override is audited. | Person & Identity | Creation, verification, activation, and any override remain traceable. | L2-A02 — Lifecycle Business Rules; Duplicate Detection |

---

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Person | `Draft` | Create and establish the canonical Person. | `Created` | Person & Identity | L2-A02 — Person Lifecycle; Person Creation Workflow |
| Person | `Created` | Complete the approved verification process. | `Verified` | Person & Identity | L2-A02 — Allowed State Transitions; Verification Workflow |
| Person | `Verified` | Complete governed activation. | `Active` | Person & Identity | L2-A02 — Allowed State Transitions; approved 2026-08-20 sequencing decision |
| Identity | `Pending` | Complete the approved Identity verification process. | `Verified` | Person & Identity | L2-A02 — Identity States; Identity Verification Workflow |
| Identity | `Verified` | Complete governed Identity activation. | `Active` | Person & Identity | L2-A02 — Identity States; approved 2026-08-20 sequencing decision |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| `PersonCreated` | Person aggregate | Downstream business domains where subscribed | Announces creation of the canonical Person. | L2-A02 — Person Events; Person Creation Workflow |
| `IdentityCreated` | Identity aggregate | Downstream identity-reference consumers where subscribed | Announces creation of the associated Identity. | L2-A02 — Identity Events; approved 2026-08-20 sequencing decision |
| `PersonVerified` | Person aggregate | Downstream business domains where subscribed | Announces completion of Person verification. | L2-A02 — Person Events; Verification Workflow |
| `IdentityVerified` | Identity aggregate | Downstream identity-reference consumers where subscribed | Announces completion of Identity verification. | L2-A02 — Identity Events; Identity Verification Workflow |
| `PersonActivated` | Person aggregate | Downstream business domains where subscribed | Announces that the Person is active and available for reference. | L2-A02 — Person Events; approved 2026-08-20 sequencing decision |
| `IdentityActivated` | Identity aggregate | Downstream identity-reference consumers where subscribed | Announces that the associated Identity is active. | L2-A02 — Identity Events; approved 2026-08-20 sequencing decision |

Each event retains exactly one canonical publisher. Event publication does not transfer ownership of the Person, Identity, or lifecycle state.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Person & Identity | Downstream business domains | Immutable canonical `PersonID` and `IdentityID` references | Permit downstream domains to associate their own business records with the canonical individual without duplicating identity ownership. | L2-A02 — Canonical Ownership; Domain Objectives; Integration Contracts |

The reference contract is distinct from the published lifecycle events.

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Override an advisory duplicate warning | Authorized users | Permitted only where Tenant policy allows; the override must be audited. | Person & Identity |
| Approve verification where the applicable policy requires approval | Not defined in L2-A02 | Approval authority is determined by authorization policies; L3 does not invent the missing role. | Person & Identity |

The canonical role authorized to initiate Person establishment is not defined in L2-A02.

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty requirement is defined by the authoritative L2 architecture. Duplicate-warning overrides remain subject to Tenant policy and mandatory auditability.

## 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Preserve complete identity auditability. | Traceable records of Person and Identity creation, verification, and activation lifecycle changes. | Person & Identity |
| Audit any duplicate-warning override. | The governed override record required by L2-A02. | Person & Identity |
| Preserve verification approval where approval is required by policy. | The approval outcome associated with the governed verification process. | Person & Identity |

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Exactly one canonical Person has been created for the individual through this scenario.
- The associated Identity belongs to that Person.
- The Person is in `Active`.
- The Identity is in `Active`.
- `PersonCreated`, `IdentityCreated`, `PersonVerified`, `IdentityVerified`, `PersonActivated`, and `IdentityActivated` have been published by their canonical publishers.
- Immutable Person and Identity references are available for governed downstream use.

## 16.2 Postconditions

- Downstream domains may reference the canonical Person and Identity without owning or duplicating them.
- Business-role lifecycles remain with their respective business-domain owners.
- A User Account may subsequently be created by A02 where login capability is required.
- Authentication capability remains subject to the separately owned A03 lifecycle.
- Roles and authorization remain subject to the separately owned A04 lifecycle.

---

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Locked L1 §3.4 states that every real individual has one Person record inside an Institute, while locked L2-A01, locked L2-A02, and frozen Phase 2/Stage 3 semantics establish Tenant-scoped Person and Identity ownership. | The Draft follows the frozen Tenant-scoped scenario semantics, but the higher-level wording conflict prevents final architecture approval or locking until the canonical uniqueness boundary is aligned. | Lead Architect must clarify or amend the locked L1 wording so the Person uniqueness scope is consistently Tenant or Institution across L1 and L2. |
| L2-A02 does not name the canonical role that initiates Person establishment or the role that grants verification approval when Tenant policy requires approval. | The scenario can describe the governed responsibility but cannot assign unsupported business authority. | Person & Identity architecture must name the applicable business role or explicitly delegate role determination to an existing authoritative policy without creating a new role in L3. |

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
