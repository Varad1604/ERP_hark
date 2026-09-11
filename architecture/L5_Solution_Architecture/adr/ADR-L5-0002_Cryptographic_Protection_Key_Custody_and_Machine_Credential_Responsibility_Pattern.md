# ADR-L5-0002 — Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern

**ADR ID:** `ADR-L5-0002`  
**Title:** Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Assigned Author:** Architecture remediation for Varad Stage-1 corpus  
**Reviewers:** Independent architecture reviewer / Lead Architect required  
**Status:** Under Evaluation  
**Decision Date:** —  
**Affected L5 Specifications:** `L5-D-03-002`; `L5-D-03-003`; `L5-D-03-004`  
**Related ADRs:** `ADR-L5-0001`  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

`L5-D-03-002`, `L5-D-03-003` and `L5-D-03-004` collectively define reusable security architecture for cryptographic protection, key custody/lifecycle, secrets, certificates and machine credentials.

The three responsibilities are related but not interchangeable. Collapsing them into one technical owner would increase privilege concentration, blur custody and usage boundaries, and create duplicate ownership between protection invocation, key lifecycle, and operational credential handling.

The locked ADR methodology makes this a mandatory ADR because it materially defines cryptographic, key and secrets architecture across multiple specifications.

The ADR remains technology-neutral. It does not select algorithms, key lengths, certificate authorities, KMS/HSM products, secret stores, cloud services, rotation intervals or deployment products.

---

## 2. Scope & Affected Architecture

This ADR governs the reusable D03 responsibility pattern for:

- cryptographic protection invocation and protected representation;
- key generation/provisioning, custody, activation, rotation/versioning, recovery, revocation, retirement and destruction;
- secrets, certificates and machine credentials used by applications/services/integrations;
- separation between protection use and key custody;
- fail-closed behavior where required protection/key/credential material cannot be resolved safely;
- purpose/scope binding of cryptographic material and machine credentials.

Canonical ownership remains:

- `L5-D-03-002` — cryptographic and data-protection realization;
- `L5-D-03-003` — key lifecycle, custody, separation and recovery architecture;
- `L5-D-03-004` — application/integration secrets, certificates and machine credentials.

---

## 3. Authoritative Constraints

1. Cryptographic controls must consume authoritative classification/protection requirements rather than invent business sensitivity rules.
2. Key custody must remain separate from ordinary business-domain ownership and from protection invocation.
3. Required protection must not silently downgrade to plaintext/unprotected execution.
4. Key/credential material must be purpose- and scope-bound; broad reusable credentials are non-conformant where narrower authority is required.
5. Compromise response must support revocation/rotation/replacement without redefining upstream business lifecycles.
6. Tenant/Institution boundaries and canonical owner boundaries must be preserved where material is scoped by them.
7. Secrets, certificates and machine credentials are technical credentials; they do not create business identity or approval authority.
8. Product and algorithm selection remain governed later through technology-selection decisions where needed.

---

## 4. Decision Criteria

The selected pattern must maximize:

- separation of duties;
- least privilege;
- reduced blast radius;
- replaceability of cryptographic technologies;
- provider neutrality;
- safe lifecycle management;
- explicit compromise handling;
- testability and auditability;
- compatibility with on-premises and cloud-capable deployment.

---

## 5. Alternatives Considered

### Alternative A — One combined security store/service owns cryptography, keys and all machine credentials

**Rejected.** It concentrates privileges, obscures ownership boundaries and makes compromise impact unnecessarily broad.

### Alternative B — Each application/domain owns its own cryptographic keys/secrets lifecycle independently

**Rejected.** It fragments security policy, duplicates lifecycle logic and creates inconsistent custody/rotation/recovery behavior.

### Alternative C — Shared D03 pattern with separated canonical responsibilities

Protection invocation, key custody/lifecycle and machine-credential handling remain separate but coordinated D03 responsibilities with explicit handoffs.

**Selected.**

---

## 6. Evaluation

| Criterion | A — Combined Security Owner | B — Per-App Ownership | C — Separated D03 Responsibilities |
|---|---|---|---|
| Separation of duties | Weak | Weak | Strong |
| Consistency | Acceptable | Weak | Strong |
| Least privilege | Weak | Acceptable | Strong |
| Replaceability | Weak | Weak | Strong |
| Blast-radius control | Weak | Acceptable | Strong |
| Cross-spec reuse | Strong | Weak | Strong |
| Ownership clarity | Weak | Weak | Strong |

Alternative C best satisfies the repository's ownership and security-integrity requirements.

---

## 7. Decision

SutraERP shall use a **separated but coordinated D03 responsibility pattern**:

1. `L5-D-03-002` determines and performs the required cryptographic/data-protection operation using authoritative protection requirements and canonical key references.
2. `L5-D-03-003` exclusively owns technical key lifecycle, custody, scope/purpose binding, activation, rotation/versioning, recovery, revocation, retirement and destruction semantics.
3. `L5-D-03-004` exclusively owns application/integration secrets, certificates and machine credentials, including governed provisioning, retrieval, rotation/replacement and compromise containment.
4. Cryptographic operation shall not imply ownership of key custody.
5. Machine/service credentials shall not imply business identity or authorization authority.
6. Required protection shall fail closed where required key/credential material cannot be safely resolved.
7. No D03 specialization may bypass Tenant/Institution, authorization, canonical business ownership or audit/evidence boundaries.
8. Algorithms, products, exact key sizes, exact validity periods and concrete infrastructure remain outside this ADR unless later governed by a separate technology-selection ADR.

---

## 8. Consequences

### Positive

- clear separation of duties;
- reduced privilege concentration;
- consistent key/credential lifecycle semantics;
- smaller compromise blast radius;
- improved technology replaceability;
- one reusable family pattern for downstream D03 consumers.

### Negative / Cost

- explicit interfaces/handoffs are required among D03-002/003/004;
- runtime implementations may need multiple cooperating controls rather than one all-powerful security component;
- lifecycle evidence must correlate across protection, key and credential responsibilities.

---

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| D03-002 starts owning key lifecycle | ADR and conformance rules keep custody/rotation/recovery in D03-003. |
| D03-004 credentials become a parallel IAM system | Machine credentials remain technical and do not own human identity/authentication semantics. |
| Product-specific assumptions leak into architecture | No product/algorithm/interval is canonical without later governed selection. |
| Circular dependency among D03 specializations | Use stable responsibility contracts; D03-001 is the common baseline and peer specializations collaborate without requiring mutual finalization. |
| Required protection is bypassed during dependency failure | Fail closed; no silent downgrade. |

---

## 10. Migration / Exit Position

This ADR selects a responsibility pattern, not a technology product.

Future migration may replace cryptographic libraries, KMS/HSM products, certificate infrastructure or secret-management products provided the replacement preserves:

- responsibility separation;
- key-custody boundaries;
- purpose/scope binding;
- fail-closed protection;
- compromise response;
- canonical ownership.

A change that alters those principles requires a new ADR and explicit supersession.

---

## 11. Traceability

Primary downstream realization:

- `L5-D-03-002_Cryptography_and_Data_Protection_Architecture.md`
- `L5-D-03-003_Key_Management_Architecture.md`
- `L5-D-03-004_Secrets_Certificates_and_Machine_Credential_Architecture.md`

Related family baseline:

- `ADR-L5-0001`
- `L5-D-03-001_Security_Architecture_and_Trust_Boundary_Baseline.md`

This ADR does not replace selective exact L4 BH traceability where a behavioral obligation materially requires one of these technical contributions.

---

## 12. Approval Record

**Current disposition:** Architecture decision fully evaluated and recommended for approval.  
**Formal status:** `Under Evaluation`.  
**Reason not marked Architecture Approved:** the locked ADR methodology requires an independent architecture reviewer/approver; the authoring/remediation action cannot self-approve the ADR.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| D03 technical decision owner | Recommended | 2026-09-11 | D03 Stage-1 architecture review/remediation |
| Independent architecture reviewer / Lead Architect | Pending | — | Required before `Architecture Approved` |
