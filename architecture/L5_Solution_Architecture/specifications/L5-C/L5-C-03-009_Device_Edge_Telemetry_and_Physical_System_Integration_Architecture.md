# Device, Edge, Telemetry & Physical-System Integration Architecture

## 1. Specification Metadata
**Specification ID:** `L5-C-03-009`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** `L5-C — Contracts, Events & Integration Architecture`  
**Major Area:** `L5-C-03 — External Integration & Connector Architecture`  
**Canonical Technical Responsibility:** Reusable secure integration for scanners/readers, RFID/NFC, biometrics, ANPR, GPS, telemetry, IoT, CCTV/evidence feeds and controlled actuators; devices do not own business decisions.  
**Technical Architecture Owner:** `L5-C-03`  
**Assigned Author:** Pallav  
**Version:** `1.0.2`  
**Status:** `Locked`  
**Last Updated:** `2026-09-16`  
**Depends On:** `L5-C-03-001`; `L5-C-02`; `L5-A-08`; `L5-D-01`; `L5-D-02`; `L5-D-03`; `L5-E-01`; `L5-E-05`; applicable business-domain L5 architecture  
**Applicable ADRs:** `ADR-L5-0001`; `ADR-L5-0002`; `ADR-L5-0009`; `ADR-L5-0010`; `ADR-L5-0013`; `ADR-L5-0014`; `ADR-L5-0016`  
**Governance Authorization:** `L5_Pallav_Stage5_Architecture_Approval_Decision_2026-09-16.md`  
**Governance Lock:** `L5_Pallav_Stage5_Final_Lock_Decision_2026-09-16.md`

## 2. Purpose, Scope & Non-Goals
Defines the reusable secure technical boundary for physical devices, edge systems, telemetry, evidence feeds and controlled actuators while ensuring device observations/commands never become business decisions merely because hardware produced or executed them.

## 3. Upstream Applicability & Constraints
### 3.1 Canonical Upstream Reference Matrix
| Source | Version / Status | Exact Reference | What It Establishes | C03 Technical Use |
|---|---|---|---|---|
| L5 Coverage Audit Decision | Architecture Approved | Device / Edge / Physical-System Integration finding | L4/scenarios explicitly defer scanner/reader, RFID/NFC, biometric, ANPR, GPS, telemetry, IoT, CCTV/evidence and controller mechanisms; a reusable L5 owner is required and devices must not own business decisions. | Primary authority for C03-009 technical scope. |
| `ADR-L5-0016` | Architecture Approved | shared C03 connector pattern | Governs device/provider adapter isolation, correlation, credentials and failure containment. | Supplies reusable C03 integration mechanics. |
| `L4-G-106` | Draft | §10 Open Issues / Deferred Design | GPS tracking, telemetry, dispatch/API/persistence mechanisms are deliberately deferred from Trip business behavior. | C03 may supply telemetry/GPS integration while G04 retains Trip decisions/state. |
| `L4-G-107` | Draft | §10 Open Issues / Deferred Design | GPS/tracking/telemetry/routing technical mechanisms are deferred while route-deviation business semantics remain upstream. | C03 carries observations; Transport remains authoritative. |
| L2-G07 | Locked | future QR/NFC/GPS verification statement | Future device technologies integrate without redesign. | Requires adapter-based extensibility without transferring Security Operations authority. |

The Architecture Approved coverage decision establishes the technical owner. The cited Draft L4 examples are explicitly governance-authorized as supporting Stage-5 evidence; canonical relation state remains `Specified`, not `Validated`, until normal traceability source-status conditions are met.

### 3.2 Authoritative Constraints
- Device/edge observations are evidence/input; canonical domains determine attendance, access, trip, asset, examination, identity, disciplinary or safety outcomes.
- Device identity is not Person identity.
- Business commands originate from an authorized canonical owner; C03 only transports/executes the technical command.
- Offline replay/store-and-forward shall not create duplicate business effect.
- Raw biometric/location/video data movement shall be minimized and governed by applicable privacy/security owners.

## 4. Technical Ownership & Dependencies
**Owns:** device/edge adapters, protocol normalization, registered device identity binding, telemetry ingestion, bounded store-and-forward where required, actuator command dispatch/acknowledgement, device health and evidence references.  
**Consumes:** D01/D02/D03 identity/trust/authorization, C02 messaging, A08 background execution, E01 resilience, E05 observability and owner-domain command/decision contracts.  
**Must not own:** attendance, access authorization, Trip/route business state, asset ownership, biometric Person truth, examination outcome, disciplinary outcome or safety policy.

## 5. Solution Architecture
### 5.1 Elements
| Element | Responsibility | Boundary |
|---|---|---|
| Device/Edge Adapter | Encapsulates vendor/protocol connectivity and normalizes observations/commands. | No business decision authority. |
| Device Identity Resolver | Binds a registered non-human/device identity and applicable scope. | Device ID never becomes Person ID. |
| Telemetry Ingest Boundary | Validates technical context, correlation and duplicate characteristics before handoff. | Owner domain interprets business meaning. |
| Store-and-Forward Buffer | Retains bounded offline observations where the approved use case requires intermittent connectivity. | No new canonical business lifecycle. |
| Actuator Command Gateway | Sends an already-authorized command and returns acknowledgement/failure evidence. | C03 cannot authorize the substantive action. |
| Evidence Reference Adapter | Carries governed references/metadata for media/telemetry evidence. | Continuous media is not copied into ERP stores unless an explicit governed requirement exists. |

### 5.2 Observation Flow
```text
Registered device observation
→ authenticated/authorized edge boundary
→ protocol validation + normalization + correlation
→ optional governed buffering/deduplication
→ canonical owning workflow
→ owner makes business decision/state transition
→ E05 telemetry / governed evidence reference
```

### 5.3 Command Flow
```text
Canonical owner authorizes action
→ D02 policy enforcement / non-human authority binding
→ C03 Actuator Command Gateway
→ device/edge adapter
→ physical actuator
→ acknowledgement/failure evidence
→ canonical owner decides any business consequence
```

### 5.4 Failure Rules
Device loss/degradation shall affect only dependent capability under E01 semantics. Reconnection/offline replay preserves ordering/correlation only to the degree required by the use case and shall prevent duplicate effect. Failure acknowledgement never invents owner-domain recovery behavior.

## 6. Architecture Decisions
| ID | Decision | Position | Basis | ADR |
|---|---|---|---|---|
| AD-001 | Device integration model | Heterogeneous devices integrate through provider/device adapters and normalized internal contracts. | Approved coverage decision. | `ADR-L5-0016` |
| AD-002 | Decision authority | Devices produce observations/execute commands; canonical domains decide business meaning. | Approved coverage decision + ownership rules. | `ADR-L5-0016` ownership-preserving handoff. |
| AD-003 | Offline behavior | Bounded store-and-forward is available only where required, with duplicate/reconciliation controls. | Technical derivation; consumes A08/C02/E01. | `ADR-L5-0016`; `ADR-L5-0013`; `ADR-L5-0010` as applicable. |
| AD-004 | Actuators | Commands require authoritative owner decision, technical authorization and acknowledgement/failure evidence. | D02/D03 ownership + device boundary. | `ADR-L5-0009`; `ADR-L5-0016` |
| AD-005 | Media | Prefer governed evidence/metadata references over continuous-media replication absent explicit requirement. | Data minimization and corpus boundary. | `ADR-L5-0016` |

## 7. Security, Privacy & Operations
Device identities/certificates consume D-family controls. Sensitive biometric/location/video data is excluded from general telemetry and minimized at boundaries. E05 receives device/connector health, last contact, backlog and failure signals without becoming business audit truth.

## 8. Architecture Conformance Requirements
- `CR-001`: Device observations shall not directly establish canonical business outcomes.
- `CR-002`: Devices/edge nodes shall be authenticated and authorized before accepted exchange.
- `CR-003`: Offline replay/store-forward shall include duplicate/reconciliation controls preventing duplicate effect.
- `CR-004`: Actuator commands shall originate from an authorized canonical owner and return technical acknowledgement/failure evidence.
- `CR-005`: Continuous media shall not be copied into general ERP persistence without an explicit governed requirement.
- `CR-006`: Device/provider-specific protocols shall remain isolated behind adapters.
- `CR-007`: C03 technical health shall consume E01 semantics and remain distinct from business lifecycle state.

## 9. Open Decisions & Deferred Design
| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| OD-001 | Concrete device vendor/protocol profiles and numeric buffer limits. | Technology / Operations | No. | Decide only from concrete device/provider requirements and capacity evidence. |
| OD-002 | Promotion of Draft-L4 device examples from `Specified` to `Validated`. | Traceability | No for this Architecture Approved specification. | Revalidate after applicable L4 approval. |

## 10. Traceability Summary
**Approved purpose authority:** L5 Coverage Audit Decision — Device/Edge/Physical-System Integration finding. **Governance-authorized / Specified examples:** `L4-G-106` §10; `L4-G-107` §10; locked L2-G07 future-device integration constraint. **Peers:** C03-001, C02, A08, D01/D02/D03, E01/E05. **Applicable shared ADR:** `ADR-L5-0016`. **Blocking:** none.
