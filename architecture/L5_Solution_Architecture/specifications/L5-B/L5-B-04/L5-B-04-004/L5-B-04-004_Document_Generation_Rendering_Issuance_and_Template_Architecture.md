# L5-B-04-004 — Document Generation, Rendering, Issuance & Template Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-04-004 |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-04 — Document, Media & Object Platform Architecture |
| Canonical Technical Responsibility | Reusable document generation, rendering, template and technical issuance infrastructure while owning domains retain eligibility and approval decisions. |
| Technical Architecture Owner | Document Generation, Rendering, Issuance & Template technical architecture |
| Assigned Author | Varad |
| Version | 1.0.0 |
| Status | Draft — Authoring |
| Last Updated | 2026-09-11 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Coverage Audit Decision; L5-B-04-001; L5-B-04-002; L5-B-04-003; L5-B-04-005; L5-B-05; L5-D-03; L5-D-04; applicable current approved L4 behavior; applicable L2/L3 document/media/object workflows |
| Referenced By | L5-B-04-001; downstream document-platform consumers and implementation architecture |
| Applicable ADRs | None identified; technology choices remain subject to ADR governance |

## 2. Purpose, Scope & Non-Goals

### Purpose

Define the reusable technical architecture for turning an authorized business document request and authoritative source information into controlled document representations through template resolution, content binding, generation, rendering and technical issuance handling.

The approved L5 corpus assigns B04-004 reusable document generation, rendering, template and issuance infrastructure while preserving eligibility and approval decisions with the owning business domains. fileciteturn522file0L7-L10

### In Scope

- Technical template selection and version resolution from authoritative context.
- Binding authorized source information into document-generation inputs.
- Document composition and rendering coordination.
- Generation of technical document representations.
- Controlled technical issuance handoff and readiness handling.
- Representation identity/reference continuity across generation stages.
- Generation/rendering evidence and lineage.
- Coordination with B04-002 for registration/storage and B04-005 for signing/seal/credential-integrity capabilities where applicable.

### Out of Scope / Non-Goals

- Canonical ownership of business entities, eligibility, approval or issuance decisions.
- Business document lifecycle states or transitions.
- Registry/version/storage/retention mechanics owned by B04-002.
- Ingestion, validation and verification mechanics owned by B04-003.
- Digital signing, seal, QR and credential-integrity mechanisms owned by B04-005.
- Cryptographic primitives, key management, certificates and secrets owned by L5-D-03.
- Business policy invention or template-governance ownership outside authoritative upstream sources.
- API contracts, database schemas, infrastructure topology or product/vendor selection.
- UI/UX architecture.

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source | Version / Status | Exact Reference | What It Establishes | L5 Use |
|---|---|---|---|---|
| 105-specification corpus | v1.1.0 / Architecture Approved | L5-B-04-004 | Reusable generation/rendering/template/issuance infrastructure; owning domains retain eligibility/approval decisions. | Canonical technical responsibility |
| L5 L4-to-L5 Coverage Audit Decision | v1.0.0 / Architecture Approved | B04 decomposition | B04-004 is the reusable generation/rendering/issuance/template responsibility within B04. | Granularity and boundary authority |
| L3-C-018 | v1.0 / Locked | Student Document or Certificate Request | Document Services owns generation, rendering and issuance of generated documents; Student Lifecycle owns the request. | Business ownership boundary |
| L4-C-018 | v1.0.0 / Draft | BH-003, BH-004, BH-005, BH-006 | Request routes to canonical document-generation/issuance responsibility; business request outcome/evidence preserved; storage/rendering/issuance implementation is deferred to L5. | Traceability baseline pending approval reconciliation |
| L5-B-04-001 | Current repository state | Common platform baseline | Common object intake, identity/reference, integrity, lineage and handoff boundaries. | Parent/common platform dependency |
| L5-B-04-002 | Current repository state | Registry/version/storage/retention | Registration and storage of generated representations. | Peer dependency |
| L5-B-04-003 | Current repository state | Ingestion/validation/verification | Technical validation/verification of generated representations where applicable. | Peer dependency |
| L5-B-04-005 | Current repository state | Signing/seal/QR/credential integrity | Technical integrity capabilities consumed when generated documents require them. | Peer dependency |
| L5-B-05 | Current repository state | Classification/privacy controls | Applicable protection, masking and privacy constraints. | Technical control dependency |
| L5-D-03 | Current repository state | Security/cryptography/trust | Cryptographic and trust capabilities where required. | Security dependency |
| L5-D-04 | Current repository state | Audit/evidence/provenance | Technical generation/rendering/issuance evidence. | Evidence dependency |

The locked L3 scenario explicitly preserves the ownership boundary: Student Lifecycle owns the request lifecycle, while Document Services owns generated-document generation, rendering, issuance, storage, verification and generated-document lifecycle. fileciteturn487file0L2-L2

### 3.2 Canonical Constraints

- A business request must reach the canonical document-generation/issuance responsibility before generation/issuance work proceeds.
- Technical generation shall consume authoritative business inputs and shall not invent business values.
- Template and rendering mechanisms shall not become owners of business rules or eligibility.
- Technical issuance completion shall not independently create business approval or business eligibility.
- Generated representations shall preserve source/reference lineage and applicable scope.
- B04-005 remains responsible for signing, seals, QR and credential-integrity mechanisms; D03 remains responsible for cryptographic primitives and keys.
- Technical storage and retention remain with B04-002.
- Missing or conflicting authoritative generation context shall not be silently inferred when it materially affects the generated result.

### 3.3 Assumptions / Open Authority

**AS-001 — L4 Approval State:** Exact atomic L4→L5 mappings require reconciliation against the current approved L4 state before promotion.

**AS-002 — Business Issuance Authority:** “Issuance” in this specification denotes technical production/readiness/handoff of the requested document representation; authoritative business issuance decisions remain with the owning domain and upstream workflow.

**AS-003 — Template Authority:** Template definitions, effective applicability and institution-specific policy are consumed from authoritative upstream/configuration sources. This specification does not invent policy or universal template rules.

**AS-004 — Technology Choice:** No rendering engine, document-generation product, font platform or signing product is selected here; material technology choices remain ADR-governed.

## 4. Technical Architecture

### 4.1 Logical Components

| Component | Responsibility | Boundary |
|---|---|---|
| Generation Request Boundary | Receives an authorized document-generation request and establishes context. | Does not decide business eligibility or approval. |
| Source Context Resolver | Resolves authoritative source references and required generation context. | Does not become a source-domain business owner. |
| Template Registry & Resolver | Resolves an applicable template definition/version from authoritative configuration. | Does not own business policy or approval. |
| Content Binding Boundary | Binds authorized source information into the generation model. | Does not invent missing business facts. |
| Composition Engine | Produces a structured document representation from template and authorized inputs. | Does not own document eligibility or issuance authority. |
| Rendering Coordinator | Produces target rendered representations from the composed document. | Does not redefine source facts. |
| Representation Lineage Boundary | Preserves relationships between source context, composition and rendered representations. | Does not own registry/version lifecycle; B04-002 does. |
| Technical Issuance Boundary | Establishes controlled readiness/handoff of generated representation for business completion. | Does not create business approval or authoritative issuance policy. |
| Signing / Integrity Handoff | Routes applicable output to B04-005 for signing/seal/QR/credential-integrity processing. | Does not own cryptographic mechanisms. |
| Evidence Boundary | Preserves technical generation/rendering/issuance evidence. | Business audit meaning remains D04. |

### 4.2 Generation and Rendering Flow

```text
Authorized Business Request / Authoritative Context
                         ↓
             Generation Request Boundary
                         ↓
              Source Context Resolution
                         ↓
              Template Resolution
                         ↓
              Content Binding
                         ↓
              Document Composition
                         ↓
              Rendering Coordination
                         ↓
       ┌─────────────────┴─────────────────┐
       ↓                                   ↓
 B04-005 Integrity Path                B04-003 Verification
       ↓                                   ↓
       └─────────────────┬─────────────────┘
                         ↓
              Technical Issuance Boundary
                         ↓
                    B04-002 Registry/Storage
                         ↓
              Owning Business Workflow
```

The flow is conditional: signing/integrity and verification are invoked only where required by authoritative document processing context.

### 4.3 Authoritative Input Resolution

Generation shall begin only with an authorized request and sufficient authoritative source context. Source-domain facts shall be referenced or consumed as supplied by the owning domain. The generation platform shall not calculate, infer or fabricate business outcomes that belong to another domain.

Conflicting authoritative inputs shall remain observable and shall prevent unsupported generation from being treated as valid output until the governing source context is resolved.

### 4.4 Template Resolution

The template resolver shall identify the applicable template definition/version using authoritative contextual inputs such as document type and effective applicability where those dimensions are established upstream.

Template resolution shall be deterministic for a given resolved configuration/context where the applicable architecture requires reproducibility. A missing or ambiguous applicable template shall not silently fall back to an unrelated representation.

Template changes shall preserve lineage between the selected template and the generated representation without implying a business lifecycle change.

### 4.5 Content Binding

Content binding shall map authorized source information into the generation representation without changing the meaning of the source business facts. Fields or values required by a template but not available from authoritative context shall remain unresolved rather than fabricated.

Technical formatting, layout, labels and presentation metadata remain representation concerns and do not become new business facts.

### 4.6 Document Composition

The composition capability shall produce an intermediate technical document representation from the resolved template and bound authoritative inputs. Composition shall preserve source references and generation context sufficient for lineage and reproducibility.

Composition failures shall prevent the incomplete representation from being treated as successfully generated.

### 4.7 Rendering

Rendering shall convert the composed representation into the required technical output representation while preserving the authoritative content and source lineage. Rendering is not a business transformation.

A rendering failure shall remain distinguishable from a business document rejection or business issuance decision.

### 4.8 Technical Issuance Boundary

The technical issuance boundary establishes that the requested representation has completed required generation/rendering processing and is ready for the next governed handoff. It shall not by itself grant, revoke or modify business eligibility, approval or issuance authority.

Where upstream architecture requires a business issuance decision, that decision remains outside B04-004 and the technical result is handed back to the owning workflow.

### 4.9 Signing, Seal, QR and Credential-Integrity Handoff

Where the generated representation requires digital signing, seals, QR/barcode or credential-integrity processing, B04-004 hands the representation and relevant context to B04-005. B04-004 does not implement or own cryptographic primitives, key custody or certificate management.

The generated representation shall preserve enough lineage to associate any integrity-enhanced representation with the source generated artifact.

### 4.10 Registry and Storage Handoff

After generation and any required technical integrity processing, the representation may be handed to B04-002 for registry, versioning, storage and retention handling. B04-004 does not own physical storage lifecycle or retention policy/execution.

### 4.11 Scope and Isolation

Generation and rendering shall preserve applicable Tenant, Institution and execution scope. Shared templates or rendering capability shall not permit unrelated business contexts to be combined without an authoritative relationship.

### 4.12 Evidence and Lineage

Material generation and rendering operations shall preserve technical lineage including source reference/context, template identity/version, generation attempt, representation relationship, rendering outcome and downstream handoff. Sensitive source content shall not be unnecessarily copied into evidence.

## 5. Dependencies / Integration

### 5.1 Upstream Dependencies

- Applicable L2/L3 document workflows for business request, eligibility and issuance authority.
- Current approved L4 behavior once approval-state reconciliation is complete.
- L5-B-04-001 for common object-platform boundaries.
- L5-B-05 for applicable classification/privacy controls.
- L5-D-03 for cryptographic/trust capabilities where required.
- L5-D-04 for technical evidence/provenance.

### 5.2 Peer B04 Dependencies

- **L5-B-04-001:** common document/media/object platform baseline.
- **L5-B-04-002:** registration, versioning, storage and retention execution of generated representations.
- **L5-B-04-003:** technical validation/verification of generated representations where applicable.
- **L5-B-04-005:** digital signing, seal, QR and credential-integrity processing where required.

### 5.3 Governed Handoff

```text
Owning Business Workflow
        ↓
Authorized Generation Context
        ↓
B04-004 Generation / Rendering / Technical Issuance
        ↓
B04-005 Integrity Processing (where required)
        ↓
B04-002 Registry / Storage
        ↓
Owning Business Workflow / Authorized Consumer
```

No handoff transfers canonical business ownership.

## 6. Technical Behavior / Failure Concerns

### TB-001 — Authorized Generation Admission

Generation processing shall not proceed as a governed document-generation operation when the required authoritative request/context is materially unresolved.

### TB-002 — Source Authority Preservation

Generated content shall derive from authoritative source inputs and shall not fabricate business facts, eligibility or approval outcomes.

### TB-003 — Template Applicability

An applicable template shall be resolved from authoritative configuration/context. Missing, ambiguous or incompatible template resolution shall prevent unsupported generation.

### TB-004 — Content Binding Integrity

Missing or conflicting authoritative values required for generation shall remain observable and shall not be silently substituted with fabricated business values.

### TB-005 — Composition Failure Containment

An incomplete composition shall not be treated as a successfully generated document.

### TB-006 — Rendering Failure Containment

Rendering failure shall remain a technical generation failure and shall not be represented as successful issuance or business completion.

### TB-007 — Representation Lineage

Generated and rendered representations shall remain traceable to their authoritative source context and selected template/version.

### TB-008 — Signing/Integrity Isolation

B04-004 shall not implement or assume ownership of signing, seal, QR or credential-integrity mechanisms delegated to B04-005 or cryptographic capabilities delegated to D03.

### TB-009 — Technical Issuance / Business Issuance Separation

Technical readiness of a generated representation shall remain distinct from business issuance authority and decision.

### TB-010 — Scope Preservation

Tenant, Institution and execution scope shall remain preserved throughout generation, rendering and handoff.

### TB-011 — Recovery Without Fabrication

Recovery or re-generation shall use authoritative inputs and preserved lineage; it shall not invent missing source facts or silently replace prior authoritative representations.

### TB-012 — Evidence

Material generation, rendering and technical issuance outcomes shall produce sufficient technical evidence for reconstruction without unnecessary sensitive-content replication.

## 7. Security / Trust / Evidence

### 7.1 Security

- Generation inputs and outputs shall use applicable authorization/security context.
- Sensitive content shall remain subject to classification, masking and privacy controls.
- Generated representations shall be isolated by applicable Tenant/Institution scope.
- Cryptographic protection, keys and certificates remain under L5-D-03.
- Signing/seal/QR/credential-integrity application remains under B04-005.

### 7.2 Evidence

Evidence shall support reconstruction of generation/rendering activity, including source reference, template identity/version, technical attempt, representation lineage, integrity handoff and final technical outcome. Business audit meaning remains with the originating domain and D04.

## 8. Conformance

- **CR-001:** Generation requires authorized business context and sufficient authoritative source information.
- **CR-002:** Generated content does not become a competing business source of truth.
- **CR-003:** Template resolution uses authoritative context and preserves template lineage.
- **CR-004:** Missing/conflicting source values do not trigger silent fabrication.
- **CR-005:** Composition failures prevent false successful generation.
- **CR-006:** Rendering failures remain distinguishable from business outcomes.
- **CR-007:** Generated/rendered representations preserve source and template lineage.
- **CR-008:** B04-005 retains signing/seal/QR/credential-integrity responsibility.
- **CR-009:** B04-002 retains registry/version/storage/retention responsibility.
- **CR-010:** Tenant/Institution and execution scope are preserved.
- **CR-011:** Technical readiness/issuance does not redefine business issuance authority.
- **CR-012:** Cryptographic primitives, keys and certificates remain with L5-D-03.
- **CR-013:** Technical evidence remains distinguishable from business audit meaning.
- **CR-014:** Architecture remains independent of unapproved technology/product choices.

## 9. Traceability

| L4 BH | L4 Status | L5 Contribution | Evidence |
|---|---|---|---|
| L4-C-018-BH-003 | Draft — approval-state reconciliation required | Routes approved request into reusable generation/rendering/technical issuance architecture. | §§4.1–4.8, §6 |
| L4-C-018-BH-004 | Draft — approval-state reconciliation required | Preserves generation outcome and technical evidence/lineage for downstream business handling. | §§4.8, 4.12, §7 |
| L4-C-018-BH-005 | Draft — approval-state reconciliation required | Implements the mechanisms explicitly deferred from L4: storage/rendering/issuance technical realization, without changing upstream authority. | §§4.7–4.10 |
| Applicable generation/rendering BHs | Approval-state reconciliation required | Reusable template, composition and rendering architecture. | §§4.3–4.7 |

**Traceability qualification:** Exact atomic `(L4 BH ID, L4 Version, L5 Specification ID, L5 Version)` relations remain gated until applicable L4 approval-state reconciliation is complete.

## 10. Open Issues / Deferred Design

1. Reconcile exact atomic L4→L5 mappings against the current approved L4 corpus.
2. Confirm source-specific template-selection and rendering requirements where explicitly mandated by approved L4 behavior.
3. Reconcile peer dependency versions after B04-005 is authored.
4. Determine implementation-level rendering/document-generation technology choices through ADR governance.
5. Determine implementation-level handling of font/package dependencies, localization, document packaging and throughput only where required by approved downstream behavior.
