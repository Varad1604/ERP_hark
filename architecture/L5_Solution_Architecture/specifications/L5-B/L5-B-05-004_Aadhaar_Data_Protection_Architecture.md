# L5-B-05-004 — Aadhaar Data Protection Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-004  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Specialized technical protection architecture for Aadhaar-related information within authoritative privacy, security and access constraints  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.1  
**Status:** Review  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001; L5-B-05-002; L5-B-02; L5-D-02; L5-D-03; L5-D-04; applicable canonical L4 behavior  
**Referenced By:** L5-B-04, L5-C-01, L5-C-03, L5-G-01  
**Applicable ADRs:** ADR-L5-0001; ADR-L5-0002; ADR-L5-0004; ADR-L5-0005; ADR-L5-0009  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines a specialized technical protection boundary for Aadhaar-related information. It minimizes propagation of the protected identifier, requires controlled technical access, and delegates persistence, authorization, cryptography and evidence to their canonical L5 owners.

This specification does not claim or determine institutional Aadhaar eligibility, collection authority, AUA/KUA status, statutory interpretation, or a specific cryptographic product/cipher. Those must come from authoritative governance and approved L5 security decisions.

### 2.2 In Scope

- A logical Aadhaar protection boundary separating protected identifier material from ordinary operational projections.
- Use of an opaque technical reference so normal consumers need not carry the protected identifier.
- Controlled masked/protected projections derived from authoritative privacy rules.
- Consumption of L5-D-02 authorization outcomes for protected-resolution requests.
- Consumption of L5-D-03 approved cryptographic protection mechanisms.
- Persistence integration through L5-B-02 without defining physical tables or storage products.
- Attributable evidence handoff through L5-D-04.

### 2.3 Explicit Non-Goals

This specification does not own or define:

- Whether Aadhaar may, must or must not be collected for a business scenario.
- AUA/KUA registration, authentication-provider contracts or statutory business authority.
- Canonical Person ownership. Canonical Person remains owned by the Identity domain as established by locked L1 architecture; this specification does not transfer that ownership.
- Authorization permissions, MFA rules, step-up policy or privileged role definitions.
- Cryptographic algorithms, key hierarchy, HSM/KMS product selection or key-rotation schedules.
- Physical vault tables, database partitions, storage engines or service deployment topology.
- Biometric business policy unless explicitly established by authoritative upstream architecture.

---

## 3. Architectural Constraints & Ownership

### 3.1 Identity Ownership

Aadhaar-related technical references may be associated with the canonical Person/Identity context, but L5-B-05-004 is not an identity owner. Person remains canonically owned by the Identity domain; HR, Student and other domains consume identity according to existing contracts.

### 3.2 Security Ownership

L5-D-03 owns the cryptographic protection profile. L5-B-05-004 expresses the requirement that protected Aadhaar material use an approved D03 protection mechanism; it does not select AES-256-GCM, HSM, KMS, PKCS#11 or any equivalent technology.

### 3.3 Persistence Ownership

L5-B-02 owns the physical persistence model. This specification requires logical isolation and least propagation of protected Aadhaar material but does not define a table schema, column types, tablespace or replication topology.

### 3.4 Authorization Ownership

L5-D-02 owns authorization and any step-up/access-control decision. L5-B-05-004 consumes an affirmative authorization outcome where protected resolution is permitted by authoritative policy.

---

## 4. Technical Architecture

### 4.1 Logical Protection Boundary

```text
Operational business consumer
        |
        | opaque Aadhaar protection reference
        v
AadhaarProtectionGateway
        |
        +--> validate Tenant / Institution scope
        +--> resolve authoritative handling rule (B05-001)
        +--> consume authorization decision (D02)
        +--> request approved cryptographic operation (D03)
        +--> persist/retrieve through B02-owned persistence boundary
        +--> emit minimized evidence to D04
```

The `AadhaarProtectionGateway` is a logical responsibility, not a mandated microservice, RPC protocol or deployment unit.

### 4.2 Protection Reference Contract

```text
AadhaarProtectionReference {
    tenant_context_ref
    institution_context_ref
    person_identity_ref
    protected_identifier_ref
    protection_policy_ref
}
```

`protected_identifier_ref` is an opaque technical reference. Its physical key format, database representation and indexing strategy are owned by L5-B-02. It shall not be interpreted as a new canonical Person identifier.

### 4.3 Protected Identifier Handling

- Normal operational consumers use the opaque reference or an authoritative masked representation rather than the protected identifier where the governing workflow allows.
- Any protected resolution must pass D02 authorization and D03 security controls.
- No cleartext protected value is copied into logs, traces, audit evidence or unrelated projections.
- Masking format is supplied by authoritative policy or applicable L4 behavior; this specification does not invent a universal visible-digit pattern.

### 4.4 Architecture Position — Logical Isolation Without Ungoverned Technology Selection

SutraERP shall maintain a specialized logical protection boundary for Aadhaar-related information. Physical isolation, encryption algorithms, key custody, database topology and network/service segmentation are realized only through approved B02/D03 architecture and applicable ADRs.

---

## 5. Dependencies & Integration

- **L5-B-05-001:** supplies authoritative data-control references.
- **L5-B-05-002:** supplies generic masking/redaction mechanism behavior.
- **L5-B-02:** owns persistence topology and schema realization.
- **L5-D-02:** owns authorization decisions.
- **L5-D-03:** owns cryptographic protection and trust controls.
- **L5-D-04:** records attributable technical evidence.
- **Identity domain:** remains canonical owner of Person; this specification references but does not own identity.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001 — Missing Security Capability:** if required D03 protection cannot be performed, protected ingestion/resolution fails closed; no cleartext fallback.
- **TB-002 — Authorization Denial:** protected resolution is denied; this specification does not create an alternate approval path.
- **TB-003 — Scope Mismatch:** a reference from one Tenant context shall not resolve in another Tenant context.
- **TB-004 — Evidence Minimization:** D04 evidence records access outcome and decision references without raw Aadhaar values.
- **TB-005 — Unknown Policy:** do not infer masking, export or collection authority; require authoritative policy resolution.

---

## 7. Security, Privacy & Trust Boundaries

- Trust-boundary placement follows ADR-L5-0001 and D03 architecture.
- Cryptographic protection follows ADR-L5-0002 and D03-owned decisions.
- Tenant/Institution isolation follows ADR-L5-0004.
- Authorization follows D02 and ADR-L5-0009 where applicable to non-human principals.
- No D03/B02 implementation detail is promoted to a B05 canonical decision.

---

## 8. Conformance Requirements

- **CR-001:** L5-B-05-004 shall not redefine Person ownership; canonical Person remains Identity-domain owned.
- **CR-002:** Operational consumers shall use protected references/masked projections where authoritative policy permits rather than propagating raw Aadhaar values unnecessarily.
- **CR-003:** Physical persistence schema/topology shall remain owned by L5-B-02.
- **CR-004:** Cryptographic algorithm, key custody and security-control selection shall remain owned by L5-D-03 and applicable ADRs.
- **CR-005:** Authorization and step-up decisions shall remain owned by L5-D-02.
- **CR-006:** Protected resolution shall fail closed when required authorization/security context cannot be established.
- **CR-007:** Raw protected values shall not be copied into audit logs, diagnostics or monitoring data.
- **CR-008:** Tenant and Institution scope shall be preserved for all protected-reference operations.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Material Technical Contribution | Realization Status |
|---|---|---|---|
| L4-A-053 | L4-A-053-BH-006 | Applies authoritative identity-protection/masking directives through a specialized protection boundary without changing Identity ownership. | Verified Realization |
| L4-H-001 | L4-H-001-BH-001 | Supplies protected references or governed projections to external integration boundaries without unnecessary raw identifier propagation. | Verified Realization |
| L4-A-025 | L4-A-025-BH-001 | Binds authoritative classification/privacy-control references to Aadhaar-related technical handling. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Cryptographic Profile:** Delegated to L5-D-03 / ADR governance. No cipher suite is selected here.
- **OD-002 — Physical Persistence Pattern:** Delegated to L5-B-02. Logical protection does not mandate a dedicated table, database or service.
- **OD-003 — Masking Format:** Must come from authoritative privacy/regulatory policy or applicable L4 behavior; no local universal format is established.
- **OD-004 — Deployment Segmentation:** Governed by D03/ADR-L5-0001 and later deployment architecture; no internal-RPC or microservice mandate is created here.

**Current State:** Ready for Re-Review
