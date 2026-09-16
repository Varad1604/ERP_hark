# APAAR, DigiLocker & Academic Credit Exchange Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Technical education identity/credential/credit exchange integration without owning institutional academic facts.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-D-01`; `L5-D-02`; `L5-D-03`; applicable Academic/Identity L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`

## 2. Purpose, Scope & Non-Goals
Defines the provider-facing integration architecture for APAAR, DigiLocker and academic-credit exchange capabilities. It supports authorized issuer/requester and identity/credit exchange roles while preserving SutraERP's canonical Person, academic-result, credential and credit ownership.

## 3. Upstream Applicability & Constraints
Locked L2 H01 architecture establishes APAAR/DigiLocker workflows and institutional ownership boundaries. C03-004 consumes those meanings and does not replace them. Exact L4 BH mappings remain `OD-001` pending complete extraction.

## 4. Technical Ownership & Dependencies
**Owns:** provider adapters, external-identifier linkage mechanics, issuance/request transport, callback/status correlation, provider schema mapping, sandbox/certification configuration.  
**Must not own:** canonical Person identity, academic results, institutional credentials, Academic Credit Ledger, consent business ownership or provider business rules.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility |
|---|---|
| Education Identity Adapter | Encapsulates APAAR/provider API behavior and external identifier exchange. |
| DigiLocker Issuer/Requester Adapter | Supports authorized institution role-specific issuance/request flows. |
| Credit Exchange Adapter | Transports approved academic-credit data to/from external credit-exchange services. |
| External Identifier Link Resolver | Maintains governed reference linkage without replacing canonical Person/Student identifiers. |
| Provider Evidence Correlator | Associates external request/issuance/credit references with SutraERP-owned facts. |

### 5.2 Flow
```text
Owning domain approved fact/action
→ authorize purpose + role
→ resolve external identity/link
→ provider adapter
→ provider validation/response
→ normalized technical evidence
→ owning domain accepts/reconciles resulting business outcome
```

### 5.3 Failure Rules
External rejection or delivery failure shall not revoke or alter the underlying institutional academic fact. Conflicting external credit/identity evidence is routed to the owning business workflow for reconciliation; no silent overwrite is permitted.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Provider role | Support issuer/requester roles only when institution is authorized/onboarded. |
| AD-002 | Identity linkage | APAAR/external IDs remain governed references, not canonical Person keys. |
| AD-003 | Academic authority | DigiLocker/credit exchange transports evidence; source academic domains retain truth. |
| AD-004 | External conflict | Mismatches become reconciliation conditions, not automatic overwrites. |

## 7. Security, Privacy & Operations
Purpose, authorization and consent context shall be enforced from authoritative upstream capabilities. Sensitive identity/education data is minimized in logs. Sandbox/UAT/certification and production endpoints/credentials are isolated.

## 8. Architecture Conformance Requirements
- `CR-001`: External education identifiers shall not replace SutraERP canonical identity keys.
- `CR-002`: External issuance failure shall not mutate authoritative academic facts.
- `CR-003`: Provider payload changes shall be absorbed by the connector mapping boundary.
- `CR-004`: Credit/credential imports shall require owning-domain validation before becoming authoritative.
- `CR-005`: Role, purpose and authorization shall be enforced before provider exchange.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking? | Resolution |
|---|---|---|---|
| OD-001 | Exact L4 BH mapping for APAAR/DigiLocker/credit-exchange scenarios. | Yes for Architecture Approved. | Extract and validate applicable H-section L4 BHs. |
| OD-002 | Provider-specific protocol/profile details. | No. | Bind only from official provider onboarding/contracts. |

## 10. Traceability Summary
**Corpus:** `L5-C-03-004`. **Higher-level verification:** locked L2 H01 APAAR & DigiLocker workflows/amendments. **Peers:** C03-001/002, C01, D01/D02/D03 and applicable academic/identity L5 owners.
