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
**Authorized In-Place Refinement:** 2026-08-18 — Lead Architect approved strengthening authoring controls inside v2.0 without changing the canonical 17-section structure or creating v2.1.

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

### 2.1 Compact Authoring Principle

Completeness does not require repetition.

Every canonical L3 document shall use the minimum content necessary to express the scenario completely and unambiguously.

Authors shall:

- prefer one precise statement over repeated explanation;
- reference authoritative upstream architecture instead of reproducing it;
- include only rules, states, events, contracts, actors, evidence obligations, and exclusions materially relevant to the scenario;
- avoid repeating repository-wide governance principles unless they materially affect the scenario;
- state the absence of scenario-specific content concisely instead of adding filler;
- keep business-flow steps at the minimum granularity needed to describe the complete institutional outcome.

Document length and step count are not quality targets. Scenario complexity shall determine necessary detail.

Compact authoring must never remove information required to establish scenario boundary, ownership, authority, lifecycle impact, business rules, events, cross-domain collaboration, recovery, audit obligations, or completion.

### 2.2 Upstream Ambiguity Rule

Where authoritative upstream architecture contains an internal ambiguity, contradiction, unresolved sequencing issue, or missing material definition, L3 shall not:

- choose one interpretation silently;
- invent a compromise;
- reconcile conflicting upstream statements as if a decision had already been made;
- manufacture a missing actor, rule, state, event, contract, or authority.

L3 shall:

1. preserve all upstream architecture that is unambiguous;
2. avoid depending on the disputed interpretation where possible;
3. record the ambiguity or gap in Section 17 — Open Issues;
4. identify its impact on scenario correctness, approval, or locking.

A material unresolved issue affecting ownership, lifecycle, event publisher, business rule, scenario boundary, authority, or final outcome prevents the scenario from becoming Locked.

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

Keep the purpose concise; normally one or two precise paragraphs are sufficient unless the scenario boundary genuinely requires more explanation.

---

# 3. Business Outcome

Describe the singular institutional business outcome that marks successful scenario completion.

The outcome must be:

- institutionally meaningful;
- business-verifiable;
- consistent with the Primary Owner's canonical responsibility;
- distinguishable from a workflow step, notification, validation, or technical result.

Do not restate the Main Business Flow in the outcome.

---

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

State the business boundary at which the scenario begins.

## 4.2 End Boundary

State the business boundary at which the scenario ends. It must align with the Business Outcome and Completion Criteria.

## 4.3 In Scope

List only the material business behaviour covered by the scenario.

## 4.4 Out of Scope

List only materially adjacent business behaviour that could reasonably be confused with this scenario and is intentionally excluded because it belongs to another scenario, another owner, or a later architecture level.

Do not create exhaustive generic exclusion lists.

## 4.5 Related Scenarios — Optional

Use only when canonical related scenario IDs exist and the relationship materially clarifies the boundary.

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
- Do not repeat generic ownership principles after the table unless a scenario-specific ownership boundary requires clarification.

---

# 6. Business Actors, Roles and Responsibilities

Identify business actors and institutional roles that materially participate in the scenario.

```markdown
| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| <Actor or role> | <Business responsibility> | <Initiates / reviews / decides / approves / performs / receives outcome> |
```

Rules:

- Use only actors and roles supported by authoritative business architecture.
- Do not create descriptive placeholder roles such as `Approving Authority`, `Business Reviewer`, `Scenario Owner`, `Requesting Authority`, or similar labels unless the role is explicitly supported upstream.
- Where a business responsibility is known but the canonical actor or role is not defined upstream, record the gap in Section 17 rather than converting the responsibility into a new role.
- A domain name is not automatically a business actor.
- This section is a business input for future RBAC; it must not define technical permissions or access-control structures.
- If no scenario-specific actor can be established authoritatively, state that concisely and record any material gap in Section 17.

---

# 7. Preconditions and Entry Criteria

List business facts or conditions that must already be true before the scenario may validly begin.

```markdown
- <Business precondition or entry criterion>
```

Rules:

- Do not use technical availability checks.
- Do not repeat the trigger or final outcome as a precondition.
- Do not move an activity into Preconditions merely because upstream architecture describes it inconsistently.
- If the same matter is defined upstream both as a precondition and as an activity within the workflow, record the sequencing ambiguity in Section 17 rather than silently choosing one interpretation.

---

# 8. Trigger

Describe the business event, request, decision, condition, or lifecycle occurrence that starts the scenario.

```text
<Business-readable trigger>
```

Do not express the trigger as an API call, button click, database write, cron job, message-broker mechanism, or other implementation detail.

Keep the trigger to the minimum business statement necessary to establish scenario entry.

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
- Include only steps materially required to reach the scenario's Business Outcome.
- Do not redefine upstream rules, states, events, contracts, or ownership.
- Do not describe screens, APIs, jobs, tables, services, classes, or technical orchestration.
- Cite an authoritative source only where materially needed to establish or verify a non-obvious behaviour, ownership boundary, rule, state, event, contract, or disputed interpretation.

### Scenario End Discipline

The Main Business Flow shall stop when the defined Business Outcome and End Boundary are achieved.

Consequences that occur only after successful scenario completion belong in Postconditions, Cross-Domain Collaboration, or a separate downstream L3 scenario as appropriate.

Do not extend the Main Business Flow merely to describe every possible downstream reaction to the completed outcome.

---

# 10. Alternate Business Flows

Describe valid governed paths that differ from the Main Business Flow but remain legitimate paths inside the same scenario boundary and can still progress toward the same defined business outcome.

```markdown
| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| <Case> | <Business condition> | <Alternative path> | <Business result> | <Domain> |
```

Classification rule:

- If the condition permits a valid governed alternative path toward the same scenario outcome, it may belong here.
- If the condition prevents normal progression and requires waiting, correction, rejection, suspension, resubmission, recovery, or termination, it belongs in Section 11.

Do not populate this section merely to demonstrate that alternate flows were considered.

If no scenario-specific alternate business flow is defined upstream, state:

```text
No scenario-specific alternate business flow is defined by the authoritative L2 architecture.
```

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
- A missing approval, unsatisfied dependency, invalid business condition, or other blocker belongs here when it prevents normal scenario progression.
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

- Include only upstream rules and constraints that materially determine, constrain, or validate this scenario.
- L3 applies and references rules; it does not become a new rule owner.
- Use existing authoritative rule identifiers where available.
- Do not manufacture local `BR-01`, `BR-02`, or similar identifiers.
- Do not change rule wording, ownership, precedence, or applicability without an explicit upstream decision.
- Do not list every rule belonging to an aggregate merely because it exists upstream.

If no scenario-specific business rule materially applies beyond already referenced upstream workflow semantics, state that concisely rather than adding filler.

---

# 13. State and Lifecycle Impact

```markdown
| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| <Concept> | <Existing state(s)> | <Business transition/effect> | <Existing resulting state> | <Domain> | <L2 source> |
```

Rules:

- Use only authoritative lifecycle states and transitions.
- Include only lifecycle impacts materially involved in the scenario.
- Do not reproduce an entire lifecycle diagram when a specific transition or lifecycle position is sufficient.
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

- Include only exact canonical business events that materially participate in the scenario.
- Every business event has exactly one canonical publisher.
- Do not invent or silently rename events.
- Do not use vague placeholders such as `other applicable events`, `relevant downstream events`, or `other domain notifications`.
- If an exact event required by the scenario is not established upstream, record the gap rather than generalising it.
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

- Include only contracts and hand-offs materially required by the scenario.
- Cross-domain interaction must use architecture-approved collaboration mechanisms.
- Do not describe APIs, payload schemas, endpoints, middleware, or transport.
- A contract or hand-off must not duplicate or transfer canonical ownership.
- Do not reproduce general-purpose contracts merely because the owning domain exposes them.

If no synchronous contract or governed hand-off materially participates, state that concisely.

---

# 15. Business Authorization, Approval and Audit Obligations

This section defines scenario-specific business authority and evidence requirements. It is an input to later RBAC/access-control design, not technical RBAC itself.

Section A remains the canonical owner of platform Authorization and Audit infrastructure. The owning business domain remains the owner of its business decision, rule, approval requirement, evidence, and outcome.

## 15.1 Initiation and Decision Authority

```markdown
| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| <Action or decision> | <Role> | <Authority, approval requirement, or eligibility> | <Domain owning the business decision> |
```

Use only authority and roles supported by authoritative architecture. If authority is required but the canonical actor or role is undefined, record the gap in Section 17 rather than inventing a role.

## 15.2 Separation of Duties, Overrides and Exceptions

```markdown
| Obligation | Business Roles / Authorities Involved | Constraint / Required Handling |
|---|---|---|
| <Separation, override, or exception requirement> | <Roles> | <Business constraint> |
```

If no scenario-specific requirement exists, state that explicitly.

Do not invent separation-of-duty, override, escalation, or exception rules merely because they would be desirable operational controls.

## 15.3 Business Audit and Evidence Obligations

```markdown
| Business Obligation | Required Business Evidence / Record | Business-Domain Owner |
|---|---|---|
| <Audit, approval, decision, or evidence obligation> | <Business evidence that must exist> | <Owner of the underlying business fact/evidence obligation> |
```

Rules:

- Include only scenario-specific business evidence obligations supported by authoritative architecture or necessarily implied by an upstream-defined business decision or approval requirement.
- Do not invent evidence fields, supporting documents, comments, reasons, timestamps, or approval artefacts merely because they would be useful operationally.
- Do not define permission codes, technical roles, IAM groups, claims, ACLs, authentication mechanisms, or enforcement technology.
- Do not re-own Section A Authorization or Audit infrastructure.
- Do not repeat generic platform audit mechanics in every scenario.

---

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

List only the business-verifiable conditions necessary to prove successful completion.

```markdown
- <Business completion criterion>
```

Completion Criteria must align with the Business Outcome and End Boundary.

Do not repeat every Main Business Flow step as a completion criterion.

## 16.2 Postconditions

List only material business facts that are true after successful completion and may govern subsequent institutional behaviour.

```markdown
- <Business postcondition>
```

Do not restate the entire outcome, rules, event table, or Main Business Flow.

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

Any material upstream ambiguity identified under Section 2.2 shall be recorded here rather than silently resolved in L3.

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

Record only scenario-specific deferred architecture questions.

Do not populate this section merely to list generic implementation topics such as UI, API, database, messaging, deployment, or technical RBAC questions that are already excluded from L3 by governance.

If none:

```text
No scenario-specific deferred questions.
```

---

## 5. Canonical Scenario Review Checklist

Before a canonical scenario becomes Locked, verify:

```markdown
- [ ] One complete institutional business outcome is represented.
- [ ] Start Boundary and End Boundary are clear.
- [ ] Business Outcome, Preconditions, Trigger, Completion Criteria, and Postconditions are distinct and consistent.
- [ ] Primary Owner matches locked L1/L2 ownership of the final outcome.
- [ ] Participating domains retain their canonical responsibilities.
- [ ] Actors and business roles are supported upstream; no placeholder or invented role has been introduced.
- [ ] Any material upstream ambiguity or missing definition is recorded rather than silently resolved.
- [ ] Main Flow stops at the defined Business Outcome and End Boundary.
- [ ] Main and alternate flows remain at business level and are not decomposed into unnecessary micro-steps.
- [ ] Alternate flows represent valid paths toward the same outcome; blockers and interrupted progression are classified under Failure/Recovery.
- [ ] Failure/recovery is business-level and does not contain technical error handling.
- [ ] Only materially relevant upstream business rules are included; no rule is invented, re-owned, or silently changed.
- [ ] Only materially relevant lifecycle impacts are included; no lifecycle state is invented, renamed, merged, split, or re-owned.
- [ ] Every listed business event is exact, material, and preserves its canonical name and publisher.
- [ ] Events remain distinct from synchronous contracts and hand-offs.
- [ ] Only materially participating contracts and hand-offs are included.
- [ ] Authorization/approval/audit content is business-level, source-grounded, scenario-specific, and does not redefine Section A platform ownership.
- [ ] No unsupported evidence requirement, authority, separation-of-duty rule, or audit artefact has been invented.
- [ ] No technical RBAC, API, database, UI, deployment, infrastructure, or technology design appears in L3.
- [ ] Higher-level dependency/source metadata is present.
- [ ] Detailed Phase 1/Phase 2 lineage is not unnecessarily duplicated from the Phase 2 register.
- [ ] Repository-wide governance is referenced rather than unnecessarily repeated.
- [ ] Empty or non-applicable sections are expressed concisely without filler.
- [ ] Open issues and assumptions do not override repository decisions.
- [ ] The scenario is suitable to inform L4 without prescribing implementation.
```

---

## 6. Lock Statement

`L3_Business_Scenario_Template_v2.0.md` is the locked canonical template for all new SutraERP Level 3 Business Scenario Architecture documents.

It supersedes `L3_Business_Scenario_Template_v1.0.md` for new canonical authoring. The v1.0 file remains historical pilot evidence and must not be used as the governing template for new scenarios.

The 2026-08-18 Lead Architect-authorized in-place refinement strengthens authoring discipline, ambiguity handling, actor integrity, flow classification, scenario-end discipline, source-grounded evidence, and compact authoring without changing the canonical 17-section structure.

Any later structural change requires explicit architecture review, versioning, repository synchronization, and Lead Architect approval.