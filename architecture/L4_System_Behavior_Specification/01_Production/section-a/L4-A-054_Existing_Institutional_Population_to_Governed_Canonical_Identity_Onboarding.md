# Existing Institutional Population to Governed Canonical Identity Onboarding

# 1. Specification Metadata

**Specification ID:** L4-A-054  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Person & Identity  
**Realizes Scenario:** L3-A-054 — Existing Institutional Population to Governed Canonical Identity Onboarding  
**Primary Business Owner:** Person & Identity  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** SutraERP L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-A01; L2-A02; L2-A06; L2-A07; L3-A-004; L3-A-005; L3-A-054  
**Referenced By:** None yet

---

# 2. Behavioral Objective & Boundary

Require governed onboarding of an institution’s existing population into Tenant-scoped canonical Person identities through source validation, duplicate prevention, normal Person establishment, legacy-identifier mapping, evidenced rejection and stage-appropriate recovery without creating a new migration aggregate or implementation mechanism.

This specification defines required observable system behavior only. It does not define APIs, database schemas, software classes, user interfaces, infrastructure, deployment, transport mechanisms, or technical orchestration.

---

# 3. Required Business Context

- One Tenant/institution scope and an authorized legacy/external source are established.
- Source identifier meaning, mandatory Person attributes and governed transformation mappings are known.
- Canonical Person duplicate-detection rules and Person-establishment rules are available.
- Person & Identity owns canonical Person creation and legacy mappings; source custodians do not acquire Person ownership.

---

# 4. System Behavioral Obligations

| Behavior ID | Condition | Required System Behavior | Governed Result | Provenance |
|---|---|---|---|---|
| L4-A-054-BH-001 | Existing-population onboarding is initiated. | Validate source authority, Tenant/institution scope, source identifier meaning, required attributes and governed transformation rules before canonical creation. | A bounded onboarding population is established. | L3-A-054 §§7–9; L2-A02 |
| L4-A-054-BH-002 | Source records are prepared for intake. | Validate required structure, mandatory attributes, references, Tenant ownership and identity-quality constraints before Person establishment. | Invalid records are evidenced and excluded from canonical creation. | L3-A-054 §§9,11–12; L2-A02 Migration Validation |
| L4-A-054-BH-003 | A source record reaches duplicate assessment. | Apply canonical Tenant-scoped Person duplicate detection and classify it as new, existing-match or unresolved/ambiguous. | One-canonical-Person-per-Tenant invariant is protected. | L3-A-054 §§9,12; L2-A02; L3-A-005 |
| L4-A-054-BH-004 | A duplicate or ambiguity is unresolved. | Exclude the record from new Person creation and route only through governed correction/reconciliation behavior. | No duplicate canonical Person is created. | L3-A-054 §§9–11; L3-A-005 |
| L4-A-054-BH-005 | Validated outcomes are available before execution. | Produce an observable preview of expected creations, links/mappings and rejections and require source-defined administrative approval before canonical changes. | Onboarding consequences are reviewable before execution. | L3-A-054 §§9,15,17; L2-A02 |
| L4-A-054-BH-006 | An approved valid new record is processed. | Establish the Person/Identity only through the normal source-defined Person establishment obligations; do not bypass lifecycle invariants. | Accepted records become canonical identities under the same rules as ordinary establishment. | L3-A-054 §§9,12–14; L3-A-004; L2-A02 |
| L4-A-054-BH-007 | A normal aggregate transition succeeds for an accepted record. | Publish only its applicable canonical aggregate events such as `PersonCreated`, `IdentityCreated`, `PersonVerified`, `IdentityVerified`, `PersonActivated`, or `IdentityActivated` when those actual transitions occur. | Consumers observe completed per-identity facts, not invented migration-job events. | L3-A-054 §14; L2-A02 |
| L4-A-054-BH-008 | A source record is linked to a canonical Person. | Preserve the validated legacy identifier as a traceable mapping to the canonical Person identifier without changing canonical ownership. | Legacy-to-canonical traceability is maintained. | L3-A-054 §§3,9,12–13; L2-A02 |
| L4-A-054-BH-009 | Some records fail while others succeed. | Preserve independent record outcomes, isolate rejected/failed records, and continue only according to the approved source-defined processing policy. | Successful canonical identities remain governed and failures remain actionable. | L3-A-054 §§10–11 |
| L4-A-054-BH-010 | Recovery is required. | Use only the source-defined stage-appropriate cancellation/recovery behavior; after completed migration facts exist, require approved corrective workflows rather than destructive history rewrite. | Recovery preserves canonical history and avoids invented technical rollback semantics. | L3-A-054 §§10–12,17; L2-A02 |

---

# 5. Cross-Domain & Event Behavior

- Platform & Tenant supplies isolation/configuration context; source custodian supplies authorized source records.
- Person & Identity owns duplicate resolution, canonical Person establishment and legacy identifier mappings.
- Only normal Person/Identity aggregate events are canonical for accepted records; no migration-job started/completed/failed/rollback event is defined.
- Audit Infrastructure preserves onboarding, record, approval and mapping evidence.

---

# 6. Failure / Rejection / Continuation

- Invalid source/Tenant, missing mandatory data or unresolved duplicate → reject/exclude affected record.
- Processing interruption → preserve known record outcomes and use only source-governed recovery; do not infer a technical transaction/rollback model.
- Completed onboarding reversal request → use approved corrective business workflows rather than rewriting established history.

---

# 7. Audit & Evidence Behavior

Preserve source and Tenant scope, transformation/validation evidence, duplicate results, preview, approval evidence, each record outcome, canonical identifiers, legacy mappings, rejections, recovery decisions, actors, time and correlation.

---

# 8. Observable Completion Conditions

- Every in-scope record has an observable accepted, linked, rejected or unresolved outcome.
- Only approved valid records establish canonical Persons under normal invariants.
- Legacy identifiers map traceably to canonical identifiers.
- Rejected records and recovery decisions are evidenced.
- No Migration Job lifecycle/event/implementation construct is invented.

---

# 9. Traceability & Provenance

- **Primary parent:** `L3-A-054` — Existing Institutional Population to Governed Canonical Identity Onboarding.
- **Canonical owner:** Person & Identity.
- Higher-level ownership, business rules, lifecycle states and canonical event publishers remain governed by the cited L1/L2/L3 sources.
- This L4 specification refines observable system obligations; it does not revise or silently repair upstream architecture.

---

# 10. Open Issues / Deferred Design

| ID | Deferred Design / Upstream Gap | L4 Treatment |
|---|---|---|
| A054-L4-OPEN-01 | No canonical Migration Job aggregate, lifecycle states, commands or job-level events are defined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A054-L4-OPEN-02 | Stage-dependent recovery exists, but the approved corrective workflow for completed onboarding is not precisely defined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A054-L4-OPEN-03 | Migration/onboarding actor and administrative-approver roles are not formally canonical. | Preserve the gap explicitly; do not invent missing architecture at L4. |

No implementation design is introduced by these deferred items.
