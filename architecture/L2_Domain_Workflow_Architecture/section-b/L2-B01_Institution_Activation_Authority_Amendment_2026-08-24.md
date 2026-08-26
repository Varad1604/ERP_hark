# L2-B01 — Institution Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-B01 — Institution Workflows |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-24 |
| Approved Through | ADR-B-INST-ACT-001 |
| Authority | Explicit repository-change authorization recorded 2026-08-24 |

This amendment is part of the canonical L2-B01 architecture. Where it clarifies an ambiguous responsibility in the original L2-B01 document, this amendment is authoritative for that subject only. All unaffected L2-B01 ownership, lifecycle, rules, dependencies, and event definitions remain unchanged.

---

## 1. Canonical Actors

| Business Actor / Authority | Canonical Responsibility | Boundary |
|---|---|---|
| Institution Administrator | Establishes and configures Institution information and submits the Institution activation request for governed decision. | Does not gain universal final activation authority merely by administering Institution configuration. |
| Institution Activation Authority | Makes the final governed Institution activation decision after applicable L2-B01 validation and approval prerequisites are evaluated. May approve, reject, or return the request for correction where the existing lifecycle path permits. | Abstract business authority, not a universal institutional job title. Local role mapping is governed by institutional policy and authorization architecture. |
| Tenant / Tenant Administration | Provides the existing upstream Tenant context required by L2-B01. | Does not thereby own Institution activation business truth. |
| Institution Domain | Owns the Institution aggregate, Institution lifecycle, Institution activation decision semantics, and Institution lifecycle events. | Platform authorization capabilities do not acquire ownership of the business decision. |

### 1.1 Institution-Independent Authority Mapping

`Institution Activation Authority` is intentionally an abstract business authority so that SutraERP remains valid across different educational institution governance models.

A school, college, university, autonomous institution, affiliated institution, or other supported institution may map this authority to its locally governed office, committee, board, or other eligible actor. That local mapping shall not redefine the canonical business responsibility established here.

L2-A04 continues to own Authorization Policy, authorization evaluation, and enforcement. L2-B01 owns the Institution activation business decision and its business evidence.

No universal Principal, Registrar, Trustee, Board, Tenant Administrator, or other fixed job title is made canonical by this amendment.

### 1.2 Separation of Duties

No universal separation-of-duties rule is introduced between the Institution Administrator and Institution Activation Authority because the existing locked architecture does not establish one. Institution-specific governance may require stronger separation through its authoritative business policy and authorization model.

---

## 2. Canonical Entities / Aggregates

This section consolidates entity semantics already established by locked L1 and L2-B01. It introduces no new ownership.

| Entity / Aggregate | Canonical Owner | L2-B01 Responsibility |
|---|---|---|
| Institution Aggregate | Institution Domain | Owns Institution identity, configuration, lifecycle, status, activation, suspension, reactivation, archival, and institutional history. |
| Institution Policy | Institution Domain | Canonical institutional business-policy entity, versioned and distinct from Institution Profile. |
| Institution Profile / Master Information | Institution Domain | Provides the Institution identity, regulatory, communication, operational, and administrative business information governed by L2-B01. |
| Tenant | Platform / Tenant Foundation | Upstream canonical context consumed by L2-B01; ownership does not transfer to Institution. |

The amendment does not move Organization, Campus, Academic Structure, Academic Calendar, Student, Faculty, Finance, Examination, or other downstream ownership into L2-B01.

---

## 3. Canonical Business Contracts and Hand-offs

These are business-level contracts and hand-offs only. No API, message schema, transport, persistence, or implementation mechanism is defined.

### 3.1 Consumes

| Provider | Business Contract / Hand-off | Purpose |
|---|---|---|
| L2-A01 Platform / Tenant Foundation | Active canonical Tenant context and Tenant association | Provides the upstream business context within which an Institution is established. |
| Institution Administrator | Completed Institution configuration and activation request | Provides the governed proposal for activation decision. |

### 3.2 Internal Governed Decision Hand-off

| From | To | Business Hand-off | Purpose |
|---|---|---|---|
| Institution Administrator | Institution Activation Authority | Configured Institution activation request with required validation evidence | Enables the final Institution-owned activation decision without changing aggregate ownership. |

### 3.3 Produces / Publishes

| Provider | Consumer(s) | Business Contract / Hand-off | Purpose |
|---|---|---|---|
| Institution Domain | L2-B02, L2-B03, L2-B04, L2-B05, L2-A10 and other authorized consumers | Canonical Institution identity and authoritative Institution lifecycle facts | Enables downstream configuration and governance without transferring Institution ownership. |
| Institution Domain | Interested downstream consumers | Existing public Institution lifecycle integration events including `InstitutionActivated` | Communicates authoritative lifecycle milestones. |

Existing L2-B01 event names and canonical publisher ownership remain unchanged.

---

## 4. Activation Decision Workflow Clarification

The existing L2-B01 activation workflow is clarified as follows:

```text
Institution Administrator
        │
        ▼
Complete Institution configuration
        │
        ▼
Submit activation request
        │
        ▼
Existing L2-B01 validation and eligibility checks
        │
        ▼
Institution Activation Authority
        │
   ┌────┼───────────────┐
   ▼    ▼               ▼
Approve Return          Reject
   │    │               │
   ▼    ▼               ▼
Active Configured/      Rejected
       correction path
   │
   ▼
Publish InstitutionActivated
```

The exact return path continues to follow the existing L2-B01 lifecycle. This amendment does not create a new state or transition.

---

## 5. Canonical Activation Rules

The following locked L2-B01 rules remain authoritative and are now associated with the clarified business authority:

1. Every Institution belongs to exactly one Tenant.
2. Institution Code must remain unique within a Tenant.
3. Institution cannot become `Active` unless mandatory validation succeeds.
4. The activation request is submitted by the Institution Administrator after required configuration is complete.
5. The final activation decision is made by the Institution Activation Authority.
6. Approval may move the valid Institution to `Active` through the existing L2-B01 lifecycle.
7. Rejection produces the existing `Rejected` outcome; `Rejected` cannot transition directly to `Active`.
8. Correctable requests may follow the existing `Returned` path.
9. Activation does not establish a global operational-readiness state; applicable L2-A10/L1-06 readiness governance remains separate.
10. Authorization enforcement remains owned by L2-A04; it does not own the Institution activation business decision.

---

## 6. Locked Decisions

The following decisions are Locked under `ADR-B-INST-ACT-001`:

1. L2-B01 remains the canonical owner of the Institution aggregate and Institution lifecycle.
2. `Institution Administrator` is the canonical business role for Institution establishment/configuration and activation-request submission.
3. `Institution Activation Authority` is the canonical business authority for the final activation decision.
4. `Institution Activation Authority` is an abstract institutional business authority and shall not be replaced in canonical architecture by one universal local title.
5. Institutional policy may govern which local actor fulfils the authority; L2-A04 owns authorization policy/enforcement mechanics.
6. No universal separation-of-duties rule is introduced by this amendment.
7. The existing Institution lifecycle is preserved; no new state is introduced.
8. `InstitutionActivated` remains the canonical public activation event published by the Institution domain.
9. Downstream domains consume Institution lifecycle facts but do not acquire Institution ownership.
10. L3 scenarios shall consume these decisions without redefining actor authority, ownership, lifecycle, event publisher, or local role mapping.

---

## 7. Canonical Section Completion

For repository-governance purposes, this amendment supplies the canonical counterparts that the original L2-B01 crosswalk recorded as absent:

| Required L2 Section | Canonical Location After Amendment |
|---|---|
| Actors | This amendment §1 |
| Entities | This amendment §2 |
| Contracts | This amendment §3 |
| Locked Decisions | This amendment §6 |

The original L2-B01 document and this amendment shall be read together as the canonical L2-B01 architecture.
