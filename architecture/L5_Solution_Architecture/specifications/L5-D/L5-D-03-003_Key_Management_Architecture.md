# L5-D-03-003 — Key Management Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-03-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Canonical Technical Responsibility:** Define reusable technical architecture for cryptographic key lifecycle, generation/provisioning, scope and purpose binding, custody, controlled use, rotation/version transition, suspension/revocation, recovery, retirement/destruction and separation of key-management responsibilities across SutraERP, without owning cryptographic protection operations, application secrets/certificates/machine credentials, authorization policy, business rules or business authority.  
**Technical Architecture Owner:** L5-D-03-003 — Key Management Architecture  
**Assigned Author:** Varad  
**Version:** `1.2.0`  
**Status:** Draft  
**Last Updated:** `2026-09-22`  
**Primary Dependencies:** `L5-D-03-001`, `L5-A-04`, authoritative protection requirements.  
**Collaborates With:** `L5-D-03-002`, `L5-D-03-004`, `L5-D-01`, `L5-D-02`, `L5-D-04`, `L5-B-05`.  
**Applicable ADRs:** `ADR-L5-0002` — Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern (`Architecture Approved`, 2026-09-11).

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Provide one canonical key-management boundary so cryptographic consumers do not independently generate, store, copy, rotate, recover, retire or destroy enterprise-managed key material.

### In Scope

- controlled key generation/provisioning;
- purpose and scope binding;
- custody and protection of key material;
- lifecycle eligibility and controlled use;
- versioning and rollover;
- suspension, revocation, retirement and destruction;
- controlled recovery where applicable;
- separation of duties;
- Tenant/Institution context preservation where key scope requires it;
- evidence of material key-management actions.

### Out of Scope

- cryptographic protection execution — `L5-D-03-002`;
- application/integration secrets, certificates and machine credentials — `L5-D-03-004`;
- authorization policy — `L5-D-02`;
- privacy/classification policy — `L5-B-05` and upstream authority;
- business lifecycle, business events, business rules or institutional authority;
- provider/HSM/KMS product choice, algorithms, key sizes, rotation intervals or recovery targets without governed authority.

---

## 3. Authority, Constraints & Dependency Position

### 3.1 Authority Basis

| Source | Use |
|---|---|
| L5 Candidate Corpus Register 105 | Establishes canonical key-management responsibility. |
| L5 Architecture Family + Naming Registers | Establish canonical D03 major-area identity. |
| Locked dependency timeline | Establishes D03 as Stage-1 foundation and B05/D02 as downstream control architecture. |
| `ADR-L5-0002` | Establishes D03-003 as exclusive technical key lifecycle/custody owner. |
| `L5-D-03-001` | Supplies common trust-boundary/fail-closed baseline. |
| Applicable authoritative protection requirement | Establishes the protected purpose/context for which key capability is consumed. |

### 3.2 Architectural Constraints

1. Every managed key operation remains within the canonical D03-003 boundary.
2. Consumers shall not create uncontrolled key copies or independent key lifecycles.
3. Key use requires resolvable purpose, scope, lifecycle eligibility and security context.
4. Rotation/version transition preserves explicit key identity and controlled coexistence where necessary.
5. Suspension, revocation, retirement and destruction are distinct technical conditions/actions.
6. Recovery shall not create an unmanaged alternate custody path.
7. Tenant/Institution scope shall not broaden merely because a shared technical mechanism is used.
8. Key-management evidence shall never expose protected key material.
9. No unsupported provider, hardware mechanism, algorithm, key size, interval or topology is canonicalized here.

### 3.3 Dependency Position

`L5-D-03-002` and `L5-D-03-004` are collaborating peer responsibility contracts under `ADR-L5-0002`, not promotion prerequisites requiring mutual final approval.

`L5-B-05` consumes the D03 foundation later. B05 finalization is not a prerequisite for establishing key custody/lifecycle architecture.

---

## 4. Technical Architecture

### 4.1 Architectural Model

```text
Cryptographic Consumer
        ↓ governed key-use request
Purpose / Scope Resolution
        ↓
Lifecycle Eligibility Check
        ↓
Canonical D03-003 Key Boundary
   ├── Custody
   ├── Version / Rotation
   ├── Recovery / Continuity
   └── Retirement / Destruction
        ↓ controlled key capability
D03-002 Cryptographic Operation
```

### 4.2 Conceptual Key Lifecycle

```text
Provisioned / Generated
        ↓
Registered / Pending Use
        ↓
Active
   ↙        ↘
Suspended   Revoked
      \      /
       Retired
          ↓
      Destroyed
```

These are technical key-management conditions, not business lifecycle states.

### 4.3 Generation / Provisioning

Before a managed key becomes usable, the architecture shall establish:

- key identity/reference;
- technical purpose;
- permitted scope/context;
- lifecycle record;
- controlled custody;
- activation eligibility;
- evidence of the material provisioning action without exposing key material.

Consumers shall not independently generate enterprise-controlled managed keys when the protection architecture requires canonical custody.

### 4.4 Purpose and Scope Binding

A key reference shall resolve sufficient context to prevent accidental or unauthorized cross-use. Scope may include protection purpose, Tenant/Institution context, operation class, protected-data/exchange context, key version and lifecycle eligibility.

Key reuse across contexts must be explicit and governed.

### 4.5 Custody

Protected key material remains under canonical D03-003 custody. Ordinary consuming components do not gain custody merely because they invoke cryptographic operations.

The architecture prevents uncontrolled export, logging, unmanaged copies, direct consumer lifecycle modification and bypass of scope/lifecycle checks.

### 4.6 Controlled Use

Before controlled use, the architecture establishes that the key is registered, within permitted scope, lifecycle-eligible, context-compatible and requested through the approved boundary.

D03-003 determines technical key eligibility; it does not authorize the underlying business operation.

### 4.7 Versioning and Rollover

Key versions are independently identifiable. Rollover supports successor provisioning, controlled activation, bounded predecessor/successor coexistence where necessary, explicit selection for new operations, continued controlled interpretation of historical protected material where required, and retirement of predecessor keys.

Historical key identity shall not be silently overwritten.

### 4.8 Suspension, Revocation, Retirement and Destruction

- **Suspension:** temporarily prevents eligible use.
- **Revocation:** establishes that the key is no longer eligible for the applicable use.
- **Retirement:** controlled end-of-service after permitted operational use ends.
- **Destruction:** terminal removal of key material under governed conditions.

Consumers shall not infer these states from local cache, age or failed operations.

### 4.9 Recovery / Continuity

Recovery exists only for key classes where recoverability is authoritatively allowed/required. Recovery preserves identity/version when restoring an existing key and remains subject to custody, authorization, scope and evidence controls.

Recovery shall not create an unmanaged duplicate custody path.

### 4.10 Separation of Duties

No ordinary consumer pathway shall independently combine the ability to:

- create a managed key;
- alter its scope;
- approve exceptional use;
- export protected material;
- bypass lifecycle controls;
- destroy key material without the required independent control.

Exact identity/authorization mechanisms remain D01/D02-owned.

### 4.11 Tenant / Institution Context

Where a key is scoped to Tenant or Institution context, that scope is preserved through provisioning, selection, use, rollover, recovery, retirement and destruction.

A shared key-management mechanism does not imply cross-Tenant key reuse.

### 4.12 Peer Boundaries

**D03-002:** consumes governed key capability and owns cryptographic operations.  
**D03-004:** owns secrets/certificates/machine credentials; certificate-associated key material uses an explicit D03-003 boundary without duplicating custody.  
**D03-001:** provides trust-boundary constraints.  
**D04:** owns audit/evidence architecture.

### 4.13 Failure Model

| Failure | Required Outcome |
|---|---|
| Key not found | Prevent use; expose controlled failure. |
| Key outside scope | Reject; do not broaden scope. |
| Key suspended/revoked/retired | Prevent prohibited use. |
| Unauthorized key-management action | Reject and evidence as applicable. |
| Custody protection failure | Contain use and prevent uncontrolled exposure. |
| Recovery failure | Do not fabricate/silently substitute another key. |
| Tenant/Institution context failure | Prevent use where required scope cannot be established. |
| Cryptographic operation failure | Return responsibility to D03-002; do not absorb it into key management. |

### 4.14 Architecture Decisions

| ID | Decision | Governance |
|---|---|---|
| AD-001 | D03-003 exclusively owns technical key lifecycle and custody. | `ADR-L5-0002`. |
| AD-002 | Cryptographic operation does not imply key custody ownership. | `ADR-L5-0002`. |
| AD-003 | Key use requires explicit purpose/scope/lifecycle eligibility. | D03-003 architecture + `ADR-L5-0002`. |
| AD-004 | Key versions remain independently identifiable during rollover. | D03-003 technical derivation. |
| AD-005 | Recovery never creates an uncontrolled alternate custody path. | D03-003 security derivation. |
| AD-006 | B05/D02 are downstream consumers/collaborators, not D03 finalization prerequisites. | Locked execution timeline. |
| AD-007 | Product/provider/algorithm/interval choices remain separately governed. | ADR methodology. |

---

## 5. Technical Behaviors

**TB-001:** Managed key material enters canonical custody before applicable use.  
**TB-002:** Every managed key is bound to governed purpose/scope.  
**TB-003:** Ineligible lifecycle conditions prevent prohibited use.  
**TB-004:** Key selection is governed and version-aware.  
**TB-005:** Uncontrolled key export/logging/copying is prevented.  
**TB-006:** Rollover preserves explicit version identity.  
**TB-007:** Suspension/revocation/retirement are enforced.  
**TB-008:** Recovery is controlled and identity-preserving where applicable.  
**TB-009:** Tenant/Institution scope is preserved.  
**TB-010:** Separation of incompatible key-management authorities is enforceable.  
**TB-011:** Destruction is controlled and distinct from retirement/revocation.  
**TB-012:** Material key actions expose evidence without key disclosure.

---

## 6. Security / Trust / Evidence

The architecture preserves confidentiality of key material, integrity of lifecycle state/key identity, least-privilege access, separation of duties, context isolation, controlled continuity and traceable lifecycle transitions.

Evidence may identify key reference/version, technical action, lifecycle transition, requesting identity, authorization outcome, context and success/failure, but never protected key material.

---

## 7. Conformance

| ID | Requirement |
|---|---|
| CR-001 | Managed keys have one canonical lifecycle/custody boundary. |
| CR-002 | Purpose/scope are resolvable before use. |
| CR-003 | Ineligible lifecycle conditions prevent prohibited use. |
| CR-004 | Consumers cannot obtain uncontrolled protected key copies. |
| CR-005 | Selection is governed by identity/version/scope/eligibility. |
| CR-006 | Rollover preserves explicit key versions. |
| CR-007 | Suspension, revocation, retirement and destruction remain distinct. |
| CR-008 | Recovery is controlled and does not create unmanaged custody. |
| CR-009 | Tenant/Institution context is preserved where applicable. |
| CR-010 | Separation-of-duties constraints are enforceable. |
| CR-011 | Evidence excludes protected key material. |
| CR-012 | D03-002 remains cryptographic-operation owner; D03-004 remains credential owner. |
| CR-013 | No unsupported product/algorithm/interval is canonicalized. |
| CR-014 | The implementation conforms to `ADR-L5-0002`. |

---

## 8. Traceability

D03-003 has defensible technical purpose through the Architecture Approved candidate corpus and `ADR-L5-0002`.

Where an authoritative L4 BH materially requires D03-003 key-management contribution, the canonical relation is:

```text
(L4 BH ID, L5-D-03-003)
```

Only materially supported direct relations shall be added to the canonical traceability store. No BH ID is invented in this remediation.

| Item | State |
|---|---|
| Canonical identity/responsibility | Verified |
| Canonical major-area naming | Verified and synchronized |
| Applicable ADR | Verified — `ADR-L5-0002` Architecture Approved |
| D03-002 / D03-004 ownership split | Verified by ADR-0002 |
| B05 dependency direction | Corrected — downstream consumer/collaborator |
| Direct L4 BH relations | Selective mapping pending validated evidence |

---

## 9. Open Issues / Deferred Design

| ID | Issue | Blocking? |
|---|---|---|
| OD-001 | Add only defensible direct L4 BH relations when validated evidence exists. | No for Draft validity; required before a relation is `Validated`. |
| OD-002 | Reconcile material A04 changes affecting key scope/context. | Conditional. |
| OD-003 | Product/provider/hardware custody mechanism remains separately governed. | Conditional ADR/technical decision. |
| OD-004 | Rotation/recovery/destruction parameters remain authoritative-policy decisions where required. | Conditional. |

---

**Current Status:** Draft — Remediated and ready for revalidation.

## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0027`  
OpenBao is the canonical secrets/transit-cryptography service and rustls is the default in-process Rust TLS implementation. D03-003 retains key-custody ownership and external HSM/KMS integration remains valid.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0040`  
Protected cross-process/service communication uses authenticated encrypted transport; TLS 1.3 is preferred; mTLS is profile-driven where workload identity at transport boundary is required; no service mesh is mandatory initially. OpenBao remains production-secret authority; infrastructure encryption-at-rest is baseline with classification-driven field protection; HSM/KMS custody is profile-driven; no universal rotation interval is invented.
