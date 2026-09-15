# L5-B-04-005 — Digital Signing, Seal, QR & Credential Integrity Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-04-005 |
| Specification Title | Digital Signing, Seal, QR & Credential Integrity Architecture |
| Architecture Level | L5 — Solution Architecture |
| Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-04 — Document, Media & Object Platform Architecture |
| Canonical Technical Responsibility | Document/credential signing, seal, QR/barcode and verification orchestration while cryptographic primitives, key lifecycle and machine credentials remain D03-owned. |
| Technical Architecture Owner | Document Credential Integrity Technical Architecture |
| Assigned Author | Varad |
| Version | 1.1.0 |
| Status | Review |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-04-001; L5-B-04-002; L5-B-04-004; L5-D-03-001; L5-D-03-002; L5-D-03-003; L5-D-03-004; L5-D-04-001; L4-H-002; L4-H-004; L4-H-005; L4-C-018 |
| Referenced By | Domain credential issuance and verification workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0012 |

## 2. Purpose, Scope & Non-Goals

Defines the B04 orchestration boundary for applying approved integrity services to already-authorized institutional documents/credentials, packaging machine-verifiable integrity references, and exposing bounded technical verification outcomes without transferring cryptographic ownership from D03 or business publication authority from the owning domain.

### In Scope

- Accept already-authorized document/credential integrity requests.
- Bind exact artifact version and owning-domain credential/publication reference.
- Invoke approved D03 integrity/signing capability.
- Associate returned integrity evidence with the exact artifact version.
- Assemble seal/QR/barcode/equivalent verification carrier where required by owning workflow.
- Expose technical verification results without asserting business validity/publication state.
- Preserve technical evidence and owner correlation.

### Out of Scope / Non-Goals

- Key generation/custody/activation/rotation/revocation/retirement/destruction.
- Cryptographic algorithm, key-size, HSM/KMS, PKI, CA or timestamp-product selection.
- Consent, government-publication eligibility or credential business lifecycle.
- QR payload encoding, signature/container standard or validity-period selection without approved authority.
- External-government acknowledgement or canonical publication state.

## 3. Canonical L4 Alignment

| L4 Behavior | Canonical Meaning | L5-B-04-005 Treatment |
|---|---|---|
| `L4-H-002-BH-001` | Approved credential eligible for government publication only when upstream business constraints are satisfied. | Consumed precondition; B04-005 does not decide eligibility. |
| `L4-H-002-BH-002` | Consent, connector and duplicate constraints are evaluated. | Consumed precondition/dependency; no consent/connector ownership transfer. |
| `L4-H-002-BH-003` | Publication follows locked business lifecycle. | No lifecycle ownership transfer; integrity evidence may accompany artifact. |
| `L4-H-002-BH-004` | Published requires government acknowledgement. | B04-005 cannot declare Published; only reports integrity/signing outcomes. |

`L4-H-004`, `L4-H-005` and `L4-C-018` may require integrity/verification support where explicitly stated. This specification consumes those requirements without redefining business semantics.

## 4. D03 Canonical Ownership

Per `ADR-L5-0002` and D03 architecture:

- D03 owns cryptographic/data-protection realization.
- D03 exclusively owns key lifecycle and custody.
- D03 exclusively owns application/integration secrets, certificates and machine credentials.
- Algorithms, key lengths, HSM/KMS products, certificate authorities and rotation intervals are not selected here.

B04-005 owns orchestration, exact artifact binding, verification-carrier association and bounded technical verification semantics—not cryptography itself.

## 5. Architectural Invariants

1. Every integrity operation binds an exact immutable artifact version.
2. Integrity evidence for one version cannot silently migrate to a replacement version.
3. B04-005 never receives/custodies raw private key material.
4. D03 unavailability cannot trigger local key/algorithm fallback.
5. Cryptographic/integrity validity does not by itself establish business validity, issuance, revocation status or publication acknowledgement.
6. Verification disclosure is limited by D02/privacy/owning-domain authority.
7. Unknown/unresolved verification references fail safely and must not leak unrelated credential information.
8. Integrity application is not complete until required evidence/carrier association is durably recorded.

## 6. Technical Ownership & Dependencies

### 6.1 Owns

- Integrity-request admission after owning-domain authorization.
- Exact artifact-version binding.
- Invocation/orchestration of approved D03 cryptographic capability.
- Association of returned integrity evidence with artifact/credential record.
- Technical verification orchestration and response composition.
- QR/barcode/seal carrier generation only as a presentation/transport of approved verification reference/evidence.

### 6.2 Consumes / Delegates

| Owner | Responsibility Consumed |
|---|---|
| Owning domain | Credential approval, issuance/publication eligibility, consent and business lifecycle. |
| `L5-D-03-002` | Cryptographic protection operations and approved digest/signature capability. |
| `L5-D-03-003` | Key lifecycle and custody. |
| `L5-D-03-004` | Certificates, machine credentials and secret management. |
| `L5-D-04` | Canonical audit/evidence aggregation. |
| `L5-D-02` / privacy authority | Verification disclosure/authorization constraints. |
| `L5-B-04-002` | Artifact technical identity, version and storage reference. |
| `L5-B-04-004` | Generated artifact before integrity augmentation. |

### 6.3 Must Not Own

Cryptographic primitives, key material/policy, certificate lifecycle, external publication decisions, government connector outcomes, business credential validity/revocation semantics or repository-wide trust policy.

## 7. Solution Architecture

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `IntegrityRequestAdmissionBoundary` | Verify owner authorization/correlation, artifact identity/version and requested integrity operation. | Authorized integrity request | Admitted request | Missing context fails closed; no approval inference. |
| `ArtifactBindingResolver` | Resolve exact immutable artifact/version to which evidence applies. | Artifact reference | Bound artifact version | Ambiguity/version mismatch rejects request. |
| `D03IntegrityBroker` | Call approved D03 capability and receive opaque integrity/signature evidence. | Bound artifact/reference | D03-produced evidence | Does not inspect/manage key material; no local fallback. |
| `VerificationCarrierAssembler` | Associate approved verification evidence/reference with seal/QR/barcode/equivalent carrier requested by owner. | D03 evidence + artifact ref | Verification carrier | Carrier cannot alter bound artifact identity or claim business state. |
| `IntegrityEvidenceRecorder` | Record exact artifact version, D03 operation reference, technical result and correlation evidence. | Integrity attempt | Durable technical evidence | Failure prevents completion. |
| `VerificationReferenceResolver` | Resolve presented verification reference to the bound technical artifact/evidence under permitted disclosure rules. | Verification reference | Bound verification context | Unknown/unauthorized reference fails safely. |
| `VerificationGatewayBoundary` | Return bounded technical verification result. | Bound verification context | Technical valid/invalid/unavailable result plus authorized owner-status reference if supplied | Must not infer business validity from technical validity. |

## 8. Primary Integrity Flow

```text
Owning Domain approval/issuance decision
  -> IntegrityRequestAdmissionBoundary
  -> ArtifactBindingResolver
  -> D03IntegrityBroker
  -> VerificationCarrierAssembler (when required)
  -> IntegrityEvidenceRecorder
  -> B04 registry/storage association
  -> Owning Domain publication/issuance workflow
```

## 9. Conceptual Contracts

An integrity request conceptually identifies tenant/institution, owning-domain/correlation reference, exact artifact/version reference, requested integrity operation/profile reference and required authorization/trust context.

An integrity result conceptually returns technical attempt/reference, bound artifact/version, D03 operation/evidence reference, carrier reference where applicable, technical outcome and evidence/audit correlation.

A verification response conceptually returns technical integrity state such as valid/invalid/unavailable/not-resolved, bound artifact/reference metadata permitted for disclosure, and an owning-domain status/reference only when explicitly supplied/authorized by the canonical owner.

Exact fields, encodings and protocols remain downstream design.

## 10. Verification Semantics

A successful technical verification means only that the presented artifact/reference satisfies the approved D03 integrity check and corresponds to the bound artifact/version. It does **not** by itself establish:

- current business validity;
- issuance or publication status;
- absence of business revocation/cancellation;
- entitlement/identity of presenter;
- government acknowledgement;
- academic/financial/legal standing.

Where business status is needed, verification must resolve or return a canonical owning-domain reference under applicable authorization/privacy controls.

## 11. Carrier & Reference Semantics

- A QR/barcode/seal/equivalent carrier is a technical transport/presentation of a verification reference/evidence association, not the source of business truth.
- Carrier generation cannot modify the bound artifact identity or D03 evidence.
- Lost/copied/duplicated carrier presentation does not by itself change credential business state.
- Payload encoding, offline/online verification mode and anti-abuse limits remain governed downstream decisions.

## 12. Failure & Recovery

| Failure | Required Treatment |
|---|---|
| Artifact/version mismatch | Reject; never sign substituted version. |
| D03 service unavailable | Return technical unavailable; no local cryptographic fallback. |
| D03 operation rejected | Preserve failure evidence; owner decides business consequence. |
| Carrier generation failure | Do not mark integrity application complete until required association is recorded. |
| Evidence persistence/handoff failure | Do not report successful integrity application. |
| Verification reference unknown | Return not-resolved/invalid per governed contract; do not leak unrelated data. |
| Verification disclosure unauthorized | Fail closed/minimize response according to D02/privacy authority. |
| Business-status source unavailable | Return technical integrity state separately; do not infer business validity. |

## 13. Security, Privacy & Abuse Resistance

- B04-005 never receives raw private key material.
- Verification responses disclose only data permitted by D02/privacy/owning-domain policy.
- Public/external verification must be abuse-resistant, but exact rate limits/challenge products remain downstream governed.
- Verification references should avoid unnecessary sensitive information in externally presented carriers.
- Technical diagnostics/evidence must distinguish integrity failure from authorization/privacy failure without over-disclosure.
- Audit evidence is delegated to D04; B04 preserves correlation necessary for that handoff.

## 14. Observability & Evidence

Technical evidence should support correlation of owning-domain authorization, exact artifact/version, D03 operation reference, carrier association, verification request/outcome class, disclosure decision, business-status reference where supplied, failure category and D04 handoff. Raw secrets/private keys are never telemetry.

## 15. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B04-005-01 | B04 owns credential/document integrity orchestration; D03 owns cryptographic implementation and key lifecycle. | Repository fact / ADR-L5-0002 |
| AD-B04-005-02 | Cryptographic validity does not by itself establish business validity or publication status. | Repository-derived boundary |
| AD-B04-005-03 | Integrity evidence binds an exact artifact version and cannot silently migrate to a replacement version. | Defensible technical derivation |
| AD-B04-005-04 | Verification disclosure is independently authorization/privacy governed and cannot expose business/credential data solely because a technical reference is presented. | Security/privacy derivation |
| OD-B04-005-01 | Algorithm, key size, signature/container standard, timestamp mechanism, QR payload encoding, certificate profile and HSM/KMS product remain unresolved until governed by D03/ADR or downstream design authority. | Open decision |

## 16. Conformance Requirements

1. B04-005 shall never generate, custody, rotate, revoke or destroy cryptographic keys.
2. Every integrity operation shall bind an exact artifact/version and owner authorization/correlation reference.
3. Cryptographic operations shall use approved D03 capability; no local fallback keys/algorithms are permitted.
4. Integrity evidence shall not be reused for a different artifact/version.
5. B04-005 shall not declare a credential business-valid, issued, published or acknowledged solely from cryptographic success.
6. Verification responses shall preserve tenant/privacy/authorization boundaries and not disclose unrelated record data.
7. Business-status unavailability shall not be converted into assumed validity.
8. Required evidence/carrier persistence failure shall prevent successful completion.
9. Material algorithm/key/certificate/HSM/KMS/signature-standard/rotation decisions require D03/ADR governance before becoming normative.

## 17. Architecture Acceptance Evidence

A conforming design must demonstrate exact-version signing/integrity binding, zero raw private-key handling by B04, no cryptographic fallback outside D03, separation of technical versus business validity, fail-closed disclosure controls, and traceable evidence/carrier association.

## 18. Traceability Summary

- `L4-H-002-BH-001` — upstream publication-eligibility precondition.
- `L4-H-002-BH-002` — upstream consent/connector/duplicate precondition.
- `L4-H-002-BH-003` — business publication lifecycle remains upstream; B04 contributes integrity evidence only.
- `L4-H-002-BH-004` — government acknowledgement remains outside B04; B04 cannot declare Published.
- `ADR-L5-0002` — D03 cryptographic and key-custody ownership preserved.