# External Education & University Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-006`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** LMS/university/external-education interoperability and credential/academic handoffs.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-C-02`; `L5-D-03`; applicable Academic/Examination L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0011`

## 2. Purpose, Scope & Non-Goals
Defines provider-neutral interoperability with LMSs, universities and external education platforms using governed APIs, secure files, events or provider-specific adapters. External systems do not become canonical owners of SutraERP academic facts merely because they supply or receive data.

## 3. Upstream Applicability & Constraints
The frozen L5 corpus assigns external education/university interoperability to this specification. Exact L4 scenario/BH mappings remain `OD-001` pending repository extraction.

## 4. Technical Ownership & Dependencies
**Owns:** external education adapters, course/credential/result exchange transport, provider schema mapping, correlation and technical delivery evidence.  
**Consumes:** C01 contracts, C02 messaging where asynchronous, D03 trust, applicable academic/examination business owners.  
**Must not own:** course/catalog authority, enrollment, result/grade authority, credential issuance meaning, academic-credit business rules.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility |
|---|---|
| Education Provider Adapter | Encapsulates provider-specific API/file/event behavior. |
| Interoperability Mapper | Maps standards/provider schemas to governed SutraERP integration contracts. |
| Exchange Correlator | Preserves external and SutraERP references for requests, imports and deliveries. |
| Import Acceptance Handoff | Routes validated external data to the authoritative academic/examination workflow for acceptance. |
| Export Publisher | Delivers approved institutional facts without transferring ownership. |

### 5.2 Primary Flow
```text
Approved source-domain request/fact
→ provider adapter + governed contract
→ external education/university system
→ response/event/file
→ technical validation + normalization
→ owning academic/examination workflow validates/accepts business meaning
→ evidence/telemetry
```

### 5.3 Failure Rules
Provider result/credential data shall not become authoritative solely because transport validation succeeds. Schema/version incompatibility is contained at the adapter boundary. Duplicate asynchronous deliveries consume C02/C01 compatibility/idempotency semantics.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Interoperability posture | Prefer recognized interoperability standards when supported, with adapters for deviations. |
| AD-002 | Academic authority | Imported results/credentials require owning-domain acceptance before authority changes. |
| AD-003 | Schema isolation | External schema evolution is absorbed by the adapter/mapping boundary. |
| AD-004 | Multi-mode exchange | APIs, events and secure file exchange are supported according to provider capability; no single protocol is mandated. |

## 7. Security, Privacy & Operations
Identity, authorization, secrets and certificates are delegated to D-family owners. Student/academic payloads are minimized in logs. Provider environments and credentials are separated by environment.

## 8. Architecture Conformance Requirements
- `CR-001`: External academic data shall not bypass authoritative academic/examination acceptance.
- `CR-002`: Provider models shall not leak into canonical domain models.
- `CR-003`: Contract/event versioning shall conform to C01/C02 compatibility architecture.
- `CR-004`: Exchange failures shall preserve correlation and permit governed replay/reconciliation without inventing business outcomes.
- `CR-005`: No direct cross-system database coupling shall be introduced as the integration mechanism.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking? | Resolution |
|---|---|---|---|
| OD-001 | Exact L4 BH mapping for LMS/university/external-education scenarios. | Yes for Architecture Approved. | Complete traceability extraction. |
| OD-002 | Canonical standards profile(s) to mandate, if any. | No. | Select only from concrete interoperability requirements through governance. |

## 10. Traceability Summary
**Corpus:** `L5-C-03-006`. **Peers:** C03-001/002, C01/C02, D03 and applicable Academic/Examination L5 owners.
