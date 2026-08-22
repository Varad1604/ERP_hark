# Campus Operations Evidence to Accreditation Evidence Readiness

# 1. Scenario Metadata

**Scenario ID:** L3-G-002  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Campus Operations Governance  
**Participating Domains:** G, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G00 — Campus Operations Review & Governance; applicable G01–G10 evidence sources; L2-H09 — Accreditation & Compliance Workflows  
**Referenced By:** H09 accreditation evidence and readiness governance

# 2. Scenario Purpose

Provide H09 with governed campus-operations readiness and governance evidence without transferring ownership of the underlying operational facts or the institutional accreditation-readiness decision.

# 3. Business Outcome

G00 assesses and publishes the campus-operations evidence package for H09 without making the institutional accreditation-readiness decision

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

An L2-H09 accreditation cycle requests campus governance and operational-readiness evidence.

## 4.2 End Boundary

The scenario ends when G00 has assessed the applicable campus-operations evidence, identified unresolved gaps, and made the governed campus-operations evidence output available to H09. H09 remains owner of institutional accreditation evidence governance and readiness decisions.

## 4.3 In Scope

- G00 governance history and campus-operational readiness evidence.
- Traceability to operational source domains.
- G00 assessment of campus-operations evidence readiness.
- Identification of gaps affecting the campus-operations evidence contribution.
- Governed hand-off to H09.
- Immutable governance history.

## 4.4 Out of Scope

- Institutional accreditation readiness scoring or approval owned by H09.
- Creation or correction of operational source facts.
- SSR submission or regulatory exchange.
- Inventing an evidence-package aggregate, lifecycle, event, or publisher not defined upstream.

## 4.5 Related Scenarios — Optional

| Relationship | Related Scenario | Boundary Reason |
|---|---|---|
| Related | L3-G-001 — Campus Readiness Assessment to Governance Readiness Decision | Provides the general G00 readiness-assessment lifecycle. |

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Campus Operations Governance / G | Owns campus governance records, readiness assessments, operational-review history, and the campus-operations evidence contribution. |
| Accreditation & Compliance / H09 | Owns institutional accreditation frameworks, evidence governance, accreditation assessments, and institutional readiness decisions. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility | Source-Supported Authority |
|---|---|---|
| Governance Coordinator | Coordinates G00 readiness/governance evidence preparation. | G00-defined governance role. |
| Campus Operations Manager | Reviews campus-operational information used by G00. | G00-defined governance role. |
| IQAC | Receives and governs accreditation evidence and institutional readiness within H09. | H09 governance structure. |
| Department Coordinator | Provides departmental evidence/action updates within H09 governance where applicable. | H09 governance structure. |

# 7. Preconditions and Entry Criteria

- An active H09 accreditation cycle or governed accreditation evidence need exists.
- G00 governance and readiness history relevant to the requested scope exists or its absence can be recorded as a gap.
- Operational source facts remain traceable to their canonical owners.
- The G00 evidence scope does not include institutional accreditation decision ownership.

# 8. Trigger

An L2-H09 accreditation cycle requests campus governance and operational-readiness evidence

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the H09 evidence request and identify the requested campus-operations scope. | G00 / H09 | The evidence boundary is established. |
| 2 | Identify applicable G00 readiness assessments, operational reviews, governance history, risks, and corrective-action evidence. | G00 | Relevant governance evidence is identified. |
| 3 | Obtain referenced operational facts from their canonical G-domain owners without modifying them. | G00 / source G domains | Source provenance is preserved. |
| 4 | Validate the relevance, completeness, and traceability of the campus-operations evidence contribution. | G00 | Evidence gaps are known. |
| 5 | Assess campus-operations evidence readiness using the applicable G00 readiness/governance semantics. | G00 | A G00 evidence-readiness conclusion exists. |
| 6 | Make the governed campus-operations evidence output and unresolved gaps available to H09. | G00 | H09 receives the campus-operations contribution. |
| 7 | Preserve the evidence scope, source references, assessment result, and hand-off history. | G00 | The G00 contribution remains auditable. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Evidence gap | Required source evidence is absent or incomplete | Record the gap and provide only the supported contribution; do not invent evidence. | H09 receives a qualified campus-operations contribution. | G00 |
| Corrected source fact | A source domain later corrects its own operational fact | Reassess the G00 contribution using the corrected canonical source. | A new governed evidence view may be supplied without rewriting prior source history. | Source G domain / G00 |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Evidence provenance cannot be established | H09 cannot rely on that evidence contribution | Exclude or qualify the unsupported item and record the gap. | G00 |
| Required governance review is incomplete | G00 cannot represent the evidence as fully ready | Retain the supported current state and complete the review before an unqualified hand-off. | G00 |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| G00 never modifies operational data owned by G01–G10. | G00 / source domains | Evidence remains reference-based. | L2-G00 §13–§14 |
| G00 provides campus governance evidence and accreditation-evidence readiness outputs. | G00 | G00 owns only its campus-operations contribution. | L2-G00 §9, §25, §29 |
| H09 owns institutional evidence governance, accreditation assessments, and readiness decisions. | H09 | G00 cannot make the institutional accreditation-readiness decision. | L2-H09 §§1–2, H09-10 |
| Governance history remains immutable. | G00 | Prior evidence and assessment history remain reconstructable. | L2-G00 §13, §27 |

# 13. State and Lifecycle Impact

| Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner |
|---|---|---|---|---|
| Campus Readiness Assessment | Applicable existing or newly scheduled G00 assessment | May be used to assess the campus-operations evidence contribution through the G00 lifecycle | Source-defined G00 lifecycle only | G00 |
| H09 Evidence / Assessment | H09-governed lifecycle | Receives the G00 evidence contribution as source input | H09 lifecycle remains independently governed | H09 |
| Operational source records | Existing states | Referenced only | Unchanged | Respective source domain |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G00 may publish its source-defined readiness and governance events when their corresponding transitions occur. Neither G00 nor H09 defines a canonical event specifically meaning “campus-operations evidence package published to H09”; no such event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

| Provider | Consumer | Business Hand-off | Purpose |
|---|---|---|---|
| G00 | H09 | Campus governance history, readiness evidence, source references, and unresolved campus-operations evidence gaps | Supplies the campus-operations contribution to institutional accreditation governance. |
| Source G domains | G00 | Canonical operational facts/readiness information | Supports the G00 evidence assessment without ownership transfer. |

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

H09 owns the institutional accreditation governance request and resulting accreditation-readiness authority. G00 governance roles may prepare/review the campus-operations contribution, but L2-G00 does not name a specific approval role for publication of that contribution.

## 15.2 Separation of Duties, Overrides and Exceptions

G00 cannot approve institutional accreditation readiness, and H09 cannot modify G00 or operational source facts.

## 15.3 Business Audit and Evidence Obligations

The requested scope, source references, G00 assessment, unresolved gaps, and hand-off to H09 must remain traceable without altering source-domain history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Relevant campus governance and readiness evidence has been assessed by G00.
- Source ownership and traceability remain preserved.
- Evidence gaps are explicitly identified rather than filled by assumption.
- The supported campus-operations evidence contribution is available to H09.
- G00 has not made the institutional accreditation-readiness decision.

## 16.2 Postconditions

- H09 may continue its own evidence and readiness lifecycle using the G00 contribution.
- G00 and source-domain history remain unchanged except for new governed G00 evidence/history created by this scenario.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| L2-G00 defines accreditation evidence outputs but no canonical evidence-package aggregate or dedicated lifecycle | The frozen hand-off can be described, but downstream L4 must not invent package state | Define a package lifecycle only if required downstream |
| No canonical event/publisher exists specifically for publication of the campus-operations evidence contribution to H09 | Event-driven implementation cannot infer one | Define the event upstream if required |
| L2-G00 does not name a specific business role that approves publication of the G00 accreditation evidence contribution | Final publication authority is not explicit | Define the authority mapping if required |

**Architecture assessment:** `PASS WITH ISSUES`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No scenario-specific deferred questions.
