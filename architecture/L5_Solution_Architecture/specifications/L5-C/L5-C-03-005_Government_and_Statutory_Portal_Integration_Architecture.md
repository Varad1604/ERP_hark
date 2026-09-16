# Government & Statutory Portal Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-005`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Governed exchange with government/statutory portals without owning source-domain facts.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-03-002`; `L5-C-01`; `L5-D-01`; `L5-D-02`; `L5-D-03`; applicable source-domain L5 architecture  
**Applicable ADRs:** `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0014`

## 2. Purpose, Scope & Non-Goals
Defines the reusable technical architecture for exchanges with government and statutory portals through official APIs, secure files or other authorized machine interfaces. It does not own the source-domain facts, statutory business determinations, compliance approval or regulatory meaning of submitted data.

## 3. Upstream Applicability & Constraints
The L5 corpus fixes this boundary as technical exchange only. Source domains remain canonical. Exact L4 BH allocation across government/statutory scenarios remains `OD-001` and is not fabricated.

## 4. Technical Ownership & Dependencies
**Owns:** portal-specific adapters, transport/profile mapping, submission correlation, acknowledgement/status retrieval, technical validation, environment/certification configuration.  
**Consumes:** source-domain contracts, D01/D02/D03 trust controls, E01 resilience, E05 telemetry.  
**Must not own:** regulatory rule interpretation, business compliance status, source business record, statutory approval, canonical audit meaning.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility |
|---|---|
| Portal Adapter | Encapsulates portal-specific protocol, authentication, schema and submission mechanics. |
| Submission Packager | Maps approved source-domain information into provider-required technical representation. |
| Acknowledgement Correlator | Binds provider acknowledgement/reference/status to the originating SutraERP request. |
| Status Poll/Callback Handler | Retrieves or receives provider technical disposition where supported. |
| Certification Profile | Separates sandbox/UAT/certification/production endpoints and credentials. |

### 5.2 Primary Flow
```text
Owning domain approved submission
→ authorize + resolve portal profile
→ map/package technical payload
→ secure portal exchange
→ provider acknowledgement/reference
→ status/callback correlation
→ owning domain decides business completion/compliance state
```

### 5.3 Material Failure Rules
A submission attempt is not equivalent to statutory acceptance. Portal unavailability, timeouts or conflicting statuses remain technical conditions until the owning business workflow resolves the authoritative outcome. Browser automation is not the preferred architecture; where no supported machine interface exists, any RPA fallback requires separately governed approval.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Integration priority | Official API/file interfaces are preferred over browser automation. |
| AD-002 | Acknowledgement authority | Provider acknowledgement is technical evidence; source domain decides business completion. |
| AD-003 | Provider isolation | Each portal uses a provider-specific adapter behind the common C03 boundary. |
| AD-004 | Certification | Non-production certification profiles are isolated from production credentials/endpoints. |

## 7. Security, Privacy & Operations
Only minimum required statutory data is transmitted. Service identities, certificates and secrets are D03-governed. Operational evidence includes correlation, portal reference, time, technical outcome and status without copying unnecessary sensitive payloads.

## 8. Architecture Conformance Requirements
- `CR-001`: Government portal connectors shall not directly mutate source-domain persistence.
- `CR-002`: Submission attempt shall not be represented as accepted/compliant without authoritative upstream rules.
- `CR-003`: Official supported machine interfaces shall be preferred to scraping/RPA.
- `CR-004`: Sandbox/UAT/certification and production credentials shall be isolated.
- `CR-005`: Portal-specific schema changes shall be contained within the adapter/mapping boundary.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking? | Resolution |
|---|---|---|---|
| OD-001 | Exact L4 BH mapping for all statutory/government exchanges. | Yes for Architecture Approved. | Complete scenario-to-BH extraction. |
| OD-002 | Any RPA-based portal integration. | Yes if proposed. | Requires explicit architecture/security review and evidence no supported machine interface exists. |

## 10. Traceability Summary
**Corpus:** `L5-C-03-005`. **Peers:** C03-001/002, C01, D01/D02/D03, E01/E05, applicable source-domain L5 owners.
