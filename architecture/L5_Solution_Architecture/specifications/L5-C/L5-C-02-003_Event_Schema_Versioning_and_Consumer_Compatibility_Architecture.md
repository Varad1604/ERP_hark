# Event Schema, Versioning & Consumer Compatibility Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-02-003`  
**Architecture Level:** L5 — Solution Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-12`  
**Governing Sources:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register 105; L5 L4-to-L5 Traceability Methodology; L5 Technology Selection & ADR Methodology; L5 ADR Register.  
**Canonical Technical Responsibility:** Technical event schema evolution and consumer compatibility while canonical event names/publishers remain upstream-owned.  
**Applicable ADRs:** Schema-registry/serialization/envelope product choices remain unresolved if made canonical.

## 2. Purpose, Scope & Non-Goals
Defines event contract representation, explicit versioning, compatibility classification, schema catalog/registry capability, consumer version declarations, deprecation and retirement. It does not create/rename business events, publishers or business meaning and does not own transport/retry/replay.

## 3. Upstream Applicability, Constraints & Assumptions
### 3.1 L4 Planning Traceability
| L4 BH | Contribution | State |
|---|---|---|
| `L4-A-018-BH-003/004/005/006/007/008/009` | Schema/contract validation, activation, immutability, discoverability, governed evolution and evidence. | Specified |
| `L4-A-019-BH-002/010` | Runtime use and traceability of active event version/schema/metadata. | Specified |

The cited L4 files remain Draft; mapping is planning-only.

## 4. Technical Ownership, Boundaries & Dependencies
**Owns:** Event Contract Definition model, Schema Catalog/Registry abstraction, compatibility classifier, producer conformance, consumer supported-version declaration, deprecation/retirement technical control.  
**Consumes:** C02-001 envelope/event identity; C02-002 replay semantics; D03 sensitive-data/trust controls.  
**Must not own:** upstream event meaning/name/publisher or consumer business interpretation beyond the published contract.

## 5. Solution Architecture
A durable contract consists of: (1) transport-neutral envelope categories; (2) publisher-owned payload schema; (3) semantic field contract; (4) lifecycle/version metadata; (5) canonical publisher and known durable consumer metadata.

A shared registry provides discoverability and automated compatibility validation but does not own business semantics. Each event contract versions independently. Additive optional fields are normally backward-compatible where consumers tolerate unknown fields. Removal, requiredness/type incompatibility, or semantic reinterpretation is breaking and requires a new major version. Major versions may coexist through governed migration. Retirement requires consumer-impact evidence or approved exception.

Consumers declare supported versions, rely only on documented fields, and fail safely/quarantine unknown major versions. Historical replay retains the original event version. Blue/green and rollback windows must tolerate coexistence of old/new producer/consumer versions.

## 6. Architecture & Technology Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Ownership | Canonical publisher owns event schema/semantics; registry owns no business meaning. |
| AD-002 | Version visibility | Version is explicit, not inferred from field presence. |
| AD-003 | Compatibility | Backward-compatible additive evolution is preferred. |
| AD-004 | Breaking changes | New major version + governed coexistence/deprecation. |
| AD-005 | Registry | Central governed catalog capability; product remains unselected. |
| AD-006 | Envelope | Open/portable envelope alignment may be adopted by ADR/standard; not silently locked. |

## 7. Security, Privacy, Quality & Operations
Schema metadata carries classification where needed. Test fixtures must not copy production-sensitive data. Consumers receive only necessary facts. Registry and consumer-discovery access is least-privilege and does not grant event-payload access.

## 8. Architecture Conformance Requirements
Every durable event has explicit governed schema/version; business meaning/publisher cannot be changed by schema evolution; breaking changes cannot silently replace existing major version; consumers cannot depend on undocumented producer internals; unknown unsupported major versions fail safely; retirement requires impact evidence; registry/serialization technology remains non-canonical until governed.

## 9. Open Decisions & Deferred Design
Schema representation/serialization, registry product and optional envelope standard require ADR/technical-standard governance if made canonical. Draft L4-A-018/A-019 status blocks Architecture Approved unless upstream approval or explicit canonical Draft-L4 authority exists.

## 10. Traceability Summary
Candidate Corpus Register 105; planning BHs L4-A-018/A-019; C02-001/002; D03; C01-003 compatibility principles.
