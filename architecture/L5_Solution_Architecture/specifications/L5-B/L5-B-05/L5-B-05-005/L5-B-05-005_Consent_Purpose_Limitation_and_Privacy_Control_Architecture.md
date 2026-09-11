# L5-B-05-005 — Consent, Purpose-Limitation & Privacy-Control Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-05-005 |
| Specification Title | Consent, Purpose-Limitation & Privacy-Control Architecture |
| Architectural Level | L5 |
| Architecture Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-05 — Data Classification, Retention & Privacy Architecture |
| Version | 1.0.0 |
| Status | Draft — Authoring |
| Author | Varad |
| Primary Responsibility | Technical enforcement of authoritative consent state/withdrawal, approved purpose, minimization and disclosure constraints; never canonical consent owner |

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical architecture for enforcing authoritative privacy controls across technical processing and disclosure paths. It ensures that consent state, withdrawal, approved purpose, minimization and disclosure constraints supplied by authoritative upstream sources are respected consistently across applicable technical channels.

The architecture is an enforcement mechanism. It does not create, own or redefine the canonical consent or business/privacy decision.

### 2.2 In Scope

- Resolution of authoritative consent state and withdrawal state where applicable.
- Resolution of approved processing purpose and applicable privacy constraints.
- Technical enforcement of purpose limitation before protected processing or disclosure.
- Technical minimization of information made available to a processing path.
- Disclosure-control enforcement across applicable internal and external technical boundaries.
- Propagation/preservation of privacy-control context across supported technical flows.
- Consistent enforcement across APIs, integrations, reporting, document access, notifications and client-access paths where applicable.
- Prevention of processing/disclosure when required privacy-control context is missing, invalid or contradictory.
- Technical evidence of privacy-control enforcement without unnecessary sensitive-value capture.
- Integration with B05-001/002/003/004, authorization, security, integration and audit/evidence architectures.

### 2.3 Explicit Non-Goals

This specification does not own or redefine:

- Canonical consent records, consent ownership or consent business lifecycle.
- Whether an individual or institution should provide consent.
- Legal/regulatory interpretation or institutional privacy policy.
- General classification/privacy baseline owned by L5-B-05-001.
- Generic sensitive-data masking owned by L5-B-05-002.
- Retention, archival, deletion and legal-hold policy owned by L5-B-05-003.
- Aadhaar-specific protection owned by L5-B-05-004.
- Authentication or authorization policy owned by L5-D-01/L5-D-02.
- Cryptographic/key/security mechanisms owned by L5-D-03.
- Audit/provenance/evidence architecture owned by L5-D-04.
- APIs, database schemas, protocols, infrastructure or technology/product selection.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

This specification is subordinate to locked L1 architecture and approved L2/L3/L4 behavior. It shall not redefine upstream business ownership, business rules, lifecycle states, canonical events, publishers, institutional authority or L3/L4 outcomes.

The 2026-09-09 L5 coverage decision identified an explicit gap for cross-channel technical enforcement of authoritative consent state, withdrawal, approved purpose, minimization and disclosure constraints and added B05-005 to close that gap. fileciteturn395file0L8-L18

### 3.2 Canonical Ownership Boundary

The originating business domain retains canonical ownership of consent and purpose facts. B05-005 consumes those authoritative facts and enforces them technically; it does not become the canonical consent owner. fileciteturn395file0L8-L18

### 3.3 Privacy-Control Model

The architecture treats consent, withdrawal, approved purpose, minimization and disclosure constraints as authoritative inputs. It shall not independently infer consent, invent an approved purpose, weaken a restriction or treat technical authorization as equivalent to privacy permission.

### 3.4 Cross-Channel Applicability

The canonical coverage decision states that B05-005 is intended to enforce authoritative privacy constraints through APIs, integrations, reporting, document access, notifications and client access where required. fileciteturn395file0L8-L18

This does not make B05-005 the owner of those channels; it provides the reusable privacy-control mechanism consumed by them.

### 3.5 L4 Traceability Status

Exact atomic L4 applicability has not been independently reconciled in this authoring draft. No L4 behavioral-obligation identifiers are invented here.

The canonical traceability relation identity is:

`(L4 BH ID, L5 Specification ID)`

A validated relation record shall additionally bind the applicable `L4 Version` and `L5 Version`. Exact L4 BH applicability, exact source version/status and validation status remain **OPEN / BLOCKING** until reconciled against the applicable current approved L4 corpus.

---

## 4. Technical Architecture

### 4.1 Privacy-Control Flow

```text
Authoritative Consent / Purpose / Privacy Context
                     ↓
              Context Resolution
                     ↓
              Scope Resolution
                     ↓
             Purpose Evaluation
                     ↓
          Minimization Evaluation
                     ↓
           Disclosure Evaluation
                     ↓
              Control Decision
                     ↓
        ┌────────────┴────────────┐
        ↓                         ↓
   Permit Constrained          Block / Restrict
   Processing/Disclosure      Processing/Disclosure
        ↓                         ↓
        └────────────┬────────────┘
                     ↓
               Evidence / Handoff
```

### 4.2 Authoritative Consent State

Where consent is applicable, technical processing shall resolve the authoritative current consent state and any authoritative withdrawal state before proceeding.

A technical component shall not manufacture consent from absence of a withdrawal, infer consent from authentication, or preserve an outdated consent state after a valid authoritative withdrawal becomes effective.

The canonical consent owner remains responsible for the business fact; B05-005 enforces its technical consequence.

### 4.3 Withdrawal Enforcement

When an authoritative withdrawal invalidates or restricts a previously permitted purpose, subsequent applicable technical processing/disclosure shall be blocked or constrained according to the authoritative privacy decision.

Technical enforcement shall prevent stale privacy context from silently continuing an operation where current authoritative context is required.

Withdrawal enforcement does not itself delete information or terminate business records; retention/deletion remains B05-003 territory.

### 4.4 Purpose Limitation

A technical processing path shall establish the approved purpose applicable to the requested use before releasing information for that purpose.

The existence of a technically valid identity, role or authorization does not establish an approved privacy purpose. Purpose approval remains authoritative upstream.

A request lacking an applicable approved purpose shall be constrained or rejected rather than silently treated as general-purpose access.

### 4.5 Data Minimization

Only information necessary and permitted for the authoritative purpose shall be made available to the consuming technical path.

Minimization shall not mutate or delete the canonical source record. It controls the representation/scope of information exposed to a particular processing purpose.

B05-002 remains responsible for sensitive-value masking/representation where masking is additionally required.

### 4.6 Disclosure Control

Before information crosses a disclosure boundary, the technical architecture shall evaluate applicable consent, purpose and disclosure constraints.

Disclosure may be blocked even where technical authorization exists if authoritative privacy constraints do not permit the disclosure.

Cross-boundary technical exchange architectures remain responsible for their exchange mechanics; B05-005 supplies the privacy-control constraint.

### 4.7 Privacy Context Propagation

Where a technical workflow spans multiple components, the applicable privacy-control context shall remain resolvable throughout the processing chain.

A downstream component shall not silently broaden purpose, disclosure scope or permitted data set because upstream privacy context was lost.

### 4.8 Scope Preservation

Tenant, Institution and other applicable canonical scope context shall remain associated with privacy-control evaluation.

Where Person context is relevant to an authoritative privacy decision, it shall be carried as a reference within the applicable Tenant-scoped Person context; this does not establish Person as an independent cross-Tenant scope or transfer Person ownership into B05-005.

### 4.9 Specialist Boundaries

| Concern | Canonical L5 Owner |
|---|---|
| Classification/privacy baseline | L5-B-05-001 |
| Sensitive-data masking/controlled representation | L5-B-05-002 |
| Retention, archival, deletion and legal hold | L5-B-05-003 |
| Aadhaar-specific protection | L5-B-05-004 |
| Consent, purpose limitation, minimization and disclosure | L5-B-05-005 |
| Authorization | L5-D-02 |
| Security/cryptography/trust | L5-D-03 |
| Audit/provenance/evidence | L5-D-04 |

The candidate corpus explicitly assigns B05-005 the technical enforcement responsibility while retaining canonical consent ownership upstream. fileciteturn396file1L34-L50

### 4.10 No Privacy-Policy Fabrication

This architecture shall not define consent validity periods, legal bases, regulatory interpretations, disclosure categories or institutional privacy policy unless supplied by an authoritative source.

---

## 5. Dependencies / Integration

### 5.1 Upstream

- Locked L1 Business Domain Architecture.
- Applicable L2 workflows.
- Approved/locked L3 scenarios.
- Current approved L4 system behavior specifications.
- Authoritative consent/purpose/privacy decisions.
- L5 governance and candidate-corpus decisions.

### 5.2 Peer L5 Dependencies

- L5-B-05-001 — classification/privacy baseline.
- L5-B-05-002 — sensitive-data handling.
- L5-B-05-003 — retention/deletion/legal hold.
- L5-B-05-004 — Aadhaar-specific protection.
- L5-D-01/L5-D-02 — identity and authorization.
- L5-D-03 — security/trust.
- L5-D-04 — audit/evidence.
- Applicable L5-C integration/exchange architectures.
- Applicable B04/B06 client/reporting/document architectures.

Exact peer version alignment remains an authoring-stage validation item.

### 5.3 Integration Rule

Technical authorization is necessary only where required by the applicable security architecture; it is not sufficient to establish privacy permission. Conversely, privacy permission does not bypass security or authorization controls.

---

## 6. Technical Behavior / Failure Concerns

### TB-001 — Consent Context Resolution

Where consent is applicable, the current authoritative consent state shall be resolved before protected processing/disclosure.

### TB-002 — Withdrawal Enforcement

Authoritative withdrawal shall prevent continued processing/disclosure that is no longer permitted under the applicable privacy decision.

### TB-003 — Purpose Validation

Processing shall be associated with an approved purpose before applicable information is released.

### TB-004 — Purpose Non-Expansion

A technical consumer shall not broaden an approved purpose or reuse data for an unrelated purpose merely because it possesses access.

### TB-005 — Minimization

Only information permitted and necessary for the authoritative purpose shall be exposed to the applicable processing path.

### TB-006 — Disclosure Control

Cross-boundary disclosure shall be constrained or blocked when authoritative privacy constraints do not permit it.

### TB-007 — Privacy Context Continuity

Privacy-control context shall remain resolvable across multi-component processing flows.

### TB-008 — Scope Preservation

Privacy decisions shall not be reused across incompatible Tenant, Institution or other canonical scopes without authoritative support.

### TB-009 — No Policy Inference

Missing privacy context shall not be interpreted as permission.

### TB-010 — Failure Containment

Invalid, stale or contradictory privacy-control context shall not result in unrestricted processing or disclosure.

---

## 7. Security / Trust / Evidence

### 7.1 Security

- Privacy enforcement shall operate within applicable identity and authorization boundaries.
- Sensitive values shall remain protected under B05-002 and D03.
- Privacy controls shall not expose protected context unnecessarily.

### 7.2 Evidence

The architecture shall support evidence of:

- privacy context evaluated;
- purpose evaluated;
- minimization/disclosure control applied;
- withdrawal-driven restriction;
- blocked or constrained processing/disclosure; and
- unresolved or stale context.

Evidence shall avoid unnecessary sensitive-value or raw consent-content duplication.

---

## 8. Conformance

A conforming implementation shall demonstrate that:

1. Authoritative consent state is resolved where consent is applicable.
2. Authoritative withdrawal is respected.
3. Approved purpose is evaluated before applicable processing/disclosure.
4. Technical authorization is not treated as privacy permission.
5. Purpose cannot be silently broadened downstream.
6. Data exposure is minimized for the applicable purpose.
7. Disclosure constraints are enforced across applicable boundaries.
8. Privacy context remains resolvable across multi-component flows.
9. Scope is preserved.
10. Missing/stale/contradictory privacy context cannot silently permit unrestricted processing.
11. Retention/deletion remains owned by B05-003.
12. Sensitive-data representation remains owned by B05-002.
13. Security and authorization remain owned by their canonical L5 owners.
14. The specification does not invent privacy/legal policy.
15. Material enforcement outcomes are observable.
16. The architecture remains implementation-independent.

---

## 9. Traceability

| Traceability Item | Status |
|---|---|
| Canonical specification ID verified | PASS |
| Canonical title verified | PASS |
| Canonical major-area placement verified | PASS |
| Responsibility verified against 105-spec register | PASS |
| Canonical consent ownership boundary verified | PASS |
| Cross-channel enforcement responsibility verified | PASS |
| Upstream L1/L2/L3/L4 applicability | OPEN / BLOCKING |
| Exact atomic L4 BH mapping | OPEN / BLOCKING |
| Exact L4 source version/status | OPEN / BLOCKING |
| Peer L5 version/dependency validation | OPEN / BLOCKING |

### Canonical Traceability Form

The canonical relation identity is:

`(L4 BH ID, L5 Specification ID)`

A validated relation record shall additionally bind the applicable `L4 Version` and `L5 Version`.

Document-level statements are insufficient for final traceability.

## 10. Open Issues / Deferred Design

- **OB-001 — Exact L4 BH Applicability:** Reconcile exact L4 behavioral obligations against the current approved L4 corpus before promotion.
- **OB-002 — L4 Version/Status:** Record exact source versions and current approval/lock status for accepted atomic relations.
- **OB-003 — Canonical Consent Sources:** Identify applicable originating business-domain sources of consent and purpose facts without transferring canonical ownership.
- **OB-004 — Privacy Policy Inputs:** Confirm authoritative purpose/minimization/disclosure constraints without inventing policy.
- **OB-005 — Peer Dependency Validation:** Validate exact versions and boundaries of B05-001/002/003/004, D01/D02/D03/D04 and applicable L5-C/B04/B06 architectures.
- **OB-006 — Current L4 Baseline Reconciliation:** Where live L4 documents differ in status from the governance baseline, downstream traceability shall use the subsequently approved/current L4 state and shall not treat Draft wording as immutable.

**Promotion Gate:** This specification shall remain `Draft — Authoring` until exact L4 applicability, upstream status and peer dependency checks are independently validated.
