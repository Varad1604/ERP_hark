# Device, Edge, Telemetry & Physical-System Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-009`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Reusable secure integration for scanners/readers, RFID/NFC, biometrics, ANPR, GPS, telemetry, IoT, CCTV/evidence feeds and controlled actuators; devices do not own business decisions.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.0`  
**Status:** `Draft`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-02`; `L5-A-08`; `L5-D-01`; `L5-D-02`; `L5-D-03`; `L5-E-01`; `L5-E-05`; applicable business-domain L5 architecture  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0013`; `ADR-L5-0014`

## 2. Purpose, Scope & Non-Goals
Defines the secure technical boundary for physical devices, edge systems and telemetry while ensuring observations never become business decisions merely because hardware reported them.

## 3. Upstream Applicability & Constraints
The frozen corpus explicitly separates device integration from business authorization, identity, asset, trip, examination and other domain ownership. Exact cross-domain L4 BH mappings remain `OD-001`.

## 4. Technical Ownership & Dependencies
**Owns:** device/edge adapters, protocol normalization, device identity binding, telemetry ingestion, bounded offline buffering/store-forward where required, command dispatch/acknowledgement, device health and evidence references.  
**Consumes:** D01/D02/D03 identity/trust, C02 messaging, A08 background work, E01 resilience, E05 observability, owning-domain command/decision contracts.  
**Must not own:** attendance, access authorization, transport trip state, asset ownership, biometric identity truth, disciplinary outcome, examination outcome or safety policy.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility |
|---|---|
| Device/Edge Adapter | Encapsulates vendor/protocol-specific connectivity and normalizes observations/commands. |
| Device Identity Resolver | Binds registered device identity and authorization context without equating device ID to Person. |
| Telemetry Ingest Boundary | Validates timestamp/context, deduplicates where justified and routes observations. |
| Store-and-Forward Buffer | Retains bounded offline observations where the use case requires intermittent connectivity. |
| Actuator Command Gateway | Sends only authorized commands, requires acknowledgement and exposes failure/safety state. |
| Evidence Reference Adapter | Links CCTV/media/telemetry evidence without copying continuous media unless required. |

### 5.2 Observation Flow
```text
Device observation
→ authenticated/registered edge boundary
→ protocol validation + normalization
→ timestamp/correlation/deduplication
→ owning business workflow
→ business decision by canonical owner
→ E05 telemetry/evidence reference
```

### 5.3 Command Flow
```text
Owning domain authorizes action
→ D02 policy enforcement
→ Actuator Command Gateway
→ device/edge adapter
→ physical actuator
→ command acknowledgement / failure evidence
```

### 5.4 Failure Rules
Offline devices may buffer and later forward observations only with sequence/deduplication controls appropriate to the use case. Reconnection shall not create duplicate business effects. Device failure shall degrade only dependent capability. Raw biometric/video payload propagation is minimized.

## 6. Architecture Decisions
| ID | Decision | Position |
|---|---|---|
| AD-001 | Device model | Device/edge adapters normalize heterogeneous protocols into governed internal contracts. |
| AD-002 | Decision authority | Devices produce observations; canonical domains make business decisions. |
| AD-003 | Offline behavior | Bounded store-and-forward is supported where required, with reconciliation/deduplication. |
| AD-004 | Actuators | Commands require explicit authorization, acknowledgement and fail-safe handling. |
| AD-005 | Media | Prefer governed metadata/evidence references over continuous-media replication into ERP stores. |

## 7. Security, Privacy & Operations
Device identities, certificates and authorization are D-family governed. Sensitive biometric/location/video data is minimized, access-scoped and excluded from general logs. E05 receives connector/device health, last contact, backlog and failure telemetry.

## 8. Architecture Conformance Requirements
- `CR-001`: Device observations shall not directly establish business outcomes.
- `CR-002`: Devices/edge nodes shall be authenticated/authorized before accepted exchange.
- `CR-003`: Offline replay shall include controls preventing duplicate technical/business effect.
- `CR-004`: Actuator commands shall originate from an authorized owning-domain decision and return acknowledgement/evidence.
- `CR-005`: Continuous media shall not be copied into ERP persistence without an explicit governed requirement.
- `CR-006`: Device-specific protocols shall remain isolated behind adapters.

## 9. Open Decisions & Deferred Design
| ID | Item | Blocking? | Resolution |
|---|---|---|---|
| OD-001 | Exact L4 BH mappings across attendance/access/transport/assets/exams/facilities. | Yes for Architecture Approved. | Complete cross-domain L4 traceability extraction. |
| OD-002 | Device protocol/vendor profiles and numeric buffer limits. | No. | Decide from concrete device/provider requirements through governance. |

## 10. Traceability Summary
**Corpus:** `L5-C-03-009`. **Peers:** C03-001, C02, A08, D01/D02/D03, E01/E05 and applicable business-domain L5 owners.
