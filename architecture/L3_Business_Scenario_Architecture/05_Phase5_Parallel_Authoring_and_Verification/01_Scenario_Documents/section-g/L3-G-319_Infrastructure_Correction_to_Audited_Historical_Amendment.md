# Infrastructure Correction to Audited Historical Amendment

# 1. Scenario Metadata

**Scenario ID:** L3-G-319  
**Architecture Level:** L3 — Business Scenario Architecture  
**Primary Owner:** Infrastructure Management  
**Participating Domains:** G, A  
**Scenario Type:** Critical  
**Priority:** P0  
**Status:** Draft  
**Version:** 1.0  
**Owner:** Dhruv  
**Last Updated:** 2026-08-22  
**Depends On:** SutraERP L1 Business Domain Architecture; `DEPENDENCY_MAP.md`; L2-G10 — Infrastructure Management Workflows; L2-A07 — Audit and Compliance Workflows  
**Referenced By:** Authorized audit, investigation, governance and historical reconstruction activities

# 2. Scenario Purpose

Govern an authorised correction to a completed Infrastructure Management record without deleting, overwriting, or obscuring the original historical action, while ensuring the corrected successor information and amendment lineage remain auditable.

# 3. Business Outcome

The correction is recorded through governed procedures while the original historical action remains traceable

# 4. Scenario Boundary and Scope

## 4.1 Start Boundary

The scenario begins when an authorised correction is required for a completed infrastructure record.

## 4.2 End Boundary

The scenario ends when the corrected authoritative Infrastructure information is established through a governed amendment and the original record plus full correction lineage remain traceable.

## 4.3 In Scope

- Identification of the completed G10-owned record requiring correction.
- Authorization and business validation of the proposed correction.
- Preservation of immutable aggregate identity and historical evidence.
- Recording of previous and corrected values, reason and actor where applicable.
- Preservation of audit evidence under Section A audit ownership.
- Publication of the applicable canonical G10 change event when the corrected business fact corresponds to an already defined event.

## 4.4 Out of Scope

- Deleting or rewriting historical Infrastructure actions.
- Creating a local correction lifecycle, correction aggregate, or correction event not defined upstream.
- Changing records owned by another business domain.
- Technical rollback, database repair, implementation remediation, or system-recovery mechanics.

# 5. Ownership and Participating Domains

| Domain / Section | Responsibility in Scenario |
|---|---|
| Infrastructure Management / Section G | Owns the infrastructure record, corrected business fact, business validation and applicable Infrastructure lifecycle/attribute consequences. |
| Platform Foundation / Section A | Owns immutable enterprise audit evidence and the supporting authorization/audit framework. |

Audit participation does not transfer ownership of the Infrastructure record to Section A.

# 6. Business Actors, Roles and Responsibilities

| Business Actor / Role | Responsibility in Scenario | Participation / Authority |
|---|---|---|
| Infrastructure Administrator | Initiates or applies an authorised correction to a G10 record within assigned scope. | Must satisfy G10 authorization and business rules. |
| Campus Administrator | May correct campus-scoped information within authorised responsibility. | Limited to assigned campus scope. |
| Auditor | Reviews original history, amendment lineage and audit evidence. | Read-only audit participation. |

G10 notes that some administrative data corrections may require elevated governance approval, but no universal correction approver is canonically named.

# 7. Preconditions and Entry Criteria

- The target Infrastructure record exists and is owned by G10.
- The original completed action/history remains available.
- The correction requirement is authorised.
- The proposed corrected value is valid under the applicable G10 business rules.
- The correction does not attempt to mutate an immutable Infrastructure identifier.

# 8. Trigger

An authorised correction is required for a completed infrastructure record

# 9. Main Business Flow

| Step | Business Behaviour | Responsible Business Role / Domain | Business Result |
|---|---|---|---|
| 1 | Identify the completed Infrastructure record and the specific business fact requiring correction. | Infrastructure Management | The correction target is unambiguous. |
| 2 | Confirm that the fact is G10-owned and that the correction does not cross into another domain's canonical ownership. | Infrastructure Management | Ownership remains canonical. |
| 3 | Validate authorization, applicable business rules, lifecycle constraints and any required governance approval. | Infrastructure Management / Section A authorization support | Only an authorised and valid correction may proceed. |
| 4 | Record the corrected value as the current authoritative business fact while preserving the prior value and original completed action in history. | Infrastructure Management | The record is corrected without erasing history. |
| 5 | Record immutable audit evidence for the correction, including actor, timestamp, previous/new values, business reason and record reference where applicable. | Platform Foundation audit / Infrastructure Management | Correction lineage is auditable. |
| 6 | Publish the applicable existing canonical G10 business event where the corrected fact maps to a defined update event. | Infrastructure Management | Consumers receive the corrected authoritative fact without a fabricated generic correction event. |

# 10. Alternate Business Flows

| Case | Condition | Alternate Business Path | Result / Outcome | Responsible Domain |
|---|---|---|---|---|
| Correction is invalid | Proposed value violates G10 rules or lifecycle constraints. | Reject the correction and preserve the current authoritative record. | No amendment occurs. | Infrastructure Management |
| Correction targets immutable identity | Request attempts to change an immutable Infrastructure identifier. | Reject the identity change and require the business issue to be resolved without mutating identity. | Canonical identity remains unchanged. | Infrastructure Management |
| Corrected fact belongs to another domain | The identified value is not canonically owned by G10. | Route the correction requirement to the source-owning domain and retain only the permitted G10 reference. | Ownership remains source-domain controlled. | Infrastructure Management / source-owning domain |
| Elevated approval is required by policy | Governance policy requires approval before administrative data correction. | Await the upstream-governed approval outcome before applying the correction. | Correction proceeds only after the required approval. | Infrastructure Management / governance authority |

# 11. Failure, Interruption and Business Recovery

| Situation | Business Consequence | Business Resolution / Recovery | Responsible Domain |
|---|---|---|---|
| Original history is not available | Historical truth cannot be safely preserved. | Do not apply the correction until the authoritative history is recoverable. | Infrastructure Management / audit governance |
| Audit evidence cannot be established | Correction would lose required accountability. | Do not complete the amendment until audit obligations can be satisfied. | Platform Foundation / Infrastructure Management |
| Consumer uses stale corrected fact | Downstream context may temporarily remain inconsistent. | G10 republishes only the applicable canonical business fact/event already defined for that attribute or lifecycle change. | Infrastructure Management |

# 12. Business Rules and Decision Constraints

| Business Rule / Constraint | Owning Domain | Scenario Effect | Authoritative Source |
|---|---|---|---|
| Infrastructure identity is immutable. | Infrastructure Management | Corrections cannot replace aggregate identity. | L2-G10 §§8, 47 |
| Infrastructure history shall never be deleted and historical reconstruction must remain possible. | Infrastructure Management | Original completed action remains traceable. | L2-G10 §6 Complete Auditability; §47 |
| Every significant business action and lifecycle/administrative change is auditable. | Infrastructure Management / Platform Foundation | Correction must create immutable evidence. | L2-G10 §§48A.208–210, 49 |
| Administrative data corrections may require elevated governance approval. | Governance / Infrastructure Management | Approval is applied only when source policy requires it; L3 does not invent a universal approver. | L2-G10 §48A.211 |
| Business events communicate completed canonical facts. | Infrastructure Management | Use only the existing event corresponding to the corrected fact; no generic correction event is invented. | L2-G10 §44A |

# 13. State and Lifecycle Impact

| Owned Business Concept / Aggregate | Relevant Entry State(s) | Scenario Impact | Resulting State / Lifecycle Position | Canonical Owner | Authoritative Source |
|---|---|---|---|---|---|
| Infrastructure aggregate / record | Any source-defined non-immutable context in which the corrected fact may validly change | Corrects the affected business attribute or record content. | Existing lifecycle remains unchanged unless the applicable authoritative correction itself is a lifecycle transition. | Infrastructure Management | L2-G10 |
| Audit evidence | Existing immutable history | Adds correction evidence and lineage. | Original evidence remains immutable; new evidence records the amendment. | Platform Foundation | L2-A07 / L2-G10 audit obligations |

No separate `Correction` lifecycle is defined upstream and none is introduced.

# 14. Business Events and Cross-Domain Collaboration

## 14.1 Published / Consumed Business Events

G10 defines multiple canonical update events for specific changed facts, including examples such as `CampusUpdated`, `BuildingUpdated`, `FloorUpdated`, `SpaceUpdated`, `RoomCapacityChanged`, `InfrastructureClassificationChanged`, `SpaceAreaUpdated`, `SpaceAccessibilityUpdated`, `SustainabilityProfileUpdated`, `SpaceHealthUpdated`, and `SpaceCriticalityChanged`.

The scenario uses the event corresponding to the actual corrected business fact when such an event is defined. No canonical `InfrastructureCorrected` or `HistoricalAmendmentRecorded` event is defined in the reviewed G10 source, so neither is invented.

## 14.2 Business Contracts and Cross-Domain Hand-offs

Section A audit evidence records the correction lineage while G10 remains the source of the corrected Infrastructure business fact. Consumers continue to resolve current Infrastructure facts through approved G10 contracts/events.

# 15. Business Authorization, Approval and Audit Obligations

## 15.1 Initiation and Decision Authority

| Business Action / Decision | Authorized Business Actor / Role | Business Authority / Constraint | Business-Domain Owner |
|---|---|---|---|
| Initiate/apply Infrastructure correction | Infrastructure Administrator or applicable scoped administrator | Requires source authorization and valid G10 business change. | Infrastructure Management |
| Review correction evidence | Auditor | Read-only access to original and amended history. | Platform audit / Infrastructure Management evidence |

## 15.2 Separation of Duties, Overrides and Exceptions

G10 states that administrative data corrections are an example of operations that may require elevated approval and that approval orchestration belongs to Workflow or Governance domains. The exact approving actor, approval threshold and separation-of-duty model are not universally defined. The Draft therefore applies only explicitly configured upstream approval requirements.

## 15.3 Business Audit and Evidence Obligations

Correction evidence must preserve, where applicable, the original record/action, actor, timestamp, previous values, corrected values, business reason, aggregate reference, approval reference and immutable audit lineage.

# 16. Completion Criteria and Postconditions

## 16.1 Completion Criteria

- The target record is confirmed as G10-owned.
- The correction is authorised and business-valid.
- Immutable identity remains unchanged.
- The corrected fact becomes the current authoritative G10 value.
- The original historical action/value remains traceable.
- Required immutable audit evidence is recorded.
- Any applicable existing canonical change event is published.

## 16.2 Postconditions

- Consumers can resolve the corrected current Infrastructure fact from G10.
- Original history and correction lineage remain available for audit and historical reconstruction.
- No duplicate correction ownership or new local lifecycle/event has been introduced.

# 17. Open Issues, Assumptions and Deferred Questions

## 17.1 Open Issues

| Issue | Architecture Impact | Required Resolution |
|---|---|---|
| G10 does not define one canonical generic correction/amendment event. | Consumers cannot rely on a universal correction event; each corrected fact must use its existing source-defined event where available. | Non-blocking for this scenario because the frozen outcome requires governed correction and historical traceability, not a named generic event. |
| G10 states that administrative data corrections may require elevated approval but does not define a universal approving business role. | Approval steps vary by upstream policy and cannot be made mandatory locally. | Define correction-specific approval authority upstream if a universal approval rule is required. |

**Architecture assessment:** `PASS WITH ISSUES`

The complete business outcome can be described from locked G10 and audit obligations without inventing ownership, lifecycle or a new event. The remaining gaps concern a generic correction event and universal approval authority, neither of which is required to preserve the frozen outcome.

## 17.2 Assumptions

No assumptions.

## 17.3 Deferred Questions

- Should a future upstream governance decision define one canonical correction/amendment business event across Infrastructure records, or should corrections remain expressed through the event for the corrected business fact?
