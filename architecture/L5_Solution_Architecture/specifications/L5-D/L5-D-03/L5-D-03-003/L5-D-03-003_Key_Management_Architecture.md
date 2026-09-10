# L5-D-03-003 — Key Management Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-03-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography & Trust Architecture  
**Canonical Technical Responsibility:** Define the reusable technical architecture governing cryptographic key lifecycle, generation/provisioning, scope and purpose binding, custody, controlled use, rotation/version transition, suspension/revocation, recovery, retirement/destruction and separation of key-management responsibilities across SutraERP, without owning cryptographic data-protection operations, application secrets/certificates/machine credentials, authorization policy, business rules or business authority.  
**Technical Architecture Owner:** L5-D-03-003 — Key Management Architecture  
**Assigned Author:** Varad  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-10`  
**Depends On:** L5-D-03-001 Security Architecture & Trust Boundary Baseline; L5-D-03-002 Cryptography & Data Protection Architecture; L5-D-03-004 Secrets, Certificates & Machine Credential Architecture; L5-A-04 Tenant & Institution Isolation; L5-B-05 Privacy & Sensitive Data; applicable approved L4 System Behavior Specifications; L5 Governance and Architecture Standard; L5 Validation Standard; L5 Solution Architecture Specification Template v1.0.3.  
**Referenced By:** L5-D-03-001; L5-D-03-002; L5-D-03-004; L5-D-03-005; L5-D-04-001/002; L5-B-04-005; L5-B-05-001 through L5-B-05-005; applicable integration and runtime specifications.  
**Applicable ADRs:** None identified for this draft. Material key-management technology, custody and deployment choices remain subject to governed ADR treatment.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification establishes the common technical architecture for governing cryptographic key material throughout its technical lifecycle. It provides a single reusable key-management boundary that cryptographic consumers can depend on without independently generating, storing, copying, selecting, recovering or retiring key material.

The architecture separates key authority from cryptographic operations. L5-D-03-002 owns the technical application of cryptographic protection; this specification owns the technical lifecycle and custody of the key material required by those operations.

### In Scope

- key lifecycle architecture from controlled generation/provisioning through retirement and destruction;
- key purpose, scope and protection-context binding;
- controlled key generation and provisioning boundaries;
- key custody and protection of key material;
- controlled key activation and use eligibility;
- key reference and selection principles for cryptographic consumers;
- versioning and controlled rollover/transition between key versions;
- suspension, revocation and retirement of key material;
- recovery and continuity requirements for recoverable key classes;
- separation of duties and independent control of key-management responsibilities;
- prevention of uncontrolled key copies and direct consumer access to protected key material;
- isolation of key-management scope across established Tenant/Institution contexts where applicable;
- interaction with cryptographic operations, secrets/certificates and security trust boundaries;
- technical evidence required to demonstrate material key-management actions.

### Out of Scope / Non-Goals

- cryptographic algorithms, key sizes or cryptographic primitive selection not established by an approved technical standard or ADR;
- encryption/decryption/signing/verification operation architecture owned by L5-D-03-002;
- application secrets, certificates and machine credentials owned by L5-D-03-004;
- authentication and authorization policy owned by L5-D-01 and L5-D-02;
- business data classification, retention, consent and privacy policy owned by L5-B-05;
- Tenant/Institution business ownership or business-scope semantics;
- business lifecycle states, business events, publishers, business rules or institutional authority;
- database schemas, persistence models, key-store products or infrastructure topology;
- provider, HSM, KMS or other product selection unless separately governed by an approved ADR;
- fixed rotation intervals, recovery time targets or retention periods unless established by authoritative governance;
- business approval or authorization of protected business outcomes.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-D-03-003` | Canonical specification identity and responsibility for key generation, custody, rotation, scope, recovery and separation requirements. | Defines technical scope and ownership. |
| L5 Governance | `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md` | v1.0.0 / Locked | Stage 1 — `L5-D-03` | D03 is the security/trust/cryptography foundation and D03 work is dependency-gated. | Establishes authoring sequence. |
| L5 Governance | `L5_Solution_Architecture_Specification_Template_v1.0.md` | v1.0.3 / Locked | Sections 1–10 | L5 must define sufficient technical architecture while preserving upstream ownership and avoiding unsupported implementation choices. | Governs structure and technical depth. |
| L5 Governance | `L5_Governance_and_Architecture_Standard.md` | Locked | L5 security and technical ownership requirements | L5 realizes approved upstream behavior without redefining business architecture. | Constrains key-management architecture. |
| L5 Governance | `L5_Validation_Standard.md` | Locked | Security, ownership, traceability and implementation-leakage requirements | L5 must remain traceable, ownership-preserving and implementation-independent where decisions are not governed. | Defines validation expectations. |
| L5 | `L5-D-03-001_Security_Architecture_and_Trust_Boundary_Baseline.md` | v1.0.0 / Draft | `§4` and dependency boundary | Establishes common trust-boundary concepts and separates key-management responsibility from the general security baseline. | Provides trust context. |
| L5 | `L5-D-03-002_Cryptography_and_Data_Protection_Architecture.md` | v1.0.0 / Draft | Key-management boundary | Cryptographic protection consumes governed key material; key lifecycle/custody belongs here. | Defines the consuming peer boundary. |
| L5 | `L5-D-03-004` | Not yet stable | Specification dependency | Secrets, certificates and machine credentials are a separate technical responsibility. | Prevents credential/key ownership overlap. |
| L5 | `L5-A-04` | Not yet stable | Specification dependency | Tenant/Institution isolation is a separate technical responsibility. | Key scope must preserve established execution context. |
| L5 | `L5-B-05` | Not yet stable | Specification dependency | Privacy and sensitive-data technical controls are separately owned. | Key protection supports, but does not define, data-policy requirements. |
| L4 | Applicable L4 System Behavior Specifications | Repository status requires controlled verification | Exact atomic BH applicability not yet established | L4 is authoritative for observable behavior realized by L5. | **Blocking traceability item:** this draft does not invent BH IDs. |

### 3.2 Architectural Constraints

1. Every key-management operation shall remain within the canonical technical key-management boundary.
2. A cryptographic consumer shall not independently create or maintain uncontrolled copies of protected key material.
3. Key use shall be bound to an explicitly resolved technical purpose, scope and protection context before material use is permitted.
4. Key lifecycle state shall be technically enforceable and distinct from business lifecycle state.
5. Key rotation or rollover shall preserve controlled coexistence of required versions without silently changing the business meaning of protected data.
6. Retirement, revocation and destruction shall be distinguishable technical actions; a key shall not be treated as destroyed merely because new use is prevented.
7. Recovery shall be controlled and shall not create an uncontrolled second custody path.
8. Exact algorithms, key sizes, intervals, providers and storage technologies shall not be invented where authoritative decisions are absent.
9. Tenant/Institution context shall not be broadened or weakened by key-management reuse.
10. Material key-management actions shall produce sufficient technical evidence for security and audit consumers without exposing key material.

### 3.3 Assumptions / Approval Blockers

- Exact atomic L4 applicability remains to be extracted and validated before promotion.
- Current repository L4 files require controlled status verification; this draft therefore does not describe them as approved merely by assumption.
- D03-001, D03-002, D03-004 and A04 remain dependency-gated peer specifications and their final boundaries require reconciliation before promotion.
- No repository-approved technical choice currently establishes a specific key-management product, provider, hardware security mechanism, algorithm, key size or rotation interval for this draft.

---

## 4. Technical Architecture

### 4.1 Architectural Model

The key-management architecture is a controlled technical boundary between cryptographic consumers and protected key material:

```text
Cryptographic Consumer
        |
        | request governed key use
        v
Key Reference / Purpose Resolution
        |
        v
Key Policy & Scope Validation
        |
        v
Key Lifecycle Eligibility Check
        |
        v
Canonical Key Management Boundary
   |            |             |
   v            v             v
Custody      Lifecycle      Recovery / Continuity
   |            |             |
   +------------+-------------+
                |
                v
Controlled Key Material Use
                |
                v
Cryptographic Operation (L5-D-03-002)
```

The architecture separates:

- **key authority:** which key version is eligible for a defined technical purpose and context;
- **key custody:** where protected key material is controlled and who/what technical boundary may access it;
- **key lifecycle:** whether a key is eligible, suspended, revoked, retired or destroyed;
- **cryptographic use:** the actual cryptographic operation, owned by L5-D-03-002;
- **credential management:** application secrets, certificates and machine credentials, owned by L5-D-03-004.

### 4.2 Key Lifecycle

The technical lifecycle shall support controlled progression through these conceptual conditions:

```text
Provisioning / Generation
        ↓
Registered / Pending Use
        ↓
Active
        ↓
Suspended or Revoked (when required)
        ↓
Retired
        ↓
Destroyed
```

These are technical key-management conditions, not business lifecycle states. The exact repository terminology for technical key states remains subject to validation against the final governed implementation architecture.

A key shall not become eligible for protected use merely because key material exists. Eligibility requires successful technical registration, scope/purpose binding, custody protection and any applicable control checks.

### 4.3 Generation and Provisioning Boundary

Key generation/provisioning shall occur through the canonical key-management boundary. Consumers shall not generate enterprise-controlled keys independently when the applicable protection architecture requires centrally governed key lifecycle management.

Generation/provisioning architecture shall:

- establish the key's technical purpose;
- establish its permitted scope and context;
- associate it with the applicable lifecycle record;
- establish controlled custody before activation;
- prevent premature consumer use;
- produce evidence of the material generation/provisioning action without exposing key material.

This specification does not prescribe a particular generation algorithm, entropy mechanism, hardware component or provider.

### 4.4 Key Purpose and Scope Binding

Every managed key shall have a technically resolvable purpose and scope sufficient to prevent accidental cross-use.

Scope may include, where applicable and authorized by upstream architecture:

- protection purpose;
- Tenant context;
- Institution context;
- protected-data or exchange context;
- cryptographic operation class;
- key version;
- lifecycle eligibility.

The architecture shall reject or prevent use when the requested purpose or context is incompatible with the key's governed scope.

Key reuse across contexts shall be explicit and governed. Reuse shall never be inferred solely because two consumers can technically access the same key boundary.

### 4.5 Custody and Key Material Protection

Key material shall remain under canonical custody and shall not be exposed to ordinary application components except through the minimum controlled technical boundary required for the authorized cryptographic operation.

The architecture shall prevent:

- uncontrolled export of protected key material;
- unmanaged application copies;
- logging or evidence capture containing key material;
- persistence of key material in ordinary business-data stores unless an approved architecture explicitly establishes such handling;
- direct consumer modification of lifecycle state;
- bypass of lifecycle and scope checks.

Custody is a technical responsibility and does not constitute business ownership of the data protected by the key.

### 4.6 Key Reference and Selection

Cryptographic consumers shall resolve a governed key reference rather than independently determining custody or storing key material.

Key selection shall consider, as applicable:

1. requested protection purpose;
2. execution/security context;
3. authorized scope;
4. lifecycle eligibility;
5. required key version;
6. transition/rollover rules;
7. applicable technical policy.

A consumer shall not silently substitute a different key when the requested key is unavailable, revoked, outside scope or otherwise ineligible. Failure shall remain observable to the consuming technical architecture.

### 4.7 Activation and Controlled Use

A generated or provisioned key shall not be usable until the required lifecycle and custody conditions are satisfied.

Before use, the architecture shall establish that:

- the key is registered within the canonical key-management boundary;
- the key is within its permitted scope;
- the key is eligible for the requested operation;
- the request preserves the applicable Tenant/Institution security context;
- no suspension, revocation or retirement condition blocks use;
- the cryptographic consumer is using the approved technical boundary.

D03-003 does not authorize the business action that may cause a cryptographic operation; it only governs technical key eligibility and custody.

### 4.8 Versioning and Rollover

Key versions shall be independently identifiable so that controlled transition can occur without ambiguous key selection.

Rollover architecture shall support, where required:

- creation/provisioning of a successor key;
- controlled activation of the successor;
- bounded coexistence of predecessor and successor where technically necessary;
- explicit selection rules for new protected operations;
- continued controlled interpretation or verification of previously protected material when required by its authoritative protection policy;
- retirement of the predecessor when no longer eligible;
- evidence of the transition.

The architecture shall not silently overwrite an existing key version or mutate historical key identity.

No fixed rollover interval is defined by this specification.

### 4.9 Suspension, Revocation and Retirement

The architecture shall distinguish emergency or policy-driven prevention of new use from normal end-of-life retirement.

- **Suspension:** temporarily prevents eligible use while preserving the key as a managed object.
- **Revocation:** establishes that the key must no longer be treated as eligible for the applicable use.
- **Retirement:** establishes controlled end-of-service for a key after its permitted operational use has ended.
- **Destruction:** removes the key material according to the governed destruction process.

The transition to any condition shall be controlled and evidenced. Consumers shall not infer lifecycle state from age, local cache state or failed operations.

### 4.10 Recovery and Continuity

Recovery architecture shall exist only where the applicable key class requires recoverability. Recovery shall be governed by the same custody, authorization, scope and evidence principles as normal key management.

Recovery shall:

- preserve key identity and version where recovery is intended to restore an existing key;
- prevent creation of an ungoverned duplicate custody path;
- require controlled technical authorization;
- preserve security context;
- produce evidence of the recovery action;
- distinguish recovery from generation of a new successor key.

A key shall not be recoverable merely because recovery is technically possible; recoverability is a governed property of the applicable key class.

### 4.11 Destruction

Destruction shall be treated as a terminal technical action only after applicable retention, continuity and legal/security constraints permit it.

The architecture shall prevent ordinary consumers from directly destroying managed key material. Destruction shall be controlled, evidenced and distinguishable from retirement or revocation.

The exact destruction mechanism is intentionally not prescribed here.

### 4.12 Separation of Duties

Key-management responsibilities shall be separable so that no single ordinary consumer component can independently:

- create a managed key;
- alter its security scope;
- approve its own exceptional use;
- export protected key material;
- bypass lifecycle controls;
- destroy key material without the applicable independent control.

The exact identity/authorization mechanisms remain owned by D01/D02. This specification defines the technical separation requirement, not a new business approval model.

### 4.13 Tenant and Institution Context

Where a key is scoped to Tenant or Institution context, the key-management architecture shall preserve that scope through generation/provisioning, selection, use, rollover, recovery and retirement.

A technically reusable key-management mechanism shall not imply that a key may cross Tenant boundaries or that Institution-scoped protection may be broadened to another Institution without an established architectural basis.

This specification does not redefine the business meaning or ownership of Tenant or Institution.

### 4.14 Interaction with L5-D-03-002

The primary peer contract is:

```text
L5-D-03-002
Cryptographic Operation
        |
        | governed key-use request
        v
L5-D-03-003
Key Scope / Eligibility / Custody
        |
        | controlled key capability
        v
L5-D-03-002
Cryptographic Protection Operation
```

D03-002 remains responsible for applying cryptographic protection. D03-003 remains responsible for key lifecycle and custody. Neither specification may silently absorb the other's canonical responsibility.

### 4.15 Interaction with L5-D-03-004

Secrets, certificates and machine credentials are not automatically keys owned by this architecture. D03-004 owns the lifecycle and protection of application/integration secrets, certificates and machine credentials.

Where a certificate or machine credential incorporates or references cryptographic key material, the two specifications shall use an explicit technical boundary rather than duplicating key custody responsibilities.

### 4.16 Trust-Boundary Integration

Key-management operations shall execute only within trust boundaries permitted by L5-D-03-001. Cross-boundary requests shall preserve security context and shall not expose protected key material merely to cross a boundary.

A compromised consumer shall not acquire broader key-management authority than the technical scope explicitly granted to it.

### 4.17 Technical Decision Boundaries

The following remain intentionally open until governed decisions exist:

- key-management service/product selection;
- hardware-backed versus software-backed custody mechanisms;
- cryptographic algorithm families;
- key sizes;
- rotation/rollover intervals;
- exact destruction mechanism;
- recovery implementation mechanism;
- provider-specific interfaces;
- deployment topology.

These open choices shall not be resolved by implementation teams through silent local decisions where the choice materially changes this architecture.

---

## 5. Dependencies / Integration

### 5.1 Dependency Matrix

| Dependency | Relationship | Required Boundary |
|---|---|---|
| L5-D-03-001 | Security/trust foundation | Key-management operations remain within approved trust boundaries and preserve context. |
| L5-D-03-002 | Cryptographic consumer | Requests governed key use; performs cryptographic operation. |
| L5-D-03-004 | Peer credential architecture | Prevents overlap between managed keys and secrets/certificates/machine credentials. |
| L5-A-04 | Isolation architecture | Preserves Tenant/Institution technical context and isolation. |
| L5-B-05 | Privacy/sensitive-data architecture | Supplies applicable data-protection constraints without transferring policy ownership. |
| L5-D-01 / D02 | Identity and authorization | Establishes technical identity and authorization controls used by key-management operations. |
| L5-D-04 | Audit/evidence | Consumes evidence of material key-management actions without key disclosure. |
| Applicable L4 | Upstream behavior | Supplies authoritative observable behaviors and exact atomic applicability once verified. |

### 5.2 Integration Principles

1. Key-management consumers interact through governed technical boundaries rather than direct custody.
2. Key references shall be context-aware and lifecycle-aware.
3. Key-management failures shall not be converted into silent protection downgrades.
4. Evidence shall identify the material technical action and relevant key identity/version without exposing secret key material.
5. Cross-domain business authority remains outside this specification.

### 5.3 Failure Ownership

| Failure | Primary Technical Responsibility | Required Behavior |
|---|---|---|
| Key not found | D03-003 | Prevent use; expose observable failure. |
| Key outside scope | D03-003 | Reject use; do not broaden scope automatically. |
| Key suspended/revoked/retired | D03-003 | Prevent prohibited use; preserve lifecycle evidence. |
| Unauthorized key-management action | D01/D02 + D03-003 boundary | Reject and evidence the attempted action as applicable. |
| Custody protection failure | D03-003 | Contain use and prevent uncontrolled exposure. |
| Key recovery failure | D03-003 | Do not fabricate or silently replace the recovered key. |
| Cryptographic operation failure | D03-002 | Handle operation failure; do not transfer cryptographic responsibility to D03-003. |
| Tenant/Institution context failure | A04 + D03-003 boundary | Prevent use where required context cannot be established. |
| Evidence failure | D04 + owning technical boundary | Do not expose key material merely to produce evidence. |

---

## 6. Technical Behavior / Failure Concerns

### TB-001 — Controlled Key Provisioning
The system shall place managed key material under the canonical key-management boundary before permitting applicable cryptographic use.

### TB-002 — Purpose and Scope Binding
The system shall bind each managed key to a governed technical purpose and scope sufficient to prevent incompatible use.

### TB-003 — Lifecycle Eligibility Enforcement
The system shall prevent use of a key that is not in an eligible technical lifecycle condition for the requested operation.

### TB-004 — Controlled Key Selection
The system shall resolve key identity/version through governed key-management rules and shall not permit consumers to bypass lifecycle and scope checks.

### TB-005 — No Uncontrolled Key Export
The system shall prevent uncontrolled export, logging, copying or ordinary persistence of protected key material.

### TB-006 — Controlled Rollover
The system shall support explicit key-version transition without silently mutating or overwriting an existing key identity.

### TB-007 — Revocation and Retirement Enforcement
The system shall prevent prohibited use after suspension, revocation or retirement conditions take effect according to the applicable technical policy.

### TB-008 — Controlled Recovery
The system shall perform key recovery only through governed custody and authorization boundaries and shall preserve key identity where recovery is intended to restore an existing key.

### TB-009 — Context Preservation
The system shall preserve applicable Tenant/Institution and security context throughout key lifecycle and use decisions.

### TB-010 — Separation of Duties
The system shall technically prevent an ordinary consumer from combining incompatible key-management authorities where separation is required.

### TB-011 — Destruction Control
The system shall prevent uncontrolled destruction of managed key material and shall distinguish destruction from retirement or revocation.

### TB-012 — Key-Management Evidence
The system shall produce sufficient technical evidence for material key-management actions without disclosing protected key material.

### Failure and Recovery Principles

- **No silent substitution:** an unavailable or ineligible key shall not silently cause use of an unrelated key.
- **No silent downgrade:** inability to access required key protection shall not silently remove or weaken the applicable cryptographic protection.
- **No local authority:** a consumer shall not locally override key lifecycle state.
- **No uncontrolled recovery:** recovery shall not create an unmanaged custody path.
- **No lifecycle inference:** consumers shall rely on canonical lifecycle state rather than local assumptions.
- **No context broadening:** a key shall not become usable in another Tenant/Institution context merely because technical access exists.

---

## 7. Security / Trust / Evidence

### 7.1 Security Properties

The architecture shall preserve:

- confidentiality of protected key material;
- integrity of key lifecycle state and key identity;
- authenticity of controlled key-management actions;
- least-privilege access to key-management capabilities;
- separation of incompatible key-management duties;
- context isolation;
- resistance to uncontrolled key duplication or export;
- traceable lifecycle transitions;
- controlled continuity and recovery.

### 7.2 Evidence Requirements

Evidence for material key-management actions should establish, as applicable:

- key identity/reference and version;
- action type;
- lifecycle transition;
- technical execution context;
- requesting technical identity;
- authorization outcome where applicable;
- time/order information;
- success or failure outcome;
- relevant scope/context identifiers;
- correlation/provenance information required by the audit architecture.

Evidence shall never contain protected key material or equivalent secret representations merely for audit completeness.

### 7.3 Audit Boundary

L5-D-04 owns the canonical audit/evidence architecture. D03-003 supplies the technical facts necessary for auditable key-management actions and does not create a competing audit store or audit event ownership model.

---

## 8. Conformance

A conforming implementation architecture shall demonstrate:

| ID | Conformance Requirement |
|---|---|
| CR-001 | Managed keys have a canonical lifecycle and custody boundary. |
| CR-002 | Key purpose and scope are technically resolvable before applicable use. |
| CR-003 | Ineligible lifecycle states prevent prohibited key use. |
| CR-004 | Consumers cannot obtain uncontrolled copies of protected key material. |
| CR-005 | Key selection is governed by identity/version, scope and lifecycle eligibility. |
| CR-006 | Key rollover preserves explicit version identity and controlled transition. |
| CR-007 | Suspension, revocation, retirement and destruction are technically distinguishable. |
| CR-008 | Recovery is controlled and does not create an unmanaged custody path. |
| CR-009 | Tenant/Institution context is preserved where applicable. |
| CR-010 | Separation-of-duties requirements prevent incompatible key-management authority combinations. |
| CR-011 | Material key-management actions produce evidence without key disclosure. |
| CR-012 | The architecture does not introduce unsupported algorithms, key sizes, intervals, products or provider choices as mandatory decisions. |
| CR-013 | D03-002 remains the owner of cryptographic operations and D03-004 remains the owner of secrets/certificates/machine credentials. |
| CR-014 | Exact L4 atomic applicability and upstream status are validated before promotion. |

---

## 9. Traceability

### 9.1 Traceability Status

| Traceability Item | Status | Note |
|---|---|---|
| Canonical L5 specification identity | **Verified** | Matches frozen 105-specification register: `L5-D-03-003`. |
| Canonical responsibility | **Verified** | Key generation, custody, rotation, scope, recovery and separation are explicitly assigned to D03-003. |
| Dependency-gated placement | **Verified** | D03 is the current Varad dependency-gated sequence. |
| D03-001 security/trust boundary | **Referenced** | Current peer is Draft; final dependency reconciliation remains open. |
| D03-002 cryptographic boundary | **Referenced** | Current peer is Draft; exact peer stability remains open. |
| D03-004 credential boundary | **Referenced** | Not yet stable. |
| A04 isolation boundary | **Referenced** | Not yet stable. |
| Exact atomic L4 BH applicability | **Open — blocking** | Controlled extraction required; no BH IDs are invented in this draft. |
| Exact L4 version/status references | **Open — blocking** | Current repository status requires direct verification before promotion. |
| Business ownership / lifecycle / event preservation | **Verified at architecture boundary** | No business owner, business lifecycle or canonical event is redefined. |

### 9.2 Atomic Traceability Rule

The required traceability relation is:

`(Exact L4 BH ID, Exact L4 Version, L5-D-03-003 Version)`

Each applicable L4 behavioral obligation shall be mapped to the precise L5 behavior or architecture mechanism that realizes it. Document-level references alone are insufficient for promotion.

Until controlled extraction is completed, this draft intentionally records applicability as unresolved rather than inventing or inferring atomic BH identifiers.

### 9.3 Upstream Preservation

This specification does not:

- redefine business ownership;
- create business rules;
- create or rename business lifecycle states;
- create canonical business events or publishers;
- create institutional authority;
- change L3 outcomes;
- change L4 behavior.

Its role is to provide the technical key-management mechanisms required by approved downstream realization.

---

## 10. Open Issues / Deferred Design

| ID | Open Issue | Impact | Resolution Gate |
|---|---|---|---|
| OD-001 | Exact atomic L4 BH applicability has not yet been controlled-extracted. | Prevents complete traceability and promotion. | L4 BH extraction and validation. |
| OD-002 | Exact L4 version/status references require direct repository verification. | Prevents authoritative upstream status claims. | Controlled repository verification. |
| OD-003 | D03-001 final dependency boundary is not yet stable. | Affects trust-boundary conformance. | D03-001 validation/promotion. |
| OD-004 | D03-002 final dependency boundary is not yet stable. | Affects cryptographic/key boundary validation. | D03-002 validation/promotion. |
| OD-005 | D03-004 secret/certificate/credential boundary is not yet stable. | Potential overlap risk. | D03-004 validation. |
| OD-006 | A04 Tenant/Institution isolation boundary is not yet stable. | Affects key scope/context conformance. | A04 validation. |
| OD-007 | Specific key-management technology/custody mechanism is not governed. | Implementation choice remains deferred. | Approved ADR / technical governance decision. |
| OD-008 | Rotation, recovery and destruction parameters are not authoritatively fixed. | Prevents unsupported numeric/operational commitments. | Authoritative policy/ADR when required. |

### Draft Promotion Gate

Before this specification is promoted beyond Draft, the following shall be complete:

1. exact atomic L4 BH applicability extracted and mapped;
2. L4 source versions/statuses directly verified;
3. D03-001 and D03-002 boundary dependencies reconciled;
4. D03-004 and A04 boundaries reconciled where they affect key custody, credential handling or scope;
5. no unsupported technology/product choice introduced;
6. conformance and traceability revalidated against the current repository baseline.

**Current State:** Draft — Authoring  
**Next Controlled Action:** Complete exact L4 atomic applicability and reconcile dependency boundaries before review/promotion.
