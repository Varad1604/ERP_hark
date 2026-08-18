# Architecture Level Validation Standards

**Version:** 1.1  
**Status:** Locked  
**Owner:** Lead Architect  
**Last Updated:** 2026-08-18  
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
| L4 | System Behavior Specification | Behavior specifications trace to approved L3 scenarios and do not introduce implementation design. |
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

## Phase 3 and Canonical Authoring Gate

Phase 1 discovery is Final / Frozen and Phase 2 consolidation is Final / Closed.

Canonical L3 scenario documents may be authored only after the canonical L3 scenario template is `Locked` and must use that locked template. Final L3 IDs must follow `architecture/L3_Business_Scenario_Architecture/00_Governance/L3_Naming_Numbering_Standard.md` and must not reuse Phase 1 candidate IDs or Phase 2 working IDs.

This policy does not create scenarios, assign scenario IDs, or alter any locked L1/L2 business decision.
