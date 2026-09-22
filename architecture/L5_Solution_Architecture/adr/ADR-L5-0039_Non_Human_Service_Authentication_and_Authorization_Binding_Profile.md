# ADR-L5-0039 — Non-Human Service Authentication & Authorization Binding Profile

**ADR ID:** `ADR-L5-0039`  
**Title:** Non-Human Service Authentication & Authorization Binding Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-01 — IAM Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-D-01-002`; `L5-D-01-003`; constrains `L5-D-02-001` and technical service/job/connector consumers  
**Related ADRs:** `ADR-L5-0009`; `ADR-L5-0024`; `ADR-L5-0027`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

D01 already separates human principals from service/workload/job/integration principals, and ADR-L5-0009 binds non-human principals into D02 authorization. Backend readiness requires one service-authentication profile rather than shared static API keys or implicit internal-network trust.

## 2. Decision

1. **Explicit non-human identity:** every service, workload, job or integration execution requiring authenticated technical authority uses an explicit D01 non-human principal bound to one Tenant/Institution context where applicable and to its accountable technical owner.
2. **Authentication:** service-to-service and machine authentication uses Keycloak client/service-account credentials or another D01-approved workload credential mechanism backed by D03 protection; shared ambient API keys are not the repository default.
3. **Credential lifetime:** short-lived, renewable credentials are preferred. Long-lived static credentials require a justified integration profile and D03-managed secret lifecycle.
4. **Authorization:** D01 authentication does not grant business authority. D02 evaluates permissions, resource scope, delegation, maker-checker/SoD and emergency-access constraints against the actual non-human principal and execution context.
5. **Human attribution:** where a human initiated work executed by a service/job, both human initiator and technical executor are preserved; the technical principal shall not impersonate the human.
6. **No internal-network trust:** being inside Kubernetes, one process group, one VPC/network or one institution does not make a caller authenticated or authorized.
7. **No human/system credential substitution:** human passwords/sessions cannot be reused as service credentials, and system credentials cannot authenticate as human User Accounts.

## 3. Conformance Rules

- Client/service credentials are issued and protected through D01/D03, not hard-coded into code or images.
- Service identities must be independently revocable without deleting a human Person/User Account.
- Authorization decisions must bind Tenant/Institution/resource scope and cannot rely only on a role-like token claim.
- Anonymous internal calls are prohibited across a protected trust boundary unless an explicit public/anonymous contract authorizes them.
- Provider/integration credentials remain C03/D03-governed and cannot silently become generic internal service identity.

## 4. Consequences

Backend services can authenticate consistently without shared secrets becoming implicit platform authority. D02 remains the canonical authorization decision layer.

## 5. External Evidence

- Keycloak service accounts/client credentials: https://www.keycloak.org/docs/latest/server_admin/

## 6. Approval Record

**Decision selected:** governed non-human principals; Keycloak/service-account or approved workload authentication; short-lived credentials preferred; D02 remains authorization authority; no implicit internal-network trust.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
