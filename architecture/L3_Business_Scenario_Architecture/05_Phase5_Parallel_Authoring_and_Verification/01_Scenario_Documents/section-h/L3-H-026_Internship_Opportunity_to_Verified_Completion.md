# Internship Opportunity to Verified Completion

# 1. Scenario Metadata

**Scenario ID:** L3-H-026  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Placement  
**Participating Domains:** C, D, E, H  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-23  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; L2-C02 — Student Lifecycle & Activation Workflows; applicable locked Section D Academic Delivery sources including L2-D05 — Academic Monitoring Workflows; L2-E01 — Workforce & Engagement Workflows  
**Referenced By:** Internship analytics and optional placement conversion

# 2. Scenario Purpose

Govern an Internship opportunity through eligible Student participation, start, completion evidence and optional conversion while keeping Student, academic and workforce facts with their canonical owners.

# 3. Business Outcome

Participation, evaluation, and completion evidence produce a governed internship outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved internship opportunity is assigned to an eligible student.

## 4.2 End Boundary

H04 defines Internship start/completion/conversion and contains `Evaluation Record` and `Completion Record`, but it defines no canonical Internship evaluation workflow, verification decision, evaluator/approver, verification state, or verification event. Because the frozen semantics require evaluation and verified completion evidence, the complete end boundary is unresolved.

## 4.3 In Scope

Internship publication/participation, configured recruitment, Internship start, H04-supported progress/evidence, completion, optional conversion.

## 4.4 Out of Scope

Invented evaluation/verification authority or events; Student/academic/workforce lifecycle changes; unsupported cancellation/withdrawal semantics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / C | Owns Student identity/lifecycle. |
| Academic Delivery / D | Supplies only D-owned academic-delivery/monitoring facts where applicable; not official results. |
| People & Workforce / E | Owns workforce context for Placement staff. |
| Placement / H | Owns Internship aggregate, Placement-specific participation/start/completion/conversion history and H04 evidence records. |

# 6. Business Actors, Roles and Responsibilities

Placement Officer is source-supported for Internship lifecycle commands. Student participates in the governed Internship. H04 does not define the actor/authority who evaluates or verifies Internship completion evidence; no evaluator role is invented.

# 7. Preconditions and Entry Criteria

- Internship opportunity exists and is approved for Placement use.
- Student is eligible using authoritative source facts.
- Required recruitment is complete or available.
- Internship references one employer and one Student assignment.

# 8. Trigger

An approved internship opportunity is assigned to an eligible student

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Domain | Business Result |
|---|---|---|---|
| 1 | Publish the Internship opportunity and register the eligible candidate. | H with C reference | Governed Internship participation is established. |
| 2 | Complete configured recruitment and generate the Internship offer. | H | Candidate reaches selected Internship position. |
| 3 | Start the Internship. | H | `InternshipStarted` records `In Progress`. |
| 4 | Preserve H04-supported progress, Evaluation Record reference, and Completion Record information. | H | Internship evidence remains traceable. |
| 5 | Complete the Internship and publish `InternshipCompleted`. | H | H04 completion state is reached. |
| 6 | If permitted, convert the completed Internship to Placement. | H | Optional conversion history is preserved. |
| 7 | Establish the required verified-evaluation decision over the completion evidence. | H | **BLOCKED:** evaluator/verification lifecycle is undefined upstream. |

# 10. Alternate Business Flows

A completed Internship may remain unconverted or may convert to Placement after completion. H04 lists internship cancellation/candidate withdrawal as failure scenarios but defines no state/transition/event for those branches.

# 11. Failure, Interruption and Business Recovery

Incomplete completion evidence cannot be treated as verified. Disputed/absent evaluation remains unresolved under upstream governance. Cancellation/withdrawal branches are not invented.

# 12. Business Rules and Decision Constraints

| Rule / Constraint | Owner | Scenario Effect |
|---|---|---|
| Internship belongs to one employer and one Student assignment. | H | Internship identity/participation is singular. |
| Completed Internships cannot be modified. | H | Completion history is immutable. |
| Completion precedes conversion and conversion history is immutable. | H | Conversion cannot bypass completion. |
| Internship recruitment follows configured Placement templates. | H | Participation/recruitment remains governed. |
| D does not own official examination results. | I under dependency governance | H04's stale D result wording cannot transfer result ownership. |

# 13. State and Lifecycle Impact

Internship progresses `Created → Published → Applications Open → Selected → In Progress → Completed`, with optional conversion to `Closed`. H04 defines Evaluation Record/Completion Record children but no evaluation/verification state machine. C/D/E source lifecycles remain unchanged.

# 14. Business Events and Cross-Domain Collaboration

Exact H04 Internship events are `InternshipPublished`, `InternshipCandidateRegistered`, `InternshipOfferGenerated`, `InternshipStarted`, `InternshipCompleted`, and `InternshipConvertedToPlacement`. No canonical Internship evaluation or completion-verification event exists.

# 15. Business Authorization, Approval and Audit Obligations

Placement Officer is source-supported for H04 Internship lifecycle actions. No source defines the evaluation/verification decision authority required by the frozen outcome. Evidence must preserve Internship, employer, Student, participation, progress, Evaluation/Completion Record references where present, completion, conversion decision, and immutable history.

# 16. Completion Criteria and Postconditions

H04 can reach `InternshipCompleted` and optional conversion. Full frozen completion is blocked because evaluation and verification authority/semantics are undefined. Source-domain ownership remains unchanged.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H04 contains Evaluation Record/Completion Record but no canonical evaluation workflow, verification decision/authority, state or event. | Frozen verified-completion outcome cannot be grounded. | Define Internship evaluation/verification semantics upstream. |
| H04 lists Internship cancellation/withdrawal failures without corresponding states/transitions/events. | Exception branches cannot be canonically expressed. | Define them upstream if required. |
| H04's D/result dependency wording conflicts with the locked ownership map. | Official result ownership must not transfer to D. | Correct stale H04 dependency wording. |

**Architecture assessment:** `BLOCKED`

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional deferred questions.
