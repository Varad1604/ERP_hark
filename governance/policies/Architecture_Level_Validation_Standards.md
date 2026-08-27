# Architecture Level Validation Standards

**Version:** 1.2  
**Status:** Locked  
**Owner:** Lead Architect  
**Last Updated:** 2026-08-26  
**Purpose:** Define repository validation expectations for each approved architecture level without redesigning the architecture model.

---

## Canonical Hierarchy

```text
L0 -> L1 -> L2 -> L3 -> L4 -> L5 -> L6
```

| Level | Name | Validation Expectation |
|---|---|---|
| L0 | Business Vision | Strategic intent, architectural context, constraints, and repository-wide direction are consistent. |
| L1 | Business Domain Architecture | Domain ownership, boundaries, section ownership, and canonical responsibilities are consistent. |
| L2 | Domain Workflow Architecture | Workflow ownership, lifecycle consistency, dependency references, and domain rules remain aligned with L1. |
| L3 | Business Scenario Architecture | Each scenario represents one complete institutional business outcome; preserves L1/L2 ownership, rules, states, lifecycles, events, contracts, and dependency direction; remains implementation independent; and uses the locked canonical L3 scenario template. |
| L4 | System Behavior Specification | Behavior specifications trace to eligible approved L3 scenarios, define observable implementation-independent system obligations, preserve upstream ownership/rules/states/events/authority, and pass the canonical L4 parent-readiness and hard-fail standards. |
| L5 | Solution Architecture | Technical architecture traces to approved L4 behavior and does not redefine business rules. |
| L6 | UI / UX Architecture | UI architecture traces to approved L5 solution architecture and does not own business logic. |

---

## Repository Validation Scope

Validation must confirm:

- folder hierarchy matches the approved level names;
- templates use canonical terminology and dependency direction;
- prompts use Business Scenario and Business Scenario Architecture terminology;
- repository metadata references Section J as Enterprise Resource Management;
- L3 numbering remains independent of L2 numbering;
- L4 naming follows the scenario-aligned specialization in Repository Constitution Volume 2 v1.2;
- relative links and repository navigation remain valid;
- every artifact uses the canonical status vocabulary from `governance/constitution/Volume_2_Repository_Standards.md`;
- no lower architecture level redefines a higher-level decision.

---

## L3 Business Scenario Validation

Every canonical L3 scenario must be validated for the following:

### Scenario integrity

- one complete institutional business outcome;
- clear start boundary and end boundary;
- distinct Business Outcome, Preconditions, Trigger, Completion Criteria, and Postconditions;
- no fragmentation into step-level pseudo-scenarios;
- no uncontrolled expansion across unrelated business outcomes.

### Ownership integrity

- Primary Owner owns the final business outcome under locked L1/L2 architecture;
- participating domains retain their own canonical responsibilities;
- participation, runtime consumption, read models, events, or contracts never transfer ownership;
- no entity, lifecycle, business rule, event, or business fact acquires duplicate ownership.

### Actor and authority integrity

- business actors and roles are supported by authoritative business architecture or recorded as unresolved issues;
- L3 may define business initiation, review, decision, approval, rejection, exception authority, separation-of-duty, and audit/evidence obligations;
- technical RBAC roles, permission codes, claims, IAM groups, ACLs, and enforcement mechanisms are prohibited from L3;
- Section A retains ownership of canonical Authorization and Audit infrastructure even when a business domain owns the underlying business decision, rule, evidence, or outcome.

### Flow and recovery integrity

- main and alternate flows remain at institutional business level;
- alternate business flows are distinguished from failure/interruption/recovery paths;
- failure and recovery content describes business consequences and governed business resolution only;
- server errors, database rollback, queues, retry counts, technical idempotency, exception handlers, transport failures, and infrastructure recovery are prohibited from L3.

### Rule, state, event, and contract integrity

- business rules are referenced from their owning domains and are not re-authored or re-owned by L3;
- lifecycle states and transitions are taken only from authoritative architecture;
- business events retain their canonical name and exactly one publisher;
- published events are kept distinct from synchronous business contracts and hand-offs;
- cross-domain collaboration uses approved business contracts, workflows, read models, or published events as permitted by repository governance.

### Traceability integrity

- significant architecture decisions remain traceable to higher-level sources;
- source references are used where materially necessary to establish ownership, rule, state, event, contract, or disputed behaviour;
- detailed Phase 1 candidate and Phase 2 consolidation lineage remains canonical in the Phase 2 register and is not required to be duplicated inside every L3 scenario document;
- scenario documents include the repository dependency metadata required by the canonical L3 template.

### Architecture-level boundary

L3 must not define:

- APIs or integration protocols;
- database schemas or persistence design;
- software classes, DTOs, services, or implementation components;
- UI screens, forms, buttons, or navigation;
- deployment architecture or infrastructure configuration;
- programming languages, frameworks, or technology choices.

---

## L4 System Behavior Validation

Canonical L4 validation is governed in detail by:

```text
architecture/L4_System_Behavior_Specification/00_Governance/L4_Validation_Standard.md
```

Every L4 specification must additionally satisfy the following cross-level expectations.

### Parent readiness

- parent L3 exists and is currently `Architecture Approved` or `Locked`;
- current parent version is the approved version and has not been materially superseded;
- no unresolved parent conflict prevents behavioral derivation.

### Behavioral integrity

- every BH expresses a real, observable system responsibility;
- every BH has defensible L3 provenance;
- L4 does not mechanically convert every L3 precondition, flow step, actor action, or business rule into system behavior;
- business authority remains business authority; L4 may enforce existence/validity of an authoritative decision without becoming the approver.

### Ownership/rule/state/event integrity

- no canonical business ownership transfers to L4 or to the system;
- no business rule, lifecycle state/transition, event or publisher is invented/renamed/re-owned;
- source-owned facts may be used but not re-owned;
- L4 may not silently repair a material L3 omission directly from L2.

### Architecture-level boundary

L4 must not define:

- APIs, payloads, message schemas or protocols;
- database tables, schemas or persistence design;
- services, classes, DTOs or implementation components;
- queues/brokers, retries, transactions, rollback or exception-handler mechanics;
- infrastructure, deployment, hosting or technology choices;
- technical IAM/RBAC implementation;
- UI screens, forms, buttons, navigation or interaction design.

### Canonical L4 authoring artifacts

New L4 specifications must use:

```text
architecture/L4_System_Behavior_Specification/00_Governance/L4_Governance_and_Architecture_Standard.md
architecture/L4_System_Behavior_Specification/00_Governance/L4_System_Behavior_Specification_Template_v1.0.md
architecture/L4_System_Behavior_Specification/00_Governance/L4_Validation_Standard.md
```

The legacy `templates/L4_SystemBehavior_Template.md` is `Deprecated` and is not a canonical authoring source.

---

## Phase 3 and Canonical L3 Authoring Gate

Phase 1 discovery is Final / Frozen and Phase 2 consolidation is Final / Closed.

Canonical L3 scenario documents may be authored only after the canonical L3 scenario template is `Locked` and must use that locked template. Final L3 IDs must follow `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Naming_Numbering_Standard.md` and must not reuse Phase 1 candidate IDs or Phase 2 working IDs.

This policy does not create scenarios, assign scenario IDs, or alter any locked L1/L2 business decision.

---

## L4 Broad-Authoring Gate

The canonical L4 governance/template/validation baseline being locked does not itself authorize unrestricted mass authoring.

Broad L4 authoring may begin only after:

- the eligible L3 parent baseline is formally confirmed;
- repository navigation/indexes are synchronized;
- pilot artifacts are clearly separated from canonical production specifications;
- execution controls, author/reviewer roles, and batch governance are approved.
