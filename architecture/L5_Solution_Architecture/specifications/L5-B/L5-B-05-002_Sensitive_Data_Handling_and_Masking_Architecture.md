# L5-B-05-002 — Sensitive Data Handling & Masking Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-002  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical masking, controlled display/export and handling of classified sensitive information  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.1  
**Status:** Review  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001; L5-B-02; L5-D-02; L5-D-03; L5-D-04  
**Referenced By:** L5-B-05-004, L5-B-04, L5-B-06, L5-C-01, L5-C-03, L5-G-01  
**Applicable ADRs:** ADR-L5-0001; ADR-L5-0004; ADR-L5-0005  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines reusable technical masking, redaction and controlled-disclosure behavior for sensitive information. It consumes authoritative classification/handling directives from L5-B-05-001 and authorization decisions from L5-D-02. It never creates roles, permission codes, approval authority or business entitlement semantics.

### 2.2 In Scope

- Dynamic masking/redaction at service, projection, export and connector egress boundaries.
- Technical transformation mechanisms such as partial masking, suppression, pseudonymous representation and approved tokenized projection.
- Preservation of authoritative source values while producing constrained projections.
- Consumption of D02 authorization decisions for any unmasked disclosure.
- Minimized audit-evidence handoff to L5-D-04 for sensitive-data disclosure decisions.

### 2.3 Explicit Non-Goals

This specification does not own:

- Business classification or privacy policy.
- User roles, permission identifiers, privilege elevation, approval workflow or SoD rules.
- Canonical entity schemas or physical persistence design.
- Cryptographic algorithms, keys or token-signing mechanisms.
- Aadhaar-specific protection requirements owned by L5-B-05-004.
- UI presentation design.

---

## 3. Architectural Boundaries

### 3.1 Authorization Ownership

L5-D-02 is the sole owner of authorization policy and decisions. L5-B-05-002 receives an authoritative authorization outcome or decision reference and applies the masking consequence associated with the governing handling directive.

This specification shall not define names such as `DATA_UNMASK_PRIVILEGED`, `PrivilegedUnmaskPermission`, role elevation, administrative approval or equivalent authorization constructs.

### 3.2 Canonical Value Preservation

Masking is a projection concern unless an authoritative upstream workflow explicitly requires irreversible transformation of the canonical value. Such upstream-directed transformation shall be executed through the owning domain and persistence architecture; L5-B-05-002 shall not invent a mutation lifecycle.

---

## 4. Technical Architecture

### 4.1 Sensitive Data Projection Gate

```text
Authoritative domain value / projection
                |
                v
Resolve data-control context (L5-B-05-001)
                |
                v
Consume authorization decision (L5-D-02)
                |
        +-------+-------+
        |               |
 disclosure allowed   disclosure constrained / unresolved
        |               |
        v               v
approved projection   masking / redaction / suppression
        |               |
        +-------+-------+
                v
        Egress + evidence handoff
```

### 4.2 Transformation Categories

The following are technical mechanism classes, not business policy:

- **Partial masking:** reveal only the portion permitted by the authoritative handling directive.
- **Suppression/null projection:** omit or replace the value when disclosure is not permitted.
- **Pseudonymous/tokenized projection:** return an approved surrogate representation where the governing contract requires correlation without cleartext disclosure.
- **Irreversible redaction:** only where an authoritative upstream rule requires non-recoverable projection for a defined target channel.

No default field pattern, visible-character count or token format is established by this specification.

### 4.3 Architecture Position — Decision Consumption, Not Authorization Creation

The masking gate shall accept an authoritative authorization result from L5-D-02 together with authoritative handling directives from L5-B-05-001. The gate's responsibility is limited to applying the required technical projection.

### 4.4 Audit Evidence

When an unmasked or specially transformed projection is released, L5-B-05-002 supplies minimized technical evidence to L5-D-04 containing actor/context references, target data reference, governing decision references and outcome. It does not publish a new business/domain event and does not define an approval workflow.

---

## 5. Dependencies & Integration

- **L5-B-05-001:** supplies authoritative data-control references.
- **L5-D-02:** supplies authorization decisions; remains owner of permissions and policy evaluation.
- **L5-B-02:** remains owner of persistence/query realization.
- **L5-D-03:** supplies cryptographic mechanisms where an approved transformation requires them.
- **L5-D-04:** records technical evidence of disclosure/masking outcomes.
- **L5-C-01 / L5-C-03:** consume already-governed projections at integration boundaries.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001 — Missing Handling Directive:** apply the most restrictive technically valid projection or block disclosure according to authoritative policy; never infer permissive access.
- **TB-002 — Missing Authorization Decision:** do not return unmasked protected data.
- **TB-003 — Export Consistency:** export/report/document generation must apply the same authoritative handling rules as synchronous responses.
- **TB-004 — Evidence Minimization:** do not place cleartext sensitive values in audit, diagnostics or traces.
- **TB-005 — Scope Isolation:** masking decisions are evaluated within the validated Tenant/Institution context.

---

## 7. Security, Privacy & Trust Boundaries

- D02 authorization decisions cannot be bypassed by masking components.
- Masking components cannot create or elevate privileges.
- D03 remains owner of cryptographic implementations.
- Sensitive values shall not be replicated into diagnostic or audit channels.

---

## 8. Conformance Requirements

- **CR-001:** Masking shall be applied from authoritative handling directives, not from a taxonomy invented by this specification.
- **CR-002:** Unmasked disclosure shall require an affirmative authoritative authorization outcome from L5-D-02 where policy requires authorization.
- **CR-003:** L5-B-05-002 shall not define permission codes, roles, step-up requirements, administrative approvals or authorization policy.
- **CR-004:** Masking shall not mutate authoritative source values unless an upstream-owned workflow explicitly commands such mutation.
- **CR-005:** Export, connector and report projections shall inherit the same data-handling constraints.
- **CR-006:** Technical disclosure/masking outcomes shall be evidenced through L5-D-04 without raw sensitive values.
- **CR-007:** Tenant and Institution scope shall be preserved.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Material Technical Contribution | Realization Status |
|---|---|---|---|
| L4-A-053 | L4-A-053-BH-006 | Applies authoritative personal-data masking/anonymization directives without creating authorization semantics. | Verified Realization |
| L4-H-041 | L4-H-041-BH-003 | Applies privacy-controlled display masking to externally visible personal attributes. | Verified Realization |
| L4-A-025 | L4-A-025-BH-001 | Consumes authoritative classification/control references when choosing the required projection. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Authorization Vocabulary:** Explicitly delegated to L5-D-02; this specification consumes decision artifacts only.
- **OD-002 — Field Mask Patterns:** Must be supplied by authoritative policy/configuration; no repository-wide pattern is invented here.
- **OD-003 — High-Volume Streaming Realization:** Implementation detail deferred; it may not weaken the same masking constraints.

**Current State:** Ready for Re-Review
