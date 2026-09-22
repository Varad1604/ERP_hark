# L5-D-03-004 — Secrets, Certificates & Machine Credential Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-03-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Canonical Technical Responsibility:** Define reusable technical architecture for application/integration secrets, certificates and machine credentials, including protected provisioning, controlled retrieval/use, scope binding, renewal/replacement, revocation, retirement and exposure containment, without owning human authentication, authorization policy, cryptographic key custody, business rules or business authority.  
**Technical Architecture Owner:** L5-D-03-004 — Secrets, Certificates & Machine Credential Architecture  
**Assigned Author:** Varad  
**Version:** `1.2.1`  
**Status:** Review  
**Last Updated:** `2026-09-22`  
**Primary Dependencies:** `L5-D-03-001`, `L5-A-04`.  
**Collaborates With:** `L5-D-03-002`, `L5-D-03-003`, `L5-D-03-005`, `L5-D-01`, `L5-D-02`, `L5-D-04`, applicable integration/runtime architecture.  
**Applicable ADRs:** `ADR-L5-0002` — Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern (`Architecture Approved`, 2026-09-11).; `ADR-L5-0027`; `ADR-L5-0040`

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Establish one reusable architecture for non-human credentials and certificate material so application/service/integration components do not create competing secret, certificate or machine-credential lifecycles.

### In Scope

- secret registration/provisioning and protected retrieval;
- certificate lifecycle and controlled technical use;
- machine/service credential lifecycle;
- purpose and consuming-component scope binding;
- replacement/renewal/revocation/retirement;
- exposure/compromise containment;
- Tenant/Institution context preservation where applicable;
- collaboration with cryptographic, key, segmentation and evidence architectures.

### Out of Scope

- human identity/authentication lifecycle — D01;
- authorization policy — D02;
- cryptographic operation — D03-002;
- cryptographic key lifecycle/custody — D03-003;
- logical trust segmentation — D03-005;
- business rules, business lifecycle, canonical events/publishers or institutional authority;
- provider, secret-store, CA, credential format, certificate profile, validity/rotation interval or topology selection without governed authority.

---

## 3. Authority, Constraints & Dependency Position

### 3.1 Authority Basis

| Source | Use |
|---|---|
| L5 Candidate Corpus Register 105 | Establishes D03-004 canonical responsibility. |
| L5 Architecture Family + Naming Registers | Establish canonical D03 major-area identity. |
| Locked dependency timeline | Establishes D03 as Stage-1 security foundation. |
| `ADR-L5-0002` | Establishes D03-004 as canonical owner of application/integration secrets, certificates and machine credentials. |
| `L5-D-03-001` | Supplies trust-boundary/fail-closed baseline. |

### 3.2 Architectural Constraints

1. Every credential has explicit technical consumer, purpose, scope and lifecycle authority before activation.
2. Retrieval/use occurs only through a governed technical boundary.
3. Possession of a credential does not establish business authority or Tenant/Institution scope.
4. Secrets/private credential material shall not propagate into logs, diagnostics, ordinary evidence or user-visible outputs.
5. Replacement/revocation shall preserve or strengthen trust requirements; no silent downgrade is allowed.
6. Suspected or confirmed exposure shall support scoped containment and replacement/revocation.
7. D03-003 remains the exclusive technical key lifecycle/custody owner where key material is involved.
8. No unsupported product/provider/format/interval is canonicalized here.

### 3.3 Dependency Position

D03-002, D03-003 and D03-005 are collaborating peer contracts. Their canonical responsibility boundaries are sufficiently established by the approved corpus and ADRs; mutual final approval is not a prerequisite for defining D03-004.

---

## 4. Technical Architecture

### 4.1 Credential Domains

1. **Secrets** — confidential technical values whose disclosure can enable unauthorized technical access/action.
2. **Certificates** — technical trust/identity artifacts used to establish or validate protected technical identities/exchanges.
3. **Machine Credentials** — non-human credentials representing an application, service, workload or integration endpoint.

Association among these assets does not merge lifecycle or ownership semantics.

### 4.2 Credential Control Plane

```text
Registration / Provisioning
        ↓
Purpose + Consumer + Scope Binding
        ↓
Protected Availability
        ↓
Controlled Retrieval / Use Admission
        ↓
Active Use
        ↓
Renewal / Replacement / Restriction
        ↓
Revocation / Retirement / Destruction where applicable
        ↓
Evidence
```

### 4.3 Provisioning

Provisioning establishes:

- credential identity/reference;
- intended technical consumer;
- purpose;
- trust scope/context;
- protection requirement;
- lifecycle authority;
- replacement/revocation relationship.

Missing mandatory purpose/scope/protection information causes provisioning/activation to fail closed.

### 4.4 Controlled Retrieval / Use

A consumer may retrieve/use credential material only when its technical identity, requested purpose, credential scope and lifecycle eligibility satisfy admission controls.

Credential material shall not be copied into unrelated components, ordinary configuration, logs, diagnostics or evidence channels.

### 4.5 Secret Lifecycle

```text
Registered → Provisioned → Controlled Availability → Active → Replaced → Retired/Destroyed
```

Plaintext secret material is exposed only to the minimum technical boundary requiring it.

### 4.6 Certificate Lifecycle

```text
Requested / Provisioned → Available → Active → Renewed/Replaced → Revoked/Retired
```

Certificate lifecycle does not create business authority. Certificate trust/validity is a technical prerequisite only.

### 4.7 Machine Credential Lifecycle

```text
Provisioned → Active → Restricted/Suspended where required → Replaced/Rotated → Revoked/Retired
```

Replacing a machine credential does not alter business identity/authority.

### 4.8 Scope Binding

Credential scope may bind to a technical consumer, integration relationship, trust zone, purpose, Tenant/Institution context or execution context where supported by authoritative architecture.

Technical retrievability does not broaden credential scope.

### 4.9 Key Boundary

D03-004 does not absorb cryptographic key custody. When a certificate or credential incorporates/references key material, key lifecycle/custody remains D03-003-owned and the two architectures collaborate through an explicit boundary.

### 4.10 Segmentation Boundary

D03-005 determines permitted technical communication paths. D03-004 supplies credential validity/use within those paths. A valid credential does not bypass segmentation.

### 4.11 Exposure / Compromise Containment

The architecture supports:

1. identifying affected credential scope;
2. restricting further use;
3. replacement/revocation;
4. containment of dependent technical access;
5. preserving evidence without recording the credential material;
6. restoring service through a newly governed credential path.

Containment does not change business state or authority.

### 4.12 Recovery / Continuity

Recovery restores governed availability without bypassing purpose, scope, lifecycle or protection controls. Recovery shall not create unmanaged duplicate credential material.

### 4.13 Failure Model

| Failure | Required Outcome |
|---|---|
| Missing consumer/purpose/scope | Reject provisioning/activation/use. |
| Unauthorized retrieval | Prevent retrieval/use. |
| Invalid/expired/revoked certificate or credential | Prevent affected protected operation. |
| Credential exposure | Restrict use and initiate governed replacement/revocation/containment. |
| Key lifecycle problem | Hand off to D03-003; do not bypass key controls. |
| Segmentation violation | D03-005 prevents the prohibited path. |
| Evidence requirement | Expose technical facts to D04 without secret material. |

### 4.14 Architecture Decisions

| ID | Decision | Governance |
|---|---|---|
| AD-001 | D03-004 exclusively owns application/integration secrets, certificates and machine credentials. | `ADR-L5-0002`. |
| AD-002 | Machine credentials do not create business identity/authority. | `ADR-L5-0002` + ownership preservation. |
| AD-003 | Key custody remains D03-003-owned even when credentials reference key material. | `ADR-L5-0002`. |
| AD-004 | Credential material is retrieved through controlled scope-bound boundaries, not ordinary configuration propagation. | D03-004 security derivation. |
| AD-005 | Exposure leads to scoped containment/replacement/revocation rather than ambient trust continuation. | D03-001 + D03-004. |
| AD-006 | Peer D03 files collaborate through stable responsibility contracts rather than mutual finalization. | `ADR-L5-0002`. |
| AD-007 | Product/CA/store/format/interval choices remain separately governed. | ADR methodology. |

---

## 5. Technical Behaviors

**TB-001:** Provisioning requires consumer/purpose/scope/protection context.  
**TB-002:** Retrieval/use occurs only through the governed credential boundary.  
**TB-003:** Credential material does not propagate to unrelated components/logs/diagnostics/evidence/UI.  
**TB-004:** Use preserves established technical scope.  
**TB-005:** Certificate replacement preserves required trust.  
**TB-006:** Suspension/revocation prevents prohibited subsequent use.  
**TB-007:** Exposure produces observable containment.  
**TB-008:** Recovery does not bypass lifecycle/protection/scope controls.  
**TB-009:** Credential lifecycle does not absorb human authorization, key custody or business authority.  
**TB-010:** Material lifecycle actions expose evidence without credential disclosure.

---

## 6. Security / Trust / Evidence

The architecture preserves confidentiality of secret/private credential material, lifecycle integrity, explicit purpose/scope binding, machine-identity/business-authority separation and compromise containment.

Evidence may identify credential reference, consumer, purpose/scope, lifecycle action, technical actor/context and result. Evidence shall not contain the secret/private credential value.

---

## 7. Conformance

| ID | Requirement |
|---|---|
| CR-001 | Secrets, certificates and machine credentials have canonical lifecycle boundaries. |
| CR-002 | Provisioning is controlled and scope-bound. |
| CR-003 | Retrieval/use is restricted to eligible technical consumers/purposes. |
| CR-004 | Uncontrolled propagation is prevented. |
| CR-005 | Certificate and machine-credential replacement/revocation behavior is explicit. |
| CR-006 | Exposure containment is observable. |
| CR-007 | Recovery does not bypass lifecycle/protection controls. |
| CR-008 | Key custody remains D03-003-owned. |
| CR-009 | Segmentation remains D03-005-owned. |
| CR-010 | Human authorization/business authority is not redefined. |
| CR-011 | Evidence excludes credential material. |
| CR-012 | No unsupported product/provider/profile/interval is canonicalized. |
| CR-013 | The implementation conforms to `ADR-L5-0002`. |

---

## 8. Traceability

D03-004 has defensible technical purpose through the Architecture Approved candidate corpus and `ADR-L5-0002`.

Where an authoritative L4 BH materially requires D03-004 credential architecture, the canonical relation is:

```text
(L4 BH ID, L5-D-03-004)
```

Only materially supported direct relations shall be entered into the canonical traceability store; no BH is invented in this remediation.

| Item | State |
|---|---|
| Canonical identity/responsibility | Verified |
| Canonical major-area naming | Verified and synchronized |
| Applicable ADR | Verified — `ADR-L5-0002` Architecture Approved |
| D03-003 key/credential boundary | Verified by ADR-0002 |
| Peer finalization dependency | Corrected — stable responsibility contracts, no mutual approval deadlock |
| Direct L4 BH relations | Selective mapping pending validated evidence |

---

## 9. Open Issues / Deferred Design

| ID | Issue | Blocking? |
|---|---|---|
| OD-001 | Add only defensible direct L4 BH relations when validated evidence exists. | No for Draft validity; required before relation validation. |
| OD-002 | Reconcile material A04 scope changes. | Conditional. |
| OD-003 | **Secret store resolved:** OpenBao. CA/provider/certificate-profile choices remain separately governed. | `ADR-L5-0027`; remaining choices conditional. |
| OD-004 | Credential validity/rotation/recovery timing remains authoritative-policy or ADR territory where material. | Conditional. |

---

**Current Status:** Draft — Remediated and ready for revalidation.

## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0027`  
OpenBao is the canonical secrets/transit-cryptography service and rustls is the default in-process Rust TLS implementation. D03-003 retains key-custody ownership and external HSM/KMS integration remains valid.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0040`  
Protected cross-process/service communication uses authenticated encrypted transport; TLS 1.3 is preferred; mTLS is profile-driven where workload identity at transport boundary is required; no service mesh is mandatory initially. OpenBao remains production-secret authority; infrastructure encryption-at-rest is baseline with classification-driven field protection; HSM/KMS custody is profile-driven; no universal rotation interval is invented.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Lifecycle status restored to `Review` in accordance with the locked 2026-09-16 full-corpus Draft→Review promotion decision. OpenBao/rustls and the Group-3 trust/crypto/secrets profile are inherited where applicable; this synchronization does not promote the specification beyond Review.


**Lifecycle restoration authority:** `L5_Full_Corpus_Draft_to_Review_Promotion_Decision_2026-09-16.md` — restored to `Review` during the 2026-09-22 full-corpus post-decision audit; no Architecture Approved/Locked promotion is implied.
