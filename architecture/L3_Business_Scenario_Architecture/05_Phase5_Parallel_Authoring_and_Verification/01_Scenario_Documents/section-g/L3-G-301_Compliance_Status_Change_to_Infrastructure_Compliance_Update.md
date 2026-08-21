# Compliance Status Change to Infrastructure Compliance Update

# 1. Scenario Metadata

**Scenario ID:** L3-G-301  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-21  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; applicable Platform Foundation authorization, workflow, event and audit contracts; authoritative participating-domain L2 workflows  
**Referenced By:** Authorized upstream and downstream domains requiring this governed outcome

---

# 2. Scenario Purpose

Govern **Compliance Status Change to Infrastructure Compliance Update** from its exact frozen trigger to its exact frozen outcome under Infrastructure Management ownership, preserving all canonical source boundaries and avoiding unsupported actors, states, events or implementation mechanisms.

---

# 3. Business Outcome

The current compliance state is recorded while historical compliance information remains traceable

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a compliance condition changes.

## 4.2 End Boundary

The scenario ends only when the frozen outcome is established by the canonical owner, required references remain traceable, authorized consumers can resolve the result, applicable completed events have been published, and immutable evidence is retained.

## 4.3 In Scope

- Tenant, institution, actor, purpose, scope and correlation resolution.
- `Compliance record` identification or establishment as required by the frozen outcome.
- Source-defined authorization, validation, approval and lifecycle behavior.
- Canonical cross-domain references without ownership transfer.
- Business failure, interruption and governed recovery treatment.
- Exact source-defined event publication where applicable.
- Immutable audit evidence and historical reconstruction.

## 4.4 Out of Scope

- Authentication implementation, UI, API, schema, database, queue or deployment design.
- Direct mutation of another domain's canonical aggregate.
- Consumer-specific decisions or analytics outside the frozen outcome.
- Technical retry or rollback mechanisms.
- Inventing states, commands, events, publishers, roles or approvals absent from L2.

## 4.5 Related Scenarios

| Relationship | Related Scenario Set | Boundary Reason |
|---|---|---|
| Related | Adjacent scenarios owned by Infrastructure Management | They may provide entry facts or consume this outcome without replacing its canonical owner. |
| Platform dependency | Section A authorization, workflow, event and audit scenarios | They supply governed capabilities without owning this business result. |

---

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management | Primary owner of physical infrastructure identity, hierarchy, lifecycle, capacity, availability, reservation and governance. |
| Participating domains `G, A` | Supply or consume only canonical references governed by their own L2 boundaries. |
| Platform Foundation | Supplies Tenant isolation, authorization, workflow, event distribution and audit capabilities. |
| Audit Infrastructure | Retains immutable evidence independently from the business aggregate. |

The frozen participating-domain value remains exactly `G, A`. Assignment to Dhruv does not change domain ownership.

---

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Authority Boundary |
|---|---|---|
| Authorized Initiator | Presents the trigger and required source references. | May initiate only in assigned Tenant, institution and business scope. |
| Infrastructure Management Business Owner | Applies authoritative rules and owns the resulting business fact. | Cannot take ownership of participating-domain source data. |
| Reviewer or Approver, where required | Reviews only a source-defined decision requiring that authority. | No approval role is inferred when L2 does not require one. |
| Participating-Domain Provider | Supplies an authoritative reference or completed input. | Retains canonical ownership. |
| Auditor | Reviews immutable evidence. | Read-only and cannot change the outcome. |

---

# 7. Preconditions and Entry Criteria

- Tenant, institution, actor and business scope are valid.
- The exact frozen trigger is supported by authoritative source facts.
- The `Compliance record` and required references exist or are eligible for establishment under L2-G10 — Infrastructure Management Workflows.
- Applicable policy, configuration, lifecycle and effective-date information is available.
- Required authorization succeeds before mutation.
- Required approval is identifiable where L2 explicitly requires it.
- No unresolved invariant, duplicate outcome or cross-Tenant conflict prevents progression.
- Audit and correlation context is available.

---

# 8. Trigger

A compliance condition changes

---

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the frozen trigger: A compliance condition changes. | Infrastructure Management; applicable participant | Receive the frozen trigger: A compliance condition changes. |\n| 2 | Resolve Tenant, institution, actor, business scope, correlation and every canonical source reference. | Infrastructure Management; applicable participant | Resolve Tenant, institution, actor, business scope, correlation and every canonical source reference. |\n| 3 | Resolve the `Compliance record` and its source-defined lifecycle position without copying participating-domain facts. | Infrastructure Management; applicable participant | Resolve the `Compliance record` and its source-defined lifecycle position without copying participating-domain facts. |\n| 4 | Authorize the requested Infrastructure Management action before business mutation. | Infrastructure Management; applicable participant | Authorize the requested Infrastructure Management action before business mutation. |\n| 5 | Validate eligibility, uniqueness, policy, effective dates, dependency conditions and separation-of-duty constraints. | Infrastructure Management; applicable participant | Validate eligibility, uniqueness, policy, effective dates, dependency conditions and separation-of-duty constraints. |\n| 6 | Apply the authoritative L2-G10 — Infrastructure Management Workflows workflow and rules to the frozen business journey. | Infrastructure Management; applicable participant | Apply the authoritative L2-G10 — Infrastructure Management Workflows workflow and rules to the frozen business journey. |\n| 7 | Record the exact frozen result: The current compliance state is recorded while historical compliance information remains traceable. | Infrastructure Management; applicable participant | Record the exact frozen result: The current compliance state is recorded while historical compliance information remains traceable. |\n| 8 | Expose only the governed result and permitted canonical references through the owning business contract. | Infrastructure Management; applicable participant | Expose only the governed result and permitted canonical references through the owning business contract. |\n| 9 | Publish the applicable source-defined completed event after persistence. | Infrastructure Management; applicable participant | Publish the applicable source-defined completed event after persistence. |\n| 10 | Preserve actor, inputs, decisions, prior and resulting position, event facts and correlation as immutable audit evidence. | Infrastructure Management; applicable participant | Preserve actor, inputs, decisions, prior and resulting position, event facts and correlation as immutable audit evidence. |

---

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Policy-qualified route | Policy permits more than one valid path. | Use only the configured source-defined route. | A valid qualified outcome is recorded. | Infrastructure Management |
| Future-effective result | A valid outcome is effective later. | Preserve the current position until the effective condition. | No premature availability occurs. | Infrastructure Management |
| Source correction | A required fact is corrected by its owner. | Re-resolve the successor fact and continue only after validation. | Ownership and lineage remain intact. | Source owner; Infrastructure Management |
| Mixed result | The outcome permits more than one named result. | Record only the result supported by evidence and policy. | No unsupported success or failure is inferred. | Infrastructure Management |

---

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Tenant, institution or scope is invalid. | The result cannot be established validly. | Reject without cross-scope mutation. | Platform Foundation; Infrastructure Management |
| Canonical reference is absent or inconsistent. | Validation cannot complete. | Obtain correction from its owner; create no shadow fact. | Source owner; Infrastructure Management |
| Authorization or approval fails. | The transition is prohibited. | Stop before mutation and retain denial evidence. | Authorization & Policy; Infrastructure Management |
| Invariant or policy fails. | The requested result is invalid. | Retain the prior authoritative position. | Infrastructure Management |
| Processing is interrupted. | No reliable completed outcome exists. | Resume only through a source-defined recovery path. | Infrastructure Management |
| Success-event condition is absent. | A completed fact cannot truthfully be published. | Publish no success event. | Infrastructure Management; Event Infrastructure |
| Audit evidence cannot be retained. | The result is not governable. | Do not complete the transition. | Infrastructure Management; Audit Infrastructure |

No technical retry, transaction rollback or implementation recovery is defined at L3.

---

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure Management owns the resulting business fact. | Infrastructure Management | No participant may directly rewrite the result. | L2-G10 — Infrastructure Management Workflows |
| Source facts are consumed by reference. | Participating domains | Collaboration never duplicates ownership. | L1; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows |
| Authorization precedes mutation. | Authorization & Policy | Unauthorized work produces no success. | Platform and L2-G10 — Infrastructure Management Workflows rules |
| Validation and required approval precede outcome. | Infrastructure Management | Invalid or unapproved work cannot progress. | L2-G10 — Infrastructure Management Workflows |
| Prior authoritative history remains traceable. | Infrastructure Management; Audit Infrastructure | Correction, replacement or closure never erases evidence. | L2-G10 — Infrastructure Management Workflows; Audit architecture |
| Events represent completed facts. | Infrastructure Management; Event Infrastructure | Incomplete work publishes no success event. | L2-G10 — Infrastructure Management Workflows; Event architecture |
| Tenant and institution isolation are mandatory. | Platform Foundation; Infrastructure Management | Cross-Tenant outcomes are prohibited. | L1; `DEPENDENCY_MAP.md` |

---

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| `Compliance record` | Trigger-supported current condition | Progresses only through the lifecycle defined by L2-G10 — Infrastructure Management Workflows. | The source-defined position supporting the frozen outcome; no new state is invented. | Infrastructure Management |
| Participating source facts | Existing authoritative facts | Referenced and validated. | Unchanged unless their own scenario changes them. | Participating owner |
| Tenant and institution | Existing | Supply isolation and scope. | Unchanged. | Platform / institutional owner |
| Consumer projections | Existing or absent | May reflect the completed fact. | Changed only by their owner. | Applicable consumer |
| Audit evidence | Existing history | Appends immutable evidence. | Prior evidence remains intact. | Audit Infrastructure |

Words such as active, approved, closed, published, restored or archived in the frozen title are not treated as new state names; L2-G10 — Infrastructure Management Workflows remains authoritative.

---

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Event Treatment

| Event / Business Fact | Publication Condition | Publisher / Consumers |
|---|---|---|
| `ComplianceUpdated` | Only after the corresponding source-defined transition succeeds. | The source-defined aggregate publishes; consumers react without taking ownership. |

No additional event or publisher is inferred.

## 14.2 Synchronous Business Contracts and Hand-offs

| Contract / Hand-off | Provider | Consumer | Business Information / Outcome |
|---|---|---|---|
| Tenant and institutional context | Platform / institutional owner | Infrastructure Management | Valid isolation and configuration scope. |
| Canonical source references | Participating domains | Infrastructure Management | Current facts required by the trigger. |
| Governed business service | Infrastructure Management | Authorized initiator | Validation and business outcome. |
| Completed authoritative result | Infrastructure Management | Authorized consumers | The frozen outcome and permitted references. |
| Audit input | Infrastructure Management | Audit Infrastructure | Actor, decisions, states, event and correlation facts. |

---

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization

- Authentication alone does not authorize the transition.
- Authorization is scoped by Tenant, institution, role, permission and resource.
- Participating-domain access does not grant direct primary-aggregate mutation.
- Denial terminates processing before mutation.

## 15.2 Approval and Separation of Duty

- Approval is required only where L2-G10 — Infrastructure Management Workflows or active policy explicitly requires it.
- An approver cannot be inferred from assigned author or canonical owner.
- Separation-of-duty checks remain policy-derived.
- No override bypasses validation or audit.

## 15.3 Audit Obligations

Evidence must include Tenant, institution, scenario and aggregate identifiers, actor, authorization scope, source references, prior and resulting position, policy/configuration version, approvals where required, reason, timestamp, event facts, failures and correlation identifier. Evidence remains immutable.

---

# 16. Completion Criteria and Postconditions

The scenario is complete only when:

- the exact frozen trigger was validated;
- Tenant, institution, actor, scope and canonical references were resolved;
- authorization and every source-defined validation or approval succeeded;
- the `Compliance record` remained under Infrastructure Management ownership;
- the exact frozen outcome was recorded: The current compliance state is recorded while historical compliance information remains traceable;
- participating-domain ownership remained unchanged;
- only applicable source-defined events were published;
- the result became available through governed contracts; and
- immutable audit evidence was preserved.

Postconditions:

- No source fact was duplicated or rewritten by a consumer.
- No unsupported state, actor, approval, event or publisher was introduced.
- Failed or incomplete work is not represented as success.
- Historical and correlation evidence remains reconstructable.

---

# 17. Open Issues, Assumptions and Deferred Questions

**Architecture assessment:** `PASS`

No open issue prevents Architecture Approval. The Draft uses only the frozen semantics and applicable authoritative L2 event vocabulary.

No unsupported actor, aggregate, state, transition, event, publisher, approval authority, cross-domain contract or implementation mechanism is assumed by this Draft.
