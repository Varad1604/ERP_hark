# L5-D-03-002 — Cryptography & Data Protection Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Canonical Technical Responsibility:** Define reusable cryptographic and technical data-protection architecture for SutraERP data and protected technical exchanges, including protection boundaries, cryptographic-operation responsibilities and protected-data handling, without owning business data-classification policy, key custody, secrets/certificates, authorization policy or business authority.  
**Technical Architecture Owner:** L5-D-03-002 — Cryptography & Data Protection Architecture  
**Assigned Author:** Varad  
**Version:** `1.2.1`  
**Status:** Review  
**Last Updated:** `2026-09-22`  
**Primary Dependencies:** `L5-D-03-001`, `L5-A-04`, applicable canonical persistence/runtime boundaries, and authoritative protection requirements.  
**Collaborates With:** `L5-D-03-003`, `L5-D-03-004`, `L5-D-02`, `L5-D-04`, `L5-B-02`, `L5-B-05`, `L5-B-04-005`.  
**Applicable ADRs:** `ADR-L5-0002` — Cryptographic Protection, Key Custody & Machine Credential Responsibility Pattern (`Architecture Approved`, 2026-09-11).; `ADR-L5-0027`; `ADR-L5-0040`

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Establish the reusable technical architecture that applies required cryptographic protection without transferring key custody, credential lifecycle, privacy-policy ownership or business authority into D03-002.

### In Scope

- protection of data in transit across material trust boundaries;
- protection of stored representations where authoritatively required;
- sensitive-field/representation protection where required;
- integrity/authenticity protection for protected technical exchanges;
- controlled protection/de-protection boundaries;
- preservation of Tenant/Institution and security context;
- governed use of canonical key references;
- failure behavior when mandatory protection cannot be established;
- technical evidence of material protection operations.

### Out of Scope

- key generation/custody/rotation/recovery/destruction — `L5-D-03-003`;
- secrets/certificates/machine credentials — `L5-D-03-004`;
- authorization policy — `L5-D-02`;
- privacy/classification/retention policy — `L5-B-05` and upstream authority;
- business rules, lifecycle states, canonical events/publishers or institutional authority;
- product, algorithm, key-size, provider or numeric policy selection without governed authority.

---

## 3. Authority, Constraints & Assumptions

### 3.1 Authority Basis

| Source | Use |
|---|---|
| L5 Candidate Corpus Register 105 | Establishes D03-002 canonical responsibility. |
| L5 Architecture Family + Naming Registers | Establish canonical D03 major-area identity. |
| Locked dependency timeline | Establishes D03 as Stage-1 foundation and B05/D02 as downstream control architecture. |
| `ADR-L5-0002` | Establishes responsibility split between cryptographic operations, key custody and machine credentials. |
| `L5-D-03-001` | Supplies common trust-boundary/fail-closed baseline. |
| Applicable authoritative L4/upstream protection requirement | Establishes when protected behavior materially requires this capability. |

### 3.2 Constraints

1. D03-002 owns cryptographic protection invocation and protected representations, not key custody.
2. Required protection shall not silently downgrade to an unprotected path.
3. Plaintext exposure shall be restricted to a technical boundary that genuinely requires it.
4. Tenant/Institution scope and relevant security context shall be preserved.
5. Protected data shall not become “trusted plaintext” merely because it is inside the platform.
6. Cryptographic operation does not imply authorization to perform the underlying business action.
7. No unsupported algorithm, key size, rotation interval, provider or hardware mechanism becomes canonical here.

### 3.3 Dependency Position

`L5-D-03-003` and `L5-D-03-004` are stable peer responsibility contracts under `ADR-L5-0002`, not mutual finalization blockers.

`L5-B-05` is downstream of D03 under the locked execution timeline. B05 may determine privacy-driven protection applicability later, but its finalization is **not** a prerequisite for D03-002 to define reusable protection architecture.

---

## 4. Technical Architecture

### 4.1 Protection Model

```text
Authoritative protection requirement
        ↓
Protected-data / protected-exchange context
        ↓
D03-002 Protection Control Point
        ├── transport protection
        ├── stored-representation protection
        ├── field/representation protection
        └── integrity/authenticity protection
        ↓
Governed key capability from D03-003
        ↓
Protected representation / verified result
        ↓
Controlled downstream use
```

### 4.2 Protection Domains

| Domain | D03-002 Responsibility |
|---|---|
| Data in Transit | Establish required protection before material trust-boundary crossing. |
| Data at Rest | Produce protected representation when authoritative requirements require it. |
| Sensitive Representation | Restrict plaintext to the narrowest necessary technical boundary. |
| Protected Technical Exchange | Protect internal/external technical exchange according to trust context. |
| Integrity / Authenticity | Verify protected representation before security-sensitive acceptance where required. |

### 4.3 Cryptographic Control Plane

| Control | Owner |
|---|---|
| Protection applicability | Authoritative upstream requirement + consuming architecture |
| Cryptographic operation | **D03-002** |
| Key scope/custody/lifecycle | D03-003 |
| Secret/certificate/machine credential lifecycle | D03-004 |
| Authorization | D02 |
| Tenant/Institution context | A04 |
| Audit/evidence | D04 |
| Persistence | B02 |

### 4.4 Protected Data Lifecycle

```text
Protected input/context
  ↓
Resolve required protection
  ↓
Resolve canonical key capability
  ↓
Apply / verify protection
  ↓
Store / transmit / hand off protected representation
  ↓
At authorized technical boundary only: de-protect if required
  ↓
Perform permitted operation
  ↓
Re-protect before next mandatory protected boundary
```

### 4.5 Key Boundary

D03-002 requests governed key capability from D03-003. It shall not create an independent key lifecycle, custody store, uncontrolled key copy, rotation policy or key authority.

### 4.6 Credential Boundary

Where a protected operation needs a certificate or machine credential, D03-004 supplies that credential through its canonical lifecycle boundary. D03-002 may consume it but does not own it.

### 4.7 Context Integrity

Protection/de-protection preserves, where applicable:

- Tenant context;
- Institution context;
- technical actor/workload context;
- protection-purpose context;
- classification/privacy context supplied by its owner;
- provenance/correlation evidence context.

Caller-supplied metadata is not authoritative merely because it accompanies ciphertext/protected data.

### 4.8 Failure Model

| Failure | Required Outcome |
|---|---|
| Mandatory transport protection unavailable | Do not transmit protected information through an unprotected fallback. |
| Mandatory stored protection unavailable | Do not persist the protected representation as plaintext. |
| Key capability unavailable/ineligible | Stop affected cryptographic operation. |
| Credential/certificate invalid | Stop affected protected exchange. |
| Integrity/authenticity verification fails | Reject/contain the protected input. |
| Context inconsistent | Reject/contain; do not widen scope. |
| De-protection boundary unavailable | Do not expose plaintext through an alternate uncontrolled path. |

### 4.9 Architecture Decisions

| ID | Decision | Governance |
|---|---|---|
| AD-001 | Cryptographic protection is reusable cross-solution architecture rather than business-domain ownership. | Candidate corpus + `ADR-L5-0002`. |
| AD-002 | Mandatory protection shall not silently downgrade. | D03-001 + `ADR-L5-0002`. |
| AD-003 | Cryptographic operation and key custody are separate responsibilities. | `ADR-L5-0002`. |
| AD-004 | Plaintext exposure is constrained to the technical boundary that requires it. | D03-002 derivation + `ADR-L5-0002`. |
| AD-005 | B05 consumes D03 protection capability; B05 finalization is not a D03 prerequisite. | Locked execution timeline. |
| AD-006 | Algorithms/providers/key sizes remain governed future decisions. | ADR methodology. |

---

## 5. Dependencies / Integration

| Architecture | Relationship |
|---|---|
| D03-001 | Parent trust/security baseline. |
| D03-003 | Peer responsibility contract supplying key capability. |
| D03-004 | Peer responsibility contract supplying credential/certificate capability. |
| A04 | Supplies canonical Tenant/Institution isolation context. |
| D02 | Downstream authorization architecture consumed where a protected operation is policy-controlled. |
| B05 | Downstream privacy/sensitive-data architecture consuming D03 protection capability. |
| B02 | Persists protected representations; does not own cryptography. |
| D04 | Owns evidence architecture. |

No peer listed above must be fully approved merely to establish the ownership contract already governed by `ADR-L5-0002`.

---

## 6. Technical Behaviors

**TB-001:** Establish required transport protection before protected transmission.  
**TB-002:** Prevent silent protection downgrade.  
**TB-003:** Produce protected stored representation when required.  
**TB-004:** De-protect only at an authorized technical boundary requiring plaintext.  
**TB-005:** Verify integrity/authenticity before protected acceptance where required.  
**TB-006:** Consume key capability through D03-003.  
**TB-007:** Preserve applicable security and Tenant/Institution context.  
**TB-008:** Stop/contain the operation when mandatory protection cannot be established.  
**TB-009:** Never treat failed protected verification as successful exchange.  
**TB-010:** Expose required technical evidence without exposing plaintext/key material.

---

## 7. Security / Trust / Evidence

The architecture preserves confidentiality/integrity properties appropriate to the authoritative protection requirement while maintaining separation between business ownership, persistence, cryptographic operations and key custody.

Evidence may include protection operation class, protected context, success/failure, canonical key reference/version information and correlation/provenance, but shall not disclose protected plaintext or key material merely for audit completeness.

---

## 8. Conformance

| ID | Requirement |
|---|---|
| CR-001 | Every material path requiring cryptographic protection has an explicit D03-002 protection boundary. |
| CR-002 | Mandatory protected transport cannot silently downgrade. |
| CR-003 | Required stored-data protection produces a protected representation. |
| CR-004 | Sensitive representation exposure is bounded. |
| CR-005 | Key capability is consumed from D03-003. |
| CR-006 | Credential/certificate capability is consumed from D03-004. |
| CR-007 | Required integrity/authenticity verification occurs before protected acceptance. |
| CR-008 | Tenant/Institution context is preserved where applicable. |
| CR-009 | Mandatory protection failure stops/contains the affected operation. |
| CR-010 | Evidence excludes plaintext/key material unless separately and authoritatively required. |
| CR-011 | No unsupported crypto parameter/product is canonicalized. |
| CR-012 | The implementation conforms to `ADR-L5-0002`. |

---

## 9. Traceability

D03-002 has defensible technical purpose through the Architecture Approved L5 corpus and `ADR-L5-0002`.

Where an authoritative L4 BH materially requires D03-002 cryptographic protection, the canonical relation is:

```text
(L4 BH ID, L5-D-03-002)
```

Only materially supported relations shall be added to the canonical traceability store. Generic mentions of “encryption/security” do not justify fabricated BH mappings.

| Traceability Item | State |
|---|---|
| Canonical identity/responsibility | Verified |
| Canonical major-area naming | Verified and synchronized |
| Applicable ADR | Verified — `ADR-L5-0002` Architecture Approved |
| D03-003 / D03-004 ownership split | Verified through ADR-0002 |
| B05 dependency direction | Corrected — B05 is downstream consumer, not D03 prerequisite |
| Direct L4 BH relations | Selective mapping pending validated evidence; none invented |

---

## 10. Open Issues / Deferred Design

| ID | Issue | Blocking? |
|---|---|---|
| OD-001 | Add only defensible direct L4 BH relations to the canonical traceability store when verified. | No for Draft validity; required before a relation is `Validated`. |
| OD-002 | Reconcile material A04 changes affecting context/isolation. | Conditional. |
| OD-003 | Reconcile material D03-003/004 contract changes if ADR-0002 is amended. | Conditional. |
| OD-004 | **Partially resolved:** modern AEAD/hash/TLS baseline is governed by `ADR-L5-0040`; exact key sizes, HSM/KMS provider and certificate topology remain security-profile decisions. | Conditional remaining detail. |

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
