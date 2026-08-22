# Approved Credential Population to Bulk Publication Outcome

# 1. Scenario Metadata

**Scenario ID:** L3-H-006  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** C, D, F, H, I  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; applicable canonical credential-owning source-domain architecture; Section A Authorization and Audit architecture  
**Referenced By:** Qualification/administrative publication cycles, government credential operations, reconciliation, and audit

# 2. Scenario Purpose

Govern publication of a source-approved population of credentials through H01 so that every credential receives its own traceable H01 publication status/outcome and the institution receives a consolidated batch result without H01 acquiring credential ownership.

# 3. Business Outcome

Each credential reaches a traceable publication outcome and the institution receives a consolidated batch result

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a governed completion or administrative cycle identifies a population of approved credentials for publication.

## 4.2 End Boundary

The scenario ends when H01 has processed the identified population through the individual Publication lifecycle and records a consolidated batch result containing the traceable status/outcome of the population.

## 4.3 In Scope

- Receiving the governed approved credential population from canonical source domains.
- Validating batch eligibility and each credential's publication prerequisites.
- Preserving individual H01 Publication identity/status per credential.
- Applying H01 duplicate-prevention and consent rules to each applicable credential.
- Consolidating population-level publication results.
- Preserving publication and batch evidence.

## 4.4 Out of Scope

- Creating or approving credentials.
- Changing academic, examination, student, or finance meaning/ownership.
- Treating the batch as a substitute for the individual H01 Publication lifecycle.
- Technical processing, integration, or implementation mechanics.
- Controlled recovery of failed/pending publications beyond the batch outcome itself.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Supplies source-owned Student/qualification context where applicable. |
| Academic Delivery / Section D | Supplies source-owned academic context where applicable. |
| Finance / Section F | Frozen participating domain; H01 does not define a universal Finance batch-publication authority. |
| APAAR & DigiLocker / Section H | Owns the individual H01 Publication lifecycles, bulk-publication workflow, consolidated batch result, and H01 publication evidence. |
| Examination / Section I | Supplies approved credential/examination facts where applicable. |

# 6. Business Actors, Roles and Responsibilities

H01 lists qualification completion processing, semester completion, migration exercise, and administrative request as bulk-publication triggers, but it does not define one canonical human actor for all bulk publication cycles. No generic `Bulk Publication Approver` is invented.

# 7. Preconditions and Entry Criteria

- The population is identified by a governed completion or administrative cycle.
- Each included credential is source-approved and publication-eligible according to its canonical owner.
- H01 can resolve required Credential, Student, Institution, consent, and connector context.
- Duplicate completed publications are not treated as new ordinary publications.

# 8. Trigger

A governed completion or administrative cycle identifies a population of approved credentials for publication

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the governed population of source-approved credential references. | APAAR & DigiLocker consuming source-domain facts | H01 receives a publication population without acquiring credential ownership. |
| 2 | Validate the population and preserve each credential as an independently traceable publication subject. | APAAR & DigiLocker | Invalid population entries are identifiable without collapsing individual outcomes. |
| 3 | For each eligible credential, apply the source-defined H01 publication prerequisites, including consent where required and duplicate prevention. | APAAR & DigiLocker | Each credential has a governed individual publication path. |
| 4 | Progress each valid credential through the existing H01 Publication lifecycle and record its publication status/outcome. | APAAR & DigiLocker | Individual results remain traceable. |
| 5 | Collect the population results into the H01 bulk-publication summary. | APAAR & DigiLocker | Consolidated result reflects the individual publication statuses defined by H01. |
| 6 | Preserve the consolidated result and underlying individual publication evidence for institutional use and audit. | APAAR & DigiLocker / Platform Audit | The frozen institutional batch outcome is available without changing source ownership. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| One credential fails H01 validation | Other credentials remain eligible. | Record the invalid credential's controlled H01 outcome and continue processing independent eligible credentials. | APAAR & DigiLocker |
| One credential is already Published | Duplicate publication is detected. | Preserve the existing completed Publication and do not create an unauthorised duplicate. | APAAR & DigiLocker |
| Some publications remain pending recovery | Population processing produces non-terminal individual statuses. | Include those statuses in the consolidated result and route them to the separate controlled-recovery scenario. | APAAR & DigiLocker |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Population contains an unapproved credential | H01 cannot legitimately publish that item. | Exclude/reject that item under H01 validation while preserving source truth. | Source domain / APAAR & DigiLocker |
| Consolidated result and individual results disagree | Institutional batch evidence would be unreliable. | Reconcile the batch summary against authoritative H01 Publication records before closing the batch outcome. | APAAR & DigiLocker |
| Government processing leaves individual items unresolved | Batch contains pending recovery statuses. | Preserve those statuses and route affected Publications to the controlled-recovery scenario; do not fabricate completion. | APAAR & DigiLocker |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H01 never generates or modifies source credentials. | Source domain / APAAR & DigiLocker | Population contains references to source-owned approved credentials only. | L2-H01 §§5–7, 16 |
| Every individual Publication must satisfy credential approval, connector, policy, consent where required, and duplicate rules. | APAAR & DigiLocker | Bulk processing does not bypass individual Publication controls. | L2-H01 §§16, 29, 36, 40 |
| H01 defines a Bulk Publication workflow and consolidated summary. | APAAR & DigiLocker | Batch result is a legitimate H01 business outcome. | L2-H01 §32 |
| H01 defines a consolidated summary of total requests and individual publication results, including successful, failed, and unresolved items. | APAAR & DigiLocker | Consolidated result preserves the source-defined business statuses without importing technical processing mechanics. | L2-H01 §32 |
| Individual Publication history is auditable and retained. | APAAR & DigiLocker / Platform Audit | Batch aggregation cannot erase record-level evidence. | L2-H01 §52 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Source credential population | Source-approved credentials | Referenced only for publication. | Source credential lifecycles remain unchanged by H01. | Applicable source domains | L2-H01 source-authority principle |
| H01 Publication | Source-defined individual Publication states | Each credential progresses independently. | Per-item status may be `Published`, `Failed`, or another source-defined in-progress/recovery state represented in the H01 summary. | APAAR & DigiLocker | L2-H01 §§32, 60 |
| Bulk publication summary | No separate aggregate lifecycle defined | Consolidates individual publication statuses. | Consolidated batch result recorded. | APAAR & DigiLocker | L2-H01 §32 |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

Each individual H01 Publication uses the canonical H01 Publication events, including `CredentialPublicationRequested`, `PublicationCompleted`, and `PublicationFailed` as applicable. H01 does not define a dedicated canonical bulk-publication completion event, so none is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Canonical source domains supply the governed approved credential population. H01 applies its Publication contract per credential and produces a consolidated H01 batch result. The exact source-domain event/publisher for every credential population origin is not universally mapped upstream.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The source completion/administrative cycle must authoritatively identify the approved population. H01 does not define one universal human actor for every bulk-publication trigger, so no additional actor is introduced.

## 15.2 Separation of Duties, Overrides and Exceptions

No universal bulk-publication separation-of-duty rule is defined by H01. Source-domain credential approval remains separate from H01 publication processing.

## 15.3 Business Audit and Evidence Obligations

Preserve the governed population reference, individual Publication identifiers/statuses, source Credential references, government acknowledgements where applicable, consolidated batch result, and immutable publication/audit history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- A governed population of source-approved credentials is identified.
- Each credential is independently validated under H01 publication rules.
- Each item has a traceable H01 Publication status/outcome.
- A consolidated H01 result is produced from those individual statuses.
- Underlying publication and audit evidence remains traceable.
- Source credential ownership remains unchanged.

## 16.2 Postconditions

- The institution has a consolidated H01 bulk-publication result.
- Every included credential remains individually traceable through H01 Publication history.
- Failed/pending items remain eligible only for their source-defined recovery path.
- Source domains retain credential ownership.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H01 defines Bulk Publication as a workflow/summary but does not define a canonical Batch aggregate, batch lifecycle, or batch-completion business event. | L3 can record the consolidated result but cannot introduce an independent batch lifecycle/event. | Define them upstream only if later architecture requires a first-class Batch aggregate/event. |
| H01's summary permits “pending retries,” while the frozen outcome says each credential reaches a traceable publication outcome. | Some batch members may have a traceable non-terminal status rather than terminal publication completion at batch-summary time. | Clarify whether “traceable publication outcome” intentionally includes pending controlled-recovery status. |
| Exact source-domain event/publisher for each governed credential population is not universally mapped, and Finance participation remains undefined for the generic H01 bulk workflow. | Source initiation cannot be expressed as one universal event/role. | Register source-specific population hand-offs if mandatory. |

**Architecture assessment:** `PASS WITH ISSUES`

H01 explicitly defines the bulk-publication workflow, independent individual Publication processing, and consolidated summary. The missing first-class Batch lifecycle/event and ambiguity around pending-recovery status are non-blocking for a Draft that does not invent them.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
