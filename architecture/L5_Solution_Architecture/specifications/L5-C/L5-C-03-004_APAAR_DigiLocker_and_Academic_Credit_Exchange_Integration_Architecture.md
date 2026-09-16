# APAAR, DigiLocker & Academic Credit Exchange Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-004`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Technical education identity/credential/credit exchange integration without owning institutional academic facts.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.2`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-D-01`; `L5-D-02`; `L5-D-03`; applicable Academic/Identity L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0016`

## 2. Purpose, Scope & Non-Goals
Defines provider-facing integration for APAAR, DigiLocker and academic-credit exchange capabilities while preserving SutraERP canonical Person, Student, academic-result, credential and credit ownership.

## 3. Upstream Applicability & Constraints
### 3.1 Canonical Upstream Reference Matrix
| Source | Version / Status | Exact BHs | C03 Technical Contribution |
|---|---|---|---|
| `L4-H-001` | 1.0.0 / Draft | `BH-001`–`BH-008` | Provider linking requires valid source references, consent/authorization and connector eligibility; external acknowledgement/evidence is correlated without altering Person/Student facts. |
| `L4-H-002` | 1.0.0 / Draft | `BH-001`–`BH-007` | Publish an already approved credential, require government acknowledgement for `Published`, preserve source ownership and evidence. |
| `L4-H-006` | 1.0.0 / Draft | `BH-001`–`BH-008` | Bulk publication preserves per-credential eligibility/outcome and shall not invent a separate business Batch aggregate/lifecycle. |
| L2-H01 | Locked | APAAR & DigiLocker workflows/amendments | Establishes canonical H01 ownership, consent and connector-governance meaning. |
| `ADR-L5-0016` | Architecture Approved | shared connector pattern | Governs provider adapter isolation, configuration, credentials, correlation and failure containment. |

The L4 sources are Draft; all mappings are `Specified`, not `Validated`, until upstream approval or explicit governance authorization.

### 3.2 Authoritative Constraints
- APAAR/provider identifiers remain external references; they do not replace canonical Person/Student identifiers.
- External publication/linking failure shall not alter authoritative academic/source facts.
- C03 shall not duplicate H01 business states/events or consent lifecycle.
- Credential/credit imports become authoritative only through the owning academic workflow.
- Provider-specific payload/schema changes remain isolated at the adapter/mapping boundary.

## 4. Technical Ownership & Dependencies
**Owns:** APAAR/provider adapters, DigiLocker issuer/requester transport, academic-credit exchange adapters, external-identifier linkage mechanics, provider schema mapping, callback/status correlation and technical provider evidence.  
**Must not own:** canonical Person/Student identity, academic results, institutional credentials, Academic Credit Ledger, H01 Publication/Consent lifecycle or business reconciliation rules.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility | Boundary |
|---|---|---|
| Education Identity Adapter | Encapsulates authorized APAAR/provider exchange mechanics. | External identifier is a reference only. |
| DigiLocker Role Adapter | Executes issuer/requester operations only for institutionally/provider-authorized roles. | Does not approve or reinterpret credentials. |
| Credit Exchange Adapter | Transports approved credit information and external credit evidence. | Owning academic capability accepts/reconciles business meaning. |
| External Identifier Link Resolver | Correlates canonical internal identity references with governed provider identifiers. | Never replaces canonical Person/Student key. |
| Provider Evidence Correlator | Preserves provider request/publication/credit references, acknowledgement and technical outcome. | Evidence does not become source-domain truth. |

### 5.2 Flow
```text
Owning-domain approved action/fact
→ verify authoritative role/purpose/consent prerequisites
→ resolve external identifier/provider profile
→ C03 provider adapter + D03 trust controls
→ provider response/acknowledgement
→ normalize/correlate technical evidence
→ owning H01/academic workflow applies canonical business outcome
```

### 5.3 Failure Rules
Provider rejection, timeout or publication failure shall not revoke/correct the source credential or academic fact. External/internal mismatches are handed to the canonical reconciliation workflow; silent overwrite is prohibited. Bulk execution preserves independent item correlation and shall not create a C03 business batch lifecycle.

## 6. Architecture Decisions
| ID | Decision | Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Provider roles | Issuer/requester/provider roles are enabled only where authoritative onboarding permits them. | H01 prerequisites and connector eligibility. | `ADR-L5-0016` |
| AD-002 | Identity linkage | External education IDs are governed references, not canonical Person keys. | `L4-H-001-BH-002`; canonical identity ownership. | `ADR-L5-0016` ownership-preserving handoff. |
| AD-003 | Academic authority | External exchange transports evidence; source academic owners retain truth. | `L4-H-002-BH-002`; `L4-H-006`. | `ADR-L5-0016` |
| AD-004 | External conflict | Mismatch becomes reconciliation input; no automatic overwrite. | Ownership preservation. | `ADR-L5-0016` |

## 7. Security, Privacy & Operations
Authorization, service identity, credentials and transport protection consume D01/D02/D03. Purpose/consent is consumed from authoritative upstream state rather than recreated in C03. Non-production certification/sandbox and production profiles/credentials remain isolated where provider contracts require them. Sensitive identity/education data is minimized in telemetry.

## 8. Architecture Conformance Requirements
- `CR-001`: External education identifiers shall not replace SutraERP canonical identity keys.
- `CR-002`: Provider delivery/linking failure shall not mutate authoritative academic/source facts.
- `CR-003`: Provider schema evolution shall be contained by C03 mapping/adapters.
- `CR-004`: Imported credit/credential evidence shall require owning-domain acceptance before authoritative business mutation.
- `CR-005`: Role, purpose, authorization and required consent prerequisites shall be verified from authoritative sources before exchange.
- `CR-006`: C03 shall not create H01 business lifecycle states/events or a separate business batch lifecycle.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | H-001/H-002/H-006 are Draft. | Upstream / Traceability | Yes for Architecture Approved absent authorization. | Approve/review upstream or explicitly authorize approval-level use; then validate mappings. |
| OD-002 | Provider-specific protocol/onboarding profile. | Provider / Technology | No. | Bind from official provider contracts through governance. |

## 10. Traceability Summary
**Specified L4:** `L4-H-001-BH-001`–`008`; `L4-H-002-BH-001`–`007`; `L4-H-006-BH-001`–`008`. **Higher-level verification:** locked L2-H01. **Applicable shared ADR:** `ADR-L5-0016`. **Blocking:** OD-001.
