# L5-D-03-004 — Secrets, Certificates & Machine Credential Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-03-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography & Trust Architecture  
**Canonical Technical Responsibility:** Define the reusable technical architecture governing application and integration secrets, certificates, machine credentials, their protected lifecycle, controlled retrieval and use, scope binding, renewal/replacement, revocation and exposure containment, without owning human authentication, authorization policy, cryptographic key custody, business rules or business authority.  
**Technical Architecture Owner:** L5-D-03-004 — Secrets, Certificates & Machine Credential Architecture  
**Assigned Author:** Varad  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-10`  
**Depends On:** L5-D-03-001 Security Architecture & Trust Boundary Baseline; L5-D-03-002 Cryptography & Data Protection Architecture; L5-D-03-003 Key Management Architecture; L5-D-03-005 Application, Service & Network Trust Segmentation Architecture; L5-A-04 Tenant & Institution Isolation; applicable approved L4 System Behavior Specifications; L5 Governance and Architecture Standard; L5 Validation Standard; L5 Solution Architecture Specification Template v1.0.3.  
**Referenced By:** L5-D-03-001; L5-D-03-002; L5-D-03-003; L5-D-03-005; L5-B-04-005; applicable integration, runtime and client-platform specifications.  
**Applicable ADRs:** None identified for this draft. Material credential-management technology, certificate-authority and deployment choices remain subject to governed ADR treatment.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification establishes the common technical architecture for managing non-human credentials and certificate material required by SutraERP application and integration boundaries.

It defines how secrets, certificates and machine credentials are provisioned, scoped, accessed, used, replaced, revoked and contained when compromised, while preserving separation from cryptographic key custody and from business authorization decisions.

### In Scope

- application and integration secret-management boundaries;
- certificate lifecycle and controlled use;
- machine/service credential lifecycle and scope binding;
- protected provisioning and controlled retrieval;
- least-privilege access to credential material;
- credential identity, purpose and consuming-component binding;
- renewal, replacement, revocation and retirement behavior;
- exposure detection and containment obligations;
- credential separation across trust boundaries and Tenant/Institution contexts where applicable;
- controlled handoff to cryptographic and trust-boundary architectures;
- technical evidence for material credential lifecycle operations.

### Out of Scope / Non-Goals

- human identity lifecycle, authentication and authorization policy owned by L5-D-01 and L5-D-02;
- cryptographic key generation, custody, rotation and destruction owned by L5-D-03-003;
- cryptographic data-protection operations owned by L5-D-03-002;
- application/service/network trust segmentation owned by L5-D-03-005;
- business rules, lifecycle states, canonical business events, publishers or institutional authority;
- Tenant/Institution business ownership and canonical scope semantics;
- database schema, credential-store product selection or deployment-product selection;
- algorithms, certificate profiles, validity intervals, rotation intervals or provider choices not established by an approved ADR or authoritative technical standard.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-D-03-004` | Canonical specification identity and responsibility for application/integration secrets, certificates and machine credentials. | Defines technical scope and ownership boundary. |
| L5 Governance | `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md` | v1.0.0 / Locked | Stage 1 — `L5-D-03` | D03 is the security/trust foundation and this specification is authored within the dependency-gated D03 sequence. | Establishes authoring dependency context. |
| L5 Governance | `L5_Governance_and_Architecture_Standard.md` | Locked | L5 security and implementation-independence requirements | L5 realizes approved upstream behavior without redefining business architecture. | Constrains credential architecture and technical boundaries. |
| L5 Governance | `L5_Validation_Standard.md` | Locked | Security, ownership, traceability and implementation-leakage requirements | L5 security specifications must be concrete, traceable and implementation-independent. | Defines conformance expectations. |
| L5 | `L5-D-03-001_Security_Architecture_and_Trust_Boundary_Baseline.md` | v1.0.0 / Draft | Trust zones and protected-boundary model | Establishes common security boundaries and requires controlled protection of credentials crossing or operating within them. | Provides trust-boundary context. |
| L5 | `L5-D-03-002_Cryptography_and_Data_Protection_Architecture.md` | v1.0.0 / Draft | Key/credential protection boundary | Separates cryptographic data protection from credential and key ownership. | Consumes protected credential material without taking lifecycle ownership. |
| L5 | `L5-D-03-003_Key_Management_Architecture.md` | v1.0.0 / Draft | Key-management responsibility | Separates cryptographic key lifecycle/custody from secrets and machine-credential lifecycle. | Establishes non-overlapping key boundary. |
| L5 | `L5-D-03-005` | Not yet stable | Specification dependency | Trust segmentation is separately owned. | Credential use shall respect established trust boundaries. |
| L5 | `L5-A-04` | Not yet stable | Tenant/Institution isolation dependency | Technical execution/data isolation preserves canonical Tenant/Institution context. | Credential scope shall not silently broaden across contexts. |
| L4 | Applicable L4 System Behavior Specifications | Current repository status requires controlled verification | Exact atomic BH applicability not yet established | L4 is authoritative for observable system behavior realized by L5. | **Blocking traceability item:** no unverified BH IDs are invented in this draft. |

### 3.2 Architectural Constraints

1. Every credential shall have an explicit technical purpose and consuming scope before it is made available for use.
2. Credential retrieval shall occur only within an authorized technical trust boundary and shall not expose raw material beyond the minimum required execution context.
3. Credential lifecycle operations shall be distinguishable from human identity and business authorization decisions.
4. Certificate and machine-credential replacement shall support controlled transition without silently weakening trust requirements.
5. Credential compromise or suspected exposure shall trigger containment behavior appropriate to the credential's scope and dependency impact.
6. No secret, certificate private material or machine credential shall be treated as ordinary application configuration when its compromise could cross a trust boundary.
7. Tenant/Institution context shall not be inferred from credential possession alone; canonical scope must remain governed by the applicable architecture.

### 3.3 Assumptions / Deferred Authority

- Exact L4 atomic applicability is not yet established in this draft.
- No specific secret store, certificate authority, hardware device, cloud provider or deployment mechanism is selected here.
- Exact certificate profiles, validity periods, rotation schedules and recovery targets require authoritative technical decisions or ADRs before implementation.
- D03-003 remains the authority for cryptographic key material lifecycle; D03-004 does not create a competing key-custody model.

---

## 4. Technical Architecture

### 4.1 Credential Protection Domains

The architecture distinguishes three related but separately managed technical assets:

1. **Secrets** — confidential values used by applications or integrations where disclosure can enable unauthorized technical access or operation.
2. **Certificates** — trust and identity artifacts used to establish or validate technical identities and protected exchanges.
3. **Machine credentials** — non-human credentials representing an application, service, integration endpoint or other technical actor.

The architecture may associate these assets, but association does not merge their ownership or lifecycle semantics.

### 4.2 Credential Control Plane

The reusable control plane consists of:

**Credential Registration → Purpose/Scope Binding → Protected Provisioning → Controlled Retrieval → Use Admission → Renewal/Replacement → Revocation/Retirement → Evidence**

The control plane shall maintain a canonical relationship between credential material, its intended technical consumer, its purpose, applicable trust boundary and lifecycle status.

### 4.3 Provisioning Boundary

Credential provisioning shall be controlled separately from ordinary application configuration. Provisioning shall establish:

- credential identity;
- intended consumer;
- technical purpose;
- applicable trust scope;
- lifecycle authority;
- protection requirements;
- replacement/revocation relationship where applicable.

Provisioning shall fail closed when mandatory scope, protection or ownership information is absent.

### 4.4 Controlled Retrieval and Use

A consuming technical component shall obtain credential material only when its technical identity, purpose and scope satisfy the applicable admission conditions.

Credential retrieval shall preserve the established security context and shall prevent uncontrolled propagation into unrelated components, logs, diagnostics, evidence stores or user-visible outputs.

Where cryptographic operations require keys, D03-003 remains responsible for key lifecycle/custody. Where a protected exchange requires cryptographic protection, D03-002 remains responsible for the protection operation.

### 4.5 Scope Binding

Credential scope shall be explicit rather than inferred from application convenience. Scope may bind a credential to a defined technical consumer, integration relationship, trust zone or approved execution context.

A credential shall not become valid for a broader scope merely because a consuming component can technically retrieve it.

Tenant/Institution context shall be preserved where credential use is context-sensitive, but D03-004 shall not redefine Tenant or Institution business semantics.

### 4.6 Certificate Lifecycle Architecture

Certificates shall have a controlled technical lifecycle:

**Requested/Provisioned → Available for Use → Active → Renewal/Replacement → Revoked or Retired**

The lifecycle shall support controlled replacement before the existing certificate ceases to satisfy its technical trust requirement, subject to authoritative technical policy.

Certificate validation shall distinguish certificate material from the business authority of the transaction or workflow that uses it.

### 4.7 Machine Credential Lifecycle Architecture

Machine credentials shall follow a controlled lifecycle:

**Provisioned → Active → Suspended/Restricted where required → Replaced/Rotated → Revoked/Retired**

The architecture shall support credential replacement without requiring the consuming business workflow to acquire a new business identity or authority.

Suspension or revocation shall prevent further technical use within the governed scope once the controlling condition is established.

### 4.8 Secret Lifecycle Architecture

Secrets shall be handled through a controlled lifecycle:

**Registered → Provisioned → Available for Controlled Retrieval → Active → Replaced → Retired/Destroyed**

The architecture shall prevent unnecessary plaintext persistence and uncontrolled copying. Secret values shall not be emitted into ordinary logs, diagnostic output, audit evidence or user-visible responses.

### 4.9 Separation of Duties

Credential lifecycle responsibilities shall be separated so that no single technical pathway silently combines provisioning authority, unrestricted retrieval, lifecycle modification and evidence suppression.

This is a technical separation principle; it does not create business approvers or redefine upstream authorization policy.

### 4.10 Compromise and Exposure Containment

When credential exposure is confirmed or credibly suspected, the architecture shall support:

1. identification of the affected credential scope;
2. restriction or suspension of further use where required;
3. controlled replacement or revocation;
4. containment of dependent technical access;
5. preservation of required evidence without exposing credential material;
6. restoration of service through a newly governed credential path where authorized.

Containment shall be scoped to the affected technical trust relationship and shall not silently alter business ownership or business state.

### 4.11 Recovery and Continuity Boundary

Recovery shall restore governed credential availability without bypassing protection, scope binding or lifecycle controls.

Recovery procedures shall not create uncontrolled duplicate credential material. Where recovery requires a replacement credential, the replacement shall enter the normal provisioning and lifecycle path.

### 4.12 Technical Decision Boundaries

The following remain intentionally deferred unless established by authoritative technical standards or ADRs:

- secret-store technology;
- certificate-authority technology;
- hardware-backed credential protection;
- exact credential formats;
- validity and rotation intervals;
- provider-specific recovery mechanisms;
- deployment topology.

---

## 5. Dependencies / Integration

### 5.1 Dependency Matrix

| Dependency | Relationship | Boundary |
|---|---|---|
| L5-D-03-001 | Consumes trust-boundary baseline | D03-001 defines trust context; D03-004 defines credential lifecycle/use controls. |
| L5-D-03-002 | Integrates protected credential handling | D03-002 defines cryptographic protection operations; D03-004 owns secrets/certificates/machine credentials. |
| L5-D-03-003 | Adjacent credential/key boundary | D03-003 owns cryptographic key lifecycle/custody; D03-004 does not become key authority. |
| L5-D-03-005 | Trust-boundary consumer | D03-005 defines segmentation/connectivity restrictions; D03-004 defines credential use within those boundaries. |
| L5-A-04 | Context isolation | A04 owns technical Tenant/Institution isolation; D03-004 preserves applicable context. |
| L5-B-05 | Privacy/sensitive-data controls | B05 owns data-protection policy; D03-004 provides credential protection mechanisms without redefining privacy policy. |
| L5-D-04 | Evidence | D04 owns audit/evidence architecture; D03-004 produces required technical evidence without owning audit storage. |

### 5.2 Conceptual Credential Use Contract

A credential-consuming technical path shall conceptually establish:

**Consumer Identity + Credential Purpose + Scope Context + Lifecycle Eligibility → Controlled Credential Use**

The contract is architectural, not an API or protocol definition.

### 5.3 Cross-Specification Handoff Rules

- D03-001 provides the applicable trust-boundary context.
- D03-002 consumes protected credentials where required for cryptographic protection and remains responsible for cryptographic operations.
- D03-003 provides the separate key-management boundary where key material is involved.
- D03-005 constrains where technical identities/components may communicate.
- D04 receives material evidence requirements but does not receive secret values as audit content.

---

## 6. Technical Behavior / Failure Concerns

### TB-001 — Controlled Credential Provisioning

The architecture shall prevent credential activation when required consumer, purpose, scope or protection conditions are missing.

### TB-002 — Protected Credential Retrieval

Credential material shall be retrievable only through the governed technical boundary and only for an eligible consumer and purpose.

### TB-003 — No Uncontrolled Credential Propagation

Credential material shall not be silently copied into unrelated components, logs, diagnostics, evidence stores or user-visible outputs.

### TB-004 — Scope Preservation

Credential use shall preserve the credential's established technical scope and shall reject attempts to use it outside that scope.

### TB-005 — Controlled Certificate Transition

Certificate replacement shall support a governed transition and shall not silently weaken certificate or trust requirements.

### TB-006 — Credential Revocation Enforcement

Once revocation or suspension is established as applicable, subsequent credential use shall be prevented within the governed scope.

### TB-007 — Compromise Containment

Suspected or confirmed credential exposure shall produce an observable containment path appropriate to the affected credential and trust relationship.

### TB-008 — Recovery Without Bypass

Recovery shall not bypass provisioning, protection, scope or lifecycle controls.

### TB-009 — Credential Boundary Separation

Technical credential lifecycle behavior shall not create or absorb human authorization, cryptographic key custody or business authority.

### TB-010 — Material Credential Evidence

Material provisioning, replacement, revocation and containment actions shall produce the technical evidence required by the applicable evidence architecture without recording secret material itself.

### 6.1 Failure Ownership

| Failure | Required Boundary | Owning Architecture |
|---|---|---|
| Missing credential scope/purpose | Reject activation/use | D03-004 |
| Unauthorized retrieval | Prevent retrieval | D03-004 with D02 authorization boundary where applicable |
| Certificate validation/trust failure | Prevent protected use | D03-004 with applicable trust/cryptographic architecture |
| Credential exposure | Contain and replace/revoke | D03-004 |
| Key lifecycle failure | Do not bypass key controls | D03-003 |
| Trust segmentation violation | Prevent prohibited technical path | D03-005 |
| Evidence persistence/storage concern | Preserve required evidence through canonical evidence architecture | D04 |

---

## 7. Security / Trust / Evidence

### 7.1 Security Properties

This architecture shall preserve:

- confidentiality of secret and private credential material;
- integrity of credential lifecycle state;
- explicit technical scope and purpose binding;
- separation between machine identity and business authority;
- non-disclosure of sensitive credential values through evidence channels;
- containment of compromised credentials;
- consistency with established trust boundaries.

### 7.2 Evidence Requirements

Evidence should establish, where applicable:

- credential registration/provisioning occurred under governed conditions;
- the intended technical consumer and scope were established;
- retrieval/use was admitted under applicable controls;
- replacement, suspension, revocation or retirement occurred;
- compromise containment was initiated and completed where required;
- no credential value was unnecessarily recorded in evidence.

D04 remains responsible for canonical audit/evidence architecture and storage.

---

## 8. Conformance

A conforming implementation architecture shall demonstrate:

- **CR-001:** canonical specification identity and responsibility are preserved;
- **CR-002:** secrets, certificates and machine credentials have explicit technical lifecycle boundaries;
- **CR-003:** provisioning is controlled and scope-bound;
- **CR-004:** retrieval/use is restricted to eligible technical consumers;
- **CR-005:** uncontrolled credential propagation is prevented;
- **CR-006:** certificate replacement/revocation behavior is defined without unsupported timing assumptions;
- **CR-007:** machine credential suspension/replacement/revocation behavior is defined;
- **CR-008:** compromise containment is observable;
- **CR-009:** recovery does not bypass protection or lifecycle controls;
- **CR-010:** D03-003 key custody is not absorbed by this specification;
- **CR-011:** business authorization, business rules and business authority are not redefined;
- **CR-012:** exact L4 atomic traceability is established before promotion from Draft.

---

## 9. Traceability

| Traceability Item | Status |
|---|---|
| Canonical L5 specification identity | Verified against frozen 105-specification register |
| Canonical responsibility | Verified against register responsibility for secrets, certificates and machine credentials |
| Dependency-gated placement | Verified within D03 sequence |
| D03-001 trust-boundary dependency | Present; dependency remains Draft |
| D03-002 cryptographic boundary | Present; dependency remains Draft |
| D03-003 key-management boundary | Present; dependency remains Draft |
| D03-005 trust-segmentation boundary | Open — downstream peer not yet stable |
| Exact atomic L4 BH applicability | **Open — controlled extraction required** |
| Exact L4 version/status references | **Open — controlled extraction required** |
| L1 ownership/terminology boundary | Preserved; no business ownership introduced |
| Technology/provider decisions | Intentionally deferred pending governed ADR/standard |

No unverified L4 BH IDs are asserted by this draft.

---

## 10. Open Issues / Deferred Design

- **OD-001 — Exact atomic L4 applicability:** determine every applicable L4 BH and exact L4 version before promotion.
- **OD-002 — D03-001 dependency:** reconcile final trust-boundary baseline before final dependency validation.
- **OD-003 — D03-002 dependency:** validate the final cryptographic protection boundary before implementation-readiness promotion.
- **OD-004 — D03-003 dependency:** validate the final key-management boundary and ensure no overlapping custody responsibility.
- **OD-005 — D03-005 dependency:** validate trust-segmentation interaction when D03-005 becomes stable.
- **OD-006 — Certificate authority decisions:** exact certificate-authority and certificate-profile decisions remain deferred to governed technical decisions.
- **OD-007 — Credential rotation/recovery policy:** exact timing and recovery targets remain deferred until authoritative technical policy exists.
- **OD-008 — L4 upstream status:** applicable L4 sources must be verified for approval/status before this specification can be promoted.

**Current disposition:** `Draft — Authoring`

**Next controlled action:** perform exact atomic L4 BH applicability extraction, reconcile D03 peer boundaries, and validate credential lifecycle behavior against authoritative upstream requirements before review/promotion.
