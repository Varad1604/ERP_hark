# Institutional and Government Credential Reconciliation

# 1. Scenario Metadata

**Scenario ID:** L3-H-007  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** APAAR & DigiLocker  
**Participating Domains:** C, D, F, H, I  
**Scenario Type:** Cross-domain  
**Priority:** P1  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-H01 — APAAR & DigiLocker Workflows; applicable canonical credential-owning source-domain architecture; Section A Authorization and Audit architecture  
**Referenced By:** Government credential audit, publication recovery, revocation, corrected republication, and institutional assurance activities

# 2. Scenario Purpose

Compare authoritative institutional publication references with government records through the H01 Synchronization lifecycle so discrepancies become classified reconciliation outcomes or governed follow-up without overwriting either system's history or transferring credential ownership to H01.

# 3. Business Outcome

Discrepancies are classified and resolved or routed to governed follow-up while both systems' histories are preserved

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when a scheduled review, audit, or recovery request requires comparison with government records.

## 4.2 End Boundary

The scenario ends when H01 completes the comparison, records the reconciliation outcome, and either records the discrepancy as resolved through an authorised source-defined path or routes it to governed follow-up while histories remain preserved.

## 4.3 In Scope

- Starting an H01 Synchronization/reconciliation activity.
- Comparing government records with authoritative H01/source-domain publication references.
- Classifying source-defined discrepancy outcomes.
- Recording reconciliation status and report/evidence.
- Routing unresolved discrepancies to governed follow-up without changing source-owned credential facts.
- Preserving synchronization, publication, source, and government history.

## 4.4 Out of Scope

- Correcting academic/examination/financial/student source facts inside H01.
- Inventing source-domain correction, revocation, republication, or recovery authority.
- Treating a discrepancy report as a new canonical credential.
- Technical synchronization or implementation mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains source-owned Student/qualification facts relevant to reconciliation where applicable. |
| Academic Delivery / Section D | Retains source-owned academic facts relevant to reconciliation where applicable. |
| Finance / Section F | Frozen participating domain; H01 defines no universal Finance reconciliation correction authority. |
| APAAR & DigiLocker / Section H | Owns Synchronization/reconciliation status, discrepancy detection/classification, reconciliation reports, and H01 follow-up routing. |
| Examination / Section I | Retains examination/credential source facts where applicable. |

# 6. Business Actors, Roles and Responsibilities

H01 permits scheduled synchronization, administrative request, connector recovery, and audit activity as triggers. It does not define a canonical human business actor for generic reconciliation. Therefore no new reconciliation actor is invented. Where an audit or administrative business request exists, its authority remains with the applicable upstream governance context.

# 7. Preconditions and Entry Criteria

- H01 has authoritative Publication history and the relevant government connector/reference context.
- Canonical source credentials remain available from their owning domains.
- The reconciliation request is authorised under the applicable business context.
- Existing synchronization state permits a new source-defined reconciliation execution.

# 8. Trigger

A scheduled review, audit, or recovery request requires comparison with government records

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Initiate an H01 reconciliation for the applicable connector/publication population. | APAAR & DigiLocker | A Synchronization activity is established. |
| 2 | Resolve authoritative institutional Publication references and compare them with government records. | APAAR & DigiLocker consuming source-owned references | H01 compares references without changing source facts. |
| 3 | Identify and classify discrepancies using the outcomes defined by H01. | APAAR & DigiLocker | Differences are distinguished as missing publication, government mismatch, duplicate publication, revoked credential detection, fully synchronized, or manual-review requirement. |
| 4 | Record the reconciliation report/status and preserve both institutional and government evidence. | APAAR & DigiLocker / Platform Audit | Reconciliation evidence becomes traceable. |
| 5 | Where a discrepancy can be resolved only by a source-owned correction, H01 routes the matter to the applicable governed source/revocation/republication/recovery path rather than modifying the source credential. | APAAR & DigiLocker / source-owning domain | Ownership remains canonical and unresolved discrepancies enter governed follow-up. |
| 6 | Complete the H01 Synchronization when its comparison/reporting obligations are satisfied. | APAAR & DigiLocker | Synchronization reaches `Completed`, or `Failed` if the source-defined synchronization cannot complete. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Fully synchronized | No discrepancy exists. | Record the fully synchronized outcome and complete H01 Synchronization. | Reconciliation closes without source changes. | APAAR & DigiLocker |
| Missing government publication | Institutional record exists but expected external publication does not. | Classify the missing publication and route to the applicable publication/recovery path. | Governed follow-up is established. | APAAR & DigiLocker |
| Government mismatch | Government and institutional references disagree. | Preserve both histories and route to the source-defined correction/reconciliation authority. | H01 does not overwrite source truth. | APAAR & DigiLocker / source domain |
| Duplicate publication detected | More than one relevant government publication is identified. | Classify duplicate condition and route to governed H01/source follow-up. | Duplicate condition remains traceable. | APAAR & DigiLocker |
| Revoked credential detected | Government record shows a revoked publication condition. | Compare with authoritative source/H01 history and route any inconsistency to governed follow-up. | Revocation discrepancy is traceable. | APAAR & DigiLocker / source domain |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Government comparison cannot be completed | Reconciliation cannot establish a current comparison outcome. | Record the H01 Synchronization as `Failed` and preserve the reconciliation attempt/history for later governed recovery. | APAAR & DigiLocker |
| Source-owned credential fact is disputed | H01 cannot choose a replacement source truth. | Route to the canonical source owner and preserve the discrepancy until authoritative resolution. | Source-owning domain / APAAR & DigiLocker |
| Discrepancy requires republication | Current H01 republication lifecycle is not fully defined upstream. | Route the issue to the republication architecture gap; do not invent resolution. | APAAR & DigiLocker / architecture governance |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Synchronization verifies government-record consistency and maintains synchronization history. | APAAR & DigiLocker | H01 owns reconciliation status, not source credential truth. | L2-H01 §11.4 |
| Every Synchronization execution produces a final outcome and failed synchronization remains recoverable. | APAAR & DigiLocker | Reconciliation reaches source-defined `Completed` or `Failed`. | L2-H01 §§11.4, 63 |
| H01 possible reconciliation outcomes include fully synchronized, missing publication, government mismatch, duplicate publication, revoked credential detected, and manual review required. | APAAR & DigiLocker | Discrepancy classification is limited to supported source semantics. | L2-H01 §34 |
| H01 never owns or changes academic credential business data. | Source domain / APAAR & DigiLocker | Source discrepancies must be resolved by the source-owning authority. | L2-H01 §§5–7, 16 |
| Synchronization/audit history is retained. | APAAR & DigiLocker / Platform Audit | Reconciliation cannot erase either history. | L2-H01 §52 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| H01 Synchronization | `Scheduled` | Progresses through comparison/reporting. | `Completed` or `Failed`. | APAAR & DigiLocker | L2-H01 §63 |
| H01 Publication | Existing publication states | Read/reconciled; may become subject to separate recovery/revocation/republication scenarios. | Unchanged by reconciliation unless another governed H01 scenario is invoked. | APAAR & DigiLocker | L2-H01 |
| Source credential | Existing source-owned state/history | Compared by reference only. | Unchanged by H01 reconciliation. | Applicable source domain | L2-H01 source-authority principle |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H01 Synchronization canonically publishes `ExternalProviderSynchronizationStarted`, `ApaarSynchronizationCompleted`, or `SynchronizationFailed` according to the event catalogue. The only named completion event is APAAR-specific, while the frozen scenario is generic across institutional and government credential reconciliation; no generic completion event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

H01 compares its publication references/government records with source-owned institutional credential facts. Discrepancies requiring a source correction, revocation, republication, or publication recovery must be handed to the applicable canonical owner/scenario. The reviewed H01 source does not define one universal cross-domain discrepancy-resolution contract.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

The business request may arise from a scheduled review, audit, recovery, or authorised administrative context. H01 does not provide a canonical business-actor catalogue for generic reconciliation, so no new actor is introduced.

## 15.2 Separation of Duties, Overrides and Exceptions

Reconciliation does not authorize H01 to change source-owned credential facts. Any source correction/revocation decision remains with the relevant canonical owner.

## 15.3 Business Audit and Evidence Obligations

Preserve Synchronization execution, connector/reference scope, comparison counts/results, discrepancy classifications, generated reconciliation report, source references, government references, follow-up routing, and immutable audit history.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- An authorised reconciliation request is established.
- Institutional/source and government records are compared.
- Discrepancies are classified using supported H01 outcomes.
- H01 reconciliation evidence/report is recorded.
- Resolved discrepancies use an existing authorised source path, or unresolved discrepancies are routed to governed follow-up.
- Both institutional and government histories remain preserved.
- H01 Synchronization reaches a source-defined final outcome.

## 16.2 Postconditions

- The H01 reconciliation outcome is traceable.
- No source-domain credential fact has been re-owned or silently overwritten.
- Unresolved discrepancies remain visible in governed follow-up.
- Synchronization and publication history remain reconstructable.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H01 defines only `ApaarSynchronizationCompleted` as the named synchronization completion event although H01 and the frozen scenario cover broader government platforms including DigiLocker. | L3 cannot rename this event or invent a generic `SynchronizationCompleted`. | Define a platform-neutral completion event if required for generic H01 reconciliation. |
| H01 classifies discrepancies and allows manual review, but does not define one universal cross-domain contract for source correction, revocation, republication, or recovery resolution. | L3 can route to governed follow-up but cannot prescribe source-domain resolution mechanics. | Register source-specific discrepancy-resolution hand-offs where mandatory. |
| H01 §65 uses stale section labels and Finance participation is not given a generic reconciliation responsibility. | Participant semantics cannot be inferred from the stale table. | Current `DEPENDENCY_MAP.md`/L1 ownership control; clarify Finance participation upstream. |

**Architecture assessment:** `PASS WITH ISSUES`

The H01 Synchronization aggregate, state machine, discrepancy outcomes, history preservation and governed manual-follow-up path support the frozen outcome. The generic completion-event and cross-domain resolution-contract gaps are recorded without inventing them.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions.
