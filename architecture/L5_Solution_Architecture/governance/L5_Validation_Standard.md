# SutraERP L5 Validation Standard

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-09-09  
**Depends On:** Repository Constitution; Architecture Level Validation Standards; L5 Governance and Architecture Standard; L5 Solution Architecture Specification Template v1.0; L5 Naming and Document Granularity Specialization; approved L5 Candidate Document Corpus Register  
**Referenced By:** All L5 architecture reviews, pilot validation, corpus audits and approval decisions  

---

## 1. Purpose

This standard defines mandatory validation expectations for SutraERP L5 Solution Architecture Specifications.

L5 validation uses **hard gates first**.

A specification with a hard failure cannot pass architecture review regardless of writing quality, technical sophistication or numerical score.

Validation determines whether an L5 specification:

```text
makes its claimed portion of approved L4 behavior technically implementable
        +
preserves authoritative upstream architecture
        +
preserves canonical technical ownership
        +
provides sufficient architecture for downstream realization
```

L5 is reusable and many-to-many with L4.

No individual reusable L5 specification is required to satisfy an entire L4 BH where the BH legitimately depends on multiple L5 architectures.

---

## 2. Validation Readiness & Approval Gate

An L5 specification may be authored and reviewed while in `Draft`.

Before it can advance to `Architecture Approved`, confirm:

- its identity exists within the active approved L5 candidate/canonical corpus baseline;
- materially applicable L4 behavior is approved or explicitly governance-authorized;
- its technical responsibility is consistent with its approved corpus boundary;
- material upstream ownership, rule, state, event and authority semantics are sufficiently clear;
- blocking assumptions and unresolved decisions are explicit;
- required ADR or technology decisions are resolved or formally identified as blocking;
- no unresolved conflict with another L5 technical owner remains.

Draft or otherwise unapproved L4 material may support controlled planning.

It shall not silently become immutable L5 behavioral authority.

Where a material behavioral dependency relies solely on unapproved L4 material, the L5 specification shall not receive `Architecture Approved` or `Locked` status unless explicit governance authorizes that dependency.

---

## 3. Hard-Fail Categories

Any applicable hard failure causes architecture-review failure.

### HF-01 — Invalid Behavioral Authority

A specification claims approval while a material behavioral dependency relies only on Draft, superseded or otherwise unapproved L4 behavior without explicit governance authorization.

---

### HF-02 — Unjustified / Orphan Architecture

A material architecture responsibility or decision has no defensible basis in one or more of:

- approved L4 behavior;
- the approved L5 responsibility/corpus boundary;
- an authoritative repository-wide constraint;
- an approved ADR or technical standard;
- a legitimate dependency on another canonical L5 architecture.

Technical complexity is not self-justifying.

---

### HF-03 — Upstream Business-Semantics Corruption

L5 creates, changes, hardens or reinterprets:

- business ownership;
- business rules;
- lifecycle states or transitions;
- canonical event meaning;
- canonical event publisher;
- institutional authority;
- L3 outcome;
- L4 behavioral obligation.

Technical implementation authority does not permit business redesign.

---

### HF-04 — Canonical Ownership Transfer

A service, component, engine, database, index, device, connector or technical family is treated as the canonical owner of an upstream business fact, rule, lifecycle or authority merely because it executes or stores the behavior.

---

### HF-05 — Silent Upstream Repair / Bypass

L5 resolves a material L4 omission or ambiguity by deriving new required behavior directly from L1–L3 rather than raising the issue through architecture governance.

---

### HF-06 — Duplicate Technical Ownership

Two or more L5 specifications claim canonical ownership of the same material technical responsibility or architecture decision.

Consumption and collaboration shall not create shared canonical ownership.

---

### HF-07 — Cross-Boundary Architecture Bypass

A specification permits direct modification of another canonical technical owner's persistence or internal state without an explicitly governed architecture boundary.

Shared infrastructure does not grant ownership or write authority.

---

### HF-08 — Claimed L4 Realization Failure

The architecture cannot satisfy, contradicts or materially weakens the **technical role it claims** for an applicable approved L4 obligation.

A reusable L5 specification is assessed against its claimed contribution, not against unrelated portions of the same BH owned by other L5 specifications.

---

### HF-09 — Ungoverned or Contradictory Material Decision

A material architecture or technology decision:

- bypasses required ADR/selection governance;
- contradicts an approved ADR or technical standard;
- violates an authoritative deployment/security/portability constraint;
- is selected solely from preference or familiarity;
- exceeds the canonical authority of the specification.

Until the L5 ADR methodology is approved, a material technology decision requiring that governance shall remain unresolved rather than being silently finalized.

---

### HF-10 — Unsupported Constraint or Hidden Material Assumption

A preference or assumption is presented as an authoritative constraint or fact.

A material assumption whose falsity could materially change the architecture is hidden, unsupported or silently treated as approved.

---

### HF-11 — Material Architecture Unimplementability

A material omission or ambiguity prevents downstream teams from understanding the required architecture.

Examples include unresolved:

- ownership boundary;
- critical component responsibility;
- material interaction;
- required dependency;
- major technical decision;
- trust boundary.

This hard failure applies only where the omission materially prevents implementation or architecture review.

Minor prose weakness is not sufficient.

---

### HF-12 — Security, Privacy or Trust Violation

The architecture materially violates an authoritative security, privacy or trust requirement, including applicable:

- Tenant/Institution isolation;
- sensitive-data boundaries;
- consent/purpose limitation;
- privileged-access restrictions;
- identity/authorization separation;
- secrets or credentials handling;
- high-integrity trust boundaries.

Security isolation shall not be weakened merely for implementation convenience.

---

### HF-13 — L6 Boundary Violation

L5 makes material prescriptive UI/UX decisions belonging to L6, such as:

- screen or page layouts;
- visual design;
- navigation structure;
- forms;
- user journeys;
- interaction design.

Technical client-platform architecture remains valid L5 content.

---

### HF-14 — Blocking Unresolved Architecture

A material unresolved decision, dependency, assumption, ownership conflict or required ADR remains blocking while the specification claims `Architecture Approved` or `Locked`.

---

### HF-15 — Invalid Canonical Identity or Repository Metadata

The specification materially violates:

- canonical L5 ID;
- family or major-area assignment;
- filename rules;
- status vocabulary;
- versioning;
- technical-owner metadata;
- required repository placement;
- authoritative dependency references.

---

## 4. Architecture Review Checklist

A passing specification shall satisfy all applicable checks.

### 4.1 Level & Authority Integrity

- architecture remains technical;
- business meaning is preserved;
- no L6 interaction design is absorbed;
- historical code and legacy L5 artifacts are not treated as architecture authority;
- upstream ambiguity is escalated rather than silently repaired.

### 4.2 Applicability & Traceability

- applicable L4 specifications and BHs are identified;
- cited BHs genuinely create the technical obligation;
- the specification's claimed role is clear;
- higher levels are used only for authoritative verification or constraints;
- repository-level BH→L5 traceability can be established.

### 4.3 Technical Ownership & Boundary Integrity

- one coherent technical responsibility is owned;
- scope and non-goals are explicit;
- Owns / Consumes / Collaborates / Must Not Own are consistent;
- duplicate ownership is absent;
- another L5 owner's internal architecture is not redefined;
- cross-owner persistence/state bypass is absent.

### 4.4 Solution Completeness

- material technical responsibilities are understandable;
- boundaries, dependencies and interactions are clear;
- applicable technical views are sufficiently complete;
- irrelevant views are not included as filler;
- diagrams and written architecture agree;
- implementation can proceed without inventing major architecture.

### 4.5 Architecture Decision & ADR Integrity

- material decisions are identifiable through `AD-*`;
- rationale is defensible;
- decisions remain within L5 authority;
- complexity is justified;
- technology position is explicit;
- required ADRs are referenced or identified as unresolved;
- ADRs do not override higher authority.

### 4.6 Constraint & Assumption Integrity

- authoritative constraints are distinguished from assumptions;
- preferences are not represented as requirements;
- material assumptions are explicit;
- assumptions have validation or resolution paths;
- architecture impact can be assessed if an assumption proves false.

### 4.7 Security, Privacy & Trust Integrity

Where material:

- Tenant/Institution isolation is preserved;
- authentication and authorization remain distinct;
- privileged access is constrained;
- sensitive information boundaries are respected;
- security/privacy architecture is consumed rather than duplicated;
- high-integrity boundaries receive appropriate logical isolation.

### 4.8 Quality & Operational Integrity

Applicable architecture considers:

- availability;
- resilience;
- performance;
- capacity;
- scalability;
- recoverability;
- observability;
- maintainability;
- portability;
- testability.

Numeric targets are not invented.

Necessary but unavailable targets remain explicitly unresolved.

### 4.9 Conformance Requirement Integrity

Each material `CR-*` shall:

- derive from established L5 architecture;
- remain technical rather than business behavioral;
- be objectively implementation-verifiable;
- preserve upstream behavior;
- trace to an architecture decision, architecture section or dependency;
- avoid becoming a complete testing specification.

A defective CR shall be corrected.

It becomes a hard failure only when its defect causes another applicable HF condition.

### 4.10 Repository Integrity

- locked canonical template is followed;
- required metadata is valid;
- references resolve;
- local `AS-*`, `AD-*`, `CR-*` and `OD-*` identifiers are internally consistent;
- local IDs are not presented as repository-level architecture IDs;
- superseded material is not treated as active authority.

---

## 5. Architecture Decision & Conformance Admission

### 5.1 `AD-*` Admission Test

A material architecture decision shall remain only when:

1. a real technical decision is required;
2. it belongs to this specification's canonical responsibility;
3. it is supported by behavior, constraint or architectural need;
4. it preserves upstream meaning;
5. it does not conflict with another L5 owner;
6. unnecessary complexity has been avoided;
7. ADR requirements are correctly handled;
8. rationale is explicit;
9. downstream implementation can act on the decision.

Otherwise remove, revise, defer or escalate it.

### 5.2 `CR-*` Admission Test

A conformance requirement shall remain only when:

1. it derives from established L5 architecture;
2. it is technical rather than business behavioral;
3. it is objectively verifiable;
4. it preserves upstream behavior;
5. it expresses a meaningful architecture constraint;
6. it is more specific than generic engineering good practice;
7. it avoids unnecessary code-level prescription;
8. implementation/testing can trace to it.

Otherwise remove or revise it.

---

## 6. Diagnostic Scoring — 10 Points

Formal pilot reviews and formal file-level architecture reviews should record a diagnostic score.

The score supports consistency across authors and remediation prioritization.

It shall never override a hard failure.

Use:

```text
1.0 = strong / materially complete
0.5 = partially satisfactory / material improvement required
0.0 = materially deficient
```

For a dimension that is genuinely not applicable, score the specification on whether it correctly identifies the non-applicability or consumes the canonical owner rather than penalizing it for lacking irrelevant design.

| Dimension | Maximum |
|---|---:|
| Upstream authority & L4 traceability | 1.0 |
| Scope & canonical technical ownership | 1.0 |
| Solution completeness & coherence | 1.0 |
| Cross-L5 dependency integrity | 1.0 |
| Architecture decisions & ADR discipline | 1.0 |
| Security, privacy & trust integrity | 1.0 |
| Quality & operational architecture | 1.0 |
| Constraints, assumptions & open decisions | 1.0 |
| Conformance requirements | 1.0 |
| Repository/template integrity | 1.0 |
| **TOTAL** | **10.0** |

Interpretation:

```text
9.0–10.0   Strong architecture
8.0–8.9    Sound architecture; targeted improvements may remain
7.0–7.9    Material remediation required
<7.0       Architecturally weak or incomplete
```

The score is diagnostic.

A high score cannot override a hard failure.

A low score should trigger remediation even when no individual hard failure has yet been identified.

---

## 7. Review Verdicts

**PASS / Architecture Approved**  
Zero hard failures, no blocking issue, architecture materially complete and ownership/traceability defensible.

**PASS WITH NON-BLOCKING FINDINGS**  
Zero hard failures; only minor or non-architectural corrections remain. Findings should normally be resolved before lock.

**RETURN FOR CORRECTION**  
Material L5 defect exists but authoritative upstream architecture is sufficient for L5 remediation.

**BLOCKED BY UPSTREAM**  
Required L4 or higher architecture is absent, unapproved, ambiguous or contradictory and cannot safely be repaired at L5.

**BLOCKED BY ARCHITECTURE DECISION**  
Required ADR, technology decision, cross-L5 ownership resolution or another material technical decision prevents completion.

Only authorized governance may move an approved specification to `Locked`.

---

## 8. Change Revalidation

When material upstream or L5 architecture changes:

1. identify affected L5 specifications through traceability;
2. assess impacted `AS-*`, `AD-*`, `CR-*`, `OD-*` and dependencies;
3. move materially affected specifications to `Review` or another governance-authorized state;
4. revalidate applicable hard gates;
5. resolve dependency conflicts;
6. reapprove before relocking.

Material changes include changes to:

- L4 BHs;
- business ownership/rules/states/events/authority;
- canonical L5 ownership;
- security/privacy constraints;
- ADRs or technical standards;
- deployment constraints;
- cross-L5 contracts;
- material assumptions.

No approved L5 specification shall silently remain unchanged when its authoritative basis materially changes.

---

## 9. Corpus-Level Validation Gate

The currently approved **105-specification register is the active candidate-corpus baseline**, not the final locked L5 corpus.

Before final corpus approval/lock, validate repository-wide:

### Corpus Identity

- all intended final specification IDs are unique;
- every retained candidate exists or has an explicit approved disposition;
- retired IDs are not reused;
- family/area/filename placement is canonical.

### L4→L5 Coverage

- approved L4 obligations requiring technical realization have mapped L5 coverage or explicit approved disposition;
- no material technical obligation is orphaned;
- no retained L5 specification lacks defensible applicability or authoritative technical purpose.

### Canonical Technical Ownership

- every material technical responsibility has exactly one owner;
- no material duplicate ownership remains;
- cross-L5 dependencies preserve boundaries;
- unauthorized cross-owner persistence/state bypass is absent.

### Architecture Decision Consistency

- active ADRs and technical standards are mutually consistent;
- conflicting technology selections are resolved;
- technology neutrality is retained where no governed selection exists.

### Security & High-Integrity Consistency

- Tenant/Institution isolation is repository-consistent;
- IAM, Authorization, Security/Cryptography and Audit responsibilities remain distinct;
- secure computational-engine boundaries remain consistent;
- privacy/consent enforcement does not transfer business ownership.

### Deployment & Operational Consistency

- authoritative deployment constraints are preserved;
- on-premises support is not invalidated by cloud-only assumptions;
- cloud-capable design remains provider-neutral unless explicitly governed otherwise;
- no orchestration/cloud/vendor technology becomes mandatory without an approved decision.

### Open Decisions

- zero blocking `OD-*` items remain in specifications proposed for final approval;
- required ADRs are resolved;
- material assumptions are resolved or formally accepted.

### Repository Integrity

- one canonical L5 governance standard exists;
- one canonical L5 authoring template exists;
- one canonical L5 validation standard exists;
- repository navigation/indexes are synchronized;
- historical L5 artifacts are clearly separated;
- final lock authorization is explicit.

---

## 10. Governance Effect

Locking this validation standard does not authorize broad L5 authoring.

Broad authoring remains subject to completion of the locked L5 governance gate, including:

- approved ADR / technology-selection methodology;
- approved L4→L5 traceability methodology;
- representative controlled pilot;
- resolution of material pilot findings;
- final corpus disposition;
- workload allocation;
- explicit broad-authoring authorization.

---

**Status:** Locked  
**Authority:** Lead Architect — L5 Validation Baseline  
**Broad L5 Authoring:** Not Authorized  
**Final L5 Corpus Lock:** Not Authorized
