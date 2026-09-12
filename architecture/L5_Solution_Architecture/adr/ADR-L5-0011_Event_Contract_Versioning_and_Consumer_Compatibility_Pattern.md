# ADR-L5-0011 — Event Contract Versioning & Consumer Compatibility Pattern

**ADR ID:** `ADR-L5-0011`  
**Title:** Event Contract Versioning & Consumer Compatibility Pattern  
**Scope:** Shared / Family  
**Decision Owner:** L5-C-02 — Event & Messaging Architecture  
**Assigned Author:** Architecture remediation for Pallav Stage-2 corpus  
**Reviewers:** SAOS architecture review; remediation explicitly authorized in controlled architecture session  
**Status:** Architecture Approved  
**Decision Date:** 2026-09-12  
**Affected L5 Specifications:** `L5-C-02-003`; collaborates with `L5-C-02-001`  
**Related ADRs:** None  
**Supersedes:** None  
**Superseded By:** None  

---

## 1. Decision Context

Event contracts are consumed across independently evolving technical consumers. Without a governed family-wide compatibility model, producer changes can silently break active or replaying consumers, while a registry can accidentally be treated as the owner of business meaning. C02 therefore requires an explicit decision covering version visibility, compatible evolution, breaking change handling, coexistence, consumer declarations and retirement evidence.

## 2. Scope & Affected Architecture

This ADR governs technical event-contract evolution and compatibility within C02. It does not create or rename business events, change canonical publisher ownership, redefine event business meaning, select a schema language/serialization format/registry product, or define transport retry behavior.

## 3. Authoritative Constraints

1. Canonical event meaning, publisher authority and business ownership remain upstream.
2. Every durable event contract shall carry an explicit governed contract version.
3. Consumers shall not infer compatibility solely from field presence or undocumented producer behavior.
4. Additive optional evolution is preferred where the existing contract guarantees consumer tolerance.
5. Removal, incompatible type/requiredness change or semantic reinterpretation is breaking.
6. Breaking evolution shall create a distinct breaking successor version; this ADR does not mandate Semantic Versioning or any specific numbering convention.
7. Breaking predecessor and successor versions may coexist during governed migration.
8. Consumers shall explicitly declare or otherwise govern the versions they support.
9. Unsupported breaking versions shall fail safely/quarantine rather than be guessed or coerced.
10. Historical replay shall preserve the original event-contract version.
11. Retirement requires consumer-impact evidence or an approved exception.
12. Registry/catalog capability provides discoverability and automated compatibility control but does not own business semantics.
13. No schema representation, serialization format, envelope standard or registry product is selected by this ADR.

## 4. Decision Criteria

Consumer safety, independent evolution, replay correctness, explicit ownership, product neutrality, controlled migration and evidence-based retirement.

## 5. Alternatives Considered

### A — Unversioned schema with tolerant consumers
Rejected because compatibility becomes implicit and breaking change detection is unreliable.

### B — Global repository version shared by all event contracts
Rejected because unrelated event contracts evolve independently and would become unnecessarily coupled.

### C — Independently versioned event contracts with compatibility classification and governed breaking-successor coexistence
Selected.

### D — Registry owns canonical event semantics
Rejected because it would violate upstream business-event ownership.

## 6. Evaluation

| Criterion | A | B | C | D |
|---|---|---|---|---|
| Consumer safety | Weak | Acceptable | Strong | Acceptable |
| Independent evolution | Weak | Weak | Strong | Acceptable |
| Ownership preservation | Strong | Strong | Strong | Weak |
| Replay compatibility | Weak | Acceptable | Strong | Acceptable |
| Technology neutrality | Strong | Strong | Strong | Weak/Acceptable |

## 7. Decision

SutraERP C02 shall govern each durable event contract independently through explicit version metadata and compatibility classification.

The family-wide pattern is:

1. publisher-owned payload schema and semantic field contract are represented through a governed event contract;
2. compatible additive changes may advance the contract without forcing a breaking migration where consumer guarantees permit;
3. breaking changes create a distinct breaking successor version and require coexistence/deprecation planning;
4. consumers govern supported versions and reject/quarantine unsupported breaking versions safely;
5. producer/consumer rollout and rollback must tolerate the governed coexistence window;
6. historical replay retains the original contract version;
7. retirement requires impact evidence showing affected consumers are migrated or an explicit exception is approved;
8. a shared registry/catalog may enforce discoverability and compatibility rules without taking business ownership.

## 8. Consequences

Positive: predictable producer/consumer evolution, safe replay, visible breaking changes, controlled coexistence and preservation of upstream event ownership.  
Cost: compatibility checks, consumer support declarations, migration windows and retirement evidence must be maintained.

## 9. Risks & Mitigations

| Risk | Mitigation |
|---|---|
| Version numbering is mistaken for SemVer | Define only breaking/non-breaking semantics here; numbering convention remains separately governable. |
| Registry becomes de facto semantic owner | Keep canonical event meaning/publisher authority upstream and explicit. |
| Producer removes fields too early | Require coexistence/deprecation and consumer-impact evidence. |
| Replay cannot be interpreted | Preserve original contract version and supported historical contract access. |
| Product-specific compatibility features become canonical | Keep schema/serialization/registry products unselected. |

## 10. Migration / Exit Position

No schema technology is selected. Future technology choices shall implement these compatibility invariants or explicitly supersede this ADR.

## 11. Traceability

Primary realization: `L5-C-02-003_Event_Schema_Versioning_and_Consumer_Compatibility_Architecture.md`.  
Collaborates with `L5-C-02-001` for event-envelope/version visibility.  
Authoritative L4 behavior: `L4-A-018` and `L4-A-019` under the current L5 validation authority assumption.

## 12. Approval Record

**Current disposition:** Architecture Approved.  
**Approval basis:** The compatibility model is reusable family architecture with material effect on publishers and consumers, satisfying mandatory ADR criteria. Controlled Stage-2 remediation was explicitly authorized on 2026-09-12. This approval does not select serialization, registry or envelope technology and does not imply `Locked` status.

| Role | Decision | Date | Evidence |
|---|---|---|---|
| C02 technical decision owner | Recommended | 2026-09-12 | C02-003 architecture |
| SAOS architecture reviewer | PASS | 2026-09-12 | Pallav Stage-2 production architecture review |
| Architecture remediation authorization | Approved | 2026-09-12 | Explicit user instruction to fix highlighted issues |
