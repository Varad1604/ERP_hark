# Government & Statutory Portal Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-005`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Governed exchange with government/statutory portals without owning source-domain facts.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.2`  
**Status:** `Locked`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-D-01`; `L5-D-02`; `L5-D-03`; applicable source-domain L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0014`; `ADR-L5-0016`  
**Governance Authorization:** `L5_Pallav_Stage5_Architecture_Approval_Decision_2026-09-16.md`  
**Governance Lock:** `L5_Pallav_Stage5_Final_Lock_Decision_2026-09-16.md`

## 2. Purpose, Scope & Non-Goals
Defines reusable technical exchange with government/statutory portals through authorized machine interfaces while preserving source-domain ownership of package content, reporting facts, compliance decisions and regulatory meaning.

## 3. Upstream Applicability & Constraints
### 3.1 Canonical Upstream Reference Matrix
| Source | Version / Status | Exact BHs | C03 Technical Contribution |
|---|---|---|---|
| `L4-H-059` | 1.0.1 / Draft | `BH-001`–`BH-008` | Admit only governed approved submission, transport it without mutating the package/snapshot, correlate external acknowledgement/outcome, and preserve evidence. |
| L2-H08 | Locked + 2026-09-01 amendment | submission execution/closure governance | Separates institutional package approval, submission authority and external regulator decision ownership. |
| L5 corpus | approved population | `L5-C-03-005` | Assigns portal exchange mechanics to C03 without source-domain ownership transfer. |
| `ADR-L5-0016` | Architecture Approved | shared C03 connector pattern | Governs portal adapter isolation, configuration, credentials, correlation and failure containment. |

`L4-H-059` remains Draft, but its cited behavior is explicitly governance-authorized for this Stage-5 approval. The relation remains `Specified`, not `Validated`, until the locked traceability methodology's normal source-status conditions are met.

### 3.2 Authoritative Constraints
- Transmission/submission attempt is not external acceptance.
- C03 shall preserve the approved source package/snapshot meaning and shall not mutate authoritative source-domain facts.
- External acknowledgement, rejection, remarks or resubmission requirement is correlated evidence; business reporting/compliance state remains with the source workflow.
- C03 shall not invent statutory interpretation, compliance approval or regulatory events.

## 4. Technical Ownership & Dependencies
**Owns:** portal adapters, transport/profile mapping, technical packaging, submission correlation, acknowledgement/status retrieval, structural/transport validation and environment/certification binding.  
**Consumes:** source-domain approved submission contracts, D01/D02/D03 trust controls, E01 resilience, E05 telemetry.  
**Must not own:** regulatory rule interpretation, institutional package approval, external regulator decision, reporting lifecycle, source data or canonical audit meaning.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility | Boundary |
|---|---|---|
| Portal Adapter | Encapsulates provider-specific machine interface, authentication and schema mechanics. | No regulatory/business interpretation. |
| Submission Packager | Maps the approved source package into provider-required technical representation. | Cannot alter approved business meaning. |
| Acknowledgement Correlator | Binds external reference/acknowledgement/status to originating SutraERP Submission Record/request. | Does not declare acceptance/compliance. |
| Status/Callback Handler | Retrieves or receives external technical disposition. | Returns normalized evidence to source workflow. |
| Environment Profile | Separates provider-authorized non-production/certification and production endpoints/credentials where applicable. | D03 owns secrets/certificates. |

### 5.2 Primary Flow
```text
Authoritative approved package/submission instruction
→ verify source authorization + portal profile
→ technical map/package without changing source meaning
→ secure external exchange
→ provider acknowledgement/reference/status
→ correlate + normalize technical evidence
→ source reporting/compliance workflow establishes authoritative outcome
```

### 5.3 Material Failure Rules
Failed/incomplete transmission shall not be represented as acknowledged or accepted. Conflicting/ambiguous provider statuses remain technical evidence for source-domain resolution. Unsupported browser automation is not silently substituted for an authorized interface; if a portal has no supported machine interface, any automation fallback remains a separate architecture/security decision.

## 6. Architecture Decisions
| ID | Decision | Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Portal isolation | Portal-specific adapters behind common C03 boundary. | Frozen C03 responsibility; schema/provider isolation. | `ADR-L5-0016` |
| AD-002 | Acknowledgement authority | Provider acknowledgement is evidence; source workflow owns accepted/rejected/compliance outcome. | `L4-H-059-BH-004`–`BH-008`. | `ADR-L5-0016` ownership-preserving handoff. |
| AD-003 | Source immutability | Technical packaging must not mutate approved package/snapshot meaning. | `L4-H-059-BH-003`. | `ADR-L5-0016` |
| AD-004 | Interface selection | Use only provider-authorized/governed interfaces; no repository-wide protocol/product is selected here. | Technology-selection governance. | `ADR-L5-0016` for shared pattern; separate ADR if a material product is selected. |

## 7. Security, Privacy & Operations
Only purpose-authorized data is transmitted. Service identity, certificates and secrets consume D-family controls. Telemetry/evidence includes correlation, portal reference, time and technical disposition while minimizing sensitive payloads.

## 8. Architecture Conformance Requirements
- `CR-001`: Portal connectors shall not directly mutate source-domain persistence or approved package meaning.
- `CR-002`: Submission attempt shall not be represented as external acceptance/compliance.
- `CR-003`: External acknowledgement/outcome shall remain correlated to the authoritative source Submission Record/request.
- `CR-004`: Portal-specific schema/protocol changes shall be contained inside C03 adapters/mapping.
- `CR-005`: Provider environment credentials shall consume D03 controls and remain appropriately isolated.
- `CR-006`: Browser/RPA fallback, if ever required, shall not be introduced without separate governed architecture/security review.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | Promotion of `L4-H-059` relation from `Specified` to `Validated`. | Traceability | No for this Architecture Approved specification. | Revalidate after applicable L4 approval. |
| OD-002 | Any browser/RPA portal integration. | Architecture/Security | Yes only if proposed. | Prove no supported interface and complete governed review. |

## 10. Traceability Summary
**Governance-authorized / Specified L4:** `L4-H-059-BH-001`–`008`. **Higher-level verification:** locked L2-H08 + submission amendment. **Peers:** C03-001/002, C01, D01/D02/D03, E01/E05. **Applicable shared ADR:** `ADR-L5-0016`. **Blocking:** none.
