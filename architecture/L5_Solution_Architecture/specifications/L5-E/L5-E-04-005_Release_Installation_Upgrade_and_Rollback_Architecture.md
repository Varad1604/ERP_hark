# Release Installation, Upgrade & Rollback Architecture

## 1. Specification Metadata

**Specification ID:** `L5-E-04-005`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-E — Reliability, Deployment & Operations Architecture  
**Major Area:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Canonical Technical Responsibility:** Release installation, upgrade, compatibility validation and rollback architecture for supported SutraERP deployment profiles.  
**Technical Architecture Owner:** L5-E-04 — Runtime, Packaging & Deployment Architecture  
**Assigned Author:** Pallav  
**Version:** `1.0.1`  
**Status:** `Review`  
**Last Updated:** `2026-09-11`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register; `L5-E-04-001`, `L5-E-04-002`, `L5-E-04-003`, `L5-E-04-004`, `L5-C-01-003`, `L5-B-02-003`, `L5-B-02-004`, `L4-A-029`, `L4-A-030`, `L4-A-031`, `L4-A-032`.  
**Collaborates With / Referenced By:** `L5-E-03-*`, `L5-E-05-*`, `L5-F-01-*`.  
**Applicable ADRs:** `ADR-L5-0006`, `ADR-L5-0007`, `ADR-L5-0008`; E04 placement/isolation decisions from `ADR-L5-0003` also constrain rollout topology. Exact rolling/blue-green/canary/orchestration tooling remains unresolved.

---

## 2. Purpose, Scope & Non-Goals

Define how a governed SutraERP release is installed, upgraded, validated and, where technically safe, rolled back without inventing business recovery semantics or bypassing contract/data compatibility rules.

### In Scope

- Release/upgrade preflight.
- Deployment serialization/coordination boundary.
- Contract/schema compatibility gates.
- Mixed-version coexistence rules at architecture level.
- Post-deployment verification.
- Rollback-safety classification.
- Explicit handoff to recovery/operations architecture.

### Out of Scope

- Business compensation/reversal.
- Backup/restore internals — E03.
- Observability platform internals — E05.
- CI/CD implementation internals — F01.
- Database migration internals — B02.
- Exact rollout/orchestrator product.
- Invented maintenance windows or numeric SLOs.

---

## 3. Authoritative Basis

| Source | Effect |
| --- | --- |
| `L5-C-01-003` | Provides contract compatibility, coexistence and retirement status consumed by E04 rollout decisions. |
| `L5-B-02-003/004` | Persistence transaction/migration ownership remains B02-owned. |
| `ADR-L5-0006` | Upgrade operations shall respect owner-local transaction/concurrency boundaries. |
| `ADR-L5-0007` | Durable state-to-publication intent handoff constrains upgrade/rollback behavior where event publication state is involved. |
| `ADR-L5-0008` | Expand-Migrate-Transition-Contract establishes safe schema evolution sequencing. |
| `ADR-L5-0003` | Rollout topology follows modular-core/selective-isolation architecture; service decomposition is not invented by deployment strategy. |
| `L4-A-029/A030/A031/A032` | Successor coexistence, suspension/restoration, retirement and dependency compatibility remain upstream-governed. |

Draft L4 sources support controlled Draft authoring only and are not treated as immutable approved behavioral authority.

---

## 4. Technical Ownership & Dependencies

### Owns

- Release/upgrade preflight orchestration.
- Deployment serialization and phase coordination.
- Consumption of compatibility/migration readiness evidence.
- Runtime rollout sequencing across supported E04 profiles.
- Post-deployment technical acceptance checks.
- Rollback-safety determination at deployment boundary.

### Relationships

| Specification | Relationship | Boundary |
| --- | --- | --- |
| `L5-E-04-001/002/003/004` | Dependency | Supplies runtime, packaging, on-prem and cloud-capable deployment profiles. |
| `L5-C-01-003` | Dependency / authoritative input | E04 consumes contract compatibility/coexistence status. C01 does not depend on E04. |
| `L5-B-02-003/004` | Dependency | Data/schema migration correctness remains B02-owned. |
| `L5-E-03-*` | Collaboration / delegated recovery | E04 hands off recovery when rollback is unsafe or recovery primitives are required; E04 does not depend on later-stage E03 internals to define its own rollout gates. |
| `L5-E-05-*` | Collaboration / consumer | E05 supplies/consumes operational evidence and acceptance signals. |
| `L5-F-01-*` | Collaboration / producer-consumer | F01 may automate delivery, but release architecture remains E04-owned. |

This establishes one-way authority: `C01-003 → E04-005` for compatibility evidence and removes the previous circular relationship.

---

## 5. Solution Architecture

### 5.1 Upgrade Flow

```text
governed release candidate
→ verify release/artifact identity
→ verify target deployment profile
→ evaluate C01 contract compatibility/coexistence
→ evaluate B02 schema/data migration readiness
→ evaluate runtime/dependency/security prerequisites
→ acquire governed deployment coordination/serialization
→ execute approved deployment phases
→ validate runtime readiness + compatibility + technical acceptance evidence
→ either accept release
   or enter governed rollback/recovery path.
```

### 5.2 Rollback-Safety Classification

Rollback is not assumed to be universally safe. Before rollback, E04 shall determine whether the release introduced irreversible or forward-only conditions, including:

- schema/data transformations whose predecessor cannot safely interpret resulting state;
- contract changes that invalidate predecessor compatibility;
- durable publication/outbox state whose reversal would duplicate or contradict externally visible effects;
- security/key/credential transitions that make predecessor execution unsafe;
- other owner-declared forward-only technical changes.

When rollback is unsafe, E04 shall stop treating “redeploy previous artifact” as recovery and hand off to the governed E03/B02 recovery/migration path.

### 5.3 Mixed-Version Coexistence

Mixed-version operation is permitted only where C01/B02 compatibility evidence confirms that participating versions can safely coexist for the intended phase. Deployment tooling shall not invent compatibility.

### 5.4 Failure Model

| Failure | Required Outcome | Forbidden Outcome |
| --- | --- | --- |
| Preflight compatibility failure | Block rollout before protected change. | Force deployment through incompatibility. |
| Migration partially applied | Stop/contain and use B02-governed recovery semantics. | Ad-hoc reversal outside B02 authority. |
| Runtime acceptance failure | Halt progression; rollback only if safety gate passes. | Blind previous-version redeploy. |
| Rollback unsafe | Escalate to governed recovery path. | Claim rollback completed merely because old binaries started. |
| Mixed-version incompatibility | Prevent/terminate incompatible coexistence safely. | Continue unsupported skew. |

---

## 6. Architecture Decisions

| Decision ID | Decision | Position | Authority |
| --- | --- | --- | --- |
| AD-001 | Compatibility direction | E04 consumes C01-003 compatibility/coexistence decisions; C01-003 is not deployment-dependent. | Canonical ownership correction |
| AD-002 | Schema evolution | Upgrades follow expand-migrate-transition-contract sequencing where applicable. | `ADR-L5-0008` |
| AD-003 | Owner-local consistency | Upgrade/migration actions respect owner-local transaction/concurrency boundaries. | `ADR-L5-0006` |
| AD-004 | Publication integrity | Rollback/recovery shall preserve durable publication-intent correctness. | `ADR-L5-0007` |
| AD-005 | Rollback safety | Rollback is conditional on explicit compatibility/state safety; it is not a universal guarantee. | E04/B02/C01 correctness |
| AD-006 | Deployment topology | Rollout follows modular-core/selective-isolation topology rather than inventing service boundaries. | `ADR-L5-0003` |
| AD-007 | Rollout technique | Rolling, blue-green, canary or equivalent techniques remain profile/tool choices until governed. | ADR if standardized |

---

## 7. Conformance Requirements

| Requirement ID | Requirement |
| --- | --- |
| CR-001 | Every upgrade shall pass release identity, contract compatibility and data-migration readiness gates before protected rollout phases. |
| CR-002 | E04 shall consume C01-003 compatibility evidence; C01-003 shall not depend on E04-005 deployment mechanics. |
| CR-003 | Schema/data evolution shall preserve B02 ownership and `ADR-L5-0008` sequencing where applicable. |
| CR-004 | Rollback shall not be attempted when the predecessor cannot safely interpret post-upgrade state. |
| CR-005 | When rollback is unsafe, E04 shall hand off to governed recovery rather than inventing recovery behavior. |
| CR-006 | E03/E05/F01 shall remain collaborators/consumers, not prerequisite authorities for E04-005. |
| CR-007 | Rollout tooling shall not create business/module lifecycle states or redefine canonical compatibility. |

---

## 8. Open Decisions

| Item ID | Item | Blocking? | Resolution |
| --- | --- | --- | --- |
| OD-001 | Default rollout technique (rolling/blue-green/canary/etc.) | No for architecture pattern; Yes before normative shared rollout standard | Govern through ADR if standardized. |
| OD-002 | Exact mixed-version support durations/windows | No | Derive from approved C01/B02 compatibility and operational profile; do not invent universal numbers. |
| OD-003 | E03 recovery implementation details | No for E04 architecture | Consume E03 when authored; E04 owns only the handoff condition. |
| OD-004 | E05 acceptance/health implementation details | No for E04 architecture | Consume E05 evidence contract when authored. |
| OD-005 | Material L4 parents remain Draft | Yes for Architecture Approved | Resolve through L4 governance. |

---

## 9. Traceability Summary

**Material Dependencies:** `L5-E-04-001`, `L5-E-04-002`, `L5-E-04-003`, `L5-E-04-004`, `L5-C-01-003`, `L5-B-02-003`, `L5-B-02-004`, `L4-A-029`, `L4-A-030`, `L4-A-031`, `L4-A-032`  
**Material Collaborators:** `L5-E-03-*`, `L5-E-05-*`, `L5-F-01-*`  
**Applicable Approved ADRs:** `ADR-L5-0003`, `ADR-L5-0006`, `ADR-L5-0007`, `ADR-L5-0008`
