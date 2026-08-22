# Corrected Credential to Controlled Republication

# 1. Scenario Metadata

**Scenario ID:** L3-H-005  
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
**Referenced By:** Government credential publication correction, revocation, reconciliation, and historical reconstruction

# 2. Scenario Purpose

Govern republication after a source-authorised credential correction while preserving source-domain credential ownership and the historical relationship between the earlier government publication and its corrected successor.

# 3. Business Outcome

A corrected version is republished with explicit lineage to the earlier publication and without duplicate active records

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authoritative source approves correction of a previously published credential.

## 4.2 End Boundary

The scenario would end when the corrected credential version is successfully republished under H01 with explicit lineage to the earlier publication and no duplicate active government-publication record.

## 4.3 In Scope

- Consuming the source-approved corrected credential reference.
- Confirming the earlier H01 Publication and its historical evidence.
- Applying source-defined H01 publication prerequisites to the corrected credential.
- Preserving explicit predecessor/successor publication lineage.
- Preventing duplicate active government-publication records.
- Recording the resulting corrected publication outcome and audit evidence.

## 4.4 Out of Scope

- Creating or approving the credential correction in H01.
- Rewriting or deleting the original source credential or prior H01 publication.
- Inventing republication states, commands, events, lineage rules, or duplicate-active-record rules not defined upstream.
- Technical retry, connector, persistence, or implementation design.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Student Lifecycle / Section C | Retains source-owned student/qualification context where applicable. |
| Academic Delivery / Section D | Retains source-owned academic context where applicable. |
| Finance / Section F | Frozen participating domain; exact republication responsibility is not defined by H01. |
| APAAR & DigiLocker / Section H | Owns H01 Publication records and government publication orchestration; however, its locked L2 does not define a complete corrected-publication successor lifecycle. |
| Examination / Section I | Retains credential/examination authority where the corrected credential is examination-owned. |

# 6. Business Actors, Roles and Responsibilities

The reviewed H01 source does not define a scenario-specific human actor or canonical business role for corrected credential republication. It states only that manual recovery may support re-publication after approval and that completed publications may execute again through an approved republish workflow. No new `Republish Approver` or equivalent role is invented.

# 7. Preconditions and Entry Criteria

- A corrected credential has been approved by its canonical source owner.
- A prior H01 Publication for the earlier credential version exists.
- The original H01 publication history is preserved.
- Applicable publication consent/policy and connector eligibility can be evaluated.
- The required republication lineage semantics must be defined upstream before scenario completion can be claimed.

# 8. Trigger

An authoritative source approves correction of a previously published credential

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Receive the authoritative corrected-credential approval/reference from its canonical source owner. | APAAR & DigiLocker consuming source-domain fact | H01 receives the corrected source reference without owning the correction decision. |
| 2 | Resolve the earlier H01 Publication and preserve its immutable publication history. | APAAR & DigiLocker | The predecessor publication is identified. |
| 3 | Revalidate H01 publication prerequisites, including applicable consent, connector eligibility, and duplicate-publication constraints. | APAAR & DigiLocker | A corrected credential is not treated as an unrestricted duplicate request. |
| 4 | Establish the corrected successor republication with explicit lineage to the earlier publication and a rule preventing duplicate active publication records. | APAAR & DigiLocker | **Blocked:** H01 does not define the successor-lineage model, active-record rule, republication command/event, or distinct republication lifecycle required by the frozen outcome. |
| 5 | Publish the corrected version and record the government acknowledgement under the authoritative republication lifecycle. | APAAR & DigiLocker | Cannot be completed canonically until the missing republication semantics are defined. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Correction is not source-approved | H01 receives an unapproved correction. | Do not republish; preserve the existing publication history. | No corrected republication occurs. | Source domain / APAAR & DigiLocker |
| Earlier publication cannot be resolved | Required predecessor history is unavailable. | Do not establish a corrected successor until the prior publication is authoritatively identified. | Lineage is not fabricated. | APAAR & DigiLocker |
| Corrected publication would duplicate an active government record | Duplicate-active semantics are required. | Do not claim successful republication until the upstream rule identifies the permitted predecessor/successor treatment. | Duplicate active record is not silently created. | APAAR & DigiLocker |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Republication lifecycle is undefined | H01 cannot prove how a completed Publication becomes or produces a corrected successor. | Preserve the corrected source decision and prior publication history; await upstream architecture definition. | APAAR & DigiLocker / architecture governance |
| Source correction lineage is unclear | H01 cannot relate the corrected credential to the correct predecessor. | Resolve the canonical source credential lineage before publication. | Source-owning domain |
| Government platform cannot accept corrected republication | Frozen final outcome cannot be achieved. | Record the unresolved publication condition under the future source-defined republication path; do not invent a terminal state. | APAAR & DigiLocker |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| H01 never modifies academic credentials; corrections originate in source-owned workflows. | Source domain / APAAR & DigiLocker | Corrected credential must already be approved upstream. | L2-H01 §§5–7, 16 |
| Completed publications cannot be executed again unless initiated through an approved republish or revocation workflow. | APAAR & DigiLocker | Generic duplicate execution is prohibited. | L2-H01 §40 |
| Manual recovery may support re-publication after approval. | APAAR & DigiLocker | H01 acknowledges republication as a legitimate concept. | L2-H01 §54 |
| Publication history is immutable and retained. | APAAR & DigiLocker / Platform Audit | Earlier publication cannot be overwritten by corrected publication. | L2-H01 §52 |
| No explicit republication state machine, command, event, or lineage rule is defined. | Architecture gap | L3 cannot manufacture the missing successor semantics. | Reviewed L2-H01 |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Source credential | Source-approved corrected version | Referenced only. | Source lifecycle remains source-owned. | Applicable source domain | L2-H01 source-authority principle |
| Earlier H01 Publication | Completed/`Published` history | Must remain immutable and traceable. | Prior state/history preserved. | APAAR & DigiLocker | L2-H01 §§40, 52, 60 |
| Corrected successor Publication | Not canonically defined as a distinct republication lifecycle | Frozen outcome requires explicit successor lineage and no duplicate active record. | **Undefined upstream.** | APAAR & DigiLocker | Upstream gap |

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

H01 defines standard Publication events and `CredentialRevoked`, but it does not define a canonical `CredentialRepublished`, corrected-publication successor event, or exact incoming corrected-credential event/publisher for this generic scenario. No republication event is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

The source-owning domain must hand H01 the approved corrected credential and its canonical correction lineage. H01 has a general Publication Contract, but no republication-specific business contract establishes predecessor/successor publication lineage or duplicate-active-record handling.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Approve corrected credential | Source-domain authority | Must be completed before H01 republication starts; exact role is source-specific. | Canonical credential-owning source domain |
| Approve/initiate H01 republication | Not canonically defined | H01 mentions approved republication but does not assign a business actor or approval authority. | Upstream gap |

## 15.2 Separation of Duties, Overrides and Exceptions

No canonical republication approval chain or separation-of-duty rule is defined. None is invented.

## 15.3 Business Audit and Evidence Obligations

The frozen outcome requires the earlier publication, corrected source reference, successor publication, government acknowledgement, and predecessor/successor lineage to remain reconstructable. H01 audit supports immutable Publication history, but the specific republication lineage fields/relationship are not defined upstream.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- Source-approved corrected credential is available.
- Earlier H01 Publication is authoritatively identified and preserved.
- H01 publication prerequisites are valid.
- A canonical republication lifecycle/contract establishes the corrected successor.
- Explicit predecessor/successor publication lineage is recorded.
- Duplicate active government-publication records are prevented under an authoritative rule.
- Corrected version is government-published and acknowledged.

The last four criteria cannot currently be established from locked H01 architecture.

## 16.2 Postconditions

- No compliant completed postcondition can be asserted beyond preservation of the source correction and earlier publication history until republication semantics are defined upstream.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| H01 acknowledges “re-publication after approval” and an “approved republish workflow” but defines no republication workflow, state transition, command, or event. | L3 cannot define the lifecycle required to produce the corrected successor publication. | Define the canonical H01 republication workflow/lifecycle and event model. |
| H01 does not define explicit predecessor/successor publication lineage for corrected versions. | The frozen outcome requires explicit lineage to the earlier publication. | Define the canonical corrected-publication lineage contract/model at L2. |
| H01 duplicate rules prevent generic repeated publication but do not define the rule for preventing duplicate active records during approved corrected republication. | L3 cannot determine how the old and corrected publication coexist or supersede one another. | Define the business rule for active corrected-publication identity/supersession. |
| Exact source-domain corrected-credential event/publisher and H01 republication approver are undefined. | Mandatory hand-off and authority remain ambiguous. | Register source event/contract and republication authority upstream. |

**Architecture assessment:** `BLOCKED`

The missing republication lifecycle, explicit lineage, duplicate-active-record rule, and authority directly affect the frozen final outcome. L3 cannot invent them.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

No additional scenario-specific deferred questions beyond the blocking issues.
