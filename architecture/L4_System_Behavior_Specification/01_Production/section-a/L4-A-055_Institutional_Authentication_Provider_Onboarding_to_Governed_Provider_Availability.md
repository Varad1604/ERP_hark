# Institutional Authentication Provider Onboarding to Governed Provider Availability

# 1. Specification Metadata

**Specification ID:** L4-A-055  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authentication  
**Realizes Scenario:** L3-A-055 — Institutional Authentication Provider Onboarding to Governed Provider Availability  
**Primary Business Owner:** Authentication  
**Version:** 1.0.1  
**Status:** Architecture Approved  
**Owner:** Dhruv  
**Last Updated:** 2026-09-23  
**Depends On:** SutraERP L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-A01; L2-A02; L2-A03; L2-A06; L2-A07; L3-A-055  
**Referenced By:** None yet

---

# 2. Behavioral Objective & Boundary

Require an institutional Authentication Provider to be registered for exactly one Tenant, configured and validated against the source-defined provider contract, policy-scoped into deterministic priority/fallback selection, and enabled only when eligible, while preserving Person/Identity/Credential ownership outside the provider.

This specification defines required observable system behavior only. It does not define APIs, database schemas, software classes, user interfaces, infrastructure, deployment, transport mechanisms, or technical orchestration.

---

# 3. Required Business Context

- Exactly one Tenant and institution are resolved.
- Provider type, logical provider contract, required configuration and supported Authentication Methods are available.
- Tenant Authentication Policy can express provider eligibility, priority, availability and fallback/failover conditions.
- Authentication owns Authentication Provider/Method/Policy and provider lifecycle; Person & Identity retains canonical identity ownership.

---

# 4. System Behavioral Obligations

| Behavior ID | Condition | Required System Behavior | Governed Result | Provenance |
|---|---|---|---|---|
| L4-A-055-BH-001 | An authorized provider-onboarding requirement is received. | Resolve one Tenant/institution and validate that provider ownership and requested scope remain Tenant-isolated. | Provider onboarding is bounded to one Tenant. | L3-A-055 §§7–9; L2-A03 |
| L4-A-055-BH-002 | Provider type and required logical contract are available. | Validate the provider against the uniform authentication-provider boundary without embedding vendor-specific behavior. | Only provider-independent capabilities enter business architecture. | L3-A-055 §§4,9,12; L2-A03 |
| L4-A-055-BH-003 | Registration prerequisites are satisfied. | Execute the canonical `RegisterAuthenticationProvider` behavior with the provider’s canonical identifier and Tenant ownership. | The provider becomes registered under Authentication ownership. | L3-A-055 §9; L2-A03 |
| L4-A-055-BH-004 | Provider registration succeeds. | Publish `AuthenticationProviderRegistered` from the Authentication Provider aggregate. | Consumers observe the canonical registration fact. | L3-A-055 §14; L2-A03 |
| L4-A-055-BH-005 | Registered provider configuration and supported methods are supplied. | Associate source-governed configuration/method facts and validate availability, security and policy compatibility without inventing separate configure/validate commands or events. | Only a source-valid provider may progress to enablement. | L3-A-055 §§9,17; L2-A03 |
| L4-A-055-BH-006 | Provider policy scope is established. | Require deterministic eligibility, relative priority, availability, fallback and failover conditions under the Tenant Authentication Policy. | Selection behavior is governed and deterministic. | L3-A-055 §§3,9,12; L2-A03 |
| L4-A-055-BH-007 | Provider configuration/policy validation succeeds. | Reconfirm that canonical Identity and Credential ownership remain outside Authentication Provider onboarding before enablement. | Provider onboarding cannot transfer identity/credential ownership. | L3-A-055 §§4,9,12; L2-A02; L2-A03 |
| L4-A-055-BH-008 | The provider is valid, eligible and authorized for availability. | Execute canonical `EnableAuthenticationProvider`. | The provider becomes available for source-defined selection. | L3-A-055 §9; L2-A03 |
| L4-A-055-BH-009 | Provider enablement succeeds. | Publish `AuthenticationProviderEnabled` from Authentication Provider and make the provider selectable only while enabled and otherwise eligible/available under policy. | Governed provider availability is observable. | L3-A-055 §§9,14,16; L2-A03 |
| L4-A-055-BH-010 | Registration, validation or enablement fails. | Do not publish the corresponding success event and do not allow the provider into normal selection. | Invalid or uncertain providers cannot receive normal authentication selection. | L3-A-055 §§11–12; L2-A03 |

---

# 5. Cross-Domain & Event Behavior

- Platform & Tenant supplies Tenant context; Person & Identity supplies canonical identity references only during later authentication.
- `AuthenticationProviderRegistered` and `AuthenticationProviderEnabled` are published by Authentication Provider after their corresponding successful transitions.
- Provider policy supplies eligibility/priority/fallback to provider selection; onboarding does not perform a user login.
- Later maintenance/recovery events remain separate lifecycle behavior and are not required to claim onboarding completion.

---

# 6. Failure / Rejection / Continuation

- Invalid Tenant scope or provider contract → reject registration.
- Incompatible methods/policy or failed availability validation → do not enable.
- Interrupted/uncertain enablement → do not publish `AuthenticationProviderEnabled` or treat the provider as selectable until current governed state is re-established.

---

# 7. Audit & Evidence Behavior

Preserve Tenant, provider identifier/type, supported methods, non-secret configuration classification, validation outcomes, applicable policy version, priority/fallback/failover rules, canonical commands/events, actor context, time and correlation without storing implementation-specific secret details in architecture.

---

# 8. Observable Completion Conditions

- Provider is registered to exactly one Tenant.
- Provider contract/configuration/method/policy compatibility is validated.
- Deterministic priority and fallback/failover rules are established.
- `EnableAuthenticationProvider` succeeds and `AuthenticationProviderEnabled` is observable.
- Provider is selectable only while source-eligible, enabled and available, and identity/credential ownership remains unchanged.

---

# 9. Traceability & Provenance

- **Primary parent:** `L3-A-055` — Institutional Authentication Provider Onboarding to Governed Provider Availability.
- **Canonical owner:** Authentication.
- Higher-level ownership, business rules, lifecycle states and canonical event publishers remain governed by the cited L1/L2/L3 sources.
- This L4 specification refines observable system obligations; it does not revise or silently repair upstream architecture.

---

# 10. Open Issues / Deferred Design

| ID | Deferred Design / Upstream Gap | L4 Treatment |
|---|---|---|
| A055-L4-OPEN-01 | Source lifecycle includes Configured and Validated concepts but no separate configure/validate provider command or event. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A055-L4-OPEN-02 | Canonical human actor/approval authority for provider onboarding is not defined even though provider commands are defined. | Preserve the gap explicitly; do not invent missing architecture at L4. |

No implementation design is introduced by these deferred items.


## Canonical Validation Note

This specification passed the scoped R3A hard-fail validation against the Locked L4 Governance and Architecture Standard and L4 Validation Standard and is `Architecture Approved` under `L4_R3A_IAM_Safe_Authority_Architecture_Approval_Decision_2026-09-23.md`. Existing non-blocking upstream gaps remain bounded exactly as recorded; this approval does not invent missing actors, lifecycle states, events, publishers, recovery contracts, provider mechanics, or implementation design.
