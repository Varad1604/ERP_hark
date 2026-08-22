# Infrastructure Configuration Change to Future Policy Availability

# 1. Scenario Metadata

**Scenario ID:** L3-G-317  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Simple  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A01 — Platform & Tenant Foundation; L2-A04 — Authorization & Policy Workflows; L2-A07 — Audit Infrastructure Workflows  
**Referenced By:** Future Infrastructure workflows consuming resolved active Infrastructure configuration

# 2. Scenario Purpose

Govern an authorized change to Infrastructure-domain policy or business configuration so that the G10-owned configuration becomes available to applicable future Infrastructure workflows through the Platform Configuration Framework without transferring business-domain configuration ownership to Section A.

# 3. Business Outcome

The configuration becomes effective for applicable future infrastructure workflows without changing canonical ownership

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised institution configures an infrastructure policy or business parameter.

## 4.2 End Boundary

The scenario ends when the Infrastructure-domain configuration is validated, governed through the Platform Configuration Framework, and effective for applicable future Infrastructure workflows while ownership remains with the proper configuration layer/domain.

## 4.3 In Scope

- Infrastructure-domain configuration values and policies owned by G10.
- Effective dating, versioning and auditability of Infrastructure configuration as required by G10.
- Platform Configuration Framework ownership/scope/inheritance/validation and configuration lifecycle.
- Activation/effectiveness only after the configuration is valid and active.
- Preservation of canonical configuration ownership by layer.

## 4.4 Out of Scope

- Platform Defaults or Tenant Configuration owned by A01.
- Institution Configuration owned by B01 or Organisation Configuration owned by B02.
- Reclassifying a business-domain policy as Platform configuration.
- Applying a new configuration retroactively to completed historical Infrastructure outcomes unless a separate authoritative rule explicitly requires it.
- Inventing a configuration-change business event name.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns Infrastructure business-domain configuration values/policies and their business semantics. |
| Platform Foundation / Section A | Owns the Platform Configuration Framework, hierarchical resolution, validation/publication framework, authorization and audit infrastructure. |

Section A framework participation does not transfer ownership of Infrastructure configuration values or business semantics from G10.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates and maintains authorized Infrastructure-domain configuration within assigned scope. | Full Infrastructure-management responsibility is source-supported; configuration still follows Platform framework governance. |
| Auditor | Reviews immutable evidence of governed configuration change. | Read-only evidence role. |

No additional configuration approver is named because the reviewed sources define configuration governance/lifecycle but do not assign a scenario-specific human approval actor.

# 7. Preconditions and Entry Criteria

- The proposed value/policy is an Infrastructure business-domain configuration concern owned by G10 rather than a higher configuration layer.
- The configuration scope is valid within the canonical hierarchy.
- The change is authorized.
- The configuration can be validated against G10 business semantics and A01 configuration-framework rules.

# 8. Trigger

An authorised institution configures an infrastructure policy or business parameter

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the Infrastructure policy/business parameter and confirm that its business semantics belong to G10. | Infrastructure Management | Canonical domain ownership is established before change. |
| 2 | Establish the intended configuration scope/effective context without changing ownership at other configuration layers. | Infrastructure Management / Platform Configuration Framework | The configuration is scoped within the hierarchical framework. |
| 3 | Validate the value against G10 business rules and the Platform Configuration Framework's ownership, scope, integrity and dependency rules. | Infrastructure Management / Platform Foundation | Invalid configuration cannot become effective. |
| 4 | Progress the configuration through the A01 configuration lifecycle required for active use. | Platform Foundation with Infrastructure Management as domain-value owner | Only governed active configuration may be consumed. |
| 5 | Make the resolved active Infrastructure configuration available to applicable future G10 workflows. | Platform Foundation / Infrastructure Management | Future Infrastructure execution receives validated resolved configuration. |
| 6 | Preserve audit evidence and prior/superseded configuration history according to the upstream configuration rules. | Platform Audit / Infrastructure Management | Change history remains governed and canonical ownership remains unchanged. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Proposed value belongs to another configuration layer | Ownership validation shows the item is Platform, Tenant, Institution or Organisation configuration rather than G10 domain configuration. | Do not create a duplicate G10-owned value; route the change to the canonical configuration owner. | Ownership remains non-overlapping. | Platform Foundation / applicable canonical owner |
| Child scope inherits parent value | No permitted local override is required. | Continue using resolved inherited configuration rather than creating a duplicate local value. | Deterministic inherited configuration remains effective. | Platform Configuration Framework |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Ownership or scope validation fails | Configuration cannot become active. | Correct the ownership/scope or route the value to its canonical owner. | Platform Foundation / Infrastructure Management |
| G10 business validation fails | Invalid Infrastructure policy/parameter cannot govern future workflows. | Correct the proposed domain value before lifecycle progression. | Infrastructure Management |
| Platform configuration validation fails | Configuration does not become operational. | Preserve the prior active resolved configuration until a valid successor is approved/active. | Platform Foundation |
| Authorization fails | Change cannot proceed. | Preserve existing active configuration. | Platform Foundation / Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| The Platform owns the configuration framework; business domains own only their domain values/policies. | Platform Foundation / respective business domain | G10 owns Infrastructure configuration semantics while A01 owns framework behavior. | L2-A01 §§6.1–6.5 |
| Configuration ownership shall not overlap. | Platform Foundation | A value is not duplicated across configuration layers. | L2-A01 §6.5 / §6.15 |
| Only Active configuration may participate in business execution. | Platform Foundation | The changed configuration is not effective for future workflows before valid lifecycle activation. | L2-A01 §6.10 |
| Active configuration must be validated and configuration changes are auditable. | Platform Foundation | Invalid/unaudited change cannot become effective. | L2-A01 §6.15 |
| Infrastructure behavior is configuration driven and supports effective dating, version control, auditability and institution isolation. | Infrastructure Management | G10 configuration changes govern future Infrastructure behavior without changing ownership. | L2-G10 §43 |
| Every business operation must be authorized before execution. | Platform Foundation — Authorization | Unauthorized configuration changes are prohibited. | L2-A04 AP-01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Configuration item/context | Existing active configuration and proposed successor | Proposed change is validated/governed through the Platform Configuration Framework. | Successor configuration reaches `Active`; prior version may become `Superseded` according to A01 configuration lifecycle. | Ownership distributed by layer; Infrastructure business-domain value remains G10-owned | L2-A01 §§6.5, 6.10 |
| Physical Infrastructure aggregate | Existing G10 lifecycle state | No physical Infrastructure lifecycle transition is caused merely by making future policy/configuration active. | Unchanged by this scenario. | Infrastructure Management | L2-G10 §43 and lifecycle rules |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

The reviewed G10 and A01 sources require auditability and configuration publication/availability but do not define an exact canonical business event name for an Infrastructure-domain configuration change becoming active. No event is invented or renamed.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| Infrastructure Management | Platform Configuration Framework | G10-owned Infrastructure configuration value/policy and scope | Preserve domain semantics while using the common configuration framework. | L2-G10 §43; L2-A01 §6.5 |
| Platform Configuration Framework | Infrastructure Management | Resolved validated active configuration context | Make the configuration available to applicable future G10 workflows. | L2-A01 §§6.7–6.14 |
| Platform Foundation | Infrastructure Management | Authorization and audit capability | Govern the change and preserve evidence. | L2-A04; L2-A07 |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Propose/change Infrastructure-domain configuration | Infrastructure Administrator | Must act within authorized Infrastructure scope; the value must remain G10-owned. | Infrastructure Management |
| Validate configuration ownership/scope/framework eligibility | Platform Configuration Framework | Platform owns framework validation; it does not own G10 business semantics. | Platform Foundation |

## 15.2 Separation of Duties, Overrides and Exceptions

No scenario-specific separation-of-duty or override rule is defined for this Infrastructure configuration change. The A01 lifecycle includes an `Approved` state, but the reviewed sources do not assign a scenario-specific human approver; this Draft does not invent one.

## 15.3 Business Audit and Evidence Obligations

Configuration changes must be auditable. Required business evidence is the governed change and its ownership/scope/lifecycle outcome; Platform Audit owns immutable audit evidence while G10 retains ownership of the Infrastructure configuration business fact.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The changed policy/parameter is confirmed as G10-owned Infrastructure domain configuration.
- Configuration scope and ownership are valid and non-overlapping.
- G10 business validation and Platform configuration validation succeed.
- The successor configuration is active/effective for applicable future Infrastructure workflows.
- Existing physical Infrastructure ownership/lifecycle is not transferred or rewritten by the configuration change.
- Audit evidence is preserved.

## 16.2 Postconditions

- Applicable future G10 workflows consume the resolved active configuration.
- Prior configuration history remains governed rather than overwritten silently.
- Section A retains framework ownership; G10 retains Infrastructure-domain configuration semantics/values.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Neither G10 §43 nor A01 §6 defines an exact canonical business event for an Infrastructure-domain configuration successor becoming effective. | L3 cannot assert an event name/publisher beyond the defined configuration publication/availability behavior. This does not prevent the business outcome because no event consumer is mandatory to completion. | Define/register a canonical configuration-change event upstream if enterprise consumers require one. |
| A01 configuration lifecycle includes an `Approved` state, but the reviewed sources do not assign a scenario-specific business approver for Infrastructure-domain configuration. | The Draft can preserve the governed lifecycle without inventing approval authority, but later review should confirm whether a named business approver is required for this scenario. | Upstream configuration/workflow governance should clarify approver resolution if human approval is mandatory. |

**Architecture assessment:** `PASS WITH ISSUES`

The ownership, framework boundary, validation, lifecycle and final outcome are source-grounded and usable. The missing exact event and unspecified scenario-specific approver are recorded without inventing them; neither is required to transfer ownership or define a new Infrastructure lifecycle.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
