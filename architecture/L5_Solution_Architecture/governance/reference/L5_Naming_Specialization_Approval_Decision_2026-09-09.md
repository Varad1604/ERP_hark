# SutraERP L5 — Naming Specialization Approval Decision

**Version:** 1.0.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Architectural Level:** L5 — Solution Architecture Governance  
**Domain:** Cross-domain  
**Last Updated:** 2026-09-09  
**Depends On:** Repository Constitution Volume 2 v1.2; L5 Major Architecture Areas Baseline; L5 Architecture Family Register; L5 Naming and Document Granularity Specialization  

---

# 1. Decision

The SutraERP L5 naming specialization is approved for canonical L5 Solution Architecture specifications.

The generic repository convention:

```text
L5-[SolutionName]
```

is superseded for canonical SutraERP L5 Solution Architecture specifications by:

```text
Family:                  L5-<Family>
Major Architecture Area: L5-<Family>-<Area>
Canonical Specification: L5-<Family>-<Area>-<Document>
Filename:                L5-<Family>-<Area>-<Document>_<Canonical_Solution_Slug>.md
```

This decision specializes the generic L5 naming rule only. It does not change naming at L0-L4 or L6.

---

# 2. Family Identity

Approved technical architecture families are:

```text
L5-A — Core Platform & Application Architecture
L5-B — Data & Information Architecture
L5-C — Contracts, Events & Integration Architecture
L5-D — Identity, Security & Trust Architecture
L5-E — Reliability, Deployment & Operations Architecture
L5-F — Engineering Delivery & Quality Architecture
L5-G — Client Platform Architecture
```

These letters are L5 technical architecture families and shall not be interpreted as L1 business-domain ownership identifiers.

---

# 3. Numbering Structure

- Major architecture areas use two-digit numbers within a family, for example `L5-A-07`.
- Canonical specifications use three-digit document numbers within an area, for example `L5-A-07-003`.
- Where an area needs multiple specifications, `001` normally represents the baseline specification.
- An area requiring only one specification may use `001` for the complete architecture without creating an artificial parent document.

---

# 4. Identifier Immutability

Once a major-area or specification identifier enters the approved canonical L5 register:

- it shall not be silently renumbered;
- it shall not be reused for a different architecture concern;
- retired identifiers remain retired;
- gaps are permitted.

Planning-draft identifiers may change until the canonical corpus register is formally approved.

---

# 5. Document Granularity

One canonical L5 specification shall describe one coherent technical architecture responsibility that can be independently reviewed and changed without duplicating another specification's decisions.

A split is justified by materially different trust boundaries, technical responsibilities, architecture-decision lifecycles, deployment/operational concerns, dependency profiles, scalability characteristics, compliance requirements, or review scope.

Different names alone do not justify separate documents.

---

# 6. Traceability

L5 shall use many-to-many traceability with L4.

- One L5 specification may realize obligations from many L4 System Behavior Specifications.
- One L4 behavior may depend on multiple L5 specifications.
- L5 IDs shall not inherit L4 scenario numbers merely for numerical alignment.
- L5 shall not redefine upstream business rules, lifecycle states, events, authority or canonical ownership.

---

# 7. Author Independence

Architecture IDs shall never contain author or team-member names. Author assignment is maintained separately in the corpus/workload register.

---

# 8. Corpus Boundary

This naming decision does not approve the current candidate L5 document corpus. Candidate document IDs remain provisional until the full L4-to-L5 coverage and corpus-integrity audit is completed and the corpus register is explicitly approved.

---

# 9. Constitutional Consolidation

Repository Constitution Volume 2 v1.2 still contains the earlier generic `L5-[SolutionName]` notation. This locked decision is the approved L5 specialization and governs canonical L5 Solution Architecture naming until the next controlled consolidation/version update of Volume 2 incorporates the specialization directly.
