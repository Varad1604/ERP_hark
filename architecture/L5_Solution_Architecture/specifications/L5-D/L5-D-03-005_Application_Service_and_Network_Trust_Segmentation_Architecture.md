# L5-D-03-005 — Application, Service & Network Trust Segmentation Architecture

## 1. Specification Metadata

**Specification ID:** `L5-D-03-005`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-D — Identity, Security & Trust Architecture  
**Major Area:** L5-D-03 — Security, Cryptography, Secrets & Key Management Architecture  
**Canonical Technical Responsibility:** Define reusable technical architecture for logical trust isolation among applications, services, computational components and network communication boundaries, including trust-zone assignment, restricted connectivity, boundary enforcement, compromise containment and context-preserving cross-boundary interaction, without owning business authorization policy, cryptographic/key-management internals, secrets/certificates, business rules or business authority.  
**Technical Architecture Owner:** L5-D-03-005 — Application, Service & Network Trust Segmentation Architecture  
**Assigned Author:** Varad  
**Version:** `1.3.0`  
**Status:** Locked  
**Last Updated:** `2026-09-23`  
**Primary Dependencies:** `L5-D-03-001`, `L5-A-03`, `L5-A-04`, `L5-E-04`.  
**Collaborates With:** `L5-D-03-002`, `L5-D-03-003`, `L5-D-03-004`, `L5-D-01`, `L5-D-02`, `L5-D-04`, applicable integration/data/runtime architecture.  
**Applicable ADRs:** `ADR-L5-0001` — Canonical Trust Boundary & Segmentation Pattern (`Architecture Approved`, 2026-09-11).; `ADR-L5-0027`; `ADR-L5-0040`

---

## 2. Purpose, Scope & Non-Goals

### Purpose

Establish logical trust segmentation so components communicate only through explicitly permitted trust relationships and compromise of one component does not automatically establish trust in another.

### In Scope

- logical trust-zone assignment;
- default-deny / explicit-allow material cross-zone interaction;
- boundary admission conditions;
- restricted east-west technical communication;
- Tenant/Institution context preservation;
- privileged/restricted component isolation;
- compromise containment;
- interaction with identity, authorization, cryptography, credentials and runtime/deployment architecture;
- evidence of material allow/deny/containment decisions.

### Out of Scope

- business authorization policy — D02;
- authentication/human identity lifecycle — D01;
- cryptographic protection execution — D03-002;
- key custody — D03-003;
- secrets/certificates/machine credential lifecycle — D03-004;
- business rules/lifecycle/events/publishers/authority;
- physical topology, products, ports, addresses, protocols or quantitative network-security targets unless separately governed.

---

## 3. Authority, Constraints & Dependency Position

### 3.1 Authority Basis

| Source | Use |
|---|---|
| L5 Candidate Corpus Register 105 | Establishes D03-005 responsibility for logical trust isolation/restricted connectivity. |
| L5 Architecture Family + Naming Registers | Establish canonical D03 major-area identity. |
| Locked dependency timeline | Establishes D03 as Stage-1 security foundation. |
| `ADR-L5-0001` | Establishes explicit logical trust boundaries, default-deny protected crossing, contextual/non-transitive trust and logical segmentation without mandatory physical separation. |
| `L5-D-03-001` | Supplies parent trust/security baseline. |
| A03/A04/E04 | Supply module/dependency, Tenant/Institution isolation and runtime/deployment boundaries respectively. |

### 3.2 Architectural Constraints

1. Material trust boundaries have explicit purpose and source of authority.
2. Components are not trusted merely because they are internal or co-located.
3. Cross-zone communication is denied by default unless explicitly permitted by architecture.
4. Permitted crossings preserve required Tenant/Institution and security context.
5. Segmentation does not replace identity, authorization, cryptography or credential controls.
6. Trust is contextual and non-transitive.
7. Privileged/restricted technical components have constrained interaction paths.
8. Logical segmentation is required; physical server/network separation is not required unless separately governed.
9. No unsupported protocol/port/product/topology is canonicalized here.

### 3.3 Dependency Position

D03-002/003/004 are peer security-control contracts. Their approval state does not form a mutual finalization gate for D03-005 because ownership boundaries are already governed by the candidate corpus and approved ADRs.

A03/A04/E04 remain material peer dependencies for module structure, scope isolation and physical realization respectively; D03-005 shall not redefine them.

---

## 4. Technical Architecture

### 4.1 Trust Segmentation Model

```text
External / Untrusted Zone
        ↓ governed admission
Application Interaction Zone
        ↓ explicit permitted dependency
Privileged / Restricted Execution Zone
        ↓ separately governed access
Protected Resource / Data Zone

External integrations remain separate trust domains.
```

Trust zones are logical architecture constructs and do not mandate physical deployment topology.

### 4.2 Logical Zone Classes

| Zone | Purpose |
|---|---|
| External / Untrusted | External actors/systems/inputs with no inherited internal trust. |
| Application Interaction | Application-facing technical components coordinating approved behavior. |
| Privileged Execution | Components requiring elevated technical capability. |
| Restricted Resource / Data | Components/resources needing stronger isolation. |
| Integration Boundary | Controlled communication with external/separately trusted systems. |

Zone use is justified by responsibility and risk; not every component requires every zone type.

### 4.3 Boundary Admission

A protected cross-zone interaction establishes, as applicable:

- source and destination trust zone;
- permitted interaction/dependency class;
- technical identity;
- authorization outcome where policy-controlled;
- required credential validity;
- required cryptographic protection;
- Tenant/Institution context;
- rejection/containment behavior if a mandatory prerequisite fails.

Segmentation admission is not a business permission decision.

### 4.4 Explicit Allow Model

A route’s technical existence does not establish permission to communicate. Material cross-zone interactions must be explicitly permitted by the architecture.

The segmentation model distinguishes permitted, denied, unavailable and stronger-control-required interactions.

### 4.5 Context Preservation

A permitted crossing preserves authoritative scope. The receiving component shall not silently substitute, broaden or discard Tenant/Institution context.

Crossing into a privileged zone does not elevate the caller’s authority.

### 4.6 Non-Transitive Trust

Successful access from component A to B does not imply permission from B to C or from A to C. Every material subsequent trust boundary remains independently governed.

### 4.7 Privileged / Restricted Component Isolation

Components with elevated capability—such as computational engines, background workers, protected data-processing components, signing support or external connectors—shall expose only the interaction paths required by their canonical responsibility.

Technical privilege of the destination does not become ambient caller privilege.

### 4.8 Compromise Containment

A compromised component or trust relationship shall be containable by restricting affected crossings while preserving unrelated trust boundaries.

Containment supports:

1. identification of affected component/zone;
2. restriction/suspension of affected crossings;
3. preservation of unaffected boundaries;
4. continued unaffected operation where authoritative behavior allows;
5. evidence of enforcement decisions;
6. controlled restoration after trust prerequisites are re-established.

### 4.9 Physical Realization Boundary

E04 owns runtime/deployment topology. D03-005 defines logical isolation requirements that E04 must realize.

Physical co-location is permitted only when logical trust boundaries, protected communication and admission controls remain intact. Physical separation may be selected later where justified, but it is not the default architectural requirement.

### 4.10 Security-Control Collaboration

| Capability | Canonical Owner | D03-005 Relationship |
|---|---|---|
| Identity/authentication | D01 | Consumed at admission where required. |
| Authorization | D02 | Consumed; segmentation does not create policy. |
| Cryptographic protection | D03-002 | Consumed for protected crossings. |
| Key lifecycle/custody | D03-003 | Not owned by segmentation. |
| Machine credentials/certificates | D03-004 | Consumed; lifecycle remains D03-004-owned. |
| Audit/evidence | D04 | Receives material boundary evidence. |
| Runtime/deployment | E04 | Realizes logical segmentation. |

### 4.11 Failure Model

| Failure | Required Outcome |
|---|---|
| Interaction not explicitly permitted | Reject. |
| Required identity/auth unavailable | Reject protected crossing. |
| Authorization denied/unavailable where required | Reject operation. |
| Credential invalid | Reject affected crossing. |
| Required cryptographic protection unavailable | Reject protected crossing. |
| Tenant/Institution context missing/conflicting | Fail closed for affected interaction. |
| Component compromised | Restrict affected trust relationship and prevent propagation. |
| Runtime boundary unavailable | Do not bypass segmentation through alternate unrestricted path. |

### 4.12 Architecture Decisions

| ID | Decision | Governance |
|---|---|---|
| AD-001 | Material application/service/network trust boundaries are logical and explicit. | `ADR-L5-0001`. |
| AD-002 | Protected cross-zone interaction is default-deny / explicit-allow. | `ADR-L5-0001`. |
| AD-003 | Trust is contextual and non-transitive. | `ADR-L5-0001`. |
| AD-004 | Physical co-location shall not erase logical trust isolation. | `ADR-L5-0001`. |
| AD-005 | Privileged components expose only necessary interaction paths. | D03-005 derivation + ADR-0001. |
| AD-006 | D03-002/003/004 remain peer security-control owners; D03-005 does not absorb them. | Candidate corpus + ADRs. |
| AD-007 | E04 owns physical realization/topology. | Ownership boundary. |
| AD-008 | Protocol/port/product/topology details require separate governance where material. | ADR methodology. |

---

## 5. Technical Behaviors

**TB-001:** Unapproved material cross-zone interaction is denied.  
**TB-002:** Required execution context is preserved across permitted crossings.  
**TB-003:** Trust does not transit automatically across components.  
**TB-004:** Compromised relationships can be contained without collapsing unrelated zones.  
**TB-005:** More-privileged destinations do not expand caller authority.  
**TB-006:** Required cryptographic protection cannot be silently bypassed.  
**TB-007:** Restricted components expose only necessary paths.  
**TB-008:** Failure/recovery does not remove segmentation controls.  
**TB-009:** Material allow/deny/contain/restore outcomes expose evidence to D04.  
**TB-010:** Ambiguous identity/zone/scope context fails closed.

---

## 6. Security / Trust / Evidence

The architecture reduces blast radius through explicit trust zones, non-transitive trust, restricted connectivity and context-preserving admission.

Evidence may include source/destination technical identity, zone, interaction class, scope, decision, failure class, containment/restoration action and correlation information. Evidence shall not disclose protected credentials/key material.

---

## 7. Conformance

| ID | Requirement |
|---|---|
| CR-001 | Material trust boundaries are explicitly identified. |
| CR-002 | Zone assignment has architectural purpose. |
| CR-003 | Material cross-zone communication is explicit-allow. |
| CR-004 | Unapproved crossing is rejected. |
| CR-005 | Tenant/Institution context is preserved where applicable. |
| CR-006 | Trust is non-transitive. |
| CR-007 | Privileged components have restricted interaction paths. |
| CR-008 | Compromise containment preserves unrelated trust zones where possible. |
| CR-009 | Segmentation does not replace identity/authorization. |
| CR-010 | Required cryptographic protection cannot silently bypass. |
| CR-011 | Evidence is available without credential/key disclosure. |
| CR-012 | Logical segmentation does not require physical server separation. |
| CR-013 | No unsupported port/protocol/product/topology is canonicalized. |
| CR-014 | The implementation conforms to `ADR-L5-0001`. |

---

## 8. Traceability

D03-005 has defensible technical purpose through the Architecture Approved candidate corpus and `ADR-L5-0001`.

Where an authoritative L4 BH materially requires D03-005 trust-segmentation contribution, the canonical relation is:

```text
(L4 BH ID, L5-D-03-005)
```

Only materially supported direct relations shall be entered into the canonical traceability store; no BH is invented in this remediation.

| Item | State |
|---|---|
| Canonical identity/responsibility | Verified |
| Canonical major-area naming | Verified and synchronized |
| Applicable ADR | Verified — `ADR-L5-0001` Architecture Approved |
| D03-001 parent baseline | Verified |
| D03 peer ownership | Stable responsibility contracts; no mutual finalization gate |
| Physical deployment ownership | Preserved with E04 |
| Direct L4 BH relations | Selective mapping pending validated evidence |

---

## 9. Open Issues / Deferred Design

| ID | Issue | Blocking? |
|---|---|---|
| OD-001 | Add only defensible direct L4 BH relations when validated evidence exists. | No for Review validity; required before relation validation. |
| OD-002 | Reconcile material A03/A04/E04 changes that alter module, scope or deployment realization. | Conditional. |
| OD-003 | **Partially resolved:** authenticated encrypted cross-service transport is required; TLS 1.3 is preferred; mTLS is trust-profile driven; no service mesh is mandatory initially. Exact firewall/port/topology and any later mesh product remain deployment/security-profile decisions. | `ADR-L5-0040`. |

---

**Current Status:** Review — Remediated and revalidated in Batch 4; lifecycle promotion requires explicit governance authorization.

## 2026-09-22 Backend Readiness Group 3 Synchronization

**Applicable locked decision:** `ADR-L5-0040`  
Protected cross-process/service communication uses authenticated encrypted transport; TLS 1.3 is preferred; mTLS is profile-driven where workload identity at transport boundary is required; no service mesh is mandatory initially. OpenBao remains production-secret authority; infrastructure encryption-at-rest is baseline with classification-driven field protection; HSM/KMS custody is profile-driven; no universal rotation interval is invented.


## 2026-09-22 Full-Corpus Post-Decision Synchronization

Lifecycle status restored to `Review` in accordance with the locked 2026-09-16 full-corpus Draft→Review promotion decision. OpenBao/rustls and the Group-3 trust/crypto/secrets profile are inherited where applicable; this synchronization does not promote the specification beyond Review.


**Lifecycle restoration authority:** `L5_Full_Corpus_Draft_to_Review_Promotion_Decision_2026-09-16.md` — restored to `Review` during the 2026-09-22 full-corpus post-decision audit; no Architecture Approved/Locked promotion is implied.


## 2026-09-22 Production Validation Batch 4

D03 local hard gates remain PASS under `L5_D03_L4_Traceability_Extraction_and_Promotion_Gate_2026-09-11.md`. Later Locked ADRs `ADR-L5-0027` and `ADR-L5-0040` strengthen implementation readiness without transferring security ownership. This specification is **Architecture-Approval eligible**, but remains `Review` until a distinct approval decision is issued.


## 2026-09-23 R7 Lifecycle Promotion Decision

R7 issues the distinct lifecycle decision previously required by the D03 promotion gate. The specification had already passed its local HF-01…HF-15 gate and was Architecture-Approval eligible; R7 therefore promotes it from Review to Architecture Approved without changing canonical security ownership, trust boundaries, cryptographic responsibility, secret/key ownership, or business semantics. Final corpus lock remains a separate governance action.
