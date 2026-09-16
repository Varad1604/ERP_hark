# Backup, Restore & Disaster Recovery Architecture Baseline

## 1. Specification Metadata
**Specification ID:** `L5-E-03-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-E — Reliability, Deployment & Operations Architecture`  
**Major Area:** `L5-E-03 — Backup, Restore & Disaster Recovery Architecture`  
**Canonical Technical Responsibility:** Recovery objectives and common backup/restore/DR architecture for supported deployment models.  
**Technical Architecture Owner:** `L5-E-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-B-02`; `L5-E-01`; `L5-E-04`; `L5-D-03`; `L5-E-05`; applicable data-owner L5 architecture  
**Referenced By:** `L5-E-03-002`; `L5-E-03-003`  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0005`; `ADR-L5-0014`; repository recovery-strategy ADR required but not yet canonically allocated

## 2. Purpose, Scope & Non-Goals
Defines the common product-neutral recovery architecture that classifies recoverability requirements, protects recoverable copies and coordinates backup, restore and disaster-recovery mechanisms across SutraERP deployment models.

**In scope:** recovery classes, RPO/RTO derivation boundary, protected-copy requirements, restore validation, multi-store consistency expectations, isolated recovery copies and DR capability classification.  
**Out of scope:** inventing universal numeric RPO/RTO values, selecting backup/cloud/storage products, redefining B02 persistence, business-record retention or incident-management ownership.

## 3. Upstream Applicability & Constraints
The frozen L5 corpus assigns recovery objectives and common backup/restore/DR architecture to E03. B02 owns persistence characteristics; E01 owns availability/resilience expectations; E04 owns supported deployment/runtime models; D03 owns cryptography/key/secrets controls.

Exact L4 BH traceability for recovery-sensitive business capabilities remains `OD-001`; no numeric target is invented without an authoritative source.

## 4. Technical Ownership & Dependencies
**Owns:** recovery-class model, technical recovery objectives binding, protected-copy strategy, restore-readiness rules, cross-store recovery grouping, recoverability evidence requirements.  
**Consumes:** B02 store topology/durability; E01 resilience; E04 deployment model; D03 encryption/key management; E05 telemetry.  
**Must not own:** database product selection, source business retention, availability SLA ownership, cryptographic primitives or business recovery decisions.

## 5. Solution Architecture
### 5.1 Recovery Classification Model
| Element | Responsibility |
|---|---|
| Recovery Class | Groups technical stores/services by required recoverability and business impact without fixing unsupported numbers. |
| Recovery Objective Binding | Associates approved RPO/RTO targets with the assigned class once authoritative values exist. |
| Protected Copy Set | Defines recoverable base copies plus change-log/PITR capability where required. |
| Isolation Boundary | Ensures at least one required recovery copy is outside the primary failure/security domain. |
| Recovery Group | Coordinates stores that must return to a mutually compatible business point. |
| Recovery Evidence | Records backup completion, restore proof, integrity validation and unresolved recovery gaps. |

### 5.2 Baseline Flow
```text
Authoritative store/service classification
→ assign recovery class
→ bind approved RPO/RTO when available
→ generate protected recoverable copies
→ isolate/encrypt copies per D03/E04
→ scheduled restore validation
→ record recoverability evidence
→ feed E03-002/E03-003 execution architecture
```

### 5.3 Core Rules
- Backup frequency/technology follows approved RPO, store capability and recovery class rather than one repository-wide interval.
- Infrastructure snapshots alone do not prove application recoverability.
- Derived caches/indexes may be rebuilt from authoritative sources when safe; otherwise they require explicit recovery classification.
- Critical recovery copies require isolation appropriate to site/region/security threat; no single cloud/provider is assumed.
- Backup retention is distinct from business-record retention.

## 6. Architecture & Technology Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Recovery model | Tiered/class-based recoverability; no universal RPO/RTO. |
| AD-002 | Copy protection | Encrypted, access-controlled and isolated recovery copies according to class. |
| AD-003 | Recoverability proof | Backup success and restore success are distinct; tested restoration is required. |
| AD-004 | Portability | Recovery architecture remains provider-neutral across supported on-prem/cloud-capable deployments. |
| AD-005 | Cross-store recovery | Dependent stores may be grouped to recover to a compatible business point. |

## 7. Security, Privacy, Quality & Operations
Backup data inherits source sensitivity. Encryption and key custody are D03-owned; key material shall not be co-located in a way that defeats recovery isolation. Privileged backup deletion/expiry operations require authorization and evidence. E05 observes backup/restore health without treating operational telemetry as business audit truth.

## 8. Architecture Conformance Requirements
- `CR-001`: Every governed durable store shall have an explicit recovery classification or an explicit approved rebuild-only designation.
- `CR-002`: Numeric RPO/RTO shall not be invented by E03; they require authoritative approval.
- `CR-003`: At least one required recovery copy for critical classes shall be isolated from the primary failure/security domain.
- `CR-004`: Backup completion shall not be treated as proof of recoverability without restore validation.
- `CR-005`: Recovery of dependent stores shall preserve compatible authoritative state and avoid competing writers.
- `CR-006`: Backup encryption/key handling shall conform to D03 and preserve recoverability.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | Exact L4 BH traceability for recovery-critical business capabilities. | Traceability | Yes for Architecture Approved. | Complete repository recovery-impact mapping. |
| OD-002 | Repository/shared recovery-strategy ADR. | ADR | Yes for Architecture Approved. | Allocate/evaluate via canonical ADR register because shared recovery strategy is a mandatory ADR trigger. |
| OD-003 | Numeric recovery objectives and retention values. | Upstream/Operations | No for Draft. | Bind only from approved service/recovery profiles. |
| OD-004 | Named backup/storage/cloud technology. | Technology | No. | Remain unselected unless governed selection is required. |

## 10. Traceability Summary
**Corpus:** `L5-E-03-001`. **Peers:** B02, E01, E04, D03, E05. **ADRs:** 0002, 0005, 0014; new repository recovery ADR required before approval. **Decisions:** AD-001–005. **Conformance:** CR-001–006.
