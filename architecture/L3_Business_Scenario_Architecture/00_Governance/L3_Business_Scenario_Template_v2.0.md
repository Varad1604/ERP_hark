# SutraERP L3 Business Scenario Template v2.0

**Document:** L3 Business Scenario Template  
**Architecture Level:** L3 — Business Scenario Architecture  
**Version:** 2.0  
**Status:** Locked  
**Owner:** Lead Architect  
**Last Updated:** 2026-08-18  
**Depends On:** Repository Constitution; locked L1 Business Domain Architecture; locked L2 Domain Workflow Architecture; `L3_Naming_Numbering_Standard.md`; Phase 2 final consolidation register  
**Referenced By:** Canonical L3 business scenario documents and future L4 System Behavior Specifications  
**Design Decision:** `L3_Phase3_Canonical_Template_Design_Decision_2026-08-18.md`  
**Supersedes:** `L3_Business_Scenario_Template_v1.0.md` for all new canonical L3 scenario authoring

---

## 1. Purpose

This template is the locked canonical structure for SutraERP Level 3 Business Scenario Architecture.

An L3 scenario describes one complete institutional business outcome across one or more participating business domains while preserving canonical ownership, lifecycle, event, rule, contract, and dependency boundaries established upstream.

L3 must be precise enough to inform later L4 system-behavior architecture without becoming L4, L5, L6, or implementation design.

---

## 2. Architecture Level Boundary

L3 may define:

- business actors and business roles;
- business responsibilities and authority;
- scenario boundaries, preconditions, trigger, outcome, and postconditions;
- end-to-end business behaviour;
- valid alternate business paths;
- business-level failure, interruption, correction, reversal, rejection, suspension, resubmission, and recovery where defined upstream;
- applicable business rules and decision constraints owned upstream;
- lifecycle and state impact owned upstream;
- published/consumed business events owned upstream;
- synchronous business contracts and governed cross-domain hand-offs;
- business approval, separation-of-duty, audit, and evidence obligations.

L3 must not define:

- API endpoints, payloads, or integration protocols;
- database schemas or persistence design;
- software classes, DTOs, services, components, or code structure;
- UI screens, buttons, forms, navigation, or interaction design;
- technical RBAC roles, permission codes, IAM groups, claims, ACLs, or enforcement mechanisms;
- deployment architecture or infrastructure configuration;
- technical retries, queues, transaction handling, server failures, database rollback, exception handlers, or infrastructure recovery;
- programming languages, frameworks, or technology choices.

L3 supplies business requirements for later access-control design; it does not design technical RBAC.

---

## 3. Naming and Numbering

Every final scenario must follow `L3_Naming_Numbering_Standard.md`.

```text
Scenario ID: L3-<PRIMARY_OWNER_SECTION>-<NNN>
File:        L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md
```

Final numbering is independent of L2 numbering.

Phase 1 candidate IDs and Phase 2 working IDs are not final L3 IDs. Detailed consolidation lineage remains authoritative in the Phase 2 register and is not duplicated as mandatory scenario metadata.

---

## 4. Canonical Scenario Structure

Every canonical L3 scenario must contain:

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
**Status:** Draft / Review / Architecture Approved / Locked  
**Version:** 1.0  
**Owner:** <Document owner>  
**Last Updated:** <YYYY-MM-DD>  
**Depends On:** <Authoritative L1/L2 source documents materially governing this scenario>  
**Referenced By:** <Known downstream/related architecture or None yet>  
```

Rules:

- Use only canonical repository status values.
- Primary Owner must own the final business outcome under locked L1/L2 architecture.
- Participating Domains must contain only material business participants.
- Scenario Type and Priority must follow `L3_Scenario_Type_and_Priority_Rules.md`.
- `Depends On` provides higher-level architectural traceability. It must not become a Phase 1/Phase 2 consolidation-history table.
- Phase 1 candidate IDs, Phase 2 working IDs, dispositions, and watchlist history remain in the Phase 2 register rather than mandatory scenario metadata.

---

# 2. Scenario Purpose

Describe the institutional reason this scenario exists.

The purpose must be business-oriented, outcome-oriented, and supported by authoritative architecture. It must not describe implementation mechanics or repeat the entire L2 workflow.

---

# 3. Business Outcome

Describe the singular institutional business outcome that marks successful scenario completion.

The outcome must be:

- institutionally meaningful;
- business-verifiable;
- consistent with the Primary Owner's canonical responsibility;
- distinguishable from a workflow step, notification, validation, or technical result.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

State the business boundary at which the scenario begins.

## 4.2 End Boundary

State the business boundary at which the scenario ends. It must align with the Business Outcome and Completion Criteria.

## 4.3 In Scope

List the material business behaviour covered by the scenario.

## 4.4 Out of Scope

List materially adjacent business behaviour intentionally excluded because it belongs to another scenario, another owner, or a later architecture level.

## 4.5 Related Scenarios — Optional

Use only when canonical related scenario IDs exist.

```markdown
| Relationship | Related Scenario | Reason |
|---|---|---|
| Preceded by / Followed by / Related | <L3 ID and title> | <Business relationship> |
```

Do not invent placeholder final L3 IDs.

---

# 5. Ownership and Participating Domains

```markdown
| Domain / Section | Responsibility in Scenario |
|---|---|
| <Primary Owner> | <Owned responsibility leading to the final outcome> |
| <Participating Domain> | <Owned contribution, fact, workflow responsibility, decision, or confirmation> |
```

Rules:

- Participation never transfers ownership.
- A domain consuming another domain's fact, event, read model, or contract does not become its owner.
- Runtime dependency does not reverse foundational dependency or ownership.
- Do not list a platform/supporting domain merely because a technical capability may be used.

---

# 6. Business Actors, Roles and Responsibilities

Identify business actors and institutional roles that participate in the scenario.

```markdown
| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| <Actor or role> | <Business responsibility> | <Initiates / reviews / decides / approves / performs / receives outcome> |
```

Rules:

- Use only roles supported by authoritative business architecture.
- If a required actor or role is absent upstream, record the gap in Section 17 rather than inventing one.
- This section is a business input for future RBAC; it must not define technical permissions or access-control structures.

---

# 7. Preconditions and Entry Criteria

List business facts or conditions that must already be true before the scenario may validly begin.

```markdown
- <Business precondition or entry criterion>
```

Do not use technical availability checks. Do not repeat the trigger or final outcome as a precondition.

---

# 8. Trigger

Describe the business event, request, decision, condition, or lifecycle occurrence that starts the scenario.

```text
<Business-readable trigger>
```

Do not express the trigger as an API call, button click, database write, cron job, message-broker mechanism, or other implementation detail.

---

# 9. Main Business Flow

Describe the normal successful end-to-end business path.

```markdown
| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | <Business action or decision> | <Role / Domain> | <Result or hand-off> |
```

Rules:

- Keep steps at institutional business level.
- Do not decompose L2 workflows into unnecessary micro-steps.
- Do not redefine upstream rules, states, events, contracts, or ownership.
- Do not describe screens, APIs, jobs, tables, services, classes, or technical orchestration.
- Cite an authoritative source only where materially needed to establish or verify a non-obvious behaviour, ownership boundary, rule, state, event, contract, or disputed interpretation.

---

# 10. Alternate Business Flows

Describe valid governed paths that differ from the Main Business Flow but remain legitimate paths inside the same scenario boundary.

```markdown
| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| <Case> | <Business condition> | <Alternative path> | <Business result> | <Domain> |
```

An alternate business flow is not automatically a failure or recovery flow.

---

# 11. Failure, Interruption and Business Recovery

Describe business-level situations in which normal scenario progression cannot continue and the governed business resolution where one exists.

```markdown
| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| <Failure or interruption> | <Business impact> | <Correction / reversal / rejection / suspension / resubmission / terminal outcome> | <Canonical owner> |
```

Rules:

- Model only business-level failure/recovery supported by authoritative architecture.
- Do not invent a recovery path to fill the section.
- Technical retries, queues, server failures, rollback, exception handlers, timeout handling, and infrastructure recovery are prohibited.

If no scenario-specific business failure or recovery is defined upstream, state:

```text
No scenario-specific business failure or recovery path is defined by the authoritative L2 architecture.
```

If the missing recovery path prevents a correct scenario, record it as an Open Issue.

---

# 12. Business Rules and Decision Constraints

```markdown
| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| <Existing rule or constraint> | <Canonical owner> | <Effect on this scenario> | <L2 rule/section reference> |
```

Rules:

- L3 applies and references rules; it does not become a new rule owner.
- Use existing authoritative rule identifiers where available.
- Do not manufacture local `BR-01`, `BR-02`, or similar identifiers.
- Do not change rule wording, ownership, precedence, or applicability without an explicit upstream decision.

---

# 13. State and Lifecycle Impact

```markdown
| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| <Concept> | <Existing state(s)> | <Business transition/effect> | <Existing resulting state> | <Domain> | <L2 source> |
```

Rules:

- Use only authoritative lifecycle states and transitions.
- Do not invent, rename, merge, split, or re-own states.
- Every lifecycle remains owned by its canonical aggregate/domain.
- A scenario may coordinate multiple lifecycle impacts without combining ownership.

If no lifecycle transition occurs, state that explicitly.

---

# 14. Business Events and Cross-Domain Collaboration

Events and synchronous contracts are distinct architectural mechanisms and must remain separate.

## 14.1 Published / Consumed Business Events

```markdown
| Business Event | Canonical Publisher | Consumer(s) / Interested Domain(s) | Scenario Purpose | Authoritative Source |
|---|---|---|---|---|
| <Existing event> | <Single publisher> | <Consumers> | <Role in scenario> | <L2 source> |
```

Rules:

- Every business event has exactly one canonical publisher.
- Do not invent or silently rename events.
- Event consumption does not transfer ownership of the event or underlying business fact.
- Do not define topics, queues, schemas, protocols, or delivery technology.

If no business event materially participates, state that explicitly.

## 14.2 Business Contracts and Cross-Domain Hand-offs

```markdown
| Provider / From Domain | Consumer / To Domain | Business Contract / Hand-off | Purpose | Authoritative Source |
|---|---|---|---|---|
| <Domain> | <Domain> | <Business fact, request, decision, confirmation, read model, or governed hand-off> | <Purpose> | <L2 source where material> |
```

Rules:

- Cross-domain interaction must use architecture-approved collaboration mechanisms.
- Do not describe APIs, payload schemas, endpoints, middleware, or transport.
- A contract or hand-off must not duplicate or transfer canonical ownership.

---

# 15. Business Authorization, Approval and Audit Obligations

This section defines business authority and evidence requirements. It is an input to later RBAC/access-control design, not technical RBAC itself.

Section A remains the canonical owner of platform Authorization and Audit infrastructure. The owning business domain remains the owner of its business decision, rule, approval requirement, evidence, and outcome.

## 15.1 Initiation and Decision Authority

```markdown
| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| <Action or decision> | <Role> | <Authority, approval requirement, or eligibility> | <Domain owning the business decision> |
```

## 15.2 Separation of Duties, Overrides and Exceptions

```markdown
| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| <Separation, override, or exception requirement> | <Roles> | <Business constraint> |
```

If no scenario-specific requirement exists, state that explicitly.

## 15.3 Business Audit and Evidence Obligations

```markdown
| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| <Audit, approval, decision, or evidence obligation> | <Business evidence that must exist> | <Owner of the underlying business fact/evidence obligation> |
```

Rules:

- Do not define permission codes, technical roles, IAM groups, claims, ACLs, authentication mechanisms, or enforcement technology.
- Do not re-own Section A Authorization or Audit infrastructure.
- Do not repeat generic platform audit mechanics in every scenario; record scenario-specific business evidence obligations only.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

List the business-verifiable conditions proving successful completion.

```markdown
- <Business completion criterion>
```

Completion Criteria must align with the Business Outcome and End Boundary.

## 16.2 Postconditions

List business facts that are true after successful completion and may govern subsequent institutional behaviour.

```markdown
- <Business postcondition>
```

Do not describe technical persistence, cache state, database records, message-delivery internals, or infrastructure status.

---

# 17. Open Issues, Assumptions and Deferred Questions

This section prevents unresolved uncertainty from being silently converted into architecture.

## 17.1 Open Issues

```markdown
| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| <Issue> | <Impact on scenario correctness> | <Required decision/owner> |
```

If none:

```text
No open issues.
```

A scenario must not be Locked while an open issue materially affects ownership, lifecycle, event publisher, rule, scenario boundary, authority, or final outcome correctness.

## 17.2 Assumptions

```markdown
| Assumption | Basis | Risk if Incorrect |
|---|---|---|
| <Explicit temporary interpretation> | <Basis> | <Architecture impact> |
```

Assumptions must never override locked architecture or silently resolve a repository conflict.

If none:

```text
No assumptions.
```

## 17.3 Deferred Questions

```markdown
| Deferred Question | Target Level | Reason for Deferral |
|---|---|---|
| <Question> | L4 / L5 / L6 | <Why outside L3> |
```

Implementation choices should normally be deferred rather than answered in L3.

---

## 5. Canonical Scenario Review Checklist

Before a canonical scenario becomes Locked, verify:

```markdown
- [ ] One complete institutional business outcome is represented.
- [ ] Start Boundary and End Boundary are clear.
- [ ] Business Outcome, Preconditions, Trigger, Completion Criteria, and Postconditions are distinct and consistent.
- [ ] Primary Owner matches locked L1/L2 ownership of the final outcome.
- [ ] Participating domains retain their canonical responsibilities.
- [ ] Actors and business roles are supported upstream or explicitly recorded as issues.
- [ ] Main and alternate flows remain at business level.
- [ ] Failure/recovery is business-level and does not contain technical error handling.
- [ ] No business rule is invented, re-owned, or silently changed.
- [ ] No lifecycle state is invented, renamed, merged, split, or re-owned.
- [ ] Every business event preserves its canonical name and publisher.
- [ ] Events remain distinct from synchronous contracts and hand-offs.
- [ ] Authorization/approval/audit content is business-level and does not redefine Section A platform ownership.
- [ ] No technical RBAC, API, database, UI, deployment, infrastructure, or technology design appears in L3.
- [ ] Higher-level dependency/source metadata is present.
- [ ] Detailed Phase 1/Phase 2 lineage is not unnecessarily duplicated from the Phase 2 register.
- [ ] Open issues and assumptions do not override repository decisions.
- [ ] The scenario is suitable to inform L4 without prescribing implementation.
```

---

## 6. Lock Statement

`L3_Business_Scenario_Template_v2.0.md` is the locked canonical template for all new SutraERP Level 3 Business Scenario Architecture documents.

It supersedes `L3_Business_Scenario_Template_v1.0.md` for new canonical authoring. The v1.0 file remains historical pilot evidence and must not be used as the governing template for new scenarios.

Any later structural change requires explicit architecture review, versioning, repository synchronization, and Lead Architect approval.