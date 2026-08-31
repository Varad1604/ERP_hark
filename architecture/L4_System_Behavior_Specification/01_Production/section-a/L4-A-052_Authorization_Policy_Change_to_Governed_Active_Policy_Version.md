# Authorization Policy Change to Governed Active Policy Version

# 1. Specification Metadata

**Specification ID:** L4-A-052  
**Architecture Level:** L4 — System Behavior Specification  
**Domain:** Authorization & Policy  
**Realizes Scenario:** L3-A-052 — Authorization Policy Change to Governed Active Policy Version  
**Primary Business Owner:** Authorization & Policy  
**Version:** 1.0.0  
**Status:** Draft  
**Owner:** Dhruv  
**Last Updated:** 2026-08-31  
**Depends On:** SutraERP L1 Business Domain Architecture; DEPENDENCY_MAP.md; L2-A01; L2-A04; L2-A06; L2-A07; L3-A-052  
**Referenced By:** None yet

---

# 2. Behavioral Objective & Boundary

Require changed institutional access-control requirements to produce a governed successor Authorization Policy version whose evaluation remains deterministic and explicit-deny preserving, while predecessor versions and every historical Authorization Decision retain immutable version traceability.

This specification defines required observable system behavior only. It does not define APIs, database schemas, software classes, user interfaces, infrastructure, deployment, transport mechanisms, or technical orchestration.

---

# 3. Required Business Context

- Exactly one Tenant and applicable institutional policy scope are resolvable.
- A predecessor policy version exists or an initial governed policy requirement is in scope.
- Role, Permission, SoD, scope and institution-configuration references can be validated.
- Authorization & Policy owns policy versioning and Authorization Decisions; historical decisions must retain the policy version actually evaluated.

---

# 4. System Behavioral Obligations

| Behavior ID | Condition | Required System Behavior | Governed Result | Provenance |
|---|---|---|---|---|
| L4-A-052-BH-001 | An access-control requirement changes. | Resolve Tenant, institutional scope, predecessor policy context and affected authorization consumers. | The successor change boundary is explicit. | L3-A-052 §§7–9; L2-A04 |
| L4-A-052-BH-002 | A successor policy definition is prepared. | Establish immutable predecessor/successor lineage and compose the intended rules, conditions, scope and evaluation order without rewriting the predecessor. | Policy evolution preserves history. | L3-A-052 §§9,12–13; L2-A04 Policy Governance |
| L4-A-052-BH-003 | Successor evaluation semantics are available. | Validate deterministic ordering and explicit-deny precedence before governance progression. | The successor cannot introduce ambiguous results or weaken Deny precedence. | L3-A-052 §§9,12; L2-A04 AP-06 |
| L4-A-052-BH-004 | Successor references and configuration are evaluated. | Validate Tenant isolation, least privilege, Role/Permission references, SoD and permitted institution configuration boundaries. | Invalid policy composition cannot become authoritative. | L3-A-052 §§9,11–12; L2-A04 |
| L4-A-052-BH-005 | Required approval is sought. | Require source-governed approval evidence without naming undefined author, validator, approver, state or approval command. | Governance is mandatory but authority is not invented. | L3-A-052 §§9,15,17; L2-A04 |
| L4-A-052-BH-006 | An approved successor is intended to become the governed active version. | Change the authoritative policy version only through the future source-defined lifecycle/activation contract; do not invent a state name, command or canonical activation event. | No false lifecycle semantics are introduced. | L3-A-052 §§3,9,13–17; L2-A04 |
| L4-A-052-BH-007 | A future protected request is evaluated after a governed active-version change. | Use the authoritative current policy version and bind the resulting Authorization Decision to that exact version. | Future decisions are reproducible by policy version. | L3-A-052 §§9,12,16; L2-A04 |
| L4-A-052-BH-008 | Historical Authorization Decisions already exist. | Preserve every original policy-version reference and predecessor definition without rewrite. | Past authorization outcomes remain reconstructible. | L3-A-052 §§3,9,12–16; L2-A04 |
| L4-A-052-BH-009 | Governance requires rollback to a prior valid policy. | Preserve full version history and use only a future canonical rollback contract; do not invent rollback state, command, event or authority. | Rollback remains governed without destructive history change. | L3-A-052 §§10,17; L2-A04 |

---

# 5. Cross-Domain & Event Behavior

- Platform & Tenant supplies Tenant/institution configuration boundaries.
- Authorization & Policy is canonical owner of policy versions and decisions.
- No canonical successor-created, approved, activated, rolled-back or superseded policy event/publisher is defined.
- Audit Infrastructure preserves policy lineage and decision evidence; human-readable audit activities are not treated as canonical events.

---

# 6. Failure / Rejection / Continuation

- Unresolved scope, nondeterministic order, weakened Deny precedence, invalid references or absent approval → current governed version remains authoritative.
- Interrupted or undefined activation → do not treat successor as active absent an authoritative governed outcome.
- Attempt to rewrite historical policy definitions or decision references → reject.

---

# 7. Audit & Evidence Behavior

Preserve change requirement, Tenant/institution scope, predecessor/successor lineage, rule composition, ordering and Deny validation, references, impact assessment, governance evidence, active-version determination, historical decision-version references, actor, time and correlation.

---

# 8. Observable Completion Conditions

- A successor definition exists without modifying its predecessor.
- Deterministic evaluation, explicit-deny precedence, scope and reference validations succeed.
- Historical policy versions and Authorization Decision references remain immutable.
- Future decisions use the governed authoritative version once a canonical activation contract exists.
- Lifecycle, command, event and authority details remain explicitly deferred rather than inferred.

---

# 9. Traceability & Provenance

- **Primary parent:** `L3-A-052` — Authorization Policy Change to Governed Active Policy Version.
- **Canonical owner:** Authorization & Policy.
- Higher-level ownership, business rules, lifecycle states and canonical event publishers remain governed by the cited L1/L2/L3 sources.
- This L4 specification refines observable system obligations; it does not revise or silently repair upstream architecture.

---

# 10. Open Issues / Deferred Design

| ID | Deferred Design / Upstream Gap | L4 Treatment |
|---|---|---|
| A052-L4-OPEN-01 | Authorization Policy lifecycle states and transition matrix, including supersession/rollback, are undefined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A052-L4-OPEN-02 | Policy creation, validation, approval, activation and rollback commands/workflows are undefined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A052-L4-OPEN-03 | Canonical policy-version events and sole publishers are undefined. | Preserve the gap explicitly; do not invent missing architecture at L4. |
| A052-L4-OPEN-04 | Policy author, validator, approver, activation authority and rollback authority are unnamed. | Preserve the gap explicitly; do not invent missing architecture at L4. |

No implementation design is introduced by these deferred items.
