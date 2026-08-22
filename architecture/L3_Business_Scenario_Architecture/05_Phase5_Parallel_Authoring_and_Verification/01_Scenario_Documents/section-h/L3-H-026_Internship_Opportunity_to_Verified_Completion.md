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
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H04 — Placement Workflows; applicable locked Student Lifecycle, Academic Delivery, and People & Workforce architecture  
**Referenced By:** Internship recruitment, internship progress, placement conversion, placement analytics, and audit activities

# 2. Scenario Purpose

Govern an institutional internship opportunity from eligible Student participation through Internship start, progress, completion evidence, and optional placement conversion while preserving source-domain ownership and Placement's Internship lifecycle.

# 3. Business Outcome

Participation, evaluation, and completion evidence produce a governed internship outcome

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an approved internship opportunity is assigned to or made available for an eligible Student under Placement governance.

## 4.2 End Boundary

The frozen end boundary requires participation, evaluation, and completion evidence to produce a verified governed internship outcome. H04 defines Internship completion and audit evidence but does not define a canonical internship evaluation/verification workflow, decision authority, or verification event; therefore the full verified-completion boundary cannot be established authoritatively.

## 4.3 In Scope

- Publishing the internship opportunity.
- Registering an eligible Student/candidate.
- Completing configured screening/recruitment required by the internship.
- Generating the internship offer and starting the Internship.
- Preserving internship progress, Evaluation Record, Completion Record, and completion details defined by H04.
- Completing the Internship and optionally converting a completed Internship to placement.
- Preserving immutable conversion and completion history.

## 4.4 Out of Scope

- Defining a new internship-evaluation workflow or verification authority.
- Changing canonical Student, academic, or workforce records.
- Finance/stipend settlement beyond the Placement-owned internship information.
- General placement offer/joining outcomes outside internship-specific flow.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains Student identity/lifecycle information referenced by Internship participation. |
| Academic Delivery / Section D | Retains applicable academic-delivery information used for eligibility/evaluation where authoritative. |
| People & Workforce / Section E | Retains workforce identity/assignment information for Placement staff and any source-owned workforce participants. |
| Placement / Section H | Owns the Internship aggregate, internship application/selection/start/completion/conversion history, and Placement-specific completion evidence. |

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Placement Officer | Publishes/manages the H04 internship lifecycle and records supported completion/conversion activity. | Performs Placement-owned Internship actions. |
| Student | Participates in the governed internship opportunity once selected/eligible. | Provides candidate/internship participation. |

H04 does not define a canonical Internship evaluator or verification approver. No such role is invented.

# 7. Preconditions and Entry Criteria

- An Internship opportunity exists under Placement governance.
- The Student is eligible for participation using authoritative source information.
- Required recruitment/screening for the internship is complete or available.
- The Internship can reference one employer and one Student assignment as H04 requires.
- Completion/verification criteria beyond H04's defined Completion Record are not assumed.

# 8. Trigger

An approved internship opportunity is assigned to an eligible student

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Publish the Internship opportunity and register the eligible candidate. | Placement Officer / Student / Placement | `InternshipPublished` and `InternshipCandidateRegistered` establish governed participation. |
| 2 | Complete configured screening/recruitment and generate the Internship offer for the selected candidate. | Placement | `InternshipOfferGenerated` records the selected Internship opportunity. |
| 3 | Start the Internship after the governed selection/offer conditions are satisfied. | Placement Officer / Placement | `InternshipStarted` records entry to `In Progress`. |
| 4 | Preserve H04-supported progress information, Evaluation Record references, and completion evidence during the Internship. | Placement | Internship history remains traceable. |
| 5 | When the Internship is complete, apply `CompleteInternship` and publish `InternshipCompleted`. | Placement Officer / Placement | The Internship reaches `Completed`. |
| 6 | If permitted after completion, convert the Internship to a placement and preserve conversion history. | Placement Officer / Placement | Optional conversion follows H04. |

H04 does not define the business decision/authority that verifies the Evaluation Record and completion evidence before `CompleteInternship`, so the frozen “verified completion” semantics remain unresolved.

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Optional conversion | Completed Internship qualifies for conversion. | Apply `ConvertInternshipToPlacement`. | Conversion history is preserved and Internship closes under H04. | Placement |
| Conversion denied/not pursued | Internship completes without conversion. | Preserve `InternshipCompleted` as the Internship outcome. | Internship completion remains recorded without conversion. | Placement |
| Internship cancellation/withdrawal | H04 lists cancellation/withdrawal as failure scenarios but provides no Internship cancellation/withdrawal state, transition, or event. | Do not invent a terminal Internship cancellation lifecycle. | **Unsupported branch remains unresolved upstream.** | Placement |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Completion evidence is incomplete | Verified completion cannot be established. | Do not infer verification; preserve the incomplete state and require upstream-governed resolution. | Placement |
| Evaluation outcome is disputed or absent | Frozen final outcome cannot be validated. | Preserve available Evaluation Record/history and route for governed resolution; no evaluator authority is invented. | Placement |
| Internship is cancelled or candidate withdraws | H04 identifies the condition but no canonical terminal lifecycle is defined. | Record the architecture gap rather than inventing cancellation/withdrawal states. | Placement |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Internship belongs to one employer and the Internship assignment belongs to one Student. | Placement | Internship identity/assignment remains singular. | L2-H04 §17.7 |
| Completed Internships cannot be modified. | Placement | Completion history becomes immutable. | L2-H04 §17.7 |
| Internship completion precedes conversion and conversion history remains immutable. | Placement | Conversion cannot bypass completion. | L2-H04 §17.7 |
| Internship workflows follow configured recruitment templates. | Placement | Candidate progression must respect Placement recruitment governance. | L2-H04 §19.17 |
| Internship conversion is optional. | Placement | Completed Internship may remain unconverted. | L2-H04 §19.17 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Internship | `Created` / `Published` / `Applications Open` / `Selected` | Candidate participates, Internship starts, completion is recorded, optional conversion may follow. | `Completed`, or closed after optional conversion | Placement | L2-H04 §17.7 |
| Source Student / academic / workforce facts | Current canonical states | Referenced for eligibility/participation only. | Unchanged by Placement. | C / D / E as applicable | `DEPENDENCY_MAP.md`; H04 ownership boundary |

H04 defines `Evaluation Record` and `Completion Record` child entities, but no canonical evaluation state machine or verification transition is defined.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Exact Placement internship events are `InternshipPublished`, `InternshipCandidateRegistered`, `InternshipOfferGenerated`, `InternshipStarted`, `InternshipCompleted`, and `InternshipConvertedToPlacement`.

H04 defines no canonical `InternshipEvaluated`, `InternshipCompletionVerified`, or equivalent verification event.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Placement references Student, academic, and workforce information from their canonical owners. H04 preserves only Placement-owned Internship state/evidence. No cross-domain ownership transfer or invented evaluation contract is introduced.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

Placement Officer is source-supported for Internship lifecycle actions. H04 does not define the business actor/authority that evaluates or verifies Internship completion evidence.

## 15.2 Separation of Duties, Overrides and Exceptions

No additional verification separation-of-duty rule is defined. L3 cannot create a local evaluator/approver merely to satisfy the frozen title/outcome.

## 15.3 Business Audit and Evidence Obligations

H04 requires audit capture of Internship lifecycle, employer, Student, conversion decision, and completion details. Evaluation Record and Completion Record history must remain traceable where present.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

H04 can authoritatively complete the Internship lifecycle through `InternshipCompleted` and optional conversion. The frozen verified-completion outcome additionally requires evaluation and completion evidence to be governed by a defined verification decision/authority, which H04 does not provide.

## 16.2 Postconditions

- Supported Internship completion/conversion history remains immutable.
- Placement retains Internship ownership.
- Student, academic, and workforce source ownership remains unchanged.
- Verification authority/event remains unresolved.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H04's Internship aggregate contains `Evaluation Record` and `Completion Record`, and the workflow records completion details, but no canonical Internship evaluation workflow, verification decision, verification actor/authority, state transition, or verification event is defined. | The frozen title and outcome require **verified completion** based on participation, evaluation, and completion evidence. L3 cannot invent the missing authority or event. | Define the canonical Internship evaluation/verification semantics and authority upstream. |
| H04 lists “Internship cancelled” and “Candidate withdraws” as failure scenarios but defines no corresponding Internship state, transition, or event. | Exception outcomes cannot be expressed canonically if they are required during this scenario. | Define Internship cancellation/withdrawal lifecycle semantics upstream if those branches are intended. |

**Architecture assessment:** `BLOCKED`

The publication/start/completion/conversion lifecycle is source-supported, but the required verified-evaluation outcome is materially undefined.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
