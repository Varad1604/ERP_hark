# Master & Reference Data Technical Architecture

## 1. Specification Metadata

**Specification ID:** `L5-B-03-001`  
**Architecture Level:** L5 — Solution Architecture  
**Family:** L5-B — Data & Information Architecture  
**Major Area:** L5-B-03 — Master & Reference Data Technical Architecture  
**Canonical Technical Responsibility:** Technical representation and distribution boundary for governed Master Data and Reference Data while `L2-A09` and the applicable business domains retain governance and business meaning.  
**Technical Architecture Owner:** L5-B-03 — Master & Reference Data Technical Architecture  
**Assigned Author:** Dhruv  
**Version:** `1.0.1`  
**Status:** `Locked`  
**Last Updated:** `2026-09-22`  
**Depends On:** L5 Governance and Architecture Standard v0.3.1; L5 Solution Architecture Specification Template v1.0.3; L5 Validation Standard v1.0.1; L5 Candidate Document Corpus Register v1.1.0; L5 L4-to-L5 Traceability Methodology v1.0.0; L5 Technology Selection & ADR Methodology v1.0.1; L5 ADR Register v1.0.0; `L5-B-02-001`; `L5-B-02-002`; `L2-A09` v1.0 Locked; applicable L4 Master Data specifications identified in §3.  
**Referenced By:** `L5-B-03-002` and L5 consumers requiring governed Master/Reference Data discovery or consumption.  
**Applicable ADRs:** `ADR-L5-0005` — Persistence Ownership, Hybrid Topology and Tenant Isolation Pattern. No Master Data product/platform technology is selected by this specification.

---

## 2. Purpose, Scope & Non-Goals

### 2.1 Purpose

This specification defines the common technical architecture through which SutraERP represents, registers, discovers and exposes governed Master Data and Reference Data without transferring business meaning or business ownership to the technical platform.

It establishes the baseline technical boundary consumed by `L5-B-03-002` for versioning, validation, publication and distribution.

### 2.2 In Scope

- stable technical Dataset identity and registration boundary;
- technical distinction between Master Data, Reference Data and Configuration Data;
- canonical technical representation boundary independent of physical storage;
- owner/steward/governance metadata references required for technical enforcement and discovery;
- Dataset Technical Descriptor and representation-version binding;
- Tenant/scope-aware Dataset registry and discovery;
- governed read/consumption boundary;
- prevention of cross-owner persistence bypass;
- immutable historical identity/reference preservation;
- handoff to B03-002 for governed version lifecycle and publication.

### 2.3 Out of Scope / Non-Goals

- business meaning, business constraints or authoritative dataset content ownership — owning business domains / `L2-A09` governance;
- Dataset lifecycle-state definitions, approval authorities or canonical event semantics — upstream L2/L4;
- detailed versioning, validation execution, publication, localization and synchronization — `L5-B-03-002`;
- configuration hierarchy, runtime settings or feature configuration — `L5-A-06-*`;
- persistence technology, physical schema, transaction or concurrency technology — `L5-B-02-*`;
- API/wire protocol mechanics — `L5-C-01-*`;
- event transport, retry, acknowledgement, ordering or replay — `L5-C-02-*`;
- bulk file transfer, staging, import/export packaging or external file exchange — `L5-C-03-008`;
- authorization policy — `L5-D-02-*`;
- audit infrastructure — `L5-D-04-*`;
- retention/archival/privacy policy enforcement — `L5-B-05-*`;
- search/index ownership — `L5-B-01-*`;
- UI/UX — L6.

---

## 3. Upstream Applicability, Constraints & Assumptions

### 3.1 Canonical Upstream Reference Matrix

| Source Level | Document / Decision | Version / Status | Exact Reference | What the Source Establishes | L5-B-03-001 Use |
|---|---|---|---|---|---|
| L2 | `L2-A09` | v1.0 / Locked | §§1.1–1.5, 3, 7–10, 22.2, 23, 27, 54–57 | Master/Reference Data platform governs common lifecycle/governance infrastructure while business domains retain meaning; canonical representation, stable identity, discovery, versioning, contracts and events are governed. | Establishes baseline platform responsibility and non-ownership boundary. |
| L4 | `L4-A-033` | v1.0.0 / Draft on `main` | `BH-001`–`BH-008` | Dataset onboarding requires resolved Tenant/owner, stable identity, governance metadata, Draft registration, discoverability and no ordinary consumer access to unpublished content. | Drives Dataset Registry, identity, descriptor and discovery boundaries. |
| L4 | `L4-A-034` | v1.0.0 / Draft on `main` | `BH-001`, `BH-008`, `BH-009` | Dataset content must be validated before governed use; non-published content is not ordinary-consumer data; evidence remains traceable. | Drives representation/consumption boundary used by B03-002 publication. |
| L4 | `L4-A-035` | v1.0.0 / Draft on `main` | `BH-001`, `BH-003`, `BH-008`, `BH-009` | Historical source versions remain immutable and successor lineage is preserved. | Requires stable Dataset identity and historical reference integrity. |
| L4 | `L4-A-039` | v1.0.0 / Draft on `main` | `BH-002`, `BH-006`, `BH-008` | Localization shall preserve canonical identifiers/business meaning and historical localized versions. | Requires canonical identity to remain independent of localized representations. |
| L4 | `L4-A-040` | v1.0.0 / Draft on `main` | `BH-005`, `BH-006`, `BH-008`, `BH-009` | Retired Dataset information remains read-only/historical and archival is a separate outcome. | Requires historical registry/discovery without absorbing archival ownership. |
| L5 Corpus | `L5-B-03-001` | Frozen 105-specification baseline | Canonical responsibility | Technical representation/distribution of governed Master/Reference Data while L2-A09 retains business governance. | Defines the exact B03-001 responsibility boundary. |
| L5 Peer | `L5-B-02-001/002` | Review | Owner-scoped persistence and isolation | Authoritative persistence remains owner-scoped; direct cross-owner persistence is prohibited. | Constrains Dataset technical state and access. |

### 3.2 Authoritative Constraints

| ID | Constraint | Basis | L5 Effect |
|---|---|---|---|
| `C-001` | Every governed Dataset has one authoritative business owner. | `L2-A09`; `L4-A-033` | B03 records/binds owner identity but never acquires business meaning. |
| `C-002` | A Dataset obtains a stable identity only after source-defined registration conditions succeed. | `L4-A-033-BH-001/003` | Dataset Registry rejects ambiguous or duplicate registration. |
| `C-003` | Ordinary consumers cannot consume unpublished Draft/Review/Approved content. | `L4-A-033-BH-007`; `L4-A-034-BH-008` | Consumption boundary admits only source-governed consumable versions. |
| `C-004` | Published/historical Dataset identity and lineage shall remain traceable. | `L2-A09` §§8–10; `L4-A-035` | Stable identity is independent of mutable representation. |
| `C-005` | Configuration Data is not Master/Reference Data. | `L2-A09` §3.4 | Runtime configuration remains A06-owned. |
| `C-006` | Canonical representation is not a license for direct shared-database coupling. | `L2-A09`; B02 | Consumers use governed contracts/events/read boundaries. |
| `C-007` | Tenant/Institution isolation is preserved across registry, discovery and consumption. | `L2-A09`; B02/A04 | Cross-Tenant discovery/access fails closed. |
| `C-008` | Archival is not owned by B03 merely because retired metadata remains discoverable. | `L4-A-040-BH-008` | B03 preserves historical references; archival remains separate. |

### 3.3 Assumptions

No material architecture assumption is required for this product-neutral baseline.

---

## 4. Technical Ownership, Boundaries & Dependencies

### 4.1 Owns

`L5-B-03-001` owns:

- **Dataset Registry** technical boundary;
- stable Dataset technical identity binding;
- **Dataset Technical Descriptor** semantics;
- **Canonical Representation Boundary**;
- technical metadata catalog/discovery boundary;
- Master/Reference/Configuration classification enforcement boundary;
- Dataset owner/scope/governance-reference binding;
- governed Dataset read/consumption boundary;
- historical Dataset identity and representation-reference preservation baseline.

### 4.2 Consumes / Collaborates With

| L5 Specification | Capability | Boundary |
|---|---|---|
| `L5-B-02-001/002` | owner-scoped persistence and isolation | B03 state remains within the canonical Master Data technical owner boundary; consumers do not bypass it. |
| `L5-B-03-002` | versioning/validation/publication/distribution | B03-001 supplies identity, descriptor and canonical-representation baseline. |
| `L5-A-04-*` | Tenant/Institution scope | B03 consumes validated protected scope. |
| `L5-C-01-*` | synchronous contracts | Realizes public Dataset discovery/read contracts where synchronous consumption is used. |
| `L5-C-02-*` | event/messaging | Delivers canonical upstream-defined Dataset events; B03 does not own transport. |
| `L5-D-02-*` | authorization | Supplies access decisions for registration/discovery/consumption. |
| `L5-D-04-*` | audit/evidence | Preserves technical evidence without becoming Dataset owner. |
| `L5-B-05-*` | privacy/retention | Applies classification/retention/privacy enforcement where applicable. |
| `L5-C-03-008` | bulk/file exchange | Owns transfer/staging/packaging; accepted Dataset content enters B03 governance only through a governed handoff. |

### 4.3 Must Not Own

B03-001 shall not own business semantics, business rules, approval authority, configuration meaning, physical persistence technology, event delivery, authorization policy, bulk-transfer mechanics, retention policy, archival workflow or UI.

---

## 5. Solution Architecture

### 5.1 Baseline Model

```text
Owning business domain / L2-A09 governance
                │
                ▼
       Dataset Registration Boundary
                │
                ▼
           Dataset Registry
                │
       ┌────────┴────────┐
       ▼                 ▼
Dataset Technical   Governance / Discovery
Descriptor          Metadata Catalog
       │                 │
       └────────┬────────┘
                ▼
   Canonical Representation Boundary
                │
                ▼
             B03-002
 version / validation / publication
                │
                ▼
 Governed Consumption / Distribution
```

B02 owns physical persistence and owner/Tenant isolation beneath these logical B03 elements.

### 5.2 Dataset Registration Boundary

Registration is admitted only when the source-governed prerequisites can be established, including:

- protected Tenant/scope where applicable;
- one authoritative business owner;
- required stewardship/governance references;
- source-defined Dataset category and uniqueness context;
- required technical metadata.

The boundary does not decide who the business owner or steward should be. It verifies and binds authoritative upstream outcomes.

A rejected registration shall not create a usable Dataset identity.

### 5.3 Dataset Registry

The Dataset Registry is the technical system-of-record for Dataset registration identity and technical governance references; it is not the business owner of the Dataset content.

The registry maintains architecture-level information categories sufficient to establish:

```text
Dataset Technical Descriptor
{
    stable Dataset identity
    Master / Reference category
    authoritative business-owner reference
    protected scope / Tenant context where applicable
    governance / steward references
    canonical-representation definition reference
    current governed lifecycle/version references
    classification / discovery metadata references
}
```

This is a conceptual architecture contract, not a database schema.

### 5.4 Stable Identity

A Dataset identity is stable across later version, localization, deprecation and retirement activity.

A version/localization identity is not permitted to replace or mutate the canonical Dataset identity.

Business keys remain owned by the applicable business domain. Any technical surrogate used by implementation shall not replace business-defined identifiers where business meaning is required.

### 5.5 Master / Reference / Configuration Classification Boundary

B03 accepts only source-governed Master Data or Reference Data responsibilities.

```text
Master Data
→ long-lived governed shared business entities

Reference Data
→ governed reusable code/value sets

Configuration Data
→ runtime/settings/feature/operational configuration
→ L5-A-06, not B03
```

When the category cannot be resolved without ambiguity, B03 shall reject registration rather than absorb the concern by convenience.

### 5.6 Canonical Representation Boundary

B03 defines a technology-independent representation contract through which a governed Dataset can be discovered and consumed without exposing its physical persistence model.

The representation contract binds:

- Dataset identity;
- representation/version identity;
- authoritative owner;
- protected scope;
- compatibility/definition reference;
- governed content categories;
- provenance needed to identify the published source.

Exact business attributes and their semantics remain owner-defined. Exact wire schemas belong to C01 and physical storage schemas to B02/implementation.

### 5.7 Governance & Discovery Metadata Catalog

Metadata required for enterprise discovery is exposed separately from unrestricted Dataset content.

Discovery metadata may identify owner, Dataset category, lifecycle/publication position, version references, classification and governance references where source architecture permits them.

Sensitive metadata remains subject to authorization/privacy controls; discoverability never implies content access.

### 5.8 Governed Consumption Boundary

Ordinary consumers may obtain only an upstream-authorized consumable Dataset version through a public B03 boundary.

Consumers shall not:

- read B03 authoritative persistence directly;
- mutate the Dataset through a read contract;
- infer publication from registration alone;
- substitute a local copy as the new enterprise authority;
- consume another Tenant's Dataset through missing or caller-supplied scope alone.

Consumer-local caches/projections remain derived and non-authoritative.

### 5.9 Historical Registry Position

When a Dataset/version becomes historical or Retired, B03 retains the registry/identity references needed for governed historical lookup and evidence.

This does not execute archival, retention or deletion. Those responsibilities remain with their canonical owners.

### 5.10 Failure Model

| Failure | Required Outcome |
|---|---|
| Tenant/scope cannot be established | Reject protected registration/access. |
| Dataset owner is unresolved or ambiguous | Reject registration/change; do not assign ownership in B03. |
| Dataset identity/name uniqueness fails | Do not create conflicting canonical Dataset identity. |
| Master/Reference/Configuration category is ambiguous | Reject classification; do not absorb configuration into B03. |
| Consumer attempts unpublished content access | Reject. |
| Consumer attempts direct B03 persistence access | Reject and use governed contract/event boundary. |
| Historical identity/reference cannot be preserved | Prevent destructive replacement; preserve last valid governed state. |
| Cross-Tenant discovery/consumption attempt | Fail closed. |

---

## 6. Architecture & Technology Decisions

| Decision ID | Decision | Selected Architecture / Position | Basis | ADR |
|---|---|---|---|---|
| `AD-001` | Dataset identity | Stable Dataset identity is independent of Dataset Version/localization identities. | `L2-A09` §8; `L4-A-033`; `L4-A-039` | None |
| `AD-002` | Representation boundary | Consumers use a canonical technical representation boundary rather than B03 physical storage. | `L2-A09` §§8, 57; B02 ownership | None |
| `AD-003` | Ownership binding | B03 binds authoritative owner/governance references but does not own business meaning. | `L2-A09` §§3, 5–6; `L4-A-033` | None |
| `AD-004` | Configuration exclusion | Runtime/feature/operational configuration remains A06-owned even when reused across domains. | `L2-A09` §3.4 | None |
| `AD-005` | Persistence boundary | B03 authoritative state remains owner-scoped and inaccessible through direct cross-owner persistence. | B02-001/002 | `ADR-L5-0005` |
| `AD-006` | Technology position | No MDM product, database product, schema technology or integration product is selected here. | ADR methodology | None |

---

## 7. Security, Privacy, Quality & Operational Impact

- Tenant/Institution context is enforced before protected registry, discovery or content access.
- Metadata discoverability is authorization-aware; discovery does not grant content access.
- Business-domain ownership and technical persistence authority remain distinct.
- Sensitive Dataset content/metadata consumes B05/D02/D03 controls where applicable.
- B03 logical elements do not require separate processes/databases/deployment units.
- No performance, retention, retry or availability numeric target is invented.

---

## 8. Architecture Conformance Requirements

| ID | Architecture Conformance Requirement | Derived From |
|---|---|---|
| `CR-001` | Every registered Dataset shall bind exactly one authoritative business-owner reference. | `C-001`, `AD-003` |
| `CR-002` | A stable Dataset identity shall not be created when required uniqueness/scope/ownership prerequisites fail. | `L4-A-033-BH-001/003` |
| `CR-003` | Dataset identity shall remain stable across version, localization, deprecation and retirement activity. | `AD-001` |
| `CR-004` | Master Data, Reference Data and Configuration Data shall remain technically distinguishable; configuration shall not be absorbed into B03. | `AD-004` |
| `CR-005` | Canonical representation shall not expose B03 physical persistence as the integration contract. | `AD-002/005` |
| `CR-006` | Ordinary consumers shall not receive unpublished Dataset content. | `L4-A-033-BH-007`; `L4-A-034-BH-008` |
| `CR-007` | Consumer access shall preserve protected Tenant/Institution scope. | `C-007` |
| `CR-008` | Direct cross-owner B03 persistence access shall not be a supported collaboration path. | B02 / `ADR-L5-0005` |
| `CR-009` | Consumer-local copies/projections shall remain derived and shall not silently become authoritative Dataset sources. | Ownership boundary |
| `CR-010` | Discovery metadata shall not imply authorization to consume Dataset content. | Security boundary |
| `CR-011` | Historical Dataset identity and representation references shall remain traceable after successor publication or retirement. | `L4-A-035`; `L4-A-040` |
| `CR-012` | B03-001 shall delegate versioning/validation/publication/distribution mechanics to B03-002. | Frozen corpus boundary |
| `CR-013` | Bulk/file transfer mechanics shall remain outside B03 and use the governed C03-008 boundary when applicable. | Frozen corpus boundary |
| `CR-014` | No product/vendor-specific MDM or persistence behavior shall become canonical without applicable ADR authority. | `AD-006` |

---

## 9. Open Decisions & Deferred Design

No blocking open decision remains under the current Stage-2 validation program. Product selection, physical schema and wire schemas are intentionally delegated to their canonical owners and are not B03 blockers.

---

## 10. Traceability Summary

### Primary L4 Relations

- `L4-A-033-BH-001..008` → Dataset registration, stable identity, governance metadata, discoverability and non-public Draft boundary.
- `L4-A-034-BH-001/BH-008/BH-009` → validation/consumption/evidence baseline consumed by B03-002.
- `L4-A-035-BH-001/BH-003/BH-008/BH-009` → immutable source identity and successor/historical lineage baseline.
- `L4-A-039-BH-002/BH-006/BH-008` → canonical identity preservation across localized versions.
- `L4-A-040-BH-005/BH-006/BH-008/BH-009` → historical read-only registry position and explicit archival boundary.

### Cross-L5

- B02 owns persistence placement, isolation and transaction mechanics.
- B03-002 owns Dataset versioning, validation, publication and distribution.
- C01/C02 own contract and messaging transport mechanics.
- A06 owns configuration.
- C03-008 owns bulk/file transfer mechanics.
- B05/D02/D03/D04 supply privacy, authorization, trust and evidence controls without acquiring Dataset business ownership.

### Review Position

The specification is **Architecture Approved** following full Stage-2 validation at repository baseline `2876bde9994bcd0a9ae17d9fb09c104a7be86d27`. Under the governing Stage-2 validation authorization, the cited canonical L4 Master Data specifications are accepted as behavioral authority for this review. The specification is lock-ready but shall not be marked `Locked` until distinct final governance authorization is recorded.

## 2026-09-22 Batch 7A Impact Revalidation Synchronization

Non-material lifecycle synchronization only: the `L5-B-02-001/002` peer dependency is now identified as `Review`, matching current canonical metadata. No Master Data ownership, architecture semantics, conformance requirement, traceability relation state, or Architecture Approved basis changed.
