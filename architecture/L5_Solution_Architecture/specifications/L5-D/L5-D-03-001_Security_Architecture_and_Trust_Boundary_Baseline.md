# L5-D-03-001 — Security Architecture & Trust Boundary Baseline

## 1. Specification Metadata

**Specification ID:** `L5-D-03-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Canonical Technical Responsibility:** Define the cross-solution security architecture baseline, trust-boundary model, security-control placement principles and isolation requirements that constrain SutraERP technical architecture without owning authentication, authorization policy, cryptographic/key-management internals, audit storage, business rules or business authority.  
**Technical Architecture Owner:** L5-D-03-001 — Security Architecture & Trust Boundary Baseline  
**Assigned Author:** Varad  
**Version:** `1.3.0`  
**Status:** Locked  
**Last Updated:** `2026-09-23`  
**Primary L5 Dependencies:** L5-D-01 — IAM; L5-A-04 — Tenant & Institutional Isolation; L5-A-01 — Application & Platform Architecture; L5-E-04 — Runtime, Packaging & Deployment.  
**Collaborating / Downstream L5 Architecture:** L5-D-02 — Authorization / RBAC; L5-D-03-002; L5-D-03-003; L5-D-03-004; L5-D-03-005; L5-D-04; L5-B-05; L5-C-01; L5-C-02; L5-C-03; L5-A-07; L5-A-08.  
**Applicable ADRs:** `ADR-L5-0001` — Canonical Trust Boundary & Segmentation Pattern (`Architecture Approved`, 2026-09-11).; `ADR-L5-0027`; `ADR-L5-0040`

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification establishes the common technical security architecture for protected SutraERP execution, data access, cross-boundary interaction and trusted technical components.

```text
Authoritative business / behavioral context
        ↓
Approved technical context
        ↓
Security & trust-boundary admission
        ↓
Controlled technical execution / handoff
        ↓
Security evidence where required
```

The architecture defines technical trust and control boundaries. It does not create business security policy or institutional authority.

### 2.2 In Scope

- explicit logical trust boundaries;
- security-control placement across material technical boundaries;
- trusted, conditionally trusted and untrusted execution contexts;
- protected application, service, data and integration crossings;
- Tenant/Institution context preservation at security boundaries;
- non-transitive trust;
- privileged/restricted technical execution isolation;
- compromise containment;
- fail-closed behavior where a mandatory security prerequisite is unavailable;
- collaboration with IAM, authorization, cryptography, keys, credentials, segmentation and audit/evidence architecture.

### 2.3 Out of Scope / Non-Goals

- Person, Identity, User Account or business-identity ownership;
- authentication mechanisms and human credential lifecycle — L5-D-01;
- authorization policy, permission evaluation, RBAC or approval semantics — L5-D-02;
- cryptographic operations — L5-D-03-002;
- key lifecycle/custody — L5-D-03-003;
- secrets, certificates and machine credentials — L5-D-03-004;
- concrete application/service/network trust segmentation — L5-D-03-005;
- audit/evidence storage and audit semantics — L5-D-04;
- Tenant/Institution business semantics — L5-A-04;
- business rules, lifecycle states, canonical business events or publishers;
- physical network topology, infrastructure products or deployment products.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Authority Matrix

| Source | Status | Authority Used Here |
|---|---|---|
| L5 Candidate Document Corpus Register 105 | Architecture Approved | Establishes `L5-D-03-001` identity and cross-solution trust/security responsibility. |
| L5 Architecture Family Register | Architecture Approved | Establishes canonical major-area name `Security, Cryptography, Secrets & Key Management Architecture`. |
| L5 Naming & Document Granularity Specialization | Architecture Approved | Governs canonical L5-D-03 naming and identity. |
| L5 Dependency-Gated Parallel Authoring Timeline | Locked | Establishes D03 as Stage-1 foundation and D02 as downstream Stage-2 architecture depending on D03. |
| L5 Validation Standard | Locked | Governs hard gates, ownership, traceability and promotion. |
| L5 Technology Selection & ADR Methodology | Locked | Requires ADR governance for material trust-boundary/security decisions. |
| `ADR-L5-0001` | Architecture Approved | Establishes explicit logical trust boundaries, governed admission, non-transitive trust and logical segmentation. |
| Locked L1 architecture and amendment | Locked / Approved | Preserve Platform Foundation/security ownership and Tenant-scoped Person context. |
| Applicable L4 behavior | Per canonical L4 status | Supplies behavioral obligations when a direct D03 technical contribution is established. |

### 3.2 Architectural Constraints

1. Security controls shall preserve canonical business ownership.
2. Authentication and authorization are distinct technical responsibilities.
3. Tenant and Institution context shall not be silently dropped, substituted or widened at a protected boundary.
4. Internal placement alone shall not establish trust.
5. Trust shall be contextual and non-transitive.
6. Mandatory security-control failure shall not degrade into an unprotected path.
7. Specialized D03 mechanisms remain with their canonical owners.
8. Physical co-location shall not erase a logical trust boundary.
9. No unsupported technology, algorithm, protocol, numeric security target or deployment topology is established here.

### 3.3 Assumptions / Dependency Position

| ID | Classification | Statement | Resolution / Effect |
|---|---|---|---|
| AS-001 | Stage-1 peer dependency | D01 supplies technical identity/authentication context where identity is required. | D03 consumes the D01 boundary; it does not own identity lifecycle. |
| AS-002 | Downstream collaboration | D02 owns authorization decisions and enforcement semantics. D02 consumes D03 trust prerequisites under the locked execution timeline. | D02 finalization is **not** a prerequisite for stabilizing this D03 baseline. |
| AS-003 | Stage-1 peer dependency | A04 owns Tenant/Institution technical isolation and context semantics. | D03 preserves validated scope and must be reconciled if A04 materially changes it. |
| AS-004 | Stage-1 peer dependency | E04 owns physical runtime/deployment realization. | D03 remains logical; physical placement may strengthen but never weaken the trust model. |
| AS-005 | Traceability | Exact BH relations are recorded only when an approved/authorized L4 BH materially requires this technical contribution. | No BH is inferred solely because it mentions “security”. |

---

## 4. Technical Architecture

### 4.1 Security Architecture Model

SutraERP uses explicit logical trust boundaries and controlled crossings rather than one implicitly trusted internal execution space.

```text
External / Untrusted Context
        ↓
Protected Boundary Admission
        ↓
Application / Access Context
        ↓
Service / Execution Boundary
        ↓
Trusted or Restricted Technical Execution
        ├── Protected Data Boundary
        └── External Integration Boundary
```

This is a logical security view and does not prescribe physical topology.

### 4.2 Trust Contexts

| Context | Architectural Treatment |
|---|---|
| External / Untrusted | No inherited internal trust; validate identity/input/context as applicable. |
| Authenticated Access | Identity established; no authorization is implied. |
| Authorized Application | D02 authorization result may permit the requested operation; scope remains constrained. |
| Trusted Technical Execution | Component/workload identity and permitted dependencies remain explicit. |
| Protected Data | Applicable authorization, privacy/classification and cryptographic controls are consumed. |
| External Integration | External system remains a distinct trust domain. |
| Restricted / High-Risk Execution | Stronger logical isolation, constrained admission and evidence are required where justified. |

### 4.3 Security Control Plane

| Control Point | Responsibility | Canonical Owner |
|---|---|---|
| Technical identity establishment | Establish actor/workload identity | L5-D-01 |
| Authorization decision/enforcement | Decide permitted action in context | L5-D-02 |
| Tenant/Institution isolation | Establish and preserve canonical technical scope | L5-A-04 |
| Boundary admission | Validate trust prerequisites for protected crossing | **L5-D-03-001** |
| Data protection | Cryptographic protection operations | L5-D-03-002 |
| Key management | Key lifecycle/custody | L5-D-03-003 |
| Secrets/certificates/machine credentials | Non-human credential lifecycle/use | L5-D-03-004 |
| Trust segmentation | Application/service/network logical isolation | L5-D-03-005 |
| Audit/evidence | Canonical technical evidence architecture | L5-D-04 |

### 4.4 Boundary Admission Pattern

```text
Invocation
  ↓
Resolve caller/workload identity where required
  ↓
Resolve authoritative Tenant/Institution scope where applicable
  ↓
Validate trust-boundary prerequisites
  ↓
Consume authorization outcome where policy-controlled
  ↓
Apply required cryptographic / credential / segmentation controls
  ↓
Cross boundary only if mandatory controls succeed
  ↓
Expose security evidence where required
```

Boundary admission does not itself create business permission.

### 4.5 Trust Establishment Rules

1. Trust is explicit and contextual.
2. Trust granted for one interaction does not transit to another boundary.
3. External systems remain distinct trust contexts.
4. A privileged destination does not elevate the caller’s authority.
5. Security context cannot be widened merely by crossing a technical boundary.
6. Co-location does not imply trusted equivalence.

### 4.6 Tenant / Institution Context Integrity

Where Tenant or Institution context applies, a protected crossing shall preserve and validate that context. D03 consumes the canonical A04 isolation model and does not redefine Tenant or Institution semantics.

A receiving component shall reject missing, conflicting or unsafe scope rather than infer a more permissive context.

### 4.7 Internal Component Trust

Internal components remain subject to explicit dependency and trust rules. Material protected access considers, as applicable:

- actor/workload identity;
- source and destination technical responsibility;
- permitted dependency relationship;
- Tenant/Institution scope;
- operation/purpose;
- authorization result;
- credential validity;
- cryptographic protection;
- evidence requirements;
- failure/containment behavior.

### 4.8 External Boundary Protection

Inbound external interaction is a trust transition and shall establish the applicable identity, validation, scope and authorization prerequisites before controlled internal handoff.

Outbound external interaction shall preserve disclosure/security constraints and shall not treat the external recipient as an internal trusted component.

### 4.9 Compromise Containment

When a component, credential, integration or trust relationship is compromised or no longer trustworthy, the architecture shall support restriction of the affected relationship without automatically collapsing unrelated trust zones.

Containment includes:

- blocking affected privileged crossings;
- restricting protected-resource access;
- preserving required evidence;
- preventing trust propagation;
- handing business/operational recovery decisions to their canonical owners.

Concrete segmentation realization belongs to D03-005.

### 4.10 Security Context Integrity

Security context is protected technical context. Caller-controlled assertions are not authoritative merely because they are present.

Where applicable, context includes:

- technical actor/workload identity;
- Tenant/Institution scope;
- authorization result/context;
- trust relationship;
- correlation/provenance context.

### 4.11 Failure Model

| Failure | Required Technical Outcome |
|---|---|
| Identity cannot be established where required | Do not admit the protected crossing. |
| Required authorization denied/unavailable | Do not perform the protected operation. |
| Scope/context missing or inconsistent | Reject or contain; do not reconstruct privileged context. |
| Trust relationship invalid | Prevent the crossing. |
| Required cryptographic protection unavailable | Prevent the protected operation. |
| Key/secret/credential invalid | Stop affected operation and hand off to owning D03 mechanism. |
| Boundary dependency unavailable | Fail without bypassing the boundary. |
| Component becomes untrusted | Restrict affected trust relationship and prevent propagation. |

### 4.12 Architecture Decisions

| ID | Decision | Governance |
|---|---|---|
| AD-001 | SutraERP uses explicit logical trust boundaries rather than one implicit internal trust zone. | Governed by `ADR-L5-0001`. |
| AD-002 | Trust is contextual and non-transitive. | Governed by `ADR-L5-0001`. |
| AD-003 | Tenant/Institution scope is protected security context and shall not be silently widened. | A04 constraint + `ADR-L5-0001` realization. |
| AD-004 | Authentication, authorization, cryptography, key management, credentials, segmentation and audit remain separately owned. | Approved L5 corpus ownership. |
| AD-005 | Required security-control failure is fail-closed for the affected protected operation. | `ADR-L5-0001`. |
| AD-006 | Physical placement may strengthen but shall not weaken logical trust boundaries. | `ADR-L5-0001`; E04 retains deployment ownership. |
| AD-007 | Product/protocol/algorithm/numeric selection remains outside this specification unless separately governed. | Locked ADR methodology. |

---

## 5. Dependencies / Integration

| L5 Architecture | Relationship | Boundary |
|---|---|---|
| D01 | Stage-1 peer | Supplies identity/authentication context; no D03 identity ownership. |
| D02 | Downstream collaborator/consumer | Consumes D03 trust prerequisites; retains authorization ownership. |
| A04 | Stage-1 peer | Owns Tenant/Institution technical isolation; D03 preserves validated scope. |
| A01 | Stage-1 peer | Supplies logical application/platform boundaries consumed by security architecture. |
| E04 | Stage-1 peer | Realizes logical controls in deployment/runtime without redefining D03 trust semantics. |
| D03-002 | Specialized child/peer | Cryptographic protection. |
| D03-003 | Specialized child/peer | Key lifecycle/custody. |
| D03-004 | Specialized child/peer | Secrets/certificates/machine credentials. |
| D03-005 | Specialized child/peer | Application/service/network segmentation. |
| D04 | Downstream control architecture | Owns audit/evidence architecture. |
| B05 | Downstream control architecture | Consumes D03 protection capabilities for privacy/sensitive-data concerns. |

D03-001 is the common baseline. D03-002/003/004/005 specialize it through stable responsibility boundaries; they do not require mutual final approval to establish their canonical ownership contracts.

### 5.1 Conceptual Boundary Contract

A protected boundary may consume:

- established technical identity/workload identity;
- validated Tenant/Institution context;
- requested operation/purpose;
- authorization result when required;
- trust relationship/boundary classification;
- credential/protection prerequisites;
- correlation/evidence context.

Possible technical outcomes are permitted, denied, invalid-context rejection, trust containment, or handoff to an owning security mechanism. No business event is created by this contract.

---

## 6. Technical Behavior / Failure Concerns

**TB-001 — Explicit Boundary Enforcement:** Material protected interactions cross an architecture-defined boundary.  
**TB-002 — Context Preservation:** Required authoritative scope is preserved and validated.  
**TB-003 — Identity/Authorization Separation:** Authentication does not imply authorization.  
**TB-004 — Non-Transitive Trust:** One admitted relationship does not create unrelated trust.  
**TB-005 — Governed Admission:** Required prerequisites must succeed before protected crossing.  
**TB-006 — Fail-Closed Protection:** A missing mandatory security prerequisite does not create bypass.  
**TB-007 — Compromise Containment:** Invalid trust can be isolated without automatically collapsing unrelated zones.  
**TB-008 — Evidence Enablement:** Material boundary outcomes expose required technical evidence to D04.

Security failure handling remains separate from business recovery. D03-001 does not invent business retries, compensation, lifecycle transitions or approval decisions.

---

## 7. Security / Trust / Evidence

The baseline requires explicit trust boundaries, least necessary trust, identity/authorization separation, scope preservation, non-transitive trust, protected external boundaries, compromise containment and no silent bypass of mandatory controls.

Where D04 requires evidence, the boundary shall expose sufficient technical facts to identify the actor/workload context, applicable scope, boundary decision, trust context, failure class and correlation/provenance information without creating a new business event.

Exact evidence structure/storage remains D04-owned.

---

## 8. Conformance

| ID | Requirement |
|---|---|
| CR-001 | Material protected technical boundaries are explicit. |
| CR-002 | Protected boundaries do not rely solely on implicit internal trust. |
| CR-003 | Authentication and authorization remain distinct. |
| CR-004 | Required Tenant/Institution context is not silently dropped or widened. |
| CR-005 | Trust is non-transitive across unrelated components/boundaries. |
| CR-006 | Mandatory security prerequisites fail closed for the affected operation. |
| CR-007 | External systems and external-originated inputs remain distinct trust contexts. |
| CR-008 | Security-sensitive workload identity is attributable through D01 where required. |
| CR-009 | Material boundary outcomes support D04 evidence where applicable. |
| CR-010 | D03-001 creates no business rule, business lifecycle, event publisher or institutional authority. |
| CR-011 | Specialized D03 mechanisms remain with D03-002/003/004/005. |
| CR-012 | No unsupported product, protocol, algorithm, topology or numeric target is canonicalized here. |
| CR-013 | D02 is treated as downstream authorization architecture, not a finalization prerequisite for the Stage-1 D03 baseline. |
| CR-014 | The implementation preserves `ADR-L5-0001`. |

---

## 9. Traceability

### 9.1 Traceability Basis

`L5-D-03-001` has defensible repository-level technical purpose through the Architecture Approved candidate corpus, canonical family/naming registers, locked Stage-1 execution timeline and `ADR-L5-0001`.

Where an approved or governance-authorized L4 BH materially requires the trust-boundary contribution, the canonical relation is:

```text
(L4 BH ID, L5-D-03-001)
```

No BH relation shall be inferred merely because an L4 document contains generic security wording.

### 9.2 Current Traceability State

| Item | State |
|---|---|
| Canonical L5 identity/responsibility | Verified |
| Canonical D03 major-area naming | Verified and synchronized |
| Stage-1 placement | Verified |
| Applicable shared ADR | Verified — `ADR-L5-0001` Architecture Approved |
| Direct L4 BH relations | To be recorded selectively when materially established; none invented in this remediation |
| D02 relationship | Corrected — downstream collaborator/consumer, not Stage-1 prerequisite |

The absence of an invented BH mapping does not make the canonical D03 technical responsibility orphaned. Complete repository BH coverage remains governed by the canonical L4-to-L5 traceability store.

---

## 10. Open Issues / Deferred Design

| ID | Type | Issue | Blocking? |
|---|---|---|---|
| OD-001 | Traceability | Populate only defensible direct L4 BH→D03-001 relations in the canonical traceability store when validated evidence is available. | No for Review architecture validity; required before any claimed relation becomes `Validated`. |
| OD-002 | Stage-1 peer | Reconcile material D01 changes affecting identity context. | Only if D01 materially changes the consumed boundary. |
| OD-003 | Stage-1 peer | Reconcile material A04 changes affecting Tenant/Institution scope. | Only if A04 materially changes the consumed boundary. |
| OD-004 | Stage-1 peer | Reconcile material E04 changes affecting realization while preserving logical trust semantics. | No for logical baseline; required for implementation readiness where placement is material. |
| OD-005 | Technology | Product/protocol/topology choices remain subject to separate ADR/technical governance where material. | Conditional. |

---

**Production Status:** Review — Remediated and ready for revalidation.  
**Next controlled action:** Revalidate hard gates against the current Stage-1 peer architecture and populate only defensible canonical L4 BH relations; do not wait for downstream D02 or B05 finalization.

## 2026-09-22 Locked Technology Decision Synchronization

**Applicable locked decision:** `ADR-L5-0027`  
OpenBao is the canonical secrets/transit-cryptography service and rustls is the default in-process Rust TLS implementation. D03-003 retains key-custody ownership and external HSM/KMS integration remains valid.


## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0040`  
Protected cross-process/service communication uses authenticated encrypted transport; TLS 1.3 is preferred; mTLS is profile-driven where workload identity at transport boundary is required; no service mesh is mandatory initially. OpenBao remains production-secret authority; infrastructure encryption-at-rest is baseline with classification-driven field protection; HSM/KMS custody is profile-driven; no universal rotation interval is invented.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Lifecycle status restored to `Review` in accordance with the locked 2026-09-16 full-corpus Draft→Review promotion decision. OpenBao/rustls and the Group-3 trust/crypto/secrets profile are inherited where applicable; this synchronization does not promote the specification beyond Review.


**Lifecycle restoration authority:** `L5_Full_Corpus_Draft_to_Review_Promotion_Decision_2026-09-16.md` — restored to `Review` during the 2026-09-22 full-corpus post-decision audit; no Architecture Approved/Locked promotion is implied.


## 2026-09-22 Production Validation Batch 4

D03 local hard gates remain PASS under `L5_D03_L4_Traceability_Extraction_and_Promotion_Gate_2026-09-11.md`. Later Locked ADRs `ADR-L5-0027` and `ADR-L5-0040` strengthen implementation readiness without transferring security ownership. This specification is **Architecture-Approval eligible**, but remains `Review` until a distinct approval decision is issued.


## 2026-09-23 R7 Lifecycle Promotion Decision

R7 issues the distinct lifecycle decision previously required by the D03 promotion gate. The specification had already passed its local HF-01…HF-15 gate and was Architecture-Approval eligible; R7 therefore promotes it from Review to Architecture Approved without changing canonical security ownership, trust boundaries, cryptographic responsibility, secret/key ownership, or business semantics. Final corpus lock remains a separate governance action.
