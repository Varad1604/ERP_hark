# Government Connector to Governed Active or Retired State

# 1. Scenario Metadata

**Scenario ID:** L3-H-009  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** A, H  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Locked  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-09-01  
**Depends On:** L2-H01; `L2-H01_Connector_Retirement_Amendment_2026-09-01.md`; ADR-L4-UPSTREAM-BLOCKER-RESOLUTION-001

# 2. Scenario Purpose
Govern H01 Connector registration/configuration/activation, disablement and terminal retirement while preserving auditability.

# 3. Business Outcome
Only validated connectors are active and every configuration/lifecycle change, including terminal retirement, remains auditable.

# 4. Scenario Boundary and Scope
H01 owns Connector identity, configuration, lifecycle and health. Section A provides authorization/audit capability only.

# 5. Ownership and Participating Domains
H01 exclusively owns the government Connector aggregate.

# 6. Business Actors, Roles and Responsibilities
Integration Administrator registers/configures. Platform Administrator activates, disables and authorizes retirement.

# 7. Preconditions and Entry Criteria
Connector identity is valid/unique; activation requires validation; retirement requires the connector to be unavailable for normal use.

# 8. Trigger
An authorized administrator registers, changes, suspends, disables or retires a government connector.

# 9. Main Business Flow
1. Register/configure and validate the Connector using existing H01 rules.  
2. Activate only after validation and publish `ConnectorActivated`.  
3. For governed withdrawal from normal use, transition to `Disabled` and publish `ConnectorDisabled`.  
4. When terminal retirement is authorized, transition `Disabled → Retired` and publish `ConnectorRetired`.

# 10. Alternate Business Flows
Maintenance/disablement is non-terminal; a Retired Connector cannot be reactivated.

# 11. Failure, Interruption and Business Recovery
Failed validation prevents activation. Retirement cannot bypass disablement.

# 12. Business Rules and Decision Constraints
`Disabled` and `Retired` are distinct; Retired is terminal.

# 13. State and Lifecycle Impact
Existing H01 states remain; terminal `Retired` is added after Disabled.

# 14. Business Events and Cross-Domain Collaboration
H01 publishes existing Connector events plus `ConnectorRetired`.

# 15. Business Authorization, Approval and Audit Obligations
Preserve acting administrator, validation, configuration history, disablement and retirement decision evidence.

# 16. Completion Criteria and Postconditions
Active branch completes at validated activation; retirement branch completes at terminal Retired with `ConnectorRetired` published.

# 17. Open Issues, Assumptions and Deferred Questions
No blocking issues. No assumptions.
