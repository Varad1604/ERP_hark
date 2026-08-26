# External Connector Configuration to Governed Activation or Retirement

## 1. Scenario Metadata

**Scenario ID:** L3-H-073  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** External Education Integration  
**Participating Domains:** A, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP_L1_Business_Domain_Architecture_Locked.md; DEPENDENCY_MAP.md; L2-A03_Authentication_Workflows.md; L2-H10_External_Education_Integration_Workflows.md  
**Referenced By:** None yet

## 2. Scenario Purpose

Govern a new or changed external connector and its versioned integration configuration through validation, certification and activation, while preserving suspension, retirement and controlled prior-version restoration semantics only where upstream architecture defines them.

## 3. Business Outcome

The connector and versioned configuration are validated, activated, suspended, rolled back, or retired with complete audit history

## 4. Scenario Boundary and Scope

### 4.1 Start Boundary

An approved provider requires a new or changed connector.

### 4.2 End Boundary

The connector and associated governed configuration reach an authoritative activated, suspended, retired, or other source-defined outcome required by the frozen scenario semantics, with history retained.

### 4.3 In Scope

- H10 connector registration/configuration, validation, certification, activation, suspension, deprecation and retirement.
- Versioned integration configuration validation and activation.
- Business-level restoration to a prior approved version only to the extent explicitly defined upstream.
- Section A authentication facts referenced by connector configuration without ownership transfer.

### 4.4 Out of Scope

- Provider relationship onboarding governed by L3-H-072.
- Credential or Authentication Provider lifecycle ownership in L2-A03.
- Later architecture-level implementation concerns.
- External synchronization and reconciliation outcomes after connector activation.

## 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| External Education Integration (H) | Owns Integration Connector and Integration Configuration governance and their H10 lifecycle outcomes. |
| Platform Foundation (A) | Retains canonical authentication and credential ownership referenced by connector configuration. |

## 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Integration Administrator | Governs connector registration, configuration and lifecycle administration. | Initiates / performs |
| Security Administrator | Reviews authentication and security conditions required for governed connector use. | Reviews |
| Integration Operator | Participates in governed operational validation and connector lifecycle administration where defined by H10. | Performs |

## 7. Preconditions and Entry Criteria

- The external provider relationship is approved for the relevant integration scope.
- Authentication references required by the configuration remain governed by Section A.
- A connector proposal or changed connector version is available for H10 governance.

## 8. Trigger

An approved provider requires a new or changed connector

## 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Register or identify the connector version and associated configuration requiring governance. | Integration Administrator / H10 | Connector/configuration change is established. |
| 2 | Define the governed connector configuration, mappings, compatibility, authentication references and applicable operational policy metadata. | External Education Integration | Configuration is complete for validation. |
| 3 | Validate the connector and configuration against H10 compatibility, mapping, authentication and governance requirements. | H10 / Security Administrator | Connector/configuration is validated or returned to a non-active position. |
| 4 | Complete connector certification and the required production-approval governance. | External Education Integration | Connector is Certified and eligible for activation. |
| 5 | Activate the certified connector and validated configuration for the approved provider scope. | External Education Integration | Connector/configuration reaches the governed active position. |

## 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Connector suspended | Risk, incident or governance decision temporarily halts use | Transition under the H10 connector lifecycle to Suspended. | No new governed operational use proceeds until re-validation and approval. | External Education Integration |
| Connector deprecated | Controlled transition away from an active connector is required | Move the connector to Deprecated within the governed support window. | Existing controlled use may continue only within H10 governance. | External Education Integration |
| Connector retired | End-of-life is approved and dependencies are cleared | Transition to Retired. | Retired is terminal. | External Education Integration |
| Prior-version restoration required | A change must be reversed to a previously approved version | H10 identifies prior-version restoration as a compensation/recovery concept, but the canonical lifecycle transition and business event for the frozen `rolled back` outcome are not defined consistently. | Upstream resolution required before this branch can be authored as a complete lifecycle path. | External Education Integration |

## 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Connector or configuration fails validation | Activation is prohibited. | Return to a non-production governance position for correction and re-validation. | External Education Integration |
| Active connector is suspended | New operational use is prohibited. | Re-validation and governed approval are required before Active may be restored. | External Education Integration |
| Prior approved version must be restored | Frozen outcome requires a governed rollback branch. | BLOCKED: H10 mentions restoration/rollback but does not define the authoritative connector/configuration lifecycle transition and canonical event for that branch. | External Education Integration |

## 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| CN-002 — Connector Version must increase monotonically. | External Education Integration | Preserves governed connector version progression. | L2-H10 §12.4 |
| Connector lifecycle governance — activation requires certification and explicit approval. | External Education Integration | Prevents operational activation before governed certification and approval. | L2-H10 §4.8 |
| CN-004 — Deprecated connectors cannot receive new deployments. | External Education Integration | Limits use during controlled transition. | L2-H10 §12.4 |
| CN-006 — Connector health status must be Operational before activation. | External Education Integration | Prevents activation without required readiness. | L2-H10 §12.4 |
| CF-004 — Authentication policy is mandatory. | External Education Integration | Requires governed authentication configuration. | L2-H10 §12.5 |
| CF-006 — Transformation mappings must pass validation. | External Education Integration | Prevents activation of invalid mappings. | L2-H10 §12.5 |
| Credential and Authentication Provider lifecycles remain exclusively owned by A03. | Platform Foundation — Authentication | Connector configuration may reference authentication facts but cannot re-own them. | L2-A03 Canonical Ownership |

## 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Integration Connector | Draft / Configured / Validated / Certified / Active | Governed validation, certification, activation, suspension, deprecation or retirement. | Active / Suspended / Deprecated / Retired | External Education Integration | L2-H10 §4.8 |
| Integration Configuration | Created / validated configuration | Validate and activate versioned configuration; archive when retired. | Active / Archived | External Education Integration | L2-H10 §6.5 / §10.5 |
| Credential / Authentication Provider | Existing A03 lifecycle position | Referenced by configuration; H10 does not own state change. | Unchanged by H10 | Platform Foundation — Authentication | L2-A03 Canonical Ownership |

The authoritative lifecycle position after the frozen `rolled back` outcome is not defined upstream and is therefore not invented here.

## 14. Business Events and Cross-Domain Collaboration

### 14.1 Published / Consumed Business Events

| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| ConnectorValidated | Integration Connector aggregate / H10 connector lifecycle | H10 governance consumers | Records successful connector validation. | L2-H10 §4.8 / §10.4 |
| ConnectorDeprecated | Integration Connector aggregate | H10 connector consumers | Records governed connector deprecation. | L2-H10 §6.4 / §10.4 |
| ConnectorRetired | Integration Connector aggregate | H10 connector consumers | Records terminal connector retirement. | L2-H10 §6.4 / §10.4 |
| ConfigurationValidated | Integration Configuration aggregate | H10 configuration consumers | Records successful configuration validation. | L2-H10 §6.5 / §10.5 |
| ConfigurationActivated | Integration Configuration aggregate | H10 synchronization consumers | Records active configuration availability. | L2-H10 §6.5 / §10.5 |
| ConfigurationArchived | Integration Configuration aggregate | H10 configuration consumers | Records configuration retirement/archival. | L2-H10 §6.5 / §10.5 |

### 14.2 Business Contracts and Cross-Domain Hand-offs

- H10 references Section A authentication facts required by connector configuration without altering A03-owned credential or Authentication Provider lifecycles.
- H10 does not define a separate named business contract for the frozen rollback branch; that missing definition contributes to the BLOCKED classification.

## 15. Business Authorization, Approval and Audit Obligations

### 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Register/configure connector | Integration Administrator | H10 enterprise role for connector and configuration governance. | External Education Integration |
| Review authentication/security conditions | Security Administrator | Reviews H10 configuration security while A03 retains credential lifecycle ownership. | External Education Integration / Platform Foundation — Authentication |
| Connector certification / activation | H10 governance roles defined by connector certification workflow | Certification and explicit approval are mandatory before Active. | External Education Integration |
| Connector retirement | H10 governance | Retirement requires dependency review and preserves history. | External Education Integration |

### 15.2 Separation of Duties, Overrides and Exceptions

H10 connector certification explicitly separates development approval, security review, certification and production approval. This scenario does not create additional override authority.

### 15.3 Business Audit and Evidence Obligations

| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| Connector validation and certification | Versioned validation, certification and approval evidence | External Education Integration |
| Connector/configuration lifecycle history | State-change, configuration-version and retirement history | External Education Integration |
| Authentication evidence | Canonical credential/authentication history remains with Section A owner | Platform Foundation — Authentication |

## 16. Completion Criteria and Postconditions

### 16.1 Completion Criteria

- The connector/configuration has reached a source-supported Active, Suspended, Deprecated or Retired position, or the unsupported rollback branch has been explicitly identified as unresolved.
- Required validation, certification and approval evidence is retained.
- Section A authentication ownership remains unchanged.

### 16.2 Postconditions

- Only certified and approved connector/configuration versions may be active.
- Suspended connectors cannot return to Active without governed re-validation.
- Retired connector state is terminal.
- The frozen rollback branch remains unresolved pending upstream lifecycle/event definition.

## 17. Open Issues, Assumptions and Deferred Questions

### 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| The frozen outcome requires the connector/configuration to be `rolled back`, while H10 only mentions configuration rollback and restoration of a previous version in compensation/recovery text; it does not define an authoritative connector/configuration rollback state transition or canonical rollback event. | A material frozen outcome branch cannot be expressed as an authoritative lifecycle path without invention. | Upstream H10 must define the governed business lifecycle transition, resulting state and canonical event/publisher for rollback/restoration. |
| H10 §4.8 connector state machine and H10 §6.4 aggregate lifecycle use different states and event vocabulary (`Draft/Validated/Active/Suspended` versus `Registered/Deployed/Operational`, among others). | Lifecycle and event interpretation is internally inconsistent even though activation, suspension and retirement concepts are broadly supported. | Reconcile the H10 connector lifecycle and event vocabulary before this scenario can be locked. |
| `ConnectorActivated` and `ConnectorSuspended` appear in the §4.8 state-machine publication column but are absent from the canonical §10.4 Connector Event catalogue. | Exact canonical event publication for activation/suspension is ambiguous. | Reconcile the connector event catalogue and publisher assignments. |

### 17.2 Assumptions

No assumptions.

### 17.3 Deferred Questions

No scenario-specific deferred questions.
