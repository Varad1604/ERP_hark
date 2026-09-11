# L5-D-03-002 — Cryptography & Data Protection Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography & Trust Architecture  
**Canonical Technical Responsibility:** Define the reusable cryptographic and technical data-protection architecture used to protect SutraERP data and protected technical exchanges, including protection boundaries, cryptographic-operation responsibilities, protected-data handling and integration with key-management controls, without owning business data classification policy, key custody, secrets/certificates, authorization policy or business authority.  
**Technical Architecture Owner:** L5-D-03-002 — Cryptography & Data Protection Architecture  
**Assigned Author:** Varad  
**Version:** `1.0.0`  
**Status:** Draft  
**Last Updated:** `2026-09-10`  
**Depends On:** L5-D-03-001 Security Architecture & Trust Boundary Baseline; L5-D-03-003 Key Management Architecture; L5-D-03-004 Secrets, Certificates & Machine Credential Architecture; L5-A-04 Tenant & Institution Isolation; L5-B-05 Privacy & Sensitive Data; applicable approved L4 System Behavior Specifications; L5 Governance and Architecture Standard; L5 Validation Standard; L5 Solution Architecture Specification Template v1.0.3.  
**Referenced By:** L5-B-04-005 Digital Signing, Seal, QR & Credential Integrity Architecture; L5-B-05-001 through L5-B-05-005; L5-D-03-001; L5-D-03-003; L5-D-03-004; L5-D-03-005; applicable data, integration and runtime specifications.  
**Applicable ADRs:** None identified for this draft. Material cryptographic technology choices remain subject to governed ADR treatment.

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification establishes the common technical architecture for protecting SutraERP data and protected technical exchanges through cryptographic controls and controlled data-protection mechanisms.

It defines where protection is required, which technical component is responsible for applying it, how protected data moves through trusted boundaries, and how cryptographic operations consume governed key material without transferring key ownership into this specification.

### In Scope

- logical cryptographic protection boundaries for data and protected technical exchanges;
- protection of data during transport across material trust boundaries;
- protection of stored protected data where required by authoritative policy or architecture;
- protection of sensitive data representations and selected fields where the applicable data-protection architecture requires stronger treatment;
- separation of plaintext handling from protected representations;
- cryptographic operation control and separation of duties between data protection and key management;
- integrity and authenticity protection where the protected exchange requires it;
- controlled use of canonical key-management services and key references;
- failure behavior when required cryptographic protection cannot be established;
- technical evidence needed to demonstrate application of material data-protection controls.

### Out of Scope / Non-Goals

- cryptographic key generation, custody, rotation, recovery and destruction owned by L5-D-03-003;
- application/integration secrets, certificates and machine credentials owned by L5-D-03-004;
- authentication and authorization policy owned by L5-D-01 and L5-D-02;
- business data classification, retention, consent and privacy policy owned by L5-B-05;
- Tenant/Institution business ownership and canonical scope semantics;
- document-signing business eligibility or issuance authority;
- business rules, lifecycle states, canonical business events, publishers or institutional authority;
- database schema, storage-product selection or deployment-product selection;
- cryptographic algorithms, key sizes, rotation intervals or provider choices not established by an approved ADR or authoritative technical standard.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5 Use |
|---|---|---|---|---|---|
| L5 Governance | `L5_Candidate_Document_Corpus_Register_105_2026-09-09.md` | v1.1.0 / Architecture Approved | `L5-D-03-002` | Canonical specification identity and responsibility for cryptographic primitives, encryption and approved data-protection patterns. | Defines scope and technical ownership boundary. |
| L5 Governance | `L5_Dependency_Gated_Parallel_Authoring_Execution_Timeline_2026-09-09.md` | v1.0.0 / Locked | Stage 1 — `L5-D-03` | D03 is the security/trust/cryptography foundation. | Establishes dependency-gated execution position. |
| L5 Governance | `L5_Solution_Architecture_Specification_Template_v1.0.md` | v1.0.3 / Locked | Sections 1–10 | L5 must provide concrete technical realization while remaining ownership-preserving and technology-neutral where no governed choice exists. | Governs structure and technical depth. |
| L5 Governance | `L5_Governance_and_Architecture_Standard.md` | Locked | L5 security/data-protection ownership and boundary requirements | L5 realizes approved upstream behavior without redefining business architecture. | Constrains cryptographic architecture and ownership. |
| L5 Governance | `L5_Validation_Standard.md` | Locked | Security, ownership, traceability and implementation-leakage requirements | L5 security architecture must be concrete, traceable and implementation-independent. | Defines conformance expectations. |
| L5 | `L5-D-03-001_Security_Architecture_and_Trust_Boundary_Baseline.md` | v1.0.0 / Draft | `§4` trust zones and protected-boundary model | Establishes the common trust-boundary model and delegates cryptographic protection to D03-002. | Provides the security-boundary context consumed here. |
| L5 | `L5-D-03-003` | Not yet stable | Specification dependency | Key-management responsibility is separate from cryptographic data-protection responsibility. | Required peer boundary; detailed dependency remains open. |
| L5 | `L5-D-03-004` | Not yet stable | Specification dependency | Secrets, certificates and machine credentials are separate technical responsibility. | Required peer boundary; detailed dependency remains open. |
| L5 | `L5-A-04` | Not yet stable | Specification dependency | Tenant/Institution execution isolation is a separate technical responsibility. | Protected data handling must preserve its established context. |
| L5 | `L5-B-05` | Not yet stable | Specification dependency | Privacy and sensitive-data technical controls are separately owned. | This specification provides cryptographic mechanisms consumed by that architecture. |
| L2 | `L2-E04_HR_Workflows.md` | Repository source | Security controls including encryption in transit, encryption at rest and sensitive-field protection | Establishes upstream security expectations for People & Workforce workflows. | Confirms the need for technical protection patterns; not treated as new L5 business authority. |
| L2 | `L2-H06_Alumni_Workflows.md` | Repository source | Security requirements including encryption in transit, encryption at rest and event-level traceability | Establishes upstream protection expectations for Alumni workflows. | Confirms protection applicability; detailed L4 applicability remains unresolved. |
| L2 | `L2-D03_Timetable_Workflows.md` | Repository source | Security controls including encryption in transit and encryption at rest where required by enterprise policy | Establishes upstream protection expectations for timetable workflows. | Confirms protected exchange/storage concerns; not a substitute for approved L4 authority. |
| L4 | Applicable L4 System Behavior Specifications | Current repository status requires controlled verification | Exact atomic BH applicability not yet established | L4 is the authoritative source for observable system behavior that this L5 architecture realizes. | **Blocking traceability item:** no unverified BH IDs are invented in this draft. |

### 3.2 Architectural Constraints

1. Cryptographic protection shall be applied according to the protected-data or protected-exchange requirement established by authoritative upstream policy and approved L4 behavior.
2. This specification owns the technical protection mechanism and boundary, not the business classification or privacy policy that determines the business meaning of the protected information.
3. Key material shall be consumed through the canonical L5-D-03-003 key-management responsibility; this specification shall not create a parallel key-custody architecture.
4. Secrets, certificates and machine credentials shall be consumed through L5-D-03-004 and shall not be duplicated here.
5. Cryptographic protection shall preserve Tenant and Institution security context where that context is applicable to the protected data or exchange.
6. Protected data shall not be treated as plaintext merely because a technical component is internal or trusted.
7. A failed mandatory protection operation shall not silently downgrade the protected operation to an unprotected path.
8. No unsupported algorithm, key size, rotation interval, cryptographic provider, hardware-security mechanism or numeric security target is established by this specification.
9. Technical protection shall not alter the canonical business meaning, lifecycle, ownership or authority of the protected data.

### 3.3 Assumptions / Open Dependencies

| ID | Classification | Statement | Impact / Resolution |
|---|---|---|---|
| AS-001 | Dependency assumption | L5-D-03-003 will define canonical key lifecycle and custody responsibilities consumed by cryptographic operations. | Reconcile this specification before promotion if the key-management boundary changes materially. |
| AS-002 | Dependency assumption | L5-D-03-004 will define the canonical handling of secrets, certificates and machine credentials. | Prevents duplicate credential-protection responsibility. |
| AS-003 | Dependency assumption | L5-B-05 will establish the technical privacy/data-classification controls that determine where specialized protection is required. | This specification provides protection mechanisms rather than policy ownership. |
| AS-004 | Dependency assumption | L5-A-04 will establish the technical Tenant/Institution isolation model. | Protected data paths must preserve the established context. |
| OD-001 | Open decision / blocker | Exact atomic L4 BH applicability must be extracted and reconciled from the current L4 corpus before final traceability validation. | Blocking for promotion; no invented BH mappings are used. |
| OD-002 | Open dependency / blocker | D03-003 key-management architecture is not yet stable enough to finalize key-custody handoff details. | Blocking for dependency completeness, not for the logical protection model. |
| OD-003 | Open dependency | D03-004 secrets/certificates/machine-credential boundaries are not yet stable. | Must be reconciled before downstream finalization. |
| OD-004 | Open dependency | B05 privacy/sensitive-data architecture is not yet stable. | Required to finalize policy-derived protection applicability. |

---

## 4. Technical Architecture

### 4.1 Cryptographic Protection Architecture

The architecture separates the decision that data requires protection from the mechanism that applies the protection.

```text
Authoritative protection requirement
                ↓
Protected-data / protected-exchange context
                ↓
Cryptographic protection control point
        ┌───────┼────────┐
        ↓       ↓        ↓
   Transport   Stored   Field / representation
   protection  data     protection where required
        \       |        /
         \      |       /
          ↓     ↓       ↓
       Protected representation
                ↓
     Controlled downstream use
                ↓
   De-protection only at an authorized
   technical boundary requiring plaintext
```

The control point shall not independently invent business classification or authorization policy. It consumes the applicable upstream requirement and the security context supplied by canonical peer architectures.

### 4.2 Protection Domains

| Protection Domain | Technical Responsibility | Required Boundary |
|---|---|---|
| Data in Transit | Protect information crossing a material trust boundary during transport. | Do not permit an unprotected fallback where protection is mandatory. |
| Data at Rest | Protect stored representations when authoritative policy or architecture requires protection. | Storage architecture shall receive protected representation without making cryptographic protection a persistence-owner responsibility. |
| Sensitive Field / Representation | Apply stronger protection to selected sensitive representations when required. | Plaintext exposure shall be limited to the technical boundary that requires it. |
| Protected Technical Exchange | Protect information exchanged between trusted technical components or with external systems when required by the trust model. | Protection requirements follow the trust boundary and applicable context. |
| Integrity / Authenticity Protection | Protect against unauthorized alteration or establish technical authenticity where required by the protected exchange. | Verification occurs before protected data is accepted for a security-sensitive technical use. |

### 4.3 Cryptographic Control Plane

The reusable control plane consists of distinct responsibilities:

| Control Point | Responsibility | Canonical Owner / Collaborator |
|---|---|---|
| Protection Applicability | Resolve whether the current operation requires cryptographic protection from authoritative context. | Upstream policy/L4 + L5-D-03-002 technical realization |
| Protection Invocation | Apply the required cryptographic operation to the protected representation. | L5-D-03-002 |
| Key Resolution | Obtain the required key reference/material through the canonical key-management boundary. | L5-D-03-003 |
| Secret / Credential Resolution | Obtain required secret, certificate or machine credential through the canonical credential boundary. | L5-D-03-004 |
| Context Preservation | Preserve Tenant, Institution and applicable security context through protection/de-protection. | L5-A-04 + L5-D-03-001 |
| Authorization | Determine whether the actor/context may perform the protected operation. | L5-D-02 |
| Evidence | Record technical evidence of material protection operations where required. | L5-D-04 |

### 4.4 Protected Data Lifecycle

```text
Data enters protected technical boundary
                ↓
Resolve security / protection context
                ↓
Determine required protection
                ↓
Resolve canonical cryptographic key reference
                ↓
Apply protection
                ↓
Store / transmit / hand off protected representation
                ↓
At authorized technical boundary:
resolve protection context and required key reference
                ↓
Verify / de-protect as required
                ↓
Perform only the authorized downstream operation
                ↓
Re-protect before crossing the next required boundary
```

The architecture does not require every component to de-protect data. Components shall receive the representation appropriate to their responsibility, and plaintext shall not be widened merely for convenience.

### 4.5 Transport Protection

Material trust-boundary crossings shall use the applicable cryptographic transport protection defined by the governed security architecture.

The architecture shall:

1. establish the protected transport context before protected data is transmitted;
2. prevent silent downgrade to an unprotected channel where protection is mandatory;
3. preserve relevant Tenant/Institution and security context independently of the transport mechanism;
4. ensure external integrations remain distinct trust contexts;
5. terminate or contain the protected exchange when required cryptographic protection cannot be established;
6. avoid embedding transport implementation details in business-domain architecture.

Exact protocol, algorithm and certificate topology remain unresolved until governed technical decisions establish them.

### 4.6 Stored Data Protection

Where stored data requires cryptographic protection, the persistence path shall receive protected representation through the data-protection boundary.

The architecture separates:

```text
Business data ownership
        ↓
Persistence responsibility
        ↓
Cryptographic protection boundary
        ↓
Protected stored representation
```

Cryptographic protection does not transfer ownership of the underlying business data to the security architecture. The persistence architecture remains responsible for persistence realization, while D03-002 is responsible for the cryptographic protection mechanism.

### 4.7 Sensitive Representation Protection

Where an authoritative privacy/data-protection requirement identifies a sensitive representation requiring stronger protection, D03-002 shall support protection at the narrowest technically appropriate boundary.

The architecture shall support:

- protected storage representation;
- protected transport representation;
- controlled de-protection only where technically required;
- prevention of unnecessary plaintext propagation;
- preservation of security context during de-protection and re-protection;
- evidence of material protection operations where required.

This mechanism does not establish the business classification of the information. That determination remains with the applicable upstream and L5-B-05 architecture.

### 4.8 Integrity and Authenticity Protection

Where a protected exchange requires integrity or technical authenticity, the receiving technical boundary shall verify the protected representation before accepting it for the security-sensitive operation.

A failed verification shall not be treated as a successful protected exchange.

This architecture does not establish business acceptance, issuance or publication authority. Those decisions remain with their canonical owners.

### 4.9 Key-Management Boundary

D03-002 shall consume key-management capability through an explicit boundary:

```text
D03-002
Cryptographic Operation
        │
        │ key requirement / protected operation context
        ▼
D03-003
Key Management
        │
        │ governed key reference / controlled key operation
        ▼
Cryptographic protection execution
```

D03-002 shall not:

- create an independent key lifecycle;
- own key custody;
- define key rotation policy;
- persist uncontrolled copies of key material;
- bypass D03-003 to obtain cryptographic authority.

The exact key interface and custody mechanics remain a D03-003 concern and are not defined here.

### 4.10 Protection Boundary and Context Integrity

Protection and de-protection operations shall preserve the security context applicable to the protected information.

At minimum, where applicable, the technical protection path shall preserve or bind:

- Tenant context;
- Institution context;
- protected-data classification context when supplied by the owning architecture;
- operation purpose/context where required by the applicable privacy architecture;
- technical actor/workload context;
- provenance/correlation information needed for evidence.

A component shall not accept caller-controlled metadata as authoritative merely because it accompanies protected data.

### 4.11 Reuse Across Technical Components

The cryptographic architecture is reusable across application, workflow, integration, document, reporting and other technical components.

Reuse shall occur through the canonical D03-002 protection boundary rather than by each component independently defining cryptographic architecture.

Where a specialized L5 specification requires cryptographic protection, it shall consume this architecture and the canonical key/credential boundaries rather than creating a parallel cryptographic owner.

### 4.12 Technical Decision Boundaries

| ID | Decision | Classification |
|---|---|---|
| AD-001 | Cryptographic protection is a reusable cross-solution technical capability rather than a business-domain responsibility. | Defensible L5 technical derivation |
| AD-002 | Mandatory protected operations shall not silently downgrade to an unprotected execution path. | Defensible L5 technical derivation from D03 trust-boundary principles |
| AD-003 | Cryptographic operation and key custody are separate technical responsibilities. | Established L5 corpus boundary |
| AD-004 | Plaintext exposure shall be limited to the technical boundary that requires it. | Defensible L5 technical derivation |
| AD-005 | Algorithm, key-size, rotation and provider choices remain unresolved until governed technical decisions establish them. | Established governance constraint |

---

## 5. Dependencies / Integration

### 5.1 Dependency Relationships

| Dependency | Relationship | Required Handoff |
|---|---|---|
| L5-D-03-001 | Security/trust baseline | Protected boundary, trust context and failure principles. |
| L5-D-03-003 | Key Management | Canonical key reference and governed key operations. |
| L5-D-03-004 | Secrets, Certificates & Machine Credentials | Required technical credentials without duplicating their lifecycle. |
| L5-A-04 | Tenant & Institution Isolation | Canonical execution/data isolation context. |
| L5-B-05 | Privacy & Sensitive Data | Policy-derived protection applicability and sensitive-data handling context. |
| L5-D-02 | Authorization | Authorization decision for protected technical operation. |
| L5-D-04 | Audit & Evidence | Security-relevant evidence of material protection operations. |
| L5-B-02 | Database & Persistence | Storage realization for protected representations; persistence remains B02-owned. |
| L5-B-04-005 | Digital Signing, Seal, QR & Credential Integrity | Cryptographic integrity/signing mechanisms consumed through D03-002/D03-003 boundaries. |

### 5.2 Protected Operation Conceptual Contract

A protected operation conceptually requires:

```text
Protected Operation Context
- operation / technical purpose
- protected representation or data reference
- security context
- applicable protection requirement
- required cryptographic operation
- canonical key reference requirement
- integrity/authenticity requirement where applicable
- evidence requirement where applicable
```

This is a conceptual architecture boundary, not an API or implementation payload definition.

### 5.3 Downstream Consumption Rule

A downstream L5 specification shall not redefine cryptographic primitives, key custody or protection semantics when it consumes D03-002.

Where specialized protection is needed, the downstream specification shall identify:

1. the protected asset or exchange;
2. the authoritative protection requirement;
3. the D03-002 capability consumed;
4. the D03-003 key-management dependency where applicable;
5. the required failure behavior if protection cannot be established.

---

## 6. Technical Behavior / Failure Concerns

### 6.1 Required Technical Behaviors

**TB-001 — Protected Transport Admission**  
The architecture shall establish the required cryptographic transport protection before protected information crosses a boundary for which such protection is mandatory.

**TB-002 — No Silent Protection Downgrade**  
When mandatory cryptographic protection cannot be established, the protected operation shall not silently continue through an unprotected alternative.

**TB-003 — Protected Storage Representation**  
Where authoritative requirements require stored-data protection, the persistence path shall receive the protected representation rather than relying on implicit trust in the storage boundary.

**TB-004 — Controlled De-Protection**  
De-protection shall occur only at a technical boundary that requires the protected information in an authorized execution context.

**TB-005 — Integrity Verification**  
Where integrity or technical authenticity protection is required, the receiving boundary shall verify it before accepting the protected representation for the protected operation.

**TB-006 — Canonical Key Boundary**  
Cryptographic operations shall obtain required key capability through the canonical key-management responsibility and shall not create an independent key-custody path.

**TB-007 — Security Context Preservation**  
Protection and de-protection operations shall preserve the applicable Tenant, Institution and security context and shall not silently widen that context.

**TB-008 — Protection Failure Containment**  
If a mandatory cryptographic protection, verification or key-access prerequisite fails, the affected protected operation shall stop or be contained without silently bypassing the protection boundary.

**TB-009 — Protected Representation Integrity**  
A protected representation that fails required cryptographic verification shall not be treated as valid merely because it is structurally readable or otherwise well formed.

**TB-010 — Evidence of Material Protection Operation**  
Where the applicable audit/evidence architecture requires evidence of a material cryptographic protection or verification operation, the operation shall expose the technical evidence required by L5-D-04 without transferring audit ownership into D03-002.

### 6.2 Failure / Recovery Boundaries

| Failure | Immediate Technical Outcome | Recovery Ownership |
|---|---|---|
| Required transport protection unavailable | Prevent protected transmission. | Relevant runtime/integration architecture. |
| Required stored-data protection unavailable | Prevent protected persistence operation where protection is mandatory. | Relevant persistence/runtime architecture. |
| Key-management dependency unavailable | Prevent the affected cryptographic operation; do not bypass key-management boundary. | L5-D-03-003 and relevant runtime architecture. |
| Required credential/certificate unavailable or invalid | Prevent the affected protected operation where the credential is material to the protection boundary. | L5-D-03-004 and relevant runtime architecture. |
| Integrity/authenticity verification fails | Reject or contain the protected input. | Owning technical workflow/integration architecture for subsequent recovery. |
| Security context inconsistent | Reject or contain the operation; do not silently reconstruct privileged context. | L5-D-03-001 / L5-A-04 / L5-D-02 as applicable. |
| De-protection boundary unavailable | Do not expose plaintext through an uncontrolled alternative. | Relevant application/runtime architecture. |

This specification does not define business retry, recovery or compensation policy.

---

## 7. Security / Trust / Evidence

### 7.1 Security Properties

The architecture shall preserve these technical properties:

- protected information is not exposed across a mandatory protected boundary without the required protection;
- cryptographic operations remain separated from key custody;
- protection failures do not create implicit security bypasses;
- plaintext exposure is constrained to necessary technical boundaries;
- Tenant/Institution security context is preserved where applicable;
- integrity/authenticity failures are not accepted as successful protected exchanges;
- cryptographic decisions remain traceable to their authoritative protection requirement or explicit L5 technical derivation.

### 7.2 Evidence Requirements

Where evidence is required by the applicable audit/evidence architecture, technical evidence should be sufficient to establish:

- that a material protection operation was invoked;
- the protected technical context in which it occurred;
- the applicable operation or boundary reference;
- success, failure or verification outcome;
- key-management reference information necessary for provenance without exposing protected key material;
- correlation/provenance information needed by the canonical audit architecture.

Evidence shall not contain plaintext protected data or secret/key material merely to demonstrate that protection occurred.

### 7.3 Evidence Boundary

L5-D-03-002 produces or exposes technical protection evidence as required by the canonical audit architecture. L5-D-04 owns the audit/evidence architecture and storage semantics.

---

## 8. Conformance

| ID | Conformance Requirement |
|---|---|
| CR-001 | Every material protected-data or protected-exchange path identified as requiring cryptographic protection has a defined D03-002 protection boundary. |
| CR-002 | Mandatory protected transport cannot silently downgrade to an unprotected path. |
| CR-003 | Stored protected representations are cryptographically protected where the authoritative requirement requires such protection. |
| CR-004 | Sensitive representations requiring stronger protection are protected at an explicit technical boundary. |
| CR-005 | Cryptographic operations consume key-management capability through L5-D-03-003 rather than maintaining independent key custody. |
| CR-006 | Secrets, certificates and machine credentials are consumed through L5-D-03-004 rather than duplicated within D03-002. |
| CR-007 | Required integrity/authenticity verification occurs before a protected representation is accepted for the protected technical operation. |
| CR-008 | Protection and de-protection preserve applicable Tenant/Institution security context. |
| CR-009 | Failed mandatory cryptographic prerequisites prevent or contain the affected protected operation without silent bypass. |
| CR-010 | Plaintext is not propagated beyond the technical boundary requiring it merely for convenience. |
| CR-011 | Material protection operations expose the evidence required by the canonical audit/evidence architecture without exposing key material or protected plaintext solely for evidence. |
| CR-012 | Unsupported algorithm, key-size, rotation, provider and similar cryptographic parameters are not treated as established architecture without governed authority. |

---

## 9. Traceability

### 9.1 Canonical Traceability Status

| Traceability Item | Status | Evidence / Action |
|---|---|---|
| Canonical L5 ID / title | Verified | 105-specification candidate corpus register. |
| Canonical L5 responsibility | Verified | 105-specification candidate corpus register. |
| D03 Stage-1 placement | Verified | Dependency-gated execution timeline. |
| Dependency on D03-001 trust baseline | Verified | D03-001 current draft and canonical D03 corpus boundary. |
| Exact atomic L4 BH applicability | **Open / Blocking** | Controlled extraction and reconciliation required before promotion. |
| Exact L4 source version/status | **Open / Blocking** | Current repository contains L4 material whose status must be verified before treating it as approval-level authority. |
| D03-003 key-management boundary | Open | Reconcile when D03-003 is authored/stable. |
| D03-004 credential boundary | Open | Reconcile when D03-004 is authored/stable. |
| B05 privacy/sensitive-data boundary | Open | Reconcile when B05 is authored/stable. |
| A04 Tenant/Institution isolation boundary | Open | Reconcile with A04 before downstream promotion. |

### 9.2 Traceability Principle

The atomic traceability relation for this specification shall ultimately be recorded as:

```text
(L4 BH ID, L5-D-03-002)
```

No exact BH mapping is asserted in this draft until the applicable L4 source, version and status are controlled and verified.

### 9.3 Upstream Authority Boundary

L2 sources establish security expectations such as encryption in transit, encryption at rest and sensitive-field protection in applicable workflows, but they do not replace L4 as the authoritative source for L5 behavioral realization. Exact L4-to-L5 applicability therefore remains a promotion blocker until controlled extraction is completed.

---

## 10. Open Issues / Deferred Design

| ID | Issue | Status / Impact | Resolution Path |
|---|---|---|---|
| OD-001 | Exact atomic L4 BH applicability for D03-002 is not yet reconciled. | **Blocking for promotion.** | Perform controlled L4 corpus extraction and record exact BH-to-L5 relations. |
| OD-002 | Current L4 source status must be verified before any L4 behavior is treated as approved authority. | **Blocking for promotion.** | Reconcile live L4 status against locked repository governance. |
| OD-003 | D03-003 key-management architecture is not yet stable. | **Blocking for final dependency validation.** | Author and stabilize D03-003, then reconcile the key-operation boundary. |
| OD-004 | D03-004 secrets/certificates/machine-credential boundary is not yet stable. | Open dependency. | Reconcile after D03-004 authoring. |
| OD-005 | B05 privacy/sensitive-data architecture is not yet stable. | Open dependency. | Reconcile policy-derived protection applicability after B05 authoring. |
| OD-006 | A04 Tenant/Institution isolation architecture is not yet stable. | Open dependency. | Reconcile protected-context preservation after A04 stabilization. |
| OD-007 | Cryptographic algorithms, key sizes, rotation intervals and provider choices are not established by this draft. | Deferred / ADR-dependent. | Resolve only through governed technical decision/ADR process where material. |
| OD-008 | Exact cryptographic credential/certificate topology is not established by this draft. | Deferred. | Resolve through D03-004 and applicable governed decisions. |

**Current status:** Draft — Authoring  
**Promotion condition:** Complete exact L4 atomic traceability and reconcile D03-003/D03-004/B05/A04 dependency boundaries before review/promotion.
