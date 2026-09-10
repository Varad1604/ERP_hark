# SutraERP L5 — Dependency-Gated Parallel Authoring Execution Timeline

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Decision Date:** 2026-09-09  
**Depends On:** L5 Authoring Population Freeze Decision 105; L5 Workload Complexity and Five-Person Allocation Decision; L5 Governance and Architecture Standard; L5 Validation Standard; L5 Technology Selection and ADR Methodology  

---

## 1. Decision

The 105-specification L5 corpus shall use **dependency-gated parallel authoring**.

The six execution stages below are not strict serial waves. Authors may work in parallel as soon as the specific upstream L5 dependency needed by a specification is architecturally stable enough that the downstream author does not need to invent that responsibility.

```text
Parallel authoring allowed
        BUT
only after required dependencies are stable
```

A whole stage does not need to finish before the next stage begins.

---

## 2. Stage 1 — Foundation

Start immediately in parallel.

| Area | Owner | Purpose |
|---|---|---|
| `L5-A-01` Application & Platform | Dhruv | Overall technical structure |
| `L5-A-03` Module, Dependency & Technical Boundaries | Dhruv | Technical module/dependency boundaries |
| `L5-A-04` Tenant & Institution Isolation | Dhruv | Tenant/Institution execution and isolation model |
| `L5-B-02` Database & Persistence | Dhruv | Persistence, transaction and concurrency foundation |
| `L5-D-03` Security & Trust | Varad | Security/trust/cryptography foundation |
| `L5-C-01` API & Contracts | Pallav | Synchronous contract foundation |
| `L5-E-04` Runtime, Packaging & Deployment | Pallav | Runtime/deployment foundation |
| `L5-D-01` IAM | Devang | Identity/authentication foundation |
| `L5-F-02-001` Verification & Testing Baseline | Chetan | Working verification framework |

### Stage 1 Objective

Stabilize enough shared architecture that later specifications do not invent:

- platform boundaries;
- Tenant/Institution context;
- persistence assumptions;
- trust boundaries;
- API/contract conventions;
- runtime/deployment assumptions;
- identity/authentication semantics.

---

## 3. Stage 2 — Control Architecture

Start each area when its dependencies are stable.

| Area | Owner | Minimum Dependency |
|---|---|---|
| `L5-D-02` Authorization / RBAC | Devang | `D01 + D03 + A04` |
| `L5-C-02` Events & Messaging | Pallav | `C01 + B02` |
| `L5-D-04-001/002` Audit & Evidence | Varad | `D03 + A04` |
| `L5-B-05` Privacy & Sensitive Data | Varad | `D03 + B02` |
| `L5-A-06` Configuration & Extensibility | Dhruv | `A01 + A04` |
| `L5-B-03` Master & Reference Data | Dhruv | `B02` |

### Stage 2 Objective

Stabilize common control mechanisms:

```text
Identity
+ Authorization
+ Audit
+ Privacy
+ Configuration
+ Messaging
```

---

## 4. Stage 3 — Shared Execution & Operations

| Area | Owner | Minimum Dependency |
|---|---|---|
| `L5-A-05` Workflow & Orchestration | Devang | `D02 + C01` |
| `L5-A-08` Background Processing & Jobs | Varad | `A01 + C02 + E04` |
| `L5-D-04-003` Tamper-Evident Audit Storage, Access & Export | Dhruv | `D04-001/002 + B02 + D03` |
| `L5-E-01` Availability & Resilience | Varad | `A01 + E04` |
| `L5-E-02` Performance & Capacity | Varad | `A01 + B02 + E04` |
| `L5-E-05` Observability & Operations | Pallav | `E04` |
| `L5-F-01` CI/CD & Supply Chain | Pallav | `E04 + D03` |

---

## 5. Stage 4 — Specialized Platforms

| Area | Owner | Minimum Dependency |
|---|---|---|
| `L5-A-07` Secure Computational Engines | Varad | `D03 + A08 + B02 + E04` |
| `L5-B-01` Search & Indexing | Varad | `B02 + D02 + C02` |
| `L5-B-04` Document / Media / Object Platform | Varad | `B02 + D03 + B05` |
| `L5-G-01` Client Platform | Dhruv | `C01 + D01 + D02 + A04` |
| `L5-C-04` Notification & Communication | Chetan | `C01 + C02` |
| `L5-B-06` Reporting / Analytics / Projection | Chetan | `B02 + D02 + C02` |

---

## 6. Stage 5 — External Integration & Recovery

| Area | Owner | Minimum Dependency |
|---|---|---|
| `L5-C-03` External Integrations & Connectors | Pallav | `C01 + C02 + D03 + E04` |
| `L5-E-03` Backup / Restore / Disaster Recovery | Pallav | `B02 + E04 + E01` |

---

## 7. Stage 6 — Consolidation

### `L5-A-02` Technology Platform & Standards — Dhruv

This area shall be consolidated late, after major architecture and ADR decisions are sufficiently mature.

Correct direction:

```text
Architecture need
      ↓
Technical pattern
      ↓
ADR where material
      ↓
Approved decision
      ↓
Technology Platform & Standards consolidation
```

`A02` shall not prematurely dictate implementation technology before architecture and ADR decisions exist.

### `L5-F-02` Verification & Testing — Chetan

`L5-F-02-001` is used from the beginning as the working verification baseline.

Final consolidation occurs after most L5 architecture is authored:

- `L5-F-02-001` Verification & Testing Baseline;
- `L5-F-02-002` Component, Unit & Domain Verification;
- `L5-F-02-003` Integration, Contract & End-to-End Verification;
- `L5-F-02-004` Security, Performance, Resilience & Recovery Testing.

---

## 8. Member-Wise Execution Order

### Dhruv

```text
A01
→ A03
→ A04
→ B02
→ A06
→ B03
→ D04-003
→ G01
→ A02 last
```

Dhruv may begin parallel L6 work once the relevant client-platform, contract, authentication/session and authorization boundaries are sufficiently stable.

### Varad

```text
D03
→ D04-001/002 + B05
→ A08 + E01 + E02
→ A07 + B01 + B04
```

### Pallav

```text
C01 + E04
→ C02 + E05
→ F01
→ C03
→ E03
```

### Devang

```text
D01
→ D02
→ A05
```

### Chetan

```text
F02-001 working baseline
→ C04 + B06
→ final F02 family
```

---

## 9. Execution Rule

The governing rule is:

> A specification may start as soon as its required upstream L5 dependency is architecturally stable enough that the author does not need to invent the upstream responsibility.

Therefore:

```text
Do not wait for a whole stage.
Wait only for the specific dependency needed.
```

This maximizes safe parallel authoring while protecting repository consistency.

---

## 10. Prohibited Execution Pattern

The following is not approved:

```text
All 105 specifications authored independently at once
without dependency stabilization
```

This would create unnecessary risk of:

- duplicate technical ownership;
- contradictory architecture assumptions;
- ADR duplication;
- cross-file remediation;
- vague downstream specifications;
- repository inconsistency.

---

## 11. Governance Boundary

This decision defines only the L5 execution sequence and dependency-gated parallelism.

It does not define:

- Git branches;
- reviewer allocation;
- pull-request workflow;
- manual task-assignment mechanics;
- broad-authoring repository authorization beyond any separately approved governance decision.

---

**Status:** Locked  
**Execution Model:** Dependency-Gated Parallel Authoring  
**Stages:** 6  
**Strict Serial Execution:** Not Required  
**Parallel Authoring:** Allowed after dependency stabilization
