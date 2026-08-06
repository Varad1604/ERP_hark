# SutraERP L3 Business Scenario Template v1.0

**Document:** L3 Business Scenario Template
**Architecture Level:** L3 — Business Scenario Architecture
**Version:** 1.0
**Status:** Approved for Pilot Use
**Final Lock Status:** Pending Pilot Validation
**Applies To:** Full L3 business scenario documents created after Phase 1 candidate discovery

---

## 1. Template Purpose

This template defines the standard structure for SutraERP Level 3 Business Scenario Architecture documents.

An L3 business scenario describes one complete institutional business outcome.

This template is approved for pilot use. It may be refined after pilot validation, but it should not be structurally redesigned unless pilot review proves that a major change is necessary.

---

## 2. Architecture Level Boundary

L3 describes business scenario behaviour.

L3 must remain implementation independent.

L3 must not define:

* API endpoints
* database schemas
* software classes
* DTOs
* UI screens
* buttons
* forms
* deployment architecture
* infrastructure configuration
* programming language decisions
* technology choices

Those belong to L4, L5, L6, or implementation.

---

## 3. Scenario Document Naming

Final L3 scenario files must use this format:

```text id="sajtsa"
L3-<PRIMARY_OWNER_SECTION>-<NNN>_<Scenario_Slug>.md
```

Example:

```text id="uqmrve"
L3-C-001_Approved_Candidate_to_Active_Student.md
```

Final L3 IDs are assigned only after Phase 2 consolidation.

---

## 4. L3 Scenario Document Structure

Each final L3 scenario document must use the following structure:

```text id="h3p462"
1. Scenario Metadata
2. Scenario Purpose
3. Business Outcome
4. Scope
5. Participating Domains
6. Preconditions
7. Trigger
8. Main Business Flow
9. Alternate / Exception Flows
10. Business Rules
11. Business Events
12. State / Lifecycle Impact
13. Cross-Domain Hand-offs
14. Completion Criteria
15. Traceability
16. Open Issues
```

---

# 1. Scenario Metadata

```markdown id="z17iiv"
# <Scenario Title>

**Scenario ID:** L3-<PRIMARY_OWNER_SECTION>-<NNN>  
**Original Candidate ID:** CAN-L3-<AREA>-<NNN>  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** <Domain / Section>  
**Participating Domains:** <Domains>  
**Scenario Type:** Simple / Cross-domain / Critical  
**Priority:** P0 / P1 / P2  
**Status:** Draft / Pilot / Approved / Locked  
**Version:** 1.0  
```

## Metadata Rules

* Scenario ID is assigned only after Phase 2 consolidation.
* Original Candidate ID preserves Phase 1 traceability.
* Primary Owner must reflect ownership of the final business outcome.
* Participating Domains must include only actual participants.
* Scenario Type must use only `Simple`, `Cross-domain`, or `Critical`.
* Priority must use only `P0`, `P1`, or `P2`.

---

# 2. Scenario Purpose

Describe why this scenario exists.

The purpose should explain the institutional need for the scenario.

It should not describe implementation mechanics.

Example:

```text id="tssdry"
This scenario describes how an approved admission candidate becomes an active student within the institution after required activation conditions are satisfied.
```

---

# 3. Business Outcome

Describe the final business outcome that marks successful completion of the scenario.

The outcome must be clear, institutional, and verifiable.

Example:

```text id="2wr55g"
The approved candidate is activated as a student and becomes eligible for institutionally governed student lifecycle processes.
```

---

# 4. Scope

## 4.1 In Scope

List what this scenario covers.

Example:

```markdown id="w1bpfv"
- Verification that the scenario trigger has occurred.
- Coordination between participating business domains.
- Completion of the final business outcome.
- Recording of business-relevant state or lifecycle impact.
```

## 4.2 Out of Scope

List what this scenario does not cover.

Example:

```markdown id="kfnz4o"
- API design.
- Database schema design.
- User interface design.
- Deployment design.
- Technical implementation details.
- Independent scenarios that should have separate L3 documents.
```

---

# 5. Participating Domains

List the domains that participate in the scenario.

For each domain, state its business responsibility in this scenario.

```markdown id="ekj4th"
| Domain / Section | Responsibility in Scenario |
|---|---|
| <Primary Owner> | Owns the final business outcome |
| <Participating Domain> | Provides required business input or performs owned workflow responsibility |
```

Rules:

* Do not assign responsibility that contradicts L1/L2 ownership.
* Do not make a supporting service the owner unless it owns the final business outcome.
* Do not list domains that do not materially participate.

---

# 6. Preconditions

List the business conditions that must already be true before the scenario can begin.

Example:

```markdown id="kggx31"
- Candidate admission decision has been approved.
- Required institutional structure is available.
- Required fee or clearance status is available, if applicable.
```

Preconditions must be business conditions, not technical checks.

---

# 7. Trigger

Describe the business event or condition that starts the scenario.

Example:

```text id="ui3rjq"
The admission approval becomes available for activation processing.
```

The trigger must be business-readable.

Avoid technical triggers such as:

```text id="1m3etv"
API call received
Database record inserted
Button clicked
Cron job started
```

---

# 8. Main Business Flow

Describe the normal successful business flow.

Use numbered steps.

Each step should describe business behaviour, not implementation.

```markdown id="9h4lmd"
1. <Domain> confirms that the scenario trigger is valid.
2. <Domain> performs its owned business responsibility.
3. <Participating Domain> provides required business confirmation.
4. <Primary Owner> confirms that all required conditions are satisfied.
5. <Primary Owner> completes the final business outcome.
```

Rules:

* Keep steps at business level.
* Do not describe screens, APIs, tables, jobs, services, or classes.
* Do not redefine L2 workflow internals.
* Reference domain responsibilities without duplicating the full L2 workflow.

---

# 9. Alternate / Exception Flows

Describe business alternatives or exceptions.

```markdown id="8tlrb0"
| Case | Condition | Business Handling |
|---|---|---|
| <Case Name> | <Business condition> | <Business response> |
```

Examples:

```markdown id="fegase"
| Missing Clearance | Required clearance is not available | Scenario remains pending until clearance is completed |
| Ownership Conflict | Required ownership cannot be confirmed | Issue is raised for architecture review |
```

Do not describe technical errors such as server errors, timeout errors, stack traces, or failed API calls.

---

# 10. Business Rules

List only business rules that belong to this scenario and its owning domains.

```markdown id="t6aqdg"
| Rule ID | Business Rule | Owning Domain |
|---|---|---|
| BR-01 | <Business rule> | <Owning Domain> |
```

Rules:

* Do not invent new business rules.
* Do not move business rules from one domain to another.
* Do not duplicate detailed L2 rules unless needed for scenario comprehension.
* If a rule belongs to another domain, reference that domain responsibility instead of claiming ownership.

---

# 11. Business Events

List business events published or consumed during the scenario.

```markdown id="yrfgni"
| Event | Publisher | Consumers / Interested Domains | Purpose |
|---|---|---|---|
| <Business Event> | <Canonical Publisher> | <Consumers> | <Business purpose> |
```

Rules:

* Every business event has exactly one canonical publisher.
* Do not invent new event names during drafting.
* If an event is uncertain, mark it as an open issue.
* Do not define technical message schemas.

---

# 12. State / Lifecycle Impact

Describe any business state or lifecycle changes caused by the scenario.

```markdown id="rr6cm6"
| Entity / Aggregate | State or Lifecycle Impact | Owning Domain |
|---|---|---|
| <Entity> | <Impact> | <Owning Domain> |
```

Rules:

* Do not create new lifecycle states unless already approved.
* Do not move lifecycle ownership.
* Do not modify entity ownership.
* If state impact is uncertain, mark it as an open issue.

---

# 13. Cross-Domain Hand-offs

Describe meaningful business hand-offs between domains.

```markdown id="cz3qsp"
| From Domain | To Domain | Hand-off / Contract | Purpose |
|---|---|---|---|
| <Domain> | <Domain> | <Business hand-off> | <Purpose> |
```

Rules:

* Cross-domain interaction must occur through defined business contracts, workflows, or published events.
* Do not define implementation protocols.
* Do not define APIs or integration details.

---

# 14. Completion Criteria

List the conditions that prove the scenario has completed successfully.

Example:

```markdown id="h3lb15"
- Final business outcome has been achieved.
- Required state or lifecycle impact has been recorded by the owning domain.
- Required downstream domains have been notified or made aware through approved business mechanisms.
- No unresolved blocker remains for the scenario outcome.
```

Completion criteria must be business-verifiable.

---

# 15. Traceability

Record the upstream and related architecture sources.

```markdown id="fwony0"
| Source Type | Reference |
|---|---|
| L1 Domain | <L1 domain / section> |
| L2 Workflow Document(s) | <Relevant L2 documents> |
| Original Candidate ID | <CAN-L3-...> |
| Related Scenario(s) | <Related L3 scenarios, if any> |
| Architecture Decision(s) | <L3-ADR references, if any> |
```

Traceability is mandatory for final L3 scenario documents.

---

# 16. Open Issues

Record unresolved architecture issues.

```markdown id="ttyjnv"
| Issue ID | Issue | Impact | Required Resolution |
|---|---|---|---|
| L3-ISS-001 | <Issue> | <Impact> | <Resolution needed> |
```

If there are no open issues:

```text id="6493vd"
No open issues.
```

---

## 17. Scenario Authoring Checklist

Before submitting a full L3 scenario document, verify:

```markdown id="qj15tg"
- [ ] Scenario describes one complete business outcome.
- [ ] Scenario does not describe APIs, database, UI, or implementation.
- [ ] Scenario has correct primary owner.
- [ ] Scenario does not redefine L1 or L2 ownership.
- [ ] Scenario does not invent lifecycle states.
- [ ] Scenario does not invent business events.
- [ ] Scenario does not duplicate L2 workflow internals unnecessarily.
- [ ] Scenario lists only real participating domains.
- [ ] Scenario has clear trigger.
- [ ] Scenario has clear completion criteria.
- [ ] Scenario preserves traceability to Phase 1 candidate ID.
- [ ] Scenario identifies unresolved issues instead of silently resolving conflicts.
```

---

## 18. Template Lock Statement

```text id="g8n4lq"
This L3 Business Scenario Template v1.0 is approved for pilot use.

It may be refined after pilot validation.

It must not be structurally redesigned unless pilot validation demonstrates that the template is insufficient.

L3 scenario documents must remain business architecture documents and must not include implementation architecture or technical design.
```
