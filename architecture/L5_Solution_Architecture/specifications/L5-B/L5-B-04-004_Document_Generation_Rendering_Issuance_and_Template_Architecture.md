# L5-B-04-004 — Document Generation, Rendering, Issuance & Template Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-04-004 |
| Specification Title | Document Generation, Rendering, Issuance & Template Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-04 — Document, Media & Object Platform Architecture |
| Canonical Technical Responsibility | Reusable document generation, rendering, template and issuance infrastructure; owning domains retain eligibility, approval and business issuance authority. |
| Technical Architecture Owner | Document Generation & Rendering Technical Architecture |
| Assigned Author | Varad |
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-04-001; L5-B-04-002; L5-B-02-001; L5-B-05-003; L5-D-03-001; L5-D-04-001; L5-A-08-001; L4-C-018; L4-E-039 |
| Referenced By | Domain workflows that request governed document generation or issuance |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0005; ADR-L5-0013; ADR-L5-0014 |

## 2. Purpose, Scope & Non-Goals

This specification defines reusable technical architecture for transforming an already-authorized document-generation request into a reproducible artifact, preserving exact template/source lineage and returning the artifact to the owning business workflow for its canonical issuance/publication decision.

### In Scope

- Governed template registration and immutable template-version resolution.
- Binding of authoritative source snapshot/reference to template version.
- Rendering and artifact assembly in an isolated execution boundary.
- Structural output validation and generation-evidence production.
- Technical generation-result handoff to the owning workflow.
- Delegation of background work to A08 where asynchronous execution is required.

### Out of Scope / Non-Goals

- Business eligibility, approval or issuance/publication decisions.
- Academic, finance, HR or statutory document semantics.
- Cryptographic key ownership or algorithm selection.
- Retention/legal-hold policy.
- Product/framework/runtime/deployment selection.
- Fixed performance, memory, CPU, queue, retention or batch-size targets without approved authority.

## 3. Canonical L4 Alignment

| L4 Behavior | Canonical Meaning | L5-B-04-004 Treatment |
|---|---|---|
| `L4-C-018-BH-001` | Validate Student and requested document/certificate eligibility before accepting the business request. | **Consumed precondition only.** Eligibility remains upstream. |
| `L4-C-018-BH-002` | Establish requested document type and applicable business conditions without redefining Document Services ownership. | **Consumed precondition only.** Receives resolved document type/context. |
| `L4-C-018-BH-003` | Route the request to canonical generation/issuance responsibility. | **Direct technical realization.** Accepts routed authorized generation request and produces artifact. |
| `L4-C-018-BH-004` | Preserve request outcome and business evidence. | **Technical contribution.** Returns generation lineage/evidence; business evidence ownership remains upstream. |

`L4-E-039` may provide additional applicability where it explicitly routes to shared generation infrastructure. This specification does not infer new business behavior from that reference.

## 4. Architectural Invariants

1. Generation begins only after owning-domain business authorization/context is established.
2. Template version and authoritative source snapshot/reference used for a completed generation attempt are immutable lineage references.
3. Template/render execution cannot mutate canonical business state directly.
4. Generated artifact is a technical output until the owning domain establishes issuance/publication status.
5. Template fallback to another version is not permitted when the requested/authorized version cannot be resolved.
6. Required evidence/storage handoff must complete before a generation attempt is represented successful.
7. Rendering runs within D03/E04-governed isolation and cannot gain unrestricted network/filesystem/process authority.
8. Retention/disposition consumes B05 authority; generation does not create local retention policy.

## 5. Technical Ownership & Boundaries

### 5.1 Owns

- Technical template identity and immutable template-version binding.
- Technical generation-request admission after business authorization.
- Source-data snapshot/reference binding for reproducibility.
- Isolated render orchestration and artifact assembly.
- Structural artifact validation.
- Generation-attempt lineage and result handoff.

### 5.2 Consumes / Delegates

| Owner | Consumed Responsibility |
|---|---|
| Owning business domain / L4 workflow | Eligibility, approval, document type, issuance/publication authority and business evidence. |
| `L5-B-04-002` | Artifact registry identity, version references and storage handoff. |
| `L5-B-05-003` | Retention/disposition authorization and legal-hold precedence. |
| `L5-D-03` | Cryptographic services and trust/isolation constraints. |
| `L5-D-04` | Canonical audit/evidence aggregation. |
| `L5-A-08` | Background scheduling, retry and technical work-unit execution. |
| `L5-B-02` | Persistence mechanics for B04-004-owned technical records. |

### 5.3 Must Not Own

Business eligibility/approval, document semantics, domain lifecycle, retention/legal hold, cryptographic key lifecycle, queue/retry platform policy or product-specific implementation choices.

## 6. Solution Architecture

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `GenerationAdmissionBoundary` | Verify owning-domain authorization, correlation reference, artifact type and required generation context. | Authorized generation request | Admitted generation request | Rejects incomplete/unbound request without deciding eligibility. |
| `TemplateVersionResolver` | Resolve exact immutable approved template version referenced by request. | Template reference | Bound template version | No silent substitute version. |
| `SourceSnapshotBinder` | Bind exact authoritative source-data snapshot/reference used for generation. | Source context | Immutable source binding | Rejects ambiguous or mutable/unversioned reference where reproducibility is required. |
| `RenderContextAssembler` | Construct domain-neutral render context from authorized source/template bindings. | Bound template + source | Render context | Cannot infer missing business semantics. |
| `IsolatedRenderExecutor` | Execute rendering inside D03/E04-governed isolation. | Render context | Candidate artifact | Contains template/runtime failure without business-state mutation. |
| `ArtifactStructureValidator` | Verify candidate artifact against requested technical output profile. | Candidate artifact | Validated artifact | Rejects malformed/incomplete output. |
| `GenerationEvidenceAssembler` | Bind request, template version, source reference, output identity/integrity and outcome. | Generation attempt | Evidence envelope | Evidence failure prevents successful completion. |
| `GenerationResultHandoff` | Register/store through B04-002 and return artifact/evidence reference to owner. | Validated artifact | Domain-consumable generation result | Cannot mark business issuance/publication complete. |

## 7. Primary Flow

```text
Owning Domain (eligibility + approval already resolved)
  -> GenerationAdmissionBoundary
  -> TemplateVersionResolver
  -> SourceSnapshotBinder
  -> RenderContextAssembler
  -> IsolatedRenderExecutor
  -> ArtifactStructureValidator
  -> GenerationEvidenceAssembler
  -> B04-002 registry/storage handoff
  -> GenerationResultHandoff
  -> Owning Domain issuance/publication decision
```

## 8. Conceptual Contracts

`AuthorizedGenerationRequest` conceptually contains tenant/institution context, owning-domain request/correlation reference, requested document/artifact type, exact template reference/version requirement, authoritative source snapshot/reference, output technical profile reference and required trust/authorization context.

`GenerationResult` conceptually contains correlation identity, artifact identity/version reference, exact template-version reference, source-snapshot/reference, structural-validation outcome, integrity/evidence reference where applicable, technical outcome and generation-evidence reference.

It must not claim business issuance or publication status. Exact transport fields remain downstream design.

## 9. Template Governance & Versioning

- Template identity/version is technical and must remain attributable to an authorized template source/governance context.
- Updating a template creates a new version rather than silently changing historical generation meaning.
- A generation attempt binds exactly one effective template version unless an approved composition contract explicitly defines multiple governed template components.
- Missing requested version fails generation rather than falling back to “latest.”
- Template compatibility/profile validation is technical; document/business content approval remains upstream.

## 10. Source Binding & Reproducibility

- All externally varying business facts needed for the generated artifact are supplied/referenced through the authorized source context.
- The renderer does not perform hidden direct persistence reads to discover missing business facts.
- Where reproducibility is required, the source binding identifies the exact authoritative snapshot/reference used.
- Re-generation with changed source/template inputs is a new technical attempt with predecessor lineage where evidence requirements apply.

## 11. Rendering Isolation & Side-Effect Control

- Template/render execution is treated as untrusted computation.
- It operates under least privilege and governed network/filesystem/process restrictions.
- Rendering cannot directly mutate canonical business state or invoke arbitrary external integrations.
- Resource exhaustion terminates the technical attempt safely; exact numeric resource bounds are downstream governed decisions.
- Any external asset/reference capability later introduced must be explicitly governed and reproducible enough for the intended evidence requirements.

## 12. Failure & Recovery

| Failure | Required Treatment |
|---|---|
| Missing business authorization/context | Reject; owner resolves. |
| Template/version unavailable | Fail; no silent substitution. |
| Source reference unresolved/ambiguous | Fail admission/binding. |
| Rendering deterministic/template error | Return technical failure; no blind retry. |
| Runtime/isolation failure | Abort safely; no less-isolated fallback. |
| Invalid artifact structure | Reject/quarantine candidate; do not issue. |
| Registry/storage handoff failure | Do not report generation complete until resolved. |
| Evidence handoff failure | Do not report successful completion. |
| Transient asynchronous execution failure | Delegate recovery to A08; no local retry policy. |

## 13. Security, Privacy & Evidence

- Render context should contain only data required for the authorized document.
- Sensitive source data and diagnostics are minimized according to privacy/classification controls.
- Template execution has no access to raw cryptographic private keys.
- Integrity/signing, when required, is delegated to B04-005/D03 after artifact version binding.
- Generation evidence preserves lineage without duplicating unnecessary sensitive source content.

## 14. Observability & Operability

Technical evidence should support correlation of request, template version, source snapshot/reference, render attempt, structural-validation result, artifact/version identity, predecessor regeneration reference, failure class and evidence/storage handoff outcome. Numeric performance/capacity targets remain downstream governed.

## 15. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B04-004-01 | Business eligibility and issuance authority remain upstream; B04-004 only performs technical generation and returns evidence. | Repository-derived architecture position |
| AD-B04-004-02 | Completed generation binds immutable template and source references for reproducibility. | Defensible technical derivation |
| AD-B04-004-03 | Rendering executes in an isolated trust boundary and cannot mutate canonical business state directly. | Repository-derived architecture position |
| AD-B04-004-04 | Missing requested template version fails generation rather than silently selecting another version. | Technical integrity derivation |
| OD-B04-004-01 | Concrete template technology, renderer, artifact profile/standard and performance targets remain unresolved until governed by approved decision/downstream design authority. | Open decision — non-blocking for L5 architecture |

## 16. Conformance Requirements

1. Implementation shall reject generation requests lacking required owning-domain authorization/context.
2. Implementation shall not perform independent business eligibility or approval decisions.
3. Every completed generation shall identify exact template version and source binding used.
4. Requested/unavailable template versions shall not be silently substituted.
5. Template/render execution shall be isolated according to approved D03/E04 controls.
6. Generated artifact shall not be represented as business-issued/published by B04-004 alone.
7. Retention/disposition actions shall consume B05 authorization.
8. Background retries/work scheduling shall be delegated to A08/C02 as applicable.
9. Required storage/evidence handoff failure shall prevent successful completion.
10. Material technology, numeric SLO or document-standard selections shall be governed before becoming normative.

## 17. Architecture Acceptance Evidence

A conforming design must demonstrate reproducible template/source binding, no hidden business-data reads, no silent template fallback, isolated side-effect-controlled rendering, no direct business issuance/publication, and durable registry/evidence handoff before success.

## 18. Traceability Summary

- `L4-C-018-BH-001` — upstream eligibility precondition only.
- `L4-C-018-BH-002` — upstream document-type/business-condition resolution precondition only.
- `L4-C-018-BH-003` — technically realized through generation admission/rendering/handoff.
- `L4-C-018-BH-004` — supported through generation lineage/evidence returned to owning workflow.
- Canonical business ownership remains unchanged.