# L5-B-05-005 — Consent, Purpose-Limitation & Privacy-Control Architecture

## 1. Specification Metadata

**Specification ID:** L5-B-05-005  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-05 — Data Classification, Retention & Privacy Architecture  
**Canonical Technical Responsibility:** Technical enforcement of authoritative consent state/withdrawal, approved purpose, minimization and disclosure constraints; never canonical consent owner  
**Technical Architecture Owner:** Lead Solution Architect / Data & Information Architecture  
**Assigned Author:** Varad  
**Version:** 1.0.1  
**Status:** Locked  
**Last Updated:** 2026-09-12  
**Depends On:** L5-B-05-001; L5-B-05-002; L5-C-02; L5-D-02; L5-D-04; applicable canonical L4 behavior  
**Referenced By:** L5-C-01, L5-C-03, L5-C-04, L5-B-04, L5-B-06, L5-G-01  
**Applicable ADRs:** ADR-L5-0001; ADR-L5-0004; ADR-L5-0010; ADR-L5-0011  

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical enforcement boundary for authoritative consent, purpose-limitation, minimization and disclosure constraints. It consumes exact consent facts and events from their canonical publishers and applies the resulting technical gate across relevant egress channels.

It does not create a global SutraERP consent lifecycle, generic consent event names, purpose taxonomy, withdrawal publisher or performance SLA.

### 2.2 In Scope

- Resolution of authoritative consent/purpose decision references.
- Technical disclosure blocking when the authoritative source indicates that disclosure is not permitted.
- Data minimization according to an authoritative purpose-to-attribute decision or policy reference.
- Invalidation of locally cached enforcement decisions when an authoritative source event indicates a material consent change.
- Application across APIs, connectors, notifications, reporting and export boundaries where relevant.
- Audit-evidence handoff through L5-D-04.

### 2.3 Explicit Non-Goals

This specification does not own:

- Canonical consent records, consent wording, consent legal validity or consent lifecycle states.
- Consent event names or canonical publishers.
- Repository-wide purpose categories or channel enumerations.
- Authorization policy or permissions.
- Event delivery, retry, ordering, dead-letter or propagation-time guarantees.
- UI consent controls or preference-center design.

---

## 3. Architectural Constraints & Ownership

### 3.1 Source-Domain Consent Ownership

Consent remains owned by the domain/workflow identified in authoritative upstream architecture. L5-B-05-005 consumes the exact source fact or event, preserving its canonical name and publisher. For example, where L4 establishes `GuardianConsentWithdrawn`, this specification consumes that event; it does not rename it to a generic `ConsentWithdrawn` event or become its publisher.

### 3.2 Purpose Semantics

Purpose categories and permitted attributes must be supplied by authoritative business/governance policy. This specification carries opaque `purpose_ref` and `permitted_attribute_policy_ref` values rather than defining a universal enumeration such as Admissions, Payroll or Alumni Relations.

### 3.3 Event Transport Ownership

L5-C-02 owns event-delivery reliability, ordering, retry, dead-letter behavior and transport guarantees. L5-B-05-005 may react to an authoritative event but shall not define sub-second or other transport/SLA targets.

---

## 4. Technical Architecture

### 4.1 Privacy Enforcement Flow

```text
Request / egress attempt
        |
        v
Resolve authoritative subject + consent + purpose references
        |
        v
Consume D02 authorization decision where required
        |
        v
Evaluate disclosure against authoritative privacy decision
        |
   +----+----+
   |         |
allowed    blocked / unresolved
   |         |
   v         v
minimize    deny disclosure
payload      |
   |         |
   +----+----+
        v
Evidence handoff to D04
```

### 4.2 Policy-Neutral Privacy Context

```text
PrivacyEnforcementContext {
    tenant_context_ref
    institution_context_ref
    subject_identity_ref
    consent_owner_ref
    consent_decision_ref
    purpose_ref
    permitted_attribute_policy_ref
    source_event_ref?
}
```

No consent-status enum, purpose enum or channel enum is canonicalized here.

### 4.3 Authoritative Change Invalidation

Where the authoritative consent owner publishes a canonical change event, L5-B-05-005 invalidates any cached enforcement result derived from the superseded consent decision. The exact event name and publisher are preserved from upstream architecture. Delivery timing and retry semantics are delegated to L5-C-02.

### 4.4 Data Minimization

The gate projects only the attributes permitted by the authoritative purpose/disclosure policy reference. L5-B-05-005 does not author the allow-list; it applies the allow-list supplied by the authoritative source/configuration owner.

---

## 5. Dependencies & Integration

- **L5-B-05-001:** supplies policy-neutral control context.
- **L5-B-05-002:** applies masking/redaction after a disclosure is permitted but constrained.
- **L5-D-02:** owns authorization decisions.
- **L5-C-02:** transports authoritative consent-related events and owns delivery/recovery semantics.
- **L5-D-04:** records technical privacy-enforcement evidence.
- **L5-C-01 / L5-C-03 / L5-C-04:** consume the privacy enforcement result at channel boundaries.

---

## 6. Technical Behavior & Failure Concerns

- **TB-001 — Authoritative Withdrawal/Restriction:** once the canonical change is observed, locally cached decisions based on superseded consent data are invalidated before further disclosure using that stale decision.
- **TB-002 — Missing Purpose Decision:** fail closed where purpose authorization is required; do not invent a purpose category.
- **TB-003 — Purpose Non-Expansion:** a decision for one authoritative purpose cannot be reused for another purpose without a new evaluation.
- **TB-004 — Unknown Consent Semantics:** preserve the source reference and require authoritative resolution; do not map to a locally invented status.
- **TB-005 — Audit Minimization:** record decision references and outcome, not unnecessary personal data.

---

## 7. Security, Privacy & Trust Boundaries

- Authorization does not substitute for privacy permission; both are independently consumed where required by authoritative policy.
- Privacy enforcement cannot create business consent or alter the source consent record.
- Event transport behavior remains governed by C02/ADR-L5-0010 and event compatibility by ADR-L5-0011.
- Tenant and Institution scope remain mandatory under ADR-L5-0004.

---

## 8. Conformance Requirements

- **CR-001:** L5-B-05-005 shall consume consent facts/events from their canonical owner and preserve exact event names/publishers.
- **CR-002:** L5-B-05-005 shall not define a repository-wide consent lifecycle or status enumeration.
- **CR-003:** L5-B-05-005 shall not define a repository-wide purpose taxonomy.
- **CR-004:** Where authoritative policy requires consent/purpose approval, unresolved or disallowed processing shall fail closed.
- **CR-005:** Data minimization shall apply the authoritative permitted-attribute decision without locally expanding it.
- **CR-006:** Authorization decisions remain owned by L5-D-02.
- **CR-007:** Delivery/retry/order/dead-letter and propagation guarantees remain owned by L5-C-02.
- **CR-008:** No numeric propagation SLA is established by this specification unless an authoritative source/ADR establishes it.
- **CR-009:** Privacy-enforcement outcomes shall be evidenced through L5-D-04.

---

## 9. Traceability

| L4 Specification ID | Exact L4 BH ID | Material Technical Contribution | Realization Status |
|---|---|---|---|
| L4-C-054 | L4-C-054-BH-007 | Consumes the canonical `GuardianConsentWithdrawn` fact/event and invalidates affected technical enforcement decisions without changing its publisher or name. | Verified Realization |
| L4-C-054 | L4-C-054-BH-008 | Distributes/consumes authoritative consent decision references without transferring consent ownership. | Verified Realization |
| L4-C-055 | L4-C-055-BH-002 | Applies authoritative consent/privacy constraints before governed communication dispatch. | Verified Realization |
| L4-H-001 | L4-H-001-BH-005 | Enforces the authoritative consent decision required for external linking without inventing a global lifecycle. | Verified Realization |
| L4-H-003 | L4-H-003-BH-003 | Consumes locked consent decisions for credential-publication enforcement. | Verified Realization |
| L4-H-003 | L4-H-003-BH-004 | Blocks future publication when the authoritative consent decision requires blocking. | Verified Realization |
| L4-H-041 | L4-H-041-BH-003 | Applies authoritative directory visibility/purpose constraints to personal profile disclosure. | Verified Realization |

---

## 10. Open Decisions & Deferred Design

- **OD-001 — Consent Event Vocabulary:** Not owned here; exact canonical events come from authoritative source domains/L4 behavior.
- **OD-002 — Purpose Vocabulary:** Not owned here; purpose references remain source-governed.
- **OD-003 — Event Propagation Timing:** Delegated to L5-C-02 and operational SLO governance; no sub-second requirement is asserted here.
- **OD-004 — Cached Decision Implementation:** Technology-specific cache selection/invalidation protocol is deferred and must preserve C02 and source-domain ownership.

**Current State:** Locked
