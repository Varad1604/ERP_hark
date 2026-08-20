# Institutional Authentication Provider Onboarding to Governed Provider Availability

# 1. Scenario Metadata

**Scenario ID:** L3-A-055  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Authentication  
**Participating Domains:** A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-20  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-A01 — Platform & Tenant Foundation; L2-A02 — Person & Identity Workflows; L2-A03 — Authentication Workflows; L2-A06 — Event Infrastructure Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** L3-A-007 and every provider-independent authentication flow

---

# 2. Scenario Purpose

Register, configure, validate, policy-scope and enable an institutional Authentication Provider so it becomes deterministically selectable with governed fallback and failover while canonical identity remains owned by Person & Identity.

---

# 3. Business Outcome

The provider is registered, configured, policy-scoped to the tenant and activated within the governed selection order with fallback and failover behaviour defined, becoming available for authentication without altering canonical identity ownership

Source assessment: `PASS WITH ISSUES`.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An institution requires its users to authenticate through an external or additional authentication provider.

## 4.2 End Boundary

The provider is registered for one Tenant, validated, enabled and available in deterministic policy selection with fallback behaviour, and its lifecycle and audit facts are preserved.

## 4.3 In Scope

- Tenant and provider-type validation.
- Provider registration and configuration.
- Uniform provider-contract validation.
- Authentication Method association.
- Tenant policy, priority, availability, fallback and failover rules.
- Provider enablement and publication of aggregate events.
- Audit evidence and later maintenance/recovery boundaries.

## 4.4 Out of Scope

- Creating or modifying Person, Identity or User Account.
- Credential and MFA enrollment.
- Executing a user login.
- Vendor-specific protocol, secret-storage, network, API or UI implementation.

## 4.5 Related Scenarios

| Relationship | Scenario | Boundary |
|---|---|---|
| Enables | L3-A-007 — Eligible Identity to Authenticated Session Availability | A007 selects an available provider; it does not onboard one. |
| Related | L3-A-055 provider maintenance lifecycle | Disablement, maintenance, recovery and retirement remain provider-owned later transitions. |

---

# 5. Ownership and Participating Domains

| Domain | Responsibility |
|---|---|
| Authentication / Section A | Owns Authentication Provider, Method, Policy, selection and provider lifecycle. |
| Platform & Tenant / Section A | Supplies Tenant configuration and availability context. |
| Person & Identity / Section A | Supplies canonical references without transferring ownership. |
| Event Infrastructure / Section A | Distributes canonical provider events. |
| Audit Infrastructure / Section A | Preserves immutable onboarding evidence. |

---

# 6. Business Actors, Roles and Responsibilities

| Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Authorized Authentication Administrator | Submits and governs provider registration and enablement. | L2-A03 defines commands but does not name this actor canonically. |
| Institutional Security Authority | Supplies Tenant policy, priority and fallback requirements. | Does not own provider state. |
| External Provider | Performs authentication through the uniform provider contract. | Does not own canonical Person or Identity. |

No vendor-specific or unnamed approval role is invented.

---

# 7. Preconditions and Entry Criteria

- Exactly one Tenant and institution are resolved.
- Provider type, endpoint contract and required configuration are available.
- Associated Authentication Methods are supported.
- Tenant Authentication Policy can express provider eligibility, priority and fallback.
- Provider validation and availability checks can be completed.
- The request is authorized and auditable.

---

# 8. Trigger

An institution requires its users to authenticate through an external or additional authentication provider

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authorized provider-onboarding requirement. | Authentication | A governed provider path begins. |
| 2 | Resolve Tenant configuration and verify Tenant isolation. | Platform & Tenant; Authentication | Provider scope is fixed to one Tenant. |
| 3 | Validate provider type and the uniform logical authentication contract. | Authentication — Authentication Provider | Provider technology remains isolated from business workflows. |
| 4 | Execute `RegisterAuthenticationProvider` with canonical identifier and Tenant ownership. | Authentication Provider | Provider is registered and `AuthenticationProviderRegistered` is published. |
| 5 | Record provider configuration, supported methods and tenant-specific enablement parameters. | Authentication Provider; Authentication Method | Configuration and method associations are established. |
| 6 | Validate configuration, provider reachability/availability, security and policy compatibility. | Authentication | Only a valid provider may proceed. |
| 7 | Configure Authentication Policy eligibility, deterministic priority, fallback and failover conditions. | Authentication Policy; Provider Selection Service | Selection order is governed. |
| 8 | Revalidate that canonical Identity remains external and no credential ownership is transferred. | Authentication; Person & Identity | Ownership boundaries are preserved. |
| 9 | Execute `EnableAuthenticationProvider`. | Authentication Provider | Provider becomes selectable and `AuthenticationProviderEnabled` is published. |
| 10 | Verify selection using eligible, enabled and healthy provider criteria without performing a business login. | Provider Selection Service | Governed provider availability is confirmed. |
| 11 | Preserve registration, validation, policy, priority, fallback, events and actor evidence. | Authentication; Audit Infrastructure | Onboarding is traceable. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Path | Outcome |
|---|---|---|---|
| Additional provider | Another provider is already available. | Assign policy-governed relative priority and fallback. | Multiple providers coexist deterministically. |
| Fallback-only provider | Provider should not be preferred. | Enable it only for policy-defined fallback conditions. | It is selectable only when higher-priority providers are unavailable. |
| Validation pending | Configuration exists but validation is incomplete. | Retain the provider outside selectable availability. | No enabled event is published. |
| Maintenance after onboarding | Provider later becomes unavailable. | Use `PlaceProviderInMaintenance` and later `RecoverAuthenticationProvider`. | Normal routing excludes it until validated recovery. |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Consequence | Resolution | Owner |
|---|---|---|---|
| Tenant scope is invalid. | Provider isolation cannot be preserved. | Reject registration. | Platform & Tenant; Authentication |
| Provider contract is invalid. | Provider cannot participate uniformly. | Reject or correct configuration. | Authentication |
| Method or policy is incompatible. | Deterministic selection cannot be assured. | Do not enable the provider. | Authentication |
| Availability validation fails. | Provider must not receive authentication requests. | Keep unavailable or place in maintenance. | Authentication Provider |
| Enablement is interrupted. | Selectability is uncertain. | Do not publish `AuthenticationProviderEnabled`; revalidate current lifecycle state. | Authentication Provider |

---

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Effect | Source |
|---|---|---|
| A03 exclusively owns Authentication Provider lifecycle. | No other domain registers or enables providers. | L2-A03 — INT-301 |
| Every provider belongs to one Tenant. | Cross-Tenant provider selection is prohibited. | L2-A03 — Provider validation rules |
| Provider selection is deterministic and policy-driven. | Priority, availability and fallback are evaluated before selection. | L2-A03 — Provider Selection Workflow |
| Disabled, unavailable or maintenance providers cannot be selected normally. | Availability governs routing. | L2-A03 — Provider invariants |
| Providers do not own credentials or canonical identities. | Onboarding cannot change A02 ownership. | L2-A03 — Provider Principles |
| Events follow successful aggregate transitions. | Registration or enablement failure cannot publish success. | L2-A03 — Aggregate events |

---

# 13. State and Lifecycle Impact

| Concept | Entry | Impact | Result |
|---|---|---|---|
| Authentication Provider | Not registered | `Register → Configured → Validated → Available`. | Enabled and selectable. |
| Authentication Method | Existing or separately registered | Associate with provider and policy eligibility. | Available through governed provider. |
| Authentication Policy | Existing active policy | Add scoped eligibility, priority and fallback rules through its governed lifecycle. | Selection rules available. |
| Person / Identity | Existing | Referenced only during later login. | Unchanged. |
| Credential | Existing or absent | No provider-onboarding transition. | Unchanged. |

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

| Event | Publisher | Condition |
|---|---|---|
| `AuthenticationProviderRegistered` | Authentication Provider | Registration succeeds. |
| `AuthenticationProviderEnabled` | Authentication Provider | Provider becomes selectable. |
| `AuthenticationProviderPlacedInMaintenance` | Authentication Provider | Later maintenance transition succeeds. |
| `AuthenticationProviderRecovered` | Authentication Provider | Later validated recovery succeeds. |

## 14.2 Synchronous Contracts and Hand-offs

| Contract | Provider | Consumer | Information / Outcome |
|---|---|---|---|
| Tenant configuration | Platform & Tenant | Authentication | Scope and platform defaults. |
| External provider contract | External Provider | Authentication Provider | Uniform authentication capability and status. |
| Canonical Identity reference | Person & Identity | Authentication | Identity used only during later authentication. |
| Provider policy | Authentication Policy | Provider Selection Service | Eligibility, priority and fallback. |
| Audit input | Authentication | Audit Infrastructure | Registration and enablement evidence. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Provider registration, configuration and enablement require privileged Tenant-scoped authorization.
- Provider availability never grants business authorization.

## 15.2 Approval and Separation of Duty

- L2-A03 does not name a provider-onboarding approver or separation-of-duty requirement.
- No approval role is inferred.

## 15.3 Audit Obligations

Record Tenant, ProviderId, type, supported methods, configuration classification without secrets, validation results, policy version, priority, fallback, commands, events, actor, timestamp and correlation.

---

# 16. Completion Criteria and Postconditions

- Provider was registered for one Tenant.
- Configuration and uniform contract were validated.
- Supported methods and active policy scope were established.
- Deterministic priority, fallback and failover were defined.
- `EnableAuthenticationProvider` succeeded and `AuthenticationProviderEnabled` was published.
- Provider is selectable only while enabled and healthy.
- Person, Identity and Credential ownership remain unchanged.
- Audit evidence is preserved.

---

# 17. Open Issues, Assumptions and Deferred Questions

| ID | Severity | Issue | Treatment / Required Resolution |
|---|---|---|---|
| A055-ISSUE-01 | Non-blocking | The provider lifecycle names `Configured` and `Validated`, but the owned command/event catalogue has no configure or validate provider command/event. | Treat configuration and validation as prerequisites inside registration-to-enable progression; define exact contracts before L4. |
| A055-ISSUE-02 | Non-blocking | L2-A03 defines provider commands but not the actors or any approval authority for onboarding. | Use logical responsibilities without inventing canonical roles; define them before locking. |

No vendor-specific mechanism, secret, actor, event or transition is invented.
