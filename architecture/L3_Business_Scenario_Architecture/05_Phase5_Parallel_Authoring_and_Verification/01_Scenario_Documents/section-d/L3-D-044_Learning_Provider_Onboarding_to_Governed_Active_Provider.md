# Learning Provider Onboarding to Governed Active Provider

# 1. Scenario Metadata

**Scenario ID:** L3-D-044  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Open Electives & MOOCs  
**Participating Domains:** D  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Review  
**Version:** 1.1  
**Owner:** Pallav  
**Last Updated:** 2026-08-25  
**Depends On:** `L2-D06_Open_Electives_and_MOOCs_Workflows.md`  
**Frozen Stage 3 Source:** P2W-D-0764 / CAN-L3-D-044

---

# 2. Scenario Purpose

Onboard an institution-approved external learning provider through governed provider identity, approved institutional capability/scope and activation.

# 3. Business Outcome

A uniquely identified provider reaches Active for its approved institutional capabilities/scope; suspended providers cannot receive new institutional enrollments.

# 4. Scenario Boundary and Scope

Starts when institutional approval/provider evidence are available; ends when provider reaches Active and `LearningProviderActivated` is published. In scope: unique provider identity, approved profile/capabilities/scope, business-readiness validation, activation and later governed suspension/reactivation/migration where supported. Out of scope: authentication credentials, connectivity tests, API versions, transport or vendor-specific implementation.

# 5. Ownership and Participating Domains

D06 owns institutional provider configuration and Active/Suspended provider availability/relationship. External provider registry/accreditation authority owns provider identity/accreditation/approval evidence.

# 6. Business Actors, Roles and Responsibilities

Academic Affairs approves institutional provider use/capabilities. Any additional institutional administrative lifecycle authority must be explicitly established by D06 policy; `Integration Administrator`, `Integration Service` and technical configuration authority are removed from L3.

# 7. Preconditions and Entry Criteria

Institutional approval/provider evidence exist; provider identity can be uniquely governed; approved capabilities/scope are defined.

# 8. Trigger

A new provider is approved for institutional onboarding or an existing provider requires governed lifecycle/scope change.

# 9. Main Business Flow

1. D06 establishes provider onboarding record and validates uniqueness/approval evidence.
2. Academic Affairs/D06 establishes approved profile, capabilities and institutional scope.
3. D06 records the governed provider relationship/configuration business fact — `LearningProviderConfigured` / `ProviderIntegrationConfigured` where defined upstream.
4. D06 validates business readiness for institutional use.
5. Authorized institutional decision activates provider.
6. D06 publishes `LearningProviderActivated`.

# 10. Alternate Business Flows

No alternate route reaches Active if approval/readiness is missing. Governed suspension/reactivation/migration are separate supported lifecycle paths, not technical recovery.

# 11. Failure, Interruption and Business Recovery

Invalid provider identity/accreditation/approval blocks activation. Missing institutional authority retains prior provider state. Interrupted onboarding preserves last authoritative D06 provider position/evidence until continuation is valid.

# 12. Business Rules and Decision Constraints

D06 owns institutional provider relationship/state; external accreditation source retains provider evidence ownership; Active status requires approved scope/readiness; technical integration/authentication/configuration is deferred.

# 13. State and Lifecycle Impact

Learning Provider: Onboarding → Configured → Active; governed alternate Suspended/Reactivated as defined in L2-D06 W60/W65.

# 14. Business Events and Cross-Domain Collaboration

D06 publishes `LearningProviderConfigured`, `LearningProviderActivated`, and `ProviderIntegrationConfigured` as defined in L2-D06 W60/W65/Event Catalogue. Event names are preserved even though their technical implementation is outside L3.

# 15. Business Authorization, Approval and Audit Obligations

Academic Affairs governs institutional provider use/capability approval. Other provider lifecycle authority must come from D06 policy. Audit retains source provider evidence, approved scope/capabilities, authority, readiness decision and activation/suspension history.

# 16. Completion Criteria and Postconditions

Provider is Active for approved institutional scope; D06 retains institutional relationship/state ownership; external provider/accreditation ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

No new open issue or assumption is introduced. Authentication, connectivity, API/version configuration, secrets and integration implementation are deferred to L4/L5.

---

**Draft Validation Verdict:** REVIEW REQUIRED  
**Validation Basis:** L2-D06; Template v2.0; Architecture Level Validation Standards.
