# L5-B-05-001 — Data Classification, Retention & Privacy Architecture Baseline

## 1. Specification Metadata

**Specification ID:** L5-B-05-001  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical privacy and data-classification control model derived from upstream policy and ownership  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.1  
**Status:** Review  
**Last Updated:** 2026-09-12  
**Depends On:** Locked L1 Domain Architecture; Approved L2/L3 Workflows; Applicable canonical L4 System Behavior Specifications; L5-D-03; L5-B-02  
**Referenced By:** L5-B-05-002, L5-B-05-003, L5-B-05-004, L5-B-05-005, L5-B-04, L5-A-07, L5-C-01, L5-C-03  
**Applicable ADRs:** ADR-L5-0001; ADR-L5-0004; ADR-L5-0005  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the reusable technical control baseline by which SutraERP carries, validates and enforces classification and privacy constraints supplied by authoritative upstream owners. It standardizes how technical components consume those decisions without creating a repository-wide classification taxonomy, privacy policy, retention rule, consent rule or business exception.

### 2.2 In Scope

- A technical envelope for carrying authoritative classification and privacy-control references.
- Scope validation for Tenant and Institution context.
- Fail-closed treatment when required authoritative control context is absent, invalid or contradictory.
- Propagation of authoritative control metadata across service, persistence-projection and messaging boundaries.
- Audit-evidence handoff to L5-D-04 without copying protected payload content.
- Common integration contract consumed by L5-B-05-002 through L5-B-05-005.

### 2.3 Explicit Non-Goals

This specification does not own or define:

- Classification category names, sensitivity tiers, handling labels or privacy taxonomies.
- Retention periods, deletion eligibility, legal-hold authority or disposal lifecycle states.
- Consent states, purpose categories, consent events or consent publishers.
- Authorization permissions, roles, elevation rules or approval authority.
- Cryptographic algorithms, keys, signatures or credential mechanisms.
- Database schemas, physical persistence topology or job scheduling.
- Canonical business entity ownership or source-of-truth records.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Architectural Constraints

This specification is subordinate to locked L1, approved L2/L3 and canonical L4 behavior. L5-B-05-001 may enforce an authoritative upstream rule but shall not create, rename, generalize or replace that rule.

### 3.2 Ownership Preservation

Classification and privacy semantics remain owned by their authoritative upstream business/governance source. This baseline carries opaque references and technical enforcement directives whose meaning is resolved against that authoritative source.

A downstream component shall never infer that a missing classification reference means public, unrestricted or retention-free data.

### 3.3 Peer Ownership Boundaries

- **L5-B-02:** owns persistence topology, schema realization and transaction mechanics.
- **L5-D-02:** owns authorization decisions and permissions.
- **L5-D-03:** owns cryptographic protection and integrity mechanisms.
- **L5-D-04:** owns audit/evidence capture and persistence handoff.
- **L5-C-02:** owns durable event-delivery, retry, ordering and dead-letter behavior.

---

## 4. Technical Architecture

### 4.1 Authoritative Control Context

The baseline uses a policy-neutral technical envelope. Values are opaque references to authoritative source decisions rather than a taxonomy invented by this specification.

```text
AuthoritativeDataControlContext {
    tenant_context_ref
    institution_context_ref
    originating_owner_ref
    classification_decision_ref
    handling_directive_refs[]
    retention_policy_ref? 
    consent_or_privacy_policy_ref?
    integrity_proof_ref?
}
```

Rules:

1. `classification_decision_ref` identifies an authoritative classification decision; this specification does not define its possible values.
2. `handling_directive_refs` carry authoritative technical constraints without defining their business semantics here.
3. Optional retention and consent references are present only where upstream behavior requires them.
4. `integrity_proof_ref`, where required, is produced and verified using mechanisms governed by L5-D-03.

### 4.2 Control Evaluation Flow

```text
Authoritative request / event / projection
                |
                v
Resolve authoritative control references
                |
                v
Validate Tenant / Institution scope
                |
        +-------+-------+
        |               |
     valid          unresolved / invalid
        |               |
        v               v
Apply supplied       fail closed for the
technical control    protected operation
        |               |
        +-------+-------+
                v
       Evidence handoff to L5-D-04
```

### 4.3 Architecture Position — Policy-Neutral Fail-Closed Enforcement

L5-B-05-001 standardizes **how** authoritative data-control decisions are carried and enforced. It does not standardize **what** classifications, purposes, retention rules or consent states exist. Where the governing source requires control context and that context cannot be resolved or validated, the protected operation fails closed.

### 4.4 Scope Preservation

Tenant and Institution context shall be preserved exactly as established by authoritative tenancy architecture. This specification does not create cross-institutional authority; any cross-scope operation must arrive with authority already established by its owning workflow/contract.

---

## 5. Dependencies & Integration

- **L5-B-02:** persists domain data and any technically required references under B02-owned schemas and transactions.
- **L5-D-03:** supplies cryptographic/integrity mechanisms where approved.
- **L5-D-04:** records attributable technical enforcement evidence.
- **L5-B-05-002 through L5-B-05-005:** consume this policy-neutral context and specialize technical enforcement without changing upstream semantics.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001 — Missing Required Context:** fail closed; do not substitute a default classification or policy.
- **TB-002 — Conflicting Context:** fail closed and record attributable technical evidence.
- **TB-003 — Scope Mismatch:** reject processing; never coerce one Tenant/Institution context into another.
- **TB-004 — Unknown Policy Value:** preserve the opaque reference and delegate semantic interpretation to the authoritative owner/adapter; do not invent a mapping.
- **TB-005 — Evidence Minimization:** evidence records contain decision references and outcomes, not protected payload values.

---

## 7. Security, Privacy & Trust Boundaries

- Authorization outcomes are consumed from L5-D-02 and are not recreated here.
- Cryptographic protection is consumed from L5-D-03.
- Control metadata must not become a side channel that exposes protected payload values.
- Cross-boundary processing requires validated Tenant/Institution context under ADR-L5-0004.

---

## 8. Conformance Requirements

- **CR-001:** L5-B-05-001 shall not define a canonical classification taxonomy, privacy taxonomy, consent taxonomy or retention taxonomy.
- **CR-002:** Required authoritative control context shall be resolved before protected processing proceeds.
- **CR-003:** Missing, invalid or contradictory required context shall fail closed.
- **CR-004:** Tenant and Institution scope shall be preserved across all control-evaluation boundaries.
- **CR-005:** Downstream components shall not downgrade, rename or reinterpret authoritative decisions without upstream authority.
- **CR-006:** Authorization decisions shall remain owned by L5-D-02.
- **CR-007:** Persistence schemas and transaction mechanics shall remain owned by L5-B-02.
- **CR-008:** Cryptographic mechanisms shall remain owned by L5-D-03.
- **CR-009:** Enforcement outcomes shall be handed to L5-D-04 as minimized evidence.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Material Technical Contribution | Realization Status |
|---|---|---|---|
| L4-A-025 | L4-A-025-BH-001 | Carries authoritative classification/privacy references with Tenant/Institution scope before downstream control evaluation. | Verified Realization |
| L4-A-053 | L4-A-053-BH-006 | Propagates authoritative personal-data protection directives without inventing their taxonomy or lifecycle. | Verified Realization |
| L4-H-001 | L4-H-001-BH-001 | Preserves authoritative scope and protection references across external connector boundaries. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Global Classification Taxonomy:** Not owned here. Any repository-wide taxonomy requires authoritative upstream establishment and, where it materially constrains multiple L5 families, ADR/governance treatment before this specification may encode it.
- **OD-002 — Integrity Mechanism:** Delegated to L5-D-03; no HMAC, signature or cipher is selected here.
- **OD-003 — Runtime Integration Hooks:** Implementation binding is deferred; this specification requires an enforceable boundary but does not mandate HTTP, gRPC, ORM or broker-specific interceptors.

**Current State:** Ready for Re-Review
