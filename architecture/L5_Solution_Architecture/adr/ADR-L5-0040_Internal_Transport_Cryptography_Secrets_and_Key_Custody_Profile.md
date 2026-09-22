# ADR-L5-0040 — Internal Transport, Cryptography, Secrets & Key Custody Profile

**ADR ID:** `ADR-L5-0040`  
**Title:** Internal Transport, Cryptography, Secrets & Key Custody Profile  
**Scope:** Repository  
**Decision Owner:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Status:** Locked  
**Decision Date:** 2026-09-22  
**Affected L5 Specifications:** `L5-D-03-001`; `L5-D-03-002`; `L5-D-03-003`; `L5-D-03-004`; `L5-D-03-005`; constrains runtime/deployment and protected persistence  
**Related ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0027`; `ADR-L5-0028`; `ADR-L5-0039`  
**Supersedes:** None  
**Superseded By:** None

## 1. Decision Context

OpenBao and rustls are already canonical, but backend implementation still needs a governed transport/encryption/key-custody profile and an explicit position on service mesh, production secret injection and key rotation.

## 2. Decision

1. **Protected transport:** all external traffic and cross-process protected service communication uses authenticated encrypted transport. **TLS 1.3 is preferred**; TLS 1.2 may remain only where an approved compatibility profile requires it.
2. **Mutual authentication:** mTLS is required where the applicable trust profile requires cryptographic workload identity at the transport boundary. It is not inferred solely from network placement.
3. **Service mesh:** no service mesh is mandatory initially. Istio, Linkerd or another mesh requires a later architecture decision if operational requirements justify the additional control plane.
4. **Production secrets:** production applications shall not store secrets in source, images or ordinary `.env`/configuration files. OpenBao is the canonical secret-management boundary; runtime delivery/injection may use platform mechanisms provided OpenBao/D03 remains authoritative.
5. **Encryption at rest:** persistent production storage uses infrastructure/storage encryption at rest. Data classifications requiring stronger separation use D03-governed field/application-level protection in addition to infrastructure encryption.
6. **Symmetric AEAD baseline:** where application-controlled symmetric authenticated encryption is required, approved modern AEAD algorithms include **AES-GCM** and **ChaCha20-Poly1305** according to the applicable security profile. Algorithm choice is purpose/profile-specific, not developer preference.
7. **Hash/integrity baseline:** SHA-256/SHA-384 family algorithms are the default approved general-purpose cryptographic hash family where appropriate. Password hashing remains Keycloak/IAM-owned and is not specified by this generic hash rule.
8. **TLS cipher/profile authority:** TLS implementation follows the approved rustls/platform TLS profile and current standards; teams shall not independently enable legacy ciphers/protocols.
9. **Key custody:** OpenBao software-managed keys are valid for normal assurance profiles. Higher-assurance signing/root/master keys use HSM or cloud/on-prem KMS-backed custody where the security/compliance profile requires it.
10. **Rotation:** there is no universal “rotate every N days” rule. Each credential/key/certificate class has a risk- and capability-governed replacement/rotation profile. Suspected compromise triggers immediate containment/revocation/replacement regardless of normal schedule.
11. **Audit separation:** security operations may emit E05 telemetry, but audit-grade security evidence is handed to D04 and protected under D04 integrity/provenance rules. Logs/metrics/traces are not the canonical audit record.

## 3. Conformance Rules

- Internal traffic is not exempt from protection merely because components are co-located.
- Secrets/private keys shall not appear in logs, traces, events or general configuration.
- Data-at-rest encryption does not replace D02 authorization or B05 privacy/masking controls.
- HSM/KMS selection remains deployment/security-profile specific; this ADR does not mandate one vendor.
- Certificate/key rotation shall preserve overlap/transition required for availability and verification where technically necessary.
- Security evidence must identify technical principal/context without exposing key/secret material.

## 4. Service-Mesh Position

The initial backend architecture deliberately avoids a mandatory mesh. D03 trust is enforced through application/runtime identity, TLS/mTLS where required, network policy/segmentation and explicit authorization. A mesh may later automate these controls but shall not become business or authorization authority.

## 5. Consequences

The backend has a concrete modern security baseline without making a particular cloud/KMS/service mesh mandatory. Higher-assurance deployments can strengthen custody without changing business architecture.

## 6. External Standards Evidence

- TLS 1.3: https://www.rfc-editor.org/info/rfc9846/
- ChaCha20-Poly1305: https://www.rfc-editor.org/rfc/rfc8439.html
- OpenBao: https://openbao.org/docs/

## 7. Approval Record

**Decision selected:** encrypted/authenticated cross-service transport; profile-driven mTLS; no mandatory service mesh; OpenBao production-secret authority; infrastructure encryption plus classification-driven field protection; modern AEAD/hash baseline; profile-driven HSM/KMS; risk-based rotation; D04 audit truth separate from E05 telemetry.  
**Approval source:** Explicit Lead Architect approval on 2026-09-22.  
**Formal Status:** `Locked`.
