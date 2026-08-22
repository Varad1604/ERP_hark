# Campus Readiness Assessment to Governance Readiness Decision

# 1. Scenario Metadata

**Scenario ID:** L3-G-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G, A, B, C, D, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance; applicable G01–G10 readiness sources; L2-I03 — Examination Planning & Scheduling where the configured readiness type is Examination  
**Referenced By:** Authorized campus-governance and institutional consumers of the completed readiness decision

# 2. Scenario Purpose

Govern one configured campus-operational readiness assessment using readiness information published by its canonical operational owners, while preserving the ownership of the associated admission, academic, examination, event, emergency, or other institutional activity.

# 3. Business Outcome

Current operational indicators for the applicable readiness type are consolidated, reviewed, and formally closed with a governance readiness decision, permitting, restricting, or deferring the associated activity

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

A configured campus readiness assessment is scheduled for a supported readiness type (daily, academic session, examination, admission, event, or emergency).

## 4.2 End Boundary

The scenario ends when the Campus Readiness Assessment has reached `Closed` with its source references, calculated readiness result, review evidence, and governance readiness decision preserved. The readiness decision governs campus-operational readiness only; it does not take ownership of the associated activity from its canonical domain.

## 4.3 In Scope

- One configured G00 Campus Readiness Assessment.
- Published readiness information from applicable Campus Operations domains.
- The configured readiness type, indicators, thresholds, and assessment rules.
- Consolidation and calculation of the G00 readiness result.
- Review and recording of the governance readiness decision.
- Source-defined readiness lifecycle and governance events.
- Immutable governance history.

## 4.4 Out of Scope

- Execution or correction of operational work owned by G01–G10.
- Examination planning, scheduling, approval, conduct, or examination-domain readiness decisions owned by Section I.
- Admission, academic-session, or other associated-activity lifecycle decisions owned outside G00.
- Dashboard composition or notification delivery owned by Section H.
- Inventing an approval authority, readiness state, or event not defined upstream.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Follow-up | L3-G-003 — Campus Operational Review to Governance Closure | A readiness concern may require a separate governance review. |
| Follow-up | L3-G-005 — Operational Risk Identification to Risk Closure | Material readiness concerns may be governed as operational risks. |
| Follow-up | L3-G-006 — Multi-Domain Action Plan to Verified Closure | A governance finding may require corrective action coordination. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Campus Operations Governance / G | Canonical owner of the Campus Readiness Assessment, its lifecycle, readiness result, governance decision, and governance history. |
| Platform Foundation / A | Participates according to the frozen readiness-variant lineage; no G00 business ownership is transferred to Section A. |
| Institution & Structure / B | Retains ownership of the institutional context associated with applicable readiness types; G00 determines only campus-operational readiness. |
| Student Lifecycle / C | Retains ownership of admission/student-lifecycle decisions when Admission Readiness is the configured type. |
| Academic Delivery / D | Retains ownership of academic-session and academic-delivery decisions when the readiness assessment supports those activities. |
| Institutional Services & Intelligence / H | Consumes governed readiness information within its own reporting, notification, dashboard, or governance boundaries; it does not own the G00 assessment. |
| Examination / I | Retains Examination planning, scheduling, approval, and conduct ownership when Examination Readiness is the configured type. |

The frozen participating-domain value is preserved exactly as `G, A, B, C, D, H, I`.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Source-Supported Authority |
|---|---|---|
| Governance Coordinator | Creates and coordinates the Campus Readiness Assessment within authorized scope. | L2-G00 authorization matrix assigns `Create Readiness Assessment` to this role. |
| Campus Operations Manager | Reviews campus-operational information where an operational review is required. | L2-G00 authorization matrix assigns conduct of operational review to this role. |
| Executive Reviewer | Participates in the source-defined executive review step of readiness governance. | L2-G00 defines this as a typical governance role; no universal final approval authority is assigned. |

No additional actor is inferred for the associated admission, academic, event, emergency, or Examination activity.

# 7. Preconditions and Entry Criteria

- The readiness type is one supported by G00 and is configured for the relevant governance scope.
- Applicable readiness indicators are available from their canonical operational owners, or their absence can be explicitly treated as a governance exception.
- The assessment scope and applicable readiness rules can be resolved.
- G00 has not assumed ownership of the underlying operational facts or associated institutional activity.
- The governance activity is authorized under the applicable institutional policy.

# 8. Trigger

A configured campus readiness assessment is scheduled for a supported readiness type (daily, academic session, examination, admission, event, or emergency)

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Create the readiness assessment for the configured readiness type. | Governance Coordinator / G00 | The Campus Readiness Assessment enters `Scheduled`. |
| 2 | Resolve the configured indicators and obtain the applicable published readiness information from its owning operational domains. | G00 | The assessment enters `Collecting` without changing source-domain facts. |
| 3 | Validate that the readiness information belongs to the required assessment scope and remains attributable to its canonical owner. | G00 | Only applicable source information participates in the assessment. |
| 4 | Consolidate the validated indicators and calculate the configured readiness result. | G00 | The assessment reaches `Calculated`. |
| 5 | Record the resulting classification as one of the source-defined readiness results: `Ready`, `Ready with Observations`, `Partially Ready`, or `Not Ready`. | G00 | A governed readiness result exists. |
| 6 | Review the calculated result and unresolved observations. | Executive Reviewer / G00 | The assessment reaches `Reviewed`. |
| 7 | Record the governance readiness decision permitting, restricting, or deferring the associated activity, without changing that activity’s canonical lifecycle. | G00 | The frozen governance decision is recorded. |
| 8 | Close the assessment and preserve its readiness result, decision, source references, and review history. | G00 | The assessment reaches `Closed` and the frozen business outcome is complete. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome |
|---|---|---|---|
| Missing readiness information | Required operational readiness information is unavailable or inconsistent | Treat the condition as a G00 governance exception; do not manufacture a readiness fact. | The assessment remains incomplete until corrected, deferred, or escalated under governed recovery. |
| Examination readiness | The configured type is Examination Readiness | G00 assesses campus-operational readiness only; Section I continues to own Examination planning, scheduling, approval, and conduct. | No ownership collision occurs. |
| Qualified readiness | The calculated result is `Ready with Observations` or `Partially Ready` | Preserve the exact G00 classification and any follow-up governance need. | The decision reflects the configured readiness result without inventing a new state or event. |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Required readiness information is missing | G00 cannot calculate a reliable result | Correct, defer, or escalate the governance exception while preserving source ownership. | G00 and the owning operational domain |
| Source information is outside the assessment scope | The result could be misleading | Exclude the inapplicable information and retain the valid assessment scope. | G00 |
| Review cannot be completed | The assessment cannot be formally closed | Retain the current source-defined lifecycle position until review can be completed. | G00 |

Recovery remains business-level; no technical retry, queue, rollback, or infrastructure mechanism is specified.

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Campus readiness shall reference readiness information published by operational domains. | G00 | G00 cannot manufacture or take ownership of source readiness facts. | L2-G00 §13, §16 |
| G00 shall never modify operational data owned by G01–G10. | G00 / source domains | Readiness inputs remain reference-only. | L2-G00 §13–§14 |
| Readiness types and assessment rules remain configurable. | G00 | Daily, academic session, Examination, admission, event, and emergency readiness remain variants of one workflow. | L2-G00 §16, §26 |
| The canonical lifecycle is `Scheduled → Collecting → Calculated → Reviewed → Closed`. | G00 | No local lifecycle state may be added or skipped. | L2-G00 §23 |
| Governance history remains immutable and every lifecycle transition is audited. | G00 | The completed assessment remains reconstructable. | L2-G00 §13, §27 |
| Examination planning and scheduling remain owned by the Examination Domain. | Examination / I | G00 readiness cannot become an Examination schedule or approval decision. | Phase 2 SG-01 resolution; L2-I03 |

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Entry State or Condition | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Campus Readiness Assessment | Configured assessment scheduled | Collects, calculates, reviews, and closes the assessment | `Scheduled → Collecting → Calculated → Reviewed → Closed` | Campus Operations Governance / G00 |
| Operational source fact | Existing authoritative state | Referenced only | Unchanged | Respective operational domain |
| Associated admission, academic, event, emergency, or Examination activity | Existing owner-controlled state | Receives or considers the G00 governance readiness decision where applicable | No G00-owned transition is introduced | Respective canonical domain |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Canonical Business Events

L2-G00 explicitly states that Campus Operations Review publishes immutable governance domain events. Therefore the canonical publisher for the following G00 events is **Campus Operations Review (G00)**.

| Event | Publisher | Business Meaning in this Scenario |
|---|---|---|
| `CampusReadinessAssessmentScheduled` | Campus Operations Review (G00) | The readiness assessment has been scheduled. |
| `CampusReadinessCalculated` | Campus Operations Review (G00) | The configured readiness calculation has completed. |
| `CampusReady` | Campus Operations Review (G00) | The source-defined ready outcome has been reached where applicable. |
| `CampusNotReady` | Campus Operations Review (G00) | The source-defined not-ready outcome has been reached where applicable. |

No new event is created for `Ready with Observations` or `Partially Ready` because L2-G00 does not define one.

## 14.2 Business Contracts and Hand-offs

| Provider | Consumer | Business Hand-off | Ownership Effect |
|---|---|---|---|
| Applicable G01–G10 operational domain | G00 | Published readiness indicator or operational readiness information | Source domain retains ownership. |
| G00 | Relevant authorized institutional domain | Closed campus-operational readiness decision | Consumer may use the decision but does not acquire G00 ownership. |
| G00 | Section H where applicable | Governed readiness information for authorized intelligence/notification/dashboard use | Section H retains only its own downstream responsibilities. |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Authorization and Decision Authority

L2-G00 assigns creation of a Campus Readiness Assessment to the `Governance Coordinator` and defines `Executive Reviewer` as a typical governance role. It does not explicitly assign a universal institutional role as the final authority for every readiness decision. This Draft therefore does not invent a final approver.

## 15.2 Separation of Duties and Exceptions

No universal separation-of-duty rule for readiness decisions is defined in L2-G00. Source-domain owners remain separate from G00 governance ownership, and G00 cannot alter their operational facts.

## 15.3 Audit Obligations

The assessment schedule, source references, calculated result, review, decision, lifecycle transitions, and applicable published events must remain in immutable governance history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The configured assessment has progressed through the complete G00 lifecycle to `Closed`.
- Applicable source readiness information remains attributable to its canonical owner.
- A source-defined readiness result has been calculated and reviewed.
- The governance readiness decision permitting, restricting, or deferring the associated activity has been recorded.
- Governance history and applicable canonical events are preserved.

## 16.2 Postconditions

- The Campus Readiness Assessment is closed and auditable.
- Operational source facts remain unchanged by G00.
- The associated institutional activity remains governed by its canonical owner.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G00 does not explicitly assign a universal final business authority for the readiness decision | The Draft can model the complete G00 lifecycle and outcome, but downstream L4 must not invent a final approver | Define the applicable decision-authority mapping if a named institutional approver is required |
| L2-G00 defines `Ready with Observations` and `Partially Ready` results but no corresponding canonical events | Those classifications can be preserved in the assessment result, but a distinct event cannot be emitted without upstream definition | Define additional events only if cross-domain consumers require distinct publication facts |
| Frozen participant A is retained from the absorbed Emergency Readiness lineage, but G00 does not define a specific G001 business responsibility for Section A | Ownership remains safe, but detailed A participation cannot be expanded locally | Clarify the business hand-off if downstream specification requires one |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
