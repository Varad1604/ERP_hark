# SutraERP L3 Business Scenario Template v2.0

**Document:** L3 Business Scenario Template  
**Architecture Level:** L3 — Business Scenario Architecture  
**Version:** 2.0  
**Status:** Proposed Final — Phase 3  
**Final Lock Status:** Pending Architecture Review  
**Applies To:** Canonical L3 business scenario documents created during canonical scenario set construction  
**Design Baseline:** `L3_Phase3_Canonical_Template_Design_Decision_2026-08-18.md`  
**Predecessor:** `L3_Business_Scenario_Template_v1.0.md` — retained as the historical pilot template

---

## 1. Template Purpose

This template defines the proposed final structure for SutraERP Level 3 Business Scenario Architecture documents.

An L3 business scenario describes one complete institutional business outcome across one or more participating business domains while preserving canonical ownership, lifecycle, event, rule, and dependency boundaries established upstream.

The template is designed to make L3 sufficiently precise for later implementation architecture without turning L3 itself into implementation architecture.

---

## 2. Architecture Level Boundary

L3 describes institutional business behaviour and business collaboration.

L3 must remain implementation independent.

L3 may define:

- business actors and business roles;
- business responsibilities;
- business authority and approval obligations;
- scenario entry conditions and trigger;
- end-to-end business flow;
- valid alternate paths;
- business-level failure, interruption, and recovery;
- applicable business rules and decision constraints;
- lifecycle and state impact already defined by authoritative architecture;
- published and consumed business events already defined by authoritative architecture;
- synchronous business contracts and cross-domain hand-offs;
- business audit and evidence obligations;
- completion criteria and postconditions.

L3 must not define:

- API endpoints or integration protocols;
- database schemas or storage design;
- software classes, DTOs, services, or components;
- UI screens, buttons, forms, or navigation;
- technical RBAC roles, permission codes, IAM groups, claims, or enforcement mechanisms;
- deployment architecture or infrastructure configuration;
- retry policies, queue behaviour, transaction handling, server failures, or other technical recovery mechanics;
- programming languages, frameworks, or technology choices.

Technical RBAC and access-control design is derived later from L3 business roles, responsibilities, authority, approval, separation-of-duty, and audit obligations.

---

## 3. Scenario Document Naming

Final L3 scenario files must follow `L3_Naming_Numbering_Standard.md`.

```text
L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md
```

Example:

```text
L3-C-001_Approved_Candidate_to_Active_Student.md
```

Final L3 numbering remains independent of L2 numbering.

Detailed Phase 1 and Phase 2 consolidation lineage is maintained in the Phase 2 canonical scenario register and is not duplicated as a mandatory traceability structure inside each final scenario document.

---

## 4. Canonical Scenario Document Structure

Each canonical L3 scenario document must use the following structure:

```text
1. Scenario Metadata
2. Scenario Purpose
3. Business Outcome
4. Scenario Boundary and Scope
5. Ownership and Participating Domains
6. Business Actors, Roles and Responsibilities
7. Preconditions and Entry Criteria
8. Trigger
9. Main Business Flow
10. Alternate Business Flows
11. Failure, Interruption and Business Recovery
12. Business Rules and Decision Constraints
13. State and Lifecycle Impact
14. Business Events and Cross-Domain Collaboration
15. Business Authorization, Approval and Audit Obligations
16. Completion Criteria and Postconditions
17. Open Issues, Assumptions and Deferred Questions
```

---

# 1. Scenario Metadata

```markdown
# <Scenario Title>

**Scenario ID:** L3-<PRIMARY_OWNER_SECTION>-<NNN>  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** <Canonical owning domain / section>  
**Participating Domains:** <Actual participating domains>  
**Scenario Type:** Simple / Cross-domain / Critical  
**Priority:** P0 / P1 / P2  
**Status:** Draft / Review / Approved / Locked  
**Version:** 1.0  
```

## Metadata Rules

- Primary Owner must own the final business outcome under authoritative L1/L2 architecture.
- Participating Domains must include only domains that materially participate.
- Scenario Type must use only the governed values defined by `L3_Scenario_Type_and_Priority_Rules.md`.
- Priority must use only the governed values defined by `L3_Scenario_Type_and_Priority_Rules.md`.
- Phase 1 candidate IDs and Phase 2 working IDs are not mandatory metadata in final scenario documents; consolidation lineage remains in the Phase 2 canonical scenario register.
- Metadata must not imply ownership that conflicts with L1 or L2.

---

# 2. Scenario Purpose

Describe why the scenario exists and the institutional business need it serves.

The purpose must:

- remain outcome-oriented;
- describe business need rather than implementation mechanics;
- avoid restating the full L2 workflow;
- avoid introducing new scope not supported by authoritative architecture.

---

# 3. Business Outcome

Describe the final institutional business outcome that represents successful scenario completion.

The outcome must be:

- singular and clear;
- institutionally meaningful;
- business-verifiable;
- consistent with the Primary Owner's canonical responsibility;
- distinguishable from a workflow step or technical result.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

State the business boundary at which this L3 scenario begins.

Do not confuse the start boundary with a technical invocation.

## 4.2 End Boundary

State the business boundary at which this L3 scenario ends.

The end boundary must align with the Business Outcome and Completion Criteria.

## 4.3 In Scope

List the business behaviour covered by this scenario.

## 4.4 Out of Scope

List materially adjacent business behaviour that is intentionally excluded, especially behaviour owned by another scenario or architecture level.

Implementation design is always out of scope and need not be repeated item by item unless a specific boundary risk exists.

## 4.5 Related Scenarios — Optional

Use this subsection only when a known canonical L3 scenario is materially adjacent to the current scenario.

```markdown
| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by / Followed by / Related | <L3 ID and title> | <Business relationship> |
```

Do not invent placeholder L3 IDs. Leave this subsection empty or state `Not yet assigned` until canonical related scenario IDs exist.

---

# 5. Ownership and Participating Domains

Identify the canonical owner of the final outcome and the responsibility of every materially participating domain.

```markdown
| Domain / Section | Responsibility in Scenario |
|---|---|
| <Primary Owner> | <Owned responsibility leading to final business outcome> |
| <Participating Domain> | <Owned contribution, business fact, workflow responsibility, or decision supplied to the scenario> |
```

Rules:

- Every responsibility must preserve authoritative L1/L2 ownership.
- Participation does not transfer ownership.
- A domain consuming another domain's business fact does not become owner of that fact.
- A runtime dependency does not reverse canonical ownership.
- Do not list a supporting domain merely because its platform capability may be used technically.

---

# 6. Business Actors, Roles and Responsibilities

Identify the institutional actors or business roles that participate in the scenario and what each is responsible for.

These are business-level roles. They are authoritative inputs for later access-control and RBAC design, but this section must not define technical permissions.

```markdown
| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| <Actor or role> | <Business responsibility> | <Initiates / reviews / decides / approves / performs / receives outcome, as supported> |
```

Rules:

- Use actors or roles supported by authoritative business architecture.
- Do not invent organizational roles merely to make the table complete.
- If authoritative L2 sources do not define a required actor or role, record the gap in Section 17 rather than guessing.
- Do not define permission names, application roles, IAM groups, claims, or access-control implementation.

---

# 7. Preconditions and Entry Criteria

List business facts or conditions that must already be true before the scenario may validly begin.

```markdown
- <Business precondition or entry criterion>
```

Rules:

- Preconditions are established business facts, not technical availability checks.
- Do not state the trigger as a precondition.
- Do not state the final outcome as a precondition.
- Cross-domain preconditions must be described as business facts supplied by their canonical owners.

---

# 8. Trigger

Describe the business event, decision, request, condition, or lifecycle occurrence that starts the scenario.

```text
<Business-readable trigger>
```

The trigger must be distinct from Preconditions and must not be expressed as an API call, database write, button click, scheduled job, or other technical mechanism.

---

# 9. Main Business Flow

Describe the normal successful end-to-end business path.

```markdown
| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | <Business action or decision> | <Role / Domain> | <Result or hand-off> |
```

Rules:

- Keep every step at institutional business level.
- Show the responsible business role and/or canonical domain where material.
- Do not decompose L2 workflows into unnecessary micro-steps.
- Do not redefine upstream business rules, states, or events.
- Do not describe screens, APIs, tables, jobs, services, classes, or technical orchestration.
- Add an authoritative source reference only where a material behaviour, ownership boundary, rule, state, event, or disputed interpretation requires evidence. Do not add a source column merely for blanket traceability.

---

# 10. Alternate Business Flows

Describe valid business paths that differ from the Main Business Flow while remaining legitimate paths through the scenario.

```markdown
| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| <Case> | <Business condition> | <Alternative path> | <Business result> | <Domain> |
```

An alternate flow is not a technical error and is not automatically a failure.

Use this section for governed alternatives such as different valid decisions, eligibility routes, approval paths, or institutional variants when those alternatives remain inside the same canonical scenario boundary.

---

# 11. Failure, Interruption and Business Recovery

Describe business-level situations in which normal scenario progression cannot continue, together with the governed business resolution or recovery path where one exists.

```markdown
| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| <Failure or interruption> | <Business impact> | <Recovery, correction, reversal, suspension, rejection, resubmission, or terminal outcome> | <Canonical owner> |
```

Rules:

- Model only business-level failure, interruption, correction, reversal, rejection, suspension, resubmission, or recovery supported by authoritative architecture.
- Do not invent a recovery path merely to populate the section.
- Do not describe HTTP errors, server failures, database rollback, queue retries, retry counts, exception handlers, technical idempotency mechanisms, or infrastructure recovery.
- If no scenario-specific business failure or recovery behaviour is defined upstream, state:

```text
No scenario-specific business failure or recovery path is defined by the authoritative L2 architecture.
```

- If missing recovery behaviour creates an architecture gap that prevents a complete scenario, record it in Section 17.

---

# 12. Business Rules and Decision Constraints

Record only rules and decision constraints that materially govern this scenario.

```markdown
| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| <Existing rule or constraint> | <Canonical owner> | <How it governs this scenario> | <L2 document / rule or section reference> |
```

Rules:

- L3 references and applies business rules; it does not become a new canonical rule owner.
- Use an existing rule identifier when one exists.
- Do not manufacture local `BR-01`, `BR-02`, or similar identifiers that could imply new rule ownership.
- Do not change rule wording, ownership, precedence, or applicability without an explicit upstream architecture decision.
- When a rule is owned by another domain, preserve that owner and describe only its scenario effect.

---

# 13. State and Lifecycle Impact

Describe business state or lifecycle changes that occur as part of the scenario.

```markdown
| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| <Concept> | <Existing state(s)> | <Business transition or lifecycle effect> | <Existing resulting state> | <Domain> | <L2 source> |
```

Rules:

- Use only states and lifecycle positions defined by authoritative architecture.
- Do not invent, rename, merge, or split states in L3.
- Every lifecycle remains owned by its canonical aggregate/domain.
- A scenario may coordinate multiple owned lifecycle impacts without combining their ownership.
- If no lifecycle transition occurs, state that explicitly.

---

# 14. Business Events and Cross-Domain Collaboration

Cross-domain collaboration must preserve the distinction between published business events and synchronous business contracts or hand-offs.

## 14.1 Published / Consumed Business Events

```markdown
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| <Existing event> | <Single publisher> | <Consumers> | <Role in scenario> | <L2 source> |
```

Rules:

- Every business event has exactly one canonical publisher.
- Do not invent or silently rename events.
- Consuming an event does not transfer ownership of the event or the underlying business fact.
- Do not define message schemas, topics, queues, transport, or delivery technology.

If no business event materially participates in the scenario, state that explicitly.

## 14.2 Business Contracts and Cross-Domain Hand-offs

```markdown
| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| <Domain> | <Domain> | <Business fact, decision, request, confirmation, or governed hand-off> | <Scenario purpose> | <L2 source where material> |
```

Rules:

- Cross-domain interaction must use an approved business contract, workflow hand-off, or published event.
- Do not describe APIs, payload schemas, protocols, endpoints, or integration middleware.
- Do not use a hand-off to duplicate or transfer canonical ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

Define business-level authority required by the scenario. This section provides architecture inputs for later RBAC and access-control design but must not itself define technical RBAC.

## 15.1 Initiation and Decision Authority

```markdown
| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Owning Domain |
|---|---|---|---|
| <Action or decision> | <Role> | <Authority boundary, approval requirement, or eligibility> | <Domain> |
```

## 15.2 Separation of Duties, Overrides and Exceptions

Record only scenario-specific business obligations defined by authoritative architecture.

```markdown
| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| <Separation, override, exception authority> | <Roles> | <Business constraint> |
```

If no scenario-specific separation-of-duty, override, or exception requirement exists, state that explicitly.

## 15.3 Business Audit and Evidence Obligations

```markdown
| Business Obligation | Required Business Evidence / Record | Owning Domain |
|---|---|---|
| <Audit, approval, decision, or evidence obligation> | <Business evidence that must exist> | <Canonical owner> |
```

Rules:

- Define only business authority, approval, separation-of-duty, override, and evidence obligations.
- Do not define permission codes, technical roles, role hierarchies, access-control lists, IAM configuration, authentication mechanisms, or enforcement technology.
- Do not repeat generic platform audit infrastructure in every scenario. Record scenario-specific audit or evidence obligations and rely on the canonical platform audit architecture for cross-cutting infrastructure responsibility.
- If authoritative architecture defines no scenario-specific obligation, state that rather than inventing one.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

List the business-verifiable conditions that prove the scenario has completed successfully.

```markdown
- <Business completion criterion>
```

Completion Criteria must align with the Business Outcome and End Boundary.

## 16.2 Postconditions

List business facts that are true after successful completion and that may govern subsequent institutional behaviour.

```markdown
- <Business postcondition>
```

Rules:

- Postconditions are not a repetition of the trigger or preconditions.
- Do not describe technical persistence, cache state, message delivery internals, or database records.
- Cross-domain postconditions must preserve the canonical owner of each resulting business fact.

---

# 17. Open Issues, Assumptions and Deferred Questions

This section prevents unresolved uncertainty from being silently converted into architecture.

## 17.1 Open Issues

```markdown
| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| <Unresolved architecture issue> | <Impact on scenario correctness> | <Required owner / decision> |
```

If there are no open issues, state:

```text
No open issues.
```

A scenario must not be locked while an open issue materially affects ownership, lifecycle, event publisher, business rule, scenario boundary, or final outcome correctness.

## 17.2 Assumptions

```markdown
| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| <Explicit temporary interpretation> | <Why it is currently usable> | <Architecture impact> |
```

Rules:

- Assumptions must never override locked repository architecture.
- Do not use an assumption to resolve a repository conflict silently.

If there are no assumptions, state:

```text
No assumptions.
```

## 17.3 Deferred Questions

Record questions deliberately deferred to a later architecture level.

```markdown
| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| <Question> | L4 / L5 / L6 | <Why it is outside L3> |
```

Implementation choices should normally be deferred rather than answered in L3.

---

## Scenario Authoring and Review Checklist

Before approving or locking a canonical L3 scenario, verify:

```markdown
- [ ] The scenario represents one complete institutional business outcome.
- [ ] The scenario has a clear Start Boundary and End Boundary.
- [ ] Business Outcome, Preconditions, Trigger, Completion Criteria, and Postconditions are distinct and consistent.
- [ ] Primary Owner matches authoritative L1/L2 ownership of the final outcome.
- [ ] Participating domains retain their own canonical responsibilities and do not acquire duplicate ownership.
- [ ] Business actors and roles are supported by authoritative architecture or explicitly recorded as an issue.
- [ ] Main and alternate flows remain at business level.
- [ ] Failure and recovery content is business-level and does not contain technical error handling.
- [ ] The scenario does not invent or redefine business rules.
- [ ] The scenario does not invent, rename, merge, split, or re-own lifecycle states.
- [ ] Every referenced business event preserves its canonical publisher and canonical name.
- [ ] Events are not conflated with synchronous business contracts or hand-offs.
- [ ] Business authorization, approval, separation-of-duty, and audit obligations remain business-level.
- [ ] No technical RBAC, API, database, UI, deployment, infrastructure, or technology design appears in L3.
- [ ] Selective authoritative source references are present where needed to establish ownership, rule, state, event, or disputed behaviour.
- [ ] Detailed Phase 1/Phase 2 lineage has not been unnecessarily duplicated from the canonical consolidation register.
- [ ] Open issues and assumptions do not silently override repository decisions.
- [ ] The scenario is ready to inform later L4 architecture without prescribing L4 implementation.
```

---

## Template Governance Statement

This v2.0 template is the proposed final Phase 3 canonical L3 scenario structure.

Until its architecture review is completed and its lock status is changed to `Locked`, `L3_Business_Scenario_Template_v1.0.md` remains the historical approved pilot baseline and this v2.0 file is the candidate replacement for canonical scenario authoring.

When locked, v2.0 supersedes v1.0 for new canonical L3 scenario documents. The v1.0 file must remain in the repository as historical governance evidence.

Any later structural change to this template must be explicit, versioned, repository-reviewed, and must preserve L0/L1/L2/L3 architecture boundaries.