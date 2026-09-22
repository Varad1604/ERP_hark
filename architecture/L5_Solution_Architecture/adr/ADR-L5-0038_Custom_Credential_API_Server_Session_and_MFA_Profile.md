# ADR-L5-0038 — Custom Credential API, Server Session & MFA Profile

**ADR ID:** `ADR-L5-0038`  
**Title:** Custom Credential API, Server Session & MFA Profile  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-01 — IAM Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-D-01-001`; `L5-D-01-003`; `L5-D-01-004`; constrains web/mobile authentication consumers  
**Related ADRs:** `ADR-L5-0024`; `ADR-L5-0034`; `ADR-L5-0027`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

Keycloak 26.x is the canonical authentication/federation platform, but the Lead Architect has selected a SutraERP-owned username/password API rather than a browser redirect-based OIDC Authorization Code + PKCE login as the default application login surface.

This choice increases credential-handling responsibility inside the SutraERP authentication boundary. The architecture therefore constrains that API so it does not create a second credential store or bypass Keycloak/IAM ownership.

## 2. Scope & Ownership

D01 owns this authentication/session profile. Canonical Person/Identity/User Account ownership remains upstream. D02 remains the authorization authority. D03 owns secret/cryptographic protection.

## 3. Decision

1. **Custom login API:** SutraERP exposes a D01-owned HTTPS credential-authentication endpoint (conceptually `POST /api/v1/auth/login`) accepting the governed login identifier and password.
2. **Keycloak remains credential authority:** the SutraERP API shall not persist password hashes, duplicate Keycloak credential state, or access Keycloak credential tables directly. Credentials are handled transiently and delegated to a supported Keycloak-side authentication mechanism/adapter.
3. **No OAuth password grant:** the architecture shall **not** use OAuth Resource Owner Password Credentials / Keycloak Direct Access Grant as the canonical realization of this API.
4. **Server-side browser session:** successful browser authentication establishes an opaque SutraERP session using `Secure`, `HttpOnly`, same-site-governed cookies. Keycloak access/refresh credentials and session-sensitive token material are not exposed to JavaScript-accessible browser storage.
5. **Session authority:** authentication session state remains D01-owned and may bind to Keycloak session/token state internally. Cookie identity alone is not authorization truth.
6. **JWT use:** short-lived JWT access tokens may be used for bounded service/API authentication where appropriate, but D02 authorization and freshness-sensitive policy/session checks remain authoritative. JWT claims shall not become permanent permission truth.
7. **Password handling:** business/domain services never receive or store user passwords. Credential processing is restricted to the D01 authentication boundary and the Keycloak-side mechanism.
8. **MFA minimum:** MFA is mandatory for privileged/high-risk roles and operations; institutional policy may require it for additional populations.
9. **MFA methods:** WebAuthn/passkey authentication is preferred for privileged users where the approved Keycloak capability is production-supported for the selected release/profile; TOTP is the broadly supported fallback. SMS OTP is not the default MFA method and requires an approved institutional/provider requirement.
10. **Federation exception:** SAML/OIDC federation remains supported for institutional identity-provider integration where required, but federation does not replace the default SutraERP custom login API unless a deployment profile explicitly selects federated sign-in.

## 4. Custom Credential API Security Constraints

- Credentials are accepted only over protected transport.
- Password values are never logged, traced, audited, cached, persisted, placed in events or exposed to downstream services.
- Credential buffers/objects are retained only for the minimum authentication operation lifetime.
- Authentication failures use generic external responses sufficient to avoid account-enumeration leakage while preserving attributable internal evidence.
- Rate limiting, abuse controls and lockout/risk controls are D01/E02/D03-governed and must not invent business account states.
- The custom API shall support MFA challenge/continuation without exposing Keycloak administrative credentials.
- CSRF/session-fixation/replay protections are mandatory for cookie-based browser session establishment and renewal.
- Session identifiers are rotated at successful authentication and other security-sensitive session transitions.

## 5. Rationale for Prohibiting Direct Access Grant

Keycloak documents Direct Access Grant / Resource Owner Password Credentials but also states current OAuth security best practice says it must not be used because it exposes user credentials to the application and removes important browser authentication-flow capabilities. SutraERP's custom API therefore requires a controlled Keycloak-side adapter/authentication integration rather than making password grant the architecture standard.

## 6. Consequences

SutraERP retains full control of the login API/UX while Keycloak remains the credential authority. The cost is a larger credential-handling attack surface than redirect-based OIDC, so the D01 boundary carries stricter no-log/no-store/isolation/testing requirements.

## 7. External Evidence

- Keycloak OIDC guidance: https://www.keycloak.org/securing-apps/oidc-layers
- Keycloak Server Administration Guide: https://www.keycloak.org/docs/latest/server_admin/
- Keycloak WebAuthn guidance: https://www.keycloak.org/docs/26.7.0/server_admin/

## 8. Approval Record

**Decision selected:** custom SutraERP username/password API; Keycloak remains credential authority; no Direct Access Grant; server-side secure cookie sessions; bounded JWT use; privileged/high-risk MFA; WebAuthn/passkeys preferred where approved/stable with TOTP fallback.  
**Approval source:** Explicit Lead Architect decision on 2026-09-22: Q1 custom username/password API; all other Group-3 recommendations approved.  
**Formal Status:** `Locked`.
