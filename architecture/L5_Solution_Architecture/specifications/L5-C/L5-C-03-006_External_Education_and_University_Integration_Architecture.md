# External Education & University Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-006`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** LMS/university/external-education interoperability and credential/academic handoffs.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.1`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-C-02`; `L5-D-03`; applicable Academic/Examination L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0011`; shared C03 family ADR required but not yet canonically allocated

## 2. Purpose, Scope & Non-Goals
Defines provider-neutral interoperability with LMSs, universities and external education platforms while preserving canonical academic, Student, Examination and credit ownership.

## 3. Upstream Applicability & Constraints
### 3.1 Canonical Upstream Reference Matrix
| Source | Version / Status | Exact BHs | C03 Technical Contribution |
|---|---|---|---|
| `L4-H-077` | 1.0.0 / Draft | `BH-001`–`BH-008` | Synchronize external learning progress only from valid provider/mapping evidence and never infer canonical Student/academic progression. |
| `L4-H-078` | 1.0.1 / Draft | `BH-001`–`BH-011` | Carry verified external credit evidence to the canonical academic owner and synchronize its authoritative decision without updating the Academic Credit Ledger. |
| `L4-H-081` | 1.0.1 / Draft | `BH-001`–`BH-012` | Govern technical exchange/send/receipt/evidence while source-domain facts remain canonical; quarantine is business-governed, not a C03 retry state. |
| L2-H10 | 1.0 / Locked | External Education Integration Workflows | Establishes external-provider orchestration, standards-capable interoperability and canonical-ownership preservation. |

The L4 relations are `Specified`, not `Validated`, while their source files remain Draft.

### 3.2 Authoritative Constraints
- Imported learning/result/credential/credit data shall not become authoritative solely because transport/schema validation succeeds.
- C03 shall not calculate academic equivalency, determine progression, update the Academic Credit Ledger or republish an academic decision as C03/H10-owned unless upstream explicitly defines an H10 orchestration event.
- Business `Quarantined` or other H10 lifecycle states shall not be replaced by C03 technical retry states.
- External schemas/protocols are isolated behind adapters/mapping.

## 4. Technical Ownership & Dependencies
**Owns:** external education adapters, provider schema/protocol mapping, exchange correlation, technical send/receipt evidence, and handoff to authoritative academic/examination workflows.  
**Consumes:** C01 contracts, C02 messaging where asynchronous, D03 trust, source-domain contracts/events.  
**Must not own:** course/catalog authority, enrollment truth, result/grade authority, academic progression, credit recognition, Academic Credit Ledger or credential business meaning.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility | Boundary |
|---|---|---|
| Education Provider Adapter | Encapsulates provider API/file/event behavior. | Does not own academic facts. |
| Interoperability Mapper | Maps provider/standards representation to governed integration contracts. | Provider model does not leak into canonical domain model. |
| Exchange Correlator | Preserves SutraERP/provider references across send/receipt/import/export. | Does not create source-domain identity. |
| Import Acceptance Handoff | Routes technically valid external evidence to authoritative academic/examination workflow. | Technical validity is not business acceptance. |
| Export Publisher | Delivers approved source-domain facts. | Cannot modify source facts. |

### 5.2 Primary Flow
```text
Authoritative source request/fact or verified provider input
→ provider adapter + governed contract
→ technical validation + mapping + correlation
→ external send/receipt or normalized inbound evidence
→ authoritative academic/examination/Student owner validates business meaning
→ canonical owner decides/mutates its state
→ C03 records technical delivery evidence
```

### 5.3 Failure Rules
Invalid mapping/provider evidence shall not alter canonical academic facts. Failed external synchronization after an authoritative academic decision shall leave that decision intact. Duplicate asynchronous deliveries consume C01/C02 idempotency/compatibility rules and shall not create duplicate business effect.

## 6. Architecture Decisions
| ID | Decision | Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Interoperability | Provider-authorized APIs/events/secure files and recognized interoperability profiles may be supported through adapters; no single mandatory standard is selected here. | L2-H10 + C03 product-neutrality. | Required only if a shared mandatory technology/profile is selected. |
| AD-002 | Academic authority | Import requires canonical owner acceptance; transport validity does not confer authority. | H-077/H-078/H-081. | None. |
| AD-003 | Schema isolation | External evolution is absorbed at mapping/adaptor boundary. | H-081 ownership preservation. | Shared C03 ADR may govern family pattern. |
| AD-004 | Async compatibility | Asynchronous exchange consumes C02 delivery/versioning semantics. | C02 + H-077/H-078 handoffs. | `ADR-L5-0010`, `0011`. |

## 7. Security, Privacy & Operations
Identity, authorization, secrets and certificates consume D-family owners. Student/academic payloads are minimized in logs. Provider profiles and credentials are environment-scoped. C03 exposes exchange correlation, status and dependency health to E05 without treating telemetry as academic audit truth.

## 8. Architecture Conformance Requirements
- `CR-001`: External academic evidence shall not bypass authoritative academic/examination/Student acceptance.
- `CR-002`: Provider schemas/protocols shall remain isolated behind C03 adapters/mapping.
- `CR-003`: Contract/event evolution shall conform to C01/C02 architecture.
- `CR-004`: External synchronization failure after an authoritative academic decision shall not invalidate that decision.
- `CR-005`: C03 shall not update the Academic Credit Ledger or infer progression/equivalency.
- `CR-006`: Technical retry state shall not replace upstream business exception/quarantine state.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | H-077/H-078/H-081 are Draft. | Upstream / Traceability | Yes for Architecture Approved absent authorization. | Approve/review upstream or authorize approval-level use; then validate relations. |
| OD-002 | Shared C03 adapter/outcome-isolation pattern. | ADR | Yes for Architecture Approved if retained as family standard. | Allocate/evaluate canonically. |
| OD-003 | Mandatory interoperability standard/profile, if any. | Technology | No. | Select only from concrete provider/institution requirements through governance. |

## 10. Traceability Summary
**Specified L4:** `L4-H-077-BH-001`–`008`; `L4-H-078-BH-001`–`011`; `L4-H-081-BH-001`–`012`. **Higher-level:** locked L2-H10. **Blocking:** OD-001, OD-002.
