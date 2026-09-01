# L4-G-155 — Health Centre Configuration to Operational Activation

**Architecture Level:** L4 — System Behavior Specification  
**Primary Owner:** Institutional Health Services (G06)  
**Realizes:** L3-G-155 — Health Centre Configuration to Operational Activation  
**Authoritative L2:** L2-G06  
**Status:** Draft

## Objective
Govern Health Centre configuration, operational validation and activation so the Health Centre becomes Active and healthcare services are available within authorised scope.

## Boundary
**Start:** Health Centre is ready for configuration and validation.  
**End:** Health Centre is Active and services are operationally available.

## Required Behaviors
1. Identify Health Centre, campus/location and facility type.
2. Establish operating hours, emergency availability and applicable medical-facility configuration.
3. Validate operational readiness under G06 policy.
4. Resolve configuration deficiencies before activation.
5. Authorise activation through the applicable business authority.
6. Transition Health Centre to Active.
7. Publish canonical `HealthCentreActivated` through G06.
8. Preserve configuration and activation history.

## Constraints
G01 retains biomedical-equipment maintenance ownership; Finance retains accounting/billing ownership; Platform Foundation supports without owning the Health Centre lifecycle.

## Failure / Recovery
Missing required configuration or failed validation prevents activation. Once resolved, validation resumes from the business deficiency boundary. No technical retry or deployment behavior is defined.

## Audit & Completion
Completion requires authoritative configuration, validation, activation, `HealthCentreActivated`, and preserved history.

## L4 Boundary
Technical APIs, databases, infrastructure, IAM, UI and deployment are outside L4.

**Traceability:** L3-G-155 — Health Centre Configuration to Operational Activation.