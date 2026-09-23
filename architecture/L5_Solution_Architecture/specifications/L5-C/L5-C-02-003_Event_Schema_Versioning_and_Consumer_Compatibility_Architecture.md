# Event Schema, Versioning & Consumer Compatibility Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-02-003`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-02 — Event & Messaging Architecture`  
**Canonical Technical Responsibility:** Technical event schema evolution and consumer compatibility while canonical event names/publishers remain upstream-owned.  
**Technical Architecture Owner:** `L5-C-02 — Event & Messaging Architecture`  
**Assigned Author:** Pallav  
**Version:** `1.2.1`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-C-02-001`; `L5-C-02-002`; `ADR-L5-0011`  
**Referenced By:** event publishers/consumers and downstream L5 specifications that depend on governed event-contract compatibility  
**Applicable ADRs:** `ADR-L5-0011`; `ADR-L5-0035`.
**Governing Sources:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology; L5 ADR Register.

## 2. Purpose, Scope & Non-Goals
Defines event contract representation, explicit versioning, compatibility classification, schema catalog/registry capability, consumer version declarations, deprecation and retirement. It does not create/rename business events, publishers or business meaning and does not own transport/retry/replay.

## 3. Upstream Applicability, Constraints & Assumptions
### 3.1 Authoritative L4 Traceability
For this production validation/remediation cycle, canonical L4 specifications are treated as behavioral authority regardless of repository metadata status. These relations are therefore **Validated architectural realization mappings**.

| L4 BH | Contribution | State |
|---|---|---|
| `L4-A-018-BH-003/004/005/006/007/008/009` | Schema/contract validation, activation, immutability, discoverability, governed evolution and evidence. | Validated |
| `L4-A-019-BH-002/010` | Runtime use and traceability of active event version/schema/metadata. | Validated |

### 3.2 Authoritative Constraints
- Canonical event meaning, name and publisher authority remain upstream-owned.
- Technical compatibility/versioning semantics are governed by `ADR-L5-0011`.
- Registry/catalog capability never becomes the business owner of event semantics.
- JSON + governed JSON Schema and a CloudEvents 1.0-compatible envelope are canonical under `ADR-L5-0035`; only the concrete registry/catalog product remains unselected.

## 4. Technical Ownership, Boundaries & Dependencies
**Owns:** Event Contract Definition model, Schema Catalog/Registry abstraction, compatibility classifier, producer conformance, consumer supported-version declaration, deprecation/retirement technical control.  
**Consumes:** C02-001 envelope/event identity; C02-002 replay semantics; D03 sensitive-data/trust controls.  
**Must not own:** upstream event meaning/name/publisher or consumer business interpretation beyond the published contract.

## 5. Solution Architecture
A durable contract consists of: (1) transport-neutral envelope information categories; (2) publisher-owned payload schema; (3) semantic field contract; (4) lifecycle/version metadata; (5) canonical publisher and known durable consumer metadata.

A shared registry provides discoverability and automated compatibility validation but does not own business semantics. Each event contract versions independently. Additive optional fields are normally backward-compatible where the existing consumer contract permits unknown fields. Removal, incompatible requiredness/type change, or semantic reinterpretation is breaking and requires a distinct **breaking successor version**. This architecture does not mandate Semantic Versioning or any specific numbering convention. Breaking predecessor and successor versions may coexist through governed migration. Retirement requires consumer-impact evidence or approved exception.

Consumers declare supported versions, rely only on documented fields, and fail safely/quarantine unsupported breaking versions. Historical replay retains the original event-contract version. Blue/green, rollback, or equivalent deployment transitions must tolerate the governed coexistence of predecessor and successor producer/consumer versions.

## 6. Architecture & Technology Decisions
| ID | Decision | Position | Authority |
|---|---|---|---|
| AD-001 | Ownership | Canonical publisher owns event schema/semantics; registry owns no business meaning. | Repository ownership rules + `ADR-L5-0011`. |
| AD-002 | Version visibility | Version is explicit, not inferred from field presence. | `ADR-L5-0011`. |
| AD-003 | Compatibility | Backward-compatible additive evolution is preferred where contract guarantees allow it. | `ADR-L5-0011`. |
| AD-004 | Breaking changes | Distinct breaking successor version + governed coexistence/deprecation; no SemVer convention implied. | `ADR-L5-0011`. |
| AD-005 | Registry | Central governed catalog capability; product remains unselected. | `ADR-L5-0011`. |
| AD-006 | Envelope | CloudEvents 1.0-compatible envelope; JSON payload; governed JSON Schema 2020-12-family contract representation. | `ADR-L5-0035` |

## 7. Security, Privacy, Quality & Operations
Schema metadata carries classification where needed. Test fixtures must not copy production-sensitive data. Consumers receive only necessary facts. Registry and consumer-discovery access is least-privilege and does not grant event-payload access.

## 8. Architecture Conformance Requirements
- `CR-001`: Every durable event contract shall expose an explicit governed version.
- `CR-002`: Schema/version evolution shall not transfer or redefine canonical business-event meaning, name or publisher authority.
- `CR-003`: Compatible additive evolution is permitted only where existing consumer guarantees make the change non-breaking.
- `CR-004`: Breaking changes shall create a distinct breaking successor version and shall not silently replace the active predecessor contract.
- `CR-005`: The versioning model shall not assume Semantic Versioning or another numbering convention unless separately governed.
- `CR-006`: Consumers shall rely only on documented contract fields and shall govern the versions they support.
- `CR-007`: Unsupported breaking versions shall fail safely or quarantine rather than be guessed/coerced.
- `CR-008`: Historical replay shall preserve the original event-contract version.
- `CR-009`: Retirement shall require consumer-impact evidence or approved exception.
- `CR-010`: Registry/catalog capability shall not become canonical owner of business-event semantics.
- `CR-011`: JSON/JSON Schema and the CloudEvents-compatible envelope are canonical under `ADR-L5-0035`; the concrete schema-registry/catalog product remains non-canonical until separately governed.
- `CR-012`: Event-contract compatibility behavior shall conform to `ADR-L5-0011`.

## 9. Open Decisions & Deferred Design
Schema representation/serialization and envelope are resolved by `ADR-L5-0035` as JSON + governed JSON Schema with a CloudEvents-compatible envelope. Registry product remains separately governed; numeric payload/retention/capacity limits remain deferred.

No unresolved L4-authority blocker remains for this validation cycle because canonical L4 behavior is being treated as authoritative by explicit review instruction.

## 10. Traceability Summary
Candidate Corpus Register 105; validated BHs L4-A-018/A-019; `ADR-L5-0011`; C02-001/002; D03; C01-003 compatibility boundary. C01-003 retains synchronous/general contract compatibility responsibility while event-schema compatibility remains canonically owned by C02-003.

## 11. Review Position
This specification is **Architecture Approved** following completed Stage-2 remediation and explicit architecture approval on 2026-09-12. It is lock-ready but shall not be marked `Locked` until distinct final-governance authorization is recorded.


## 2026-09-22 Backend Readiness Group 1 Synchronization

**Applicable locked decision:** `ADR-L5-0035`  
Event schemas use governed JSON Schema; compatible additive evolution is preferred and breaking successors follow ADR-L5-0011. Binary serialization is exception-only for measured bounded technical streams.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Reconciled event schema/envelope text with `ADR-L5-0035`; registry product and numeric payload/retention/capacity limits remain intentionally deferred.
