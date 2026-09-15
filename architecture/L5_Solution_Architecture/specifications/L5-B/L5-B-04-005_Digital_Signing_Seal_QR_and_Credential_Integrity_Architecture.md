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
| Version | 1.0.4 |
| Status | Draft |
| Last Updated | 2026-09-15 |
| Depends On | L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard; L5-B-04-001; L5-B-04-002; L5-B-04-004; L5-D-03-001; L5-D-03-002; L5-D-03-003; L5-D-03-004; L5-D-04-001; L4-H-002; L4-H-004; L4-H-005; L4-C-018 |
| Referenced By | Domain credential issuance and verification workflows |
| Applicable ADRs | ADR-L5-0001; ADR-L5-0002; ADR-L5-0012 |

---

## 2. Purpose, Scope & Non-Goals

### Purpose

This specification defines the B04 orchestration boundary for applying approved integrity services to already-authorized institutional documents and credentials, packaging machine-verifiable integrity references, and exposing verification outcomes without transferring cryptographic ownership from D03 or business publication authority from the owning domain.

### In Scope

- Accept an already-authorized credential/document integrity request.
- Bind the exact artifact version and owning-domain credential/publication reference.
- Request signing/integrity operations from approved D03 services.
- Attach or associate returned integrity evidence, seal representation, QR/barcode verification reference or equivalent verification carrier.
- Produce verification responses that report technical validity without inventing business status.
- Preserve technical evidence and correlation to the owning business workflow.

### Out of Scope / Non-Goals

- Key generation, key custody, key activation, rotation, revocation, retirement or destruction.
- Selection of cryptographic algorithms, key sizes, HSM/KMS products, PKI products, certificate authorities, timestamp services or credential standards.
- Defining consent, government-publication eligibility or domain credential lifecycle.
- Choosing QR payload encodings, signature formats, document-signature standards or validity periods without approved repository authority.
- Owning external-government acknowledgement or business publication state.

---

## 3. Upstream Applicability & Canonical Boundaries

### 3.1 L4 Alignment

`L4-H-002` is a locked business-behavior authority for approved credential publication to government channels. Its behaviors are not cryptographic design requirements:

| L4 Behavior | Canonical Meaning | L5-B-04-005 Treatment |
|---|---|---|
| `L4-H-002-BH-001` | An approved credential becomes eligible for government publication only when upstream business constraints are satisfied. | Consumed precondition. B04-005 does not decide publication eligibility. |
| `L4-H-002-BH-002` | Publication consent, connector and duplicate constraints are evaluated. | Consumed precondition/dependency; B04-005 does not replace consent or connector ownership. |
| `L4-H-002-BH-003` | Publication follows the locked business lifecycle. | No lifecycle ownership transfer; technical integrity evidence may accompany the publication artifact. |
| `L4-H-002-BH-004` | Successful Published status requires government acknowledgement. | B04-005 cannot declare Published; it only reports integrity/signing outcomes to the owning workflow. |

`L4-H-004`, `L4-H-005` and `L4-C-018` may require document/credential integrity or verification support where their approved behaviors explicitly call for it. This specification consumes those requirements without redefining their business semantics.

### 3.2 D03 Canonical Ownership

Per `ADR-L5-0002` and the D03 architecture:

- D03 owns cryptographic/data-protection realization.
- D03 exclusively owns key lifecycle and custody.
- D03 exclusively owns application/integration secrets, certificates and machine credentials.
- Algorithms, key lengths, concrete HSM/KMS products, certificate authorities and rotation intervals are not selected here.

B04-005 therefore owns **orchestration and document/credential binding**, not cryptography itself.

---

## 4. Technical Ownership & Dependencies

### 4.1 Owns

- Integrity-request admission after owning-domain authorization.
- Exact artifact-version binding before integrity processing.
- Invocation of approved D03 cryptographic capability through governed contracts.
- Association of returned integrity evidence with the artifact/credential record.
- Technical verification orchestration and response composition.
- QR/barcode/seal carrier generation only as a presentation/transport of approved verification reference/evidence, without selecting cryptographic primitives.

### 4.2 Consumes / Delegates

| Owner | Responsibility Consumed |
|---|---|
| Owning domain | Credential approval, issuance/publication eligibility, consent and lifecycle. |
| `L5-D-03-002` | Cryptographic protection operations and approved digest/signature capability. |
| `L5-D-03-003` | Key lifecycle and custody. |
| `L5-D-03-004` | Certificates, machine credentials and secret management. |
| `L5-D-04` | Canonical audit evidence aggregation. |
| `L5-B-04-002` | Artifact/credential technical identity, version and storage reference. |
| `L5-B-04-004` | Generated artifact before integrity augmentation. |

### 4.3 Must Not Own

Cryptographic primitives, key material, cryptographic policy, certificate lifecycle, external publication decisions, government connector outcomes, business credential validity or revocation semantics, or repository-wide trust policy.

---

## 5. Solution Architecture

### 5.1 Realization Components

| Element | Responsibility | Input | Output | Failure Boundary |
|---|---|---|---|---|
| `IntegrityRequestAdmissionBoundary` | Verifies owning-domain authorization, artifact identity/version and requested integrity operation are present. | Authorized integrity request | Admitted request | Rejects missing context; never infers approval. |
| `ArtifactBindingResolver` | Resolves the exact immutable artifact/version to which integrity evidence will apply. | Artifact reference | Bound artifact identity | Fails closed on ambiguity or version mismatch. |
| `D03IntegrityBroker` | Calls the approved D03 cryptographic capability and receives opaque integrity/signature evidence. | Bound artifact or approved digest input | D03-produced integrity evidence | Does not inspect or manage key material. |
| `VerificationCarrierAssembler` | Associates approved verification evidence/reference with a seal, QR/barcode or equivalent carrier required by the calling workflow. | D03 evidence + artifact reference | Verification carrier | Carrier construction cannot alter the signed/bound artifact identity. |
| `IntegrityEvidenceRecorder` | Records artifact version, D03 operation reference, technical result and correlation evidence. | Integrity attempt | Technical evidence | Failure prevents reporting integrity application as complete. |
| `VerificationGatewayBoundary` | Resolves a presented verification reference and returns a bounded technical verification result. | Verification request/reference | Valid/invalid/unavailable technical result plus authoritative business-status reference where supplied by owner | Must not infer business validity from cryptographic validity alone. |

### 5.2 Primary Flow

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

### 5.3 Verification Semantics

A successful technical verification means only that the presented artifact/reference satisfies the approved integrity check returned by D03 and corresponds to the bound artifact/version. Business validity, current standing, revocation by the owning domain, publication acknowledgement and entitlement remain authoritative outside B04-005.

### 5.4 Failure & Recovery

- Artifact/version mismatch: reject; never sign a substituted version.
- D03 service unavailable: return technical-unavailable outcome; do not bypass cryptography or use local keys.
- Integrity operation rejected by D03: preserve failure evidence; owning workflow decides business consequence.
- Carrier generation failure: do not mark integrity application complete until the required carrier/evidence association is durably recorded.
- Verification reference unknown: return not-resolved/invalid according to governed contract; do not leak unrelated credential data.

### 5.5 Security & Privacy

- B04-005 never receives raw private key material.
- Verification responses disclose only information authorized by D02/privacy policy and the owning domain.
- Public or external verification must be abuse-resistant and authorization/privacy aware, but concrete rate limits and anti-abuse products are downstream governed decisions.
- Audit evidence is delegated to D04; B04 preserves correlation identifiers necessary for that handoff.

---

## 6. Architecture Decisions & Open Decisions

| ID | Position | Classification |
|---|---|---|
| AD-B04-005-01 | B04 owns credential/document integrity orchestration; D03 owns cryptographic implementation and key lifecycle. | Repository fact / ADR-L5-0002 |
| AD-B04-005-02 | Cryptographic validity does not by itself establish business validity or publication status. | Repository-derived boundary |
| AD-B04-005-03 | Integrity evidence binds an exact artifact version and cannot silently migrate to a replacement version. | Defensible technical derivation |
| OD-B04-005-01 | Algorithm, key size, signature/container standard, timestamp mechanism, QR payload encoding, certificate profile and HSM/KMS product remain unresolved until governed by D03/ADR or downstream design authority. | Open decision |

No local architecture decision in this document selects cryptographic algorithms, products, key lengths, rotation periods or public-credential standards.

---

## 7. Conformance Requirements

1. B04-005 shall never generate, custody, rotate, revoke or destroy cryptographic keys.
2. Every integrity operation shall bind an exact artifact/version and an owning-domain authorization/correlation reference.
3. Cryptographic operations shall be obtained through the approved D03 capability; no local fallback keys or algorithms are permitted.
4. B04-005 shall not declare a credential business-valid, issued, published or acknowledged solely from cryptographic success.
5. Verification responses shall preserve tenant/privacy/authorization boundaries and shall not disclose unrelated record data.
6. Any material algorithm, key, certificate, HSM/KMS, signature-standard or rotation-period selection shall be governed by D03/ADR before becoming normative.

---

## 8. Traceability Summary

- `L4-H-002-BH-001` — consumed as upstream publication-eligibility precondition.
- `L4-H-002-BH-002` — consumed as upstream consent/connector/duplicate precondition.
- `L4-H-002-BH-003` — business publication lifecycle remains upstream; B04 contributes integrity evidence only.
- `L4-H-002-BH-004` — government acknowledgement remains outside B04; B04 cannot declare Published.
- `ADR-L5-0002` — D03 cryptographic and key-custody ownership preserved.