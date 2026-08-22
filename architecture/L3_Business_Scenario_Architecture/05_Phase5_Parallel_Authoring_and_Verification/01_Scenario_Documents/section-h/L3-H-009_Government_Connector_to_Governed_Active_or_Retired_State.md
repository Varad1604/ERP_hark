# Government Connector to Governed Active or Retired State

# 1. Scenario Metadata

**Scenario ID:** L3-H-009  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; Section A Authorization and Audit architecture  
**Referenced By:** Government publication, linking, synchronization, revocation, connector governance, and audit activities

# 2. Scenario Purpose

Govern the H01 Connector lifecycle so that only validated connectors become available for government integration and all connector configuration and lifecycle changes remain auditable, while preserving the frozen requirement for a governed active-or-retired outcome.

# 3. Business Outcome

Only validated connectors are active and every configuration or lifecycle change remains auditable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorized administrator registers, changes, suspends, or retires a government connector.

## 4.2 End Boundary

The frozen end boundary requires the Connector to reach a governed active or retired state. Locked H01 supports validated activation, operational use, maintenance, disabling, and reactivation, but does not define a canonical `Retired` Connector state or retirement transition. The retirement branch therefore cannot reach its frozen end boundary authoritatively.

## 4.3 In Scope

- H01 Connector registration, configuration, validation, activation, suspension/maintenance, disabling, and configuration change where explicitly defined.
- Validation before activation and prohibition on using disabled connectors for publication.
- Exact H01 Connector events for defined lifecycle/configuration actions.
- Preservation of connector audit evidence.
- Identification of the missing retirement lifecycle semantics required by the frozen outcome.

## 4.4 Out of Scope

- Provider-specific technical configuration details or authentication mechanisms.
- Government credential publication itself.
- Inventing a `Retired` state, event, command, or transition not present in locked H01.
- Changing Section A ownership of Authorization or Audit.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Platform Foundation / Section A | Retains canonical Authorization and Audit ownership for connector-governance authority and immutable evidence. |
| APAAR & DigiLocker / Section H | Owns Connector identity, Connector lifecycle/configuration, validation, activation/disablement, health status, and canonical Connector events. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Integration Administrator | Registers and configures a connector and may update connector configuration. | Source-defined H01 command authority. |
| Platform Administrator | Activates a validated connector or disables a connector. | Source-defined H01 command authority. |

Locked H01 does not assign a business actor to a connector-retirement command because no retirement command is defined.

# 7. Preconditions and Entry Criteria

- Connector identity is unique for registration.
- Required connector configuration is complete and valid before activation.
- Activation occurs only after validation.
- The initiating administrator holds the source-defined H01 authority for the applicable action.
- Existing connector status and configuration history are available for change governance.

# 8. Trigger

An authorized administrator registers, changes, suspends, or retires a government connector

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive an authorized connector registration or change request and identify the intended H01 Connector. | Integration Administrator / APAAR & DigiLocker | Connector governance begins against the H01-owned Connector. |
| 2 | Validate connector uniqueness for registration or validate the existing Connector and requested configuration change. | APAAR & DigiLocker | Invalid connector governance changes are prevented. |
| 3 | Register and configure the Connector using the source-defined lifecycle. | Integration Administrator / APAAR & DigiLocker | Connector reaches the defined Registered/Configured progression. |
| 4 | Validate the Connector before activation. | APAAR & DigiLocker | Only a validated Connector is eligible for activation. |
| 5 | Activate the validated Connector under Platform Administrator authority and make it eligible for governed operational use. | Platform Administrator / APAAR & DigiLocker | `ConnectorActivated` records the defined activation fact. |
| 6 | If suspension or disablement is required, move the Connector only through the source-defined Maintenance/Disabled behavior and preserve audit evidence. | Platform Administrator / APAAR & DigiLocker | Connector becomes unavailable for normal publication as governed. |
| 7 | If retirement is requested, stop at the architecture gap: locked H01 does not define the required Retired state or retirement transition. | APAAR & DigiLocker | Frozen retirement outcome remains unresolved rather than locally invented. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Configuration update | Existing Connector configuration changes. | Apply only a valid source-defined configuration change. | `ConnectorConfigurationUpdated` records the completed change. | APAAR & DigiLocker |
| Temporary suspension/maintenance | Connector must not be used normally for a period. | Use the source-defined Maintenance or Disabled behavior as applicable. | Connector is not eligible for normal use while disabled. | APAAR & DigiLocker |
| Reactivation | A previously Maintenance/Disabled Connector is again eligible. | Re-enter the source-defined activation path after required validation/governance. | Connector may return to Activated/Operational. | APAAR & DigiLocker |
| Retirement requested | Institution intends permanent retirement. | No canonical retirement path can be completed from locked H01. | Scenario remains blocked on upstream lifecycle definition. | APAAR & DigiLocker |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Connector fails validation | Connector cannot be activated. | Correct the governed connector definition/configuration and repeat source-defined validation before activation. | APAAR & DigiLocker |
| Connector becomes unavailable for normal use | Publication cannot proceed through a disabled connector. | Preserve Connector history and use only a source-defined valid Connector state before future use. | APAAR & DigiLocker |
| Retirement is required | Frozen end state is not represented in H01 lifecycle. | Escalate for architecture change control; do not equate Disabled with Retired without upstream authority. | APAAR & DigiLocker |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Connector identifiers are unique. | APAAR & DigiLocker | Registration cannot create duplicate Connector identity. | L2-H01 §11.3 |
| Disabled connectors cannot accept publication requests. | APAAR & DigiLocker | Disabled Connector is not eligible for publication. | L2-H01 §11.3 |
| Connector configuration changes are auditable. | APAAR & DigiLocker / Platform Audit | Every governed change preserves evidence. | L2-H01 §§11.3, 52 |
| Activation requires completed validation. | APAAR & DigiLocker | `ConnectorActivated` can occur only after validation. | L2-H01 §§35–36, 39 |
| `ConnectorActivated` must occur before connector usage. | APAAR & DigiLocker | Publication cannot use an unactivated Connector. | L2-H01 §39 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Connector | New, `Registered`, `Configured`, `Validated`, `Activated`, `Operational`, `Maintenance`, or `Disabled` as applicable | Apply only the source-defined registration, configuration, validation, activation, maintenance/disablement, or reactivation behavior. | Defined outcomes include `Activated`/`Operational`, `Maintenance`, or `Disabled`. | APAAR & DigiLocker | L2-H01 §§11.3, 23, 61 |
| Connector retirement | Retirement requested | No source-defined transition exists. | `Retired` cannot be assigned by L3. | APAAR & DigiLocker | Gap against frozen Stage 3 outcome |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Exact H01 Connector events include `ConnectorRegistered`, `ConnectorConfigured`, `ConnectorActivated`, `ConnectorDisabled`, `ConnectorConfigurationUpdated`, and `ConnectorHealthValidated`. H01 remains the publisher of these events.

No canonical `ConnectorRetired` event exists in locked H01, so none is introduced.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Connector governance remains within H01. Section A supplies canonical authorization and audit capabilities but does not acquire Connector lifecycle ownership. Government-publication scenarios consume only H01 Connectors that are in a valid source-defined state.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register connector | Integration Administrator | Unique Connector required. | APAAR & DigiLocker |
| Configure or update connector configuration | Integration Administrator | Configuration must be valid. | APAAR & DigiLocker |
| Activate connector | Platform Administrator | Validation must be complete. | APAAR & DigiLocker |
| Disable connector | Platform Administrator | Source-defined authorization required. | APAAR & DigiLocker |
| Retire connector | Not defined upstream | No retirement command, state, or actor is defined. | APAAR & DigiLocker |

## 15.2 Separation of Duties, Overrides and Exceptions

H01 defines different administration roles for configuration and activation/disablement, but no additional separation-of-duty rule is stated and no retirement authority is defined. L3 does not add either.

## 15.3 Business Audit and Evidence Obligations

Connector governance must preserve Connector identity, configuration changes, activation/deactivation or disablement evidence, version changes, health validation, acting administrator where applicable, timestamps, and immutable audit lineage. Platform Audit retains canonical audit-evidence ownership.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

The activation branch completes when a validated Connector reaches the source-defined activated/operational position and required audit evidence is preserved. The frozen retirement branch cannot complete because locked H01 does not define `Retired` or an equivalent authoritative terminal lifecycle state.

## 16.2 Postconditions

- Only validated and activated H01 Connectors are eligible for normal governed use.
- Disabled Connectors cannot accept publication requests.
- Defined configuration/lifecycle changes remain auditable.
- A retirement request remains unresolved until upstream architecture defines the canonical retirement semantics.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| Frozen Stage 3 requires an active-or-`Retired` outcome, but locked H01 Connector states/lifecycle define Registered, Configured, Validated, Activated, Operational, Maintenance and Disabled, with no `Retired` state. | The frozen retirement end boundary and lifecycle transition cannot be expressed without inventing a state. | Add or formally map the canonical Connector retirement state and transition through architecture change control. |
| H01 defines no connector-retirement command, event, or authorized retirement actor. | L3 cannot establish the retirement business action, publisher, or authority. | Define the retirement command/business action, canonical event if required, and business authority upstream. |
| H01 §50 refers to deprecated connector versions being explicitly retired under institutional upgrade policy, but that prose is not represented in the Connector lifecycle/state machine. | The word “retired” appears as version-policy prose but cannot be treated as an aggregate lifecycle state without upstream confirmation. | Reconcile version-retirement policy with the Connector aggregate lifecycle. |

**Architecture assessment:** `BLOCKED`

The registration/configuration/validation/activation/disablement portions are source-grounded, but the frozen `Retired` outcome is missing from the authoritative Connector lifecycle and event/authority model. Upstream resolution is required.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
