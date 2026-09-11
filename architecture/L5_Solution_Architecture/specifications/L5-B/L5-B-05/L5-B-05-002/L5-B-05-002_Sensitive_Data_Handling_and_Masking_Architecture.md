# L5-B-05-002 — Sensitive Data Handling & Masking Architecture

## 1. Specification Metadata

| Field | Value |
|---|---|
| Specification ID | L5-B-05-002 |
| Specification Title | Sensitive Data Handling & Masking Architecture |
| Architectural Level | L5 |
| Architecture Family | L5-B — Data & Information Architecture |
| Major Area | L5-B-05 — Data Classification, Retention & Privacy Architecture |
| Version | 1.0.0 |
| Status | Draft — Authoring |
| Author | Varad |
| Primary Responsibility | Technical masking, controlled display/export and handling of classified sensitive information |

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical architecture for handling sensitive or classified information while preventing unnecessary exposure of protected values. It provides controlled masking, redaction, representation and disclosure mechanisms derived from authoritative upstream classification, privacy and authorization context.

The architecture protects information during technical access, display, transformation, export, exchange and processing. It does not determine the business meaning of the information or independently decide whether disclosure is legally, institutionally or business-authorized.

### 2.2 In Scope

- Technical identification and consumption of authoritative sensitive/classification context.
- Controlled representations of sensitive information for authorized technical consumers.
- Masking, redaction and restricted-value presentation mechanisms.
- Consistent sensitive-data handling across supported technical processing paths.
- Preservation of Tenant, Institution and applicable scope context.
- Protection of sensitive values during transformation, projection, export and exchange.
- Prevention of accidental disclosure when required handling context is missing or contradictory.
- Technical evidence of applied masking/handling controls without unnecessarily exposing protected values.
- Integration with B05-001 baseline classification/privacy controls, authorization, security and audit/evidence architecture.

### 2.3 Explicit Non-Goals

This specification does not own or redefine:

- Canonical business entities, records or source-of-truth ownership.
- Business classification meaning or institutional privacy policy.
- Consent ownership, consent lifecycle or purpose approval.
- Retention periods, archival policy, deletion policy or legal-hold decisions.
- Aadhaar-specific protection policy or regulatory status.
- Authentication, authorization policy, cryptographic primitives, key management or machine credentials.
- Database schemas, APIs, protocols, infrastructure or technology/product selection.
- General classification/privacy baseline owned by L5-B-05-001.
- Retention/archival/deletion/legal-hold execution owned by L5-B-05-003.
- Aadhaar-specific controls owned by L5-B-05-004.
- Consent, purpose-limitation and privacy-control enforcement owned by L5-B-05-005.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

This specification is subordinate to locked L1 architecture and approved L2/L3/L4 behavior. It shall not redefine upstream business ownership, business rules, lifecycle states, canonical events, publishers, institutional authority or L3/L4 outcomes.

The approved L5 candidate corpus defines B05-002 as the technical owner of masking, controlled display/export and handling of classified sensitive information. fileciteturn385file2L75-L83

### 3.2 Ownership Preservation

The architecture consumes authoritative classification, privacy and authorization context. It shall not invent a sensitivity level, disclosure approval, consent state, business exception or legal interpretation.

Masking is a technical representation/control decision derived from supplied authoritative constraints; it does not transfer ownership of the underlying information away from its canonical domain.

### 3.3 Dependency Boundaries

Primary dependencies include:

- L5-B-05-001 for common classification/privacy context and control baseline.
- L5-D-01/L5-D-02 for identity and authorization boundaries.
- L5-D-03 for security/trust and cryptographic protection boundaries.
- L5-D-04 for audit, provenance and evidence.
- L5-B-02 for persistence-related handling boundaries.
- L5-C-03-008 for governed bulk/file exchange where applicable.
- L5-C integration architectures where sensitive information crosses external boundaries.
- L5-B-05-004 for Aadhaar-specific protection.
- L5-B-05-005 for consent, purpose limitation, minimization and disclosure constraints.

These dependencies do not transfer canonical ownership.

### 3.3 L4 Traceability Status

Exact atomic L4 applicability has not been independently reconciled in this authoring draft. No L4 behavioral-obligation identifiers are invented here.

The canonical traceability relation identity is:

`(L4 BH ID, L5 Specification ID)`

A validated relation record shall additionally bind the applicable `L4 Version` and `L5 Version`. Exact L4 BH applicability, exact source version/status and validation status remain **OPEN / BLOCKING** until reconciled against the applicable current approved L4 corpus.

---

## 4. Technical Architecture

### 4.1 Sensitive-Handling Control Flow

```text
Authoritative Classification / Privacy / Authorization Context
                         ↓
                  Context Resolution
                         ↓
                 Scope Verification
                         ↓
              Handling Requirement Check
                         ↓
             Representation Selection
                         ↓
        ┌────────────────┴────────────────┐
        ↓                                 ↓
   Controlled Value                 Masked / Redacted
        ↓                                 ↓
  Authorized Use                 Restricted Exposure
        └────────────────┬────────────────┘
                         ↓
                 Evidence of Control
```

The architecture is enforcement-oriented. It does not become the authority that establishes the underlying business or privacy decision.

### 4.2 Classification-Driven Handling

Sensitive handling shall consume authoritative classification/context supplied through the applicable upstream or B05-001 control boundary.

Where required sensitivity context is unavailable, ambiguous or contradictory, the architecture shall not silently select an unrestricted representation. The affected operation shall be rejected, constrained or marked unresolved according to the applicable upstream behavior.

### 4.3 Controlled Representation

A technical consumer shall receive only the representation permitted by the supplied control context. Representations may preserve enough information for an authorized technical or business purpose while preventing unnecessary exposure of the protected value.

The architecture shall distinguish the protected source value from any masked, redacted, abbreviated or otherwise restricted representation. A derived representation shall not become a new canonical business fact.

### 4.4 Masking and Redaction

Masking/redaction controls shall be deterministic with respect to the supplied control context and shall not expose protected portions merely because another technical component can access the underlying source value.

The architecture shall prevent uncontrolled unmasking or bypass of a required restriction. Any privileged technical path requiring access to the underlying value remains subject to the applicable authorization and security architecture.

No specific masking algorithm, character pattern, cryptographic primitive or product is mandated by this specification.

### 4.5 Display, Export and Exchange Boundaries

Sensitive information shall be evaluated before being exposed through:

- user-facing or operational display surfaces;
- generated or projected representations;
- exports and file exchange;
- synchronous or asynchronous technical exchange;
- analytics/reporting projections where sensitive information is included; or
- downstream technical processing.

This architecture supplies the sensitive-data handling mechanism. The consuming architecture remains responsible for its own business and interaction semantics.

### 4.6 Scope Preservation

Tenant, Institution and other authoritative scope context shall remain associated with sensitive information throughout technical handling.

A technical component shall not broaden scope or combine sensitive information across scopes merely because the data is technically reachable. Missing or contradictory scope context shall prevent uncontrolled exposure.

### 4.7 Specialist Boundaries

| Concern | Canonical L5 Owner |
|---|---|
| Classification/privacy baseline | L5-B-05-001 |
| Sensitive-data masking and controlled representation | L5-B-05-002 |
| Retention, archival, deletion and legal hold | L5-B-05-003 |
| Aadhaar-specific data protection | L5-B-05-004 |
| Consent, purpose limitation, minimization and disclosure enforcement | L5-B-05-005 |
| Authorization policy and enforcement | L5-D-02 |
| Security/trust/cryptographic protection | L5-D-03 |
| Audit/provenance/evidence architecture | L5-D-04 |

The approved candidate corpus explicitly separates these technical responsibilities. fileciteturn384file1L24-L44

### 4.8 Canonical Value Preservation

Masking or redaction shall never overwrite, mutate or replace the canonical source value solely for presentation or restricted-use purposes. Any controlled representation remains derived from the authoritative source and is not a competing canonical record.

---

## 5. Dependencies / Integration

### 5.1 Upstream

- Locked L1 Business Domain Architecture.
- Applicable L2 workflows.
- Approved/locked L3 scenarios.
- Current approved L4 system behavior specifications.
- L5 governance and candidate-corpus decisions.

### 5.2 Peer L5 Dependencies

- L5-B-05-001 — Data Classification, Retention & Privacy Architecture Baseline.
- L5-B-05-004 — Aadhaar Data Protection Architecture where applicable.
- L5-B-05-005 — Consent, Purpose-Limitation & Privacy-Control Architecture where applicable.
- L5-D-01/L5-D-02 — Identity and authorization.
- L5-D-03 — Security, cryptography and trust.
- L5-D-04 — Audit and evidence.
- L5-B-02 — Persistence.
- Applicable L5-C exchange architectures.

Exact peer version alignment remains an authoring-stage validation item.

### 5.3 Integration Rule

No technical consumer may use B05-002 as justification to bypass an authoritative authorization, consent, purpose or disclosure decision. Conversely, possession of an authorized source value does not imply permission to expose that value without the applicable handling controls.

---

## 6. Technical Behavior / Failure Concerns

### TB-001 — Required Context Availability

The technical boundary shall verify required classification/privacy/authorization context before sensitive handling.

### TB-002 — No Silent Downgrade

A sensitive value shall not be exposed in a less restrictive representation by silently discarding or downgrading authoritative handling constraints.

### TB-003 — Controlled Representation

Where restrictions apply, the technical consumer shall receive the representation permitted by the supplied control context.

### TB-004 — Unmasking Protection

Required masking/redaction shall not be bypassed by an alternate technical path that lacks the applicable authority.

### TB-005 — Scope Preservation

Sensitive-data handling shall preserve applicable Tenant, Institution and other authoritative scope context.

### TB-006 — Export/Exchange Protection

Sensitive values shall remain subject to applicable handling constraints when exported, exchanged, projected or transferred between technical components.

### TB-007 — Canonical Value Preservation

Masking, redaction or restricted representation shall not replace the authoritative source value with a derived representation.

### TB-008 — Failure Containment

Missing, contradictory or invalid handling context shall not silently result in unrestricted exposure.

### TB-009 — Evidence Preservation

Material sensitive-data handling decisions, failures and control applications shall remain technically observable without unnecessarily recording protected values.

### TB-010 — Cross-Path Consistency

Equivalent technical handling paths shall consume the same authoritative control context rather than independently deriving conflicting masking or exposure decisions.

---

## 7. Security / Trust / Evidence

### 7.1 Security

Sensitive-data handling shall remain subordinate to the canonical identity, authorization, security and privacy architectures.

### 7.2 Evidence

Evidence shall record control application and outcome without unnecessary capture of protected source values.

## 8. Conformance

A conforming implementation shall demonstrate that:

- **CR-001:** Required classification/privacy/authorization context is resolved before controlled handling.
- **CR-002:** Sensitive values are not silently downgraded to less restrictive representations.
- **CR-003:** Masked/redacted representations remain distinguishable from canonical source values.
- **CR-004:** Unmasking is constrained by applicable authorization/security context.
- **CR-005:** Tenant/Institution scope is preserved.
- **CR-006:** Sensitive handling remains consistent across applicable technical paths.
- **CR-007:** Material control outcomes remain observable without unnecessary protected-value replication.
- **CR-008:** The architecture does not acquire canonical ownership of the source information or privacy decision.

## 9. Traceability

| Traceability Item | Status |
|---|---|
| Canonical L5 identity | Verified |
| Canonical responsibility | Verified |
| Exact atomic L4 BH applicability | Open / Blocking |
| Exact L4 source version/status | Open / Blocking |
| Validated relation version binding | Required before promotion |
| Higher-level ownership boundary | Preserved |

## 10. Open Issues / Deferred Design

- **OD-001 — Exact L4 BH Applicability:** Blocking until applicable approved L4 behaviors are identified.
- **OD-002 — L4/L5 Version Binding:** Blocking until each validated relation records the L4 and L5 versions.
- **OD-003 — Specialist B05 Alignment:** Reconcile specialist boundaries during promotion.
- **OD-004 — Technical Control Placement:** Resolve final enforcement placement where downstream L5 owners specialize a control.

**Current State:** Draft — Authoring
