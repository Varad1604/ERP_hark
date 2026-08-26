# L2-A01 — Platform & Tenant Workflows — Locked Amendment

| Attribute | Value |
|---|---|
| Target | L2-A01 — Platform & Tenant Foundation |
| Amendment Version | 1.1.0 |
| Status | Locked |
| Effective Date | 2026-08-26 |
| Approved Through | Explicit Lead Architect remediation authorization recorded 2026-08-26 |
| Authority | Repository-change authorization recorded in the L3 Phase 5 architecture review remediation |

This amendment is part of the canonical L2-A01 architecture. It closes the actor/authority and Tenant-event traceability gaps identified during L3-A-001 review. Where this amendment clarifies a responsibility or event meaning not defined in the original L2-A01 document, this amendment is authoritative for that subject only. All unaffected L2-A01 ownership, lifecycle, rules, dependencies and contracts remain unchanged.

---

## 1. Canonical Business Actors and Authority

Locked L1 identifies `Platform Administrator` as a primary actor for Tenant Provisioning. This amendment carries that actor into L2-A01 and defines the missing business authority without introducing a new organizational role.

| Business Actor / Responsibility | Canonical Responsibility | Boundary |
|---|---|---|
| Platform Administrator | Reviews the Tenant Registration Request and records the business authorization for an eligible Tenant request to proceed into provisioning. Authorizes commencement of the canonical `Requested → Provisioning` path after applicable Platform/Tenant prerequisites are satisfied. | Does not transfer Tenant lifecycle ownership away from Platform & Tenant and does not gain authority over Institution or downstream-domain lifecycles. |
| Platform & Tenant Domain | Owns Tenant identity, Tenant lifecycle validation, Tenant lifecycle transitions, provisioning outcome, activation and Tenant business facts/events. | A human actor authorizes governed progression; the owning domain remains the canonical owner of the Tenant aggregate and lifecycle. |

### 1.1 Provisioning Initiation Decision

The business decision at the start of Tenant provisioning is:

```text
Tenant Registration Request
        ↓
Platform Administrator review
        ↓
Eligible and authorized to provision?
     ┌──┴──┐
    No    Yes
    │      │
    │      ↓
    │   Requested
    │      ↓
    │   Provisioning
    │
Request does not enter provisioning
```

Rules:

1. `Platform Administrator` is the canonical business authority for accepting an eligible Tenant Registration Request into the provisioning path.
2. Authorization to commence provisioning does not itself make the Tenant `Active`.
3. The Tenant may reach `Active` only through the existing L2-A01 lifecycle and validation rules.
4. The `Provisioning → Active` transition remains a Platform & Tenant lifecycle transition and may occur only when the existing canonical activation conditions are satisfied.
5. No separate second human approval is introduced for `Provisioning → Active` by this amendment.
6. Any execution automation associated with provisioning does not own the business decision, Tenant aggregate, lifecycle or events.
7. Platform authorization/policy capabilities may enforce the decision but do not become the owner of Tenant provisioning business truth.

No universal additional separation-of-duties rule is introduced because the existing locked architecture does not establish one.

---

## 2. Canonical Tenant Event Definitions

The existing locked L1 event names are preserved exactly. This amendment supplies their missing L2-A01 publication meaning and interested-domain interpretation.

| Business Event | Canonical Publisher | Publication Meaning | Consumer(s) / Interested Domain(s) |
|---|---|---|---|
| `TenantCreated` | Platform & Tenant / L2-A01 | A canonical Tenant has been established with its Tenant identity and entered the governed Tenant lifecycle. | No mandatory cross-domain event subscriber is established by current authoritative architecture. Platform capabilities and future explicitly governed consumers may use the fact without changing publisher ownership. |
| `TenantActivated` | Platform & Tenant / L2-A01 | The canonical Tenant has successfully completed provisioning and reached `Active`, making the Active Tenant context available for permitted downstream setup. | Section B — Institution & Structure is materially interested because L2-B01 requires an Active Tenant as its upstream context. Other downstream domains may consume Active Tenant context where their authoritative workflow requires it. |

### 2.1 Event Ownership and Consumption Rules

1. L2-A01 is the sole semantic publisher of `TenantCreated` and `TenantActivated`.
2. L2-A06 Event Infrastructure may distribute or govern event delivery but does not become the semantic publisher of Tenant business truth.
3. `TenantCreated` does not require an invented mandatory subscriber merely to remain a canonical business event.
4. `TenantActivated` communicates the Active Tenant fact; consumption of that fact does not transfer Tenant ownership.
5. Section B's foundational dependency on Section A is unchanged. This amendment creates no reverse dependency.
6. No event payload, transport, topic, queue, delivery mechanism or implementation protocol is defined at L2.

---

## 3. Active Tenant Business Hand-off to Institution & Structure

L2-B01 already declares `Active Tenant (L2-A01 Platform Foundation)` as an input and depends on L2-A01. The authoritative cross-domain business hand-off is therefore clarified as follows:

| Provider | Consumer | Business Fact / Hand-off | Purpose |
|---|---|---|---|
| Platform & Tenant / L2-A01 | Institution & Structure / L2-B01 | Active canonical Tenant context | Establishes the required upstream Tenant boundary within which Institution creation/configuration may begin. |

This is a business hand-off only. It does not prescribe whether a later implementation resolves the fact through an event, read contract, workflow context or other approved mechanism.

---

## 4. Audit and Evidence Obligations

The following Tenant-provisioning business decisions and lifecycle facts shall remain auditable:

- Platform Administrator authorization to commence provisioning;
- `Requested → Provisioning` progression;
- successful `Provisioning → Active` progression;
- unsuccessful provisioning and governed re-entry where applicable;
- publication of `TenantCreated` and `TenantActivated` where those events occur in the governed lifecycle.

L2-A07 retains ownership of canonical audit infrastructure and immutable audit evidence. L2-A01 retains ownership of the Tenant business decision, lifecycle meaning and Tenant events.

---

## 5. Locked Decisions

The following decisions are Locked by this amendment:

1. `Platform Administrator` is the canonical business authority for accepting an eligible Tenant Registration Request into the provisioning path.
2. Platform & Tenant remains the canonical owner of the Tenant aggregate, Tenant lifecycle and provisioning/activation business truth.
3. Authorization to commence provisioning does not itself activate the Tenant.
4. No new Tenant lifecycle state or transition is introduced.
5. `TenantCreated` and `TenantActivated` remain exact canonical event names.
6. Platform & Tenant / L2-A01 is the sole semantic publisher of both Tenant events.
7. `TenantCreated` has no mandatory cross-domain subscriber under current authoritative architecture.
8. Section B — Institution & Structure is a materially interested consumer of the `TenantActivated`/Active Tenant fact because L2-B01 requires Active Tenant context.
9. L2-A06 distributes events but does not own Tenant event semantics.
10. Consumption of Tenant facts does not transfer Tenant ownership or reverse foundational dependency direction.
11. Downstream L3 scenarios shall consume these decisions without inventing additional Tenant approval roles, event publishers or mandatory consumers.

---

## 6. Canonical Section Completion

For repository-governance purposes, this amendment closes the original L2-A01 crosswalk gap for Actors and completes the missing L3-relevant Tenant event semantics:

| Concern | Canonical Location After Amendment |
|---|---|
| Actors / business authority | This amendment §1 |
| Tenant event definitions / publisher / interested domains | This amendment §2 |
| Active Tenant cross-domain hand-off | This amendment §3 |
| Locked decisions | This amendment §5 |

The original L2-A01 document and this amendment shall be read together as the canonical L2-A01 architecture.