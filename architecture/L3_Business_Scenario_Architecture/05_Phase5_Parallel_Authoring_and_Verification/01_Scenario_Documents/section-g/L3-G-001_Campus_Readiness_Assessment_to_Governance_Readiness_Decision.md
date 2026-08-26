# Campus Readiness Assessment to Governance Readiness Decision

# 1. Scenario Metadata

**Scenario ID:** L3-G-001  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G, A, B, C, D, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.1  
**Owner:** Dhruv  
**Last Updated:** 2026-08-26  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance; L2-G00 Locked Amendment 2026-08-26; ADR-G-READINESS-001; applicable G01–G10 readiness sources; L2-I03 where the configured readiness type is Examination  
**Referenced By:** Authorized campus-governance and institutional consumers of the completed readiness decision

# 2. Scenario Purpose

Govern one configured campus-operational readiness assessment using readiness information published by canonical operational owners while preserving ownership of the associated admission, academic, examination, event, emergency or other institutional activity.

# 3. Business Outcome

Current operational indicators are consolidated, classified, reviewed and conclusively closed by the `Executive Reviewer` with a G00 governance readiness decision that informs—but does not take ownership of—the associated institutional activity.

# 4. Scenario Boundary and Scope

**Start:** A configured Campus Readiness Assessment is scheduled.  
**End:** The assessment reaches `Closed` with source references, calculated classification, Executive Reviewer decision and governance history preserved.

In scope: readiness information from G01–G10, configured readiness type/rules, calculation, classification, executive review, G00 decision and readiness events. Out of scope: modifying operational source facts, approving another domain's lifecycle, dashboard/notification ownership, and implementation mechanisms.

# 5. Ownership and Participating Domains

G00 owns the Campus Readiness Assessment, classification, governance decision and history. G01–G10 retain ownership of source readiness facts. B/C/D/I retain ownership of associated institutional lifecycles. H may consume governed readiness information. A supplies platform authorization/workflow/event/audit capability without owning readiness facts or decisions.

The frozen participating-domain value remains `G, A, B, C, D, H, I`.

# 6. Business Actors, Roles and Responsibilities

| Actor | Responsibility |
|---|---|
| Governance Coordinator | Creates and coordinates the Campus Readiness Assessment. |
| Campus Operations Manager | Conducts operational review where required by the configured governance path. |
| Executive Reviewer | Makes the final G00 governance readiness decision and closes the assessment. |

The Executive Reviewer has no authority through this scenario to change an Examination, admission, academic-delivery, event, emergency or other externally owned lifecycle.

# 7. Preconditions and Entry Criteria

- The readiness type and assessment rules are configured.
- Applicable source readiness information is available or can be treated as a governed exception.
- Assessment scope can be resolved.
- Governing authorization conditions are satisfied.

# 8. Trigger

A configured campus readiness assessment is scheduled for a supported readiness type.

# 9. Main Business Flow

| Step | Behaviour | Responsible Role / Domain | Result |
|---|---|---|---|
| 1 | Create/schedule the assessment. | Governance Coordinator / G00 | Assessment enters `Scheduled`; `CampusReadinessAssessmentScheduled` is published. |
| 2 | Obtain applicable readiness information from canonical operational owners. | G00 / G01–G10 | Assessment enters `Collecting`; source ownership remains unchanged. |
| 3 | Validate assessment scope and source attribution. | G00 | Only applicable source facts participate. |
| 4 | Calculate the configured readiness result. | G00 | Assessment reaches `Calculated`; `CampusReadinessCalculated` is published with/for the resulting classification. |
| 5 | Classify as `Ready`, `Ready with Observations`, `Partially Ready`, or `Not Ready`. | G00 | Canonical readiness classification exists. |
| 6 | Review the classification, observations and material exceptions. | Campus Operations Manager / Executive Reviewer as applicable | Assessment reaches `Reviewed`. |
| 7 | Make the final G00 governance readiness decision. | Executive Reviewer / G00 | Decision permits, restricts or defers the associated activity at the campus-operational governance level only. |
| 8 | Close the assessment and preserve history. | Executive Reviewer / G00 | Assessment reaches `Closed`. |

# 10. Alternate Business Flows

- Missing source readiness information is treated as a governance exception; no readiness fact is manufactured.
- Examination Readiness remains a G00 campus-operational assessment; Section I retains Examination planning, approval, scheduling and conduct.
- `Ready with Observations` and `Partially Ready` are preserved as exact classifications without introducing new event names.

# 11. Failure, Interruption and Business Recovery

- Missing/invalid readiness inputs prevent reliable calculation until corrected, deferred or escalated.
- Out-of-scope source information is excluded.
- If executive review/decision cannot complete, the assessment remains at its current valid lifecycle position and is not closed.

# 12. Business Rules and Decision Constraints

- G00 references readiness information published by operational domains and never rewrites their facts.
- Lifecycle is `Scheduled → Collecting → Calculated → Reviewed → Closed`.
- Governance history remains immutable and transitions auditable.
- `Executive Reviewer` is the final G00 readiness-decision authority.
- `CampusReadinessCalculated` represents any of the four calculated classifications.
- `CampusReady` is additionally published only for exact `Ready`.
- `CampusNotReady` is additionally published only for exact `Not Ready`.
- G00 readiness cannot become an approval decision for an externally owned lifecycle.

# 13. State and Lifecycle Impact

| Concept | Impact | Result | Owner |
|---|---|---|---|
| Campus Readiness Assessment | Full governed progression | `Scheduled → Collecting → Calculated → Reviewed → Closed` | G00 |
| Source readiness facts | Referenced only | Unchanged | Respective G01–G10 owner |
| Associated institutional activity | Informed by G00 decision | No G00-owned transition introduced | Respective owner |

# 14. Business Events and Cross-Domain Collaboration

| Event | Publisher | Rule |
|---|---|---|
| `CampusReadinessAssessmentScheduled` | G00 | Assessment scheduled. |
| `CampusReadinessCalculated` | G00 | Published for all four calculated classifications. |
| `CampusReady` | G00 | Published additionally only for exact `Ready`. |
| `CampusNotReady` | G00 | Published additionally only for exact `Not Ready`. |

A supplies authorization/workflow/event-transport/audit capability where applicable; it does not publish G00 readiness facts.

# 15. Business Authorization, Approval and Audit Obligations

Governance Coordinator creates/co-ordinates the assessment; Campus Operations Manager performs operational review where required; Executive Reviewer owns the final G00 readiness decision and closure. Audit evidence preserves source references, calculation, classification, review, decision and events.

# 16. Completion Criteria and Postconditions

- Complete valid assessment lifecycle reaches `Closed`.
- Source ownership is preserved.
- One of the four canonical classifications is recorded.
- Executive Reviewer records the final G00 governance readiness decision.
- Applicable events and immutable governance history are preserved.

# 17. Open Issues, Assumptions and Deferred Questions

**Open Issues:** None.  
**Assumptions:** None.  
**Deferred Questions:** None.  
**Architecture assessment:** `PASS`
