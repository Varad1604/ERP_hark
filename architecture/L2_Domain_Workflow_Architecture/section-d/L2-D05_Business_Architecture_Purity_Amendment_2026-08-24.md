# L2-D05 — Academic Monitoring Workflows — Business Architecture Purity Amendment

| Attribute | Value |
|---|---|
| Target | L2-D05 — Academic Monitoring Workflows |
| Amendment Version | 1.2.1 |
| Status | Locked |
| Effective Date | 2026-08-24 |
| Authority | Explicit repository-change authorization recorded 2026-08-24 |
| Change Category | Patch — architecture-level clarification |

This amendment is part of the canonical L2-D05 architecture. It preserves the existing Academic Monitoring ownership, business outcomes, workflows, authorities, rules, events and lifecycles while replacing implementation-oriented execution wording with business-architecture semantics.

---

## 1. Purpose

The original L2-D05 document contains execution-pattern terminology such as durable workflow state, idempotent processing, compensation workflows and controlled parallel execution. Those terms describe solution or implementation mechanisms and shall not be interpreted as canonical L2 business architecture requirements.

The business requirements behind them are preserved below in implementation-independent form.

---

## 2. Canonical Business-Level Interpretation

| Implementation-Oriented Phrase | Canonical L2 Business-Architecture Requirement |
|---|---|
| Durable workflow state | If monitoring activity is interrupted, the last authoritative business position, decision evidence and governed context required for legitimate continuation shall remain preserved. |
| Idempotent processing | Repeated receipt or reconsideration of the same authoritative business fact shall not create duplicate institutional monitoring outcomes or duplicate governance consequences. |
| Compensation workflows | Where a completed monitoring business action must be corrected, withdrawn or reversed, only an explicitly governed correction, withdrawal or reversal path may change the authoritative business position; prior history remains preserved. |
| Controlled parallel execution | Concurrent monitoring activities are permitted only where their governed scopes, authorities and resulting decisions do not conflict or create duplicate authoritative outcomes. |
| SLA management | Any monitoring timeliness obligation is a business policy requirement only where established by an authoritative institutional policy; implementation scheduling mechanisms are outside L2. |
| Priority management | Business priority may influence governance attention where explicitly established by policy, but technical scheduling or queue priority is outside L2. |

---

## 3. Failure and Recovery Semantics

For L2-D05 business architecture:

1. An interruption does not create a new business outcome by itself.
2. Recovery resumes from the last valid authoritative business position or follows an explicitly governed correction/restart path.
3. Repeated evidence or repeated initiation signals shall not create duplicate institutional consequences.
4. Prior approved decisions, historical evaluations and published business facts remain historically traceable.
5. Technical retry, queue, lock, transaction, orchestration, persistence or deduplication mechanisms are deferred to later architecture levels.

---

## 4. Business Actor Boundary

System-internal capabilities may perform calculation, evaluation or publication activity where the original L2-D05 architecture permits it, but they shall not be represented as institutional business actors or governance decision authorities in L3.

L3 actor sections shall use institutional business roles and authorities. Where automated execution matters to the scenario, it may be described only as implementation-independent automated execution behavior and shall not acquire business approval or ownership authority.

---

## 5. Preserved Architecture

This amendment does **not** change:

- D05 ownership of Academic Monitoring.
- Monitoring Case, Monitoring Evaluation, Academic Health Assessment, Academic Risk Assessment, Governance Exception, Monitoring Intervention, Monitoring Snapshot or related ownership already established upstream.
- Existing lifecycle states.
- Existing business rules.
- Existing canonical event names or publishers.
- Existing governance authority.
- Existing cross-domain ownership boundaries.

---

## 6. Downstream Rule

L3 scenarios derived from L2-D05 shall:

- express recovery as business continuity or governed correction, not technical retry design;
- express duplicate-prevention as one authoritative business effect, not an implementation mechanism;
- keep automated execution separate from institutional actor/authority definitions;
- preserve exact upstream event names where L2-D05 defines them; and
- avoid introducing APIs, queues, persistence, locks, transactions, services or other L4/L5 mechanics.

---

## 7. Traceability

Authoritative sources:

- `architecture/L2_Domain_Workflow_Architecture/section-d/L2-D05_Academic_Monitoring_Workflows.md`
- `governance/constitution/Volume_5_Change_Management.md`
- `architecture/L3_Business_Scenario_Architecture/05_Phase5_Parallel_Authoring_and_Verification/01_Scenario_Documents/section-d/L3-D-027_Academic_Monitoring_Requirement_to_Published_Monitoring_Outcome.md`

The original L2-D05 document and this amendment shall be read together as the canonical L2-D05 architecture for the amended subject.
