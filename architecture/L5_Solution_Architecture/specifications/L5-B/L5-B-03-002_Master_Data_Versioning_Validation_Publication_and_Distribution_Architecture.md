# Master Data Versioning, Validation, Publication & Distribution Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-03-002`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-03 — Master & Reference Data Technical Architecture  
**Canonical Technical Responsibility:** Technical versioning, validation execution, publication and distribution of approved Master/Reference Data while upstream owners retain business semantics, governance authority and canonical event ownership.  
**Technical Architecture Owner:** L5-B-03 — Master & Reference Data Technical Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.1`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** `L5-B-03-001`; `L5-B-02-001`; `L5-B-02-002`; `L5-B-02-003`; `L5-C-02-001`; `L5-C-02-002`; `L2-A09` v1.0 Locked; applicable L4 Master Data specifications identified in §3; L5 governance, validation, traceability and ADR standards.  
**Referenced By:** L5 consumers requiring governed Master/Reference Data publication, synchronization or historical version access.  
**Applicable ADRs:** `ADR-L5-0005` — Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern; `ADR-L5-0006` — Owner-Local Transaction and Optimistic Concurrency Strategy; `ADR-L5-0007` — Atomic State-to-Publication Intent Handoff Pattern. No broker, MDM product or database product is selected here.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the technical realization through which a registered Master Data or Reference Data Dataset evolves through immutable versions, executes source-governed validation, becomes technically publishable only after authoritative governance prerequisites succeed, and is distributed to eligible consumers without transferring business ownership or event-delivery ownership.

### 2.2 In Scope

- immutable Dataset Version identity and lineage;
- successor-version creation;
- technical validation pipeline and owner-semantic validation handoff;
- validation evidence binding;
- publication-readiness gate;
- technical publication state materialization corresponding to source-governed lifecycle outcomes;
- exact Published-version binding;
- effective-period consistency checks;
- localized representation version linkage;
- publication-intent handoff to C02;
- subscription/consumer eligibility handoff;
- per-consumer synchronization work identity and acknowledgement correlation;
- failure isolation between consumers;
- historical version availability and non-destructive replacement;
- retirement publication boundary without owning archival.

### 2.3 Out of Scope / Non-Goals

- Dataset registration and canonical representation baseline — `L5-B-03-001`;
- business meaning, data ownership, stewardship or approval authority — upstream owners / `L2-A09`;
- invention of lifecycle states or canonical Dataset events — upstream L2/L4;
- physical persistence, database, isolation level or locking primitive — B02;
- event schema/transport/retry/ordering/replay mechanics — C02;
- synchronous API protocol — C01;
- bulk file import/export staging, packaging and transfer — C03-008;
- workflow approval mechanics — A05 where applicable;
- authorization policy — D02;
- retention/archival/deletion/legal hold — B05;
- UI/UX — L6.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-B-03-002 Use |
|---|---|---|---|---|---|
| L2 | `L2-A09` | v1.0 / Locked | §§7, 9–10, 21.5, 25, 27, 56 | Governed Dataset lifecycle/versioning/effective dating; publication/synchronization infrastructure; business meaning remains domain-owned. | Primary semantic boundary for version/publication mechanics. |
| L4 | `L4-A-034` | v1.0.0 / Draft on `main` | `BH-001`–`BH-009` | Validation, Draft/Review/Approved/Published progression, immutable publication and restricted pre-publication consumption. | Drives validation and publication gates. |
| L4 | `L4-A-035` | v1.0.0 / Draft on `main` | `BH-001`–`BH-009` | Successor versions preserve immutable source history, validation and publication governance. | Drives successor/lineage architecture. |
| L4 | `L4-A-038` | v1.0.0 / Draft on `main` | `BH-001`–`BH-008` | Synchronization binds one Published source version, resolves eligible consumers, isolates outcomes and requires acknowledgement. | Drives distribution-work and consumer-isolation architecture. |
| L4 | `L4-A-039` | v1.0.0 / Draft on `main` | `BH-001`–`BH-008` | Localized Dataset representations preserve canonical identity and use governed validation/approval/publication. | Drives localized-version linkage. |
| L4 | `L4-A-040` | v1.0.0 / Draft on `main` | `BH-001`–`BH-009` | Published→Deprecated→Retired remains governed, historical data preserved, archival separate. | Drives retirement/distribution boundary without archival ownership. |
| L5 Peer | `L5-B-03-001` | Architecture Approved | Dataset Registry, identity, descriptor and canonical representation boundary | Supplies registered Dataset and representation identity. | B03-002 evolves registered datasets without redefining identity. |
| L5 Peer | `L5-B-02-003` | Review | owner-local transactions, optimistic concurrency and atomic publication-intent handoff | Supplies persistence consistency and B02→C02 handoff. | Used for version publication/concurrency without distributed ACID. |
| ADR | `ADR-L5-0007` | Architecture Approved | Atomic state + Durable Publication Intent | Committed publication state and durable publication intent are atomic; C02 owns delivery after handoff. | Prevents B03 from owning event transport. |

### 3.2 Authoritative Constraints

| ID | Constraint | Basis | L5 Effect |
|---|---|---|---|
| `C-001` | Published/historical Dataset Versions are immutable. | `L2-A09`; `L4-A-034/035` | Changes/restoration create successor versions. |
| `C-002` | Validation and governance prerequisites must succeed before publication. | `L4-A-034-BH-001/BH-006/BH-007` | Publication Readiness Gate is mandatory. |
| `C-003` | B03 may execute technical validation but cannot invent business semantics. | `L2-A09` ownership model | Semantic validation is delegated to canonical owner contracts. |
| `C-004` | Publication binds one exact Dataset Version. | `L4-A-034/035`; `L4-A-038-BH-001` | Consumers/distribution work always reference exact immutable version. |
| `C-005` | Consumer synchronization outcomes are independent. | `L4-A-038-BH-003/BH-006` | One consumer failure cannot modify source or successful consumer outcomes. |
| `C-006` | Completion requires authoritative consumer acknowledgement where upstream requires it. | `L4-A-038-BH-005` | Distribution Work records acknowledgement correlation; C02 owns transport mechanics. |
| `C-007` | Canonical Dataset event names/publishers come from upstream. | `L2-A09`; applicable L4 | B03 emits only source-defined publication intent/event instances; no alternate event names. |
| `C-008` | Retirement does not imply archival. | `L4-A-040-BH-008` | Retired version remains historical; archival belongs elsewhere. |
| `C-009` | Effective-period overlap/consistency must not be resolved silently. | `L2-A09` §9; `L4-A-035-BH-005` | Invalid temporal ambiguity blocks publication. |

### 3.3 Assumptions

No material architecture assumption is required.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

`L5-B-03-002` owns:

- **Dataset Version Repository** logical boundary;
- immutable version and predecessor/successor lineage semantics;
- **Dataset Validation Coordinator**;
- validation evidence binding;
- **Publication Readiness Gate**;
- **Published Version Pointer/Binding** semantics;
- effective-period technical consistency validation;
- localized representation version linkage;
- B03 side of publication-intent creation/handoff;
- **Distribution Work Registry** semantics for per-consumer source-version binding and outcome correlation;
- technical synchronization lifecycle state necessary to distinguish pending/completed/failed work without redefining upstream business lifecycle;
- historical version-access baseline.

### 4.2 Consumes / Collaborates With

| Exact L5 Specification | Capability | Boundary |
|---|---|---|
| `L5-B-03-001` | Dataset identity/representation/discovery | Supplies stable Dataset identity and descriptor. |
| `L5-B-02-001/002` | persistence ownership/isolation | Physical state remains B02-owned. |
| `L5-B-02-003` | transaction/concurrency/publication handoff | Supplies owner-local commit and optimistic conflict behavior. |
| `L5-C-01-*` | synchronous contracts | Realizes synchronous Dataset read/ack contracts where applicable. |
| `L5-C-02-001/002/003` | event/messaging | Owns routing, durable delivery, retries, acknowledgement transport and event schema compatibility. |
| `L5-D-02-*` | authorization | Authorizes protected version, publication and consumer operations. |
| `L5-D-04-*` | audit/evidence | Preserves technical evidence. |
| `L5-B-05-*` | privacy/retention | Governs sensitive content and historical retention/archival. |
| `L5-E-05-*` | observability | Observes stalled publication/distribution without becoming business owner. |

### 4.3 Must Not Own

Business validation rules, approval authority, stewardship, canonical lifecycle/event definitions, consumer business processing, event transport, physical persistence, retry/backoff policy, archival/retention policy or UI.

---

## 5. Solution Architecture

### 5.1 Version & Publication Model

```text
Registered Dataset (B03-001)
        │
        ▼
Create Dataset Version Draft
        │
        ▼
Dataset Validation Coordinator
        │
        ├── technical validation
        └── canonical-owner semantic validation handoff
        │
        ▼
source-governed review / approval outcome
        │
        ▼
Publication Readiness Gate
        │
        ▼
owner-local commit (B02-003)
  Published Version Binding
        +
  Durable Publication Intent
        │
        ▼
C02 publication / delivery
        │
        ▼
per-consumer Distribution Work
```

### 5.2 Dataset Version

Every material governed content change is represented as an immutable Dataset Version once source-governed publication/historical immutability applies.

Conceptually:

```text
DatasetVersion
{
    Dataset identity reference
    Dataset Version identity
    predecessor/source version reference
    canonical representation definition/version reference
    effective-period reference where applicable
    validation evidence reference
    governance outcome reference
    localization relationship where applicable
}
```

This is not a physical schema.

### 5.3 Successor-Version Creation

A change, correction or governed historical restoration shall never overwrite an immutable Published/historical version.

```text
Immutable Version V1
       │
       └── approved change/restoration request
                    ↓
             Successor Draft V2
                    ↓
       validation + normal governance
                    ↓
              Published V2
```

The predecessor remains addressable and unchanged.

Concurrent successor creation that would create conflicting authoritative successors is detected through B02-003 owner-local concurrency control rather than resolved by last-write-wins.

### 5.4 Validation Coordinator

The Validation Coordinator separates technical validation from business-semantic authority.

Technical validation may include architecture-supported checks such as:

- Dataset/representation registration;
- required technical metadata completeness;
- structural compatibility against the governed representation definition;
- duplicate/identity conditions exposed by upstream rules;
- protected Tenant/scope consistency;
- effective-period technical consistency;
- predecessor/source lineage consistency.

Business-rule validation remains under the authoritative dataset owner. B03 coordinates the call/result and records evidence; it does not create business rules.

### 5.5 Publication Readiness Gate

Publication requires a complete authoritative prerequisite set established upstream.

The gate verifies that required technical evidence is present for the source-defined conditions, including applicable:

- successful validation;
- authorized review/approval outcome;
- publication policy outcome;
- effective-date eligibility;
- authorization;
- exact Dataset Version identity.

The gate does not infer an approver, fabricate an approval, or silently bypass missing governance.

### 5.6 Atomic Publication Handoff

Where source-defined publication creates a canonical event/fact, B03 uses the B02-003 / ADR-L5-0007 pattern:

```text
Published Version Binding
        +
Durable Publication Intent
        ↓
same owner-local transaction
        ↓
commit
        ↓
C02 durable publication/delivery
```

B03 supplies the upstream-defined event identity/publisher context only where already canonical. C02 owns delivery mechanics after the durable handoff.

### 5.7 Effective-Period Consistency

B03 enforces only the technical consistency needed to prevent an invalid published version-set according to source-defined temporal semantics.

It shall not invent the business meaning or granularity of an effective date.

Where two candidate versions would violate source-defined non-overlap or governing-version rules, publication fails explicitly.

### 5.8 Localized Representations

Localization creates/version-controls a governed localized representation linked to the canonical Dataset and source representation version.

A localized version:

- cannot change canonical Dataset identity;
- cannot change language-independent business meaning;
- follows source-governed validation/approval/publication;
- preserves prior localized versions.

B03 does not select localization libraries or UI rendering behavior.

### 5.9 Distribution Work Registry

For each eligible consumer synchronization outcome required by `L4-A-038`, B03 records technical work identity bound to:

```text
DistributionWork
{
    exact Published Dataset Version
    eligible consumer/subscription reference
    protected scope
    source publication reference
    acknowledgement correlation reference
    technical outcome/provenance
}
```

This is a B03 state/provenance boundary, not the transport queue/broker state owned by C02.

### 5.10 Consumer Eligibility and Delivery Handoff

```text
Published Dataset Version
        ↓
resolve eligible authorized subscription/consumer
        ↓
create/maintain Distribution Work identity
        ↓
C02 delivery boundary
        ↓
consumer-owned processing
        ↓
authoritative acknowledgement
        ↓
B03 records completed governed synchronization outcome
```

A consumer's failure does not modify the source Published Dataset and does not invalidate another consumer's completed outcome.

C02 owns retry/backoff/dead-message/replay mechanics; B03 owns only the source-version/consumer/outcome correlation required by the Master Data synchronization behavior.

### 5.11 Retirement Boundary

When upstream governance validly transitions a Dataset through deprecation/retirement:

- no new ordinary operational consumption shall treat Retired content as current;
- historical Dataset/version identity remains available according to governance/privacy controls;
- source-defined `DatasetDeprecated` / `DatasetRetired` publication uses the same owner-safe B02→C02 handoff where applicable;
- archival is not initiated or owned by B03 merely because retirement occurred.

### 5.12 Failure Model

| Failure | Required Outcome |
|---|---|
| Source/predecessor Dataset Version missing | Reject successor/version operation. |
| Technical or owner-semantic validation fails | Do not advance publication readiness. |
| Approval/governance prerequisite unavailable | Keep candidate outside Published use. |
| Effective-period ambiguity/overlap violates source rules | Reject publication; no implicit winner. |
| Concurrent publication/successor conflict | Surface explicit B02 concurrency conflict; no silent overwrite. |
| Owner-local commit fails | Neither Published binding nor publication intent is committed. |
| C02 unavailable after commit | Preserve Published source and durable intent for later C02 delivery. |
| One consumer delivery/acknowledgement fails | Preserve source and other consumer outcomes; retain governed incomplete/failure position. |
| Duplicate/replayed delivery | C02/consumer idempotency handles transport repetition; B03 correlates to the same source/work identity. |
| Retirement attempted without upstream prerequisites | Preserve current valid lifecycle/publication position. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis | ADR |
|---|---|---|---|---|
| `AD-001` | Version immutability | Published/historical content changes create successor versions; immutable sources are never rewritten. | `L4-A-034/035`; `L2-A09` | None |
| `AD-002` | Validation separation | Technical validation and canonical-owner semantic validation are separate responsibilities coordinated by B03. | Ownership integrity | None |
| `AD-003` | Publication gating | A Dataset Version becomes technically publishable only after all applicable authoritative prerequisites are evidenced. | `L4-A-034` | None |
| `AD-004` | Concurrency | Concurrent successor/publication conflicts use explicit owner-local conflict detection, not last-write-wins. | B02-003 | `ADR-L5-0006` |
| `AD-005` | Publication handoff | Published state and Durable Publication Intent commit atomically; C02 owns delivery afterward. | B02-003 | `ADR-L5-0007` |
| `AD-006` | Distribution isolation | Consumer synchronization is tracked independently per exact Published version/consumer, so one consumer failure does not alter source or other outcomes. | `L4-A-038` | None |
| `AD-007` | Localization identity | Localized versions remain subordinate representations of the same canonical Dataset identity. | `L4-A-039` | None |
| `AD-008` | Retirement boundary | Retirement preserves historical references and does not implicitly execute archival. | `L4-A-040` | None |
| `AD-009` | Technology position | No broker, MDM product, database product or localization product is selected. | ADR methodology | None |

---

## 7. Security, Privacy, Quality & Operational Impact

- Protected Tenant/Institution scope accompanies all version, publication and distribution work.
- Authorization is required for protected mutation/publication/subscription operations; B03 does not own authorization policy.
- Sensitive Dataset payloads and historical copies remain subject to B05/D03 controls.
- Distribution observability includes stuck publication intent, pending consumer work, failed acknowledgement and source-version correlation without inventing numeric targets.
- C02 failure isolation prevents one consumer backlog from blocking unrelated consumers.
- No retry count, timeout, retention period or throughput target is invented here.

---

## 8. Architecture Conformance Requirements

| ID | Architecture Conformance Requirement | Derived From |
|---|---|---|
| `CR-001` | Published/historical Dataset Versions shall not be modified in place. | `AD-001` |
| `CR-002` | Every successor Dataset Version shall preserve predecessor/source lineage. | `AD-001` |
| `CR-003` | Technical validation shall not replace canonical-owner business-semantic validation. | `AD-002` |
| `CR-004` | Publication shall fail when any required validation, approval/governance, policy, effective-date or authorization prerequisite is absent. | `AD-003` |
| `CR-005` | Publication shall bind one exact Dataset Version. | `C-004` |
| `CR-006` | Concurrent successor/publication conflict shall not be resolved by silent overwrite or storage-order choice. | `AD-004` |
| `CR-007` | Where canonical publication is required, Published state and Durable Publication Intent shall commit atomically. | `AD-005` |
| `CR-008` | C02 shall own transport delivery, retry, ordering, acknowledgement transport and replay after the durable B02 handoff. | `AD-005` |
| `CR-009` | B03 shall not invent canonical Dataset event names or publishers. | `C-007` |
| `CR-010` | Distribution work shall remain bound to one exact Published Dataset Version. | `L4-A-038-BH-001` |
| `CR-011` | Consumer synchronization outcomes shall be isolated so one failure cannot mutate source or successful peer-consumer outcomes. | `AD-006` |
| `CR-012` | A completed synchronization outcome requiring acknowledgement shall not be recorded without authoritative consumer acknowledgement. | `C-006` |
| `CR-013` | Localized representations shall preserve canonical Dataset identity and source-governed business meaning. | `AD-007` |
| `CR-014` | Retirement shall preserve historical identity/version references and shall not implicitly perform archival. | `AD-008` |
| `CR-015` | Effective-period technical checks shall preserve source-defined temporal meaning and reject invalid ambiguity. | `C-009` |
| `CR-016` | B03 shall not own bulk/file-transfer mechanics or direct consumer persistence writes. | Boundary |
| `CR-017` | No product-specific broker, MDM or database behavior shall become canonical without applicable ADR authority. | `AD-009` |

---

## 9. Open Decisions & Deferred Design

| ID | Item | Classification | Blocking? | Resolution |
|---|---|---|---|---|
| `OD-002` | Exact operational numeric targets for distribution lag/backlog/recovery are not defined here. | Quality target | No | Consume E02/E05 approved targets when established; do not invent values. |

---

## 10. Traceability Summary

### Primary L4 Relations

- `L4-A-034-BH-001..009` → validation, governance gating, immutable publication and pre-publication access control.
- `L4-A-035-BH-001..009` → immutable source selection, successor lineage, validation and republishing.
- `L4-A-038-BH-001..008` → exact source-version binding, eligible consumers, independent work, acknowledgement and failure isolation.
- `L4-A-039-BH-001..008` → localized version governance without identity/meaning transfer.
- `L4-A-040-BH-001..009` → deprecation/retirement publication and historical preservation with archival explicitly excluded.

### Cross-L5

- B03-001 owns Dataset identity, registration and canonical representation baseline.
- B02 owns persistence, transaction and concurrency mechanics.
- C02 owns event/messaging transport and delivery mechanics.
- D02/D03/B05/D04 supply authorization, trust, privacy and evidence controls without owning Dataset business meaning.

### Review Position

The specification is **Architecture Approved** following full Stage-2 validation at repository baseline `2876bde9994bcd0a9ae17d9fb09c104a7be86d27`. Under the governing Stage-2 validation authorization, the cited canonical L4 Master Data specifications are accepted as behavioral authority for this review. The remaining `OD-002` is non-blocking and intentionally defers unsupported numeric operational targets. The specification is lock-ready but shall not be marked `Locked` until distinct final governance authorization is recorded.

## 2026-09-22 Batch 7A Impact Revalidation Synchronization

Non-material lifecycle synchronization only: the `L5-B-02-003` peer dependency is now identified as `Review`, matching current canonical metadata. No Master Data versioning/publication ownership, architecture semantics, conformance requirement, traceability relation state, or Architecture Approved basis changed.
